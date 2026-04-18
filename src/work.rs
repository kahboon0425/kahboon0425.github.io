use crate::{
    components::HamburgerMenu,
    extract, get_directory_node, AssetNode,
};
use leptos::prelude::*;
use leptos::web_sys;

/// "Theme-Santa's-Dessert-Party" → "Santa's Dessert Party"
/// "Chinese-New-Year" → "Chinese New Year"
fn folder_to_display_name(folder: &str) -> String {
    folder
        .strip_prefix("Theme-")
        .unwrap_or(folder)
        .replace('-', " ")
}

#[component]
pub fn Work() -> impl IntoView {
    // Dynamically read category folders from the asset tree (folder_name, display_name)
    const CAT_ORDER: &[&str] = &["Christmas", "Chinese-New-Year", "Hari-Raya", "Mid-Autumn", "Small-Event"];
    let categories = StoredValue::new({
        let mut cats: Vec<(String, String)> = get_directory_node(&["images", "Work-Projects"])
            .unwrap_or(&[])
            .iter()
            .filter_map(|n| match n {
                AssetNode::Directory { name, .. } => Some((name.clone(), folder_to_display_name(name))),
                _ => None,
            })
            .collect();
        cats.sort_by_key(|(name, _)| {
            CAT_ORDER.iter().position(|&o| o == name.as_str()).unwrap_or(usize::MAX)
        });
        cats
    });

    let (selected_cat, set_selected_cat) = signal(0usize);
    // (category index, theme index within that category)
    let (selected_theme, set_selected_theme) = signal::<Option<(usize, usize)>>(None);

    view! {
        <div class="min-h-screen bg-white">
            <HamburgerMenu />

            {move || match selected_theme.get() {

                // ── Project detail view ───────────────────────────────
                Some((cat_idx, theme_idx)) => {
                    let (cat_folder, cat_display) = categories.with_value(|c: &Vec<(String,String)>| c[cat_idx].clone());
                    let cat_path = ["images", "Work-Projects", cat_folder.as_str()];
                    let themes = get_directory_node(&cat_path).unwrap_or(&[]);

                    let theme_dirs: Vec<(&str, &[AssetNode])> = themes
                        .iter()
                        .filter_map(|n| match n {
                            AssetNode::Directory { name, children } => {
                                Some((name.as_str(), children.as_slice()))
                            }
                            _ => None,
                        })
                        .collect();

                    let Some(&(theme_folder, theme_children)) = theme_dirs.get(theme_idx) else {
                        return view! { <p>"Not found."</p> }.into_any();
                    };

                    let display_name = folder_to_display_name(theme_folder);

                    let images: Vec<String> = theme_children
                        .iter()
                        .filter_map(|n| extract!(n, AssetNode::File = ()))
                        .filter(|name| {
                            let lower = name.to_lowercase();
                            lower.ends_with(".png")
                                || lower.ends_with(".jpg")
                                || lower.ends_with(".mp4")
                        })
                        .map(|name| {
                            format!(
                                "assets/images/Work-Projects/{}/{}/{}",
                                cat_folder.as_str(), theme_folder, name
                            )
                        })
                        .collect();

                    let description: Option<String> = theme_children
                        .iter()
                        .find_map(|n| match n {
                            AssetNode::TextFile { content, .. } => Some(content.clone()),
                            _ => None,
                        });

                    let images_for_lightbox = images.clone();
                    let images_for_kb = images.clone();
                    let (zoomed_idx, set_zoomed_idx) = signal::<Option<usize>>(None);

                    // Keyboard navigation for lightbox
                    let _kb = window_event_listener(leptos::ev::keydown, move |e: web_sys::KeyboardEvent| {
                        if let Some(idx) = zoomed_idx.get() {
                            let total = images_for_kb.len();
                            match e.key().as_str() {
                                "ArrowLeft"  => set_zoomed_idx.set(Some(if idx == 0 { total - 1 } else { idx - 1 })),
                                "ArrowRight" => set_zoomed_idx.set(Some((idx + 1) % total)),
                                "Escape"     => set_zoomed_idx.set(None),
                                _ => {}
                            }
                        }
                    });

                    view! {
                        // Lightbox overlay
                        {move || zoomed_idx.get().map(|idx| {
                            let src = images_for_lightbox[idx].clone();
                            let total = images_for_lightbox.len();
                            view! {
                                <div
                                    class="flex fixed inset-0 z-50 justify-center items-center bg-black/80"
                                    on:click=move |_| set_zoomed_idx.set(None)
                                >
                                    // Left arrow
                                    <button
                                        class="absolute left-4 z-10 flex justify-center items-center w-12 h-12 text-2xl text-white rounded-full transition bg-white/20 hover:bg-white/40"
                                        on:click=move |e| {
                                            e.stop_propagation();
                                            set_zoomed_idx.set(Some(if idx == 0 { total - 1 } else { idx - 1 }));
                                        }
                                    >"‹"</button>

                                    // Image
                                    <img
                                        class="max-w-[80vw] max-h-[90vh] rounded-xl shadow-2xl object-contain cursor-zoom-out"
                                        src=src
                                        on:click=move |_| set_zoomed_idx.set(None)
                                    />

                                    // Right arrow
                                    <button
                                        class="absolute right-4 z-10 flex justify-center items-center w-12 h-12 text-2xl text-white rounded-full transition bg-white/20 hover:bg-white/40"
                                        on:click=move |e| {
                                            e.stop_propagation();
                                            set_zoomed_idx.set(Some((idx + 1) % total));
                                        }
                                    >"›"</button>

                                    // Counter
                                    <div class="absolute bottom-4 text-sm text-white/70">
                                        {format!("{} / {}", idx + 1, total)}
                                    </div>
                                </div>
                            }
                        })}

                        <div class="px-10 py-12 md:px-20">
                            // Back + title
                            <div class="relative flex justify-center items-center mb-8">
                                <button
                                    class="absolute left-0 text-xl transition cursor-pointer hover:text-[#fdbf3a] hover:scale-110"
                                    on:click=move |_| set_selected_theme.set(None)
                                >
                                    "← Back"
                                </button>
                                <h1 class="text-4xl font-bold">{display_name.clone()}</h1>
                            </div>

                            // Two-column layout: images left, details right
                            <div class="flex flex-col gap-8 lg:flex-row">
                                // Left — image grid
                                <div class="grid flex-1 grid-cols-2 gap-4 md:grid-cols-3">
                                    {images.into_iter().enumerate().map(|(i, img)| {
                                        view! {
                                            <img
                                                class="object-cover w-full rounded-xl shadow-md aspect-square transition cursor-zoom-in hover:scale-[1.02] hover:shadow-xl"
                                                src=img
                                                alt=display_name.clone()
                                                on:click=move |_| set_zoomed_idx.set(Some(i))
                                            />
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>

                                // Right — project details
                                <div class="flex flex-col gap-4 p-8 w-full rounded-2xl border border-gray-200 shadow-md self-start lg:w-80 xl:w-96">
                                    <h2 class="text-2xl font-bold">"Project Details"</h2>
                                    <div class="w-10 h-1 rounded-full" style="background-color: #fdbf3a;"></div>
                                    {description.map(|desc| view! {
                                        <p class="leading-relaxed text-gray-600">{desc}</p>
                                    })}
                                    <div class="flex gap-2 mt-2">
                                        <span class="inline-block px-3 py-1 text-sm font-medium text-black rounded-full" style="background-color: #fdbf3a;">
                                            {cat_display.clone()}
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                    .into_any()
                }

                // ── Category tabs + theme grid ──────────────────────
                None => view! {
                    <div class="px-10 py-12 md:px-20">
                        // Header
                        <div class="relative flex justify-center items-center mb-8">
                            <a
                                href="/projects"
                                class="absolute left-0 text-xl transition hover:text-[#fdbf3a] hover:scale-110"
                            >
                                "← Back"
                            </a>
                            <h1 class="text-5xl font-bold">"Work"</h1>
                        </div>

                        // Category tab bar
                        <div class="flex gap-6 mb-8 border-b border-gray-200 overflow-x-auto justify-center">
                            {categories.with_value(|c: &Vec<(String,String)>| {
                                c.iter()
                                    .enumerate()
                                    .map(|(i, (_, display))| {
                                        let display = display.clone();
                                        view! {
                                            <button
                                                class=move || {
                                                    let base = "pb-3 text-lg font-medium whitespace-nowrap transition hover:text-[#fdbf3a]";
                                                    if selected_cat.get() == i {
                                                        format!("{base} text-[#fdbf3a] border-b-2 border-[#fdbf3a]")
                                                    } else {
                                                        format!("{base} text-gray-400")
                                                    }
                                                }
                                                on:click=move |_| set_selected_cat.set(i)
                                            >
                                                {display.clone()}
                                            </button>
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            })}
                        </div>

                        // Theme cards for selected category
                        <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                            {move || {
                                let cat_idx = selected_cat.get();
                                let cat_folder = categories.with_value(|c: &Vec<(String,String)>| c[cat_idx].0.clone());
                                let cat_path = ["images", "Work-Projects", cat_folder.as_str()];
                                let themes = get_directory_node(&cat_path).unwrap_or(&[]);

                                let items: Vec<_> = themes
                                    .iter()
                                    .enumerate()
                                    .filter_map(|(theme_idx, n)| {
                                        let (theme_folder, children) = match n {
                                            AssetNode::Directory { name, children } => {
                                                (name.as_str(), children.as_slice())
                                            }
                                            _ => return None,
                                        };

                                        // Use cover.png if it exists, otherwise first image found
                                        let cover = {
                                            let explicit = children
                                                .iter()
                                                .filter_map(|c| extract!(c, AssetNode::File = ()))
                                                .find(|name| name.to_lowercase() == "cover.png");
                                            let fallback = children
                                                .iter()
                                                .filter_map(|c| extract!(c, AssetNode::File = ()))
                                                .find(|name| {
                                                    let lower = name.to_lowercase();
                                                    lower.ends_with(".png") || lower.ends_with(".jpg")
                                                });
                                            explicit.or(fallback)
                                        };

                                        let cover = cover.map(|name| {
                                            format!(
                                                "assets/images/Work-Projects/{}/{}/{}",
                                                cat_folder.as_str(), theme_folder, name
                                            )
                                        })?;

                                        let display_name = folder_to_display_name(theme_folder);

                                        Some(view! {
                                            <div
                                                class="group relative cursor-pointer aspect-[3/4] p-2"
                                                on:click=move |_| set_selected_theme.set(Some((cat_idx, theme_idx)))
                                            >
                                                // Shadow layer
                                                <div class="absolute inset-2 rounded-2xl bg-gray-300 shadow transition-all duration-300 ease-out rotate-[3deg] group-hover:rotate-[4deg]"></div>
                                                // Top card
                                                <div class="overflow-hidden absolute inset-2 rounded-2xl shadow-lg transition-all duration-300 ease-out group-hover:-translate-y-1 group-hover:shadow-2xl">
                                                    <img
                                                        class="object-cover w-full h-full"
                                                        src=cover
                                                        alt=display_name.clone()
                                                    />
                                                    <div class="flex absolute inset-0 items-end p-5 bg-gradient-to-t from-black/70 to-transparent">
                                                        <h2 class="text-lg font-bold leading-snug text-white">
                                                            {display_name.clone()}
                                                        </h2>
                                                    </div>
                                                    <div class="flex absolute inset-0 flex-col gap-3 justify-center items-center translate-y-full backdrop-blur-md bg-black/50 transition-transform duration-500 ease-out group-hover:translate-y-0">
                                                        <p class="text-xl font-bold text-white">"Click to view more"</p>
                                                    </div>
                                                </div>
                                            </div>
                                        })
                                    })
                                    .collect();

                                if items.is_empty() {
                                    view! {
                                        <p class="col-span-3 py-20 text-center text-gray-400">
                                            "No projects in this category yet."
                                        </p>
                                    }
                                    .into_any()
                                } else {
                                    items.into_any()
                                }
                            }}
                        </div>
                    </div>
                }
                .into_any(),
            }}
        </div>
    }
}
