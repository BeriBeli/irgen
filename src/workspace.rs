mod action;
mod title_bar;

use action::{open, save};
use title_bar::AppTitleBar;

use crate::services::{export_ipxact_xml, export_regvue_json, load_excel};
use crate::state::{AppState, ExportFormat};
use gpui::prelude::*;
use gpui::*;
use gpui_component::{notification::NotificationType, Disableable as _, Root, WindowExt as _};
use std::sync::Arc;

use gpui_component::{
    ActiveTheme as _,
    Icon, IconName, Sizable as _,
    button::{Button, ButtonVariants as _, DropdownButton},
    menu::PopupMenuItem,
    green_500,
};
use std::time::Duration;
use std::path::Path;

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

fn is_supported_spreadsheet(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase()),
        Some(ext) if ext == "xlsx" || ext == "xlsm"
    )
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
        let sheet_count = app_state.get_sheet_count();
        let register_count = {
            app_state
                .component()
                .map(|compo| compo.blks().iter().map(|blk| blk.regs().len()).sum::<usize>())
        };
        let export_format = app_state.get_export_format();
        let export_label = match export_format {
            ExportFormat::Ipxact => "IP-XACT",
            ExportFormat::Regvue => "RegVue",
        };
        let main = div()
            .id("workspace-main")
            .bg(cx.theme().muted)
            .text_color(cx.theme().foreground)
            .flex()
            .flex_col()
            .h_full()
            .w_full()
            .px_8()
            .py_6()
            .child(
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
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .child("irgen"),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(cx.theme().muted_foreground)
                                    .child("Register Generation Tool"),
                            ),
                    ),
            )
            .child(
                div()
                    .id("workspace-body")
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .flex_grow()
                    .child(
                        div()
                            .id("file-upload")
                            .w_full()
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .px_4()
                            .py_8()
                            .h(px(260.0))
                            .text_color(cx.theme().muted_foreground)
                            .rounded(cx.theme().radius)
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
                            .drag_over::<ExternalPaths>(|style, _, _, cx| {
                                style
                                    .border(px(1.0))
                                    .border_dashed()
                                    .border_color(green_500())
                                    .bg(cx.theme().background)
                            })
                            .can_drop(|data, _, _| {
                                data.downcast_ref::<ExternalPaths>()
                                    .and_then(|paths| paths.paths().first())
                                    .map(is_supported_spreadsheet)
                                    .unwrap_or(false)
                            })
                            .on_drop({
                                let app_state = app_state.clone();
                                move |paths: &ExternalPaths, window, cx| {
                                    let Some(path) = paths.paths().first().cloned() else {
                                        return;
                                    };
                                    if !is_supported_spreadsheet(&path) {
                                        window.push_notification(
                                            (
                                                NotificationType::Error,
                                                SharedString::from(
                                                    "Only .xlsx or .xlsm files are supported.",
                                                ),
                                            ),
                                            cx,
                                        );
                                        return;
                                    }
                                    let handle = window.window_handle();
                                    let app_state = app_state.clone();
                                    let workspace_id = workspace_id;
                                    cx.spawn(async move |cx| {
                                        let result = load_excel(&path, app_state);
                                        let _ = cx.update_window(handle, |_, window, cx| {
                                            match result {
                                                Ok(_) => {
                                                    window.push_notification(
                                                        (
                                                            NotificationType::Success,
                                                            SharedString::from(
                                                                "File loaded successfully! Ready to export.",
                                                            ),
                                                        ),
                                                        cx,
                                                    );
                                                }
                                                Err(err) => {
                                                    window.push_notification(
                                                        (
                                                            NotificationType::Error,
                                                            SharedString::from(err.to_string()),
                                                        ),
                                                        cx,
                                                    );
                                                }
                                            }
                                            cx.notify(workspace_id);
                                        });
                                    })
                                    .detach();
                                }
                            })
                            .cursor_pointer()
                            .when_else(
                                is_selected,
                                |this| {
                                    let replace_button = Button::new("replace-file")
                                        .text()
                                        .compact()
                                        .label("Replace file")
                                        .on_click({
                                            let app_state = app_state.clone();
                                            move |_, window, cx| {
                                                cx.stop_propagation();
                                                open(app_state.clone(), load_excel, window, cx)
                                            }
                                        });
                                    let delete_button = Button::new("clear-selection")
                                        .text()
                                        .compact()
                                        .icon(IconName::Close)
                                        .text_color(cx.theme().muted_foreground)
                                        .ml_2()
                                        .on_click({
                                            let app_state = app_state.clone();
                                            move |_, _, cx| {
                                                cx.stop_propagation();
                                                app_state.clear_selection();
                                                cx.notify(workspace_id);
                                            }
                                        });
                                    let remove_button = Button::new("remove-file")
                                        .text()
                                        .compact()
                                        .label("Remove")
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
                                            .w_full()
                                            .max_w(px(560.0))
                                            .bg(cx.theme().background)
                                            .border_1()
                                            .border_color(cx.theme().border)
                                            .rounded(cx.theme().radius)
                                            .px_5()
                                            .py_4()
                                            .flex()
                                            .flex_col()
                                            .gap_4()
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_start()
                                                    .justify_between()
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
                                                                            .font_family(
                                                                                "monospace",
                                                                            )
                                                                            .text_color(
                                                                                cx.theme().foreground,
                                                                            )
                                                                            .truncate()
                                                                            .child(
                                                                                selected_name
                                                                                    .clone(),
                                                                            ),
                                                                    )
                                                                    .when(!file_size.is_empty(), |this| {
                                                                        this.child(
                                                                            div()
                                                                                .text_xs()
                                                                                .text_color(
                                                                                    cx.theme()
                                                                                        .muted_foreground,
                                                                                )
                                                                                .child(
                                                                                    file_size
                                                                                        .clone(),
                                                                                ),
                                                                        )
                                                                    }),
                                                            ),
                                                    )
                                                    .child(delete_button),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap_2()
                                                    .text_color(cx.theme().foreground)
                                                    .when_some(sheet_count, |this, sheets| {
                                                        this.child(
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
                                                                .child(
                                                                    Icon::new(IconName::File)
                                                                        .with_size(px(12.0))
                                                                        .text_color(green_500()),
                                                                )
                                                                .child(format!(
                                                                    "{} Sheets",
                                                                    sheets
                                                                )),
                                                        )
                                                    })
                                                    .when_some(register_count, |this, registers| {
                                                        this.child(
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
                                                                .child(
                                                                    Icon::new(
                                                                        IconName::SquareTerminal,
                                                                    )
                                                                    .with_size(px(12.0))
                                                                    .text_color(green_500()),
                                                                )
                                                                .child(format!(
                                                                    "{} Registers",
                                                                    registers
                                                                )),
                                                        )
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap_2()
                                                    .text_xs()
                                                    .text_color(cx.theme().muted_foreground)
                                                    .child(replace_button)
                                                    .child("/")
                                                    .child(remove_button),
                                            ),
                                    )
                                },
                                |this| {
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

                                    this.child(
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
                                            ),
                                    )
                                },
                            )
                            .on_click({
                                let app_state = app_state.clone();
                                move |_, window, cx| {
                                    open(app_state.clone(), load_excel, window, cx)
                                }
                            }),
                    ),
            )
            .child(
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
