use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{IconsWorkflow, IconsWorkflowSize, IconsWorkflowTone, IconsetGlyph};

pub(super) fn icons_workflow() -> AnyView {
    let defaults_code = Signal::derive(move || {
        r#"<IconsWorkflow icon="success".to_string() size=IconsWorkflowSize::Md tone=IconsWorkflowTone::Accent is_decorative=false />
<IconsWorkflow icon="warning".to_string() size=IconsWorkflowSize::Md tone=IconsWorkflowTone::Danger is_decorative=false />"#.to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<IconsWorkflow
  icon="workflow:deploy".to_string()
  glyphs=vec![IconsetGlyph::new("workflow:deploy", "🚀").with_aria_label("Workflow Deploy")]
  size=IconsWorkflowSize::Lg
  tone=IconsWorkflowTone::Default
  is_decorative=false
  class_name="docs-icons-workflow-custom".to_string()
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"<IconsWorkflow
  icon="success".to_string()
  glyphs=vec![IconsetGlyph::new("workflow:success", "✓").with_aria_label("Workflow Success")]
  size=IconsWorkflowSize::Lg
  tone=IconsWorkflowTone::Muted
  is_decorative=false
  aria_label="Explicit workflow success icon".to_string()
  class_name="docs-icons-workflow-state".to_string()
/>"#
        .to_string()
    });

    let (interactive_icon_mode, set_interactive_icon_mode) = signal(0_usize);
    let (interactive_size_lg, set_interactive_size_lg) = signal(false);
    let (interactive_tone_mode, set_interactive_tone_mode) = signal(1_usize);
    let (interactive_disabled, set_interactive_disabled) = signal(false);
    let (interactive_decorative, set_interactive_decorative) = signal(false);
    let (interactive_custom_aria, set_interactive_custom_aria) = signal(false);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let (interactive_custom_glyph, set_interactive_custom_glyph) = signal(false);

    let interactive_code = Signal::derive(move || {
        let icon_mode = interactive_icon_mode.get();
        let icon = match icon_mode {
            1 => "workflow:warning",
            2 => "",
            _ => "success",
        };
        let size = if interactive_size_lg.get() {
            IconsWorkflowSize::Lg
        } else {
            IconsWorkflowSize::Md
        };
        let tone = match interactive_tone_mode.get() {
            1 => IconsWorkflowTone::Accent,
            2 => IconsWorkflowTone::Danger,
            3 => IconsWorkflowTone::Muted,
            _ => IconsWorkflowTone::Default,
        };

        let mut snippet = vec![
            "<IconsWorkflow".to_string(),
            format!("  icon=\"{icon}\".into()"),
            format!("  size=IconsWorkflowSize::{size:?}"),
            format!("  tone=IconsWorkflowTone::{tone:?}"),
            format!("  is_decorative={}", interactive_decorative.get()),
        ];

        if interactive_disabled.get() {
            snippet.push("  is_disabled=true".to_string());
        }
        if interactive_custom_aria.get() {
            snippet.push("  aria_label=\"Interactive workflow icon\".into()".to_string());
        }
        if interactive_custom_class.get() {
            snippet.push("  class_name=\"docs-icons-workflow-workbench\".into()".to_string());
        }
        if interactive_custom_glyph.get() {
            snippet.push(
                "  glyphs=vec![IconsetGlyph::new(\"workflow:deploy\", \"🚀\").with_aria_label(\"Workflow Deploy\")]".to_string(),
            );
        }
        snippet.push("/>".to_string());
        snippet.join("\n")
    });
    let interactive_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/icon/workflow/styles.rs */\n{}",
            ui_components::icons_workflow::styles::CSS
        )
    });
    let interactive_actual_config = Signal::derive(move || {
        let icon_mode = interactive_icon_mode.get();
        let icon = match icon_mode {
            1 => "workflow:warning",
            2 => "",
            _ => "success",
        };
        let icon_reference_source = match icon_mode {
            1 => "explicit",
            2 => "default",
            _ => "prefixed",
        };
        let size = if interactive_size_lg.get() {
            IconsWorkflowSize::Lg
        } else {
            IconsWorkflowSize::Md
        };
        let tone = match interactive_tone_mode.get() {
            1 => IconsWorkflowTone::Accent,
            2 => IconsWorkflowTone::Danger,
            3 => IconsWorkflowTone::Muted,
            _ => IconsWorkflowTone::Default,
        };
        let state = if interactive_disabled.get() {
            "disabled"
        } else if interactive_decorative.get() {
            "decorative"
        } else {
            "ready"
        };

        format!(
            "IconsWorkflowActualConfig {{\n  icon: \"{icon}\",\n  icon_reference_source: \"{icon_reference_source}\",\n  size: IconsWorkflowSize::{size:?},\n  tone: IconsWorkflowTone::{tone:?},\n  disabled: {},\n  decorative: {},\n  aria_source: {},\n  class_source: {},\n  glyph_source: {},\n  state: \"{state}\",\n}}",
            interactive_disabled.get(),
            interactive_decorative.get(),
            if interactive_custom_aria.get() {
                "\"custom\""
            } else {
                "\"default\""
            },
            if interactive_custom_class.get() {
                "\"custom\""
            } else {
                "\"default\""
            },
            if interactive_custom_glyph.get() {
                "\"custom\""
            } else {
                "\"default\""
            }
        )
    });

    view! {
        <ComponentPage
            title="IconsWorkflow"
            slug="icons-workflow"
            group="Display"
            description="baseline-compatible icons-workflow wrapper with workflow namespace normalization, built-in workflow glyph defaults, and Iconset accessibility/source-state contracts."
        >
            <Playground title="Built-in Workflow Glyphs" code_signal=defaults_code>
                <div class="docs-row">
                    <IconsWorkflow
                        icon="success".to_string()
                        size=IconsWorkflowSize::Md
                        tone=IconsWorkflowTone::Accent
                        is_decorative=false
                    />
                    <IconsWorkflow
                        icon="warning".to_string()
                        size=IconsWorkflowSize::Md
                        tone=IconsWorkflowTone::Danger
                        is_decorative=false
                    />
                </div>
            </Playground>

            <Playground title="Custom Workflow Extension" code_signal=custom_code>
                <div class="docs-row">
                    <IconsWorkflow
                        icon="workflow:deploy".to_string()
                        glyphs=vec![
                            IconsetGlyph::new("workflow:deploy", "🚀")
                                .with_aria_label("Workflow Deploy"),
                        ]
                        size=IconsWorkflowSize::Lg
                        tone=IconsWorkflowTone::Default
                        is_decorative=false
                        class_name="docs-icons-workflow-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect wrapper markers like `data-state`, `data-icon-reference-source`, `data-aria-source`, `data-class-source`, `data-glyph-source`, `data-size-source`, and `data-tone-source`."
                code_signal=markers_code
            >
                <div class="docs-row">
                    <IconsWorkflow
                        icon="success".to_string()
                        glyphs=vec![
                            IconsetGlyph::new("workflow:success", "✓")
                                .with_aria_label("Workflow Success"),
                        ]
                        size=IconsWorkflowSize::Lg
                        tone=IconsWorkflowTone::Muted
                        is_decorative=false
                        aria_label="Explicit workflow success icon".to_string()
                        class_name="docs-icons-workflow-state".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Display / Config / Code / CSS Test)"
                code_signal=interactive_code
                test_css_source=interactive_test_css_source
                test_source_path="crates/ui-components/src/icon/workflow/styles.rs".to_string()
                test_config_signal=interactive_actual_config
                description="展示区用于 current 与 baseline 对比；Config/Code/CSS Test 区用于验证 icon/source/aria/class/glyph 合同。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="icons-workflow-config-controls">
                        <button
                            type="button"
                            on:click=move |_| {
                                set_interactive_icon_mode.update(|mode| *mode = (*mode + 1) % 3);
                            }
                        >
                            "Cycle icon reference (prefixed/explicit/default)"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| {
                                set_interactive_size_lg.update(|value| *value = !*value);
                            }
                        >
                            "Toggle size (md/lg)"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| {
                                set_interactive_tone_mode.update(|mode| *mode = (*mode + 1) % 4);
                            }
                        >
                            "Cycle tone (default/accent/danger/muted)"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_interactive_disabled.update(|value| *value = !*value)
                        >
                            "Toggle disabled"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_interactive_decorative.update(|value| *value = !*value)
                        >
                            "Toggle decorative"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| {
                                set_interactive_custom_aria.update(|value| *value = !*value);
                            }
                        >
                            "Toggle custom aria label"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| {
                                set_interactive_custom_class.update(|value| *value = !*value);
                            }
                        >
                            "Toggle custom class"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| {
                                set_interactive_custom_glyph.update(|value| *value = !*value);
                            }
                        >
                            "Toggle custom glyph registry"
                        </button>
                        <p class="ui-muted" data-slot="icons-workflow-config-summary">
                            {move || {
                                let icon = match interactive_icon_mode.get() {
                                    1 => "workflow:warning",
                                    2 => "<empty -> default>",
                                    _ => "success",
                                };
                                let tone = match interactive_tone_mode.get() {
                                    1 => "accent",
                                    2 => "danger",
                                    3 => "muted",
                                    _ => "default",
                                };
                                format!(
                                    "config: icon={} size={} tone={} is_disabled={} is_decorative={} custom_aria={} custom_class={} custom_glyph={}",
                                    icon,
                                    if interactive_size_lg.get() { "lg" } else { "md" },
                                    tone,
                                    interactive_disabled.get(),
                                    interactive_decorative.get(),
                                    interactive_custom_aria.get(),
                                    interactive_custom_class.get(),
                                    interactive_custom_glyph.get()
                                )
                            }}
                        </p>
                    </div>
                }
            >
                {move || {
                    let icon = match interactive_icon_mode.get() {
                        1 => "workflow:warning".to_string(),
                        2 => "".to_string(),
                        _ => "success".to_string(),
                    };
                    let size = if interactive_size_lg.get() {
                        IconsWorkflowSize::Lg
                    } else {
                        IconsWorkflowSize::Md
                    };
                    let tone = match interactive_tone_mode.get() {
                        1 => IconsWorkflowTone::Accent,
                        2 => IconsWorkflowTone::Danger,
                        3 => IconsWorkflowTone::Muted,
                        _ => IconsWorkflowTone::Default,
                    };
                    let disabled = interactive_disabled.get();
                    let decorative = interactive_decorative.get();
                    let aria_label = if interactive_custom_aria.get() {
                        "Interactive workflow icon".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if interactive_custom_class.get() {
                        "docs-icons-workflow-workbench".to_string()
                    } else {
                        String::new()
                    };
                    let glyphs = if interactive_custom_glyph.get() {
                        vec![
                            IconsetGlyph::new("workflow:deploy", "🚀")
                                .with_aria_label("Workflow Deploy"),
                        ]
                    } else {
                        Vec::new()
                    };

                    view! {
                        <div class="docs-stack" data-slot="icons-workflow-workbench-display">
                            <span class="ui-muted">"display: current config vs baseline"</span>
                            <div class="docs-row">
                                <div class="docs-stack docs-stack--tight">
                                    <span class="ui-muted">"Current"</span>
                                    <IconsWorkflow
                                        icon=icon
                                        glyphs=glyphs
                                        size=size
                                        tone=tone
                                        is_disabled=disabled
                                        is_decorative=decorative
                                        aria_label=aria_label
                                        class_name=class_name
                                    />
                                </div>
                                <div class="docs-stack docs-stack--tight">
                                    <span class="ui-muted">"Baseline"</span>
                                    <IconsWorkflow
                                        icon="success".to_string()
                                        size=IconsWorkflowSize::Md
                                        tone=IconsWorkflowTone::Accent
                                        is_decorative=false
                                    />
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
