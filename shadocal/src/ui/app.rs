use anyhow::Result;
use dioxus::prelude::*;

use super::{state::UIState, Route};

// const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

pub struct App;

impl App {
    pub fn run() -> Result<()> {
        // Init logger
        // dioxus_logger::init(Level::INFO).expect("failed to init logger");
        // dioxus_logger::tracing::info!("Starting App");

        #[cfg(feature = "server")]
        {
            use super::auth;
            use axum::routing::get;
            use shadocal_lib::{Db, State};
            use std::{
                net::{Ipv4Addr, SocketAddr},
                sync::Arc,
            };

            let (ip, port) = shadocal_lib::ip_port();
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move {
                    let state = Arc::new(
                        State::new(Db::new(None).expect("Failed to init db"))
                            .await
                            .expect("Failed to init state"),
                    );

                    let statec = state.clone();
                    // Move the `Arc<state>` into the closure
                    let state_fn: Box<dyn Fn() -> Box<dyn std::any::Any> + Sync + Send> =
                        Box::new(move || Box::new(Arc::clone(&statec)));

                    let app = axum::Router::new()
                        .route("/account/auth/login", get(auth::login))
                        .route("/account/auth/authenticate", get(auth::authenticate))
                        .with_state(state)
                        .register_server_functions_with_context(Arc::new(vec![state_fn]));

                    let listener = tokio::net::TcpListener::bind(&SocketAddr::from((
                        ip.parse::<Ipv4Addr>().expect("Invalid IP address"),
                        port,
                    )))
                    .await
                    .unwrap();

                    axum::serve(listener, app.into_make_service())
                        .await
                        .unwrap();
                });
        }

        // #[cfg(feature = "web")]
        // dioxus::web::launch(app);

        #[cfg(feature = "desktop")]
        {
            dioxus::fullstack::prelude::server_fn::client::set_server_url("http://127.0.0.1:7117");
            LaunchBuilder::desktop()
                .with_cfg(desktop! {
                    use dioxus::desktop::{Config, self, tao::dpi::PhysicalSize};
                    use crate::{SHADOCAL_TITLE, SHADOCAL_VERSION, SHADOCAL_TITLE_DESC};

                    const MIN_WINDOW_WIDTH: u32 = 302;
                    const MIN_WINDOW_HEIGHT: u32 = 574;

                    Config::new()
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
                        .with_disable_context_menu(!cfg!(debug_assertions))
                })
                .launch(app);
        }

        // #[cfg(not(any(feature = "web", feature = "desktop")))]
        #[cfg(not(feature = "desktop"))]
        anyhow::bail!("Either `web` or `server` feature must be enabled")
    }
}

#[component]
pub fn app() -> Element {
    use_context_provider(|| Signal::new(UIState::default()));

    rsx! {
        div {
            class: "bg-white dark:bg-gray-900",
            Router::<Route> {}
        }
    }
}
