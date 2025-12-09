Labels in knots namespaced by their Knot.
*   -> knot.foo
    // ^^^^^^^^ references knot.foo

Nested labels (labels in stitches inside knots) are a bit surprising, in that you can leave out the stitch name:
*   Go to Knot's bar -> knot.bar
                     // ^^^^^^^^ references knot.bar
*   Go to Knot's bar -> knot.stitch.bar
                     // ^^^^^^^^^^^^^^^ references knot.bar

However, {foo} and {bar} mean nothing out here, because those names aren't global.
//        |||       ^^^ references-nothing
//        ^^^ references-nothing

=== knot ===

Labels inside knots are namespaced by their knot.

-   (foo) This is knot.foo -> DONE
//   ^^^ defines knot.foo

= stitch

Crucially, labels inside nested stitches are _also_ namespaced by their knot, meaning defining a `(foo)` in this stitch is ambiguous

*   (bar) This is knot.bar -> DONE
//   ^^^ defines knot.bar

Within the definining knot (and nested stitches), the label can be referenced without the leading knot,
<> so {foo} and {bar} are the number of times we've visited those labels, respectively.
//               ^^^ references knot.bar
//     ^^^ references knot.foo

And so are {knot.foo}, {knot.bar} and {knot.stitch.bar}.
//                                     ^^^^^^^^^^^^^^^ references knot.bar
//                      ^^^^^^^^ references knot.bar
//          ^^^^^^^^ references knot.foo
