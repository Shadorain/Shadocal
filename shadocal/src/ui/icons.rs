#![allow(unused)]
use dioxus::prelude::*;

#[component]
pub(crate) fn DropdownChevron() -> Element {
    rsx! {
        svg {
            "viewBox": "0 0 18 18",
            height: "18",
            width: "18",
            title { "chevron-down" }
            g { fill: "#7456fc",
                path {
                    "stroke-width": "1.5",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    fill: "none",
                    d: "M15.25 6.5L9 12.75L2.75 6.5",
                    stroke: "#7456fc",
                }
            }
        }
    }
}

#[component]
pub(crate) fn AccountList() -> Element {
    rsx! {
    svg {
            height: "24",
            width: "24",
            "viewBox": "0 0 24 24",
            title { "paragraph-2" }
            g { fill: "#7456fc",
                rect { height: "2", fill: "#7456fc", x: "13", y: "3", width: "9", "stroke-width": "0" }
                rect { fill: "#7456fc", y: "7", x: "13", width: "9", height: "2", "stroke-width": "0" }
                rect { x: "2", y: "2", rx: "2", "stroke-width": "0", width: "8", height: "8", fill: "#7456fc", ry: "2" }
                rect { ry: "2", "stroke-width": "0", y: "14", height: "8", rx: "2", fill: "#7456fc", x: "2", width: "8" }
                rect { width: "9", x: "13", fill: "#7456fc", height: "2", "stroke-width": "0", y: "15" }
                rect { fill: "#7456fc", width: "9", "stroke-width": "0", y: "19", height: "2", x: "13" ,}
            }
        }
    }
}
