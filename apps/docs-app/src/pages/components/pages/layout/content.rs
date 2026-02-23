use super::*;

pub(crate) fn content() -> AnyView {
    let semantic_code = Signal::derive(move || {
        r#"<Content>
  <p>"Primary body content"</p>
</Content>
<Content tone=ContentTone::Muted>
  <p>"Secondary muted content"</p>
</Content>"#
            .to_string()
    });

    let padded_code = Signal::derive(move || {
        r#"<View border=ViewBorder::Subtle radius=ViewRadius::Md>
  <Content
    padded=true
    aria_label="Dialog content".to_string()
    class_name="docs-content-custom".to_string()
  >
    <p>"Padded container content"</p>
  </Content>
</View>"#
            .to_string()
    });
    let (interactive_muted, set_interactive_muted) = signal(false);
    let (interactive_padded, set_interactive_padded) = signal(false);
    let (interactive_custom_aria, set_interactive_custom_aria) = signal(false);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let interactive_code = Signal::derive(move || {
        let muted = interactive_muted.get();
        let padded = interactive_padded.get();
        let custom_aria = interactive_custom_aria.get();
        let custom_class = interactive_custom_class.get();

        let mut snippet = vec!["<Content".to_string()];
        if muted {
            snippet.push("  tone=ContentTone::Muted".to_string());
        }
        if padded {
            snippet.push("  padded=true".to_string());
        }
        if custom_aria {
            snippet.push("  aria_label=\"Docs content area\".into()".to_string());
        }
        if custom_class {
            snippet.push("  class_name=\"docs-content-workbench\".into()".to_string());
        }
        snippet.extend([
            ">".to_string(),
            "  <p>\"Interactive content region\"</p>".to_string(),
            "</Content>".to_string(),
        ]);
        snippet.join("\n")
    });
    let interactive_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-layout/src/content/styles.rs */\n{}",
            ui_layout::content::styles::CSS
        )
    });
    let interactive_actual_config = Signal::derive(move || {
        let tone = if interactive_muted.get() {
            ContentTone::Muted
        } else {
            ContentTone::Default
        };
        let padded = interactive_padded.get();
        let custom_aria = interactive_custom_aria.get();
        let custom_class = interactive_custom_class.get();
        let state = if padded && tone == ContentTone::Muted {
            "muted-padded"
        } else if padded {
            "padded"
        } else if tone == ContentTone::Muted {
            "muted"
        } else {
            "default"
        };

        format!(
            "ContentActualConfig {{\n  tone: ContentTone::{tone:?},\n  padded: {padded},\n  aria_label: {},\n  class_name: {},\n  aria_source: {},\n  class_source: {},\n  data_state: \"{state}\",\n}}",
            if custom_aria {
                "Some(\"Docs content area\")"
            } else {
                "None"
            },
            if custom_class {
                "Some(\"docs-content-workbench\")"
            } else {
                "None"
            },
            if custom_aria {
                "\"custom\""
            } else {
                "\"default\""
            },
            if custom_class {
                "\"custom\""
            } else {
                "\"default\""
            }
        )
    });

    view! {
        <ComponentPage
            title="Content"
            slug="content"
            group="Layout"
            description="Semantic primary-content region (`<section>`) with centralized tone/padding/source state contracts."
        >
            <Playground title="Semantic Section + Tone" code_signal=semantic_code>
                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                    <div class="docs-stack">
                        <Content>
                            <p>"Primary body content for a container region."</p>
                        </Content>
                        <Content tone=ContentTone::Muted>
                            <p>"Secondary muted notes that still stay in the same semantic content slot."</p>
                        </Content>
                    </div>
                </View>
            </Playground>

            <Playground
                title="Interactive Playground (Display / Config / Code / CSS Test)"
                code_signal=interactive_code
                test_css_source=interactive_test_css_source
                test_source_path="crates/ui-layout/src/content/styles.rs".to_string()
                test_config_signal=interactive_actual_config
                description="展示区用于当前配置与 baseline 对比；Config/Code/CSS Test 区用于快速验证语义与样式契约。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="content-config-controls">
                        <button
                            type="button"
                            on:click=move |_| set_interactive_muted.update(|value| *value = !*value)
                        >
                            "Toggle tone (default/muted)"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_interactive_padded.update(|value| *value = !*value)
                        >
                            "Toggle padded"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_interactive_custom_aria.update(|value| *value = !*value)
                        >
                            "Toggle custom aria label"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_interactive_custom_class.update(|value| *value = !*value)
                        >
                            "Toggle custom class"
                        </button>
                        <p class="ui-muted" data-slot="content-config-summary">
                            {move || {
                                format!(
                                    "config: tone={} padded={} custom_aria={} custom_class={}",
                                    if interactive_muted.get() { "muted" } else { "default" },
                                    interactive_padded.get(),
                                    interactive_custom_aria.get(),
                                    interactive_custom_class.get()
                                )
                            }}
                        </p>
                    </div>
                }
            >
                {move || {
                    let tone = if interactive_muted.get() {
                        ContentTone::Muted
                    } else {
                        ContentTone::Default
                    };
                    let padded = interactive_padded.get();
                    let aria_label = if interactive_custom_aria.get() {
                        "Docs content area".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if interactive_custom_class.get() {
                        "docs-content-workbench".to_string()
                    } else {
                        String::new()
                    };

                    view! {
                        <div class="docs-stack" data-slot="content-workbench-display">
                            <span class="ui-muted">"display: current config vs baseline"</span>
                            <div class="docs-row">
                                <View border=ViewBorder::Subtle radius=ViewRadius::Md padding=ViewPadding::Sm>
                                    <div class="ui-muted">"Current"</div>
                                    <Content
                                        tone=tone
                                        padded=padded
                                        aria_label=aria_label
                                        class_name=class_name
                                    >
                                        <p>"Interactive content region."</p>
                                    </Content>
                                </View>

                                <View border=ViewBorder::Subtle radius=ViewRadius::Md padding=ViewPadding::Sm>
                                    <div class="ui-muted">"Baseline"</div>
                                    <Content>
                                        <p>"Baseline content region."</p>
                                    </Content>
                                </View>
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Tone / Padding / Source Comparison)"
                code_signal=padded_code
            >
                <div class="docs-stack docs-stack--tight">
                    <Content>
                        <p>"Default content"</p>
                    </Content>
                    <Content tone=ContentTone::Muted padded=true>
                        <p>"Muted + padded"</p>
                    </Content>
                    <Content
                        padded=true
                        aria_label="Dialog content".to_string()
                        class_name="docs-content-custom".to_string()
                    >
                        <p>"Custom aria + class"</p>
                    </Content>
                </div>
            </Playground>

            <Playground title="Padded + Custom Aria/Class" code_signal=padded_code>
                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                    <Content
                        padded=true
                        aria_label="Dialog content".to_string()
                        class_name="docs-content-custom".to_string()
                    >
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Padded content region"</strong>
                            <span class="ui-muted">"Verifies padding marker + custom class source contract."</span>
                        </div>
                    </Content>
                </View>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
