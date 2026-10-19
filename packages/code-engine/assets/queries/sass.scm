; Sass Syntax Highlighting Queries
; ==================================

; Comments
(comment) @comment.block
(single_line_comment) @comment.line

; Keywords
[
  "@import"
  "@use"
  "@forward"
  "@media"
  "@charset"
  "@namespace"
  "@supports"
  "@scope"
  "@layer"
  "@container"
  "@font-face"
  "@mixin"
  "@include"
  "@extend"
  "@if"
  "@else"
  "@each"
  "@for"
  "@while"
  "@function"
  "@return"
  "@at-root"
  "@error"
  "@warn"
  "@debug"
  "@content"
] @keyword

(at_keyword) @keyword

; Shorthand keywords (= for @mixin, + for @include)
(shorthand_mixin) @keyword
(shorthand_include) @keyword

; Control flow keywords
[
  "from"
  "through"
  "to"
  "in"
  "and"
  "or"
  "not"
  "only"
] @keyword.control

; Module keywords
[
  "as"
  "with"
  "using"
  "hide"
  "show"
] @keyword.import

; Boolean and null literals
(boolean_value) @constant.builtin.boolean
(null_value) @constant.builtin

; CSS Custom Properties
(custom_property_name) @property.definition

; Important/default/global flags
(important) @keyword.modifier
(default) @keyword.modifier
(global) @keyword.modifier
(optional_flag) @keyword.modifier

; Numbers and units
(integer_value) @number
(float_value) @number.float
(unit) @type

; Colors
(color_value) @constant.numeric.color

; Unicode Range
(unicode_range) @string.special

; Strings
(string_value) @string
(escape_sequence) @string.escape

; Variables
(variable_name) @variable
(variable_value) @variable
(variable_identifier) @variable

; Properties
(property_name) @property

; Functions
(function_name) @function
(function_statement
  (name) @function.definition)
(mixin_statement
  (name) @function.definition)
(mixin_name) @function
(call_expression
  (function_name) @function.call)

; Color functions
(color_function
  (function_name) @function.builtin)

; Gradient functions
(gradient_function
  (function_name) @function.builtin)

; Math functions
(math_function
  (function_name) @function.builtin)

; var() function
(var_function
  (function_name) @function.builtin)

; Selectors
(tag_name) @tag
(class_name) @type.class
(id_name) @type.id
(placeholder_name) @type.placeholder
(nesting_selector) @punctuation.special
(universal_selector) @tag

; Pseudo-classes and pseudo-elements
(pseudo_class_selector
  (class_name) @type.pseudo)
(pseudo_element_selector
  (element_name) @type.pseudo)

; Attribute selectors
(attribute_name) @attribute
(attribute_selector
  [
    "="
    "~="
    "^="
    "|="
    "*="
    "$="
  ] @operator)

; Namespace
(namespace_name) @namespace
(module) @namespace

; Interpolation
(interpolation
  "#{" @punctuation.special
  "}" @punctuation.special)

; Operators
[
  "*"
  "/"
  "=="
  "!="
  "<"
  ">"
  "<="
  ">="
] @operator

; Punctuation
[
  "("
  ")"
] @punctuation.bracket

[
  "["
  "]"
] @punctuation.bracket

[
  ":"
  "::"
  ","
  "."
  "~"
  "|"
] @punctuation.delimiter

; Spread operator
"..." @punctuation.special

; Media queries
(keyword_query) @keyword.media
(feature_name) @property

; Range queries
(range_query) @keyword.media

; Layer and Container
(layer_name) @type
(container_name) @type

; Keyframes
(keyframes_name) @string.special
