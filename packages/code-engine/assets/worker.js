// Tree-sitter runs only in this module Worker. Offsets and columns use UTF-16,
// matching web-tree-sitter; the Rust client converts validated spans to bytes.
function isBoundary(source, index) {
  if (index <= 0 || index >= source.length) return true;
  const previous = source.charCodeAt(index - 1);
  const current = source.charCodeAt(index);
  return !(previous >= 0xd800 && previous <= 0xdbff && current >= 0xdc00 && current <= 0xdfff);
}

function pointAt(source, index) {
  let row = 0;
  let lineStart = 0;
  for (let offset = 0; offset < index; offset++) {
    if (source.charCodeAt(offset) === 10) {
      row++;
      lineStart = offset + 1;
    }
  }
  return { row, column: index - lineStart };
}

export function contiguousEdit(previous, source) {
  if (previous === source) return null;
  let startIndex = 0;
  const sharedLength = Math.min(previous.length, source.length);
  while (startIndex < sharedLength && previous[startIndex] === source[startIndex]) startIndex++;
  if (!isBoundary(previous, startIndex) || !isBoundary(source, startIndex)) startIndex--;
  let oldEndIndex = previous.length;
  let newEndIndex = source.length;
  while (oldEndIndex > startIndex && newEndIndex > startIndex && previous[oldEndIndex - 1] === source[newEndIndex - 1]) {
    oldEndIndex--;
    newEndIndex--;
  }
  if (!isBoundary(previous, oldEndIndex) || !isBoundary(source, newEndIndex)) {
    oldEndIndex++;
    newEndIndex++;
  }
  return {
    startIndex, oldEndIndex, newEndIndex,
    startPosition: pointAt(previous, startIndex),
    oldEndPosition: pointAt(previous, oldEndIndex),
    newEndPosition: pointAt(source, newEndIndex),
  };
}

const captureRoots = new Set([
  'keyword', 'type', 'constructor', 'string', 'character', 'number', 'float',
  'boolean', 'constant', 'comment', 'function', 'method', 'operator',
  'punctuation', 'property', 'variable', 'attribute', 'tag', 'label',
  'namespace', 'module', 'embedded', 'markup', 'text', 'delimiter', 'include',
  'preproc', 'storageclass', 'conditional', 'repeat', 'exception', 'parameter',
  'field', 'escape', 'none',
]);

function priority(left, right) {
  return right.depth - left.depth || left.length - right.length || right.specificity - left.specificity || right.order - left.order;
}

function pushHeap(heap, item) {
  heap.push(item);
  let index = heap.length - 1;
  while (index > 0) {
    const parent = (index - 1) >> 1;
    if (priority(heap[parent], item) <= 0) break;
    heap[index] = heap[parent];
    index = parent;
  }
  heap[index] = item;
}

function popHeap(heap) {
  const last = heap.pop();
  if (!heap.length) return;
  let index = 0;
  while (index * 2 + 1 < heap.length) {
    let child = index * 2 + 1;
    if (child + 1 < heap.length && priority(heap[child + 1], heap[child]) < 0) child++;
    if (priority(last, heap[child]) <= 0) break;
    heap[index] = heap[child];
    index = child;
  }
  heap[index] = last;
}

export function normalizeCaptures(captures, source) {
  const events = [];
  captures.forEach(({ name, node, injectionDepth = 0 }, order) => {
    const start = node.startIndex;
    const end = node.endIndex;
    if (!captureRoots.has(name.split('.')[0]) || !Number.isInteger(start) || !Number.isInteger(end)
      || start < 0 || end <= start || end > source.length || !isBoundary(source, start) || !isBoundary(source, end)) return;
    const item = { order, name, depth: injectionDepth, length: end - start, specificity: name.split('.').length };
    events.push({ offset: start, item, add: true }, { offset: end, item, add: false });
  });
  events.sort((left, right) => left.offset - right.offset);
  const active = new Set();
  const heap = [];
  const spans = [];
  let index = 0;
  while (index < events.length) {
    const startIndex = events[index].offset;
    while (index < events.length && events[index].offset === startIndex) {
      const event = events[index++];
      if (event.add) {
        active.add(event.item.order);
        pushHeap(heap, event.item);
      } else active.delete(event.item.order);
    }
    while (heap.length && !active.has(heap[0].order)) popHeap(heap);
    if (!heap.length || index === events.length) continue;
    const endIndex = events[index].offset;
    const capture = heap[0].name;
    const previous = spans.at(-1);
    if (previous?.endIndex === startIndex && previous.capture === capture) previous.endIndex = endIndex;
    else spans.push({ startIndex, endIndex, capture });
  }
  return spans;
}

