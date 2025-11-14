use crate::error::Error;
use crate::state::AppState;
use gpui::*;
use gpui_component::{WindowExt as _, notification::NotificationType};
use std::path::Path;
use std::sync::Arc;

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

    cx.spawn(
        async move |cx| match path.await.map_err(Into::into).and_then(|res| res) {
            Ok(Some(path)) => {
                let path = &path[0];
                *state.selected_file.lock().unwrap() = Some(path.to_path_buf());
                *state.is_selected.lock().unwrap() = Some(true);
                match function(path, state.clone()) {
                    Ok(_) => {
                        let _ = cx.update_window(handle, |_, window, cx| {
                            window.push_notification(
                                (
                                    NotificationType::Success,
                                    "File loaded successfully! Ready to export.",
                                ),
                                cx,
                            );
                        });
                    }
                    Err(err) => {
                        let _ = cx.update_window(handle, |_, window, cx| {
                            window.push_notification(
                                (NotificationType::Error, SharedString::from(err.to_string())),
                                cx,
                            );
                        });
                    }
                }
            }
            Ok(None) => {
                let _ = cx.update_window(handle, |_, window, cx| {
                    window.push_notification(
                        (NotificationType::Warning, "File selection canceled."),
                        cx,
                    );
                });
            }
            Err(err) => {
                let _ = cx.update_window(handle, |_, window, cx| {
                    window.push_notification(
                        (NotificationType::Error, SharedString::from(err.to_string())),
                        cx,
                    );
                });
            }
        },
    )
    .detach();
}

pub fn save<F>(state: Arc<AppState>, function: F, window: &mut Window, cx: &mut App)
where
    F: Fn(&Path, Arc<AppState>) -> anyhow::Result<(), Error> + 'static,
{
    let guard = state.directory.lock().unwrap();
    let directory = guard
        .as_ref()
        .map(|p| p.as_path())
        .unwrap_or(Path::new("."));
    let path = cx.prompt_for_new_path(directory, None);

    let state = state.clone();
    let handle = window.window_handle();

    cx.spawn(
        async move |cx| match path.await.map_err(Into::into).and_then(|res| res) {
            Ok(Some(path)) => {
                let path = &path;
                match function(path, state) {
                    Ok(()) => {
                        let _ = cx.update_window(handle, |_, window, cx| {
                            window.push_notification(
                                (
                                    NotificationType::Success,
                                    SharedString::from("File exported successfully."),
                                ),
                                cx,
                            );
                        });
                    }
                    Err(err) => {
                        let _ = cx.update_window(handle, |_, window, cx| {
                            window.push_notification(
                                (NotificationType::Error, SharedString::from(err.to_string())),
                                cx,
                            );
                        });
                    }
                }
            }
            Ok(None) => {
                let _ = cx.update_window(handle, |_, window, cx| {
                    window.push_notification(
                        (NotificationType::Warning, "File export canceled."),
                        cx,
                    );
                });
            }
            Err(err) => {
                let _ = cx.update_window(handle, |_, window, cx| {
                    window.push_notification(
                        (NotificationType::Error, SharedString::from(err.to_string())),
                        cx,
                    );
                });
            }
        },
    )
    .detach();
}
