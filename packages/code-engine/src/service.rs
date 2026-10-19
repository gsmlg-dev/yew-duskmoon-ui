//! Asynchronous, document-scoped syntax highlighting in a browser Worker.

use std::cell::Cell;
use std::fmt;
use std::rc::Rc;

use crate::{CodeLanguage, SyntaxToken, SyntaxTokenKind};

/// Location of the self-hosted assets written by [`crate::write_syntax_assets`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SyntaxConfig {
    /// Directory URL, resolved against `document.baseURI` in the browser.
    pub assets_url: String,
}

impl Default for SyntaxConfig {
    fn default() -> Self {
        Self {
            assets_url: "tree-sitter/".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SyntaxError {
    UnsupportedPlatform,
    UnsupportedLanguage(String),
    Worker(String),
    Closed,
    Stale,
    InvalidResponse(String),
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPlatform => output.write_str("syntax highlighting requires a browser"),
            Self::UnsupportedLanguage(language) => {
                write!(output, "no Tree-sitter grammar is bundled for {language}")
            }
            Self::Worker(message) => write!(output, "syntax Worker failed: {message}"),
            Self::Closed => output.write_str("syntax document is closed"),
            Self::Stale => output.write_str("syntax result was superseded by a newer revision"),
            Self::InvalidResponse(message) => write!(output, "invalid syntax result: {message}"),
        }
    }
}

impl std::error::Error for SyntaxError {}

/// A non-overlapping highlighted range. Offsets are UTF-8 bytes, not DOM offsets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HighlightSpan {
    pub start_byte: usize,
    pub end_byte: usize,
    /// Tree-sitter capture name, for example `comment` or `function.method`.
    pub capture: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HighlightResult {
    pub document_id: u32,
    pub revision: u32,
    pub language: String,
    pub source: String,
    pub spans: Vec<HighlightSpan>,
}

impl HighlightResult {
    /// Adapt spans to the existing editor token API, including all plain-text gaps.
    /// Rejects results for another snapshot or malformed UTF-8 ranges.
    pub fn tokens(&self, source: &str) -> Result<Vec<SyntaxToken>, SyntaxError> {
        if self.source != source {
            return Err(SyntaxError::Stale);
        }
        let mut tokens: Vec<SyntaxToken> = Vec::new();
        let mut offset = 0;
        let mut push = |kind, text: &str| {
            if text.is_empty() {
                return;
            }
            if let Some(last) = tokens.last_mut().filter(|last| last.kind == kind) {
                last.text.push_str(text);
            } else {
                tokens.push(SyntaxToken {
                    kind,
                    text: text.to_owned(),
                });
            }
        };
        for span in &self.spans {
            if span.start_byte < offset || span.start_byte >= span.end_byte {
                return Err(SyntaxError::InvalidResponse(
                    "overlapping or empty ranges".into(),
                ));
            }
            let plain = source
                .get(offset..span.start_byte)
                .ok_or_else(|| SyntaxError::InvalidResponse("invalid UTF-8 range".into()))?;
            let highlighted = source
                .get(span.start_byte..span.end_byte)
                .ok_or_else(|| SyntaxError::InvalidResponse("invalid UTF-8 range".into()))?;
            push(SyntaxTokenKind::Plain, plain);
            push(capture_kind(&span.capture), highlighted);
            offset = span.end_byte;
        }
        push(SyntaxTokenKind::Plain, &source[offset..]);
        Ok(tokens)
    }
}

fn capture_kind(capture: &str) -> SyntaxTokenKind {
    if capture.starts_with("constant.numeric") {
        return SyntaxTokenKind::Number;
    }
    if capture == "constant.character" {
        return SyntaxTokenKind::String;
    }
    if capture.starts_with("text.") || capture.starts_with("markup.") {
        return match capture.split('.').nth(1).unwrap_or("") {
            "literal" | "raw" => SyntaxTokenKind::String,
            "uri" | "reference" | "link" => SyntaxTokenKind::Function,
            _ => SyntaxTokenKind::Keyword,
        };
    }
    match capture.split('.').next().unwrap_or(capture) {
        "keyword" | "boolean" | "constant" | "tag" | "include" | "preproc" | "storageclass"
        | "conditional" | "repeat" | "exception" => SyntaxTokenKind::Keyword,
        "type" | "constructor" | "attribute" | "property" | "field" | "namespace" | "module" => {
            SyntaxTokenKind::Type
        }
        "string" | "character" | "escape" => SyntaxTokenKind::String,
        "number" | "float" => SyntaxTokenKind::Number,
        "comment" => SyntaxTokenKind::Comment,
        "function" | "method" | "label" => SyntaxTokenKind::Function,
        "variable" if capture.ends_with(".builtin") => SyntaxTokenKind::Type,
        "operator" | "punctuation" | "delimiter" => SyntaxTokenKind::Punctuation,
        _ => SyntaxTokenKind::Plain,
    }
}

struct ServiceInner {
    config: SyntaxConfig,
    next_document: Cell<u32>,
    #[cfg(target_arch = "wasm32")]
    client: std::cell::RefCell<Option<Rc<browser::WorkerClient>>>,
}

/// A shared, lazily initialized module Worker. Clones share languages and documents.
///
/// Network and parsing work starts only when a document is updated. Native callers
/// receive [`SyntaxError::UnsupportedPlatform`]; no browser API runs on creation.
#[derive(Clone)]
pub struct SyntaxService(Rc<ServiceInner>);

impl PartialEq for SyntaxService {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

thread_local! {
    static DEFAULT_SERVICE: SyntaxService = SyntaxService::new(SyntaxConfig::default());
}

impl Default for SyntaxService {
    fn default() -> Self {
        DEFAULT_SERVICE.with(Clone::clone)
    }
}

impl SyntaxService {
    pub fn new(config: SyntaxConfig) -> Self {
        Self(Rc::new(ServiceInner {
            config,
            next_document: Cell::new(0),
            #[cfg(target_arch = "wasm32")]
            client: std::cell::RefCell::new(None),
        }))
    }

