LIST foo = bar, baz
//              ^^^ defines lists:foo.baz
//         ^^^ defines lists:foo.bar
//   ^^^ defines lists:foo

LIST pass = (bar), (buck = 3)
//                  ^^^^ defines lists:pass.buck
//           ^^^ defines lists:pass.bar
//   ^^^^ defines lists:pass

VAR bar = baz
//        ^^^ references lists:foo.baz
//            (this is an error in inklecate, but we allow it in the LSP)
//  ^^^ defines lists:var.bar

* {foo has bar} Foo has many bars!
//         ^^^ references lists:foo.bar lists:pass.bar lists:var.bar
// ^^^ references lists:foo

* {pass ? (buck, pass.bar, foo.bar)} Nice and unambiguous.
//                             ^^^ references lists:foo.bar
//                         ^^^ references lists:foo
//                    ^^^ references lists:pass.bar
//               ^^^^ references lists:pass
//         ^^^^ references lists:pass.buck
// ^^^^ references lists:pass

