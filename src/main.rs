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
        <Router base="/portfolio">
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
        <main class="flex flex-col items-center p-10 h-screen bg-white">
            <div class="flex justify-between w-full">

                // <SmallButton href="./" content="BACK" width="w-30" bg_hover="hover:bg-pink-300"/>
                <Icon
                    href="./"
                    src="assets/svg/arrow-left-solid.svg"
                    alt="Back Arrow SVG"
                />
                <SmallButton
                    href="./portfolio"
                    content="Visit My Portfolio"
                    width="w-50"
                />

            </div>

            <h1 class="mt-5 mb-8 text-7xl">"About"</h1>

            <div class="flex flex-col justify-center p-10 pt-0">
                <div class="self-center py-2 animate-bounce xl:self-end">
                    <a href="https://drive.google.com/file/d/1n6NcOnok-7g97gj7ykd_XPxBP4CSDYW1/view?usp=sharing">
                        <p class="underline transition cursor-pointer hover:text-blue-300 hover:scale-110">
                            View Resume
                        </p>
                    </a>
                </div>

                <div class="flex flex-row flex-wrap justify-center gap-15">
                    <div class="p-8 bg-gray-50 rounded-md border-2 border-black md:w-lg">
                        <p>
                            "Hi, I’m Kahboon. I graduated with an IT background but have a strong interest in 3D art. Over the past few months, I’ve been self-learning 3D modeling using Blender. I enjoy creating cute and stylized characters. I have also been learning character rigging and animation to bring my characters to life. In addition to that, I have also been exploring CGI, VFX, and 3D design for booths and pop-up stores."
                        </p>
                        <br />
                        <p>
                            "I’m currently looking for a junior 3D artist position or a 3D booth designer role. Feel free to check out my portfolio. Thanks for visiting!"
                        </p>
                    </div>

                    <div class="p-6 bg-gray-50 rounded-md border-2 border-black w-sm">
                        <div class="flex flex-wrap gap-4 items-center mb-4">
                            <div class="flex justify-center items-center bg-pink-300 rounded-md border size-10">

                                <Icon
                                    href="mailto:changkahboon25@gmail.com"
                                    src="assets/svg/envelope-solid.svg"
                                    alt="Email SVG"
                                />

                            </div>
                            <p>"changkahboon25@gmail.com"</p>
                        </div>

                        <div class="flex flex-wrap gap-4 items-center mb-4">
                            <div class="flex justify-center items-center bg-blue-300 rounded-md border size-10">

                                <Icon
                                    href="https://www.instagram.com/the_sushi_queen_art/"
                                    src="assets/svg/instagram-brands.svg"
                                    alt="Instagram SVG"
                                />

                            </div>
                            <p>"the_sushi_queen_art"</p>
                        </div>

                        <div class="flex flex-wrap gap-4 items-center mb-4">
                            <div class="flex justify-center items-center bg-pink-300 rounded-md border size-10">

                                <Icon
                                    src="assets/svg/whatsapp-brands.svg"
                                    alt="Whatsapp SVG"
                                />

                            </div>
                            <p>"012-7645817"</p>
                        </div>

                        <div class="flex flex-wrap gap-4 items-center mb-4">
                            <div class="flex justify-center items-center bg-blue-300 rounded-md border size-10">

                                <Icon
                                    href="https://www.linkedin.com/in/kahboon"
                                    src="assets/svg/linkedin-brands.svg"
                                    alt="Linkedin SVG"
                                />

                            </div>
                            <p>"https://www.linkedin.com/in/kahboon"</p>
                        </div>

                        <div class="flex flex-wrap gap-4 items-center mb-4">
                            <div class="flex justify-center items-center bg-pink-300 rounded-md border size-10">

                                <Icon
                                    href="https://github.com/kahboon0425"
                                    src="assets/svg/github-brands.svg"
                                    alt="Github SVG"
                                />

                            </div>
                            <p>"https://github.com/kahboon0425"</p>
                        </div>

                    </div>
                </div>
            </div>
        </main>
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
            <Icon
                href="./"
                src="assets/svg/arrow-left-solid.svg"
                alt="Back Arrow SVG"
            />

            <Title title="Portfolio" />

            <div class="flex justify-center mt-5 border-b border-gray-200">
                <nav class="flex gap-8">

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
                                                        class="transition duration-300 md:opacity-50 hover:opacity-100 active:opacity-100 big-img"
                                                        loop
                                                        autoplay
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
                                                        class="transition duration-300 md:opacity-50 hover:opacity-100 active:opacity-100 big-img"
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
        <main class="flex flex-col justify-center items-center h-screen bg-white">
            <img
                class="size-30 md:size-50"
                src="assets/images/Sushi Queen Logo Transparent.png"
            />
            // <img class="size-30 md:size-50" src="assets/images/Sushi Queen Logo Transparent.png" />
            <Title title="Hi, I am Kah Boon" />
            <div class="flex flex-wrap gap-4 justify-center my-4 md:gap-6">
                <Button
                    href="./about"
                    content="About"
                    bg="bg-pink-300"
                    bg_hover="hover:bg-pink-200"
                />
                <Button
                    href="./portfolio"
                    content="Portfolio"
                    bg="bg-sky-300"
                    bg_hover="hover:bg-sky-200"
                />
            </div>

            <div class="flex flex-row gap-4 justify-center items-center mt-5 w-md">
                <div class="">
                    // email
                    <Icon
                        href="mailto:changkahboon25@gmail.com"
                        src="assets/svg/envelope-solid.svg"
                        alt="Email SVG"
                    />
                </div>
                <div class="">
                    // insta
                    <Icon
                        href="https://www.instagram.com/the_sushi_queen_art/"
                        src="assets/svg/instagram-brands.svg"
                        alt="Instagram SVG"
                    />
                </div>
                <div class="">
                    // linkedin
                    <Icon
                        href="https://www.linkedin.com/in/kahboon"
                        src="assets/svg/linkedin-brands.svg"
                        alt="Linkedin SVG"
                    />
                </div>
                <div class="">
                    // Github
                    <Icon
                        href="https://github.com/kahboon0425"
                        src="assets/svg/github-brands.svg"
                        alt="Github SVG"
                    />
                </div>
            </div>
        </main>
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
            class="flex overflow-hidden justify-center items-center text-xl font-semibold text-center bg-gray-300 rounded-xl transition duration-300 cursor-pointer md:text-2xl lg:text-4xl hover:border-none hover:scale-110 focus:border-none focus:scale-110 size-60"
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
