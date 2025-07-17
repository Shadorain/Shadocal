use shadocal_lib::Profile;

use super::*;

#[component]
pub fn AccountDropdown() -> Element {
    let mut state = use_context::<UIState>();
    let mut show_accounts = use_signal(|| false);
    let accounts = use_server_future(server::account::account_list)?;
    let Ok(accounts) = accounts().unwrap() else {
        return rsx! {
            div { class: "border border-gray-700 dark:hover:bg-gray-800 dark:hover:border-gray-600 rounded w-1/4 py-3 items-center gap-2 px-1.5",
                p { class: "font-medium text-red-500", "Error loading accounts" }
            }
        };
    };
    let accounts = use_signal(|| accounts);
    rsx! {
        div {
            tabindex: "0",
            cursor: "pointer",
            role: "button",
            // onmouseleave: move |_| show_accounts.set(false),
            onclick: move |_| show_accounts.toggle(),
            // Dropdown Button
            div { class: "border border-gray-700 dark:hover:bg-gray-800 dark:hover:border-gray-600 rounded w-1/4 py-1.5 px-0.5",
                if let Some(account) = state.read().current_profile {
                    AccountProfile { account }
                } else {
                    div { class: "grid grid-cols-[auto,1fr,auto] items-center gap-2 px-1",
                        div { class: "w-8 h-8 rounded-md border flex items-center justify-center bg-gray-50 border-gray-200 text-gray-900 dark:bg-inherit dark:text-gray-500 dark:border-gray-700",
                            icons::AccountList {}
                        }
                        div { class: "leading-snug text-xs text-left",
                            p { class: "select-none font-medium text-gray-700 dark:text-gray-100", "All Accounts" }
                        }
                        icons::DropdownChevron {}
                    }
                }
            }
            // Dropdown Section
            div { class: "relative w-1/4 z-50",
                class: if !show_accounts() { "hidden" },
                div { class: "absolute flex flex-col bg-white dark:bg-gray-800 text-left rounded-lg border dark:border-gray-700 w-full overflow-hidden text-gray-500 dark:text-gray-100 text-xs shadow-lg",
                    for (i, account) in accounts().into_iter().enumerate() {
                        div {
                            onclick: move |_| { state.write().set_profile(use_signal(|| account.clone())); show_accounts.set(false); },
                            AccountProfile { account: accounts.map(move |v| &v[i]) }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn AccountProfile<T: Readable<Target = Profile> + PartialEq + 'static>(account: T) -> Element {
    let account = account.read();
    rsx! {
        div { class: "bg-gray-100 dark:bg-gray-900 flex flex-row items-center hover:bg-gray-100 dark:hover:bg-gray-800 py-2 space-x-2 px-2",
            span { class: "block h-8 w-8 rounded-md border flex items-center justify-center bg-gray-50 overflow-hidden border-2 border-gray-600 focus:outline-none focus:border-white",
                img { class: "h-full w-full",
                    src: "{account.picture_link}"
                }
            }
            div { class: "flex flex-col",
                span { class: "text-gray-700 dark:text-gray-100 font-semibold",
                    "{account.name}"
                }
                span { class: "row-span-2 col-span-2 dark:text-gray-300", "{account.email}" }
            }
        }
    }
}
