//! Where you are on the server: the channel you are in, after the server key.

use zgui::prelude::*;

use crate::ui::state::AppState;

/// The channel you are in, as a quiet step after the server.
#[component]
pub fn Where() -> impl IntoView {
    let state = AppState::expect();
    let channel = move || state.my_channel().map(|c| c.name);
    view! {
        if move || channel().is_some() {
            row(class = "zs-where") {
                text(class = "zs-where__sep") {"/"}
                text(class = "zs-where__name") {{move || channel().unwrap_or_default()}}
            }
        }
    }
}
