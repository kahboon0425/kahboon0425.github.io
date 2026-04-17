use crate::components::Navbar;
use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-100">
            <Navbar />
            <div class="flex flex-col items-center px-8 pt-18 pb-25 md:px-16">
                <h1 class="mb-12 text-6xl font-bold text-center">"Projects"</h1>

                <div class="grid grid-cols-1 gap-10 w-full max-w-5xl md:grid-cols-2">

                        // Work card
                        <a
                            href="/work"
                            class="group flex flex-col overflow-hidden rounded-2xl bg-white shadow-md transition duration-300 hover:shadow-2xl hover:scale-[1.03]"
                        >
                            // Image top
                            <div class="overflow-hidden h-72">
                                <img
                                    class="object-cover w-full h-full transition duration-500 group-hover:scale-110"
                                    src="assets/images/Work-Projects/Christmas/Theme-Golden-Christmas-Funfair/2.png"
                                    alt="Work"
                                />
                            </div>
                            // Text bottom
                            <div class="flex flex-col gap-2 p-6">
                                <h2 class="text-3xl font-bold text-black">"Work"</h2>
                                <p class="text-base text-gray-500">"3D Event Design."</p>
                                <div class="flex justify-end mt-2">
                                    <span
                                        class="flex justify-center items-center w-9 h-9 rounded-full font-bold text-black transition group-hover:scale-110"
                                        style="background-color: #fdbf3a;"
                                    >"→"</span>
                                </div>
                            </div>
                        </a>

                        // Personal card
                        <a
                            href="/personal"
                            class="group flex flex-col overflow-hidden rounded-2xl bg-white shadow-md transition duration-300 hover:shadow-2xl hover:scale-[1.03]"
                        >
                            // Video top
                            <div class="overflow-hidden h-72">
                                <video
                                    class="object-cover w-full h-full transition duration-500 group-hover:scale-110"
                                    autoplay
                                    muted
                                    loop
                                    playsinline
                                    oncanplay="this.muted=true"
                                >
                                    <source
                                        src="assets/images/Personal-Projects/Character-Design/cover.mp4"
                                        type="video/mp4"
                                    />
                                </video>
                            </div>
                            // Text bottom
                            <div class="flex flex-col gap-2 p-6">
                                <h2 class="text-3xl font-bold text-black">"Personal"</h2>
                                <p class="text-base text-gray-500">"Characters Design, Sculpting, CGI, 2D Art & More."</p>
                                <div class="flex justify-end mt-2">
                                    <span
                                        class="flex justify-center items-center w-9 h-9 rounded-full font-bold text-black transition group-hover:scale-110"
                                        style="background-color: #fdbf3a;"
                                    >"→"</span>
                                </div>
                            </div>
                        </a>

                </div>
            </div>
        </div>
    }
}
