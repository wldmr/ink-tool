*   -> KKKa
    // ^^^^ references knot:KKKa
*   -> KKKa.SSSa
    // |    ^ references stitch:KKKa.SSSa
    //     ^ references stitch:KKKa.SSSa
    //       (if the cursor is over the dot, we assume the user means the next ident (since we prefer maximum specificity))
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
