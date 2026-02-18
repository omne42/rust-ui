use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Iconset, IconsetGlyph, IconsetSize, IconsetTone, SegmentedControl, SegmentedControlSize, Switch,
};

pub(super) fn iconset() -> AnyView {
    let icon_options = vec![
        "workflow:check".to_string(),
        "workflow:alert".to_string(),
        "ui:unknown".to_string(),
    ];
    let size_options = vec!["sm".to_string(), "md".to_string(), "lg".to_string()];
    let tone_options = vec![
        "default".to_string(),
        "muted".to_string(),
        "accent".to_string(),
        "danger".to_string(),
    ];

    let (icon_index, set_icon_index) = signal(Some(0_usize));
    let (size_index, set_size_index) = signal(Some(1_usize));
    let (tone_index, set_tone_index) = signal(Some(2_usize));
    let (disabled, set_disabled) = signal(false);
    let (decorative, set_decorative) = signal(false);
    let (custom_aria_label, set_custom_aria_label) = signal(false);
    let (custom_class, set_custom_class) = signal(false);

    let icon_value: Signal<String> = Signal::derive(move || match icon_index.get().unwrap_or(0) {
        1 => "workflow:alert".to_string(),
        2 => "ui:unknown".to_string(),
        _ => "workflow:check".to_string(),
    });
    let size_value: Signal<IconsetSize> =
        Signal::derive(move || match size_index.get().unwrap_or(1) {
            0 => IconsetSize::Sm,
            2 => IconsetSize::Lg,
            _ => IconsetSize::Md,
        });
    let tone_value: Signal<IconsetTone> =
        Signal::derive(move || match tone_index.get().unwrap_or(2) {
            0 => IconsetTone::Default,
            1 => IconsetTone::Muted,
            3 => IconsetTone::Danger,
            _ => IconsetTone::Accent,
        });

    let workbench_code = Signal::derive(move || {
        let size = size_value.get();
        let tone = tone_value.get();
        let mut lines = vec![
            "<Iconset".to_string(),
            format!("  icon=\"{}\".to_string()", icon_value.get()),
            "  glyphs=vec![".to_string(),
            "    IconsetGlyph::new(\"workflow:check\", \"✓\").with_aria_label(\"Workflow Check\"),"
                .to_string(),
            "    IconsetGlyph::new(\"workflow:alert\", \"⚠\").with_aria_label(\"Workflow Alert\"),"
                .to_string(),
            "  ]".to_string(),
        ];
        if size != IconsetSize::Md {
            lines.push(format!("  size=IconsetSize::{size:?}"));
        }
        if tone != IconsetTone::Accent {
            lines.push(format!("  tone=IconsetTone::{tone:?}"));
        }
        if disabled.get() {
            lines.push("  disabled=true".to_string());
        }
        if decorative.get() {
            lines.push("  decorative=true".to_string());
        } else {
            lines.push("  decorative=false".to_string());
        }
        if custom_aria_label.get() {
            lines.push("  aria_label=\"Custom icon label\".to_string()".to_string());
        }
        if custom_class.get() {
            lines.push("  class_name=\"docs-iconset-custom\".to_string()".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let registry_code = Signal::derive(move || {
        r#"<Iconset
  icon="workflow:check".to_string()
  glyphs=vec![
    IconsetGlyph::new("workflow:check", "✓").with_aria_label("Workflow Check"),
    IconsetGlyph::new("workflow:alert", "⚠").with_aria_label("Workflow Alert"),
  ]
  size=IconsetSize::Md
  tone=IconsetTone::Accent
  decorative=false
/>"#
        .to_string()
    });

    let fallback_code = Signal::derive(move || {
        r#"<Iconset
  icon="ui:unknown".to_string()
  iconset="ui".to_string()
  size=IconsetSize::Lg
  tone=IconsetTone::Muted
  decorative=false
  class_name="docs-iconset-custom".to_string()
/>"#
        .to_string()
    });

    let workflow_glyphs = vec![
        IconsetGlyph::new("workflow:check", "✓").with_aria_label("Workflow Check"),
        IconsetGlyph::new("workflow:alert", "⚠").with_aria_label("Workflow Alert"),
    ];
    let workbench_glyphs = workflow_glyphs.clone();
    let comparison_glyphs = workflow_glyphs.clone();

    let test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/iconset/styles.rs */\n{}",
            ui_components::iconset::styles::CSS
        )
    });

    let actual_config = Signal::derive(move || {
        let tone = tone_value.get();
        let size = size_value.get();
        let mut classes = vec![
            "ui-icon".to_string(),
            size.class_name().to_string(),
            tone.class_name().to_string(),
        ];
        if disabled.get() {
            classes.push("ui-icon--disabled".to_string());
        }
        if decorative.get() {
            classes.push("ui-icon--decorative".to_string());
        }
        if custom_class.get() {
            classes.push("docs-iconset-custom".to_string());
        }
        format!(
            "IconsetActualConfig {{\n  icon: \"{}\",\n  size: {size:?},\n  tone: {tone:?},\n  disabled: {},\n  decorative: {},\n  has_custom_aria_label: {},\n  has_custom_class_name: {},\n  class: \"{}\",\n}}",
            icon_value.get(),
            disabled.get(),
            decorative.get(),
            custom_aria_label.get(),
            custom_class.get(),
            classes.join(" ")
        )
    });

    let comparison_code = Signal::derive(move || {
        r#"<Iconset icon="workflow:check".to_string() glyphs=glyphs.clone() decorative=false />
<Iconset
  icon="workflow:alert".to_string()
  glyphs=glyphs.clone()
  tone=IconsetTone::Danger
  decorative=false
/>
<Iconset
  icon="ui:unknown".to_string()
  iconset="ui".to_string()
  tone=IconsetTone::Muted
  decorative=false
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"<Iconset
  icon="workflow:check".to_string()
  iconset="workflow".to_string()
  glyphs=vec![IconsetGlyph::new("workflow:check", "✓").with_aria_label("Registry Check")]
  size=IconsetSize::Lg
  tone=IconsetTone::Danger
  decorative=false
  aria_label="Explicit workflow check".to_string()
  class_name="docs-iconset-state".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Iconset"
            slug="iconset"
            group="Display"
            description="baseline-compatible Iconset registry wrapper for namespace + icon-name resolution, composed on Icon accessibility contracts with stable source markers."
        >
            <Playground
                title="Workbench"
                description="Interactive display/config/code/css-test playground for Iconset registry behavior."
                code_signal=workbench_code
                test_css_source=test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/iconset/styles.rs".to_string()
                test_config_signal=actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Icon"</div>
                        <SegmentedControl
                            id_base="docs-iconset-icon".to_string()
                            options=icon_options.clone()
                            selected_index=icon_index
                            set_selected_index=set_icon_index
                            size=SegmentedControlSize::Sm
                            aria_label="Iconset icon".to_string()
                        />
                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-iconset-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="Iconset size".to_string()
                        />
                        <div class="docs-search__label">"Tone"</div>
                        <SegmentedControl
                            id_base="docs-iconset-tone".to_string()
                            options=tone_options.clone()
                            selected_index=tone_index
                            set_selected_index=set_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="Iconset tone".to_string()
                        />
                        <Switch checked=disabled set_checked=set_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=decorative set_checked=set_decorative>
                            "Decorative"
                        </Switch>
                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    view! {
                        <div class="docs-row">
                            <Iconset
                                icon=icon_value.get()
                                glyphs=workbench_glyphs.clone()
                                size=size_value.get()
                                tone=tone_value.get()
                                disabled=disabled.get()
                                decorative=decorative.get()
                                aria_label=if custom_aria_label.get() {
                                    "Custom icon label".to_string()
                                } else {
                                    String::new()
                                }
                                class_name=if custom_class.get() {
                                    "docs-iconset-custom".to_string()
                                } else {
                                    String::new()
                                }
                            />
                        </div>
                    }
                }}
            </Playground>

            <Playground title="Registry Namespace Resolution" code_signal=registry_code>
                <div class="docs-row">
                    <Iconset
                        icon="workflow:check".to_string()
                        glyphs=workflow_glyphs.clone()
                        size=IconsetSize::Md
                        tone=IconsetTone::Accent
                        decorative=false
                    />
                    <Iconset
                        icon="workflow:alert".to_string()
                        glyphs=workflow_glyphs.clone()
                        size=IconsetSize::Md
                        tone=IconsetTone::Danger
                        decorative=false
                    />
                </div>
            </Playground>

            <Playground title="Fallback + Source State" code_signal=fallback_code>
                <div class="docs-row">
                    <Iconset
                        icon="ui:unknown".to_string()
                        iconset="ui".to_string()
                        size=IconsetSize::Lg
                        tone=IconsetTone::Muted
                        decorative=false
                        class_name="docs-iconset-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground title="State Comparison" code_signal=comparison_code>
                <div class="docs-row">
                    <Iconset
                        icon="workflow:check".to_string()
                        glyphs=comparison_glyphs.clone()
                        decorative=false
                    />
                    <Iconset
                        icon="workflow:alert".to_string()
                        glyphs=comparison_glyphs.clone()
                        tone=IconsetTone::Danger
                        decorative=false
                    />
                    <Iconset
                        icon="ui:unknown".to_string()
                        iconset="ui".to_string()
                        tone=IconsetTone::Muted
                        decorative=false
                    />
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect wrapper markers like `data-state`, `data-icon-source`, `data-iconset-source`, `data-label-source`, `data-class-source`, `data-size-source`, and `data-tone-source`."
                code_signal=markers_code
            >
                <div class="docs-row">
                    <Iconset
                        icon="workflow:check".to_string()
                        iconset="workflow".to_string()
                        glyphs=vec![
                            IconsetGlyph::new("workflow:check", "✓")
                                .with_aria_label("Registry Check"),
                        ]
                        size=IconsetSize::Lg
                        tone=IconsetTone::Danger
                        decorative=false
                        aria_label="Explicit workflow check".to_string()
                        class_name="docs-iconset-state".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
