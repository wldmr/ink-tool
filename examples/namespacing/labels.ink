Labels in knots namespaced by their Knot.
We only redirect the user to the specific thing that the cursor points to; i.e. we asume the user is being specific.

*   -> knot.foo
    //      ^^^ references labels:knot.foo
    // ^^^^     references labels:knot

Nested labels (labels in stitches inside knots) are a bit surprising, in that you can leave out the stitch name:
*   Go to Knot's bar -> knot.bar
                     //      ^ references labels:knot.bar
                     // ^ references labels:knot

*   Go to Knot's bar -> knot.stitch.bar
                     // |  | |    | ^^^ references labels:knot.bar
                     // |  | ^^^^^^ references labels:knot.stitch 
                     // ^^^^ references labels:knot

However, {foo} and {bar} mean nothing out here, because those names aren't global.
//                  ^^^ references-nothing
//        ^^^ references-nothing

=== knot ===
//  ^^^^ defines labels:knot

Labels inside knots are namespaced by their knot.

-   (foo) This is knot.foo -> DONE
//   ^^^ defines labels:knot.foo

= stitch
//^^^^^^ defines labels:knot.stitch

Crucially, labels inside nested stitches are _also_ namespaced by their knot, meaning defining a `(foo)` in this stitch is ambiguous

*   (bar) This is knot.bar -> DONE
//   ^^^ defines labels:knot.bar

Within the definining knot (and nested stitches), the label can be referenced without the leading knot,
<> so {foo} and {bar} are the number of times we've visited those labels, respectively.
//               ^^^ references labels:knot.bar
//     ^^^ references labels:knot.foo

And so are {knot.foo}, {knot.bar} and {knot.stitch.bar}.
//          |    |      |    |         |    |      ^ references labels:knot.bar
//          |    |      |    |         |    ^ references labels:knot.stitch
//          |    |      |    |         ^ references labels:knot
//          |    |      |    ^ references labels:knot.bar
//          |    |      ^ references labels:knot
//          |    ^ references labels:knot.foo
//          ^ references labels:knot
