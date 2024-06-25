((knot !function start_mark: _ @replace.this)
 (#set! "replacement" "==="))

(choice_marks _+ @indent.fixed.count) @indent.fixed
(gather_marks _+ @indent.fixed.count) @indent.fixed

((choice (choice_marks) @replace.start . (_) @replace.end)
 (#set! "replacement" " "))

((choice_marks _ @replace.start . _ @replace.end)
 (#set! "replacement" " "))

((choice_block (choice) @replace.before)
 (#set! "replacement" "="))

((divert "->" @replace.start . _ @replace.end)
 (#set! "replacement" " "))

((condition "{" @replace.start . (_) @replace.end)
 (#set! "replacement" " "))
((condition (_) @replace.start . "}" @replace.end)
 (#set! "replacement" " "))

((list_value_def "(" (_) @name ")" "=" (_) @value) @rewrite
 (#rewrite-to "(" @name "=" @value ")"))

((list_value_def [_ (_)] @replace.start . "=" @replace.end)
 (#set! "replacement" " "))

((list_value_def "=" @replace.start . [(_) _] @replace.end)
 (#set! "replacement" " "))

((choice main: _ @indent.anchor)
 (content) @ident.to.anchor)

((gather (gather_marks) . _ @indent.anchor)
 (content) @ident.to.anchor)
