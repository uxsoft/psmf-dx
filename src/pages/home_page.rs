use dioxus::prelude::*;

use crate::components::bottom_nav::DaisyBottomNavigation;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        DaisyBottomNavigation{}
        button { class: "btn",
            "Button"
        }
    }
}
