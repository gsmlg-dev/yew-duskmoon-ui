((html_tag) @injection.content
  (#set! injection.include-children) (#set! injection.language "html"))
((latex_block) @injection.content
  (#set! injection.include-children) (#set! injection.language "latex"))
