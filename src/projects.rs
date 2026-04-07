use crate::components::Navbar;
use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-white">
            <Navbar />
            <div class="flex flex-col items-center px-10 py-12 md:px-20">
                <h1 class="mb-10 text-6xl font-bold text-center">"Projects"</h1>
                <div class="grid grid-cols-1 gap-8 w-full max-w-4xl md:grid-cols-2">
                    // Work card
                    <a
                        href="/work"
                        class="group relative overflow-hidden rounded-2xl shadow-md aspect-[4/3] transition duration-300 hover:shadow-2xl hover:scale-[1.03]"
                    >
                        <img
                            class="object-cover w-full h-full"
                            src="assets/images/Work-Projects/Christmas/Theme-Santa's-Dessert-Party/01.png"
                            alt="Work"
                        />
                        <div class="flex absolute inset-0 items-end p-8 bg-gradient-to-t from-black/70 to-transparent">
                            <h2 class="text-4xl font-bold text-white">"Work"</h2>
                        </div>
                        <div class="flex absolute inset-0 flex-col gap-3 justify-center items-center translate-y-full backdrop-blur-md bg-black/50 transition-transform duration-500 ease-out group-hover:translate-y-0">
                            <span class="text-4xl">"👆"</span>
                            <p class="text-xl font-bold text-white">"Click to view more"</p>
                        </div>
                    </a>

                    // Personal card
                    <a
                        href="/personal"
                        class="group relative overflow-hidden rounded-2xl shadow-md aspect-[4/3] transition duration-300 hover:shadow-2xl hover:scale-[1.03]"
                    >
                        <video
                            class="object-cover w-full h-full"
                            autoplay
                            muted
                            loop
                            playsinline
                            oncanplay="this.muted=true"
                        >
                            <source
                                src="assets/images/portfolio/3D-Anim-Char-Sushi Queen/showcase.mp4"
                                type="video/mp4"
                            />
                        </video>
                        <div class="flex absolute inset-0 items-end p-8 bg-gradient-to-t from-black/70 to-transparent">
                            <h2 class="text-4xl font-bold text-white">"Personal"</h2>
                        </div>
                        <div class="flex absolute inset-0 flex-col gap-3 justify-center items-center translate-y-full backdrop-blur-md bg-black/50 transition-transform duration-500 ease-out group-hover:translate-y-0">
                            <span class="text-4xl">"👆"</span>
                            <p class="text-xl font-bold text-white">"Click to view more"</p>
                        </div>
                    </a>
                </div>

                // Big orange banner below the cards
                <div
                    class="flex flex-col gap-4 justify-center items-center mt-12 w-full max-w-4xl rounded-3xl px-12 py-16 shadow-lg"
                    style="background-color: #F5AA45;"
                >
                    <h2 class="text-5xl font-black text-black text-center leading-tight">
                        "More work coming soon"
                    </h2>
                    <p class="text-lg text-black/70 text-center max-w-md">
                        "Stay tuned for new projects, characters, and designs."
                    </p>
                </div>
            </div>
        </div>
    }
}
