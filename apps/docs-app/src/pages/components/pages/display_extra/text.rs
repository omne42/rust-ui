use super::*;

pub(crate) fn text() -> AnyView {
    let text_imports =
        "use leptos::prelude::*;\nuse ui::{Text, TextAlign, TextElement, TextTone, TextWeight};"
            .to_string();

    let (tone_index, set_tone_index) = signal(0usize);
    let (align_index, set_align_index) = signal(0usize);
    let (weight_index, set_weight_index) = signal(1usize);
    let (element_index, set_element_index) = signal(1usize);
    let (disabled, set_disabled) = signal(false);
    let (truncate, set_truncate) = signal(false);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_slot, set_custom_slot) = signal(false);

    let workbench_tone = Signal::derive(move || match tone_index.get() {
        1 => TextTone::Subtle,
        2 => TextTone::Strong,
        _ => TextTone::Default,
    });
    let workbench_align = Signal::derive(move || match align_index.get() {
        1 => TextAlign::Center,
        2 => TextAlign::End,
        _ => TextAlign::Start,
    });
    let workbench_weight = Signal::derive(move || match weight_index.get() {
        0 => TextWeight::Regular,
        2 => TextWeight::Bold,
        _ => TextWeight::Semibold,
    });
    let workbench_element = Signal::derive(move || match element_index.get() {
        0 => TextElement::Span,
        2 => TextElement::Div,
        _ => TextElement::Paragraph,
    });
    let workbench_aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Release summary text".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-text-custom".to_string()
        } else {
            String::new()
        }
    });
    let workbench_slot = Signal::derive(move || {
        if custom_slot.get() {
            "body".to_string()
        } else {
            String::new()
        }
    });

    let showcase_code =
        Signal::derive(move || r#"<Text text=\"Primary body copy\".into() />"#.to_string());

    let workbench_code = Signal::derive(move || {
        format!(
            "<Text\n  text=\"{}\".into()\n  tone=TextTone::{:?}\n  align=TextAlign::{:?}\n  weight=TextWeight::{:?}\n  disabled={}\n  truncate={}\n  element=TextElement::{:?}\n  aria_label={}\n  class_name={}\n  slot={}\n/>",
            if disabled.get() {
                "Read-only release summary"
            } else {
                "Release summary"
            },
            workbench_tone.get(),
            workbench_align.get(),
            workbench_weight.get(),
            if disabled.get() { "true" } else { "false" },
            if truncate.get() { "true" } else { "false" },
            workbench_element.get(),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
            rust_string_literal(&workbench_slot.get()),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Text text=\"Primary body copy\".into() />
<Text
  text=\"Centered metadata\".into()
  tone=TextTone::Subtle
  align=TextAlign::Center
  weight=TextWeight::Semibold
  element=TextElement::Paragraph
/>
<Text
  text=\"Critical long status message that truncates in compact cards\".into()
  tone=TextTone::Strong
  align=TextAlign::End
  weight=TextWeight::Bold
  disabled=true
  truncate=true
  element=TextElement::Div
  aria_label=\"Release status\".into()
  class_name=\"docs-text-custom\".into()
  slot=\"body\".into()
/>"#
        .to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/text/src/styles.rs */\\n{}",
            ui::text::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "TextActualConfig {{\\n  text: {:?},\\n  tone: {:?},\\n  align: {:?},\\n  weight: {:?},\\n  disabled: {},\\n  truncate: {},\\n  element: {:?},\\n  aria_label: {:?},\\n  class_name: {:?},\\n  slot: {:?},\\n}}",
            if disabled.get() {
                "Read-only release summary"
            } else {
                "Release summary"
            },
            workbench_tone.get(),
            workbench_align.get(),
            workbench_weight.get(),
            disabled.get(),
            truncate.get(),
            workbench_element.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
            workbench_slot.get(),
        )
    });

    view! {
        <ComponentPage
            title="Text"
            slug="text"
            group="Display"
            description="Typography primitive with full API workbench and state matrix."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports=text_imports.clone()
            >
                <div class="ui-text">"Primary body copy"</div>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports=text_imports.clone()
                test_css_source=workbench_test_css_source
                test_source_path="components/text/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="text-workbench-controls">
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
                            <option value="1">"Subtle"</option>
                            <option value="2">"Strong"</option>
                        </select>

                        <div class="docs-search__label">"Align"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || align_index.get().to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_align_index.set(value.min(2));
                                }
                            }
                        >
                            <option value="0">"Start"</option>
                            <option value="1">"Center"</option>
                            <option value="2">"End"</option>
                        </select>

                        <div class="docs-search__label">"Weight"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || weight_index.get().to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_weight_index.set(value.min(2));
                                }
                            }
                        >
                            <option value="0">"Regular"</option>
                            <option value="1">"Semibold"</option>
                            <option value="2">"Bold"</option>
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
                                prop:checked=move || disabled.get()
                                on:change=move |event| set_disabled.set(event_target_checked(&event))
                            />
                            <span>"Disabled"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || truncate.get()
                                on:change=move |event| set_truncate.set(event_target_checked(&event))
                            />
                            <span>"Truncate"</span>
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
                                prop:checked=move || custom_slot.get()
                                on:change=move |event| set_custom_slot.set(event_target_checked(&event))
                            />
                            <span>"Named slot"</span>
                        </label>
                    </div>
                }
            >
                <div
                    class=move || {
                        if workbench_class_name.get().is_empty() {
                            "ui-text".to_string()
                        } else {
                            format!("ui-text {}", workbench_class_name.get())
                        }
                    }
                    aria-label=move || workbench_aria_label.get()
                    slot=move || workbench_slot.get()
                    data-text-tone=move || format!("{:?}", workbench_tone.get())
                    data-text-align=move || format!("{:?}", workbench_align.get())
                    data-text-weight=move || format!("{:?}", workbench_weight.get())
                    data-text-element=move || format!("{:?}", workbench_element.get())
                    data-text-disabled=move || bool_word(disabled.get())
                    data-text-truncate=move || bool_word(truncate.get())
                >
                    {move || {
                        if disabled.get() {
                            "Read-only release summary".to_string()
                        } else {
                            "Release summary".to_string()
                        }
                    }}
                </div>
            </Playground>

            <Playground
                title="State Matrix (Tone / Align / Truncate Comparison)"
                code_signal=matrix_code
                code_imports=text_imports
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="ui-text">"Primary body copy"</div>
                    <p class="ui-text" data-text-tone="Subtle" data-text-align="Center">
                        "Centered metadata"
                    </p>
                    <div
                        class="ui-text docs-text-custom"
                        data-text-tone="Strong"
                        data-text-align="End"
                        data-text-disabled="true"
                        data-text-truncate="true"
                        aria-label="Release status"
                        slot="body"
                    >
                        "Critical long status message that truncates in compact cards"
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
