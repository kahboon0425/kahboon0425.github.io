use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// Shared sticky navigation bar used on every page.
#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <header class="flex sticky top-0 z-50 items-center justify-between px-10 py-5 bg-white border-b border-gray-200">
            <nav class="flex gap-10">
                <a href="/" class="text-xl font-medium transition hover:text-pink-400">"Home"</a>
                <a href="/about" class="text-xl font-medium transition hover:text-pink-400">
                    "About"
                </a>
                <a href="/projects" class="text-xl font-medium transition hover:text-pink-400">
                    "Projects"
                </a>
            </nav>
            <a
                href="/projects"
                class="text-3xl font-bold transition hover:scale-110 hover:text-pink-400"
            >
                "→"
            </a>
        </header>
    }
}

/// Clickable icon link.
#[component]
pub fn Icon<'a>(
    src: &'a str,
    alt: &'a str,
    #[prop(default = "/")] href: &'a str,
) -> impl IntoView {
    view! {
        <a href=href>
            <img class="w-6 h-6 transition hover:scale-110" src=src alt=alt />
        </a>
    }
}

/// Portfolio item cell with hover scale.
#[component]
pub fn PortfolioCell(
    content: impl IntoView,
    on_click: impl Fn(MouseEvent) + 'static,
) -> impl IntoView {
    view! {
        <div
            class="flex overflow-hidden justify-center items-center rounded-xl transition duration-300 cursor-pointer hover:scale-110 size-80"
            on:click=on_click
        >
            {content}
        </div>
    }
}

/// Reusable project category card with cover media + gradient title overlay.
#[component]
pub fn CategoryCard(
    name: &'static str,
    cover: &'static str,
    is_video: bool,
    #[prop(default = "aspect-[4/5]")] aspect: &'static str,
    on_click: impl Fn() + 'static,
) -> impl IntoView {
    let class = format!(
        "group relative overflow-hidden rounded-2xl shadow-md cursor-pointer {} transition duration-300 hover:shadow-2xl hover:scale-[1.03]",
        aspect
    );
    view! {
        <div class=class on:click=move |_| on_click()>
            {if is_video {
                view! {
                    <video
                        class="object-cover w-full h-full"
                        autoplay
                        muted
                        loop
                        playsinline
                        oncanplay="this.muted=true"
                    >
                        <source src=cover type="video/mp4" />
                    </video>
                }
                .into_any()
            } else {
                view! { <img class="object-cover w-full h-full" src=cover alt=name /> }.into_any()
            }}
            <div class="flex absolute inset-0 flex-col justify-between p-6 bg-gradient-to-t from-black/70 to-transparent transition duration-300 group-hover:from-black/80">
                // "Click to view" hint — visible only on hover
                <div class="flex justify-end opacity-0 transition duration-300 group-hover:opacity-100">
                    <span class="flex gap-2 items-center px-4 py-2 text-sm font-semibold text-white rounded-full border border-white/60 backdrop-blur-sm bg-white/20">
                        "Click to view →"
                    </span>
                </div>
                <h2 class="text-3xl font-bold text-white">{name}</h2>
            </div>
        </div>
    }
}
