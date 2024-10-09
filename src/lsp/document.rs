pub struct Document {
    tree: tree_sitter::Tree,
    text: String,
}

type DocumentEdit = (Option<lsp_types::Range>, String);

/// Public API
impl Document {
    pub(crate) fn new(text: String, parser: &mut tree_sitter::Parser) -> Self {
        let tree = parser
            .parse(&text, None)
            .expect("parsing should always work");
        Self { tree, text }
    }

    pub fn apply_edits(&mut self, edits: Vec<DocumentEdit>, parser: &mut tree_sitter::Parser) {
        eprintln!("{} edits to apply", edits.len());
        for (range, new_text) in edits.into_iter() {
            let edit = if let Some(range) = range {
                self.edit_range(range, &new_text)
            } else {
                self.edit_whole_document(&new_text)
            };
            self.tree.edit(&edit);
            self.text
                .replace_range(edit.start_byte..edit.old_end_byte, &new_text);
            self.tree = parser
                .parse(&self.text, Some(&self.tree))
                .expect("parsing must work");
        }
    }
}

/// Private API
impl Document {
    fn edit_range(&self, range: lsp_types::Range, new_text: &str) -> tree_sitter::InputEdit {
        let start_byte = self
            .byte_for(range.start)
            .expect("range must be within document");
        let old_end_byte = self
            .byte_for(range.end)
            .expect("range must be within document");
        let new_end_byte = start_byte + new_text.bytes().len();

        let start_position = point(&range.start);
        let old_end_position = point(&range.end);
        let mut new_end_position = start_position.clone();
        for char in new_text.chars() {
            if char == '\n' {
                new_end_position.row += 1;
                new_end_position.column = 0;
            } else {
                new_end_position.column += 1;
            }
        }

        tree_sitter::InputEdit {
            start_byte,
            old_end_byte,
            new_end_byte,
            start_position,
            old_end_position,
            new_end_position,
        }
    }

    fn edit_whole_document(&self, new_text: &str) -> tree_sitter::InputEdit {
        tree_sitter::InputEdit {
            start_byte: 0,
            old_end_byte: self.text.len(),
            new_end_byte: new_text.len(),
            start_position: tree_sitter::Point::new(0, 0),
            old_end_position: tree_sitter::Point::new(0, 0),
            new_end_position: tree_sitter::Point::new(0, 0),
        }
    }

    fn byte_for(&self, pos: lsp_types::Position) -> Option<usize> {
        let lsp_types::Position {
            mut line,
            mut character,
        } = pos;
        // Exceptionally stupid way to do it. TODO: Make more smart!
        let mut prev_idx = 0;
        for (idx, char) in self.text.char_indices() {
            let width = idx - prev_idx;
            // eprintln!("{idx} - {line}:{character}");
            prev_idx = idx;
            if line > 0 {
                if char == '\n' {
                    line -= 1
                }
            } else {
                if character == 0 {
                    return Some(idx);
                } else {
                    character -= width as u32;
                }
            }
        }

        None
    }
}

fn point(pos: &lsp_types::Position) -> tree_sitter::Point {
    tree_sitter::Point {
        row: pos.line as usize,
        column: pos.character as usize,
    }
}
