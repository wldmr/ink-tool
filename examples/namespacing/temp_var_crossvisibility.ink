/* Temp variables of the same name share the same value.

Note how in these examples, the value for `telepath` is determined by the
cronologically last assigment to that name, not any kind of scope.

It is very unclear to me what the LSP is supposed to even do here.

- Where to link?
  - All definitions of the same name?
  - Only in the same scope?
- What warnings to show?
  - Only if temp is not defined right at the top?
  - Only if it can’t be guaranteed that the temp assignment is encountered
    before the first reference?

I really don’t want to get into full static program analysis for such a dynamic
language … :-/
*/

VAR stop = false
~ temp telepath = "?"

* [a] -> a
* [b] -> b

== a ==
a: I sense {telepath}.
~ temp telepath = "a"
a: Now I sense {telepath}.
{stop: -> END}
~ stop = true
-> b

== b ==
b: I sense {telepath}.
~ temp telepath = "b"
b: Now I sense {telepath}.
{stop: -> END}
~ stop = true
-> a



/* TEST

1: a
2: b
?> 1
a: I sense ?.
a: Now I sense a.
b: I sense a.
b: Now I sense b.
*/

/* TEST

1: a
2: b
?> 2
b: I sense ?.
b: Now I sense b.
a: I sense b.
a: Now I sense a.
*/


