use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{FieldLabel, FieldLabelTone};

const FIELD_LABEL_DOC_IMPORTS: &str =
    "use leptos::prelude::*;\nuse ui_components::{FieldLabel, FieldLabelTone};";

pub(super) fn field_label() -> AnyView {
    let hello_code = Signal::derive(move || {
        r#"<FieldLabel text=\"Email\".into() for_id=\"email\".into() is_required=true />
<input id=\"email\" type=\"email\" />"#
            .to_string()
    });

    let tone_code = Signal::derive(move || {
        r#"<FieldLabel text=\"Email\".into() for_id=\"email\".into() is_required=true />
<FieldLabel text=\"Helper\".into() tone=FieldLabelTone::Muted />
<FieldLabel text=\"Critical\".into() tone=FieldLabelTone::Strong is_required=true />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<FieldLabel
  text=\"Assignee\".into()
  for_id=\"assignee\".into()
  is_required=true
  required_indicator=\"(required)\".into()
  aria_label=\"Assignee field label\".into()
  class_name=\"docs-field-label-custom\".into()
/>"#
        .to_string()
    });

    let controlled_na_code = Signal::derive(move || {
        r#"// FieldLabel has no controlled axis (no value/on_change/default_value triad).
// It always renders from the full snapshot props passed by parent.
<FieldLabel text=\"Display Name\".into() for_id=\"display-name\".into() is_required=true />"#
            .to_string()
    });

    let streaming_snapshot_code = Signal::derive(move || {
        r#"// Streaming is optional for FieldLabel. Snapshot rendering is always supported.
<FieldLabel text=\"Reviewer\".into() for_id=\"reviewer\".into() is_required=true />"#
            .to_string()
    });

    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let workbench_tone = Signal::derive(move || match workbench_tone_index.get().unwrap_or(0) {
        1 => FieldLabelTone::Muted,
        2 => FieldLabelTone::Strong,
        _ => FieldLabelTone::Default,
    });
    let (workbench_required, set_workbench_required) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_has_for, set_workbench_has_for) = signal(true);
    let (workbench_custom_indicator, set_workbench_custom_indicator) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_code = Signal::derive(move || {
        format!(
            "<FieldLabel\n  text=\"Workbench\".into()\n  tone=FieldLabelTone::{:?}\n  is_required={}\n  is_disabled={}\n  for_id={}\n  required_indicator={}\n  aria_label={}\n  class_name={}\n/>",
            workbench_tone.get(),
            workbench_required.get(),
            workbench_disabled.get(),
            if workbench_has_for.get() {
                "\"docs-field-label-workbench\".into()"
            } else {
                "\"\".into()"
            },
            if workbench_custom_indicator.get() {
                "\"(required)\".into()"
            } else {
                "\"\".into()"
            },
            if workbench_custom_aria.get() {
                "\"Workbench field label\".into()"
            } else {
                "\"\".into()"
            },
            if workbench_custom_class.get() {
                "\"docs-field-label-custom\".into()"
            } else {
                "\"\".into()"
            }
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/field_form/field_label/styles.rs */\n{}",
            ui_components::field_form::field_label::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let tone = workbench_tone.get();
        let required = workbench_required.get();
        let disabled = workbench_disabled.get();
        let has_for = workbench_has_for.get();
        let custom_indicator = workbench_custom_indicator.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();

        let mut classes = vec!["ui-field-label".to_string(), tone.class_name().into()];
        if required {
            classes.push("ui-field-label--required".to_string());
        }
        if disabled {
            classes.push("ui-field-label--disabled".to_string());
        }
        if has_for {
            classes.push("ui-field-label--for".to_string());
        }
        if custom_indicator {
            classes.push("ui-field-label--indicator-custom".to_string());
        }
        if custom_aria {
            classes.push("ui-field-label--aria-custom".to_string());
        }
        if custom_class {
            classes.push("ui-field-label--custom-class".to_string());
            classes.push("docs-field-label-custom".to_string());
        }

        format!(
            "FieldLabelActualConfig {{\n  tone: {tone:?},\n  required: {required},\n  disabled: {disabled},\n  has_for_id: {has_for},\n  indicator_source: \"{}\",\n  aria_source: \"{}\",\n  class_source: \"{}\",\n  data_state: \"{}\",\n  class: \"{}\",\n}}",
            if custom_indicator {
                "custom"
            } else {
                "default"
            },
            if custom_aria { "custom" } else { "default" },
            if custom_class { "custom" } else { "default" },
            if required { "required" } else { "optional" },
            classes.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="FieldLabel"
            slug="field-label"
            group="Forms"
            description="baseline-compatible field label primitive with centralized tone/required/source-state modeling and stable data contracts."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=hello_code
                code_imports=FIELD_LABEL_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack" data-visual-baseline="field-label-default">
                    <FieldLabel
                        text="Email".to_string()
                        for_id="docs-field-label-hello".to_string()
                        is_required=true
                    />
                    <input
                        id="docs-field-label-hello"
                        class="docs-search__input"
                        type="email"
                        placeholder="name@example.com"
                    />
                </div>
            </Playground>

            <Playground
                title="Tone + Required"
                code_signal=tone_code
                code_imports=FIELD_LABEL_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack">
                    <FieldLabel
                        text="Email".to_string()
                        for_id="docs-field-label-email".to_string()
                        is_required=true
                    />
                    <input
                        id="docs-field-label-email"
                        class="docs-search__input"
                        type="email"
                        placeholder="name@example.com"
                    />

                    <FieldLabel text="Helper".to_string() tone=FieldLabelTone::Muted />
                    <FieldLabel
                        text="Critical".to_string()
                        tone=FieldLabelTone::Strong
                        is_required=true
                    />
                </div>
            </Playground>

            <Playground
                title="Custom Indicator + Aria + Class"
                code_signal=custom_code
                code_imports=FIELD_LABEL_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack">
                    <FieldLabel
                        text="Assignee".to_string()
                        for_id="docs-field-label-assignee".to_string()
                        is_required=true
                        required_indicator="(required)".to_string()
                        aria_label="Assignee field label".to_string()
                        class_name="docs-field-label-custom".to_string()
                    />
                    <input
                        id="docs-field-label-assignee"
                        class="docs-search__input"
                        type="text"
                        placeholder="Owner"
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A for FieldLabel)"
                code_signal=controlled_na_code
                code_imports=FIELD_LABEL_DOC_IMPORTS.to_string()
                description="FieldLabel has no controllable value axis; parent passes a full snapshot props set each render."
            >
                <div class="docs-stack">
                    <FieldLabel
                        text="Display Name".to_string()
                        for_id="docs-field-label-controlled-na".to_string()
                        is_required=true
                    />
                    <input
                        id="docs-field-label-controlled-na"
                        class="docs-search__input"
                        type="text"
                        placeholder="Jane Doe"
                    />
                    <p class="ui-muted">
                        "No value/on_change/default_value triad. Controlled/uncontrolled contrast is N/A."
                    </p>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                code_signal=streaming_snapshot_code
                code_imports=FIELD_LABEL_DOC_IMPORTS.to_string()
                description="FieldLabel is snapshot-first; streaming stays optional with snapshot fallback."
            >
                <div class="docs-stack">
                    <FieldLabel
                        text="Reviewer".to_string()
                        for_id="docs-field-label-streaming".to_string()
                        is_required=true
                    />
                    <input
                        id="docs-field-label-streaming"
                        class="docs-search__input"
                        type="text"
                        placeholder="Owner"
                    />
                    <p class="ui-muted">
                        "Agent contract markers: data-ui-streaming=optional data-ui-fallback=snapshot data-ui-output-state=verified"
                    </p>
                </div>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                code_imports=FIELD_LABEL_DOC_IMPORTS.to_string()
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/field_form/field_label/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                description="展示区对比 default/workbench；Config 调 tone/required/disabled/source，Code 与 CSS Test 用于契约检查。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="field-label-config-controls">
                        <button
                            type="button"
                            data-action="cycle-tone-config"
                            on:click=move |_| {
                                set_workbench_tone_index.update(|value| {
                                    *value = Some((value.unwrap_or(0) + 1) % 3);
                                });
                            }
                        >
                            "Cycle tone"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-required-config"
                            on:click=move |_| {
                                set_workbench_required.update(|value| *value = !*value);
                            }
                        >
                            "Toggle required"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-disabled-config"
                            on:click=move |_| {
                                set_workbench_disabled.update(|value| *value = !*value);
                            }
                        >
                            "Toggle disabled"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-for-config"
                            on:click=move |_| {
                                set_workbench_has_for.update(|value| *value = !*value);
                            }
                        >
                            "Toggle for_id"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-indicator-config"
                            on:click=move |_| {
                                set_workbench_custom_indicator.update(|value| *value = !*value);
                            }
                        >
                            "Toggle custom indicator"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-aria-config"
                            on:click=move |_| {
                                set_workbench_custom_aria.update(|value| *value = !*value);
                            }
                        >
                            "Toggle custom aria"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-class-config"
                            on:click=move |_| {
                                set_workbench_custom_class.update(|value| *value = !*value);
                            }
                        >
                            "Toggle custom class"
                        </button>
                        <p class="ui-muted" data-slot="field-label-config-summary">
                            {move || {
                                format!(
                                    "config: tone={:?} required={} disabled={} for={} indicator={} aria={} class={}",
                                    workbench_tone.get(),
                                    workbench_required.get(),
                                    workbench_disabled.get(),
                                    workbench_has_for.get(),
                                    if workbench_custom_indicator.get() { "custom" } else { "default" },
                                    if workbench_custom_aria.get() { "custom" } else { "default" },
                                    if workbench_custom_class.get() { "custom" } else { "default" },
                                )
                            }}
                        </p>
                    </div>
                }
            >
                <div class="docs-stack">
                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"default"</span>
                        <FieldLabel
                            text="Email".to_string()
                            for_id="docs-field-label-compare-default".to_string()
                            is_required=true
                        />
                        <input
                            id="docs-field-label-compare-default"
                            class="docs-search__input"
                            type="email"
                            placeholder="default@example.com"
                        />
                    </div>

                    <div class="docs-stack docs-stack--tight">
                        <span class="ui-muted">"workbench"</span>
                        <FieldLabel
                            text="Workbench".to_string()
                            tone=workbench_tone.get()
                            is_required=workbench_required.get()
                            is_disabled=workbench_disabled.get()
                            for_id=if workbench_has_for.get() {
                                "docs-field-label-workbench".to_string()
                            } else {
                                "".to_string()
                            }
                            required_indicator=if workbench_custom_indicator.get() {
                                "(required)".to_string()
                            } else {
                                "".to_string()
                            }
                            aria_label=if workbench_custom_aria.get() {
                                "Workbench field label".to_string()
                            } else {
                                "".to_string()
                            }
                            class_name=if workbench_custom_class.get() {
                                "docs-field-label-custom".to_string()
                            } else {
                                "".to_string()
                            }
                        />
                        <input
                            id="docs-field-label-workbench"
                            class="docs-search__input"
                            type="text"
                            placeholder="workbench-owner"
                            disabled=workbench_disabled.get()
                        />
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
