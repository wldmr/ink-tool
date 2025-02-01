*   -> KKKa
    // ^^^^ references knot:KKKa
*   -> KKKa.SSSa
    // ^^^^^^^^^ references stitch:KKKa.SSSa
*   -> doesnt_exist
    // ^^^^^^^^^^^^ references-nothing

=== KKKa ===
//  ^^^^ defines knot:KKKa

*   -> KKKa
    // ^^^^ references knot:KKKa
*   -> SSSa
    // ^^^^ references stitch:KKKa.SSSa
*   -> KKKa.SSSa
    // ^^^^^^^^^ references stitch:KKKa.SSSa

= SSSa
//^^^^ defines stitch:KKKa.SSSa

*   -> KKKa
    // ^^^^ references knot:KKKa
*   -> SSSa
    // ^^^^ references stitch:KKKa.SSSa
*   -> SSSb
    // ^^^^ references stitch:KKKa.SSSb
    // ^^^^ references-not stitch:SSSb
    //      (because while the global stitch name exists, it is shadowed by the local one)
*   -> KKKa.SSSa
    // ^^^^^^^^^ references stitch:KKKa.SSSa

= SSSb
//^^^^ defines stitch:KKKa.SSSb

Some text
