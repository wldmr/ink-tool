/// Build a query struct and an extension trait out of an “impl block” looking
/// syntax.
///
/// This looks more natural than the query provided by the library, and has the
/// added advantage that rustfmt works on it.
///
/// ``` .rust
/// compose_query!({
///     pub impl Ops<OpsV, Getters> {
///         fn thing(id: Id, count: Count) -> Thing;
///     }
/// })
/// ```
///
/// will create an `Ops` query enum with a `thing` variant, a corresponding `OpsV`
/// enum, all the conversion implementations, and, notably, an extension trait on
/// `Db` to get the cached result of `db.thing(id, count) -> Cached<'_, Ops, Thing`.
macro_rules! composite_query {
    ({
        // Query input and output types
        $vis:vis impl $query:ident<$value:ident, $trait:ident> {

            // Custom types for subquery variants:
            $(
                $(#[doc = $subquery_doc:literal])*
                $subquery_vis:vis fn $subquery:ident($($sub_name:ident: $sub_ty:ty),*) -> $ty:ty;
            )+

        }
    }) => {

        // We wrap the query enum in a struct, to be able to keep the variants private.
        #[derive(PartialEq, Eq, Clone, Hash, Copy)]
        $vis struct $query(_Q);
        // Toplevel query with delegating variants
        #[derive(PartialEq, Eq, Clone, Hash, Copy)]
        enum _Q {
            $(
                $(#[doc = $subquery_doc])*
                $subquery($subquery),
            )+
        }

        // Toplevel value (i.e. the result type for each subquery)
        $vis enum $value {
            $(
                $(#[doc = $subquery_doc])*
                $subquery($ty),
            )*
        }

        // Implement the toplevel query (by simply delegating via compose_query)
        impl mini_milc::Query for $query {
            type Value = $value;
            fn value(&self, db: &impl mini_milc::Db<$query>, old: mini_milc::Old<$value>) -> mini_milc::Updated<$value> {
                match &self.0 {
                    $(_Q::$subquery(sub) => mini_milc::compose_query(db, sub, old),)+
                }
            }
        }

        // The individual subquery structs that we need to define:
        $(
            #[derive(PartialEq, Eq, Clone, Hash, Copy)]
            $(#[doc = $subquery_doc])*
            $subquery_vis struct $subquery {$($sub_name: $sub_ty),*}
        )*

        // And the conversion traits.
        // First for the subquery types we just generated:
        $(
            impl From<$subquery> for $query {
                fn from(value: $subquery) -> $query {
                    $query(_Q::$subquery(value))
                }
            }

            impl mini_milc::Subvalue<$query, $subquery> for $ty {
                fn into_parent(self) -> $value {
                    $value::$subquery(self)
                }
                fn from_parent(value: $value) -> Self {
                    match value {
                        $value::$subquery(it) => it,
                        _ => unsafe { std::hint::unreachable_unchecked() },
                    }
                }
                fn from_parent_ref(value: &$value) -> &Self {
                    match value {
                        $value::$subquery(it) => it,
                        _ => unsafe { std::hint::unreachable_unchecked() },
                    }
                }
            }
        )*

        // The extension trait
        pub trait $trait: mini_milc::Db<$query> {
            $(
            $(#[doc = $subquery_doc])*
            fn $subquery(&self, $($sub_name: $sub_ty),*) -> mini_milc::Cached<'_, $query, $ty> {
                self.get($subquery {$($sub_name),*})
            }
            )+
        }
        impl<D: mini_milc::Db<$query>> $trait for D {}

    };
}

pub(super) use composite_query;
