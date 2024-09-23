use derive_builder::Builder;
use markdown::{mdast::Node, unist::Position, ParseOptions};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("userdoc.rs");
    println!("cargo::rerun-if-changed=doc/");
    let userdoc_tests: String = fs::read_dir("doc")
        .expect("doc directory should exist")
        .filter_map(|entry| entry.ok().map(|file| file.path()))
        .filter(|path| path.extension().is_some_and(|it| it == "md"))
        .map(|path| mod_per_file(&path))
        .collect();
    fs::write(&dest_path, &userdoc_tests).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
}

fn mod_per_file(path: &Path) -> String {
    let mod_name: String = path
        .file_stem()
        .expect("path should be a file with a stem")
        .to_str()
        .expect("filename should be a string")
        .to_lowercase()
        .chars()
        .filter_map(|c| match c {
            'a'..='z' | '0'..='9' => Some(c),
            ' ' | '-' => Some('_'),
            _ => None,
        })
        .collect();
    let tests: String = get_test_cases(path)
        .into_iter()
        .enumerate()
        .map(|(idx, case)| {
            let test_description = case
                .heading
                .map(|(_pos, _level, text)| text)
                .unwrap_or_else(|| format!("Case {}", idx + 1));
            let function_name: String = test_description
                .to_lowercase()
                .chars()
                .filter_map(|c| match c {
                    'a'..='z' | '0'..='9' => Some(c),
                    ' ' | '-' => Some('_'),
                    _ => None,
                })
                .collect();
            format!(
                r###"
                #[test]
                fn test_{idx:0>2}_{function_name}() {{
                    let output = ink_tool::format(r#"{input}"#.to_string());
                    pretty_assertions::assert_str_eq!(
                        output,
                        r#"{expected}"#,
                        r#"{path}:{line}:{column}: {test_description}"#
                    );
                }}
                "###,
                input = case.input.1,
                expected = case.expected_output.1,
                path = path.to_str().unwrap(),
                line = case.input.0.start.line,
                column = case.input.0.start.column,
            )
        })
        .collect();
    format!(
        r#"
        mod {} {{
            {}
        }}
        "#,
        mod_name, tests
    )
}

#[derive(Builder)]
struct TestCase {
    #[builder(setter(into, strip_option), default)]
    pub heading: Option<(Position, u8, String)>,
    pub input: (Position, String),
    pub expected_output: (Position, String),
}

fn get_test_cases(path: &Path) -> Vec<TestCase> {
    let input = std::fs::read_to_string(path).expect(&format!(
        "There should be a file at '{}'",
        path.to_str().unwrap()
    ));
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
    return result;
}
