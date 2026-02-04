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
    Icon, IconName, Sizable as _,
    button::{Button, ButtonVariants as _},
    group_box::{GroupBox, GroupBoxVariants as _},
    blue_50, blue_400, blue_500, blue_600,
};
use std::time::Duration;

pub struct Workspace {
    title_bar: Entity<AppTitleBar>,
    app_state: Arc<AppState>,
}

fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;

    let bytes_f = bytes as f64;
    if bytes_f >= GB {
        format!("{:.1} GB", bytes_f / GB)
    } else if bytes_f >= MB {
        format!("{:.1} MB", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.1} KB", bytes_f / KB)
    } else {
        format!("{} B", bytes)
    }
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
        let notification_layer = Root::render_notification_layer(window, cx).map(|layer| {
            div()
                .absolute()
                .top_0()
                .right_0()
                .mt(px(-6.0))
                .mr(px(6.0))
                .opacity(0.95)
                .child(layer)
        });
        let app_state = self.app_state.clone();

        let workspace_id = cx.entity_id();

        // 使用便捷的访问方法
        let is_selected = app_state.is_file_selected();
        let selected_file = app_state.get_selected_file();
        let selected_name = selected_file
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();
        let file_size = app_state
            .get_selected_file_size()
            .map(format_bytes)
            .unwrap_or_default();
        let file_label = if !selected_name.is_empty() && !file_size.is_empty() {
            format!("{} ({})", selected_name, file_size)
        } else {
            selected_name.clone()
        };
        let sheet_count = app_state.get_sheet_count();
        let register_count = {
            let guard = app_state.component_guard();
            guard
                .as_ref()
                .map(|compo| compo.blks().iter().map(|blk| blk.regs().len()).sum::<usize>())
        };
        let summary = match (sheet_count, register_count) {
            (Some(sheets), Some(regs)) => {
                Some(format!("Detected {} sheets, {} registers", sheets, regs))
            }
            (Some(sheets), None) => Some(format!("Detected {} sheets", sheets)),
            _ => None,
        };
        let main = div()
            .id("workspace-main")
            .bg(cx.theme().muted)
            .text_color(cx.theme().foreground)
            .flex()
            .items_center()
            .justify_center()
            .h_full()
            .w_full()
            .child(
                div().w_full().max_w(px(672.0)).mx_auto().p_6().child(
                    GroupBox::new()
                        .outline()
                        .content_style(
                            StyleRefinement::default()
                                .p_6()
                                .bg(cx.theme().background),
                        )
                        .child(
                            div()
                                .text_2xl()
                                .text_center()
                                .font_weight(FontWeight::SEMIBOLD)
                                .mb_1()
                                .child("irgen"),
                        )
                        .child(
                            div()
                                .text_sm()
                                .text_center()
                                .text_color(cx.theme().muted_foreground)
                                .mb_5()
                        )
                        .child(
                            div()
                                .id("file-upload")
                                .w_full()
                                .flex()
                                .flex_col()
                                .items_center()
                                .px_4()
                                .py_8()
                                .when_else(
                                    is_selected,
                                    |this| this.min_h(px(120.0)),
                                    |this| this.min_h(px(120.0)),
                                )
                                .mb_4()
                                .bg(cx.theme().background)
                                .text_color(blue_500())
                                .rounded(cx.theme().radius)
                                .border(px(1.0))
                                .border_dashed()
                                .border_color(cx.theme().border)
                                .hover(|this| {
                                    this.bg(blue_50())
                                        .text_color(blue_600())
                                        .border_color(blue_400())
                                })
                                .cursor_pointer()
                                .when_else(
                                    is_selected,
                                    |this| {
                                        let delete_button = Button::new("clear-selection")
                                            .text()
                                            .compact()
                                            .icon(IconName::Close)
                                            .text_color(cx.theme().muted_foreground)
                                            .on_click({
                                                let app_state = app_state.clone();
                                                move |_, _, cx| {
                                                    cx.stop_propagation();
                                                    app_state.clear_selection();
                                                    cx.notify(workspace_id);
                                                }
                                            });

                                        this.child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .items_center()
                                                .gap_2()
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap_2()
                                                        .child(
                                                            Icon::new(IconName::CircleCheck)
                                                                .with_size(px(20.0))
                                                                .text_color(cx.theme().success),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_sm()
                                                                .text_color(cx.theme().foreground)
                                                                .max_w(px(420.0))
                                                                .truncate()
                                                                .child(file_label.clone()),
                                                        )
                                                        .child(delete_button),
                                                )
                                                .when_some(summary.clone(), |this, summary| {
                                                    this.child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(
                                                                cx.theme().foreground.opacity(0.7),
                                                            )
                                                            .child(summary),
                                                    )
                                                }),
                                        )
                                    },
                                    |this| {
                                        let upload_icon = svg()
                                            .path("icons/excel.svg")
                                            .w_12()
                                            .h_12()
                                            .text_color(blue_500())
                                            .with_animation(
                                                "upload-breath",
                                                Animation::new(Duration::from_secs_f32(2.4))
                                                    .repeat()
                                                    .with_easing(pulsating_between(0.6, 1.0)),
                                                |this, delta| this.opacity(delta),
                                            );

                                        this.child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .items_center()
                                                .gap_2()
                                                .child(upload_icon)
                                                .child("Click to select a spreadsheet")
                                        )
                                    },
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
                                .flex()
                                .justify_center()
                                .gap_4()
                                .mt_2()
                                .child({
                                    let button = Button::new("button0")
                                        .w_48()
                                        .items_center()
                                        .label("Export IP-XACT")
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
                                        });
                                    if is_selected {
                                        button.primary().shadow_md().cursor_pointer()
                                    } else {
                                        button.outline()
                                    }
                                })
                                .child({
                                    let button = Button::new("button1")
                                        .w_48()
                                        .items_center()
                                        .label("Export RegVue")
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
                                        });
                                    if is_selected {
                                        button.primary().cursor_pointer()
                                    } else {
                                        button.outline()
                                    }
                                }),
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
