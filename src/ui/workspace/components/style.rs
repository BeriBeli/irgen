use gpui::prelude::*;
use gpui::*;
use gpui_component::ActiveTheme as _;

pub fn file_upload_container_base(cx: &App, is_selected: bool) -> Stateful<Div> {
    div()
        .id("file-upload")
        .w_full()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .h(px(260.0))
        .text_color(cx.theme().muted_foreground)
        .rounded(cx.theme().radius)
        .when(!is_selected, |this| this.px_4().py_8())
        .when_else(
            is_selected,
            |this| this.border(px(0.0)),
            |this| {
                this.border(px(1.0))
                    .border_dashed()
                    .border_color(cx.theme().border)
            },
        )
        .when(!is_selected, |this| {
            this.hover(|this| {
                this.bg(cx.theme().background)
                    .text_color(cx.theme().foreground)
                    .border_color(cx.theme().foreground.opacity(0.2))
            })
        })
}

pub fn file_info_card(cx: &App) -> Div {
    div()
        .w_full()
        .h_full()
        .bg(cx.theme().background)
        .border_1()
        .border_color(cx.theme().border)
        .rounded(cx.theme().radius)
        .px_5()
        .py_4()
        .flex()
        .flex_col()
        .gap_4()
}

pub fn info_pill(cx: &App) -> Div {
    div()
        .flex()
        .items_center()
        .gap_1()
        .px_2()
        .py(px(3.0))
        .text_xs()
        .rounded(px(6.0))
        .border_1()
        .border_color(cx.theme().border)
        .bg(cx.theme().background)
}
