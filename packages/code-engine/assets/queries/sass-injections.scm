; Sass Code Injection Queries
; ============================

; URL contents could be injected with different highlighting
(call_expression
  (function_name) @_url
  (arguments) @injection.content
  (#eq? @_url "url")
  (#set! injection.language "uri"))

; Comments could contain documentation markup
((comment) @injection.content
  (#set! injection.language "comment"))

; String interpolations - the content inside #{} uses Sass expressions
(interpolation
  (_) @injection.content
  (#set! injection.language "sass"))
