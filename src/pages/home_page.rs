use dioxus::prelude::*;

use crate::components::bottom_nav::{BtmTab, DaisyBottomNavigation};

#[component]
pub fn HomePage() -> Element {
    let mut teams = use_signal(|| vec![]);

    rsx! {
        button {  class: "btn",
            onclick: move |_| async move {
                if let Ok(data) = crate::server::search_teams("64".into()).await {
                    teams.set(data.clone());
                }
            },
            "Search Teams"
        }
        button { class: "btn",
        onclick: move |_| async move {
                if let Ok(data) = crate::server::search_teams("64".into()).await {
                    teams.set(data.clone());
                }
            },
            "Fetch Team Details"
        }
        p { "Server data: {teams:?}"}

        DaisyBottomNavigation{
            initial_tab: 0,
            tabs: vec![
                BtmTab { title: "Home".into(), icon: rsx!{}, content: rsx!{
                        p {
                            "Home tab"
                        }
                    }
                },
                BtmTab { title: "Settings".into(), icon: rsx!{}, content: rsx!{
                        p {
                            "Settings tab"
                        }
                    }
                }
            ]
        }
    }
}
