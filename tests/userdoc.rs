use ink_fmt::{format, FormatConfig};
use pretty_assertions::assert_str_eq;

mod common;

#[test]
fn test_ink_fmt_md() {
    let file = "doc/ink-fmt.md";
    for (idx, case) in common::get_test_cases(file).enumerate() {
        let test_name = case
            .heading
            .map(|(_pos, _level, text)| text)
            .unwrap_or_else(|| format!("Case {}", idx + 1));
        let output = format(FormatConfig::default(), case.input.1);
        assert_str_eq!(
            output,
            case.expected_output.1,
            "{}: {} (input:{} output:{})",
            file,
            test_name,
            case.input.0.start.line,
            case.expected_output.0.start.line,
        );
    }
}
