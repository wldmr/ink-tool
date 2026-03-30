Definitions reference themselves.

VAR var = 1
//  ^^^ defines self-reference:var
//  ^^^ references self-reference:var

CONST const = 1
//    ^^^^^ defines self-reference:const
//    ^^^^^ references self-reference:const

- (label) lorem ipsum
// ^^^^^ defines self-reference:label
// ^^^^^ references self-reference:label

EXTERNAL external()
//       ^^^^^^^^ defines self-reference:external
//       ^^^^^^^^ references self-reference:external

== knot ==
// ^^^^ defines self-reference:knot
// ^^^^ references self-reference:knot
lorem

= stitch
//^^^^^^ defines self-reference:stitch
//^^^^^^ references self-reference:stitch
ipsum

~ temp tmp = 2
//     ^^^ defines self-reference:temp
//     ^^^ references self-reference:temp
