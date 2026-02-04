use crate::error::Error;
use crate::state::AppState;
use gpui::*;
use gpui_component::{notification::NotificationType, WindowExt as _};
use std::path::Path;
use std::sync::Arc;

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
        // Log error silently - notification failures are non-critical
        let _ = e;
    }
}

pub fn open<F>(state: Arc<AppState>, function: F, window: &mut Window, cx: &mut App)
where
    F: Fn(&Path, Arc<AppState>) -> Result<()> + 'static,
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
                let selected_path = &paths[0];
                match function(selected_path, state) {
                    Ok(_) => {
                        send_notification(
                            handle,
                            cx,
                            NotificationType::Success,
                            "File loaded successfully! Ready to export.",
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

pub fn save<F>(state: Arc<AppState>, function: F, window: &mut Window, cx: &mut App)
where
    F: Fn(&Path, Arc<AppState>) -> Result<()> + 'static,
{
    let directory = state
        .get_directory()
        .unwrap_or_else(|| Path::new(".").to_path_buf());
    let path = cx.prompt_for_new_path(&directory, None);

    let handle = window.window_handle();

    cx.spawn(async move |cx| {
        match path.await.map_err(Into::into).and_then(|res| res) {
            Ok(Some(selected_path)) => {
                match function(&selected_path, state) {
                    Ok(_) => {
                        send_notification(
                            handle,
                            cx,
                            NotificationType::Success,
                            "File exported successfully.",
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
