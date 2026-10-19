import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import test, { after } from 'node:test';
import { fileURLToPath } from 'node:url';
import { Language, Parser, Query } from '../assets/web-tree-sitter.js';
import { createWorkerHandler, resolveLanguage } from '../assets/worker.js';

const assets = process.env.SYNTAX_INJECTION_ASSETS
  ? resolve(process.env.SYNTAX_INJECTION_ASSETS)
  : fileURLToPath(new URL('../assets/', import.meta.url));
const manifest = JSON.parse(await readFile(`${assets}/manifest.json`, 'utf8'));
await Parser.init({ locateFile: () => `${assets}/${manifest.runtime.wasm}` });
const languages = new Map();

async function loadLanguage(requested) {
  const id = resolveLanguage(manifest, requested);
  if (!languages.has(id)) {
    languages.set(id, (async () => {
      const entry = manifest.languages[id];
      const language = await Language.load(`${assets}/${entry.wasm}`);
      const query = new Query(language, await readFile(`${assets}/${entry.query}`, 'utf8'));
      // Tests require every configured injection query to compile correctly.
      const injections = entry.injections
        ? new Query(language, await readFile(`${assets}/${entry.injections}`, 'utf8'))
        : null;
      return {
        id, query, injections,
        createParser() {
          const parser = new Parser();
          parser.setLanguage(language);
          return parser;
        },
      };
    })());
  }
  return languages.get(id);
}

after(async () => {
  for (const loading of languages.values()) {
    const grammar = await loading;
    grammar.injections?.delete();
    grammar.query.delete();
  }
});

function worker() {
  let nextRequest = 0;
  let revision = 0;
  const pending = new Map();
  const handle = createWorkerHandler(loadLanguage, result => {
    const resolve = pending.get(result.id);
    pending.delete(result.id);
    resolve(result);
  });
  return {
    highlight(language, source) {
      const id = ++nextRequest;
      return new Promise(resolve => {
        pending.set(id, resolve);
        handle({ id, type: 'highlight', documentId: 1, revision: ++revision, language, source });
      });
    },
    close() { handle({ type: 'close', documentId: 1 }); },
  };
}

function hasCapture(result, text, root) {
  return result.spans.some(span => span.capture.split('.')[0] === root
    && result.source.slice(span.startIndex, span.endIndex) === text);
}

test('HTML injects JavaScript and CSS at original Unicode and CRLF offsets', async () => {
  const session = worker();
  const source = '😀\r\n<div><script>const answer = "😃"; // comment\n</script><style>.demo { color: red; }</style></div>';
  try {
    const result = await session.highlight('html', source);
    assert.ok(!result.error, JSON.stringify(result.error));
    assert.ok(hasCapture(result, 'div', 'tag'));
    assert.ok(hasCapture(result, 'const', 'keyword'), 'script contents use the JavaScript grammar');
    assert.ok(hasCapture(result, '"😃"', 'string'));
    assert.ok(hasCapture(result, 'color', 'property'), 'style contents use the CSS grammar');
    const keyword = result.spans.find(span => span.capture === 'keyword' && source.slice(span.startIndex, span.endIndex) === 'const');
    assert.equal(keyword.startIndex, source.indexOf('const'));
    const edited = source.replace('😀', '中文😀').replace('const answer', 'let changed');
    const incremental = await session.highlight('html', edited);
    const freshSession = worker();
    try {
      const fresh = await freshSession.highlight('html', edited);
      assert.deepEqual(incremental.spans, fresh.spans);
      assert.ok(hasCapture(incremental, 'let', 'keyword'));
    } finally {
      freshSession.close();
    }
  } finally {
    session.close();
  }
});

test('PHP combines HTML ranges and recursively highlights nested script contents', async () => {
  const session = worker();
  const source = '<div class="demo"><?php echo "😀"; ?><script>const answer = "😃";</script></div>';
  try {
    const result = await session.highlight('php', source);
    assert.ok(!result.error, JSON.stringify(result.error));
    assert.ok(hasCapture(result, 'echo', 'keyword'), 'primary PHP highlighting remains intact');
    assert.ok(hasCapture(result, 'div', 'tag'), 'outer HTML fragments use the HTML grammar');
    assert.ok(hasCapture(result, 'const', 'keyword'), 'PHP to HTML to JavaScript recursion preserves source ranges');
    assert.ok(hasCapture(result, '"😃"', 'string'));
    assert.ok(result.spans.every((span, index) => index === 0 || result.spans[index - 1].endIndex <= span.startIndex));
  } finally {
    session.close();
  }
});

test('Markdown preserves headings and injects inline emphasis, code, and links', async () => {
  const session = worker();
  const source = '# 中文😀标题\r\n\r\n*斜体* 和 **粗体**、`code`、[链接](https://example.com)。\r\n';
  try {
    const result = await session.highlight('markdown', source);
    assert.ok(!result.error, JSON.stringify(result.error));
    assert.ok(hasCapture(result, '中文😀标题', 'text'), 'plain heading text retains its block capture');
    for (const [text, capture] of [
      ['斜体', 'text.emphasis'],
      ['粗体', 'text.strong'],
      ['code', 'text.literal'],
      ['链接', 'text.reference'],
      ['https://example.com', 'text.uri'],
    ]) {
      assert.ok(result.spans.some(span => span.capture === capture
        && source.slice(span.startIndex, span.endIndex) === text), `${text} uses ${capture}`);
    }
    assert.ok(result.spans.every((span, index) => index === 0 || result.spans[index - 1].endIndex <= span.startIndex));
  } finally {
    session.close();
  }
});

test('Markdown fenced aliases inject Rust, JavaScript, and nested HTML at original offsets', async () => {
  const session = worker();
  const source = '# 中文😀\r\n\r\n```rs\r\nfn main() { let answer = 42; }\r\n```\r\n\r\n```js\r\n42\r\n```\r\n\r\n```htm\r\n<script>const text = "😃";</script>\r\n```\r\n';
  try {
    const result = await session.highlight('markdown', source);
    assert.ok(!result.error, JSON.stringify(result.error));
    assert.ok(hasCapture(result, 'fn', 'keyword'), 'rs resolves to Rust');
    const numericStart = source.indexOf('42', source.indexOf('```js'));
    assert.ok(result.spans.some(span => span.capture === 'number'
      && span.startIndex === numericStart && span.endIndex === numericStart + 2),
    'a JavaScript leaf overrides the outer fenced-content capture');
    assert.ok(hasCapture(result, 'script', 'tag'), 'htm resolves to HTML');
    const keywordStart = source.indexOf('const');
    assert.ok(result.spans.some(span => span.capture === 'keyword'
      && span.startIndex === keywordStart && span.endIndex === keywordStart + 5),
    'Markdown to HTML to JavaScript recursion preserves UTF-16 offsets');
    assert.ok(hasCapture(result, '"😃"', 'string'));
    const edited = source.replace('中文😀', '新增😃中文😀').replace('let answer = 42', 'let answer = 123');
    const incremental = await session.highlight('markdown', edited);
    const freshSession = worker();
    try {
      const fresh = await freshSession.highlight('markdown', edited);
      assert.deepEqual(incremental.spans, fresh.spans);
      assert.ok(hasCapture(incremental, '123', 'constant'), 'Rust numbers use the upstream constant.builtin capture');
    } finally {
      freshSession.close();
    }
  } finally {
    session.close();
  }
});
