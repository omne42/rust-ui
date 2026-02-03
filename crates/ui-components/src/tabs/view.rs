use leptos::{children::ChildrenFragment as Children, ev, html, prelude::*};
use std::sync::Arc;
use ui_headless::{RovingOrientation, RovingTabIndexOptions, use_roving_tabindex};

#[cfg(target_arch = "wasm32")]
fn focus_tab(tab_refs: &Arc<Vec<NodeRef<html::Button>>>, index: usize) {
    let Some(node_ref) = tab_refs.get(index) else {
        return;
    };
    let Some(el) = node_ref.get_untracked() else {
        return;
    };
    let _ = el.focus();
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_tab(_tab_refs: &Arc<Vec<NodeRef<html::Button>>>, _index: usize) {}

#[component]
pub fn Tabs(labels: Vec<&'static str>, id_base: String, children: Children) -> impl IntoView {
    let panels = children().nodes;

    debug_assert_eq!(
        labels.len(),
        panels.iter().len(),
        "Tabs: expected `labels.len() == children.len()`; got labels={}, children={}",
        labels.len(),
        panels.iter().len()
    );

    let item_count = labels.len().min(panels.iter().len());
    let (item_count_signal, _set_item_count) = signal(item_count);

    let roving = use_roving_tabindex(RovingTabIndexOptions {
        is_disabled: false,
        default_index: 0,
        should_loop: true,
        orientation: RovingOrientation::Horizontal,
        item_count: item_count_signal,
        is_item_disabled: None,
    });

    let tab_refs: Arc<Vec<NodeRef<html::Button>>> =
        Arc::new((0..item_count).map(|_| NodeRef::new()).collect());

    let tabs_view = labels
        .into_iter()
        .take(item_count)
        .enumerate()
        .map({
            let id_base = id_base.clone();
            let roving = roving.clone();
            let tab_refs = tab_refs.clone();
            move |(index, label)| {
                let tab_id = format!("{id_base}-tab-{index}");
                let panel_id = format!("{id_base}-panel-{index}");
                let node_ref = tab_refs[index];

                let is_selected = move || roving.active_index.get() == index;

                let on_key_down = {
                    let on_key_down = roving.handlers.on_key_down;
                    let active_index = roving.active_index;
                    let tab_refs = tab_refs.clone();
                    move |ev: ev::KeyboardEvent| {
                        if on_key_down.run(ev.key()) {
                            ev.prevent_default();
                            focus_tab(&tab_refs, active_index.get_untracked());
                        }
                    }
                };

                view! {
                    <button
                        type="button"
                        class="ui-tabs__tab"
                        id=tab_id
                        node_ref=node_ref
                        role="tab"
                        tabindex=move || if is_selected() { 0 } else { -1 }
                        aria-selected=move || if is_selected() { "true" } else { "false" }
                        aria-controls=panel_id
                        data-active=move || if is_selected() { Some("true") } else { None }
                        on:focus=move |_| roving.handlers.on_item_focus.run(index)
                        on:click=move |_| roving.handlers.on_item_focus.run(index)
                        on:keydown=on_key_down
                    >
                        {label}
                    </button>
                }
            }
        })
        .collect_view();

    let panels_view = panels
        .into_iter()
        .take(item_count)
        .enumerate()
        .map({
            let id_base = id_base.clone();
            let roving = roving.clone();
            move |(index, panel)| {
                let tab_id = format!("{id_base}-tab-{index}");
                let panel_id = format!("{id_base}-panel-{index}");
                let is_selected = move || roving.active_index.get() == index;

                view! {
                    <div
                        class="ui-tabs__panel"
                        id=panel_id
                        role="tabpanel"
                        aria-labelledby=tab_id
                        hidden=move || !is_selected()
                    >
                        {panel}
                    </div>
                }
            }
        })
        .collect_view();

    view! {
        <div class="ui-tabs" data-slot="tabs">
            <div class="ui-tabs__list" role="tablist" data-slot="tabs-list">
                {tabs_view}
            </div>
            {panels_view}
        </div>
    }
}
