use super::*;

pub(crate) fn header() -> AnyView {
    let (interactive_strong_tone, set_interactive_strong_tone) = signal(false);
    let (interactive_bordered, set_interactive_bordered) = signal(false);

    let semantic_code = Signal::derive(move || {
        r#"<Header>
  <h3>"Dialog title"</h3>
</Header>
<Header tone=HeaderTone::Strong>
  <h3>"Strong header"</h3>
</Header>"#
            .to_string()
    });

    let bordered_code = Signal::derive(move || {
        r#"<View border=ViewBorder::Subtle radius=ViewRadius::Md>
  <Header
    tone=HeaderTone::Strong
    bordered=true
    aria_label="Settings header".to_string()
    class_name="docs-header-custom".to_string()
  >
    <h3>"Settings"</h3>
  </Header>
  <Content padded=true>
    <p>"Header above content, matching baseline container semantics."</p>
  </Content>
</View>"#
            .to_string()
    });

    let interactive_code = Signal::derive(move || {
        format!(
            "let (strong_tone, set_strong_tone) = signal({});\nlet (bordered, set_bordered) = signal({});\n\n<Header\n  tone=if strong_tone.get() {{ HeaderTone::Strong }} else {{ HeaderTone::Default }}\n  bordered=bordered.get()\n>\n  <h3>\"Interactive header\"</h3>\n</Header>",
            bool_word(interactive_strong_tone.get()),
            bool_word(interactive_bordered.get()),
        )
    });
    let test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-layout/src/header/styles.rs */\n{}",
            ui_layout::header::styles::CSS
        )
    });
    let actual_config = Signal::derive(move || {
        let strong_tone = interactive_strong_tone.get();
        let bordered = interactive_bordered.get();

        let mut classes = vec![
            "ui-header".to_string(),
            if strong_tone {
                "ui-header--tone-strong".to_string()
            } else {
                "ui-header--tone-default".to_string()
            },
        ];
        if bordered {
            classes.push("ui-header--bordered".to_string());
        }
        classes.push("ui-header--custom-class".to_string());
        classes.push("docs-header-interactive".to_string());

        format!(
            "HeaderActualConfig {{\n  tone: {},\n  bordered: {},\n  aria_label: \"Interactive docs header\",\n  class_name: \"docs-header-interactive\",\n  motion: HeaderMotion::default(),\n  lang: Some(\"en-US\"),\n  dir: Some(A11yDirection::Ltr),\n  class: \"{}\",\n}}",
            if strong_tone { "Strong" } else { "Default" },
            bordered,
            classes.join(" ")
        )
    });

    view! {
        <ComponentPage
            title="Header"
            slug="header"
            group="Layout"
            description="Semantic container header (`<header>`) with centralized tone/border/source state contracts."
        >
            <Playground title="Semantic Header + Tone" code_signal=semantic_code>
                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                    <div class="docs-stack">
                        <Header>
                            <h3>"Dialog title"</h3>
                        </Header>
                        <Header tone=HeaderTone::Strong>
                            <h3>"Strong header"</h3>
                        </Header>
                    </div>
                </View>
            </Playground>

            <Playground
                title="Interactive Playground (State + Source Markers)"
                code_signal=interactive_code
                test_css_source=test_css_source
                test_source_path="/root/code/personal/omne/rust-ui/crates/ui-layout/src/header/styles.rs".to_string()
                test_config_signal=actual_config
                description="Workbench canvas: 展示区负责状态对比；Config/Code/CSS Test 区用于快速验证契约。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="header-config-controls">
                        <div class="docs-search__label">"Tone"</div>
                        <button
                            type="button"
                            data-action="toggle-tone-config"
                            on:click=move |_| {
                                set_interactive_strong_tone.update(|value| *value = !*value);
                            }
                        >
                            "Toggle strong tone"
                        </button>
                        <div class="docs-search__label">"Border"</div>
                        <button
                            type="button"
                            data-action="toggle-bordered-config"
                            on:click=move |_| {
                                set_interactive_bordered.update(|value| *value = !*value);
                            }
                        >
                            "Toggle bordered"
                        </button>
                        <p class="ui-muted" data-slot="header-config-summary">
                            {move || {
                                format!(
                                    "config: tone={} bordered={}",
                                    if interactive_strong_tone.get() {
                                        "strong"
                                    } else {
                                        "default"
                                    },
                                    if interactive_bordered.get() {
                                        "true"
                                    } else {
                                        "false"
                                    }
                                )
                            }}
                        </p>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="header-interactive-controls">
                    <div class="docs-row" data-slot="header-actions">
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
                            data-action="toggle-bordered"
                            on:click=move |_| {
                                set_interactive_bordered.update(|value| *value = !*value);
                            }
                        >
                            "Toggle bordered"
                        </button>
                    </div>

                    <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                        <Header
                            tone=if interactive_strong_tone.get() {
                                HeaderTone::Strong
                            } else {
                                HeaderTone::Default
                            }
                            bordered=interactive_bordered.get()
                            motion=ui_layout::header::motion::HeaderMotion::default()
                            class_name="docs-header-interactive".to_string()
                            aria_label="Interactive docs header".to_string()
                            lang="en-US".to_string()
                            dir=A11yDirection::Ltr
                        >
                            <h3>"Interactive header"</h3>
                        </Header>
                        <Content padded=true>
                            <p data-slot="header-interactive-summary">
                                {move || {
                                    format!(
                                        "tone={} bordered={}",
                                        if interactive_strong_tone.get() {
                                            "strong"
                                        } else {
                                            "default"
                                        },
                                        if interactive_bordered.get() {
                                            "true"
                                        } else {
                                            "false"
                                        }
                                    )
                                }}
                            </p>
                        </Content>
                    </View>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Tone / Border / Locale Comparison)"
                code_signal=bordered_code
            >
                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                    <Header
                        tone=HeaderTone::Default
                        bordered=false
                        aria_label="Default header".to_string()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        <h3>"Default header"</h3>
                    </Header>
                    <Header
                        tone=HeaderTone::Strong
                        bordered=true
                        aria_label="Strong bordered header".to_string()
                        class_name="docs-header-custom".to_string()
                        motion=ui_layout::header::motion::HeaderMotion::default()
                        lang="ar".to_string()
                        dir=A11yDirection::Rtl
                    >
                        <h3>"Strong + bordered + RTL"</h3>
                    </Header>
                </View>
            </Playground>

            <Playground title="Bordered + Custom Aria/Class" code_signal=bordered_code>
                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                    <Header
                        tone=HeaderTone::Strong
                        bordered=true
                        aria_label="Settings header".to_string()
                        class_name="docs-header-custom".to_string()
                    >
                        <h3>"Settings"</h3>
                    </Header>
                    <Content padded=true>
                        <p>"Header above content, matching baseline container semantics."</p>
                    </Content>
                </View>
            </Playground>

            <section class="docs-card docs-prose" data-slot="header-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each header playground supports "
                    <code>"Show code"</code>
                    " + copy. Copied snippets are import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui_layout::*;\n\n<Header>\n  <h3>\"Settings\"</h3>\n</Header>".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-header-source-copy".to_string()
                />
                <ul data-slot="header-source-paths">
                    <li><code>"crates/ui-layout/src/header/mod.rs"</code></li>
                    <li><code>"crates/ui-layout/src/header/logic.rs"</code></li>
                    <li><code>"crates/ui-layout/src/header/view.rs"</code></li>
                    <li><code>"crates/ui-layout/src/header/styles.rs"</code></li>
                    <li><code>"crates/ui-layout/src/header/motion.rs"</code></li>
                </ul>
                <ul data-slot="header-source-prerequisites">
                    <li><code>"component-header"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
