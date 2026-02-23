use super::*;

pub(crate) fn label() -> AnyView {
    let label_imports =
        "use leptos::prelude::*;\nuse ui::{Label, LabelEmphasis};\nuse ui_headless::A11yDirection;"
            .to_string();

    let (emphasis_index, set_emphasis_index) = signal(0usize);
    let (is_required, set_is_required) = signal(true);
    let (is_disabled, set_is_disabled) = signal(false);
    let (has_for_id, set_has_for_id) = signal(true);
    let (custom_indicator, set_custom_indicator) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let workbench_emphasis = Signal::derive(move || match emphasis_index.get() {
        1 => LabelEmphasis::Subtle,
        2 => LabelEmphasis::Strong,
        _ => LabelEmphasis::Default,
    });
    let workbench_text = Signal::derive(move || {
        if is_required.get() {
            "Assignee".to_string()
        } else {
            "Optional assignee".to_string()
        }
    });
    let workbench_for_id = Signal::derive(move || {
        if has_for_id.get() {
            "docs-label-workbench-input".to_string()
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
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-label-custom".to_string()
        } else {
            String::new()
        }
    });
    let workbench_motion = Signal::derive(move || {
        if custom_motion.get() {
            ui::label::LabelMotion {
                color_transition_ms: 420,
                weight_transition_ms: 420,
            }
        } else {
            ui::label::LabelMotion::default()
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
        r#"<Label
  text=\"Email\".into()
  for_id=\"email\".into()
  is_required=true
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Label\n  text={text}\n  for_id={for_id}\n  is_required={}\n  is_disabled={}\n  emphasis=LabelEmphasis::{:?}\n  required_indicator={required_indicator}\n  class_name={class_name}\n  motion=ui::label::LabelMotion {{ color_transition_ms: {}, weight_transition_ms: {} }}\n  lang={lang}\n  dir=ui_headless::A11yDirection::{}\n/>",
            bool_word(is_required.get()),
            bool_word(is_disabled.get()),
            workbench_emphasis.get(),
            workbench_motion.get().color_transition_ms,
            workbench_motion.get().weight_transition_ms,
            if rtl.get() { "Rtl" } else { "Ltr" },
            text = rust_string_literal(&workbench_text.get()),
            for_id = rust_string_literal(&workbench_for_id.get()),
            required_indicator = rust_string_literal(&workbench_required_indicator.get()),
            class_name = rust_string_literal(&workbench_class_name.get()),
            lang = rust_string_literal(&workbench_lang.get()),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Label text=\"Name\".into() for_id=\"name\".into() is_required=true />
<Label
  text=\"Helper\".into()
  emphasis=LabelEmphasis::Subtle
  is_disabled=true
/>
<Label
  text=\"Critical owner\".into()
  emphasis=LabelEmphasis::Strong
  required_indicator=\"(required)\".into()
  class_name=\"docs-label-custom\".into()
  motion=ui::label::LabelMotion { color_transition_ms: 420, weight_transition_ms: 420 }
  lang=\"ar\".into()
  dir=A11yDirection::Rtl
/>"#
        .to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/label/src/styles.rs */\n{}",
            ui::label::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "LabelActualConfig {{\n  text: {:?},\n  for_id: {:?},\n  is_required: {},\n  is_disabled: {},\n  emphasis: {:?},\n  required_indicator: {:?},\n  class_name: {:?},\n  motion: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            workbench_text.get(),
            workbench_for_id.get(),
            is_required.get(),
            is_disabled.get(),
            workbench_emphasis.get(),
            workbench_required_indicator.get(),
            workbench_class_name.get(),
            workbench_motion.get(),
            workbench_lang.get(),
            workbench_dir.get(),
        )
    });

    view! {
        <ComponentPage
            title="Label"
            slug="label"
            group="Forms"
            description="Form label primitive with full API workbench and multi-state matrix."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports=label_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <Label
                        text="Email".to_string()
                        for_id="docs-label-showcase".to_string()
                        is_required=true
                    />
                    <input
                        id="docs-label-showcase"
                        class="docs-search__input"
                        type="email"
                        placeholder="name@example.com"
                    />
                </div>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports=label_imports.clone()
                test_css_source=workbench_test_css_source
                test_source_path="components/label/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="label-workbench-controls">
                        <div class="docs-search__label">"Emphasis"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || emphasis_index.get().to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_emphasis_index.set(value.min(2));
                                }
                            }
                        >
                            <option value="0">"Default"</option>
                            <option value="1">"Subtle"</option>
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
                <div class="docs-stack docs-stack--tight">
                    <Label
                        text=workbench_text.get()
                        for_id=workbench_for_id.get()
                        is_required=is_required.get()
                        is_disabled=is_disabled.get()
                        emphasis=workbench_emphasis.get()
                        required_indicator=workbench_required_indicator.get()
                        class_name=workbench_class_name.get()
                        motion=workbench_motion.get()
                        lang=workbench_lang.get()
                        dir=workbench_dir.get()
                    />
                    <input
                        id="docs-label-workbench-input"
                        class="docs-search__input"
                        type="text"
                        placeholder="Owner"
                        disabled=is_disabled.get()
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Emphasis / Disabled / Locale Comparison)"
                code_signal=matrix_code
                code_imports=label_imports
            >
                <div class="docs-stack docs-stack--tight">
                    <Label
                        text="Name".to_string()
                        for_id="docs-label-matrix-name".to_string()
                        is_required=true
                    />
                    <Label
                        text="Helper".to_string()
                        emphasis=LabelEmphasis::Subtle
                        is_disabled=true
                    />
                    <Label
                        text="Critical owner".to_string()
                        emphasis=LabelEmphasis::Strong
                        required_indicator="(required)".to_string()
                        class_name="docs-label-custom".to_string()
                        motion=ui::label::LabelMotion {
                            color_transition_ms: 420,
                            weight_transition_ms: 420,
                        }
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
