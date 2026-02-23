use super::*;

pub(crate) fn field() -> AnyView {
    let field_imports = "use leptos::prelude::*;\nuse ui::{Field, FieldOrientation, FieldTone, field_form::field::FieldMotion};".to_string();

    let persisted_workbench_state = load_field_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let FieldWorkbenchState {
        orientation_key: initial_orientation_key,
        tone_key: initial_tone_key,
        required: initial_required,
        invalid: initial_invalid,
        disabled: initial_disabled,
        custom_class: initial_custom_class,
        custom_error: initial_custom_error,
        motion_ms: initial_motion_ms,
    } = persisted_workbench_state.unwrap_or_default();

    let (workbench_orientation_key, set_workbench_orientation_key) =
        signal(initial_orientation_key);
    let (workbench_tone_key, set_workbench_tone_key) = signal(initial_tone_key);
    let (workbench_required, set_workbench_required) = signal(initial_required);
    let (workbench_invalid, set_workbench_invalid) = signal(initial_invalid);
    let (workbench_disabled, set_workbench_disabled) = signal(initial_disabled);
    let (workbench_custom_class, set_workbench_custom_class) = signal(initial_custom_class);
    let (workbench_custom_error, set_workbench_custom_error) = signal(initial_custom_error);
    let (workbench_motion_ms, set_workbench_motion_ms) = signal(initial_motion_ms);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);

    Effect::new(move |_| {
        let state = FieldWorkbenchState {
            orientation_key: workbench_orientation_key.get(),
            tone_key: workbench_tone_key.get(),
            required: workbench_required.get(),
            invalid: workbench_invalid.get(),
            disabled: workbench_disabled.get(),
            custom_class: workbench_custom_class.get(),
            custom_error: workbench_custom_error.get(),
            motion_ms: workbench_motion_ms.get(),
        };

        if workbench_persist_state.get() {
            save_field_workbench_state(state);
        } else {
            clear_field_workbench_state();
        }
    });

    let workbench_orientation =
        Signal::derive(move || match workbench_orientation_key.get().as_str() {
            "horizontal" => FieldOrientation::Horizontal,
            _ => FieldOrientation::Vertical,
        });
    let workbench_tone = Signal::derive(move || match workbench_tone_key.get().as_str() {
        "muted" => FieldTone::Muted,
        _ => FieldTone::Default,
    });

    let workbench_code = Signal::derive(move || {
        let orientation = workbench_orientation.get();
        let tone = workbench_tone.get();
        let required = workbench_required.get();
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let custom_class = workbench_custom_class.get();
        let custom_error = workbench_custom_error.get();
        let motion_ms = workbench_motion_ms.get();

        let mut snippet = vec!["<Field".to_string()];
        if orientation != FieldOrientation::Vertical {
            snippet.push(format!("  orientation=FieldOrientation::{orientation:?}"));
        }
        if tone != FieldTone::Default {
            snippet.push(format!("  tone=FieldTone::{tone:?}"));
        }
        if required {
            snippet.push("  is_required=true".to_string());
        }
        if invalid {
            snippet.push("  is_invalid=true".to_string());
        }
        if disabled {
            snippet.push("  is_disabled=true".to_string());
        }
        if custom_error {
            snippet.push("  error_message=\"Custom validation error\".into()".to_string());
        }
        if custom_class {
            snippet.push("  class_name=\"docs-field-custom\".into()".to_string());
        }
        if motion_ms != 160 {
            snippet.push(format!(
                "  motion=FieldMotion {{ duration_ms: {motion_ms}.0 }}"
            ));
        }
        snippet.push("  label=\"Email\".into()".to_string());
        snippet.push("  description=\"Inspect contracts in test panel.\".into()".to_string());
        snippet.push("  aria_label=\"Workbench field\".into()".to_string());
        snippet.push("  lang=\"en-US\".into()".to_string());
        snippet.push("  dir=ui_headless::A11yDirection::Ltr".to_string());
        snippet.push(">".to_string());
        snippet.push(
            "  <input class=\"docs-search__input\" type=\"email\" placeholder=\"owner@company.com\" />".to_string(),
        );
        snippet.push("</Field>".to_string());
        snippet.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui/src/field_form/field/styles.rs */\n{}",
            ui::field_form::field::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let orientation = workbench_orientation.get();
        let tone = workbench_tone.get();
        let required = workbench_required.get();
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let custom_class = workbench_custom_class.get();
        let custom_error = workbench_custom_error.get();
        let motion_ms = workbench_motion_ms.get();
        let persist = workbench_persist_state.get();

        let mut classes = vec![
            "ui-field".to_string(),
            orientation.class_name().into(),
            tone.class_name().into(),
        ];
        if required {
            classes.push("ui-field--required".to_string());
        }
        if disabled {
            classes.push("ui-field--disabled".to_string());
        }
        if invalid {
            classes.push("ui-field--invalid".to_string());
        }
        classes.push("ui-field--has-label".to_string());
        classes.push("ui-field--has-description".to_string());
        if invalid {
            classes.push("ui-field--has-error".to_string());
        }
        if custom_class {
            classes.push("ui-field--custom-class".to_string());
            classes.push("docs-field-custom".to_string());
        }

        let data_state = if invalid && disabled {
            "invalid-disabled"
        } else if invalid {
            "invalid"
        } else if disabled {
            "disabled"
        } else if required {
            "required"
        } else if orientation == FieldOrientation::Horizontal {
            "horizontal"
        } else if tone == FieldTone::Muted {
            "muted"
        } else {
            "default"
        };

        format!(
            "FieldActualConfig {{\n  orientation: {orientation:?},\n  tone: {tone:?},\n  is_required: {required},\n  is_invalid: {invalid},\n  is_disabled: {disabled},\n  label: {:?},\n  description: {:?},\n  error_message: {:?},\n  motion: {:?},\n  aria_label: {:?},\n  lang: {:?},\n  dir: {:?},\n  class_name: {:?},\n  custom_error: {custom_error},\n  custom_class: {custom_class},\n  motion_ms: {motion_ms},\n  persist: {},\n  data_state: \"{data_state}\",\n  error_source: \"{}\",\n  class_source: \"{}\",\n  class: \"{}\",\n}}",
            Some("Email"),
            Some("Inspect source/state marker contracts"),
            if invalid {
                Some(if custom_error {
                    "Custom validation error"
                } else {
                    "A valid email is required"
                })
            } else {
                None
            },
            FieldMotion {
                duration_ms: f64::from(motion_ms),
                ..FieldMotion::default()
            },
            Some("Workbench field"),
            Some("en-US"),
            ui_headless::A11yDirection::Ltr,
            if custom_class {
                Some("docs-field-custom")
            } else {
                None
            },
            if persist { "on" } else { "off" },
            if !invalid {
                "none"
            } else if custom_error {
                "custom"
            } else {
                "default"
            },
            if custom_class { "custom" } else { "default" },
            classes.join(" "),
        )
    });

    let hello_world_code = Signal::derive(move || {
        r#"<Field label="Email".to_string()>
  <input class="docs-search__input" type="email" placeholder="name@example.com" />
</Field>"#
            .to_string()
    });

    let required_code = Signal::derive(move || {
        r#"<Field
  label="Email".to_string()
  required=true
  description="We'll only use this for release notes.".to_string()
>
  <input class="docs-search__input" type="email" placeholder="name@example.com" />
</Field>"#
            .to_string()
    });

    let invalid_code = Signal::derive(move || {
        r#"<Field
  orientation=FieldOrientation::Horizontal
  tone=FieldTone::Muted
  invalid=true
  error_message="A valid email is required".to_string()
  class_name="docs-field-custom".to_string()
>
  <input class="docs-search__input" type="email" placeholder="owner@company.com" />
</Field>"#
            .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<Field
  label="Email".to_string()
  required=true
  description="Required: this field must be provided.".to_string()
>
  <input class="docs-search__input" type="email" placeholder="name@example.com" />
</Field>
<Field
  label="Email".to_string()
  invalid=true
  error_message="A valid email is required".to_string()
>
  <input class="docs-search__input" type="email" placeholder="owner@company.com" />
</Field>
<Field
  label="Email".to_string()
  disabled=true
  description="Disabled: read-only snapshot.".to_string()
>
  <input class="docs-search__input" type="email" placeholder="disabled@example.com" />
</Field>"#
            .to_string()
    });

    let controlled_uncontrolled_code = Signal::derive(move || {
        r#"// Uncontrolled-style: pass final validation snapshot props directly.
<Field
  label="Email".to_string()
  description="Uncontrolled snapshot: email is required".to_string()
>
  <input class="docs-search__input" type="email" placeholder="name@example.com" />
</Field>

// Controlled-style: parent chooses derived flags/messages, Field only renders semantic output.
<Field
  label="Email".to_string()
  invalid=true
  error_message="Controlled snapshot: email format is invalid".to_string()
>
  <input class="docs-search__input" type="email" placeholder="owner@company.com" />
</Field>"#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"// Streaming Optional: Field is not a 正文阅读面, keep fallback=snapshot.
<Field
  label="Email".to_string()
  description="Streaming fallback=snapshot: waiting for final validation".to_string()
  aria_label="Email field".to_string()
>
  <input class="docs-search__input" type="email" placeholder="owner@company.com" />
</Field>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Field"
            slug="field"
            group="Forms"
            description="Form field wrapper with centralized orientation/tone/validation/message-state modeling and stable data contracts."
        >
            <Playground
                title="Hello World (Default API)"
                description="Minimal path: no manual wiring to ui-state-primitives/ui-headless state machines."
                code_signal=hello_world_code
                code_imports=field_imports.clone()
            >
                <Field label="Email".to_string()>
                    <input
                        class="docs-search__input"
                        type="email"
                        placeholder="name@example.com"
                    />
                </Field>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with display/config/code/css-test panels, plus optional persisted context."
                code_signal=workbench_code
                code_imports=field_imports.clone()
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/field_form/field/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="field-workbench-controls">
                        <label class="docs-search__label">
                            "Orientation"
                            <select
                                data-action="field-workbench-orientation"
                                prop:value=move || workbench_orientation_key.get()
                                on:change=move |ev| {
                                    set_workbench_orientation_key.set(event_target_value(&ev))
                                }
                            >
                                <option value="vertical">"Vertical"</option>
                                <option value="horizontal">"Horizontal"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Tone"
                            <select
                                data-action="field-workbench-tone"
                                prop:value=move || workbench_tone_key.get()
                                on:change=move |ev| set_workbench_tone_key.set(event_target_value(&ev))
                            >
                                <option value="default">"Default"</option>
                                <option value="muted">"Muted"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Motion ms (" {move || workbench_motion_ms.get()} ")"
                            <input
                                type="range"
                                data-action="field-workbench-motion-ms"
                                min="1"
                                max="800"
                                step="1"
                                prop:value=move || workbench_motion_ms.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev)
                                        .parse::<u16>()
                                        .unwrap_or(160)
                                        .clamp(1, 800);
                                    set_workbench_motion_ms.set(next);
                                }
                            />
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                data-action="field-workbench-toggle-required"
                                prop:checked=move || workbench_required.get()
                                on:change=move |ev| set_workbench_required.set(event_target_checked(&ev))
                            />
                            " Required"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                data-action="field-workbench-toggle-invalid"
                                prop:checked=move || workbench_invalid.get()
                                on:change=move |ev| set_workbench_invalid.set(event_target_checked(&ev))
                            />
                            " Invalid"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                data-action="field-workbench-toggle-disabled"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                            />
                            " Disabled"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                data-action="field-workbench-toggle-custom-error"
                                prop:checked=move || workbench_custom_error.get()
                                on:change=move |ev| set_workbench_custom_error.set(event_target_checked(&ev))
                            />
                            " Custom error text"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                data-action="field-workbench-toggle-custom-class"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
                        <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                            "Persist workbench state"
                        </Switch>
                        <p class="ui-muted" data-slot="field-workbench-summary">
                            {move || {
                                format!(
                                    "config: orientation={} tone={} required={} invalid={} disabled={} custom_error={} custom_class={} motion_ms={} persist={}",
                                    workbench_orientation_key.get(),
                                    workbench_tone_key.get(),
                                    if workbench_required.get() { "true" } else { "false" },
                                    if workbench_invalid.get() { "true" } else { "false" },
                                    if workbench_disabled.get() { "true" } else { "false" },
                                    if workbench_custom_error.get() { "true" } else { "false" },
                                    if workbench_custom_class.get() { "true" } else { "false" },
                                    workbench_motion_ms.get(),
                                    if workbench_persist_state.get() { "on" } else { "off" },
                                )
                            }}
                        </p>
                    </div>
                }
            >
                {move || {
                    let orientation = workbench_orientation.get();
                    let tone = workbench_tone.get();
                    let required = workbench_required.get();
                    let invalid = workbench_invalid.get();
                    let disabled = workbench_disabled.get();
                    let custom_error = workbench_custom_error.get();
                    let custom_class = workbench_custom_class.get();
                    let motion = FieldMotion {
                        duration_ms: f64::from(workbench_motion_ms.get()),
                        ..FieldMotion::default()
                    };

                    if custom_error && custom_class {
                        view! {
                            <Field
                                orientation=orientation
                                tone=tone
                                is_required=required
                                required=required
                                is_invalid=invalid
                                invalid=invalid
                                is_disabled=disabled
                                disabled=disabled
                                label="Email".to_string()
                                description="Inspect source/state marker contracts".to_string()
                                error_message="Custom validation error".to_string()
                                class_name="docs-field-custom".to_string()
                                motion=motion
                                aria_label="Workbench field".to_string()
                                lang="en-US".to_string()
                                dir=ui_headless::A11yDirection::Ltr
                            >
                                <input
                                    class="docs-search__input"
                                    type="email"
                                    placeholder="owner@company.com"
                                />
                            </Field>
                        }
                        .into_any()
                    } else if custom_error {
                        view! {
                            <Field
                                orientation=orientation
                                tone=tone
                                is_required=required
                                required=required
                                is_invalid=invalid
                                invalid=invalid
                                is_disabled=disabled
                                disabled=disabled
                                label="Email".to_string()
                                description="Inspect source/state marker contracts".to_string()
                                error_message="Custom validation error".to_string()
                                motion=motion
                                aria_label="Workbench field".to_string()
                                lang="en-US".to_string()
                                dir=ui_headless::A11yDirection::Ltr
                            >
                                <input
                                    class="docs-search__input"
                                    type="email"
                                    placeholder="owner@company.com"
                                />
                            </Field>
                        }
                        .into_any()
                    } else if custom_class {
                        view! {
                            <Field
                                orientation=orientation
                                tone=tone
                                is_required=required
                                required=required
                                is_invalid=invalid
                                invalid=invalid
                                is_disabled=disabled
                                disabled=disabled
                                label="Email".to_string()
                                description="Inspect source/state marker contracts".to_string()
                                class_name="docs-field-custom".to_string()
                                motion=motion
                                aria_label="Workbench field".to_string()
                                lang="en-US".to_string()
                                dir=ui_headless::A11yDirection::Ltr
                            >
                                <input
                                    class="docs-search__input"
                                    type="email"
                                    placeholder="owner@company.com"
                                />
                            </Field>
                        }
                        .into_any()
                    } else {
                        view! {
                            <Field
                                orientation=orientation
                                tone=tone
                                is_required=required
                                required=required
                                is_invalid=invalid
                                invalid=invalid
                                is_disabled=disabled
                                disabled=disabled
                                label="Email".to_string()
                                description="Inspect source/state marker contracts".to_string()
                                motion=motion
                                aria_label="Workbench field".to_string()
                                lang="en-US".to_string()
                                dir=ui_headless::A11yDirection::Ltr
                            >
                                <input
                                    class="docs-search__input"
                                    type="email"
                                    placeholder="owner@company.com"
                                />
                            </Field>
                        }
                        .into_any()
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Workbench Compare)"
                description="Compare baseline/invalid/disabled combinations after Workbench controls."
                code_signal=state_matrix_code
                code_imports=field_imports.clone()
            >
                <div class="docs-stack" data-slot="field-state-matrix-post-workbench">
                    <Field
                        label="Email".to_string()
                        is_required=true
                        description="Required: this field must be provided.".to_string()
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                    >
                        <input class="docs-search__input" type="email" placeholder="name@example.com" />
                    </Field>
                    <Field
                        label="Email".to_string()
                        is_invalid=true
                        error_message="A valid email is required".to_string()
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                    >
                        <input class="docs-search__input" type="email" placeholder="owner@company.com" />
                    </Field>
                    <Field
                        label="Email".to_string()
                        is_disabled=true
                        description="Disabled: read-only snapshot.".to_string()
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                    >
                        <input class="docs-search__input" type="email" placeholder="disabled@example.com" />
                    </Field>
                </div>
            </Playground>

            <Playground
                title="Required + Description"
                code_signal=required_code
                code_imports=field_imports.clone()
            >
                <Field
                    label="Email".to_string()
                    required=true
                    description="We'll only use this for release notes.".to_string()
                    aria_label="Email field".to_string()
                >
                    <input
                        class="docs-search__input"
                        type="email"
                        placeholder="name@example.com"
                    />
                </Field>
            </Playground>

            <Playground
                title="Horizontal + Invalid + Custom Class"
                code_signal=invalid_code
                code_imports=field_imports.clone()
            >
                <Field
                    orientation=FieldOrientation::Horizontal
                    tone=FieldTone::Muted
                    invalid=true
                    error_message="A valid email is required".to_string()
                    class_name="docs-field-custom".to_string()
                >
                    <input
                        class="docs-search__input"
                        type="email"
                        placeholder="owner@company.com"
                    />
                </Field>
            </Playground>

            <section class="docs-card docs-prose" data-slot="field-api-matrix">
                <h3>"API Matrix"</h3>
                <ul data-slot="field-api-rows">
                    <li>
                        <code>"orientation: FieldOrientation"</code>
                        " default = vertical"
                    </li>
                    <li>
                        <code>"tone: FieldTone"</code>
                        " default = default"
                    </li>
                    <li>
                        <code>"is_required / is_disabled / is_invalid"</code>
                        " default = false（优先命名）"
                    </li>
                    <li>
                        <code>"required / disabled / invalid"</code>
                        " 历史别名，默认 = false，且低于 `is_*` 优先级"
                    </li>
                    <li>
                        <code>"label / description / error_message / aria_label / lang / class_name"</code>
                        " optional semantic content (normalized in logic.rs)"
                    </li>
                    <li>
                        <code>"dir: Option&lt;A11yDirection&gt;"</code>
                        " optional"
                    </li>
                    <li>
                        <code>"motion: FieldMotion"</code>
                        " default = FieldMotion::default()"
                    </li>
                </ul>
            </section>

            <Playground
                title="Required / Invalid / Disabled Examples"
                description="State matrix baseline for required/invalid/disabled semantic markers."
                code_signal=state_matrix_code
                code_imports=field_imports.clone()
            >
                <div class="docs-stack" data-slot="field-state-matrix">
                    <Field
                        label="Email".to_string()
                        required=true
                        description="Required: this field must be provided.".to_string()
                    >
                        <input
                            class="docs-search__input"
                            type="email"
                            placeholder="name@example.com"
                        />
                    </Field>
                    <Field
                        label="Email".to_string()
                        invalid=true
                        error_message="A valid email is required".to_string()
                    >
                        <input
                            class="docs-search__input"
                            type="email"
                            placeholder="owner@company.com"
                        />
                    </Field>
                    <Field
                        label="Email".to_string()
                        disabled=true
                        description="Disabled: read-only snapshot.".to_string()
                    >
                        <input
                            class="docs-search__input"
                            type="email"
                            placeholder="disabled@example.com"
                        />
                    </Field>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (Stateless Contract)"
                description="Field is stateless: parent controls derived flags/messages; Field renders semantic snapshot."
                code_signal=controlled_uncontrolled_code
                code_imports=field_imports.clone()
            >
                <div class="docs-stack" data-slot="field-controlled-matrix">
                    <Field
                        label="Email".to_string()
                        description="Uncontrolled snapshot: email is required".to_string()
                    >
                        <input
                            class="docs-search__input"
                            type="email"
                            placeholder="name@example.com"
                        />
                    </Field>
                    <Field
                        label="Email".to_string()
                        invalid=true
                        error_message="Controlled snapshot: email format is invalid".to_string()
                    >
                        <input
                            class="docs-search__input"
                            type="email"
                            placeholder="owner@company.com"
                        />
                    </Field>
                </div>
            </Playground>

            <Playground
                title="Streaming Optional (fallback=snapshot)"
                description="Field is not a 正文阅读面; docs expose snapshot mode + fallback=snapshot for copy/paste verification."
                code_signal=stream_snapshot_code
                code_imports=field_imports.clone()
            >
                <Field
                    label="Email".to_string()
                    description="Streaming fallback=snapshot: waiting for final validation".to_string()
                    aria_label="Email field".to_string()
                >
                    <input
                        class="docs-search__input"
                        type="email"
                        placeholder="owner@company.com"
                    />
                    <span class="ui-muted">
                        "Inspect data-ui-stream-support/data-ui-stream-mode/data-ui-stream-fallback."
                    </span>
                </Field>
            </Playground>

            <section class="docs-card docs-prose" data-slot="field-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p data-slot="field-source-first-contract">
                    "Use any Field Playground's "
                    <code>"Show code"</code>
                    " + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <p data-slot="field-source-first-dependency-baseline">
                    "Dependency baseline (Cargo.toml): "
                    <code>
                        "ui = { default-features = false, features = [\"component-field\", \"inject-css\"] }"
                    </code>
                </p>
                <Snippet
                    text=r#"components/field/src/mod.rs
components/field/src/logic.rs
components/field/src/view.rs
components/field/src/styles.rs
components/field/src/motion.rs
apps/docs-app/src/pages/components/pages/forms_extra.rs::field"#.to_string()
                    copyable=true
                    class_name="docs-field-source-copy".to_string()
                />
                <ul data-slot="field-source-prerequisites">
                    <li><code>"component-field"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
