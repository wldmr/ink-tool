(ink
 [(content_block) (knot_block) (stitch_block)] @align
) @align

(choice_block
 (choice (choice_marks) . (_) @align)
 (_)+ @align)

(gather_block
 (gather !eol) . (_) @align
 (_)+ @align)

;;; Normalize Knot/Stitch marks
((knot start_mark: _ @it)
 (#replace @it "===")
 (#after @it " "))

((knot end_mark: _ @it)
 (#before @it " ")
 (#replace @it "==="))

((knot !end_mark (_) @it .)
 (#after @it " ==="))

;;; Normalize Choices and gathers
; space afer each mark
((choice_mark) @it
 (#after @it " "))
; offset content after the last mark, for better visibility
((choice_marks) @it
 (#after @it "   "))

((gather_mark) @it
 (#after @it " "))
((gather_marks) @it
 (#after @it "   "))

((list "=" @it)
 (#before @it " ")
 (#after @it " "))

; Move parens around list definitions to the outside: (name) = 1 -> (name = 1)
((list_value_def name: (_) . ")" @paren value: (_) @value)
 (#replace @paren "")
 (#after @value ")"))

((list_value_def "(" @open ")" @close)
 (#after @open "")
 (#before @close ""))

((list_value_def "=" @it)
 (#before @it " ")
 (#after @it " "))

((list_value_defs "," @it)
 (#before @it "")
 (#after @it " "))
