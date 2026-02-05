use std::path::Path;
use std::sync::Arc;
use super::file_upload_empty::WorkspaceFileUploadEmpty;
use super::file_upload_selected::WorkspaceFileUploadSelected;
use super::style::file_upload_container_base;

use gpui::prelude::*;
use gpui::*;
use gpui_component::{ActiveTheme as _, WindowExt as _, green_500, notification::NotificationType};

use crate::processing::load_excel;
use crate::state::AppState;
use crate::ui::workspace::actions::open;

#[derive(IntoElement)]
pub struct WorkspaceFileUpload {
    app_state: Arc<AppState>,
    workspace_id: EntityId,
}

impl WorkspaceFileUpload {
    pub fn new(app_state: Arc<AppState>, workspace_id: EntityId) -> Self {
        Self {
            app_state,
            workspace_id,
        }
    }
}

impl RenderOnce for WorkspaceFileUpload {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let app_state = self.app_state.clone();
        let workspace_id = self.workspace_id;

        let is_selected = app_state.is_file_selected();
        let selected_file = app_state.get_selected_file();
        let selected_name = selected_file
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();
        let file_size = app_state
            .get_selected_file_size()
            .map(format_bytes)
            .unwrap_or_default();
        let sheet_count = app_state.get_sheet_count();
        let register_count = app_state
            .component()
            .map(|compo| compo.blks().iter().map(|blk| blk.regs().len()).sum::<usize>());

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
                let app_state = app_state.clone();
                move |paths: &ExternalPaths, window, cx| {
                    let Some(path) = paths.paths().first().cloned() else {
                        return;
                    };
                    if !is_supported_spreadsheet(&path) {
                        window.push_notification(
                            (
                                NotificationType::Error,
                                SharedString::from("Only .xlsx or .xlsm files are supported."),
                            ),
                            cx,
                        );
                        return;
                    }
                    let handle = window.window_handle();
                    let app_state = app_state.clone();
                    let workspace_id = workspace_id;
                    cx.spawn(async move |cx| {
                        let result = load_excel(&path, app_state);
                        let _ = cx.update_window(handle, |_, window, cx| {
                            match result {
                                Ok(_) => {
                                    window.push_notification(
                                        (
                                            NotificationType::Success,
                                            SharedString::from(
                                                "File loaded successfully! Ready to export.",
                                            ),
                                        ),
                                        cx,
                                    );
                                }
                                Err(err) => {
                                    window.push_notification(
                                        (
                                            NotificationType::Error,
                                            SharedString::from(err.to_string()),
                                        ),
                                        cx,
                                    );
                                }
                            }
                            cx.notify(workspace_id);
                        });
                    })
                    .detach();
                }
            })
            .cursor_pointer()
            .child(if is_selected {
                WorkspaceFileUploadSelected::new(
                    app_state.clone(),
                    workspace_id,
                    selected_name,
                    file_size,
                    sheet_count,
                    register_count,
                )
                .into_any_element()
            } else {
                WorkspaceFileUploadEmpty::new().into_any_element()
            })
            .on_click({
                let app_state = app_state.clone();
                move |_, window, cx| open(app_state.clone(), load_excel, window, cx)
            })
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;

    let bytes_f = bytes as f64;
    if bytes_f >= GB {
        format!("{:.1} GB", bytes_f / GB)
    } else if bytes_f >= MB {
        format!("{:.1} MB", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.1} KB", bytes_f / KB)
    } else {
        format!("{} B", bytes)
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
