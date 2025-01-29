Temp variables are visible _only_ at the exact level they are defined in.
That means they aren't visible in higher levels, NOR in lower levels.
(That last one is a bit unusual; in other programming languages
<> local variables are visible in enclosed scopes.)

~ temp t = "temporarily"
//     ^ defines t.toplevel
He was {t} indisposed.
//      ^ references t.toplevel

TODO: Add link testing features like references-not or references-exactly

=== knot ===
~ temp t = "knot"
//     ^ defines t.knot
He was {t} amused.
//      ^ references t.knot

= stitch
~ temp t = "stitch"
//     ^ defines t.knot.stitch
He was in {t}es.
//         ^ references t.knot.stitch

= stitch_2
~ temp t = 2
//     ^ defines t.knot.stitch_2
It was {t} hot.
//      ^ references t.knot.stitch_2

=== k2 ===
~ temp t = "k2"
//     ^ defines t.k2
That was o{t}.
//         ^ references t.k2
