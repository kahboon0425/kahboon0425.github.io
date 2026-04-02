use crate::components::Navbar;
use leptos::prelude::*;

struct WorkProject {
    name: &'static str,
    category: &'static str,
    cover: &'static str,
    description: &'static str,
    images: &'static [&'static str],
}

const CATEGORIES: &[&str] = &[
    "Christmas",
    "CNY",
    "Hari Raya",
    "Mid Autumn",
    "Other Events",
];

// Add real project names / descriptions / images here as you get more work.
const PROJECTS: &[WorkProject] = &[
    WorkProject {
        name: "Setapak Mall Christmas 2025",
        category: "Christmas",
        cover: "assets/images/portfolio/3D-Booth-Design 1/showcase.png",
        description: "3D booth design for Setapak Mall Christmas 2025 event. Designed to create a festive and immersive shopping experience.",
        images: &[
            "assets/images/portfolio/3D-Booth-Design 1/showcase.png",
            "assets/images/portfolio/3D-Booth-Design 1/booth 1.png",
        ],
    },
    WorkProject {
        name: "CNY Booth 2025",
        category: "CNY",
        cover: "assets/images/portfolio/3D-Booth-Design 2/showcase.png",
        description: "Chinese New Year booth design featuring traditional festive elements and vibrant decorations.",
        images: &[
            "assets/images/portfolio/3D-Booth-Design 2/showcase.png",
            "assets/images/portfolio/3D-Booth-Design 2/booth 2.png",
        ],
    },
    WorkProject {
        name: "Hari Raya Booth 2025",
        category: "Hari Raya",
        cover: "assets/images/portfolio/3D-Booth-Design 3/showcase.png",
        description: "Hari Raya Aidilfitri booth design blending modern aesthetics with traditional Malay motifs.",
        images: &[
            "assets/images/portfolio/3D-Booth-Design 3/showcase.png",
            "assets/images/portfolio/3D-Booth-Design 3/booth 3.png",
        ],
    },
    WorkProject {
        name: "Travel Fair Booth",
        category: "Other Events",
        cover: "assets/images/portfolio/3D-Booth-Design 4/showcase.png",
        description: "3D booth design for a travel fair event, highlighting destination themes and travel experiences.",
        images: &[
            "assets/images/portfolio/3D-Booth-Design 4/showcase.png",
            "assets/images/portfolio/3D-Booth-Design 4/booth 4-1.png",
            "assets/images/portfolio/3D-Booth-Design 4/booth 4-2.png",
            "assets/images/portfolio/3D-Booth-Design 4/booth 4-3.png",
        ],
    },
];

#[component]
pub fn Work() -> impl IntoView {
    let (selected_cat, set_selected_cat) = signal(0usize);
    let (selected_project, set_selected_project) = signal::<Option<usize>>(None);

    view! {
        <div class="min-h-screen bg-white">
            <Navbar />

            {move || match selected_project.get() {

                // ── Project detail view ───────────────────────────────
                Some(proj_idx) => {
                    let p = &PROJECTS[proj_idx];
                    view! {
                        <div class="px-10 py-12 md:px-20">
                            // Back + title
                            <div class="relative flex justify-center items-center mb-8">
                                <button
                                    class="absolute left-0 text-xl transition hover:text-pink-400 hover:scale-110"
                                    on:click=move |_| set_selected_project.set(None)
                                >
                                    "← Back"
                                </button>
                                <h1 class="text-4xl font-bold">{p.name}</h1>
                            </div>

                            // Two-column layout: images left, details right
                            <div class="flex flex-col gap-8 lg:flex-row">
                                // Left — image grid
                                <div class="grid flex-1 grid-cols-2 gap-4 md:grid-cols-3">
                                    {p.images.iter().map(|img| view! {
                                        <img
                                            class="object-cover w-full rounded-xl shadow-md aspect-square transition hover:scale-[1.02] hover:shadow-xl"
                                            src=*img
                                            alt=p.name
                                        />
                                    }).collect::<Vec<_>>()}
                                </div>

                                // Right — project details
                                <div class="flex flex-col gap-4 p-8 w-full rounded-2xl border border-gray-200 shadow-md lg:w-80 xl:w-96">
                                    <h2 class="text-2xl font-bold">"Project Details"</h2>
                                    <div class="w-10 h-1 bg-pink-400 rounded-full"></div>
                                    <p class="leading-relaxed text-gray-600">{p.description}</p>
                                    <div class="flex gap-2 mt-4">
                                        <span class="inline-block px-3 py-1 text-sm font-medium text-pink-600 bg-pink-100 rounded-full">
                                            {p.category}
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                    .into_any()
                }

                // ── Category tabs + project grid ──────────────────────
                None => view! {
                    <div class="px-10 py-12 md:px-20">
                        // Header
                        <div class="relative flex justify-center items-center mb-8">
                            <a
                                href="/projects"
                                class="absolute left-0 text-xl transition hover:text-pink-400 hover:scale-110"
                            >
                                "← Back"
                            </a>
                            <h1 class="text-5xl font-bold">"Work"</h1>
                        </div>

                        // Category tab bar
                        <div class="flex gap-6 mb-8 border-b border-gray-200 overflow-x-auto justify-center">
                            {CATEGORIES
                                .iter()
                                .enumerate()
                                .map(|(i, cat)| view! {
                                    <button
                                        class=move || {
                                            let base = "pb-3 text-lg font-medium whitespace-nowrap transition hover:text-pink-400";
                                            if selected_cat.get() == i {
                                                format!("{base} text-black border-b-2 border-black")
                                            } else {
                                                format!("{base} text-gray-400")
                                            }
                                        }
                                        on:click=move |_| set_selected_cat.set(i)
                                    >
                                        {*cat}
                                    </button>
                                })
                                .collect::<Vec<_>>()}
                        </div>

                        // Project cards for selected category
                        <div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3">
                            {move || {
                                let cat = CATEGORIES[selected_cat.get()];
                                let items: Vec<_> = PROJECTS
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, p)| p.category == cat)
                                    .map(|(i, p)| view! {
                                        // Stacked card wrapper
                                        <div
                                            class="group relative cursor-pointer aspect-[3/4] p-2"
                                            on:click=move |_| set_selected_project.set(Some(i))
                                        >
                                            // Shadow layer — slight rotation
                                            <div class="absolute inset-2 rounded-2xl bg-gray-300 shadow transition-all duration-300 ease-out rotate-[3deg] group-hover:rotate-[4deg]"></div>
                                            // Top card
                                            <div class="overflow-hidden absolute inset-2 rounded-2xl shadow-lg transition-all duration-300 ease-out group-hover:-translate-y-1 group-hover:shadow-2xl">
                                                <img
                                                    class="object-cover w-full h-full"
                                                    src=p.cover
                                                    alt=p.name
                                                />
                                                <div class="flex absolute inset-0 items-end p-5 bg-gradient-to-t from-black/70 to-transparent">
                                                    <h2 class="text-lg font-bold leading-snug text-white">{p.name}</h2>
                                                </div>
                                                <div class="flex absolute inset-0 flex-col gap-3 justify-center items-center translate-y-full backdrop-blur-md bg-black/50 transition-transform duration-500 ease-out group-hover:translate-y-0">
                                                    <p class="text-xl font-bold text-white">"Click to view more"</p>
                                                </div>
                                            </div>
                                        </div>
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
