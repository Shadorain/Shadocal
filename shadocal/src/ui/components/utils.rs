use super::*;

#[component]
pub fn ChildrenOrLoading(children: Element) -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |context: SuspenseContext| {
                rsx! {
                    if let Some(placeholder) = context.suspense_placeholder() {
                        {placeholder}
                    } else {
                        LoadingIndicator {}
                    }
                }
            },
            children
        }
    }
}

#[component]
pub fn LoadingIndicator() -> Element {
    rsx! {
        div { class: "spinner" }
    }
}
