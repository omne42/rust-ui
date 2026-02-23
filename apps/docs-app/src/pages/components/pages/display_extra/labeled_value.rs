use super::*;

pub(crate) fn labeled_value() -> AnyView {
    let orientation_options = vec!["Stacked".to_string(), "Inline".to_string()];
    let tone_options = vec![
        "Default".to_string(),
        "Subtle".to_string(),
        "Strong".to_string(),
    ];
    let labeled_value_imports =
        "use leptos::prelude::*;\nuse ui::{LabeledValue, LabeledValueOrientation, LabeledValueTone};"
            .to_string();
    let (orientation_index, set_orientation_index) = signal(Some(0_usize));
    let (tone_index, set_tone_index) = signal(Some(0_usize));
    let (show_description, set_show_description) = signal(true);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_lang_zh, set_custom_lang_zh) = signal(false);
    let (rtl_dir, set_rtl_dir) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);

    let hello_world_code = Signal::derive(move || {
        r#"<LabeledValue label="Project".to_string() value="Omne".to_string() />"#.to_string()
    });

    let orientation_code = Signal::derive(move || {
        r#"<LabeledValue label="Project".to_string() value="Omne".to_string() />
<LabeledValue
  label="Status".to_string()
  value="Healthy".to_string()
  orientation=LabeledValueOrientation::Inline
  tone=LabeledValueTone::Subtle
/>"#
        .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<LabeledValue label="Project".to_string() value="Omne".to_string() />
<LabeledValue
  label="Status".to_string()
  value="Healthy".to_string()
  orientation=LabeledValueOrientation::Inline
  tone=LabeledValueTone::Subtle
/>
<LabeledValue
  label="SLA".to_string()
  value="99.95%".to_string()
  orientation=LabeledValueOrientation::Stacked
  tone=LabeledValueTone::Strong
  description="SLA snapshot".to_string()
/>"#
        .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<LabeledValue
  label="Build".to_string()
  value="passing".to_string()
  description="Updated 2 minutes ago".to_string()
  aria_label="Build status".to_string()
  class_name="docs-labeled-value-custom".to_string()
  tone=LabeledValueTone::Strong
/>"#
        .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"<LabeledValue label="Default path".to_string() value="No controlled axis".to_string() />
<LabeledValue
  label="App state mapped".to_string()
  value="Map upstream state to orientation/tone/class_name".to_string()
  orientation=LabeledValueOrientation::Inline
  tone=LabeledValueTone::Subtle
/>"#
        .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<LabeledValue
  label="Output mode".to_string()
  value="Snapshot".to_string()
  description="Inspect data-output-mode=snapshot and data-output-status=validated".to_string()
/>"#
        .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<LabeledValue
  label="Build".to_string()
  value="passing".to_string()
  orientation=LabeledValueOrientation::Inline
  tone=LabeledValueTone::Subtle
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let (orientation, orientation_variant) = match orientation_index.get().unwrap_or(0) {
            0 => ("stacked", "LabeledValueOrientation::Stacked"),
            _ => ("inline", "LabeledValueOrientation::Inline"),
        };
        let tone_variant = match tone_index.get().unwrap_or(0) {
            0 => "LabeledValueTone::Default",
            1 => "LabeledValueTone::Subtle",
            _ => "LabeledValueTone::Strong",
        };
        let description_line = if show_description.get() {
            "  description=\"Updated 2 minutes ago\".into()\n"
        } else {
            ""
        };
        let aria_line = if custom_aria.get() {
            "  aria_label=\"Build status\".into()\n"
        } else {
            ""
        };
        let class_line = if custom_class.get() {
            "  class_name=\"docs-labeled-value-workbench\".into()\n"
        } else {
            ""
        };
        let lang_line = if custom_lang_zh.get() {
            "  lang=\"zh-CN\".into()\n"
        } else {
            "  lang=\"en-US\".into()\n"
        };
        let dir_line = if rtl_dir.get() {
            "  dir=A11yDirection::Rtl\n"
        } else {
            "  dir=A11yDirection::Ltr\n"
        };
        let motion_line = if custom_motion.get() {
            "  motion=ui::labeled_value::LabeledValueMotion { enabled: true, duration_ms: 260 }\n"
        } else {
            "  motion=ui::labeled_value::LabeledValueMotion::default()\n"
        };

        format!(
            "<LabeledValue\n  label=\"Build\".into()\n  value=\"passing\".into()\n  orientation={orientation_variant} // {orientation}\n  tone={tone_variant}\n{description_line}{aria_line}{class_line}{lang_line}{dir_line}{motion_line}/>"
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let (orientation, orientation_variant) = match orientation_index.get().unwrap_or(0) {
            0 => ("stacked", "LabeledValueOrientation::Stacked"),
            _ => ("inline", "LabeledValueOrientation::Inline"),
        };
        let tone = match tone_index.get().unwrap_or(0) {
            0 => "default",
            1 => "subtle",
            _ => "strong",
        };
        let tone_variant = match tone_index.get().unwrap_or(0) {
            0 => "LabeledValueTone::Default",
            1 => "LabeledValueTone::Subtle",
            _ => "LabeledValueTone::Strong",
        };
        let dir = if rtl_dir.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        let motion = if custom_motion.get() {
            ui::labeled_value::LabeledValueMotion {
                enabled: true,
                duration_ms: 260,
            }
        } else {
            ui::labeled_value::LabeledValueMotion::default()
        };

        format!(
            "LabeledValueActualConfig {{\n  value: {:?},\n  orientation: {orientation_variant} ({orientation}),\n  tone: {tone_variant} ({tone}),\n  lang: {:?},\n  dir: {:?},\n  motion: {:?},\n  has_description: {},\n  custom_aria_label: {},\n  custom_class_name: {},\n}}",
            "passing",
            if custom_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            },
            dir,
            motion,
            show_description.get(),
            custom_aria.get(),
            custom_class.get(),
        )
    });

    view! {
        <ComponentPage
            title="LabeledValue"
            slug="labeled-value"
            group="Display"
            description="Label-value pair primitive with centralized orientation/tone/source state contracts and baseline-style data markers."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=hello_world_code
                code_imports=labeled_value_imports.clone()
            >
                <LabeledValue label="Project".to_string() value="Omne".to_string() />
            </Playground>

            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。"
                code_signal=workbench_code
                code_imports=labeled_value_imports.clone()
                test_source_path="components/labeled-value/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Orientation"</div>
                        <SegmentedControl
                            id_base="docs-labeled-value-workbench-orientation".to_string()
                            options=orientation_options.clone()
                            selected_index=orientation_index
                            set_selected_index=set_orientation_index
                            size=SegmentedControlSize::Sm
                            aria_label="LabeledValue orientation".to_string()
                        />

                        <div class="docs-search__label">"Tone"</div>
                        <SegmentedControl
                            id_base="docs-labeled-value-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=tone_index
                            set_selected_index=set_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="LabeledValue tone".to_string()
                        />

                        <Switch checked=show_description set_checked=set_show_description>
                            "Description"
                        </Switch>
                        <Switch checked=custom_aria set_checked=set_custom_aria>
                            "Custom aria_label"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=custom_lang_zh set_checked=set_custom_lang_zh>
                            "Lang=zh-CN"
                        </Switch>
                        <Switch checked=rtl_dir set_checked=set_rtl_dir>"dir=rtl"</Switch>
                        <Switch checked=custom_motion set_checked=set_custom_motion>
                            "Custom motion"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let orientation = match orientation_index.get().unwrap_or(0) {
                        0 => LabeledValueOrientation::Stacked,
                        _ => LabeledValueOrientation::Inline,
                    };
                    let tone = match tone_index.get().unwrap_or(0) {
                        0 => LabeledValueTone::Default,
                        1 => LabeledValueTone::Subtle,
                        _ => LabeledValueTone::Strong,
                    };
                    let description = if show_description.get() {
                        "Updated 2 minutes ago".to_string()
                    } else {
                        "".to_string()
                    };
                    let aria_label = if custom_aria.get() {
                        "Build status".to_string()
                    } else {
                        "".to_string()
                    };
                    let class_name = if custom_class.get() {
                        "docs-labeled-value-workbench".to_string()
                    } else {
                        "".to_string()
                    };
                    let lang = if custom_lang_zh.get() {
                        "zh-CN".to_string()
                    } else {
                        "en-US".to_string()
                    };
                    let dir = if rtl_dir.get() {
                        A11yDirection::Rtl
                    } else {
                        A11yDirection::Ltr
                    };
                    let motion = if custom_motion.get() {
                        ui::labeled_value::LabeledValueMotion {
                            enabled: true,
                            duration_ms: 260,
                        }
                    } else {
                        ui::labeled_value::LabeledValueMotion::default()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <LabeledValue
                                label="Build".to_string()
                                value="passing".to_string()
                                description=description
                                orientation=orientation
                                tone=tone
                                aria_label=aria_label
                                class_name=class_name
                                lang=lang.clone()
                                dir=dir
                                motion=motion
                            />
                            <LabeledValue
                                label="Compare / Inline".to_string()
                                value="Healthy".to_string()
                                orientation=LabeledValueOrientation::Inline
                                tone=LabeledValueTone::Subtle
                                lang=lang.clone()
                                dir=dir
                                motion=motion
                            />
                            <LabeledValue
                                label="Compare / Stacked".to_string()
                                value="99.95%".to_string()
                                orientation=LabeledValueOrientation::Stacked
                                tone=LabeledValueTone::Strong
                                description="SLA snapshot".to_string()
                                lang=lang
                                dir=dir
                                motion=motion
                            />
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix"
                code_signal=state_matrix_code
                code_imports=labeled_value_imports.clone()
            >
                <div class="docs-stack">
                    <LabeledValue label="Project".to_string() value="Omne".to_string() />
                    <LabeledValue
                        label="Status".to_string()
                        value="Healthy".to_string()
                        orientation=LabeledValueOrientation::Inline
                        tone=LabeledValueTone::Subtle
                    />
                    <LabeledValue
                        label="SLA".to_string()
                        value="99.95%".to_string()
                        orientation=LabeledValueOrientation::Stacked
                        tone=LabeledValueTone::Strong
                        description="SLA snapshot".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Orientation + Tone" code_signal=orientation_code>
                <div class="docs-stack">
                    <LabeledValue label="Project".to_string() value="Omne".to_string() />
                    <LabeledValue
                        label="Status".to_string()
                        value="Healthy".to_string()
                        orientation=LabeledValueOrientation::Inline
                        tone=LabeledValueTone::Subtle
                    />
                </div>
            </Playground>

            <Playground title="Description + Custom Aria/Class" code_signal=custom_code>
                <div class="docs-stack">
                    <LabeledValue
                        label="Build".to_string()
                        value="passing".to_string()
                        description="Updated 2 minutes ago".to_string()
                        aria_label="Build status".to_string()
                        class_name="docs-labeled-value-custom".to_string()
                        tone=LabeledValueTone::Strong
                    />
                    <LabeledValue
                        label="SLA".to_string()
                        value="99.95%".to_string()
                        orientation=LabeledValueOrientation::Inline
                        tone=LabeledValueTone::Default
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="LabeledValue has no controlled/uncontrolled runtime axis; compare default usage vs app-state mapped props."
                code_signal=controlled_contrast_code
                code_imports=labeled_value_imports.clone()
            >
                <div class="docs-stack">
                    <LabeledValue
                        label="Default path".to_string()
                        value="No controlled axis".to_string()
                    />
                    <LabeledValue
                        label="App state mapped".to_string()
                        value="Map upstream state to orientation/tone/class_name".to_string()
                        orientation=LabeledValueOrientation::Inline
                        tone=LabeledValueTone::Subtle
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="LabeledValue is a display leaf: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports=labeled_value_imports.clone()
            >
                <div class="docs-stack">
                    <LabeledValue
                        label="Output mode".to_string()
                        value="Snapshot".to_string()
                        description="Inspect data-output-mode=snapshot and data-output-status=validated".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Reference Gallery (Tone / Compact / Bordered)"
                code_signal=state_matrix_code
                code_imports=labeled_value_imports.clone()
            >
                <div class="docs-stack">
                    <EmptyState />
                    <EmptyState
                        title="Nothing matched".to_string()
                        description="Try a different query or clear filters.".to_string()
                        tone=EmptyStateTone::Muted
                        align=EmptyStateAlign::Center
                        motion=ui::empty_state::EmptyStateMotion::default()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <EmptyState
                        title="Deployments paused".to_string()
                        description="Approvals are required before resuming this environment.".to_string()
                        tone=EmptyStateTone::Accent
                        is_compact=true
                        is_bordered=true
                        aria_label="Deployments paused".to_string()
                        class_name="docs-empty-state-custom".to_string()
                        motion=ui::empty_state::EmptyStateMotion::default()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=labeled_value_imports.clone()
            >
                <LabeledValue
                    label="Build".to_string()
                    value="passing".to_string()
                    orientation=LabeledValueOrientation::Inline
                    tone=LabeledValueTone::Subtle
                />
            </Playground>

            <Playground
                title="State Matrix (Tone / Orientation / Locale Comparison)"
                code_signal=state_matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{LabeledValue, LabeledValueOrientation, LabeledValueTone};".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <LabeledValue
                        label="Project".to_string()
                        value="Omne".to_string()
                        motion=ui::labeled_value::LabeledValueMotion::default()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                    <LabeledValue
                        label="Status".to_string()
                        value="Healthy".to_string()
                        orientation=LabeledValueOrientation::Inline
                        tone=LabeledValueTone::Subtle
                        motion=ui::labeled_value::LabeledValueMotion::default()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                    />
                    <LabeledValue
                        label="SLA".to_string()
                        value="99.95%".to_string()
                        orientation=LabeledValueOrientation::Stacked
                        tone=LabeledValueTone::Strong
                        description="SLA snapshot".to_string()
                        motion=ui::labeled_value::LabeledValueMotion::default()
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="labeled-value-streaming-modes">
                <h3>"Streaming / Snapshot"</h3>
                <ul data-slot="labeled-value-streaming-rows">
                    <li><code>"data-output-mode"</code>" = snapshot"</li>
                    <li><code>"data-output-status"</code>" = validated"</li>
                    <li><code>"streaming support"</code>" = optional (fallback=snapshot)"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="labeled-value-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}
