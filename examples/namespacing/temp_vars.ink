/*
Temp variables are visible _only_ at the exact level they are defined in.
That means they aren't visible in higher levels, NOR in lower levels.
(That last one is a bit unusual; in other programming languages local variables are visible in enclosed scopes.)

Due to the non-linear nature of Ink, we consider a name to exist for the entire scope it is defined in,
even if it is referenced _before_ it is declared.

inklecate will warn the user about a problem at runtime, and will use a default value.
*/

~ temp t = "temporarily"
//     ^ defines temp_vars:toplevel
He was {t} indisposed.
//      ^ references temp_vars:toplevel
-> knot

=== knot ===
~ temp t = "knot"
//     ^ defines temp_vars:knot
He was {t} amused.
//      ^ references temp_vars:knot
-> stitch

= stitch
~ temp t = "stitch"
//     ^ defines temp_vars:knot.stitch
He was in {t}es.
//         ^ references temp_vars:knot.stitch
-> stitch_2

= stitch_2
It was {t} hot.
//      ^ references temp_vars:knot.stitch_2
~ temp t = 2
//     ^ defines temp_vars:knot.stitch_2
-> k2

=== k2 ===
That was o{t}.
//         ^ references temp_vars:k2
~ temp t = "k2"
//     ^ defines temp_vars:k2
-> END



/* TEST Temps are local and can NOT be referenced before their definition.
He was temporarily indisposed.
He was knot amused.
He was in stitches.
It was stitch hot.
That was o2.
*/
