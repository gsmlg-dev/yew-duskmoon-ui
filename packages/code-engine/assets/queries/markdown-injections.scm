(fenced_code_block
  (info_string
    (language) @injection.language)
  (code_fence_content) @injection.content
  (#set! injection.include-children))

((html_block) @injection.content
  (#set! injection.include-children) (#set! injection.language "html"))

(document . (section . (thematic_break) (_) @injection.content
  (#set! injection.include-children) (thematic_break)) (#set! injection.language "yaml"))

([(minus_metadata) (plus_metadata)] @injection.content
  (#set! injection.include-children) (#set! injection.language "yml"))

((inline) @injection.content
  (#set! injection.include-children) (#set! injection.language "markdown_inline"))
