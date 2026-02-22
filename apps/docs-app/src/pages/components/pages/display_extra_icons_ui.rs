use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    IconsUi, IconsUiSize, IconsUiTone, IconsetGlyph, SegmentedControl, SegmentedControlSize, Switch,
};

pub(super) fn icons_ui() -> AnyView {
    let workbench_icon_options = vec![
        "check (default)".to_string(),
        "ui:help (explicit)".to_string(),
        "spark (fallback/custom)".to_string(),
    ];
    let (workbench_icon_index, set_workbench_icon_index) = signal(Some(0_usize));
    let workbench_icon = Signal::derive(move || match workbench_icon_index.get().unwrap_or(0) {
        1 => "ui:help".to_string(),
        2 => "spark".to_string(),
        _ => "check".to_string(),
    });

    let workbench_size_options = vec!["Md".to_string(), "Lg".to_string()];
    let (workbench_size_index, set_workbench_size_index) = signal(Some(0_usize));
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(0) {
        1 => IconsUiSize::Lg,
        _ => IconsUiSize::Md,
    });

    let workbench_tone_options = vec![
        "Accent".to_string(),
        "Muted".to_string(),
        "Danger".to_string(),
    ];
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let workbench_tone = Signal::derive(move || match workbench_tone_index.get().unwrap_or(0) {
        1 => IconsUiTone::Muted,
        2 => IconsUiTone::Danger,
        _ => IconsUiTone::Accent,
    });

    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_decorative, set_workbench_decorative) = signal(false);
    let (workbench_custom_glyph, set_workbench_custom_glyph) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_glyphs = Signal::derive(move || {
        if !workbench_custom_glyph.get() {
            return Vec::new();
        }
        vec![IconsetGlyph::new("ui:spark", "✦").with_aria_label("UI Spark")]
    });

    let workbench_code = Signal::derive(move || {
        let icon = workbench_icon.get();
        let size = workbench_size.get();
        let tone = workbench_tone.get();
        let disabled = workbench_disabled.get();
        let decorative = workbench_decorative.get();
        let custom_glyph = workbench_custom_glyph.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();

        let mut lines = vec![
            "<IconsUi".to_string(),
            format!("  icon=\"{icon}\".into()"),
            format!("  size=IconsUiSize::{size:?}"),
            format!("  tone=IconsUiTone::{tone:?}"),
            format!("  is_decorative={decorative}"),
        ];
        if disabled {
            lines.push("  is_disabled=true".to_string());
        }
        if custom_glyph {
            lines.push(
                "  glyphs=vec![IconsetGlyph::new(\"ui:spark\", \"✦\").with_aria_label(\"UI Spark\")]".to_string(),
            );
        }
        if custom_aria {
            lines.push("  aria_label=\"Workbench icon\".into()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-icons-ui-workbench\".into()".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/icon/ui/styles.rs */\n{}\n\n/* crates/ui/src/icon/set/styles.rs */\n{}",
            ui::icons_ui::styles::CSS,
            ui::iconset::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "IconsUiWorkbenchConfig {{\n  icon: \"{}\",\n  size: {:?},\n  tone: {:?},\n  disabled: {},\n  decorative: {},\n  custom_glyph: {},\n  custom_aria: {},\n  custom_class: {},\n}}",
            workbench_icon.get(),
            workbench_size.get(),
            workbench_tone.get(),
            workbench_disabled.get(),
            workbench_decorative.get(),
            workbench_custom_glyph.get(),
            workbench_custom_aria.get(),
            workbench_custom_class.get()
        )
    });

    let defaults_code = Signal::derive(move || {
        r#"<IconsUi icon="check".to_string() size=IconsUiSize::Md tone=IconsUiTone::Accent is_decorative=false />
<IconsUi icon="close".to_string() size=IconsUiSize::Md tone=IconsUiTone::Danger is_decorative=false />"#.to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<IconsUi
  icon="ui:save".to_string()
  glyphs=vec![IconsetGlyph::new("ui:save", "💾").with_aria_label("UI Save")]
  size=IconsUiSize::Lg
  tone=IconsUiTone::Default
  is_decorative=false
  class_name="docs-icons-ui-custom".to_string()
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"<IconsUi
  icon="help".to_string()
  glyphs=vec![IconsetGlyph::new("ui:help", "?").with_aria_label("UI Help")]
  size=IconsUiSize::Lg
  tone=IconsUiTone::Muted
  is_decorative=false
  aria_label="Explicit UI help icon".to_string()
  class_name="docs-icons-ui-state".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="IconsUi"
            slug="icons-ui"
            group="Display"
            description="baseline-compatible icons-ui wrapper with built-in UI icon registry defaults, namespace normalization, and Iconset accessibility/source-state contracts."
        >
            <Playground title="Built-in UI Glyphs" code_signal=defaults_code>
                <div class="docs-row">
                    <IconsUi
                        icon="check".to_string()
                        size=IconsUiSize::Md
                        tone=IconsUiTone::Accent
                        is_decorative=false
                    />
                    <IconsUi
                        icon="close".to_string()
                        size=IconsUiSize::Md
                        tone=IconsUiTone::Danger
                        is_decorative=false
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test workbench for icons-ui source/state contract tuning."
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/icon/ui/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="icons-ui-workbench-controls">
                        <div class="docs-search__label">"Icon"</div>
                        <SegmentedControl
                            id_base="docs-icons-ui-workbench-icon".to_string()
                            options=workbench_icon_options.clone()
                            selected_index=workbench_icon_index
                            set_selected_index=set_workbench_icon_index
                            size=SegmentedControlSize::Sm
                            aria_label="IconsUi icon picker".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-icons-ui-workbench-size".to_string()
                            options=workbench_size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="IconsUi size".to_string()
                        />

                        <div class="docs-search__label">"Tone"</div>
                        <SegmentedControl
                            id_base="docs-icons-ui-workbench-tone".to_string()
                            options=workbench_tone_options.clone()
                            selected_index=workbench_tone_index
                            set_selected_index=set_workbench_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="IconsUi tone".to_string()
                        />

                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_decorative set_checked=set_workbench_decorative>
                            "Decorative"
                        </Switch>
                        <Switch checked=workbench_custom_glyph set_checked=set_workbench_custom_glyph>
                            "Custom glyph (ui:spark)"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "Custom aria label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <IconsUi
                            icon=workbench_icon.get()
                            size=workbench_size.get()
                            tone=workbench_tone.get()
                            is_disabled=workbench_disabled.get()
                            is_decorative=workbench_decorative.get()
                            aria_label=if workbench_custom_aria.get() {
                                "Workbench icon".to_string()
                            } else {
                                String::new()
                            }
                            class_name=if workbench_custom_class.get() {
                                "docs-icons-ui-workbench".to_string()
                            } else {
                                String::new()
                            }
                            glyphs=workbench_glyphs.get()
                        />
                    </div>
                    <span class="ui-muted">{move || format!("icon: {}", workbench_icon.get())}</span>
                </div>
            </Playground>

            <Playground title="Custom Registry Extension" code_signal=custom_code>
                <div class="docs-row">
                    <IconsUi
                        icon="ui:save".to_string()
                        glyphs=vec![IconsetGlyph::new("ui:save", "💾").with_aria_label("UI Save")]
                        size=IconsUiSize::Lg
                        tone=IconsUiTone::Default
                        is_decorative=false
                        class_name="docs-icons-ui-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect wrapper markers like `data-state`, `data-icon-reference-source`, `data-aria-source`, `data-class-source`, `data-glyph-source`, `data-size-source`, and `data-tone-source`."
                code_signal=markers_code
            >
                <div class="docs-row">
                    <IconsUi
                        icon="help".to_string()
                        glyphs=vec![
                            IconsetGlyph::new("ui:help", "?").with_aria_label("UI Help"),
                        ]
                        size=IconsUiSize::Lg
                        tone=IconsUiTone::Muted
                        is_decorative=false
                        aria_label="Explicit UI help icon".to_string()
                        class_name="docs-icons-ui-state".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
