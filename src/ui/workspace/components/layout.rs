use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme as _;

use crate::state::AppState;

use super::{WorkspaceFileUpload, WorkspaceFooter, WorkspaceHeader};

#[derive(IntoElement)]
pub struct WorkspaceLayout {
    app_state: Arc<AppState>,
    workspace_id: EntityId,
}

impl WorkspaceLayout {
    pub fn new(app_state: Arc<AppState>, workspace_id: EntityId) -> Self {
        Self {
            app_state,
            workspace_id,
        }
    }
}

impl RenderOnce for WorkspaceLayout {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id("workspace-main")
            .bg(cx.theme().muted)
            .text_color(cx.theme().foreground)
            .flex()
            .flex_col()
            .h_full()
            .w_full()
            .px_8()
            .py_6()
            .child(WorkspaceHeader::new())
            .child(
                div()
                    .id("workspace-body")
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .flex_grow()
                    .child(WorkspaceFileUpload::view()),
            )
            .child(WorkspaceFooter::new(self.app_state, self.workspace_id))
    }
}
