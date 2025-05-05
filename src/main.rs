use leptos::ev::MouseEvent;
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
        <main class="h-screen flex flex-col items-center bg-white">
        <div class="w-full flex-wrap flex justify-between m-10">

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
        <main class="bg-white h-full px-10 py-20">
            <Title title="Portfolio" />
            <div class="items-center justify-center py-10" />
            <div class="flex flex-row flex-wrap gap-8 h-full items-center justify-center">
                <PortfolioCell content={view! {
                    <img src="assets/images/portfolio/Booth Design 1/showcase.png" class="big-img"/>
                }} on_click=move |m| {}/>
                // <PortfolioCell content="CGI/VFX" on_click=move |m| {}/>
                // <PortfolioCell content="Event Booth Design" on_click=move |m| {}/>
                // <PortfolioCell content="3D Modeling" on_click=move |m| {}/>
                // <PortfolioCell content="2D Art" on_click=move |m| {}/>
            </div>
        </main>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
    <main class="h-screen flex flex-col items-center justify-center bg-white">
        <img class="size-40 md:size-60" src="assets/images/Sushi Queen Logo Transparent.png" />
        <Title title="Hi, I am Kah Boon" />
        <div class="flex flex-wrap gap-4 my-4 justify-center md:gap-6">
            <Button href="./about" content="About" bg="bg-pink-300" bg_hover="hover:bg-pink-200" />
            <Button href="./portfolio" content="Portfolio" bg="bg-sky-300" bg_hover="hover:bg-sky-200" />
        </div>


        <div class="mt-5 w-md flex flex-row  items-center justify-center gap-4">
          <div class="">
            // email
            <svg
            class="h-6 w-6"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 512 512">
            <path d="M48 64C21.5 64 0 85.5 0 112c0 15.1 7.1 29.3 19.2 38.4L236.8 313.6c11.4 8.5 27 8.5 38.4 0L492.8 150.4c12.1-9.1 19.2-23.3 19.2-38.4c0-26.5-21.5-48-48-48L48 64zM0 176L0 384c0 35.3 28.7 64 64 64l384 0c35.3 0 64-28.7 64-64l0-208L294.4 339.2c-22.8 17.1-54 17.1-76.8 0L0 176z"/>
            </svg>
          </div>
          <div class="">
            // insta
            <svg
            class="h-6 w-6"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 448 512">
            <path d="M224.1 141c-63.6 0-114.9 51.3-114.9 114.9s51.3 114.9 114.9 114.9S339 319.5 339 255.9 287.7 141 224.1 141zm0 189.6c-41.1 0-74.7-33.5-74.7-74.7s33.5-74.7 74.7-74.7 74.7 33.5 74.7 74.7-33.6 74.7-74.7 74.7zm146.4-194.3c0 14.9-12 26.8-26.8 26.8-14.9 0-26.8-12-26.8-26.8s12-26.8 26.8-26.8 26.8 12 26.8 26.8zm76.1 27.2c-1.7-35.9-9.9-67.7-36.2-93.9-26.2-26.2-58-34.4-93.9-36.2-37-2.1-147.9-2.1-184.9 0-35.8 1.7-67.6 9.9-93.9 36.1s-34.4 58-36.2 93.9c-2.1 37-2.1 147.9 0 184.9 1.7 35.9 9.9 67.7 36.2 93.9s58 34.4 93.9 36.2c37 2.1 147.9 2.1 184.9 0 35.9-1.7 67.7-9.9 93.9-36.2 26.2-26.2 34.4-58 36.2-93.9 2.1-37 2.1-147.8 0-184.8zM398.8 388c-7.8 19.6-22.9 34.7-42.6 42.6-29.5 11.7-99.5 9-132.1 9s-102.7 2.6-132.1-9c-19.6-7.8-34.7-22.9-42.6-42.6-11.7-29.5-9-99.5-9-132.1s-2.6-102.7 9-132.1c7.8-19.6 22.9-34.7 42.6-42.6 29.5-11.7 99.5-9 132.1-9s102.7-2.6 132.1 9c19.6 7.8 34.7 22.9 42.6 42.6 11.7 29.5 9 99.5 9 132.1s2.7 102.7-9 132.1z"/>
            </svg>
          </div>
          <div class="">
            // phone
            <svg
            class="h-6 w-6"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 512 512">
            <path d="M164.9 24.6c-7.7-18.6-28-28.5-47.4-23.2l-88 24C12.1 30.2 0 46 0 64C0 311.4 200.6 512 448 512c18 0 33.8-12.1 38.6-29.5l24-88c5.3-19.4-4.6-39.7-23.2-47.4l-96-40c-16.3-6.8-35.2-2.1-46.3 11.6L304.7 368C234.3 334.7 177.3 277.7 144 207.3L193.3 167c13.7-11.2 18.4-30 11.6-46.3l-40-96z"/>
            </svg>
          </div>
          <div class="">
            //linkedin
            <svg
            class="h-6 w-6"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 448 512">
            <path d="M416 32H31.9C14.3 32 0 46.5 0 64.3v383.4C0 465.5 14.3 480 31.9 480H416c17.6 0 32-14.5 32-32.3V64.3c0-17.8-14.4-32.3-32-32.3zM135.4 416H69V202.2h66.5V416zm-33.2-243c-21.3 0-38.5-17.3-38.5-38.5S80.9 96 102.2 96c21.2 0 38.5 17.3 38.5 38.5 0 21.3-17.2 38.5-38.5 38.5zm282.1 243h-66.4V312c0-24.8-.5-56.7-34.5-56.7-34.6 0-39.9 27-39.9 54.9V416h-66.4V202.2h63.7v29.2h.9c8.9-16.8 30.6-34.5 62.9-34.5 67.2 0 79.7 44.3 79.7 101.9V416z"/>
            </svg>
          </div>
        </div>
    </main>
        }
}

#[component]
fn PortfolioCell(content: impl IntoView, on_click: fn(MouseEvent)) -> impl IntoView {
    view! {
        <div class="
            flex
            items-center
            justify-center
            bg-blue-200
            font-semibold
            rounded-xl
            border-2
            overflow-hidden
            text-center
            text-xl
            size-40
            md:size-60
            md:text-2xl
            lg:text-4xl
            transition
            hover:scale-110
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
