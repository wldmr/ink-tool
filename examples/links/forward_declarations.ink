Due to the non-linear nature of Ink, we consider a name to exists for the entire scope it is defined in, even if it is referenced _before_ it is declared.

With globals this is not problematic, since these will be evaluated before any Ink is run, so the will always be defined.

It is {some_var} the `some_var` is true.
//     ^^^^^^^^ references var
VAR some_var = true
//  ^^^^^^^^ defines var

VARs are always global, so they can be (forward) referenced,
<> even if they are inside a knot or stitch.
So this is {inside_stitch}. :)
//          ^^^^^^^^^^^^^ references inside

This is not surprising, since this is the way it _must_ work for addresses.
-> knot
// ^^^^ references knot

=== knot ===
//  ^^^^ defines knot

I can see the globally defined "{inside_stitch}".
//                               ^^^^^^^^^^^^^ references inside

= stitch

I am {inside_stitch} and {tmp} years old.
//    |           |       ^^^ references k.s.tmp
//    ^^^^^^^^^^^^^ references inside

VAR inside_stitch = "cool"
//  ^^^^^^^^^^^^^ defines inside

~ temp tmp = 1
//     ^^^ defines k.s.tmp

