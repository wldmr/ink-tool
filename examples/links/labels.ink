Labels in knots namespaced by their Knot.
When the cursor is over a "less specific" part of the label reference, then we treat these as references to the containing scopes (because that's what the user might be interested in when they put the cursor there).
However, the more "specific" the cursor, the fewer alternate options we present.

*   -> knot.foo
    //      ^^^ references knot.foo
    // ^^^^     references knot.foo knot

Nested labels (labels in stitches inside knots) are a bit surprising, in that you can leave out the stitch name:
*   Go to Knot's bar -> knot.bar
                     //      ^ references knot.bar
                     // ^ references knot

// Note the references-not claims below:
// We're trying to be as specific as possible when interpreting the user's intent.
*   Go to Knot's bar -> knot.stitch.bar
                     //             ^ references knot.bar
                     //             ^ references-not knot knot.stitch
                     //      ^ references knot.stitch knot.bar
                     //      ^ references-not knot
                     // ^ references knot knot.stitch knot.bar

However, {foo} and {bar} mean nothing out here, because those names aren't global.
//                  ^^^ references-nothing
//        ^^^ references-nothing

=== knot ===
//  ^^^^ defines knot

Labels inside knots are namespaced by their knot.

-   (foo) This is knot.foo -> DONE
//   ^^^ defines knot.foo

= stitch
//^^^^^^ defines knot.stitch

Crucially, labels inside nested stitches are _also_ namespaced by their knot, meaning defining a `(foo)` in this stitch is ambiguous

*   (bar) This is knot.bar -> DONE
//   ^^^ defines knot.bar

Within the definining knot (and nested stitches), the label can be referenced without the leading knot,
<> so {foo} and {bar} are the number of times we've visited those labels, respectively.
//               ^^^ references knot.bar
//     ^^^ references knot.foo

And so are {knot.foo}, {knot.bar} and {knot.stitch.bar}.
//                                                 ^^^ references knot.bar
//                                          ^^^^^^     references knot.bar knot.stitch
//                                     ^^^^            references knot.bar knot.stitch knot
//                           ^^^ references knot.bar
//                      ^^^^     references knot.bar knot
//               ^^^ references knot.foo
//          ^^^^     references knot.foo knot
