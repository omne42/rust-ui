use crate::ai_space::{AiRenderMode, use_ai_space_state};
use crate::tree::{
    TreeDensity, TreeMotion, TreeNode, TreeStateInput, TreeVisibleNode,
    logic::{self, TreeTone},
    motion,
};
use leptos::{html, prelude::*};
use std::collections::BTreeSet;
use ui_headless::{
    A11yDirection, CommonStrings, TreeItemA11yInput, TreeItemOptions, tree_root_attrs,
    use_controllable_state, use_tree_item, use_ui_i18n,
};

const TREE_CHEVRON_EXPANDED: &str = "▾";
const TREE_CHEVRON_COLLAPSED: &str = "▸";
const TREE_CHEVRON_LEAF: &str = "•";
const TREE_NODE_SLOT: &str = "tree-node";
const TREE_ITEM_SLOT: &str = "tree-item";
const TREE_CHEVRON_SLOT: &str = "tree-chevron";
const TREE_LABEL_SLOT: &str = "tree-label";
const TREE_ITEM_CLASS_BASE: &str = "ui-tree__item";
const TREE_ITEM_CLASS_BRANCH: &str = "ui-tree__item--branch";
const TREE_ITEM_CLASS_LEAF: &str = "ui-tree__item--leaf";
const TREE_ITEM_CLASS_SELECTED: &str = "ui-tree__item--selected";

#[derive(Clone)]
struct TreeRenderContext {
    is_tree_disabled: bool,
    expanded_ids: Signal<BTreeSet<String>>,
    expandable_ids: StoredValue<BTreeSet<String>>,
    request_selected_change: Callback<Option<String>>,
    request_expanded_change: Callback<BTreeSet<String>>,
}

fn tree_item_row_class(depth: usize, has_children: bool, is_selected: bool) -> String {
    let selected_suffix = if is_selected {
        format!(" {TREE_ITEM_CLASS_SELECTED}")
    } else {
        String::new()
    };

    format!(
        "{TREE_ITEM_CLASS_BASE} {} {}{}",
        logic::depth_class_name(depth),
        if has_children {
            TREE_ITEM_CLASS_BRANCH
        } else {
            TREE_ITEM_CLASS_LEAF
        },
        selected_suffix
    )
}

fn tree_item_chevron(has_children: bool, is_expanded: bool) -> &'static str {
    if has_children {
        if is_expanded {
            TREE_CHEVRON_EXPANDED
        } else {
            TREE_CHEVRON_COLLAPSED
        }
    } else {
        TREE_CHEVRON_LEAF
    }
}

fn render_tree_node(
    index: usize,
    node: TreeVisibleNode,
    context: TreeRenderContext,
) -> impl IntoView {
    let node_id_for_selection = node.id.clone();
    let node_id_for_toggle = node.id.clone();

    let on_select = {
        let request_selected_change = context.request_selected_change;
        Callback::new(move |_| {
            request_selected_change.run(Some(node_id_for_selection.clone()));
        })
    };

    let on_toggle = if node.has_children {
        let request_expanded_change = context.request_expanded_change;
        let expanded_ids = context.expanded_ids;
        let expandable_ids = context.expandable_ids;
        Some(Callback::new(move |_| {
            let next = logic::resolve_expanded_toggle_request(
                expanded_ids.get_untracked(),
                &node_id_for_toggle,
                &expandable_ids.get_value(),
            );
            request_expanded_change.run(next);
        }))
    } else {
        None
    };

    let tree_item = use_tree_item(
        TreeItemA11yInput {
            depth: node.depth,
            has_children: node.has_children,
            is_expanded: node.is_expanded,
            is_selected: node.is_selected,
            is_disabled: node.is_disabled,
            is_tree_disabled: context.is_tree_disabled,
            is_first_visible: index == 0,
        },
        TreeItemOptions {
            on_select,
            on_toggle,
        },
    );

    let row_class = tree_item_row_class(node.depth, node.has_children, node.is_selected);
    let chevron = tree_item_chevron(node.has_children, node.is_expanded);

    view! {
        <li class="ui-tree__node" data-slot=TREE_NODE_SLOT data-node-id=node.id.clone()>
            <button
                type="button"
                class=row_class
                data-slot=TREE_ITEM_SLOT
                data-depth=node.depth.to_string()
                data-expanded=node.is_expanded.then_some("true")
                data-selected=node.is_selected.then_some("true")
                data-disabled=node.is_disabled.then_some("true")
                data-branch=node.has_children.then_some("true")
                data-leaf=(!node.has_children).then_some("true")
                role=tree_item.attrs.role
                attr:aria-level=tree_item.attrs.aria_level.to_string()
                aria-expanded=tree_item.attrs.aria_expanded
                aria-selected=Some(tree_item.attrs.aria_selected)
                aria-disabled=tree_item.attrs.aria_disabled
                tabindex=tree_item.attrs.tabindex
                disabled=!tree_item.state.is_interactive
                on:click=move |_| tree_item.handlers.on_click.run(())
                on:keydown=move |ev| {
                    if tree_item.handlers.on_key_down.run(ev.key()) {
                        ev.prevent_default();
                    }
                }
            >
                <span class="ui-tree__chevron" data-slot=TREE_CHEVRON_SLOT aria-hidden="true">{chevron}</span>
                <span class="ui-tree__label" data-slot=TREE_LABEL_SLOT>{node.label}</span>
            </button>
        </li>
    }
}

