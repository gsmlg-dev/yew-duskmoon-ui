import assert from 'node:assert/strict';
import test from 'node:test';
import { contiguousEdit, createWorkerHandler, normalizeCaptures, resolveLanguage } from '../assets/worker.js';

const flush = () => new Promise(resolve => setImmediate(resolve));
const capture = (name, startIndex, endIndex) => ({ name, node: { startIndex, endIndex } });
const request = (revision, source = `source ${revision}`, documentId = 1, language = 'rust') => ({
  id: revision, type: 'highlight', documentId, revision, language, source,
});

function fakeGrammar() {
  const parsers = [];
  const parses = [];
  const trees = [];
  return {
    parsers, parses, trees,
    query: { captures: () => [] },
    createParser() {
      const parser = {
        deleted: false,
        delete() { this.deleted = true; },
        parse(source, previousTree) {
          parses.push({ source, previousTree });
          const tree = {
            rootNode: {}, edits: [], deleted: false,
            edit(edit) { this.edits.push(edit); },
            delete() { this.deleted = true; },
          };
          trees.push(tree);
          return tree;
        },
      };
      parsers.push(parser);
      return parser;
    },
  };
}

test('contiguous edits preserve surrogate pairs at shared prefixes and suffixes', () => {
  for (const [before, after] of [
    ['a😀z', 'a😃z'],
    ['a😀', `a${String.fromCodePoint(0x1e600)}`],
    ['😀你好\r\nline', '😀你好\r\nnew line'],
    ['', '😀'], ['😀', ''], ['hello', 'hello world'],
  ]) {
    const edit = contiguousEdit(before, after);
    assert.equal(before.slice(0, edit.startIndex) + after.slice(edit.startIndex, edit.newEndIndex) + before.slice(edit.oldEndIndex), after);
    for (const [source, index] of [[before, edit.startIndex], [before, edit.oldEndIndex], [after, edit.newEndIndex]]) {
      assert.ok(!(index > 0 && /[\ud800-\udbff]/u.test(source[index - 1]) && /[\udc00-\udfff]/u.test(source[index])));
    }
  }
  assert.deepEqual(contiguousEdit('a😀z', 'a😃z'), {
    startIndex: 1, oldEndIndex: 3, newEndIndex: 3,
    startPosition: { row: 0, column: 1 },
    oldEndPosition: { row: 0, column: 3 },
    newEndPosition: { row: 0, column: 3 },
  });
  assert.equal(contiguousEdit('unchanged', 'unchanged'), null);
});

test('edit points count LF rows and UTF-16 columns with CRLF and Unicode', () => {
  const edit = contiguousEdit('头\r\n😀old\r\nend', '头\r\n😀new\nline\r\nend');
  assert.deepEqual(edit.startPosition, { row: 1, column: 2 });
  assert.deepEqual(edit.oldEndPosition, { row: 1, column: 5 });
  assert.deepEqual(edit.newEndPosition, { row: 2, column: 4 });
});

test('nested captures override broad captures and restore surrounding ranges', () => {
  assert.deepEqual(normalizeCaptures([
    capture('string', 0, 12), capture('function', 3, 9), capture('variable.parameter', 5, 7),
  ], 'abcdefghijkl'), [
    { startIndex: 0, endIndex: 3, capture: 'string' },
    { startIndex: 3, endIndex: 5, capture: 'function' },
    { startIndex: 5, endIndex: 7, capture: 'variable.parameter' },
    { startIndex: 7, endIndex: 9, capture: 'function' },
    { startIndex: 9, endIndex: 12, capture: 'string' },
  ]);
});

test('capture ties prefer specificity, ignore invalid spans, and merge adjacent styles', () => {
  assert.deepEqual(normalizeCaptures([
    capture('function', 0, 2), capture('function.builtin', 0, 2),
    capture('function.builtin', 2, 4), capture('_internal', 0, 6),
    capture('unknown', 0, 6), capture('string', 5, 6),
    capture('string', -1, 3), capture('comment', 0, 100),
  ], 'abcd😀'), [
    { startIndex: 0, endIndex: 4, capture: 'function.builtin' },
  ]);
});

