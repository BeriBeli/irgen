use gpui::prelude::*;
use gpui::*;
use gpui_component::{
    ActiveTheme as _, Icon, IconName, Sizable as _,
    button::{Button, ButtonCustomVariant, ButtonVariants as _},
    green_500,
};

use crate::global::GlobalState;

use super::style::{file_info_card, info_pill};

pub struct WorkspaceFileUploadSelected {}

impl WorkspaceFileUploadSelected {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let _ = cx;
        Self {}
    }
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for WorkspaceFileUploadSelected {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = GlobalState::global(cx);
        let workspace_id = state.workspace_id();
        let selected_file = state.get_selected_file();
        let selected_name = selected_file
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();
        let file_size = state
            .get_selected_file_size()
            .map(format_bytes)
            .unwrap_or_default();
        let sheet_count = state.get_sheet_count();
        let register_count = state.component().map(|compo| {
            compo
                .blks()
                .iter()
                .map(|blk| blk.regs().len())
                .sum::<usize>()
        });

        let delete_button = Button::new("clear-selection")
            .custom(
                ButtonCustomVariant::new(cx)
                    .foreground(cx.theme().muted_foreground)
                    .hover(cx.theme().danger_hover)
                    .active(cx.theme().danger_active),
            )
            .compact()
            .icon(IconName::Close)
            .on_click({
                let workspace_id = workspace_id;
                move |_, _, cx| {
                    cx.stop_propagation();
                    GlobalState::global(cx).clear_selection();
                    if let Some(workspace_id) = workspace_id {
                        cx.notify(workspace_id);
                    }
                }
            });

        let content = div()
            .flex()
            .flex_col()
            .gap_4()
            .justify_center()
            .h_full()
            .child(
                div()
                    .flex()
                    .items_start()
                    .gap_3()
                    .child(
                        Icon::new(Icon::empty())
                            .path("icons/excel.svg")
                            .w_12()
                            .h_12()
                            .text_color(green_500()),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_base()
                                    .font_family("monospace")
                                    .text_color(cx.theme().foreground)
                                    .truncate()
                                    .child(selected_name),
                            )
                            .when(!file_size.is_empty(), |this| {
                                this.child(
                                    div()
                                        .text_xs()
                                        .text_color(cx.theme().muted_foreground)
                                        .child(file_size),
                                )
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .text_color(cx.theme().foreground)
                    .when_some(sheet_count, |this, sheets| {
                        this.child(
                            info_pill(cx)
                                .child(
                                    Icon::new(IconName::File)
                                        .with_size(px(12.0))
                                        .text_color(green_500()),
                                )
                                .child(format!("{} Sheets", sheets)),
                        )
                    })
                    .when_some(register_count, |this, registers| {
                        this.child(
                            info_pill(cx)
                                .child(
                                    Icon::new(Icon::empty())
                                        .path("icons/cpu.svg")
                                        .with_size(px(12.0))
                                        .text_color(green_500()),
                                )
                                .child(format!("{} Registers", registers)),
                        )
                    }),
            );

        file_info_card(cx)
            .items_center()
            .justify_center()
            .child(content)
            .child(
                div()
                    .absolute()
                    .top_0()
                    .right_0()
                    .mt(px(10.0))
                    .mr(px(10.0))
                    .child(delete_button),
            )
    }
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
