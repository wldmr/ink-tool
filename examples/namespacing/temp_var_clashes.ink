// VARs will clash with temps
VAR name = "Inigo Montoya"
//  ^^^^ diagnostic Multiple definitions
-> knot

=== knot ===
~ temp name = "Slim Shady"
//     ^^^^ diagnostic Multiple definitions
-> END


/* TEST
ERROR: 'temp_var_clashes.ink' line 6: temp 'name': name has already been used for a var on line 2 of temp_var_clashes.ink
*/
