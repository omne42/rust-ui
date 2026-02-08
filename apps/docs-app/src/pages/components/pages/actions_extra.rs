use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::collections::BTreeSet;
use ui_components::{
    ActionBar, ActionBarMotion, ActionBarPosition, ActionButton, ActionGroup, ActionGroupItem,
    ActionGroupSelectionMode, ActionGroupTone,
};

pub(super) fn action_bar() -> AnyView {
    let (selected_count, set_selected_count) = signal(0_usize);
    let selected_count_signal = Signal::derive(move || selected_count.get());

    let clear_selection = Callback::new(move |_| set_selected_count.set(0));

    let code = r#"let (selected_count, set_selected_count) = signal(3_usize);
let selected_count_signal = Signal::derive(move || selected_count.get());

<ActionBar
  selected_count=selected_count_signal
  on_clear_selection=Callback::new(move |_| set_selected_count.set(0))
>
  <ActionButton>"Delete"</ActionButton>
  <ActionButton is_quiet=true>"Archive"</ActionButton>
</ActionBar>"#;

    let state_code = r#"<ActionBar
  selected_count=Signal::derive(move || selected_count.get())
  position=ActionBarPosition::Top
  force_visible=true
  selection_text="Rows selected".to_string()
  clear_label="Clear all".to_string()
  motion=ActionBarMotion::disabled()
/>"#;

    view! {
        <ComponentPage
            title="ActionBar"
            slug="action-bar"
            group="Actions"
            description="Bulk-action surface with Spectrum-style selection contracts and HeroUI-grade spring visibility motion."
        >
            <Playground title="Selection + clear action" code=code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_selected_count.update(|count| *count = count.saturating_add(1));
                            })
                        >
                            "Select +1"
                        </ui_components::Button>
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Outline
                            on_press=Callback::new(move |_| {
                                set_selected_count.update(|count| *count = count.saturating_sub(1));
                            })
                        >
                            "Select -1"
                        </ui_components::Button>
                        <span class="ui-muted">
                            "selected: " {move || selected_count.get().to_string()}
                        </span>
                    </div>

                    <ActionBar
                        selected_count=selected_count_signal
                        on_clear_selection=clear_selection
                        aria_label="Bulk actions".to_string()
                        class_name="docs-action-bar".to_string()
                    >
                        <ActionButton>"Delete"</ActionButton>
                        <ActionButton is_quiet=true>"Archive"</ActionButton>
                    </ActionBar>
                </div>
            </Playground>

            <Playground title="Top placement + custom text + reduced motion" code=state_code>
                <div class="docs-stack">
                    <ActionBar
                        selected_count=selected_count_signal
                        position=ActionBarPosition::Top
                        force_visible=true
                        selection_text="Rows selected".to_string()
                        clear_label="Clear all".to_string()
                        motion=ActionBarMotion::disabled()
                    >
                        <ActionButton is_quiet=true>"Tag"</ActionButton>
                        <ActionButton is_quiet=true>"Assign"</ActionButton>
                    </ActionBar>
                    <span class="ui-muted">
                        "Top placement + custom labels + motion disabled."
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn action_group() -> AnyView {
    let items = vec![
        ActionGroupItem::new("align-left", "Align Left"),
        ActionGroupItem::new("align-center", "Align Center"),
        ActionGroupItem::new("align-right", "Align Right"),
        ActionGroupItem::new("align-justify", "Justify").disabled(true),
    ];

    let (selected_ids, set_selected_ids) = signal(BTreeSet::from(["align-left".to_string()]));
    let (last_action, set_last_action) = signal("none".to_string());

    let on_selected_change = Callback::new(move |next: BTreeSet<String>| {
        set_selected_ids.set(next);
    });

    let on_action = Callback::new(move |id: String| {
        set_last_action.set(id);
    });

    let items_primary = items.clone();
    let items_secondary = items;

    let code = r#"let items = vec![
  ActionGroupItem::new("align-left", "Align Left"),
  ActionGroupItem::new("align-center", "Align Center"),
  ActionGroupItem::new("align-right", "Align Right"),
];

<ActionGroup
  id_base="text-align".to_string()
  items=items
  selected_ids=selected_ids
  on_selected_change=on_selected_change
  on_action=on_action
/>"#;

    let states_code = r#"<ActionGroup
  id_base="text-style".to_string()
  items=items
  selection_mode=ActionGroupSelectionMode::Multiple
  default_selected_ids=BTreeSet::from(["align-left".to_string(), "align-center".to_string()])
  tone=ActionGroupTone::Strong
  class_name="docs-action-group-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="ActionGroup"
            slug="action-group"
            group="Actions"
            description="Selectable action cluster with centralized selection normalization and Spectrum-style state/source data contracts."
        >
            <Playground title="Single Selection + Action Callback" code=code>
                <div class="docs-stack">
                    <ActionGroup
                        id_base="docs-action-group-single".to_string()
                        items=items_primary
                        selected_ids=selected_ids
                        on_selected_change=on_selected_change
                        on_action=on_action
                    />
                    <span class="ui-muted">
                        "selected: " {move || selected_ids.get().iter().cloned().collect::<Vec<_>>().join(", ")}
                        " · last action: " {move || last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Multiple + Strong Tone" code=states_code>
                <ActionGroup
                    id_base="docs-action-group-multiple".to_string()
                    items=items_secondary
                    selection_mode=ActionGroupSelectionMode::Multiple
                    default_selected_ids=BTreeSet::from([
                        "align-left".to_string(),
                        "align-center".to_string(),
                    ])
                    tone=ActionGroupTone::Strong
                    class_name="docs-action-group-custom".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