    pub fn config(&self) -> &SyntaxConfig {
        &self.0.config
    }

    /// Open a lazy document session. Keep it for incremental editor updates.
    pub fn document(&self) -> SyntaxDocument {
        let id = self.0.next_document.get() + 1;
        self.0.next_document.set(id);
        SyntaxDocument(Rc::new(DocumentInner {
            service: self.clone(),
            id,
            revision: Cell::new(0),
            closed: Cell::new(false),
        }))
    }

    /// Highlight a standalone snapshot and release its parser/tree afterwards.
    pub async fn highlight(
        &self,
        language: CodeLanguage,
        source: &str,
    ) -> Result<HighlightResult, SyntaxError> {
        self.highlight_language(language.as_str(), source).await
    }

    /// Accept grammar identifiers such as `go` and `tsx`, as well as common aliases.
    pub async fn highlight_language(
        &self,
        language: &str,
        source: &str,
    ) -> Result<HighlightResult, SyntaxError> {
        self.document().update_language(language, source).await
    }
}

struct DocumentInner {
    service: SyntaxService,
    id: u32,
    revision: Cell<u32>,
    closed: Cell<bool>,
}

impl DocumentInner {
    fn close(&self) {
        #[cfg(target_arch = "wasm32")]
        if self.closed.replace(true) {
            return;
        }
        #[cfg(not(target_arch = "wasm32"))]
        self.closed.set(true);
        #[cfg(target_arch = "wasm32")]
        if let Some(client) = self.service.0.client.borrow().as_ref() {
            client.close(self.id);
        }
    }
}

impl Drop for DocumentInner {
    fn drop(&mut self) {
        self.close();
    }
}

/// A document owned by the Worker. Closing any clone invalidates pending requests.
#[derive(Clone)]
pub struct SyntaxDocument(Rc<DocumentInner>);

impl PartialEq for SyntaxDocument {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl SyntaxDocument {
    pub fn id(&self) -> u32 {
        self.0.id
    }

    pub fn close(&self) {
        self.0.close();
    }

    pub async fn update(
        &self,
        language: CodeLanguage,
        source: &str,
    ) -> Result<HighlightResult, SyntaxError> {
        self.update_language(language.as_str(), source).await
    }

