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
((knot start_mark: _ @it @space.after)
 (#replace @it "==="))

((knot end_mark: _ @it @space.before)
 (#replace @it "==="))

((knot !end_mark (_) @it .)
 (#after @it " ==="))

;;; Normalize Choices and gathers
; space afer each mark
[(choice_mark) (gather_mark)] @space.after
[(choice) (gather)] @newline.after

(list "LIST" @space.after name: (_) @space.after "=" @space.after)
(external "EXTERNAL" @space.after (params) @nothing.before)
 

; Move parens around list definitions to the outside: (name) = 1 -> (name = 1)
; ((list_value_def "(" @open name: (_) ")" @close value: (_) @value)
;  (#after @open "")
;  (#after @value ")")
;  (#replace @close ""))

(list_value_def "(" @nothing.after ")" @nothing.before)
(list_value_def "=" @space.before @space.after)
(list_value_defs "," @nothing.before @space.after)

(params "("  @nothing.before @nothing.after
        ","* @nothing.before @space.after
        ")"  @nothing.before @nothing.after)

(divert "->" @space.after)

(block_comment) @space.before @space.after

[(paragraph) (knot) (stitch) (code) (external) (global)] @newline.after

((_) @space.after . (line_comment) @newline.after)
((_) @space.after . (block_comment))
((block_comment) . (_) @space.before)

; Lists stand alone, except a run of consecutive lists
(list) @blankline.before @blankline.after
((list) @newline.after . (list) @newline.before)

((list) @space.after . (line_comment) @blankline.after)

(eval "{" @nothing.after "}" @nothing.before)
(binary op: _ @space.before @space.after)
(unary op: _ @nothing.after)
(conditional_text "{" @nothing.after ":" @nothing.before)

(ink) @newline.after
