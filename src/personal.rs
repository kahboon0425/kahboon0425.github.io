use crate::{
    components::HamburgerMenu,
    extract, get_directory_node, AssetNode,
};
use leptos::prelude::*;
use leptos::either::Either;
use leptos::web_sys;

fn folder_to_display_name(folder: &str) -> String {
    folder.replace('-', " ").replace('&', " & ")
}

fn render_description(text: String) -> impl IntoView {
    let lines: Vec<String> = text.lines().map(|l| l.to_string()).collect();
    lines.into_iter().map(|line| {
        if let Some(url_start) = line.find("https://") {
            let label = line[..url_start].trim_end_matches(|c: char| c == ':' || c.is_whitespace()).to_string();
            let url = line[url_start..].trim().to_string();
            let is_youtube = url.contains("youtube.com") || url.contains("youtu.be");
            let is_itch = url.contains("itch.io");
            let icon = if is_youtube { "assets/svg/youtube-brands.svg" } else if is_itch { "assets/svg/itch-brands.svg" } else { "" };
            let bg = if is_youtube { "#FF0000" } else if is_itch { "#fa5c5c" } else { "#555" };
            let btn_label = if is_youtube { "Watch on YouTube" } else if is_itch { "Play on itch.io" } else { "Open link" };
            view! {
                <div class="flex flex-col gap-1.5">
                    <span class="text-base font-medium text-gray-700">{label}</span>
                    <a href=url target="_blank" rel="noopener noreferrer"
                        class="flex items-center gap-2 px-4 py-2 rounded-lg text-base font-semibold text-white transition hover:opacity-80"
                        style=format!("background-color: {bg}; width: fit-content;")>
                        {if !icon.is_empty() { view! { <img src=icon class="w-5 h-5 invert" /> }.into_any() } else { view! { <span /> }.into_any() }}
                        {btn_label}
                    </a>
                </div>
            }.into_any()
        } else if line.trim().is_empty() {
            view! { <div class="h-2" /> }.into_any()
        } else {
            view! { <p class="text-base leading-relaxed text-gray-600">{line}</p> }.into_any()
        }
    }).collect::<Vec<_>>()
}

fn is_media(name: &str) -> bool {
    let l = name.to_lowercase();
    l.ends_with(".png") || l.ends_with(".jpg") || l.ends_with(".mp4")
}

fn first_media_path_in(children: &[AssetNode]) -> Option<String> {
    // Prefer a file named cover.* at this level
    let cover = children.iter().find_map(|n| match n {
        AssetNode::File(name) => {
            let l = name.to_lowercase();
            if l.starts_with("cover.") && (l.ends_with(".png") || l.ends_with(".jpg") || l.ends_with(".mp4")) {
                Some(name.clone())
            } else { None }
        }
        _ => None,
    });
    if cover.is_some() { return cover; }
    // Fall back to first media found (recurses into subdirs)
    children.iter().find_map(|n| match n {
        AssetNode::File(name) => {
            let l = name.to_lowercase();
            if l.ends_with(".png") || l.ends_with(".jpg") || l.ends_with(".mp4") { Some(name.clone()) } else { None }
        }
        AssetNode::Directory { name: dir, children } => {
            first_media_path_in(children).map(|img| format!("{}/{}", dir, img))
        }
        _ => None,
    })
}

fn render_lightbox_media(src: String, on_click: impl Fn() + 'static) -> impl IntoView {
    if src.to_lowercase().ends_with(".mp4") {
        Either::Left(view! {
            <video class="max-w-[80vw] max-h-[90vh] rounded-xl shadow-2xl object-contain cursor-pointer"
                autoplay loop playsinline prop:muted=true
                on:click=move |_| on_click()>
                <source src=src type="video/mp4" />
            </video>
        })
    } else {
        Either::Right(view! {
            <img class="max-w-[80vw] max-h-[90vh] rounded-xl shadow-2xl object-contain cursor-zoom-out"
                src=src on:click=move |_| on_click() />
        })
    }
}

fn render_cover(src: String) -> impl IntoView {
    if src.to_lowercase().ends_with(".mp4") {
        Either::Left(view! {
            <video class="object-cover w-full h-full" autoplay loop playsinline prop:muted=true>
                <source src=src type="video/mp4" />
            </video>
        })
    } else {
        Either::Right(view! { <img class="object-cover w-full h-full" src=src /> })
    }
}

