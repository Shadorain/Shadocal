use super::*;

#[component]
pub fn AccountDropdown() -> Element {
    let mut show_accounts = use_signal(|| false);
    let accounts = use_server_future(server::account::account_list)?;
    match &*accounts.read_unchecked() {
        Some(Ok(value)) => rsx! { "{value:?}" },
        Some(Err(err)) => rsx! { "Error: {err}" },
        None => rsx! { "Loading..." },
    }

    // rsx! {
    //     div {
    //         tabindex: "0",
    //         cursor: "pointer",
    //         role: "button",
    //         onmouseleave: move |_| show_accounts.set(false),
    //         onclick: move |_| show_accounts.toggle(),
    //         div { class: "hover:bg-gray-100 dark:hover:bg-ghdarkmetal rounded w-full py-1",
    //             div { class: "grid grid-cols-[auto,1fr,auto] items-center gap-2 px-1",
    //                 div { class: "w-8 h-8 rounded-md border flex items-center justify-center bg-gray-50 border-gray-200 text-gray-900 dark:bg-inherit dark:text-gray-500 dark:border-gray-700 ",
    //                     // icons::VersionTagIcon {}
    //                     "Icon"
    //                 }
    //                 div { class: "leading-snug text-xs text-left",
    //                     p { class: "font-medium text-gray-700 dark:text-gray-100",
    //                         "Using  Version"
    //                     }
    //                     p { class: "font-light", "vLONG" }
    //                 }
    //                 "DROPDOWN ICON"
    //             }
    //         }
    //         div {
    //             class: "relative w-full z-50",
    //             class: if !show_accounts() { "hidden" },
    //             div { class: "absolute flex flex-col bg-white dark:bg-ghdarkmetal text-left rounded-lg border  dark:border-gray-700 w-full overflow-hidden text-gray-500 dark:text-gray-100 text-xs shadow-lg",
    //                 for account in accounts().unwrap() {
    //                     "{account.email}"
    //                 }
    //             }
    //         }
    //     }
    // }
}
