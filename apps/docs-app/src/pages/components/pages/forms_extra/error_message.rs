use super::*;

pub(crate) fn error_message() -> AnyView {
    let error_message_imports = "use leptos::prelude::*;\nuse ui::{ErrorMessage, ErrorMessageElement, ErrorMessageTone};\nuse ui_headless::A11yDirection;".to_string();

    let (tone_index, set_tone_index) = signal(0usize);
    let (element_index, set_element_index) = signal(1usize);
    let (disabled_state, set_disabled_state) = signal(false);
    let (truncate_state, set_truncate_state) = signal(false);
    let (use_disabled_alias, set_use_disabled_alias) = signal(false);
    let (use_truncate_alias, set_use_truncate_alias) = signal(false);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let workbench_tone = Signal::derive(move || match tone_index.get() {
        1 => ErrorMessageTone::Neutral,
        2 => ErrorMessageTone::Negative,
        _ => ErrorMessageTone::Auto,
    });
    let workbench_element = Signal::derive(move || match element_index.get() {
        0 => ErrorMessageElement::Span,
        2 => ErrorMessageElement::Div,
        _ => ErrorMessageElement::Paragraph,
    });
    let workbench_motion = Signal::derive(move || {
        if custom_motion.get() {
            ui::error_message::ErrorMessageMotion { transition_ms: 320 }
        } else {
            ui::error_message::ErrorMessageMotion::default()
        }
    });
    let workbench_aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Email validation error".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-error-message-custom".to_string()
        } else {
            String::new()
        }
    });
    let workbench_lang = Signal::derive(move || {
        if rtl.get() {
            "ar".to_string()
        } else {
            "en-US".to_string()
        }
    });
    let workbench_dir = Signal::derive(move || {
        if rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<ErrorMessage text=\"Invalid email address\".into() />"#.to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<ErrorMessage\n  text=\"{}\".into()\n  tone=ErrorMessageTone::{:?}\n  is_disabled={}\n  disabled={}\n  is_truncated={}\n  truncate={}\n  element=ErrorMessageElement::{:?}\n  motion=ui::error_message::ErrorMessageMotion {{ transition_ms: {} }}\n  aria_label={}\n  class_name={}\n  lang={}\n  dir=ui_headless::A11yDirection::{}\n/>",
            if disabled_state.get() {
                "Email is required for account creation"
            } else {
                "Invalid email address"
            },
            workbench_tone.get(),
            bool_word(if use_disabled_alias.get() {
                false
            } else {
                disabled_state.get()
            }),
            bool_word(if use_disabled_alias.get() {
                disabled_state.get()
            } else {
                false
            }),
            bool_word(if use_truncate_alias.get() {
                false
            } else {
                truncate_state.get()
            }),
            bool_word(if use_truncate_alias.get() {
                truncate_state.get()
            } else {
                false
            }),
            workbench_element.get(),
            workbench_motion.get().transition_ms,
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
            rust_string_literal(&workbench_lang.get()),
            if rtl.get() { "Rtl" } else { "Ltr" },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ErrorMessage text=\"Invalid email address\".into() />
<ErrorMessage
  text=\"Username already exists\".into()
  tone=ErrorMessageTone::Neutral
  is_disabled=true
  element=ErrorMessageElement::Div
/>
<ErrorMessage
  text=\"Verification code expired\".into()
  tone=ErrorMessageTone::Negative
  disabled=true
  truncate=true
  motion=ui::error_message::ErrorMessageMotion { transition_ms: 280 }
  aria_label=\"Verification error\".into()
  class_name=\"docs-error-message-custom\".into()
  lang=\"ar\".into()
  dir=A11yDirection::Rtl
/>"#
        .to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/error-message/src/styles.rs */\n{}",
            ui::error_message::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let text = if disabled_state.get() {
            "Email is required for account creation".to_string()
        } else {
            "Invalid email address".to_string()
        };
        let is_disabled = if use_disabled_alias.get() {
            false
        } else {
            disabled_state.get()
        };
        let disabled = if use_disabled_alias.get() {
            disabled_state.get()
        } else {
            false
        };
        let is_truncated = if use_truncate_alias.get() {
            false
        } else {
            truncate_state.get()
        };
        let truncate = if use_truncate_alias.get() {
            truncate_state.get()
        } else {
            false
        };

        format!(
            "ErrorMessageActualConfig {{\n  text: {:?},\n  tone: {:?},\n  is_disabled: {},\n  disabled: {},\n  is_truncated: {},\n  truncate: {},\n  element: {:?},\n  motion: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            text,
            workbench_tone.get(),
            is_disabled,
            disabled,
            is_truncated,
            truncate,
            workbench_element.get(),
            workbench_motion.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
            workbench_lang.get(),
            workbench_dir.get(),
        )
    });

    view! {
        <ComponentPage
            title="ErrorMessage"
            slug="error-message"
            group="Forms"
            description="Inline form error primitive with full API workbench and state matrix."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports=error_message_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <ErrorMessage text="Invalid email address".to_string() />
                </div>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports=error_message_imports.clone()
                test_css_source=workbench_test_css_source
                test_source_path="components/error-message/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="error-message-workbench-controls">
                        <div class="docs-search__label">"Tone"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || tone_index.get().to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_tone_index.set(value.min(2));
                                }
                            }
                        >
                            <option value="0">"Auto"</option>
                            <option value="1">"Neutral"</option>
                            <option value="2">"Negative"</option>
                        </select>

                        <div class="docs-search__label">"Element"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || element_index.get().to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_element_index.set(value.min(2));
                                }
                            }
                        >
                            <option value="0">"span"</option>
                            <option value="1">"paragraph"</option>
                            <option value="2">"div"</option>
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || disabled_state.get()
                                on:change=move |event| set_disabled_state.set(event_target_checked(&event))
                            />
                            <span>"Disabled state"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || use_disabled_alias.get()
                                on:change=move |event| set_use_disabled_alias.set(event_target_checked(&event))
                            />
                            <span>"Use disabled alias prop"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || truncate_state.get()
                                on:change=move |event| set_truncate_state.set(event_target_checked(&event))
                            />
                            <span>"Truncate state"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || use_truncate_alias.get()
                                on:change=move |event| set_use_truncate_alias.set(event_target_checked(&event))
                            />
                            <span>"Use truncate alias prop"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_aria.get()
                                on:change=move |event| set_custom_aria.set(event_target_checked(&event))
                            />
                            <span>"Custom aria label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_class.get()
                                on:change=move |event| set_custom_class.set(event_target_checked(&event))
                            />
                            <span>"Custom class"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_motion.get()
                                on:change=move |event| set_custom_motion.set(event_target_checked(&event))
                            />
                            <span>"Custom motion"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || rtl.get()
                                on:change=move |event| set_rtl.set(event_target_checked(&event))
                            />
                            <span>"RTL (lang=ar, dir=rtl)"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight docs-error-message-limit">
                    <ErrorMessage
                        text=if disabled_state.get() {
                            "Email is required for account creation".to_string()
                        } else {
                            "Invalid email address".to_string()
                        }
                        tone=workbench_tone.get()
                        is_disabled=if use_disabled_alias.get() {
                            false
                        } else {
                            disabled_state.get()
                        }
                        disabled=if use_disabled_alias.get() {
                            disabled_state.get()
                        } else {
                            false
                        }
                        is_truncated=if use_truncate_alias.get() {
                            false
                        } else {
                            truncate_state.get()
                        }
                        truncate=if use_truncate_alias.get() {
                            truncate_state.get()
                        } else {
                            false
                        }
                        element=workbench_element.get()
                        motion=workbench_motion.get()
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                        lang=workbench_lang.get()
                        dir=workbench_dir.get()
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Tone / Alias / Element Comparison)"
                code_signal=matrix_code
                code_imports=error_message_imports
            >
                <div class="docs-stack docs-stack--tight docs-error-message-limit">
                    <ErrorMessage text="Invalid email address".to_string() />
                    <ErrorMessage
                        text="Username already exists".to_string()
                        tone=ErrorMessageTone::Neutral
                        is_disabled=true
                        element=ErrorMessageElement::Div
                    />
                    <ErrorMessage
                        text="Verification code expired".to_string()
                        tone=ErrorMessageTone::Negative
                        disabled=true
                        truncate=true
                        motion=ui::error_message::ErrorMessageMotion {
                            transition_ms: 280,
                        }
                        aria_label="Verification error".to_string()
                        class_name="docs-error-message-custom".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
