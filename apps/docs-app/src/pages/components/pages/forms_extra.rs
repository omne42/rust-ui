use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Calendar, CalendarFirstWeekday, CalendarTone, DateField, DateFieldTone, DatePicker,
    DatePickerMotion, DatePickerTone, DateRangePicker, DateRangePickerTone, Description,
    DescriptionElement, DescriptionTone, ErrorMessage, ErrorMessageElement, ErrorMessageTone,
    Field, FieldError, FieldErrorTone, FieldOrientation, FieldTone, Fieldset, FieldsetOrientation,
    FieldsetTone, HelpText, HelpTextTone, Label, LabelEmphasis, PopoverMotion, SegmentedControl,
    SegmentedControlSize, Slider, SliderMotion, Snippet, Switch, Textarea, TimeField,
    TimeFieldTone, field_form::field::FieldMotion,
};

pub(super) fn field_error() -> AnyView {
    let default_code = Signal::derive(move || {
        r#"<FieldError
  visible=true
  message="Email is required".to_string()
/>
<FieldError
  visible=true
  tone=FieldErrorTone::Neutral
  message="Password should include at least one symbol".to_string()
/>
<FieldError
  visible=true
  tone=FieldErrorTone::Negative
  show_icon=true
  message="Two-factor code is invalid".to_string()
/>
"#
        .to_string()
    });

    let hidden_code = Signal::derive(move || {
        r#"<FieldError
  visible=false
  message="This text should not render when hidden".to_string()
/>
<FieldError
  visible=true
  disabled=true
  show_icon=true
  class_name="docs-field-error-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="FieldError"
            slug="field-error"
            group="Forms"
            description="baseline-style field error primitive with centralized visibility/tone/message normalization and stable data contracts."
        >
            <Playground title="Visible + Tone" code_signal=default_code>
                <div class="docs-stack">
                    <FieldError
                        visible=true
                        message="Email is required".to_string()
                        aria_label="Email error".to_string()
                    />
                    <FieldError
                        visible=true
                        tone=FieldErrorTone::Neutral
                        message="Password should include at least one symbol".to_string()
                    />
                    <FieldError
                        visible=true
                        tone=FieldErrorTone::Negative
                        show_icon=true
                        message="Two-factor code is invalid".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Hidden + Disabled + Custom Class" code_signal=hidden_code>
                <div class="docs-stack">
                    <FieldError
                        visible=false
                        message="This text should not render when hidden".to_string()
                    />
                    <FieldError
                        visible=true
                        disabled=true
                        show_icon=true
                        class_name="docs-field-error-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn error_message() -> AnyView {
    let tone_code = Signal::derive(move || {
        r#"<ErrorMessage text="Invalid email address".to_string() />
<ErrorMessage
  text="Username contains unsupported characters.".to_string()
  tone=ErrorMessageTone::Neutral
/>
<ErrorMessage
  text="Verification code expired, request a new one.".to_string()
  tone=ErrorMessageTone::Negative
/>"#
        .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"<ErrorMessage
  text="A very long validation message that should truncate in constrained layouts to keep form rhythm predictable.".to_string()
  truncate=true
  class_name="docs-error-message-custom".to_string()
/>
<ErrorMessage
  text="This error remains visible but marked as disabled for read-only states.".to_string()
  disabled=true
  element=ErrorMessageElement::Div
  aria_label="Disabled error message".to_string()
/>"#.to_string()
    });

    let display_code = Signal::derive(move || {
        r#"<ErrorMessage text="Auto tone (maps to negative) for default invalid feedback.".to_string() />
<ErrorMessage
  text="Neutral tone for low-priority guidance.".to_string()
  tone=ErrorMessageTone::Neutral
/>
<ErrorMessage
  text="Negative tone rendered with span element.".to_string()
  tone=ErrorMessageTone::Negative
  element=ErrorMessageElement::Span
/>
<ErrorMessage
  text="Disabled + truncate comparison for dense layouts.".to_string()
  disabled=true
  truncate=true
  class_name="docs-error-message-custom".to_string()
/>"#.to_string()
    });

    let (workbench_tone_index, set_workbench_tone_index) = signal(0usize);
    let (workbench_element_index, set_workbench_element_index) = signal(1usize);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_truncate, set_workbench_truncate) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);

    let workbench_code = Signal::derive(move || {
        let tone_line = match workbench_tone_index.get() {
            1 => "  tone=ErrorMessageTone::Neutral\n",
            2 => "  tone=ErrorMessageTone::Negative\n",
            _ => "",
        };
        let element_line = match workbench_element_index.get() {
            0 => "  element=ErrorMessageElement::Span\n",
            2 => "  element=ErrorMessageElement::Div\n",
            _ => "",
        };
        let disabled_line = if workbench_disabled.get() {
            "  disabled=true\n"
        } else {
            ""
        };
        let truncate_line = if workbench_truncate.get() {
            "  truncate=true\n"
        } else {
            ""
        };
        let class_line = if workbench_custom_class.get() {
            "  class_name=\"docs-error-message-custom\".into()\n"
        } else {
            ""
        };
        let aria_line = if workbench_custom_aria.get() {
            "  aria_label=\"Workbench error\".into()\n"
        } else {
            ""
        };

        format!(
            "<ErrorMessage\n  text=\"Config + code + css test workbench message\".into()\n{tone_line}{element_line}{disabled_line}{truncate_line}{class_line}{aria_line}/>"
        )
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/error-message/src/styles.rs */\n{}",
            ui_components::error_message::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let tone = match workbench_tone_index.get() {
            1 => ErrorMessageTone::Neutral,
            2 => ErrorMessageTone::Negative,
            _ => ErrorMessageTone::Auto,
        };
        let element = match workbench_element_index.get() {
            0 => ErrorMessageElement::Span,
            2 => ErrorMessageElement::Div,
            _ => ErrorMessageElement::Paragraph,
        };
        let class_source = if workbench_custom_class.get() {
            "custom"
        } else {
            "default"
        };
        let aria_source = if workbench_custom_aria.get() {
            "custom"
        } else {
            "default"
        };

        format!(
            "ErrorMessageWorkbenchConfig {{\n  tone: {tone:?},\n  element: {element:?},\n  disabled: {},\n  truncate: {},\n  class_source: \"{class_source}\",\n  aria_source: \"{aria_source}\",\n}}",
            workbench_disabled.get(),
            workbench_truncate.get(),
        )
    });

    view! {
        <ComponentPage
            title="ErrorMessage"
            slug="error-message"
            group="Forms"
            description="baseline-style inline error primitive with centralized tone/disabled/truncate/source normalization and stable slot/data contracts."
        >
            <Playground title="Tone Variants" code_signal=tone_code>
                <div class="docs-stack">
                    <ErrorMessage
                        text="Invalid email address".to_string()
                        aria_label="Email error".to_string()
                    />
                    <ErrorMessage
                        text="Username contains unsupported characters.".to_string()
                        tone=ErrorMessageTone::Neutral
                    />
                    <ErrorMessage
                        text="Verification code expired, request a new one.".to_string()
                        tone=ErrorMessageTone::Negative
                    />
                </div>
            </Playground>

            <Playground title="Truncate + Disabled + Element + Custom Class" code_signal=state_code>
                <div class="docs-stack docs-error-message-limit">
                    <ErrorMessage
                        text="A very long validation message that should truncate in constrained layouts to keep form rhythm predictable.".to_string()
                        truncate=true
                        class_name="docs-error-message-custom".to_string()
                    />
                    <ErrorMessage
                        text="This error remains visible but marked as disabled for read-only states.".to_string()
                        disabled=true
                        element=ErrorMessageElement::Div
                        aria_label="Disabled error message".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Display Comparisons (Tone / State / Element)"
                description="Display matrix for multiple semantic states to compare tone, disabled, truncate, and element mappings."
                code_signal=display_code
            >
                <div class="docs-stack docs-error-message-limit">
                    <ErrorMessage
                        text="Auto tone (maps to negative) for default invalid feedback.".to_string()
                    />
                    <ErrorMessage
                        text="Neutral tone for low-priority guidance.".to_string()
                        tone=ErrorMessageTone::Neutral
                    />
                    <ErrorMessage
                        text="Negative tone rendered with span element.".to_string()
                        tone=ErrorMessageTone::Negative
                        element=ErrorMessageElement::Span
                    />
                    <ErrorMessage
                        text="Disabled + truncate comparison for dense layouts.".to_string()
                        disabled=true
                        truncate=true
                        class_name="docs-error-message-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Config + Code + CSS Test Workbench"
                description="Use settings to mutate one instance, then inspect copy-ready code and scoped CSS test panel."
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="components/error-message/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Tone"</div>
                            <select
                                class="docs-search__input"
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

                            <div class="docs-search__label">"Element"</div>
                            <select
                                class="docs-search__input"
                                prop:value=move || workbench_element_index.get().to_string()
                                on:change=move |ev| {
                                    if let Ok(next) = event_target_value(&ev).parse::<usize>() {
                                        set_workbench_element_index.set(next.min(2));
                                    }
                                }
                            >
                                <option value="0">"span"</option>
                                <option value="1">"p"</option>
                                <option value="2">"div"</option>
                            </select>

                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    prop:checked=move || workbench_disabled.get()
                                    on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                                />
                                <span>"Disabled"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    prop:checked=move || workbench_truncate.get()
                                    on:change=move |ev| set_workbench_truncate.set(event_target_checked(&ev))
                                />
                                <span>"Truncate"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    prop:checked=move || workbench_custom_class.get()
                                    on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                                />
                                <span>"Custom class source"</span>
                            </label>
                            <label class="docs-choice-row">
                                <input
                                    type="checkbox"
                                    prop:checked=move || workbench_custom_aria.get()
                                    on:change=move |ev| set_workbench_custom_aria.set(event_target_checked(&ev))
                                />
                                <span>"Custom aria label source"</span>
                            </label>
                        </div>
                    }
                }
            >
                {move || {
                    let tone = match workbench_tone_index.get() {
                        1 => ErrorMessageTone::Neutral,
                        2 => ErrorMessageTone::Negative,
                        _ => ErrorMessageTone::Auto,
                    };
                    let element = match workbench_element_index.get() {
                        0 => ErrorMessageElement::Span,
                        2 => ErrorMessageElement::Div,
                        _ => ErrorMessageElement::Paragraph,
                    };
                    let class_name = if workbench_custom_class.get() {
                        "docs-error-message-custom".to_string()
                    } else {
                        String::new()
                    };
                    let aria_label = if workbench_custom_aria.get() {
                        "Workbench error".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <div class="docs-stack docs-error-message-limit">
                            <ErrorMessage
                                text="Config + code + css test workbench message".to_string()
                                tone=tone
                                disabled=workbench_disabled.get()
                                truncate=workbench_truncate.get()
                                element=element
                                class_name=class_name
                                aria_label=aria_label
                            />
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn description() -> AnyView {
    let tone_options = vec![
        "default".to_string(),
        "muted".to_string(),
        "negative".to_string(),
    ];
    let element_options = vec![
        "paragraph".to_string(),
        "span".to_string(),
        "div".to_string(),
    ];

    let (tone_index, set_tone_index) = signal(Some(0_usize));
    let (element_index, set_element_index) = signal(Some(0_usize));
    let (is_disabled, set_is_disabled) = signal(false);
    let (truncate, set_truncate) = signal(false);
    let (custom_aria_label, set_custom_aria_label) = signal(false);
    let (custom_class, set_custom_class) = signal(false);

    let selected_tone: Signal<DescriptionTone> =
        Signal::derive(move || match tone_index.get().unwrap_or(0) {
            1 => DescriptionTone::Muted,
            2 => DescriptionTone::Negative,
            _ => DescriptionTone::Default,
        });
    let selected_element: Signal<DescriptionElement> =
        Signal::derive(move || match element_index.get().unwrap_or(0) {
            1 => DescriptionElement::Span,
            2 => DescriptionElement::Div,
            _ => DescriptionElement::Paragraph,
        });

    let workbench_code = Signal::derive(move || {
        let tone = selected_tone.get();
        let element = selected_element.get();
        let mut lines = vec![
            "<Description".to_string(),
            "  text=\"Helper text for this field.\".into()".to_string(),
        ];

        if tone != DescriptionTone::Default {
            lines.push(format!("  tone=DescriptionTone::{tone:?}"));
        }
        if element != DescriptionElement::Paragraph {
            lines.push(format!("  element=DescriptionElement::{element:?}"));
        }
        if is_disabled.get() {
            lines.push("  disabled=true".to_string());
        }
        if truncate.get() {
            lines.push("  truncate=true".to_string());
        }
        if custom_aria_label.get() {
            lines.push("  aria_label=\"Description helper text\".into()".to_string());
        }
        if custom_class.get() {
            lines.push("  class_name=\"docs-description-custom\".into()".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let test_css_source = Signal::derive(move || {
        format!(
            "/* components/description/src/styles.rs */\n{}",
            ui_components::description::styles::CSS
        )
    });

    let actual_config = Signal::derive(move || {
        let tone = selected_tone.get();
        let element = selected_element.get();
        let mut classes = vec!["ui-description".to_string(), tone.class_name().into()];
        if is_disabled.get() {
            classes.push("ui-description--disabled".to_string());
        }
        if truncate.get() {
            classes.push("ui-description--truncate".to_string());
        }
        if custom_class.get() {
            classes.push("ui-description--custom-class".to_string());
            classes.push("docs-description-custom".to_string());
        }

        format!(
            "DescriptionActualConfig {{\n  tone: {tone:?},\n  element: {element:?},\n  disabled: {},\n  truncate: {},\n  has_custom_aria_label: {},\n  has_custom_class_name: {},\n  class: \"{}\",\n}}",
            is_disabled.get(),
            truncate.get(),
            custom_aria_label.get(),
            custom_class.get(),
            classes.join(" ")
        )
    });

    let tone_code = Signal::derive(move || {
        r#"<Description
  text="This appears below the field as guidance.".to_string()
  tone=DescriptionTone::Default
  aria_label="Name helper".to_string()
/>
<Description
  text="Optional details are only visible to admins.".to_string()
  tone=DescriptionTone::Muted
/>
<Description
  text="Two-factor code expired. Request a new one.".to_string()
  tone=DescriptionTone::Negative
/>"#
        .to_string()
    });

    let truncate_code = Signal::derive(move || {
        r#"<Description
  text="A very long assistant text that should truncate in constrained layouts to avoid breaking form rhythm.".to_string()
  element=DescriptionElement::Span
  truncate=true
  class_name="docs-description-custom".to_string()
/>
<Description
  text="Disabled helper text".to_string()
  disabled=true
  tone=DescriptionTone::Muted
/>"#.to_string()
    });

    view! {
        <ComponentPage
            title="Description"
            slug="description"
            group="Forms"
            description="baseline-style form description primitive with centralized tone/state/source contracts and stable slot semantics."
        >
            <Playground
                title="Workbench"
                description="Interactive display/config/code/css-test playground for Description state contracts."
                code_signal=workbench_code
                test_css_source=test_css_source
                test_source_path="components/description/src/styles.rs".to_string()
                test_config_signal=actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Tone"</div>
                        <SegmentedControl
                            id_base="docs-description-tone".to_string()
                            options=tone_options.clone()
                            selected_index=tone_index
                            set_selected_index=set_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="Description tone".to_string()
                        />

                        <div class="docs-search__label">"Element"</div>
                        <SegmentedControl
                            id_base="docs-description-element".to_string()
                            options=element_options.clone()
                            selected_index=element_index
                            set_selected_index=set_element_index
                            size=SegmentedControlSize::Sm
                            aria_label="Description element".to_string()
                        />

                        <Switch checked=is_disabled set_checked=set_is_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=truncate set_checked=set_truncate>
                            "Truncate"
                        </Switch>
                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let tone = selected_tone.get();
                    let element = selected_element.get();

                    view! {
                        <div class="docs-stack docs-description-limit">
                            <Description
                                text="Helper text for this field.".to_string()
                                tone=tone
                                element=element
                                disabled=is_disabled.get()
                                truncate=truncate.get()
                                aria_label=if custom_aria_label.get() {
                                    "Description helper text".to_string()
                                } else {
                                    "".to_string()
                                }
                                class_name=if custom_class.get() {
                                    "docs-description-custom".to_string()
                                } else {
                                    "".to_string()
                                }
                            />
                        </div>
                    }
                }}
            </Playground>

            <Playground title="Tone Variants" code_signal=tone_code>
                <div class="docs-stack">
                    <Description
                        text="This appears below the field as guidance.".to_string()
                        tone=DescriptionTone::Default
                        aria_label="Name helper".to_string()
                    />
                    <Description
                        text="Optional details are only visible to admins.".to_string()
                        tone=DescriptionTone::Muted
                    />
                    <Description
                        text="Two-factor code expired. Request a new one.".to_string()
                        tone=DescriptionTone::Negative
                    />
                </div>
            </Playground>

            <Playground title="Truncate + Element + Disabled" code_signal=truncate_code>
                <div class="docs-stack docs-description-limit">
                    <Description
                        text="A very long assistant text that should truncate in constrained layouts to avoid breaking form rhythm.".to_string()
                        element=DescriptionElement::Span
                        truncate=true
                        class_name="docs-description-custom".to_string()
                    />
                    <Description
                        text="Disabled helper text".to_string()
                        disabled=true
                        tone=DescriptionTone::Muted
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn fieldset() -> AnyView {
    let hello_world_code = Signal::derive(move || {
        r#"<Fieldset legend="Channels".to_string()>
  <label><input type="checkbox" /> "Email"</label>
</Fieldset>"#
            .to_string()
    });

    let default_code = Signal::derive(move || {
        r#"<Fieldset
  legend="Notification channels".to_string()
  description="Pick every channel you want to receive release updates from.".to_string()
  required=true
>
  <label><input type="checkbox" /> "Email"</label>
  <label><input type="checkbox" /> "SMS"</label>
</Fieldset>"#
            .to_string()
    });

    let invalid_code = Signal::derive(move || {
        r#"<Fieldset
  orientation=FieldsetOrientation::Horizontal
  tone=FieldsetTone::Muted
  is_invalid=true
  error_message="Pick at least one channel".to_string()
  class_name="docs-fieldset-custom".to_string()
  actions=move || view! {
    <ui_components::Button variant=ui_components::ButtonVariant::Secondary size=ui_components::ButtonSize::Sm>
      "Manage channels"
    </ui_components::Button>
  }
>
  <label><input type="checkbox" /> "Email"</label>
  <label><input type="checkbox" /> "SMS"</label>
</Fieldset>"#.to_string()
    });

    let orientation_options = vec!["vertical".to_string(), "horizontal".to_string()];
    let tone_options = vec!["default".to_string(), "muted".to_string()];
    let locale_options = vec!["en-US".to_string(), "zh-CN".to_string(), "ar".to_string()];

    let (workbench_orientation_index, set_workbench_orientation_index) = signal(Some(0_usize));
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let (workbench_locale_index, set_workbench_locale_index) = signal(Some(0_usize));
    let (workbench_required, set_workbench_required) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_show_actions, set_workbench_show_actions) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_orientation =
        Signal::derive(
            move || match workbench_orientation_index.get().unwrap_or(0) {
                1 => FieldsetOrientation::Horizontal,
                _ => FieldsetOrientation::Vertical,
            },
        );
    let workbench_tone = Signal::derive(move || match workbench_tone_index.get().unwrap_or(0) {
        1 => FieldsetTone::Muted,
        _ => FieldsetTone::Default,
    });

    let workbench_code = Signal::derive(move || {
        let orientation = workbench_orientation.get();
        let tone = workbench_tone.get();
        let locale_index = workbench_locale_index.get().unwrap_or(0);
        let required = workbench_required.get();
        let disabled = workbench_disabled.get();
        let invalid = workbench_invalid.get();
        let show_description = workbench_show_description.get();
        let show_actions = workbench_show_actions.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let lang = match locale_index {
            1 => Some("zh-CN"),
            2 => Some("ar"),
            _ => None,
        };

        let mut lines = vec!["<Fieldset".to_string()];
        if orientation != FieldsetOrientation::Vertical {
            lines.push(format!(
                "  orientation=FieldsetOrientation::{orientation:?}"
            ));
        }
        if tone != FieldsetTone::Default {
            lines.push(format!("  tone=FieldsetTone::{tone:?}"));
        }
        if required {
            lines.push("  is_required=true".to_string());
        }
        if disabled {
            lines.push("  is_disabled=true".to_string());
        }
        if invalid {
            lines.push("  is_invalid=true".to_string());
            lines.push("  error_message=\"Pick at least one channel\".into()".to_string());
        } else if show_description {
            lines.push("  description=\"Choose channels for release updates.\".into()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-fieldset-custom\".into()".to_string());
        }
        if let Some(lang) = lang {
            lines.push(format!("  lang=\"{lang}\".into()"));
        }
        if rtl {
            lines.push("  dir=ui_headless::A11yDirection::Rtl".to_string());
        }
        if show_actions {
            lines.push(
                "  actions=move || view! { <ui_components::Button variant=ui_components::ButtonVariant::Secondary size=ui_components::ButtonSize::Sm>\"Manage\"</ui_components::Button> }".to_string(),
            );
        }
        lines.extend([
            "  legend=\"Notification channels\".into()".to_string(),
            ">".to_string(),
            "  <label><input type=\"checkbox\" /> \"Email\"</label>".to_string(),
            "  <label><input type=\"checkbox\" /> \"SMS\"</label>".to_string(),
            "</Fieldset>".to_string(),
        ]);
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/field_form/fieldset/styles.rs */\n{}",
            ui_components::field_form::fieldset::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let orientation = workbench_orientation.get();
        let tone = workbench_tone.get();
        let locale_index = workbench_locale_index.get().unwrap_or(0);
        let required = workbench_required.get();
        let disabled = workbench_disabled.get();
        let invalid = workbench_invalid.get();
        let show_description = workbench_show_description.get();
        let show_actions = workbench_show_actions.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let lang = match locale_index {
            1 => "zh-CN",
            2 => "ar",
            _ => "default",
        };

        let mut class = vec![
            "ui-fieldset".to_string(),
            orientation.class_name().into(),
            tone.class_name().into(),
        ];
        if required {
            class.push("ui-fieldset--required".to_string());
        }
        if disabled {
            class.push("ui-fieldset--disabled".to_string());
        }
        if invalid {
            class.push("ui-fieldset--invalid".to_string());
        }
        if custom_class {
            class.push("ui-fieldset--custom-class".to_string());
            class.push("docs-fieldset-custom".to_string());
        }

        let message_kind = if invalid {
            "error"
        } else if show_description {
            "description"
        } else {
            "none"
        };

        format!(
            "FieldsetActualConfig {{\n  orientation: {orientation:?},\n  tone: {tone:?},\n  is_required: {required},\n  is_disabled: {disabled},\n  is_invalid: {invalid},\n  has_description: {},\n  has_actions: {show_actions},\n  class_source: \"{}\",\n  message_kind: \"{message_kind}\",\n  lang: \"{lang}\",\n  dir: \"{}\",\n  class: \"{}\",\n}}",
            show_description && !invalid,
            if custom_class { "custom" } else { "default" },
            if rtl { "rtl" } else { "auto" },
            class.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="Fieldset"
            slug="fieldset"
            group="Forms"
            description="baseline-style fieldset primitive with centralized orientation/tone/validation/message/action-state modeling and stable data contracts."
        >
            <Playground title="Hello World" code_signal=hello_world_code>
                <Fieldset legend="Channels".to_string() aria_label="Channel fieldset".to_string()>
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Email"</span>
                    </label>
                </Fieldset>
            </Playground>

            <Playground title="Legend + Description" code_signal=default_code>
                <Fieldset
                    legend="Notification channels".to_string()
                    description="Pick every channel you want to receive release updates from.".to_string()
                    required=true
                    aria_label="Notification channel group".to_string()
                >
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Email"</span>
                    </label>
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"SMS"</span>
                    </label>
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Push"</span>
                    </label>
                </Fieldset>
            </Playground>

            <Playground title="Horizontal + Invalid + Actions" code_signal=invalid_code>
                <Fieldset
                    orientation=FieldsetOrientation::Horizontal
                    tone=FieldsetTone::Muted
                    is_invalid=true
                    error_message="Pick at least one channel".to_string()
                    class_name="docs-fieldset-custom".to_string()
                    actions=move || {
                        view! {
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                size=ui_components::ButtonSize::Sm
                            >
                                "Manage channels"
                            </ui_components::Button>
                        }
                    }
                >
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"Email"</span>
                    </label>
                    <label class="docs-choice-row">
                        <input type="checkbox" />
                        <span>"SMS"</span>
                    </label>
                </Fieldset>
            </Playground>

            <Playground
                title="Fieldset Workbench (Display + Config + Code + CSS Test)"
                description="展示 / config / code / css test 一体化工作台，并提供多场景对比。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="crates/ui-components/src/field_form/fieldset/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="fieldset-workbench-controls">
                        <div class="docs-search__label">"Orientation"</div>
                        <SegmentedControl
                            id_base="docs-fieldset-workbench-orientation".to_string()
                            options=orientation_options.clone()
                            selected_index=workbench_orientation_index
                            set_selected_index=set_workbench_orientation_index
                            size=SegmentedControlSize::Sm
                            aria_label="Fieldset orientation".to_string()
                        />

                        <div class="docs-search__label">"Tone"</div>
                        <SegmentedControl
                            id_base="docs-fieldset-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=workbench_tone_index
                            set_selected_index=set_workbench_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="Fieldset tone".to_string()
                        />

                        <div class="docs-search__label">"Locale"</div>
                        <SegmentedControl
                            id_base="docs-fieldset-workbench-locale".to_string()
                            options=locale_options.clone()
                            selected_index=workbench_locale_index
                            set_selected_index=set_workbench_locale_index
                            size=SegmentedControlSize::Sm
                            aria_label="Fieldset locale".to_string()
                        />

                        <Switch checked=workbench_required set_checked=set_workbench_required>
                            "Required"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_invalid set_checked=set_workbench_invalid>
                            "Invalid"
                        </Switch>
                        <Switch checked=workbench_show_description set_checked=set_workbench_show_description>
                            "Description"
                        </Switch>
                        <Switch checked=workbench_show_actions set_checked=set_workbench_show_actions>
                            "Actions"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL direction"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let orientation = workbench_orientation.get();
                    let tone = workbench_tone.get();
                    let locale_index = workbench_locale_index.get().unwrap_or(0);
                    let required = workbench_required.get();
                    let disabled = workbench_disabled.get();
                    let invalid = workbench_invalid.get();
                    let show_description = workbench_show_description.get();
                    let show_actions = workbench_show_actions.get();
                    let custom_class = workbench_custom_class.get();
                    let rtl = workbench_rtl.get();
                    let lang = match locale_index {
                        1 => "zh-CN".to_string(),
                        2 => "ar".to_string(),
                        _ => String::new(),
                    };
                    let description = if show_description && !invalid {
                        "Choose channels for release updates.".to_string()
                    } else {
                        String::new()
                    };
                    let error_message = if invalid {
                        "Pick at least one channel".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if custom_class {
                        "docs-fieldset-custom".to_string()
                    } else {
                        String::new()
                    };
                    let dir = if rtl {
                        ui_headless::A11yDirection::Rtl
                    } else {
                        ui_headless::A11yDirection::Ltr
                    };

                    view! {
                        <div class="docs-stack" data-slot="fieldset-workbench-compare">
                            <div class="docs-search__label">"Baseline"</div>
                            <Fieldset legend="Notification channels".to_string()>
                                <label class="docs-choice-row">
                                    <input type="checkbox" />
                                    <span>"Email"</span>
                                </label>
                                <label class="docs-choice-row">
                                    <input type="checkbox" />
                                    <span>"SMS"</span>
                                </label>
                            </Fieldset>

                            <div class="docs-search__label">"Configured"</div>
                            {if show_actions {
                                view! {
                                    <Fieldset
                                        orientation=orientation
                                        tone=tone
                                        is_required=required
                                        is_disabled=disabled
                                        is_invalid=invalid
                                        legend="Notification channels".to_string()
                                        description=description.clone()
                                        error_message=error_message.clone()
                                        class_name=class_name.clone()
                                        lang=lang.clone()
                                        dir=dir
                                        actions=move || {
                                            view! {
                                                <ui_components::Button
                                                    variant=ui_components::ButtonVariant::Secondary
                                                    size=ui_components::ButtonSize::Sm
                                                >
                                                    "Manage"
                                                </ui_components::Button>
                                            }
                                        }
                                    >
                                        <label class="docs-choice-row">
                                            <input type="checkbox" />
                                            <span>"Email"</span>
                                        </label>
                                        <label class="docs-choice-row">
                                            <input type="checkbox" />
                                            <span>"SMS"</span>
                                        </label>
                                    </Fieldset>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <Fieldset
                                        orientation=orientation
                                        tone=tone
                                        is_required=required
                                        is_disabled=disabled
                                        is_invalid=invalid
                                        legend="Notification channels".to_string()
                                        description=description
                                        error_message=error_message
                                        class_name=class_name
                                        lang=lang
                                        dir=dir
                                    >
                                        <label class="docs-choice-row">
                                            <input type="checkbox" />
                                            <span>"Email"</span>
                                        </label>
                                        <label class="docs-choice-row">
                                            <input type="checkbox" />
                                            <span>"SMS"</span>
                                        </label>
                                    </Fieldset>
                                }
                                    .into_any()
                            }}

                            <div class="docs-search__label">"Scenario compare"</div>
                            <div class="docs-stack docs-stack--tight">
                                <Fieldset
                                    legend="Required vertical".to_string()
                                    is_required=true
                                    description="Required + description".to_string()
                                >
                                    <label class="docs-choice-row">
                                        <input type="checkbox" />
                                        <span>"Email"</span>
                                    </label>
                                </Fieldset>
                                <Fieldset
                                    legend="Invalid horizontal".to_string()
                                    orientation=FieldsetOrientation::Horizontal
                                    tone=FieldsetTone::Muted
                                    is_invalid=true
                                    error_message="At least one option is required".to_string()
                                >
                                    <label class="docs-choice-row">
                                        <input type="checkbox" />
                                        <span>"SMS"</span>
                                    </label>
                                </Fieldset>
                            </div>
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn label() -> AnyView {
    let emphasis_options = vec![
        "default".to_string(),
        "subtle".to_string(),
        "strong".to_string(),
    ];
    let (emphasis_index, set_emphasis_index) = signal(Some(0usize));
    let selected_emphasis = Signal::derive(move || match emphasis_index.get().unwrap_or(0) {
        1 => LabelEmphasis::Subtle,
        2 => LabelEmphasis::Strong,
        _ => LabelEmphasis::Default,
    });

    let (is_required, set_is_required) = signal(true);
    let (is_disabled, set_is_disabled) = signal(false);
    let (has_for_id, set_has_for_id) = signal(true);
    let (custom_text, set_custom_text) = signal(true);
    let (custom_indicator, set_custom_indicator) = signal(false);
    let (custom_class, set_custom_class) = signal(false);

    let workbench_code = Signal::derive(move || {
        let emphasis = match selected_emphasis.get() {
            LabelEmphasis::Default => "LabelEmphasis::Default",
            LabelEmphasis::Subtle => "LabelEmphasis::Subtle",
            LabelEmphasis::Strong => "LabelEmphasis::Strong",
        };

        let mut lines = vec![
            "<Label".to_string(),
            format!("  emphasis={emphasis}"),
            format!("  required={}", is_required.get()),
            format!("  disabled={}", is_disabled.get()),
        ];

        if custom_text.get() {
            lines.push("  text=\"Assignee\".into()".to_string());
        }
        if has_for_id.get() {
            lines.push("  for_id=\"docs-label-workbench-input\".into()".to_string());
        }
        if custom_indicator.get() {
            lines.push("  required_indicator=\"(required)\".into()".to_string());
        }
        if custom_class.get() {
            lines.push("  class_name=\"docs-label-workbench\".into()".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/label/src/styles.rs */\n{}",
            ui_components::label::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let emphasis = selected_emphasis.get();
        let required = is_required.get();
        let disabled = is_disabled.get();
        let with_for_id = has_for_id.get();
        let with_custom_text = custom_text.get();
        let with_custom_indicator = custom_indicator.get();
        let with_custom_class = custom_class.get();

        let emphasis_attr = match emphasis {
            LabelEmphasis::Default => "default",
            LabelEmphasis::Subtle => "subtle",
            LabelEmphasis::Strong => "strong",
        };
        let data_state = if required { "required" } else { "optional" };
        let data_label_source = if with_custom_text {
            "custom"
        } else {
            "default"
        };
        let data_indicator_source = if with_custom_indicator {
            "custom"
        } else {
            "default"
        };
        let data_class_source = if with_custom_class {
            "custom"
        } else {
            "default"
        };

        let mut classes = vec![
            "ui-label".to_string(),
            format!("ui-label--emphasis-{emphasis_attr}"),
        ];
        if required {
            classes.push("ui-label--required".to_string());
        }
        if disabled {
            classes.push("ui-label--disabled".to_string());
        }
        if with_for_id {
            classes.push("ui-label--for".to_string());
        }
        if with_custom_text {
            classes.push("ui-label--text-custom".to_string());
        }
        if with_custom_indicator {
            classes.push("ui-label--indicator-custom".to_string());
        }
        if with_custom_class {
            classes.push("ui-label--custom-class".to_string());
            classes.push("docs-label-workbench".to_string());
        }

        format!(
            "LabelActualConfig {{\n  emphasis: {emphasis:?},\n  required: {required},\n  disabled: {disabled},\n  has_for_id: {with_for_id},\n  custom_text: {with_custom_text},\n  custom_indicator: {with_custom_indicator},\n  custom_class: {with_custom_class},\n  data_emphasis: \"{emphasis_attr}\",\n  data_state: \"{data_state}\",\n  data_label_source: \"{data_label_source}\",\n  data_indicator_source: \"{data_indicator_source}\",\n  data_class_source: \"{data_class_source}\",\n  class: \"{}\",\n}}",
            classes.join(" ")
        )
    });

    let emphasis_code = Signal::derive(move || {
        r#"<Label text="Name".to_string() for_id="name".to_string() required=true />
<Label text="Hint".to_string() emphasis=LabelEmphasis::Subtle />
<Label text="Critical".to_string() emphasis=LabelEmphasis::Strong required=true />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Label
  text="Assignee".to_string()
  for_id="assignee".to_string()
  required=true
  required_indicator="(required)".to_string()
  class_name="docs-label-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Label"
            slug="label"
            group="Forms"
            description="Form label primitive with centralized required/emphasis/source state contracts."
        >
            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="components/label/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Emphasis"</div>
                        <SegmentedControl
                            id_base="docs-label-workbench-emphasis".to_string()
                            options=emphasis_options.clone()
                            selected_index=emphasis_index
                            set_selected_index=set_emphasis_index
                            size=SegmentedControlSize::Sm
                            aria_label="Label workbench emphasis".to_string()
                        />

                        <Switch checked=is_required set_checked=set_is_required>
                            "Required"
                        </Switch>
                        <Switch checked=is_disabled set_checked=set_is_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=has_for_id set_checked=set_has_for_id>
                            "Bind for/id"
                        </Switch>
                        <Switch checked=custom_text set_checked=set_custom_text>
                            "Custom text"
                        </Switch>
                        <Switch checked=custom_indicator set_checked=set_custom_indicator>
                            "Custom required indicator"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-row" style="align-items: flex-start;">
                                <div class="docs-card" style="flex: 1 1 22rem; min-width: 16rem;">
                                    <div class="ui-muted">"Workbench"</div>
                                    <div class="docs-stack docs-stack--tight">
                                        <Label
                                            text=if custom_text.get() {
                                                "Assignee".to_string()
                                            } else {
                                                String::new()
                                            }
                                            for_id=if has_for_id.get() {
                                                "docs-label-workbench-input".to_string()
                                            } else {
                                                String::new()
                                            }
                                            required=is_required.get()
                                            disabled=is_disabled.get()
                                            emphasis=selected_emphasis.get()
                                            required_indicator=if custom_indicator.get() {
                                                "(required)".to_string()
                                            } else {
                                                String::new()
                                            }
                                            class_name=if custom_class.get() {
                                                "docs-label-workbench".to_string()
                                            } else {
                                                String::new()
                                            }
                                        />
                                        <input
                                            id="docs-label-workbench-input"
                                            class="docs-search__input"
                                            type="text"
                                            placeholder="Owner"
                                            disabled=is_disabled.get()
                                        />
                                    </div>
                                </div>

                                <div class="docs-card" style="flex: 1 1 22rem; min-width: 16rem;">
                                    <div class="ui-muted">"Comparison (Strong + Required + Custom Indicator)"</div>
                                    <div class="docs-stack docs-stack--tight">
                                        <Label
                                            text="Critical".to_string()
                                            for_id="docs-label-workbench-compare".to_string()
                                            required=true
                                            emphasis=LabelEmphasis::Strong
                                            required_indicator="(required)".to_string()
                                            class_name="docs-label-custom".to_string()
                                        />
                                        <input
                                            id="docs-label-workbench-compare"
                                            class="docs-search__input"
                                            type="text"
                                            placeholder="Critical owner"
                                        />
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="Emphasis + Required" code_signal=emphasis_code>
                <div class="docs-stack">
                    <Label text="Name".to_string() for_id="docs-label-name".to_string() required=true />
                    <input id="docs-label-name" class="docs-search__input" type="text" placeholder="Type name" />

                    <Label text="Hint".to_string() emphasis=LabelEmphasis::Subtle />
                    <Label text="Critical".to_string() emphasis=LabelEmphasis::Strong required=true />
                </div>
            </Playground>

            <Playground title="Custom Indicator + Class" code_signal=custom_code>
                <div class="docs-stack">
                    <Label
                        text="Assignee".to_string()
                        for_id="docs-label-assignee".to_string()
                        required=true
                        required_indicator="(required)".to_string()
                        class_name="docs-label-custom".to_string()
                    />
                    <input
                        id="docs-label-assignee"
                        class="docs-search__input"
                        type="text"
                        placeholder="Owner"
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn field() -> AnyView {
    let (workbench_orientation_key, set_workbench_orientation_key) = signal("vertical".to_string());
    let (workbench_tone_key, set_workbench_tone_key) = signal("default".to_string());
    let (workbench_required, set_workbench_required) = signal(true);
    let (workbench_invalid, set_workbench_invalid) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_error, set_workbench_custom_error) = signal(false);
    let (workbench_motion_ms, set_workbench_motion_ms) = signal(160_u16);

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
            snippet.push("  required=true".to_string());
        }
        if invalid {
            snippet.push("  invalid=true".to_string());
        }
        if disabled {
            snippet.push("  disabled=true".to_string());
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
        snippet.push(">".to_string());
        snippet.push(
            "  <input class=\"docs-search__input\" type=\"email\" placeholder=\"owner@company.com\" />".to_string(),
        );
        snippet.push("</Field>".to_string());
        snippet.join("\n")
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/field_form/field/styles.rs */\n{}",
            ui_components::field_form::field::styles::CSS
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
            "FieldActualConfig {{\n  orientation: {orientation:?},\n  tone: {tone:?},\n  required: {required},\n  invalid: {invalid},\n  disabled: {disabled},\n  custom_error: {custom_error},\n  custom_class: {custom_class},\n  motion_ms: {motion_ms},\n  data_state: \"{data_state}\",\n  error_source: \"{}\",\n  class_source: \"{}\",\n  class: \"{}\",\n}}",
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

    view! {
        <ComponentPage
            title="Field"
            slug="field"
            group="Forms"
            description="Form field wrapper with centralized orientation/tone/validation/message-state modeling and stable data contracts."
        >
            <Playground title="Required + Description" code_signal=required_code>
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

            <Playground title="Horizontal + Invalid + Custom Class" code_signal=invalid_code>
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

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with display/config/code/css-test panels for field state/source contracts."
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/field_form/field/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="field-workbench-controls">
                        <label class="docs-search__label">
                            "Orientation"
                            <select
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
                                prop:checked=move || workbench_required.get()
                                on:change=move |ev| set_workbench_required.set(event_target_checked(&ev))
                            />
                            " Required"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_invalid.get()
                                on:change=move |ev| set_workbench_invalid.set(event_target_checked(&ev))
                            />
                            " Invalid"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                            />
                            " Disabled"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_error.get()
                                on:change=move |ev| set_workbench_custom_error.set(event_target_checked(&ev))
                            />
                            " Custom error text"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
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
                    };

                    if custom_error && custom_class {
                        view! {
                            <Field
                                orientation=orientation
                                tone=tone
                                required=required
                                invalid=invalid
                                disabled=disabled
                                label="Email".to_string()
                                description="Inspect source/state marker contracts".to_string()
                                error_message="Custom validation error".to_string()
                                class_name="docs-field-custom".to_string()
                                motion=motion
                                aria_label="Workbench field".to_string()
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
                                required=required
                                invalid=invalid
                                disabled=disabled
                                label="Email".to_string()
                                description="Inspect source/state marker contracts".to_string()
                                error_message="Custom validation error".to_string()
                                motion=motion
                                aria_label="Workbench field".to_string()
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
                                required=required
                                invalid=invalid
                                disabled=disabled
                                label="Email".to_string()
                                description="Inspect source/state marker contracts".to_string()
                                class_name="docs-field-custom".to_string()
                                motion=motion
                                aria_label="Workbench field".to_string()
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
                                required=required
                                invalid=invalid
                                disabled=disabled
                                label="Email".to_string()
                                description="Inspect source/state marker contracts".to_string()
                                motion=motion
                                aria_label="Workbench field".to_string()
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
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn help_text() -> AnyView {
    let description_code = Signal::derive(move || {
        r#"<HelpText
  description="Use at least 12 characters.".to_string()
/>"#
        .to_string()
    });

    let error_code = Signal::derive(move || {
        r#"<HelpText
  invalid=true
  show_error_icon=true
  error_message="Password does not meet complexity requirements.".to_string()
  class_name="docs-help-text-custom".to_string()
/>"#
        .to_string()
    });
    let tone_options = vec![
        "Auto".to_string(),
        "Neutral".to_string(),
        "Negative".to_string(),
    ];
    let (tone_index, set_tone_index) = signal(Some(0_usize));
    let (is_invalid, set_is_invalid) = signal(false);
    let (is_disabled, set_is_disabled) = signal(false);
    let (show_error_icon, set_show_error_icon) = signal(true);
    let (use_error_message, set_use_error_message) = signal(true);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (show_compare, set_show_compare) = signal(true);

    let active_tone = Signal::derive(move || match tone_index.get().unwrap_or(0) {
        1 => HelpTextTone::Neutral,
        2 => HelpTextTone::Negative,
        _ => HelpTextTone::Auto,
    });
    let active_description = Signal::derive(move || {
        if is_invalid.get() && use_error_message.get() {
            None
        } else {
            Some("Use at least 12 characters.".to_string())
        }
    });
    let active_error_message = Signal::derive(move || {
        if is_invalid.get() && use_error_message.get() {
            Some("Password does not meet complexity requirements.".to_string())
        } else {
            None
        }
    });
    let active_aria_label = Signal::derive(move || {
        if custom_aria.get() {
            Some("Custom help text aria label".to_string())
        } else {
            None
        }
    });
    let active_class_name = Signal::derive(move || {
        if custom_class.get() {
            Some("docs-help-text-custom".to_string())
        } else {
            None
        }
    });
    let interactive_code = Signal::derive(move || {
        let tone = active_tone.get();
        let invalid = is_invalid.get();
        let disabled = is_disabled.get();
        let show_icon = show_error_icon.get();
        let description = active_description.get();
        let error_message = active_error_message.get();
        let aria = active_aria_label.get();
        let class_name = active_class_name.get();

        let mut lines = vec![
            "<HelpText".to_string(),
            format!("  tone=HelpTextTone::{tone:?}"),
            format!("  invalid={invalid}"),
            format!("  disabled={disabled}"),
            format!("  show_error_icon={show_icon}"),
        ];
        if let Some(description) = description {
            lines.push(format!("  description={description:?}.into()"));
        }
        if let Some(error_message) = error_message {
            lines.push(format!("  error_message={error_message:?}.into()"));
        }
        if let Some(aria) = aria {
            lines.push(format!("  aria_label={aria:?}.into()"));
        }
        if let Some(class_name) = class_name {
            lines.push(format!("  class_name={class_name:?}.into()"));
        }
        lines.push("/>".to_string());

        lines.join("\n")
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* components/help-text/src/styles.rs */\n{}",
            ui_components::field_form::help_text::styles::CSS
        )
    });
    let actual_config = Signal::derive(move || {
        let tone = active_tone.get();
        let invalid = is_invalid.get();
        let disabled = is_disabled.get();
        let show_icon = show_error_icon.get();
        let has_description = active_description.get().is_some();
        let has_error = active_error_message.get().is_some();
        let has_custom_aria = custom_aria.get();
        let has_custom_class = custom_class.get();
        format!(
            "HelpTextActualConfig {{\n  tone: HelpTextTone::{tone:?},\n  invalid: {invalid},\n  disabled: {disabled},\n  show_error_icon: {show_icon},\n  has_description: {has_description},\n  has_error_message: {has_error},\n  has_custom_aria_label: {has_custom_aria},\n  has_custom_class_name: {has_custom_class},\n}}"
        )
    });

    view! {
        <ComponentPage
            title="HelpText"
            slug="help-text"
            group="Forms"
            description="baseline-style form assistance primitive that resolves description vs error message and tone/icon state through centralized logic contracts."
        >
            <Playground title="Description (Neutral)" code_signal=description_code>
                <div class="docs-stack">
                    <HelpText
                        description="Use at least 12 characters.".to_string()
                        aria_label="Password hint".to_string()
                    />
                    <HelpText
                        tone=HelpTextTone::Neutral
                        description="This value is visible to project admins only.".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Invalid + Error Icon" code_signal=error_code>
                <div class="docs-stack">
                    <HelpText
                        invalid=true
                        show_error_icon=true
                        error_message="Password does not meet complexity requirements.".to_string()
                        class_name="docs-help-text-custom".to_string()
                    />
                    <HelpText
                        invalid=true
                        tone=HelpTextTone::Negative
                        error_message="Two-factor token expired. Request a new code.".to_string()
                        disabled=true
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                code_signal=interactive_code
                test_css_source=test_css_source
                test_source_path="components/help-text/src/styles.rs".to_string()
                test_config_signal=actual_config
                description="展示区 + Config 区 + Code 区 + CSS Test 区；支持 description/error/disabled/tone 的多场景对比。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"配置区 · Tone"</div>
                        <ui_components::SegmentedControl
                            id_base="docs-help-text-tone".to_string()
                            options=tone_options.clone()
                            selected_index=tone_index
                            set_selected_index=set_tone_index
                            size=ui_components::SegmentedControlSize::Sm
                            aria_label="HelpText tone".to_string()
                        />
                        <ui_components::Switch checked=is_invalid set_checked=set_is_invalid>
                            "Invalid"
                        </ui_components::Switch>
                        <ui_components::Switch checked=is_disabled set_checked=set_is_disabled>
                            "Disabled"
                        </ui_components::Switch>
                        <ui_components::Switch checked=show_error_icon set_checked=set_show_error_icon>
                            "Show error icon"
                        </ui_components::Switch>
                        <ui_components::Switch checked=use_error_message set_checked=set_use_error_message>
                            "Use error message"
                        </ui_components::Switch>
                        <ui_components::Switch checked=custom_aria set_checked=set_custom_aria>
                            "Custom aria label"
                        </ui_components::Switch>
                        <ui_components::Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </ui_components::Switch>
                        <ui_components::Switch checked=show_compare set_checked=set_show_compare>
                            "Show compare matrix"
                        </ui_components::Switch>
                    </div>
                }
            >
                {move || {
                    let tone = active_tone.get();
                    let invalid = is_invalid.get();
                    let disabled = is_disabled.get();
                    let show_icon = show_error_icon.get();
                    let description = active_description.get().unwrap_or_default();
                    let error_message = active_error_message.get().unwrap_or_default();
                    let aria_label = active_aria_label.get().unwrap_or_default();
                    let class_name = active_class_name.get().unwrap_or_default();
                    let compare = show_compare.get();

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"展示区 · Primary"</div>
                            <div class="docs-card docs-stack docs-stack--tight">
                                <HelpText
                                    tone=tone
                                    invalid=invalid
                                    disabled=disabled
                                    show_error_icon=show_icon
                                    description=description
                                    error_message=error_message
                                    aria_label=aria_label
                                    class_name=class_name
                                />
                            </div>

                            <Show when=move || compare>
                                <div class="docs-search__label">"展示区 · 对比矩阵"</div>
                                <div class="docs-stack docs-stack--tight">
                                    <HelpText
                                        tone=HelpTextTone::Neutral
                                        description="Neutral description state.".to_string()
                                    />
                                    <HelpText
                                        tone=HelpTextTone::Negative
                                        invalid=true
                                        show_error_icon=true
                                        error_message="Negative error state.".to_string()
                                    />
                                    <HelpText
                                        invalid=true
                                        disabled=true
                                        error_message="Disabled + invalid state.".to_string()
                                    />
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn textarea() -> AnyView {
    let (value_marker, set_value_marker) = signal("Pending review".to_string());
    let on_marker_value_change = Callback::new(move |next: String| set_value_marker.set(next));
    let (marker_invalid, set_marker_invalid) = signal(false);

    let basic_code = Signal::derive(move || {
        r#"<Textarea id="about".to_string()
  label="About".to_string()
  default_value="Write your release summary".to_string()
  placeholder="Write something…".to_string()
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"let (value, set_value) = signal("Pending review".to_string());
let on_value_change = Callback::new(move |next: String| {
  set_value.set(next);
});
let (invalid, set_invalid) = signal(false);
<Textarea
  id="summary".to_string()
  label="Summary".to_string()
  value=value
  on_value_change=on_value_change
  is_required=true
  is_invalid=Signal::derive(move || invalid.get())
  description="Inspect source/state marker contracts".to_string()
  error="Summary must include at least 20 characters.".to_string()
  placeholder="Write a summary".to_string()
  rows=5
  class_name="docs-textarea-state".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Textarea"
            slug="textarea"
            group="Forms"
            description="baseline-compatible textarea primitive with baseline-style text-field semantics and stable state/source markers. value axis uses `value/on_value_change/default_value`, and state booleans use canonical `is_*` inputs."
        >
            <Playground title="Basic Textarea" code_signal=basic_code>
                <Textarea
                    id="docs-textarea-basic".to_string()
                    label="About".to_string()
                    default_value="Write your release summary".to_string()
                    placeholder="Write something…".to_string()
                />
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect root markers like `data-state`, `data-value`, `data-requirement`, `data-label-source`, `data-description-source`, `data-error-source`, `data-placeholder-source`, and `data-rows-source`."
                code_signal=markers_code
            >
                <div class="docs-stack docs-stack--tight">
                    <Textarea
                        id="docs-textarea-marker".to_string()
                        label="Summary".to_string()
                        value=value_marker
                        on_value_change=on_marker_value_change
                        is_required=true
                        is_invalid=Signal::derive(move || marker_invalid.get())
                        description="Inspect source/state marker contracts".to_string()
                        error="Summary must include at least 20 characters.".to_string()
                        placeholder="Write a summary".to_string()
                        rows=5
                        class_name="docs-textarea-state".to_string()
                    />
                    <ui_components::Button
                        variant=ui_components::ButtonVariant::Secondary
                        on_press=Callback::new(move |_| {
                            set_marker_invalid.update(|value| *value = !*value)
                        })
                    >
                        {move || if marker_invalid.get() { "Clear marker invalid" } else { "Mark marker invalid" }}
                    </ui_components::Button>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn slider() -> AnyView {
    let (controlled_value_raw, set_controlled_value_raw) = signal(36.0_f64);
    let controlled_value = Signal::derive(move || controlled_value_raw.get());
    let (last_change, set_last_change) = signal("none".to_string());
    let on_value_change = Callback::new(move |next: f64| {
        set_controlled_value_raw.set(next);
        set_last_change.set(format!("{next:.1}"));
    });

    let (fine_value_raw, set_fine_value_raw) = signal(0.35_f64);
    let fine_value = Signal::derive(move || fine_value_raw.get());
    let on_fine_value_change = Callback::new(move |next: f64| set_fine_value_raw.set(next));

    let hello_code = Signal::derive(move || {
        r#"use leptos::prelude::*;
use ui_components::Slider;

<Slider
  label="Volume".to_string()
  default_value=36.0
  min=0.0
  max=100.0
  step=1.0
/>"#
        .to_string()
    });

    let code = Signal::derive(move || {
        r#"use leptos::prelude::*;
use ui_components::Slider;

let (value_raw, set_value_raw) = signal(36.0_f64);
let value = Signal::derive(move || value_raw.get());
let on_value_change = Callback::new(move |next: f64| set_value_raw.set(next));

<Slider
  id="docs-slider-volume".to_string()
  label="Volume".to_string()
  value=value
  default_value=20.0
  on_value_change=on_value_change
  min=0.0
  max=100.0
  step=1.0
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"use leptos::prelude::*;
use ui_components::{Slider, SliderMotion};

let (fine_value_raw, set_fine_value_raw) = signal(0.35_f64);
let fine_value = Signal::derive(move || fine_value_raw.get());
let on_fine_value_change = Callback::new(move |next: f64| set_fine_value_raw.set(next));

<Slider
  id="docs-slider-disabled".to_string()
  label="Disabled".to_string()
  default_value=68.0
  is_disabled=true
/>
<Slider
  id="docs-slider-fine".to_string()
  label="Fine Step".to_string()
  value=fine_value
  default_value=0.2
  on_value_change=on_fine_value_change
  min=0.0
  max=1.0
  step=0.05
  motion=SliderMotion::disabled()
/>"#
        .to_string()
    });

    let fine_motion = SliderMotion::disabled();

    view! {
        <ComponentPage
            title="Slider"
            slug="slider"
            group="Forms"
            description="Range slider with spring-driven fill/thumb motion and baseline-style state data contracts."
        >
            <Playground title="Hello World (Uncontrolled)" code_signal=hello_code>
                <Slider label="Volume".to_string() default_value=36.0 min=0.0 max=100.0 step=1.0 />
            </Playground>

            <Playground title="Controlled + Source Markers" code_signal=code>
                <div class="docs-stack">
                    <Slider
                        id="docs-slider-volume".to_string()
                        label="Volume".to_string()
                        value=controlled_value
                        default_value=20.0
                        on_value_change=on_value_change
                        min=0.0
                        max=100.0
                        step=1.0
                    />
                    <span class="ui-muted">
                        "value: " {move || format!("{:.1}", controlled_value_raw.get())}
                        " · last on_value_change: " {move || last_change.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Disabled + Fine Step" code_signal=states_code>
                <div class="docs-stack">
                    <Slider
                        id="docs-slider-disabled".to_string()
                        label="Disabled".to_string()
                        default_value=68.0
                        is_disabled=true
                    />
                    <Slider
                        id="docs-slider-fine".to_string()
                        label="Fine Step".to_string()
                        value=fine_value
                        default_value=0.2
                        on_value_change=on_fine_value_change
                        min=0.0
                        max=1.0
                        step=0.05
                        motion=fine_motion
                        class_name="docs-slider--fine".to_string()
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="slider-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui_components::*;\n\n<Slider\n  id=\"volume\".into()\n  label=\"Volume\".into()\n  default_value=36.0\n  min=0.0\n  max=100.0\n  step=1.0\n/>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-slider-source-copy".to_string()
                />
                <ul data-slot="slider-source-paths">
                    <li><code>"components/slider/src/mod.rs"</code></li>
                    <li><code>"components/slider/src/logic.rs"</code></li>
                    <li><code>"components/slider/src/view.rs"</code></li>
                    <li><code>"components/slider/src/styles.rs"</code></li>
                    <li><code>"components/slider/src/motion.rs"</code></li>
                </ul>
                <ul data-slot="slider-source-prerequisites">
                    <li><code>"component-slider"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn calendar() -> AnyView {
    let (interactive_month, set_interactive_month) = signal(3_u8);
    let (interactive_selected_day, set_interactive_selected_day) = signal(Some(12_u8));
    let (interactive_show_outside_days, set_interactive_show_outside_days) = signal(true);
    let (interactive_monday_first, set_interactive_monday_first) = signal(false);
    let (interactive_strong_tone, set_interactive_strong_tone) = signal(false);

    let code = Signal::derive(move || {
        r#"<Calendar
  year=2026
  month=1
  selected_day=Some(6)
  tone=CalendarTone::Default
  first_weekday=CalendarFirstWeekday::Sunday
  show_outside_days=true
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<Calendar
  year=2026
  month=2
  selected_day=Some(14)
  tone=CalendarTone::Strong
  first_weekday=CalendarFirstWeekday::Monday
  show_outside_days=false
  class_name="docs-calendar-custom".to_string()
/>"#
        .to_string()
    });

    let interactive_code = Signal::derive(move || {
        r#"let (month, set_month) = signal(3_u8);
let (selected_day, set_selected_day) = signal(Some(12_u8));
let (show_outside_days, set_show_outside_days) = signal(true);
let (monday_first, set_monday_first) = signal(false);
let (strong_tone, set_strong_tone) = signal(false);

<Calendar
  year=2026
  month=month.get()
  selected_day=selected_day.get()
  tone=if strong_tone.get() { CalendarTone::Strong } else { CalendarTone::Default }
  first_weekday=if monday_first.get() { CalendarFirstWeekday::Monday } else { CalendarFirstWeekday::Sunday }
  show_outside_days=show_outside_days.get()
/>"#.to_string()
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* components/calendar/src/styles.rs */\n{}",
            ui_components::calendar::styles::CSS
        )
    });
    let actual_config = Signal::derive(move || {
        let month = interactive_month.get();
        let selected_day = interactive_selected_day.get();
        let show_outside_days = interactive_show_outside_days.get();
        let monday_first = interactive_monday_first.get();
        let strong_tone = interactive_strong_tone.get();

        let mut classes = vec![
            "ui-calendar".to_string(),
            if strong_tone {
                "ui-calendar--tone-strong".to_string()
            } else {
                "ui-calendar--tone-default".to_string()
            },
            if monday_first {
                "ui-calendar--weekday-monday".to_string()
            } else {
                "ui-calendar--weekday-sunday".to_string()
            },
        ];
        if show_outside_days {
            classes.push("ui-calendar--outside-days".to_string());
        }
        if selected_day.is_some() {
            classes.push("ui-calendar--has-selection".to_string());
        }
        classes.push("ui-calendar--custom-class".to_string());
        classes.push("docs-calendar-interactive".to_string());

        format!(
            "CalendarActualConfig {{\n  year: 2026,\n  month: {month},\n  selected_day: {selected_day:?},\n  tone: {},\n  first_weekday: {},\n  show_outside_days: {show_outside_days},\n  class_name: \"docs-calendar-interactive\",\n  class: \"{}\",\n}}",
            if strong_tone { "Strong" } else { "Default" },
            if monday_first { "Monday" } else { "Sunday" },
            classes.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="Calendar"
            slug="calendar"
            group="Forms"
            description="Month-grid calendar with centralized date normalization and baseline-style tone/weekday/source state contracts."
        >
            <Playground title="Default + Outside Days" code_signal=code>
                <Calendar
                    year=2026
                    month=1
                    selected_day=Some(6)
                    tone=CalendarTone::Default
                    first_weekday=CalendarFirstWeekday::Sunday
                    show_outside_days=true
                />
            </Playground>

            <Playground title="Monday First + Strong Tone" code_signal=states_code>
                <Calendar
                    year=2026
                    month=2
                    selected_day=Some(14)
                    tone=CalendarTone::Strong
                    first_weekday=CalendarFirstWeekday::Monday
                    show_outside_days=false
                    class_name="docs-calendar-custom".to_string()
                />
            </Playground>

            <Playground
                title="Interactive Playground (State + Source Markers)"
                code_signal=interactive_code
                test_css_source=test_css_source
                test_source_path="/root/code/personal/omne/rust-ui/components/calendar/src/styles.rs".to_string()
                test_config_signal=actual_config
                description="Workbench canvas: 展示区覆盖默认/强强调对比，Config/Code/CSS Test 区用于契约校验。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="calendar-config-controls">
                        <div class="docs-search__label">"Month"</div>
                        <div class="docs-row">
                            <button
                                type="button"
                                data-action="prev-month-config"
                                on:click=move |_| {
                                    set_interactive_month
                                        .update(|month| *month = if *month <= 1 { 12 } else { *month - 1 });
                                }
                            >
                                "Prev"
                            </button>
                            <button
                                type="button"
                                data-action="next-month-config"
                                on:click=move |_| {
                                    set_interactive_month
                                        .update(|month| *month = if *month >= 12 { 1 } else { *month + 1 });
                                }
                            >
                                "Next"
                            </button>
                        </div>
                        <div class="docs-search__label">"Axes"</div>
                        <div class="docs-row">
                            <button
                                type="button"
                                data-action="toggle-weekday-config"
                                on:click=move |_| {
                                    set_interactive_monday_first.update(|value| *value = !*value);
                                }
                            >
                                "Weekday"
                            </button>
                            <button
                                type="button"
                                data-action="toggle-tone-config"
                                on:click=move |_| {
                                    set_interactive_strong_tone.update(|value| *value = !*value);
                                }
                            >
                                "Tone"
                            </button>
                            <button
                                type="button"
                                data-action="toggle-outside-days-config"
                                on:click=move |_| {
                                    set_interactive_show_outside_days.update(|value| *value = !*value);
                                }
                            >
                                "Outside days"
                            </button>
                        </div>
                        <button
                            type="button"
                            data-action="clear-selection-config"
                            on:click=move |_| {
                                set_interactive_selected_day.set(None);
                            }
                        >
                            "Clear selection"
                        </button>
                        <p class="ui-muted" data-slot="calendar-config-summary">
                            {move || {
                                format!(
                                    "config: month={} weekday={} tone={} outside_days={} selected_day={:?}",
                                    interactive_month.get(),
                                    if interactive_monday_first.get() {
                                        "monday"
                                    } else {
                                        "sunday"
                                    },
                                    if interactive_strong_tone.get() {
                                        "strong"
                                    } else {
                                        "default"
                                    },
                                    if interactive_show_outside_days.get() {
                                        "true"
                                    } else {
                                        "false"
                                    },
                                    interactive_selected_day.get(),
                                )
                            }}
                        </p>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="calendar-interactive-controls">
                    <div class="docs-row" data-slot="calendar-actions">
                        <button
                            type="button"
                            data-action="prev-month"
                            on:click=move |_| {
                                set_interactive_month
                                    .update(|month| *month = if *month <= 1 { 12 } else { *month - 1 });
                            }
                        >
                            "Prev month"
                        </button>
                        <button
                            type="button"
                            data-action="next-month"
                            on:click=move |_| {
                                set_interactive_month
                                    .update(|month| *month = if *month >= 12 { 1 } else { *month + 1 });
                            }
                        >
                            "Next month"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-weekday"
                            on:click=move |_| {
                                set_interactive_monday_first.update(|value| *value = !*value);
                            }
                        >
                            "Toggle first weekday"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-tone"
                            on:click=move |_| {
                                set_interactive_strong_tone.update(|value| *value = !*value);
                            }
                        >
                            "Toggle tone"
                        </button>
                        <button
                            type="button"
                            data-action="toggle-outside-days"
                            on:click=move |_| {
                                set_interactive_show_outside_days.update(|value| *value = !*value);
                            }
                        >
                            "Toggle outside days"
                        </button>
                        <button
                            type="button"
                            data-action="clear-selection"
                            on:click=move |_| {
                                set_interactive_selected_day.set(None);
                            }
                        >
                            "Clear selection"
                        </button>
                    </div>

                    <Calendar
                        year=2026
                        month=interactive_month.get()
                        selected_day=interactive_selected_day.get()
                        tone=if interactive_strong_tone.get() {
                            CalendarTone::Strong
                        } else {
                            CalendarTone::Default
                        }
                        first_weekday=if interactive_monday_first.get() {
                            CalendarFirstWeekday::Monday
                        } else {
                            CalendarFirstWeekday::Sunday
                        }
                        show_outside_days=interactive_show_outside_days.get()
                        on_day_press=Some(Callback::new(move |day| {
                            set_interactive_selected_day.set(Some(day));
                        }))
                        class_name="docs-calendar-interactive".to_string()
                    />

                    <p data-slot="calendar-interactive-summary">
                        {move || {
                            format!(
                                "month={} selected_day={:?} weekday={} tone={} outside_days={}",
                                interactive_month.get(),
                                interactive_selected_day.get(),
                                if interactive_monday_first.get() { "monday" } else { "sunday" },
                                if interactive_strong_tone.get() { "strong" } else { "default" },
                                if interactive_show_outside_days.get() {
                                    "true"
                                } else {
                                    "false"
                                }
                            )
                        }}
                    </p>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="calendar-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each calendar playground supports "
                    <code>"Show code"</code>
                    " + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui_components::*;\n\n<Calendar\n  year=2026\n  month=3\n  selected_day=Some(12)\n/>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-calendar-source-copy".to_string()
                />
                <ul data-slot="calendar-source-paths">
                    <li><code>"components/calendar/src/mod.rs"</code></li>
                    <li><code>"components/calendar/src/logic.rs"</code></li>
                    <li><code>"components/calendar/src/view.rs"</code></li>
                    <li><code>"components/calendar/src/styles.rs"</code></li>
                    <li><code>"components/calendar/src/motion.rs"</code></li>
                </ul>
                <ul data-slot="calendar-source-prerequisites">
                    <li><code>"component-calendar"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn date_picker() -> AnyView {
    let (workbench_month, set_workbench_month) = signal(5_u8);
    let (workbench_selected_day, set_workbench_selected_day) = signal(Some(18_u8));
    let (workbench_open, set_workbench_open) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_strong_tone, set_workbench_strong_tone) = signal(false);
    let (workbench_monday_first, set_workbench_monday_first) = signal(false);
    let (workbench_show_outside_days, set_workbench_show_outside_days) = signal(true);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_text, set_workbench_custom_text) = signal(false);

    let workbench_open_signal = Signal::derive(move || workbench_open.get());
    let workbench_selected_day_signal = Signal::derive(move || workbench_selected_day.get());
    let on_workbench_open_change = Callback::new(move |next: bool| set_workbench_open.set(next));
    let on_workbench_selected_day_change =
        Callback::new(move |next: Option<u8>| set_workbench_selected_day.set(next));

    let code = Signal::derive(move || {
        r#"<DatePicker
  id_base="release-date".to_string()
  year=2026
  month=3
  default_selected_day=12
  tone=DatePickerTone::Default
  first_weekday=CalendarFirstWeekday::Sunday
  show_outside_days=true
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<DatePicker
  id_base="ship-date".to_string()
  year=2026
  month=4
  default_selected_day=21
  tone=DatePickerTone::Strong
  first_weekday=CalendarFirstWeekday::Monday
  show_outside_days=false
  placeholder="Pick ship date".to_string()
  class_name="docs-date-picker-custom".to_string()
  motion=DatePickerMotion {
    popover: PopoverMotion { initial_scale: 0.95, offset_y_px: 10.0, ..PopoverMotion::default() },
  }
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let month = workbench_month.get();
        let selected_day = workbench_selected_day.get();
        let open = workbench_open.get();
        let disabled = workbench_disabled.get();
        let tone = if workbench_strong_tone.get() {
            "DatePickerTone::Strong"
        } else {
            "DatePickerTone::Default"
        };
        let first_weekday = if workbench_monday_first.get() {
            "CalendarFirstWeekday::Monday"
        } else {
            "CalendarFirstWeekday::Sunday"
        };
        let show_outside_days = workbench_show_outside_days.get();
        let motion = if workbench_custom_motion.get() {
            "DatePickerMotion { popover: PopoverMotion { initial_scale: 0.92, offset_y_px: 12.0, ..PopoverMotion::default() } }"
        } else {
            "DatePickerMotion::default()"
        };
        let placeholder = if workbench_custom_text.get() {
            "\"Pick ship date\".into()"
        } else {
            "\"\".into()"
        };
        let aria_label = if workbench_custom_text.get() {
            "\"Ship date picker\".into()"
        } else {
            "\"\".into()"
        };
        let class_name = if workbench_custom_text.get() {
            "\"docs-date-picker-custom\".into()"
        } else {
            "\"\".into()"
        };
        format!(
            "let (open, set_open) = signal({open});\nlet (selected_day, set_selected_day) = signal({selected_day:?});\n\n<DatePicker\n  id_base=\"docs-date-picker-workbench\".into()\n  year=2026\n  month={month}\n  tone={tone}\n  disabled={disabled}\n  open=Signal::derive(move || open.get())\n  on_open_change=Callback::new(move |next| set_open.set(next))\n  selected_day=Signal::derive(move || selected_day.get())\n  on_selected_day_change=Callback::new(move |next| set_selected_day.set(next))\n  first_weekday={first_weekday}\n  show_outside_days={show_outside_days}\n  motion={motion}\n  placeholder={placeholder}\n  aria_label={aria_label}\n  class_name={class_name}\n/>"
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "DatePickerActualConfig {{\n  month: {},\n  selected_day: {:?},\n  open: {},\n  disabled: {},\n  tone: {},\n  first_weekday: {},\n  show_outside_days: {},\n  custom_motion: {},\n  custom_text: {},\n}}",
            workbench_month.get(),
            workbench_selected_day.get(),
            workbench_open.get(),
            workbench_disabled.get(),
            if workbench_strong_tone.get() {
                "Strong"
            } else {
                "Default"
            },
            if workbench_monday_first.get() {
                "Monday"
            } else {
                "Sunday"
            },
            workbench_show_outside_days.get(),
            workbench_custom_motion.get(),
            workbench_custom_text.get(),
        )
    });

    let workbench_test_css_source: Signal<String> = Signal::derive(move || {
        include_str!("../../../../../../components/text-input/src/date_picker/styles.rs")
            .to_string()
    });

    view! {
        <ComponentPage
            title="DatePicker"
            slug="date-picker"
            group="Forms"
            description="Date picker trigger + popover calendar with centralized open/value/source state contracts and baseline-level popover motion handoff."
        >
            <Playground title="Default + Outside Days" code_signal=code>
                <DatePicker
                    id_base="docs-date-picker-release".to_string()
                    year=2026
                    month=3
                    default_selected_day=12
                    tone=DatePickerTone::Default
                    first_weekday=CalendarFirstWeekday::Sunday
                    show_outside_days=true
                />
            </Playground>

            <Playground title="Monday First + Strong Tone" code_signal=states_code>
                <DatePicker
                    id_base="docs-date-picker-ship".to_string()
                    year=2026
                    month=4
                    default_selected_day=21
                    tone=DatePickerTone::Strong
                    first_weekday=CalendarFirstWeekday::Monday
                    show_outside_days=false
                    placeholder="Pick ship date".to_string()
                    class_name="docs-date-picker-custom".to_string()
                    motion=DatePickerMotion {
                        popover: PopoverMotion {
                            initial_scale: 0.95,
                            offset_y_px: 10.0,
                            ..PopoverMotion::default()
                        },
                    }
                />
            </Playground>

            <Playground
                title="展示 / Config / Code / CSS Test"
                description="Workbench canvas: preview (展示) + settings panel (config) + copy-ready source (code) + scoped css verification (css test)."
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="components/text-input/src/date_picker/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="date-picker-workbench-controls">
                        <div class="docs-row">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_month
                                        .update(|value| *value = if *value <= 1 { 12 } else { *value - 1 });
                                })
                            >
                                "Prev month"
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_month
                                        .update(|value| *value = if *value >= 12 { 1 } else { *value + 1 });
                                })
                            >
                                "Next month"
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_selected_day.set(None);
                                })
                            >
                                "Clear day"
                            </ui_components::Button>
                        </div>

                        <div class="docs-row">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_open.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_open.get() { "Set closed" } else { "Set open" }}
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_disabled.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_disabled.get() { "Set enabled" } else { "Set disabled" }}
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_strong_tone.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_strong_tone.get() { "Tone strong" } else { "Tone default" }}
                            </ui_components::Button>
                        </div>

                        <div class="docs-row">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_monday_first.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_monday_first.get() {
                                    "Weekday Monday"
                                } else {
                                    "Weekday Sunday"
                                }}
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_show_outside_days.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_show_outside_days.get() {
                                    "Outside days on"
                                } else {
                                    "Outside days off"
                                }}
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_motion.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_custom_motion.get() {
                                    "Motion custom"
                                } else {
                                    "Motion default"
                                }}
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_text.update(|value| *value = !*value);
                                })
                            >
                                {move || if workbench_custom_text.get() {
                                    "Text custom"
                                } else {
                                    "Text default"
                                }}
                            </ui_components::Button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="date-picker-workbench">
                    {move || {
                        let tone = if workbench_strong_tone.get() {
                            DatePickerTone::Strong
                        } else {
                            DatePickerTone::Default
                        };
                        let first_weekday = if workbench_monday_first.get() {
                            CalendarFirstWeekday::Monday
                        } else {
                            CalendarFirstWeekday::Sunday
                        };
                        let motion = if workbench_custom_motion.get() {
                            DatePickerMotion {
                                popover: PopoverMotion {
                                    initial_scale: 0.92,
                                    offset_y_px: 12.0,
                                    ..PopoverMotion::default()
                                },
                            }
                        } else {
                            DatePickerMotion::default()
                        };
                        let placeholder = if workbench_custom_text.get() {
                            "Pick ship date".to_string()
                        } else {
                            String::new()
                        };
                        let aria_label = if workbench_custom_text.get() {
                            "Ship date picker".to_string()
                        } else {
                            String::new()
                        };
                        let class_name = if workbench_custom_text.get() {
                            "docs-date-picker-custom".to_string()
                        } else {
                            String::new()
                        };

                        view! {
                            <DatePicker
                                id_base="docs-date-picker-workbench".to_string()
                                year=2026
                                month=workbench_month.get()
                                tone=tone
                                disabled=workbench_disabled.get()
                                open=workbench_open_signal
                                on_open_change=on_workbench_open_change
                                selected_day=workbench_selected_day_signal
                                on_selected_day_change=on_workbench_selected_day_change
                                first_weekday=first_weekday
                                show_outside_days=workbench_show_outside_days.get()
                                motion=motion
                                placeholder=placeholder
                                aria_label=aria_label
                                class_name=class_name
                            />
                        }
                    }}

                    <span class="ui-muted" data-slot="date-picker-workbench-summary">
                        {move || format!(
                            "month={} · selected_day={:?} · open={} · disabled={} · tone={} · weekday={} · outside_days={} · custom_motion={} · custom_text={}",
                            workbench_month.get(),
                            workbench_selected_day.get(),
                            workbench_open.get(),
                            workbench_disabled.get(),
                            if workbench_strong_tone.get() { "strong" } else { "default" },
                            if workbench_monday_first.get() { "monday" } else { "sunday" },
                            workbench_show_outside_days.get(),
                            workbench_custom_motion.get(),
                            workbench_custom_text.get(),
                        )}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Comparison Matrix (Default / Quiet / Strong / Disabled)"
                description="Multiple side-by-side states for quick visual and semantic contract comparison."
                code_signal=Signal::derive(move || r#"<DatePicker id_base="date-default".to_string() year=2026 month=3 default_selected_day=12 />
<DatePicker id_base="date-quiet".to_string() year=2026 month=3 tone=DatePickerTone::Quiet />
<DatePicker id_base="date-strong".to_string() year=2026 month=4 tone=DatePickerTone::Strong default_selected_day=21 first_weekday=CalendarFirstWeekday::Monday />
<DatePicker id_base="date-disabled".to_string() year=2026 month=4 disabled=true placeholder="Unavailable".to_string() />"#.to_string())
            >
                <div class="docs-grid docs-grid--2" data-slot="date-picker-comparison-matrix">
                    <DatePicker
                        id_base="docs-date-picker-compare-default".to_string()
                        year=2026
                        month=3
                        default_selected_day=12
                    />
                    <DatePicker
                        id_base="docs-date-picker-compare-quiet".to_string()
                        year=2026
                        month=3
                        tone=DatePickerTone::Quiet
                    />
                    <DatePicker
                        id_base="docs-date-picker-compare-strong".to_string()
                        year=2026
                        month=4
                        tone=DatePickerTone::Strong
                        default_selected_day=21
                        first_weekday=CalendarFirstWeekday::Monday
                    />
                    <DatePicker
                        id_base="docs-date-picker-compare-disabled".to_string()
                        year=2026
                        month=4
                        disabled=true
                        placeholder="Unavailable".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn time_field() -> AnyView {
    let (value, set_value) = signal(Some("09:30".to_string()));
    let on_value_change = Callback::new(move |next: Option<String>| {
        set_value.set(next);
    });
    let (marker_value, set_marker_value) = signal(Some("08:15".to_string()));
    let on_marker_value_change = Callback::new(move |next: Option<String>| {
        set_marker_value.set(next);
    });
    let (marker_is_disabled, set_marker_is_disabled) = signal(false);
    let (marker_minute_step, set_marker_minute_step) = signal(5_u8);
    let (marker_strong_tone, set_marker_strong_tone) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<TimeField
  id_base="meeting-time".to_string()
  label="Meeting time".to_string()
/>"#
        .to_string()
    });

    let marker_code = Signal::derive(move || {
        r#"let (marker_value, set_marker_value) = signal(Some("08:15".to_string()));
let on_value_change = Callback::new(move |next: Option<String>| {
  set_marker_value.set(next);
});
let (marker_is_disabled, set_marker_is_disabled) = signal(false);
let (marker_minute_step, set_marker_minute_step) = signal(5_u8);

<TimeField
  id_base="docs-time-field-marker".to_string()
  label="Marker playground".to_string()
  value=marker_value
  on_value_change=on_value_change
  is_disabled=marker_is_disabled.get()
  minute_step=marker_minute_step.get()
/>"#
        .to_string()
    });

    let code = Signal::derive(move || {
        r#"let (value, set_value) = signal(Some("09:30".to_string()));
let on_value_change = Callback::new(move |next: Option<String>| {
  set_value.set(next);
});

<TimeField
  id_base="meeting-time".to_string()
  label="Meeting time".to_string()
  value=value
  on_value_change=on_value_change
  minute_step=15
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<TimeField
  id_base="ship-window".to_string()
  label="Ship window".to_string()
  tone=TimeFieldTone::Strong
  minute_step=5
  default_value="18:45".to_string()
  placeholder="hour:minute".to_string()
  class_name="docs-time-field-custom".to_string()
/>"#
        .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<TimeField
  id_base="quiet-hours".to_string()
  label="Quiet hours".to_string()
  default_value="22:00".to_string()
  is_disabled=true
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="TimeField"
            slug="time-field"
            group="Forms"
            description="Time entry field with centralized hour/minute normalization and baseline-style state/source data contracts."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <TimeField
                    id_base="docs-time-field-hello".to_string()
                    label="Meeting time".to_string()
                />
            </Playground>

            <Playground
                title="Interactive Playground (State + Source Markers)"
                description="Inspect `data-state`, `data-control-mode`, `data-value-source`, `data-default-value-source`, and `data-value-change-source` while toggling disabled/step/tone and editing value."
                code_signal=marker_code
            >
                <div class="docs-stack">
                    <TimeField
                        id_base="docs-time-field-marker".to_string()
                        label="Marker playground".to_string()
                        value=marker_value
                        on_value_change=on_marker_value_change
                        is_disabled=marker_is_disabled.get()
                        minute_step=marker_minute_step.get()
                        tone=if marker_strong_tone.get() {
                            TimeFieldTone::Strong
                        } else {
                            TimeFieldTone::Default
                        }
                    />
                    <div class="docs-row" data-slot="time-field-marker-controls">
                        <div data-slot="time-field-toggle-disabled">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_is_disabled.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_is_disabled.get() {
                                    "Set enabled"
                                } else {
                                    "Set disabled"
                                }}
                            </ui_components::Button>
                        </div>

                        <div data-slot="time-field-toggle-step">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_minute_step.update(|value| {
                                        *value = if *value == 5 { 15 } else { 5 };
                                    })
                                })
                            >
                                {move || if marker_minute_step.get() == 5 {
                                    "Step 5"
                                } else {
                                    "Step 15"
                                }}
                            </ui_components::Button>
                        </div>

                        <div data-slot="time-field-toggle-tone">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_strong_tone.update(|value| *value = !*value)
                                })
                            >
                                {move || if marker_strong_tone.get() {
                                    "Tone strong"
                                } else {
                                    "Tone default"
                                }}
                            </ui_components::Button>
                        </div>

                        <div data-slot="time-field-reset-value">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_marker_value.set(Some("08:15".to_string()));
                                })
                            >
                                "Reset 08:15"
                            </ui_components::Button>
                        </div>
                    </div>
                    <span class="ui-muted" data-slot="time-field-marker-summary">
                        "value: " {move || marker_value.get().unwrap_or_else(|| "none".to_string())}
                        " · disabled: " {move || marker_is_disabled.get()}
                        " · step: " {move || marker_minute_step.get()}
                        " · tone: "
                        {move || if marker_strong_tone.get() {
                            "strong"
                        } else {
                            "default"
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Controlled + Step 15" code_signal=code>
                <div class="docs-stack">
                    <TimeField
                        id_base="docs-time-field-controlled".to_string()
                        label="Meeting time".to_string()
                        value=value
                        on_value_change=on_value_change
                        minute_step=15
                    />
                    <span class="ui-muted">
                        "value: "
                        {move || value.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Strong Tone + Custom Placeholder" code_signal=states_code>
                <TimeField
                    id_base="docs-time-field-strong".to_string()
                    label="Ship window".to_string()
                    tone=TimeFieldTone::Strong
                    minute_step=5
                    default_value="18:45".to_string()
                    placeholder="hour:minute".to_string()
                    class_name="docs-time-field-custom".to_string()
                />
            </Playground>

            <Playground title="Disabled + Uncontrolled (Default Step)" code_signal=disabled_code>
                <TimeField
                    id_base="docs-time-field-disabled".to_string()
                    label="Quiet hours".to_string()
                    default_value="22:00".to_string()
                    is_disabled=true
                />
            </Playground>

            <section class="docs-card docs-prose" data-slot="time-field-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui_components::*;\n\n<TimeField\n  id_base=\"meeting-time\".into()\n  label=\"Meeting time\".into()\n/>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-time-field-source-copy".to_string()
                />
                <ul data-slot="time-field-source-paths">
                    <li><code>"components/text-input/src/time_field/mod.rs"</code></li>
                    <li><code>"components/text-input/src/time_field/logic.rs"</code></li>
                    <li><code>"components/text-input/src/time_field/view.rs"</code></li>
                    <li><code>"components/text-input/src/time_field/styles.rs"</code></li>
                    <li><code>"components/text-input/src/time_field/motion.rs"</code></li>
                </ul>
                <ul data-slot="time-field-source-prerequisites">
                    <li><code>"component-time_field"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn date_range_picker() -> AnyView {
    let (start_day, set_start_day) = signal(Some(8_u8));
    let (end_day, set_end_day) = signal(Some(19_u8));
    let (workbench_start_day, set_workbench_start_day) = signal(Some(10_u8));
    let (workbench_end_day, set_workbench_end_day) = signal(Some(18_u8));
    let (workbench_strong_tone, set_workbench_strong_tone) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_labels, set_workbench_custom_labels) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let on_start_day_change = Callback::new(move |next: Option<u8>| {
        set_start_day.set(next);
    });

    let on_end_day_change = Callback::new(move |next: Option<u8>| {
        set_end_day.set(next);
    });

    let code = Signal::derive(move || {
        r#"let (start_day, set_start_day) = signal(Some(8_u8));
let (end_day, set_end_day) = signal(Some(19_u8));

<DateRangePicker
  id_base="release-window".to_string()
  start_year=2026
  start_month=6
  end_year=2026
  end_month=6
  start_day=start_day
  end_day=end_day
  on_start_day_change=Callback::new(move |next| set_start_day.set(next))
  on_end_day_change=Callback::new(move |next| set_end_day.set(next))
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<DateRangePicker
  id_base="ship-window".to_string()
  start_year=2026
  start_month=7
  end_year=2026
  end_month=7
  default_start_day=20
  default_end_day=12
  tone=DateRangePickerTone::Strong
  class_name="docs-date-range-picker-custom".to_string()
/>"#
        .to_string()
    });
    let workbench_code = Signal::derive(move || {
        let mut lines = vec![
            "<DateRangePicker".to_string(),
            "  id_base=\"docs-date-range-picker-workbench\".into()".to_string(),
            "  start_year=2026".to_string(),
            "  start_month=8".to_string(),
            "  end_year=2026".to_string(),
            "  end_month=8".to_string(),
            format!(
                "  start_day=Signal::derive(|| Some({}_u8))",
                workbench_start_day.get().unwrap_or(0)
            ),
            format!(
                "  end_day=Signal::derive(|| Some({}_u8))",
                workbench_end_day.get().unwrap_or(0)
            ),
        ];

        if workbench_strong_tone.get() {
            lines.push("  tone=DateRangePickerTone::Strong".to_string());
        }
        if workbench_disabled.get() {
            lines.push("  disabled=true".to_string());
        }
        if workbench_custom_labels.get() {
            lines.push("  start_label=\"From\".into()".to_string());
            lines.push("  end_label=\"To\".into()".to_string());
            lines.push("  start_placeholder=\"Start window\".into()".to_string());
            lines.push("  end_placeholder=\"End window\".into()".to_string());
            lines.push("  invalid_range_message=\"Range is reversed\".into()".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-date-range-picker-custom\".into()".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* components/text-input/src/date_range_picker/styles.rs */\n{}",
            ui_components::text_input::date_range_picker::styles::CSS
        )
    });
    let actual_config = Signal::derive(move || {
        let start = workbench_start_day.get();
        let end = workbench_end_day.get();
        let tone = if workbench_strong_tone.get() {
            DateRangePickerTone::Strong
        } else {
            DateRangePickerTone::Default
        };
        let disabled = workbench_disabled.get();
        let has_custom_labels = workbench_custom_labels.get();
        let has_custom_class = workbench_custom_class.get();
        let has_start = start.is_some();
        let has_end = end.is_some();
        let is_invalid = matches!((start, end), (Some(s), Some(e)) if s > e);
        let has_full_value = has_start && has_end;
        let is_partial = has_start ^ has_end;
        let data_state = if disabled {
            "disabled"
        } else if is_invalid {
            "invalid"
        } else if has_full_value {
            "value"
        } else if is_partial {
            "partial"
        } else {
            "empty"
        };

        let mut classes = vec!["ui-date-range-picker".to_string(), tone.class_name().into()];
        if disabled {
            classes.push("ui-date-range-picker--disabled".to_string());
        }
        if has_start {
            classes.push("ui-date-range-picker--has-start".to_string());
        }
        if has_end {
            classes.push("ui-date-range-picker--has-end".to_string());
        }
        if has_full_value {
            classes.push("ui-date-range-picker--has-full-value".to_string());
        }
        if is_partial {
            classes.push("ui-date-range-picker--partial".to_string());
        }
        if is_invalid {
            classes.push("ui-date-range-picker--invalid-range".to_string());
        }
        if has_custom_class {
            classes.push("ui-date-range-picker--custom-class".to_string());
            classes.push("docs-date-range-picker-custom".to_string());
        }

        format!(
            "DateRangePickerActualConfig {{\n  tone: {tone:?},\n  disabled: {disabled},\n  start_day: {start:?},\n  end_day: {end:?},\n  has_custom_label_set: {has_custom_labels},\n  has_custom_class_name: {has_custom_class},\n  is_invalid_range: {is_invalid},\n  data_state: \"{data_state}\",\n  class: \"{}\",\n}}",
            classes.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="DateRangePicker"
            slug="date-range-picker"
            group="Forms"
            description="Two DatePicker composition with centralized range validity/value-shape derivation and baseline-style state/source contracts."
        >
            <Playground title="Controlled + Shared Month" code_signal=code>
                <div class="docs-stack">
                    <DateRangePicker
                        id_base="docs-date-range-picker-controlled".to_string()
                        start_year=2026
                        start_month=6
                        end_year=2026
                        end_month=6
                        start_day=start_day
                        end_day=end_day
                        on_start_day_change=on_start_day_change
                        on_end_day_change=on_end_day_change
                    />

                    <span class="ui-muted">
                        "start: " {move || start_day.get().map(|d| d.to_string()).unwrap_or_else(|| "none".to_string())}
                        " · end: " {move || end_day.get().map(|d| d.to_string()).unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Strong Tone + Invalid Range Hint" code_signal=states_code>
                <DateRangePicker
                    id_base="docs-date-range-picker-strong".to_string()
                    start_year=2026
                    start_month=7
                    end_year=2026
                    end_month=7
                    default_start_day=20
                    default_end_day=12
                    tone=DateRangePickerTone::Strong
                    class_name="docs-date-range-picker-custom".to_string()
                />
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="展示区含实时配置与对比样例；Config/Code/CSS Test 区用于契约回归。"
                code_signal=workbench_code
                test_css_source=test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/text-input/src/date_range_picker/styles.rs".to_string()
                test_config_signal=actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="date-range-picker-workbench-controls">
                        <div class="docs-row">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_start_day
                                        .update(|v| *v = Some(v.unwrap_or(1).saturating_sub(1).max(1)))
                                })
                            >
                                "Start -1"
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_start_day
                                        .update(|v| *v = Some((v.unwrap_or(1) + 1).min(31)))
                                })
                            >
                                "Start +1"
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_end_day
                                        .update(|v| *v = Some(v.unwrap_or(1).saturating_sub(1).max(1)))
                                })
                            >
                                "End -1"
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_end_day
                                        .update(|v| *v = Some((v.unwrap_or(1) + 1).min(31)))
                                })
                            >
                                "End +1"
                            </ui_components::Button>
                        </div>

                        <div class="docs-row">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_start_day.set(Some(8));
                                    set_workbench_end_day.set(Some(20));
                                })
                            >
                                "Preset valid"
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_start_day.set(Some(20));
                                    set_workbench_end_day.set(Some(8));
                                })
                            >
                                "Preset invalid"
                            </ui_components::Button>
                        </div>

                        <div class="docs-row">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_strong_tone.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_strong_tone.get() {
                                    "Tone: Strong"
                                } else {
                                    "Tone: Default"
                                }}
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_disabled.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_disabled.get() {
                                    "Disabled: on"
                                } else {
                                    "Disabled: off"
                                }}
                            </ui_components::Button>
                        </div>

                        <div class="docs-row">
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_labels.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_custom_labels.get() {
                                    "Custom labels on"
                                } else {
                                    "Custom labels off"
                                }}
                            </ui_components::Button>
                            <ui_components::Button
                                variant=ui_components::ButtonVariant::Secondary
                                on_press=Callback::new(move |_| {
                                    set_workbench_custom_class.update(|v| *v = !*v)
                                })
                            >
                                {move || if workbench_custom_class.get() {
                                    "Custom class: on"
                                } else {
                                    "Custom class: off"
                                }}
                            </ui_components::Button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="date-range-picker-workbench-preview">
                    <DateRangePicker
                        id_base="docs-date-range-picker-workbench".to_string()
                        start_year=2026
                        start_month=8
                        end_year=2026
                        end_month=8
                        start_day=workbench_start_day
                        end_day=workbench_end_day
                        on_start_day_change=Callback::new(move |next| set_workbench_start_day.set(next))
                        on_end_day_change=Callback::new(move |next| set_workbench_end_day.set(next))
                        tone=if workbench_strong_tone.get() {
                            DateRangePickerTone::Strong
                        } else {
                            DateRangePickerTone::Default
                        }
                        disabled=workbench_disabled.get()
                        start_label=if workbench_custom_labels.get() {
                            "From".to_string()
                        } else {
                            String::new()
                        }
                        end_label=if workbench_custom_labels.get() {
                            "To".to_string()
                        } else {
                            String::new()
                        }
                        start_placeholder=if workbench_custom_labels.get() {
                            "Start window".to_string()
                        } else {
                            String::new()
                        }
                        end_placeholder=if workbench_custom_labels.get() {
                            "End window".to_string()
                        } else {
                            String::new()
                        }
                        invalid_range_message=if workbench_custom_labels.get() {
                            "Range is reversed".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-date-range-picker-custom".to_string()
                        } else {
                            String::new()
                        }
                    />

                    <div class="docs-row">
                        <span class="ui-muted">
                            "start: " {move || workbench_start_day.get().map(|d| d.to_string()).unwrap_or_else(|| "none".to_string())}
                            " · end: " {move || workbench_end_day.get().map(|d| d.to_string()).unwrap_or_else(|| "none".to_string())}
                            " · invalid: "
                            {move || match (workbench_start_day.get(), workbench_end_day.get()) {
                                (Some(s), Some(e)) => (s > e).to_string(),
                                _ => "false".to_string(),
                            }}
                        </span>
                    </div>

                    <div class="docs-row">
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"对比：Valid"</span>
                            <DateRangePicker
                                id_base="docs-date-range-picker-compare-valid".to_string()
                                start_year=2026
                                start_month=8
                                end_year=2026
                                end_month=8
                                default_start_day=5
                                default_end_day=16
                            />
                        </div>
                        <div class="docs-stack docs-stack--tight">
                            <span class="ui-muted">"对比：Invalid + Strong"</span>
                            <DateRangePicker
                                id_base="docs-date-range-picker-compare-invalid".to_string()
                                start_year=2026
                                start_month=8
                                end_year=2026
                                end_month=8
                                default_start_day=22
                                default_end_day=7
                                tone=DateRangePickerTone::Strong
                                class_name="docs-date-range-picker-custom".to_string()
                            />
                        </div>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn date_field() -> AnyView {
    let (value, set_value) = signal(Some("2026-03-14".to_string()));
    let on_value_change = Callback::new(move |next: Option<String>| {
        set_value.set(next);
    });

    let code = Signal::derive(move || {
        r#"let (value, set_value) = signal(Some("2026-03-14".to_string()));
let on_value_change = Callback::new(move |next: Option<String>| {
  set_value.set(next);
});

<DateField
  id_base="invoice-date".to_string()
  label="Invoice date".to_string()
  value=value
  on_value_change=on_value_change
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<DateField
  id_base="ship-date".to_string()
  label="Ship date".to_string()
  tone=DateFieldTone::Strong
  default_value="2026-07-22".to_string()
  placeholder="year-month-day".to_string()
  class_name="docs-date-field-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="DateField"
            slug="date-field"
            group="Forms"
            description="Segmented date entry field with centralized year/month/day normalization and baseline-style state/source contracts."
        >
            <Playground title="Controlled Value" code_signal=code>
                <div class="docs-stack">
                    <DateField
                        id_base="docs-date-field-controlled".to_string()
                        label="Invoice date".to_string()
                        value=value
                        on_value_change=on_value_change
                    />
                    <span class="ui-muted">
                        "value: " {move || value.get().unwrap_or_else(|| "none".to_string())}
                    </span>
                </div>
            </Playground>

            <Playground title="Strong Tone + Custom Placeholder" code_signal=states_code>
                <DateField
                    id_base="docs-date-field-strong".to_string()
                    label="Ship date".to_string()
                    tone=DateFieldTone::Strong
                    default_value="2026-07-22".to_string()
                    placeholder="year-month-day".to_string()
                    class_name="docs-date-field-custom".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
