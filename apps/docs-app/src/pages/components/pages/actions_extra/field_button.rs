use super::*;

pub(crate) fn field_button() -> AnyView {
    let (showcase_presses, set_showcase_presses) = signal(0_u32);
    let on_showcase_press = Callback::new(move |_| {
        set_showcase_presses.update(|count| *count += 1);
    });

    let button_type_options = vec![
        "Button".to_string(),
        "Submit".to_string(),
        "Reset".to_string(),
    ];
    let (workbench_button_type_index, set_workbench_button_type_index) = signal(Some(0_usize));
    let (workbench_is_quiet, set_workbench_is_quiet) = signal(false);
    let (workbench_is_invalid, set_workbench_is_invalid) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_is_active, set_workbench_is_active) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let workbench_button_type =
        Signal::derive(
            move || match workbench_button_type_index.get().unwrap_or(0) {
                1 => ButtonType::Submit,
                2 => ButtonType::Reset,
                _ => ButtonType::Button,
            },
        );
    let workbench_node_ref = NodeRef::new();
    let (workbench_presses, set_workbench_presses) = signal(0_u32);
    let on_workbench_press = Callback::new(move |_| {
        set_workbench_presses.update(|count| *count += 1);
    });

    let hello_code = Signal::derive(move || {
        r#"<FieldButton aria_label="Open options".to_string() on_press=on_press>
  "Options"
</FieldButton>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let button_type = match workbench_button_type.get() {
            ButtonType::Submit => "ui::ButtonType::Submit",
            ButtonType::Reset => "ui::ButtonType::Reset",
            ButtonType::Button => "ui::ButtonType::Button",
        };
        let aria_label = if workbench_custom_aria.get() {
            "FieldButton workbench"
        } else {
            ""
        };
        let class_name = if workbench_custom_class.get() {
            "docs-field-button-custom"
        } else {
            ""
        };

        [
            "<FieldButton".to_string(),
            format!("  is_quiet={}", bool_word(workbench_is_quiet.get())),
            format!("  is_invalid={}", bool_word(workbench_is_invalid.get())),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  is_active={}", bool_word(workbench_is_active.get())),
            format!("  aria_label={}", rust_string_literal(aria_label)),
            format!("  class_name={}", rust_string_literal(class_name)),
            format!("  button_type={button_type}"),
            "  node_ref=node_ref".to_string(),
            "  on_press=on_press".to_string(),
            ">".to_string(),
            "  \"Field action\"".to_string(),
            "</FieldButton>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let aria_label = if workbench_custom_aria.get() {
            Some("FieldButton workbench")
        } else {
            Some("")
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-field-button-custom")
        } else {
            Some("")
        };

        format!(
            "FieldButtonActualConfig {{\n  is_quiet: {},\n  is_invalid: {},\n  is_disabled: {},\n  is_active: {},\n  aria_label: {aria_label:?},\n  class_name: {class_name:?},\n  button_type: {:?},\n  node_ref: Some(\"field_button_node_ref\"),\n  on_press: \"count={}\",\n}}",
            bool_word(workbench_is_quiet.get()),
            bool_word(workbench_is_invalid.get()),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_is_active.get()),
            workbench_button_type.get(),
            workbench_presses.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<FieldButton aria_label="Default trigger".to_string()>"Default"</FieldButton>
<FieldButton is_quiet=true button_type=ui::ButtonType::Submit aria_label="Quiet submit".to_string()>"Quiet submit"</FieldButton>
<FieldButton is_invalid=true is_active=true class_name="docs-field-button-custom".to_string() aria_label="Invalid active".to_string()>"Invalid"</FieldButton>
<FieldButton is_disabled=true button_type=ui::ButtonType::Reset aria_label="Disabled reset".to_string()>"Disabled"</FieldButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="FieldButton"
            slug="field-button"
            group="Actions"
            description="baseline-style field trigger button with centralized quiet/invalid/active/disabled state contracts and headless press/hover/focus behavior."
        >
            <Playground title="Hello World (Default FieldButton)" code_signal=hello_code>
                <div class="docs-row">
                    <FieldButton
                        aria_label="Open options".to_string()
                        on_press=on_showcase_press
                    >
                        "Options"
                    </FieldButton>
                    <span class="ui-muted">"on_press count: " {move || showcase_presses.get()}</span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="field-button-workbench-controls">
                        <SegmentedControl
                            id_base="docs-field-button-workbench-type".to_string()
                            options=button_type_options.clone()
                            selected_index=workbench_button_type_index
                            set_selected_index=set_workbench_button_type_index
                            size=SegmentedControlSize::Sm
                            aria_label="FieldButton button_type".to_string()
                        />
                        <Switch checked=workbench_is_quiet set_checked=set_workbench_is_quiet>
                            "is_quiet"
                        </Switch>
                        <Switch checked=workbench_is_invalid set_checked=set_workbench_is_invalid>
                            "is_invalid"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_is_active set_checked=set_workbench_is_active>
                            "is_active"
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
                <div class="docs-row">
                    <FieldButton
                        is_quiet=workbench_is_quiet.get()
                        is_invalid=workbench_is_invalid.get()
                        is_disabled=workbench_is_disabled.get()
                        is_active=workbench_is_active.get()
                        aria_label=if workbench_custom_aria.get() {
                            "FieldButton workbench".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-field-button-custom".to_string()
                        } else {
                            String::new()
                        }
                        button_type=workbench_button_type.get()
                        node_ref=workbench_node_ref
                        on_press=on_workbench_press
                    >
                        "Field action"
                    </FieldButton>
                    <span class="ui-muted">"on_press count: " {move || workbench_presses.get()}</span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Quiet / Invalid / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <FieldButton aria_label="Default trigger".to_string()>"Default"</FieldButton>
                    <FieldButton
                        is_quiet=true
                        button_type=ButtonType::Submit
                        aria_label="Quiet submit".to_string()
                    >
                        "Quiet submit"
                    </FieldButton>
                    <FieldButton
                        is_invalid=true
                        is_active=true
                        class_name="docs-field-button-custom".to_string()
                        aria_label="Invalid active".to_string()
                    >
                        "Invalid"
                    </FieldButton>
                    <FieldButton
                        is_disabled=true
                        button_type=ButtonType::Reset
                        aria_label="Disabled reset".to_string()
                    >
                        "Disabled"
                    </FieldButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
