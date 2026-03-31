// Labels in knots before the first stitch prevent similarly named labels in stitches

-> knot.label
//      ^^^^^ references rmi gja
//      We can't know at this point, so we just link to both.
== knot
- (label) // knot-level
// ^^^^^ defines rmi
// ^^^^^ diagnostic Multiple definitions of knot.label
-> DONE

= a
- (label) // stitch-level
// ^^^^^ defines gja
// ^^^^^ diagnostic Multiple definitions of knot.label
-> DONE




// Inclecate fails to compile this, but the LSP "accepts" it, showing an error.

/* TEST
ERROR: 'labels_in_knot_header.ink' line 13: Gather 'label' has the same label name as a Gather (on line 7 of labels_in_knot_header.ink)
*/
