use gpui_component::tree::TreeItem;
use irgen_core::processing::base;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegisterNodeKind {
    Component,
    Block {
        block_index: usize,
    },
    Register {
        block_index: usize,
        register_index: usize,
    },
    Field {
        block_index: usize,
        register_index: usize,
        field_index: usize,
    },
}

#[derive(Clone, Debug)]
struct Node {
    id: String,
    label: String,
    search_text: String,
    kind: RegisterNodeKind,
    children: Vec<Node>,
}

pub struct RegisterTreeBuild {
    pub items: Vec<TreeItem>,
    pub id_to_kind: HashMap<String, RegisterNodeKind>,
    pub visible_ids: Vec<String>,
}

pub fn build(component: &base::Component, query: &str) -> RegisterTreeBuild {
    let root = build_full_tree(component);
    let normalized = normalize(query);
    let filtered = if normalized.is_empty() {
        Some(root)
    } else {
        filter_node(&root, &normalized)
    };

    let mut id_to_kind = HashMap::new();
    let mut visible_ids = Vec::new();
    let mut items = Vec::new();
    let expand_all = !normalized.is_empty();

    if let Some(node) = filtered {
        items.push(to_tree_item(
            &node,
            0,
            expand_all,
            &mut id_to_kind,
            &mut visible_ids,
            true,
        ));
    }

    RegisterTreeBuild {
        items,
        id_to_kind,
        visible_ids,
    }
}

fn build_full_tree(component: &base::Component) -> Node {
    let mut root = Node {
        id: "component".to_string(),
        label: component.name().to_string(),
        search_text: format!(
            "{} {} {} {}",
            component.vendor(),
            component.library(),
            component.name(),
            component.version()
        ),
        kind: RegisterNodeKind::Component,
        children: Vec::new(),
    };

    for (block_index, block) in component.blks().iter().enumerate() {
        let mut block_node = Node {
            id: format!("block:{block_index}"),
            label: block.name().to_string(),
            search_text: format!("{} {} {} {}", block.name(), block.offset(), block.range(), block.size()),
            kind: RegisterNodeKind::Block { block_index },
            children: Vec::new(),
        };

        for (register_index, reg) in block.regs().iter().enumerate() {
            let mut register_node = Node {
                id: format!("register:{block_index}:{register_index}"),
                label: reg.name().to_string(),
                search_text: format!("{} {} {}", reg.name(), reg.offset(), reg.size()),
                kind: RegisterNodeKind::Register {
                    block_index,
                    register_index,
                },
                children: Vec::new(),
            };

            for (field_index, field) in reg.fields().iter().enumerate() {
                register_node.children.push(Node {
                    id: format!("field:{block_index}:{register_index}:{field_index}"),
                    label: field.name().to_string(),
                    search_text: format!(
                        "{} {} {} {} {} {}",
                        field.name(),
                        field.offset(),
                        field.width(),
                        field.attr(),
                        field.reset(),
                        field.desc()
                    ),
                    kind: RegisterNodeKind::Field {
                        block_index,
                        register_index,
                        field_index,
                    },
                    children: Vec::new(),
                });
            }

            block_node.children.push(register_node);
        }

        root.children.push(block_node);
    }

    root
}

fn filter_node(node: &Node, query: &str) -> Option<Node> {
    let child_matches = node
        .children
        .iter()
        .filter_map(|child| filter_node(child, query))
        .collect::<Vec<_>>();
    let self_match = normalize(&node.search_text).contains(query);

    if self_match || !child_matches.is_empty() {
        let mut out = node.clone();
        out.children = child_matches;
        Some(out)
    } else {
        None
    }
}

fn to_tree_item(
    node: &Node,
    depth: usize,
    expand_all: bool,
    id_to_kind: &mut HashMap<String, RegisterNodeKind>,
    visible_ids: &mut Vec<String>,
    ancestor_visible: bool,
) -> TreeItem {
    id_to_kind.insert(node.id.clone(), node.kind.clone());
    if ancestor_visible {
        visible_ids.push(node.id.clone());
    }

    let expanded = if expand_all {
        true
    } else {
        depth <= 1
    };

    TreeItem::new(node.id.clone(), node.label.clone())
        .expanded(expanded)
        .children(node.children.iter().map(|child| {
            to_tree_item(
                child,
                depth + 1,
                expand_all,
                id_to_kind,
                visible_ids,
                ancestor_visible && expanded,
            )
        }))
}

fn normalize(value: &str) -> String {
    value.to_ascii_lowercase()
}
