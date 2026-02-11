use crate::global::GlobalState;
use crate::app::workspace::notifications as workspace_notifications;
use gpui::*;
use gpui_component::notification::NotificationType;
use irgen_core::error::Error;
use irgen_core::processing::LoadResult;
use irgen_core::processing::base;
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
    if let Err(e) = workspace_notifications::push_on_window_handle(
        handle,
        cx,
        notification_type,
        message.into(),
    ) {
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

    cx.spawn(
        async move |cx| match path.await.map_err(Into::into).and_then(|res| res) {
            Ok(Some(paths)) => {
                let Some(selected_path) = paths.into_iter().next() else {
                    send_notification(
                        handle,
                        cx,
                        NotificationType::Warning,
                        "File selection canceled.",
                    );
                    return;
                };
                let task = cx.background_spawn(async move { function(&selected_path) });
                let result = task.await;
                let _ = cx.update_window(handle, |_, window, cx| {
                    match result {
                        Ok(load) => {
                            GlobalState::apply_load_result(cx, load);
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
                send_notification(handle, cx, NotificationType::Error, err.to_string());
            }
        },
    )
    .detach();
}

pub fn save<F>(function: F, window: &mut Window, cx: &mut App)
where
    F: Fn(&Path, &base::Component) -> Result<()> + Send + 'static,
{
    let directory = GlobalState::global(cx)
        .get_directory()
        .unwrap_or_else(|| Arc::new(Path::new(".").to_path_buf()));
    let Some(component) = GlobalState::global(cx).component() else {
        workspace_notifications::push(window, cx, NotificationType::Error, "Component not loaded.");
        return;
    };
    let path = cx.prompt_for_new_path(directory.as_ref(), None);

    let handle = window.window_handle();

    cx.spawn(
        async move |cx| match path.await.map_err(Into::into).and_then(|res| res) {
            Ok(Some(selected_path)) => {
                let task = cx
                    .background_spawn(async move { function(&selected_path, component.as_ref()) });
                let result = task.await;
                let _ = cx.update_window(handle, |_, window, cx| {
                    match result {
                        Ok(_) => {
                            workspace_notifications::push(
                                window,
                                cx,
                                NotificationType::Success,
                                "File exported successfully.",
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
                send_notification(handle, cx, NotificationType::Error, err.to_string());
            }
        },
    )
    .detach();
}
