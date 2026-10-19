import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import test, { after } from 'node:test';
import { fileURLToPath } from 'node:url';
import { Language, Parser, Query } from '../assets/web-tree-sitter.js';
import { createWorkerHandler, normalizeCaptures, resolveLanguage } from '../assets/worker.js';

const assets = new URL('../assets/', import.meta.url);
const manifest = JSON.parse(await readFile(new URL('manifest.json', assets), 'utf8'));
await Parser.init({ locateFile: () => fileURLToPath(new URL(manifest.runtime.wasm, assets)) });
const grammars = new Map();

async function loadLanguage(requested) {
  const name = resolveLanguage(manifest, requested);
  if (!grammars.has(name)) {
    const entry = manifest.languages[name];
    const language = await Language.load(fileURLToPath(new URL(entry.wasm, assets)));
    const query = new Query(language, await readFile(new URL(entry.query, assets), 'utf8'));
    grammars.set(name, {
      name,
      language,
      query,
      createParser() {
        const parser = new Parser();
        parser.setLanguage(language);
        return parser;
      },
    });
  }
  return grammars.get(name);
}

after(() => {
  for (const grammar of grammars.values()) grammar.query.delete();
});

const samples = {
  rust: ['fn greet() { let message = r#"hello 😀"#; /* block\ncomment */ }', ['keyword', 'string', 'comment']],
  javascript: ['function greet() { return "hello 😀"; } // comment', ['keyword', 'string', 'comment']],
  typescript: ['interface User { name: string }\nconst value: User = { name: "😀" }; // comment', ['keyword', 'type', 'string', 'comment']],
  tsx: ['const view = <div title="😀">hello</div>; // comment', ['keyword', 'string', 'comment']],
  json: ['{"message": "😀", "number": 123, "enabled": true}', ['string', 'number']],
  elixir: ['defmodule Demo do\n  def greet, do: "hello 😀"\n  # comment\nend', ['keyword', 'string', 'comment']],
  go: ['package main\n// comment\nfunc main() { value := "😀"; _ = value }\n', ['keyword', 'string', 'comment']],
  zig: ['const message = "😀"; // comment\npub fn main() void { _ = message; }\n', ['keyword', 'string', 'comment']],
  yaml: ['message: "😀"\nnumber: 123\n# comment\nenabled: true\n', ['string', 'number', 'comment']],
  "c": ["int main(void) { const char *message = \"😀\"; return 0; } // comment", ["keyword", "string", "comment"]],
  "cpp": ["class Demo { public: void greet() { auto message = \"😀\"; } }; // comment", ["keyword", "string", "comment", "property"]],
  "objective-c": ["#import <Foundation/Foundation.h>\n@interface Demo : NSObject\n- (void)greet;\n@end\n@implementation Demo\n- (void)greet { NSLog(@\"😀\"); }\n@end\n", ["keyword", "string", "property"]],
  "csharp": ["class Demo { static void Main() { string s = \"😀\"; } } // comment", ["keyword", "string", "comment"]],
  "java": ["class Demo { public static void main(String[] args) { String message = \"😀\"; } } // comment", ["keyword", "string", "comment"]],
  "python": ["def greet():\n    message = \"😀\"\n    return message # comment\n", ["keyword", "string", "comment"]],
  "ruby": ["def greet\n  message = \"😀\"\n  puts message # comment\nend\n", ["keyword", "string", "comment"]],
  "php": ["<?php function greet() { echo \"😀\"; } // comment\n", ["keyword", "string", "comment", "tag"]],
  "html": ["<div title=\"😀\"><script>const x = 1;</script></div><!-- comment -->", ["string", "comment", "tag"]],
  "css": ["/* comment */ .demo { color: red; content: \"😀\"; }", ["string", "comment", "property"]],
  "toml": ["# comment\nmessage = \"😀\"\ncount = 1\n", ["string", "comment", "property"]],
  "kotlin": ["fun main() { val message = \"😀\"; println(message) } // comment", ["keyword", "string", "comment"]],
  "swift": ["func greet() { let message = \"😀\"; print(message) } // comment", ["keyword", "string", "comment"]],
  "markdown": ["# Heading\n\n中文😀 **bold** word and `code` with [link](https://example.com).\n\n```rust\nfn main() {}\n```\n", ["text"]],
  "markdown_inline": ["中文😀 **bold** word and `code` with [link](https://example.com).", ["text"]],
  "sass": ["// comment\n$accent: #fff\n.button\n  color: $accent\n  content: \"中文😀\"\n  &:hover\n    color: red\n", ["string", "comment"]],
  "scss": ["/* comment */\n$accent: #fff;\n.button { color: $accent; content: \"中文😀\"; &:hover { color: red; } }\n", ["string", "comment"]],
  "less": ["// comment\n@accent: #fff;\n.rounded(@radius: 4px) { border-radius: @radius; }\n.button { color: @accent; content: \"中文😀\"; .rounded(); }\n", ["string", "comment"]],
  "erlang": ["% comment\n-module(demo).\n-export([hello/0]).\nhello() -> \"hello 😀\".\n", ["string", "comment"]],
  "lisp": ["(defun greet (name) (format nil \"Hello ~A 😀\" name)) ; comment\n", ["string", "comment"]],
  "emacs-lisp": ["(defun greet (name) (message \"Hello %s 😀\" name)) ; comment\n", ["string", "comment"]],
  "bash": ["#!/bin/bash\n# 中文😀\nvalues=(one two)\nfor name in \"${values[@]}\"; do echo \"$name\"; done\n", ["string", "comment"]],
  "zsh": ["# comment\ntypeset -a values=(one two)\nprint -l -- ${(U)values}\nsetopt extendedglob\nfiles=(**/*.rs(.N))\nprint -r -- \"中文😀\"\n", ["string", "comment"]],
  "nix": ["# comment\n{ pkgs ? import <nixpkgs> {} }: let message = \"hello 😀\"; in pkgs.writeText \"demo\" message\n", ["string", "comment"]],
  "assembly": [".globl main # 中文😀\nmain:\n  mov $1, %eax\n  ret\n", ["comment"]],
  "wasm": [";; 中文😀\n(module (func $add (param i32 i32) (result i32) local.get 0 local.get 1 i32.add))\n", ["keyword", "comment"]],
  "shell": ["#!/bin/bash\n# 中文😀\nvalues=(one two)\nfor name in \"${values[@]}\"; do echo \"$name\"; done\n", ["string", "comment"]],
};

