use super::file_upload_empty::WorkspaceFileUploadEmpty;
use super::file_upload_selected::WorkspaceFileUploadSelected;
use super::style::file_upload_container_base;
use std::path::Path;

use gpui::prelude::*;
use gpui::*;
use gpui_component::{ActiveTheme as _, green_500, notification::NotificationType};
use irgen_core::processing::load_excel;

use crate::global::GlobalState;
use crate::app::workspace::actions::open;
use crate::app::workspace::notifications as workspace_notifications;

pub struct WorkspaceFileUpload {
    file_upload_empty: Entity<WorkspaceFileUploadEmpty>,
    file_upload_selected: Entity<WorkspaceFileUploadSelected>,
}

impl WorkspaceFileUpload {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let file_upload_empty = WorkspaceFileUploadEmpty::view(window, cx);
        let file_upload_selected = WorkspaceFileUploadSelected::view(window, cx);
        Self {
            file_upload_empty,
            file_upload_selected,
        }
    }
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for WorkspaceFileUpload {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = GlobalState::global(cx);

        let is_selected = state.is_file_selected();

        file_upload_container_base(cx, is_selected)
            .drag_over::<ExternalPaths>(|style, _, _, cx| {
                style
                    .border(px(1.0))
                    .border_dashed()
                    .border_color(green_500())
                    .bg(cx.theme().background)
            })
            .can_drop(|data, _, _| {
                data.downcast_ref::<ExternalPaths>()
                    .and_then(|paths| paths.paths().first())
                    .map(is_supported_spreadsheet)
                    .unwrap_or(false)
            })
            .on_drop({
                move |paths: &ExternalPaths, window, cx| {
                    let Some(path) = paths.paths().first().cloned() else {
                        return;
                    };
                    if !is_supported_spreadsheet(&path) {
                        workspace_notifications::push(
                            window,
                            cx,
                            NotificationType::Error,
                            "Only .xlsx or .xlsm files are supported.",
                        );
                        return;
                    }
                    let handle = window.window_handle();
                    cx.spawn(async move |cx| {
                        let result = cx.background_spawn(async move { load_excel(&path) }).await;
                        let _ = cx.update_window(handle, |_, window, cx| {
                            match result {
                                Ok(load) => {
                                    GlobalState::global(cx).apply_load_result(load);
                                    workspace_notifications::push(
                                        window,
                                        cx,
                                        NotificationType::Success,
                                        "File loaded successfully! Ready to export.",
                                    );
                                }
                                Err(err) => {
                                    workspace_notifications::push(
                                        window,
                                        cx,
                                        NotificationType::Error,
                                        err.to_string(),
                                    );
                                }
                            }
                            GlobalState::notify_workspaces(cx);
                        });
                    })
                    .detach();
                }
            })
            .when_else(
                is_selected,
                |this| this.child(self.file_upload_selected.clone()),
                |this| this.child(self.file_upload_empty.clone()),
            )
            .on_click(move |_, window, cx| open(load_excel, window, cx))
    }
}

fn is_supported_spreadsheet(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase()),
        Some(ext) if ext == "xlsx" || ext == "xlsm"
    )
}
