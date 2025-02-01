LIST foo = bar, baz
//              ^^^ defines foo.baz
//         ^^^ defines foo.bar
//   ^^^ defines foo

LIST pass = (bar), (buck = 3)
//                  ^^^^ defines pass.buck
//           ^^^ defines pass.bar
//   ^^^^ defines pass

VAR bar = baz
//        ^^^ references foo.baz
//            (this is an error in inklecate, but we allow it in the LSP)
//  ^^^ defines var:bar

* {foo has bar} Foo has many bars!
//         ^^^ references foo.bar
//         ^^^ references pass.bar
//         ^^^ references var:bar
// ^^^ references foo

* {pass ? (buck, pass.bar, foo.bar)} Nice and unambiguous.
//                         ^^^^^^^ references foo.bar
//                         ^^^^^^^ references-not pass.bar
//               ^^^^^^^^ references pass.bar
//               ^^^^^^^^ references-not foo.bar
//         ^^^^ references pass.buck
// ^^^^ references pass

