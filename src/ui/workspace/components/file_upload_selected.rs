use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;
use gpui_component::{
    ActiveTheme as _, Icon, IconName, Sizable as _,
    button::{Button, ButtonCustomVariant, ButtonVariants as _},
    green_500,
};

use crate::state::AppState;

use super::style::{file_info_card, info_pill};

#[derive(IntoElement)]
pub struct WorkspaceFileUploadSelected {
    app_state: Arc<AppState>,
    workspace_id: EntityId,
    selected_name: String,
    file_size: String,
    sheet_count: Option<usize>,
    register_count: Option<usize>,
}

impl WorkspaceFileUploadSelected {
    pub fn new(
        app_state: Arc<AppState>,
        workspace_id: EntityId,
        selected_name: String,
        file_size: String,
        sheet_count: Option<usize>,
        register_count: Option<usize>,
    ) -> Self {
        Self {
            app_state,
            workspace_id,
            selected_name,
            file_size,
            sheet_count,
            register_count,
        }
    }
}

impl RenderOnce for WorkspaceFileUploadSelected {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let app_state = self.app_state.clone();
        let workspace_id = self.workspace_id;
        let selected_name = self.selected_name;
        let file_size = self.file_size;
        let sheet_count = self.sheet_count;
        let register_count = self.register_count;

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
                let app_state = app_state.clone();
                move |_, _, cx| {
                    cx.stop_propagation();
                    app_state.clear_selection();
                    cx.notify(workspace_id);
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
                        svg()
                            .path("icons/excel.svg")
                            .w_10()
                            .h_10()
                            .text_color(green_500()),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_sm()
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
                                    Icon::new(IconName::SquareTerminal)
                                        .with_size(px(12.0))
                                        .text_color(green_500()),
                                )
                                .child(format!("{} Registers", registers)),
                        )
                    }),
            );

        file_info_card(cx)
            .relative()
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
