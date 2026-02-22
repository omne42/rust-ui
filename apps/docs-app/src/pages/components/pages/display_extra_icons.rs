use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    Icons, IconsGlyph, IconsScale, IconsSet, IconsTone, SegmentedControl, SegmentedControlSize,
};

pub(super) fn icons() -> AnyView {
    let picker_options = vec![
        "✓ check".to_string(),
        "⚠ workflow:warning".to_string(),
        "✓ workflow:check".to_string(),
    ];
    let (picker_index, set_picker_index) = signal(Some(0_usize));
    let picker_name = Signal::derive(move || match picker_index.get().unwrap_or(0) {
        1 => "workflow:warning".to_string(),
        2 => "workflow:check".to_string(),
        _ => "check".to_string(),
    });
    let picker_set = Signal::derive(move || match picker_index.get().unwrap_or(0) {
        0 => IconsSet::Ui,
        _ => IconsSet::Workflow,
    });
    let picker_scale = Signal::derive(move || match picker_index.get().unwrap_or(0) {
        0 => IconsScale::Medium,
        _ => IconsScale::Large,
    });
    let picker_tone = Signal::derive(move || match picker_index.get().unwrap_or(0) {
        1 => IconsTone::Danger,
        2 => IconsTone::Muted,
        _ => IconsTone::Accent,
    });
    let picker_code = Signal::derive(move || {
        let name = picker_name.get();
        let set = picker_set.get();
        let scale = picker_scale.get();
        let tone = picker_tone.get();

        format!(
            "<Icons\n  name=\"{name}\".into()\n  set=IconsSet::{set:?}\n  scale=IconsScale::{scale:?}\n  tone=IconsTone::{tone:?}\n  is_decorative=false\n/>"
        )
    });
    let picker_actual_config = Signal::derive(move || {
        let name = picker_name.get();
        let set = picker_set.get();
        let scale = picker_scale.get();
        let tone = picker_tone.get();

        format!(
            "IconsActualConfig {{\n  name: \"{name}\",\n  set: IconsSet::{set:?},\n  scale: IconsScale::{scale:?},\n  tone: IconsTone::{tone:?},\n  decorative: false,\n}}"
        )
    });

    let default_code = Signal::derive(move || {
        r#"<Icons name="check".to_string() set=IconsSet::Ui scale=IconsScale::Medium tone=IconsTone::Accent is_decorative=false />
<Icons name="workflow:warning".to_string() scale=IconsScale::Large tone=IconsTone::Danger is_decorative=false />"#.to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Icons
  name="workflow:deploy".to_string()
  set=IconsSet::Workflow
  scale=IconsScale::Large
  tone=IconsTone::Default
  glyphs=vec![IconsGlyph::new("workflow:deploy", "🚀").with_aria_label("Workflow Deploy")]
  is_decorative=false
  class_name="docs-icons-custom".to_string()
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"<Icons
  name="check".to_string()
  set=IconsSet::Workflow
  scale=IconsScale::Large
  tone=IconsTone::Muted
  glyphs=vec![IconsGlyph::new("workflow:check", "✓").with_aria_label("Workflow Check")]
  is_decorative=false
  aria_label="Explicit icon label".to_string()
  class_name="docs-icons-state".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Icons"
            slug="icons"
            group="Display"
            description="baseline-compatible `icons` package wrapper that maps medium/large scale and ui/workflow set selection onto IconsUi/IconsWorkflow with stable source-state contracts."
        >
            <Playground title="Medium + Large Set Selection" code_signal=default_code>
                <div class="docs-row">
                    <Icons
                        name="check".to_string()
                        set=IconsSet::Ui
                        scale=IconsScale::Medium
                        tone=IconsTone::Accent
                        is_decorative=false
                    />
                    <Icons
                        name="workflow:warning".to_string()
                        scale=IconsScale::Large
                        tone=IconsTone::Danger
                        is_decorative=false
                    />
                </div>
            </Playground>

            <Playground
                title="Icon Picker Panel"
                code_signal=picker_code
                test_config_signal=picker_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Pick icon"</div>
                        <SegmentedControl
                            id_base="docs-icons-picker".to_string()
                            options=picker_options.clone()
                            selected_index=picker_index
                            set_selected_index=set_picker_index
                            size=SegmentedControlSize::Sm
                            aria_label="Icon picker panel".to_string()
                        />
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Icons
                            name=picker_name.get()
                            set=picker_set.get()
                            scale=picker_scale.get()
                            tone=picker_tone.get()
                            is_decorative=false
                        />
                    </div>
                    <span class="ui-muted">{move || format!("selected: {}", picker_name.get())}</span>
                </div>
            </Playground>

            <Playground title="Custom Workflow Glyph Extension" code_signal=custom_code>
                <div class="docs-row">
                    <Icons
                        name="workflow:deploy".to_string()
                        set=IconsSet::Workflow
                        scale=IconsScale::Large
                        tone=IconsTone::Default
                        glyphs=vec![
                            IconsGlyph::new("workflow:deploy", "🚀")
                                .with_aria_label("Workflow Deploy"),
                        ]
                        is_decorative=false
                        class_name="docs-icons-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect wrapper markers like `data-state`, `data-set`, `data-scale`, `data-set-source`, `data-aria-source`, `data-class-source`, `data-glyph-source`, and `data-tone-source`."
                code_signal=markers_code
            >
                <div class="docs-row">
                    <Icons
                        name="check".to_string()
                        set=IconsSet::Workflow
                        scale=IconsScale::Large
                        tone=IconsTone::Muted
                        glyphs=vec![
                            IconsGlyph::new("workflow:check", "✓")
                                .with_aria_label("Workflow Check"),
                        ]
                        is_decorative=false
                        aria_label="Explicit icon label".to_string()
                        class_name="docs-icons-state".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
