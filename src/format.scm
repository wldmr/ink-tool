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

((list_value_def "(" (_) @name ")" "=" (_) @value) @rewrite
 (#rewrite-to "(" @name "=" @value ")"))

((list_value_def [_ (_)] @replace.start . "=" @replace.end)
 (#set! "replacement" " "))

((list_value_def "=" @replace.start . [(_) _] @replace.end)
 (#set! "replacement" " "))

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

; Second line is aligned to the start of the content of the firs
([(content) @indent.anchor
  (choice (choice_marks) . [_ (_)] @indent.anchor)
  (gather (gather_marks) . [_ (_)] @indent.anchor) ]
 .
 [(content) @indent.to.anchor
  (choice_block (choice) @indent.to.anchor)
  (gather_block (gather) @indent.to.anchor)])

; Content following content: Same indentation
((content) @indent.anchor . (content) @indent.to.anchor)

