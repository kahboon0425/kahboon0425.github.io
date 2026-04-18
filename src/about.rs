use crate::components::Navbar;
use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-100">
            <Navbar />
            <div class="flex flex-col items-center px-10 py-16 md:px-24">
                <div class="flex flex-col items-center mb-8 w-full max-w-5xl md:relative md:flex-row md:justify-center">
                    <h1 class="text-7xl font-bold">"About"</h1>
                    <a
                        href="https://docs.google.com/viewer?url=https://raw.githubusercontent.com/kahboon0425/kahboon_resume/refs/heads/main/resume.pdf"
                        class="mt-3 text-base underline transition animate-bounce hover:text-[#fdbf3a] hover:scale-105 md:absolute md:right-0 md:mt-0"
                    >
                        "View Resume"
                    </a>
                </div>

                <div class="mb-8 w-full max-w-5xl text-lg leading-relaxed text-gray-700 text-justify">
                    <p class="mb-4">
                        "Hi, my name is Kahboon. I graduated with a degree in Data Analytics, but later discovered my passion for 3D art. After completing my studies, I spent a few months learning 3D design using Blender, focusing on modeling and animation, with the goal of pursuing a career in this field. I later started working as a 3D Designer, where I was introduced to 3ds Max and now use it in my daily work."
                    </p>
                    <p>
                        "Currently, I am open to new opportunities to further grow my skills and gain exposure to more exciting work in this field."
                    </p>
                </div>

                <div class="grid grid-cols-1 gap-8 w-full max-w-5xl md:grid-cols-2">
                    // Skills — orange
                    <div class="p-8 rounded-2xl shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02]" style="background-color: #fdbf3a;">
                        <h2 class="mb-5 text-3xl font-bold text-black">"Skills"</h2>
                        <div class="flex flex-wrap gap-3">
                            {["3D Modeling", "Sculpting", "Animation"]
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
                            {["Mandarin", "English", "Bahasa Malaysia"]
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
                                <p class="text-xl font-semibold text-black">"IGMAX Deco Sdn Bhd"</p>
                                <p class="text-base text-gray-500">"3D Designer (Jun 2025 - Current)"</p>
                                <p class="mt-2 text-base text-gray-600">"Create 3D designs for shopping mall events, primarily focusing on festive occasions such as Chinese New Year, Hari Raya, and Christmas."</p>
                            </li>
                        </ul>
                    </div>

                    // Software — white
                    <div class="p-8 bg-white rounded-2xl border border-gray-200 shadow-md transition duration-300 cursor-default hover:shadow-xl hover:scale-[1.02]">
                        <h2 class="mb-5 text-3xl font-bold text-black">"Software"</h2>
                        <ul class="flex flex-col gap-3">
                            {["3ds Max", "ZBrush", "Blender", "Photoshop", "Autocad"]
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
