// Unlike CONSTs, VARs will fail to compile if they clash with parameters
VAR name = "Inigo Montoya"

=== knot(name) ===
//       ^^^^ diagnostic Multiple definitions
-> END

= stitch(name)
//       ^^^^ diagnostic Multiple definitions
-> END


/* TEST
ERROR: 'var.ink' line 8: argument 'name': name has already been used for a var on line 2 of var.ink
ERROR: 'var.ink' line 4: argument 'name': name has already been used for a var on line 2 of var.ink
*/
