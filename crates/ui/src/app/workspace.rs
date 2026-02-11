mod actions;
mod components;
pub(crate) mod notifications;

use components::{WorkspaceLayout, WorkspaceTitleBar};

use gpui::prelude::*;
use gpui::*;
use gpui_component::{ActiveTheme as _, Root, Theme};

use crate::global::{GlobalState, ThemeModeSetting};
pub struct Workspace {
    title_bar: Entity<WorkspaceTitleBar>,
    layout: Entity<WorkspaceLayout>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let workspace_id = cx.entity_id();
        if cx.has_global::<GlobalState>() {
            GlobalState::global(cx).register_workspace(workspace_id);
        } else {
            cx.set_global(GlobalState::with_workspace_id(workspace_id));
        }

        cx.observe_window_appearance(window, |_, window, cx| {
            if GlobalState::global(cx).get_theme_mode() == ThemeModeSetting::System {
                Theme::sync_system_appearance(Some(window), cx);
                cx.notify();
            }
        })
        .detach();

        let this = Self {
            title_bar: WorkspaceTitleBar::view(window, cx),
            layout: WorkspaceLayout::view(window, cx),
        };

        let entity = cx.entity();
        cx.observe_release(&entity, move |_, _, cx| {
            if cx.has_global::<GlobalState>() {
                GlobalState::global(cx).unregister_workspace(workspace_id);
            }
        })
        .detach();

        this
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notification_layer = Root::render_notification_layer(window, cx).map(|layer| {
            div()
                .absolute()
                .top_0()
                .right_0()
                .mt(px(-6.0))
                .mr(px(6.0))
                .opacity(0.95)
                .child(layer)
        });

        let content = div()
            .id("workspace-content")
            .flex()
            .flex_grow()
            .bg(cx.theme().background)
            .child(self.layout.clone());

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.title_bar.clone())
            .child(content)
            .children(notification_layer)
    }
}
