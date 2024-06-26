use ink_fmt::{format, FormatConfig};

mod common;

#[test]
fn test_ink_fmt_md() {
    for (idx, case) in common::get_test_cases("doc/ink-fmt.md").enumerate() {
        let test_name = case.heading.unwrap_or_else(|| format!("Case {}", idx + 1));
        println!("{}", test_name);
        let output = format(FormatConfig::default(), case.input);
        assert_eq!(case.expected_output, output);
    }
}
