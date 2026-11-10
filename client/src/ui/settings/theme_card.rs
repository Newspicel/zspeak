//! One theme, drawn as the colours it wears at both surfaces.

use zgui::prelude::*;

use crate::ui::parts::Erase;
use crate::ui::state::AppState;
use crate::ui::theme::{Swatch, Variant};

/// One surface of a theme: the page, the plane beside it, the accent and a line of text.
fn surface(swatch: Swatch) -> AnyView {
    view! {
        row(class = "zs-theme__surface", style:background-color = swatch.background) {
            box(class = "zs-theme__plane", style:background-color = swatch.sidebar)
            column(class = "zs-theme__marks") {
                box(class = "zs-theme__dot", style:background-color = swatch.primary)
                box(class = "zs-theme__line", style:background-color = swatch.foreground)
            }
        }
    }
    .any()
}

/// A preview of one theme with its name. A press wears it.
#[component]
pub fn ThemeCard(variant: Variant) -> impl IntoView {
    let state = AppState::expect();
    let chosen = move || state.variant.get() == variant;
    view! {
        control(
            class = "zs-theme",
            attr:data-chosen = move || chosen().then(|| "true".to_owned()),
            tabindex = Focus::Sequential,
            a11y:role = Role::RadioButton,
            a11y:label = variant.label(),
            a11y:toggled_on = chosen,
            on:click = move |_| state.set_variant(variant)
        ) {
            row(class = "zs-theme__surfaces") {
                {surface(variant.swatch(false))}
                {surface(variant.swatch(true))}
            }
            text(class = "zs-theme__name") {{variant.label()}}
        }
    }
}
