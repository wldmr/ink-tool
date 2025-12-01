Definitions reference themselves.

VAR var = 1
//  ^^^ defines var
//  ^^^ references var

CONST const = 1
//    ^^^^^ defines const
//    ^^^^^ references const

- (label) lorem ipsum
// ^^^^^ defines label
// ^^^^^ references label

EXTERNAL external()
//       ^^^^^^^^ defines external
//       ^^^^^^^^ references external

== knot ==
// ^^^^ defines knot
// ^^^^ references knot
lorem

= stitch
//^^^^^^ defines stitch
//^^^^^^ references stitch
ipsum

~ temp tmp = 2
//     ^^^ defines temp
//     ^^^ references temp
