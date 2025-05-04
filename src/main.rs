use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
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
                <Routes fallback=|| "Not found.">
                        <Route path=path!("/") view=Home/>
                        <Route path=path!("/about") view=About/>
                        <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {}
}

#[component]
fn Home() -> impl IntoView {
    let btn_classes = [
        "text-black",
        "rounded-md",
        "border",
        "border-black",
        "text-center",
        "text-xl",
        "w-32",
        "py-2",
        "md:text-3xl",
        "md:w-48",
        "md:py-4",
    ]
    .join(" ")
        + " ";

    let about_btn_classes = btn_classes.clone() + &["bg-pink-200", "hover:bg-pink-300"].join(" ");
    let port_btn_classes = btn_classes + &["bg-blue-200", "hover:bg-blue-300"].join(" ");

    view! {
        <main class="h-screen flex flex-col items-center justify-center bg-white">
            <h1 class="m-6 text-center text-4xl md:text-6xl">"Hi, I am Kah Boon"</h1>
            <div class="flex flex-wrap gap-4 my-4 justify-center md:gap-6">
                <a href="./about" class={about_btn_classes}>
                    "About"
                </a>
                <a href="./portfolio" class={port_btn_classes}>
                    "Portfolio"
                </a>
            </div>
        </main>
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
