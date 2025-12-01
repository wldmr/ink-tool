There can be multiple definions of the same name.

This won't compile in most cases, but the LSP accepts it so that you can navigate between the different definitons.

VAR yo = true
//  ^^ defines var.yo
CONST yo = false
//    ^^ defines const.yo

This much is {yo}, he said ambiguously.
//            ^^ references var.yo
//            ^^ references const.yo
//            ^^ references a.stitch.yo
//            ^^ references b.knot.yo

 = yo
// ^^ defines a.stitch.yo
He was unique, he thought!
