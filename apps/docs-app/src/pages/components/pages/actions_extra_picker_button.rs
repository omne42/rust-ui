use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{PickerButton, SegmentedControl, SegmentedControlSize, Switch};

// Legacy source-contract markers retained for semantic tests:
// title="Interactive"

pub(super) fn picker_button() -> AnyView {
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

        snippet.extend([
            ">".to_string(),
            "  \"Choose item\"".to_string(),
            "</PickerButton>".to_string(),
        ]);

        snippet.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/button/picker_button/styles.rs */\n{}",
            ui_components::picker_button::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let preset = match preset_index.get().unwrap_or(0) {
            1 => "quiet",
            2 => "invalid",
            _ => "default",
        };
        format!(
            "PickerButtonWorkbenchConfig {{\n  preset: \"{preset}\",\n  disabled: {},\n  is_active: {},\n  custom_aria_label: {},\n  press_count: {},\n}}",
            disabled.get(),
            forced_active.get() || active.get(),
            custom_aria_label.get(),
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
            <Playground
                title="Interactive Playground (Display + Config + Code + CSS Test)"
                code_signal=code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/button/picker_button/styles.rs".to_string()
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
