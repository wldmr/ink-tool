use super::Db;

#[salsa::tracked]
pub(crate) struct Var<'db> {
    pub(crate) name: String,
    pub(crate) init_value: String,
    pub(crate) is_const: bool,
}

#[salsa::tracked]
pub(crate) struct Temp<'db> {
    pub(crate) name: String,
    pub(crate) init_value: String,
}

#[salsa::tracked]
pub(crate) struct List<'db> {
    pub(crate) name: String,
    pub(crate) values: Vec<ListValue<'db>>,
}

#[salsa::tracked]
pub(crate) struct ListValue<'db> {
    pub(crate) list: List<'db>,
    /// the name as defined in the ink; full name is `list.simple_name`
    pub(crate) simple_name: String,
}

#[salsa::tracked]
impl<'db> ListValue<'db> {
    pub(crate) fn full_name(&self, db: &'db dyn Db) -> String {
        format!("{}.{}", self.list(db).name(db), self.simple_name(db))
    }
}

#[salsa::tracked]
pub(crate) struct Knot<'db> {
    #[id]
    pub(crate) name: String,
    pub(crate) params: Vec<Param<'db>>,
}

#[salsa::tracked]
pub(crate) struct Function<'db> {
    #[id]
    pub(crate) name: String,
    pub(crate) params: Vec<Param<'db>>,
}

#[salsa::tracked]
pub(crate) struct External<'db> {
    #[id]
    pub(crate) name: String,
    pub(crate) params: Vec<Param<'db>>,
}

#[salsa::tracked]
pub(crate) struct Stitch<'db> {
    /// full name is `knot.local_name`
    #[id]
    pub(crate) local_name: String,
    #[id]
    pub(crate) parent: Option<Knot<'db>>,
    pub(crate) params: Vec<Param<'db>>,
}

#[salsa::tracked]
pub(crate) struct Label<'db> {
    /// full name is `knot.stitch.local_name`
    #[id]
    pub(crate) local_name: String,
    #[id]
    pub(crate) parent: Option<KnotOrStitch<'db>>,
}

#[salsa::tracked]
pub(crate) struct Param<'db> {
    pub(crate) name: String,
    pub(crate) is_ref: bool,
    pub(crate) is_divert: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, salsa::Update, derive_more::From)]
pub(crate) enum KnotOrStitch<'db> {
    Knot(Knot<'db>),
    Stitch(Stitch<'db>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, salsa::Update, derive_more::From)]
pub(crate) enum Definition<'db> {
    Var(Var<'db>),
    Temp(Temp<'db>),
    List(List<'db>),
    ListValue(ListValue<'db>),
    Knot(Knot<'db>),
    External(External<'db>),
    Stitch(Stitch<'db>),
    Label(Label<'db>),
    Param(Param<'db>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, salsa::Update, derive_more::From)]
pub(crate) enum Global<'db> {
    Var(Var<'db>),
    List(List<'db>),
    ListValue(ListValue<'db>),
    Knot(Knot<'db>),
    External(External<'db>),
    Stitch(Stitch<'db>),
    Label(Label<'db>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, salsa::Update, derive_more::From)]
pub(crate) enum Local<'db> {
    Temp(Temp<'db>),
    Stitch(Stitch<'db>),
    Label(Label<'db>),
    Param(Param<'db>),
}
