INCLUDE b.ink

There can be multiple definions of the same name.

This won't compile in most cases, but the LSP accepts it so that you can navigate between the different definitons.

VAR yo = true
//  ^^ defines var.yo
//  ^^ diagnostic Multiple definitions
CONST yo = false
//    ^^ defines const.yo
//    ^^ diagnostic Multiple definitions

This much is {yo}, he said ambiguously.
//            ^^ references var.yo const.yo a.stitch.yo b.knot.yo

 = yo
// ^^ defines a.stitch.yo
// ^^ diagnostic Multiple definitions
He was unique, he thought!
