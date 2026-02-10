use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::collections::BTreeSet;
use ui_components::{
    ActionBar, ActionBarMotion, ActionBarPosition, ActionButton, ActionGroup, ActionGroupItem,
    ActionGroupSelectionMode, ActionGroupTone, ClearButton, CloseButton, CloseButtonSize,
    CloseButtonVariant, FieldButton, InfieldButton, LogicButton, LogicButtonVariant, Toggle,
    ToggleGroup, ToggleGroupItem, ToggleGroupOrientation, ToggleGroupSelectionMode, ToggleMotion,
    ToggleSize, ToggleVariant,
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

pub(super) fn field_button() -> AnyView {
    let default_code = r#"<FieldButton aria_label="Open options".to_string()>
  "Options"
</FieldButton>
<FieldButton quiet=true aria_label="Open calendar".to_string()>
  "📅"
</FieldButton>"#;

    let state_code = r#"<FieldButton
  invalid=true
  is_active=true
  aria_label="Invalid trigger".to_string()
  class_name="docs-field-button-custom".to_string()
>
  "Needs fix"
</FieldButton>
<FieldButton disabled=true aria_label="Disabled trigger".to_string()>
  "Disabled"
</FieldButton>"#;

    view! {
        <ComponentPage
            title="FieldButton"
            slug="field-button"
            group="Actions"
            description="Spectrum-style field trigger button with centralized quiet/invalid/active/disabled state contracts and headless press/hover/focus behavior."
        >
            <Playground title="Default + Quiet" code=default_code>
                <div class="docs-row">
                    <FieldButton aria_label="Open options".to_string()>
                        "Options"
                    </FieldButton>
                    <FieldButton quiet=true aria_label="Open calendar".to_string()>
                        "📅"
                    </FieldButton>
                </div>
            </Playground>

            <Playground title="Invalid + Active + Disabled" code=state_code>
                <div class="docs-row">
                    <FieldButton
                        invalid=true
                        is_active=true
                        aria_label="Invalid trigger".to_string()
                        class_name="docs-field-button-custom".to_string()
                    >
                        "Needs fix"
                    </FieldButton>
                    <FieldButton disabled=true aria_label="Disabled trigger".to_string()>
                        "Disabled"
                    </FieldButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn infield_button() -> AnyView {
    let default_code = r#"<InfieldButton aria_label="Open in-field options".to_string()>
  "⋯"
</InfieldButton>
<InfieldButton quiet=true aria_label="Open calendar".to_string()>
  "📅"
</InfieldButton>"#;

    let state_code = r#"<InfieldButton
  invalid=true
  is_active=true
  aria_label="Invalid in-field trigger".to_string()
  class_name="docs-infield-button-custom".to_string()
>
  "Needs fix"
</InfieldButton>
<InfieldButton disabled=true aria_label="Disabled in-field trigger".to_string()>
  "Disabled"
</InfieldButton>"#;

    view! {
        <ComponentPage
            title="InfieldButton"
            slug="infield-button"
            group="Actions"
            description="Spectrum-compatible in-field trigger button with centralized quiet/invalid/active/disabled state contracts and headless press/hover/focus behavior."
        >
            <Playground title="Default + Quiet" code=default_code>
                <div class="docs-row">
                    <InfieldButton aria_label="Open in-field options".to_string()>
                        "⋯"
                    </InfieldButton>
                    <InfieldButton quiet=true aria_label="Open calendar".to_string()>
                        "📅"
                    </InfieldButton>
                </div>
            </Playground>

            <Playground title="Invalid + Active + Disabled" code=state_code>
                <div class="docs-row">
                    <InfieldButton
                        invalid=true
                        is_active=true
                        aria_label="Invalid in-field trigger".to_string()
                        class_name="docs-infield-button-custom".to_string()
                    >
                        "Needs fix"
                    </InfieldButton>
                    <InfieldButton disabled=true aria_label="Disabled in-field trigger".to_string()>
                        "Disabled"
                    </InfieldButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn clear_button() -> AnyView {
    let basic_code = r#"<ClearButton aria_label="Clear query".to_string()>
  "×"
</ClearButton>
<ClearButton variant=ClearButtonVariant::OverBackground aria_label="Dismiss overlay".to_string()>
  "×"
</ClearButton>"#;

    let state_code = r#"<ClearButton
  inset=true
  prevent_focus=true
  aria_label="Clear token".to_string()
  class_name="docs-clear-button-custom".to_string()
>
  "×"
</ClearButton>
<ClearButton disabled=true exclude_from_tab_order=true aria_label="Disabled clear".to_string()>
  "×"
</ClearButton>"#;

    view! {
        <ComponentPage
            title="ClearButton"
            slug="clear-button"
            group="Actions"
            description="Spectrum-style clear affordance with centralized variant/inset/focus-mode normalization and stable state/source data contracts."
        >
            <Playground title="Default + OverBackground" code=basic_code>
                <div class="docs-row">
                    <ClearButton aria_label="Clear query".to_string()>
                        "×"
                    </ClearButton>
                    <ClearButton
                        variant=ui_components::ClearButtonVariant::OverBackground
                        aria_label="Dismiss overlay".to_string()
                    >
                        "×"
                    </ClearButton>
                </div>
            </Playground>

            <Playground title="Inset + Focus Mode + Disabled" code=state_code>
                <div class="docs-row">
                    <ClearButton
                        inset=true
                        prevent_focus=true
                        aria_label="Clear token".to_string()
                        class_name="docs-clear-button-custom".to_string()
                    >
                        "×"
                    </ClearButton>
                    <ClearButton
                        disabled=true
                        exclude_from_tab_order=true
                        aria_label="Disabled clear".to_string()
                    >
                        "×"
                    </ClearButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn close_button() -> AnyView {
    let basic_code = r#"<CloseButton />
<CloseButton variant=CloseButtonVariant::OverBackground />
<CloseButton aria_label="Dismiss popover".to_string() />"#;

    let state_code = r#"<CloseButton size=CloseButtonSize::Sm />
<CloseButton size=CloseButtonSize::Lg />
<CloseButton
  size=CloseButtonSize::Xl
  disabled=true
  class_name="docs-close-button-custom".to_string()
/>"#;

    view! {
        <ComponentPage
            title="CloseButton"
            slug="close-button"
            group="Actions"
            description="Spectrum/HeroUI-style close affordance with default icon fallback, centralized variant+size contracts, and stable state/source data markers."
        >
            <Playground title="Default + OverBackground + Custom Label" code=basic_code>
                <div class="docs-row">
                    <CloseButton />
                    <CloseButton variant=CloseButtonVariant::OverBackground />
                    <CloseButton aria_label="Dismiss popover".to_string() />
                </div>
            </Playground>

            <Playground title="Size Matrix + Disabled + Custom Class" code=state_code>
                <div class="docs-row">
                    <CloseButton size=CloseButtonSize::Sm />
                    <CloseButton size=CloseButtonSize::Lg />
                    <CloseButton
                        size=CloseButtonSize::Xl
                        disabled=true
                        class_name="docs-close-button-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn logic_button() -> AnyView {
    let basic_code = r#"<LogicButton variant=LogicButtonVariant::And>
  "AND"
</LogicButton>
<LogicButton variant=LogicButtonVariant::Or>
  "OR"
</LogicButton>"#;

    let state_code = r#"<LogicButton
  variant=LogicButtonVariant::And
  class_name="docs-logic-button-custom".to_string()
>
  "Custom"
</LogicButton>
<LogicButton variant=LogicButtonVariant::Or disabled=true>
  "Disabled"
</LogicButton>"#;

    view! {
        <ComponentPage
            title="LogicButton"
            slug="logic-button"
            group="Actions"
            description="Spectrum-style boolean operator button with centralized variant normalization, headless press/hover/focus behavior, and stable state/source data contracts."
        >
            <Playground title="AND + OR variants" code=basic_code>
                <div class="docs-row">
                    <LogicButton variant=LogicButtonVariant::And>
                        "AND"
                    </LogicButton>
                    <LogicButton variant=LogicButtonVariant::Or>
                        "OR"
                    </LogicButton>
                </div>
            </Playground>

            <Playground title="Custom class + Disabled" code=state_code>
                <div class="docs-row">
                    <LogicButton
                        variant=LogicButtonVariant::And
                        class_name="docs-logic-button-custom".to_string()
                    >
                        "Custom"
                    </LogicButton>
                    <LogicButton variant=LogicButtonVariant::Or disabled=true>
                        "Disabled"
                    </LogicButton>
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

pub(super) fn toggle() -> AnyView {
    let (pressed, set_pressed) = signal(false);
    let on_pressed_change = Callback::new(move |next: bool| set_pressed.set(next));

    let basic_code = r#"let (pressed, set_pressed) = signal(false);
let on_pressed_change = Callback::new(move |next: bool| set_pressed.set(next));

<Toggle
  pressed=pressed
  set_pressed=set_pressed
  on_pressed_change=on_pressed_change
>
  "Bold"
</Toggle>"#;

    let states_code = r#"<Toggle
  pressed=pressed
  set_pressed=set_pressed
  variant=ToggleVariant::Outline
  size=ToggleSize::Sm
>
  "Italic"
</Toggle>
<Toggle
  pressed=pressed
  set_pressed=set_pressed
  variant=ToggleVariant::Ghost
  disabled=true
>
  "Disabled"
</Toggle>"#;

    let markers_code = r##"<Toggle
  pressed=pressed
  set_pressed=set_pressed
  variant=ToggleVariant::Outline
  size=ToggleSize::Sm
  motion=ToggleMotion {
    tap_scale: 0.92,
    ..ToggleMotion::default()
  }
  class_name="docs-toggle-state".to_string()
  aria_label="Toggle formatting".to_string()
  on_pressed_change=on_pressed_change
>
  "Markers"
</Toggle>"##;

    view! {
        <ComponentPage
            title="Toggle"
            slug="toggle"
            group="Actions"
            description="Shadcn-compatible single toggle primitive with Spectrum-style press/focus contracts and HeroUI-grade spring press motion."
        >
            <Playground title="Controlled Toggle" code=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Toggle
                            pressed=pressed
                            set_pressed=set_pressed
                            on_pressed_change=on_pressed_change
                        >
                            "Bold"
                        </Toggle>
                        <span class="ui-muted">"pressed: " {move || pressed.get().to_string()}</span>
                    </div>
                </div>
            </Playground>

            <Playground title="Outline + Ghost + Disabled" code=states_code>
                <div class="docs-row">
                    <Toggle
                        pressed=pressed
                        set_pressed=set_pressed
                        variant=ToggleVariant::Outline
                        size=ToggleSize::Sm
                    >
                        "Italic"
                    </Toggle>
                    <Toggle
                        pressed=pressed
                        set_pressed=set_pressed
                        variant=ToggleVariant::Ghost
                        disabled=true
                    >
                        "Disabled"
                    </Toggle>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-interaction`, `data-variant-source`, `data-motion-source`, `data-aria-source`, and `data-handler-source` contracts."
                code=markers_code
            >
                <div class="docs-row">
                    <Toggle
                        pressed=pressed
                        set_pressed=set_pressed
                        variant=ToggleVariant::Outline
                        size=ToggleSize::Sm
                        motion=ToggleMotion {
                            tap_scale: 0.92,
                            ..ToggleMotion::default()
                        }
                        class_name="docs-toggle-state".to_string()
                        aria_label="Toggle formatting".to_string()
                        on_pressed_change=on_pressed_change
                    >
                        "Markers"
                    </Toggle>
                    <span class="ui-muted">"pressed: " {move || pressed.get().to_string()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn toggle_group() -> AnyView {
    let style_items = vec![
        ToggleGroupItem::new("bold", "Bold"),
        ToggleGroupItem::new("italic", "Italic"),
        ToggleGroupItem::new("underline", "Underline"),
    ];

    let (style_selected_raw, set_style_selected_raw) =
        signal(BTreeSet::from(["bold".to_string(), "italic".to_string()]));
    let style_selected: Signal<BTreeSet<String>> = Signal::derive(move || style_selected_raw.get());
    let on_style_selected_change =
        Callback::new(move |next: BTreeSet<String>| set_style_selected_raw.set(next));

    let alignment_items = vec![
        ToggleGroupItem::new("left", "Left"),
        ToggleGroupItem::new("center", "Center"),
        ToggleGroupItem::new("right", "Right").disabled(true),
    ];

    let (alignment_selected_raw, set_alignment_selected_raw) =
        signal(BTreeSet::from(["center".to_string()]));
    let alignment_selected: Signal<BTreeSet<String>> =
        Signal::derive(move || alignment_selected_raw.get());
    let on_alignment_selected_change =
        Callback::new(move |next: BTreeSet<String>| set_alignment_selected_raw.set(next));

    let code = r#"let (selected, set_selected) = signal(BTreeSet::from(["bold".to_string()]));
let selected_signal: Signal<BTreeSet<String>> = Signal::derive(move || selected.get());

<ToggleGroup
  id_base="formatting".to_string()
  items=items
  selected_ids=selected_signal
  on_selected_ids_change=Callback::new(move |next| set_selected.set(next))
  selection_mode=ToggleGroupSelectionMode::Multiple
  attached=true
/>"#;

    let states_code = r#"<ToggleGroup
  id_base="alignment".to_string()
  items=alignment_items
  selected_ids=alignment_selected
  on_selected_ids_change=on_alignment_selected_change
  selection_mode=ToggleGroupSelectionMode::Single
  orientation=ToggleGroupOrientation::Vertical
  attached=false
  aria_label="Alignment controls".to_string()
/>"#;

    view! {
        <ComponentPage
            title="ToggleGroup"
            slug="toggle-group"
            group="Actions"
            description="Shadcn-compatible grouped toggle primitive with controlled selection modes and Spectrum-style root state contracts."
        >
            <Playground title="Multiple + Attached" code=code>
                <div class="docs-stack docs-stack--tight">
                    <ToggleGroup
                        id_base="docs-toggle-group-formatting".to_string()
                        items=style_items
                        selected_ids=style_selected
                        on_selected_ids_change=on_style_selected_change
                        selection_mode=ToggleGroupSelectionMode::Multiple
                        attached=true
                    />
                    <span class="ui-muted">
                        "selected ids: "
                        {move || {
                            style_selected_raw
                                .get()
                                .iter()
                                .cloned()
                                .collect::<Vec<_>>()
                                .join(", ")
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Single + Vertical + Disabled Item" code=states_code>
                <div class="docs-stack docs-stack--tight">
                    <ToggleGroup
                        id_base="docs-toggle-group-alignment".to_string()
                        items=alignment_items
                        selected_ids=alignment_selected
                        on_selected_ids_change=on_alignment_selected_change
                        selection_mode=ToggleGroupSelectionMode::Single
                        orientation=ToggleGroupOrientation::Vertical
                        attached=false
                        aria_label="Alignment controls".to_string()
                        class_name="docs-toggle-group-custom".to_string()
                    />
                    <span class="ui-muted">
                        "alignment selected: "
                        {move || {
                            alignment_selected_raw
                                .get()
                                .iter()
                                .cloned()
                                .collect::<Vec<_>>()
                                .join(", ")
                        }}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
