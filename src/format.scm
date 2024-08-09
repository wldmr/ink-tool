(ink
 [(content_block) (knot_block) (stitch_block)] @align
) @align @no.space.before @no.space.after

(choice_block
 (choice (choice_marks) . (_) @align)
 (_)+ @align)

(gather_block
 (gather !eol) . (_) @align
 (_)+ @align)

;;; Normalize Knot/Stitch marks
((knot start_mark: _ @it @space.after)
 (#replace @it "===")) 

[(knot) (stitch)] @blankline.before @blankline.after

; ensure that knots end in a mark
((knot !function end_mark: _ @it @space.before)
 (#replace @it "==="))
((knot !function !end_mark (_) @it .)
 (#append @it " ==="))

;; Functions should look a little differently
((knot start_mark: _ @start @space.after
       function: _
       end_mark: _? @delete)
 (#replace @start "==")) @newline.after

(stitch start_mark: _ @space.after) @blankline.before

;;; Normalize Choices and gathers
; space afer each mark
[(choice_mark) (gather_mark)] @space.after
[(choice) (gather)] @newline.after

(list "LIST" @space.after name: (_) @space.after "=" @space.after)
(external "EXTERNAL" @space.after (params) @no.space.before)
 
; Move parens around list definitions to the outside: (name) = 1 -> (name = 1)
((list_value_def name: (_) . ")" @delete value: (_) @value)
 (#append @value ")"))

(list_value_def "(" @no.space.after ")" @no.space.before)
(list_value_def "=" @space.before @space.after)

(list_value_defs "," @no.space.before @space.after)

(params "("  @no.space.before @no.space.after
        ","* @no.space.before @space.after
        ")"  @no.space.before)

(divert "->" @space.after)

; Lists stand alone, except a run of consecutive lists
(list) @blankline.before @blankline.after
((list) @newline.after . (list) @newline.before)

((list) @space.after . (line_comment) @blankline.after)

(eval "{" @no.space.after "}" @no.space.before)
(binary op: _ @space.before @space.after)
(unary op: _ @no.space.after)
(conditional_text "{" @no.space.after ":" @no.space.before)

