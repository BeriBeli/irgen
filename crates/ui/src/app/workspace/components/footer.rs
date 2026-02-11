use gpui::prelude::*;
use gpui::*;
use gpui_component::{
    Disableable as _,
    button::{Button, ButtonCustomVariant, ButtonVariants as _},
    green_500,
    menu::{DropdownMenu as _, PopupMenuItem},
    white,
};
use irgen_core::processing::{
    export_c_header, export_html, export_ipxact_xml, export_regvue_json, export_sv_rtl,
    export_uvm_ral,
};

use crate::global::{ExportFormat, GlobalState};
use crate::app::workspace::actions::save;

pub struct WorkspaceFooter {}

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

        let is_selected = state.is_file_selected();
        let export_format = state.get_export_format();
        let export_label = match export_format {
            ExportFormat::Ipxact => "IP-XACT",
            ExportFormat::Regvue => "RegVue",
            ExportFormat::CHeader => "C Header",
            ExportFormat::UvmRal => "UVM RAL",
            ExportFormat::Rtl => "Verilog RTL",
            ExportFormat::Html => "HTML",
        };

        div()
            .id("workspace-footer")
            .flex()
            .items_center()
            .justify_between()
            .pt(px(24.0))
            .w_full()
            .child({
                div().flex().items_center().gap_2().child(
                    Button::new("export-format")
                        .label(export_label)
                        .items_center()
                        .dropdown_caret(true)
                        .compact()
                        .outline()
                        .disabled(!is_selected)
                        .text_xs()
                        .dropdown_menu({
                            move |menu, _, _cx| {
                                menu.item(PopupMenuItem::label("Format"))
                                    .item(PopupMenuItem::separator())
                                    .item(
                                        PopupMenuItem::new("IP-XACT")
                                            .checked(export_format == ExportFormat::Ipxact)
                                            .on_click(move |_, _, cx| {
                                                GlobalState::global(cx)
                                                    .set_export_format(ExportFormat::Ipxact);
                                                GlobalState::notify_workspaces(cx);
                                            }),
                                    )
                                    .item(
                                        PopupMenuItem::new("RegVue")
                                            .checked(export_format == ExportFormat::Regvue)
                                            .on_click(move |_, _, cx| {
                                                GlobalState::global(cx)
                                                    .set_export_format(ExportFormat::Regvue);
                                                GlobalState::notify_workspaces(cx);
                                            }),
                                    )
                                    .item(
                                        PopupMenuItem::new("C Header")
                                            .checked(export_format == ExportFormat::CHeader)
                                            .on_click(move |_, _, cx| {
                                                GlobalState::global(cx)
                                                    .set_export_format(ExportFormat::CHeader);
                                                GlobalState::notify_workspaces(cx);
                                            }),
                                    )
                                    .item(
                                        PopupMenuItem::new("UVM RAL")
                                            .checked(export_format == ExportFormat::UvmRal)
                                            .on_click(move |_, _, cx| {
                                                GlobalState::global(cx)
                                                    .set_export_format(ExportFormat::UvmRal);
                                                GlobalState::notify_workspaces(cx);
                                            }),
                                    )
                                    .item(
                                        PopupMenuItem::new("Verilog RTL")
                                            .checked(export_format == ExportFormat::Rtl)
                                            .on_click(move |_, _, cx| {
                                                GlobalState::global(cx)
                                                    .set_export_format(ExportFormat::Rtl);
                                                GlobalState::notify_workspaces(cx);
                                            }),
                                    )
                                    .item(
                                        PopupMenuItem::new("HTML")
                                            .checked(export_format == ExportFormat::Html)
                                            .on_click(move |_, _, cx| {
                                                GlobalState::global(cx)
                                                    .set_export_format(ExportFormat::Html);
                                                GlobalState::notify_workspaces(cx);
                                            }),
                                    )
                            }
                        }),
                )
            })
            .child({
                let button = Button::new("export-button")
                    .items_center()
                    .w_24()
                    .label("Export")
                    .compact()
                    .disabled(!is_selected)
                    .on_click({
                        move |_, window, cx| {
                            let export_fn = match GlobalState::global(cx).get_export_format() {
                                ExportFormat::Ipxact => export_ipxact_xml,
                                ExportFormat::Regvue => export_regvue_json,
                                ExportFormat::CHeader => export_c_header,
                                ExportFormat::UvmRal => export_uvm_ral,
                                ExportFormat::Rtl => export_sv_rtl,
                                ExportFormat::Html => export_html,
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
                } else {
                    button.outline()
                }
            })
    }
}
