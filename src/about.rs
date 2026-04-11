use crate::components::Navbar;
use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-100">
            <Navbar />
            <div class="flex flex-col items-center px-10 py-16 md:px-24">
                <h1 class="mb-6 text-7xl font-bold">"About"</h1>
                <a
                    href="https://docs.google.com/viewer?url=https://raw.githubusercontent.com/kahboon0425/kahboon_resume/refs/heads/main/resume.pdf"
                    class="mb-12 text-xl underline transition animate-bounce hover:text-[#fdbf3a] hover:scale-105"
                >
                    "View Resume"
                </a>

                <div class="grid grid-cols-1 gap-8 w-full max-w-5xl md:grid-cols-2">
                    // Skills — orange
                    <div class="p-8 rounded-2xl shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02]" style="background-color: #fdbf3a;">
                        <h2 class="mb-5 text-3xl font-bold text-black">"Skills"</h2>
                        <div class="flex flex-wrap gap-3">
                            {["3D Modeling", "Character Rigging", "Animation", "CGI / VFX", "Booth Design", "ZBrush Sculpting"]
                                .iter()
                                .map(|s| view! {
                                    <span class="px-4 py-2 text-base font-medium text-black bg-white rounded-full">
                                        {*s}
                                    </span>
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    </div>

                    // Languages — white
                    <div class="p-8 bg-white rounded-2xl border border-gray-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02]">
                        <h2 class="mb-5 text-3xl font-bold text-black">"Languages"</h2>
                        <div class="flex flex-wrap gap-3">
                            {["English", "Mandarin", "Bahasa Malaysia"]
                                .iter()
                                .map(|l| view! {
                                    <span class="px-4 py-2 text-base font-medium text-white rounded-full" style="background-color: #fdbf3a;">
                                        {*l}
                                    </span>
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    </div>

                    // Experience — white
                    <div class="p-8 bg-white rounded-2xl border border-gray-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02]">
                        <h2 class="mb-5 text-3xl font-bold text-black">"Experience"</h2>
                        <ul class="flex flex-col gap-4">
                            <li>
                                <p class="text-xl font-semibold text-black">"Igmax — Focus on Festival"</p>
                                <p class="text-base text-gray-500">"Freelance 3D Artist"</p>
                                <p class="mt-2 text-base text-gray-600">
                                    "Created 3D assets and visual designs for festival events and promotional materials."
                                </p>
                            </li>
                        </ul>
                    </div>

                    // Software — white
                    <div class="p-8 bg-white rounded-2xl border border-gray-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02]">
                        <h2 class="mb-5 text-3xl font-bold text-black">"Software"</h2>
                        <ul class="flex flex-col gap-3">
                            {["Blender", "ZBrush", "3ds Max", "Adobe After Effects"]
                                .iter()
                                .map(|s| view! {
                                    <li class="flex gap-3 items-center text-black text-lg font-medium">
                                        <span class="inline-block w-2.5 h-2.5 rounded-full shrink-0" style="background-color: #fdbf3a;" />
                                        {*s}
                                    </li>
                                })
                                .collect::<Vec<_>>()}
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    }
}
