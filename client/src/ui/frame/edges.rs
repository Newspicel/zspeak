//! The strips along the window's edges that resize it.

use zgui::prelude::*;

use crate::ui::frame::source::own_frame;
use crate::ui::parts::Erase;

/// Every edge and corner, with the class that places it.
const EDGES: [(ResizeEdge, &str); 8] = [
    (ResizeEdge::North, "zs-edge zs-edge--n"),
    (ResizeEdge::South, "zs-edge zs-edge--s"),
    (ResizeEdge::West, "zs-edge zs-edge--w"),
    (ResizeEdge::East, "zs-edge zs-edge--e"),
    (ResizeEdge::NorthWest, "zs-edge zs-edge--nw"),
    (ResizeEdge::NorthEast, "zs-edge zs-edge--ne"),
    (ResizeEdge::SouthWest, "zs-edge zs-edge--sw"),
    (ResizeEdge::SouthEast, "zs-edge zs-edge--se"),
];

/// The grab strips, where the window draws its own frame. CSS hides them while the window is
/// maximised or full screen.
#[component]
pub fn ResizeEdges() -> impl IntoView {
    let Some(window) = try_use_window() else {
        return ().any();
    };
    let strips = EDGES
        .into_iter()
        .map(|(edge, class)| {
            view! { box(class = class, on:pointer_down = window.resize_drag_handler(edge)) }.any()
        })
        .collect::<Vec<_>>();
    own_frame(view! { box(class = "zs-edges") {{strips}} })
}
