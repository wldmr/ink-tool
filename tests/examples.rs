#[test]
fn the_intercept() {
    let input = std::fs::read_to_string("examples/input/the_intercept.ink").unwrap();
    let expected = std::fs::read_to_string("examples/output/the_intercept.ink").unwrap();
    let output = ink_fmt::format(input);
    pretty_assertions::assert_str_eq!(output, expected);
}

#[test]
fn emoji() {
    let input = std::fs::read_to_string("examples/input/ld41-emoji.ink").unwrap();
    let expected = std::fs::read_to_string("examples/output/ld41-emoji.ink").unwrap();
    let output = ink_fmt::format(input);
    pretty_assertions::assert_str_eq!(output, expected);
}
