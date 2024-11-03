use super::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: |error| {
                rsx! { "The home page encountered an error: {error:?}" }
            },
            div { class: "p-4 sm:ml-64 bg-white dark:bg-gray-900",
                AccountDropdown {}
                GoogleSignIn {}
                EventList {}
            }
        }
    }
}
