use std::fs;
use std::path::Path;

use serde::Serialize;

fn main() {
    let tree = build_tree(Path::new("assets"));

    let ron = ron::ser::to_string_pretty(
        &tree,
        ron::ser::PrettyConfig::default(),
    )
    .unwrap();
    fs::write(Path::new("assets").join("assets_index.ron"), ron)
        .unwrap();

    println!("cargo:rerun-if-changed=assets");
}

fn build_tree(path: &Path) -> Vec<AssetNode> {
    let mut nodes = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            let file_name = entry.file_name().into_string().unwrap();

            if path.is_dir() {
                nodes.push(AssetNode::Directory {
                    name: file_name,
                    children: build_tree(&path),
                });
            } else {
                nodes.push(AssetNode::File(file_name));
            }
        }
    }

    nodes
}

#[derive(Serialize)]
enum AssetNode {
    File(String),
    Directory {
        name: String,
        children: Vec<AssetNode>,
    },
}