function failure(code, message) {
  return Object.assign(new Error(message), { code });
}

function mergeRanges(ranges) {
  const result = [];
  for (const range of ranges.sort((left, right) => left.startIndex - right.startIndex)) {
    const previous = result.at(-1);
    if (previous && range.startIndex <= previous.endIndex) previous.endIndex = Math.max(previous.endIndex, range.endIndex);
    else result.push({ startIndex: range.startIndex, endIndex: range.endIndex });
  }
  return result;
}

function intersectRanges(ranges, allowed) {
  const result = [];
  for (const range of ranges) {
    let low = 0;
    let high = allowed.length;
    while (low < high) {
      const middle = (low + high) >> 1;
      if (allowed[middle].endIndex <= range.startIndex) low = middle + 1;
      else high = middle;
    }
    for (let index = low; index < allowed.length && allowed[index].startIndex < range.endIndex; index++) {
      const limit = allowed[index];
      const startIndex = Math.max(range.startIndex, limit.startIndex);
      const endIndex = Math.min(range.endIndex, limit.endIndex);
      if (endIndex > startIndex) result.push({ startIndex, endIndex });
    }
  }
  return mergeRanges(result);
}

function contentRanges(node, includeChildren) {
  if (includeChildren) return [{ startIndex: node.startIndex, endIndex: node.endIndex }];
  const ranges = [];
  let startIndex = node.startIndex;
  for (const child of node.children || []) {
    if (child.startIndex > startIndex) ranges.push({ startIndex, endIndex: child.startIndex });
    startIndex = child.endIndex;
  }
  if (startIndex < node.endIndex) ranges.push({ startIndex, endIndex: node.endIndex });
  return ranges;
}

// Copy every node-dependent value synchronously: a newer generation may delete
// the tree while an injected language is loading.
function snapshotTree(grammar, tree, source, allowed) {
  const captures = [];
  for (const { name, node } of grammar.query.captures(tree.rootNode)) {
    for (const range of intersectRanges([{ startIndex: node.startIndex, endIndex: node.endIndex }], allowed)) {
      captures.push({ name, node: range });
    }
  }
  const jobs = [];
  const combined = new Map();
  let matches;
  try {
    matches = grammar.injections?.matches(tree.rootNode) || [];
  } catch {
    return { captures, jobs };
  }
  for (const match of matches) {
    const properties = match.setProperties || {};
    const languageCapture = match.captures.find(capture => capture.name === 'injection.language');
    const language = properties['injection.language']
      || (languageCapture && source.slice(languageCapture.node.startIndex, languageCapture.node.endIndex));
    if (typeof language !== 'string' || !language.trim()) continue;
    const ranges = intersectRanges(match.captures
      .filter(capture => capture.name === 'injection.content')
      .flatMap(capture => contentRanges(capture.node, Object.hasOwn(properties, 'injection.include-children'))), allowed)
      .filter(range => isBoundary(source, range.startIndex) && isBoundary(source, range.endIndex));
    if (!ranges.length) continue;
    if (Object.hasOwn(properties, 'injection.combined')) {
      const key = `${match.patternIndex}:${language}`;
      let job = combined.get(key);
      if (!job) {
        job = { language, ranges: [] };
        combined.set(key, job);
        jobs.push(job);
      }
      job.ranges.push(...ranges);
    } else jobs.push({ language, ranges });
  }
  for (const job of jobs) job.ranges = mergeRanges(job.ranges);
  return { captures, jobs };
}

function sourcePositions(source) {
  const lines = [0];
  for (let index = 0; index < source.length; index++) {
    if (source.charCodeAt(index) === 10) lines.push(index + 1);
  }
  return index => {
    let low = 0;
    let high = lines.length;
    while (low + 1 < high) {
      const middle = (low + high) >> 1;
      if (lines[middle] <= index) low = middle;
      else high = middle;
    }
    return { row: low, column: index - lines[low] };
  };
}

