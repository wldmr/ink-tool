pub mod types {
    use type_sitter_lib as type_sitter;
    include!(concat!(env!("OUT_DIR"), "/type_sitter_ink.rs"));
}

mod visitor;
pub(crate) use visitor::Visit;
pub(crate) use visitor::VisitInstruction;
pub(crate) use visitor::Visitor;
