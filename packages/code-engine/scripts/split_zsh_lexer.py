"""Bound generated Zsh lexer function sizes without changing its state machine."""
import re
from pathlib import Path


def split_zsh_lexer(path: Path):
    # Specific to the integrity-pinned tree-sitter-zsh 0.63.1 generated lexer.
    states_per_function = 64
    source = path.read_text()
    signature = 'static bool ts_lex(TSLexer *lexer, TSStateId state) {'
    start = source.index(signature)
    end = source.index('\nstatic bool ts_lex_keywords(', start)
    original = source[start:end]
    prefix = signature + '\n  START_LEXER();\n  eof = lexer->eof(lexer);\n  switch (state) {\n'
    suffix = '    default:\n      return false;\n  }\n}\n'
    assert original.startswith(prefix)
    assert original.endswith(suffix + '\n') or original.endswith(suffix)
    body = original[len(prefix):original.rindex('    default:')]
    matches = list(re.finditer(r'^    case (\d+):\n', body, re.M))
    assert [int(m[1]) for m in matches] == list(range(len(matches)))
    assert len(matches) == 2392, 'Pinned Zsh lexer changed; review partition compatibility'
    assert body.count('case ') == len(matches)
    # Each helper executes one original state. Transitions advance exactly once,
    # retaining accepted-token state in the dispatcher; -1 means END_STATE.
    # noinline prevents LLVM from recreating the giant V8-hostile function.
    generated = ['// Partitioned for bounded browser WASM optimization; original state bodies retained.\n',
        '#define ZSH_ACCEPT_TOKEN(symbol_value) do { *result = true; \\\n  lexer->result_symbol = symbol_value; lexer->mark_end(lexer); } while (0)\n']
    for group, offset in enumerate(range(0, len(matches), states_per_function)):
        stop = min(offset + states_per_function, len(matches))
        cases = body[matches[offset].start():matches[stop].start() if stop < len(matches) else len(body)]
        cases = cases.replace('ACCEPT_TOKEN(', 'ZSH_ACCEPT_TOKEN(').replace('END_STATE();', 'return -1;')
        generated.append(f'''static __attribute__((noinline)) int ts_lex_part_{group}(TSLexer *lexer, TSStateId state, bool *result) {{
  bool skip = false;
  int32_t lookahead = lexer->lookahead;
  bool eof = lexer->eof(lexer);
  switch (state) {{
{cases}    default: return -2;
  }}
next_state:
  lexer->advance(lexer, skip);
  return state;
}}
''')
    generated.append('#undef ZSH_ACCEPT_TOKEN\n' + signature + '\n  bool result = false;\n  for (;;) {\n    int next;\n    switch (state / ' + str(states_per_function) + ') {\n')
    for group in range((len(matches) + states_per_function - 1) // states_per_function):
        generated.append(f'      case {group}: next = ts_lex_part_{group}(lexer, state, &result); break;\n')
    generated.append('      default: return false;\n    }\n    if (next == -1) return result;\n    if (next == -2) return false;\n    state = (TSStateId)next;\n  }\n}\n')
    path.write_text(source[:start] + ''.join(generated) + source[end:])
