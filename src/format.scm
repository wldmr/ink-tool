(ink
 [_ (_)] @align
) @aligned @no.space.before @no.space.after

((_) . [(content_block) (knot_block) (stitch_block)] @blankline.before)
([(content_block) (knot_block) (stitch_block)] @blankline.after . (_))

(choice_block
 (choice (choice_marks) . (_) @align)
 (_)+ @align) @aligned

(gather_block
 (gather !eol) . (_) @align
 (_)+ @align) @aligned

;;; Normalize Knot/Stitch marks
((knot start_mark: _ @it @space.after)
 (#replace @it "==="))

[(knot) (stitch)] @blankline.before @blankline.after

; ensure that knots end in a mark
((knot !function end_mark: _ @it @space.before)
 (#replace @it "==="))
((knot !function !end_mark (_) @it .)
; TODO: find a way to insert a formatting space instead of a text space here.
 (#append @it " ==="))

;; Functions should look a little differently
((knot start_mark: _ @start @space.after
       function: _
       end_mark: _? @delete)
 (#replace @start "==")) @newline.after

(stitch start_mark: _ @space.after) @blankline.before

;;; Normalize Choices and gathers
[(choice_mark) (gather_mark)] @space.after

[(paragraph) (choice)] @newline.after

; just leaving the eol as-is will bunch up multiple lines if the (eol) is followed by a formatter based line break.
; To get around this, we replace it by a formatting newline right away.
(gather eol: (_) @delete) @newline.after

(list "LIST" @space.after name: (_) @space.after "=" @space.after)
(external "EXTERNAL" @space.after (params) @no.space.before)
 
(list_value_defs (list_value_def) @align ",") @aligned

; Move parens around list definitions to the outside: (name) = 1 -> (name = 1)
((list_value_def name: (_) . ")" @delete value: (_) @value)
 (#append @value ")"))

(list_value_def "(" @no.space.after ")" @no.space.before)
(list_value_def "=" @space.before @space.after)

"," @no.space.before @space.after

(params "(" @no.space.after
        ")" @no.space.before
) @no.space.before

(divert "->" @space.after)

; Lists stand alone, except a run of consecutive lists
((_) . (list) @blankline.before)
((list) @blankline.after . (_))

((list) @newline.after . (list) @newline.before)

((_) @space.after . (line_comment) @blankline.after)

(eval "{" @no.space.after "}" @no.space.before)
(binary op: _ @space.before @space.after)
(unary op: _ @no.space.after)
(conditional_text "{" @no.space.after ":" @no.space.before)

