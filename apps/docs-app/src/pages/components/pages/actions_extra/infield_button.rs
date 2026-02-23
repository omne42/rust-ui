use super::*;

pub(crate) fn infield_button() -> AnyView {
    let workbench_node_ref: NodeRef<leptos::html::Button> = NodeRef::new();
    let preset_options = vec![
        "Default".to_string(),
        "Quiet".to_string(),
        "Invalid".to_string(),
    ];
    let (preset_index, set_preset_index) = signal(Some(0_usize));
    let (disabled, set_disabled) = signal(false);
    let (forced_active, set_forced_active) = signal(false);
    let (custom_aria_label, set_custom_aria_label) = signal(false);
    let (custom_class_name, set_custom_class_name) = signal(false);
    let button_type_options = vec![
        "button".to_string(),
        "submit".to_string(),
        "reset".to_string(),
    ];
    let (button_type_index, set_button_type_index) = signal(Some(0_usize));
    let button_type = Signal::derive(move || match button_type_index.get().unwrap_or(0) {
        1 => "submit",
        2 => "reset",
        _ => "button",
    });
    let (press_count, set_press_count) = signal(0_usize);

    let quiet = Signal::derive(move || preset_index.get().unwrap_or(0) == 1);
    let invalid = Signal::derive(move || preset_index.get().unwrap_or(0) == 2);

    let on_press = Callback::new(move |_| {
        set_press_count.update(|next| *next += 1);
    });

    let workbench_code = Signal::derive(move || {
        let mut lines = vec!["<InfieldButton".to_string()];
        if quiet.get() {
            lines.push("  quiet=true".to_string());
        }
        if invalid.get() {
            lines.push("  invalid=true".to_string());
        }
        if disabled.get() {
            lines.push("  disabled=true".to_string());
        }
        if forced_active.get() {
            lines.push("  is_active=true".to_string());
        }
        if custom_aria_label.get() {
            lines.push("  aria_label=\"Inspect in-field trigger\".into()".to_string());
        }
        if custom_class_name.get() {
            lines.push("  class_name=\"docs-infield-button-custom\".into()".to_string());
        }
        if button_type.get() != "button" {
            lines.push(format!("  button_type={:?}", button_type.get()));
        }
        lines.push("  node_ref=NodeRef::new()".to_string());
        lines.push("  on_press=Callback::new(move |_| {})".to_string());
        lines.push(">".to_string());
        lines.push("  \"⋯\"".to_string());
        lines.push("</InfieldButton>".to_string());
        lines.join("\n")
    });

    let comparison_code = Signal::derive(move || {
        r#"<InfieldButton>"Default"</InfieldButton>
<InfieldButton quiet=true>"Quiet"</InfieldButton>
<InfieldButton invalid=true is_active=true>"Invalid + Active"</InfieldButton>
<InfieldButton disabled=true>"Disabled"</InfieldButton>"#
            .to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/button/infield_button/styles.rs */\n{}",
            ui::infield_button::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "InfieldButtonWorkbenchConfig {{\n  quiet: {},\n  invalid: {},\n  disabled: {},\n  is_active: {},\n  aria_label: {:?},\n  class_name: {:?},\n  button_type: {:?},\n  node_ref: \"bound\",\n  on_press: {:?},\n  press_count: {},\n}}",
            quiet.get(),
            invalid.get(),
            disabled.get(),
            forced_active.get(),
            if custom_aria_label.get() {
                "Inspect in-field trigger"
            } else {
                ""
            },
            if custom_class_name.get() {
                "docs-infield-button-custom"
            } else {
                ""
            },
            button_type.get(),
            "handler",
            press_count.get()
        )
    });

    view! {
        <ComponentPage
            title="InfieldButton"
            slug="infield-button"
            group="Actions"
            description="baseline-compatible in-field trigger button with centralized quiet/invalid/active/disabled state contracts and headless press/hover/focus behavior."
        >
            <Playground title="Hello World (Default API)" code_signal=comparison_code>
                <div class="docs-row">
                    <InfieldButton on_press=on_press>"⋯"</InfieldButton>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/button/infield_button/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Preset"</div>
                        <SegmentedControl
                            id_base="docs-infield-button-preset".to_string()
                            options=preset_options.clone()
                            selected_index=preset_index
                            set_selected_index=set_preset_index
                            size=SegmentedControlSize::Sm
                            aria_label="InfieldButton preset".to_string()
                        />
                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=forced_active set_checked=set_forced_active>
                            "Force active"
                        </Switch>
                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                        <Switch checked=custom_class_name set_checked=set_custom_class_name>
                            "Custom class"
                        </Switch>
                        <div class="docs-search__label">"Button type"</div>
                        <SegmentedControl
                            id_base="docs-infield-button-button-type".to_string()
                            options=button_type_options.clone()
                            selected_index=button_type_index
                            set_selected_index=set_button_type_index
                            size=SegmentedControlSize::Sm
                            aria_label="InfieldButton button type".to_string()
                        />
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        if custom_aria_label.get() {
                            view! {
                                <InfieldButton
                                    quiet=quiet.get()
                                    invalid=invalid.get()
                                    disabled=disabled.get()
                                    is_active=forced_active.get()
                                    aria_label="Inspect in-field trigger".to_string()
                                    class_name=if custom_class_name.get() {
                                        "docs-infield-button-custom".to_string()
                                    } else {
                                        String::new()
                                    }
                                    button_type=button_type.get()
                                    node_ref=workbench_node_ref
                                    on_press=on_press
                                >
                                    "⋯"
                                </InfieldButton>
                            }
                                .into_any()
                        } else {
                            view! {
                                <InfieldButton
                                    quiet=quiet.get()
                                    invalid=invalid.get()
                                    disabled=disabled.get()
                                    is_active=forced_active.get()
                                    class_name=if custom_class_name.get() {
                                        "docs-infield-button-custom".to_string()
                                    } else {
                                        String::new()
                                    }
                                    button_type=button_type.get()
                                    node_ref=workbench_node_ref
                                    on_press=on_press
                                >
                                    "⋯"
                                </InfieldButton>
                            }
                                .into_any()
                        }
                    }}
                    <span class="ui-muted">"presses: " {move || press_count.get().to_string()}</span>
                </div>
            </Playground>

            <Playground title="Comparison Matrix (Default / Quiet / Invalid+Active / Disabled)" code_signal=comparison_code>
                <div class="docs-row">
                    <InfieldButton>"Default"</InfieldButton>
                    <InfieldButton quiet=true>"Quiet"</InfieldButton>
                    <InfieldButton invalid=true is_active=true>"Invalid + Active"</InfieldButton>
                    <InfieldButton disabled=true>"Disabled"</InfieldButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
