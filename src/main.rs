use leptos::ev::MouseEvent;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
// use leptos_use::{
//     UseMouseCoordType, UseMouseEventExtractor, UseMouseOptions, UseMouseReturn,
//     use_mouse_with_options,
// };
// use bevy::prelude::*;
// use leptos_bevy_canvas::prelude::*;

mod portfolio;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router base="/portfolio">
            <main>
                <Routes fallback=|| view! { <h1>"Not Found"</h1> }>
                        <Route path=path!("/") view=Home/>
                        <Route path=path!("/about") view=About/>
                        <Route path=path!("/portfolio") view=Portfolio/>
                        // <Route path=path!("/portfolio/character") view=portfolio::Character/>
                        <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <main class="h-screen flex flex-col items-center bg-white p-10">
        <div class="w-full flex justify-between">

        // <SmallButton href="./" content="BACK" width="w-30" bg_hover="hover:bg-pink-300"/>
        <Icon href="./" src="assets/svg/arrow-left-solid.svg" alt="Back Arrow SVG"/>
        <SmallButton href="./portfolio" content="Visit My Portfolio" width="w-50"/>

        </div>

        <h1 class="text-7xl mt-5 mb-8">"About"</h1>

       <div class=" flex w-full xl:px-32 py-2 justify-center xl:justify-end animate-bounce">
            <a href="https://drive.google.com/file/d/1n6NcOnok-7g97gj7ykd_XPxBP4CSDYW1/view?usp=sharing"><p class="underline hover:text-blue-300 hover:scale-110 cursor-pointer">View Resume</p></a>
        </div>

        // <div class="border-2 border-black"><p>Open Resume</p></div>
         <div class="flex-wrap flex justify-center gap-15 p-10 pt-0">
                <div class="md:w-lg p-8 border-2 border-black rounded-md bg-gray-50">
                    <p>
                        "Hi, I’m Kahboon. I graduated with an IT background but have a strong interest in 3D art. Over the past few months, I’ve been self-learning 3D modeling using Blender. I enjoy creating cute and stylized characters. I have also been learning character rigging and animation to bring my characters to life. In addition to that, I have also been exploring CGI, VFX, and 3D design for booths and pop-up stores."
                    </p>
                    <br/>
                    <p>
                        "I’m currently looking for a junior 3D artist position or a 3D booth designer role. Feel free to check out my portfolio. Thanks for visiting!"
                    </p>
                </div>

                <div class="w-sm p-6 border-2 border-black rounded-md bg-gray-50">
                    <div class="flex flex-wrap gap-4 mb-4 items-center">
                        <div class="flex size-10 bg-pink-300 rounded-md border items-center justify-center">

                        <Icon href="mailto:changkahboon25@gmail.com" src="assets/svg/envelope-solid.svg" alt="Email SVG"/>

                        </div>
                        <p>"changkahboon25@gmail.com"</p>
                    </div>

                    <div class="flex flex-wrap gap-4 mb-4 items-center">
                        <div class="flex size-10 bg-blue-300 rounded-md border items-center justify-center">

                        <Icon href="https://www.instagram.com/the_sushi_queen_art/" src="assets/svg/instagram-brands.svg" alt="Instagram SVG"/>

                        </div>
                        <p>"the_sushi_queen_art"</p>
                    </div>

                    <div class="flex flex-wrap gap-4 mb-4 items-center">
                        <div class="flex size-10 bg-pink-300 rounded-md border items-center justify-center">

                        <Icon src="assets/svg/whatsapp-brands.svg" alt="Whatsapp SVG"/>

                        </div>
                        <p>"012-7645817"</p>
                    </div>

                    <div class="flex flex-wrap gap-4 mb-4 items-center">
                        <div class="flex size-10 bg-blue-300 rounded-md border items-center justify-center">

                        <Icon href="https://www.linkedin.com/in/kahboon" src="assets/svg/linkedin-brands.svg" alt="Linkedin SVG"/>

                        </div>
                        <p>"https://www.linkedin.com/in/kahboon"</p>
                    </div>

                    <div class="flex flex-wrap gap-4 mb-4 items-center">
                        <div class="flex size-10 bg-pink-300 rounded-md border items-center justify-center">

                        <Icon href="https://github.com/kahboon0425" src="assets/svg/github-brands.svg" alt="Github SVG"/>

                        </div>
                        <p>"https://github.com/kahboon0425"</p>
                    </div>

                </div>
            </div>
        </main>
    }
}

