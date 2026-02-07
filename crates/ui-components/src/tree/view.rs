use crate::overlay_open;
use crate::tree::{
    TreeDensity, TreeNode, TreeStateInput,
    logic::{self, TreeTone},
};
use leptos::prelude::*;
use std::collections::BTreeSet;

#[component]
pub fn Tree(
    id_base: String,
    nodes: Vec<TreeNode>,
    #[prop(optional)] tone: TreeTone,
    #[prop(optional)] density: TreeDensity,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] expanded_ids: Option<Signal<BTreeSet<String>>>,
    #[prop(optional)] default_expanded_ids: Option<BTreeSet<String>>,
    #[prop(optional)] on_expanded_change: Option<Callback<BTreeSet<String>>>,
    #[prop(optional)] selected_id: Option<Signal<Option<String>>>,
    #[prop(optional)] default_selected_id: Option<String>,
    #[prop(optional)] on_selected_change: Option<Callback<Option<String>>>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let nodes = logic::normalize_nodes(nodes);
    let all_ids = logic::collect_all_ids(&nodes);
    let expandable_ids = logic::collect_expandable_ids(&nodes);
    let total_node_count = logic::count_nodes(&nodes);

    let default_expanded_ids =
        logic::sanitize_expanded_ids(default_expanded_ids.unwrap_or_default(), &expandable_ids);

    let expanded_state = overlay_open::use_controllable_state(
        expanded_ids,
        Some(default_expanded_ids),
        on_expanded_change,
    );
    let expanded_ids = expanded_state.value;
    let request_expanded_change = expanded_state.request_change;

    let default_selected_id = logic::sanitize_selected_id(
        logic::normalize_optional_text(default_selected_id),
        &all_ids,
    );

    let selected_state = overlay_open::use_controllable_state(
        selected_id,
        Some(default_selected_id),
        on_selected_change,
    );
    let selected_id = selected_state.value;
    let request_selected_change = selected_state.request_change;

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let nodes = StoredValue::new(nodes);
    let all_ids = StoredValue::new(all_ids);
    let expandable_ids = StoredValue::new(expandable_ids);

    let visible_nodes = Memo::new(move |_| {
        let nodes = nodes.get_value();
        let expanded_ids =
            logic::sanitize_expanded_ids(expanded_ids.get(), &expandable_ids.get_value());
        let selected_id = logic::sanitize_selected_id(selected_id.get(), &all_ids.get_value());

        logic::flatten_visible_nodes(&nodes, &expanded_ids, selected_id.as_deref(), disabled)
    });

    let state = Memo::new(move |_| {
        let expanded_ids =
            logic::sanitize_expanded_ids(expanded_ids.get(), &expandable_ids.get_value());
        let selected_id = logic::sanitize_selected_id(selected_id.get(), &all_ids.get_value());

        logic::resolve_state(TreeStateInput {
            tone,
            density,
            disabled,
            node_count: total_node_count,
            visible_count: visible_nodes.get().len(),
            expanded_count: expanded_ids.len(),
            has_selection: selected_id.is_some(),
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <div
            id=id_base
            class=move || class.get()
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
            role="tree"
            aria-label=aria_label
        >
            <ul class="ui-tree__list" data-slot="tree-list">
                {move || {
                    let visible_nodes = visible_nodes.get();
                    visible_nodes
                        .into_iter()
                        .enumerate()
                        .map(|(index, node)| {
                            let node_id_for_selection = node.id.clone();
                            let node_id_for_toggle = node.id.clone();
                            let has_children = node.has_children;
                            let is_disabled = node.is_disabled;

                            let on_click = move |_| {
                                if disabled || is_disabled {
                                    return;
                                }

                                request_selected_change.run(Some(node_id_for_selection.clone()));

                                if has_children {
                                    let expanded_ids = logic::sanitize_expanded_ids(
                                        expanded_ids.get_untracked(),
                                        &expandable_ids.get_value(),
                                    );
                                    let next = logic::toggle_expanded(
                                        expanded_ids,
                                        &node_id_for_toggle,
                                        &expandable_ids.get_value(),
                                    );
                                    request_expanded_change.run(next);
                                }
                            };

                            let row_class = format!(
                                "ui-tree__item {} {}{}",
                                logic::depth_class_name(node.depth),
                                if node.has_children {
                                    "ui-tree__item--branch"
                                } else {
                                    "ui-tree__item--leaf"
                                },
                                if node.is_selected {
                                    " ui-tree__item--selected"
                                } else {
                                    ""
                                }
                            );

                            let chevron = if node.has_children {
                                if node.is_expanded { "▾" } else { "▸" }
                            } else {
                                "•"
                            };

                            view! {
                                <li class="ui-tree__node" data-slot="tree-node" data-node-id=node.id.clone()>
                                    <button
                                        type="button"
                                        class=row_class
                                        data-slot="tree-item"
                                        data-depth=node.depth.to_string()
                                        data-expanded=node.is_expanded.then_some("true")
                                        data-selected=node.is_selected.then_some("true")
                                        data-disabled=node.is_disabled.then_some("true")
                                        data-branch=node.has_children.then_some("true")
                                        data-leaf=(!node.has_children).then_some("true")
                                        role="treeitem"
                                        attr:aria-level=(node.depth + 1).to_string()
                                        aria-expanded=if node.has_children {
                                            Some(if node.is_expanded { "true" } else { "false" })
                                        } else {
                                            None
                                        }
                                        aria-selected=if node.is_selected { Some("true") } else { Some("false") }
                                        aria-disabled=node.is_disabled.then_some("true")
                                        tabindex=if node.is_selected || index == 0 { 0 } else { -1 }
                                        disabled=node.is_disabled
                                        on:click=on_click
                                    >
                                        <span class="ui-tree__chevron" data-slot="tree-chevron" aria-hidden="true">{chevron}</span>
                                        <span class="ui-tree__label" data-slot="tree-label">{node.label}</span>
                                    </button>
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>
        </div>
    }
}
