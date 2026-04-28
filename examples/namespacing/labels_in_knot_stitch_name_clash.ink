/// Label-stitch name clashes *outside* the knot are resolved in favor of the stitch
/// Label-stitch name clashes *inside* the knot are resolved in favor of the label

* [knot]        -> knot
//                 ^^^^ references clash:knot
* [knot.stitch] -> knot.stitch
//                      ^^^^^^ references clash:stitch
//                             Counter-intuitively, this links to the *stitch*, not the label!

== knot
// ^^^^ defines clash:knot

knot text

- (stitch)
// ^^^^^^ defines clash:label
// ^^^^^^ no-diagnostics

  label text{stitch>1: again}
  * {stitch > 1}
    -> END
  * -> stitch
//     ^^^^^^ references clash:label

= stitch
//^^^^^^ defines clash:stitch
//^^^^^^ no-diagnostics

stitch text
-> END





/* TEST Link from inside favors the label

1: knot
2: knot.stitch
?> 1
knot text
label text
label text again
*/

/* TEST Link from outside favors the stitch

1: knot
2: knot.stitch
?> 2
stitch text
*/
