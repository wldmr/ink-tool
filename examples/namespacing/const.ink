/*
In Ink, there actually is no shadowing as such. Even params don’t override
global names. It’s pretty much the opposite: Name clashes with VARs don’t
compile, and CONSTs override params of the same name. A value behind a name
depends on what it was last set to, not where it was defined. Incredibly, even
`temp`s with the same name can interefere with anything of the same name, even
temps in other knots/stitches. It’s nuts!

The rules for clashing names are:

1.  All names share the the same namespace
2.  If there is at least one clashing VAR, it doesn’t compile.
3.  If there is a clashing CONST, the CONST value is used everywhere.
4.  Parameters and temps freely overwrite each other, globally. Whichever
    param/temp was last visited wins. UNLESS you return from tunnels using
    `->->`, because tunnels actually take care to be “hygienic” that way. But
    simply navigating by `->` will clober your environment.

The way ink-tool deals with this is “not really” ;). If a local usage would
conflict with a VAR/CONST, the navigation prefers the local definition. Any
trouble is relegated to the diagnostics.
*/

* [Bypass 1] -> knot.bypass
* [Bypass 2] -> knot.bypass2
* [Knot] -> knot("Slim Shady")
* [Knot stitch] -> knot.stitch("Lieutenant Frank Drebin, Detective Lieutenant, Police Squad")

CONST name = "Inigo Montoya"
//    ^^^^ defines const:global

My name is {name}.
//          ^^^^ references const:global

=== knot(name) ===
//       ^^^^ defines const:knot
//       ^^^^ diagnostic clashes with CONST

- (bypass)

My name is {name}.
//          ^^^^ references const:knot

-> END

= stitch(name)
//       ^^^^ defines const:stitch
//       ^^^^ diagnostic clashes with CONST

- (bypass2)

My name is {name}.
//          ^^^^ references const:stitch

-> END


/* TEST

1: Bypass 1
2: Bypass 2
3: Knot
4: Knot stitch
?> 1
My name is Inigo Montoya.
*/

/* TEST

1: Bypass 1
2: Bypass 2
3: Knot
4: Knot stitch
?> 2
My name is Inigo Montoya.
*/

/* TEST

1: Bypass 1
2: Bypass 2
3: Knot
4: Knot stitch
?> 3
My name is Inigo Montoya.
*/

/* TEST

1: Bypass 1
2: Bypass 2
3: Knot
4: Knot stitch
?> 4
My name is Inigo Montoya.
*/
