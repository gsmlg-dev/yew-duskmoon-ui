# code-engine

`code-engine` is the Rust editor foundation used by `yew-duskmoon`.

The crate exposes:

- `TextDocument` for line-aware source text operations
- `CodeLanguage` for editor language metadata
- `CursorStatus` for logical and visual cursor reporting
- `SyntaxService` and `SyntaxDocument` for asynchronous Tree-sitter highlighting
- `highlight_tokens` for legacy synchronous syntax tokenization
- `CodeEditor` for a Yew editor component with controlled and uncontrolled value modes,
  syntax highlighting, status metadata, and a rendered block cursor

In browsers, `CodeEditor` requests highlighting through a Web Worker so parsing does
not block typing. Each editor keeps a document session for incremental parsing and
closes it when the component unmounts. Source and language changes request new
highlights; cursor movement, focus, and scrolling reuse the current result.

The editor displays plain source while highlighting is pending or unavailable.
Results from obsolete requests cannot replace the current source's highlighting.
Both controlled `value` updates and uncontrolled edits follow the same asynchronous
path. The `syntax_service` prop accepts a configured `SyntaxService` when an
application needs to override the default worker assets. Native consumers can use
the document and cursor utilities; the browser highlighting service reports an
unsupported platform error outside WebAssembly.

`CodeLanguage::ALL` contains every language mode: plain text, Rust, Zig, C, C++,
Objective-C, C#/.NET, Java, Kotlin, Swift, Python, Ruby, PHP, JavaScript, TypeScript,
JSON, Markdown, HTML, CSS, Sass, SCSS, Less, Elixir, Erlang, Lisp, Emacs Lisp, Shell,
Bash, Zsh, Nix, Assembly, WebAssembly, TOML, and YAML.

Every code mode above has a bundled Tree-sitter grammar and upstream highlight
queries. The bundle also supports Go and TSX through `highlight_language`. Plain
text skips parsing; Shell shares the Bash grammar, while Zsh, indented Sass, SCSS,
and Less use their own grammars. Markdown loads its block and inline grammars and
fenced-code languages; HTML and PHP use upstream injection queries for embedded
code. Grammars load on demand and are cached by the shared service. Unknown
languages passed to the asynchronous API return `SyntaxError::UnsupportedLanguage`.

Export the bundled assets to a directory served by the application. A native
`build.rs` can call `code_engine::write_syntax_assets("static/tree-sitter")` with
`code-engine` added under `[build-dependencies]`. From this workspace:

```sh
cargo run -p code-engine --example export_syntax_assets -- example/static/tree-sitter
```

With Trunk, add `<link data-trunk rel="copy-dir" href="./tree-sitter">` to the
HTML shell next to that directory. Export before Trunk copies assets, for example
in a pre-build hook; a `build.rs` running during compilation can be too late on
the first build. The demo exports assets automatically through its Trunk pre-build
hook. No npm install or CDN access is required by consumers.
Serve `.wasm` files as `application/wasm` and `.js` files as JavaScript modules.

The default URL is `tree-sitter/`, relative to `document.baseURI`, including any
deployment subpath. Override it by sharing a configured service:

```rust,no_run
use code_engine::{CodeLanguage, SyntaxConfig, SyntaxError, SyntaxService};

async fn highlight() -> Result<(), SyntaxError> {
    let syntax = SyntaxService::new(SyntaxConfig {
        assets_url: "/my-app/tree-sitter/".into(),
    });
    let result = syntax.highlight(CodeLanguage::Rust, "fn main() {}").await?;
    let tokens = result.tokens("fn main() {}")?;
    assert!(!tokens.is_empty());
    Ok(())
}
```

For continuous editing, keep `let document = syntax.document()` and call
`document.update(language, source).await` for each new snapshot. Call
`document.close()` on disposal; dropping the last document handle also closes it.
Each snapshot has a revision, and superseded or closed requests return a typed
error. Spans use UTF-8 byte offsets; the bridge validates and converts the Worker's
UTF-16 offsets. Parsing reuses the previous edited tree, while highlight queries
currently traverse the full tree and snapshots still copy the full source.

Assets are pinned to official `web-tree-sitter` 0.27.1 and the grammar versions in
`scripts/sources.json`. Their upstream licenses ship in `assets/licenses/`.
Maintainers can reproduce the bundle with
`python3 packages/code-engine/scripts/vendor_syntax_assets.py` using Tree-sitter CLI
0.26.8 on `PATH`; this CLI downloads WASI SDK 29 for source builds. Most parsers use upstream prebuilt WASM;
grammars without published WASM compile from pinned sources. The script verifies
every download's integrity before replacing assets and generates the Rust export
manifest. Consumers do not need these build tools. Common Lisp uses the matching
Helix highlight query; inherited C/CSS/JavaScript queries are composed with their
dependent grammars. All source and query licenses are included. Run worker/runtime tests
with `node --test packages/code-engine/tests/*.test.mjs`.

The vendoring script adds `injection.include-children` to Markdown injection
queries so embedded parsers receive punctuation from the block and inline trees.
For Zsh 0.63.1, `scripts/split_zsh_lexer.py` partitions the generated lexer's
unchanged state bodies into bounded functions before compilation. This avoids a
V8 optimizer out-of-memory crash on its original giant function while retaining
the genuine Zsh grammar, queries, and incremental parsing behavior. The build
checks the pinned lexer shape; grammar upgrades must revalidate this adaptation.

```rust
use yew::prelude::*;
use code_engine::{CodeEditor, CodeLanguage};

html! {
    <CodeEditor
        language={CodeLanguage::Rust}
        default_value={"fn main() {\n    println!(\"hello\");\n}"}
    />
}
```
