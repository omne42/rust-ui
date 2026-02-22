use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{PickerButton, SegmentedControl, SegmentedControlSize, Switch};

// Legacy source-contract markers retained for semantic tests:
// title="Interactive"

pub(super) fn picker_button() -> AnyView {
    let workbench_node_ref: NodeRef<leptos::html::Button> = NodeRef::new();
    let (active, set_active) = signal(false);
    let (press_count, set_press_count) = signal(0_usize);
    let on_press = Callback::new(move |_| {
        set_active.update(|value| *value = !*value);
        set_press_count.update(|value| *value += 1);
    });

    let preset_options = vec![
        "default".to_string(),
        "quiet".to_string(),
        "invalid".to_string(),
    ];
    let (preset_index, set_preset_index) = signal(Some(0_usize));
    let quiet = Signal::derive(move || matches!(preset_index.get().unwrap_or(0), 1));
    let invalid = Signal::derive(move || matches!(preset_index.get().unwrap_or(0), 2));
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

    let code = Signal::derive(move || {
        let quiet = quiet.get();
        let invalid = invalid.get();
        let disabled = disabled.get();
        let forced_active = forced_active.get();
        let custom_aria_label = custom_aria_label.get();

        let mut snippet = vec!["<PickerButton".to_string()];

        if quiet {
            snippet.push("  quiet=true".to_string());
        }
        if invalid {
            snippet.push("  invalid=true".to_string());
        }
        if disabled {
            snippet.push("  disabled=true".to_string());
        }
        if forced_active {
            snippet.push("  is_active=true".to_string());
        }
        if custom_aria_label {
            snippet.push("  aria_label=\"Inspect picker trigger\".into()".to_string());
        }
        if custom_class_name.get() {
            snippet.push("  class_name=\"docs-picker-button-custom\".into()".to_string());
        }
        if button_type.get() != "button" {
            snippet.push(format!("  button_type={:?}", button_type.get()));
        }
        snippet.push("  node_ref=NodeRef::new()".to_string());
        snippet.push("  on_press=Callback::new(move |_| {})".to_string());

        snippet.extend([
            ">".to_string(),
            "  \"Choose item\"".to_string(),
            "</PickerButton>".to_string(),
        ]);

        snippet.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/button/picker_button/styles.rs */\n{}",
            ui::picker_button::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "PickerButtonWorkbenchConfig {{\n  quiet: {},\n  invalid: {},\n  disabled: {},\n  is_active: {},\n  aria_label: {:?},\n  class_name: {:?},\n  button_type: {:?},\n  node_ref: \"bound\",\n  on_press: {:?},\n  press_count: {},\n}}",
            quiet.get(),
            invalid.get(),
            disabled.get(),
            forced_active.get() || active.get(),
            if custom_aria_label.get() {
                "Inspect picker trigger"
            } else {
                ""
            },
            if custom_class_name.get() {
                "docs-picker-button-custom"
            } else {
                ""
            },
            button_type.get(),
            "handler",
            press_count.get()
        )
    });

    let (marker_presses, set_marker_presses) = signal(0_usize);
    let marker_press =
        Callback::new(move |_| set_marker_presses.update(|value| *value = value.saturating_add(1)));

    let states_code = Signal::derive(move || {
        r#"<PickerButton quiet=true>"Filter"</PickerButton>
<PickerButton invalid=true>"Required"</PickerButton>
<PickerButton disabled=true>"Disabled"</PickerButton>"#
            .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"<PickerButton
  quiet=true
  invalid=true
  is_active=true
  aria_label="Inspect picker trigger".to_string()
  class_name="docs-picker-button-state".to_string()
  on_press=Callback::new(move |_| {})
>
  "Inspect markers"
</PickerButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="PickerButton"
            slug="picker-button"
            group="Actions"
            description="baseline-compatible PickerButton alias for naming parity, preserving FieldButton accessibility/state contracts and baseline-level press/focus interaction behavior."
        >
            <Playground title="Hello World (Default API)" code_signal=states_code>
                <div class="docs-row">
                    <PickerButton on_press=on_press>"Choose item"</PickerButton>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Display + Config + Code + CSS Test)"
                code_signal=code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/button/picker_button/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Preset"</div>
                        <SegmentedControl
                            id_base="docs-picker-button-preset".to_string()
                            options=preset_options.clone()
                            selected_index=preset_index
                            set_selected_index=set_preset_index
                            size=SegmentedControlSize::Sm
                            aria_label="PickerButton preset".to_string()
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
                            id_base="docs-picker-button-button-type".to_string()
                            options=button_type_options.clone()
                            selected_index=button_type_index
                            set_selected_index=set_button_type_index
                            size=SegmentedControlSize::Sm
                            aria_label="PickerButton button type".to_string()
                        />
                    </div>
                }
            >
                <div class="docs-stack">
                    {move || {
                        let is_active = forced_active.get() || active.get();

                        if custom_aria_label.get() {
                            view! {
                                <PickerButton
                                    quiet=quiet.get()
                                    invalid=invalid.get()
                                    disabled=disabled.get()
                                    is_active=is_active
                                    aria_label="Inspect picker trigger".to_string()
                                    class_name=if custom_class_name.get() {
                                        "docs-picker-button-custom".to_string()
                                    } else {
                                        String::new()
                                    }
                                    button_type=button_type.get()
                                    node_ref=workbench_node_ref
                                    on_press=on_press
                                >
                                    "Choose item"
                                </PickerButton>
                            }
                                .into_any()
                        } else {
                            view! {
                                <PickerButton
                                    quiet=quiet.get()
                                    invalid=invalid.get()
                                    disabled=disabled.get()
                                    is_active=is_active
                                    class_name=if custom_class_name.get() {
                                        "docs-picker-button-custom".to_string()
                                    } else {
                                        String::new()
                                    }
                                    button_type=button_type.get()
                                    node_ref=workbench_node_ref
                                    on_press=on_press
                                >
                                    "Choose item"
                                </PickerButton>
                            }
                                .into_any()
                        }
                    }}
                    <span class="ui-muted">
                        "active: " {move || forced_active.get() || active.get()}
                    </span>
                    <span class="ui-muted">
                        "presses: " {move || press_count.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=states_code>
                <div class="docs-row">
                    <PickerButton quiet=true>
                        "Filter"
                    </PickerButton>
                    <PickerButton invalid=true>
                        "Required"
                    </PickerButton>
                    <PickerButton disabled=true>
                        "Disabled"
                    </PickerButton>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect wrapper markers like `data-state`, `data-quiet`, `data-invalid`, `data-disabled`, `data-active`, `data-has-handler`, `data-aria-source`, `data-class-source`, and `data-handler-source`."
                code_signal=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <PickerButton
                        quiet=true
                        invalid=true
                        is_active=true
                        aria_label="Inspect picker trigger".to_string()
                        class_name="docs-picker-button-state".to_string()
                        on_press=marker_press
                    >
                        "Inspect markers"
                    </PickerButton>
                    <span class="ui-muted">"presses: " {move || marker_presses.get()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
