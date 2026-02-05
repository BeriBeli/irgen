use std::time::Duration;

use gpui::prelude::*;
use gpui::*;
use gpui_component::{ActiveTheme as _, green_500};

#[derive(IntoElement)]
pub struct WorkspaceFileUploadEmpty;

impl WorkspaceFileUploadEmpty {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for WorkspaceFileUploadEmpty {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let upload_icon = svg()
            .path("icons/excel.svg")
            .w_12()
            .h_12()
            .text_color(green_500())
            .with_animation(
                "upload-breath",
                Animation::new(Duration::from_secs_f32(2.4))
                    .repeat()
                    .with_easing(pulsating_between(0.6, 1.0)),
                |this, delta| this.opacity(delta),
            );

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_2()
            .text_sm()
            .child(upload_icon)
            .child("Click to select a spreadsheet")
            .child(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("or drag and drop file here"),
            )
    }
}
