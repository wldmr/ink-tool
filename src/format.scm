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

([
  (choice (choice_marks) . [_ (_)] @indent.anchor)
  (gather (gather_marks) . [_ (_)] @indent.anchor)
  ]
 .
 [(content) @indent.to.anchor
  (choice_block (choice) @indent.to.anchor)
  (gather_block (gather) @indent.to.anchor)])
