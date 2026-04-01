use crate::components::Navbar;
use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-white">
            <Navbar />
            <div class="flex flex-col items-center px-10 py-12 md:px-20">
                <h1 class="mb-4 text-6xl font-bold">"About"</h1>
                <p class="mb-3 max-w-2xl text-center text-lg text-gray-600">
                    "Hi, I'm Kah Boon — an IT graduate passionate about 3D art. I create cute and stylized characters using Blender and ZBrush, and explore CGI, VFX, and booth design."
                </p>
                <a
                    href="https://docs.google.com/viewer?url=https://raw.githubusercontent.com/kahboon0425/kahboon_resume/refs/heads/main/resume.pdf"
                    class="mb-10 underline transition animate-bounce hover:text-blue-400 hover:scale-105"
                >
                    "View Resume"
                </a>

                <div class="grid grid-cols-1 gap-6 w-full max-w-4xl md:grid-cols-2">
                    // Skills — pink
                    <div class="p-6 bg-pink-50 rounded-xl border-2 border-pink-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02] hover:bg-pink-100">
                        <h2 class="mb-4 text-2xl font-bold text-pink-700">"Skills"</h2>
                        <div class="flex flex-wrap gap-2">
                            {["3D Modeling", "Character Rigging", "Animation", "CGI / VFX", "Booth Design", "ZBrush Sculpting"]
                                .iter()
                                .map(|s| view! {
                                    <span class="px-3 py-1 text-sm font-medium text-pink-800 bg-pink-200 rounded-full">
                                        {*s}
                                    </span>
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    </div>

                    // Languages — blue
                    <div class="p-6 bg-sky-50 rounded-xl border-2 border-sky-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02] hover:bg-sky-100">
                        <h2 class="mb-4 text-2xl font-bold text-sky-700">"Languages"</h2>
                        <div class="flex flex-wrap gap-2">
                            {["English", "Mandarin", "Bahasa Malaysia"]
                                .iter()
                                .map(|l| view! {
                                    <span class="px-3 py-1 text-sm font-medium text-sky-800 bg-sky-200 rounded-full">
                                        {*l}
                                    </span>
                                })
                                .collect::<Vec<_>>()}
                        </div>
                    </div>

                    // Experience — blue
                    <div class="p-6 bg-sky-50 rounded-xl border-2 border-sky-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02] hover:bg-sky-100">
                        <h2 class="mb-4 text-2xl font-bold text-sky-700">"Experience"</h2>
                        <ul class="flex flex-col gap-3">
                            <li>
                                <p class="font-semibold">"Igmax — Focus on Festival"</p>
                                <p class="text-sm text-gray-500">"Freelance 3D Artist"</p>
                                <p class="mt-1 text-sm text-gray-600">
                                    "Created 3D assets and visual designs for festival events and promotional materials."
                                </p>
                            </li>
                        </ul>
                    </div>

                    // Software — pink
                    <div class="p-6 bg-pink-50 rounded-xl border-2 border-pink-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02] hover:bg-pink-100">
                        <h2 class="mb-4 text-2xl font-bold text-pink-700">"Software"</h2>
                        <ul class="flex flex-col gap-2">
                            {["Blender", "ZBrush", "3ds Max", "Adobe After Effects"]
                                .iter()
                                .map(|s| view! {
                                    <li class="flex gap-2 items-center text-gray-700">
                                        <span class="inline-block w-2 h-2 rounded-full bg-pink-400 shrink-0" />
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
