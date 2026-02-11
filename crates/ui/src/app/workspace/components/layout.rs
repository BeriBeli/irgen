use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme as _;

use crate::global::GlobalState;

use super::{WorkspaceFileUpload, WorkspaceFooter, WorkspaceHeader, WorkspaceRegisterExplorer};

pub struct WorkspaceLayout {
    footer: Entity<WorkspaceFooter>,
    header: Entity<WorkspaceHeader>,
    file_upload: Entity<WorkspaceFileUpload>,
    register_explorer: Entity<WorkspaceRegisterExplorer>,
}

impl WorkspaceLayout {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            footer: WorkspaceFooter::view(window, cx),
            header: WorkspaceHeader::view(window, cx),
            file_upload: WorkspaceFileUpload::view(window, cx),
            register_explorer: WorkspaceRegisterExplorer::view(window, cx),
        }
    }
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for WorkspaceLayout {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_file_selected = GlobalState::global(cx).is_file_selected();

        div()
            .id("workspace-main")
            .bg(cx.theme().muted)
            .text_color(cx.theme().foreground)
            .flex()
            .flex_col()
            .h_full()
            .min_h_0()
            .w_full()
            .px_8()
            .py_6()
            .child(self.header.clone())
            .child(
                div()
                    .id("workspace-body")
                    .flex()
                    .flex_col()
                    .flex_grow()
                    .min_h_0()
                    .when_else(
                        is_file_selected,
                        |this| this.child(self.register_explorer.clone()),
                        |this| {
                            this.items_center()
                                .justify_center()
                                .child(self.file_upload.clone())
                        },
                    ),
            )
            .child(self.footer.clone())
    }
}
