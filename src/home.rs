use crate::components::{Icon, Navbar};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen bg-white">
            <Navbar />
            <section class="flex flex-col flex-1 gap-16 justify-center items-center px-10 py-16 md:flex-row md:px-24">
                // Left: circular logo + social icons
                <div class="flex flex-col gap-8 items-center">
                    <div class="flex overflow-hidden justify-center items-center rounded-full border-4 border-black size-64 md:size-80">
                        <img
                            class="object-contain w-full h-full"
                            src="assets/images/Sushi Queen Logo Transparent.png"
                            alt="Sushi Queen Logo"
                        />
                    </div>
                    <div class="flex flex-row gap-8">
                        <Icon
                            href="mailto:changkahboon25@gmail.com"
                            src="assets/svg/envelope-solid.svg"
                            alt="Email SVG"
                        />
                        <Icon
                            href="https://www.instagram.com/the_sushi_queen_art/"
                            src="assets/svg/instagram-brands.svg"
                            alt="Instagram SVG"
                        />
                        <Icon
                            href="https://www.linkedin.com/in/kahboon"
                            src="assets/svg/linkedin-brands.svg"
                            alt="LinkedIn SVG"
                        />
                        <Icon
                            href="https://wa.me/60127645817"
                            src="assets/svg/whatsapp-brands.svg"
                            alt="WhatsApp SVG"
                        />
                    </div>
                </div>

                // Right: intro text
                <div class="flex flex-col gap-6 max-w-md">
                    <h1 class="text-5xl font-bold md:text-6xl">"Hi, I am Kah Boon"</h1>
                    <p class="text-lg leading-relaxed text-gray-600">
                        "3D Artist & Designer passionate about creating cute and stylized characters. Self-taught in Blender, ZBrush."
                    </p>
                    <div class="flex gap-3 mt-2">
                        <span class="inline-block w-3 h-3 rounded-full bg-pink-400" />
                        <span class="inline-block w-3 h-3 rounded-full bg-gray-300" />
                        <span class="inline-block w-3 h-3 rounded-full bg-gray-300" />
                        <span class="inline-block w-3 h-3 rounded-full bg-gray-300" />
                    </div>
                </div>
            </section>
        </div>
    }
}