for (const [name, [source, expected]] of Object.entries(samples)) {
  test(`${name}: official grammar and bundled query produce representative captures`, async () => {
    const grammar = await loadLanguage(name);
    const parser = grammar.createParser();
    const tree = parser.parse(source);
    try {
      assert.ok(!tree.rootNode.hasError, tree.rootNode.toString());
      const captures = grammar.query.captures(tree.rootNode);
      const entry = manifest.languages[resolveLanguage(manifest, name)];
      if (entry.injections) {
        const injections = new Query(grammar.language, await readFile(new URL(entry.injections, assets), 'utf8'));
        injections.delete();
      }
      for (const category of expected) {
        assert.ok(captures.some(capture => capture.name.split('.')[0] === category), `${name} should capture ${category}`);
      }
      const spans = normalizeCaptures(captures, source);
      assert.ok(spans.length > 0);
      assert.ok(spans.every((span, index) => index === 0 || spans[index - 1].endIndex <= span.startIndex));
    } finally {
      tree.delete();
      parser.delete();
    }
  });
}

test('manifest and runtime smoke cover every vendored language', () => {
  assert.deepEqual(Object.keys(manifest.languages).sort(), Object.keys(samples).sort());
});

test('bundled binary, query, and license checksums match the pinned export', async () => {
  const checksums = JSON.parse(await readFile(new URL('SHA256SUMS.json', assets), 'utf8'));
  for (const [name, expected] of Object.entries(checksums)) {
    const contents = await readFile(new URL(name, assets));
    assert.equal(createHash('sha256').update(contents).digest('hex'), expected, name);
  }
});

test('incomplete code still produces a syntax tree and preserves multiline captures', async () => {
  const grammar = await loadLanguage('rust');
  const parser = grammar.createParser();
  const source = 'fn main() {\r\n /* comment 😀\r\nstill comment */\r\n let text = r#"raw\r\nstring 😀"#;\r\n';
  const tree = parser.parse(source);
  try {
    assert.ok(tree.rootNode.hasError, 'the unfinished function must remain editable');
    const captures = grammar.query.captures(tree.rootNode);
    assert.ok(captures.some(({ name, node }) => name.startsWith('comment') && node.text.includes('still comment')));
    assert.ok(captures.some(({ name, node }) => name.startsWith('string') && node.text.includes('string 😀')));
  } finally {
    tree.delete();
    parser.delete();
  }
});

test('real worker incremental results match fresh trees after Unicode and CRLF edits', async () => {
  const waiting = new Map();
  let nextId = 1;
  const handle = createWorkerHandler(loadLanguage, reply => {
    const resolve = waiting.get(reply.id);
    waiting.delete(reply.id);
    resolve(reply);
  });
  function highlight(documentId, revision, language, source) {
    const id = nextId++;
    return new Promise(resolve => {
      waiting.set(id, resolve);
      handle({ id, type: 'highlight', documentId, revision, language, source });
    });
  }
  let documentId = 0;
  for (const [language, [initialSource]] of Object.entries(samples)) {
    documentId++;
    let revision = 0;
    const sources = [
      initialSource,
      initialSource.replace('😀', '😃'),
      `\r\n${initialSource.replace('😀', '中文😀').replaceAll('\n', '\r\n')}`,
      `\n${initialSource.replace('😀', String.fromCodePoint(0x1e600))}`,
      '', initialSource,
    ];
    try {
      for (const source of sources) {
        const result = await highlight(documentId, ++revision, language, source);
        assert.ok(!result.error, `${language}: ${JSON.stringify(result.error)}`);
        const grammar = await loadLanguage(language);
        const parser = grammar.createParser();
        const fresh = parser.parse(source);
        try {
          assert.deepEqual(result.spans, normalizeCaptures(grammar.query.captures(fresh.rootNode), source), `${language} revision ${revision}`);
          assert.equal(result.source, source);
          assert.equal(result.revision, revision);
        } finally {
          fresh.delete();
          parser.delete();
        }
      }
      // The same document session can switch grammars and return equivalent data.
      const result = await highlight(documentId, ++revision, 'json', samples.json[0]);
      const grammar = await loadLanguage('json');
      const parser = grammar.createParser();
      const fresh = parser.parse(samples.json[0]);
      try {
        assert.ok(!result.error, JSON.stringify(result.error));
        assert.deepEqual(result.spans, normalizeCaptures(grammar.query.captures(fresh.rootNode), samples.json[0]));
      } finally {
        fresh.delete();
        parser.delete();
      }
    } finally {
      handle({ type: 'close', documentId });
    }
  }
  assert.equal(waiting.size, 0);
});
