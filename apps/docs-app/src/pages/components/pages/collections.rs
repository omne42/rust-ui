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
    let (selected, set_selected) = signal(None::<usize>);
    let code = r#"let items: Arc<[String]> = vec!["Apple".to_string(), "Banana".to_string()].into();
let (selected, set_selected) = signal(None::<usize>);
<ListBox id_base="fruit".to_string() items=items selected_index=selected set_selected_index=set_selected />"#;

    view! {
        <ComponentPage
            title="ListBox"
            slug="listbox"
            group="Collections"
            description="Listbox with active highlight spring motion and typeahead."
        >
            <Playground title="ListBox" code=code>
                <div class="docs-stack">
                    <ListBox
                        id_base="docs-listbox".to_string()
                        items=items
                        selected_index=selected
                        set_selected_index=set_selected
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || selected.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn menu() -> AnyView {
    let items: Arc<[String]> = vec![
        "New file".to_string(),
        "Rename".to_string(),
        "Delete".to_string(),
    ]
    .into();
    let (last, set_last) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last.set(Some(index)));

    let code = r#"let items: Arc<[String]> = vec!["New file".to_string(), "Rename".to_string()].into();
<Menu id_base="menu".to_string() items=items on_action=on_action />"#;

    view! {
        <ComponentPage
            title="Menu"
            slug="menu"
            group="Collections"
            description="ARIA menu with menuitem kinds (action/checkbox/radio)."
        >
            <Playground title="Menu" code=code>
                <div class="docs-stack">
                    <Menu
                        id_base="docs-menu".to_string()
                        items=items
                        on_action=on_action
                        item_kinds=vec![
                            MenuItemKind::Action,
                            MenuItemKind::Checkbox { is_checked: Signal::derive(|| true) },
                            MenuItemKind::Radio { is_checked: Signal::derive(|| false) },
                        ]
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn menu_trigger() -> AnyView {
    let items = vec![
        "Profile".to_string(),
        "Settings".to_string(),
        "Log out".to_string(),
    ];
    let (last, set_last) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last.set(Some(index)));

    let code = r#"<MenuTrigger id_base="trigger".to_string() items=items on_action=on_action>
  "Open menu"
</MenuTrigger>"#;

    view! {
        <ComponentPage
            title="MenuTrigger"
            slug="menu-trigger"
            group="Collections"
            description="Button trigger that opens a Popover-based Menu."
        >
            <Playground title="Trigger" code=code>
                <div class="docs-row">
                    <MenuTrigger id_base="docs-menu-trigger".to_string() items=items on_action=on_action>
                        "Open menu"
                    </MenuTrigger>
                    <span class="ui-muted">
                        "last: "
                        {move || last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
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
    let (selected, set_selected) = signal(None::<usize>);
    let code = r#"let items = vec!["Apple".to_string(), "Banana".to_string()];
let (selected, set_selected) = signal(None::<usize>);
<Select id_base="fruit".to_string() items=items selected_index=selected set_selected_index=set_selected />"#;

    view! {
        <ComponentPage
            title="Select"
            slug="select"
            group="Collections"
            description="Select = Button + Popover + ListBox composition."
        >
            <Playground title="Select" code=code>
                <Select
                    id_base="docs-select".to_string()
                    items=items
                    selected_index=selected
                    set_selected_index=set_selected
                />
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
    let (selected, set_selected) = signal(None::<usize>);
    let code = r#"let (selected, set_selected) = signal(None::<usize>);
<ComboBox id_base="lang".to_string() label="Language".to_string()
  items=items selected_index=selected set_selected_index=set_selected />"#;

    view! {
        <ComponentPage
            title="ComboBox"
            slug="combo-box"
            group="Collections"
            description="Combobox with input + listbox + popover."
        >
            <Playground title="ComboBox" code=code>
                <ComboBox
                    id_base="docs-combo-box".to_string()
                    label="Language".to_string()
                    items=items
                    selected_index=selected
                    set_selected_index=set_selected
                />
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
    let (selected, set_selected) = signal(None::<usize>);

    let code = r#"let (selected, set_selected) = signal(None::<usize>);
<Autocomplete id_base="city".to_string() label="City".to_string()
  items=items selected_index=selected set_selected_index=set_selected />"#;

    view! {
        <ComponentPage
            title="Autocomplete"
            slug="autocomplete"
            group="Collections"
            description="Combobox-like autocomplete with filtered options and active highlight motion."
        >
            <Playground title="Autocomplete" code=code>
                <Autocomplete
                    id_base="docs-autocomplete".to_string()
                    label="City".to_string()
                    items=items
                    selected_index=selected
                    set_selected_index=set_selected
                    placeholder="Type…".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn dropdown_menu() -> AnyView {
    let items = vec![
        "Duplicate".to_string(),
        "Move".to_string(),
        "Archive".to_string(),
    ];
    let (last, set_last) = signal(None::<usize>);
    let on_action = Callback::new(move |index: usize| set_last.set(Some(index)));

    let code = r#"<DropdownMenu id_base="dd".to_string() items=items on_action=on_action>
  "Open"
</DropdownMenu>"#;

    view! {
        <ComponentPage
            title="DropdownMenu"
            slug="dropdown-menu"
            group="Collections"
            description="Button trigger that opens a Menu in a Popover (bb/ui-web port)."
        >
            <Playground title="Dropdown" code=code>
                <div class="docs-row">
                    <DropdownMenu id_base="docs-dropdown".to_string() items=items on_action=on_action>
                        "Open"
                    </DropdownMenu>
                    <span class="ui-muted">
                        "last: "
                        {move || last.get().map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())}
                    </span>
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
        </ComponentPage>
    }
    .into_any()
}
