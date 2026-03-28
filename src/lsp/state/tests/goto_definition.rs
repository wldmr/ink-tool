use std::collections::HashMap;

use assert2::check;
use indoc::indoc;
use lsp_types::{Location, Position, Range};

use crate::lsp::{
    location::TextPos,
    salsa::InkGetters,
    state::tests::{new_state, uri},
};

#[test]
fn can_navigate_to_resolved_imports_from_anywhere_on_the_import_line() {
    let main = indoc! {"
        INCLUDE greet.ink
        //          ^ on path
        // ^ before path
        "};
    let mut state = new_state();
    state.edit(uri("main.ink"), main);
    state.edit(uri("greet.ink"), "Hi");

    let locations: HashMap<&str, Range> = text_annotations::scan_default_annotations(main)
        .map(|it| (it.claim(), it.text_location.into()))
        .collect();

    let expected = Ok(vec![Location::new(uri("greet.ink"), Range::default())]);

    let location_name = "before path";
    let pos = locations[location_name].start;
    check!(
        state.goto_definition(uri("main.ink"), pos) == expected,
        "{location_name} {}",
        TextPos::from(pos)
    );

    let location_name = "on path";
    let pos = locations[location_name].start;
    check!(
        state.goto_definition(uri("main.ink"), pos) == expected,
        "{location_name} {}",
        TextPos::from(pos)
    );
}

#[test]
fn can_not_navigate_to_unresolved_imports() {
    let mut state = new_state().with_comment_separated_files(indoc! {"
        // file: start.ink
        INCLUDE phantom.ink
        "});

    let over_import = Position::new(0, 8);
    let defs = state.goto_definition(uri("start.ink"), over_import);
    check!(defs == Ok(Vec::new()), "phantom.ink doesn't exist yet.");

    state.edit(uri("phantom.ink"), "Boo!");

    let defs = state.goto_definition(uri("start.ink"), over_import);
    check!(defs == Ok(vec![Location::new(uri("phantom.ink"), Range::default())]));
}
