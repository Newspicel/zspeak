//! The page the window shows while it holds no server: the name, the connect field and the
//! saved servers.

use zgui::prelude::*;
use zgui_ui::prelude::*;

use crate::ui::home::connect::ConnectFieldProps;
use crate::ui::home::recent::RecentListProps;
use crate::ui::home::servers::ServerListProps;

/// One centred column.
#[component]
pub fn Home() -> impl IntoView {
    view! {
        ScrollArea(class = "zs-home", label = "Connect") {
            column(class = "zs-home__center") {
            column(class = "zs-home__column") {
                column(class = "zs-home__brand") {
                    text(class = "zs-home__name") {"zspeak"}
                    text(class = "zs-home__tagline") {"Voice over Media over QUIC"}
                }
                ConnectField()
                ServerList()
                RecentList()
            }
            }
        }
    }
}