fn render_media(src: String, on_click: impl Fn() + 'static) -> impl IntoView {
    if src.to_lowercase().ends_with(".mp4") {
        Either::Left(view! {
            <video
                class="object-cover w-full rounded-xl shadow-md aspect-square transition cursor-pointer hover:scale-[1.02] hover:shadow-xl"
                autoplay loop playsinline prop:muted=true
                on:click=move |_| on_click()
            >
                <source src=src type="video/mp4" />
            </video>
        })
    } else {
        Either::Right(view! {
            <img
                class="object-cover w-full rounded-xl shadow-md aspect-square transition cursor-zoom-in hover:scale-[1.02] hover:shadow-xl"
                src=src
                on:click=move |_| on_click()
            />
        })
    }
}

#[component]
pub fn Personal() -> impl IntoView {
    // Store (&'static str folder_name, &'static [AssetNode] children)
    const CAT_ORDER: &[&str] = &["Character-Design", "Sculpting", "Game-Art", "CGI", "2D-Art", "More"];
    let categories = StoredValue::new({
        let mut cats: Vec<(&'static str, &'static [AssetNode])> = get_directory_node(&["images", "Personal-Projects"])
            .unwrap_or(&[])
            .iter()
            .filter_map(|n| match n {
                AssetNode::Directory { name, children } => Some((name.as_str(), children.as_slice())),
                _ => None,
            })
            .collect();
        cats.sort_by_key(|(name, _)| {
            CAT_ORDER.iter().position(|&o| o == *name).unwrap_or(usize::MAX)
        });
        cats
    });

    let (selected_cat, set_selected_cat) = signal::<Option<usize>>(None);
    let (selected_theme, set_selected_theme) = signal::<Option<usize>>(None);
    let (zoomed_idx, set_zoomed_idx) = signal::<Option<usize>>(None);
    let images_total = RwSignal::new(0usize);

    let _kb = window_event_listener(leptos::ev::keydown, move |e: web_sys::KeyboardEvent| {
        if let Some(idx) = zoomed_idx.get() {
            let total = images_total.get();
            if total == 0 { return; }
            match e.key().as_str() {
                "ArrowLeft"  => set_zoomed_idx.set(Some(if idx == 0 { total - 1 } else { idx - 1 })),
                "ArrowRight" => set_zoomed_idx.set(Some((idx + 1) % total)),
                "Escape"     => set_zoomed_idx.set(None),
                _ => {}
            }
        }
    });

    view! {
        <div class="min-h-screen bg-white">
            <HamburgerMenu />

            {move || {
                let zoom = zoomed_idx.get();

                match (selected_cat.get(), selected_theme.get()) {

                    // ── Root: all category cards ──────────────────────
                    (None, _) => {
                        let cards = categories.with_value(|cats: &Vec<(&str, &[AssetNode])>| {
                            cats.iter().enumerate().filter_map(|(i, (folder, children))| {
                                let display = folder_to_display_name(folder);
                                // Find first image (may be inside a subfolder)
                                let cover = first_media_path_in(children)
                                    .map(|img| format!("assets/images/Personal-Projects/{}/{}", folder, img))?;
                                Some((i, display, cover))
                            }).collect::<Vec<_>>()
                        });

                        view! {
                            <div class="px-10 py-12 md:px-20">
                                <div class="mb-10">
                                    <div class="flex items-center pr-16 pt-3 mb-6 md:hidden">
                                        <a href="/projects" class="text-xl transition cursor-pointer hover:text-[#fdbf3a] hover:scale-110">"← Back"</a>
                                    </div>
                                    <div class="hidden relative justify-center items-center md:flex">
                                        <a href="/projects" class="absolute left-0 text-xl transition cursor-pointer hover:text-[#fdbf3a] hover:scale-110">"← Back"</a>
                                        <h1 class="text-5xl font-bold">"Personal"</h1>
                                    </div>
                                    <h1 class="text-4xl font-bold text-center w-full md:hidden">"Personal"</h1>
                                </div>
                                <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                                    {cards.into_iter().map(|(i, display, cover): (usize, String, String)| view! {
                                        <div class="group relative cursor-pointer aspect-[3/4] p-2"
                                            on:click=move |_| { set_selected_cat.set(Some(i)); set_selected_theme.set(None); }>
                                            <div class="absolute inset-2 rounded-2xl bg-gray-300 shadow transition-all duration-300 ease-out rotate-[3deg] group-hover:rotate-[4deg]"></div>
                                            <div class="overflow-hidden absolute inset-2 rounded-2xl shadow-lg transition-all duration-300 ease-out group-hover:-translate-y-1 group-hover:shadow-2xl">
                                                {render_cover(cover)}
                                                <div class="flex absolute inset-0 items-end p-5 bg-gradient-to-t from-black/70 to-transparent">
                                                    <div class="flex items-center justify-between w-full">
                                                        <h2 class="text-lg font-bold leading-snug text-white">{display.clone()}</h2>
                                                        <span class="text-white text-xl font-bold shrink-0 ml-2 md:hidden">{"→"}</span>
                                                    </div>
                                                </div>
                                                <div class="flex absolute inset-0 flex-col gap-3 justify-center items-center translate-y-full backdrop-blur-md bg-black/50 transition-transform duration-500 ease-out group-hover:translate-y-0">
                                                    <p class="text-xl font-bold text-white">"Click to view more"</p>
                                                </div>
                                            </div>
                                        </div>
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        }.into_any()
                    }

                    // ── Category selected ─────────────────────────────
                    (Some(cat_idx), None) => {
                        let (cat_folder, cat_children) = categories.with_value(|cats: &Vec<(&str, &[AssetNode])>| cats[cat_idx]);
                        let cat_display = folder_to_display_name(cat_folder);
                        let has_subdirs = cat_children.iter().any(|n| matches!(n, AssetNode::Directory { .. }));

                        if has_subdirs {
                            // Show subfolder cards
                            const SUB_ORDER: &[&str] = &["Sushi-Prawn&Salmon", "Sushi-Queen", "Baguette&PoloBun"];
                            let mut sub_dirs_sorted: Vec<(&str, &[AssetNode])> = cat_children.iter()
                                .filter_map(|n| match n {
                                    AssetNode::Directory { name, children } => Some((name.as_str(), children.as_slice())),
                                    _ => None,
                                }).collect();
                            sub_dirs_sorted.sort_by_key(|(name, _)| {
                                SUB_ORDER.iter().position(|&o| o == *name).unwrap_or(usize::MAX)
                            });
                            let cards: Vec<_> = sub_dirs_sorted.iter()
                                .enumerate()
                                .filter_map(|(ti, &(sub_folder, sub_children))| {
                                    let display = folder_to_display_name(sub_folder);
                                    let cover = first_media_path_in(sub_children)
                                        .map(|img| format!("assets/images/Personal-Projects/{}/{}/{}", cat_folder, sub_folder, img))?;
                                    Some((ti, display, cover))
                                }).collect();

                            view! {
                                <div class="px-10 py-12 md:px-20">
                                    <div class="mb-10">
                                        <div class="flex items-center pr-16 pt-3 mb-6 md:hidden">
                                            <button class="text-xl transition cursor-pointer hover:text-[#fdbf3a] hover:scale-110"
                                                on:click=move |_| set_selected_cat.set(None)>"← Back"</button>
                                        </div>
                                        <div class="hidden relative justify-center items-center md:flex">
                                            <button class="absolute left-0 text-xl transition cursor-pointer hover:text-[#fdbf3a] hover:scale-110"
                                                on:click=move |_| set_selected_cat.set(None)>"← Back"</button>
                                            <h1 class="text-5xl font-bold">{cat_display.clone()}</h1>
                                        </div>
                                        <h1 class="text-4xl font-bold text-center w-full md:hidden">{cat_display}</h1>
                                    </div>
                                    <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                                        {cards.into_iter().map(|(ti, display, cover): (usize, String, String)| view! {
                                            <div class="group relative cursor-pointer aspect-[3/4] p-2"
                                                on:click=move |_| set_selected_theme.set(Some(ti))>
                                                <div class="absolute inset-2 rounded-2xl bg-gray-300 shadow transition-all duration-300 ease-out rotate-[3deg] group-hover:rotate-[4deg]"></div>
                                                <div class="overflow-hidden absolute inset-2 rounded-2xl shadow-lg transition-all duration-300 ease-out group-hover:-translate-y-1 group-hover:shadow-2xl">
                                                    {render_cover(cover)}
                                                    <div class="flex absolute inset-0 items-end p-5 bg-gradient-to-t from-black/70 to-transparent">
                                                        <div class="flex items-center justify-between w-full">
                                                            <h2 class="text-lg font-bold leading-snug text-white">{display.clone()}</h2>
                                                            <span class="text-white text-xl font-bold shrink-0 ml-2 md:hidden">{"→"}</span>
                                                        </div>
                                                    </div>
                                                    <div class="flex absolute inset-0 flex-col gap-3 justify-center items-center translate-y-full backdrop-blur-md bg-black/50 transition-transform duration-500 ease-out group-hover:translate-y-0">
                                                        <p class="text-xl font-bold text-white">"Click to view more"</p>
                                                    </div>
                                                </div>
                                            </div>
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            // Direct images in category
                            let images: Vec<String> = cat_children.iter()
                                .filter_map(|n| extract!(n, AssetNode::File = ()))
                                .filter(|name| is_media(name))
                                .map(|name| format!("assets/images/Personal-Projects/{}/{}", cat_folder, name))
                                .collect();
                            images_total.set(images.len());
                            let images_lb = images.clone();
                            let description: Option<String> = cat_children.iter().find_map(|n| match n {
                                AssetNode::TextFile { content, .. } => Some(content.clone()),
                                _ => None,
                            });

                            view! {
                                {move || zoom.map(|idx| {
                                    let src = images_lb[idx].clone();
                                    let total = images_lb.len();
                                    view! {
                                        <div class="flex fixed inset-0 z-50 justify-center items-center bg-black/80"
                                            on:click=move |_| set_zoomed_idx.set(None)>
                                            <button class="absolute left-4 z-10 flex justify-center items-center w-12 h-12 text-2xl text-white rounded-full transition bg-white/20 hover:bg-white/40"
                                                on:click=move |e| { e.stop_propagation(); set_zoomed_idx.set(Some(if idx == 0 { total - 1 } else { idx - 1 })); }>"‹"</button>
                                            {render_lightbox_media(src, move || set_zoomed_idx.set(None))}
                                            <button class="absolute right-4 z-10 flex justify-center items-center w-12 h-12 text-2xl text-white rounded-full transition bg-white/20 hover:bg-white/40"
                                                on:click=move |e| { e.stop_propagation(); set_zoomed_idx.set(Some((idx + 1) % total)); }>"›"</button>
                                            <div class="absolute bottom-4 text-sm text-white/70">{format!("{} / {}", idx + 1, total)}</div>
                                        </div>
                                    }
                                })}
                                <div class="px-10 pt-10 pb-12 md:px-20 md:pt-12">
                                    <div class="mb-6">
                                        <div class="flex items-center pr-16 pt-3 mb-6 md:hidden">
                                            <button class="text-xl transition cursor-pointer hover:text-[#fdbf3a] hover:scale-110"
                                                on:click=move |_| set_selected_cat.set(None)>"← Back"</button>
                                        </div>
                                        <div class="hidden relative justify-center items-center md:flex">
                                            <button class="absolute left-0 text-xl transition cursor-pointer hover:text-[#fdbf3a] hover:scale-110"
                                                on:click=move |_| set_selected_cat.set(None)>"← Back"</button>
                                            <h1 class="text-5xl font-bold">{cat_display.clone()}</h1>
                                        </div>
                                        <h1 class="text-2xl font-bold text-center w-full md:hidden">{cat_display.clone()}</h1>
                                    </div>
                                    <div class="flex flex-col gap-8 lg:flex-row">
                                        <div class="grid flex-1 grid-cols-1 gap-4 md:grid-cols-3">
                                            {images.into_iter().enumerate().map(|(i, img)| {
                                                render_media(img, move || set_zoomed_idx.set(Some(i)))
                                            }).collect::<Vec<_>>()}
                                        </div>
                                        {description.map(|desc| view! {
                                            <div class="flex flex-col gap-4 p-8 w-full rounded-2xl border border-gray-200 shadow-md self-start lg:w-80 xl:w-96">
                                                <h2 class="text-2xl font-bold">"Project Details"</h2>
                                                <div class="w-10 h-1 rounded-full" style="background-color: #fdbf3a;"></div>
                                                {render_description(desc)}
                                            </div>
                                        })}
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }

                    // ── Subfolder detail: show images ─────────────────
                    (Some(cat_idx), Some(theme_idx)) => {
                        let (cat_folder, cat_children) = categories.with_value(|cats: &Vec<(&str, &[AssetNode])>| cats[cat_idx]);
                        const SUB_ORDER: &[&str] = &["Sushi-Prawn&Salmon", "Sushi-Queen", "Baguette&PoloBun"];
                        let mut sub_dirs: Vec<(&str, &[AssetNode])> = cat_children.iter().filter_map(|n| match n {
                            AssetNode::Directory { name, children } => Some((name.as_str(), children.as_slice())),
                            _ => None,
                        }).collect();
                        sub_dirs.sort_by_key(|(name, _)| {
                            SUB_ORDER.iter().position(|&o| o == *name).unwrap_or(usize::MAX)
                        });
                        let Some(&(theme_folder, theme_children)) = sub_dirs.get(theme_idx) else {
                            return view! { <p>"Not found."</p> }.into_any();
                        };
                        let display_name = folder_to_display_name(theme_folder);
                        let images: Vec<String> = theme_children.iter()
                            .filter_map(|n| extract!(n, AssetNode::File = ()))
                            .filter(|name| is_media(name))
                            .map(|name| format!("assets/images/Personal-Projects/{}/{}/{}", cat_folder, theme_folder, name))
                            .collect();
                        images_total.set(images.len());
                        let images_lb = images.clone();

                        view! {
                            {move || zoom.map(|idx| {
                                let src = images_lb[idx].clone();
                                let total = images_lb.len();
                                view! {
                                    <div class="flex fixed inset-0 z-50 justify-center items-center bg-black/80"
                                        on:click=move |_| set_zoomed_idx.set(None)>
                                        <button class="absolute left-4 z-10 flex justify-center items-center w-12 h-12 text-2xl text-white rounded-full transition bg-white/20 hover:bg-white/40"
                                            on:click=move |e| { e.stop_propagation(); set_zoomed_idx.set(Some(if idx == 0 { total - 1 } else { idx - 1 })); }>"‹"</button>
                                        {render_lightbox_media(src, move || set_zoomed_idx.set(None))}
                                        <button class="absolute right-4 z-10 flex justify-center items-center w-12 h-12 text-2xl text-white rounded-full transition bg-white/20 hover:bg-white/40"
                                            on:click=move |e| { e.stop_propagation(); set_zoomed_idx.set(Some((idx + 1) % total)); }>"›"</button>
                                        <div class="absolute bottom-4 text-sm text-white/70">{format!("{} / {}", idx + 1, total)}</div>
                                    </div>
                                }
                            })}
                            <div class="px-10 py-12 md:px-20">
                                <div class="mb-6">
                                    <div class="flex items-center pr-16 pt-3 mb-6 md:hidden">
                                        <button class="text-xl transition cursor-pointer hover:text-[#fdbf3a] hover:scale-110"
                                            on:click=move |_| set_selected_theme.set(None)>"← Back"</button>
                                    </div>
                                    <div class="hidden relative justify-center items-center md:flex">
                                        <button class="absolute left-0 text-xl transition cursor-pointer hover:text-[#fdbf3a] hover:scale-110"
                                            on:click=move |_| set_selected_theme.set(None)>"← Back"</button>
                                        <h1 class="text-4xl font-bold">{display_name.clone()}</h1>
                                    </div>
                                    <h1 class="text-2xl font-bold text-center w-full md:hidden">{display_name.clone()}</h1>
                                </div>
                                <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
                                    {images.into_iter().enumerate().map(|(i, img)| {
                                        render_media(img, move || set_zoomed_idx.set(Some(i)))
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        }.into_any()
                    }
                }
            }}
        </div>
    }
}