async function injectedCaptures(snapshot, context, ancestors, depth = 0) {
  const captures = snapshot.captures.map(capture => ({ ...capture, injectionDepth: depth }));
  if (depth >= 6) return captures;
  for (const job of snapshot.jobs) {
    try {
      context.assertCurrent();
      const grammar = await context.loadLanguage(job.language);
      context.assertCurrent();
      const identity = `${grammar.id || job.language}:${job.ranges.map(range => `${range.startIndex}-${range.endIndex}`).join(',')}`;
      if (ancestors.has(identity)) continue;
      const parser = grammar.createParser();
      let tree;
      let nested;
      try {
        tree = parser.parse(context.source, null, {
          includedRanges: job.ranges.map(range => ({
            ...range,
            startPosition: context.pointAt(range.startIndex),
            endPosition: context.pointAt(range.endIndex),
          })),
        });
        if (!tree) continue;
        nested = snapshotTree(grammar, tree, context.source, job.ranges);
      } finally {
        tree?.delete();
        parser.delete();
      }
      for (const capture of await injectedCaptures(nested, context, new Set([...ancestors, identity]), depth + 1)) captures.push(capture);
      context.assertCurrent();
    } catch (error) {
      context.assertCurrent();
      // An optional grammar, injection query, or parser can be unavailable;
      // primary-language captures remain useful in that case.
    }
  }
  return captures;
}

// Dependency injection keeps lifecycle and race tests independent of WASM assets.
export function createWorkerHandler(loadLanguage, postMessage) {
  const documents = new Map();

  function reject(request, code, message) {
    if (request.responded) return;
    request.responded = true;
    postMessage({ id: request.id, documentId: request.documentId, revision: request.revision, error: { code, message } });
  }

  function release(document) {
    document.tree?.delete();
    document.parser?.delete();
    document.tree = null;
    document.parser = null;
    document.language = null;
    document.source = '';
  }

  function newDocument(revision = -1) {
    return { revision, source: '', language: null, parser: null, tree: null, pending: null, current: null, running: false, closed: false, phase: 'loading' };
  }

  async function drain(document) {
    document.running = true;
    while (document.pending && !document.closed) {
      const request = document.pending;
      document.pending = null;
      document.current = request;
      document.phase = 'loading';
      try {
        const grammar = await loadLanguage(request.language);
        if (document.closed) {
          reject(request, 'closed', 'The syntax document is closed.');
          break;
        }
        if (request.revision !== document.revision) {
          reject(request, 'stale', 'A newer document revision superseded this request.');
          continue;
        }
        if (document.language !== request.language) {
          release(document);
          document.parser = grammar.createParser();
          document.language = request.language;
        }
        if (!document.tree || document.source !== request.source) {
          const previousTree = document.tree;
          const edit = previousTree && contiguousEdit(document.source, request.source);
          if (edit) previousTree.edit(edit);
          document.tree = null;
          try {
            document.tree = document.parser.parse(request.source, previousTree);
          } finally {
            previousTree?.delete();
          }
          if (!document.tree) throw failure('parse', 'Tree-sitter did not return a syntax tree.');
          document.source = request.source;
        }
        const assertCurrent = () => {
          if (document.closed) throw failure('closed', 'The syntax document is closed.');
          if (request.revision !== document.revision) throw failure('stale', 'A newer document revision superseded this request.');
        };
        const snapshot = snapshotTree(grammar, document.tree, request.source, [{ startIndex: 0, endIndex: request.source.length }]);
        document.phase = 'injections';
        const captures = await injectedCaptures(snapshot, {
          source: request.source, loadLanguage, assertCurrent, pointAt: sourcePositions(request.source),
        }, new Set([`${grammar.id || request.language}:0-${request.source.length}`]));
        assertCurrent();
        const spans = normalizeCaptures(captures, request.source);
        request.responded = true;
        postMessage({ id: request.id, documentId: request.documentId, revision: request.revision, language: request.language, source: request.source, spans });
      } catch (error) {
        reject(request, document.closed ? 'closed' : request.revision !== document.revision ? 'stale' : error.code || 'parse', String(error.message || error));
      } finally {
        document.current = null;
      }
    }
    document.running = false;
  }

  return function handle(request) {
    if (request.type === 'close') {
      const document = documents.get(request.documentId);
      if (document) {
        // The client prevents updates after close, and its Worker messages are
        // ordered. This closure tombstone invalidates pending loads without
        // retaining every historical document id for the Worker's lifetime.
        document.closed = true;
        if (document.pending) reject(document.pending, 'closed', 'The syntax document is closed.');
        if (document.current) reject(document.current, 'closed', 'The syntax document is closed.');
        document.pending = null;
        release(document);
        documents.delete(request.documentId);
      }
      return;
    }
    if (request.type !== 'highlight') return;
    if (!Number.isSafeInteger(request.id) || !Number.isSafeInteger(request.documentId) || !Number.isSafeInteger(request.revision)
      || typeof request.language !== 'string' || typeof request.source !== 'string') {
      reject(request, 'parse', 'Invalid syntax request.');
      return;
    }
    let document = documents.get(request.documentId);
    if (!document) {
      document = newDocument();
      documents.set(request.documentId, document);
    }
    if (request.revision <= document.revision) {
      reject(request, 'stale', 'Document revisions must increase.');
      return;
    }
    if (document.current) reject(document.current, 'stale', 'A newer document revision superseded this request.');
    if (document.pending) reject(document.pending, 'stale', 'A newer document revision superseded this request.');
    if (document.current && (document.current.language !== request.language || document.phase === 'injections')) {
      // A stalled obsolete grammar must not block another language. Detach its
      // generation so completion cannot mutate or recreate the live session.
      document.closed = true;
      document.pending = null;
      const previous = document;
      document = newDocument(previous.revision);
      if (previous.current.language === request.language) {
        // Injection jobs copied their nodes already; retain the primary tree for
        // the newest same-language snapshot instead of waiting on optional loads.
        for (const key of ['tree', 'parser', 'source', 'language']) document[key] = previous[key];
        previous.tree = null;
        previous.parser = null;
        previous.source = '';
      } else release(previous);
      documents.set(request.documentId, document);
    }
    document.revision = request.revision;
    document.pending = request;
    if (!document.running) void drain(document);
  };
}

