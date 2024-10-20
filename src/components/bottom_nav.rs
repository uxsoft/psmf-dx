use dioxus::prelude::*;

pub fn DaisyBottomNavigation() -> Element {
    let mut page = use_signal(|| 0);

    rsx! {
        div { class: "btm-nav",
            button {
                "Home"
            }
            button { class: "active",
                "Fields"
            }
            button {
                "Settings"
            }
        }
    }
}
