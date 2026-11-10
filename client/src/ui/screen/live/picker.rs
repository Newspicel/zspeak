//! Pick the display to share.

use zgui::prelude::*;
use zgui::reactive::StoredValue;
use zgui_ui::prelude::*;

use crate::engine::Command;
use crate::screen::{MonitorInfo, Source};
use crate::ui::parts::icons::{self, IconProps, IconSize};
use crate::ui::state::{AppState, Modal};

/// One row per display. A press shares it.
#[component]
pub fn ShareBody() -> impl IntoView {
    let state = AppState::expect();
    let monitors = StoredValue::new(match state.modal.get_untracked() {
        Modal::Share { monitors } => monitors,
        _ => Vec::new(),
    });
    view! {
        DialogHeader {
            DialogTitle {"Share your screen"}
            DialogDescription {"Everyone in your channel can watch."}
        }
        column(class = "zs-monitors") {
            for m in move || monitors.get_value(), key = |m: &MonitorInfo| m.id {
                row(
                    class = "zs-monitor",
                    tabindex = Focus::Sequential,
                    a11y:role = Role::Button,
                    a11y:label = m.name.clone(),
                    on:click = move |_| {
                        state.send(Command::StartShare(Source::Monitor(m.id)));
                        state.modal.set(Modal::None);
                    }
                ) {
                    Icon(svg = icons::MONITOR, size = IconSize::Sm)
                    text(class = "zs-monitor__name") {{format!("{}{}", m.name, if m.primary { " · main" } else { "" })}}
                    text(class = "zs-monitor__size") {{format!("{} × {}", m.width, m.height)}}
                }
            }
        }
        DialogFooter { DialogClose(variant = ButtonVariant::Outline) {"Cancel"} }
    }
}
