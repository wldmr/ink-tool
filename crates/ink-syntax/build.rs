use std::env;
use std::path::Path;
use tree_sitter_ink;
use type_sitter_gen::{generate_nodes, NodeTypeMap};

fn main() {
    compile_type_sitter();
    println!("cargo::rerun-if-changed=build.rs");
}

fn compile_type_sitter() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let mut node_map = NodeTypeMap::try_from(tree_sitter_ink::NODE_TYPES)
        .expect("generating nodes should work, otherwise the feature is pointless");
    let named: Vec<_> = node_map
        .values()
        // Ignore implicit nodes (i.e. supertypes).
        // Including supertypes would lead to ambiguities (different possible typed nodes) when iterating over the tree.
        .filter(|it| it.name.is_named && !it.name.is_implicit())
        .map(|node| node.name.clone())
        .collect();
    _ = node_map
        .add_custom_supertype("_all_named", named.clone())
        .expect("this shouldn't already exist");
    let scope_block = node_map
        .add_custom_supertype(
            "_scope_block",
            named
                .iter()
                .filter(|it| matches!(it.sexp_name.as_str(), "ink" | "knot_block" | "stitch_block"))
                .cloned()
                .collect::<Vec<_>>(),
        )
        .expect("this shouldn't already exist");
    let flow_block = node_map
        .add_custom_supertype(
            "_flow_block",
            named
                .iter()
                .filter(|it| matches!(it.sexp_name.as_str(), "choice_block" | "gather_block"))
                .cloned()
                .collect::<Vec<_>>(),
        )
        .expect("this shouldn't already exist");
    let block = node_map
        .add_custom_supertype("_block", vec![scope_block, flow_block])
        .expect("this shouldn't already exist");
    let definitions = node_map
        .add_custom_supertype(
            "_definitions",
            named
                .iter()
                .filter(|it| {
                    matches!(
                        it.sexp_name.as_str(),
                        "external"
                            | "global"
                            | "knot"
                            | "label"
                            | "list"
                            | "list_value_def"
                            | "param"
                            | "stitch"
                            | "temp_def"
                    )
                })
                .cloned()
                .collect::<Vec<_>>(),
        )
        .expect("this shouldn't already exist");
    let usages = node_map
        .add_custom_supertype(
            "_usages",
            named
                .iter()
                .filter(|it| matches!(it.sexp_name.as_str(), "qualified_name" | "identifier"))
                .cloned()
                .collect::<Vec<_>>(),
        )
        .expect("this shouldn't already exist");
    let _of_interest = node_map
        .add_custom_supertype("_of_interest", vec![definitions, usages, block])
        .expect("this shouldn't already exist");
    let type_sitter_ink_types = generate_nodes(node_map)
        .expect("generating rust code should work, otherwise the feature is pointless")
        .into_string();

    let type_sitter_ink_path = Path::new(&out_dir).join("type_sitter_ink.rs");
    println!("cargo::warning=Writing {}", type_sitter_ink_path.display());
    std::fs::write(&type_sitter_ink_path, type_sitter_ink_types).unwrap();
}
