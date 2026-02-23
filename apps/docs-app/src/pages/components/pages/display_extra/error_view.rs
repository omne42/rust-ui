use super::*;

pub(crate) fn error_view() -> AnyView {
    let error_view_imports =
        "use leptos::prelude::*;\nuse ui::{ErrorView, ErrorViewMotion, ErrorViewTone, Icon, IconSize, IconTone};"
            .to_string();
    let tone_options = ["Negative".to_string(), "Neutral".to_string()];
    let message_options = ["Email".to_string(), "Retry".to_string()];
    let (tone_index, set_tone_index) = signal(Some(0_usize));
    let (message_index, set_message_index) = signal(Some(0_usize));
    let (is_invalid, set_is_invalid) = signal(true);
    let (is_compact, set_is_compact) = signal(false);
    let (is_bordered, set_is_bordered) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let (custom_aria, set_custom_aria) = signal(true);
    let (custom_class, set_custom_class) = signal(false);
    let (with_icon, set_with_icon) = signal(true);
    let (with_actions, set_with_actions) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let workbench_tone = Signal::derive(move || match tone_index.get().unwrap_or(0) {
        1 => ErrorViewTone::Neutral,
        _ => ErrorViewTone::Negative,
    });
    let workbench_message = Signal::derive(move || match message_index.get().unwrap_or(0) {
        1 => "Retry request failed. Try again.".to_string(),
        _ => "Please enter a valid email address".to_string(),
    });
    let workbench_motion = Signal::derive(move || {
        if custom_motion.get() {
            ErrorViewMotion {
                hidden_translate_px: 12.0,
                hidden_opacity: 0.0,
                hidden_scale: 0.95,
                ..ErrorViewMotion::default()
            }
        } else {
            ErrorViewMotion::default()
        }
    });
    let workbench_aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Validation feedback".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-error-view-custom".to_string()
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
            ui_headless::A11yDirection::Rtl
        } else {
            ui_headless::A11yDirection::Ltr
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<ErrorView
  is_invalid=true
  message="Please enter a valid email address".to_string()
  lang="en-US".to_string()
  dir=ui_headless::A11yDirection::Ltr
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<ErrorView\n  is_invalid={}\n  tone={:?}\n  is_compact={}\n  is_bordered={}\n  motion={:?}\n  message={}\n  aria_label={}\n  class_name={}\n  icon={}\n  actions={}\n  lang={}\n  dir={:?}\n/>",
            bool_word(is_invalid.get()),
            workbench_tone.get(),
            bool_word(is_compact.get()),
            bool_word(is_bordered.get()),
            workbench_motion.get(),
            rust_string_literal(&workbench_message.get()),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
            if with_icon.get() {
                "Some(icon)"
            } else {
                "None"
            },
            if with_actions.get() {
                "Some(actions)"
            } else {
                "None"
            },
            rust_string_literal(&workbench_lang.get()),
            workbench_dir.get(),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ErrorViewActualConfig {{\n  is_invalid: {},\n  tone: {:?},\n  is_compact: {},\n  is_bordered: {},\n  motion: {:?},\n  message: {:?},\n  aria_label: {:?},\n  class_name: {:?},\n  icon: {},\n  actions: {},\n  lang: {:?},\n  dir: {:?},\n}}",
            is_invalid.get(),
            workbench_tone.get(),
            is_compact.get(),
            is_bordered.get(),
            workbench_motion.get(),
            workbench_message.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
            if with_icon.get() { "Some" } else { "None" },
            if with_actions.get() { "Some" } else { "None" },
            workbench_lang.get(),
            workbench_dir.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ErrorView
  is_invalid=true
  tone=ErrorViewTone::Negative
  is_compact=false
  is_bordered=false
  message="Email format invalid".to_string()
  lang="en-US".to_string()
  dir=ui_headless::A11yDirection::Ltr
/>
<ErrorView
  is_invalid=true
  tone=ErrorViewTone::Neutral
  is_compact=true
  is_bordered=true
  motion=ErrorViewMotion { hidden_translate_px: 12.0, hidden_opacity: 0.0, hidden_scale: 0.95, ..ErrorViewMotion::default() }
  message="Retry request failed".to_string()
  aria_label="Validation feedback".to_string()
  class_name="docs-error-view-custom".to_string()
  icon=move || view! { <Icon size=IconSize::Sm tone=IconTone::Danger is_decorative=true>"⚠"</Icon> }
  actions=move || view! { <ui::Button variant=ui::ButtonVariant::Secondary>"Retry"</ui::Button> }
  lang="ar".to_string()
  dir=ui_headless::A11yDirection::Rtl
/>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ErrorView"
            slug="error-view"
            group="Display"
            description="Validation error container with tone/layout/message/motion and slot actions."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports=error_view_imports.clone()
                test_source_path="components/error-view/src/view.rs".to_string()
            >
                <ErrorView
                    is_invalid=true
                    message="Please enter a valid email address".to_string()
                    lang="en-US".to_string()
                    dir=ui_headless::A11yDirection::Ltr
                />
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports=error_view_imports.clone()
                test_source_path="components/error-view/src/view.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="error-view-workbench-controls">
                            <div class="docs-search__label">"Tone"</div>
                            <select
                                class="docs-search__input"
                                prop:value=move || tone_index.get().unwrap_or(0).to_string()
                                on:change=move |event| {
                                    if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                        set_tone_index.set(Some(value.min(1)));
                                    }
                                }
                            >
                                {tone_options
                                    .iter()
                                    .enumerate()
                                    .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                    .collect_view()}
                            </select>

                            <div class="docs-search__label">"Message"</div>
                            <select
                                class="docs-search__input"
                                prop:value=move || message_index.get().unwrap_or(0).to_string()
                                on:change=move |event| {
                                    if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                        set_message_index.set(Some(value.min(1)));
                                    }
                                }
                            >
                                {message_options
                                    .iter()
                                    .enumerate()
                                    .map(|(index, label)| view! { <option value=index.to_string()>{label.clone()}</option> })
                                    .collect_view()}
                            </select>

                            <label class="docs-choice-row"><input type="checkbox" prop:checked=move || is_invalid.get() on:change=move |event| set_is_invalid.set(event_target_checked(&event)) /><span>"is_invalid"</span></label>
                            <label class="docs-choice-row"><input type="checkbox" prop:checked=move || is_compact.get() on:change=move |event| set_is_compact.set(event_target_checked(&event)) /><span>"is_compact"</span></label>
                            <label class="docs-choice-row"><input type="checkbox" prop:checked=move || is_bordered.get() on:change=move |event| set_is_bordered.set(event_target_checked(&event)) /><span>"is_bordered"</span></label>
                            <label class="docs-choice-row"><input type="checkbox" prop:checked=move || custom_motion.get() on:change=move |event| set_custom_motion.set(event_target_checked(&event)) /><span>"custom motion"</span></label>
                            <label class="docs-choice-row"><input type="checkbox" prop:checked=move || custom_aria.get() on:change=move |event| set_custom_aria.set(event_target_checked(&event)) /><span>"custom aria_label"</span></label>
                            <label class="docs-choice-row"><input type="checkbox" prop:checked=move || custom_class.get() on:change=move |event| set_custom_class.set(event_target_checked(&event)) /><span>"custom class_name"</span></label>
                            <label class="docs-choice-row"><input type="checkbox" prop:checked=move || with_icon.get() on:change=move |event| set_with_icon.set(event_target_checked(&event)) /><span>"with icon"</span></label>
                            <label class="docs-choice-row"><input type="checkbox" prop:checked=move || with_actions.get() on:change=move |event| set_with_actions.set(event_target_checked(&event)) /><span>"with actions"</span></label>
                            <label class="docs-choice-row"><input type="checkbox" prop:checked=move || rtl.get() on:change=move |event| set_rtl.set(event_target_checked(&event)) /><span>"RTL locale"</span></label>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        let common = (
                            is_invalid.get(),
                            workbench_tone.get(),
                            is_compact.get(),
                            is_bordered.get(),
                            workbench_motion.get(),
                            workbench_message.get(),
                            workbench_aria_label.get(),
                            workbench_class_name.get(),
                            workbench_lang.get(),
                            workbench_dir.get(),
                        );

                        if with_icon.get() && with_actions.get() {
                            view! {
                                <ErrorView
                                    is_invalid=common.0
                                    tone=common.1
                                    is_compact=common.2
                                    is_bordered=common.3
                                    motion=common.4
                                    message=common.5
                                    aria_label=common.6
                                    class_name=common.7
                                    icon=move || view! { <Icon size=IconSize::Sm tone=IconTone::Danger is_decorative=true>"⚠"</Icon> }
                                    actions=move || view! { <ui::Button variant=ui::ButtonVariant::Secondary>"Retry"</ui::Button> }
                                    lang=common.8
                                    dir=common.9
                                />
                            }.into_any()
                        } else if with_icon.get() {
                            view! {
                                <ErrorView
                                    is_invalid=common.0
                                    tone=common.1
                                    is_compact=common.2
                                    is_bordered=common.3
                                    motion=common.4
                                    message=common.5
                                    aria_label=common.6
                                    class_name=common.7
                                    icon=move || view! { <Icon size=IconSize::Sm tone=IconTone::Danger is_decorative=true>"⚠"</Icon> }
                                    lang=common.8
                                    dir=common.9
                                />
                            }.into_any()
                        } else if with_actions.get() {
                            view! {
                                <ErrorView
                                    is_invalid=common.0
                                    tone=common.1
                                    is_compact=common.2
                                    is_bordered=common.3
                                    motion=common.4
                                    message=common.5
                                    aria_label=common.6
                                    class_name=common.7
                                    actions=move || view! { <ui::Button variant=ui::ButtonVariant::Secondary>"Retry"</ui::Button> }
                                    lang=common.8
                                    dir=common.9
                                />
                            }.into_any()
                        } else {
                            view! {
                                <ErrorView
                                    is_invalid=common.0
                                    tone=common.1
                                    is_compact=common.2
                                    is_bordered=common.3
                                    motion=common.4
                                    message=common.5
                                    aria_label=common.6
                                    class_name=common.7
                                    lang=common.8
                                    dir=common.9
                                />
                            }.into_any()
                        }
                    }}
                </div>
            </Playground>

            <Playground
                title="State Matrix (Tone / Layout / Slots Comparison)"
                code_signal=matrix_code
                code_imports=error_view_imports
                test_source_path="components/error-view/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <ErrorView
                        is_invalid=true
                        tone=ErrorViewTone::Negative
                        is_compact=false
                        is_bordered=false
                        message="Email format invalid".to_string()
                        lang="en-US".to_string()
                        dir=ui_headless::A11yDirection::Ltr
                    />
                    <ErrorView
                        is_invalid=true
                        tone=ErrorViewTone::Neutral
                        is_compact=true
                        is_bordered=true
                        motion=ErrorViewMotion {
                            hidden_translate_px: 12.0,
                            hidden_opacity: 0.0,
                            hidden_scale: 0.95,
                            ..ErrorViewMotion::default()
                        }
                        message="Retry request failed".to_string()
                        aria_label="Validation feedback".to_string()
                        class_name="docs-error-view-custom".to_string()
                        icon=move || view! { <Icon size=IconSize::Sm tone=IconTone::Danger is_decorative=true>"⚠"</Icon> }
                        actions=move || view! { <ui::Button variant=ui::ButtonVariant::Secondary>"Retry"</ui::Button> }
                        lang="ar".to_string()
                        dir=ui_headless::A11yDirection::Rtl
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
