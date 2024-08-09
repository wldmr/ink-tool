use super::format_item::FormatItem;

#[derive(Debug)]
pub struct Formatter(pub(super) Vec<FormatItem>);

impl Formatter {
    pub fn normalize(&mut self) {
        // This merging could be done directly while building the outputs,
        // but taking it in steps will help with debugging.
        let mut original = std::mem::take(&mut self.0).into_iter();

        if let Some(mut accumulator) = original.next() {
            while let Some(output) = original.next() {
                match accumulator.merge(output) {
                    Ok(merged) => accumulator = merged,
                    Err((left, right)) => {
                        self.0.push(left);
                        accumulator = right;
                    }
                }
            }
        }
    }
}

impl ToString for Formatter {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for output in self.0.iter() {
            result.push_str(output.as_str())
        }
        result
    }
}
