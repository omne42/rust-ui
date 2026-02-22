use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{FieldLabel, FieldLabelTone};
use ui_headless::A11yDirection;

const FIELD_LABEL_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui::{FieldLabel, FieldLabelTone};\nuse ui_headless::A11yDirection;";

pub(super) fn field_label() -> AnyView {
    let (tone_index, set_tone_index) = signal(0usize);
    let (is_required, set_is_required) = signal(true);
    let (is_disabled, set_is_disabled) = signal(false);
    let (has_for_id, set_has_for_id) = signal(true);
    let (custom_indicator, set_custom_indicator) = signal(false);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let workbench_tone = Signal::derive(move || match tone_index.get() {
        1 => FieldLabelTone::Muted,
        2 => FieldLabelTone::Strong,
        _ => FieldLabelTone::Default,
    });

    let workbench_text = Signal::derive(move || {
        if is_required.get() {
            "Owner".to_string()
        } else {
            "Optional owner".to_string()
        }
    });
    let workbench_for_id = Signal::derive(move || {
        if has_for_id.get() {
            "docs-field-label-workbench-input".to_string()
        } else {
            String::new()
        }
    });
    let workbench_required_indicator = Signal::derive(move || {
        if custom_indicator.get() {
            "(required)".to_string()
        } else {
            String::new()
        }
    });
    let workbench_aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Owner field label".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-field-label-custom".to_string()
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
        r#"<FieldLabel
  text=\"Email\".into()
  for_id=\"email\".into()
  is_required=true
/>
<input id=\"email\" type=\"email\" />"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<FieldLabel\n  text={text}\n  for_id={for_id}\n  is_required={}\n  is_disabled={}\n  tone=FieldLabelTone::{:?}\n  required_indicator={required_indicator}\n  aria_label={aria_label}\n  class_name={class_name}\n  lang={lang}\n  dir=ui_headless::A11yDirection::{dir}\n/>",
            bool_word(is_required.get()),
            bool_word(is_disabled.get()),
            workbench_tone.get(),
            text = rust_string_literal(&workbench_text.get()),
            for_id = rust_string_literal(&workbench_for_id.get()),
            required_indicator = rust_string_literal(&workbench_required_indicator.get()),
            aria_label = rust_string_literal(&workbench_aria_label.get()),
            class_name = rust_string_literal(&workbench_class_name.get()),
            lang = rust_string_literal(&workbench_lang.get()),
            dir = if rtl.get() { "Rtl" } else { "Ltr" },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<FieldLabel text=\"Email\".into() for_id=\"email\".into() is_required=true />
<FieldLabel
  text=\"Disabled reviewer\".into()
  is_disabled=true
  tone=FieldLabelTone::Muted
/>
<FieldLabel
  text=\"Critical owner\".into()
  tone=FieldLabelTone::Strong
  is_required=true
  required_indicator=\"(required)\".into()
  aria_label=\"Critical owner field label\".into()
/>"#
        .to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/field-label/src/styles.rs */\\n{}",
            ui::field_form::field_label::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let text = workbench_text.get();
        let for_id = workbench_for_id.get();
        let required_indicator = workbench_required_indicator.get();
        let aria_label = workbench_aria_label.get();
        let class_name = workbench_class_name.get();
        let lang = workbench_lang.get();
        let dir = workbench_dir.get();
        format!(
            "FieldLabelActualConfig {{\\n  text: {:?},\\n  for_id: {:?},\\n  is_required: {},\\n  is_disabled: {},\\n  tone: {:?},\\n  required_indicator: {:?},\\n  aria_label: {:?},\\n  class_name: {:?},\\n  lang: {:?},\\n  dir: {:?},\\n}}",
            text,
            for_id,
            is_required.get(),
            is_disabled.get(),
            workbench_tone.get(),
            required_indicator,
            aria_label,
            class_name,
            lang,
            dir,
        )
    });

    view! {
        <ComponentPage
            title="FieldLabel"
            slug="field-label"
            group="Forms"
            description="Form field label with required/disabled/tone/source-state contracts."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports=FIELD_LABEL_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <FieldLabel
                        text="Email".to_string()
                        for_id="docs-field-label-showcase".to_string()
                        is_required=true
                    />
                    <input
                        id="docs-field-label-showcase"
                        class="docs-search__input"
                        type="email"
                        placeholder="name@example.com"
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports=FIELD_LABEL_DOC_IMPORTS.to_string()
                test_css_source=workbench_test_css_source
                test_source_path="components/field-label/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="field-label-workbench-controls">
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
                            <option value="0">"Default"</option>
                            <option value="1">"Muted"</option>
                            <option value="2">"Strong"</option>
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || is_required.get()
                                on:change=move |event| set_is_required.set(event_target_checked(&event))
                            />
                            <span>"Required"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || is_disabled.get()
                                on:change=move |event| set_is_disabled.set(event_target_checked(&event))
                            />
                            <span>"Disabled"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || has_for_id.get()
                                on:change=move |event| set_has_for_id.set(event_target_checked(&event))
                            />
                            <span>"Bind for/id"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_indicator.get()
                                on:change=move |event| set_custom_indicator.set(event_target_checked(&event))
                            />
                            <span>"Custom required indicator"</span>
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
                                prop:checked=move || rtl.get()
                                on:change=move |event| set_rtl.set(event_target_checked(&event))
                            />
                            <span>"RTL (lang=ar, dir=rtl)"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <FieldLabel
                        text=workbench_text.get()
                        for_id=workbench_for_id.get()
                        is_required=is_required.get()
                        is_disabled=is_disabled.get()
                        tone=workbench_tone.get()
                        required_indicator=workbench_required_indicator.get()
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                        lang=workbench_lang.get()
                        dir=workbench_dir.get()
                    />
                    <input
                        id="docs-field-label-workbench-input"
                        class="docs-search__input"
                        type="text"
                        placeholder="Owner"
                        disabled=is_disabled.get()
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Tone / Disabled / Source Comparison)"
                code_signal=matrix_code
                code_imports=FIELD_LABEL_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <FieldLabel
                        text="Email".to_string()
                        for_id="docs-field-label-matrix-default".to_string()
                        is_required=true
                    />
                    <FieldLabel
                        text="Disabled reviewer".to_string()
                        is_disabled=true
                        tone=FieldLabelTone::Muted
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <FieldLabel
                        text="Critical owner".to_string()
                        tone=FieldLabelTone::Strong
                        is_required=true
                        required_indicator="(required)".to_string()
                        aria_label="Critical owner field label".to_string()
                        class_name="docs-field-label-custom".to_string()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
