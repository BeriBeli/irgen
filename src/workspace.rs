mod action;
mod title_bar;

use action::{open, save};
use title_bar::AppTitleBar;

use crate::services::{export_ipxact_xml, export_regvue_json, load_excel};
use crate::state::AppState;
use gpui::prelude::*;
use gpui::*;
use gpui_component::{Disableable as _, Root};
use std::sync::Arc;

use gpui_component::{
    ActiveTheme as _,
    button::{Button, ButtonVariants as _},
};

pub struct Workspace {
    title_bar: Entity<AppTitleBar>,
    app_state: Arc<AppState>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let title_bar = AppTitleBar::view(window, cx);

        Self {
            title_bar,
            app_state: Arc::new(AppState::new()),
        }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notification_layer = Root::render_notification_layer(window, cx);
        let app_state = self.app_state.clone();

        // 使用便捷的访问方法
        let is_selected = app_state.is_file_selected();
        let selected_path = app_state
            .get_selected_file()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(String::new);
        let main = div()
            .id("workspace-main")
            .bg(cx.theme().background)
            .text_color(rgb(0x1f2937))
            .flex()
            .items_center()
            .justify_center()
            .h_full()
            .w_full()
            .child(
                div().w_full().max_w(px(672.0)).mx_auto().p_4().child(
                    div()
                        .bg(gpui::white())
                        .rounded_xl()
                        .shadow_lg()
                        .p_8()
                        .child(div().text_2xl().text_center().mb_6().child("irgen"))
                        .child(
                            div()
                                .id("file-upload")
                                .w_full()
                                .flex()
                                .flex_col()
                                .items_center()
                                .px_4()
                                .py_8()
                                .mb_8()
                                .bg(gpui::white())
                                .text_color(rgb(0x3b82f6))
                                .rounded_lg()
                                .shadow_md()
                                .border(px(1.0))
                                .border_dashed()
                                .border_color(cx.theme().border)
                                .hover(|this| this.bg(rgb(0xeff6ff)).text_color(rgb(0x2563eb)))
                                .cursor_pointer()
                                .child(
                                    svg()
                                        .path("icons/excel.svg")
                                        .w_12()
                                        .h_12()
                                        .text_color(rgb(0x3b82f6)),
                                )
                                .when_else(
                                    is_selected,
                                    |this| this.child(selected_path),
                                    |this| this.child("Click to select a spreadsheet"),
                                )
                                .on_click({
                                    let app_state = app_state.clone();
                                    move |_, window, cx| {
                                        open(app_state.clone(), load_excel, window, cx)
                                    }
                                }),
                        )
                        .child(
                            div()
                                .grid()
                                .grid_cols(1)
                                .justify_center()
                                .flex()
                                .gap_12()
                                .child(
                                    Button::new("button0")
                                        .primary()
                                        .w_56()
                                        .items_center()
                                        .label("export ipxact")
                                        .disabled(!is_selected)
                                        .on_click({
                                            let app_state = app_state.clone();
                                            move |_, window, cx| {
                                                save(
                                                    app_state.clone(),
                                                    export_ipxact_xml,
                                                    window,
                                                    cx,
                                                )
                                            }
                                        })
                                        .cursor_pointer(),
                                )
                                .child(
                                    Button::new("button1")
                                        .primary()
                                        .w_56()
                                        .items_center()
                                        .label("export regvue")
                                        .disabled(!is_selected)
                                        .on_click({
                                            let app_state = app_state.clone();
                                            move |_, window, cx| {
                                                save(
                                                    app_state.clone(),
                                                    export_regvue_json,
                                                    window,
                                                    cx,
                                                )
                                            }
                                        })
                                        .cursor_pointer(),
                                ),
                        ),
                ),
            );

        let content = div()
            .id("workspace-content")
            .flex()
            .flex_grow()
            .bg(cx.theme().background)
            .child(main);

        div()
            .flex()
            .flex_col()
            .size_full()
            .child(self.title_bar.clone())
            .child(content)
            .children(notification_layer)
    }
}
