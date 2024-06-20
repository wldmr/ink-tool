#[doc = "Typed node `alt_arm`\n\nThis node has children: `{choice_block | code | content | external | global | include | list | todo_comment}*`:\n- [ChoiceBlock]\n- [Code]\n- [Content]\n- [External]\n- [Global]\n- [Include]\n- [List]\n- [TodoComment]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AltArm<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AltArm<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment<
                    'tree,
                >,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment<
                    'tree,
                >,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment<
                    'tree,
                >,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment<
                    'tree,
                >,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AltArm<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "alt_arm" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AltArm<'tree> {
    const KIND: &'static str = "alt_arm";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `alternatives`\n\nThis node has children: `{alternatives | cond_block | conditional_text | content | eval | glue | multiline_alternatives | text}*`:\n- [Alternatives]\n- [CondBlock]\n- [ConditionalText]\n- [Content]\n- [Eval]\n- [Glue]\n- [MultilineAlternatives]\n- [Text]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Alternatives<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Alternatives<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Alternatives_CondBlock_ConditionalText_Content_Eval_Glue_MultilineAlternatives_Text < 'tree > > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Alternatives_CondBlock_ConditionalText_Content_Eval_Glue_MultilineAlternatives_Text < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Alternatives_CondBlock_ConditionalText_Content_Eval_Glue_MultilineAlternatives_Text < 'tree > > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Alternatives_CondBlock_ConditionalText_Content_Eval_Glue_MultilineAlternatives_Text < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Alternatives<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "alternatives" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Alternatives<'tree> {
    const KIND: &'static str = "alternatives";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `args`\n\nThis node has children: `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}+`:\n- [Binary]\n- [Boolean]\n- [Call]\n- [Divert]\n- [Identifier]\n- [ListValues]\n- [Number]\n- [Paren]\n- [Postfix]\n- [QualifiedName]\n- [String]\n- [Unary]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Args<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Args<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Args<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "args" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Args<'tree> {
    const KIND: &'static str = "args";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `assignment`\n\nThis node has these fields:\n- `name`: `identifier` ([Identifier])\n- `op`: `{+= | -= | =}` ([anon_unions::Add_Eq__Sub_Eq__Eq_])\n- `value`: `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}` ([anon_unions::Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Assignment<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Assignment<'tree> {
    #[doc = "Get the field `name` which has kind `identifier` ([Identifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `op` which has kind `{+= | -= | =}` ([anon_unions::Add_Eq__Sub_Eq__Eq_])"]
    #[allow(dead_code)]
    #[inline]
    pub fn op(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::Add_Eq__Sub_Eq__Eq_<'tree>> {
        self . 0 . child_by_field_name ("op") . map (< anon_unions :: Add_Eq__Sub_Eq__Eq_ < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `value` which has kind `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}` ([anon_unions::Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary])"]
    #[allow(dead_code)]
    #[inline]    pub fn value (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > >{
        self . 0 . child_by_field_name ("value") . map (< anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Assignment<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "assignment" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Assignment<'tree> {
    const KIND: &'static str = "assignment";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `binary`\n\nThis node has these fields:\n- `op`: `{!= | !? | % | && | * | + | - | / | < | <= | == | > | >= | ? | ^ | and | has | hasnt | mod | or | ||}` ([anon_unions::Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_])\n\nAnd additional children: `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}+`:\n- [Binary]\n- [Boolean]\n- [Call]\n- [Divert]\n- [Identifier]\n- [ListValues]\n- [Number]\n- [Paren]\n- [Postfix]\n- [QualifiedName]\n- [String]\n- [Unary]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Binary<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Binary<'tree> {
    #[doc = "Get the field `op` which has kind `{!= | !? | % | && | * | + | - | / | < | <= | == | > | >= | ? | ^ | and | has | hasnt | mod | or | ||}` ([anon_unions::Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_])"]
    #[allow(dead_code)]
    #[inline]    pub fn op (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_ < 'tree > >{
        self . 0 . child_by_field_name ("op") . map (< anon_unions :: Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_ < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_ < 'tree > > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_ < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_ < 'tree > > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_ < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Binary<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "binary" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Binary<'tree> {
    const KIND: &'static str = "binary";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `boolean`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Boolean<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Boolean<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Boolean<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "boolean" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Boolean<'tree> {
    const KIND: &'static str = "boolean";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `call`\n\nThis node has these fields:\n- `args`: `args?` ([Args])\n- `name`: `{identifier | qualified_name}` ([anon_unions::Identifier_QualifiedName])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Call<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Call<'tree> {
    #[doc = "Get the field `args` which has kind `args?` ([Args])"]
    #[allow(dead_code)]
    #[inline]
    pub fn args(&self) -> Option<type_sitter_lib::NodeResult<'tree, Args<'tree>>> {
        self.0
            .child_by_field_name("args")
            .map(<Args<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `name` which has kind `{identifier | qualified_name}` ([anon_unions::Identifier_QualifiedName])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::Identifier_QualifiedName<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< anon_unions :: Identifier_QualifiedName < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Call<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "call" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Call<'tree> {
    const KIND: &'static str = "call";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `choice`\n\nThis node has these fields:\n- `condition`: `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary | { | }}*` ([anon_unions::Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_LBrace__RBrace_])\n- `final`: `content?` ([Content])\n- `label`: `{( | ) | identifier}*` ([anon_unions::LParen__RParen__Identifier])\n- `main`: `content?` ([Content])\n- `temporary`: `content?` ([Content])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Choice<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Choice<'tree> {
    #[doc = "Get the field `condition` which has kind `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary | { | }}*` ([anon_unions::Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_LBrace__RBrace_])"]
    #[allow(dead_code)]
    #[inline]    pub fn conditions < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl Iterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_LBrace__RBrace_ < 'tree > > >> + 'a{
        self . 0 . children_by_field_name ("condition" , c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_LBrace__RBrace_ < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the field `final` which has kind `content?` ([Content])"]
    #[allow(dead_code)]
    #[inline]
    pub fn r#final(&self) -> Option<type_sitter_lib::NodeResult<'tree, Content<'tree>>> {
        self.0
            .child_by_field_name("final")
            .map(<Content<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `label` which has kind `{( | ) | identifier}*` ([anon_unions::LParen__RParen__Identifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn labels<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::LParen__RParen__Identifier<'tree>>,
        >,
    > + 'a {
        self . 0 . children_by_field_name ("label" , c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: LParen__RParen__Identifier < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the field `main` which has kind `content?` ([Content])"]
    #[allow(dead_code)]
    #[inline]
    pub fn main(&self) -> Option<type_sitter_lib::NodeResult<'tree, Content<'tree>>> {
        self.0
            .child_by_field_name("main")
            .map(<Content<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `temporary` which has kind `content?` ([Content])"]
    #[allow(dead_code)]
    #[inline]
    pub fn temporary(&self) -> Option<type_sitter_lib::NodeResult<'tree, Content<'tree>>> {
        self.0
            .child_by_field_name("temporary")
            .map(<Content<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Choice<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "choice" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Choice<'tree> {
    const KIND: &'static str = "choice";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `choice_block`\n\nThis node has children: `{choice | choice_block | code | comment | content | external | gather_block | global | include | list | todo_comment}+`:\n- [Choice]\n- [ChoiceBlock]\n- [Code]\n- [Comment]\n- [Content]\n- [External]\n- [GatherBlock]\n- [Global]\n- [Include]\n- [List]\n- [TodoComment]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ChoiceBlock<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ChoiceBlock<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Choice_ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment < 'tree > > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Choice_ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Choice_ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment < 'tree > > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Choice_ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ChoiceBlock<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "choice_block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ChoiceBlock<'tree> {
    const KIND: &'static str = "choice_block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `code`\n\nThis node has a child: `{assignment | binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | return | string | temp | unary}`:\n- [Assignment]\n- [Binary]\n- [Boolean]\n- [Call]\n- [Divert]\n- [Identifier]\n- [ListValues]\n- [Number]\n- [Paren]\n- [Postfix]\n- [QualifiedName]\n- [Return]\n- [String]\n- [Temp]\n- [Unary]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Code<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Code<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: Assignment_Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_Return_String_Temp_Unary < 'tree > >{
        self . 0 . named_child (0) . map (< anon_unions :: Assignment_Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_Return_String_Temp_Unary < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Code<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "code" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Code<'tree> {
    const KIND: &'static str = "code";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `cond_arm`\n\nThis node has these fields:\n- `condition`: `{binary | boolean | call | divert | else | identifier | list_values | number | paren | postfix | qualified_name | string | unary}` ([anon_unions::Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary])\n\nAnd additional children: `{choice_block | code | content | external | global | include | list | todo_comment}*`:\n- [ChoiceBlock]\n- [Code]\n- [Content]\n- [External]\n- [Global]\n- [Include]\n- [List]\n- [TodoComment]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct CondArm<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> CondArm<'tree> {
    #[doc = "Get the field `condition` which has kind `{binary | boolean | call | divert | else | identifier | list_values | number | paren | postfix | qualified_name | string | unary}` ([anon_unions::Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary])"]
    #[allow(dead_code)]
    #[inline]    pub fn condition (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > >{
        self . 0 . child_by_field_name ("condition") . map (< anon_unions :: Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment_Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment_Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment_Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment_Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for CondArm<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "cond_arm" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for CondArm<'tree> {
    const KIND: &'static str = "cond_arm";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `cond_block`\n\nThis node has children: `cond_arm*` ([CondArm])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct CondBlock<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> CondBlock<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, CondArm<'tree>>>,
    > + 'a {
        self.0
            .named_children(c)
            .map(|n| <type_sitter_lib::ExtraOr<'tree, CondArm<'tree>> as TryFrom<_>>::try_from(n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, CondArm<'tree>>>>
    {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, CondArm<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for CondBlock<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "cond_block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for CondBlock<'tree> {
    const KIND: &'static str = "cond_block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `conditional_text`\n\nThis node has these fields:\n- `condition`: `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}` ([anon_unions::Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary])\n\nAnd additional children: `content+` ([Content])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ConditionalText<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ConditionalText<'tree> {
    #[doc = "Get the field `condition` which has kind `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}` ([anon_unions::Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary])"]
    #[allow(dead_code)]
    #[inline]    pub fn condition (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > >{
        self . 0 . child_by_field_name ("condition") . map (< anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Content_Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Content_Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Content_Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Content_Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ConditionalText<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "conditional_text" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ConditionalText<'tree> {
    const KIND: &'static str = "conditional_text";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `content`\n\nThis node has children: `{alternatives | cond_block | conditional_text | divert | eval | glue | multiline_alternatives | tag | text | thread | tunnel}+`:\n- [Alternatives]\n- [CondBlock]\n- [ConditionalText]\n- [Divert]\n- [Eval]\n- [Glue]\n- [MultilineAlternatives]\n- [Tag]\n- [Text]\n- [Thread]\n- [Tunnel]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Content<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Content<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Alternatives_CondBlock_ConditionalText_Divert_Eval_Glue_MultilineAlternatives_Tag_Text_Thread_Tunnel < 'tree > > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Alternatives_CondBlock_ConditionalText_Divert_Eval_Glue_MultilineAlternatives_Tag_Text_Thread_Tunnel < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Alternatives_CondBlock_ConditionalText_Divert_Eval_Glue_MultilineAlternatives_Tag_Text_Thread_Tunnel < 'tree > > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Alternatives_CondBlock_ConditionalText_Divert_Eval_Glue_MultilineAlternatives_Tag_Text_Thread_Tunnel < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Content<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "content" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Content<'tree> {
    const KIND: &'static str = "content";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `content_block`\n\nThis node has children: `{choice_block | code | comment | content | external | gather_block | global | include | list | todo_comment}+`:\n- [ChoiceBlock]\n- [Code]\n- [Comment]\n- [Content]\n- [External]\n- [GatherBlock]\n- [Global]\n- [Include]\n- [List]\n- [TodoComment]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ContentBlock<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ContentBlock<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment < 'tree > > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment < 'tree > > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ContentBlock<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "content_block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ContentBlock<'tree> {
    const KIND: &'static str = "content_block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `divert`\n\nThis node has these fields:\n- `target`: `{call | identifier | qualified_name}` ([anon_unions::Call_Identifier_QualifiedName])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Divert<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Divert<'tree> {
    #[doc = "Get the field `target` which has kind `{call | identifier | qualified_name}` ([anon_unions::Call_Identifier_QualifiedName])"]
    #[allow(dead_code)]
    #[inline]
    pub fn target(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::Call_Identifier_QualifiedName<'tree>> {
        self . 0 . child_by_field_name ("target") . map (< anon_unions :: Call_Identifier_QualifiedName < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Divert<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "divert" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Divert<'tree> {
    const KIND: &'static str = "divert";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `eval`\n\nThis node has a child: `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}`:\n- [Binary]\n- [Boolean]\n- [Call]\n- [Divert]\n- [Identifier]\n- [ListValues]\n- [Number]\n- [Paren]\n- [Postfix]\n- [QualifiedName]\n- [String]\n- [Unary]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Eval<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Eval<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > >{
        self . 0 . named_child (0) . map (< anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Eval<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "eval" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Eval<'tree> {
    const KIND: &'static str = "eval";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `external`\n\nThis node has these fields:\n- `name`: `identifier` ([Identifier])\n- `params`: `{( | ) | params}+` ([anon_unions::LParen__RParen__Params])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct External<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> External<'tree> {
    #[doc = "Get the field `name` which has kind `identifier` ([Identifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `params` which has kind `{( | ) | params}+` ([anon_unions::LParen__RParen__Params])"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn paramss<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::LParen__RParen__Params<'tree>>,
        >,
    > + 'a {
        self . 0 . children_by_field_name ("params" , c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: LParen__RParen__Params < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for External<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "external" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for External<'tree> {
    const KIND: &'static str = "external";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `gather`\n\nThis node has these fields:\n- `label`: `{( | ) | identifier}*` ([anon_unions::LParen__RParen__Identifier])\n\nAnd additional children: `{content | divert | thread | tunnel}*`:\n- [Content]\n- [Divert]\n- [Thread]\n- [Tunnel]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Gather<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Gather<'tree> {
    #[doc = "Get the field `label` which has kind `{( | ) | identifier}*` ([anon_unions::LParen__RParen__Identifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn labels<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::LParen__RParen__Identifier<'tree>>,
        >,
    > + 'a {
        self . 0 . children_by_field_name ("label" , c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: LParen__RParen__Identifier < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Content_Divert_Thread_Tunnel_LParen__RParen__Identifier<'tree>,
            >,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Content_Divert_Thread_Tunnel_LParen__RParen__Identifier<'tree>,
            > as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Content_Divert_Thread_Tunnel_LParen__RParen__Identifier<'tree>,
            >,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<
                'tree,
                anon_unions::Content_Divert_Thread_Tunnel_LParen__RParen__Identifier<'tree>,
            > as TryFrom<_>>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Gather<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "gather" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Gather<'tree> {
    const KIND: &'static str = "gather";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `gather_block`\n\nThis node has children: `{choice_block | code | comment | content | external | gather | gather_block | global | include | list | todo_comment}+`:\n- [ChoiceBlock]\n- [Code]\n- [Comment]\n- [Content]\n- [External]\n- [Gather]\n- [GatherBlock]\n- [Global]\n- [Include]\n- [List]\n- [TodoComment]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct GatherBlock<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> GatherBlock<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ChoiceBlock_Code_Comment_Content_External_Gather_GatherBlock_Global_Include_List_TodoComment < 'tree > > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ChoiceBlock_Code_Comment_Content_External_Gather_GatherBlock_Global_Include_List_TodoComment < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ChoiceBlock_Code_Comment_Content_External_Gather_GatherBlock_Global_Include_List_TodoComment < 'tree > > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ChoiceBlock_Code_Comment_Content_External_Gather_GatherBlock_Global_Include_List_TodoComment < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for GatherBlock<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "gather_block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for GatherBlock<'tree> {
    const KIND: &'static str = "gather_block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `global`\n\nThis node has these fields:\n- `name`: `identifier` ([Identifier])\n- `op`: `=` ([symbols::Eq_])\n- `type`: `{CONST | VAR}` ([anon_unions::Const_Var])\n- `value`: `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}` ([anon_unions::Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Global<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Global<'tree> {
    #[doc = "Get the field `name` which has kind `identifier` ([Identifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `op` which has kind `=` ([symbols::Eq_])"]
    #[allow(dead_code)]
    #[inline]
    pub fn op(&self) -> type_sitter_lib::NodeResult<'tree, symbols::Eq_<'tree>> {
        self . 0 . child_by_field_name ("op") . map (< symbols :: Eq_ < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `type` which has kind `{CONST | VAR}` ([anon_unions::Const_Var])"]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, anon_unions::Const_Var<'tree>> {
        self . 0 . child_by_field_name ("type") . map (< anon_unions :: Const_Var < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `value` which has kind `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}` ([anon_unions::Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary])"]
    #[allow(dead_code)]
    #[inline]    pub fn value (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > >{
        self . 0 . child_by_field_name ("value") . map (< anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Global<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "global" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Global<'tree> {
    const KIND: &'static str = "global";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `identifier`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Identifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Identifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Identifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Identifier<'tree> {
    const KIND: &'static str = "identifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `include`\n\nThis node has a child: `path` ([Path])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Include<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Include<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Path<'tree>> {
        self . 0 . named_child (0) . map (< Path < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Include<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "include" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Include<'tree> {
    const KIND: &'static str = "include";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `ink`\n\nThis node has children: `{content_block | knot_block | stitch_block}*`:\n- [ContentBlock]\n- [KnotBlock]\n- [StitchBlock]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Ink<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Ink<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::ContentBlock_KnotBlock_StitchBlock<'tree>>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ContentBlock_KnotBlock_StitchBlock < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::ContentBlock_KnotBlock_StitchBlock<'tree>>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ContentBlock_KnotBlock_StitchBlock < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Ink<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "ink" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Ink<'tree> {
    const KIND: &'static str = "ink";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `knot`\n\nThis node has these fields:\n- `end_mark`: `==?` ([symbols::Eq_Eq_])\n- `function`: `function?` ([unnamed::Function])\n- `name`: `identifier` ([Identifier])\n- `params`: `{( | ) | params}*` ([anon_unions::LParen__RParen__Params])\n- `start_mark`: `==` ([symbols::Eq_Eq_])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Knot<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Knot<'tree> {
    #[doc = "Get the field `end_mark` which has kind `==?` ([symbols::Eq_Eq_])"]
    #[allow(dead_code)]
    #[inline]
    pub fn end_mark(&self) -> Option<type_sitter_lib::NodeResult<'tree, symbols::Eq_Eq_<'tree>>> {
        self.0
            .child_by_field_name("end_mark")
            .map(<symbols::Eq_Eq_<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `function` which has kind `function?` ([unnamed::Function])"]
    #[allow(dead_code)]
    #[inline]
    pub fn function(&self) -> Option<type_sitter_lib::NodeResult<'tree, unnamed::Function<'tree>>> {
        self.0
            .child_by_field_name("function")
            .map(<unnamed::Function<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `name` which has kind `identifier` ([Identifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `params` which has kind `{( | ) | params}*` ([anon_unions::LParen__RParen__Params])"]
    #[allow(dead_code)]
    #[inline]
    pub fn paramss<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::LParen__RParen__Params<'tree>>,
        >,
    > + 'a {
        self . 0 . children_by_field_name ("params" , c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: LParen__RParen__Params < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the field `start_mark` which has kind `==` ([symbols::Eq_Eq_])"]
    #[allow(dead_code)]
    #[inline]
    pub fn start_mark(&self) -> type_sitter_lib::NodeResult<'tree, symbols::Eq_Eq_<'tree>> {
        self . 0 . child_by_field_name ("start_mark") . map (< symbols :: Eq_Eq_ < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Knot<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "knot" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Knot<'tree> {
    const KIND: &'static str = "knot";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `knot_block`\n\nThis node has children: `{content_block | knot | stitch_block}+`:\n- [ContentBlock]\n- [Knot]\n- [StitchBlock]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct KnotBlock<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> KnotBlock<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::ContentBlock_Knot_StitchBlock<'tree>>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ContentBlock_Knot_StitchBlock < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::ContentBlock_Knot_StitchBlock<'tree>>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: ContentBlock_Knot_StitchBlock < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for KnotBlock<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "knot_block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for KnotBlock<'tree> {
    const KIND: &'static str = "knot_block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `list`\n\nThis node has these fields:\n- `name`: `identifier` ([Identifier])\n- `op`: `=` ([symbols::Eq_])\n- `values`: `list_value_defs` ([ListValueDefs])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct List<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> List<'tree> {
    #[doc = "Get the field `name` which has kind `identifier` ([Identifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `op` which has kind `=` ([symbols::Eq_])"]
    #[allow(dead_code)]
    #[inline]
    pub fn op(&self) -> type_sitter_lib::NodeResult<'tree, symbols::Eq_<'tree>> {
        self . 0 . child_by_field_name ("op") . map (< symbols :: Eq_ < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `values` which has kind `list_value_defs` ([ListValueDefs])"]
    #[allow(dead_code)]
    #[inline]
    pub fn values(&self) -> type_sitter_lib::NodeResult<'tree, ListValueDefs<'tree>> {
        self . 0 . child_by_field_name ("values") . map (< ListValueDefs < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for List<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for List<'tree> {
    const KIND: &'static str = "list";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `list_value_def`\n\nThis node has these fields:\n- `name`: `identifier` ([Identifier])\n- `op`: `=?` ([symbols::Eq_])\n- `paren`: `{( | )}*` ([anon_unions::LParen__RParen_])\n- `value`: `number?` ([Number])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ListValueDef<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ListValueDef<'tree> {
    #[doc = "Get the field `name` which has kind `identifier` ([Identifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `op` which has kind `=?` ([symbols::Eq_])"]
    #[allow(dead_code)]
    #[inline]
    pub fn op(&self) -> Option<type_sitter_lib::NodeResult<'tree, symbols::Eq_<'tree>>> {
        self.0
            .child_by_field_name("op")
            .map(<symbols::Eq_<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the field `paren` which has kind `{( | )}*` ([anon_unions::LParen__RParen_])"]
    #[allow(dead_code)]
    #[inline]
    pub fn parens<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::LParen__RParen_<'tree>>,
        >,
    > + 'a {
        self . 0 . children_by_field_name ("paren" , c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: LParen__RParen_ < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the field `value` which has kind `number?` ([Number])"]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> Option<type_sitter_lib::NodeResult<'tree, Number<'tree>>> {
        self.0
            .child_by_field_name("value")
            .map(<Number<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ListValueDef<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "list_value_def" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ListValueDef<'tree> {
    const KIND: &'static str = "list_value_def";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `list_value_defs`\n\nThis node has children: `list_value_def+` ([ListValueDef])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ListValueDefs<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ListValueDefs<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, ListValueDef<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, ListValueDef<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, ListValueDef<'tree>>>,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, ListValueDef<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ListValueDefs<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "list_value_defs" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ListValueDefs<'tree> {
    const KIND: &'static str = "list_value_defs";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `list_values`\n\nThis node has children: `{identifier | qualified_name}*`:\n- [Identifier]\n- [QualifiedName]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ListValues<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ListValues<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Identifier_QualifiedName<'tree>>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Identifier_QualifiedName < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Identifier_QualifiedName<'tree>>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Identifier_QualifiedName < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ListValues<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "list_values" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ListValues<'tree> {
    const KIND: &'static str = "list_values";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `multiline_alternatives`\n\nThis node has these fields:\n- `type`: `{cycle | once | shuffle | stopping}+` ([anon_unions::Cycle_Once_Shuffle_Stopping])\n\nAnd additional children: `alt_arm*` ([AltArm])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MultilineAlternatives<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MultilineAlternatives<'tree> {
    #[doc = "Get the field `type` which has kind `{cycle | once | shuffle | stopping}+` ([anon_unions::Cycle_Once_Shuffle_Stopping])"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn types<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Cycle_Once_Shuffle_Stopping<'tree>>,
        >,
    > + 'a {
        self . 0 . children_by_field_name ("type" , c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Cycle_Once_Shuffle_Stopping < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::AltArm_Cycle_Once_Shuffle_Stopping<'tree>>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: AltArm_Cycle_Once_Shuffle_Stopping < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::AltArm_Cycle_Once_Shuffle_Stopping<'tree>>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: AltArm_Cycle_Once_Shuffle_Stopping < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MultilineAlternatives<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "multiline_alternatives" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MultilineAlternatives<'tree> {
    const KIND: &'static str = "multiline_alternatives";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `number`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Number<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Number<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Number<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "number" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Number<'tree> {
    const KIND: &'static str = "number";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `params`\n\nThis node has these fields:\n- `ref`: `ref*` ([unnamed::Ref])\n\nAnd additional children: `{divert | identifier}+`:\n- [Divert]\n- [Identifier]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Params<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Params<'tree> {
    #[doc = "Get the field `ref` which has kind `ref*` ([unnamed::Ref])"]
    #[allow(dead_code)]
    #[inline]
    pub fn refs<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, unnamed::Ref<'tree>>,
        >,
    > + 'a {
        self.0.children_by_field_name("ref", c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, unnamed::Ref<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Divert_Identifier_Ref<'tree>>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Divert_Identifier_Ref < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Divert_Identifier_Ref<'tree>>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Divert_Identifier_Ref < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Params<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "params" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Params<'tree> {
    const KIND: &'static str = "params";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `paren`\n\nThis node has a child: `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}`:\n- [Binary]\n- [Boolean]\n- [Call]\n- [Divert]\n- [Identifier]\n- [ListValues]\n- [Number]\n- [Paren]\n- [Postfix]\n- [QualifiedName]\n- [String]\n- [Unary]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Paren<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Paren<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > >{
        self . 0 . named_child (0) . map (< anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Paren<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "paren" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Paren<'tree> {
    const KIND: &'static str = "paren";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `postfix`\n\nThis node has these fields:\n- `op`: `{++ | --}` ([anon_unions::Add_Add__Sub_Sub_])\n\nAnd an additional child: `identifier` ([Identifier])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Postfix<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Postfix<'tree> {
    #[doc = "Get the field `op` which has kind `{++ | --}` ([anon_unions::Add_Add__Sub_Sub_])"]
    #[allow(dead_code)]
    #[inline]
    pub fn op(&self) -> type_sitter_lib::NodeResult<'tree, anon_unions::Add_Add__Sub_Sub_<'tree>> {
        self . 0 . child_by_field_name ("op") . map (< anon_unions :: Add_Add__Sub_Sub_ < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Identifier_Add_Add__Sub_Sub_<'tree>>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Identifier_Add_Add__Sub_Sub_ < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Identifier_Add_Add__Sub_Sub_<'tree>>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Identifier_Add_Add__Sub_Sub_ < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Postfix<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "postfix" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Postfix<'tree> {
    const KIND: &'static str = "postfix";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `qualified_name`\n\nThis node has children: `identifier+` ([Identifier])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct QualifiedName<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> QualifiedName<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Identifier<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, Identifier<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Identifier<'tree>>>,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Identifier<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for QualifiedName<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "qualified_name" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for QualifiedName<'tree> {
    const KIND: &'static str = "qualified_name";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `return`\n\nThis node has a child: `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}`:\n- [Binary]\n- [Boolean]\n- [Call]\n- [Divert]\n- [Identifier]\n- [ListValues]\n- [Number]\n- [Paren]\n- [Postfix]\n- [QualifiedName]\n- [String]\n- [Unary]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Return<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Return<'tree> {
    #[doc = "Get the node's only named child"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > >{
        self . 0 . named_child (0) . map (< anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Return<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "return" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Return<'tree> {
    const KIND: &'static str = "return";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `stitch`\n\nThis node has these fields:\n- `name`: `identifier` ([Identifier])\n- `params`: `{( | ) | params}*` ([anon_unions::LParen__RParen__Params])\n- `start_mark`: `=` ([symbols::Eq_])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Stitch<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Stitch<'tree> {
    #[doc = "Get the field `name` which has kind `identifier` ([Identifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `params` which has kind `{( | ) | params}*` ([anon_unions::LParen__RParen__Params])"]
    #[allow(dead_code)]
    #[inline]
    pub fn paramss<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::LParen__RParen__Params<'tree>>,
        >,
    > + 'a {
        self . 0 . children_by_field_name ("params" , c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: LParen__RParen__Params < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the field `start_mark` which has kind `=` ([symbols::Eq_])"]
    #[allow(dead_code)]
    #[inline]
    pub fn start_mark(&self) -> type_sitter_lib::NodeResult<'tree, symbols::Eq_<'tree>> {
        self . 0 . child_by_field_name ("start_mark") . map (< symbols :: Eq_ < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Stitch<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "stitch" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Stitch<'tree> {
    const KIND: &'static str = "stitch";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `stitch_block`\n\nThis node has children: `{content_block | stitch}+`:\n- [ContentBlock]\n- [Stitch]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct StitchBlock<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> StitchBlock<'tree> {
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::ContentBlock_Stitch<'tree>>,
        >,
    > + 'a {
        self.0.named_children(c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, anon_unions::ContentBlock_Stitch<'tree>> as TryFrom<
                _,
            >>::try_from(n)
        })
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::ContentBlock_Stitch<'tree>>,
        >,
    > {
        self.0.named_child(i).map(
            <type_sitter_lib::ExtraOr<'tree, anon_unions::ContentBlock_Stitch<'tree>> as TryFrom<
                _,
            >>::try_from,
        )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for StitchBlock<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "stitch_block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for StitchBlock<'tree> {
    const KIND: &'static str = "stitch_block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `string`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct String<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> String<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for String<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "string" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for String<'tree> {
    const KIND: &'static str = "string";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `tag`\n\nThis node has children: `{alternatives | cond_block | conditional_text | eval | glue | multiline_alternatives | text}*`:\n- [Alternatives]\n- [CondBlock]\n- [ConditionalText]\n- [Eval]\n- [Glue]\n- [MultilineAlternatives]\n- [Text]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Tag<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Tag<'tree> {
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Alternatives_CondBlock_ConditionalText_Eval_Glue_MultilineAlternatives_Text < 'tree > > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Alternatives_CondBlock_ConditionalText_Eval_Glue_MultilineAlternatives_Text < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Alternatives_CondBlock_ConditionalText_Eval_Glue_MultilineAlternatives_Text < 'tree > > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Alternatives_CondBlock_ConditionalText_Eval_Glue_MultilineAlternatives_Text < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Tag<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tag" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Tag<'tree> {
    const KIND: &'static str = "tag";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `temp`\n\nThis node has these fields:\n- `name`: `identifier` ([Identifier])\n- `op`: `=` ([symbols::Eq_])\n- `value`: `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}` ([anon_unions::Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Temp<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Temp<'tree> {
    #[doc = "Get the field `name` which has kind `identifier` ([Identifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self . 0 . child_by_field_name ("name") . map (< Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `op` which has kind `=` ([symbols::Eq_])"]
    #[allow(dead_code)]
    #[inline]
    pub fn op(&self) -> type_sitter_lib::NodeResult<'tree, symbols::Eq_<'tree>> {
        self . 0 . child_by_field_name ("op") . map (< symbols :: Eq_ < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the field `value` which has kind `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}` ([anon_unions::Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary])"]
    #[allow(dead_code)]
    #[inline]    pub fn value (& self) -> type_sitter_lib :: NodeResult < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > >{
        self . 0 . child_by_field_name ("value") . map (< anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Temp<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "temp" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Temp<'tree> {
    const KIND: &'static str = "temp";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `text`\n\nThis node has these fields:\n- `args`: `{! | != | !? | % | && | ( | ) | * | + | ++ | , | - | -- | -> | . | / | < | <= | == | > | >= | ? | ^ | and | false | has | hasnt | mod | not | or | true | ||}*` ([anon_unions::Not__Not_Eq__Not_Question__Mod__And_And__LParen__RParen__Mul__Add__Add_Add__Comma__Sub__Sub_Sub__Sub_Gt__Dot__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_False_Has_Hasnt_Mod_Not_Or_True_Or_Or_])\n- `name`: `.*` ([symbols::Dot_])\n- `op`: `{! | != | !? | % | && | * | + | ++ | - | -- | / | < | <= | == | > | >= | ? | ^ | and | has | hasnt | mod | not | or | ||}*` ([anon_unions::Not__Not_Eq__Not_Question__Mod__And_And__Mul__Add__Add_Add__Sub__Sub_Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Not_Or_Or_Or_])\n- `target`: `{! | != | !? | % | && | ( | ) | * | + | ++ | , | - | -- | -> | . | / | < | <= | == | > | >= | ? | ^ | and | false | has | hasnt | mod | not | or | true | ||}*` ([anon_unions::Not__Not_Eq__Not_Question__Mod__And_And__LParen__RParen__Mul__Add__Add_Add__Comma__Sub__Sub_Sub__Sub_Gt__Dot__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_False_Has_Hasnt_Mod_Not_Or_True_Or_Or_])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Text<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Text<'tree> {
    #[doc = "Get the field `args` which has kind `{! | != | !? | % | && | ( | ) | * | + | ++ | , | - | -- | -> | . | / | < | <= | == | > | >= | ? | ^ | and | false | has | hasnt | mod | not | or | true | ||}*` ([anon_unions::Not__Not_Eq__Not_Question__Mod__And_And__LParen__RParen__Mul__Add__Add_Add__Comma__Sub__Sub_Sub__Sub_Gt__Dot__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_False_Has_Hasnt_Mod_Not_Or_True_Or_Or_])"]
    #[allow(dead_code)]
    #[inline]    pub fn argss < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl Iterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Not__Not_Eq__Not_Question__Mod__And_And__LParen__RParen__Mul__Add__Add_Add__Comma__Sub__Sub_Sub__Sub_Gt__Dot__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_False_Has_Hasnt_Mod_Not_Or_True_Or_Or_ < 'tree > > >> + 'a{
        self . 0 . children_by_field_name ("args" , c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Not__Not_Eq__Not_Question__Mod__And_And__LParen__RParen__Mul__Add__Add_Add__Comma__Sub__Sub_Sub__Sub_Gt__Dot__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_False_Has_Hasnt_Mod_Not_Or_True_Or_Or_ < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the field `name` which has kind `.*` ([symbols::Dot_])"]
    #[allow(dead_code)]
    #[inline]
    pub fn names<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, symbols::Dot_<'tree>>,
        >,
    > + 'a {
        self.0.children_by_field_name("name", c).map(|n| {
            <type_sitter_lib::ExtraOr<'tree, symbols::Dot_<'tree>> as TryFrom<_>>::try_from(n)
        })
    }
    #[doc = "Get the field `op` which has kind `{! | != | !? | % | && | * | + | ++ | - | -- | / | < | <= | == | > | >= | ? | ^ | and | has | hasnt | mod | not | or | ||}*` ([anon_unions::Not__Not_Eq__Not_Question__Mod__And_And__Mul__Add__Add_Add__Sub__Sub_Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Not_Or_Or_Or_])"]
    #[allow(dead_code)]
    #[inline]    pub fn ops < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl Iterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Not__Not_Eq__Not_Question__Mod__And_And__Mul__Add__Add_Add__Sub__Sub_Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Not_Or_Or_Or_ < 'tree > > >> + 'a{
        self . 0 . children_by_field_name ("op" , c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Not__Not_Eq__Not_Question__Mod__And_And__Mul__Add__Add_Add__Sub__Sub_Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Not_Or_Or_Or_ < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the field `target` which has kind `{! | != | !? | % | && | ( | ) | * | + | ++ | , | - | -- | -> | . | / | < | <= | == | > | >= | ? | ^ | and | false | has | hasnt | mod | not | or | true | ||}*` ([anon_unions::Not__Not_Eq__Not_Question__Mod__And_And__LParen__RParen__Mul__Add__Add_Add__Comma__Sub__Sub_Sub__Sub_Gt__Dot__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_False_Has_Hasnt_Mod_Not_Or_True_Or_Or_])"]
    #[allow(dead_code)]
    #[inline]    pub fn targets < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl Iterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Not__Not_Eq__Not_Question__Mod__And_And__LParen__RParen__Mul__Add__Add_Add__Comma__Sub__Sub_Sub__Sub_Gt__Dot__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_False_Has_Hasnt_Mod_Not_Or_True_Or_Or_ < 'tree > > >> + 'a{
        self . 0 . children_by_field_name ("target" , c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Not__Not_Eq__Not_Question__Mod__And_And__LParen__RParen__Mul__Add__Add_Add__Comma__Sub__Sub_Sub__Sub_Gt__Dot__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_False_Has_Hasnt_Mod_Not_Or_True_Or_Or_ < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Text<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "text" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Text<'tree> {
    const KIND: &'static str = "text";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `thread`\n\nThis node has these fields:\n- `target`: `{call | identifier}` ([anon_unions::Call_Identifier])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Thread<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Thread<'tree> {
    #[doc = "Get the field `target` which has kind `{call | identifier}` ([anon_unions::Call_Identifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn target(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, anon_unions::Call_Identifier<'tree>> {
        self . 0 . child_by_field_name ("target") . map (< anon_unions :: Call_Identifier < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Thread<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "thread" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Thread<'tree> {
    const KIND: &'static str = "thread";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `todo_comment`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TodoComment<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TodoComment<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TodoComment<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "todo_comment" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TodoComment<'tree> {
    const KIND: &'static str = "todo_comment";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `tunnel`\n\nThis node has these fields:\n- `target`: `{call | identifier}?` ([anon_unions::Call_Identifier])\n\nAnd additional children: `divert*` ([Divert])\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Tunnel<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Tunnel<'tree> {
    #[doc = "Get the field `target` which has kind `{call | identifier}?` ([anon_unions::Call_Identifier])"]
    #[allow(dead_code)]
    #[inline]
    pub fn target(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, anon_unions::Call_Identifier<'tree>>> {
        self.0
            .child_by_field_name("target")
            .map(<anon_unions::Call_Identifier<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = "Get the node's named children"]
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl ExactSizeIterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Divert_Call_Identifier<'tree>>,
        >,
    > + 'a {
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Divert_Call_Identifier < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, anon_unions::Divert_Call_Identifier<'tree>>,
        >,
    > {
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Divert_Call_Identifier < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Tunnel<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tunnel" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Tunnel<'tree> {
    const KIND: &'static str = "tunnel";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `unary`\n\nThis node has these fields:\n- `op`: `{! | - | not}` ([anon_unions::Not__Sub__Not])\n\nAnd an additional child: `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}`:\n- [Binary]\n- [Boolean]\n- [Call]\n- [Divert]\n- [Identifier]\n- [ListValues]\n- [Number]\n- [Paren]\n- [Postfix]\n- [QualifiedName]\n- [String]\n- [Unary]\n\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Unary<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Unary<'tree> {
    #[doc = "Get the field `op` which has kind `{! | - | not}` ([anon_unions::Not__Sub__Not])"]
    #[allow(dead_code)]
    #[inline]
    pub fn op(&self) -> type_sitter_lib::NodeResult<'tree, anon_unions::Not__Sub__Not<'tree>> {
        self . 0 . child_by_field_name ("op") . map (< anon_unions :: Not__Sub__Not < 'tree > as TryFrom < _ >> :: try_from) . expect ("tree-sitter node missing its required child, there should at least be a MISSING node in its place")
    }
    #[doc = "Get the node's named children"]
    #[doc = "This is guaranteed to return at least one child"]
    #[allow(dead_code)]
    #[inline]    pub fn children < 'a > (& self , c : & 'a mut tree_sitter :: TreeCursor < 'tree >) -> impl ExactSizeIterator < Item = type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not__Sub__Not < 'tree > > >> + 'a{
        self . 0 . named_children (c) . map (| n | < type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not__Sub__Not < 'tree > > as TryFrom < _ >> :: try_from (n))
    }
    #[doc = "Get the node's named child #i"]
    #[allow(dead_code)]
    #[inline]    pub fn child (& self , i : usize) -> Option < type_sitter_lib :: NodeResult < 'tree , type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not__Sub__Not < 'tree > > >>{
        self . 0 . named_child (i) . map (< type_sitter_lib :: ExtraOr < 'tree , anon_unions :: Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not__Sub__Not < 'tree > > as TryFrom < _ >> :: try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Unary<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unary" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Unary<'tree> {
    const KIND: &'static str = "unary";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `comment`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Comment<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Comment<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Comment<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "comment" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Comment<'tree> {
    const KIND: &'static str = "comment";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `else`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Else<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Else<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Else<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "else" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Else<'tree> {
    const KIND: &'static str = "else";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `glue`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Glue<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Glue<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Glue<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "glue" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Glue<'tree> {
    const KIND: &'static str = "glue";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = "Typed node `path`\n\nThis node has no children\n"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Path<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Path<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Path<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "path" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Path<'tree> {
    const KIND: &'static str = "path";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
        &mut self.0
    }
    #[inline]
    fn into_node(self) -> tree_sitter::Node<'tree> {
        self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
pub mod unnamed {
    #[allow(unused_imports)]
    use super::*;
    #[doc = "Typed node `textrest`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Textrest<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Textrest<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Textrest<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "textrest" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Textrest<'tree> {
        const KIND: &'static str = "textrest";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `CONST`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Const<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Const<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Const<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "CONST" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Const<'tree> {
        const KIND: &'static str = "CONST";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `EXTERNAL`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct External<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> External<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for External<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "EXTERNAL" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for External<'tree> {
        const KIND: &'static str = "EXTERNAL";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `INCLUDE`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Include<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Include<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Include<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "INCLUDE" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Include<'tree> {
        const KIND: &'static str = "INCLUDE";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `LIST`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct List<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> List<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for List<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "LIST" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for List<'tree> {
        const KIND: &'static str = "LIST";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `TODO`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Todo<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Todo<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Todo<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "TODO" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Todo<'tree> {
        const KIND: &'static str = "TODO";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `VAR`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Var<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Var<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Var<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "VAR" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Var<'tree> {
        const KIND: &'static str = "VAR";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `and`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct And<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> And<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for And<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "and" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for And<'tree> {
        const KIND: &'static str = "and";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `cycle`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Cycle<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Cycle<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Cycle<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "cycle" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Cycle<'tree> {
        const KIND: &'static str = "cycle";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `false`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct False<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> False<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for False<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "false" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for False<'tree> {
        const KIND: &'static str = "false";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `function`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Function<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Function<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Function<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "function" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Function<'tree> {
        const KIND: &'static str = "function";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `has`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Has<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Has<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Has<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "has" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Has<'tree> {
        const KIND: &'static str = "has";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `hasnt`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Hasnt<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Hasnt<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Hasnt<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "hasnt" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Hasnt<'tree> {
        const KIND: &'static str = "hasnt";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `mod`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Mod<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Mod<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Mod<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "mod" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Mod<'tree> {
        const KIND: &'static str = "mod";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `not`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Not<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Not<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Not<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "not" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Not<'tree> {
        const KIND: &'static str = "not";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `once`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Once<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Once<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Once<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "once" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Once<'tree> {
        const KIND: &'static str = "once";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `or`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Or<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Or<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Or<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "or" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Or<'tree> {
        const KIND: &'static str = "or";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `ref`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Ref<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Ref<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Ref<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "ref" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Ref<'tree> {
        const KIND: &'static str = "ref";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `return`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Return<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Return<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Return<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "return" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Return<'tree> {
        const KIND: &'static str = "return";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `shuffle`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Shuffle<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Shuffle<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Shuffle<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "shuffle" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Shuffle<'tree> {
        const KIND: &'static str = "shuffle";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `stopping`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Stopping<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Stopping<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Stopping<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "stopping" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Stopping<'tree> {
        const KIND: &'static str = "stopping";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `temp`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Temp<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Temp<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Temp<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "temp" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Temp<'tree> {
        const KIND: &'static str = "temp";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `true`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct True<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> True<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for True<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "true" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for True<'tree> {
        const KIND: &'static str = "true";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `word`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Word<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Word<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Word<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "word" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Word<'tree> {
        const KIND: &'static str = "word";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
}
pub mod symbols {
    #[allow(unused_imports)]
    use super::*;
    #[doc = "Typed node `!`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Not_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Not_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Not_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "!" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Not_<'tree> {
        const KIND: &'static str = "!";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `!=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Not_Eq_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Not_Eq_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Not_Eq_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "!=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Not_Eq_<'tree> {
        const KIND: &'static str = "!=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `!?`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Not_Question_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Not_Question_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Not_Question_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "!?" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Not_Question_<'tree> {
        const KIND: &'static str = "!?";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `#`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Hash_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Hash_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Hash_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "#" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Hash_<'tree> {
        const KIND: &'static str = "#";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `$`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Dollar_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Dollar_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Dollar_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "$" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Dollar_<'tree> {
        const KIND: &'static str = "$";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `%`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Mod_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Mod_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Mod_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "%" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Mod_<'tree> {
        const KIND: &'static str = "%";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `&`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct And_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> And_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for And_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "&" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for And_<'tree> {
        const KIND: &'static str = "&";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `&&`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct And_And_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> And_And_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for And_And_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "&&" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for And_And_<'tree> {
        const KIND: &'static str = "&&";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `(`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LParen_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LParen_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LParen_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "(" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LParen_<'tree> {
        const KIND: &'static str = "(";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `)`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct RParen_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> RParen_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RParen_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ")" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for RParen_<'tree> {
        const KIND: &'static str = ")";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `*`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Mul_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Mul_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Mul_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "*" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Mul_<'tree> {
        const KIND: &'static str = "*";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `+`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Add_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Add_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Add_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "+" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Add_<'tree> {
        const KIND: &'static str = "+";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `++`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Add_Add_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Add_Add_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Add_Add_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "++" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Add_Add_<'tree> {
        const KIND: &'static str = "++";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `+=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Add_Eq_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Add_Eq_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Add_Eq_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "+=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Add_Eq_<'tree> {
        const KIND: &'static str = "+=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `,`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Comma_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Comma_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Comma_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "," {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Comma_<'tree> {
        const KIND: &'static str = ",";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `-`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Sub_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Sub_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Sub_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "-" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Sub_<'tree> {
        const KIND: &'static str = "-";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `--`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Sub_Sub_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Sub_Sub_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Sub_Sub_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "--" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Sub_Sub_<'tree> {
        const KIND: &'static str = "--";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `-=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Sub_Eq_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Sub_Eq_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Sub_Eq_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "-=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Sub_Eq_<'tree> {
        const KIND: &'static str = "-=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `->`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Sub_Gt_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Sub_Gt_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Sub_Gt_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "->" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Sub_Gt_<'tree> {
        const KIND: &'static str = "->";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `->->`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Sub_Gt_Sub_Gt_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Sub_Gt_Sub_Gt_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Sub_Gt_Sub_Gt_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "->->" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Sub_Gt_Sub_Gt_<'tree> {
        const KIND: &'static str = "->->";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `.`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Dot_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Dot_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Dot_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "." {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Dot_<'tree> {
        const KIND: &'static str = ".";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `/`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Div_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Div_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Div_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "/" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Div_<'tree> {
        const KIND: &'static str = "/";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `:`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Colon_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Colon_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Colon_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ":" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Colon_<'tree> {
        const KIND: &'static str = ":";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `<`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Lt_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Lt_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Lt_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "<" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Lt_<'tree> {
        const KIND: &'static str = "<";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `<-`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Lt_Sub_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Lt_Sub_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Lt_Sub_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "<-" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Lt_Sub_<'tree> {
        const KIND: &'static str = "<-";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `<=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Lt_Eq_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Lt_Eq_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Lt_Eq_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "<=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Lt_Eq_<'tree> {
        const KIND: &'static str = "<=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Eq_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Eq_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Eq_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Eq_<'tree> {
        const KIND: &'static str = "=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `==`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Eq_Eq_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Eq_Eq_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Eq_Eq_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "==" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Eq_Eq_<'tree> {
        const KIND: &'static str = "==";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `>`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Gt_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Gt_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Gt_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ">" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Gt_<'tree> {
        const KIND: &'static str = ">";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `>=`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Gt_Eq_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Gt_Eq_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Gt_Eq_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == ">=" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Gt_Eq_<'tree> {
        const KIND: &'static str = ">=";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `?`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Question_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Question_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Question_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "?" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Question_<'tree> {
        const KIND: &'static str = "?";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `[`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LBracket_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LBracket_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LBracket_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "[" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LBracket_<'tree> {
        const KIND: &'static str = "[";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `\\`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Backslash_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Backslash_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Backslash_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "\\" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Backslash_<'tree> {
        const KIND: &'static str = "\\";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `\\#`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Backslash_Hash_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Backslash_Hash_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Backslash_Hash_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "\\#" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Backslash_Hash_<'tree> {
        const KIND: &'static str = "\\#";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `\\[`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Backslash_LBracket_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Backslash_LBracket_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Backslash_LBracket_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "\\[" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Backslash_LBracket_<'tree> {
        const KIND: &'static str = "\\[";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `\\]`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Backslash_RBracket_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Backslash_RBracket_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Backslash_RBracket_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "\\]" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Backslash_RBracket_<'tree> {
        const KIND: &'static str = "\\]";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `\\{`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Backslash_LBrace_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Backslash_LBrace_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Backslash_LBrace_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "\\{" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Backslash_LBrace_<'tree> {
        const KIND: &'static str = "\\{";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `\\|`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Backslash_Or_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Backslash_Or_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Backslash_Or_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "\\|" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Backslash_Or_<'tree> {
        const KIND: &'static str = "\\|";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `\\}`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Backslash_RBrace_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Backslash_RBrace_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Backslash_RBrace_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "\\}" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Backslash_RBrace_<'tree> {
        const KIND: &'static str = "\\}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `]`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct RBracket_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> RBracket_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RBracket_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "]" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for RBracket_<'tree> {
        const KIND: &'static str = "]";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `^`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct BitXor_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> BitXor_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BitXor_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "^" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for BitXor_<'tree> {
        const KIND: &'static str = "^";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `{`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct LBrace_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> LBrace_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LBrace_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "{" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LBrace_<'tree> {
        const KIND: &'static str = "{";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `|`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Or_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Or_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Or_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "|" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Or_<'tree> {
        const KIND: &'static str = "|";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `||`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct Or_Or_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> Or_Or_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Or_Or_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "||" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Or_Or_<'tree> {
        const KIND: &'static str = "||";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `}`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct RBrace_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> RBrace_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RBrace_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "}" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for RBrace_<'tree> {
        const KIND: &'static str = "}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
    #[doc = "Typed node `~`\n\nThis node has no children\n"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub struct BitNot_<'tree>(tree_sitter::Node<'tree>);
    #[automatically_derived]
    impl<'tree> BitNot_<'tree> {}
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BitNot_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            if node.kind() == "~" {
                Ok(Self(node))
            } else {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for BitNot_<'tree> {
        const KIND: &'static str = "~";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            &self.0
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            &mut self.0
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            self.0
        }
        #[inline]
        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
            Self(node)
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::*;
    #[doc = "one of `{choice_block | code | content | external | global | include | list | todo_comment}`:\n- [ChoiceBlock]\n- [Code]\n- [Content]\n- [External]\n- [Global]\n- [Include]\n- [List]\n- [TodoComment]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment<'tree> {
        ChoiceBlock(ChoiceBlock<'tree>),
        Code(Code<'tree>),
        Content(Content<'tree>),
        External(External<'tree>),
        Global(Global<'tree>),
        Include(Include<'tree>),
        List(List<'tree>),
        TodoComment(TodoComment<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment<'tree> {
        #[doc = "Returns the node if it is of kind `choice_block` ([ChoiceBlock]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn choice_block(self) -> Option<ChoiceBlock<'tree>> {
            match self {
                Self::ChoiceBlock(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `code` ([Code]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn code(self) -> Option<Code<'tree>> {
            match self {
                Self::Code(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `content` ([Content]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn content(self) -> Option<Content<'tree>> {
            match self {
                Self::Content(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `external` ([External]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn external(self) -> Option<External<'tree>> {
            match self {
                Self::External(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `global` ([Global]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn global(self) -> Option<Global<'tree>> {
            match self {
                Self::Global(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `include` ([Include]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn include(self) -> Option<Include<'tree>> {
            match self {
                Self::Include(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `list` ([List]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn list(self) -> Option<List<'tree>> {
            match self {
                Self::List(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `todo_comment` ([TodoComment]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn todo_comment(self) -> Option<TodoComment<'tree>> {
            match self {
                Self::TodoComment(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "choice_block" => Ok(unsafe {
                    Self :: ChoiceBlock (< ChoiceBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "code" => Ok(unsafe {
                    Self::Code(
                        <Code<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                            node,
                        ),
                    )
                }),
                "content" => {
                    Ok(unsafe {
                        Self :: Content (< Content < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "external" => {
                    Ok(unsafe {
                        Self :: External (< External < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "global" => {
                    Ok(unsafe {
                        Self :: Global (< Global < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "include" => {
                    Ok(unsafe {
                        Self :: Include (< Include < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "list" => Ok(unsafe {
                    Self::List(
                        <List<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                            node,
                        ),
                    )
                }),
                "todo_comment" => Ok(unsafe {
                    Self :: TodoComment (< TodoComment < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment<'tree>
    {
        const KIND: &'static str =
            "{choice_block | code | content | external | global | include | list | todo_comment}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::ChoiceBlock(x) => x.node(),
                Self::Code(x) => x.node(),
                Self::Content(x) => x.node(),
                Self::External(x) => x.node(),
                Self::Global(x) => x.node(),
                Self::Include(x) => x.node(),
                Self::List(x) => x.node(),
                Self::TodoComment(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::ChoiceBlock(x) => x.node_mut(),
                Self::Code(x) => x.node_mut(),
                Self::Content(x) => x.node_mut(),
                Self::External(x) => x.node_mut(),
                Self::Global(x) => x.node_mut(),
                Self::Include(x) => x.node_mut(),
                Self::List(x) => x.node_mut(),
                Self::TodoComment(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::ChoiceBlock(x) => x.into_node(),
                Self::Code(x) => x.into_node(),
                Self::Content(x) => x.into_node(),
                Self::External(x) => x.into_node(),
                Self::Global(x) => x.into_node(),
                Self::Include(x) => x.into_node(),
                Self::List(x) => x.into_node(),
                Self::TodoComment(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{alternatives | cond_block | conditional_text | content | eval | glue | multiline_alternatives | text}`:\n- [Alternatives]\n- [CondBlock]\n- [ConditionalText]\n- [Content]\n- [Eval]\n- [Glue]\n- [MultilineAlternatives]\n- [Text]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Alternatives_CondBlock_ConditionalText_Content_Eval_Glue_MultilineAlternatives_Text<
        'tree,
    > {
        Alternatives(Alternatives<'tree>),
        CondBlock(CondBlock<'tree>),
        ConditionalText(ConditionalText<'tree>),
        Content(Content<'tree>),
        Eval(Eval<'tree>),
        Glue(Glue<'tree>),
        MultilineAlternatives(MultilineAlternatives<'tree>),
        Text(Text<'tree>),
    }
    #[automatically_derived]
    impl<'tree>
        Alternatives_CondBlock_ConditionalText_Content_Eval_Glue_MultilineAlternatives_Text<'tree>
    {
        #[doc = "Returns the node if it is of kind `alternatives` ([Alternatives]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn alternatives(self) -> Option<Alternatives<'tree>> {
            match self {
                Self::Alternatives(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `cond_block` ([CondBlock]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn cond_block(self) -> Option<CondBlock<'tree>> {
            match self {
                Self::CondBlock(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `conditional_text` ([ConditionalText]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn conditional_text(self) -> Option<ConditionalText<'tree>> {
            match self {
                Self::ConditionalText(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `content` ([Content]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn content(self) -> Option<Content<'tree>> {
            match self {
                Self::Content(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `eval` ([Eval]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn eval(self) -> Option<Eval<'tree>> {
            match self {
                Self::Eval(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `glue` ([Glue]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn glue(self) -> Option<Glue<'tree>> {
            match self {
                Self::Glue(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `multiline_alternatives` ([MultilineAlternatives]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn multiline_alternatives(self) -> Option<MultilineAlternatives<'tree>> {
            match self {
                Self::MultilineAlternatives(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `text` ([Text]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn text(self) -> Option<Text<'tree>> {
            match self {
                Self::Text(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for Alternatives_CondBlock_ConditionalText_Content_Eval_Glue_MultilineAlternatives_Text<
            'tree,
        >
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "alternatives" => Ok(unsafe {
                    Self :: Alternatives (< Alternatives < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "cond_block" => Ok(unsafe {
                    Self :: CondBlock (< CondBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "conditional_text" => {
                    Ok(unsafe {
                        Self :: ConditionalText (< ConditionalText < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "content" => {
                    Ok(unsafe {
                        Self :: Content (< Content < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "eval" => Ok(unsafe {
                    Self::Eval(
                        <Eval<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                            node,
                        ),
                    )
                }),
                "glue" => Ok(unsafe {
                    Self::Glue(
                        <Glue<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                            node,
                        ),
                    )
                }),
                "multiline_alternatives" => Ok(unsafe {
                    Self :: MultilineAlternatives (< MultilineAlternatives < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "text" => Ok(unsafe {
                    Self::Text(
                        <Text<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                            node,
                        ),
                    )
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for Alternatives_CondBlock_ConditionalText_Content_Eval_Glue_MultilineAlternatives_Text<
            'tree,
        >
    {
        const KIND : & 'static str = "{alternatives | cond_block | conditional_text | content | eval | glue | multiline_alternatives | text}" ;
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Alternatives(x) => x.node(),
                Self::CondBlock(x) => x.node(),
                Self::ConditionalText(x) => x.node(),
                Self::Content(x) => x.node(),
                Self::Eval(x) => x.node(),
                Self::Glue(x) => x.node(),
                Self::MultilineAlternatives(x) => x.node(),
                Self::Text(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Alternatives(x) => x.node_mut(),
                Self::CondBlock(x) => x.node_mut(),
                Self::ConditionalText(x) => x.node_mut(),
                Self::Content(x) => x.node_mut(),
                Self::Eval(x) => x.node_mut(),
                Self::Glue(x) => x.node_mut(),
                Self::MultilineAlternatives(x) => x.node_mut(),
                Self::Text(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::Alternatives(x) => x.into_node(),
                Self::CondBlock(x) => x.into_node(),
                Self::ConditionalText(x) => x.into_node(),
                Self::Content(x) => x.into_node(),
                Self::Eval(x) => x.into_node(),
                Self::Glue(x) => x.into_node(),
                Self::MultilineAlternatives(x) => x.into_node(),
                Self::Text(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}`:\n- [Binary]\n- [Boolean]\n- [Call]\n- [Divert]\n- [Identifier]\n- [ListValues]\n- [Number]\n- [Paren]\n- [Postfix]\n- [QualifiedName]\n- [String]\n- [Unary]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary<
        'tree,
    > {
        Binary(Binary<'tree>),
        Boolean(Boolean<'tree>),
        Call(Call<'tree>),
        Divert(Divert<'tree>),
        Identifier(Identifier<'tree>),
        ListValues(ListValues<'tree>),
        Number(Number<'tree>),
        Paren(Paren<'tree>),
        Postfix(Postfix<'tree>),
        QualifiedName(QualifiedName<'tree>),
        String(String<'tree>),
        Unary(Unary<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > { # [doc = "Returns the node if it is of kind `binary` ([Binary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn binary (self) -> Option < Binary < 'tree > > { match self { Self :: Binary (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `boolean` ([Boolean]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn boolean (self) -> Option < Boolean < 'tree > > { match self { Self :: Boolean (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `call` ([Call]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn call (self) -> Option < Call < 'tree > > { match self { Self :: Call (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `divert` ([Divert]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn divert (self) -> Option < Divert < 'tree > > { match self { Self :: Divert (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn identifier (self) -> Option < Identifier < 'tree > > { match self { Self :: Identifier (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `list_values` ([ListValues]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn list_values (self) -> Option < ListValues < 'tree > > { match self { Self :: ListValues (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `number` ([Number]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn number (self) -> Option < Number < 'tree > > { match self { Self :: Number (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `paren` ([Paren]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn paren (self) -> Option < Paren < 'tree > > { match self { Self :: Paren (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `postfix` ([Postfix]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn postfix (self) -> Option < Postfix < 'tree > > { match self { Self :: Postfix (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `qualified_name` ([QualifiedName]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn qualified_name (self) -> Option < QualifiedName < 'tree > > { match self { Self :: QualifiedName (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `string` ([String]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn string (self) -> Option < String < 'tree > > { match self { Self :: String (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `unary` ([Unary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn unary (self) -> Option < Unary < 'tree > > { match self { Self :: Unary (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "binary" => Ok (unsafe { Self :: Binary (< Binary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "boolean" => Ok (unsafe { Self :: Boolean (< Boolean < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "call" => Ok (unsafe { Self :: Call (< Call < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "divert" => Ok (unsafe { Self :: Divert (< Divert < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "identifier" => Ok (unsafe { Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "list_values" => Ok (unsafe { Self :: ListValues (< ListValues < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "number" => Ok (unsafe { Self :: Number (< Number < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "paren" => Ok (unsafe { Self :: Paren (< Paren < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "postfix" => Ok (unsafe { Self :: Postfix (< Postfix < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "qualified_name" => Ok (unsafe { Self :: QualifiedName (< QualifiedName < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "string" => Ok (unsafe { Self :: String (< String < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "unary" => Ok (unsafe { Self :: Unary (< Unary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > { const KIND : & 'static str = "{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Binary (x) => x . node () , Self :: Boolean (x) => x . node () , Self :: Call (x) => x . node () , Self :: Divert (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: ListValues (x) => x . node () , Self :: Number (x) => x . node () , Self :: Paren (x) => x . node () , Self :: Postfix (x) => x . node () , Self :: QualifiedName (x) => x . node () , Self :: String (x) => x . node () , Self :: Unary (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Binary (x) => x . node_mut () , Self :: Boolean (x) => x . node_mut () , Self :: Call (x) => x . node_mut () , Self :: Divert (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: ListValues (x) => x . node_mut () , Self :: Number (x) => x . node_mut () , Self :: Paren (x) => x . node_mut () , Self :: Postfix (x) => x . node_mut () , Self :: QualifiedName (x) => x . node_mut () , Self :: String (x) => x . node_mut () , Self :: Unary (x) => x . node_mut () , } } # [inline] fn into_node (self) -> tree_sitter :: Node < 'tree > { match self { Self :: Binary (x) => x . into_node () , Self :: Boolean (x) => x . into_node () , Self :: Call (x) => x . into_node () , Self :: Divert (x) => x . into_node () , Self :: Identifier (x) => x . into_node () , Self :: ListValues (x) => x . into_node () , Self :: Number (x) => x . into_node () , Self :: Paren (x) => x . into_node () , Self :: Postfix (x) => x . into_node () , Self :: QualifiedName (x) => x . into_node () , Self :: String (x) => x . into_node () , Self :: Unary (x) => x . into_node () , } } }
    #[doc = "one of `{+= | -= | =}`:\n- [symbols::Add_Eq_]\n- [symbols::Sub_Eq_]\n- [symbols::Eq_]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Add_Eq__Sub_Eq__Eq_<'tree> {
        Add_Eq_(symbols::Add_Eq_<'tree>),
        Sub_Eq_(symbols::Sub_Eq_<'tree>),
        Eq_(symbols::Eq_<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Add_Eq__Sub_Eq__Eq_<'tree> {
        #[doc = "Returns the node if it is of kind `+=` ([symbols::Add_Eq_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn add_eq_(self) -> Option<symbols::Add_Eq_<'tree>> {
            match self {
                Self::Add_Eq_(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `-=` ([symbols::Sub_Eq_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn sub_eq_(self) -> Option<symbols::Sub_Eq_<'tree>> {
            match self {
                Self::Sub_Eq_(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `=` ([symbols::Eq_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn eq_(self) -> Option<symbols::Eq_<'tree>> {
            match self {
                Self::Eq_(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Add_Eq__Sub_Eq__Eq_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "+=" => Ok(unsafe {
                    Self::Add_Eq_(<symbols::Add_Eq_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "-=" => Ok(unsafe {
                    Self::Sub_Eq_(<symbols::Sub_Eq_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "=" => Ok(unsafe {
                    Self :: Eq_ (< symbols :: Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Add_Eq__Sub_Eq__Eq_<'tree> {
        const KIND: &'static str = "{+= | -= | =}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Add_Eq_(x) => x.node(),
                Self::Sub_Eq_(x) => x.node(),
                Self::Eq_(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Add_Eq_(x) => x.node_mut(),
                Self::Sub_Eq_(x) => x.node_mut(),
                Self::Eq_(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::Add_Eq_(x) => x.into_node(),
                Self::Sub_Eq_(x) => x.into_node(),
                Self::Eq_(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary | != | !? | % | && | * | + | - | / | < | <= | == | > | >= | ? | ^ | and | has | hasnt | mod | or | ||}`:\n- [Binary]\n- [Boolean]\n- [Call]\n- [Divert]\n- [Identifier]\n- [ListValues]\n- [Number]\n- [Paren]\n- [Postfix]\n- [QualifiedName]\n- [String]\n- [Unary]\n- [symbols::Not_Eq_]\n- [symbols::Not_Question_]\n- [symbols::Mod_]\n- [symbols::And_And_]\n- [symbols::Mul_]\n- [symbols::Add_]\n- [symbols::Sub_]\n- [symbols::Div_]\n- [symbols::Lt_]\n- [symbols::Lt_Eq_]\n- [symbols::Eq_Eq_]\n- [symbols::Gt_]\n- [symbols::Gt_Eq_]\n- [symbols::Question_]\n- [symbols::BitXor_]\n- [unnamed::And]\n- [unnamed::Has]\n- [unnamed::Hasnt]\n- [unnamed::Mod]\n- [unnamed::Or]\n- [symbols::Or_Or_]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_<
        'tree,
    > {
        Binary(Binary<'tree>),
        Boolean(Boolean<'tree>),
        Call(Call<'tree>),
        Divert(Divert<'tree>),
        Identifier(Identifier<'tree>),
        ListValues(ListValues<'tree>),
        Number(Number<'tree>),
        Paren(Paren<'tree>),
        Postfix(Postfix<'tree>),
        QualifiedName(QualifiedName<'tree>),
        String(String<'tree>),
        Unary(Unary<'tree>),
        Not_Eq_(symbols::Not_Eq_<'tree>),
        Not_Question_(symbols::Not_Question_<'tree>),
        Mod_(symbols::Mod_<'tree>),
        And_And_(symbols::And_And_<'tree>),
        Mul_(symbols::Mul_<'tree>),
        Add_(symbols::Add_<'tree>),
        Sub_(symbols::Sub_<'tree>),
        Div_(symbols::Div_<'tree>),
        Lt_(symbols::Lt_<'tree>),
        Lt_Eq_(symbols::Lt_Eq_<'tree>),
        Eq_Eq_(symbols::Eq_Eq_<'tree>),
        Gt_(symbols::Gt_<'tree>),
        Gt_Eq_(symbols::Gt_Eq_<'tree>),
        Question_(symbols::Question_<'tree>),
        BitXor_(symbols::BitXor_<'tree>),
        And(unnamed::And<'tree>),
        Has(unnamed::Has<'tree>),
        Hasnt(unnamed::Hasnt<'tree>),
        Mod(unnamed::Mod<'tree>),
        Or(unnamed::Or<'tree>),
        Or_Or_(symbols::Or_Or_<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_ < 'tree > { # [doc = "Returns the node if it is of kind `binary` ([Binary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn binary (self) -> Option < Binary < 'tree > > { match self { Self :: Binary (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `boolean` ([Boolean]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn boolean (self) -> Option < Boolean < 'tree > > { match self { Self :: Boolean (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `call` ([Call]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn call (self) -> Option < Call < 'tree > > { match self { Self :: Call (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `divert` ([Divert]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn divert (self) -> Option < Divert < 'tree > > { match self { Self :: Divert (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn identifier (self) -> Option < Identifier < 'tree > > { match self { Self :: Identifier (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `list_values` ([ListValues]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn list_values (self) -> Option < ListValues < 'tree > > { match self { Self :: ListValues (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `number` ([Number]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn number (self) -> Option < Number < 'tree > > { match self { Self :: Number (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `paren` ([Paren]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn paren (self) -> Option < Paren < 'tree > > { match self { Self :: Paren (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `postfix` ([Postfix]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn postfix (self) -> Option < Postfix < 'tree > > { match self { Self :: Postfix (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `qualified_name` ([QualifiedName]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn qualified_name (self) -> Option < QualifiedName < 'tree > > { match self { Self :: QualifiedName (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `string` ([String]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn string (self) -> Option < String < 'tree > > { match self { Self :: String (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `unary` ([Unary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn unary (self) -> Option < Unary < 'tree > > { match self { Self :: Unary (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `!=` ([symbols::Not_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn not_eq_ (self) -> Option < symbols :: Not_Eq_ < 'tree > > { match self { Self :: Not_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `!?` ([symbols::Not_Question_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn not_question_ (self) -> Option < symbols :: Not_Question_ < 'tree > > { match self { Self :: Not_Question_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `%` ([symbols::Mod_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn mod_ (self) -> Option < symbols :: Mod_ < 'tree > > { match self { Self :: Mod_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `&&` ([symbols::And_And_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn and_and_ (self) -> Option < symbols :: And_And_ < 'tree > > { match self { Self :: And_And_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `*` ([symbols::Mul_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn mul_ (self) -> Option < symbols :: Mul_ < 'tree > > { match self { Self :: Mul_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `+` ([symbols::Add_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn add_ (self) -> Option < symbols :: Add_ < 'tree > > { match self { Self :: Add_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `-` ([symbols::Sub_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn sub_ (self) -> Option < symbols :: Sub_ < 'tree > > { match self { Self :: Sub_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `/` ([symbols::Div_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn div_ (self) -> Option < symbols :: Div_ < 'tree > > { match self { Self :: Div_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `<` ([symbols::Lt_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn lt_ (self) -> Option < symbols :: Lt_ < 'tree > > { match self { Self :: Lt_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `<=` ([symbols::Lt_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn lt_eq_ (self) -> Option < symbols :: Lt_Eq_ < 'tree > > { match self { Self :: Lt_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `==` ([symbols::Eq_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn eq_eq_ (self) -> Option < symbols :: Eq_Eq_ < 'tree > > { match self { Self :: Eq_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `>` ([symbols::Gt_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn gt_ (self) -> Option < symbols :: Gt_ < 'tree > > { match self { Self :: Gt_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `>=` ([symbols::Gt_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn gt_eq_ (self) -> Option < symbols :: Gt_Eq_ < 'tree > > { match self { Self :: Gt_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `?` ([symbols::Question_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn question_ (self) -> Option < symbols :: Question_ < 'tree > > { match self { Self :: Question_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `^` ([symbols::BitXor_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn bit_xor_ (self) -> Option < symbols :: BitXor_ < 'tree > > { match self { Self :: BitXor_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `and` ([unnamed::And]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn and (self) -> Option < unnamed :: And < 'tree > > { match self { Self :: And (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `has` ([unnamed::Has]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn has (self) -> Option < unnamed :: Has < 'tree > > { match self { Self :: Has (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `hasnt` ([unnamed::Hasnt]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn hasnt (self) -> Option < unnamed :: Hasnt < 'tree > > { match self { Self :: Hasnt (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `mod` ([unnamed::Mod]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn r#mod (self) -> Option < unnamed :: Mod < 'tree > > { match self { Self :: Mod (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `or` ([unnamed::Or]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn or (self) -> Option < unnamed :: Or < 'tree > > { match self { Self :: Or (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `||` ([symbols::Or_Or_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn or_or_ (self) -> Option < symbols :: Or_Or_ < 'tree > > { match self { Self :: Or_Or_ (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_ < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "binary" => Ok (unsafe { Self :: Binary (< Binary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "boolean" => Ok (unsafe { Self :: Boolean (< Boolean < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "call" => Ok (unsafe { Self :: Call (< Call < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "divert" => Ok (unsafe { Self :: Divert (< Divert < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "identifier" => Ok (unsafe { Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "list_values" => Ok (unsafe { Self :: ListValues (< ListValues < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "number" => Ok (unsafe { Self :: Number (< Number < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "paren" => Ok (unsafe { Self :: Paren (< Paren < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "postfix" => Ok (unsafe { Self :: Postfix (< Postfix < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "qualified_name" => Ok (unsafe { Self :: QualifiedName (< QualifiedName < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "string" => Ok (unsafe { Self :: String (< String < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "unary" => Ok (unsafe { Self :: Unary (< Unary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "!=" => Ok (unsafe { Self :: Not_Eq_ (< symbols :: Not_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "!?" => Ok (unsafe { Self :: Not_Question_ (< symbols :: Not_Question_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "%" => Ok (unsafe { Self :: Mod_ (< symbols :: Mod_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "&&" => Ok (unsafe { Self :: And_And_ (< symbols :: And_And_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "*" => Ok (unsafe { Self :: Mul_ (< symbols :: Mul_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "+" => Ok (unsafe { Self :: Add_ (< symbols :: Add_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "-" => Ok (unsafe { Self :: Sub_ (< symbols :: Sub_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "/" => Ok (unsafe { Self :: Div_ (< symbols :: Div_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "<" => Ok (unsafe { Self :: Lt_ (< symbols :: Lt_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "<=" => Ok (unsafe { Self :: Lt_Eq_ (< symbols :: Lt_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "==" => Ok (unsafe { Self :: Eq_Eq_ (< symbols :: Eq_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , ">" => Ok (unsafe { Self :: Gt_ (< symbols :: Gt_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , ">=" => Ok (unsafe { Self :: Gt_Eq_ (< symbols :: Gt_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "?" => Ok (unsafe { Self :: Question_ (< symbols :: Question_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "^" => Ok (unsafe { Self :: BitXor_ (< symbols :: BitXor_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "and" => Ok (unsafe { Self :: And (< unnamed :: And < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "has" => Ok (unsafe { Self :: Has (< unnamed :: Has < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "hasnt" => Ok (unsafe { Self :: Hasnt (< unnamed :: Hasnt < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "mod" => Ok (unsafe { Self :: Mod (< unnamed :: Mod < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "or" => Ok (unsafe { Self :: Or (< unnamed :: Or < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "||" => Ok (unsafe { Self :: Or_Or_ (< symbols :: Or_Or_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_ < 'tree > { const KIND : & 'static str = "{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary | != | !? | % | && | * | + | - | / | < | <= | == | > | >= | ? | ^ | and | has | hasnt | mod | or | ||}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Binary (x) => x . node () , Self :: Boolean (x) => x . node () , Self :: Call (x) => x . node () , Self :: Divert (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: ListValues (x) => x . node () , Self :: Number (x) => x . node () , Self :: Paren (x) => x . node () , Self :: Postfix (x) => x . node () , Self :: QualifiedName (x) => x . node () , Self :: String (x) => x . node () , Self :: Unary (x) => x . node () , Self :: Not_Eq_ (x) => x . node () , Self :: Not_Question_ (x) => x . node () , Self :: Mod_ (x) => x . node () , Self :: And_And_ (x) => x . node () , Self :: Mul_ (x) => x . node () , Self :: Add_ (x) => x . node () , Self :: Sub_ (x) => x . node () , Self :: Div_ (x) => x . node () , Self :: Lt_ (x) => x . node () , Self :: Lt_Eq_ (x) => x . node () , Self :: Eq_Eq_ (x) => x . node () , Self :: Gt_ (x) => x . node () , Self :: Gt_Eq_ (x) => x . node () , Self :: Question_ (x) => x . node () , Self :: BitXor_ (x) => x . node () , Self :: And (x) => x . node () , Self :: Has (x) => x . node () , Self :: Hasnt (x) => x . node () , Self :: Mod (x) => x . node () , Self :: Or (x) => x . node () , Self :: Or_Or_ (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Binary (x) => x . node_mut () , Self :: Boolean (x) => x . node_mut () , Self :: Call (x) => x . node_mut () , Self :: Divert (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: ListValues (x) => x . node_mut () , Self :: Number (x) => x . node_mut () , Self :: Paren (x) => x . node_mut () , Self :: Postfix (x) => x . node_mut () , Self :: QualifiedName (x) => x . node_mut () , Self :: String (x) => x . node_mut () , Self :: Unary (x) => x . node_mut () , Self :: Not_Eq_ (x) => x . node_mut () , Self :: Not_Question_ (x) => x . node_mut () , Self :: Mod_ (x) => x . node_mut () , Self :: And_And_ (x) => x . node_mut () , Self :: Mul_ (x) => x . node_mut () , Self :: Add_ (x) => x . node_mut () , Self :: Sub_ (x) => x . node_mut () , Self :: Div_ (x) => x . node_mut () , Self :: Lt_ (x) => x . node_mut () , Self :: Lt_Eq_ (x) => x . node_mut () , Self :: Eq_Eq_ (x) => x . node_mut () , Self :: Gt_ (x) => x . node_mut () , Self :: Gt_Eq_ (x) => x . node_mut () , Self :: Question_ (x) => x . node_mut () , Self :: BitXor_ (x) => x . node_mut () , Self :: And (x) => x . node_mut () , Self :: Has (x) => x . node_mut () , Self :: Hasnt (x) => x . node_mut () , Self :: Mod (x) => x . node_mut () , Self :: Or (x) => x . node_mut () , Self :: Or_Or_ (x) => x . node_mut () , } } # [inline] fn into_node (self) -> tree_sitter :: Node < 'tree > { match self { Self :: Binary (x) => x . into_node () , Self :: Boolean (x) => x . into_node () , Self :: Call (x) => x . into_node () , Self :: Divert (x) => x . into_node () , Self :: Identifier (x) => x . into_node () , Self :: ListValues (x) => x . into_node () , Self :: Number (x) => x . into_node () , Self :: Paren (x) => x . into_node () , Self :: Postfix (x) => x . into_node () , Self :: QualifiedName (x) => x . into_node () , Self :: String (x) => x . into_node () , Self :: Unary (x) => x . into_node () , Self :: Not_Eq_ (x) => x . into_node () , Self :: Not_Question_ (x) => x . into_node () , Self :: Mod_ (x) => x . into_node () , Self :: And_And_ (x) => x . into_node () , Self :: Mul_ (x) => x . into_node () , Self :: Add_ (x) => x . into_node () , Self :: Sub_ (x) => x . into_node () , Self :: Div_ (x) => x . into_node () , Self :: Lt_ (x) => x . into_node () , Self :: Lt_Eq_ (x) => x . into_node () , Self :: Eq_Eq_ (x) => x . into_node () , Self :: Gt_ (x) => x . into_node () , Self :: Gt_Eq_ (x) => x . into_node () , Self :: Question_ (x) => x . into_node () , Self :: BitXor_ (x) => x . into_node () , Self :: And (x) => x . into_node () , Self :: Has (x) => x . into_node () , Self :: Hasnt (x) => x . into_node () , Self :: Mod (x) => x . into_node () , Self :: Or (x) => x . into_node () , Self :: Or_Or_ (x) => x . into_node () , } } }
    #[doc = "one of `{!= | !? | % | && | * | + | - | / | < | <= | == | > | >= | ? | ^ | and | has | hasnt | mod | or | ||}`:\n- [symbols::Not_Eq_]\n- [symbols::Not_Question_]\n- [symbols::Mod_]\n- [symbols::And_And_]\n- [symbols::Mul_]\n- [symbols::Add_]\n- [symbols::Sub_]\n- [symbols::Div_]\n- [symbols::Lt_]\n- [symbols::Lt_Eq_]\n- [symbols::Eq_Eq_]\n- [symbols::Gt_]\n- [symbols::Gt_Eq_]\n- [symbols::Question_]\n- [symbols::BitXor_]\n- [unnamed::And]\n- [unnamed::Has]\n- [unnamed::Hasnt]\n- [unnamed::Mod]\n- [unnamed::Or]\n- [symbols::Or_Or_]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_<
        'tree,
    > {
        Not_Eq_(symbols::Not_Eq_<'tree>),
        Not_Question_(symbols::Not_Question_<'tree>),
        Mod_(symbols::Mod_<'tree>),
        And_And_(symbols::And_And_<'tree>),
        Mul_(symbols::Mul_<'tree>),
        Add_(symbols::Add_<'tree>),
        Sub_(symbols::Sub_<'tree>),
        Div_(symbols::Div_<'tree>),
        Lt_(symbols::Lt_<'tree>),
        Lt_Eq_(symbols::Lt_Eq_<'tree>),
        Eq_Eq_(symbols::Eq_Eq_<'tree>),
        Gt_(symbols::Gt_<'tree>),
        Gt_Eq_(symbols::Gt_Eq_<'tree>),
        Question_(symbols::Question_<'tree>),
        BitXor_(symbols::BitXor_<'tree>),
        And(unnamed::And<'tree>),
        Has(unnamed::Has<'tree>),
        Hasnt(unnamed::Hasnt<'tree>),
        Mod(unnamed::Mod<'tree>),
        Or(unnamed::Or<'tree>),
        Or_Or_(symbols::Or_Or_<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_ < 'tree > { # [doc = "Returns the node if it is of kind `!=` ([symbols::Not_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn not_eq_ (self) -> Option < symbols :: Not_Eq_ < 'tree > > { match self { Self :: Not_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `!?` ([symbols::Not_Question_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn not_question_ (self) -> Option < symbols :: Not_Question_ < 'tree > > { match self { Self :: Not_Question_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `%` ([symbols::Mod_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn mod_ (self) -> Option < symbols :: Mod_ < 'tree > > { match self { Self :: Mod_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `&&` ([symbols::And_And_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn and_and_ (self) -> Option < symbols :: And_And_ < 'tree > > { match self { Self :: And_And_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `*` ([symbols::Mul_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn mul_ (self) -> Option < symbols :: Mul_ < 'tree > > { match self { Self :: Mul_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `+` ([symbols::Add_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn add_ (self) -> Option < symbols :: Add_ < 'tree > > { match self { Self :: Add_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `-` ([symbols::Sub_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn sub_ (self) -> Option < symbols :: Sub_ < 'tree > > { match self { Self :: Sub_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `/` ([symbols::Div_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn div_ (self) -> Option < symbols :: Div_ < 'tree > > { match self { Self :: Div_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `<` ([symbols::Lt_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn lt_ (self) -> Option < symbols :: Lt_ < 'tree > > { match self { Self :: Lt_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `<=` ([symbols::Lt_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn lt_eq_ (self) -> Option < symbols :: Lt_Eq_ < 'tree > > { match self { Self :: Lt_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `==` ([symbols::Eq_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn eq_eq_ (self) -> Option < symbols :: Eq_Eq_ < 'tree > > { match self { Self :: Eq_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `>` ([symbols::Gt_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn gt_ (self) -> Option < symbols :: Gt_ < 'tree > > { match self { Self :: Gt_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `>=` ([symbols::Gt_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn gt_eq_ (self) -> Option < symbols :: Gt_Eq_ < 'tree > > { match self { Self :: Gt_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `?` ([symbols::Question_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn question_ (self) -> Option < symbols :: Question_ < 'tree > > { match self { Self :: Question_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `^` ([symbols::BitXor_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn bit_xor_ (self) -> Option < symbols :: BitXor_ < 'tree > > { match self { Self :: BitXor_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `and` ([unnamed::And]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn and (self) -> Option < unnamed :: And < 'tree > > { match self { Self :: And (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `has` ([unnamed::Has]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn has (self) -> Option < unnamed :: Has < 'tree > > { match self { Self :: Has (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `hasnt` ([unnamed::Hasnt]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn hasnt (self) -> Option < unnamed :: Hasnt < 'tree > > { match self { Self :: Hasnt (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `mod` ([unnamed::Mod]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn r#mod (self) -> Option < unnamed :: Mod < 'tree > > { match self { Self :: Mod (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `or` ([unnamed::Or]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn or (self) -> Option < unnamed :: Or < 'tree > > { match self { Self :: Or (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `||` ([symbols::Or_Or_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn or_or_ (self) -> Option < symbols :: Or_Or_ < 'tree > > { match self { Self :: Or_Or_ (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_ < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "!=" => Ok (unsafe { Self :: Not_Eq_ (< symbols :: Not_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "!?" => Ok (unsafe { Self :: Not_Question_ (< symbols :: Not_Question_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "%" => Ok (unsafe { Self :: Mod_ (< symbols :: Mod_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "&&" => Ok (unsafe { Self :: And_And_ (< symbols :: And_And_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "*" => Ok (unsafe { Self :: Mul_ (< symbols :: Mul_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "+" => Ok (unsafe { Self :: Add_ (< symbols :: Add_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "-" => Ok (unsafe { Self :: Sub_ (< symbols :: Sub_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "/" => Ok (unsafe { Self :: Div_ (< symbols :: Div_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "<" => Ok (unsafe { Self :: Lt_ (< symbols :: Lt_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "<=" => Ok (unsafe { Self :: Lt_Eq_ (< symbols :: Lt_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "==" => Ok (unsafe { Self :: Eq_Eq_ (< symbols :: Eq_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , ">" => Ok (unsafe { Self :: Gt_ (< symbols :: Gt_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , ">=" => Ok (unsafe { Self :: Gt_Eq_ (< symbols :: Gt_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "?" => Ok (unsafe { Self :: Question_ (< symbols :: Question_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "^" => Ok (unsafe { Self :: BitXor_ (< symbols :: BitXor_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "and" => Ok (unsafe { Self :: And (< unnamed :: And < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "has" => Ok (unsafe { Self :: Has (< unnamed :: Has < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "hasnt" => Ok (unsafe { Self :: Hasnt (< unnamed :: Hasnt < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "mod" => Ok (unsafe { Self :: Mod (< unnamed :: Mod < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "or" => Ok (unsafe { Self :: Or (< unnamed :: Or < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "||" => Ok (unsafe { Self :: Or_Or_ (< symbols :: Or_Or_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Not_Eq__Not_Question__Mod__And_And__Mul__Add__Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Or_Or_Or_ < 'tree > { const KIND : & 'static str = "{!= | !? | % | && | * | + | - | / | < | <= | == | > | >= | ? | ^ | and | has | hasnt | mod | or | ||}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Not_Eq_ (x) => x . node () , Self :: Not_Question_ (x) => x . node () , Self :: Mod_ (x) => x . node () , Self :: And_And_ (x) => x . node () , Self :: Mul_ (x) => x . node () , Self :: Add_ (x) => x . node () , Self :: Sub_ (x) => x . node () , Self :: Div_ (x) => x . node () , Self :: Lt_ (x) => x . node () , Self :: Lt_Eq_ (x) => x . node () , Self :: Eq_Eq_ (x) => x . node () , Self :: Gt_ (x) => x . node () , Self :: Gt_Eq_ (x) => x . node () , Self :: Question_ (x) => x . node () , Self :: BitXor_ (x) => x . node () , Self :: And (x) => x . node () , Self :: Has (x) => x . node () , Self :: Hasnt (x) => x . node () , Self :: Mod (x) => x . node () , Self :: Or (x) => x . node () , Self :: Or_Or_ (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Not_Eq_ (x) => x . node_mut () , Self :: Not_Question_ (x) => x . node_mut () , Self :: Mod_ (x) => x . node_mut () , Self :: And_And_ (x) => x . node_mut () , Self :: Mul_ (x) => x . node_mut () , Self :: Add_ (x) => x . node_mut () , Self :: Sub_ (x) => x . node_mut () , Self :: Div_ (x) => x . node_mut () , Self :: Lt_ (x) => x . node_mut () , Self :: Lt_Eq_ (x) => x . node_mut () , Self :: Eq_Eq_ (x) => x . node_mut () , Self :: Gt_ (x) => x . node_mut () , Self :: Gt_Eq_ (x) => x . node_mut () , Self :: Question_ (x) => x . node_mut () , Self :: BitXor_ (x) => x . node_mut () , Self :: And (x) => x . node_mut () , Self :: Has (x) => x . node_mut () , Self :: Hasnt (x) => x . node_mut () , Self :: Mod (x) => x . node_mut () , Self :: Or (x) => x . node_mut () , Self :: Or_Or_ (x) => x . node_mut () , } } # [inline] fn into_node (self) -> tree_sitter :: Node < 'tree > { match self { Self :: Not_Eq_ (x) => x . into_node () , Self :: Not_Question_ (x) => x . into_node () , Self :: Mod_ (x) => x . into_node () , Self :: And_And_ (x) => x . into_node () , Self :: Mul_ (x) => x . into_node () , Self :: Add_ (x) => x . into_node () , Self :: Sub_ (x) => x . into_node () , Self :: Div_ (x) => x . into_node () , Self :: Lt_ (x) => x . into_node () , Self :: Lt_Eq_ (x) => x . into_node () , Self :: Eq_Eq_ (x) => x . into_node () , Self :: Gt_ (x) => x . into_node () , Self :: Gt_Eq_ (x) => x . into_node () , Self :: Question_ (x) => x . into_node () , Self :: BitXor_ (x) => x . into_node () , Self :: And (x) => x . into_node () , Self :: Has (x) => x . into_node () , Self :: Hasnt (x) => x . into_node () , Self :: Mod (x) => x . into_node () , Self :: Or (x) => x . into_node () , Self :: Or_Or_ (x) => x . into_node () , } } }
    #[doc = "one of `{identifier | qualified_name}`:\n- [Identifier]\n- [QualifiedName]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Identifier_QualifiedName<'tree> {
        Identifier(Identifier<'tree>),
        QualifiedName(QualifiedName<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Identifier_QualifiedName<'tree> {
        #[doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `qualified_name` ([QualifiedName]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn qualified_name(self) -> Option<QualifiedName<'tree>> {
            match self {
                Self::QualifiedName(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Identifier_QualifiedName<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "qualified_name" => Ok(unsafe {
                    Self::QualifiedName(<QualifiedName<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Identifier_QualifiedName<'tree> {
        const KIND: &'static str = "{identifier | qualified_name}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.node(),
                Self::QualifiedName(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.node_mut(),
                Self::QualifiedName(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_node(),
                Self::QualifiedName(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary | { | }}`:\n- [Binary]\n- [Boolean]\n- [Call]\n- [Divert]\n- [Identifier]\n- [ListValues]\n- [Number]\n- [Paren]\n- [Postfix]\n- [QualifiedName]\n- [String]\n- [Unary]\n- [symbols::LBrace_]\n- [symbols::RBrace_]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_LBrace__RBrace_<
        'tree,
    > {
        Binary(Binary<'tree>),
        Boolean(Boolean<'tree>),
        Call(Call<'tree>),
        Divert(Divert<'tree>),
        Identifier(Identifier<'tree>),
        ListValues(ListValues<'tree>),
        Number(Number<'tree>),
        Paren(Paren<'tree>),
        Postfix(Postfix<'tree>),
        QualifiedName(QualifiedName<'tree>),
        String(String<'tree>),
        Unary(Unary<'tree>),
        LBrace_(symbols::LBrace_<'tree>),
        RBrace_(symbols::RBrace_<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_LBrace__RBrace_ < 'tree > { # [doc = "Returns the node if it is of kind `binary` ([Binary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn binary (self) -> Option < Binary < 'tree > > { match self { Self :: Binary (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `boolean` ([Boolean]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn boolean (self) -> Option < Boolean < 'tree > > { match self { Self :: Boolean (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `call` ([Call]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn call (self) -> Option < Call < 'tree > > { match self { Self :: Call (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `divert` ([Divert]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn divert (self) -> Option < Divert < 'tree > > { match self { Self :: Divert (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn identifier (self) -> Option < Identifier < 'tree > > { match self { Self :: Identifier (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `list_values` ([ListValues]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn list_values (self) -> Option < ListValues < 'tree > > { match self { Self :: ListValues (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `number` ([Number]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn number (self) -> Option < Number < 'tree > > { match self { Self :: Number (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `paren` ([Paren]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn paren (self) -> Option < Paren < 'tree > > { match self { Self :: Paren (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `postfix` ([Postfix]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn postfix (self) -> Option < Postfix < 'tree > > { match self { Self :: Postfix (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `qualified_name` ([QualifiedName]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn qualified_name (self) -> Option < QualifiedName < 'tree > > { match self { Self :: QualifiedName (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `string` ([String]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn string (self) -> Option < String < 'tree > > { match self { Self :: String (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `unary` ([Unary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn unary (self) -> Option < Unary < 'tree > > { match self { Self :: Unary (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `{` ([symbols::LBrace_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn l_brace_ (self) -> Option < symbols :: LBrace_ < 'tree > > { match self { Self :: LBrace_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `}` ([symbols::RBrace_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn r_brace_ (self) -> Option < symbols :: RBrace_ < 'tree > > { match self { Self :: RBrace_ (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_LBrace__RBrace_ < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "binary" => Ok (unsafe { Self :: Binary (< Binary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "boolean" => Ok (unsafe { Self :: Boolean (< Boolean < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "call" => Ok (unsafe { Self :: Call (< Call < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "divert" => Ok (unsafe { Self :: Divert (< Divert < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "identifier" => Ok (unsafe { Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "list_values" => Ok (unsafe { Self :: ListValues (< ListValues < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "number" => Ok (unsafe { Self :: Number (< Number < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "paren" => Ok (unsafe { Self :: Paren (< Paren < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "postfix" => Ok (unsafe { Self :: Postfix (< Postfix < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "qualified_name" => Ok (unsafe { Self :: QualifiedName (< QualifiedName < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "string" => Ok (unsafe { Self :: String (< String < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "unary" => Ok (unsafe { Self :: Unary (< Unary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "{" => Ok (unsafe { Self :: LBrace_ (< symbols :: LBrace_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "}" => Ok (unsafe { Self :: RBrace_ (< symbols :: RBrace_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_LBrace__RBrace_ < 'tree > { const KIND : & 'static str = "{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary | { | }}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Binary (x) => x . node () , Self :: Boolean (x) => x . node () , Self :: Call (x) => x . node () , Self :: Divert (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: ListValues (x) => x . node () , Self :: Number (x) => x . node () , Self :: Paren (x) => x . node () , Self :: Postfix (x) => x . node () , Self :: QualifiedName (x) => x . node () , Self :: String (x) => x . node () , Self :: Unary (x) => x . node () , Self :: LBrace_ (x) => x . node () , Self :: RBrace_ (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Binary (x) => x . node_mut () , Self :: Boolean (x) => x . node_mut () , Self :: Call (x) => x . node_mut () , Self :: Divert (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: ListValues (x) => x . node_mut () , Self :: Number (x) => x . node_mut () , Self :: Paren (x) => x . node_mut () , Self :: Postfix (x) => x . node_mut () , Self :: QualifiedName (x) => x . node_mut () , Self :: String (x) => x . node_mut () , Self :: Unary (x) => x . node_mut () , Self :: LBrace_ (x) => x . node_mut () , Self :: RBrace_ (x) => x . node_mut () , } } # [inline] fn into_node (self) -> tree_sitter :: Node < 'tree > { match self { Self :: Binary (x) => x . into_node () , Self :: Boolean (x) => x . into_node () , Self :: Call (x) => x . into_node () , Self :: Divert (x) => x . into_node () , Self :: Identifier (x) => x . into_node () , Self :: ListValues (x) => x . into_node () , Self :: Number (x) => x . into_node () , Self :: Paren (x) => x . into_node () , Self :: Postfix (x) => x . into_node () , Self :: QualifiedName (x) => x . into_node () , Self :: String (x) => x . into_node () , Self :: Unary (x) => x . into_node () , Self :: LBrace_ (x) => x . into_node () , Self :: RBrace_ (x) => x . into_node () , } } }
    #[doc = "one of `{( | ) | identifier}`:\n- [symbols::LParen_]\n- [symbols::RParen_]\n- [Identifier]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum LParen__RParen__Identifier<'tree> {
        LParen_(symbols::LParen_<'tree>),
        RParen_(symbols::RParen_<'tree>),
        Identifier(Identifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> LParen__RParen__Identifier<'tree> {
        #[doc = "Returns the node if it is of kind `(` ([symbols::LParen_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn l_paren_(self) -> Option<symbols::LParen_<'tree>> {
            match self {
                Self::LParen_(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `)` ([symbols::RParen_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r_paren_(self) -> Option<symbols::RParen_<'tree>> {
            match self {
                Self::RParen_(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LParen__RParen__Identifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "(" => Ok(unsafe {
                    Self::LParen_(<symbols::LParen_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                ")" => Ok(unsafe {
                    Self::RParen_(<symbols::RParen_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LParen__RParen__Identifier<'tree> {
        const KIND: &'static str = "{( | ) | identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::LParen_(x) => x.node(),
                Self::RParen_(x) => x.node(),
                Self::Identifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::LParen_(x) => x.node_mut(),
                Self::RParen_(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::LParen_(x) => x.into_node(),
                Self::RParen_(x) => x.into_node(),
                Self::Identifier(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{choice | choice_block | code | comment | content | external | gather_block | global | include | list | todo_comment}`:\n- [Choice]\n- [ChoiceBlock]\n- [Code]\n- [Comment]\n- [Content]\n- [External]\n- [GatherBlock]\n- [Global]\n- [Include]\n- [List]\n- [TodoComment]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Choice_ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment<
        'tree,
    > {
        Choice(Choice<'tree>),
        ChoiceBlock(ChoiceBlock<'tree>),
        Code(Code<'tree>),
        Comment(Comment<'tree>),
        Content(Content<'tree>),
        External(External<'tree>),
        GatherBlock(GatherBlock<'tree>),
        Global(Global<'tree>),
        Include(Include<'tree>),
        List(List<'tree>),
        TodoComment(TodoComment<'tree>),
    }
    #[automatically_derived]
    impl<'tree>
        Choice_ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment<
            'tree,
        >
    {
        #[doc = "Returns the node if it is of kind `choice` ([Choice]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn choice(self) -> Option<Choice<'tree>> {
            match self {
                Self::Choice(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `choice_block` ([ChoiceBlock]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn choice_block(self) -> Option<ChoiceBlock<'tree>> {
            match self {
                Self::ChoiceBlock(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `code` ([Code]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn code(self) -> Option<Code<'tree>> {
            match self {
                Self::Code(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `comment` ([Comment]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn comment(self) -> Option<Comment<'tree>> {
            match self {
                Self::Comment(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `content` ([Content]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn content(self) -> Option<Content<'tree>> {
            match self {
                Self::Content(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `external` ([External]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn external(self) -> Option<External<'tree>> {
            match self {
                Self::External(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `gather_block` ([GatherBlock]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn gather_block(self) -> Option<GatherBlock<'tree>> {
            match self {
                Self::GatherBlock(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `global` ([Global]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn global(self) -> Option<Global<'tree>> {
            match self {
                Self::Global(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `include` ([Include]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn include(self) -> Option<Include<'tree>> {
            match self {
                Self::Include(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `list` ([List]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn list(self) -> Option<List<'tree>> {
            match self {
                Self::List(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `todo_comment` ([TodoComment]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn todo_comment(self) -> Option<TodoComment<'tree>> {
            match self {
                Self::TodoComment(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Choice_ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "choice" => Ok (unsafe { Self :: Choice (< Choice < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "choice_block" => Ok (unsafe { Self :: ChoiceBlock (< ChoiceBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "code" => Ok (unsafe { Self :: Code (< Code < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "comment" => Ok (unsafe { Self :: Comment (< Comment < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "content" => Ok (unsafe { Self :: Content (< Content < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "external" => Ok (unsafe { Self :: External (< External < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "gather_block" => Ok (unsafe { Self :: GatherBlock (< GatherBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "global" => Ok (unsafe { Self :: Global (< Global < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "include" => Ok (unsafe { Self :: Include (< Include < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "list" => Ok (unsafe { Self :: List (< List < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "todo_comment" => Ok (unsafe { Self :: TodoComment (< TodoComment < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Choice_ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment < 'tree > { const KIND : & 'static str = "{choice | choice_block | code | comment | content | external | gather_block | global | include | list | todo_comment}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Choice (x) => x . node () , Self :: ChoiceBlock (x) => x . node () , Self :: Code (x) => x . node () , Self :: Comment (x) => x . node () , Self :: Content (x) => x . node () , Self :: External (x) => x . node () , Self :: GatherBlock (x) => x . node () , Self :: Global (x) => x . node () , Self :: Include (x) => x . node () , Self :: List (x) => x . node () , Self :: TodoComment (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Choice (x) => x . node_mut () , Self :: ChoiceBlock (x) => x . node_mut () , Self :: Code (x) => x . node_mut () , Self :: Comment (x) => x . node_mut () , Self :: Content (x) => x . node_mut () , Self :: External (x) => x . node_mut () , Self :: GatherBlock (x) => x . node_mut () , Self :: Global (x) => x . node_mut () , Self :: Include (x) => x . node_mut () , Self :: List (x) => x . node_mut () , Self :: TodoComment (x) => x . node_mut () , } } # [inline] fn into_node (self) -> tree_sitter :: Node < 'tree > { match self { Self :: Choice (x) => x . into_node () , Self :: ChoiceBlock (x) => x . into_node () , Self :: Code (x) => x . into_node () , Self :: Comment (x) => x . into_node () , Self :: Content (x) => x . into_node () , Self :: External (x) => x . into_node () , Self :: GatherBlock (x) => x . into_node () , Self :: Global (x) => x . into_node () , Self :: Include (x) => x . into_node () , Self :: List (x) => x . into_node () , Self :: TodoComment (x) => x . into_node () , } } }
    #[doc = "one of `{assignment | binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | return | string | temp | unary}`:\n- [Assignment]\n- [Binary]\n- [Boolean]\n- [Call]\n- [Divert]\n- [Identifier]\n- [ListValues]\n- [Number]\n- [Paren]\n- [Postfix]\n- [QualifiedName]\n- [Return]\n- [String]\n- [Temp]\n- [Unary]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Assignment_Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_Return_String_Temp_Unary<
        'tree,
    > {
        Assignment(Assignment<'tree>),
        Binary(Binary<'tree>),
        Boolean(Boolean<'tree>),
        Call(Call<'tree>),
        Divert(Divert<'tree>),
        Identifier(Identifier<'tree>),
        ListValues(ListValues<'tree>),
        Number(Number<'tree>),
        Paren(Paren<'tree>),
        Postfix(Postfix<'tree>),
        QualifiedName(QualifiedName<'tree>),
        Return(Return<'tree>),
        String(String<'tree>),
        Temp(Temp<'tree>),
        Unary(Unary<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > Assignment_Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_Return_String_Temp_Unary < 'tree > { # [doc = "Returns the node if it is of kind `assignment` ([Assignment]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn assignment (self) -> Option < Assignment < 'tree > > { match self { Self :: Assignment (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `binary` ([Binary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn binary (self) -> Option < Binary < 'tree > > { match self { Self :: Binary (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `boolean` ([Boolean]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn boolean (self) -> Option < Boolean < 'tree > > { match self { Self :: Boolean (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `call` ([Call]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn call (self) -> Option < Call < 'tree > > { match self { Self :: Call (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `divert` ([Divert]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn divert (self) -> Option < Divert < 'tree > > { match self { Self :: Divert (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn identifier (self) -> Option < Identifier < 'tree > > { match self { Self :: Identifier (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `list_values` ([ListValues]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn list_values (self) -> Option < ListValues < 'tree > > { match self { Self :: ListValues (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `number` ([Number]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn number (self) -> Option < Number < 'tree > > { match self { Self :: Number (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `paren` ([Paren]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn paren (self) -> Option < Paren < 'tree > > { match self { Self :: Paren (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `postfix` ([Postfix]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn postfix (self) -> Option < Postfix < 'tree > > { match self { Self :: Postfix (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `qualified_name` ([QualifiedName]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn qualified_name (self) -> Option < QualifiedName < 'tree > > { match self { Self :: QualifiedName (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `return` ([Return]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn r#return (self) -> Option < Return < 'tree > > { match self { Self :: Return (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `string` ([String]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn string (self) -> Option < String < 'tree > > { match self { Self :: String (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `temp` ([Temp]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn temp (self) -> Option < Temp < 'tree > > { match self { Self :: Temp (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `unary` ([Unary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn unary (self) -> Option < Unary < 'tree > > { match self { Self :: Unary (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Assignment_Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_Return_String_Temp_Unary < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "assignment" => Ok (unsafe { Self :: Assignment (< Assignment < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "binary" => Ok (unsafe { Self :: Binary (< Binary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "boolean" => Ok (unsafe { Self :: Boolean (< Boolean < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "call" => Ok (unsafe { Self :: Call (< Call < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "divert" => Ok (unsafe { Self :: Divert (< Divert < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "identifier" => Ok (unsafe { Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "list_values" => Ok (unsafe { Self :: ListValues (< ListValues < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "number" => Ok (unsafe { Self :: Number (< Number < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "paren" => Ok (unsafe { Self :: Paren (< Paren < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "postfix" => Ok (unsafe { Self :: Postfix (< Postfix < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "qualified_name" => Ok (unsafe { Self :: QualifiedName (< QualifiedName < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "return" => Ok (unsafe { Self :: Return (< Return < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "string" => Ok (unsafe { Self :: String (< String < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "temp" => Ok (unsafe { Self :: Temp (< Temp < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "unary" => Ok (unsafe { Self :: Unary (< Unary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Assignment_Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_Return_String_Temp_Unary < 'tree > { const KIND : & 'static str = "{assignment | binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | return | string | temp | unary}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Assignment (x) => x . node () , Self :: Binary (x) => x . node () , Self :: Boolean (x) => x . node () , Self :: Call (x) => x . node () , Self :: Divert (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: ListValues (x) => x . node () , Self :: Number (x) => x . node () , Self :: Paren (x) => x . node () , Self :: Postfix (x) => x . node () , Self :: QualifiedName (x) => x . node () , Self :: Return (x) => x . node () , Self :: String (x) => x . node () , Self :: Temp (x) => x . node () , Self :: Unary (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Assignment (x) => x . node_mut () , Self :: Binary (x) => x . node_mut () , Self :: Boolean (x) => x . node_mut () , Self :: Call (x) => x . node_mut () , Self :: Divert (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: ListValues (x) => x . node_mut () , Self :: Number (x) => x . node_mut () , Self :: Paren (x) => x . node_mut () , Self :: Postfix (x) => x . node_mut () , Self :: QualifiedName (x) => x . node_mut () , Self :: Return (x) => x . node_mut () , Self :: String (x) => x . node_mut () , Self :: Temp (x) => x . node_mut () , Self :: Unary (x) => x . node_mut () , } } # [inline] fn into_node (self) -> tree_sitter :: Node < 'tree > { match self { Self :: Assignment (x) => x . into_node () , Self :: Binary (x) => x . into_node () , Self :: Boolean (x) => x . into_node () , Self :: Call (x) => x . into_node () , Self :: Divert (x) => x . into_node () , Self :: Identifier (x) => x . into_node () , Self :: ListValues (x) => x . into_node () , Self :: Number (x) => x . into_node () , Self :: Paren (x) => x . into_node () , Self :: Postfix (x) => x . into_node () , Self :: QualifiedName (x) => x . into_node () , Self :: Return (x) => x . into_node () , Self :: String (x) => x . into_node () , Self :: Temp (x) => x . into_node () , Self :: Unary (x) => x . into_node () , } } }
    #[doc = "one of `{choice_block | code | content | external | global | include | list | todo_comment | binary | boolean | call | divert | else | identifier | list_values | number | paren | postfix | qualified_name | string | unary}`:\n- [ChoiceBlock]\n- [Code]\n- [Content]\n- [External]\n- [Global]\n- [Include]\n- [List]\n- [TodoComment]\n- [Binary]\n- [Boolean]\n- [Call]\n- [Divert]\n- [Else]\n- [Identifier]\n- [ListValues]\n- [Number]\n- [Paren]\n- [Postfix]\n- [QualifiedName]\n- [String]\n- [Unary]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment_Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary<
        'tree,
    > {
        ChoiceBlock(ChoiceBlock<'tree>),
        Code(Code<'tree>),
        Content(Content<'tree>),
        External(External<'tree>),
        Global(Global<'tree>),
        Include(Include<'tree>),
        List(List<'tree>),
        TodoComment(TodoComment<'tree>),
        Binary(Binary<'tree>),
        Boolean(Boolean<'tree>),
        Call(Call<'tree>),
        Divert(Divert<'tree>),
        Else(Else<'tree>),
        Identifier(Identifier<'tree>),
        ListValues(ListValues<'tree>),
        Number(Number<'tree>),
        Paren(Paren<'tree>),
        Postfix(Postfix<'tree>),
        QualifiedName(QualifiedName<'tree>),
        String(String<'tree>),
        Unary(Unary<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment_Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > { # [doc = "Returns the node if it is of kind `choice_block` ([ChoiceBlock]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn choice_block (self) -> Option < ChoiceBlock < 'tree > > { match self { Self :: ChoiceBlock (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `code` ([Code]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn code (self) -> Option < Code < 'tree > > { match self { Self :: Code (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `content` ([Content]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn content (self) -> Option < Content < 'tree > > { match self { Self :: Content (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `external` ([External]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn external (self) -> Option < External < 'tree > > { match self { Self :: External (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `global` ([Global]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn global (self) -> Option < Global < 'tree > > { match self { Self :: Global (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `include` ([Include]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn include (self) -> Option < Include < 'tree > > { match self { Self :: Include (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `list` ([List]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn list (self) -> Option < List < 'tree > > { match self { Self :: List (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `todo_comment` ([TodoComment]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn todo_comment (self) -> Option < TodoComment < 'tree > > { match self { Self :: TodoComment (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `binary` ([Binary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn binary (self) -> Option < Binary < 'tree > > { match self { Self :: Binary (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `boolean` ([Boolean]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn boolean (self) -> Option < Boolean < 'tree > > { match self { Self :: Boolean (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `call` ([Call]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn call (self) -> Option < Call < 'tree > > { match self { Self :: Call (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `divert` ([Divert]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn divert (self) -> Option < Divert < 'tree > > { match self { Self :: Divert (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `else` ([Else]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn r#else (self) -> Option < Else < 'tree > > { match self { Self :: Else (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn identifier (self) -> Option < Identifier < 'tree > > { match self { Self :: Identifier (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `list_values` ([ListValues]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn list_values (self) -> Option < ListValues < 'tree > > { match self { Self :: ListValues (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `number` ([Number]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn number (self) -> Option < Number < 'tree > > { match self { Self :: Number (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `paren` ([Paren]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn paren (self) -> Option < Paren < 'tree > > { match self { Self :: Paren (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `postfix` ([Postfix]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn postfix (self) -> Option < Postfix < 'tree > > { match self { Self :: Postfix (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `qualified_name` ([QualifiedName]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn qualified_name (self) -> Option < QualifiedName < 'tree > > { match self { Self :: QualifiedName (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `string` ([String]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn string (self) -> Option < String < 'tree > > { match self { Self :: String (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `unary` ([Unary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn unary (self) -> Option < Unary < 'tree > > { match self { Self :: Unary (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment_Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "choice_block" => Ok (unsafe { Self :: ChoiceBlock (< ChoiceBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "code" => Ok (unsafe { Self :: Code (< Code < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "content" => Ok (unsafe { Self :: Content (< Content < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "external" => Ok (unsafe { Self :: External (< External < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "global" => Ok (unsafe { Self :: Global (< Global < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "include" => Ok (unsafe { Self :: Include (< Include < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "list" => Ok (unsafe { Self :: List (< List < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "todo_comment" => Ok (unsafe { Self :: TodoComment (< TodoComment < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "binary" => Ok (unsafe { Self :: Binary (< Binary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "boolean" => Ok (unsafe { Self :: Boolean (< Boolean < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "call" => Ok (unsafe { Self :: Call (< Call < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "divert" => Ok (unsafe { Self :: Divert (< Divert < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "else" => Ok (unsafe { Self :: Else (< Else < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "identifier" => Ok (unsafe { Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "list_values" => Ok (unsafe { Self :: ListValues (< ListValues < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "number" => Ok (unsafe { Self :: Number (< Number < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "paren" => Ok (unsafe { Self :: Paren (< Paren < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "postfix" => Ok (unsafe { Self :: Postfix (< Postfix < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "qualified_name" => Ok (unsafe { Self :: QualifiedName (< QualifiedName < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "string" => Ok (unsafe { Self :: String (< String < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "unary" => Ok (unsafe { Self :: Unary (< Unary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for ChoiceBlock_Code_Content_External_Global_Include_List_TodoComment_Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > { const KIND : & 'static str = "{choice_block | code | content | external | global | include | list | todo_comment | binary | boolean | call | divert | else | identifier | list_values | number | paren | postfix | qualified_name | string | unary}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: ChoiceBlock (x) => x . node () , Self :: Code (x) => x . node () , Self :: Content (x) => x . node () , Self :: External (x) => x . node () , Self :: Global (x) => x . node () , Self :: Include (x) => x . node () , Self :: List (x) => x . node () , Self :: TodoComment (x) => x . node () , Self :: Binary (x) => x . node () , Self :: Boolean (x) => x . node () , Self :: Call (x) => x . node () , Self :: Divert (x) => x . node () , Self :: Else (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: ListValues (x) => x . node () , Self :: Number (x) => x . node () , Self :: Paren (x) => x . node () , Self :: Postfix (x) => x . node () , Self :: QualifiedName (x) => x . node () , Self :: String (x) => x . node () , Self :: Unary (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: ChoiceBlock (x) => x . node_mut () , Self :: Code (x) => x . node_mut () , Self :: Content (x) => x . node_mut () , Self :: External (x) => x . node_mut () , Self :: Global (x) => x . node_mut () , Self :: Include (x) => x . node_mut () , Self :: List (x) => x . node_mut () , Self :: TodoComment (x) => x . node_mut () , Self :: Binary (x) => x . node_mut () , Self :: Boolean (x) => x . node_mut () , Self :: Call (x) => x . node_mut () , Self :: Divert (x) => x . node_mut () , Self :: Else (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: ListValues (x) => x . node_mut () , Self :: Number (x) => x . node_mut () , Self :: Paren (x) => x . node_mut () , Self :: Postfix (x) => x . node_mut () , Self :: QualifiedName (x) => x . node_mut () , Self :: String (x) => x . node_mut () , Self :: Unary (x) => x . node_mut () , } } # [inline] fn into_node (self) -> tree_sitter :: Node < 'tree > { match self { Self :: ChoiceBlock (x) => x . into_node () , Self :: Code (x) => x . into_node () , Self :: Content (x) => x . into_node () , Self :: External (x) => x . into_node () , Self :: Global (x) => x . into_node () , Self :: Include (x) => x . into_node () , Self :: List (x) => x . into_node () , Self :: TodoComment (x) => x . into_node () , Self :: Binary (x) => x . into_node () , Self :: Boolean (x) => x . into_node () , Self :: Call (x) => x . into_node () , Self :: Divert (x) => x . into_node () , Self :: Else (x) => x . into_node () , Self :: Identifier (x) => x . into_node () , Self :: ListValues (x) => x . into_node () , Self :: Number (x) => x . into_node () , Self :: Paren (x) => x . into_node () , Self :: Postfix (x) => x . into_node () , Self :: QualifiedName (x) => x . into_node () , Self :: String (x) => x . into_node () , Self :: Unary (x) => x . into_node () , } } }
    #[doc = "one of `{binary | boolean | call | divert | else | identifier | list_values | number | paren | postfix | qualified_name | string | unary}`:\n- [Binary]\n- [Boolean]\n- [Call]\n- [Divert]\n- [Else]\n- [Identifier]\n- [ListValues]\n- [Number]\n- [Paren]\n- [Postfix]\n- [QualifiedName]\n- [String]\n- [Unary]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary<
        'tree,
    > {
        Binary(Binary<'tree>),
        Boolean(Boolean<'tree>),
        Call(Call<'tree>),
        Divert(Divert<'tree>),
        Else(Else<'tree>),
        Identifier(Identifier<'tree>),
        ListValues(ListValues<'tree>),
        Number(Number<'tree>),
        Paren(Paren<'tree>),
        Postfix(Postfix<'tree>),
        QualifiedName(QualifiedName<'tree>),
        String(String<'tree>),
        Unary(Unary<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > { # [doc = "Returns the node if it is of kind `binary` ([Binary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn binary (self) -> Option < Binary < 'tree > > { match self { Self :: Binary (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `boolean` ([Boolean]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn boolean (self) -> Option < Boolean < 'tree > > { match self { Self :: Boolean (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `call` ([Call]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn call (self) -> Option < Call < 'tree > > { match self { Self :: Call (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `divert` ([Divert]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn divert (self) -> Option < Divert < 'tree > > { match self { Self :: Divert (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `else` ([Else]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn r#else (self) -> Option < Else < 'tree > > { match self { Self :: Else (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn identifier (self) -> Option < Identifier < 'tree > > { match self { Self :: Identifier (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `list_values` ([ListValues]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn list_values (self) -> Option < ListValues < 'tree > > { match self { Self :: ListValues (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `number` ([Number]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn number (self) -> Option < Number < 'tree > > { match self { Self :: Number (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `paren` ([Paren]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn paren (self) -> Option < Paren < 'tree > > { match self { Self :: Paren (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `postfix` ([Postfix]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn postfix (self) -> Option < Postfix < 'tree > > { match self { Self :: Postfix (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `qualified_name` ([QualifiedName]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn qualified_name (self) -> Option < QualifiedName < 'tree > > { match self { Self :: QualifiedName (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `string` ([String]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn string (self) -> Option < String < 'tree > > { match self { Self :: String (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `unary` ([Unary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn unary (self) -> Option < Unary < 'tree > > { match self { Self :: Unary (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "binary" => Ok (unsafe { Self :: Binary (< Binary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "boolean" => Ok (unsafe { Self :: Boolean (< Boolean < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "call" => Ok (unsafe { Self :: Call (< Call < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "divert" => Ok (unsafe { Self :: Divert (< Divert < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "else" => Ok (unsafe { Self :: Else (< Else < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "identifier" => Ok (unsafe { Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "list_values" => Ok (unsafe { Self :: ListValues (< ListValues < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "number" => Ok (unsafe { Self :: Number (< Number < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "paren" => Ok (unsafe { Self :: Paren (< Paren < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "postfix" => Ok (unsafe { Self :: Postfix (< Postfix < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "qualified_name" => Ok (unsafe { Self :: QualifiedName (< QualifiedName < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "string" => Ok (unsafe { Self :: String (< String < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "unary" => Ok (unsafe { Self :: Unary (< Unary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Binary_Boolean_Call_Divert_Else_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > { const KIND : & 'static str = "{binary | boolean | call | divert | else | identifier | list_values | number | paren | postfix | qualified_name | string | unary}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Binary (x) => x . node () , Self :: Boolean (x) => x . node () , Self :: Call (x) => x . node () , Self :: Divert (x) => x . node () , Self :: Else (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: ListValues (x) => x . node () , Self :: Number (x) => x . node () , Self :: Paren (x) => x . node () , Self :: Postfix (x) => x . node () , Self :: QualifiedName (x) => x . node () , Self :: String (x) => x . node () , Self :: Unary (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Binary (x) => x . node_mut () , Self :: Boolean (x) => x . node_mut () , Self :: Call (x) => x . node_mut () , Self :: Divert (x) => x . node_mut () , Self :: Else (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: ListValues (x) => x . node_mut () , Self :: Number (x) => x . node_mut () , Self :: Paren (x) => x . node_mut () , Self :: Postfix (x) => x . node_mut () , Self :: QualifiedName (x) => x . node_mut () , Self :: String (x) => x . node_mut () , Self :: Unary (x) => x . node_mut () , } } # [inline] fn into_node (self) -> tree_sitter :: Node < 'tree > { match self { Self :: Binary (x) => x . into_node () , Self :: Boolean (x) => x . into_node () , Self :: Call (x) => x . into_node () , Self :: Divert (x) => x . into_node () , Self :: Else (x) => x . into_node () , Self :: Identifier (x) => x . into_node () , Self :: ListValues (x) => x . into_node () , Self :: Number (x) => x . into_node () , Self :: Paren (x) => x . into_node () , Self :: Postfix (x) => x . into_node () , Self :: QualifiedName (x) => x . into_node () , Self :: String (x) => x . into_node () , Self :: Unary (x) => x . into_node () , } } }
    #[doc = "one of `{content | binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}`:\n- [Content]\n- [Binary]\n- [Boolean]\n- [Call]\n- [Divert]\n- [Identifier]\n- [ListValues]\n- [Number]\n- [Paren]\n- [Postfix]\n- [QualifiedName]\n- [String]\n- [Unary]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Content_Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary<
        'tree,
    > {
        Content(Content<'tree>),
        Binary(Binary<'tree>),
        Boolean(Boolean<'tree>),
        Call(Call<'tree>),
        Divert(Divert<'tree>),
        Identifier(Identifier<'tree>),
        ListValues(ListValues<'tree>),
        Number(Number<'tree>),
        Paren(Paren<'tree>),
        Postfix(Postfix<'tree>),
        QualifiedName(QualifiedName<'tree>),
        String(String<'tree>),
        Unary(Unary<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > Content_Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > { # [doc = "Returns the node if it is of kind `content` ([Content]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn content (self) -> Option < Content < 'tree > > { match self { Self :: Content (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `binary` ([Binary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn binary (self) -> Option < Binary < 'tree > > { match self { Self :: Binary (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `boolean` ([Boolean]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn boolean (self) -> Option < Boolean < 'tree > > { match self { Self :: Boolean (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `call` ([Call]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn call (self) -> Option < Call < 'tree > > { match self { Self :: Call (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `divert` ([Divert]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn divert (self) -> Option < Divert < 'tree > > { match self { Self :: Divert (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn identifier (self) -> Option < Identifier < 'tree > > { match self { Self :: Identifier (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `list_values` ([ListValues]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn list_values (self) -> Option < ListValues < 'tree > > { match self { Self :: ListValues (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `number` ([Number]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn number (self) -> Option < Number < 'tree > > { match self { Self :: Number (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `paren` ([Paren]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn paren (self) -> Option < Paren < 'tree > > { match self { Self :: Paren (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `postfix` ([Postfix]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn postfix (self) -> Option < Postfix < 'tree > > { match self { Self :: Postfix (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `qualified_name` ([QualifiedName]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn qualified_name (self) -> Option < QualifiedName < 'tree > > { match self { Self :: QualifiedName (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `string` ([String]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn string (self) -> Option < String < 'tree > > { match self { Self :: String (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `unary` ([Unary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn unary (self) -> Option < Unary < 'tree > > { match self { Self :: Unary (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Content_Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "content" => Ok (unsafe { Self :: Content (< Content < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "binary" => Ok (unsafe { Self :: Binary (< Binary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "boolean" => Ok (unsafe { Self :: Boolean (< Boolean < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "call" => Ok (unsafe { Self :: Call (< Call < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "divert" => Ok (unsafe { Self :: Divert (< Divert < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "identifier" => Ok (unsafe { Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "list_values" => Ok (unsafe { Self :: ListValues (< ListValues < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "number" => Ok (unsafe { Self :: Number (< Number < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "paren" => Ok (unsafe { Self :: Paren (< Paren < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "postfix" => Ok (unsafe { Self :: Postfix (< Postfix < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "qualified_name" => Ok (unsafe { Self :: QualifiedName (< QualifiedName < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "string" => Ok (unsafe { Self :: String (< String < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "unary" => Ok (unsafe { Self :: Unary (< Unary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Content_Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary < 'tree > { const KIND : & 'static str = "{content | binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Content (x) => x . node () , Self :: Binary (x) => x . node () , Self :: Boolean (x) => x . node () , Self :: Call (x) => x . node () , Self :: Divert (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: ListValues (x) => x . node () , Self :: Number (x) => x . node () , Self :: Paren (x) => x . node () , Self :: Postfix (x) => x . node () , Self :: QualifiedName (x) => x . node () , Self :: String (x) => x . node () , Self :: Unary (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Content (x) => x . node_mut () , Self :: Binary (x) => x . node_mut () , Self :: Boolean (x) => x . node_mut () , Self :: Call (x) => x . node_mut () , Self :: Divert (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: ListValues (x) => x . node_mut () , Self :: Number (x) => x . node_mut () , Self :: Paren (x) => x . node_mut () , Self :: Postfix (x) => x . node_mut () , Self :: QualifiedName (x) => x . node_mut () , Self :: String (x) => x . node_mut () , Self :: Unary (x) => x . node_mut () , } } # [inline] fn into_node (self) -> tree_sitter :: Node < 'tree > { match self { Self :: Content (x) => x . into_node () , Self :: Binary (x) => x . into_node () , Self :: Boolean (x) => x . into_node () , Self :: Call (x) => x . into_node () , Self :: Divert (x) => x . into_node () , Self :: Identifier (x) => x . into_node () , Self :: ListValues (x) => x . into_node () , Self :: Number (x) => x . into_node () , Self :: Paren (x) => x . into_node () , Self :: Postfix (x) => x . into_node () , Self :: QualifiedName (x) => x . into_node () , Self :: String (x) => x . into_node () , Self :: Unary (x) => x . into_node () , } } }
    #[doc = "one of `{alternatives | cond_block | conditional_text | divert | eval | glue | multiline_alternatives | tag | text | thread | tunnel}`:\n- [Alternatives]\n- [CondBlock]\n- [ConditionalText]\n- [Divert]\n- [Eval]\n- [Glue]\n- [MultilineAlternatives]\n- [Tag]\n- [Text]\n- [Thread]\n- [Tunnel]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Alternatives_CondBlock_ConditionalText_Divert_Eval_Glue_MultilineAlternatives_Tag_Text_Thread_Tunnel<
        'tree,
    > {
        Alternatives(Alternatives<'tree>),
        CondBlock(CondBlock<'tree>),
        ConditionalText(ConditionalText<'tree>),
        Divert(Divert<'tree>),
        Eval(Eval<'tree>),
        Glue(Glue<'tree>),
        MultilineAlternatives(MultilineAlternatives<'tree>),
        Tag(Tag<'tree>),
        Text(Text<'tree>),
        Thread(Thread<'tree>),
        Tunnel(Tunnel<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > Alternatives_CondBlock_ConditionalText_Divert_Eval_Glue_MultilineAlternatives_Tag_Text_Thread_Tunnel < 'tree > { # [doc = "Returns the node if it is of kind `alternatives` ([Alternatives]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn alternatives (self) -> Option < Alternatives < 'tree > > { match self { Self :: Alternatives (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `cond_block` ([CondBlock]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn cond_block (self) -> Option < CondBlock < 'tree > > { match self { Self :: CondBlock (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `conditional_text` ([ConditionalText]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn conditional_text (self) -> Option < ConditionalText < 'tree > > { match self { Self :: ConditionalText (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `divert` ([Divert]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn divert (self) -> Option < Divert < 'tree > > { match self { Self :: Divert (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `eval` ([Eval]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn eval (self) -> Option < Eval < 'tree > > { match self { Self :: Eval (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `glue` ([Glue]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn glue (self) -> Option < Glue < 'tree > > { match self { Self :: Glue (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `multiline_alternatives` ([MultilineAlternatives]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn multiline_alternatives (self) -> Option < MultilineAlternatives < 'tree > > { match self { Self :: MultilineAlternatives (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `tag` ([Tag]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn tag (self) -> Option < Tag < 'tree > > { match self { Self :: Tag (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `text` ([Text]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn text (self) -> Option < Text < 'tree > > { match self { Self :: Text (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `thread` ([Thread]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn thread (self) -> Option < Thread < 'tree > > { match self { Self :: Thread (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `tunnel` ([Tunnel]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn tunnel (self) -> Option < Tunnel < 'tree > > { match self { Self :: Tunnel (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Alternatives_CondBlock_ConditionalText_Divert_Eval_Glue_MultilineAlternatives_Tag_Text_Thread_Tunnel < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "alternatives" => Ok (unsafe { Self :: Alternatives (< Alternatives < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "cond_block" => Ok (unsafe { Self :: CondBlock (< CondBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "conditional_text" => Ok (unsafe { Self :: ConditionalText (< ConditionalText < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "divert" => Ok (unsafe { Self :: Divert (< Divert < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "eval" => Ok (unsafe { Self :: Eval (< Eval < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "glue" => Ok (unsafe { Self :: Glue (< Glue < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "multiline_alternatives" => Ok (unsafe { Self :: MultilineAlternatives (< MultilineAlternatives < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "tag" => Ok (unsafe { Self :: Tag (< Tag < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "text" => Ok (unsafe { Self :: Text (< Text < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "thread" => Ok (unsafe { Self :: Thread (< Thread < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "tunnel" => Ok (unsafe { Self :: Tunnel (< Tunnel < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Alternatives_CondBlock_ConditionalText_Divert_Eval_Glue_MultilineAlternatives_Tag_Text_Thread_Tunnel < 'tree > { const KIND : & 'static str = "{alternatives | cond_block | conditional_text | divert | eval | glue | multiline_alternatives | tag | text | thread | tunnel}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Alternatives (x) => x . node () , Self :: CondBlock (x) => x . node () , Self :: ConditionalText (x) => x . node () , Self :: Divert (x) => x . node () , Self :: Eval (x) => x . node () , Self :: Glue (x) => x . node () , Self :: MultilineAlternatives (x) => x . node () , Self :: Tag (x) => x . node () , Self :: Text (x) => x . node () , Self :: Thread (x) => x . node () , Self :: Tunnel (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Alternatives (x) => x . node_mut () , Self :: CondBlock (x) => x . node_mut () , Self :: ConditionalText (x) => x . node_mut () , Self :: Divert (x) => x . node_mut () , Self :: Eval (x) => x . node_mut () , Self :: Glue (x) => x . node_mut () , Self :: MultilineAlternatives (x) => x . node_mut () , Self :: Tag (x) => x . node_mut () , Self :: Text (x) => x . node_mut () , Self :: Thread (x) => x . node_mut () , Self :: Tunnel (x) => x . node_mut () , } } # [inline] fn into_node (self) -> tree_sitter :: Node < 'tree > { match self { Self :: Alternatives (x) => x . into_node () , Self :: CondBlock (x) => x . into_node () , Self :: ConditionalText (x) => x . into_node () , Self :: Divert (x) => x . into_node () , Self :: Eval (x) => x . into_node () , Self :: Glue (x) => x . into_node () , Self :: MultilineAlternatives (x) => x . into_node () , Self :: Tag (x) => x . into_node () , Self :: Text (x) => x . into_node () , Self :: Thread (x) => x . into_node () , Self :: Tunnel (x) => x . into_node () , } } }
    #[doc = "one of `{choice_block | code | comment | content | external | gather_block | global | include | list | todo_comment}`:\n- [ChoiceBlock]\n- [Code]\n- [Comment]\n- [Content]\n- [External]\n- [GatherBlock]\n- [Global]\n- [Include]\n- [List]\n- [TodoComment]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment<
        'tree,
    > {
        ChoiceBlock(ChoiceBlock<'tree>),
        Code(Code<'tree>),
        Comment(Comment<'tree>),
        Content(Content<'tree>),
        External(External<'tree>),
        GatherBlock(GatherBlock<'tree>),
        Global(Global<'tree>),
        Include(Include<'tree>),
        List(List<'tree>),
        TodoComment(TodoComment<'tree>),
    }
    #[automatically_derived]
    impl<'tree>
        ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment<'tree>
    {
        #[doc = "Returns the node if it is of kind `choice_block` ([ChoiceBlock]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn choice_block(self) -> Option<ChoiceBlock<'tree>> {
            match self {
                Self::ChoiceBlock(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `code` ([Code]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn code(self) -> Option<Code<'tree>> {
            match self {
                Self::Code(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `comment` ([Comment]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn comment(self) -> Option<Comment<'tree>> {
            match self {
                Self::Comment(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `content` ([Content]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn content(self) -> Option<Content<'tree>> {
            match self {
                Self::Content(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `external` ([External]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn external(self) -> Option<External<'tree>> {
            match self {
                Self::External(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `gather_block` ([GatherBlock]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn gather_block(self) -> Option<GatherBlock<'tree>> {
            match self {
                Self::GatherBlock(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `global` ([Global]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn global(self) -> Option<Global<'tree>> {
            match self {
                Self::Global(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `include` ([Include]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn include(self) -> Option<Include<'tree>> {
            match self {
                Self::Include(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `list` ([List]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn list(self) -> Option<List<'tree>> {
            match self {
                Self::List(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `todo_comment` ([TodoComment]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn todo_comment(self) -> Option<TodoComment<'tree>> {
            match self {
                Self::TodoComment(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment<
            'tree,
        >
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "choice_block" => Ok(unsafe {
                    Self :: ChoiceBlock (< ChoiceBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "code" => Ok(unsafe {
                    Self::Code(
                        <Code<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                            node,
                        ),
                    )
                }),
                "comment" => {
                    Ok(unsafe {
                        Self :: Comment (< Comment < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "content" => {
                    Ok(unsafe {
                        Self :: Content (< Content < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "external" => {
                    Ok(unsafe {
                        Self :: External (< External < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "gather_block" => Ok(unsafe {
                    Self :: GatherBlock (< GatherBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "global" => {
                    Ok(unsafe {
                        Self :: Global (< Global < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "include" => {
                    Ok(unsafe {
                        Self :: Include (< Include < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "list" => Ok(unsafe {
                    Self::List(
                        <List<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                            node,
                        ),
                    )
                }),
                "todo_comment" => Ok(unsafe {
                    Self :: TodoComment (< TodoComment < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for ChoiceBlock_Code_Comment_Content_External_GatherBlock_Global_Include_List_TodoComment<
            'tree,
        >
    {
        const KIND : & 'static str = "{choice_block | code | comment | content | external | gather_block | global | include | list | todo_comment}" ;
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::ChoiceBlock(x) => x.node(),
                Self::Code(x) => x.node(),
                Self::Comment(x) => x.node(),
                Self::Content(x) => x.node(),
                Self::External(x) => x.node(),
                Self::GatherBlock(x) => x.node(),
                Self::Global(x) => x.node(),
                Self::Include(x) => x.node(),
                Self::List(x) => x.node(),
                Self::TodoComment(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::ChoiceBlock(x) => x.node_mut(),
                Self::Code(x) => x.node_mut(),
                Self::Comment(x) => x.node_mut(),
                Self::Content(x) => x.node_mut(),
                Self::External(x) => x.node_mut(),
                Self::GatherBlock(x) => x.node_mut(),
                Self::Global(x) => x.node_mut(),
                Self::Include(x) => x.node_mut(),
                Self::List(x) => x.node_mut(),
                Self::TodoComment(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::ChoiceBlock(x) => x.into_node(),
                Self::Code(x) => x.into_node(),
                Self::Comment(x) => x.into_node(),
                Self::Content(x) => x.into_node(),
                Self::External(x) => x.into_node(),
                Self::GatherBlock(x) => x.into_node(),
                Self::Global(x) => x.into_node(),
                Self::Include(x) => x.into_node(),
                Self::List(x) => x.into_node(),
                Self::TodoComment(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{call | identifier | qualified_name}`:\n- [Call]\n- [Identifier]\n- [QualifiedName]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Call_Identifier_QualifiedName<'tree> {
        Call(Call<'tree>),
        Identifier(Identifier<'tree>),
        QualifiedName(QualifiedName<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Call_Identifier_QualifiedName<'tree> {
        #[doc = "Returns the node if it is of kind `call` ([Call]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn call(self) -> Option<Call<'tree>> {
            match self {
                Self::Call(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `qualified_name` ([QualifiedName]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn qualified_name(self) -> Option<QualifiedName<'tree>> {
            match self {
                Self::QualifiedName(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Call_Identifier_QualifiedName<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "call" => Ok(unsafe {
                    Self::Call(
                        <Call<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                            node,
                        ),
                    )
                }),
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "qualified_name" => Ok(unsafe {
                    Self::QualifiedName(<QualifiedName<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Call_Identifier_QualifiedName<'tree> {
        const KIND: &'static str = "{call | identifier | qualified_name}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Call(x) => x.node(),
                Self::Identifier(x) => x.node(),
                Self::QualifiedName(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Call(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
                Self::QualifiedName(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::Call(x) => x.into_node(),
                Self::Identifier(x) => x.into_node(),
                Self::QualifiedName(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{( | ) | params}`:\n- [symbols::LParen_]\n- [symbols::RParen_]\n- [Params]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum LParen__RParen__Params<'tree> {
        LParen_(symbols::LParen_<'tree>),
        RParen_(symbols::RParen_<'tree>),
        Params(Params<'tree>),
    }
    #[automatically_derived]
    impl<'tree> LParen__RParen__Params<'tree> {
        #[doc = "Returns the node if it is of kind `(` ([symbols::LParen_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn l_paren_(self) -> Option<symbols::LParen_<'tree>> {
            match self {
                Self::LParen_(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `)` ([symbols::RParen_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r_paren_(self) -> Option<symbols::RParen_<'tree>> {
            match self {
                Self::RParen_(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `params` ([Params]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn params(self) -> Option<Params<'tree>> {
            match self {
                Self::Params(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LParen__RParen__Params<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "(" => Ok(unsafe {
                    Self::LParen_(<symbols::LParen_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                ")" => Ok(unsafe {
                    Self::RParen_(<symbols::RParen_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "params" => {
                    Ok(unsafe {
                        Self :: Params (< Params < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LParen__RParen__Params<'tree> {
        const KIND: &'static str = "{( | ) | params}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::LParen_(x) => x.node(),
                Self::RParen_(x) => x.node(),
                Self::Params(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::LParen_(x) => x.node_mut(),
                Self::RParen_(x) => x.node_mut(),
                Self::Params(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::LParen_(x) => x.into_node(),
                Self::RParen_(x) => x.into_node(),
                Self::Params(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{content | divert | thread | tunnel | ( | ) | identifier}`:\n- [Content]\n- [Divert]\n- [Thread]\n- [Tunnel]\n- [symbols::LParen_]\n- [symbols::RParen_]\n- [Identifier]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Content_Divert_Thread_Tunnel_LParen__RParen__Identifier<'tree> {
        Content(Content<'tree>),
        Divert(Divert<'tree>),
        Thread(Thread<'tree>),
        Tunnel(Tunnel<'tree>),
        LParen_(symbols::LParen_<'tree>),
        RParen_(symbols::RParen_<'tree>),
        Identifier(Identifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Content_Divert_Thread_Tunnel_LParen__RParen__Identifier<'tree> {
        #[doc = "Returns the node if it is of kind `content` ([Content]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn content(self) -> Option<Content<'tree>> {
            match self {
                Self::Content(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `divert` ([Divert]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn divert(self) -> Option<Divert<'tree>> {
            match self {
                Self::Divert(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `thread` ([Thread]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn thread(self) -> Option<Thread<'tree>> {
            match self {
                Self::Thread(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `tunnel` ([Tunnel]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn tunnel(self) -> Option<Tunnel<'tree>> {
            match self {
                Self::Tunnel(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `(` ([symbols::LParen_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn l_paren_(self) -> Option<symbols::LParen_<'tree>> {
            match self {
                Self::LParen_(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `)` ([symbols::RParen_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r_paren_(self) -> Option<symbols::RParen_<'tree>> {
            match self {
                Self::RParen_(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for Content_Divert_Thread_Tunnel_LParen__RParen__Identifier<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "content" => {
                    Ok(unsafe {
                        Self :: Content (< Content < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "divert" => {
                    Ok(unsafe {
                        Self :: Divert (< Divert < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "thread" => {
                    Ok(unsafe {
                        Self :: Thread (< Thread < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "tunnel" => {
                    Ok(unsafe {
                        Self :: Tunnel (< Tunnel < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "(" => Ok(unsafe {
                    Self::LParen_(<symbols::LParen_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                ")" => Ok(unsafe {
                    Self::RParen_(<symbols::RParen_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for Content_Divert_Thread_Tunnel_LParen__RParen__Identifier<'tree>
    {
        const KIND: &'static str = "{content | divert | thread | tunnel | ( | ) | identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Content(x) => x.node(),
                Self::Divert(x) => x.node(),
                Self::Thread(x) => x.node(),
                Self::Tunnel(x) => x.node(),
                Self::LParen_(x) => x.node(),
                Self::RParen_(x) => x.node(),
                Self::Identifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Content(x) => x.node_mut(),
                Self::Divert(x) => x.node_mut(),
                Self::Thread(x) => x.node_mut(),
                Self::Tunnel(x) => x.node_mut(),
                Self::LParen_(x) => x.node_mut(),
                Self::RParen_(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::Content(x) => x.into_node(),
                Self::Divert(x) => x.into_node(),
                Self::Thread(x) => x.into_node(),
                Self::Tunnel(x) => x.into_node(),
                Self::LParen_(x) => x.into_node(),
                Self::RParen_(x) => x.into_node(),
                Self::Identifier(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{choice_block | code | comment | content | external | gather | gather_block | global | include | list | todo_comment}`:\n- [ChoiceBlock]\n- [Code]\n- [Comment]\n- [Content]\n- [External]\n- [Gather]\n- [GatherBlock]\n- [Global]\n- [Include]\n- [List]\n- [TodoComment]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ChoiceBlock_Code_Comment_Content_External_Gather_GatherBlock_Global_Include_List_TodoComment<
        'tree,
    > {
        ChoiceBlock(ChoiceBlock<'tree>),
        Code(Code<'tree>),
        Comment(Comment<'tree>),
        Content(Content<'tree>),
        External(External<'tree>),
        Gather(Gather<'tree>),
        GatherBlock(GatherBlock<'tree>),
        Global(Global<'tree>),
        Include(Include<'tree>),
        List(List<'tree>),
        TodoComment(TodoComment<'tree>),
    }
    #[automatically_derived]
    impl<'tree>
        ChoiceBlock_Code_Comment_Content_External_Gather_GatherBlock_Global_Include_List_TodoComment<
            'tree,
        >
    {
        #[doc = "Returns the node if it is of kind `choice_block` ([ChoiceBlock]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn choice_block(self) -> Option<ChoiceBlock<'tree>> {
            match self {
                Self::ChoiceBlock(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `code` ([Code]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn code(self) -> Option<Code<'tree>> {
            match self {
                Self::Code(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `comment` ([Comment]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn comment(self) -> Option<Comment<'tree>> {
            match self {
                Self::Comment(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `content` ([Content]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn content(self) -> Option<Content<'tree>> {
            match self {
                Self::Content(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `external` ([External]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn external(self) -> Option<External<'tree>> {
            match self {
                Self::External(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `gather` ([Gather]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn gather(self) -> Option<Gather<'tree>> {
            match self {
                Self::Gather(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `gather_block` ([GatherBlock]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn gather_block(self) -> Option<GatherBlock<'tree>> {
            match self {
                Self::GatherBlock(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `global` ([Global]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn global(self) -> Option<Global<'tree>> {
            match self {
                Self::Global(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `include` ([Include]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn include(self) -> Option<Include<'tree>> {
            match self {
                Self::Include(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `list` ([List]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn list(self) -> Option<List<'tree>> {
            match self {
                Self::List(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `todo_comment` ([TodoComment]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn todo_comment(self) -> Option<TodoComment<'tree>> {
            match self {
                Self::TodoComment(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for ChoiceBlock_Code_Comment_Content_External_Gather_GatherBlock_Global_Include_List_TodoComment < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "choice_block" => Ok (unsafe { Self :: ChoiceBlock (< ChoiceBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "code" => Ok (unsafe { Self :: Code (< Code < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "comment" => Ok (unsafe { Self :: Comment (< Comment < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "content" => Ok (unsafe { Self :: Content (< Content < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "external" => Ok (unsafe { Self :: External (< External < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "gather" => Ok (unsafe { Self :: Gather (< Gather < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "gather_block" => Ok (unsafe { Self :: GatherBlock (< GatherBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "global" => Ok (unsafe { Self :: Global (< Global < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "include" => Ok (unsafe { Self :: Include (< Include < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "list" => Ok (unsafe { Self :: List (< List < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "todo_comment" => Ok (unsafe { Self :: TodoComment (< TodoComment < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for ChoiceBlock_Code_Comment_Content_External_Gather_GatherBlock_Global_Include_List_TodoComment < 'tree > { const KIND : & 'static str = "{choice_block | code | comment | content | external | gather | gather_block | global | include | list | todo_comment}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: ChoiceBlock (x) => x . node () , Self :: Code (x) => x . node () , Self :: Comment (x) => x . node () , Self :: Content (x) => x . node () , Self :: External (x) => x . node () , Self :: Gather (x) => x . node () , Self :: GatherBlock (x) => x . node () , Self :: Global (x) => x . node () , Self :: Include (x) => x . node () , Self :: List (x) => x . node () , Self :: TodoComment (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: ChoiceBlock (x) => x . node_mut () , Self :: Code (x) => x . node_mut () , Self :: Comment (x) => x . node_mut () , Self :: Content (x) => x . node_mut () , Self :: External (x) => x . node_mut () , Self :: Gather (x) => x . node_mut () , Self :: GatherBlock (x) => x . node_mut () , Self :: Global (x) => x . node_mut () , Self :: Include (x) => x . node_mut () , Self :: List (x) => x . node_mut () , Self :: TodoComment (x) => x . node_mut () , } } # [inline] fn into_node (self) -> tree_sitter :: Node < 'tree > { match self { Self :: ChoiceBlock (x) => x . into_node () , Self :: Code (x) => x . into_node () , Self :: Comment (x) => x . into_node () , Self :: Content (x) => x . into_node () , Self :: External (x) => x . into_node () , Self :: Gather (x) => x . into_node () , Self :: GatherBlock (x) => x . into_node () , Self :: Global (x) => x . into_node () , Self :: Include (x) => x . into_node () , Self :: List (x) => x . into_node () , Self :: TodoComment (x) => x . into_node () , } } }
    #[doc = "one of `{CONST | VAR}`:\n- [unnamed::Const]\n- [unnamed::Var]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Const_Var<'tree> {
        Const(unnamed::Const<'tree>),
        Var(unnamed::Var<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Const_Var<'tree> {
        #[doc = "Returns the node if it is of kind `CONST` ([unnamed::Const]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#const(self) -> Option<unnamed::Const<'tree>> {
            match self {
                Self::Const(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `VAR` ([unnamed::Var]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn var(self) -> Option<unnamed::Var<'tree>> {
            match self {
                Self::Var(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Const_Var<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "CONST" => Ok(unsafe {
                    Self::Const(<unnamed::Const<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "VAR" => Ok(unsafe {
                    Self :: Var (< unnamed :: Var < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Const_Var<'tree> {
        const KIND: &'static str = "{CONST | VAR}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Const(x) => x.node(),
                Self::Var(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Const(x) => x.node_mut(),
                Self::Var(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::Const(x) => x.into_node(),
                Self::Var(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{content_block | knot_block | stitch_block}`:\n- [ContentBlock]\n- [KnotBlock]\n- [StitchBlock]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ContentBlock_KnotBlock_StitchBlock<'tree> {
        ContentBlock(ContentBlock<'tree>),
        KnotBlock(KnotBlock<'tree>),
        StitchBlock(StitchBlock<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ContentBlock_KnotBlock_StitchBlock<'tree> {
        #[doc = "Returns the node if it is of kind `content_block` ([ContentBlock]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn content_block(self) -> Option<ContentBlock<'tree>> {
            match self {
                Self::ContentBlock(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `knot_block` ([KnotBlock]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn knot_block(self) -> Option<KnotBlock<'tree>> {
            match self {
                Self::KnotBlock(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `stitch_block` ([StitchBlock]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn stitch_block(self) -> Option<StitchBlock<'tree>> {
            match self {
                Self::StitchBlock(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ContentBlock_KnotBlock_StitchBlock<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "content_block" => Ok(unsafe {
                    Self :: ContentBlock (< ContentBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "knot_block" => Ok(unsafe {
                    Self :: KnotBlock (< KnotBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "stitch_block" => Ok(unsafe {
                    Self :: StitchBlock (< StitchBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for ContentBlock_KnotBlock_StitchBlock<'tree> {
        const KIND: &'static str = "{content_block | knot_block | stitch_block}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::ContentBlock(x) => x.node(),
                Self::KnotBlock(x) => x.node(),
                Self::StitchBlock(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::ContentBlock(x) => x.node_mut(),
                Self::KnotBlock(x) => x.node_mut(),
                Self::StitchBlock(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::ContentBlock(x) => x.into_node(),
                Self::KnotBlock(x) => x.into_node(),
                Self::StitchBlock(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{content_block | knot | stitch_block}`:\n- [ContentBlock]\n- [Knot]\n- [StitchBlock]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ContentBlock_Knot_StitchBlock<'tree> {
        ContentBlock(ContentBlock<'tree>),
        Knot(Knot<'tree>),
        StitchBlock(StitchBlock<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ContentBlock_Knot_StitchBlock<'tree> {
        #[doc = "Returns the node if it is of kind `content_block` ([ContentBlock]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn content_block(self) -> Option<ContentBlock<'tree>> {
            match self {
                Self::ContentBlock(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `knot` ([Knot]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn knot(self) -> Option<Knot<'tree>> {
            match self {
                Self::Knot(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `stitch_block` ([StitchBlock]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn stitch_block(self) -> Option<StitchBlock<'tree>> {
            match self {
                Self::StitchBlock(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ContentBlock_Knot_StitchBlock<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "content_block" => Ok(unsafe {
                    Self :: ContentBlock (< ContentBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "knot" => Ok(unsafe {
                    Self::Knot(
                        <Knot<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                            node,
                        ),
                    )
                }),
                "stitch_block" => Ok(unsafe {
                    Self :: StitchBlock (< StitchBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for ContentBlock_Knot_StitchBlock<'tree> {
        const KIND: &'static str = "{content_block | knot | stitch_block}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::ContentBlock(x) => x.node(),
                Self::Knot(x) => x.node(),
                Self::StitchBlock(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::ContentBlock(x) => x.node_mut(),
                Self::Knot(x) => x.node_mut(),
                Self::StitchBlock(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::ContentBlock(x) => x.into_node(),
                Self::Knot(x) => x.into_node(),
                Self::StitchBlock(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{( | )}`:\n- [symbols::LParen_]\n- [symbols::RParen_]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum LParen__RParen_<'tree> {
        LParen_(symbols::LParen_<'tree>),
        RParen_(symbols::RParen_<'tree>),
    }
    #[automatically_derived]
    impl<'tree> LParen__RParen_<'tree> {
        #[doc = "Returns the node if it is of kind `(` ([symbols::LParen_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn l_paren_(self) -> Option<symbols::LParen_<'tree>> {
            match self {
                Self::LParen_(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `)` ([symbols::RParen_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r_paren_(self) -> Option<symbols::RParen_<'tree>> {
            match self {
                Self::RParen_(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LParen__RParen_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "(" => Ok(unsafe {
                    Self::LParen_(<symbols::LParen_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                ")" => Ok(unsafe {
                    Self::RParen_(<symbols::RParen_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for LParen__RParen_<'tree> {
        const KIND: &'static str = "{( | )}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::LParen_(x) => x.node(),
                Self::RParen_(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::LParen_(x) => x.node_mut(),
                Self::RParen_(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::LParen_(x) => x.into_node(),
                Self::RParen_(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{alt_arm | cycle | once | shuffle | stopping}`:\n- [AltArm]\n- [unnamed::Cycle]\n- [unnamed::Once]\n- [unnamed::Shuffle]\n- [unnamed::Stopping]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum AltArm_Cycle_Once_Shuffle_Stopping<'tree> {
        AltArm(AltArm<'tree>),
        Cycle(unnamed::Cycle<'tree>),
        Once(unnamed::Once<'tree>),
        Shuffle(unnamed::Shuffle<'tree>),
        Stopping(unnamed::Stopping<'tree>),
    }
    #[automatically_derived]
    impl<'tree> AltArm_Cycle_Once_Shuffle_Stopping<'tree> {
        #[doc = "Returns the node if it is of kind `alt_arm` ([AltArm]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn alt_arm(self) -> Option<AltArm<'tree>> {
            match self {
                Self::AltArm(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `cycle` ([unnamed::Cycle]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn cycle(self) -> Option<unnamed::Cycle<'tree>> {
            match self {
                Self::Cycle(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `once` ([unnamed::Once]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn once(self) -> Option<unnamed::Once<'tree>> {
            match self {
                Self::Once(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `shuffle` ([unnamed::Shuffle]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn shuffle(self) -> Option<unnamed::Shuffle<'tree>> {
            match self {
                Self::Shuffle(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `stopping` ([unnamed::Stopping]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn stopping(self) -> Option<unnamed::Stopping<'tree>> {
            match self {
                Self::Stopping(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AltArm_Cycle_Once_Shuffle_Stopping<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "alt_arm" => {
                    Ok(unsafe {
                        Self :: AltArm (< AltArm < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "cycle" => Ok(unsafe {
                    Self::Cycle(<unnamed::Cycle<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "once" => Ok(unsafe {
                    Self::Once(<unnamed::Once<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "shuffle" => Ok(unsafe {
                    Self::Shuffle(<unnamed::Shuffle<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "stopping" => Ok(unsafe {
                    Self::Stopping(<unnamed::Stopping<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for AltArm_Cycle_Once_Shuffle_Stopping<'tree> {
        const KIND: &'static str = "{alt_arm | cycle | once | shuffle | stopping}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::AltArm(x) => x.node(),
                Self::Cycle(x) => x.node(),
                Self::Once(x) => x.node(),
                Self::Shuffle(x) => x.node(),
                Self::Stopping(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::AltArm(x) => x.node_mut(),
                Self::Cycle(x) => x.node_mut(),
                Self::Once(x) => x.node_mut(),
                Self::Shuffle(x) => x.node_mut(),
                Self::Stopping(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::AltArm(x) => x.into_node(),
                Self::Cycle(x) => x.into_node(),
                Self::Once(x) => x.into_node(),
                Self::Shuffle(x) => x.into_node(),
                Self::Stopping(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{cycle | once | shuffle | stopping}`:\n- [unnamed::Cycle]\n- [unnamed::Once]\n- [unnamed::Shuffle]\n- [unnamed::Stopping]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Cycle_Once_Shuffle_Stopping<'tree> {
        Cycle(unnamed::Cycle<'tree>),
        Once(unnamed::Once<'tree>),
        Shuffle(unnamed::Shuffle<'tree>),
        Stopping(unnamed::Stopping<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Cycle_Once_Shuffle_Stopping<'tree> {
        #[doc = "Returns the node if it is of kind `cycle` ([unnamed::Cycle]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn cycle(self) -> Option<unnamed::Cycle<'tree>> {
            match self {
                Self::Cycle(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `once` ([unnamed::Once]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn once(self) -> Option<unnamed::Once<'tree>> {
            match self {
                Self::Once(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `shuffle` ([unnamed::Shuffle]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn shuffle(self) -> Option<unnamed::Shuffle<'tree>> {
            match self {
                Self::Shuffle(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `stopping` ([unnamed::Stopping]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn stopping(self) -> Option<unnamed::Stopping<'tree>> {
            match self {
                Self::Stopping(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Cycle_Once_Shuffle_Stopping<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "cycle" => Ok(unsafe {
                    Self::Cycle(<unnamed::Cycle<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "once" => Ok(unsafe {
                    Self::Once(<unnamed::Once<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "shuffle" => Ok(unsafe {
                    Self::Shuffle(<unnamed::Shuffle<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "stopping" => Ok(unsafe {
                    Self::Stopping(<unnamed::Stopping<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Cycle_Once_Shuffle_Stopping<'tree> {
        const KIND: &'static str = "{cycle | once | shuffle | stopping}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Cycle(x) => x.node(),
                Self::Once(x) => x.node(),
                Self::Shuffle(x) => x.node(),
                Self::Stopping(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Cycle(x) => x.node_mut(),
                Self::Once(x) => x.node_mut(),
                Self::Shuffle(x) => x.node_mut(),
                Self::Stopping(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::Cycle(x) => x.into_node(),
                Self::Once(x) => x.into_node(),
                Self::Shuffle(x) => x.into_node(),
                Self::Stopping(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{divert | identifier | ref}`:\n- [Divert]\n- [Identifier]\n- [unnamed::Ref]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Divert_Identifier_Ref<'tree> {
        Divert(Divert<'tree>),
        Identifier(Identifier<'tree>),
        Ref(unnamed::Ref<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Divert_Identifier_Ref<'tree> {
        #[doc = "Returns the node if it is of kind `divert` ([Divert]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn divert(self) -> Option<Divert<'tree>> {
            match self {
                Self::Divert(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `ref` ([unnamed::Ref]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#ref(self) -> Option<unnamed::Ref<'tree>> {
            match self {
                Self::Ref(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Divert_Identifier_Ref<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "divert" => {
                    Ok(unsafe {
                        Self :: Divert (< Divert < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "ref" => Ok(unsafe {
                    Self :: Ref (< unnamed :: Ref < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Divert_Identifier_Ref<'tree> {
        const KIND: &'static str = "{divert | identifier | ref}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Divert(x) => x.node(),
                Self::Identifier(x) => x.node(),
                Self::Ref(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Divert(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
                Self::Ref(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::Divert(x) => x.into_node(),
                Self::Identifier(x) => x.into_node(),
                Self::Ref(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{identifier | ++ | --}`:\n- [Identifier]\n- [symbols::Add_Add_]\n- [symbols::Sub_Sub_]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Identifier_Add_Add__Sub_Sub_<'tree> {
        Identifier(Identifier<'tree>),
        Add_Add_(symbols::Add_Add_<'tree>),
        Sub_Sub_(symbols::Sub_Sub_<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Identifier_Add_Add__Sub_Sub_<'tree> {
        #[doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `++` ([symbols::Add_Add_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn add_add_(self) -> Option<symbols::Add_Add_<'tree>> {
            match self {
                Self::Add_Add_(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `--` ([symbols::Sub_Sub_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn sub_sub_(self) -> Option<symbols::Sub_Sub_<'tree>> {
            match self {
                Self::Sub_Sub_(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Identifier_Add_Add__Sub_Sub_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "++" => Ok(unsafe {
                    Self::Add_Add_(<symbols::Add_Add_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "--" => Ok(unsafe {
                    Self::Sub_Sub_(<symbols::Sub_Sub_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Identifier_Add_Add__Sub_Sub_<'tree> {
        const KIND: &'static str = "{identifier | ++ | --}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.node(),
                Self::Add_Add_(x) => x.node(),
                Self::Sub_Sub_(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.node_mut(),
                Self::Add_Add_(x) => x.node_mut(),
                Self::Sub_Sub_(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_node(),
                Self::Add_Add_(x) => x.into_node(),
                Self::Sub_Sub_(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{++ | --}`:\n- [symbols::Add_Add_]\n- [symbols::Sub_Sub_]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Add_Add__Sub_Sub_<'tree> {
        Add_Add_(symbols::Add_Add_<'tree>),
        Sub_Sub_(symbols::Sub_Sub_<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Add_Add__Sub_Sub_<'tree> {
        #[doc = "Returns the node if it is of kind `++` ([symbols::Add_Add_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn add_add_(self) -> Option<symbols::Add_Add_<'tree>> {
            match self {
                Self::Add_Add_(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `--` ([symbols::Sub_Sub_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn sub_sub_(self) -> Option<symbols::Sub_Sub_<'tree>> {
            match self {
                Self::Sub_Sub_(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Add_Add__Sub_Sub_<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "++" => Ok(unsafe {
                    Self::Add_Add_(<symbols::Add_Add_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "--" => Ok(unsafe {
                    Self::Sub_Sub_(<symbols::Sub_Sub_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Add_Add__Sub_Sub_<'tree> {
        const KIND: &'static str = "{++ | --}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Add_Add_(x) => x.node(),
                Self::Sub_Sub_(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Add_Add_(x) => x.node_mut(),
                Self::Sub_Sub_(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::Add_Add_(x) => x.into_node(),
                Self::Sub_Sub_(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{content_block | stitch}`:\n- [ContentBlock]\n- [Stitch]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ContentBlock_Stitch<'tree> {
        ContentBlock(ContentBlock<'tree>),
        Stitch(Stitch<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ContentBlock_Stitch<'tree> {
        #[doc = "Returns the node if it is of kind `content_block` ([ContentBlock]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn content_block(self) -> Option<ContentBlock<'tree>> {
            match self {
                Self::ContentBlock(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `stitch` ([Stitch]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn stitch(self) -> Option<Stitch<'tree>> {
            match self {
                Self::Stitch(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ContentBlock_Stitch<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "content_block" => Ok(unsafe {
                    Self :: ContentBlock (< ContentBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "stitch" => {
                    Ok(unsafe {
                        Self :: Stitch (< Stitch < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for ContentBlock_Stitch<'tree> {
        const KIND: &'static str = "{content_block | stitch}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::ContentBlock(x) => x.node(),
                Self::Stitch(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::ContentBlock(x) => x.node_mut(),
                Self::Stitch(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::ContentBlock(x) => x.into_node(),
                Self::Stitch(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{alternatives | cond_block | conditional_text | eval | glue | multiline_alternatives | text}`:\n- [Alternatives]\n- [CondBlock]\n- [ConditionalText]\n- [Eval]\n- [Glue]\n- [MultilineAlternatives]\n- [Text]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Alternatives_CondBlock_ConditionalText_Eval_Glue_MultilineAlternatives_Text<'tree> {
        Alternatives(Alternatives<'tree>),
        CondBlock(CondBlock<'tree>),
        ConditionalText(ConditionalText<'tree>),
        Eval(Eval<'tree>),
        Glue(Glue<'tree>),
        MultilineAlternatives(MultilineAlternatives<'tree>),
        Text(Text<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Alternatives_CondBlock_ConditionalText_Eval_Glue_MultilineAlternatives_Text<'tree> {
        #[doc = "Returns the node if it is of kind `alternatives` ([Alternatives]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn alternatives(self) -> Option<Alternatives<'tree>> {
            match self {
                Self::Alternatives(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `cond_block` ([CondBlock]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn cond_block(self) -> Option<CondBlock<'tree>> {
            match self {
                Self::CondBlock(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `conditional_text` ([ConditionalText]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn conditional_text(self) -> Option<ConditionalText<'tree>> {
            match self {
                Self::ConditionalText(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `eval` ([Eval]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn eval(self) -> Option<Eval<'tree>> {
            match self {
                Self::Eval(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `glue` ([Glue]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn glue(self) -> Option<Glue<'tree>> {
            match self {
                Self::Glue(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `multiline_alternatives` ([MultilineAlternatives]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn multiline_alternatives(self) -> Option<MultilineAlternatives<'tree>> {
            match self {
                Self::MultilineAlternatives(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `text` ([Text]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn text(self) -> Option<Text<'tree>> {
            match self {
                Self::Text(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>>
        for Alternatives_CondBlock_ConditionalText_Eval_Glue_MultilineAlternatives_Text<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "alternatives" => Ok(unsafe {
                    Self :: Alternatives (< Alternatives < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "cond_block" => Ok(unsafe {
                    Self :: CondBlock (< CondBlock < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "conditional_text" => {
                    Ok(unsafe {
                        Self :: ConditionalText (< ConditionalText < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "eval" => Ok(unsafe {
                    Self::Eval(
                        <Eval<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                            node,
                        ),
                    )
                }),
                "glue" => Ok(unsafe {
                    Self::Glue(
                        <Glue<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                            node,
                        ),
                    )
                }),
                "multiline_alternatives" => Ok(unsafe {
                    Self :: MultilineAlternatives (< MultilineAlternatives < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "text" => Ok(unsafe {
                    Self::Text(
                        <Text<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                            node,
                        ),
                    )
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree>
        for Alternatives_CondBlock_ConditionalText_Eval_Glue_MultilineAlternatives_Text<'tree>
    {
        const KIND : & 'static str = "{alternatives | cond_block | conditional_text | eval | glue | multiline_alternatives | text}" ;
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Alternatives(x) => x.node(),
                Self::CondBlock(x) => x.node(),
                Self::ConditionalText(x) => x.node(),
                Self::Eval(x) => x.node(),
                Self::Glue(x) => x.node(),
                Self::MultilineAlternatives(x) => x.node(),
                Self::Text(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Alternatives(x) => x.node_mut(),
                Self::CondBlock(x) => x.node_mut(),
                Self::ConditionalText(x) => x.node_mut(),
                Self::Eval(x) => x.node_mut(),
                Self::Glue(x) => x.node_mut(),
                Self::MultilineAlternatives(x) => x.node_mut(),
                Self::Text(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::Alternatives(x) => x.into_node(),
                Self::CondBlock(x) => x.into_node(),
                Self::ConditionalText(x) => x.into_node(),
                Self::Eval(x) => x.into_node(),
                Self::Glue(x) => x.into_node(),
                Self::MultilineAlternatives(x) => x.into_node(),
                Self::Text(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{! | != | !? | % | && | ( | ) | * | + | ++ | , | - | -- | -> | . | / | < | <= | == | > | >= | ? | ^ | and | false | has | hasnt | mod | not | or | true | ||}`:\n- [symbols::Not_]\n- [symbols::Not_Eq_]\n- [symbols::Not_Question_]\n- [symbols::Mod_]\n- [symbols::And_And_]\n- [symbols::LParen_]\n- [symbols::RParen_]\n- [symbols::Mul_]\n- [symbols::Add_]\n- [symbols::Add_Add_]\n- [symbols::Comma_]\n- [symbols::Sub_]\n- [symbols::Sub_Sub_]\n- [symbols::Sub_Gt_]\n- [symbols::Dot_]\n- [symbols::Div_]\n- [symbols::Lt_]\n- [symbols::Lt_Eq_]\n- [symbols::Eq_Eq_]\n- [symbols::Gt_]\n- [symbols::Gt_Eq_]\n- [symbols::Question_]\n- [symbols::BitXor_]\n- [unnamed::And]\n- [unnamed::False]\n- [unnamed::Has]\n- [unnamed::Hasnt]\n- [unnamed::Mod]\n- [unnamed::Not]\n- [unnamed::Or]\n- [unnamed::True]\n- [symbols::Or_Or_]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Not__Not_Eq__Not_Question__Mod__And_And__LParen__RParen__Mul__Add__Add_Add__Comma__Sub__Sub_Sub__Sub_Gt__Dot__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_False_Has_Hasnt_Mod_Not_Or_True_Or_Or_<
        'tree,
    > {
        Not_(symbols::Not_<'tree>),
        Not_Eq_(symbols::Not_Eq_<'tree>),
        Not_Question_(symbols::Not_Question_<'tree>),
        Mod_(symbols::Mod_<'tree>),
        And_And_(symbols::And_And_<'tree>),
        LParen_(symbols::LParen_<'tree>),
        RParen_(symbols::RParen_<'tree>),
        Mul_(symbols::Mul_<'tree>),
        Add_(symbols::Add_<'tree>),
        Add_Add_(symbols::Add_Add_<'tree>),
        Comma_(symbols::Comma_<'tree>),
        Sub_(symbols::Sub_<'tree>),
        Sub_Sub_(symbols::Sub_Sub_<'tree>),
        Sub_Gt_(symbols::Sub_Gt_<'tree>),
        Dot_(symbols::Dot_<'tree>),
        Div_(symbols::Div_<'tree>),
        Lt_(symbols::Lt_<'tree>),
        Lt_Eq_(symbols::Lt_Eq_<'tree>),
        Eq_Eq_(symbols::Eq_Eq_<'tree>),
        Gt_(symbols::Gt_<'tree>),
        Gt_Eq_(symbols::Gt_Eq_<'tree>),
        Question_(symbols::Question_<'tree>),
        BitXor_(symbols::BitXor_<'tree>),
        And(unnamed::And<'tree>),
        False(unnamed::False<'tree>),
        Has(unnamed::Has<'tree>),
        Hasnt(unnamed::Hasnt<'tree>),
        Mod(unnamed::Mod<'tree>),
        Not(unnamed::Not<'tree>),
        Or(unnamed::Or<'tree>),
        True(unnamed::True<'tree>),
        Or_Or_(symbols::Or_Or_<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > Not__Not_Eq__Not_Question__Mod__And_And__LParen__RParen__Mul__Add__Add_Add__Comma__Sub__Sub_Sub__Sub_Gt__Dot__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_False_Has_Hasnt_Mod_Not_Or_True_Or_Or_ < 'tree > { # [doc = "Returns the node if it is of kind `!` ([symbols::Not_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn not_ (self) -> Option < symbols :: Not_ < 'tree > > { match self { Self :: Not_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `!=` ([symbols::Not_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn not_eq_ (self) -> Option < symbols :: Not_Eq_ < 'tree > > { match self { Self :: Not_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `!?` ([symbols::Not_Question_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn not_question_ (self) -> Option < symbols :: Not_Question_ < 'tree > > { match self { Self :: Not_Question_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `%` ([symbols::Mod_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn mod_ (self) -> Option < symbols :: Mod_ < 'tree > > { match self { Self :: Mod_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `&&` ([symbols::And_And_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn and_and_ (self) -> Option < symbols :: And_And_ < 'tree > > { match self { Self :: And_And_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `(` ([symbols::LParen_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn l_paren_ (self) -> Option < symbols :: LParen_ < 'tree > > { match self { Self :: LParen_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `)` ([symbols::RParen_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn r_paren_ (self) -> Option < symbols :: RParen_ < 'tree > > { match self { Self :: RParen_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `*` ([symbols::Mul_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn mul_ (self) -> Option < symbols :: Mul_ < 'tree > > { match self { Self :: Mul_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `+` ([symbols::Add_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn add_ (self) -> Option < symbols :: Add_ < 'tree > > { match self { Self :: Add_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `++` ([symbols::Add_Add_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn add_add_ (self) -> Option < symbols :: Add_Add_ < 'tree > > { match self { Self :: Add_Add_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `,` ([symbols::Comma_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn comma_ (self) -> Option < symbols :: Comma_ < 'tree > > { match self { Self :: Comma_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `-` ([symbols::Sub_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn sub_ (self) -> Option < symbols :: Sub_ < 'tree > > { match self { Self :: Sub_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `--` ([symbols::Sub_Sub_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn sub_sub_ (self) -> Option < symbols :: Sub_Sub_ < 'tree > > { match self { Self :: Sub_Sub_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `->` ([symbols::Sub_Gt_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn sub_gt_ (self) -> Option < symbols :: Sub_Gt_ < 'tree > > { match self { Self :: Sub_Gt_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `.` ([symbols::Dot_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn dot_ (self) -> Option < symbols :: Dot_ < 'tree > > { match self { Self :: Dot_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `/` ([symbols::Div_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn div_ (self) -> Option < symbols :: Div_ < 'tree > > { match self { Self :: Div_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `<` ([symbols::Lt_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn lt_ (self) -> Option < symbols :: Lt_ < 'tree > > { match self { Self :: Lt_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `<=` ([symbols::Lt_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn lt_eq_ (self) -> Option < symbols :: Lt_Eq_ < 'tree > > { match self { Self :: Lt_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `==` ([symbols::Eq_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn eq_eq_ (self) -> Option < symbols :: Eq_Eq_ < 'tree > > { match self { Self :: Eq_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `>` ([symbols::Gt_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn gt_ (self) -> Option < symbols :: Gt_ < 'tree > > { match self { Self :: Gt_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `>=` ([symbols::Gt_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn gt_eq_ (self) -> Option < symbols :: Gt_Eq_ < 'tree > > { match self { Self :: Gt_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `?` ([symbols::Question_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn question_ (self) -> Option < symbols :: Question_ < 'tree > > { match self { Self :: Question_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `^` ([symbols::BitXor_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn bit_xor_ (self) -> Option < symbols :: BitXor_ < 'tree > > { match self { Self :: BitXor_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `and` ([unnamed::And]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn and (self) -> Option < unnamed :: And < 'tree > > { match self { Self :: And (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `false` ([unnamed::False]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn r#false (self) -> Option < unnamed :: False < 'tree > > { match self { Self :: False (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `has` ([unnamed::Has]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn has (self) -> Option < unnamed :: Has < 'tree > > { match self { Self :: Has (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `hasnt` ([unnamed::Hasnt]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn hasnt (self) -> Option < unnamed :: Hasnt < 'tree > > { match self { Self :: Hasnt (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `mod` ([unnamed::Mod]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn r#mod (self) -> Option < unnamed :: Mod < 'tree > > { match self { Self :: Mod (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `not` ([unnamed::Not]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn not (self) -> Option < unnamed :: Not < 'tree > > { match self { Self :: Not (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `or` ([unnamed::Or]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn or (self) -> Option < unnamed :: Or < 'tree > > { match self { Self :: Or (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `true` ([unnamed::True]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn r#true (self) -> Option < unnamed :: True < 'tree > > { match self { Self :: True (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `||` ([symbols::Or_Or_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn or_or_ (self) -> Option < symbols :: Or_Or_ < 'tree > > { match self { Self :: Or_Or_ (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Not__Not_Eq__Not_Question__Mod__And_And__LParen__RParen__Mul__Add__Add_Add__Comma__Sub__Sub_Sub__Sub_Gt__Dot__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_False_Has_Hasnt_Mod_Not_Or_True_Or_Or_ < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "!" => Ok (unsafe { Self :: Not_ (< symbols :: Not_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "!=" => Ok (unsafe { Self :: Not_Eq_ (< symbols :: Not_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "!?" => Ok (unsafe { Self :: Not_Question_ (< symbols :: Not_Question_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "%" => Ok (unsafe { Self :: Mod_ (< symbols :: Mod_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "&&" => Ok (unsafe { Self :: And_And_ (< symbols :: And_And_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "(" => Ok (unsafe { Self :: LParen_ (< symbols :: LParen_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , ")" => Ok (unsafe { Self :: RParen_ (< symbols :: RParen_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "*" => Ok (unsafe { Self :: Mul_ (< symbols :: Mul_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "+" => Ok (unsafe { Self :: Add_ (< symbols :: Add_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "++" => Ok (unsafe { Self :: Add_Add_ (< symbols :: Add_Add_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "," => Ok (unsafe { Self :: Comma_ (< symbols :: Comma_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "-" => Ok (unsafe { Self :: Sub_ (< symbols :: Sub_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "--" => Ok (unsafe { Self :: Sub_Sub_ (< symbols :: Sub_Sub_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "->" => Ok (unsafe { Self :: Sub_Gt_ (< symbols :: Sub_Gt_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "." => Ok (unsafe { Self :: Dot_ (< symbols :: Dot_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "/" => Ok (unsafe { Self :: Div_ (< symbols :: Div_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "<" => Ok (unsafe { Self :: Lt_ (< symbols :: Lt_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "<=" => Ok (unsafe { Self :: Lt_Eq_ (< symbols :: Lt_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "==" => Ok (unsafe { Self :: Eq_Eq_ (< symbols :: Eq_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , ">" => Ok (unsafe { Self :: Gt_ (< symbols :: Gt_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , ">=" => Ok (unsafe { Self :: Gt_Eq_ (< symbols :: Gt_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "?" => Ok (unsafe { Self :: Question_ (< symbols :: Question_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "^" => Ok (unsafe { Self :: BitXor_ (< symbols :: BitXor_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "and" => Ok (unsafe { Self :: And (< unnamed :: And < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "false" => Ok (unsafe { Self :: False (< unnamed :: False < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "has" => Ok (unsafe { Self :: Has (< unnamed :: Has < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "hasnt" => Ok (unsafe { Self :: Hasnt (< unnamed :: Hasnt < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "mod" => Ok (unsafe { Self :: Mod (< unnamed :: Mod < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "not" => Ok (unsafe { Self :: Not (< unnamed :: Not < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "or" => Ok (unsafe { Self :: Or (< unnamed :: Or < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "true" => Ok (unsafe { Self :: True (< unnamed :: True < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "||" => Ok (unsafe { Self :: Or_Or_ (< symbols :: Or_Or_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Not__Not_Eq__Not_Question__Mod__And_And__LParen__RParen__Mul__Add__Add_Add__Comma__Sub__Sub_Sub__Sub_Gt__Dot__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_False_Has_Hasnt_Mod_Not_Or_True_Or_Or_ < 'tree > { const KIND : & 'static str = "{! | != | !? | % | && | ( | ) | * | + | ++ | , | - | -- | -> | . | / | < | <= | == | > | >= | ? | ^ | and | false | has | hasnt | mod | not | or | true | ||}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Not_ (x) => x . node () , Self :: Not_Eq_ (x) => x . node () , Self :: Not_Question_ (x) => x . node () , Self :: Mod_ (x) => x . node () , Self :: And_And_ (x) => x . node () , Self :: LParen_ (x) => x . node () , Self :: RParen_ (x) => x . node () , Self :: Mul_ (x) => x . node () , Self :: Add_ (x) => x . node () , Self :: Add_Add_ (x) => x . node () , Self :: Comma_ (x) => x . node () , Self :: Sub_ (x) => x . node () , Self :: Sub_Sub_ (x) => x . node () , Self :: Sub_Gt_ (x) => x . node () , Self :: Dot_ (x) => x . node () , Self :: Div_ (x) => x . node () , Self :: Lt_ (x) => x . node () , Self :: Lt_Eq_ (x) => x . node () , Self :: Eq_Eq_ (x) => x . node () , Self :: Gt_ (x) => x . node () , Self :: Gt_Eq_ (x) => x . node () , Self :: Question_ (x) => x . node () , Self :: BitXor_ (x) => x . node () , Self :: And (x) => x . node () , Self :: False (x) => x . node () , Self :: Has (x) => x . node () , Self :: Hasnt (x) => x . node () , Self :: Mod (x) => x . node () , Self :: Not (x) => x . node () , Self :: Or (x) => x . node () , Self :: True (x) => x . node () , Self :: Or_Or_ (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Not_ (x) => x . node_mut () , Self :: Not_Eq_ (x) => x . node_mut () , Self :: Not_Question_ (x) => x . node_mut () , Self :: Mod_ (x) => x . node_mut () , Self :: And_And_ (x) => x . node_mut () , Self :: LParen_ (x) => x . node_mut () , Self :: RParen_ (x) => x . node_mut () , Self :: Mul_ (x) => x . node_mut () , Self :: Add_ (x) => x . node_mut () , Self :: Add_Add_ (x) => x . node_mut () , Self :: Comma_ (x) => x . node_mut () , Self :: Sub_ (x) => x . node_mut () , Self :: Sub_Sub_ (x) => x . node_mut () , Self :: Sub_Gt_ (x) => x . node_mut () , Self :: Dot_ (x) => x . node_mut () , Self :: Div_ (x) => x . node_mut () , Self :: Lt_ (x) => x . node_mut () , Self :: Lt_Eq_ (x) => x . node_mut () , Self :: Eq_Eq_ (x) => x . node_mut () , Self :: Gt_ (x) => x . node_mut () , Self :: Gt_Eq_ (x) => x . node_mut () , Self :: Question_ (x) => x . node_mut () , Self :: BitXor_ (x) => x . node_mut () , Self :: And (x) => x . node_mut () , Self :: False (x) => x . node_mut () , Self :: Has (x) => x . node_mut () , Self :: Hasnt (x) => x . node_mut () , Self :: Mod (x) => x . node_mut () , Self :: Not (x) => x . node_mut () , Self :: Or (x) => x . node_mut () , Self :: True (x) => x . node_mut () , Self :: Or_Or_ (x) => x . node_mut () , } } # [inline] fn into_node (self) -> tree_sitter :: Node < 'tree > { match self { Self :: Not_ (x) => x . into_node () , Self :: Not_Eq_ (x) => x . into_node () , Self :: Not_Question_ (x) => x . into_node () , Self :: Mod_ (x) => x . into_node () , Self :: And_And_ (x) => x . into_node () , Self :: LParen_ (x) => x . into_node () , Self :: RParen_ (x) => x . into_node () , Self :: Mul_ (x) => x . into_node () , Self :: Add_ (x) => x . into_node () , Self :: Add_Add_ (x) => x . into_node () , Self :: Comma_ (x) => x . into_node () , Self :: Sub_ (x) => x . into_node () , Self :: Sub_Sub_ (x) => x . into_node () , Self :: Sub_Gt_ (x) => x . into_node () , Self :: Dot_ (x) => x . into_node () , Self :: Div_ (x) => x . into_node () , Self :: Lt_ (x) => x . into_node () , Self :: Lt_Eq_ (x) => x . into_node () , Self :: Eq_Eq_ (x) => x . into_node () , Self :: Gt_ (x) => x . into_node () , Self :: Gt_Eq_ (x) => x . into_node () , Self :: Question_ (x) => x . into_node () , Self :: BitXor_ (x) => x . into_node () , Self :: And (x) => x . into_node () , Self :: False (x) => x . into_node () , Self :: Has (x) => x . into_node () , Self :: Hasnt (x) => x . into_node () , Self :: Mod (x) => x . into_node () , Self :: Not (x) => x . into_node () , Self :: Or (x) => x . into_node () , Self :: True (x) => x . into_node () , Self :: Or_Or_ (x) => x . into_node () , } } }
    #[doc = "one of `{! | != | !? | % | && | * | + | ++ | - | -- | / | < | <= | == | > | >= | ? | ^ | and | has | hasnt | mod | not | or | ||}`:\n- [symbols::Not_]\n- [symbols::Not_Eq_]\n- [symbols::Not_Question_]\n- [symbols::Mod_]\n- [symbols::And_And_]\n- [symbols::Mul_]\n- [symbols::Add_]\n- [symbols::Add_Add_]\n- [symbols::Sub_]\n- [symbols::Sub_Sub_]\n- [symbols::Div_]\n- [symbols::Lt_]\n- [symbols::Lt_Eq_]\n- [symbols::Eq_Eq_]\n- [symbols::Gt_]\n- [symbols::Gt_Eq_]\n- [symbols::Question_]\n- [symbols::BitXor_]\n- [unnamed::And]\n- [unnamed::Has]\n- [unnamed::Hasnt]\n- [unnamed::Mod]\n- [unnamed::Not]\n- [unnamed::Or]\n- [symbols::Or_Or_]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Not__Not_Eq__Not_Question__Mod__And_And__Mul__Add__Add_Add__Sub__Sub_Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Not_Or_Or_Or_<
        'tree,
    > {
        Not_(symbols::Not_<'tree>),
        Not_Eq_(symbols::Not_Eq_<'tree>),
        Not_Question_(symbols::Not_Question_<'tree>),
        Mod_(symbols::Mod_<'tree>),
        And_And_(symbols::And_And_<'tree>),
        Mul_(symbols::Mul_<'tree>),
        Add_(symbols::Add_<'tree>),
        Add_Add_(symbols::Add_Add_<'tree>),
        Sub_(symbols::Sub_<'tree>),
        Sub_Sub_(symbols::Sub_Sub_<'tree>),
        Div_(symbols::Div_<'tree>),
        Lt_(symbols::Lt_<'tree>),
        Lt_Eq_(symbols::Lt_Eq_<'tree>),
        Eq_Eq_(symbols::Eq_Eq_<'tree>),
        Gt_(symbols::Gt_<'tree>),
        Gt_Eq_(symbols::Gt_Eq_<'tree>),
        Question_(symbols::Question_<'tree>),
        BitXor_(symbols::BitXor_<'tree>),
        And(unnamed::And<'tree>),
        Has(unnamed::Has<'tree>),
        Hasnt(unnamed::Hasnt<'tree>),
        Mod(unnamed::Mod<'tree>),
        Not(unnamed::Not<'tree>),
        Or(unnamed::Or<'tree>),
        Or_Or_(symbols::Or_Or_<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > Not__Not_Eq__Not_Question__Mod__And_And__Mul__Add__Add_Add__Sub__Sub_Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Not_Or_Or_Or_ < 'tree > { # [doc = "Returns the node if it is of kind `!` ([symbols::Not_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn not_ (self) -> Option < symbols :: Not_ < 'tree > > { match self { Self :: Not_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `!=` ([symbols::Not_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn not_eq_ (self) -> Option < symbols :: Not_Eq_ < 'tree > > { match self { Self :: Not_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `!?` ([symbols::Not_Question_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn not_question_ (self) -> Option < symbols :: Not_Question_ < 'tree > > { match self { Self :: Not_Question_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `%` ([symbols::Mod_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn mod_ (self) -> Option < symbols :: Mod_ < 'tree > > { match self { Self :: Mod_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `&&` ([symbols::And_And_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn and_and_ (self) -> Option < symbols :: And_And_ < 'tree > > { match self { Self :: And_And_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `*` ([symbols::Mul_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn mul_ (self) -> Option < symbols :: Mul_ < 'tree > > { match self { Self :: Mul_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `+` ([symbols::Add_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn add_ (self) -> Option < symbols :: Add_ < 'tree > > { match self { Self :: Add_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `++` ([symbols::Add_Add_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn add_add_ (self) -> Option < symbols :: Add_Add_ < 'tree > > { match self { Self :: Add_Add_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `-` ([symbols::Sub_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn sub_ (self) -> Option < symbols :: Sub_ < 'tree > > { match self { Self :: Sub_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `--` ([symbols::Sub_Sub_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn sub_sub_ (self) -> Option < symbols :: Sub_Sub_ < 'tree > > { match self { Self :: Sub_Sub_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `/` ([symbols::Div_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn div_ (self) -> Option < symbols :: Div_ < 'tree > > { match self { Self :: Div_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `<` ([symbols::Lt_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn lt_ (self) -> Option < symbols :: Lt_ < 'tree > > { match self { Self :: Lt_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `<=` ([symbols::Lt_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn lt_eq_ (self) -> Option < symbols :: Lt_Eq_ < 'tree > > { match self { Self :: Lt_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `==` ([symbols::Eq_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn eq_eq_ (self) -> Option < symbols :: Eq_Eq_ < 'tree > > { match self { Self :: Eq_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `>` ([symbols::Gt_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn gt_ (self) -> Option < symbols :: Gt_ < 'tree > > { match self { Self :: Gt_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `>=` ([symbols::Gt_Eq_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn gt_eq_ (self) -> Option < symbols :: Gt_Eq_ < 'tree > > { match self { Self :: Gt_Eq_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `?` ([symbols::Question_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn question_ (self) -> Option < symbols :: Question_ < 'tree > > { match self { Self :: Question_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `^` ([symbols::BitXor_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn bit_xor_ (self) -> Option < symbols :: BitXor_ < 'tree > > { match self { Self :: BitXor_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `and` ([unnamed::And]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn and (self) -> Option < unnamed :: And < 'tree > > { match self { Self :: And (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `has` ([unnamed::Has]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn has (self) -> Option < unnamed :: Has < 'tree > > { match self { Self :: Has (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `hasnt` ([unnamed::Hasnt]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn hasnt (self) -> Option < unnamed :: Hasnt < 'tree > > { match self { Self :: Hasnt (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `mod` ([unnamed::Mod]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn r#mod (self) -> Option < unnamed :: Mod < 'tree > > { match self { Self :: Mod (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `not` ([unnamed::Not]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn not (self) -> Option < unnamed :: Not < 'tree > > { match self { Self :: Not (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `or` ([unnamed::Or]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn or (self) -> Option < unnamed :: Or < 'tree > > { match self { Self :: Or (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `||` ([symbols::Or_Or_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn or_or_ (self) -> Option < symbols :: Or_Or_ < 'tree > > { match self { Self :: Or_Or_ (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Not__Not_Eq__Not_Question__Mod__And_And__Mul__Add__Add_Add__Sub__Sub_Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Not_Or_Or_Or_ < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "!" => Ok (unsafe { Self :: Not_ (< symbols :: Not_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "!=" => Ok (unsafe { Self :: Not_Eq_ (< symbols :: Not_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "!?" => Ok (unsafe { Self :: Not_Question_ (< symbols :: Not_Question_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "%" => Ok (unsafe { Self :: Mod_ (< symbols :: Mod_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "&&" => Ok (unsafe { Self :: And_And_ (< symbols :: And_And_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "*" => Ok (unsafe { Self :: Mul_ (< symbols :: Mul_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "+" => Ok (unsafe { Self :: Add_ (< symbols :: Add_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "++" => Ok (unsafe { Self :: Add_Add_ (< symbols :: Add_Add_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "-" => Ok (unsafe { Self :: Sub_ (< symbols :: Sub_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "--" => Ok (unsafe { Self :: Sub_Sub_ (< symbols :: Sub_Sub_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "/" => Ok (unsafe { Self :: Div_ (< symbols :: Div_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "<" => Ok (unsafe { Self :: Lt_ (< symbols :: Lt_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "<=" => Ok (unsafe { Self :: Lt_Eq_ (< symbols :: Lt_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "==" => Ok (unsafe { Self :: Eq_Eq_ (< symbols :: Eq_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , ">" => Ok (unsafe { Self :: Gt_ (< symbols :: Gt_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , ">=" => Ok (unsafe { Self :: Gt_Eq_ (< symbols :: Gt_Eq_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "?" => Ok (unsafe { Self :: Question_ (< symbols :: Question_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "^" => Ok (unsafe { Self :: BitXor_ (< symbols :: BitXor_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "and" => Ok (unsafe { Self :: And (< unnamed :: And < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "has" => Ok (unsafe { Self :: Has (< unnamed :: Has < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "hasnt" => Ok (unsafe { Self :: Hasnt (< unnamed :: Hasnt < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "mod" => Ok (unsafe { Self :: Mod (< unnamed :: Mod < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "not" => Ok (unsafe { Self :: Not (< unnamed :: Not < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "or" => Ok (unsafe { Self :: Or (< unnamed :: Or < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "||" => Ok (unsafe { Self :: Or_Or_ (< symbols :: Or_Or_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Not__Not_Eq__Not_Question__Mod__And_And__Mul__Add__Add_Add__Sub__Sub_Sub__Div__Lt__Lt_Eq__Eq_Eq__Gt__Gt_Eq__Question__BitXor__And_Has_Hasnt_Mod_Not_Or_Or_Or_ < 'tree > { const KIND : & 'static str = "{! | != | !? | % | && | * | + | ++ | - | -- | / | < | <= | == | > | >= | ? | ^ | and | has | hasnt | mod | not | or | ||}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Not_ (x) => x . node () , Self :: Not_Eq_ (x) => x . node () , Self :: Not_Question_ (x) => x . node () , Self :: Mod_ (x) => x . node () , Self :: And_And_ (x) => x . node () , Self :: Mul_ (x) => x . node () , Self :: Add_ (x) => x . node () , Self :: Add_Add_ (x) => x . node () , Self :: Sub_ (x) => x . node () , Self :: Sub_Sub_ (x) => x . node () , Self :: Div_ (x) => x . node () , Self :: Lt_ (x) => x . node () , Self :: Lt_Eq_ (x) => x . node () , Self :: Eq_Eq_ (x) => x . node () , Self :: Gt_ (x) => x . node () , Self :: Gt_Eq_ (x) => x . node () , Self :: Question_ (x) => x . node () , Self :: BitXor_ (x) => x . node () , Self :: And (x) => x . node () , Self :: Has (x) => x . node () , Self :: Hasnt (x) => x . node () , Self :: Mod (x) => x . node () , Self :: Not (x) => x . node () , Self :: Or (x) => x . node () , Self :: Or_Or_ (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Not_ (x) => x . node_mut () , Self :: Not_Eq_ (x) => x . node_mut () , Self :: Not_Question_ (x) => x . node_mut () , Self :: Mod_ (x) => x . node_mut () , Self :: And_And_ (x) => x . node_mut () , Self :: Mul_ (x) => x . node_mut () , Self :: Add_ (x) => x . node_mut () , Self :: Add_Add_ (x) => x . node_mut () , Self :: Sub_ (x) => x . node_mut () , Self :: Sub_Sub_ (x) => x . node_mut () , Self :: Div_ (x) => x . node_mut () , Self :: Lt_ (x) => x . node_mut () , Self :: Lt_Eq_ (x) => x . node_mut () , Self :: Eq_Eq_ (x) => x . node_mut () , Self :: Gt_ (x) => x . node_mut () , Self :: Gt_Eq_ (x) => x . node_mut () , Self :: Question_ (x) => x . node_mut () , Self :: BitXor_ (x) => x . node_mut () , Self :: And (x) => x . node_mut () , Self :: Has (x) => x . node_mut () , Self :: Hasnt (x) => x . node_mut () , Self :: Mod (x) => x . node_mut () , Self :: Not (x) => x . node_mut () , Self :: Or (x) => x . node_mut () , Self :: Or_Or_ (x) => x . node_mut () , } } # [inline] fn into_node (self) -> tree_sitter :: Node < 'tree > { match self { Self :: Not_ (x) => x . into_node () , Self :: Not_Eq_ (x) => x . into_node () , Self :: Not_Question_ (x) => x . into_node () , Self :: Mod_ (x) => x . into_node () , Self :: And_And_ (x) => x . into_node () , Self :: Mul_ (x) => x . into_node () , Self :: Add_ (x) => x . into_node () , Self :: Add_Add_ (x) => x . into_node () , Self :: Sub_ (x) => x . into_node () , Self :: Sub_Sub_ (x) => x . into_node () , Self :: Div_ (x) => x . into_node () , Self :: Lt_ (x) => x . into_node () , Self :: Lt_Eq_ (x) => x . into_node () , Self :: Eq_Eq_ (x) => x . into_node () , Self :: Gt_ (x) => x . into_node () , Self :: Gt_Eq_ (x) => x . into_node () , Self :: Question_ (x) => x . into_node () , Self :: BitXor_ (x) => x . into_node () , Self :: And (x) => x . into_node () , Self :: Has (x) => x . into_node () , Self :: Hasnt (x) => x . into_node () , Self :: Mod (x) => x . into_node () , Self :: Not (x) => x . into_node () , Self :: Or (x) => x . into_node () , Self :: Or_Or_ (x) => x . into_node () , } } }
    #[doc = "one of `{call | identifier}`:\n- [Call]\n- [Identifier]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Call_Identifier<'tree> {
        Call(Call<'tree>),
        Identifier(Identifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Call_Identifier<'tree> {
        #[doc = "Returns the node if it is of kind `call` ([Call]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn call(self) -> Option<Call<'tree>> {
            match self {
                Self::Call(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Call_Identifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "call" => Ok(unsafe {
                    Self::Call(
                        <Call<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                            node,
                        ),
                    )
                }),
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Call_Identifier<'tree> {
        const KIND: &'static str = "{call | identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Call(x) => x.node(),
                Self::Identifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Call(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::Call(x) => x.into_node(),
                Self::Identifier(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{divert | call | identifier}`:\n- [Divert]\n- [Call]\n- [Identifier]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Divert_Call_Identifier<'tree> {
        Divert(Divert<'tree>),
        Call(Call<'tree>),
        Identifier(Identifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Divert_Call_Identifier<'tree> {
        #[doc = "Returns the node if it is of kind `divert` ([Divert]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn divert(self) -> Option<Divert<'tree>> {
            match self {
                Self::Divert(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `call` ([Call]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn call(self) -> Option<Call<'tree>> {
            match self {
                Self::Call(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Divert_Call_Identifier<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "divert" => {
                    Ok(unsafe {
                        Self :: Divert (< Divert < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "call" => Ok(unsafe {
                    Self::Call(
                        <Call<'tree> as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(
                            node,
                        ),
                    )
                }),
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Divert_Call_Identifier<'tree> {
        const KIND: &'static str = "{divert | call | identifier}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Divert(x) => x.node(),
                Self::Call(x) => x.node(),
                Self::Identifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Divert(x) => x.node_mut(),
                Self::Call(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::Divert(x) => x.into_node(),
                Self::Call(x) => x.into_node(),
                Self::Identifier(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary | ! | - | not}`:\n- [Binary]\n- [Boolean]\n- [Call]\n- [Divert]\n- [Identifier]\n- [ListValues]\n- [Number]\n- [Paren]\n- [Postfix]\n- [QualifiedName]\n- [String]\n- [Unary]\n- [symbols::Not_]\n- [symbols::Sub_]\n- [unnamed::Not]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not__Sub__Not<
        'tree,
    > {
        Binary(Binary<'tree>),
        Boolean(Boolean<'tree>),
        Call(Call<'tree>),
        Divert(Divert<'tree>),
        Identifier(Identifier<'tree>),
        ListValues(ListValues<'tree>),
        Number(Number<'tree>),
        Paren(Paren<'tree>),
        Postfix(Postfix<'tree>),
        QualifiedName(QualifiedName<'tree>),
        String(String<'tree>),
        Unary(Unary<'tree>),
        Not_(symbols::Not_<'tree>),
        Sub_(symbols::Sub_<'tree>),
        Not(unnamed::Not<'tree>),
    }
    #[automatically_derived]
    impl < 'tree > Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not__Sub__Not < 'tree > { # [doc = "Returns the node if it is of kind `binary` ([Binary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn binary (self) -> Option < Binary < 'tree > > { match self { Self :: Binary (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `boolean` ([Boolean]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn boolean (self) -> Option < Boolean < 'tree > > { match self { Self :: Boolean (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `call` ([Call]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn call (self) -> Option < Call < 'tree > > { match self { Self :: Call (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `divert` ([Divert]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn divert (self) -> Option < Divert < 'tree > > { match self { Self :: Divert (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn identifier (self) -> Option < Identifier < 'tree > > { match self { Self :: Identifier (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `list_values` ([ListValues]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn list_values (self) -> Option < ListValues < 'tree > > { match self { Self :: ListValues (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `number` ([Number]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn number (self) -> Option < Number < 'tree > > { match self { Self :: Number (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `paren` ([Paren]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn paren (self) -> Option < Paren < 'tree > > { match self { Self :: Paren (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `postfix` ([Postfix]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn postfix (self) -> Option < Postfix < 'tree > > { match self { Self :: Postfix (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `qualified_name` ([QualifiedName]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn qualified_name (self) -> Option < QualifiedName < 'tree > > { match self { Self :: QualifiedName (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `string` ([String]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn string (self) -> Option < String < 'tree > > { match self { Self :: String (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `unary` ([Unary]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn unary (self) -> Option < Unary < 'tree > > { match self { Self :: Unary (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `!` ([symbols::Not_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn not_ (self) -> Option < symbols :: Not_ < 'tree > > { match self { Self :: Not_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `-` ([symbols::Sub_]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn sub_ (self) -> Option < symbols :: Sub_ < 'tree > > { match self { Self :: Sub_ (x) => Some (x) , _ => None , } } # [doc = "Returns the node if it is of kind `not` ([unnamed::Not]), otherwise returns None"] # [inline] # [allow (unused , non_snake_case)] pub fn not (self) -> Option < unnamed :: Not < 'tree > > { match self { Self :: Not (x) => Some (x) , _ => None , } } }
    #[automatically_derived]
    impl < 'tree > TryFrom < tree_sitter :: Node < 'tree >> for Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not__Sub__Not < 'tree > { type Error = type_sitter_lib :: IncorrectKind < 'tree > ; # [inline] fn try_from (node : tree_sitter :: Node < 'tree >) -> Result < Self , Self :: Error > { match node . kind () { "binary" => Ok (unsafe { Self :: Binary (< Binary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "boolean" => Ok (unsafe { Self :: Boolean (< Boolean < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "call" => Ok (unsafe { Self :: Call (< Call < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "divert" => Ok (unsafe { Self :: Divert (< Divert < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "identifier" => Ok (unsafe { Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "list_values" => Ok (unsafe { Self :: ListValues (< ListValues < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "number" => Ok (unsafe { Self :: Number (< Number < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "paren" => Ok (unsafe { Self :: Paren (< Paren < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "postfix" => Ok (unsafe { Self :: Postfix (< Postfix < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "qualified_name" => Ok (unsafe { Self :: QualifiedName (< QualifiedName < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "string" => Ok (unsafe { Self :: String (< String < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "unary" => Ok (unsafe { Self :: Unary (< Unary < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "!" => Ok (unsafe { Self :: Not_ (< symbols :: Not_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "-" => Ok (unsafe { Self :: Sub_ (< symbols :: Sub_ < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , "not" => Ok (unsafe { Self :: Not (< unnamed :: Not < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node)) }) , _ => Err (type_sitter_lib :: IncorrectKind { node , kind : < Self as type_sitter_lib :: TypedNode < 'tree >> :: KIND , }) } } }
    #[automatically_derived]
    impl < 'tree > type_sitter_lib :: TypedNode < 'tree > for Binary_Boolean_Call_Divert_Identifier_ListValues_Number_Paren_Postfix_QualifiedName_String_Unary_Not__Sub__Not < 'tree > { const KIND : & 'static str = "{binary | boolean | call | divert | identifier | list_values | number | paren | postfix | qualified_name | string | unary | ! | - | not}" ; # [inline] fn node (& self) -> & tree_sitter :: Node < 'tree > { match self { Self :: Binary (x) => x . node () , Self :: Boolean (x) => x . node () , Self :: Call (x) => x . node () , Self :: Divert (x) => x . node () , Self :: Identifier (x) => x . node () , Self :: ListValues (x) => x . node () , Self :: Number (x) => x . node () , Self :: Paren (x) => x . node () , Self :: Postfix (x) => x . node () , Self :: QualifiedName (x) => x . node () , Self :: String (x) => x . node () , Self :: Unary (x) => x . node () , Self :: Not_ (x) => x . node () , Self :: Sub_ (x) => x . node () , Self :: Not (x) => x . node () , } } # [inline] fn node_mut (& mut self) -> & mut tree_sitter :: Node < 'tree > { match self { Self :: Binary (x) => x . node_mut () , Self :: Boolean (x) => x . node_mut () , Self :: Call (x) => x . node_mut () , Self :: Divert (x) => x . node_mut () , Self :: Identifier (x) => x . node_mut () , Self :: ListValues (x) => x . node_mut () , Self :: Number (x) => x . node_mut () , Self :: Paren (x) => x . node_mut () , Self :: Postfix (x) => x . node_mut () , Self :: QualifiedName (x) => x . node_mut () , Self :: String (x) => x . node_mut () , Self :: Unary (x) => x . node_mut () , Self :: Not_ (x) => x . node_mut () , Self :: Sub_ (x) => x . node_mut () , Self :: Not (x) => x . node_mut () , } } # [inline] fn into_node (self) -> tree_sitter :: Node < 'tree > { match self { Self :: Binary (x) => x . into_node () , Self :: Boolean (x) => x . into_node () , Self :: Call (x) => x . into_node () , Self :: Divert (x) => x . into_node () , Self :: Identifier (x) => x . into_node () , Self :: ListValues (x) => x . into_node () , Self :: Number (x) => x . into_node () , Self :: Paren (x) => x . into_node () , Self :: Postfix (x) => x . into_node () , Self :: QualifiedName (x) => x . into_node () , Self :: String (x) => x . into_node () , Self :: Unary (x) => x . into_node () , Self :: Not_ (x) => x . into_node () , Self :: Sub_ (x) => x . into_node () , Self :: Not (x) => x . into_node () , } } }
    #[doc = "one of `{! | - | not}`:\n- [symbols::Not_]\n- [symbols::Sub_]\n- [unnamed::Not]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Not__Sub__Not<'tree> {
        Not_(symbols::Not_<'tree>),
        Sub_(symbols::Sub_<'tree>),
        Not(unnamed::Not<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Not__Sub__Not<'tree> {
        #[doc = "Returns the node if it is of kind `!` ([symbols::Not_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn not_(self) -> Option<symbols::Not_<'tree>> {
            match self {
                Self::Not_(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `-` ([symbols::Sub_]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn sub_(self) -> Option<symbols::Sub_<'tree>> {
            match self {
                Self::Sub_(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `not` ([unnamed::Not]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn not(self) -> Option<unnamed::Not<'tree>> {
            match self {
                Self::Not(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Not__Sub__Not<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
            match node.kind() {
                "!" => Ok(unsafe {
                    Self::Not_(<symbols::Not_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "-" => Ok(unsafe {
                    Self::Sub_(<symbols::Sub_<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "not" => Ok(unsafe {
                    Self :: Not (< unnamed :: Not < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Not__Sub__Not<'tree> {
        const KIND: &'static str = "{! | - | not}";
        #[inline]
        fn node(&self) -> &tree_sitter::Node<'tree> {
            match self {
                Self::Not_(x) => x.node(),
                Self::Sub_(x) => x.node(),
                Self::Not(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut tree_sitter::Node<'tree> {
            match self {
                Self::Not_(x) => x.node_mut(),
                Self::Sub_(x) => x.node_mut(),
                Self::Not(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> tree_sitter::Node<'tree> {
            match self {
                Self::Not_(x) => x.into_node(),
                Self::Sub_(x) => x.into_node(),
                Self::Not(x) => x.into_node(),
            }
        }
    }
}
