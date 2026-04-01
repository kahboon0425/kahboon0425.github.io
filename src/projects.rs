use crate::components::Navbar;
use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-white">
            <Navbar />
            <div class="flex flex-col items-center px-10 py-12 md:px-20">
                <h1 class="mb-10 text-6xl font-bold">"Projects"</h1>
                <div class="grid grid-cols-1 gap-8 w-full max-w-4xl md:grid-cols-2">
                    // Work card
                    <a
                        href="/work"
                        class="group relative overflow-hidden rounded-2xl shadow-md aspect-[4/3] transition duration-300 hover:shadow-2xl hover:scale-[1.03]"
                    >
                        <img
                            class="object-cover w-full h-full"
                            src="assets/images/portfolio/3D-Booth-Design 1/showcase.png"
                            alt="Work"
                        />
                        <div class="flex absolute inset-0 flex-col justify-between p-8 bg-gradient-to-t from-black/70 to-transparent transition duration-300 group-hover:from-black/80">
                            <div class="flex justify-end opacity-0 transition duration-300 group-hover:opacity-100">
                                <span class="flex gap-2 items-center px-4 py-2 text-sm font-semibold text-white rounded-full border border-white/60 backdrop-blur-sm bg-white/20">
                                    "Click to view →"
                                </span>
                            </div>
                            <h2 class="text-4xl font-bold text-white">"Work"</h2>
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
                        <div class="flex absolute inset-0 flex-col justify-between p-8 bg-gradient-to-t from-black/70 to-transparent transition duration-300 group-hover:from-black/80">
                            <div class="flex justify-end opacity-0 transition duration-300 group-hover:opacity-100">
                                <span class="flex gap-2 items-center px-4 py-2 text-sm font-semibold text-white rounded-full border border-white/60 backdrop-blur-sm bg-white/20">
                                    "Click to view →"
                                </span>
                            </div>
                            <h2 class="text-4xl font-bold text-white">"Personal"</h2>
                        </div>
                    </a>
                </div>
            </div>
        </div>
    }
}