test('overlapping equal-size captures use deterministic query order', () => {
  assert.deepEqual(normalizeCaptures([
    capture('keyword', 0, 4), capture('type', 2, 6),
  ], 'abcdef'), [
    { startIndex: 0, endIndex: 2, capture: 'keyword' },
    { startIndex: 2, endIndex: 6, capture: 'type' },
  ]);
});

test('document updates reuse and edit the old tree, then release it', async () => {
  const grammar = fakeGrammar();
  const replies = [];
  const handle = createWorkerHandler(async () => grammar, reply => replies.push(reply));
  handle(request(1, 'fn 😀() {}'));
  await flush();
  handle(request(2, 'fn 😃() {}'));
  await flush();
  assert.equal(grammar.parsers.length, 1);
  assert.equal(grammar.parses[1].previousTree, grammar.trees[0]);
  assert.equal(grammar.trees[0].edits.length, 1);
  assert.equal(grammar.trees[0].edits[0].startIndex, 3);
  assert.ok(grammar.trees[0].deleted);
  assert.equal(replies[1].revision, 2);
  assert.equal(replies[1].source, 'fn 😃() {}');
  handle(request(3, 'fn 😃() {}'));
  await flush();
  assert.equal(grammar.parses.length, 2, 'unchanged source reuses its tree');
  handle({ type: 'close', documentId: 1 });
  assert.ok(grammar.parsers[0].deleted);
  assert.ok(grammar.trees[1].deleted);
});

test('language changes discard the previous parser and tree', async () => {
  const grammar = fakeGrammar();
  const handle = createWorkerHandler(async () => grammar, () => {});
  handle(request(1));
  await flush();
  handle(request(2, '{}', 1, 'json'));
  await flush();
  assert.equal(grammar.parsers.length, 2);
  assert.ok(grammar.parsers[0].deleted);
  assert.ok(grammar.trees[0].deleted);
  assert.equal(grammar.parses[1].previousTree, null);
});

test('coalesces pending snapshots while grammar loads and rejects superseded revisions', async () => {
  const grammar = fakeGrammar();
  let resolveLoad;
  const loading = new Promise(resolve => { resolveLoad = resolve; });
  const replies = [];
  const handle = createWorkerHandler(() => loading, reply => replies.push(reply));
  handle(request(1));
  handle(request(2));
  handle(request(3));
  assert.deepEqual(replies.map(reply => [reply.revision, reply.error.code]), [[1, 'stale'], [2, 'stale']]);
  resolveLoad(grammar);
  await flush();
  assert.deepEqual(grammar.parses.map(parse => parse.source), ['source 3']);
  assert.equal(replies.find(reply => reply.revision === 1).error.code, 'stale');
  assert.equal(replies.find(reply => reply.revision === 3).source, 'source 3');
  handle(request(2));
  assert.equal(replies.at(-1).error.code, 'stale');
});

test('a ready new language bypasses an unresolved obsolete grammar load', async () => {
  const grammar = fakeGrammar();
  let resolveOldLoad;
  const oldLoading = new Promise(resolve => { resolveOldLoad = resolve; });
  const loads = [];
  const replies = [];
  const handle = createWorkerHandler(language => {
    loads.push(language);
    return language === 'rust' ? oldLoading : Promise.resolve(grammar);
  }, reply => replies.push(reply));
  handle(request(1));
  handle(request(2));
  handle(request(3, '{}', 1, 'json'));
  assert.deepEqual(replies.map(reply => [reply.revision, reply.error.code]), [[1, 'stale'], [2, 'stale']]);
  assert.deepEqual(loads, ['rust', 'json']);
  await flush();
  assert.equal(replies.find(reply => reply.revision === 3).source, '{}');
  handle(request(2));
  assert.equal(replies.at(-1).error.code, 'stale', 'replacement must retain revision ordering');
  resolveOldLoad(grammar);
  await flush();
  assert.equal(grammar.parsers.length, 1, 'obsolete load cannot resurrect the detached generation');
  assert.deepEqual(grammar.parses.map(parse => parse.source), ['{}']);
  handle({ type: 'close', documentId: 1 });
  assert.ok(grammar.parsers[0].deleted, 'close applies to the replacement generation');
});

