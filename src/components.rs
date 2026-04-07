use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_router::hooks::use_location;

/// Shared sticky navigation bar used on every page.
#[component]
pub fn Navbar() -> impl IntoView {
    let location = use_location();

    let nav_class = move |path: &'static str| {
        let current = location.pathname.get();
        if current == path {
            "text-xl font-semibold pb-1 transition text-[#F5AA45] border-b-2 border-[#F5AA45]"
        } else {
            "text-xl font-semibold pb-1 transition text-black hover:text-[#F5AA45]"
        }
    };

    view! {
        <header class="sticky top-0 z-50 px-10 py-5 bg-white">
            <nav class="flex gap-12 justify-center">
                <a href="/" class=move || nav_class("/")>"Home"</a>
                <a href="/about" class=move || nav_class("/about")>"About"</a>
                <a href="/projects" class=move || nav_class("/projects")>"Projects"</a>
            </nav>
        </header>
    }
}

/// Hamburger menu for inner pages (Work / Personal) — shown in top-right corner.
#[component]
pub fn HamburgerMenu() -> impl IntoView {
    let (open, set_open) = signal(false);
    view! {
        <div class="fixed top-10 right-8 z-50">
            // Toggle button
            <button
                class="flex flex-col gap-2 justify-center items-center p-3 w-14 h-14 rounded-xl border-2 border-blue-200 bg-white shadow-md transition hover:bg-blue-50 hover:border-blue-400"
                on:click=move |_| set_open.update(|v| *v = !*v)
            >
                <span class="block w-7 h-0.5 bg-blue-500"></span>
                <span class="block w-7 h-0.5 bg-blue-500"></span>
                <span class="block w-7 h-0.5 bg-blue-500"></span>
            </button>
            // Dropdown menu
            {move || open.get().then(|| view! {
                <div class="absolute right-0 mt-2 w-48 rounded-xl border border-gray-200 bg-white shadow-xl overflow-hidden">
                    <a href="/" class="block px-6 py-4 text-base font-medium transition hover:bg-pink-50 hover:text-pink-500">"Home"</a>
                    <a href="/about" class="block px-6 py-4 text-base font-medium transition hover:bg-pink-50 hover:text-pink-500">"About"</a>
                    <a href="/projects" class="block px-6 py-4 text-base font-medium transition hover:bg-pink-50 hover:text-pink-500">"Projects"</a>
                </div>
            })}
        </div>
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
    let wrapper_class = format!("group relative cursor-pointer p-2 {}", aspect);
    view! {
        <div class=wrapper_class on:click=move |_| on_click()>
            // Shadow layer — slight rotation
            <div class="absolute inset-2 rounded-2xl bg-gray-300 shadow transition-all duration-300 ease-out rotate-[3deg] group-hover:rotate-[4deg]"></div>
            // Top card
            <div class="overflow-hidden absolute inset-2 rounded-2xl shadow-lg transition-all duration-300 ease-out group-hover:-translate-y-1 group-hover:shadow-2xl">
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
                // Static gradient + title (always visible)
                <div class="flex absolute inset-0 items-end p-6 bg-gradient-to-t from-black/70 to-transparent">
                    <h2 class="text-3xl font-bold text-white">{name}</h2>
                </div>
                // Slide-up blur overlay on hover
                <div class="flex absolute inset-0 flex-col gap-3 justify-center items-center translate-y-full backdrop-blur-md bg-black/50 transition-transform duration-500 ease-out group-hover:translate-y-0">
                    <p class="text-xl font-bold text-white">"Click to view more"</p>
                </div>
            </div>
        </div>
    }
}
