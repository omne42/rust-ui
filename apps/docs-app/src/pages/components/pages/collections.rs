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

    let label_only_items = vec![
        BreadcrumbItem {
            label: "Library".to_string(),
            href: None,
        },
        BreadcrumbItem {
            label: "UI".to_string(),
            href: None,
        },
        BreadcrumbItem {
            label: "Current".to_string(),
            href: None,
        },
    ];

    let empty_items = Vec::<BreadcrumbItem>::new();

    let code = Signal::derive(move || {
        r##"let items = vec![
  BreadcrumbItem { label: "Home".to_string(), href: Some("#/docs/welcome".to_string()) },
  BreadcrumbItem { label: "Components".to_string(), href: Some("#/components".to_string()) },
  BreadcrumbItem { label: "Breadcrumbs".to_string(), href: None },
];
<Breadcrumbs items=items />"##
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<Breadcrumbs
  items=vec![
    BreadcrumbItem { label: "Library".to_string(), href: None },
    BreadcrumbItem { label: "UI".to_string(), href: None },
    BreadcrumbItem { label: "Current".to_string(), href: None },
  ]
  aria_label="Label-only trail".to_string()
/>
<Breadcrumbs items=Vec::<BreadcrumbItem>::new() aria_label="Empty trail".to_string() />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Breadcrumbs"
            slug="breadcrumbs"
            group="Collections"
            description="Breadcrumb nav with current-page semantics and Spectrum-style root state attrs."
        >
            <Playground title="Trail" code_signal=code>
                <Breadcrumbs items=items />
            </Playground>

            <Playground title="Label-Only + Empty" code_signal=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <Breadcrumbs items=label_only_items aria_label="Label-only trail".to_string() />
                        <span class="ui-muted">"all labels (no links)"</span>
                    </div>
                    <div class="docs-stack">
                        <Breadcrumbs items=empty_items aria_label="Empty trail".to_string() />
                        <span class="ui-muted">"empty trail (0 items)"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn accordion() -> AnyView {
    let multi_labels = vec![
        "First".to_string(),
        "Second".to_string(),
        "Third".to_string(),
    ];
    let single_labels = vec![
        "Overview".to_string(),
        "Details".to_string(),
        "History".to_string(),
    ];

    let (open_multi, set_open_multi) = signal(BTreeSet::from([0_usize]));
    let on_multi_change = Callback::new(move |next: BTreeSet<usize>| set_open_multi.set(next));

    let (open_single, set_open_single) = signal(BTreeSet::from([1_usize]));
    let on_single_change = Callback::new(move |next: BTreeSet<usize>| set_open_single.set(next));

    let code = Signal::derive(move || {
        r#"let (open, set_open) = signal(BTreeSet::from([0_usize]));
let on_open_change = Callback::new(move |next: BTreeSet<usize>| set_open.set(next));
<Accordion
  labels=vec!["First".to_string(), "Second".to_string(), "Third".to_string()]
  id_base="accordion".to_string()
  open_indices=open.into()
  on_open_change=on_open_change
  selection_mode=AccordionSelectionMode::Multiple
>
  <div>"Panel 1"</div>
  <div>"Panel 2"</div>
  <div>"Panel 3"</div>
</Accordion>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(BTreeSet::from([1_usize]));
let on_open_change = Callback::new(move |next: BTreeSet<usize>| set_open.set(next));
<Accordion
  labels=vec!["Overview".to_string(), "Details".to_string(), "History".to_string()]
  id_base="accordion-single".to_string()
  open_indices=open.into()
  on_open_change=on_open_change
  selection_mode=AccordionSelectionMode::Single
  disabled_indices=vec![2]
>
  <div>"Overview"</div>
  <div>"Details"</div>
  <div>"History"</div>
</Accordion>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Accordion"
            slug="accordion"
            group="Collections"
            description="Multi-panel disclosure with roving tabindex, HeroUI-level spring motion, and Spectrum-style root state attrs."
        >
            <Playground title="Multiple + Controlled" code_signal=code>
                <div class="docs-stack">
                    <Accordion
                        labels=multi_labels
                        id_base="docs-accordion".to_string()
                        open_indices=open_multi.into()
                        on_open_change=on_multi_change
                        selection_mode=AccordionSelectionMode::Multiple
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
                            <div class="ui-muted">"Multiple mode allows multiple open panels."</div>
                        </div>
                    </Accordion>
                    <span class="ui-muted">
                        "open indices: "
                        {move || {
                            let open = open_multi.get().iter().copied().collect::<Vec<_>>();
                            format!("{open:?}")
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Single + Disabled" code_signal=states_code>
                <div class="docs-stack">
                    <Accordion
                        labels=single_labels
                        id_base="docs-accordion-single".to_string()
                        open_indices=open_single.into()
                        on_open_change=on_single_change
                        selection_mode=AccordionSelectionMode::Single
                        disabled_indices=vec![2]
                    >
                        <div class="docs-stack">
                            <div>"Overview content"</div>
                            <div class="ui-muted">"Single mode keeps at most one panel open."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Details content"</div>
                            <div class="ui-muted">"Selection is fully controlled by open indices."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"History content"</div>
                            <div class="ui-muted">"This trigger is disabled and skipped by roving focus."</div>
                        </div>
                    </Accordion>
                    <span class="ui-muted">
                        "single open: "
                        {move || {
                            let open = open_single.get().iter().copied().collect::<Vec<_>>();
                            format!("{open:?}")
                        }}
                    </span>
                    <span class="ui-muted">"disabled index: 2"</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn disclosure() -> AnyView {
    let (open, set_open) = signal(true);
    let on_open_change = Callback::new(move |next: bool| set_open.set(next));

    let code = Signal::derive(move || {
        r#"let (open, set_open) = signal(true);
let on_open_change = Callback::new(move |next: bool| set_open.set(next));
<Disclosure
  id_base="disc".to_string()
  label="Details".to_string()
  open=open.into()
  on_open_change=on_open_change
>
  <div>"Hidden content"</div>
</Disclosure>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<Disclosure
  id_base="disc-disabled".to_string()
  label="Disabled details".to_string()
  default_open=false
  disabled=true
>
  <div>"Disabled content"</div>
</Disclosure>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Disclosure"
            slug="disclosure"
            group="Collections"
            description="Single disclosure panel with HeroUI-level spring motion and Spectrum-style root state attrs."
        >
            <Playground title="Controlled" code_signal=code>
                <div class="docs-stack">
                    <Disclosure
                        id_base="docs-disclosure".to_string()
                        label="Details".to_string()
                        open=open.into()
                        on_open_change=on_open_change
                    >
                        <div class="docs-stack">
                            <div>"Hidden content"</div>
                            <div class="ui-muted">"Uses the same open-state contract as overlays."</div>
                        </div>
                    </Disclosure>
                    <span class="ui-muted">
                        "open: "
                        {move || open.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled" code_signal=states_code>
                <div class="docs-stack">
                    <Disclosure
                        id_base="docs-disclosure-disabled".to_string()
                        label="Disabled details".to_string()
                        default_open=false
                        disabled=true
                    >
                        <div class="docs-stack">
                            <div>"Disabled content"</div>
                            <div class="ui-muted">"Disabled disclosure keeps trigger non-interactive."</div>
                        </div>
                    </Disclosure>
                    <span class="ui-muted">"disabled: true"</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn tabs() -> AnyView {
    let labels = vec!["Overview", "Details", "Settings"];
    let manual_labels = vec!["Profile", "Billing", "Team"];

    let (selected_auto, set_selected_auto) = signal(0_usize);
    let on_auto_change = Callback::new(move |index: usize| set_selected_auto.set(index));

    let (selected_manual, set_selected_manual) = signal(1_usize);
    let on_manual_change = Callback::new(move |index: usize| set_selected_manual.set(index));

    let code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(0_usize);
let on_change = Callback::new(move |next: usize| set_selected.set(next));
<Tabs
  labels=vec!["Overview", "Details", "Settings"]
  id_base="tabs".to_string()
  selected_index=selected
  on_selection_change=on_change
  keyboard_activation=TabsKeyboardActivation::Automatic
>
  <div>"Overview panel"</div>
  <div>"Details panel"</div>
  <div>"Settings panel"</div>
</Tabs>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(1_usize);
let on_change = Callback::new(move |next: usize| set_selected.set(next));
<Tabs
  labels=vec!["Profile", "Billing", "Team"]
  id_base="tabs-manual".to_string()
  keyboard_activation=TabsKeyboardActivation::Manual
  selected_index=selected
  on_selection_change=on_change
  disabled_indices=vec![2]
>
  <div>"Profile panel"</div>
  <div>"Billing panel"</div>
  <div>"Team panel"</div>
</Tabs>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Tabs"
            slug="tabs"
            group="Collections"
            description="Tabs with roving tabindex, HeroUI-level indicator motion, and Spectrum-style root state attrs."
        >
            <Playground title="Automatic + Controlled" code_signal=code>
                <div class="docs-stack">
                    <Tabs
                        labels=labels
                        id_base="docs-tabs".to_string()
                        selected_index=selected_auto
                        on_selection_change=on_auto_change
                        keyboard_activation=TabsKeyboardActivation::Automatic
                    >
                        <div class="docs-stack">
                            <div>"Overview"</div>
                            <div class="ui-muted">"Arrow keys move + select in automatic mode."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Details"</div>
                            <div class="ui-muted">"Selection change is controlled by signal callback."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Settings"</div>
                            <div class="ui-muted">"Indicator motion stays spring-driven."</div>
                        </div>
                    </Tabs>
                    <span class="ui-muted">
                        "selected: "
                        {move || selected_auto.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Manual + Disabled" code_signal=states_code>
                <div class="docs-stack">
                    <Tabs
                        labels=manual_labels
                        id_base="docs-tabs-manual".to_string()
                        selected_index=selected_manual
                        on_selection_change=on_manual_change
                        keyboard_activation=TabsKeyboardActivation::Manual
                        disabled_indices=vec![2]
                    >
                        <div class="docs-stack">
                            <div>"Profile"</div>
                            <div class="ui-muted">"Manual mode: focus moves first, Enter/Space commits."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Billing"</div>
                            <div class="ui-muted">"Current selected index reflects committed tab."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Team"</div>
                            <div class="ui-muted">"This tab is disabled and skipped by roving focus."</div>
                        </div>
                    </Tabs>
                    <span class="ui-muted">
                        "manual selected: "
                        {move || selected_manual.get().to_string()}
                    </span>
                    <span class="ui-muted">"disabled tab index: 2"</span>
                </div>
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

    let (selected, set_selected) = signal(Some(1_usize));
    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let code = Signal::derive(move || {
        r#"let items: Arc<[String]> = vec!["Apple".to_string(), "Banana".to_string()].into();
let (selected, set_selected) = signal(Some(1_usize));
<ListBox
  id_base="fruit".to_string()
  items=items
  selected_index=selected
  set_selected_index=set_selected
  aria_label="Fruit".to_string()
  disabled_indices=vec![1]
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));
let (empty_selected, set_empty_selected) = signal(None::<usize>);

<ListBox
  id_base="cities-disabled".to_string()
  items=vec!["London".to_string(), "Paris".to_string(), "Tokyo".to_string()].into()
  selected_index=disabled_selected
  set_selected_index=set_disabled_selected
  aria_label="Disabled list".to_string()
  disabled=true
/>
<ListBox
  id_base="cities-empty".to_string()
  items=Vec::<String>::new().into()
  selected_index=empty_selected
  set_selected_index=set_empty_selected
  aria_label="Empty list".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="ListBox"
            slug="listbox"
            group="Collections"
            description="Listbox with active highlight spring motion, typeahead, and Spectrum-style root state attrs."
        >
            <Playground title="Selection + Typeahead" code_signal=code>
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

            <Playground title="Disabled + Empty" code_signal=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <ListBox
                            id_base="docs-listbox-disabled".to_string()
                            items=disabled_items
                            selected_index=disabled_selected
                            set_selected_index=set_disabled_selected
                            aria_label="Disabled city list".to_string()
                            disabled=true
                        />
                        <span class="ui-muted">
                            "disabled selected: "
                            {move || disabled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        </span>
                    </div>

                    <div class="docs-stack">
                        <ListBox
                            id_base="docs-listbox-empty".to_string()
                            items=empty_items
                            selected_index=empty_selected
                            set_selected_index=set_empty_selected
                            aria_label="Empty city list".to_string()
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

    let code = Signal::derive(move || {
        r#"let on_action = Callback::new(move |_: usize| {});

<Menu
  id_base="menu".to_string()
  items=vec!["New file".to_string(), "Share with team".to_string()].into()
  on_action=on_action
  aria_label="File actions".to_string()
  item_kinds=vec![
    MenuItemKind::Action,
    MenuItemKind::Checkbox { is_checked: Signal::derive(|| true) },
  ]
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let on_action = Callback::new(move |_: usize| {});

<Menu
  id_base="menu-disabled".to_string()
  items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()].into()
  on_action=on_action
  aria_label="Disabled menu".to_string()
  disabled=true
/>
<Menu
  id_base="menu-empty".to_string()
  items=Vec::<String>::new().into()
  on_action=Callback::new(move |_: usize| {})
  aria_label="Empty menu".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Menu"
            slug="menu"
            group="Collections"
            description="ARIA menu with action / checkbox / radio roles, active-highlight motion, and Spectrum-style root state attrs."
        >
            <Playground title="Kinds + Selection" code_signal=code>
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

            <Playground title="Disabled + Empty" code_signal=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <Menu
                            id_base="docs-menu-disabled".to_string()
                            items=disabled_items
                            on_action=noop_action
                            aria_label="Disabled menu".to_string()
                            disabled=true
                        />
                        <span class="ui-muted">"disabled menu (no action)"</span>
                    </div>

                    <div class="docs-stack">
                        <Menu
                            id_base="docs-menu-empty".to_string()
                            items=empty_items
                            on_action=noop_action
                            aria_label="Empty menu".to_string()
                        />
                        <span class="ui-muted">"empty menu (0 items)"</span>
                    </div>
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

    let code = Signal::derive(move || {
        r#"<MenuTrigger
  id_base="trigger".to_string()
  items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
>
  "Open menu"
</MenuTrigger>"#
            .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);

<MenuTrigger
  id_base="trigger-controlled".to_string()
  items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
  close_on_action=false
  disabled_indices=vec![1]
  open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
>
  "Controlled"
</MenuTrigger>"#
            .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<MenuTrigger
  id_base="trigger-disabled".to_string()
  items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
  disabled=true
>
  "Disabled"
</MenuTrigger>
<MenuTrigger
  id_base="trigger-empty".to_string()
  items=Vec::<String>::new()
  on_action=Callback::new(move |_: usize| {})
>
  "Empty"
</MenuTrigger>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="MenuTrigger"
            slug="menu-trigger"
            group="Collections"
            description="Button-triggered menu surface with Spectrum state attrs and controlled/uncontrolled close-strategy semantics."
        >
            <Playground title="Default" code_signal=code>
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

            <Playground title="Controlled + persistent open" code_signal=controlled_code>
                <div class="docs-stack">
                    <MenuTrigger
                        id_base="docs-menu-trigger-controlled".to_string()
                        items=controlled_items
                        on_action=on_action
                        close_on_action=false
                        disabled_indices=vec![1]
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

            <Playground title="Disabled + Empty" code_signal=disabled_code>
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

    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = controlled_open_raw.into();
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let disabled_indices = vec![3_usize];
    let disabled_option_count = disabled_indices.len();
    let has_selection = Signal::derive(move || selected.get().is_some());

    let disabled_items = vec!["Oak".to_string(), "Pine".to_string(), "Birch".to_string()];
    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));

    let empty_items: Vec<String> = Vec::new();
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(1_usize));
let (open, set_open) = signal(false);
let on_open_change = Callback::new(move |next: bool| set_open.set(next));

<Select
  id_base="fruit".to_string()
  items=vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string(), "Durian".to_string()]
  selected_index=selected
  set_selected_index=set_selected
  open=open.into()
  on_open_change=on_open_change
  disabled_indices=vec![3]
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(0_usize));
let (empty_selected, set_empty_selected) = signal(None::<usize>);

<Select
  id_base="select-disabled".to_string()
  items=vec!["Oak".to_string(), "Pine".to_string(), "Birch".to_string()]
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
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Select"
            slug="select"
            group="Collections"
            description="Select with controlled open state, listbox semantics, and Spectrum-style root state attrs."
        >
            <Playground title="Controlled Open + Selection" code_signal=code>
                <div class="docs-stack">
                    <Select
                        id_base="docs-select-controlled".to_string()
                        items=items
                        selected_index=selected
                        set_selected_index=set_selected
                        open=controlled_open
                        on_open_change=on_open_change
                        disabled_indices=disabled_indices
                    />
                    <div class="docs-row">
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            on_press=Callback::new(move |_| {
                                set_controlled_open_raw.update(|value| *value = !*value);
                            })
                        >
                            "Toggle open"
                        </ui_components::Button>
                        <span class="ui-muted">
                            "open: "
                            {move || controlled_open_raw.get().to_string()}
                        </span>
                    </div>
                    <span class="ui-muted">
                        "selected: "
                        {move || selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                        " · has selection: "
                        {move || has_selection.get().to_string()}
                        " · disabled options: "
                        {disabled_option_count.to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=states_code>
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
    let controlled_items = vec![
        "Rust".to_string(),
        "TypeScript".to_string(),
        "Go".to_string(),
        "Python".to_string(),
        "Zig".to_string(),
    ];
    let (selected, set_selected) = signal(Some(1_usize));
    let (invalid, set_invalid) = signal(false);

    let (controlled_selected, set_controlled_selected) = signal(Some(2_usize));
    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let disabled_items = vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()];
    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));

    let empty_items: Vec<String> = Vec::new();
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(1_usize));
let (invalid, set_invalid) = signal(false);

<ComboBox
  id_base="lang".to_string()
  label="Language".to_string()
  items=vec![
    "Rust".to_string(),
    "TypeScript".to_string(),
    "Go".to_string(),
    "Python".to_string(),
    "Swift".to_string(),
  ]
  selected_index=selected
  set_selected_index=set_selected
  disabled_indices=vec![4]
  description="Pick one runtime language".to_string()
  error="Language is required".to_string()
  invalid=Signal::derive(move || invalid.get())
/>"#
        .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(2_usize));
let (open, set_open) = signal(false);

<ComboBox
  id_base="lang-controlled".to_string()
  label="Controlled language".to_string()
  items=vec![
    "Rust".to_string(),
    "TypeScript".to_string(),
    "Go".to_string(),
    "Python".to_string(),
    "Swift".to_string(),
  ]
  selected_index=selected
  set_selected_index=set_selected
  open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
  disabled_indices=vec![4]
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(0_usize));
let (empty_selected, set_empty_selected) = signal(None::<usize>);

<ComboBox
  id_base="lang-disabled".to_string()
  label="Disabled language".to_string()
  items=vec!["Alpha".to_string(), "Beta".to_string(), "Gamma".to_string()]
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
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="ComboBox"
            slug="combo-box"
            group="Collections"
            description="Combobox with input + listbox + popover, Spectrum-style root attrs, and HeroUI-level panel/highlight motion."
        >
            <Playground title="Selection + Validation" code_signal=code>
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

            <Playground title="Controlled Open State" code_signal=controlled_code>
                <div class="docs-stack">
                    <ComboBox
                        id_base="docs-combo-box-controlled".to_string()
                        label="Controlled language".to_string()
                        items=controlled_items
                        selected_index=controlled_selected
                        set_selected_index=set_controlled_selected
                        open=controlled_open
                        on_open_change=on_open_change
                        disabled_indices=vec![4]
                        description="Open state is externally controlled".to_string()
                    />
                    <span class="ui-muted">
                        "open: "
                        {move || controlled_open_raw.get().to_string()}
                    </span>
                    <span class="ui-muted">
                        "selected: "
                        {move || controlled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=states_code>
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
    let controlled_items = vec![
        "San Francisco".to_string(),
        "Seattle".to_string(),
        "Shanghai".to_string(),
        "Shenzhen".to_string(),
        "Singapore".to_string(),
    ];
    let (selected, set_selected) = signal(Some(1_usize));
    let (invalid, set_invalid) = signal(false);

    let (controlled_selected, set_controlled_selected) = signal(Some(2_usize));
    let (controlled_open_raw, set_controlled_open_raw) = signal(false);
    let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
    let on_open_change = Callback::new(move |next: bool| set_controlled_open_raw.set(next));

    let disabled_items = vec![
        "Berlin".to_string(),
        "Boston".to_string(),
        "Brisbane".to_string(),
    ];
    let (disabled_selected, set_disabled_selected) = signal(Some(0_usize));

    let empty_items: Vec<String> = Vec::new();
    let (empty_selected, set_empty_selected) = signal(None::<usize>);

    let code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(1_usize));
let (invalid, set_invalid) = signal(false);

<Autocomplete
  id_base="city".to_string()
  label="City".to_string()
  items=vec![
    "Sydney".to_string(),
    "Melbourne".to_string(),
    "Perth".to_string(),
    "Brisbane".to_string(),
  ]
  selected_index=selected
  set_selected_index=set_selected
  disabled_indices=vec![3]
  description="Search and pick one city".to_string()
  error="City is required".to_string()
  invalid=Signal::derive(move || invalid.get())
/>"#
        .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(2_usize));
let (open, set_open) = signal(false);

<Autocomplete
  id_base="city-controlled".to_string()
  label="Controlled city".to_string()
  items=vec![
    "Sydney".to_string(),
    "Melbourne".to_string(),
    "Perth".to_string(),
    "Brisbane".to_string(),
  ]
  selected_index=selected
  set_selected_index=set_selected
  open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
  disabled_indices=vec![3]
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(Some(0_usize));
let (empty_selected, set_empty_selected) = signal(None::<usize>);

<Autocomplete
  id_base="city-disabled".to_string()
  label="Disabled city".to_string()
  items=vec!["Sydney".to_string(), "Melbourne".to_string(), "Perth".to_string()]
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
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Autocomplete"
            slug="autocomplete"
            group="Collections"
            description="Combobox-like autocomplete with Spectrum-style root attrs, controlled/uncontrolled open state, and HeroUI-level active highlight motion."
        >
            <Playground title="Selection + Validation" code_signal=code>
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

            <Playground title="Controlled Open State" code_signal=controlled_code>
                <div class="docs-stack">
                    <Autocomplete
                        id_base="docs-autocomplete-controlled".to_string()
                        label="Controlled city".to_string()
                        items=controlled_items
                        selected_index=controlled_selected
                        set_selected_index=set_controlled_selected
                        open=controlled_open
                        on_open_change=on_open_change
                        disabled_indices=vec![3]
                        description="Open state is externally controlled".to_string()
                    />
                    <span class="ui-muted">
                        "open: "
                        {move || controlled_open_raw.get().to_string()}
                    </span>
                    <span class="ui-muted">
                        "selected: "
                        {move || controlled_selected.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=states_code>
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

    let code = Signal::derive(move || {
        r#"<DropdownMenu
  id_base="dd".to_string()
  items=vec!["Duplicate".to_string(), "Move".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
>
  "Open"
</DropdownMenu>"#
            .to_string()
    });

    let controlled_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(false);

<DropdownMenu
  id_base="dd-controlled".to_string()
  items=vec!["Rename".to_string(), "Move".to_string(), "Share".to_string()]
  on_action=Callback::new(move |_: usize| {})
  open=Signal::derive(move || open.get())
  on_open_change=Callback::new(move |next| set_open.set(next))
  close_on_action=false
  disabled_indices=vec![1]
>
  "Persistent"
</DropdownMenu>"#
            .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<DropdownMenu
  id_base="dd-disabled".to_string()
  items=vec!["Duplicate".to_string(), "Archive".to_string()]
  on_action=Callback::new(move |_: usize| {})
  disabled=true
>
  "Disabled"
</DropdownMenu>
<DropdownMenu
  id_base="dd-empty".to_string()
  items=Vec::<String>::new()
  on_action=Callback::new(move |_: usize| {})
>
  "Empty"
</DropdownMenu>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="DropdownMenu"
            slug="dropdown-menu"
            group="Collections"
            description="Button trigger that opens a Menu in a Popover with Spectrum-style root attrs, controlled/uncontrolled state, and persistent-open action handling."
        >
            <Playground title="Default" code_signal=code>
                <div class="docs-row">
                    <DropdownMenu
                        id_base="docs-dropdown".to_string()
                        items=default_items
                        on_action=on_action
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    >
                        "Open"
                    </DropdownMenu>
                    <span class="ui-muted">
                        "last: "
                        {move || last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled + Persistent Open" code_signal=controlled_code>
                <div class="docs-stack">
                    <DropdownMenu
                        id_base="docs-dropdown-controlled".to_string()
                        items=controlled_items
                        on_action=on_action
                        open=controlled_open
                        on_open_change=on_open_change
                        close_on_action=false
                        disabled_indices=vec![1]
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                            MenuItemKind::Action,
                        ]
                    >
                        "Controlled"
                    </DropdownMenu>
                    <span class="ui-muted">
                        "open: "
                        {move || controlled_open_raw.get().to_string()}
                    </span>
                    <span class="ui-muted">"close_on_action: false (select keeps popover open)"</span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=disabled_code>
                <div class="docs-row">
                    <DropdownMenu
                        id_base="docs-dropdown-disabled".to_string()
                        items=disabled_items
                        on_action=on_action
                        disabled=true
                        item_kinds=vec![MenuItemKind::Action, MenuItemKind::Action]
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
    let (last_change, set_last_change) = signal(None::<usize>);
    let on_change = Callback::new(move |next: usize| set_last_change.set(Some(next)));

    let (disabled_page, set_disabled_page) = signal(1_usize);
    let (empty_page, set_empty_page) = signal(1_usize);

    let code = Signal::derive(move || {
        r#"let (page, set_page) = signal(1_usize);
let on_change = Callback::new(move |next: usize| { /* ... */ });
<Pagination total_pages=12 page=page set_page=set_page siblings=1 boundaries=1 on_change=on_change />"#.to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<Pagination total_pages=1 page=disabled_page set_page=set_disabled_page disabled=true />
<Pagination total_pages=0 page=empty_page set_page=set_empty_page />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Pagination"
            slug="pagination"
            group="Collections"
            description="Pagination control with sibling/boundary range logic and Spectrum-style state attrs."
        >
            <Playground title="Pages + on_change" code_signal=code>
                <div class="docs-stack">
                    <Pagination
                        total_pages=12
                        page=page
                        set_page=set_page
                        siblings=1
                        boundaries=1
                        on_change=on_change
                    />
                    <span class="ui-muted">"page: " {move || page.get().to_string()}</span>
                    <span class="ui-muted">
                        "last change: "
                        {move || last_change.get().map(|value| value.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=states_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <Pagination
                            total_pages=1
                            page=disabled_page
                            set_page=set_disabled_page
                            disabled=true
                        />
                        <span class="ui-muted">
                            "disabled page: "
                            {move || disabled_page.get().to_string()}
                        </span>
                    </div>

                    <div class="docs-stack">
                        <Pagination
                            total_pages=0
                            page=empty_page
                            set_page=set_empty_page
                        />
                        <span class="ui-muted">
                            "empty page signal: "
                            {move || empty_page.get().to_string()}
                        </span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn tag_group() -> AnyView {
    let (removable_tags, set_removable_tags) = signal(vec![
        Tag::new("tag-rust", "Rust"),
        Tag::new("tag-leptos", "Leptos"),
        Tag::disabled("tag-a11y", "Accessibility"),
    ]);

    let on_remove_removable = Callback::new(move |tag: Tag| {
        set_removable_tags.update(|list| list.retain(|item| item.id != tag.id));
    });

    let removable_count = Signal::derive(move || removable_tags.get().len());
    let removable_has_disabled =
        Signal::derive(move || removable_tags.get().iter().any(|tag| tag.disabled));

    let (validation_tags, set_validation_tags) = signal(vec![
        Tag::new("tag-required", "Required"),
        Tag::new("tag-spectrum", "Spectrum"),
    ]);

    let on_remove_validation = Callback::new(move |tag: Tag| {
        set_validation_tags.update(|list| list.retain(|item| item.id != tag.id));
    });

    let validation_invalid = Signal::derive(move || validation_tags.get().is_empty());
    let validation_required = Signal::derive(|| true);

    let (disabled_tags, _set_disabled_tags) = signal(vec![
        Tag::new("tag-motion", "Motion"),
        Tag::new("tag-tokens", "Tokens"),
    ]);
    let (empty_tags, _set_empty_tags) = signal(Vec::<Tag>::new());

    let code = Signal::derive(move || {
        r#"let (tags, set_tags) = signal(vec![
  Tag::new("tag-rust", "Rust"),
  Tag::disabled("tag-a11y", "Accessibility"),
]);
let on_remove = Callback::new(move |tag: Tag| {
  set_tags.update(|list| list.retain(|item| item.id != tag.id));
});
<TagGroup tags=tags on_remove=on_remove label="Framework tags".to_string() />"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (tags, set_tags) = signal(vec![
  Tag::new("tag-rust", "Rust"),
  Tag::disabled("tag-a11y", "Accessibility"),
]);
let on_remove = Callback::new(move |tag: Tag| {
  set_tags.update(|list| list.retain(|item| item.id != tag.id));
});
let invalid = Signal::derive(move || tags.get().is_empty());

<TagGroup
  tags=tags
  on_remove=on_remove
  label="Required tags".to_string()
  description="Remove all tags to trigger invalid state".to_string()
  error="At least one tag is required".to_string()
  invalid=invalid
  required=Signal::derive(|| true)
/>"#
        .to_string()
    });

    let disabled_empty_code = Signal::derive(move || {
        r#"<TagGroup
  tags=disabled_tags
  disabled=true
  label="Disabled tags".to_string()
  description="All chips are non-removable when disabled".to_string()
/>
<TagGroup
  tags=empty_tags
  label="Empty tags".to_string()
  description="No tags currently selected".to_string()
  error="At least one tag is required".to_string()
  invalid=Signal::derive(|| true)
  required=Signal::derive(|| true)
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="TagGroup"
            slug="tag-group"
            group="Collections"
            description="Tag list with removable chips, validation semantics, and Spectrum-style root state attrs."
        >
            <Playground title="Removable + State" code_signal=code>
                <div class="docs-stack">
                    <TagGroup
                        tags=removable_tags
                        on_remove=on_remove_removable
                        label="Framework tags".to_string()
                        description="Remove any non-disabled tag".to_string()
                    />
                    <span class="ui-muted">
                        "count: "
                        {move || removable_count.get().to_string()}
                    </span>
                    <span class="ui-muted">
                        "has disabled tags: "
                        {move || removable_has_disabled.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Validation + Required" code_signal=states_code>
                <div class="docs-stack">
                    <TagGroup
                        tags=validation_tags
                        on_remove=on_remove_validation
                        label="Required tags".to_string()
                        description="Remove all tags to trigger invalid state".to_string()
                        error="At least one tag is required".to_string()
                        invalid=validation_invalid
                        required=validation_required
                    />
                    <span class="ui-muted">
                        "invalid: "
                        {move || validation_invalid.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Empty" code_signal=disabled_empty_code>
                <div class="docs-row">
                    <div class="docs-stack">
                        <TagGroup
                            tags=disabled_tags
                            disabled=true
                            label="Disabled tags".to_string()
                            description="All chips are non-removable when disabled".to_string()
                        />
                        <span class="ui-muted">"disabled: true"</span>
                    </div>

                    <div class="docs-stack">
                        <TagGroup
                            tags=empty_tags
                            label="Empty tags".to_string()
                            description="No tags currently selected".to_string()
                            error="At least one tag is required".to_string()
                            invalid=Signal::derive(|| true)
                            required=Signal::derive(|| true)
                        />
                        <span class="ui-muted">"empty: true"</span>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
