use super::*;

pub(crate) fn keyboard() -> AnyView {
    let keyboard_imports = "use leptos::prelude::*;\nuse ui::{Keyboard, KeyboardTone};".to_string();
    let tone_options = vec!["default".to_string(), "muted".to_string()];
    let key_options = vec![
        "⌘K".to_string(),
        "Ctrl+Shift+P".to_string(),
        "⌥⇧P".to_string(),
    ];
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0));
    let (workbench_key_index, set_workbench_key_index) = signal(Some(0));
    let (workbench_is_compact, set_workbench_is_compact) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_tone = Signal::derive(move || match workbench_tone_index.get().unwrap_or(0) {
        1 => KeyboardTone::Muted,
        _ => KeyboardTone::Default,
    });
    let workbench_key_text = Signal::derive(move || match workbench_key_index.get().unwrap_or(0) {
        1 => "Ctrl+Shift+P",
        2 => "⌥⇧P",
        _ => "⌘K",
    });

    let workbench_code = Signal::derive(move || {
        let tone = workbench_tone.get();
        let key_text = workbench_key_text.get();
        let is_compact = workbench_is_compact.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();
        let lang_zh = workbench_lang_zh.get();
        let rtl = workbench_rtl.get();

        let mut snippet = vec!["<Keyboard".to_string()];
        if tone == KeyboardTone::Muted {
            snippet.push("  tone=KeyboardTone::Muted".to_string());
        }
        if is_compact {
            snippet.push("  is_compact=true".to_string());
        }
        if custom_aria {
            snippet.push("  aria_label=\"Open command palette\".into()".to_string());
        }
        if custom_class {
            snippet.push("  class_name=\"docs-keyboard-custom\".into()".to_string());
        }
        if lang_zh {
            snippet.push("  lang=\"zh-CN\".into()".to_string());
        }
        snippet.push(format!(
            "  dir={}",
            if rtl {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            }
        ));
        snippet.push(">".to_string());
        snippet.push(format!("  \"{key_text}\""));
        snippet.push("</Keyboard>".to_string());
        snippet.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let tone = workbench_tone.get();
        let key_text = workbench_key_text.get();
        let is_compact = workbench_is_compact.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();
        let lang = if workbench_lang_zh.get() {
            Some("zh-CN")
        } else {
            None
        };
        let dir = if workbench_rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        let mut class_tokens = vec![
            "ui-keyboard".to_string(),
            match tone {
                KeyboardTone::Muted => "ui-keyboard--tone-muted".to_string(),
                KeyboardTone::Default => "ui-keyboard--tone-default".to_string(),
            },
        ];
        if is_compact {
            class_tokens.push("ui-keyboard--compact".to_string());
        }
        if custom_class {
            class_tokens.push("ui-keyboard--custom-class".to_string());
            class_tokens.push("docs-keyboard-custom".to_string());
        }

        format!(
            "KeyboardActualConfig {{\n  tone: {tone:?},\n  key_text: \"{key_text}\",\n  is_compact: {is_compact},\n  custom_aria_label: {custom_aria},\n  custom_class_name: {custom_class},\n  lang: {:?},\n  dir: {:?},\n  class: \"{}\",\n  marker_expectations: [\"data-tone\", \"data-state\", \"data-compact\", \"data-aria-source\", \"data-class-source\"],\n}}",
            lang,
            dir,
            class_tokens.join(" ")
        )
    });

    let keyboard_test_css_source = Signal::derive(move || {
        format!(
            "/* components/keyboard/src/styles.rs */\n{}",
            ui::keyboard::styles::CSS
        )
    });

    let hello_world_code = Signal::derive(move || r#"<Keyboard>"⌘K"</Keyboard>"#.to_string());

    let state_matrix_code = Signal::derive(move || {
        r#"<Keyboard>"⌘K"</Keyboard>
<Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>
<Keyboard is_compact=true>"Ctrl+K"</Keyboard>
<Keyboard
  tone=KeyboardTone::Muted
  is_compact=true
  aria_label="Open command palette".to_string()
  class_name="docs-keyboard-custom".to_string()
>
  "Ctrl+Shift+P"
</Keyboard>"#
            .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"<Keyboard>"⌘K"</Keyboard>
<Keyboard
  tone=KeyboardTone::Muted
  is_compact=true
  class_name="docs-keyboard-custom".to_string()
>
  "Mapped from upstream app state"
</Keyboard>"#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<Keyboard
  tone=KeyboardTone::Muted
  aria_label="Snapshot contract marker".to_string()
>
  "⌘K"
</Keyboard>"#
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>"#.to_string()
    });

    view! {
        <ComponentPage
            title="Keyboard"
            slug="keyboard"
            group="Display"
            description="Keyboard command primitive (`<kbd>`) with centralized tone/compact/source state contracts."
        >
            <Playground
                title="Hello World (Default Path)"
                code_signal=hello_world_code
                code_imports=keyboard_imports.clone()
            >
                <Keyboard>"⌘K"</Keyboard>
            </Playground>

            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                code_imports=keyboard_imports.clone()
                test_css_source=keyboard_test_css_source
                test_source_path="components/keyboard/src/styles.rs".to_string()
                test_config_signal=workbench_config
                description="可调 tone/key/is_compact/aria/class，并在同一面板查看 code + config + scoped css test。"
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Tone"</div>
                            <SegmentedControl
                                id_base="docs-keyboard-tone".to_string()
                                options=tone_options.clone()
                                selected_index=workbench_tone_index
                                set_selected_index=set_workbench_tone_index
                                size=SegmentedControlSize::Sm
                                aria_label="Keyboard tone".to_string()
                            />

                            <div class="docs-search__label">"Key Text"</div>
                            <SegmentedControl
                                id_base="docs-keyboard-key".to_string()
                                options=key_options.clone()
                                selected_index=workbench_key_index
                                set_selected_index=set_workbench_key_index
                                size=SegmentedControlSize::Sm
                                aria_label="Keyboard key text".to_string()
                            />

                            <Switch checked=workbench_is_compact set_checked=set_workbench_is_compact>
                                "is_compact"
                            </Switch>
                            <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                                "Custom aria_label"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom class_name"
                            </Switch>
                            <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                                "Lang=zh-CN"
                            </Switch>
                            <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                                "dir=rtl"
                            </Switch>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        let tone = workbench_tone.get();
                        let key_text = workbench_key_text.get();
                        let is_compact = workbench_is_compact.get();
                        let aria_label = if workbench_custom_aria.get() {
                            "Open command palette".to_string()
                        } else {
                            "".to_string()
                        };
                        let class_name = if workbench_custom_class.get() {
                            "docs-keyboard-custom".to_string()
                        } else {
                            "".to_string()
                        };
                        let lang = if workbench_lang_zh.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        };
                        let dir = if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        };

                        view! {
                            <Keyboard
                                tone=tone
                                is_compact=is_compact
                                aria_label=aria_label
                                class_name=class_name
                                lang=lang
                                dir=dir
                            >
                                {key_text}
                            </Keyboard>
                        }
                    }}

                    <div class="docs-row">
                        <span class="ui-muted">"Compare baseline:"</span>
                        <Keyboard>"⌘K"</Keyboard>
                        <Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>
                    </div>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Tone / Compact / Source Markers)"
                code_signal=state_matrix_code
                code_imports=keyboard_imports.clone()
            >
                <div class="docs-row">
                    <div class="docs-card" style="flex: 1 1 180px;">
                        <span class="ui-muted">"Default"</span>
                        <Keyboard>"⌘K"</Keyboard>
                    </div>
                    <div class="docs-card" style="flex: 1 1 180px;">
                        <span class="ui-muted">"Muted"</span>
                        <Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>
                    </div>
                    <div class="docs-card" style="flex: 1 1 180px;">
                        <span class="ui-muted">"Compact"</span>
                        <Keyboard is_compact=true>"Ctrl+K"</Keyboard>
                    </div>
                    <div class="docs-card" style="flex: 1 1 180px;">
                        <span class="ui-muted">"Muted + Compact + Custom"</span>
                        <Keyboard
                            tone=KeyboardTone::Muted
                            is_compact=true
                            aria_label="Open command palette".to_string()
                            class_name="docs-keyboard-custom".to_string()
                        >
                            "Ctrl+Shift+P"
                        </Keyboard>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled Contrast (N/A for Keyboard)"
                description="Keyboard has no controllable state axis; compare default rendering with upstream state mapped into plain props."
                code_signal=controlled_contrast_code
                code_imports=keyboard_imports.clone()
            >
                <div class="docs-row">
                    <Keyboard>"⌘K"</Keyboard>
                    <Keyboard
                        tone=KeyboardTone::Muted
                        is_compact=true
                        class_name="docs-keyboard-custom".to_string()
                    >
                        "Mapped from upstream app state"
                    </Keyboard>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="Keyboard is a display leaf: streaming is optional and falls back to snapshot (`data-ui-streaming=optional`, `data-ui-streaming-fallback=snapshot`)."
                code_signal=stream_snapshot_code
                code_imports=keyboard_imports.clone()
            >
                <Keyboard
                    tone=KeyboardTone::Muted
                    aria_label="Snapshot contract marker".to_string()
                >
                    "⌘K"
                </Keyboard>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=keyboard_imports.clone()
            >
                <Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>
            </Playground>

            <Playground
                title="State Matrix (Tone / Compact / Locale Comparison)"
                code_signal=state_matrix_code
                code_imports=keyboard_imports.clone()
            >
                <div class="docs-row">
                    <Keyboard lang="en-US".to_string() dir=A11yDirection::Ltr>"⌘K"</Keyboard>
                    <Keyboard tone=KeyboardTone::Muted is_compact=true lang="zh-CN".to_string() dir=A11yDirection::Rtl>
                        "⌥⇧P"
                    </Keyboard>
                    <Keyboard
                        tone=KeyboardTone::Muted
                        is_compact=true
                        aria_label="Open command palette".to_string()
                        class_name="docs-keyboard-custom".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        "Ctrl+Shift+P"
                    </Keyboard>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="keyboard-parameter-matrix">
                <h3>"Parameter Matrix (API + Defaults)"</h3>
                <ul data-slot="keyboard-parameter-rows">
                    <li><code>"tone"</code>" = KeyboardTone::Default (default)"</li>
                    <li><code>"is_compact"</code>" = false (default)"</li>
                    <li><code>"aria_label"</code>" = \"Keyboard\" fallback after trim/normalize"</li>
                    <li><code>"class_name"</code>" = optional custom class (default none)"</li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