test('close settles the newest generation while both language loads remain pending', async () => {
  const grammar = fakeGrammar();
  const loaders = {};
  const replies = [];
  const handle = createWorkerHandler(language => new Promise(resolve => { loaders[language] = resolve; }), reply => replies.push(reply));
  handle(request(1));
  handle(request(2, '{}', 1, 'json'));
  handle(request(3, '{"x": 1}', 1, 'json'));
  handle({ type: 'close', documentId: 1 });
  assert.deepEqual(replies.map(reply => [reply.revision, reply.error.code]), [[1, 'stale'], [2, 'stale'], [3, 'closed']]);
  loaders.json(grammar);
  loaders.rust(grammar);
  await flush();
  assert.equal(grammar.parsers.length, 0);
  assert.equal(replies.length, 3, 'detached load completions cannot reply twice');
});

test('close immediately rejects pending loads and their completion cannot resurrect the document', async () => {
  const grammar = fakeGrammar();
  let resolveLoad;
  const loading = new Promise(resolve => { resolveLoad = resolve; });
  const replies = [];
  const handle = createWorkerHandler(() => loading, reply => replies.push(reply));
  handle(request(1));
  handle(request(2));
  handle({ type: 'close', documentId: 1 });
  assert.deepEqual(replies.map(reply => reply.error.code), ['stale', 'closed']);
  resolveLoad(grammar);
  await flush();
  assert.equal(grammar.parsers.length, 0);
  assert.equal(replies.length, 2, 'in-flight completion must not reply twice');
});

test('documents retain independent parsers and revisions', async () => {
  const grammar = fakeGrammar();
  const replies = [];
  const handle = createWorkerHandler(async () => grammar, reply => replies.push(reply));
  handle(request(1, 'one', 1));
  handle(request(1, 'two', 2));
  await flush();
  assert.equal(grammar.parsers.length, 2);
  handle({ type: 'close', documentId: 1 });
  handle(request(2, 'three', 2));
  await flush();
  assert.equal(replies.at(-1).source, 'three');
  assert.ok(!grammar.parsers[1].deleted);
});

test('grammar failures report structured errors and later revisions can recover', async () => {
  const grammar = fakeGrammar();
  const replies = [];
  const handle = createWorkerHandler(async language => {
    if (language === 'missing') throw Object.assign(new Error('No grammar'), { code: 'unsupported_language' });
    return grammar;
  }, reply => replies.push(reply));
  handle(request(1, 'code', 1, 'missing'));
  await flush();
  assert.deepEqual(replies[0].error, { code: 'unsupported_language', message: 'No grammar' });
  handle(request(2, 'code'));
  await flush();
  assert.equal(replies[1].source, 'code');
});

test('manifest fence aliases and explicit grammar aliases resolve without cycles', () => {
  const manifest = {
    aliases: { rs: 'rust', sh: 'shell' },
    languages: { rust: {}, shell: { alias: 'bash' }, bash: {}, one: { alias: 'two' }, two: { alias: 'one' } },
  };
  assert.equal(resolveLanguage(manifest, ' RS '), 'rust');
  assert.equal(resolveLanguage(manifest, 'sh'), 'bash');
  assert.throws(() => resolveLanguage(manifest, 'zsh'), { code: 'unsupported_language' });
  assert.throws(() => resolveLanguage(manifest, 'one'), { code: 'unsupported_language' });
});

function injectableGrammar(id, captures = () => [], matches = () => []) {
  const grammar = fakeGrammar();
  const originalCreateParser = grammar.createParser;
  grammar.id = id;
  grammar.query.captures = captures;
  grammar.injections = { matches };
  grammar.createParser = () => {
    const parser = originalCreateParser();
    const originalParse = parser.parse;
    parser.parse = (source, previous, options) => {
      const tree = originalParse.call(parser, source, previous);
      tree.rootNode = { source, options };
      grammar.parses.at(-1).options = options;
      return tree;
    };
    return parser;
  };
  return grammar;
}

