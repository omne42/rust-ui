use super::*;

pub(crate) fn field_error() -> AnyView {
    let field_error_imports =
        "use leptos::prelude::*;\nuse ui::{FieldError, FieldErrorTone};".to_string();

    let hello_world_code = Signal::derive(move || {
        r#"<FieldError
  is_visible=true
  message="Email is required".to_string()
/>"#
        .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<FieldError
  is_visible=true
  message="Email is required".to_string()
/>
<FieldError
  is_visible=true
  tone=FieldErrorTone::Neutral
  message="Password should include at least one symbol".to_string()
/>
<FieldError
  is_visible=true
  tone=FieldErrorTone::Negative
  is_icon_visible=true
  message="Two-factor code is invalid".to_string()
/>
<FieldError
  is_visible=true
  is_disabled=true
  is_icon_visible=true
  message="Read-only mode keeps the latest error snapshot".to_string()
/>
<FieldError
  is_visible=false
  message="Hidden state keeps semantic contracts without visual output".to_string()
/>"#
        .to_string()
    });

    let controlled_uncontrolled_code = Signal::derive(move || {
        r#"// FieldError has no internal controllable state axis.
// Uncontrolled-style: pass final validation snapshot directly.
<FieldError
  is_visible=true
  message="Uncontrolled snapshot: email is required".to_string()
/>

// Controlled-style (by parent form store): parent updates props and FieldError re-renders.
<FieldError
  is_visible=true
  tone=FieldErrorTone::Negative
  message="Controlled snapshot: email format is invalid".to_string()
/>"#
        .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"// Snapshot: render validated final output in one shot.
<FieldError
  is_visible=true
  message="Snapshot: email is required".to_string()
/>

// Streaming Optional: keep fallback=snapshot and render last stable message.
<FieldError
  is_visible=true
  tone=FieldErrorTone::Neutral
  message="Streaming fallback=snapshot: waiting for final validation".to_string()
/>"#
        .to_string()
    });

    let default_code = Signal::derive(move || {
        r#"<FieldError
  is_visible=true
  message="Email is required".to_string()
/>
<FieldError
  is_visible=true
  tone=FieldErrorTone::Neutral
  message="Password should include at least one symbol".to_string()
/>
<FieldError
  is_visible=true
  tone=FieldErrorTone::Negative
  is_icon_visible=true
  message="Two-factor code is invalid".to_string()
/>
"#
        .to_string()
    });

    let hidden_code = Signal::derive(move || {
        r#"<FieldError
  is_visible=false
  message="This text should not render when hidden".to_string()
/>
<FieldError
  is_visible=true
  is_disabled=true
  is_icon_visible=true
  class_name="docs-field-error-custom".to_string()
/>"#
        .to_string()
    });

    let (workbench_tone_index, set_workbench_tone_index) = signal(0usize);
    let (workbench_visible, set_workbench_visible) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_icon_visible, set_workbench_icon_visible) = signal(false);
    let (workbench_custom_message, set_workbench_custom_message) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_code = Signal::derive(move || {
        let tone_line = match workbench_tone_index.get() {
            1 => "  tone=FieldErrorTone::Neutral\n",
            2 => "  tone=FieldErrorTone::Negative\n",
            _ => "",
        };
        let message_line = if workbench_custom_message.get() {
            "  message=\"Workbench custom error message\".to_string()\n"
        } else {
            ""
        };
        let aria_line = if workbench_custom_aria.get() {
            "  aria_label=\"Workbench aria label\".to_string()\n"
        } else {
            ""
        };
        let class_line = if workbench_custom_class.get() {
            "  class_name=\"docs-field-error-custom\".to_string()\n"
        } else {
            ""
        };

        format!(
            "<FieldError\n{tone_line}  is_visible={}\n  is_disabled={}\n  is_icon_visible={}\n  show_icon={}\n  lang={:?}.to_string()\n  dir=A11yDirection::{:?}\n{message_line}{aria_line}{class_line}/>",
            workbench_visible.get(),
            workbench_disabled.get(),
            workbench_icon_visible.get(),
            workbench_icon_visible.get(),
            if workbench_rtl.get() { "ar" } else { "en-US" },
            if workbench_rtl.get() {
                A11yDirection::Rtl
            } else {
                A11yDirection::Ltr
            },
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/field-error/src/styles.rs */\n{}",
            ui::field_form::field_error::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let tone = match workbench_tone_index.get() {
            1 => FieldErrorTone::Neutral,
            2 => FieldErrorTone::Negative,
            _ => FieldErrorTone::Auto,
        };
        let message = if workbench_custom_message.get() {
            "Workbench custom error message".to_string()
        } else {
            String::new()
        };
        let aria_label = if workbench_custom_aria.get() {
            "Workbench aria label".to_string()
        } else {
            String::new()
        };
        let class_name = if workbench_custom_class.get() {
            "docs-field-error-custom".to_string()
        } else {
            String::new()
        };
        let lang = if workbench_rtl.get() { "ar" } else { "en-US" };
        let dir = if workbench_rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };

        format!(
            "FieldErrorWorkbenchConfig {{\n  tone: {tone:?},\n  is_visible: {},\n  visible: {},\n  is_disabled: {},\n  disabled: {},\n  is_icon_visible: {},\n  show_icon: {},\n  message: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            workbench_visible.get(),
            workbench_visible.get(),
            workbench_disabled.get(),
            workbench_disabled.get(),
            workbench_icon_visible.get(),
            workbench_icon_visible.get(),
            message,
            aria_label,
            class_name,
            lang,
            dir,
        )
    });

    view! {
        <ComponentPage
            title="FieldError"
            slug="field-error"
            group="Forms"
            description="baseline-style field error primitive with centralized visibility/tone/message normalization and stable data contracts."
        >
            <Playground
                title="Hello World (Snapshot Baseline)"
                code_signal=hello_world_code
                code_imports=field_error_imports.clone()
            >
                <div class="docs-stack">
                    <FieldError
                        is_visible=true
                        message="Email is required".to_string()
                        aria_label="Email error".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Props + State + Source Markers)"
                description="Use settings to toggle FieldError props/state and inspect semantic marker feedback in real time."
                code_signal=workbench_code
                code_imports=field_error_imports.clone()
                test_css_source=workbench_test_css_source
                test_source_path="components/field-error/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="field-error-config-controls">
                            <div class="docs-search__label">"Tone"</div>
                            <select
                                class="docs-search__input"
                                data-action="cycle-tone-config"
                                prop:value=move || workbench_tone_index.get().to_string()
                                on:change=move |ev| {
                                    if let Ok(next) = event_target_value(&ev).parse::<usize>() {
                                        set_workbench_tone_index.set(next.min(2));
                                    }
                                }
                            >
                                <option value="0">"Auto"</option>
                                <option value="1">"Neutral"</option>
                                <option value="2">"Negative"</option>
                            </select>

                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    data-action="toggle-visible-config"
                                    prop:checked=move || workbench_visible.get()
                                    on:change=move |ev| set_workbench_visible.set(event_target_checked(&ev))
                                />
                                <span>"Visible"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    data-action="toggle-disabled-config"
                                    prop:checked=move || workbench_disabled.get()
                                    on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                                />
                                <span>"Disabled"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    data-action="toggle-icon-config"
                                    prop:checked=move || workbench_icon_visible.get()
                                    on:change=move |ev| set_workbench_icon_visible.set(event_target_checked(&ev))
                                />
                                <span>"Show icon"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    data-action="toggle-message-config"
                                    prop:checked=move || workbench_custom_message.get()
                                    on:change=move |ev| set_workbench_custom_message.set(event_target_checked(&ev))
                                />
                                <span>"Custom message source"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    data-action="toggle-aria-config"
                                    prop:checked=move || workbench_custom_aria.get()
                                    on:change=move |ev| set_workbench_custom_aria.set(event_target_checked(&ev))
                                />
                                <span>"Custom aria source"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    data-action="toggle-class-config"
                                    prop:checked=move || workbench_custom_class.get()
                                    on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                                />
                                <span>"Custom class source"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    data-action="toggle-rtl-config"
                                    prop:checked=move || workbench_rtl.get()
                                    on:change=move |ev| set_workbench_rtl.set(event_target_checked(&ev))
                                />
                                <span>"RTL locale"</span>
                            </label>

                            <p class="ui-muted" data-slot="field-error-config-summary">
                                {move || {
                                    let tone = match workbench_tone_index.get() {
                                        1 => "neutral",
                                        2 => "negative",
                                        _ => "auto",
                                    };
                                    let message_source = if workbench_custom_message.get() {
                                        "custom"
                                    } else {
                                        "default"
                                    };
                                    let aria_source = if workbench_custom_aria.get() {
                                        "custom"
                                    } else {
                                        "default"
                                    };
                                    let class_source = if workbench_custom_class.get() {
                                        "custom"
                                    } else {
                                        "default"
                                    };

                                    format!(
                                        "config: tone={} visible={} disabled={} icon={} message_source={} aria_source={} class_source={}",
                                        tone,
                                        workbench_visible.get(),
                                        workbench_disabled.get(),
                                        workbench_icon_visible.get(),
                                        message_source,
                                        aria_source,
                                        class_source,
                                    )
                                }}
                            </p>
                        </div>
                    }
                }
            >
                {move || {
                    let tone = match workbench_tone_index.get() {
                        1 => FieldErrorTone::Neutral,
                        2 => FieldErrorTone::Negative,
                        _ => FieldErrorTone::Auto,
                    };
                    let message = if workbench_custom_message.get() {
                        "Workbench custom error message".to_string()
                    } else {
                        String::new()
                    };
                    let aria_label = if workbench_custom_aria.get() {
                        "Workbench aria label".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if workbench_custom_class.get() {
                        "docs-field-error-custom".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <div class="docs-stack" data-slot="field-error-interactive-stage">
                            <FieldError
                                tone=tone
                                is_visible=workbench_visible.get()
                                is_disabled=workbench_disabled.get()
                                is_icon_visible=workbench_icon_visible.get()
                                show_icon=workbench_icon_visible.get()
                                message=message
                                aria_label=aria_label
                                class_name=class_name
                                lang=if workbench_rtl.get() { "ar" } else { "en-US" }
                                dir=if workbench_rtl.get() {
                                    A11yDirection::Rtl
                                } else {
                                    A11yDirection::Ltr
                                }
                            />
                            <p class="ui-muted" data-slot="field-error-interactive-hint">
                                "Inspect data-state/data-message-source/data-aria-source/data-class-source while toggling controls."
                            </p>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Visible / Hidden / Disabled)"
                code_signal=state_matrix_code
                code_imports=field_error_imports.clone()
            >
                <div class="docs-stack">
                    <FieldError
                        is_visible=true
                        message="Email is required".to_string()
                        aria_label="Email error".to_string()
                        show_icon=true
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Neutral
                        is_icon_visible=true
                        show_icon=true
                        message="Password should include at least one symbol".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Negative
                        is_icon_visible=true
                        show_icon=true
                        message="Two-factor code is invalid".to_string()
                        class_name="docs-field-error-custom".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>

            <Playground
                title="Visible / Hidden / Disabled Gallery"
                code_signal=state_matrix_code
                code_imports=field_error_imports.clone()
            >
                <div class="docs-stack">
                    <FieldError
                        is_visible=true
                        message="Email is required".to_string()
                        aria_label="Email error".to_string()
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Neutral
                        message="Password should include at least one symbol".to_string()
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Negative
                        is_icon_visible=true
                        message="Two-factor code is invalid".to_string()
                    />
                    <FieldError
                        is_visible=true
                        is_disabled=true
                        is_icon_visible=true
                        message="Read-only mode keeps the latest error snapshot".to_string()
                    />
                    <FieldError
                        is_visible=false
                        message="Hidden state keeps semantic contracts without visual output"
                            .to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (Stateless Contract)"
                code_signal=controlled_uncontrolled_code
                code_imports=field_error_imports.clone()
            >
                <div class="docs-stack">
                    <FieldError
                        is_visible=true
                        message="Uncontrolled snapshot: email is required".to_string()
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Negative
                        message="Controlled snapshot: email format is invalid".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional (fallback=snapshot)"
                code_signal=stream_snapshot_code
                code_imports=field_error_imports.clone()
            >
                <div class="docs-stack">
                    <FieldError
                        is_visible=true
                        message="Snapshot: email is required".to_string()
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Neutral
                        message="Streaming fallback=snapshot: waiting for final validation"
                            .to_string()
                    />
                </div>
            </Playground>

            <Playground title="Visible + Tone" code_signal=default_code>
                <div class="docs-stack">
                    <FieldError
                        is_visible=true
                        message="Email is required".to_string()
                        aria_label="Email error".to_string()
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Neutral
                        message="Password should include at least one symbol".to_string()
                    />
                    <FieldError
                        is_visible=true
                        tone=FieldErrorTone::Negative
                        is_icon_visible=true
                        message="Two-factor code is invalid".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Hidden + Disabled + Custom Class" code_signal=hidden_code>
                <div class="docs-stack">
                    <FieldError
                        is_visible=false
                        message="This text should not render when hidden".to_string()
                    />
                    <FieldError
                        is_visible=true
                        is_disabled=true
                        is_icon_visible=true
                        class_name="docs-field-error-custom".to_string()
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="field-error-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Use any FieldError Playground's "
                    <code>"Show code"</code>
                    " + copy button. Snippets are import-ready for direct paste."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::{FieldError, FieldErrorTone};\n\n<FieldError\n  is_visible=true\n  message=\"Email is required\".to_string()\n/>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-field-error-source-copy".to_string()
                />
                <ul data-slot="field-error-source-paths">
                    <li><code>"components/field-error/src/mod.rs"</code></li>
                    <li><code>"components/field-error/src/logic.rs"</code></li>
                    <li><code>"components/field-error/src/view.rs"</code></li>
                    <li><code>"components/field-error/src/styles.rs"</code></li>
                </ul>
                <ul data-slot="field-error-source-prerequisites">
                    <li><code>"component-field_error"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
