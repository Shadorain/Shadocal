use anyhow::Result;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use super::Route;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

pub struct App;

impl App {
    pub fn run() -> Result<()> {
        // Init logger
        dioxus_logger::init(Level::INFO).expect("failed to init logger");
        info!("Starting App");

        #[cfg(feature = "web")]
        dioxus::web::launch(app);

        #[cfg(feature = "desktop")]
        {
            use crate::{SHADOCAL_TITLE, SHADOCAL_TITLE_DESC, SHADOCAL_VERSION};
            use dioxus::desktop::{launch, tao, Config, WindowBuilder};

            const MIN_WINDOW_WIDTH: u32 = 302;
            const MIN_WINDOW_HEIGHT: u32 = 574;

            launch::launch_virtual_dom(
                VirtualDom::new(app),
                Config::new()
                    .with_custom_head(
                        r#"<script src="https://cdn.tailwindcss.com"> </script>"#.to_string(),
                    )
                    // .with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string())
                    .with_window(
                        WindowBuilder::new()
                            .with_decorations(false)
                            .with_title(format!(
                                "{SHADOCAL_TITLE} v{SHADOCAL_VERSION} - {SHADOCAL_TITLE_DESC}"
                            ))
                            .with_maximizable(true)
                            .with_minimizable(true)
                            .with_min_inner_size(tao::dpi::PhysicalSize::new(
                                MIN_WINDOW_WIDTH,
                                MIN_WINDOW_HEIGHT,
                            ))
                            .with_inner_size_constraints(tao::window::WindowSizeConstraints::new(
                                Some(tao::dpi::PixelUnit::Physical(MIN_WINDOW_WIDTH.into())),
                                Some(tao::dpi::PixelUnit::Physical(MIN_WINDOW_HEIGHT.into())),
                                None,
                                None,
                            ))
                            .with_inner_size(tao::dpi::PhysicalSize::new(1300, 800)),
                    )
                    .with_menu(None)
                    .with_disable_context_menu(!cfg!(debug_assertions)),
            );
        }
        #[cfg(not(any(feature = "web", feature = "desktop")))]
        anyhow::bail!("Either `web` or `desktop` feature must be enabled")
    }
}

#[component]
pub fn app() -> Element {
    rsx! {
        div {
            class: "bg-white dark:bg-gray-900",
            Router::<Route> {}
        }
    }
}
