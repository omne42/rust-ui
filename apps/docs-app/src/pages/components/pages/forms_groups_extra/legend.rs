use super::*;

pub(crate) fn legend() -> AnyView {
    let (controlled_required, set_controlled_required) = signal(true);
    let text_options = vec![
        "Notification settings".to_string(),
        "Billing preferences".to_string(),
    ];
    let tone_options = vec![
        "Default".to_string(),
        "Muted".to_string(),
        "Strong".to_string(),
    ];
    let (workbench_text_index, set_workbench_text_index) = signal(Some(0_usize));
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let (workbench_required, set_workbench_required) = signal(true);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_indicator, set_workbench_custom_indicator) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);

    let workbench_text = Signal::derive(move || {
        if workbench_text_index.get().unwrap_or(0) == 1 {
            "Billing preferences".to_string()
        } else {
            "Notification settings".to_string()
        }
    });
    let workbench_tone = Signal::derive(move || match workbench_tone_index.get().unwrap_or(0) {
        1 => LegendTone::Muted,
        2 => LegendTone::Strong,
        _ => LegendTone::Default,
    });
    let workbench_dir = Signal::derive(move || {
        if workbench_rtl_dir.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            ui::legend::LegendMotion {
                duration_ms: 320.0,
                ..ui::legend::LegendMotion::default()
            }
        } else {
            ui::legend::LegendMotion::default()
        }
    });

    let hello_code = Signal::derive(move || {
        r#"use ui::Legend;

<fieldset class=\"docs-stack\"> 
  <Legend text=\"Notification settings\".into() />
</fieldset>"#
            .to_string()
    });

    let required_code = Signal::derive(move || {
        r#"use ui::Legend;

<fieldset class=\"docs-stack\"> 
  <Legend
    text=\"Notification settings\".into()
    is_required=true
  />
</fieldset>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"use ui::{Legend, LegendTone};

<fieldset class=\"docs-stack\"> 
  <Legend
    text=\"Billing preferences\".into()
    tone=LegendTone::Muted
    is_required=true
    required_indicator=\"(required)\".into()
    class_name=\"docs-legend-custom\".into()
  />
  <Legend
    text=\"Read-only group\".into()
    tone=LegendTone::Strong
    is_disabled=true
  />
</fieldset>"#
            .to_string()
    });

    let controlled_code = Signal::derive(move || {
        let is_required = controlled_required.get();
        format!(
            "use ui::{{Legend, LegendTone, Switch}};\n\nlet (is_required, set_is_required) = signal({is_required});\n\n<fieldset class=\"docs-stack\">\n  <Legend text=\"Notification settings\".into() />\n  <Legend\n    text=\"Notification settings\".into()\n    tone=LegendTone::Muted\n    is_required=is_required\n  />\n  <Switch checked=is_required set_checked=set_is_required>\n    \"Controlled required\"\n  </Switch>\n</fieldset>"
        )
    });
    let workbench_code = Signal::derive(move || {
        let text = workbench_text.get();
        let tone = workbench_tone.get();
        let required = workbench_required.get();
        let disabled = workbench_disabled.get();
        let dir = workbench_dir.get();
        let custom_indicator = workbench_custom_indicator.get();
        let custom_class = workbench_custom_class.get();
        let motion = workbench_motion.get();
        let lang = if workbench_rtl_dir.get() {
            "ar"
        } else {
            "en-US"
        };
        format!(
            "use ui::{{A11yDirection, Legend, LegendTone}};\n\n<fieldset class=\"docs-stack\">\n  <Legend\n    text={text:?}.to_string()\n    tone=LegendTone::{tone:?}\n    is_required={required}\n    is_disabled={disabled}\n    motion=ui::legend::LegendMotion {{ duration_ms: {duration_ms}, ..ui::legend::LegendMotion::default() }}\n    required_indicator={required_indicator}\n    class_name={class_name}\n    lang={lang:?}.to_string()\n    dir=A11yDirection::{dir:?}\n  />\n</fieldset>",
            duration_ms = motion.duration_ms,
            required_indicator = if custom_indicator {
                "\"(required)\".to_string()"
            } else {
                "String::new()"
            },
            class_name = if custom_class {
                "\"docs-legend-custom\".to_string()"
            } else {
                "String::new()"
            },
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        let motion = workbench_motion.get();
        format!(
            "LegendActualConfig {{\n  text: Some({text:?}),\n  tone: {tone:?},\n  is_required: Some({is_required}),\n  is_disabled: Some({is_disabled}),\n  motion: LegendMotion {{ duration_ms: {duration_ms}, spring: \"spring_soft\" }},\n  required_indicator: {required_indicator},\n  class_name: {class_name},\n  lang: Some({lang:?}),\n  dir: Some(A11yDirection::{dir:?}),\n}}",
            text = workbench_text.get(),
            tone = workbench_tone.get(),
            is_required = workbench_required.get(),
            is_disabled = workbench_disabled.get(),
            duration_ms = motion.duration_ms,
            required_indicator = if workbench_custom_indicator.get() {
                "Some(\"(required)\")"
            } else {
                "None"
            },
            class_name = if workbench_custom_class.get() {
                "Some(\"docs-legend-custom\")"
            } else {
                "None"
            },
            lang = if workbench_rtl_dir.get() {
                "ar"
            } else {
                "en-US"
            },
            dir = workbench_dir.get(),
        )
    });
    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui/src/legend/styles.rs */\n{}",
            ui::legend::styles::CSS
        )
    });

    view! {
        <ComponentPage
            title="Legend"
            slug="legend"
            group="Forms"
            description="baseline-compatible fieldset legend primitive with centralized tone/required/disabled contracts and stable slot/data-state markers."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <fieldset class="docs-stack">
                    <Legend text="Notification settings".to_string() />
                    <div class="ui-muted">"Default path: only pass text; no state wiring required."</div>
                </fieldset>
            </Playground>

            <Playground
                title="Legend Workbench (Display + Config + Code + CSS Test)"
                description="Adjust every Legend API field and inspect live actual config."
                code_signal=workbench_code
                code_imports=LEGEND_DOC_IMPORTS.to_string()
                test_css_source=workbench_test_css
                test_source_path="crates/ui/src/legend/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Text"</div>
                        <SegmentedControl
                            id_base="docs-legend-workbench-text".to_string()
                            options=text_options.clone()
                            selected_index=workbench_text_index
                            set_selected_index=set_workbench_text_index
                            size=SegmentedControlSize::Sm
                            aria_label="Legend text".to_string()
                        />
                        <div class="docs-search__label">"Tone"</div>
                        <SegmentedControl
                            id_base="docs-legend-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=workbench_tone_index
                            set_selected_index=set_workbench_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="Legend tone".to_string()
                        />
                        <Switch checked=workbench_required set_checked=set_workbench_required>
                            "Required"
                        </Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch
                            checked=workbench_custom_indicator
                            set_checked=set_workbench_custom_indicator
                        >
                            "Custom indicator"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "RTL direction"
                        </Switch>
                        <Switch
                            checked=workbench_custom_motion
                            set_checked=set_workbench_custom_motion
                        >
                            "Custom motion"
                        </Switch>
                    </div>
                }
            >
                <fieldset class="docs-stack">
                    <Legend
                        text=workbench_text.get()
                        tone=workbench_tone.get()
                        is_required=workbench_required.get()
                        is_disabled=workbench_disabled.get()
                        motion=workbench_motion.get()
                        required_indicator=if workbench_custom_indicator.get() {
                            "(required)".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-legend-custom".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_rtl_dir.get() {
                            "ar".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=workbench_dir.get()
                    />
                    <div class="ui-muted">
                        "required: " {move || workbench_required.get()}
                        " · disabled: " {move || workbench_disabled.get()}
                    </div>
                </fieldset>
            </Playground>

            <Playground title="Required Legend" code_signal=required_code>
                <fieldset class="docs-stack">
                    <Legend text="Notification settings".to_string() is_required=true />
                    <div class="ui-muted">
                        "Legend stays semantic inside fieldset and exposes required marker contracts."
                    </div>
                </fieldset>
            </Playground>

            <Playground title="Tone + Custom Indicator + Disabled" code_signal=states_code>
                <fieldset class="docs-stack">
                    <Legend
                        text="Billing preferences".to_string()
                        tone=LegendTone::Muted
                        is_required=true
                        required_indicator="(required)".to_string()
                        class_name="docs-legend-custom".to_string()
                    />

                    <Legend
                        text="Read-only group".to_string()
                        tone=LegendTone::Strong
                        is_disabled=true
                    />
                </fieldset>
            </Playground>

            <Playground title="Controlled vs Default (Comparison)" code_signal=controlled_code>
                <fieldset class="docs-stack">
                    <Legend text="Notification settings".to_string() />
                    <Legend
                        text="Notification settings".to_string()
                        tone=LegendTone::Muted
                        is_required=controlled_required.get()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                        motion=ui::legend::LegendMotion::default()
                    />
                    <Switch checked=controlled_required set_checked=set_controlled_required>
                        "Controlled required (parent signal)"
                    </Switch>
                    <div class="ui-muted">
                        "Uncontrolled path keeps default props; controlled path keeps parent signal as source of truth."
                    </div>
                </fieldset>
            </Playground>

            <p class="ui-muted" data-slot="legend-streaming-policy">
                "Streaming Optional; fallback=snapshot."
            </p>
            <p class="ui-muted" data-slot="legend-streaming-modes">
                "Snapshot mode renders verified full output for legend semantics."
            </p>
            <p class="ui-muted" data-slot="legend-copy-ready">
                "Copy-ready snippets prepend imports automatically: use ui::{Legend, LegendTone, Switch}; source: apps/docs-app/src/pages/components/pages/forms_groups_extra.rs."
            </p>
            <p class="ui-muted" data-slot="legend-source-paths">
                "Source paths: components/legend/src/mod.rs, components/legend/src/logic.rs, components/legend/src/view.rs, components/legend/src/styles.rs, components/legend/src/motion.rs."
            </p>
            <p class="ui-muted" data-slot="legend-source-prerequisites">
                "Feature prerequisites: component-legend (inject-css optional for runtime style injection)."
            </p>
        </ComponentPage>
    }
    .into_any()
}
