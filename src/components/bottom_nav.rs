use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub struct BtmTab {
    pub title: String,
    pub icon: Element,
    pub content: Element,
}

#[component]
pub fn DaisyBottomNavigation(tabs: Vec<BtmTab>, initial_tab: usize) -> Element {
    let mut current_index = use_signal(|| initial_tab);

    fn is_active(cp: Signal<usize>, p: usize) -> &'static str {
        if cp() == p {
            "active"
        } else {
            ""
        }
    }

    rsx! {
        div {
            div {
                {tabs.get(current_index()).map(|t| t.content.clone()) }
            }
            div { class: "btm-nav",
                for (i, tab) in tabs.into_iter().enumerate() {
                    button {
                        class: is_active(current_index, i),
                        onclick: move |_| {
                            current_index.set(i);
                        },
                        "{tab.title}"
                    }
                }
            }
        }
    }
}
