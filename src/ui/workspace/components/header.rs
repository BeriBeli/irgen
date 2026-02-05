use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme as _;

#[derive(IntoElement)]
pub struct WorkspaceHeader;

impl WorkspaceHeader {
    pub fn new() -> Self {
        Self
    }
}

impl RenderOnce for WorkspaceHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id("workspace-header")
            .flex()
            .items_center()
            .gap_3()
            .mb_4()
            .child(
                div()
                    .w_8()
                    .h_8()
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().background)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(cx.theme().foreground)
                    .child("IR"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(div().text_2xl().font_weight(FontWeight::SEMIBOLD).child("irgen"))
                    .child(
                        div()
                            .text_sm()
                            .text_color(cx.theme().muted_foreground)
                            .child("Register Generation Tool"),
                    ),
            )
    }
}
