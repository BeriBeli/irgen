use std::sync::Arc;

use gpui::prelude::*;
use gpui::*;
use gpui_component::{
    ActiveTheme as _, Disableable as _,
    button::{Button, ButtonVariants as _, DropdownButton},
    menu::PopupMenuItem,
};

use crate::processing::{export_ipxact_xml, export_regvue_json};
use crate::state::{AppState, ExportFormat};
use crate::ui::workspace::actions::save;

#[derive(IntoElement)]
pub struct WorkspaceFooter {
    app_state: Arc<AppState>,
    workspace_id: EntityId,
}

impl WorkspaceFooter {
    pub fn new(app_state: Arc<AppState>, workspace_id: EntityId) -> Self {
        Self {
            app_state,
            workspace_id,
        }
    }
}

impl RenderOnce for WorkspaceFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let app_state = self.app_state.clone();
        let workspace_id = self.workspace_id;

        let is_selected = app_state.is_file_selected();
        let export_format = app_state.get_export_format();
        let export_label = match export_format {
            ExportFormat::Ipxact => "IP-XACT",
            ExportFormat::Regvue => "RegVue",
        };

        div()
            .id("workspace-footer")
            .flex()
            .items_center()
            .justify_center()
            .pt_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .w_full()
                    .max_w(px(560.0))
                    .px_4()
                    .py_2()
                    .rounded(cx.theme().radius)
                    .border_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().background)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .px_2()
                            .py_1()
                            .rounded(px(6.0))
                            .border_1()
                            .border_color(cx.theme().border)
                            .bg(cx.theme().muted)
                            .child(
                                DropdownButton::new("export-format")
                                    .button(
                                        Button::new("export-format-label")
                                            .label(export_label)
                                            .items_center(),
                                    )
                                    .ghost()
                                    .compact()
                                    .disabled(!is_selected)
                                    .dropdown_menu({
                                        let app_state = app_state.clone();
                                        move |menu, _, _cx| {
                                            let app_state_ipxact = app_state.clone();
                                            let app_state_regvue = app_state.clone();
                                            let workspace_id = workspace_id;
                                            menu.item(PopupMenuItem::label("Format"))
                                                .item(PopupMenuItem::separator())
                                                .item(
                                                    PopupMenuItem::new("IP-XACT")
                                                        .checked(
                                                            export_format
                                                                == ExportFormat::Ipxact,
                                                        )
                                                        .on_click(move |_, _, cx| {
                                                            app_state_ipxact
                                                                .set_export_format(
                                                                    ExportFormat::Ipxact,
                                                                );
                                                            cx.notify(workspace_id);
                                                        }),
                                                )
                                                .item(
                                                    PopupMenuItem::new("RegVue")
                                                        .checked(
                                                            export_format
                                                                == ExportFormat::Regvue,
                                                        )
                                                        .on_click(move |_, _, cx| {
                                                            app_state_regvue
                                                                .set_export_format(
                                                                    ExportFormat::Regvue,
                                                                );
                                                            cx.notify(workspace_id);
                                                        }),
                                                )
                                        }
                                    }),
                            ),
                    )
                    .child({
                        let button = Button::new("export-button")
                            .items_center()
                            .label("Export")
                            .compact()
                            .disabled(!is_selected)
                            .on_click({
                                let app_state = app_state.clone();
                                move |_, window, cx| {
                                    let export_fn = match app_state.get_export_format() {
                                        ExportFormat::Ipxact => export_ipxact_xml,
                                        ExportFormat::Regvue => export_regvue_json,
                                    };
                                    save(app_state.clone(), export_fn, window, cx)
                                }
                            });
                        if is_selected {
                            button.primary().shadow_md().cursor_pointer()
                        } else {
                            button.outline()
                        }
                    }),
            )
    }
}
