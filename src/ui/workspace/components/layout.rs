use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme as _;

use super::{WorkspaceFileUpload, WorkspaceFooter, WorkspaceHeader};

pub struct WorkspaceLayout {
    footer: Entity<WorkspaceFooter>,
    header: Entity<WorkspaceHeader>,
    file_upload: Entity<WorkspaceFileUpload>,
}

impl WorkspaceLayout {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            footer: WorkspaceFooter::view(window, cx),
            header: WorkspaceHeader::view(window, cx),
            file_upload: WorkspaceFileUpload::view(window, cx),
        }
    }
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for WorkspaceLayout {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
            .child(self.header.clone())
            .child(
                div()
                    .id("workspace-body")
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .flex_grow()
                    .child(self.file_upload.clone()),
            )
            .child(self.footer.clone())
    }
}