test('fixed-language injections preserve original UTF-16 offsets and CRLF positions', async () => {
  const source = '😀\r\nconst x = 1;\nTAIL';
  const startIndex = source.indexOf('const');
  const endIndex = source.indexOf('TAIL');
  const outer = injectableGrammar('outer', root => [capture('text.literal', 0, root.source.length)], () => [{
    patternIndex: 0,
    setProperties: { 'injection.language': 'javascript' },
    captures: [capture('injection.content', startIndex, endIndex)],
  }]);
  const inner = injectableGrammar('javascript', () => [capture('keyword', startIndex, startIndex + 5)]);
  const replies = [];
  const handle = createWorkerHandler(async language => language === 'outer' ? outer : inner, reply => replies.push(reply));
  handle(request(1, source, 1, 'outer'));
  await flush();
  assert.ok(replies[0].spans.some(span => span.capture === 'keyword' && source.slice(span.startIndex, span.endIndex) === 'const'));
  assert.deepEqual(inner.parses[0].options.includedRanges, [{
    startIndex, endIndex, startPosition: { row: 1, column: 0 }, endPosition: { row: 2, column: 0 },
  }]);
  assert.ok(inner.parsers[0].deleted && inner.trees[0].deleted);
  assert.ok(!outer.parsers[0].deleted && !outer.trees[0].deleted);
  handle({ type: 'close', documentId: 1 });
});

test('captured languages and combined ranges exclude gaps from injected captures', async () => {
  const source = 'js\nconst ONE {{ excluded }} const TWO';
  const first = { startIndex: source.indexOf('const'), endIndex: source.indexOf('{{') };
  const second = { startIndex: source.lastIndexOf('const'), endIndex: source.length };
  const outer = injectableGrammar('outer', () => [], () => [first, second].map(range => ({
    patternIndex: 3,
    setProperties: { 'injection.combined': null },
    captures: [capture('injection.language', 0, 2), capture('injection.content', range.startIndex, range.endIndex)],
  })));
  const inner = injectableGrammar('javascript', () => [capture('variable', 0, source.length)]);
  const loads = [];
  const replies = [];
  const handle = createWorkerHandler(async language => {
    loads.push(language);
    return language === 'outer' ? outer : inner;
  }, reply => replies.push(reply));
  handle(request(1, source, 1, 'outer'));
  await flush();
  assert.deepEqual(loads, ['outer', 'js']);
  assert.equal(inner.parsers.length, 1);
  assert.equal(inner.parses[0].options.includedRanges.length, 2);
  assert.deepEqual(replies[0].spans.map(span => [span.startIndex, span.endIndex]), [[first.startIndex, first.endIndex], [second.startIndex, second.endIndex]]);
  handle({ type: 'close', documentId: 1 });
});

test('injection content excludes child nodes unless include-children is set', async () => {
  for (const include of [false, true]) {
    const inner = injectableGrammar('inner');
    const outer = injectableGrammar('outer', () => [], () => [{
      setProperties: { 'injection.language': 'inner', ...(include ? { 'injection.include-children': null } : {}) },
      captures: [{ name: 'injection.content', node: { startIndex: 0, endIndex: 10, children: [{ startIndex: 3, endIndex: 7 }] } }],
    }]);
    const handle = createWorkerHandler(async language => language === 'outer' ? outer : inner, () => {});
    handle(request(1, '0123456789', 1, 'outer'));
    await flush();
    assert.deepEqual(inner.parses[0].options.includedRanges.map(({ startIndex, endIndex }) => [startIndex, endIndex]), include ? [[0, 10]] : [[0, 3], [7, 10]]);
    handle({ type: 'close', documentId: 1 });
  }
});

test('unavailable injected languages leave primary captures usable', async () => {
  const outer = injectableGrammar('outer', () => [capture('text.literal', 0, 4)], () => [{
    setProperties: { 'injection.language': 'missing' }, captures: [capture('injection.content', 0, 4)],
  }]);
  const replies = [];
  const handle = createWorkerHandler(async language => {
    if (language === 'outer') return outer;
    throw Object.assign(new Error('No optional grammar'), { code: 'unsupported_language' });
  }, reply => replies.push(reply));
  handle(request(1, 'code', 1, 'outer'));
  await flush();
  assert.deepEqual(replies[0].spans, [{ startIndex: 0, endIndex: 4, capture: 'text.literal' }]);
  handle({ type: 'close', documentId: 1 });
});