#[component]
fn Portfolio() -> impl IntoView {
    const FOLDERS: &[&str] = &[
        "Booth Design 1",
        "Booth Design 2",
        "Booth Design 3",
        "Booth Design 4",
        "Cat Dropping",
        "Cat in Box",
        "CGI-Balls",
        "CGI-Gift-Box",
        "CGI-headphone",
        "CGI-Sushi-Queen",
        "Donut",
        "Factory",
        "Gatcha Machine",
        "Milk",
        "Profile Image",
        "Room",
        "Sushi Character Prawn",
        "Sushi Character Salmon",
        "Sushi Queen",
        "Sushi Queen Typing",
        "Tin",
        "Tree",
        "Vendor Machine",
    ];

    view! {
        <main class="bg-white h-full px-10 py-10">

        <Icon href="./" src="assets/svg/arrow-left-solid.svg" alt="Back Arrow SVG"/>

            <Title title="Portfolio" />

            <div class="flex justify-center border-b border-gray-200 mt-5">
              <nav class="flex gap-8">
                <a href="#" class="text-gray-500 hover:text-gray-700 px-3 py-2 text-sm font-medium active:border-blue-600 active:border-b-2">
                  Character Design
                </a>
                <a href="#" class="text-gray-500 hover:text-gray-700 active:border-blue-600 active:border-b-2 px-3 py-2 text-sm font-medium active:border-blue-600 active:border-b-2">
                  CGI
                </a>
                <a href="#" class="text-gray-500 hover:text-gray-700 px-3 py-2 text-sm font-medium active:border-blue-600 active:border-b-2">
                  Animation
                </a>
                <a href="#" class="text-gray-500 hover:text-gray-700 px-3 py-2 text-sm font-medium active:border-blue-600 active:border-b-2">
                  3D Booth Design
                </a>
                <a href="#" class="text-gray-500 hover:text-gray-700 px-3 py-2 text-sm font-medium active:border-blue-600 active:border-b-2">
                  2D Art
                </a>
              </nav>
            </div>

            <div class="items-center justify-center py-10" />
            <div class="flex flex-row flex-wrap gap-8 h-full items-center justify-center">
                {
                    FOLDERS.iter().map(|f| view!{
                        <PortfolioCell content={view! {
                            <img class="
                                big-img
                                transition
                                duration-300
                                md:opacity-50
                                hover:opacity-100
                                active:opacity-100
                            "
                                src="assets/images/portfolio/".to_string() + f + "/showcase.png"
                            />
                        }} on_click=move |m| {
                            console_log(&("assets/images/portfolio/".to_string() + f));
                            console_log(&format!("{m:?}"));
                        }/>
                    }).collect::<Vec<_>>()
                }
            </div>
        </main>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
    <main class="h-screen flex flex-col items-center justify-center bg-white">
        <img class="size-30 md:size-50" src="assets/images/Sushi Queen Logo Transparent.png" />
        // <img class="size-30 md:size-50" src="assets/images/Sushi Queen Logo Transparent.png" />
        <Title title="Hi, I am Kah Boon" />
        <div class="flex flex-wrap gap-4 my-4 justify-center md:gap-6">
            <Button href="./about" content="About" bg="bg-pink-300" bg_hover="hover:bg-pink-200" />
            <Button href="./portfolio" content="Portfolio" bg="bg-sky-300" bg_hover="hover:bg-sky-200" />
        </div>


        <div class="mt-5 w-md flex flex-row  items-center justify-center gap-4">
          <div class="">
            // email
            <Icon href="mailto:changkahboon25@gmail.com" src="assets/svg/envelope-solid.svg" alt="Email SVG"/>
          </div>
          <div class="">
            // insta
            <Icon href="https://www.instagram.com/the_sushi_queen_art/" src="assets/svg/instagram-brands.svg" alt="Instagram SVG"/> </div>
          <div class="">
            //linkedin
            <Icon href="https://www.linkedin.com/in/kahboon" src="assets/svg/linkedin-brands.svg" alt="Linkedin SVG"/>
          </div>
          <div class="">
            //Github
            <Icon href="https://github.com/kahboon0425" src="assets/svg/github-brands.svg" alt="Github SVG"/>
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
        <a href={href}>
            <img class="hover:scale-110 transition h-6 w-6"
                src={src} alt={alt}/>
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
fn PortfolioCell(content: impl IntoView, on_click: impl Fn(MouseEvent) + 'static) -> impl IntoView {
    view! {
        <div class="
            cursor-pointer
            flex
            items-center
            justify-center
            bg-gray-300
            font-semibold
            rounded-xl
            overflow-hidden
            text-center
            text-xl
            size-40
            md:size-60
            md:text-2xl
            lg:text-4xl
            transition
            duration-300
            hover:scale-110
            hover:border-none
            focus:scale-110
            focus:border-none
            "
            on:click={on_click}
        >
            {content}
        </div>
    }
}

#[component]
fn Title<'a>(title: &'a str, #[prop(optional)] color: &'a str) -> impl IntoView {
    view! {
        <p class="text-center font-bold text-4xl md:text-6xl ".to_owned() + {color}>{title}</p>
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
        <a href={href} class={btn_classes}>{content}</a>
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
        <a href={href} class={btn_classes}>{content}</a>
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
