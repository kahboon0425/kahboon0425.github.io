use crate::{
    components::{CategoryCard, Navbar, PortfolioCell},
    extract, get_directory_node, portfolio_filter, AssetNode, SUBCATS,
};
use leptos::prelude::*;

const GROUP_IDX: usize = 0;
const GROUP_NAME: &str = "Work";

#[component]
pub fn Work() -> impl IntoView {
    let folders = get_directory_node(&["images", "portfolio"]).unwrap();
    let (selected_cat, set_selected_cat) = signal::<Option<usize>>(None);

    view! {
        <div class="min-h-screen bg-white">
            <Navbar />

            {move || match selected_cat.get() {
                // ── Subcategory cards ─────────────────────────────────
                None => view! {
                    <div class="px-10 py-12 md:px-20">
                        <div class="flex gap-4 items-center mb-10">
                            <a
                                href="/projects"
                                class="text-xl transition hover:text-pink-400 hover:scale-110"
                            >
                                "← Back"
                            </a>
                            <h1 class="text-5xl font-bold">{GROUP_NAME}</h1>
                        </div>
                        <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
                            {SUBCATS
                                .iter()
                                .enumerate()
                                .filter(|(_, s)| s.0 == GROUP_IDX)
                                .map(|(i, (_, name, cover, is_video, _))| {
                                    view! {
                                        <CategoryCard
                                            name=name
                                            cover=cover
                                            is_video=*is_video
                                            aspect="aspect-[4/3]"
                                            on_click=move || set_selected_cat.set(Some(i))
                                        />
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    </div>
                }
                .into_any(),

                // ── Portfolio items ───────────────────────────────────
                Some(subcat_idx) => {
                    let (_, cat_name, _, _, filter_idx) = SUBCATS[subcat_idx];
                    view! {
                        <div class="px-10 py-12 md:px-20">
                            <div class="flex gap-4 items-center mb-8">
                                <button
                                    class="text-xl transition hover:text-pink-400 hover:scale-110"
                                    on:click=move |_| set_selected_cat.set(None)
                                >
                                    "← Back"
                                </button>
                                <h1 class="text-5xl font-bold">{cat_name}</h1>
                            </div>
                            <div class="flex flex-wrap gap-8 justify-center items-center">
                                {folders
                                    .iter()
                                    .filter_map(|f| {
                                        let folder_name = extract!(
                                            &f, AssetNode::Directory = {name}
                                        )?;
                                        if !portfolio_filter(folder_name, filter_idx) {
                                            return None;
                                        }
                                        Some(view! {
                                            <PortfolioCell
                                                content=move || {
                                                    if folder_name.contains("Anim") {
                                                        view! {
                                                            <video
                                                                class="transition duration-300 active:opacity-100 big-img"
                                                                oncanplay="this.muted=true"
                                                                loop
                                                                autoplay
                                                                playsinline
                                                                controls
                                                            >
                                                                <source
                                                                    src=format!(
                                                                        "assets/images/portfolio/{}/showcase.mp4",
                                                                        folder_name,
                                                                    )
                                                                    type="video/mp4"
                                                                />
                                                            </video>
                                                        }
                                                        .into_any()
                                                    } else {
                                                        view! {
                                                            <img
                                                                class="transition duration-300 active:opacity-100 big-img"
                                                                src=format!(
                                                                    "assets/images/portfolio/{}/showcase.png",
                                                                    folder_name,
                                                                )
                                                            />
                                                        }
                                                        .into_any()
                                                    }
                                                }
                                                on_click=move |_| {}
                                            />
                                        })
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                        </div>
                    }
                    .into_any()
                }
            }}
        </div>
    }
}
