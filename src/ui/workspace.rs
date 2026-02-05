mod actions;
mod components;

use components::{WorkspaceTitleBar, WorkspaceLayout};

use crate::state::AppState;
use gpui::prelude::*;
use gpui::*;
use gpui_component::{ActiveTheme as _, Root};
use std::sync::Arc;

pub struct Workspace {
    title_bar: Entity<WorkspaceTitleBar>,
    app_state: Arc<AppState>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let title_bar = WorkspaceTitleBar::view(window, cx);

        Self {
            title_bar,
            app_state: Arc::new(AppState::new()),
        }
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
        let workspace_id = cx.entity_id();
        let main = WorkspaceLayout::new(self.app_state.clone(), workspace_id);

        let content = div()
            .id("workspace-content")
            .flex()
            .flex_grow()
            .bg(cx.theme().background)
            .child(main);

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.title_bar.clone())
            .child(content)
            .children(notification_layer)
    }
}
