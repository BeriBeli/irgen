use gpui::prelude::*;
use gpui::*;
use gpui_component::{
    ActiveTheme as _, Icon, IconName, Sizable as _,
    button::Button,
    green_500,
    input::{Input, InputEvent, InputState},
    list::ListItem,
    notification::NotificationType,
    scroll::ScrollableElement as _,
    tree::{TreeItem, TreeState, tree},
};
use irgen_core::processing::load_excel;
use std::collections::HashMap;

use crate::app::workspace::actions::open;
use crate::app::workspace::notifications as workspace_notifications;
use crate::global::GlobalState;

use super::register_detail::WorkspaceRegisterDetail;
use super::register_tree::{self, RegisterNodeKind};

pub struct WorkspaceRegisterExplorer {
    search_input: Entity<InputState>,
    tree_state: Entity<TreeState>,
    filter_query: String,
    id_to_kind: HashMap<String, RegisterNodeKind>,
    visible_ids: Vec<String>,
    last_file_key: Option<String>,
    register_detail: Entity<WorkspaceRegisterDetail>,
    _search_subscription: Subscription,
    _tree_observer: Subscription,
    _global_observer: Subscription,
}

impl WorkspaceRegisterExplorer {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder("Search"));
        let tree_state = cx.new(|cx| TreeState::new(cx).items(Vec::<TreeItem>::new()));

        let _search_subscription =
            cx.subscribe_in(&search_input, window, Self::on_search_input_event);
        let _tree_observer = cx.observe(&tree_state, |this, _, cx| {
            this.sync_detail_selection(cx);
            cx.notify();
        });
        let _global_observer = cx.observe_global::<GlobalState>(|this, cx| {
            this.sync_with_loaded_file(cx);
        });
        let register_detail = WorkspaceRegisterDetail::view(window, cx);

        let mut this = Self {
            search_input,
            tree_state,
            filter_query: String::new(),
            id_to_kind: HashMap::new(),
            visible_ids: Vec::new(),
            last_file_key: None,
            register_detail,
            _search_subscription,
            _tree_observer,
            _global_observer,
        };
        this.rebuild_tree(cx);
        this
    }

    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn on_search_input_event(
        &mut self,
        state: &Entity<InputState>,
        event: &InputEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, InputEvent::Change) {
            self.filter_query = state.read(cx).value().to_string();
            self.rebuild_tree(cx);
        }
    }

    fn sync_with_loaded_file(&mut self, cx: &mut Context<Self>) {
        let file_key = GlobalState::global(cx)
            .get_selected_file()
            .as_ref()
            .map(|path| path.to_string_lossy().to_string());

        if self.last_file_key != file_key {
            self.last_file_key = file_key;
            self.rebuild_tree(cx);
        }
    }

    fn rebuild_tree(&mut self, cx: &mut Context<Self>) {
        let current_selected_id = self
            .tree_state
            .read(cx)
            .selected_entry()
            .map(|entry| entry.item().id.to_string());

        let Some(component) = GlobalState::global(cx).component() else {
            self.id_to_kind.clear();
            self.visible_ids.clear();
            self.tree_state.update(cx, |tree, cx| {
                tree.set_items(Vec::<TreeItem>::new(), cx);
                tree.set_selected_index(None, cx);
            });
            self.sync_detail_selection(cx);
            cx.notify();
            return;
        };

        let built = register_tree::build(component.as_ref(), &self.filter_query);
        let next_selected_ix = current_selected_id
            .as_ref()
            .and_then(|id| {
                built
                    .visible_ids
                    .iter()
                    .position(|candidate| candidate == id)
            })
            .or_else(|| (!built.visible_ids.is_empty()).then_some(0));

        self.id_to_kind = built.id_to_kind;
        self.visible_ids = built.visible_ids;

        self.tree_state.update(cx, |tree, cx| {
            tree.set_items(built.items, cx);
            tree.set_selected_index(next_selected_ix, cx);
        });

        self.sync_detail_selection(cx);
        cx.notify();
    }

    fn sync_detail_selection(&mut self, cx: &mut Context<Self>) {
        let selected_kind = self.selected_kind(cx);
        self.register_detail.update(cx, |detail, cx| {
            detail.set_selected(selected_kind, cx);
        });
    }

    fn selected_kind(&self, cx: &App) -> Option<RegisterNodeKind> {
        self.tree_state
            .read(cx)
            .selected_entry()
            .and_then(|entry| self.id_to_kind.get(entry.item().id.as_ref()))
            .cloned()
    }
}

