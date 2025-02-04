In Ink, there actually is no shadowing as such.
Even params don't override global names. It's pretty much the opposite: Name clashes with VARs don't compile, and CONSTs override params of the same name.
A value behind a name depends on what it was last set to, not where it was defined.
Incredibly, even `temp`s with the same name can interefere with anything of the same name, even temps in other knots/stitches. It's nuts!

The rules for clashing names are:

1. All names share the the same namespace
2. If there is at least one clashing VAR, it doesn't compile.
3. If there is a clashing CONST, the CONST value is used everywhere.
4. Parameters and temps freely overwrite each other, globally. Whichever param/temp was last visited wins.

That said, ink-tool treats all params as local (visible in sub-scopes, and shadowable by sub-scopes) and temps as very-local (not visible in sub-scopes), like you would expect from a "normal" programming language.
Non-local surprises can be pointed out by diagnostics.

CONST name = "default"
//    ^^^^ defines global

My {name} is default.
//  ^^^^ references global
//  ^^^^ references-not param param2

=== knot(name) ===
//       ^^^^ defines param

- (bypass)

My {name} is good, and what that says depends on how we got here.
//  ^^^^ references param global
//  ^^^^ references-not param2

= stitch(name)
//       ^^^^ defines param2

- (bypass2)

My {name} is confusing, and what that says depends on how we got here.
//  ^^^^ references param2 param global
