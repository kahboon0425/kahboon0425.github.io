use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
// use leptos_use::{
//     UseMouseCoordType, UseMouseEventExtractor, UseMouseOptions, UseMouseReturn,
//     use_mouse_with_options,
// };
// use bevy::prelude::*;
// use leptos_bevy_canvas::prelude::*;

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
        <div class="w-full flex-wrap flex md:justify-between justify-center">

        <SmallButton href="./" content="BACK" width="w-30" bg_hover="hover:bg-pink-300"/>
        <SmallButton href="./portfolio" content="Visit My Portfolio" width="w-50"/>

        </div>

        <h1 class="text-7xl mt-5 mb-12">"About"</h1>


         <div class="flex-wrap flex justify-center gap-15">
                <div class="md:w-lg p-8 border-2 border-black rounded-md bg-gray-50">
                    <p>
                        "Hi, I’m Kahboon. I graduated with an IT background but have a strong interest in 3D art. Over the past few months, I’ve been self-learning 3D modeling using Blender. I enjoy creating cute and stylized characters. I have also been learning character rigging and animation to bring my characters to life. In addition to that, I have also been exploring CGI, VFX, and 3D design for booths and pop-up stores."
                    </p>
                    <br/>
                    <p>
                        "I’m currently looking for a junior 3D artist position or a 3D booth designer role. Feel free to check out my portfolio. Thanks for visiting!"
                    </p>
                </div>

                <div class="w-xs p-6 border-2 border-black rounded-md bg-gray-50">
                    <div class="flex flex-wrap gap-4 mb-4 items-center">
                        <div class="flex size-10 bg-pink-300 rounded-md border items-center justify-center">
                        // email
                        <svg
                        class="h-6 w-6"
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 512 512">
                        <path d="M48 64C21.5 64 0 85.5 0 112c0 15.1 7.1 29.3 19.2 38.4L236.8 313.6c11.4 8.5 27 8.5 38.4 0L492.8 150.4c12.1-9.1 19.2-23.3 19.2-38.4c0-26.5-21.5-48-48-48L48 64zM0 176L0 384c0 35.3 28.7 64 64 64l384 0c35.3 0 64-28.7 64-64l0-208L294.4 339.2c-22.8 17.1-54 17.1-76.8 0L0 176z"/>
                        </svg>
                        </div>
                        <p>"changkahboon25@gmail.com"</p>
                    </div>

                    <div class="flex flex-wrap gap-4 mb-4 items-center">
                        <div class="w-10 h-10 bg-pink-300 rounded-md border"></div>
                        <p>"the_sushi_queen_art"</p>
                    </div>

                    <div class="flex flex-wrap gap-4 mb-4 items-center">
                        <div class="w-10 h-10 bg-pink-300 rounded-md border"></div>
                        <p>"012-7645817"</p>
                    </div>

                </div>
            </div>
        </main>
    }
}

#[component]
fn Portfolio() -> impl IntoView {
    view! {
        <main class="h-screen flex flex-col items-center justify-center bg-white">
            <Title title="Portfolio" />
            <div class="py-8" />
            <div class="flex flex-row flex-wrap gap-8 items-center justify-center">
                <PortfolioCell label="Character Design" />
                <PortfolioCell label="CGI/VFX" />
                <PortfolioCell label="Event Booth Design" />
                <PortfolioCell label="3D Modeling" />
                <PortfolioCell label="2D Art" />
            </div>
        </main>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
    <main class="h-screen flex flex-col items-center justify-center bg-white">
        <Title title="Hi, I am Kah Boon" />
        <div class="flex flex-wrap gap-4 my-4 justify-center md:gap-6">
            <Button href="./about" content="About" bg="bg-pink-300" bg_hover="hover:bg-pink-200" />
            <Button href="./portfolio" content="Portfolio" bg="bg-sky-300" bg_hover="hover:bg-sky-200" />
        </div>


        <div class="mt-5 w-md flex flex-row  items-center justify-center gap-4">
          <div class="">
            // email
            <img class="h-6 w-6" src="assets/svg/envelope-solid.svg" alt="Email SVG"/>
          </div>
          <div class="">
            // insta
            <img class="h-6 w-6" src="assets/svg/instagram-brands.svg" alt="Instagram SVG"/>
          </div>
          <div class="">
            // phone
            <img class="h-6 w-6" src="assets/svg/whatsapp-brands.svg" alt="Whatsapp SVG"/>
          </div>
          <div class="">
            //linkedin
            <img class="h-6 w-6" src="assets/svg/linkedin-brands.svg" alt="Linkedin SVG"/>
          </div>
          <div class="">
            //Github
            <img class="h-6 w-6" src="assets/svg/github-brands.svg" alt="Github SVG"/>
          </div>
        </div>
    </main>
        }
}

#[component]
fn PortfolioCell<'a>(label: &'a str) -> impl IntoView {
    view! {
        <a href="" class="
            flex
            items-center
            justify-center
            bg-blue-200
            p-6
            font-semibold
            text-center
            text-xl
            rounded-4xl
            size-40
            md:size-60
            lg:size-80
            md:text-2xl
            lg:text-4xl
            "
        >
            {label}
        </a>
    }
}

#[component]
fn Title<'a>(title: &'a str) -> impl IntoView {
    view! {
        <h1 class="m-6 text-center font-bold text-4xl md:text-6xl">{title}</h1>
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
        "animate",
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
