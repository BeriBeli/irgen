use gpui::*;
use gpui_component::{WindowExt as _, notification::NotificationType};

use crate::global::GlobalState;

pub fn push(
    window: &mut Window,
    cx: &mut App,
    notification_type: NotificationType,
    message: impl Into<SharedString>,
) {
    let message = message.into();
    GlobalState::global(cx).push_notification(notification_type, message.clone());
    window.push_notification((notification_type, message), cx);
    GlobalState::notify_workspaces(cx);
}

pub fn push_on_window_handle(
    handle: AnyWindowHandle,
    cx: &mut AsyncApp,
    notification_type: NotificationType,
    message: impl Into<SharedString>,
) -> Result<()> {
    let message = message.into();
    cx.update_window(handle, move |_, window, cx| {
        push(window, cx, notification_type, message.clone());
    })
}
