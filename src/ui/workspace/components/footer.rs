use gpui::prelude::*;
use gpui::*;
use gpui_component::{
    ActiveTheme as _, Disableable as _,
    button::{Button, ButtonCustomVariant, ButtonVariants as _, DropdownButton},
    green_500,
    menu::PopupMenuItem,
    white,
};

use crate::processing::{export_ipxact_xml, export_regvue_json};
use crate::global::{ExportFormat, GlobalState};
use crate::ui::workspace::actions::save;

pub struct WorkspaceFooter {
}

impl WorkspaceFooter {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let _ = cx;
        Self {}
    }
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
}

impl Render for WorkspaceFooter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = GlobalState::global(cx);
        let workspace_id = state.workspace_id();

        let is_selected = state.is_file_selected();
        let export_format = state.get_export_format();
        let export_label = match export_format {
            ExportFormat::Ipxact => "IP-XACT",
            ExportFormat::Regvue => "RegVue",
        };

        div()
            .id("workspace-footer")
            .flex()
            .items_center()
            .justify_between()
            .pt(px(24.0))
            .w_full()
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
                                let workspace_id = workspace_id;
                                move |menu, _, _cx| {
                                    menu.item(PopupMenuItem::label("Format"))
                                        .item(PopupMenuItem::separator())
                                        .item(
                                            PopupMenuItem::new("IP-XACT")
                                                .checked(export_format == ExportFormat::Ipxact)
                                                .on_click(move |_, _, cx| {
                                                    GlobalState::global(cx)
                                                        .set_export_format(ExportFormat::Ipxact);
                                                    if let Some(workspace_id) = workspace_id {
                                                        cx.notify(workspace_id);
                                                    }
                                                }),
                                        )
                                        .item(
                                            PopupMenuItem::new("RegVue")
                                                .checked(export_format == ExportFormat::Regvue)
                                                .on_click(move |_, _, cx| {
                                                    GlobalState::global(cx)
                                                        .set_export_format(ExportFormat::Regvue);
                                                    if let Some(workspace_id) = workspace_id {
                                                        cx.notify(workspace_id);
                                                    }
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
                        move |_, window, cx| {
                            let export_fn = match GlobalState::global(cx).get_export_format() {
                                ExportFormat::Ipxact => export_ipxact_xml,
                                ExportFormat::Regvue => export_regvue_json,
                            };
                            save(export_fn, window, cx)
                        }
                    });
                if is_selected {
                    button
                        .custom(
                            ButtonCustomVariant::new(cx)
                                .color(green_500())
                                .foreground(white())
                                .hover(green_500().opacity(0.9))
                                .active(green_500().opacity(0.8)),
                        )
                        .shadow_md()
                        .cursor_pointer()
                } else {
                    button.outline()
                }
            })
    }
}