fn render_tree_list(
    visible_nodes: Vec<TreeVisibleNode>,
    context: TreeRenderContext,
) -> impl IntoView {
    visible_nodes
        .into_iter()
        .enumerate()
        .map(|(index, node)| render_tree_node(index, node, context.clone()))
        .collect_view()
}

#[component]
pub fn Tree(
    id_base: String,
    nodes: Vec<TreeNode>,
    #[prop(optional)] tone: TreeTone,
    #[prop(optional)] density: TreeDensity,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] expanded_ids: Option<Signal<BTreeSet<String>>>,
    #[prop(optional)] default_expanded_ids: Option<BTreeSet<String>>,
    #[prop(optional)] on_expanded_ids_change: Option<Callback<BTreeSet<String>>>,
    #[prop(optional)] on_expanded_change: Option<Callback<BTreeSet<String>>>,
    #[prop(optional)] selected_id: Option<Signal<Option<String>>>,
    #[prop(optional)] default_selected_id: Option<String>,
    #[prop(optional)] on_selected_id_change: Option<Callback<Option<String>>>,
    #[prop(optional)] on_selected_change: Option<Callback<Option<String>>>,
    #[prop(optional)] motion: TreeMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let is_disabled = logic::normalize_is_disabled(is_disabled, disabled);
    let nodes = logic::normalize_nodes(nodes);
    let all_ids = logic::collect_all_ids(&nodes);
    let expandable_ids = logic::collect_expandable_ids(&nodes);
    let total_node_count = logic::count_nodes(&nodes);

    let (agent_source, set_agent_source) = signal(logic::TreeAgentSource::Init);

    let expanded_axis = logic::normalize_expanded_axis(
        logic::TreeExpandedAxisInput {
            is_controlled: expanded_ids.is_some(),
            default_expanded_ids,
            on_expanded_ids_change,
            on_expanded_change,
        },
        &expandable_ids,
    );
    let expanded_mode_attr = expanded_axis.control_mode.as_attr();
    let default_expanded_source_attr = expanded_axis.default_source.as_attr();
    let expanded_change_source_attr = expanded_axis.change_source.as_attr();

    let expanded_state = use_controllable_state(
        expanded_ids,
        Some(expanded_axis.default_expanded_ids),
        expanded_axis.on_expanded_change,
    );
    let expanded_ids = expanded_state.value;
    let request_expanded_change = {
        let request_change = expanded_state.request_change;
        Callback::new(move |next| {
            set_agent_source.update(|source| *source = logic::TreeAgentSource::Pointer);
            request_change.run(next);
        })
    };

    let selected_axis = logic::normalize_selected_axis(
        logic::TreeSelectedAxisInput {
            is_controlled: selected_id.is_some(),
            default_selected_id,
            on_selected_id_change,
            on_selected_change,
        },
        &all_ids,
    );
    let selected_mode_attr = selected_axis.control_mode.as_attr();
    let default_selected_source_attr = selected_axis.default_source.as_attr();
    let selected_change_source_attr = selected_axis.change_source.as_attr();

    let selected_state = use_controllable_state(
        selected_id,
        Some(selected_axis.default_selected_id),
        selected_axis.on_selected_change,
    );
    let selected_id = selected_state.value;
    let request_selected_change = {
        let request_change = selected_state.request_change;
        Callback::new(move |next| {
            set_agent_source.update(|source| *source = logic::TreeAgentSource::Pointer);
            request_change.run(next);
        })
    };

    let i18n = use_ui_i18n();
    let common_strings = i18n.strings::<CommonStrings>();
    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label_with_fallback(
        aria_label,
        common_strings.tree_aria_label.as_ref(),
    );
    let root_a11y = tree_root_attrs(aria_label, lang, dir);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != TreeMotion::default();

    let nodes = StoredValue::new(nodes);
    let all_ids = StoredValue::new(all_ids);
    let expandable_ids = StoredValue::new(expandable_ids);

    let derived_state = Memo::new(move |_| {
        logic::derive_state(logic::TreeDerivedStateInput {
            nodes: nodes.get_value(),
            expanded_ids: expanded_ids.get(),
            selected_id: selected_id.get(),
            expandable_ids: expandable_ids.get_value(),
            all_ids: all_ids.get_value(),
            is_disabled,
        })
    });

    let state = Memo::new(move |_| {
        let derived_state = derived_state.get();

        logic::resolve_state(TreeStateInput {
            tone,
            density,
            disabled: is_disabled,
            node_count: total_node_count,
            visible_count: derived_state.visible_nodes.len(),
            expanded_count: derived_state.expanded_ids.len(),
            has_selection: derived_state.selected_id.is_some(),
            has_custom_aria_label,
            has_custom_class_name,
        })
    });
    let agent_contract =
        Memo::new(move |_| logic::resolve_agent_contract(state.get(), agent_source.get()));
    let ai_space_state = StoredValue::new(use_ai_space_state());
    let stream_mode = Memo::new(move |_| {
        ai_space_state
            .get_value()
            .map(|state| {
                if state.get().mode == AiRenderMode::Streaming {
                    logic::TreeStreamMode::Streaming
                } else {
                    logic::TreeStreamMode::Snapshot
                }
            })
            .unwrap_or(logic::TreeStreamMode::Snapshot)
    });

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    let expanded_for_motion = Signal::derive(move || state.get().expanded_count > 0);
    let inline_style = StoredValue::new(Some({
        let is_expanded = state.get_untracked().expanded_count > 0;
        let (scale, opacity) = motion::resolve_motion_css_vars(is_expanded, motion);
        format!("--ui-tree-motion-scale:{scale};--ui-tree-motion-opacity:{opacity};")
    }));
    let render_context = TreeRenderContext {
        is_tree_disabled: is_disabled,
        expanded_ids,
        expandable_ids,
        request_selected_change,
        request_expanded_change,
    };
    motion::attach_motion(root_ref, expanded_for_motion, motion);

    view! {
        <div
            id=id_base
            node_ref=root_ref
            class=move || logic::compose_class_name(class_name.get_value(), state.get())
            style=inline_style.get_value().unwrap_or_default()
            data-slot="tree"
            data-tone=move || state.get().tone_attr
            data-density=move || state.get().density_attr
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-node-count=move || state.get().node_count.to_string()
            data-visible-count=move || state.get().visible_count.to_string()
            data-expanded-count=move || state.get().expanded_count.to_string()
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-expanded-mode=expanded_mode_attr
            data-selected-mode=selected_mode_attr
            data-default-expanded-source=default_expanded_source_attr
            data-default-selected-source=default_selected_source_attr
            data-expanded-change-source=expanded_change_source_attr
            data-selected-change-source=selected_change_source_attr
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
            data-ui-stream-mode=move || stream_mode.get().as_str()
            data-ui-output-status=move || {
                ai_space_state
                    .get_value()
                    .map(|state| state.get().output_status.as_str())
                    .unwrap_or(agent_contract.get().output_status.as_str())
            }
            data-ui-capability-expand=move || {
                agent_contract.get().capabilities.can_expand.then_some("true")
            }
            data-ui-capability-select=move || {
                agent_contract.get().capabilities.can_select.then_some("true")
            }
            data-ui-capability-external-sync=move || {
                agent_contract
                    .get()
                    .capabilities
                    .can_external_sync
                    .then_some("true")
            }
            data-ui-capability-programmatic-replay=move || {
                agent_contract
                    .get()
                    .capabilities
                    .can_programmatic_replay
                    .then_some("true")
            }
            role=root_a11y.role
            aria-label=root_a11y.aria_label
            lang=root_a11y.lang
            dir=root_a11y.dir
        >
            <ul class="ui-tree__list" data-slot="tree-list">
                {move || {
                    let derived_state = derived_state.get();
                    render_tree_list(derived_state.visible_nodes, render_context.clone())
                }}
            </ul>
        </div>
    }
}