test('successful injected captures override equally ranged broad parent text', async () => {
  const outer = injectableGrammar('outer', () => [capture('text.literal', 0, 3)], () => [{
    setProperties: { 'injection.language': 'inner' }, captures: [capture('injection.content', 0, 3)],
  }]);
  const inner = injectableGrammar('inner', () => [capture('number', 0, 3)]);
  const replies = [];
  const handle = createWorkerHandler(async language => language === 'outer' ? outer : inner, reply => replies.push(reply));
  handle(request(1, '123', 1, 'outer'));
  await flush();
  assert.deepEqual(replies[0].spans, [{ startIndex: 0, endIndex: 3, capture: 'number' }]);
  handle({ type: 'close', documentId: 1 });
});

test('recursive injection cycles stop before allocating another parser', async () => {
  const outer = injectableGrammar('outer', () => [capture('text.literal', 0, 4)], () => [{
    setProperties: { 'injection.language': 'alias-of-outer' }, captures: [capture('injection.content', 0, 4)],
  }]);
  const replies = [];
  const handle = createWorkerHandler(async () => outer, reply => replies.push(reply));
  handle(request(1, 'code', 1, 'outer'));
  await flush();
  assert.equal(outer.parsers.length, 1);
  assert.equal(replies.length, 1);
  handle({ type: 'close', documentId: 1 });
});

test('injection recursion has a fixed depth limit and frees all temporary parsers', async () => {
  const grammars = [];
  const replies = [];
  const handle = createWorkerHandler(async language => {
    const depth = Number(language.slice(4));
    const grammar = injectableGrammar(language, () => [], () => [{
      setProperties: { 'injection.language': `lang${depth + 1}` },
      captures: [capture('injection.content', 0, 4)],
    }]);
    grammars.push(grammar);
    return grammar;
  }, reply => replies.push(reply));
  handle(request(1, 'code', 1, 'lang0'));
  await flush();
  assert.equal(grammars.length, 7);
  assert.equal(replies.length, 1);
  assert.ok(grammars.slice(1).every(grammar => grammar.parsers[0].deleted && grammar.trees[0].deleted));
  handle({ type: 'close', documentId: 1 });
  assert.ok(grammars[0].parsers[0].deleted && grammars[0].trees[0].deleted);
});

test('close during an injection load rejects immediately and releases primary resources', async () => {
  let resolveInjection;
  const loading = new Promise(resolve => { resolveInjection = resolve; });
  const inner = injectableGrammar('inner');
  const outer = injectableGrammar('outer', () => [], () => [{
    setProperties: { 'injection.language': 'inner' }, captures: [capture('injection.content', 0, 4)],
  }]);
  const replies = [];
  const handle = createWorkerHandler(async language => language === 'outer' ? outer : loading, reply => replies.push(reply));
  handle(request(1, 'code', 1, 'outer'));
  await flush();
  handle({ type: 'close', documentId: 1 });
  assert.equal(replies[0].error.code, 'closed');
  assert.ok(outer.parsers[0].deleted && outer.trees[0].deleted);
  resolveInjection(inner);
  await flush();
  assert.equal(inner.parsers.length, 0);
  assert.equal(replies.length, 1);
});

test('same-language revisions bypass stale injection loads and retain the incremental primary tree', async () => {
  let resolveInjection;
  const loading = new Promise(resolve => { resolveInjection = resolve; });
  const inner = injectableGrammar('inner');
  const outer = injectableGrammar('outer', () => [], root => root.source === 'old' ? [{
    setProperties: { 'injection.language': 'inner' }, captures: [capture('injection.content', 0, 3)],
  }] : []);
  const replies = [];
  const handle = createWorkerHandler(async language => language === 'outer' ? outer : loading, reply => replies.push(reply));
  handle(request(1, 'old', 1, 'outer'));
  await flush();
  handle(request(2, 'new', 1, 'outer'));
  assert.equal(replies[0].error.code, 'stale');
  await flush();
  assert.equal(replies[1].source, 'new');
  assert.equal(outer.parsers.length, 1);
  assert.equal(outer.parses[1].previousTree, outer.trees[0]);
  resolveInjection(inner);
  await flush();
  assert.equal(inner.parsers.length, 0);
  assert.equal(replies.length, 2);
  handle({ type: 'close', documentId: 1 });
  assert.ok(outer.parsers[0].deleted);
});
