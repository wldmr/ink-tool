=== knot(name) ===

= stitch
Hello again {name}.
//           ^^^^ diagnostic Undefined
-> END

/* TEST Knot parameters are not visible in stitches
ERROR: 'params-visibility.ink' line 4: Unresolved variable: name
*/
