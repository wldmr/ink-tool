Temp variables are visible _only_ at the exact level they are defined in.
That means they aren't visible in higher levels, NOR in lower levels.
(That last one is a bit unusual; in other programming languages local variables are visible in enclosed scopes.)

~ temp t = "temporarily"
//     ^ defines temp_vars:toplevel
He was {t} indisposed.
//      ^ references temp_vars:toplevel
//      ^ references-not temp_vars:knot temp_vars:knot.stitch temp_vars:knot.stitch2 temp_vars:k2

=== knot ===
~ temp t = "knot"
//     ^ defines temp_vars:knot
He was {t} amused.
//      ^ references temp_vars:knot
//      ^ references-not temp_vars:toplevel temp_vars:knot.stitch temp_vars:knot.stitch2 temp_vars:k2

= stitch
~ temp t = "stitch"
//     ^ defines temp_vars:knot.stitch
He was in {t}es.
//         ^ references temp_vars:knot.stitch
//         ^ references-not temp_vars:toplevel temp_vars:knot temp_vars:knot.stitch2 temp_vars:k2

= stitch_2
~ temp t = 2
//     ^ defines temp_vars:knot.stitch2
//     ^ references-not temp_vars:toplevel temp_vars:knot temp_vars:knot.stitch temp_vars:k2
It was {t} hot.
//      ^ references temp_vars:knot.stitch2

=== k2 ===
~ temp t = "k2"
//     ^ defines temp_vars:k2
That was o{t}.
//         ^ references temp_vars:k2
//         ^ references-not temp_vars:toplevel temp_vars:knot temp_vars:knot.stitch temp_vars:knot.stitch2
