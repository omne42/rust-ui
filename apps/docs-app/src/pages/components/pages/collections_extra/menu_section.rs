use super::*;

pub(crate) fn menu_section() -> AnyView {
    let (showcase_checked, set_showcase_checked) = signal(true);
    let showcase_kind = MenuItemKind::Checkbox {
        is_checked: Signal::derive(move || showcase_checked.get()),
    };
    let (showcase_presses, set_showcase_presses) = signal(0_u32);
    let showcase_on_press = Callback::new(move |_| {
        set_showcase_checked.update(|value| *value = !*value);
        set_showcase_presses.update(|count| *count += 1);
    });

    let tone_options = vec!["Default".to_string(), "Quiet".to_string()];
    let item_count_options = vec!["0".to_string(), "2".to_string(), "3".to_string()];
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let (workbench_item_count_index, set_workbench_item_count_index) = signal(Some(2_usize));
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_sticky_heading, set_workbench_sticky_heading) = signal(false);
    let (workbench_show_divider, set_workbench_show_divider) = signal(false);
    let (workbench_custom_title, set_workbench_custom_title) = signal(true);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_heading_tone =
        Signal::derive(move || match workbench_tone_index.get().unwrap_or(0) {
            1 => MenuSectionHeadingTone::Quiet,
            _ => MenuSectionHeadingTone::Default,
        });
    let workbench_item_count =
        Signal::derive(
            move || match workbench_item_count_index.get().unwrap_or(2) {
                0 => 0_usize,
                1 => 2_usize,
                _ => 3_usize,
            },
        );

    let (workbench_primary_checked, set_workbench_primary_checked) = signal(true);
    let workbench_primary_kind = MenuItemKind::Radio {
        is_checked: Signal::derive(move || workbench_primary_checked.get()),
    };
    let (workbench_pinned_checked, set_workbench_pinned_checked) = signal(true);
    let workbench_pinned_kind = MenuItemKind::Checkbox {
        is_checked: Signal::derive(move || workbench_pinned_checked.get()),
    };
    let (workbench_presses, set_workbench_presses) = signal(0_u32);
    let workbench_toggle_primary = Callback::new(move |_| {
        set_workbench_primary_checked.update(|value| *value = !*value);
        set_workbench_presses.update(|count| *count += 1);
    });
    let workbench_toggle_pinned = Callback::new(move |_| {
        set_workbench_pinned_checked.update(|value| *value = !*value);
        set_workbench_presses.update(|count| *count += 1);
    });

    let hello_code = Signal::derive(move || {
        r#"<MenuSection
  title="Workspace actions".to_string()
  item_count=3
  aria_label="Workspace actions section".to_string()
>
  <MenuItem index=0 kind=MenuItemKind::Action>"Open workspace"</MenuItem>
  <MenuItem index=1 kind=MenuItemKind::Checkbox { is_checked: Signal::derive(move || checked.get()) } on_press=on_press>
    "Pin workspace"
  </MenuItem>
  <MenuItem index=2 kind=MenuItemKind::Action>"Archive workspace"</MenuItem>
</MenuSection>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let heading_tone = match workbench_heading_tone.get() {
            MenuSectionHeadingTone::Quiet => "MenuSectionHeadingTone::Quiet",
            MenuSectionHeadingTone::Default => "MenuSectionHeadingTone::Default",
        };
        let title = if workbench_custom_title.get() {
            "Routing controls"
        } else {
            ""
        };
        let aria_label = if workbench_custom_aria.get() {
            "Routing menu section"
        } else {
            ""
        };
        let class_name = if workbench_custom_class.get() {
            "docs-menu-section-custom"
        } else {
            ""
        };

        [
            "<MenuSection".to_string(),
            format!("  title={}", rust_string_literal(title)),
            format!("  item_count={}", workbench_item_count.get()),
            format!("  heading_tone={heading_tone}"),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  disabled={}", bool_word(workbench_disabled.get())),
            format!(
                "  sticky_heading={}",
                bool_word(workbench_sticky_heading.get())
            ),
            format!("  show_divider={}", bool_word(workbench_show_divider.get())),
            format!("  aria_label={}", rust_string_literal(aria_label)),
            format!("  class_name={}", rust_string_literal(class_name)),
            ">".to_string(),
            "  ...menu items...".to_string(),
            "</MenuSection>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let title = if workbench_custom_title.get() {
            Some("Routing controls")
        } else {
            Some("")
        };
        let aria_label = if workbench_custom_aria.get() {
            Some("Routing menu section")
        } else {
            Some("")
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-menu-section-custom")
        } else {
            Some("")
        };

        format!(
            "MenuSectionActualConfig {{\n  title: {title:?},\n  item_count: Some({}),\n  heading_tone: {:?},\n  is_disabled: Some({}),\n  disabled: {},\n  sticky_heading: {},\n  show_divider: {},\n  aria_label: {aria_label:?},\n  class_name: {class_name:?},\n  item_feedback: \"presses={}, primary_checked={}, pinned_checked={}\",\n}}",
            workbench_item_count.get(),
            workbench_heading_tone.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled.get()),
            bool_word(workbench_sticky_heading.get()),
            bool_word(workbench_show_divider.get()),
            workbench_presses.get(),
            workbench_primary_checked.get(),
            workbench_pinned_checked.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<MenuSection title="Default section".to_string() item_count=3>
  <MenuItem kind=MenuItemKind::Action>"Open workspace"</MenuItem>
</MenuSection>
<MenuSection title="Quiet sticky".to_string() item_count=2 heading_tone=MenuSectionHeadingTone::Quiet sticky_heading=true show_divider=true>
  <MenuItem kind=MenuItemKind::Action>"Primary route"</MenuItem>
</MenuSection>
<MenuSection title="Disabled empty".to_string() item_count=0 is_disabled=true disabled=true class_name="docs-menu-section-custom".to_string()>
  <span class="ui-muted">"No actions available"</span>
</MenuSection>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="MenuSection"
            slug="menu-section"
            group="Collections"
            description="baseline-style menu section primitive with centralized heading/item/source normalization and stable `slot` + `data-*` contracts."
        >
            <Playground title="Hello World (Default MenuSection)" code_signal=hello_code>
                <MenuSection
                    title="Workspace actions".to_string()
                    item_count=3
                    aria_label="Workspace actions section".to_string()
                >
                    <MenuItem index=0 kind=MenuItemKind::Action>
                        "Open workspace"
                    </MenuItem>
                    <MenuItem
                        index=1
                        kind=showcase_kind
                        on_press=showcase_on_press
                    >
                        "Pin workspace"
                    </MenuItem>
                    <MenuItem index=2 kind=MenuItemKind::Action>
                        "Archive workspace"
                    </MenuItem>
                </MenuSection>
                <span class="ui-muted">
                    "showcase checked: " {move || showcase_checked.get()}
                    " · presses: " {move || showcase_presses.get()}
                </span>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="menu-section-workbench-controls">
                        <SegmentedControl
                            id_base="docs-menu-section-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=workbench_tone_index
                            set_selected_index=set_workbench_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="MenuSection heading tone".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-menu-section-workbench-item-count".to_string()
                            options=item_count_options.clone()
                            selected_index=workbench_item_count_index
                            set_selected_index=set_workbench_item_count_index
                            size=SegmentedControlSize::Sm
                            aria_label="MenuSection item_count".to_string()
                        />
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch checked=workbench_sticky_heading set_checked=set_workbench_sticky_heading>
                            "sticky_heading"
                        </Switch>
                        <Switch checked=workbench_show_divider set_checked=set_workbench_show_divider>
                            "show_divider"
                        </Switch>
                        <Switch checked=workbench_custom_title set_checked=set_workbench_custom_title>
                            "title"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "aria_label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <MenuSection
                        title=if workbench_custom_title.get() {
                            "Routing controls".to_string()
                        } else {
                            String::new()
                        }
                        item_count=workbench_item_count.get()
                        heading_tone=workbench_heading_tone.get()
                        is_disabled=workbench_is_disabled.get()
                        disabled=workbench_disabled.get()
                        sticky_heading=workbench_sticky_heading.get()
                        show_divider=workbench_show_divider.get()
                        aria_label=if workbench_custom_aria.get() {
                            "Routing menu section".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-menu-section-custom".to_string()
                        } else {
                            String::new()
                        }
                    >
                        <MenuItem
                            kind=workbench_primary_kind
                            has_submenu=true
                            on_press=workbench_toggle_primary
                        >
                            "Set as primary route"
                        </MenuItem>
                        <MenuItem
                            kind=workbench_pinned_kind
                            on_press=workbench_toggle_pinned
                        >
                            "Pin fallback route"
                        </MenuItem>
                        <MenuItem kind=MenuItemKind::Action>
                            "Archive route"
                        </MenuItem>
                    </MenuSection>

                    <span class="ui-muted">
                        "workbench presses: " {move || workbench_presses.get()}
                        " · primary: " {move || workbench_primary_checked.get()}
                        " · pinned: " {move || workbench_pinned_checked.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Quiet / Disabled Empty)" code_signal=matrix_code>
                <div class="docs-stack docs-stack--tight">
                    <MenuSection title="Default section".to_string() item_count=3>
                        <MenuItem kind=MenuItemKind::Action>"Open workspace"</MenuItem>
                        <MenuItem kind=MenuItemKind::Action>"Rename workspace"</MenuItem>
                        <MenuItem kind=MenuItemKind::Action>"Archive workspace"</MenuItem>
                    </MenuSection>
                    <MenuSection
                        title="Quiet sticky".to_string()
                        item_count=2
                        heading_tone=MenuSectionHeadingTone::Quiet
                        sticky_heading=true
                        show_divider=true
                    >
                        <MenuItem kind=MenuItemKind::Action>"Primary route"</MenuItem>
                        <MenuItem kind=MenuItemKind::Action>"Fallback route"</MenuItem>
                    </MenuSection>
                    <MenuSection
                        title="Disabled empty".to_string()
                        item_count=0
                        is_disabled=true
                        disabled=true
                        class_name="docs-menu-section-custom".to_string()
                    >
                        <span class="ui-muted">"No actions available"</span>
                    </MenuSection>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
