// Labels of the same name can be defined in different stitches.
// In that case, the stitch name can be omitted, which will default to the first stitch.

* [to a] -> knot.a.label
         //        ^^^^^ references default-labels:a
* [to b] -> knot.b.label
         //        ^^^^^ references default-labels:b

* [to ?] -> knot.label
         //      ^^^^^ references default-labels:a

== knot

= a
- (label)
// ^^^^^ defines default-labels:a
// ^^^^^ no-diagnostic
  knot.a.label
-> DONE

= b
- (label)
// ^^^^^ defines default-labels:b
// ^^^^^ no-diagnostic
  knot.b.label
-> DONE





/* TEST

1: to a
2: to b
3: to ?
?> 1
knot.a.label
*/


/* TEST

1: to a
2: to b
3: to ?
?> 2
knot.b.label
*/



/* TEST

1: to a
2: to b
3: to ?
?> 3
knot.a.label
*/