    /// Send an authoritative snapshot. The Worker derives an edit against its
    /// previous snapshot and reuses the previous tree; highlight queries cover
    /// the complete tree. This still copies and compares the complete source.
    pub async fn update_language(
        &self,
        language: &str,
        source: &str,
    ) -> Result<HighlightResult, SyntaxError> {
        if self.0.closed.get() {
            return Err(SyntaxError::Closed);
        }
        let revision = self.0.revision.get() + 1;
        self.0.revision.set(revision);
        let language = canonical_language(language);
        if language == "text" || source.is_empty() {
            return Ok(HighlightResult {
                document_id: self.id(),
                revision,
                language,
                source: source.to_owned(),
                spans: Vec::new(),
            });
        }
        #[cfg(target_arch = "wasm32")]
        {
            let client = self.0.service.0.client()?;
            let value = client
                .request(self.id(), revision, &language, source)
                .map_err(browser::error)?;
            let value = wasm_bindgen_futures::JsFuture::from(value)
                .await
                .map_err(browser::error)?;
            if self.0.closed.get() {
                return Err(SyntaxError::Closed);
            }
            if self.0.revision.get() != revision {
                return Err(SyntaxError::Stale);
            }
            browser::decode(value, self.id(), revision, &language, source)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = &self.0.service;
            Err(SyntaxError::UnsupportedPlatform)
        }
    }
}

fn canonical_language(language: &str) -> String {
    match language.trim().to_ascii_lowercase().as_str() {
        "go" | "golang" => "go".into(),
        "tsx" | "typescriptreact" => "tsx".into(),
        "jsx" | "javascriptreact" => "javascript".into(),
        value => value.parse::<CodeLanguage>().map_or_else(
            |_| value.to_owned(),
            |language| language.as_str().to_owned(),
        ),
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn utf16_spans(
    source: &str,
    spans: impl IntoIterator<Item = (usize, usize, String)>,
) -> Result<Vec<HighlightSpan>, SyntaxError> {
    let mut boundaries = vec![Some(0)];
    for (byte, ch) in source.char_indices() {
        if ch.len_utf16() == 2 {
            boundaries.push(None);
        }
        boundaries.push(Some(byte + ch.len_utf8()));
    }
    let mut result = Vec::new();
    let mut previous_end = 0;
    for (start, end, capture) in spans {
        if start < previous_end || start >= end {
            return Err(SyntaxError::InvalidResponse(
                "overlapping or empty ranges".into(),
            ));
        }
        let boundary = |index| {
            boundaries
                .get(index)
                .copied()
                .flatten()
                .ok_or_else(|| SyntaxError::InvalidResponse("invalid UTF-16 boundary".into()))
        };
        result.push(HighlightSpan {
            start_byte: boundary(start)?,
            end_byte: boundary(end)?,
            capture,
        });
        previous_end = end;
    }
    Ok(result)
}

#[cfg(target_arch = "wasm32")]
mod browser {
    use serde::Deserialize;
    use wasm_bindgen::prelude::*;

    use super::*;

    #[wasm_bindgen(module = "/src/worker_client.js")]
    extern "C" {
        pub type WorkerClient;
        #[wasm_bindgen(constructor, catch)]
        pub fn new(assets_url: &str) -> Result<WorkerClient, JsValue>;
        #[wasm_bindgen(method, catch)]
        pub fn request(
            this: &WorkerClient,
            document_id: u32,
            revision: u32,
            language: &str,
            source: &str,
        ) -> Result<js_sys::Promise, JsValue>;
        #[wasm_bindgen(method)]
        pub fn close(this: &WorkerClient, document_id: u32);
        #[wasm_bindgen(method)]
        pub fn dispose(this: &WorkerClient);
    }

    impl ServiceInner {
        pub fn client(&self) -> Result<Rc<WorkerClient>, SyntaxError> {
            if let Some(client) = self.client.borrow().as_ref() {
                return Ok(client.clone());
            }
            let client = Rc::new(WorkerClient::new(&self.config.assets_url).map_err(error)?);
            *self.client.borrow_mut() = Some(client.clone());
            Ok(client)
        }
    }

    impl Drop for ServiceInner {
        fn drop(&mut self) {
            if let Some(client) = self.client.get_mut().take() {
                client.dispose();
            }
        }
    }

    pub fn error(value: JsValue) -> SyntaxError {
        let field = |name| {
            js_sys::Reflect::get(&value, &JsValue::from_str(name))
                .ok()
                .and_then(|value| value.as_string())
                .unwrap_or_default()
        };
        match field("code").as_str() {
            "stale" => SyntaxError::Stale,
            "closed" => SyntaxError::Closed,
            "unsupported_language" => SyntaxError::UnsupportedLanguage(field("language")),
            _ => SyntaxError::Worker(value.as_string().unwrap_or_else(|| field("message"))),
        }
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        document_id: u32,
        revision: u32,
        language: String,
        source: String,
        spans: Vec<Span>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Span {
        start_index: usize,
        end_index: usize,
        capture: String,
    }

    pub fn decode(
        value: JsValue,
        document_id: u32,
        revision: u32,
        language: &str,
        source: &str,
    ) -> Result<HighlightResult, SyntaxError> {
        let value: Response = serde_wasm_bindgen::from_value(value)
            .map_err(|error| SyntaxError::InvalidResponse(error.to_string()))?;
        if value.document_id != document_id
            || value.revision != revision
            || value.language != language
            || value.source != source
        {
            return Err(SyntaxError::Stale);
        }
        let spans = utf16_spans(
            source,
            value
                .spans
                .into_iter()
                .map(|span| (span.start_index, span.end_index, span.capture)),
        )?;
        Ok(HighlightResult {
            document_id,
            revision,
            language: value.language,
            source: value.source,
            spans,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result(source: &str, spans: Vec<HighlightSpan>) -> HighlightResult {
        HighlightResult {
            document_id: 1,
            revision: 1,
            language: "rust".into(),
            source: source.into(),
            spans,
        }
    }

    #[test]
    fn translates_utf16_ranges_after_chinese_and_emoji() {
        let source = "中文🙂 fn";
        let spans = utf16_spans(source, [(5, 7, "keyword".into())]).unwrap();
        assert_eq!(&source[spans[0].start_byte..spans[0].end_byte], "fn");
        let tokens = result(source, spans).tokens(source).unwrap();
        assert_eq!(
            tokens
                .iter()
                .map(|token| token.text.as_str())
                .collect::<String>(),
            source
        );
        assert_eq!(tokens[1].kind, SyntaxTokenKind::Keyword);
    }

    #[test]
    fn rejects_surrogate_splits_overlaps_and_out_of_bounds_ranges() {
        for ranges in [
            vec![(1, 2, "string".into())],
            vec![(0, 5, "string".into())],
            vec![(0, 2, "string".into()), (1, 2, "keyword".into())],
        ] {
            assert!(utf16_spans("🙂!", ranges).is_err());
        }
    }

    #[test]
    fn rejects_wrong_snapshot_and_invalid_utf8_spans() {
        let highlighted = result(
            "🙂!",
            vec![HighlightSpan {
                start_byte: 1,
                end_byte: 4,
                capture: "string".into(),
            }],
        );
        assert_eq!(highlighted.tokens("new"), Err(SyntaxError::Stale));
        assert!(matches!(
            highlighted.tokens("🙂!"),
            Err(SyntaxError::InvalidResponse(_))
        ));
    }

    #[test]
    fn default_services_share_identity_and_documents_do_not() {
        let service = SyntaxService::default();
        assert!(service == SyntaxService::default());
        let first = service.document();
        let second = service.document();
        assert!(first != second);
        assert_ne!(first.id(), second.id());
        first.close();
        assert!(first.0.closed.get());
        assert!(!second.0.closed.get());
    }

    #[test]
    fn recognizes_browser_grammar_aliases() {
        assert_eq!(canonical_language("rs"), "rust");
        assert_eq!(canonical_language("golang"), "go");
        assert_eq!(canonical_language("tsx"), "tsx");
        assert_eq!(canonical_language("yml"), "yaml");
    }

    #[test]
    fn styles_web_and_markdown_captures() {
        for capture in [
            "tag",
            "attribute",
            "property",
            "module",
            "namespace",
            "text.title",
            "text.emphasis",
            "text.literal",
            "text.uri",
            "markup.heading",
            "markup.link",
        ] {
            assert_ne!(capture_kind(capture), SyntaxTokenKind::Plain, "{capture}");
        }
        assert_eq!(capture_kind("variable"), SyntaxTokenKind::Plain);
        assert_eq!(capture_kind("constant.numeric"), SyntaxTokenKind::Number);
        assert_eq!(capture_kind("constant.character"), SyntaxTokenKind::String);
    }
}
