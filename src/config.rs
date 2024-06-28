pub struct FormatConfig {
    pub knot_mark_size: usize,
    pub closing_mark: bool,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            knot_mark_size: 3,
            closing_mark: true,
        }
    }
}
