use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use leptos_use::{
    UseMouseCoordType, UseMouseEventExtractor, UseMouseOptions, UseMouseReturn,
    use_mouse_with_options,
};
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

        <main class="h-screen flex flex-col items-center bg-white">
        <div class="w-full flex-wrap flex justify-between m-10">

        <a href="./"
            class="
                w-30
                bg-black
                hover:bg-pink-300
                hover:text-black
                text-white
                text-md
                text-center
                rounded-md
                py-2
                ml-8
                "
        >
            "BACK"
        </a>

        <a href="./my-portfolio"
            class="
                w-50
                bg-black
                hover:bg-blue-300
                hover:text-black
                text-white
                text-md
                text-center
                rounded-md
                py-2
                mr-8
                "
        >
            "Visit My Portfolio"
        </a>
        </div>

        <h1 class="text-7xl mb-12">"About"</h1>


         <div class="flex-wrap flex justify-center gap-15">
                <div class="w-lg p-8 border-2 border-black rounded-md bg-gray-50">
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
                        <div class="w-10 h-10 bg-pink-300 rounded-md border"></div>
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
            <div class="flex flex-wrap gap-4 my-4 justify-center md:gap-6">
                <div class="snap-x ...">
                    <div class="snap-center ...">
                        <img src="/img/vacation-01.jpg" />
                    </div>
                    <div class="snap-center ...">
                        <img src="/img/vacation-02.jpg" />
                    </div>
                    <div class="snap-center ...">
                        <img src="/img/vacation-03.jpg" />
                    </div>
                    <div class="snap-center ...">
                        <img src="/img/vacation-04.jpg" />
                    </div>
                    <div class="snap-center ...">
                        <img src="/img/vacation-05.jpg" />
                    </div>
                    <div class="snap-center ...">
                        <img src="/img/vacation-06.jpg" />
                    </div>
                </div>
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
                <Button href="./about" content="About" bg="bg-pink-200" bg_hover="hover:bg-pink-300" />
                <Button href="./portfolio" content="Portfolio" bg="bg-sky-200" bg_hover="hover:bg-sky-300" />
            </div>
        </main>
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
    #[prop(default = "bg-zinc-200")] bg: &'a str,
    #[prop(default = "hover:bg-zinc-300")] bg_hover: &'a str,
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
