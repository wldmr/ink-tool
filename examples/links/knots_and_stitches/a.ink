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
    // ^^^^ references stitch:SSSb
    // ^^^^ references-not stitch:KKKa.SSSb
*   -> KKKa.SSSa
    // ^^^^^^^^^ references stitch:KKKa.SSSa

= SSSb
//^^^^ defines stitch:KKKa.SSSb

Some text
