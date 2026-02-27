use lsp_types::{CompletionItem, Position, Uri};

use super::*;

impl super::State {
    pub fn completions(
        &self,
        uri: &Uri,
        position: Position,
    ) -> Result<Option<Vec<CompletionItem>>, DocumentNotFound> {
        let (doc, this_doc) = self.get_doc_and_id(uri)?;

        let Some(usage) = doc.usage_at(position) else {
            return Ok(Default::default());
        };

        let ws_names = self.db.workspace_names();
        use ink_document::ids::DefinitionInfo::*;
        use lsp_types::CompletionItemKind;
        Ok(Some(
            ws_names
                .iter()
                .filter(|(key, _)| key.contains(usage.term))
                .flat_map(|(key, metas)| metas.iter().map(move |meta| (key, meta)))
                .filter(|(_, (docid, meta))| {
                    meta.is_global() || (*docid == this_doc && meta.is_locally_visible_at(position))
                })
                .map(|(key, (_, meta))| CompletionItem {
                    label: key.clone(),
                    label_details: None,
                    kind: Some(match meta.id.info() {
                        Section { .. } => CompletionItemKind::MODULE,
                        Subsection { .. } => CompletionItemKind::CLASS,
                        Function => CompletionItemKind::FUNCTION,
                        External => CompletionItemKind::INTERFACE,
                        Var => CompletionItemKind::VARIABLE, // TODO: Differentiate between VAR and CONST
                        Const => CompletionItemKind::CONSTANT, // TODO: Differentiate between VAR and CONST
                        List => CompletionItemKind::ENUM,
                        ListItem { .. } => CompletionItemKind::ENUM_MEMBER,
                        Label => CompletionItemKind::PROPERTY,
                        Param { .. } => CompletionItemKind::VARIABLE,
                        Temp => CompletionItemKind::UNIT,
                    }),
                    // TODO: Fetch actual definition
                    detail: Some(match meta.id.info() {
                        Section { stitch, params } => {
                            format!(
                                "{} {key}{}",
                                if stitch { "=" } else { "==" },
                                if params { "(…)" } else { "" }
                            )
                        }
                        Subsection { params, .. } => {
                            format!("= {key}{}", if params { "(…)" } else { "" })
                        }
                        Function => format!("== function {key}(…)"),
                        External => format!("EXTERNAL {key}(…)"),
                        Var => format!("VAR {key} = …"),
                        Const => format!("CONST {key} = …"),
                        List => format!("LIST {key} = …"),
                        ListItem { .. } => format!("LIST … = … {key}, "),
                        Label => format!("({key}) // label"),
                        Param { .. } => format!("param // parameter"),
                        Temp => format!("~ temp {key} = …"),
                    }),
                    // TODO: Fetch actual docs
                    documentation: None,
                    deprecated: None,
                    preselect: None,
                    sort_text: None,
                    filter_text: None,
                    insert_text: None,
                    insert_text_format: None,
                    insert_text_mode: None,
                    text_edit: Some(lsp_types::CompletionTextEdit::Edit(lsp_types::TextEdit {
                        range: usage.range,
                        new_text: key.to_owned(),
                    })),
                    additional_text_edits: None,
                    command: None,
                    commit_characters: None,
                    data: None,
                    tags: None,
                })
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::{new_state, text_with_caret, uri};

    use itertools::Itertools;
    use pretty_assertions::assert_eq;

    #[test]
    fn state() {
        let mut state = new_state();
        state.edit(
            uri("context.ink"),
            "
                VAR some_var = true

                == one
                text

                == two
                text
            ",
        );
        let (contents, caret) = text_with_caret("{o@}");
        let uri = uri("test.ink");
        state.edit(uri.clone(), contents);
        let completions = state.completions(&uri, caret).unwrap().unwrap();
        assert_eq!(
            completions
                .into_iter()
                .map(|it| it.label)
                .sorted_unstable()
                .collect::<Vec<_>>(),
            vec!["one", "some_var", "two"]
        );
    }
}
