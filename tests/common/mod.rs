use std::fmt::Pointer;

use derive_builder::Builder;
use markdown::{
    mdast::Node,
    unist::{Point, Position},
    ParseOptions,
};

#[derive(Builder)]
pub(super) struct TestCase {
    #[builder(setter(into, strip_option), default)]
    pub heading: Option<(Position, u8, String)>,
    pub input: (Position, String),
    pub expected_output: (Position, String),
}

pub(super) fn get_test_cases(path: &str) -> impl Iterator<Item = TestCase> {
    let input = std::fs::read_to_string(path).expect("Tests should be in a file that exists");
    let root = markdown::to_mdast(&input, &ParseOptions::gfm()).unwrap();
    let children = root.children().expect("Document should not be empty.");
    let mut current_case = TestCaseBuilder::create_empty();
    let mut result = Vec::new();
    for node in children {
        let position = node
            .position()
            .expect("We found it, there should be a position");
        if let Node::Code(code) = node {
            if let Some("ink") = code.lang.as_deref() {
                let meta = code.meta.as_deref();

                if let Some("input") = meta {
                    current_case.input((position.clone(), code.value.clone()));
                } else if let Some("output") = meta {
                    current_case.expected_output((position.clone(), code.value.clone()));
                    result.push(current_case.build().unwrap());
                }
            }
        } else if let Node::Heading(heading) = node {
            let text = &input[position.start.offset..position.end.offset];
            let text = text.trim_start_matches(['#', ' ']);
            current_case.heading((position.clone(), heading.depth, text.to_owned()));
        }
    }
    return result.into_iter();
}
