use gpui::prelude::*;
use gpui::*;
use gpui_component::{ActiveTheme as _, scroll::ScrollableElement as _};

use crate::global::GlobalState;

use super::register_tree::RegisterNodeKind;

pub struct WorkspaceRegisterDetail {
    selected: Option<RegisterNodeKind>,
}

impl WorkspaceRegisterDetail {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self { selected: None }
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn set_selected(&mut self, selected: Option<RegisterNodeKind>, cx: &mut Context<Self>) {
        if self.selected != selected {
            self.selected = selected;
            cx.notify();
        }
    }
}

impl Render for WorkspaceRegisterDetail {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(component) = GlobalState::global(cx).component() else {
            return detail_container(cx).child(center_message(cx, "No component loaded."));
        };

        let mut container = detail_container(cx);
        match self.selected.as_ref() {
            Some(RegisterNodeKind::Component) => {
                let block_count = component.blks().len();
                let register_count = component
                    .blks()
                    .iter()
                    .map(|block| block.regs().len())
                    .sum::<usize>();
                let field_count = component
                    .blks()
                    .iter()
                    .flat_map(|block| block.regs())
                    .map(|reg| reg.fields().len())
                    .sum::<usize>();

                container = container
                    .child(title(cx, "Component"))
                    .child(detail_scroll(rows(
                        cx,
                        vec![
                            ("Name", component.name().to_string()),
                            ("Vendor", component.vendor().to_string()),
                            ("Library", component.library().to_string()),
                            ("Version", component.version().to_string()),
                            ("Blocks", block_count.to_string()),
                            ("Registers", register_count.to_string()),
                            ("Fields", field_count.to_string()),
                        ],
                    )));
            }
            Some(RegisterNodeKind::Block { block_index }) => {
                let Some(block) = component.blks().get(*block_index) else {
                    return detail_container(cx).child(center_message(cx, "Block not found."));
                };

                container = container.child(title(cx, "Block")).child(detail_scroll(rows(
                    cx,
                    vec![
                        ("Name", block.name().to_string()),
                        ("Offset", block.offset().to_string()),
                        ("Range", block.range().to_string()),
                        ("Size", block.size().to_string()),
                        ("Registers", block.regs().len().to_string()),
                    ],
                )));
            }
            Some(RegisterNodeKind::Register {
                block_index,
                register_index,
            }) => {
                let Some(block) = component.blks().get(*block_index) else {
                    return detail_container(cx).child(center_message(cx, "Block not found."));
                };
                let Some(reg) = block.regs().get(*register_index) else {
                    return detail_container(cx).child(center_message(cx, "Register not found."));
                };

                container = container
                    .child(title(cx, "Register"))
                    .child(detail_scroll(rows(
                        cx,
                        vec![
                            ("Name", reg.name().to_string()),
                            ("Block", block.name().to_string()),
                            ("Offset", reg.offset().to_string()),
                            ("Size", reg.size().to_string()),
                            ("Fields", reg.fields().len().to_string()),
                        ],
                    )));
            }
            Some(RegisterNodeKind::Field {
                block_index,
                register_index,
                field_index,
            }) => {
                let Some(block) = component.blks().get(*block_index) else {
                    return detail_container(cx).child(center_message(cx, "Block not found."));
                };
                let Some(reg) = block.regs().get(*register_index) else {
                    return detail_container(cx).child(center_message(cx, "Register not found."));
                };
                let Some(field) = reg.fields().get(*field_index) else {
                    return detail_container(cx).child(center_message(cx, "Field not found."));
                };

                container = container.child(title(cx, "Field")).child(detail_scroll(rows(
                    cx,
                    vec![
                        ("Name", field.name().to_string()),
                        ("Register", reg.name().to_string()),
                        ("Block", block.name().to_string()),
                        ("Bit Offset", field.offset().to_string()),
                        ("Width", field.width().to_string()),
                        ("Attribute", field.attr().to_string()),
                        ("Reset", field.reset().to_string()),
                        ("Description", field.desc().to_string()),
                    ],
                )));
            }
            None => {
                container = container.child(center_message(
                    cx,
                    "Select a node in the tree to inspect details.",
                ));
            }
        }

        container
    }
}

fn detail_container(cx: &App) -> Stateful<Div> {
    div()
        .id("register-detail")
        .min_h_0()
        .h_full()
        .w_full()
        .bg(cx.theme().background)
        .border_1()
        .border_color(cx.theme().border)
        .rounded(cx.theme().radius)
        .px_4()
        .py_4()
        .flex()
        .flex_col()
        .gap_4()
}

fn detail_scroll(content: AnyElement) -> AnyElement {
    div()
        .min_h_0()
        .flex_1()
        .overflow_y_scrollbar()
        .pr_1()
        .child(content)
        .into_any_element()
}

fn title(cx: &App, value: impl Into<SharedString>) -> AnyElement {
    div()
        .text_lg()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(cx.theme().foreground)
        .child(value.into())
        .into_any_element()
}

fn rows(cx: &App, data: Vec<(&'static str, String)>) -> AnyElement {
    let mut body = div().w_full().flex().flex_col().gap_2();
    for (label, value) in data {
        body = body.child(
            div()
                .w_full()
                .flex()
                .items_start()
                .gap_4()
                .child(
                    div()
                        .w(px(112.0))
                        .flex_shrink_0()
                        .text_sm()
                        .text_color(cx.theme().muted_foreground)
                        .child(label),
                )
                .child(
                    div()
                        .min_w_0()
                        .flex_1()
                        .text_sm()
                        .text_color(cx.theme().foreground)
                        .text_left()
                        .whitespace_normal()
                        .line_height(relative(1.3))
                        .font_family("monospace")
                        .child(value),
                ),
        );
    }
    body.into_any_element()
}

fn center_message(cx: &App, message: impl Into<SharedString>) -> AnyElement {
    div()
        .h_full()
        .w_full()
        .flex()
        .items_center()
        .justify_center()
        .text_color(cx.theme().muted_foreground)
        .child(message.into())
        .into_any_element()
}
