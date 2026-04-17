use crate::components::Navbar;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen bg-white">
            <Navbar />
            <div class="mx-10 border-b border-gray-200 md:mx-20"></div>
            <section class="flex flex-col flex-1 gap-20 justify-center items-center px-12 py-12 md:flex-row md:gap-28 md:px-24">

                // Left — circular image with orange background
                <div
                    class="flex flex-shrink-0 overflow-hidden justify-center items-center rounded-full w-96 h-96 md:w-[30rem] md:h-[30rem]"
                    style="background-color: #fdbf3a;"
                >
                    <img
                        class="object-contain w-full h-full p-6"
                        src="assets/images/Sushi Queen Logo Transparent.png"
                        alt="Kah Boon"
                    />
                </div>

                // Right — text content
                <div class="flex flex-col gap-8 max-w-xl items-center text-center md:items-start md:text-left">
                    <h1 class="text-4xl font-black leading-tight text-black md:text-6xl lg:text-7xl">
                        "Chang "
                        <br class="hidden md:block" />
                        "Kah Boon"
                    </h1>
                    <p class="text-2xl font-bold md:text-3xl" style="color: #fdbf3a;">"3D Designer"</p>
                    <p class="text-lg leading-relaxed text-gray-700 md:text-xl">
                        "Passionate about creating 3D art. Experienced in creating 3D designs for various events using 3ds Max. Self-taught in Blender and ZBrush."
                    </p>

                    // Social icons in circles
                    <div class="flex gap-5 mt-2 justify-center md:justify-start">
                        <a
                            href="mailto:changkahboon25@gmail.com"
                            target="_blank" rel="noopener noreferrer"
                            class="group flex justify-center items-center w-16 h-16 rounded-full border-2 transform transition duration-200 hover:scale-125"
                            style="background-color: #FFF3E0; border-color: #fdbf3a;"
                        >
                            <img class="w-6 h-6 transition group-hover:[filter:brightness(0)_saturate(100%)_invert(68%)_sepia(77%)_saturate(548%)_hue-rotate(347deg)_brightness(104%)]" src="assets/svg/envelope-solid.svg" alt="Email" />
                        </a>
                        <a
                            href="https://www.instagram.com/the_sushi_queen_art/"
                            target="_blank" rel="noopener noreferrer"
                            class="group flex justify-center items-center w-16 h-16 rounded-full border-2 transform transition duration-200 hover:scale-125"
                            style="background-color: #FFF3E0; border-color: #fdbf3a;"
                        >
                            <img class="w-6 h-6 transition group-hover:[filter:brightness(0)_saturate(100%)_invert(68%)_sepia(77%)_saturate(548%)_hue-rotate(347deg)_brightness(104%)]" src="assets/svg/instagram-brands.svg" alt="Instagram" />
                        </a>
                        <a
                            href="https://www.linkedin.com/in/kahboon"
                            target="_blank" rel="noopener noreferrer"
                            class="group flex justify-center items-center w-16 h-16 rounded-full border-2 transform transition duration-200 hover:scale-125"
                            style="background-color: #FFF3E0; border-color: #fdbf3a;"
                        >
                            <img class="w-6 h-6 transition group-hover:[filter:brightness(0)_saturate(100%)_invert(68%)_sepia(77%)_saturate(548%)_hue-rotate(347deg)_brightness(104%)]" src="assets/svg/linkedin-brands.svg" alt="LinkedIn" />
                        </a>
                        <a
                            href="https://wa.me/60127645817"
                            target="_blank" rel="noopener noreferrer"
                            class="group flex justify-center items-center w-16 h-16 rounded-full border-2 transform transition duration-200 hover:scale-125"
                            style="background-color: #FFF3E0; border-color: #fdbf3a;"
                        >
                            <img class="w-6 h-6 transition group-hover:[filter:brightness(0)_saturate(100%)_invert(68%)_sepia(77%)_saturate(548%)_hue-rotate(347deg)_brightness(104%)]" src="assets/svg/whatsapp-brands.svg" alt="WhatsApp" />
                        </a>
                    </div>
                </div>

            </section>
        </div>
    }
}
