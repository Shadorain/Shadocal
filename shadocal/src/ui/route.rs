use dioxus::prelude::*;
use dioxus_logger::tracing::info;

use super::server;

#[derive(Routable, Debug, Clone)]
pub enum Route {
    #[layout(Header)]
    #[redirect("/", || Route::Home {})]
    #[route("/")]
    Home {},
}

#[component]
fn Home() -> Element {
    let mut text = use_signal(String::new);
    rsx! {
        button { class: "text-white bg-[#4285F4] hover:bg-[#4285F4]/90 focus:ring-4 focus:outline-none focus:ring-[#4285F4]/50 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center dark:focus:ring-[#4285F4]/55 me-2 mb-2",
            onclick: move |_| async move {
                if server::utils::new_calendar(String::new()).await.is_ok() {
                    info!("Client successful new calendar");
                    text.set("HI".to_string());
                }
            },
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                // aria_hidden: "true",
                view_box: "0 0 18 19",
                fill: "currentColor",
                class: "w-4 h-4 me-2",
                path {
                    d: "M8.842 18.083a8.8 8.8 0 0 1-8.65-8.948 8.841 8.841 0 0 1 8.8-8.652h.153a8.464 8.464 0 0 1 5.7 2.257l-2.193 2.038A5.27 5.27 0 0 0 9.09 3.4a5.882 5.882 0 0 0-.2 11.76h.124a5.091 5.091 0 0 0 5.248-4.057L14.3 11H9V8h8.34c.066.543.095 1.09.088 1.636-.086 5.053-3.463 8.449-8.4 8.449l-.186-.002Z",
                    "fill-rule": "evenodd",
                    "clip-rule": "evenodd",
                }
            }
            "\nSign in with Google\n"
        },
        "Text: {text}"
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        div { class: "h-full grid grid-rows-[auto_1fr_auto]",
            nav { class: "w-full bg-white border-gray-200 border-b-2 border-gray-200",
                div { class: "flex justify-between mx-5 my-5",
                    // Logo Button
                    a { class: "py-[3px] px-2 hover:bg-gray-100 active:bg-gray-200 items-center hover:shadow-sm transition duration-200 hint--bottom-right hint--rounded",
                    }

                    // Output folder when compressing

                    // Help Button
                    a { class: "p-[3px] hover:bg-gray-100 active:bg-gray-200 items-center hover:shadow-sm transition duration-200 hint--bottom-left hint--rounded",
                    }
                }
            }

            // Route Content
            Outlet::<Route> {}
        }
    }
}
