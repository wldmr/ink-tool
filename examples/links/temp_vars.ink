Temp variables are visible _only_ at the exact level they are defined in.
That means they aren't visible in higher levels, NOR in lower levels.
(That last one is a bit unusual; in other programming languages
<> local variables are visible in enclosed scopes.)

~ temp t = "temporarily"
//     ^ defines toplevel
He was {t} indisposed.
//      ^ references toplevel
//      ^ references-not knot knot.stitch knot.stitch2 k2

=== knot ===
~ temp t = "knot"
//     ^ defines knot
He was {t} amused.
//      ^ references knot
//      ^ references-not toplevel knot.stitch knot.stitch2 k2

= stitch
~ temp t = "stitch"
//     ^ defines knot.stitch
He was in {t}es.
//         ^ references knot.stitch
//         ^ references-not toplevel knot knot.stitch2 k2

= stitch_2
~ temp t = 2
//     ^ defines knot.stitch2
//     ^ references-not toplevel knot knot.stitch k2
It was {t} hot.
//      ^ references knot.stitch2

=== k2 ===
~ temp t = "k2"
//     ^ defines k2
That was o{t}.
//         ^ references k2
//         ^ references-not toplevel knot knot.stitch knot.stitch2