export function resolveLanguage(manifest, requested) {
  let language = requested.trim().toLowerCase();
  const seen = new Set();
  while (!seen.has(language)) {
    seen.add(language);
    const alias = manifest.aliases?.[language] || manifest.languages[language]?.alias;
    if (!alias) {
      if (manifest.languages[language]) return language;
      break;
    }
    language = alias;
  }
  throw failure('unsupported_language', `No Tree-sitter grammar is configured for ${requested}.`);
}

function createLanguageLoader() {
  let runtimePromise;
  const languages = new Map();
  async function checkedFetch(url) {
    const response = await fetch(url);
    if (!response.ok) throw failure('load', `Cannot load ${url}: HTTP ${response.status}`);
    return response;
  }
  async function runtime() {
    const manifest = await (await checkedFetch(new URL('manifest.json', import.meta.url))).json();
    if (manifest.protocol !== 1) throw failure('load', 'Unsupported syntax asset protocol.');
    const { Parser, Language, Query } = await import(new URL(manifest.runtime.module, import.meta.url).href);
    await Parser.init({ locateFile: () => new URL(manifest.runtime.wasm, import.meta.url).href });
    return { manifest, Parser, Language, Query };
  }
  return async function loadLanguage(requested) {
    if (!runtimePromise) runtimePromise = runtime().catch(error => {
      runtimePromise = null;
      throw failure('load', String(error.message || error));
    });
    const { manifest, Parser, Language, Query } = await runtimePromise;
    const language = resolveLanguage(manifest, requested);
    const entry = manifest.languages[language];
    if (!languages.has(language)) {
      const loading = (async () => {
        const [grammar, querySource] = await Promise.all([
          Language.load(new URL(entry.wasm, import.meta.url).href),
          checkedFetch(new URL(entry.query, import.meta.url)).then(response => response.text()),
        ]);
        const query = new Query(grammar, querySource);
        let injections = null;
        if (entry.injections) {
          try {
            injections = new Query(grammar, await (await checkedFetch(new URL(entry.injections, import.meta.url))).text());
          } catch {
            // Optional injection assets must not disable primary highlighting.
          }
        }
        return {
          id: language,
          query,
          injections,
          createParser() {
            const parser = new Parser();
            parser.setLanguage(grammar);
            return parser;
          },
        };
      })().catch(error => {
        languages.delete(language);
        throw failure('load', String(error.message || error));
      });
      languages.set(language, loading);
    }
    return languages.get(language);
  };
}

if (typeof WorkerGlobalScope !== 'undefined' && globalThis instanceof WorkerGlobalScope) {
  const handle = createWorkerHandler(createLanguageLoader(), message => globalThis.postMessage(message));
  globalThis.onmessage = event => handle(event.data);
}
