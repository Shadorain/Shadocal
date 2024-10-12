use dioxus::prelude::*;
use dioxus_logger::tracing::info;

mod home;
use home::Home;
mod components;
use components::*;

use super::server;

#[derive(Routable, Debug, Clone)]
pub enum Route {
    #[layout(SideBar)]
    #[redirect("/", || Route::Home {})]
    #[route("/")]
    Home {},
}

#[component]
fn SideBar() -> Element {
    rsx! {
        // Open sidebar (md, sm)
        button { class: "inline-flex items-center p-2 mt-2 ms-3 text-sm text-gray-500 rounded-lg sm:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600",
            "data-drawer-target": "logo-sidebar",
            "data-drawer-toggle": "logo-sidebar",
            r#type: "button",
            aria_controls: "logo-sidebar",
            span { class: "sr-only", "Open sidebar" }
            svg { class: "w-6 h-6",
                fill: "currentColor",
                "aria-hidden": "true",
                "viewBox": "0 0 20 20",
                clip_rule: "evenodd", fill_rule: "evenodd",
                path { d: "M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zm0 10.5a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5a.75.75 0 01-.75-.75zM2 10a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 10z" }
            }
        }

        // Sidebar
        aside { class: "fixed top-0 left-0 z-40 w-64 h-screen transition-transform -translate-x-full sm:translate-x-0 border-r-2 border-gray-200 dark:border-gray-700",
            id: "logo-sidebar",
            aria_label: "Sidebar",

            // Top Section
            div { class: "h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-900",
                // Logo Title
                a { class: "flex items-center ps-2.5 mb-5",
                    href: "https://github.com/Shadorain/Shadocal",
                    svg { class: "w-full h-24",
                        "viewBox": "-50 0 780 70",
                        "aria-hidden": "true",
                        fill: "#7456FC",
                        path { d: "M10.9 3C6.2 5.5 2.3 10.1 1 14.8c-.5 2-1 9-1 15.4V42h55v3.9c0 7.1.1 7.1-29.1 7.1H0v15.1l29.4-.3 29.4-.3 4.3-3c7.1-5 8.4-8.8 8.4-24v-13l-28.2-.3L15 27v-4.5c0-3.4.5-4.7 1.9-5.5 1.2-.6 12.1-1 28.5-1H72V1H43.3c-26.4.1-29.1.2-32.4 2M81 34.5V68h14.9l.3-12.8.3-12.7 21.3-.3L139 42v26h15V1h-15v26l-21.2-.2-21.3-.3-.3-12.8L95.9 1H81zm93.1-31.3c-2.8 1.4-5.4 3.7-7.2 6.7l-2.9 4.5V68h15V51h41v17h15V1h-28.2c-27 0-28.5.1-32.7 2.2M220 26v10h-41v-8.8c0-5.5.4-9.2 1.2-10 .9-.9 6.8-1.2 20.5-1.2H220zm25 8.5V68h22.9c28 0 30.8-.6 39.2-9 9.1-9 11.8-19.2 11.9-44.3V1h-74zm56.7-8.4c-.4 8.4-1 11-3.4 15.7-4.3 8.6-6.7 9.6-24 10l-14.3.4V16h42.3zm37.8-23.4C329.5 7.9 328 12 328 34.3c0 18.9.7 22.6 5.3 27.5 5.3 5.7 7.8 6.2 31.7 6.2s26.4-.5 31.7-6.2c4.9-5.2 5.3-8 5.3-35.3V1h-29.7c-25.2.1-30.3.3-32.8 1.7M387 32.9c0 14.1-.3 17.2-1.6 18.5-2.3 2.4-38.5 2.4-40.8 0-1.3-1.3-1.6-4.3-1.6-16.9s.3-15.6 1.6-16.9c1.3-1.4 4.7-1.6 22-1.6H387zm24.2-5.6.3 26.4 3 4.8c2 3.2 4.5 5.6 7.4 7.2 4.3 2.2 5.1 2.3 31.3 2.3H480V53h-25.3c-18.3 0-25.6-.3-26.5-1.2-.8-.8-1.2-6.4-1.2-18.5V16h53V1h-69.2zm89.3-24.6c-4.6 2.4-7.6 5.4-9.6 9.8-1.7 3.6-1.9 6.9-1.9 29.7V68h14.9l.3-8.3.3-8.2 20.8-.3L546 51v17h15V1h-28.7c-24.3.1-29.3.3-31.8 1.7m45.3 23-.3 9.8-20.2.3-20.3.2-.6-2.5c-1.1-4.3 0-15.5 1.7-16.6.7-.5 10.1-.9 20.7-.9h19.3zm24.4 1.5c.3 28.6.4 29.1 6.5 35.2 5.2 5.3 7 5.6 35.8 5.6H639V53h-25.3c-18.3 0-25.6-.3-26.5-1.2s-1.2-8-1.2-26V1h-16.2z" }
                    }
                }
                ul { class: "space-y-2 font-medium",
                    // Home Button
                    li {
                        Link { class: "flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-600 group",
                            to: Route::Home {},
                            svg { class: "w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white",
                                "viewBox": "0 0 24 24",
                                fill: "none",
                                stroke: "#7456FC", stroke_width: "1.8",
                                path { d: "M2 12.204c0-2.289 0-3.433.52-4.381.518-.949 1.467-1.537 3.364-2.715l2-1.241C9.889 2.622 10.892 2 12 2s2.11.622 4.116 1.867l2 1.241c1.897 1.178 2.846 1.766 3.365 2.715S22 9.915 22 12.203v1.522c0 3.9 0 5.851-1.172 7.063S17.771 22 14 22h-4c-3.771 0-5.657 0-6.828-1.212S2 17.626 2 13.725z" }
                                path { d: "M15 18H9", stroke_linecap: "round" }
                            }
                            span { class: "ms-3", "Home" }
                        }
                    }
                    // Accounts Button
                    li {
                        Link { class: "flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 group",
                            to: Route::Home {},
                            svg { class: "flex-shrink-0 w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white",
                                "viewBox": "0 0 18 18",
                                fill: "#7456fc",
                                circle { r: "2.25", cy: "5.25", cx: "3.75" }
                                circle { cx: "3.75", cy: "12.75", r: "2.25" }
                                path { d: "M16.25 6h-7.5a.75.75 0 0 1 0-1.5h7.5a.75.75 0 0 1 0 1.5m0 7.5h-7.5a.75.75 0 0 1 0-1.5h7.5a.75.75 0 0 1 0 1.5" }
                            }
                            span { class: "flex-1 ms-3 whitespace-nowrap", "Accounts" }
                        }
                    }
                }
            }

            // Bottom Section
            div { class: "absolute bottom-0 left-0 px-3 py-4 space-x-4 w-full overflow-y-auto z-20 bg-gray-50 dark:bg-gray-900",
                ul { class: "space-y-2 font-medium",
                    // Settings Button
                    li {
                        Link { class: "flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 group",
                            to: Route::Home {},
                            svg { class: "flex-shrink-0 w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white",
                                "viewBox": "0 0 24 24",
                                fill: "none",
                                stroke: "#7456fc", stroke_width: "1.8",
                                circle { r: "3", cx: "12", cy: "12" }
                                path { d: "M13.765 2.152C13.398 2 12.932 2 12 2s-1.398 0-1.765.152a2 2 0 0 0-1.083 1.083c-.092.223-.129.484-.143.863a1.62 1.62 0 0 1-.79 1.353 1.62 1.62 0 0 1-1.567.008c-.336-.178-.579-.276-.82-.308a2 2 0 0 0-1.478.396C4.04 5.79 3.806 6.193 3.34 7s-.7 1.21-.751 1.605a2 2 0 0 0 .396 1.479c.148.192.355.353.676.555.473.297.777.803.777 1.361s-.304 1.064-.777 1.36c-.321.203-.529.364-.676.556a2 2 0 0 0-.396 1.479c.052.394.285.798.75 1.605.467.807.7 1.21 1.015 1.453a2 2 0 0 0 1.479.396c.24-.032.483-.13.819-.308a1.62 1.62 0 0 1 1.567.008c.483.28.77.795.79 1.353.014.38.05.64.143.863a2 2 0 0 0 1.083 1.083C10.602 22 11.068 22 12 22s1.398 0 1.765-.152a2 2 0 0 0 1.083-1.083c.092-.223.129-.483.143-.863.02-.558.307-1.074.79-1.353a1.62 1.62 0 0 1 1.567-.008c.336.178.579.276.819.308a2 2 0 0 0 1.479-.396c.315-.242.548-.646 1.014-1.453s.7-1.21.751-1.605a2 2 0 0 0-.396-1.479c-.148-.192-.355-.353-.676-.555A1.62 1.62 0 0 1 19.562 12c0-.558.304-1.064.777-1.36.321-.203.529-.364.676-.556a2 2 0 0 0 .396-1.479c-.052-.394-.285-.798-.75-1.605-.467-.807-.7-1.21-1.015-1.453a2 2 0 0 0-1.479-.396c-.24.032-.483.13-.82.308a1.62 1.62 0 0 1-1.566-.008 1.62 1.62 0 0 1-.79-1.353c-.014-.38-.05-.64-.143-.863a2 2 0 0 0-1.083-1.083Z" }
                            }
                            span { class: "ms-3", "Settings" }
                        }
                    }
                    // About Button
                    li {
                        Link { class: "flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 group",
                            to: Route::Home {},
                            svg { class: "flex-shrink-0 w-6 h-6 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white",
                                "viewBox": "0 0 24 24",
                                g {
                                    fill: "none",
                                    stroke: "#7456fc", stroke_width: "2", stroke_linecap: "square",
                                    circle { cy: "12", cx: "12", r: "10" }
                                    path { d: "M12 7v6" }
                                }
                                circle { fill: "#7456fc", cy: "16.75", r: "1.25", cx: "12" }
                            }
                            span { class: "flex-1 ms-3 whitespace-nowrap", "About" }
                        }
                    }
                }
            }
        }

        // Content
        Outlet::<Route> {}
    }
}
