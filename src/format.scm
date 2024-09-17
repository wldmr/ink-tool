(ink) @break.before.0 @break.after

; We have to check for consecutive blocks (instead of simply saying `(some_block) @break.before.…`)
; because comments can get in the way and add unwanted lines.
; That means that comments can disrupt spacing rules, but let's call that a feature ;).
(knot_block (knot !function)) @break.before.4
(stitch_block) @break.before.3

; Allow individual content items to be offset by a single blank line at most
[(todo_comment)
 (paragraph)
 (code)
 (include)
 (external)
 (global)
 (list)
 (choice_block)
 (gather_block)] @break.before.0-2 @break.after.1-2

; Text is a tricky beast; it actually has individual children (such as '<' '-' etc, to allow for parsing syntax elements)
(text) @take.as-is @space.before.0-1 @space.after.0-1

; Choice blocks indent their contents and align them to the first thing after the marks indent.
(choice_block (choice (choice_marks) . (_) @indent.anchor)) @dedent

; Gathers behave a bit strangely, because they can be empty.
; If they're not empty (that is, have a label or content on the same line)
; then we indent the contents of the block;
; A completely naked gather is intended to stand out as a separation line. Indenting would look weird, so we don't.
(gather_block (gather label: (_) @indent.anchor)) @dedent
(gather_block (gather !label !eol) . (_) @indent.anchor) @dedent
(gather_block (gather eol: (_) @delete) @break.after)

; Idea for a different style, basically a more 'extreme' version of the above:
; Only, gathers with content on the same line get indentation, all the other ones don't
; (gather_block (gather !eol label: (_) @indent.anchor)) @dedent
; (gather_block (gather !label !eol) . (_) @indent.anchor) @dedent
; (gather_block (gather eol: (_)))

(label) @space.before @space.after

; Just leaving the eol as-is will bunch up multiple lines if the (eol) is followed by a formatter based line break.
; To get around this, we replace it by a formatting newline right away.
(eol) @delete

;;; Normalize Knot/Stitch marks
((knot !function start_mark: _ @it @space.after) @break.after
 (#replace @it "==="))

; ensure that knots end in a mark
((knot !function end_mark: _ @space.before)
 (#replace @space.before "==="))

((knot !function !end_mark (_) @space.after .)
; TODO: find a way to insert a formatting space instead of a text space here.
 (#append @space.after "==="))

[(knot !function) (stitch)] @break.after.2-2

;; Functions should look a little differently
((knot_block
 (knot start_mark: _ @start @space.after
       function: _ @space.before @space.after @indent.anchor
       end_mark: _? @delete) @break.after)
 (#replace @start "==")) @dedent @break.after.3

(stitch "="
        @space.before.0 ; to counteract the general rule for "=" that puts a space around it. Unfortunate side-effect of the grammar naming both syntax elements "=" :-/
        @space.after)

(global keyword: _ @space.after)

;;; Normalize Choices and gathers
[(choice_mark) (gather_mark)] @space.after
[(choice_marks) (gather_marks)] @space.after.3 ; Visually offset text. A lot of example ink does this and it seems like a neat idea.

(choice choice_only: (_) @space.before.0-1 @space.after.0-1)

"=" @space.before @space.after

; Move parens around list definitions to the outside: (name) = 1 -> (name = 1)
((list_value_def name: (_) . ")" @delete value: (_) @value)
 (#append @value ")"))

(list_value_def "(" @space.after.0 ")" @space.before.0)
(list_value_def "=" @space.before @space.after)

"," @space.before.0 @space.after

(params "(" @space.after.0 ")" @space.before.0 ) @space.before.0

(call "(" @space.before.0 @space.after.0 ")" @space.before.0)

["->" "<-"] @space.after
"=" @space.before @space.after ; Note that this conflicts with the "=" mark for stitches (see there for the rule that undoes this). This seems simpler than enumerating all the other places that "=" can appear

(line_comment) @break.after.1-3

(eval "{" @space.after.0 "}" @space.before.0)

(binary op: _ @space.before @space.after)

(unary op: "not" @space.after)
(unary op: ["-" "!"] @space.after.0)

(conditional_text "{" @space.after.0 ":" @space.before.0) @space.before.0-1 @space.after.0-1

(alternatives mark: _ @space.before.0) @space.before.0-1 @space.after.0-1

(condition "{" @space.after.0 "}" @space.before.0) @space.before @space.after

(cond_block "{" @space.after "}" @break.before) @space.before.0-1 @space.after.0-1
(cond_arm condition: (_) @space.before @space.after.0) @break.before.0-2 @break.after.1-2
; If linebreak: simple indent, if not: align the content on first line
(cond_arm ":" @break.after @indent eol: _) @dedent
(cond_arm ":" @space.after !eol . (_) @indent.anchor) @dedent

(multiline_alternatives
 type: _+ @space.before @space.after
 ":" @space.before.0 @indent
 "}" @dedent.this
)

(alt_arm "-" . (_) @indent.anchor) @dedent @break.before.0-2 @break.after.1-2

(code "~" @space.after)
["temp" "return" "VAR" "EXTERNAL" "LIST"] @space.after

(paren "(" @space.after.0 ")" @space.before.0)
