use crate::error::Error;
use crate::state::AppState;
use gpui::*;
use gpui_component::{notification::NotificationType, WindowExt as _};
use std::path::Path;
use std::sync::Arc;

/// 发送通知的辅助函数
fn send_notification(
    handle: AnyWindowHandle,
    cx: &mut AsyncApp,
    notification_type: NotificationType,
    message: impl Into<SharedString>,
) {
    let _ = cx.update_window(handle, |_, window, cx| {
        window.push_notification((notification_type, message.into()), cx);
    });
}

pub fn open<F>(state: Arc<AppState>, function: F, window: &mut Window, cx: &mut App)
where
    F: Fn(&Path, Arc<AppState>) -> anyhow::Result<(), Error> + 'static,
{
    let path = cx.prompt_for_paths(PathPromptOptions {
        files: true,
        directories: false,
        multiple: false,
        prompt: None,
    });

    let handle = window.window_handle();

    cx.spawn(async move |mut cx| {
        match path.await.map_err(Into::into).and_then(|res| res) {
            Ok(Some(paths)) => {
                let selected_path = &paths[0];
                match function(selected_path, state) {
                    Ok(_) => {
                        send_notification(
                            handle,
                            &mut cx,
                            NotificationType::Success,
                            "File loaded successfully! Ready to export.",
                        );
                    }
                    Err(err) => {
                        send_notification(
                            handle,
                            &mut cx,
                            NotificationType::Error,
                            err.to_string(),
                        );
                    }
                }
            }
            Ok(None) => {
                send_notification(
                    handle,
                    &mut cx,
                    NotificationType::Warning,
                    "File selection canceled.",
                );
            }
            Err(err) => {
                send_notification(
                    handle,
                    &mut cx,
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
    F: Fn(&Path, Arc<AppState>) -> anyhow::Result<(), Error> + 'static,
{
    let directory = state
        .get_directory()
        .unwrap_or_else(|| Path::new(".").to_path_buf());
    let path = cx.prompt_for_new_path(&directory, None);

    let handle = window.window_handle();

    cx.spawn(async move |mut cx| {
        match path.await.map_err(Into::into).and_then(|res| res) {
            Ok(Some(selected_path)) => {
                match function(&selected_path, state) {
                    Ok(_) => {
                        send_notification(
                            handle,
                            &mut cx,
                            NotificationType::Success,
                            "File exported successfully.",
                        );
                    }
                    Err(err) => {
                        send_notification(
                            handle,
                            &mut cx,
                            NotificationType::Error,
                            err.to_string(),
                        );
                    }
                }
            }
            Ok(None) => {
                send_notification(
                    handle,
                    &mut cx,
                    NotificationType::Warning,
                    "File export canceled.",
                );
            }
            Err(err) => {
                send_notification(
                    handle,
                    &mut cx,
                    NotificationType::Error,
                    err.to_string(),
                );
            }
        }
    })
    .detach();
}
