use crate::components::Navbar;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen bg-white">
            <Navbar />

            <section class="flex flex-col flex-1 gap-16 justify-center items-center px-12 py-12 md:flex-row md:gap-24 md:px-32">

                // Left — circular image with orange background
                <div
                    class="flex flex-shrink-0 overflow-hidden justify-center items-center rounded-full w-80 h-80 md:w-96 md:h-96"
                    style="background-color: #F5AA45;"
                >
                    <img
                        class="object-contain w-full h-full p-6"
                        src="assets/images/Sushi Queen Logo Transparent.png"
                        alt="Kah Boon"
                    />
                </div>

                // Right — text content
                <div class="flex flex-col gap-7 max-w-lg">
                    <h1 class="text-5xl font-black leading-tight text-black md:text-6xl">
                        "Chang"<br/>"Kah Boon"
                    </h1>
                    <p class="text-2xl font-bold text-black">"3D Artist & Designer"</p>
                    <p class="text-lg leading-relaxed text-gray-700 max-w-sm">
                        "Passionate about creating cute and stylized characters. Self-taught in Blender and ZBrush."
                    </p>

                    // Social icons in circles
                    <div class="flex gap-4 mt-1">
                        <a
                            href="mailto:changkahboon25@gmail.com"
                            class="group flex justify-center items-center w-12 h-12 rounded-full border-2 border-black transform transition duration-200 hover:scale-125 hover:bg-black hover:border-white"
                        >
                            <img class="w-5 h-5 transition group-hover:invert" src="assets/svg/envelope-solid.svg" alt="Email" />
                        </a>
                        <a
                            href="https://www.instagram.com/the_sushi_queen_art/"
                            class="group flex justify-center items-center w-12 h-12 rounded-full border-2 border-black transform transition duration-200 hover:scale-125 hover:bg-black hover:border-white"
                        >
                            <img class="w-5 h-5 transition group-hover:invert" src="assets/svg/instagram-brands.svg" alt="Instagram" />
                        </a>
                        <a
                            href="https://www.linkedin.com/in/kahboon"
                            class="group flex justify-center items-center w-12 h-12 rounded-full border-2 border-black transform transition duration-200 hover:scale-125 hover:bg-black hover:border-white"
                        >
                            <img class="w-5 h-5 transition group-hover:invert" src="assets/svg/linkedin-brands.svg" alt="LinkedIn" />
                        </a>
                        <a
                            href="https://wa.me/60127645817"
                            class="group flex justify-center items-center w-12 h-12 rounded-full border-2 border-black transform transition duration-200 hover:scale-125 hover:bg-black hover:border-white"
                        >
                            <img class="w-5 h-5 transition group-hover:invert" src="assets/svg/whatsapp-brands.svg" alt="WhatsApp" />
                        </a>
                    </div>
                </div>

            </section>
        </div>
    }
}
