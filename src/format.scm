(ink)  @no.space.before @newline.after

; Idea: Separate consecutive knot blocks by two blank lines
([(content_block) (knot_block) (stitch_block)] . [(content_block) (knot_block) (stitch_block)] @blankline.before)

; Text is a tricky beast; it actuall has individual children (such as '<' '-' etc, to allow for parsing syntax elements)⭲
(text) @leaf 

; Choice blocks indent their contents and align them to the first thing after the marks indent.
(choice_block (choice (choice_marks) . (_) @indent.anchor)) @dedent

; Gathers behave a bit strangely, because they can be empty.
; If they're not empty (that is, have a label or content on the same line)
; then we indent the contents of the block;
; A completely naked gather is intended to stand out as a separation line. Indenting would look weird, so we don't.
(gather_block (gather label: (_) @indent.anchor)) @dedent
(gather_block (gather !label !eol) . (_) @indent.anchor) @dedent
(gather_block (gather !label eol: (_)))

; Idea for a different style, basically a more 'extreme' version of the above:
; Only, gathers with content on the same line get indentation, all the other ones don't
; (gather_block (gather !eol label: (_) @indent.anchor)) @dedent
; (gather_block (gather !label !eol) . (_) @indent.anchor) @dedent
; (gather_block (gather eol: (_)))

(label) @space.before @space.after

; Just leaving the eol as-is will bunch up multiple lines if the (eol) is followed by a formatter based line break.
; To get around this, we replace it by a formatting newline right away.
(eol) @delete @newline.after

;;; Normalize Knot/Stitch marks
((knot start_mark: _ @it @space.after) @blankline.after
 (#replace @it "==="))

; ensure that knots end in a mark
((knot !function end_mark: _ @it @space.before)
 (#replace @it "==="))

((knot !function !end_mark (_) @it @space.after .)
; TODO: find a way to insert a formatting space instead of a text space here.
 (#append @it "==="))

[(knot) (stitch)] @blankline.after

;; Functions should look a little differently
((knot start_mark: _ @start @space.after
       function: _
       end_mark: _? @delete)
 (#replace @start "=="))

(stitch start_mark: _ @space.after)

(global keyword: _ @space.after)

((_) . (global)) @blankline.before
((global) . (_)) @blankline.after
((global) . (global)) @newline.before @newline.after

"=" @space.before @space.after

;;; Normalize Choices and gathers
[(choice_mark) (gather_mark)] @space.after

; [(paragraph) (choice)] @newline.after

(list "LIST" @space.after name: (_) @space.after "=" @space.after)
(external "EXTERNAL" @space.after (params) @no.space.before)

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
((_) . (list)    @blankline.before)
((list) . (_)    @newline.before)
((list) . (list) @newline.before)

(line_comment) @newline.after @space.before

(eval "{" @no.space.after "}" @no.space.before)

(binary op: _ @space.before @space.after)

(unary op: "not" @space.after)
(unary op: ["-" "!"] @no.space.after)

(conditional_text "{" @no.space.after ":" @no.space.before)

(condition "{" @no.space.after "}" @no.space.before) @space.before @space.after

(cond_block "{" @space.after "}" @newline.before)
(cond_arm condition: (_) @indent @space.before @no.space.after) @dedent

(cond_arm ":" @newline.after)
(cond_arm ":" @space.after !eol)

