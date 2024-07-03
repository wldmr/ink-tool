use std::env;
use std::fs;
use std::path::Path;

use rust_format::Formatter;
use type_sitter_gen::generate_nodes;
use type_sitter_gen::tree_sitter;

fn main() {
    let formatter = rust_format::RustFmt::new();
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let userdoc_tests_path = Path::new(&out_dir).join("userdoc.rs");
    println!("cargo::rerun-if-changed=doc/");
    let userdoc_tests: String = fs::read_dir("doc")
        .expect("doc directory should exist")
        .filter_map(|entry| entry.ok().map(|file| file.path()))
        .filter(|path| path.extension().is_some_and(|it| it == "md"))
        .map(|path| userdoc_tests::mod_per_file(&path))
        .collect();
    let userdoc_tests = formatter
        .format_str(userdoc_tests)
        .expect("formatting should work");
    fs::write(&userdoc_tests_path, &userdoc_tests).unwrap();

    let type_sitter_input = "../tree-sitter-ink/src/node-types.json";
    println!("cargo::rerun-if-changed={type_sitter_input}");
    let type_sitter_ink_path = Path::new(&out_dir).join("type_sitter_ink.rs");
    let type_sitter_ink_types = generate_nodes(type_sitter_input, &tree_sitter())
        .map(|nodes| format!("{}", nodes))
        .unwrap();
    let type_sitter_ink_types = formatter.format_str(type_sitter_ink_types).unwrap();
    fs::write(&type_sitter_ink_path, type_sitter_ink_types).unwrap();

    println!("cargo::rerun-if-changed=build.rs");
}

mod userdoc_tests {
    use derive_builder::Builder;
    use markdown;
    use markdown::mdast::Node;
    use markdown::unist::Position;
    use markdown::ParseOptions;
    use std::path::Path;

    pub(crate) fn mod_per_file(path: &Path) -> String {
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
                let function_name: String = test_description.to_lowercase().chars()
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
                    let output = ink_fmt::format(ink_fmt::config::FormatConfig::default(), r#"{input}"#.to_string());
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
    pub(crate) struct TestCase {
        #[builder(setter(into, strip_option), default)]
        pub heading: Option<(Position, u8, String)>,
        pub input: (Position, String),
        pub expected_output: (Position, String),
    }

    pub(crate) fn get_test_cases(path: &Path) -> Vec<TestCase> {
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
}
