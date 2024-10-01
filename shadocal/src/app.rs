use anyhow::Result;
use dioxus::{
    desktop::{self, tao::dpi::PhysicalSize, Config},
    prelude::*,
};
use dioxus_logger::tracing::{info, Level};

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

const SHADOCAL_TITLE: &str = "Shadocal";
const SHADOCAL_TITLE_DESC: &str = "A blazingly fast, calendar event formatter webserver tool.";
// const SHADOCAL_URL: &str = "https://github.com/Shadorain/Shadocal";
const SHADOCAL_VERSION: &str = env!("CARGO_PKG_VERSION");

const MIN_WINDOW_WIDTH: u32 = 302;
const MIN_WINDOW_HEIGHT: u32 = 574;

pub struct App;

impl App {
    pub fn run(config: Option<Config>) -> Result<()> {
        // Init logger
        dioxus_logger::init(Level::INFO).expect("failed to init logger");
        info!("Starting App");

        LaunchBuilder::desktop()
            .with_cfg(config.unwrap_or(App::config()))
            .launch(app);
        Ok(())
    }

    fn config() -> Config {
        Config::new()
            .with_custom_head(r#"<script src="https://cdn.tailwindcss.com"> </script>"#.to_string())
            // .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string())
            .with_window(
                desktop::WindowBuilder::new()
                    .with_decorations(false)
                    .with_title(format!(
                        "{SHADOCAL_TITLE} v{SHADOCAL_VERSION} - {SHADOCAL_TITLE_DESC}"
                    ))
                    .with_maximizable(true)
                    .with_minimizable(true)
                    .with_min_inner_size(PhysicalSize::new(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT))
                    .with_inner_size_constraints(desktop::tao::window::WindowSizeConstraints::new(
                        Some(desktop::tao::dpi::PixelUnit::Physical(
                            MIN_WINDOW_WIDTH.into(),
                        )),
                        Some(desktop::tao::dpi::PixelUnit::Physical(
                            MIN_WINDOW_HEIGHT.into(),
                        )),
                        None,
                        None,
                    ))
                    .with_inner_size(PhysicalSize::new(1300, 800)),
            )
            .with_menu(None)
        // .with_disable_context_menu(!cfg!(debug_assertions))
    }
}

#[component]
pub fn app() -> Element {
    // use_context_provider(|| Signal::new(Press::new().expect("Failed to initialize Press")));

    // rsx! { Router::<Route> {} }
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        img { src: "header.svg", id: "header" }
        div { id: "links",
            a { href: "https://dioxuslabs.com/learn/0.5/", "📚 Learn Dioxus" }
            a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            a { href: "https://github.com/DioxusLabs/dioxus-std", "⚙️ Dioxus Standard Library" }
            a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                "💫 VSCode Extension"
            }
            a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        }
    }
}
