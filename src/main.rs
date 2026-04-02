use std::sync::LazyLock;

use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use serde::Deserialize;

mod about;
mod components;
mod home;
mod personal;
mod projects;
mod work;
pub mod portfolio;

use about::About;
use home::Home;
use personal::Personal;
use projects::Projects;
use work::Work;

const ASSETS_INDEX: &str = include_str!("../assets/assets_index.ron");
pub(crate) static ASSET_TREE: LazyLock<Vec<AssetNode>> =
    LazyLock::new(|| ron::from_str(ASSETS_INDEX).unwrap());

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| view! { <h1>"Not Found"</h1> }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/about") view=About />
                    <Route path=path!("/projects") view=Projects />
                    <Route path=path!("/work") view=Work />
                    <Route path=path!("/personal") view=Personal />
                    <Route
                        path=path!("/*any")
                        view=|| view! { <h1>"Not Found"</h1> }
                    />
                </Routes>
            </main>
        </Router>
    }
}

// ── Shared subcategory data ───────────────────────────────────────────────────
// (group_idx, display_name, cover_path, is_video, filter_idx)
// group 1 = Personal only (Work page now has its own PROJECTS const)
pub(crate) const SUBCATS: &[(usize, &str, &str, bool, usize)] = &[
    (1, "Character Design",    "assets/images/portfolio/3D-Anim-Char-Sushi Queen/showcase.mp4", true,  0),
    (1, "Animation",           "assets/images/portfolio/3D-Anim-Map/showcase.mp4",              true,  1),
    (1, "CGI",                 "assets/images/portfolio/CGI-Anim-Balls/showcase.mp4",           true,  3),
    (1, "3D Modeling",         "assets/images/portfolio/3D-Baby-Dragon/showcase.png",           false, 4),
    (1, "2D Art",              "assets/images/portfolio/2D-Fireball/showcase.png",              false, 5),
];

/// Filter portfolio folders by subcategory filter_idx.
pub(crate) fn portfolio_filter(name: &str, cat_idx: usize) -> bool {
    match cat_idx {
        0 => name.contains("Char"),
        1 => name.contains("Anim") && !name.contains("Char") && !name.contains("CGI"),
        2 => name.contains("Booth"),
        3 => name.contains("CGI"),
        4 => name.contains("3D") && !name.contains("Anim") && !name.contains("Booth"),
        5 => name.contains("2D"),
        _ => false,
    }
}

// ── Asset tree ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub enum AssetNode {
    File(String),
    Directory {
        name: String,
        children: Vec<AssetNode>,
    },
}

pub(crate) fn get_directory_node<'a>(paths: &'a [&str]) -> Option<&'a [AssetNode]> {
    get_directory_node_recursive(paths, &ASSET_TREE)
}

fn get_directory_node_recursive<'a>(
    paths: &'a [&str],
    nodes: &'a [AssetNode],
) -> Option<&'a [AssetNode]> {
    for node in nodes {
        match node {
            AssetNode::Directory { name, children } => {
                if paths[0] != name {
                    continue;
                }
                if paths.len() == 1 {
                    return Some(children);
                }
                return get_directory_node_recursive(&paths[1..], children);
            }
            AssetNode::File(_) => {}
        }
    }
    None
}

macro_rules! extract {
    ($e:expr, $p:path = ()) => {
        match $e {
            $p(value) => Some(value),
            _ => None,
        }
    };
    ($e:expr, $p:path = {$f:ident}) => {
        match $e {
            $p { $f, .. } => Some($f),
            _ => None,
        }
    };
}
pub(crate) use extract;
