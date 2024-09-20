use crate::{formatting::Formatting, util::constrained_value::Constrained};

/// Write formatting instructions to a string.
impl Formatting for &mut String {
    fn indent(&mut self) {}

    fn dedent(&mut self) {}

    fn align_indent_to_current_column(&mut self) {}

    fn space(&mut self, repeats: impl Into<Constrained>) {
        let repeats: Constrained = repeats.into();
        for _ in 0..repeats.value() {
            self.push(' ')
        }
    }

    fn line(&mut self, repeats: impl Into<Constrained>) {
        let repeats: Constrained = repeats.into();
        for _ in 0..repeats.value() {
            self.push('\n')
        }
    }

    fn text(&mut self, s: &str) {
        self.push_str(s)
    }
}
