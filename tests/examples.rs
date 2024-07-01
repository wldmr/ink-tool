use ink_fmt::config::FormatConfig;

#[test]
fn the_intercept() {
    let input = std::fs::read_to_string("examples/the_intercept.ink").unwrap();
    let expected = std::fs::read_to_string("examples/the_intercept.fmt.ink").unwrap();
    let output = ink_fmt::format(FormatConfig::default(), input);
    let expected = expected.trim();
    let output = output.trim();
    pretty_assertions::assert_str_eq!(output, expected);
}

#[test]
fn emoji() {
    let input = std::fs::read_to_string("examples/ld41-emoji.ink").unwrap();
    let expected = std::fs::read_to_string("examples/ld41-emoji.fmt.ink").unwrap();
    let output = ink_fmt::format(FormatConfig::default(), input);
    let expected = expected.trim();
    let output = output.trim();
    pretty_assertions::assert_str_eq!(output, expected);
}
