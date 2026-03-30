use std::sync::LazyLock;

use leptos::ev::MouseEvent;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use serde::Deserialize;
// use leptos_use::{
//     UseMouseCoordType, UseMouseEventExtractor, UseMouseOptions, UseMouseReturn,
//     use_mouse_with_options,
// };
// use bevy::prelude::*;
// use leptos_bevy_canvas::prelude::*;

mod portfolio;

const ASSETS_INDEX: &str = include_str!("../assets/assets_index.ron");
static ASSET_TREE: LazyLock<Vec<AssetNode>> =
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
                    <Route path=path!("/portfolio") view=Portfolio />
                    // <Route path=path!("/portfolio/character") view=portfolio::Character/>
                    <Route
                        path=path!("/*any")
                        view=|| view! { <h1>"Not Found"</h1> }
                    />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen bg-white">
            // Navbar
            <header class="flex items-center justify-between px-10 py-5 border-b border-gray-200">
                <nav class="flex gap-10">
                    <a href="./" class="text-xl font-medium transition hover:text-pink-400">
                        "Home"
                    </a>
                    <a href="./about" class="text-xl font-medium transition hover:text-pink-400">
                        "About"
                    </a>
                    <a href="./portfolio" class="text-xl font-medium transition hover:text-pink-400">
                        "Projects"
                    </a>
                </nav>
                <a
                    href="./portfolio"
                    class="text-3xl font-bold transition hover:scale-110 hover:text-pink-400"
                >
                    "→"
                </a>
            </header>

            <div class="flex flex-col items-center px-10 py-12 md:px-20">
                <h1 class="mb-4 text-6xl font-bold">"About"</h1>

                <p class="mb-3 max-w-2xl text-center text-lg text-gray-600">
                    "Hi, I’m Kah Boon — an IT graduate passionate about 3D art. I create cute and stylized characters using Blender and ZBrush, and explore CGI, VFX, and booth design."
                </p>

                <a
                    href="https://docs.google.com/viewer?url=https://raw.githubusercontent.com/kahboon0425/kahboon_resume/refs/heads/main/resume.pdf"
                    class="mb-10 underline transition animate-bounce hover:text-blue-400 hover:scale-105"
                >
                    "View Resume"
                </a>

                // 2x2 section grid
                <div class="grid grid-cols-1 gap-6 w-full max-w-4xl md:grid-cols-2">

                    // Skills — pink
                    <div class="p-6 bg-pink-50 rounded-xl border-2 border-pink-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02] hover:bg-pink-100">
                        <h2 class="mb-4 text-2xl font-bold text-pink-700">"Skills"</h2>
                        <div class="flex flex-wrap gap-2">
                            {["3D Modeling", "Character Rigging", "Animation", "CGI / VFX", "Booth Design", "ZBrush Sculpting"]
                                .iter()
                                .map(|s| view! {
                                    <span class="px-3 py-1 text-sm font-medium text-pink-800 bg-pink-200 rounded-full">
                                        {*s}
                                    </span>
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    </div>

                    // Languages — blue
                    <div class="p-6 bg-sky-50 rounded-xl border-2 border-sky-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02] hover:bg-sky-100">
                        <h2 class="mb-4 text-2xl font-bold text-sky-700">"Languages"</h2>
                        <div class="flex flex-wrap gap-2">
                            {["English", "Mandarin", "Bahasa Malaysia"]
                                .iter()
                                .map(|l| view! {
                                    <span class="px-3 py-1 text-sm font-medium text-sky-800 bg-sky-200 rounded-full">
                                        {*l}
                                    </span>
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    </div>

                    // Experience — blue
                    <div class="p-6 bg-sky-50 rounded-xl border-2 border-sky-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02] hover:bg-sky-100">
                        <h2 class="mb-4 text-2xl font-bold text-sky-700">"Experience"</h2>
                        <ul class="flex flex-col gap-3">
                            <li>
                                <p class="font-semibold">"Igmax — Focus on Festival"</p>
                                <p class="text-sm text-gray-500">"Freelance 3D Artist"</p>
                                <p class="mt-1 text-sm text-gray-600">
                                    "Created 3D assets and visual designs for festival events and promotional materials."
                                </p>
                            </li>
                        </ul>
                    </div>

                    // Software — pink
                    <div class="p-6 bg-pink-50 rounded-xl border-2 border-pink-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02] hover:bg-pink-100">
                        <h2 class="mb-4 text-2xl font-bold text-pink-700">"Software"</h2>
                        <ul class="flex flex-col gap-2">
                            {["Blender", "ZBrush", "3ds Max", "Adobe After Effects"]
                                .iter()
                                .map(|s| view! {
                                    <li class="flex gap-2 items-center text-gray-700">
                                        <span class="w-2 h-2 rounded-full bg-pink-400 inline-block shrink-0" />
                                        {*s}
                                    </li>
                                })
                                .collect::<Vec<_>>()}
                        </ul>
                    </div>

                </div>
            </div>
        </div>
    }
}

#[component]
fn Portfolio() -> impl IntoView {
    const PATHS: &[&str] = &["images", "portfolio"];
    let folders = get_directory_node(PATHS).unwrap();

    // [(Key, Display)]
    const CATEGORIES: &[(&str, &str)] = &[
        ("All", "All"),
        ("3D", "3D"),
        ("Anim", "Animation"),
        ("CGI", "CGI"),
        ("Booth", "Booth Design"),
        ("Char", "Character"),
        ("2D", "2D"),
    ];

    let (filter_idx, set_filter_idx) = signal(0);

    view! {
        <main class="py-10 px-10 h-full bg-white">
            <div class="flex flex-row flex-wrap justify-between gap-15">
                <Icon
                    href="./"
                    src="assets/svg/arrow-left-solid.svg"
                    alt="Back Arrow SVG"
                />

                <div class="self-center py-2 animate-bounce xl:self-end">
                    <a href="https://docs.google.com/viewer?url=https://raw.githubusercontent.com/kahboon0425/kahboon_resume/refs/heads/main/resume.pdf">
                        <p class="underline transition cursor-pointer hover:text-blue-300 hover:scale-110">
                            View Resume
                        </p>
                    </a>
                </div>
            </div>


            <Title title="Portfolio" />

            <div class="flex justify-center mt-5 border-b border-gray-200">
                <nav class="flex gap-8 overflow-x-auto">

                    {CATEGORIES
                        .iter()
                        .enumerate()
                        .map(|(i, (_, c))| {
                            view! {
                                <a
                                    href=""
                                    class=move || {
                                        let base = "text-gray-500 hover:text-gray-700 px-3 py-2 text-sm font-medium";
                                        if filter_idx.get() == i {
                                            format!("{base} border-blue-600 border-b-2")
                                        } else {
                                            base.to_string()
                                        }
                                    }

                                    on:click=move |_| {
                                        set_filter_idx.set(i);
                                    }
                                >

                                    {*c}
                                </a>
                            }
                        })
                        .collect::<Vec<_>>()}

                </nav>
            </div>

            <div class="justify-center items-center py-10" />
            <div class="flex flex-row flex-wrap gap-8 justify-center items-center h-full">
                {move || {
                    folders
                        .iter()
                        .filter_map(|f| {
                            let folder_name = extract!(
                                &f, AssetNode::Directory = {name}
                            )?;
                            let idx = filter_idx.get();
                            if idx != 0
                                && folder_name.contains(CATEGORIES[idx].0) == false
                            {
                                return None;
                            }
                            Some(
                                // Filter according to folder name.
                                view! {
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
                                                            src="assets/images/portfolio/".to_string() + folder_name
                                                                + "/showcase.mp4"
                                                            type="video/mp4"
                                                        />
                                                    </video>
                                                }
                                                    .into_any()
                                            } else {
                                                view! {
                                                    <img
                                                        class="transition duration-300 active:opacity-100 big-img"
                                                        src="assets/images/portfolio/".to_string() + folder_name
                                                            + "/showcase.png"
                                                    />
                                                }
                                                    .into_any()
                                            }
                                        }
                                        on_click=move |m| {
                                            console_log(
                                                &("assets/images/portfolio/".to_string() + folder_name),
                                            );
                                            console_log(&format!("{m:?}"));
                                        }
                                    />
                                },
                            )
                        })
                        .collect::<Vec<_>>()
                }}
            </div>
        </main>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="bg-white">
            // Sticky navbar
            <header class="flex sticky top-0 z-50 items-center justify-between px-10 py-5 bg-white border-b border-gray-200">
                <nav class="flex gap-10">
                    <a href="#home" class="text-xl font-medium transition hover:text-pink-400">
                        "Home"
                    </a>
                    <a href="#about" class="text-xl font-medium transition hover:text-pink-400">
                        "About"
                    </a>
                    <a href="./portfolio" class="text-xl font-medium transition hover:text-pink-400">
                        "Projects"
                    </a>
                </nav>
                <a
                    href="./portfolio"
                    class="text-3xl font-bold transition hover:scale-110 hover:text-pink-400"
                >
                    "→"
                </a>
            </header>

            // ── Home section ──────────────────────────────────────────
            <section
                id="home"
                class="flex flex-col flex-1 gap-16 justify-center items-center px-10 py-16 min-h-screen md:flex-row md:px-24"
            >
                // Left column: circular logo + social icons
                <div class="flex flex-col gap-8 items-center">
                    <div class="flex overflow-hidden justify-center items-center rounded-full border-4 border-black size-64 md:size-80">
                        <img
                            class="object-contain w-full h-full"
                            src="assets/images/Sushi Queen Logo Transparent.png"
                            alt="Sushi Queen Logo"
                        />
                    </div>
                    <div class="flex flex-row gap-8">
                        <Icon
                            href="mailto:changkahboon25@gmail.com"
                            src="assets/svg/envelope-solid.svg"
                            alt="Email SVG"
                        />
                        <Icon
                            href="https://www.instagram.com/the_sushi_queen_art/"
                            src="assets/svg/instagram-brands.svg"
                            alt="Instagram SVG"
                        />
                        <Icon
                            href="https://www.linkedin.com/in/kahboon"
                            src="assets/svg/linkedin-brands.svg"
                            alt="LinkedIn SVG"
                        />
                        <Icon
                            href="https://wa.me/60127645817"
                            src="assets/svg/whatsapp-brands.svg"
                            alt="WhatsApp SVG"
                        />
                    </div>
                </div>

                // Right column: intro text
                <div class="flex flex-col gap-6 max-w-md">
                    <h1 class="text-5xl font-bold md:text-6xl">"Hi, I am Kah Boon"</h1>
                    <p class="text-lg leading-relaxed text-gray-600">
                        "3D Artist & Designer passionate about creating cute and stylized characters. Self-taught in Blender, ZBrush."
                    </p>
                    <div class="flex gap-3 mt-2">
                        <span class="inline-block w-3 h-3 rounded-full bg-pink-400" />
                        <span class="inline-block w-3 h-3 rounded-full bg-gray-300" />
                        <span class="inline-block w-3 h-3 rounded-full bg-gray-300" />
                        <span class="inline-block w-3 h-3 rounded-full bg-gray-300" />
                    </div>
                </div>
            </section>

            // ── About section ─────────────────────────────────────────
            <section
                id="about"
                class="flex flex-col items-center px-10 py-16 min-h-screen bg-gray-50 md:px-20"
            >
                <h1 class="mb-4 text-6xl font-bold">"About"</h1>

                <p class="mb-3 max-w-2xl text-center text-lg text-gray-600">
                    "Hi, I'm Kah Boon — an IT graduate passionate about 3D art. I create cute and stylized characters using Blender and ZBrush, and explore CGI, VFX, and booth design."
                </p>

                <a
                    href="https://docs.google.com/viewer?url=https://raw.githubusercontent.com/kahboon0425/kahboon_resume/refs/heads/main/resume.pdf"
                    class="mb-10 underline transition animate-bounce hover:text-blue-400 hover:scale-105"
                >
                    "View Resume"
                </a>

                // 2x2 section grid
                <div class="grid grid-cols-1 gap-6 w-full max-w-4xl md:grid-cols-2">

                    // Skills — pink
                    <div class="p-6 bg-pink-50 rounded-xl border-2 border-pink-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02] hover:bg-pink-100">
                        <h2 class="mb-4 text-2xl font-bold text-pink-700">"Skills"</h2>
                        <div class="flex flex-wrap gap-2">
                            {["3D Modeling", "Character Rigging", "Animation", "CGI / VFX", "Booth Design", "ZBrush Sculpting"]
                                .iter()
                                .map(|s| view! {
                                    <span class="px-3 py-1 text-sm font-medium text-pink-800 bg-pink-200 rounded-full">
                                        {*s}
                                    </span>
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    </div>

                    // Languages — blue
                    <div class="p-6 bg-sky-50 rounded-xl border-2 border-sky-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02] hover:bg-sky-100">
                        <h2 class="mb-4 text-2xl font-bold text-sky-700">"Languages"</h2>
                        <div class="flex flex-wrap gap-2">
                            {["English", "Mandarin", "Bahasa Malaysia"]
                                .iter()
                                .map(|l| view! {
                                    <span class="px-3 py-1 text-sm font-medium text-sky-800 bg-sky-200 rounded-full">
                                        {*l}
                                    </span>
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    </div>

                    // Experience — blue
                    <div class="p-6 bg-sky-50 rounded-xl border-2 border-sky-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02] hover:bg-sky-100">
                        <h2 class="mb-4 text-2xl font-bold text-sky-700">"Experience"</h2>
                        <ul class="flex flex-col gap-3">
                            <li>
                                <p class="font-semibold">"Igmax — Focus on Festival"</p>
                                <p class="text-sm text-gray-500">"Freelance 3D Artist"</p>
                                <p class="mt-1 text-sm text-gray-600">
                                    "Created 3D assets and visual designs for festival events and promotional materials."
                                </p>
                            </li>
                        </ul>
                    </div>

                    // Software — pink
                    <div class="p-6 bg-pink-50 rounded-xl border-2 border-pink-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02] hover:bg-pink-100">
                        <h2 class="mb-4 text-2xl font-bold text-pink-700">"Software"</h2>
                        <ul class="flex flex-col gap-2">
                            {["Blender", "ZBrush", "3ds Max", "Adobe After Effects"]
                                .iter()
                                .map(|s| view! {
                                    <li class="flex gap-2 items-center text-gray-700">
                                        <span class="inline-block w-2 h-2 rounded-full bg-pink-400 shrink-0" />
                                        {*s}
                                    </li>
                                })
                                .collect::<Vec<_>>()}
                        </ul>
                    </div>

                </div>
            </section>
        </div>
    }
}

#[component]
fn Icon<'a>(
    src: &'a str,
    alt: &'a str,
    #[prop(default = "./about")] href: &'a str,
) -> impl IntoView {
    view! {
        <a href=href>
            <img
                class="w-6 h-6 transition hover:scale-110"
                src=src
                alt=alt
            />
        </a>
    }
}

// #[component]
// fn IconContainer<'a>(
//     #[prop(default = "bg-pink-300")]bg: &'a str
// ) -> impl IntoView {
//     let icon_container_classes = [
//         "flex size-10",
//         "rounded-md",
//         "border",
//         "items-center",
//         "justify-center",
//         bg,
//     ]
//     .join(" ");

//     view! {
//         <div class={icon_container_classes}></div>
//     }
// }

#[component]
fn PortfolioCell(
    content: impl IntoView,
    on_click: impl Fn(MouseEvent) + 'static,
) -> impl IntoView {
    view! {
        <div
            class="flex overflow-hidden justify-center items-center text-xl font-semibold text-center rounded-xl transition duration-300 cursor-pointer md:text-2xl lg:text-4xl hover:border-none hover:scale-110 focus:border-none focus:scale-110 size-80"
            on:click=on_click
        >
            {content}
        </div>
    }
}

#[component]
fn Title<'a>(
    title: &'a str,
    #[prop(optional)] color: &'a str,
) -> impl IntoView {
    view! {
        <p class="text-center font-bold text-4xl md:text-6xl "
            .to_owned() + { color }>{title}</p>
    }
}

#[component]
fn Button<'a>(
    href: &'a str,
    content: impl IntoView,
    #[prop(default = "bg-zinc-300")] bg: &'a str,
    #[prop(default = "hover:bg-zinc-200")] bg_hover: &'a str,
) -> impl IntoView {
    let btn_classes = [
        "text-black",
        "rounded-md",
        "border",
        "border-black",
        "text-center",
        "font-medium",
        "text-xl",
        "w-32",
        "py-2",
        "md:text-3xl",
        "md:w-48",
        "md:py-4",
        "hover:scale-110",
        "transition",
        bg,
        bg_hover,
    ]
    .join(" ");

    view! {
        <a href=href class=btn_classes>
            {content}
        </a>
    }
}

#[component]
fn SmallButton<'a>(
    href: &'a str,
    content: impl IntoView,
    #[prop(default = "")] width: &'a str,
    #[prop(default = "bg-black")] bg: &'a str,
    #[prop(default = "hover:bg-blue-300")] bg_hover: &'a str,
    #[prop(default = "hover:text-black")] text_hover: &'a str,
) -> impl IntoView {
    let btn_classes = [
        "text-white",
        "text-md",
        "text-center",
        "rounded-md",
        "border",
        "border-black",
        "py-2",
        "mt-0",
        "m-2",
        "hover:scale-110",
        "transition",
        width,
        bg,
        bg_hover,
        text_hover,
    ]
    .join(" ");

    view! {
        <a href=href class=btn_classes>
            {content}
        </a>
    }
}

// fn init_bevy_app(text_receiver: BevyEventReceiver<TextEvent>) -> App {
//     let mut app = App::new();
//     app.add_plugins(DefaultPlugins.set(WindowPlugin {
//         primary_window: Some(Window {
//             // "#bevy_canvas" is the default and can be
//             // changed in the <BevyCanvas> component
//             canvas: Some("#bevy_canvas".into()),
//             ..default()
//         }),
//         ..default()
//     }))
//     // import the event here into Bevy
//     .import_event_from_leptos(text_receiver)
//     .add_systems(Update, set_text);

//     app
// }

// pub fn set_text(mut event_reader: EventReader<TextEvent>) {
//     for event in event_reader.read() {
//         info!("{event:?}")
//         // do something with the event
//     }
// }

// #[derive(Event, Debug)]
// pub struct TextEvent {
//     pub text: String,
// }

#[derive(Debug, Deserialize)]
pub enum AssetNode {
    File(String),
    Directory {
        name: String,
        children: Vec<AssetNode>,
    },
}

/// Recursively find the directory from `paths`.
pub fn get_directory_node<'a>(
    paths: &'a [&str],
) -> Option<&'a [AssetNode]> {
    get_directory_node_recursive(paths, &ASSET_TREE)
}

/// Recursively find the directory from `paths`.
pub fn get_directory_node_recursive<'a>(
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

                return get_directory_node_recursive(
                    &paths[1..],
                    children,
                );
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
