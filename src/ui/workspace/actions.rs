use crate::error::Error;
use crate::processing::LoadResult;
use crate::processing::base;
use crate::global::GlobalState;
use gpui::*;
use gpui_component::{notification::NotificationType, WindowExt as _};
use std::path::Path;

/// Unified result type for irgen operations
pub type Result<T> = std::result::Result<T, Error>;

/// Helper function to send notifications with error logging
fn send_notification(
    handle: AnyWindowHandle,
    cx: &mut AsyncApp,
    notification_type: NotificationType,
    message: impl Into<SharedString>,
) {
    if let Err(e) = cx.update_window(handle, |_, window, cx| {
        window.push_notification((notification_type, message.into()), cx);
    }) {
        // Log error - notification failures are non-critical but useful for debugging
        eprintln!("[DEBUG] Failed to show notification: {}", e);
    }
}

pub fn open<F>(function: F, window: &mut Window, cx: &mut App)
where
    F: Fn(&Path) -> Result<LoadResult> + Send + 'static,
{
    let path = cx.prompt_for_paths(PathPromptOptions {
        files: true,
        directories: false,
        multiple: false,
        prompt: None,
    });

    let handle = window.window_handle();

    cx.spawn(async move |cx| {
        match path.await.map_err(Into::into).and_then(|res| res) {
            Ok(Some(paths)) => {
                let selected_path = paths[0].clone();
                let task = cx.background_spawn(async move { function(&selected_path) });
                let result = task.await;
                let _ = cx.update_window(handle, |_, window, cx| {
                    match result {
                        Ok(load) => {
                            GlobalState::global(cx).apply_load_result(load);
                            window.push_notification(
                                (
                                    NotificationType::Success,
                                    SharedString::from("File loaded successfully! Ready to export."),
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
                    cx.notify(GlobalState::global(cx).workspace_id());
                });
            }
            Ok(None) => {
                send_notification(
                    handle,
                    cx,
                    NotificationType::Warning,
                    "File selection canceled.",
                );
            }
            Err(err) => {
                send_notification(
                    handle,
                    cx,
                    NotificationType::Error,
                    err.to_string(),
                );
            }
        }
    })
    .detach();
}

pub fn save<F>(function: F, window: &mut Window, cx: &mut App)
where
    F: Fn(&Path, base::Component) -> Result<()> + Send + 'static,
{
    let directory = GlobalState::global(cx)
        .get_directory()
        .unwrap_or_else(|| Path::new(".").to_path_buf());
    let Some(component) = GlobalState::global(cx).component() else {
        window.push_notification(
            (
                NotificationType::Error,
                SharedString::from("Component not loaded."),
            ),
            cx,
        );
        return;
    };
    let path = cx.prompt_for_new_path(&directory, None);

    let handle = window.window_handle();

    cx.spawn(async move |cx| {
        match path.await.map_err(Into::into).and_then(|res| res) {
            Ok(Some(selected_path)) => {
                let task = cx.background_spawn(async move { function(&selected_path, component) });
                let result = task.await;
                let _ = cx.update_window(handle, |_, window, cx| {
                    match result {
                        Ok(_) => {
                            window.push_notification(
                                (
                                    NotificationType::Success,
                                    SharedString::from("File exported successfully."),
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
                    cx.notify(GlobalState::global(cx).workspace_id());
                });
            }
            Ok(None) => {
                send_notification(
                    handle,
                    cx,
                    NotificationType::Warning,
                    "File export canceled.",
                );
            }
            Err(err) => {
                send_notification(
                    handle,
                    cx,
                    NotificationType::Error,
                    err.to_string(),
                );
            }
        }
    })
    .detach();
}
