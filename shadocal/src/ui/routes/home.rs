use super::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "p-4 sm:ml-64 bg-white dark:bg-gray-900",
            ErrorBoundary {
                handle_error: |error| {
                    rsx! {
                        p { class: "text-sky-500",
                            "The home page encountered an error: {error:?}"
                        }
                    }
                },
            }
            AccountDropdown {}
            GoogleSignIn {}
            EventList {}
        }
    }
}
