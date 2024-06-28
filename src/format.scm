((knot !function start_mark: _ @replace.this)
 (#set! "replacement" "==="))

; After last mark
((choice (choice_marks) @replace.start . [_ (_)] @replace.end)
 (#set! "replacement" " "))

; Between marks
((choice_marks (choice_mark) @replace.start . (choice_mark) @replace.end)
 (#set! "replacement" " "))

((divert "->" @replace.start . _ @replace.end)
 (#set! "replacement" " "))

;;; List Definitions

;; Canonized placement of parens
;; XXX: Order matters here; if this comes after the spacing rules, then it eats the last paren.
((list_value_def . "(" . (identifier) @name . ")" . "=" . (number) @value) @rewrite
 (#rewrite-to "(" @name "=" @value ")"))

;; Spaces between name and values
((list name: (identifier) @replace.start . "=" . [_] @replace.end)
 (#set! "replacement" " = "))

;; Spaces between elements
((list_value_def) @replace.start . (list_value_def) @replace.end
 (#set! "replacement" ", "))

;; Spaces inside parens
;; IDEA: Have this rule accept multiple start/end pairs
((list_value_def ["("] @replace.start . [_] @replace.end)
 (#set! "replacement" ""))
((list_value_def [_] @replace.start . [")"] @replace.end)
 (#set! "replacement" ""))

;; Spaces around name and numeric value
((list_value_def name: (identifier) @replace.start . "=" . value: (number) @replace.end)
 (#set! "replacement" " = "))

;;; Indentation

;; Top level and structure indentation: flush left
(ink [(content_block [(content)              @indent.to.anchor
                      (choice_block (choice) @indent.to.anchor)
                      (choice_block (choice) @indent.to.anchor)])
      (stitch_block (stitch)                 @indent.to.anchor)
      (knot_block   (knot)                   @indent.to.anchor)
      (knot_block   (stitch_block (stitch)   @indent.to.anchor))]
) @indent.anchor

;; Stitches are aligned to their knot

;; Indentation of flow

; Items on the same level are aligned to each other
([(content) @indent.anchor
  (choice_block (choice) @indent.anchor)
  (gather_block (gather) @indent.anchor)]
 .
 [(content) @indent.anchor
  (choice_block (choice) @indent.to.anchor)
  (gather_block (gather) @indent.to.anchor)])

; Second line is aligned to the start of the content of the first
([(content) @indent.anchor
  (choice (choice_marks) . [_] @indent.anchor)
  (gather (gather_marks) . [_] @indent.anchor) ]
 .
 [(content) @indent.to.anchor
  (choice_block (choice) @indent.to.anchor)
  (gather_block (gather) @indent.to.anchor)])

; Content following content: Same indentation
((content) @indent.anchor . (content) @indent.to.anchor)

