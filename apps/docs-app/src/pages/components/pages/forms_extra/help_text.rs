use super::*;

pub(crate) fn help_text() -> AnyView {
    let help_text_imports =
        "use leptos::prelude::*;\nuse ui::{HelpText, HelpTextTone};".to_string();

    let hello_world_code = Signal::derive(move || {
        r#"<HelpText
  description="Use at least 12 characters.".to_string()
/>"#
        .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<HelpText
  description="Use at least 12 characters.".to_string()
  aria_label="Password hint".to_string()
/>
<HelpText
  tone=HelpTextTone::Neutral
  description="This value is visible to project admins only.".to_string()
/>
<HelpText
  is_invalid=true
  is_error_icon_visible=true
  error_message="Password does not meet complexity requirements.".to_string()
  class_name="docs-help-text-custom".to_string()
/>
<HelpText
  is_invalid=true
  tone=HelpTextTone::Negative
  error_message="Two-factor token expired. Request a new code.".to_string()
  is_disabled=true
/>"#
        .to_string()
    });

    let controlled_uncontrolled_code = Signal::derive(move || {
        r#"// HelpText has no internal controllable state axis.
// Uncontrolled-style: pass final snapshot props directly.
<HelpText
  description="Uncontrolled snapshot: email must include @".to_string()
/>

// Controlled-style (parent store): parent updates props and HelpText re-renders.
<HelpText
  is_invalid=true
  error_message="Controlled snapshot: email format is invalid".to_string()
/>"#
        .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"// Snapshot: render validated final output in one shot.
<HelpText
  is_invalid=true
  error_message="Snapshot: email is required".to_string()
/>

// Streaming Optional: fallback stays snapshot until final output is ready.
<HelpText
  tone=HelpTextTone::Neutral
  description="Streaming fallback=snapshot: waiting for final validation".to_string()
/>"#
        .to_string()
    });

    let description_code = Signal::derive(move || {
        r#"<HelpText
  description="Use at least 12 characters.".to_string()
/>"#
        .to_string()
    });

    let error_code = Signal::derive(move || {
        r#"<HelpText
  is_invalid=true
  is_error_icon_visible=true
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
    let (is_error_icon_visible, set_is_error_icon_visible) = signal(true);
    let (use_error_message, set_use_error_message) = signal(true);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (show_compare, set_show_compare) = signal(true);
    let (custom_motion, set_custom_motion) = signal(false);
    let (lang_zh, set_lang_zh) = signal(false);
    let (rtl_dir, set_rtl_dir) = signal(false);

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
        let is_invalid = is_invalid.get();
        let is_disabled = is_disabled.get();
        let is_error_icon_visible = is_error_icon_visible.get();
        let description = active_description.get();
        let error_message = active_error_message.get();
        let aria = active_aria_label.get();
        let class_name = active_class_name.get();

        let mut lines = vec![
            "<HelpText".to_string(),
            format!("  tone=HelpTextTone::{tone:?}"),
            format!("  is_invalid={is_invalid}"),
            format!("  is_disabled={is_disabled}"),
            format!("  is_error_icon_visible={is_error_icon_visible}"),
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
        if custom_motion.get() {
            lines.push("  motion=HelpTextMotion::disabled()".to_string());
        }
        lines.push(format!(
            "  lang={:?}.into()",
            if lang_zh.get() { "zh-CN" } else { "en-US" }
        ));
        if rtl_dir.get() {
            lines.push("  dir=A11yDirection::Rtl".to_string());
        } else {
            lines.push("  dir=A11yDirection::Ltr".to_string());
        }
        lines.push("/>".to_string());

        lines.join("\n")
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* components/help-text/src/styles.rs */\n{}",
            ui::field_form::help_text::styles::CSS
        )
    });
    let actual_config = Signal::derive(move || {
        let tone = active_tone.get();
        let is_invalid = is_invalid.get();
        let is_disabled = is_disabled.get();
        let is_error_icon_visible = is_error_icon_visible.get();
        let has_description = active_description.get().is_some();
        let has_error = active_error_message.get().is_some();
        let has_custom_aria = custom_aria.get();
        let has_custom_class = custom_class.get();
        format!(
            "HelpTextActualConfig {{\n  tone: HelpTextTone::{tone:?},\n  is_invalid: {is_invalid},\n  is_disabled: {is_disabled},\n  is_error_icon_visible: {is_error_icon_visible},\n  motion: {},\n  lang: {},\n  dir: {},\n  has_description: {has_description},\n  has_error_message: {has_error},\n  has_custom_aria_label: {has_custom_aria},\n  has_custom_class_name: {has_custom_class},\n}}",
            if custom_motion.get() {
                "HelpTextMotion::disabled()"
            } else {
                "HelpTextMotion::default()"
            },
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
        )
    });

    view! {
        <ComponentPage
            title="HelpText"
            slug="help-text"
            group="Forms"
            description="baseline-style form assistance primitive that resolves description vs error message and tone/icon state through centralized logic contracts."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=hello_world_code
                code_imports=help_text_imports.clone()
            >
                <div class="docs-stack">
                    <HelpText description="Use at least 12 characters.".to_string() />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                code_signal=interactive_code
                code_imports=help_text_imports.clone()
                test_css_source=test_css_source
                test_source_path="components/help-text/src/styles.rs".to_string()
                test_config_signal=actual_config
                description="展示区 + Config 区 + Code 区 + CSS Test 区；支持 description/error/is_invalid/is_disabled/tone 的多场景对比。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="help-text-workbench-controls">
                        <div class="docs-search__label">"配置区 · Tone"</div>
                        <div data-slot="help-text-tone-control">
                            <ui::SegmentedControl
                                id_base="docs-help-text-tone".to_string()
                                options=tone_options.clone()
                                selected_index=tone_index
                                set_selected_index=set_tone_index
                                size=ui::SegmentedControlSize::Sm
                                aria_label="HelpText tone".to_string()
                            />
                        </div>
                        <div data-slot="help-text-toggle-invalid">
                            <ui::Switch checked=is_invalid set_checked=set_is_invalid>
                                "Invalid"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-disabled">
                            <ui::Switch checked=is_disabled set_checked=set_is_disabled>
                                "Disabled"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-show-error-icon">
                            <ui::Switch checked=is_error_icon_visible set_checked=set_is_error_icon_visible>
                                "Show error icon"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-use-error-message">
                            <ui::Switch checked=use_error_message set_checked=set_use_error_message>
                                "Use error message"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-custom-aria">
                            <ui::Switch checked=custom_aria set_checked=set_custom_aria>
                                "Custom aria label"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-custom-class">
                            <ui::Switch checked=custom_class set_checked=set_custom_class>
                                "Custom class"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-custom-motion">
                            <ui::Switch checked=custom_motion set_checked=set_custom_motion>
                                "Motion disabled"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-lang-zh">
                            <ui::Switch checked=lang_zh set_checked=set_lang_zh>
                                "lang=zh-CN"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-dir-rtl">
                            <ui::Switch checked=rtl_dir set_checked=set_rtl_dir>
                                "dir=rtl"
                            </ui::Switch>
                        </div>
                        <div data-slot="help-text-toggle-show-compare">
                            <ui::Switch checked=show_compare set_checked=set_show_compare>
                                "Show compare matrix"
                            </ui::Switch>
                        </div>
                    </div>
                }
            >
                {move || {
                    let tone = active_tone.get();
                    let is_invalid = is_invalid.get();
                    let is_disabled = is_disabled.get();
                    let is_error_icon_visible = is_error_icon_visible.get();
                    let description = active_description.get().unwrap_or_default();
                    let error_message = active_error_message.get().unwrap_or_default();
                    let aria_label = active_aria_label.get().unwrap_or_default();
                    let class_name = active_class_name.get().unwrap_or_default();
                    let compare = show_compare.get();

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="help-text-workbench-canvas">
                            <div class="docs-search__label">"展示区 · Primary"</div>
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="help-text-primary-card">
                                <HelpText
                                    tone=tone
                                    is_invalid=is_invalid
                                    is_disabled=is_disabled
                                    is_error_icon_visible=is_error_icon_visible
                                    description=description
                                    error_message=error_message
                                    aria_label=aria_label
                                    class_name=class_name
                                    motion=if custom_motion.get() {
                                        ui::field_form::help_text::HelpTextMotion::disabled()
                                    } else {
                                        ui::field_form::help_text::HelpTextMotion::default()
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

                            <Show when=move || compare>
                                <div class="docs-search__label">"展示区 · 对比矩阵"</div>
                                <div class="docs-stack docs-stack--tight">
                                    <HelpText
                                        tone=HelpTextTone::Neutral
                                        description="Neutral description state.".to_string()
                                    />
                                    <HelpText
                                        tone=HelpTextTone::Negative
                                        is_invalid=true
                                        is_error_icon_visible=true
                                        error_message="Negative error state.".to_string()
                                    />
                                    <HelpText
                                        is_invalid=true
                                        is_disabled=true
                                        error_message="Disabled + invalid state.".to_string()
                                    />
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Description / Error / Disabled)"
                code_signal=state_matrix_code
                code_imports=help_text_imports.clone()
            >
                <div class="docs-stack">
                    <HelpText
                        description="Use at least 12 characters.".to_string()
                        aria_label="Password hint".to_string()
                    />
                    <HelpText
                        tone=HelpTextTone::Neutral
                        description="This value is visible to project admins only.".to_string()
                    />
                    <HelpText
                        is_invalid=true
                        is_error_icon_visible=true
                        error_message="Password does not meet complexity requirements.".to_string()
                        class_name="docs-help-text-custom".to_string()
                    />
                    <HelpText
                        is_invalid=true
                        tone=HelpTextTone::Negative
                        error_message="Two-factor token expired. Request a new code.".to_string()
                        is_disabled=true
                    />
                </div>
            </Playground>

            <Playground
                title="Description / Error / Disabled Gallery"
                code_signal=state_matrix_code
                code_imports=help_text_imports.clone()
            >
                <div class="docs-stack">
                    <HelpText
                        description="Use at least 12 characters.".to_string()
                        aria_label="Password hint".to_string()
                    />
                    <HelpText
                        tone=HelpTextTone::Neutral
                        description="This value is visible to project admins only.".to_string()
                    />
                    <HelpText
                        is_invalid=true
                        is_error_icon_visible=true
                        error_message="Password does not meet complexity requirements.".to_string()
                        class_name="docs-help-text-custom".to_string()
                    />
                    <HelpText
                        is_invalid=true
                        tone=HelpTextTone::Negative
                        error_message="Two-factor token expired. Request a new code.".to_string()
                        is_disabled=true
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (Stateless Contract)"
                code_signal=controlled_uncontrolled_code
                code_imports=help_text_imports.clone()
            >
                <div class="docs-stack">
                    <HelpText
                        description="Uncontrolled snapshot: email must include @".to_string()
                    />
                    <HelpText
                        is_invalid=true
                        error_message="Controlled snapshot: email format is invalid".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional (fallback=snapshot)"
                description="HelpText is not a正文阅读面; docs expose snapshot mode + fallback=snapshot for copy/paste verification."
                code_signal=stream_snapshot_code
                code_imports=help_text_imports.clone()
            >
                <div class="docs-stack">
                    <HelpText
                        is_invalid=true
                        error_message="Snapshot: email is required".to_string()
                    />
                    <HelpText
                        tone=HelpTextTone::Neutral
                        description="Streaming fallback=snapshot: waiting for final validation"
                            .to_string()
                    />
                    <p class="ui-muted">
                        "Inspect data-ui-stream-support/data-ui-stream-mode/data-ui-stream-fallback."
                    </p>
                </div>
            </Playground>

            <Playground
                title="Description (Neutral)"
                code_signal=description_code
                code_imports=help_text_imports.clone()
            >
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

            <Playground
                title="Invalid + Error Icon"
                code_signal=error_code
                code_imports=help_text_imports.clone()
            >
                <div class="docs-stack">
                    <HelpText
                        is_invalid=true
                        is_error_icon_visible=true
                        error_message="Password does not meet complexity requirements.".to_string()
                        class_name="docs-help-text-custom".to_string()
                    />
                    <HelpText
                        is_invalid=true
                        tone=HelpTextTone::Negative
                        error_message="Two-factor token expired. Request a new code.".to_string()
                        is_disabled=true
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="help-text-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground supports "
                    <code>"Show code"</code>
                    " + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::{HelpText, HelpTextTone};\n\n<HelpText\n  description=\"Use at least 12 characters.\".to_string()\n/>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-help-text-source-copy".to_string()
                />
                <ul data-slot="help-text-source-paths">
                    <li><code>"components/help-text/src/mod.rs"</code></li>
                    <li><code>"components/help-text/src/logic.rs"</code></li>
                    <li><code>"components/help-text/src/view.rs"</code></li>
                    <li><code>"components/help-text/src/styles.rs"</code></li>
                    <li><code>"components/help-text/src/motion.rs"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
