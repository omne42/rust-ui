use super::*;

pub(crate) fn description() -> AnyView {
    let description_imports =
        "use leptos::prelude::*;\nuse ui::{Description, DescriptionElement, DescriptionTone};"
            .to_string();

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
    let (is_truncated, set_is_truncated) = signal(false);
    let (custom_aria_label, set_custom_aria_label) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (lang_zh, set_lang_zh) = signal(false);
    let (rtl_dir, set_rtl_dir) = signal(false);

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
            lines.push("  is_disabled=true".to_string());
        }
        if is_truncated.get() {
            lines.push("  is_truncated=true".to_string());
        }
        if custom_aria_label.get() {
            lines.push("  aria_label=\"Description helper text\".into()".to_string());
        }
        if custom_class.get() {
            lines.push("  class_name=\"docs-description-custom\".into()".to_string());
        }
        if lang_zh.get() {
            lines.push("  lang=\"zh-CN\".into()".to_string());
        }
        if rtl_dir.get() {
            lines.push("  dir=A11yDirection::Rtl".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let test_css_source = Signal::derive(move || {
        format!(
            "/* components/description/src/styles.rs */\n{}",
            ui::description::styles::CSS
        )
    });

    let actual_config = Signal::derive(move || {
        let tone = selected_tone.get();
        let element = selected_element.get();
        let mut classes = vec!["ui-description".to_string(), tone.class_name().into()];
        if is_disabled.get() {
            classes.push("ui-description--disabled".to_string());
        }
        if is_truncated.get() {
            classes.push("ui-description--truncate".to_string());
        }
        if custom_class.get() {
            classes.push("ui-description--custom-class".to_string());
            classes.push("docs-description-custom".to_string());
        }

        format!(
            "DescriptionActualConfig {{\n  text: \"Helper text for this field.\",\n  tone: {tone:?},\n  element: {element:?},\n  is_disabled: {},\n  is_truncated: {},\n  lang: {},\n  dir: {},\n  has_custom_aria_label: {},\n  has_custom_class_name: {},\n  class: \"{}\",\n}}",
            is_disabled.get(),
            is_truncated.get(),
            if lang_zh.get() {
                "Some(\"zh-CN\")"
            } else {
                "Some(\"en-US\")"
            },
            if rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            custom_aria_label.get(),
            custom_class.get(),
            classes.join(" ")
        )
    });

    let hello_world_code = Signal::derive(move || {
        r#"<Description text="This appears below the field.".to_string() />"#.to_string()
    });

    let state_matrix_code = Signal::derive(move || {
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
/>
<Description
  text="Read-only helper still keeps the latest snapshot.".to_string()
  tone=DescriptionTone::Muted
  is_disabled=true
/>"#
        .to_string()
    });

    let controlled_uncontrolled_code = Signal::derive(move || {
        r#"// Description has no internal controllable state axis.
// Uncontrolled-style: pass final helper snapshot directly.
<Description
  text="Uncontrolled snapshot: email must include @".to_string()
/>

// Controlled-style (by parent form store): parent updates props and Description re-renders.
<Description
  text="Controlled snapshot: email format is invalid".to_string()
  tone=DescriptionTone::Negative
/>"#
        .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"// Snapshot: render validated helper output in one shot.
<Description
  text="Snapshot: email is required".to_string()
/>

// Streaming Optional: fallback stays snapshot until final helper output is ready.
<Description
  text="Streaming fallback=snapshot: waiting for final validation".to_string()
  tone=DescriptionTone::Muted
/>"#
        .to_string()
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
  is_truncated=true
  class_name="docs-description-custom".to_string()
/>
<Description
  text="Disabled helper text".to_string()
  is_disabled=true
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
                title="Hello World"
                code_signal=hello_world_code
                code_imports=description_imports.clone()
            >
                <div class="docs-stack">
                    <Description text="This appears below the field.".to_string() />
                </div>
            </Playground>

            <Playground
                title="Workbench"
                description="Interactive display/config/code/css-test playground for Description state contracts."
                code_signal=workbench_code
                code_imports=description_imports.clone()
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
                        <Switch checked=is_truncated set_checked=set_is_truncated>
                            "Truncate"
                        </Switch>
                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=lang_zh set_checked=set_lang_zh>
                            "lang=zh-CN"
                        </Switch>
                        <Switch checked=rtl_dir set_checked=set_rtl_dir>
                            "dir=rtl"
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
                                is_disabled=is_disabled.get()
                                is_truncated=is_truncated.get()
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
                                lang=if lang_zh.get() {
                                    "zh-CN".to_string()
                                } else {
                                    "en-US".to_string()
                                }
                                dir=if rtl_dir.get() {
                                    A11yDirection::Rtl
                                } else {
                                    A11yDirection::Ltr
                                }
                            />
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Tone / Disabled / Truncate)"
                code_signal=state_matrix_code
                code_imports=description_imports.clone()
            >
                <div class="docs-stack docs-description-limit">
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
                    <Description
                        text="Read-only helper still keeps the latest snapshot.".to_string()
                        tone=DescriptionTone::Muted
                        is_disabled=true
                    />
                    <Description
                        text="A very long assistant text that should truncate in constrained layouts to avoid breaking form rhythm.".to_string()
                        element=DescriptionElement::Span
                        is_truncated=true
                        class_name="docs-description-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (Stateless Contract)"
                code_signal=controlled_uncontrolled_code
                code_imports=description_imports.clone()
            >
                <div class="docs-stack">
                    <Description
                        text="Uncontrolled snapshot: email must include @".to_string()
                    />
                    <Description
                        text="Controlled snapshot: email format is invalid".to_string()
                        tone=DescriptionTone::Negative
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional (fallback=snapshot)"
                description="Description is not a正文阅读面; docs expose snapshot mode + fallback=snapshot for copy/paste verification."
                code_signal=stream_snapshot_code
                code_imports=description_imports.clone()
            >
                <div class="docs-stack docs-description-limit">
                    <Description text="Snapshot: email is required".to_string() />
                    <Description
                        text="Streaming fallback=snapshot: waiting for final validation".to_string()
                        tone=DescriptionTone::Muted
                    />
                    <p class="ui-muted">
                        "Inspect data-ui-stream-support/data-ui-stream-fallback/data-ui-output-status."
                    </p>
                </div>
            </Playground>

            <Playground
                title="Tone Variants"
                code_signal=tone_code
                code_imports=description_imports.clone()
            >
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

            <Playground
                title="Truncate + Element + Disabled"
                code_signal=truncate_code
                code_imports=description_imports.clone()
            >
                <div class="docs-stack docs-description-limit">
                    <Description
                        text="A very long assistant text that should truncate in constrained layouts to avoid breaking form rhythm.".to_string()
                        element=DescriptionElement::Span
                        is_truncated=true
                        class_name="docs-description-custom".to_string()
                    />
                    <Description
                        text="Disabled helper text".to_string()
                        is_disabled=true
                        tone=DescriptionTone::Muted
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="description-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Use any Description Playground's "
                    <code>"Show code"</code>
                    " + copy button. Snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::{Description, DescriptionElement, DescriptionTone};\n\n<Description\n  text=\"This appears below the field.\".to_string()\n/>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-description-source-copy".to_string()
                />
                <ul data-slot="description-source-paths">
                    <li><code>"components/description/src/mod.rs"</code></li>
                    <li><code>"components/description/src/logic.rs"</code></li>
                    <li><code>"components/description/src/view.rs"</code></li>
                    <li><code>"components/description/src/styles.rs"</code></li>
                </ul>
                <ul data-slot="description-source-prerequisites">
                    <li><code>"component-description"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
