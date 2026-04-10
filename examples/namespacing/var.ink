// Unlike CONSTs, VARs will fail to compile if they clash with parameters
VAR name = "Inigo Montoya"

=== knot(name) ===
//       ^^^^ diagnostic clashes with VAR
-> END

= stitch(name)
//       ^^^^ diagnostic clashes with VAR
-> END


/* TEST
ERROR: 'var.ink' line 16: argument 'name': name has already been used for a var on line 3 of var.ink
ERROR: 'var.ink' line 9: argument 'name': name has already been used for a var on line 3 of var.ink
*/
