use super::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "p-4 sm:ml-64 bg-white dark:bg-gray-900",
            h1 { class: "text-2xl text-center font-semibold text-gray-900 dark:text-gray-100",
                "About Page"
            }
            EventList {}
        }
    }
}