impl Render for WorkspaceRegisterExplorer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = GlobalState::global(cx);
        let selected_name = state
            .get_selected_file()
            .as_ref()
            .and_then(|path| path.file_name())
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "No file selected".to_string());

        if state.component().is_none() {
            return div().id("workspace-register-explorer").h_full().w_full();
        }

        let replace_button = Button::new("replace-excel")
            .label("Switch")
            .icon(IconName::Replace)
            .compact()
            .outline()
            .w(px(132.0))
            .on_click(|_, window, cx| {
                cx.stop_propagation();
                open(load_excel, window, cx);
            });

        let clear_button = Button::new("clear-excel")
            .label("Remove")
            .icon(IconName::Close)
            .compact()
            .outline()
            .w(px(132.0))
            .on_click(|_, window, cx| {
                cx.stop_propagation();
                GlobalState::clear_selection(cx);
                workspace_notifications::push(
                    window,
                    cx,
                    NotificationType::Success,
                    "Selection cleared.",
                );
            });

        let tree_panel = div()
            .id("register-tree-panel")
            .h_full()
            .min_h_0()
            .min_w_0()
            .flex_basis(relative(0.4))
            .flex()
            .flex_col()
            .gap_3()
            .child(
                Input::new(&self.search_input)
                    .prefix(
                        Icon::new(IconName::Search)
                            .with_size(px(14.0))
                            .text_color(cx.theme().muted_foreground),
                    )
                    .cleanable(true),
            )
            .child(
                div()
                    .min_h_0()
                    .flex_1()
                    .bg(cx.theme().background)
                    .border_1()
                    .border_color(cx.theme().border)
                    .rounded(cx.theme().radius)
                    .p_2()
                    .child({
                        if self.visible_ids.is_empty() {
                            div()
                                .h_full()
                                .w_full()
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_sm()
                                .text_color(cx.theme().muted_foreground)
                                .child("No matching nodes.")
                                .into_any_element()
                        } else {
                            div()
                                .size_full()
                                .overflow_x_scrollbar()
                                .child(
                                    tree(&self.tree_state, |ix, entry, selected, _, cx| {
                                        let depth_padding = px((entry.depth() as f32) * 14.0);
                                        let node_icon =
                                            node_icon(entry.item().id.as_ref(), entry.is_expanded());
                                        let disclose = disclosure_icon(
                                            entry.is_folder(),
                                            entry.is_expanded(),
                                            cx,
                                        );

                                        ListItem::new(ix)
                                            .pl(depth_padding)
                                            .selected(selected)
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_shrink_0()
                                                    .items_center()
                                                    .gap_2()
                                                    .child(disclose)
                                                    .child(node_icon)
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(cx.theme().foreground)
                                                            .whitespace_nowrap()
                                                            .child(entry.item().label.clone()),
                                                    ),
                                            )
                                    })
                                    .size_full(),
                                )
                                .into_any_element()
                        }
                    }),
            );

        div()
            .id("workspace-register-explorer")
            .h_full()
            .min_h_0()
            .w_full()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Icon::new(Icon::empty())
                                    .path("icons/excel.svg")
                                    .with_size(px(18.0))
                                    .text_color(green_500()),
                            )
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::MEDIUM)
                                    .font_family("monospace")
                                    .text_color(cx.theme().foreground)
                                    .child(selected_name),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(replace_button)
                            .child(clear_button),
                    ),
            )
            .child(
                div()
                    .min_h_0()
                    .flex_1()
                    .w_full()
                    .min_w_0()
                    .flex()
                    .gap_4()
                    .child(tree_panel)
                    .child(
                        div()
                            .id("register-detail-panel")
                            .min_h_0()
                            .min_w_0()
                            .flex_basis(relative(0.6))
                            .flex()
                            .flex_col()
                            .child(self.register_detail.clone()),
                    ),
            )
    }
}

fn disclosure_icon(is_folder: bool, expanded: bool, cx: &App) -> AnyElement {
    if is_folder {
        let icon = if expanded {
            IconName::ChevronDown
        } else {
            IconName::ChevronRight
        };
        Icon::new(icon)
            .with_size(px(12.0))
            .text_color(cx.theme().muted_foreground)
            .into_any_element()
    } else {
        div().w(px(12.0)).h(px(12.0)).into_any_element()
    }
}

fn node_icon(id: &str, expanded: bool) -> AnyElement {
    if id == "component" {
        Icon::new(IconName::Frame)
            .with_size(px(13.0))
            .into_any_element()
    } else if id.starts_with("block:") {
        let icon = if expanded {
            IconName::FolderOpen
        } else {
            IconName::FolderClosed
        };
        Icon::new(icon).with_size(px(13.0)).into_any_element()
    } else if id.starts_with("register:") {
        Icon::new(Icon::empty())
            .path("icons/cpu.svg")
            .with_size(px(13.0))
            .into_any_element()
    } else {
        Icon::new(IconName::Asterisk)
            .with_size(px(12.0))
            .into_any_element()
    }
}
