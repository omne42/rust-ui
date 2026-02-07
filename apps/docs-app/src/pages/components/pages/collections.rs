use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::collections::BTreeSet;
use std::sync::Arc;
use ui_components::{
    Accordion, AccordionSelectionMode, Autocomplete, BreadcrumbItem, Breadcrumbs, ComboBox,
    Disclosure, DropdownMenu, ListBox, Menu, MenuItemKind, MenuTrigger, Pagination, Select, Tabs,
    TabsKeyboardActivation, Tag, TagGroup,
};

pub(super) fn breadcrumbs() -> AnyView {
    let items = vec![
        BreadcrumbItem {
            label: "Home".to_string(),
            href: Some("#/docs/welcome".to_string()),
        },
        BreadcrumbItem {
            label: "Components".to_string(),
            href: Some("#/components".to_string()),
        },
        BreadcrumbItem {
            label: "Breadcrumbs".to_string(),
            href: None,
        },
    ];

    let code = r#"let items = vec![
  BreadcrumbItem { label: "Home".to_string(), href: Some("/".to_string()) },
  BreadcrumbItem { label: "Breadcrumbs".to_string(), href: None },
];
<Breadcrumbs items=items />"#;

    view! {
        <ComponentPage
            title="Breadcrumbs"
            slug="breadcrumbs"
            group="Collections"
            description="A breadcrumb nav list with current-page semantics."
        >
            <Playground title="Trail" code=code>
                <Breadcrumbs items=items />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn accordion() -> AnyView {
    let labels = vec![
        "First".to_string(),
        "Second".to_string(),
        "Third".to_string(),
    ];
    let code = r#"<Accordion labels=labels id_base="acc".to_string()>
  <div>"Panel 1"</div>
  <div>"Panel 2"</div>
  <div>"Panel 3"</div>
</Accordion>"#;

    view! {
        <ComponentPage
            title="Accordion"
            slug="accordion"
            group="Collections"
            description="Multi-panel disclosure with roving tabindex and spring motion."
        >
            <Playground title="Multiple panels" code=code>
                <Accordion
                    labels=labels
                    id_base="docs-accordion".to_string()
                    selection_mode=AccordionSelectionMode::Multiple
                    default_open_indices=BTreeSet::from([0_usize])
                >
                    <div class="docs-stack">
                        <div>"Panel 1 content"</div>
                        <div class="ui-muted">"Press Enter/Space or click the triggers."</div>
                    </div>
                    <div class="docs-stack">
                        <div>"Panel 2 content"</div>
                        <div class="ui-muted">"Arrow keys move focus between triggers."</div>
                    </div>
                    <div class="docs-stack">
                        <div>"Panel 3 content"</div>
                        <div class="ui-muted">"Supports disabled indices."</div>
                    </div>
                </Accordion>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn disclosure() -> AnyView {
    let code = r#"<Disclosure id_base="disc".to_string() label="Details".to_string()>
  <div>"Hidden content"</div>
</Disclosure>"#;

    view! {
        <ComponentPage
            title="Disclosure"
            slug="disclosure"
            group="Collections"
            description="Single disclosure panel with animated indicator and panel motion."
        >
            <Playground title="Disclosure" code=code>
                <Disclosure id_base="docs-disclosure".to_string() label="Details".to_string() default_open=true>
                    <div class="docs-stack">
                        <div>"Hidden content"</div>
                        <div class="ui-muted">"Uses the same open-state contract as overlays."</div>
                    </div>
                </Disclosure>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn tabs() -> AnyView {
    let labels = vec!["Overview", "Details", "Settings"];
    let code = r#"<Tabs labels=vec!["Overview", "Details"] id_base="tabs".to_string()>
  <div>"Panel 1"</div>
  <div>"Panel 2"</div>
</Tabs>"#;

    view! {
        <ComponentPage
            title="Tabs"
            slug="tabs"
            group="Collections"
            description="Tabs with roving tabindex + Spectrum semantics."
        >
            <Playground title="Tabs" code=code>
                <Tabs
                    labels=labels
                    id_base="docs-tabs".to_string()
                    keyboard_activation=TabsKeyboardActivation::Automatic
                >
                    <div class="docs-stack">
                        <div>"Overview"</div>
                        <div class="ui-muted">"Arrow keys navigate tabs."</div>
                    </div>
                    <div class="docs-stack">
                        <div>"Details"</div>
                        <div class="ui-muted">"Enter/Space activates in manual mode."</div>
                    </div>
                    <div class="docs-stack">
                        <div>"Settings"</div>
                        <div class="ui-muted">"Disabled tabs are skipped."</div>
                    </div>
                </Tabs>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn list_box() -> AnyView {
    let items: Arc<[String]> = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Durian".to_string(),
    ]
    .into();
    let disabled_items: Arc<[String]> = vec![
        "London".to_string(),
        "Paris".to_string(),
        "Tokyo".to_string(),
    ]
    .into();
    let empty_items: Arc<[String]> = Vec::<String>::new().into();

    let (selected, set_selected) = signal(None::<usize>);
    let (disabled_selected, set_disabled_selected) = signal(None::<usize>);
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let code = r#"let items: Arc<[String]> = vec!["Apple".to_string(), "Banana".to_string()].into();
let (selected, set_selected) = signal(None::<usize>);
<ListBox
  id_base="fruit".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
  aria_label="Fruit".to_string()
  disabled_indices=vec![1]
/>"#;

    let states_code = r#"<ListBox
  id_base="cities-disabled".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
  aria_label="Disabled list".to_string()
  disabled=true
/>
<ListBox
  id_base="cities-empty".to_string()
  items=Vec::<String>::new().into()
  selected_index=selected
  set_selected_index=set_selected
  aria_label="Empty list".to_string()
/>"#;

    view! {
        <ComponentPage
            title="ListBox"
            slug="listbox"
            group="Collections"
            description="Listbox with active highlight spring motion, typeahead, and Spectrum-style state attrs."
        >
            <Playground title="Selection + Typeahead" code=code>
                <div class="docs-stack">
                    <ListBox
                        id_base="docs-listbox".to_string()
                        items=items
                        selected_index=selected
                        set_selected_index=set_selected
                        aria_label="Fruit".to_string()
                        disabled_indices=vec![3]
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code=states_code>
                <div class="docs-row">
                    <ListBox
                        id_base="docs-listbox-disabled".to_string()
                        items=disabled_items
                        selected_index=disabled_selected
                        set_selected_index=set_disabled_selected
                        aria_label="Disabled city list".to_string()
                        disabled=true
                    />
                    <ListBox
                        id_base="docs-listbox-empty".to_string()
                        items=empty_items
                        selected_index=empty_selected
                        set_selected_index=set_empty_selected
                        aria_label="Empty city list".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn menu() -> AnyView {
    let items: Arc<[String]> = vec![
        "New file".to_string(),
        "Share with team".to_string(),
        "Sort ascending".to_string(),
    ]
    .into();
    let disabled_items: Arc<[String]> = vec![
        "Duplicate".to_string(),
        "Move".to_string(),
        "Archive".to_string(),
    ]
    .into();
    let empty_items: Arc<[String]> = Vec::<String>::new().into();

    let (last, set_last) = signal(None::<usize>);
    let (share_checked, set_share_checked) = signal(true);
    let (sort_ascending, set_sort_ascending) = signal(true);

    let on_action = Callback::new(move |index: usize| {
        set_last.set(Some(index));
        match index {
            1 => set_share_checked.update(|value| *value = !*value),
            2 => set_sort_ascending.update(|value| *value = !*value),
            _ => {}
        }
    });

    let noop_action = Callback::new(|_: usize| {});

    let code = r#"let items: Arc<[String]> = vec!["New file".to_string(), "Share with team".to_string()].into();
<Menu
  id_base="menu".to_string()
  items=items
  on_action=on_action
  aria_label="File actions".to_string()
  item_kinds=vec![
    MenuItemKind::Action,
    MenuItemKind::Checkbox { is_checked: Signal::derive(|| true) },
  ]
/>"#;

    let states_code = r#"<Menu
  id_base="menu-disabled".to_string()
  items=items
  on_action=noop_action
  aria_label="Disabled menu".to_string()
  disabled=true
/>
<Menu
  id_base="menu-empty".to_string()
  items=Vec::<String>::new().into()
  on_action=noop_action
  aria_label="Empty menu".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Menu"
            slug="menu"
            group="Collections"
            description="ARIA menu with action / checkbox / radio roles, active-highlight motion, and Spectrum-style state attrs."
        >
            <Playground title="Kinds + Selection" code=code>
                <div class="docs-stack">
                    <Menu
                        id_base="docs-menu".to_string()
                        items=items
                        on_action=on_action
                        aria_label="File actions".to_string()
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Checkbox {
                                is_checked: Signal::derive(move || share_checked.get()),
                            },
                            MenuItemKind::Radio {
                                is_checked: Signal::derive(move || sort_ascending.get()),
                            },
                        ]
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                    <span class="ui-muted">
                        "share checked: "
                        {move || share_checked.get().to_string()}
                        " · sort ascending: "
                        {move || sort_ascending.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code=states_code>
                <div class="docs-row">
                    <Menu
                        id_base="docs-menu-disabled".to_string()
                        items=disabled_items
                        on_action=noop_action
                        aria_label="Disabled menu".to_string()
                        disabled=true
                    />
                    <Menu
                        id_base="docs-menu-empty".to_string()
                        items=empty_items
                        on_action=noop_action
                        aria_label="Empty menu".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn menu_trigger() -> AnyView {
    let default_items = vec![
        "Profile".to_string(),
        "Settings".to_string(),
        "Log out".to_string(),
    ];
    let controlled_items = vec![
        "Rename".to_string(),
        "Duplicate".to_string(),
        "Archive".to_string(),
    ];
    let disabled_items = vec!["Copy".to_string(), "Move".to_string()];
    let empty_items: Vec<String> = Vec::new();

    let (last, set_last) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last.set(Some(index)));

    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let code = r#"<MenuTrigger id_base="trigger".to_string() items=items on_action=on_action>
  "Open menu"
</MenuTrigger>"#;

    let controlled_code = r#"let (open, set_open) = signal(false);
let open_signal: Signal<bool> = Signal::derive(move || open.get());
<MenuTrigger
  id_base="trigger-controlled".to_string()
  items=items
  on_action=on_action
  open=open_signal
  on_open_change=Callback::new(move |next| set_open.set(next))
>
  "Controlled"
</MenuTrigger>"#;

    let disabled_code = r#"<MenuTrigger id_base="trigger-disabled".to_string() items=items on_action=on_action disabled=true>
  "Disabled"
</MenuTrigger>
<MenuTrigger id_base="trigger-empty".to_string() items=Vec::<String>::new() on_action=on_action>
  "Empty"
</MenuTrigger>"#;

    view! {
        <ComponentPage
            title="MenuTrigger"
            slug="menu-trigger"
            group="Collections"
            description="Button trigger that opens a Popover-based Menu with controlled/uncontrolled state support."
        >
            <Playground title="Default" code=code>
                <div class="docs-row">
                    <MenuTrigger
                        id_base="docs-menu-trigger".to_string()
                        items=default_items
                        on_action=on_action
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    >
                        "Open menu"
                    </MenuTrigger>
                    <span class="ui-muted">
                        "last: "
                        {move || last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled Open State" code=controlled_code>
                <div class="docs-stack">
                    <MenuTrigger
                        id_base="docs-menu-trigger-controlled".to_string()
                        items=controlled_items
                        on_action=on_action
                        open=controlled_open
                        on_open_change=on_open_change
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    >
                        "Controlled"
                    </MenuTrigger>
                    <span class="ui-muted">
                        "open: "
                        {move || controlled_open_raw.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code=disabled_code>
                <div class="docs-row">
                    <MenuTrigger
                        id_base="docs-menu-trigger-disabled".to_string()
                        items=disabled_items
                        on_action=on_action
                        disabled=true
                        item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action]
                    >
                        "Disabled"
                    </MenuTrigger>

                    <MenuTrigger
                        id_base="docs-menu-trigger-empty".to_string()
                        items=empty_items
                        on_action=on_action
                    >
                        "Empty"
                    </MenuTrigger>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn select() -> AnyView {
    let items = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Durian".to_string(),
    ];
    let (selected, set_selected) = signal(Some(1_usize));

    let disabled_items = vec!["Oak".to_string(), "Pine".to_string(), "Birch".to_string()];
    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));

    let empty_items: Vec<String> = Vec::new();
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let code = r#"let items = vec!["Apple".to_string(), "Banana".to_string()];
let (selected, set_selected) = signal(Some(1_usize));
<Select
  id_base="fruit".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
  disabled_indices=vec![3]
/>"#;

    let states_code = r#"<Select
  id_base="select-disabled".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
  disabled=true
/>
<Select
  id_base="select-empty".to_string()
  items=Vec::<String>::new()
  selected_index=empty_selected
  set_selected_index=set_empty_selected
  placeholder="No options".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Select"
            slug="select"
            group="Collections"
            description="Select = Button + Popover + ListBox composition with Spectrum-style trigger and state semantics."
        >
            <Playground title="Selection + Disabled Options" code=code>
                <div class="docs-stack">
                    <Select
                        id_base="docs-select".to_string()
                        items=items
                        selected_index=selected
                        set_selected_index=set_selected
                        disabled_indices=vec![3]
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <Select
                            id_base="docs-select-disabled".to_string()
                            items=disabled_items
                            selected_index=disabled_selected
                            set_selected_index=set_disabled_selected
                            disabled=true
                        />
                        <span class="ui-muted">
                            "disabled selected: "
                            {move || disabled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack">
                        <Select
                            id_base="docs-select-empty".to_string()
                            items=empty_items
                            selected_index=empty_selected
                            set_selected_index=set_empty_selected
                            placeholder="No options".to_string()
                        />
                        <span class="ui-muted">
                            "empty selected: "
                            {move || empty_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn combo_box() -> AnyView {
    let items = vec![
        "Rust".to_string(),
        "TypeScript".to_string(),
        "Go".to_string(),
        "Python".to_string(),
        "Zig".to_string(),
    ];
    let (selected, set_selected) = signal(Some(1_usize));
    let (invalid, set_invalid) = signal(false);

    let disabled_items = vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()];
    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));

    let empty_items: Vec<String> = Vec::new();
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let code = r#"let (selected, set_selected) = signal(Some(1_usize));
let (invalid, set_invalid) = signal(false);
<ComboBox
  id_base="lang".to_string()
  label="Language".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
  disabled_indices=vec![4]
  description="Pick one runtime language".to_string()
  error="Language is required".to_string()
  invalid=Signal::derive(move || invalid.get())
/>"#;

    let states_code = r#"<ComboBox
  id_base="lang-disabled".to_string()
  label="Disabled language".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
  disabled=true
/>
<ComboBox
  id_base="lang-empty".to_string()
  label="Empty language list".to_string()
  items=Vec::<String>::new()
  selected_index=empty_selected
  set_selected_index=set_empty_selected
  placeholder="No options".to_string()
/>"#;

    view! {
        <ComponentPage
            title="ComboBox"
            slug="combo-box"
            group="Collections"
            description="Combobox with input + listbox + popover, including Spectrum-style validation and empty states."
        >
            <Playground title="Selection + Validation" code=code>
                <div class="docs-stack">
                    <ComboBox
                        id_base="docs-combo-box".to_string()
                        label="Language".to_string()
                        items=items
                        selected_index=selected
                        set_selected_index=set_selected
                        disabled_indices=vec![4]
                        description="Pick one runtime language".to_string()
                        error="Language is required".to_string()
                        invalid=Signal::derive(move || invalid.get())
                    />
                    <div class="docs-row">
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_invalid.update(|value| *value = !*value))
                        >
                            {move || if invalid.get() { "Clear invalid" } else { "Mark invalid" }}
                        </ui_components::Button>
                        <span class="ui-muted">
                            "selected: "
                            {move || selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <ComboBox
                            id_base="docs-combo-box-disabled".to_string()
                            label="Disabled language".to_string()
                            items=disabled_items
                            selected_index=disabled_selected
                            set_selected_index=set_disabled_selected
                            disabled=true
                        />
                        <span class="ui-muted">
                            "disabled selected: "
                            {move || disabled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack">
                        <ComboBox
                            id_base="docs-combo-box-empty".to_string()
                            label="Empty language list".to_string()
                            items=empty_items
                            selected_index=empty_selected
                            set_selected_index=set_empty_selected
                            placeholder="No options".to_string()
                        />
                        <span class="ui-muted">
                            "empty selected: "
                            {move || empty_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn autocomplete() -> AnyView {
    let items = vec![
        "San Francisco".to_string(),
        "Seattle".to_string(),
        "Shanghai".to_string(),
        "Shenzhen".to_string(),
        "Singapore".to_string(),
    ];
    let (selected, set_selected) = signal(Some(1_usize));
    let (invalid, set_invalid) = signal(false);

    let disabled_items = vec![
        "Berlin".to_string(),
        "Boston".to_string(),
        "Brisbane".to_string(),
    ];
    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));

    let empty_items: Vec<String> = Vec::new();
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let code = r#"let (selected, set_selected) = signal(Some(1_usize));
let (invalid, set_invalid) = signal(false);
<Autocomplete
  id_base="city".to_string()
  label="City".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
  disabled_indices=vec![3]
  description="Search and pick one city".to_string()
  error="City is required".to_string()
  invalid=Signal::derive(move || invalid.get())
/>"#;

    let states_code = r#"<Autocomplete
  id_base="city-disabled".to_string()
  label="Disabled city".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
  disabled=true
/>
<Autocomplete
  id_base="city-empty".to_string()
  label="Empty city list".to_string()
  items=Vec::<String>::new()
  selected_index=empty_selected
  set_selected_index=set_empty_selected
  placeholder="No options".to_string()
/>"#;

    view! {
        <ComponentPage
            title="Autocomplete"
            slug="autocomplete"
            group="Collections"
            description="Combobox-like autocomplete with filtered options, validation semantics, and active highlight motion."
        >
            <Playground title="Selection + Validation" code=code>
                <div class="docs-stack">
                    <Autocomplete
                        id_base="docs-autocomplete".to_string()
                        label="City".to_string()
                        items=items
                        selected_index=selected
                        set_selected_index=set_selected
                        disabled_indices=vec![3]
                        description="Search and pick one city".to_string()
                        error="City is required".to_string()
                        invalid=Signal::derive(move || invalid.get())
                        placeholder="Type…".to_string()
                    />
                    <div class="docs-row">
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_invalid.update(|value| *value = !*value))
                        >
                            {move || if invalid.get() { "Clear invalid" } else { "Mark invalid" }}
                        </ui_components::Button>
                        <span class="ui-muted">
                            "selected: "
                            {move || selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <Autocomplete
                            id_base="docs-autocomplete-disabled".to_string()
                            label="Disabled city".to_string()
                            items=disabled_items
                            selected_index=disabled_selected
                            set_selected_index=set_disabled_selected
                            disabled=true
                        />
                        <span class="ui-muted">
                            "disabled selected: "
                            {move || disabled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack">
                        <Autocomplete
                            id_base="docs-autocomplete-empty".to_string()
                            label="Empty city list".to_string()
                            items=empty_items
                            selected_index=empty_selected
                            set_selected_index=set_empty_selected
                            placeholder="No options".to_string()
                        />
                        <span class="ui-muted">
                            "empty selected: "
                            {move || empty_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn dropdown_menu() -> AnyView {
    let default_items = vec![
        "Duplicate".to_string(),
        "Move".to_string(),
        "Archive".to_string(),
    ];
    let controlled_items = vec![
        "Rename".to_string(),
        "Move".to_string(),
        "Share".to_string(),
    ];
    let disabled_items = vec!["Duplicate".to_string(), "Archive".to_string()];
    let empty_items: Vec<String> = Vec::new();

    let (last, set_last) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last.set(Some(index)));

    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let code = r#"<DropdownMenu id_base="dd".to_string() items=items on_action=on_action>
  "Open"
</DropdownMenu>"#;

    let controlled_code = r#"let (open, set_open) = signal(false);
let open_signal: Signal<bool> = Signal::derive(move || open.get());
<DropdownMenu
  id_base="dd-controlled".to_string()
  items=items
  on_action=on_action
  open=open_signal
  on_open_change=Callback::new(move |next| set_open.set(next))
>
  "Controlled"
</DropdownMenu>"#;

    let disabled_code = r#"<DropdownMenu id_base="dd-disabled".to_string() items=items on_action=on_action disabled=true>
  "Disabled"
</DropdownMenu>
<DropdownMenu id_base="dd-empty".to_string() items=Vec::<String>::new() on_action=on_action>
  "Empty"
</DropdownMenu>"#;

    view! {
        <ComponentPage
            title="DropdownMenu"
            slug="dropdown-menu"
            group="Collections"
            description="Button trigger that opens a Menu in a Popover with controlled/uncontrolled state support."
        >
            <Playground title="Default" code=code>
                <div class="docs-row">
                    <DropdownMenu
                        id_base="docs-dropdown".to_string()
                        items=default_items
                        on_action=on_action
                    >
                        "Open"
                    </DropdownMenu>
                    <span class="ui-muted">
                        "last: "
                        {move || last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled Open State" code=controlled_code>
                <div class="docs-stack">
                    <DropdownMenu
                        id_base="docs-dropdown-controlled".to_string()
                        items=controlled_items
                        on_action=on_action
                        open=controlled_open
                        on_open_change=on_open_change
                    >
                        "Controlled"
                    </DropdownMenu>
                    <span class="ui-muted">
                        "open: "
                        {move || controlled_open_raw.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code=disabled_code>
                <div class="docs-row">
                    <DropdownMenu
                        id_base="docs-dropdown-disabled".to_string()
                        items=disabled_items
                        on_action=on_action
                        disabled=true
                    >
                        "Disabled"
                    </DropdownMenu>

                    <DropdownMenu
                        id_base="docs-dropdown-empty".to_string()
                        items=empty_items
                        on_action=on_action
                    >
                        "Empty"
                    </DropdownMenu>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn pagination() -> AnyView {
    let (page, set_page) = signal(1_usize);
    let code = r#"let (page, set_page) = signal(1_usize);
<Pagination total_pages=12 page=page set_page=set_page />"#;

    view! {
        <ComponentPage
            title="Pagination"
            slug="pagination"
            group="Collections"
            description="Pagination control with sibling/boundary range logic."
        >
            <Playground title="Pages" code=code>
                <div class="docs-stack">
                    <Pagination total_pages=12 page=page set_page=set_page siblings=1 boundaries=1 />
                    <span class="ui-muted">"page: " {move || page.get().to_string()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn tag_group() -> AnyView {
    let (tags, set_tags) = signal(vec![
        Tag::new("tag-rust", "Rust"),
        Tag::new("tag-leptos", "Leptos"),
        Tag::new("tag-spectrum", "Spectrum"),
    ]);

    let on_remove = Callback::new(move |tag: Tag| {
        set_tags.update(|list| list.retain(|t| t.id != tag.id));
    });

    let invalid = Signal::derive(move || tags.get().is_empty());
    let required = Signal::derive(|| true);

    let code = r#"let (tags, set_tags) = signal(vec![Tag { label: "Rust".to_string(), disabled: false }]);
<TagGroup tags=tags on_remove=Some(on_remove) />"#;

    view! {
        <ComponentPage
            title="TagGroup"
            slug="tag-group"
            group="Collections"
            description="A removable tag list built on Chip."
        >
            <Playground title="Removable tags" code=code>
                <TagGroup tags=tags on_remove=on_remove label="Tags".to_string() />
            </Playground>

            <Playground
                title="Validation"
                code=r#"<TagGroup
  tags=tags
  on_remove=on_remove
  label="Tags".to_string()
  description="Use remove to delete a tag".to_string()
  error="At least one tag is required".to_string()
  invalid=invalid
  required=Signal::derive(|| true)
/>"#
            >
                <TagGroup
                    tags=tags
                    on_remove=on_remove
                    label="Tags".to_string()
                    description="Use remove to delete a tag".to_string()
                    error="At least one tag is required".to_string()
                    invalid=invalid
                    required=required
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
