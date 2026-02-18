*   -> KKKa
    // ^^^^ references knot:KKKa
*   -> KKKa.SSSa
    // |   |^ references stitch:KKKa.SSSa
    // |   ^ references knot:KKKa
    // |     If the cursor is over the separator, we assume the user means the previous ident.
    // |     In non-modal editors, the cursor will appear to be at the end of the ident
    // |     (that is, left of the separator).
    // ^ references knot:KKKa
*   -> doesnt_exist
    // ^^^^^^^^^^^^ references-nothing

=== KKKa ===
//  ^^^^ defines knot:KKKa

*   -> KKKa
    // ^^^^ references knot:KKKa
*   -> SSSa
    // ^^^^ references stitch:KKKa.SSSa
*   -> KKKa.SSSa
    // |    ^ references stitch:KKKa.SSSa
    // ^ references knot:KKKa

= SSSa
//^^^^ defines stitch:KKKa.SSSa

*   -> KKKa
    // ^^^^ references knot:KKKa
*   -> SSSa
    // ^^^^ references stitch:KKKa.SSSa
*   -> SSSb
    // ^^^^ references stitch:KKKa.SSSb
    // ^^^^ references-not stitch:SSSb
*   -> KKKa.SSSa
    // |    ^ references stitch:KKKa.SSSa
    // ^ references knot:KKKa

= SSSb
//^^^^ defines stitch:KKKa.SSSb

Some text
