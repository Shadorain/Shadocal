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

        #[cfg(feature = "server")]
        App::server();

        // #[cfg(feature = "web")]
        // dioxus::fulstack::launch(app);

        #[cfg(feature = "desktop")]
        {
            // let (ip, port) = shadocal_lib::ip_port();
            // dioxus::fullstack::prelude::server_fn::client::set_server_url(&format!(
            //     "http://{ip}:{port}"
            // ));
            dioxus::fullstack::prelude::server_fn::client::set_server_url("http://127.0.0.1:7117");
            LaunchBuilder::fullstack()
                .with_cfg(desktop! {
                    use dioxus::{fullstack::Config, desktop::{self, tao::dpi::PhysicalSize}};
                    use crate::{SHADOCAL_TITLE, SHADOCAL_VERSION, SHADOCAL_TITLE_DESC};

                    const MIN_WINDOW_WIDTH: u32 = 302;
                    const MIN_WINDOW_HEIGHT: u32 = 574;

                    Config::new().with_desktop_config(
                        desktop::Config::new()
                            .with_custom_head(
                                r#"<script src="https://cdn.tailwindcss.com"> </script>"#.to_string(),
                            )
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
                                    .with_inner_size_constraints(
                                        desktop::tao::window::WindowSizeConstraints::new(
                                            Some(desktop::tao::dpi::PixelUnit::Physical(
                                                MIN_WINDOW_WIDTH.into(),
                                            )),
                                            Some(desktop::tao::dpi::PixelUnit::Physical(
                                                MIN_WINDOW_HEIGHT.into(),
                                            )),
                                            None,
                                            None,
                                        ),
                                    )
                                    .with_inner_size(PhysicalSize::new(1300, 800)),
                            )
                            .with_menu(None)
                            .with_disable_context_menu(!cfg!(debug_assertions)),
                    )
                })
                .launch(app);
        }
        Ok(())
    }

    #[cfg(feature = "server")]
    fn server() {
        use std::net::{Ipv4Addr, SocketAddr};

        let (ip, port) = shadocal_lib::ip_port();
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                // Build our application with some routes
                let app = axum::Router::new()
                    // Server side render the application, serve static assets, and register server functions
                    // .serve_static_assets()
                    .register_server_functions();
                // .serve_dioxus_application(ServeConfig::new().unwrap(), app);

                // run it
                let addr =
                    SocketAddr::from((ip.parse::<Ipv4Addr>().expect("Invalid IP address"), port));
                let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            });
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
