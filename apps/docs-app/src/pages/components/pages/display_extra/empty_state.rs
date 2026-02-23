use super::*;

pub(crate) fn empty_state() -> AnyView {
    let empty_state_imports =
        "use leptos::prelude::*;\nuse ui::{Button, ButtonVariant, EmptyState, EmptyStateAlign, EmptyStateTone};"
            .to_string();
    let hello_code = Signal::derive(move || r#"<EmptyState />"#.to_string());

    let state_matrix_code = Signal::derive(move || {
        r#"<EmptyState />
<EmptyState
  title="Nothing matched".to_string()
  description="Try a different query or clear filters.".to_string()
  tone=EmptyStateTone::Muted
  align=EmptyStateAlign::Center
/>
<EmptyState
  title="Deployments paused".to_string()
  description="Approvals are required before resuming this environment.".to_string()
  tone=EmptyStateTone::Accent
  is_compact=true
  is_bordered=true
/>"#
        .to_string()
    });

    let tone_code = Signal::derive(move || {
        r#"<EmptyState
  title="No projects yet".to_string()
  description="Create your first project to unlock dashboards and team workflows.".to_string()
  tone=EmptyStateTone::Default
  icon=move || view! { <span>"📁"</span> }
  actions=move || view! {
    <ui::Button>"Create project"</ui::Button>
  }
/>
<EmptyState
  title="Nothing matched".to_string()
  description="Try a different query or clear filters.".to_string()
  tone=EmptyStateTone::Muted
  align=EmptyStateAlign::Center
/>"#
        .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"<EmptyState
  title="Deployments paused".to_string()
  description="Approvals are required before resuming this environment.".to_string()
  tone=EmptyStateTone::Accent
  is_compact=true
  is_bordered=true
  class_name="docs-empty-state-custom".to_string()
  icon=move || view! { <span>"⏸"</span> }
  actions=move || view! {
    <ui::Button variant=ui::ButtonVariant::Secondary>
      "Review approvals"
    </ui::Button>
  }
/>"#
        .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"<EmptyState />
<EmptyState
  title="Mapped from parent state".to_string()
  description="EmptyState has no controlled/uncontrolled axis; parent can still map app state into props.".to_string()
  tone=EmptyStateTone::Muted
/>"#
        .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<EmptyState
  title="Snapshot baseline".to_string()
  description="Component default path renders complete config in one pass.".to_string()
/>
<EmptyState
  title="Streaming optional fallback".to_string()
  description="Not an LLM body reader surface: optional streaming contracts fallback to snapshot.".to_string()
  tone=EmptyStateTone::Muted
/>"#
        .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<EmptyState
  title="No incidents".to_string()
  description="Everything is healthy. If this changes, add actions below.".to_string()
  tone=EmptyStateTone::Default
/>"#
        .to_string()
    });

    let tone_options = vec![
        "Default".to_string(),
        "Muted".to_string(),
        "Accent".to_string(),
    ];
    let align_options = vec!["Start".to_string(), "Center".to_string()];
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let (workbench_align_index, set_workbench_align_index) = signal(Some(0_usize));
    let (workbench_title, set_workbench_title) = signal("No incidents".to_string());
    let (workbench_description, set_workbench_description) =
        signal("Everything is healthy. If this changes, add actions below.".to_string());
    let (workbench_is_compact, set_workbench_is_compact) = signal(false);
    let (workbench_is_bordered, set_workbench_is_bordered) = signal(false);
    let (workbench_with_icon, set_workbench_with_icon) = signal(false);
    let (workbench_with_actions, set_workbench_with_actions) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_code = Signal::derive(move || {
        let tone_index = workbench_tone_index.get().unwrap_or(0);
        let tone_variant = match tone_index {
            1 => "EmptyStateTone::Muted",
            2 => "EmptyStateTone::Accent",
            _ => "EmptyStateTone::Default",
        };
        let align_index = workbench_align_index.get().unwrap_or(0);
        let align_variant = match align_index {
            1 => "EmptyStateAlign::Center",
            _ => "EmptyStateAlign::Start",
        };
        let title = workbench_title
            .get()
            .replace('\\', "\\\\")
            .replace('"', "\\\"");
        let description = workbench_description
            .get()
            .replace('\\', "\\\\")
            .replace('"', "\\\"");

        let mut lines = vec![
            "<EmptyState".to_string(),
            format!("  title=\"{title}\".to_string()"),
            format!("  description=\"{description}\".to_string()"),
            format!("  tone={tone_variant}"),
            format!("  align={align_variant}"),
        ];

        if workbench_is_compact.get() {
            lines.push("  is_compact=true".to_string());
        }
        if workbench_is_bordered.get() {
            lines.push("  is_bordered=true".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-empty-state-workbench\".to_string()".to_string());
        }
        if workbench_with_icon.get() {
            lines.push("  icon=move || view! { <span>\"🧭\"</span> }".to_string());
        }
        if workbench_with_actions.get() {
            lines.push("  actions=move || view! {".to_string());
            lines.push("    <ui::Button variant=ui::ButtonVariant::Secondary>".to_string());
            lines.push("      \"Retry\"".to_string());
            lines.push("    </ui::Button>".to_string());
            lines.push("  }".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let tone_variant = match workbench_tone_index.get().unwrap_or(0) {
            1 => "Muted",
            2 => "Accent",
            _ => "Default",
        };
        let align_variant = match workbench_align_index.get().unwrap_or(0) {
            1 => "Center",
            _ => "Start",
        };
        format!(
            "EmptyStateActualConfig {{\n  tone: {tone_variant},\n  align: {align_variant},\n  is_compact: {},\n  is_bordered: {},\n  with_icon: {},\n  with_actions: {},\n  aria_label: {:?},\n  class_name: {:?},\n  motion: {:?},\n  lang: {:?},\n  dir: {:?},\n  custom_class: {},\n  title: \"{}\",\n  description: \"{}\",\n  marker_expectations: [\"data-tone\", \"data-align\", \"data-state\", \"data-icon\", \"data-actions\", \"data-title-source\", \"data-description-source\"],\n}}",
            workbench_is_compact.get(),
            workbench_is_bordered.get(),
            workbench_with_icon.get(),
            workbench_with_actions.get(),
            "Empty state region",
            if workbench_custom_class.get() {
                "docs-empty-state-workbench"
            } else {
                ""
            },
            ui::empty_state::EmptyStateMotion::default(),
            "en-US",
            A11yDirection::Ltr,
            workbench_custom_class.get(),
            workbench_title.get(),
            workbench_description.get(),
        )
    });

    view! {
        <ComponentPage
            title="EmptyState"
            slug="empty-state"
            group="Display"
            description="baseline-style empty-state primitive with centralized tone/align/layout/source contracts and stable slot/data markers."
        >
            <Playground
                title="Hello World (Default Path)"
                code_signal=hello_code
                code_imports=empty_state_imports.clone()
            >
                <EmptyState />
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Interactive acceptance canvas: tune props/state and verify semantic markers in real time."
                code_signal=workbench_code
                code_imports=empty_state_imports.clone()
                test_source_path="components/empty-state/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="empty-state-workbench-controls">
                            <label class="docs-search__label" data-slot="empty-state-workbench-title">
                                "Title"
                                <input
                                    type="text"
                                    prop:value=move || workbench_title.get()
                                    on:input=move |ev| set_workbench_title.set(event_target_value(&ev))
                                />
                            </label>
                            <label class="docs-search__label" data-slot="empty-state-workbench-description">
                                "Description"
                                <input
                                    type="text"
                                    prop:value=move || workbench_description.get()
                                    on:input=move |ev| set_workbench_description.set(event_target_value(&ev))
                                />
                            </label>

                            <div data-slot="empty-state-workbench-tone">
                                <div class="docs-search__label">"Tone"</div>
                                <SegmentedControl
                                    id_base="docs-empty-state-workbench-tone".to_string()
                                    options=tone_options.clone()
                                    selected_index=workbench_tone_index
                                    set_selected_index=set_workbench_tone_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="EmptyState tone".to_string()
                                />
                            </div>

                            <div data-slot="empty-state-workbench-align">
                                <div class="docs-search__label">"Align"</div>
                                <SegmentedControl
                                    id_base="docs-empty-state-workbench-align".to_string()
                                    options=align_options.clone()
                                    selected_index=workbench_align_index
                                    set_selected_index=set_workbench_align_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="EmptyState align".to_string()
                                />
                            </div>

                            <div data-slot="empty-state-workbench-toggle-compact">
                                <Switch checked=workbench_is_compact set_checked=set_workbench_is_compact>
                                    "Compact"
                                </Switch>
                            </div>
                            <div data-slot="empty-state-workbench-toggle-bordered">
                                <Switch checked=workbench_is_bordered set_checked=set_workbench_is_bordered>
                                    "Bordered"
                                </Switch>
                            </div>
                            <div data-slot="empty-state-workbench-toggle-icon">
                                <Switch checked=workbench_with_icon set_checked=set_workbench_with_icon>
                                    "Icon"
                                </Switch>
                            </div>
                            <div data-slot="empty-state-workbench-toggle-actions">
                                <Switch checked=workbench_with_actions set_checked=set_workbench_with_actions>
                                    "Actions"
                                </Switch>
                            </div>
                            <div data-slot="empty-state-workbench-toggle-class">
                                <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                    "Custom class"
                                </Switch>
                            </div>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="empty-state-workbench">
                    {move || {
                        let tone = match workbench_tone_index.get().unwrap_or(0) {
                            1 => EmptyStateTone::Muted,
                            2 => EmptyStateTone::Accent,
                            _ => EmptyStateTone::Default,
                        };
                        let align = match workbench_align_index.get().unwrap_or(0) {
                            1 => EmptyStateAlign::Center,
                            _ => EmptyStateAlign::Start,
                        };
                        let class_name = if workbench_custom_class.get() {
                            "docs-empty-state-workbench".to_string()
                        } else {
                            String::new()
                        };
                        let title = workbench_title.get();
                        let description = workbench_description.get();
                        let is_compact = workbench_is_compact.get();
                        let is_bordered = workbench_is_bordered.get();
                        let aria_label = "Empty state region".to_string();
                        let motion = ui::empty_state::EmptyStateMotion::default();
                        let lang = "en-US".to_string();
                        let dir = A11yDirection::Ltr;

                        if workbench_with_icon.get() && workbench_with_actions.get() {
                            view! {
                                <EmptyState
                                    title=title
                                    description=description
                                    tone=tone
                                    align=align
                                    is_compact=is_compact
                                    is_bordered=is_bordered
                                    aria_label=aria_label.clone()
                                    class_name=class_name
                                    motion=motion
                                    lang=lang.clone()
                                    dir=dir
                                    icon=move || view! { <span>"🧭"</span> }
                                    actions=move || {
                                        view! {
                                            <ui::Button variant=ui::ButtonVariant::Secondary>
                                                "Retry"
                                            </ui::Button>
                                        }
                                    }
                                />
                            }
                                .into_any()
                        } else if workbench_with_icon.get() {
                            view! {
                                <EmptyState
                                    title=title
                                    description=description
                                    tone=tone
                                    align=align
                                    is_compact=is_compact
                                    is_bordered=is_bordered
                                    aria_label=aria_label.clone()
                                    class_name=class_name
                                    motion=motion
                                    lang=lang.clone()
                                    dir=dir
                                    icon=move || view! { <span>"🧭"</span> }
                                />
                            }
                                .into_any()
                        } else if workbench_with_actions.get() {
                            view! {
                                <EmptyState
                                    title=title
                                    description=description
                                    tone=tone
                                    align=align
                                    is_compact=is_compact
                                    is_bordered=is_bordered
                                    aria_label=aria_label.clone()
                                    class_name=class_name
                                    motion=motion
                                    lang=lang.clone()
                                    dir=dir
                                    actions=move || {
                                        view! {
                                            <ui::Button variant=ui::ButtonVariant::Secondary>
                                                "Retry"
                                            </ui::Button>
                                        }
                                    }
                                />
                            }
                                .into_any()
                        } else {
                            view! {
                                <EmptyState
                                    title=title
                                    description=description
                                    tone=tone
                                    align=align
                                    is_compact=is_compact
                                    is_bordered=is_bordered
                                    aria_label=aria_label
                                    class_name=class_name
                                    motion=motion
                                    lang=lang
                                    dir=dir
                                />
                            }
                                .into_any()
                        }
                    }}
                </div>
            </Playground>

            <Playground
                title="State Matrix"
                code_signal=state_matrix_code
                code_imports=empty_state_imports.clone()
            >
                <div class="docs-stack">
                    <EmptyState />
                    <EmptyState
                        title="Nothing matched".to_string()
                        description="Try a different query or clear filters.".to_string()
                        tone=EmptyStateTone::Muted
                        align=EmptyStateAlign::Center
                    />
                    <EmptyState
                        title="Deployments paused".to_string()
                        description="Approvals are required before resuming this environment.".to_string()
                        tone=EmptyStateTone::Accent
                        is_compact=true
                        is_bordered=true
                    />
                </div>
            </Playground>

            <Playground
                title="Tone + Alignment + Actions"
                code_signal=tone_code
                code_imports=empty_state_imports.clone()
            >
                <div class="docs-stack">
                    <EmptyState
                        title="No projects yet".to_string()
                        description="Create your first project to unlock dashboards and team workflows.".to_string()
                        tone=EmptyStateTone::Default
                        icon=move || view! { <span>"📁"</span> }
                        actions=move || {
                            view! {
                                <ui::Button>
                                    "Create project"
                                </ui::Button>
                            }
                        }
                    />
                    <EmptyState
                        title="Nothing matched".to_string()
                        description="Try a different query or clear filters.".to_string()
                        tone=EmptyStateTone::Muted
                        align=EmptyStateAlign::Center
                    />
                </div>
            </Playground>

            <Playground
                title="Compact + Bordered + Custom Class"
                code_signal=state_code
                code_imports=empty_state_imports.clone()
            >
                <EmptyState
                    title="Deployments paused".to_string()
                    description="Approvals are required before resuming this environment.".to_string()
                    tone=EmptyStateTone::Accent
                    is_compact=true
                    is_bordered=true
                    class_name="docs-empty-state-custom".to_string()
                    icon=move || view! { <span>"⏸"</span> }
                    actions=move || {
                        view! {
                            <ui::Button variant=ui::ButtonVariant::Secondary>
                                "Review approvals"
                            </ui::Button>
                        }
                    }
                />
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="EmptyState has no controlled/uncontrolled runtime axis; compare default usage vs app-state mapped props."
                code_signal=controlled_contrast_code
                code_imports=empty_state_imports.clone()
            >
                <div class="docs-stack">
                    <EmptyState />
                    <EmptyState
                        title="Mapped from parent state".to_string()
                        description="EmptyState has no controlled/uncontrolled axis; parent can still map app state into props.".to_string()
                        tone=EmptyStateTone::Muted
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="EmptyState is not an LLM reader surface: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports=empty_state_imports.clone()
            >
                <div class="docs-stack">
                    <EmptyState
                        title="Snapshot baseline".to_string()
                        description="Component default path renders complete config in one pass.".to_string()
                    />
                    <EmptyState
                        title="Streaming optional fallback".to_string()
                        description="Not an LLM body reader surface: optional streaming contracts fallback to snapshot.".to_string()
                        tone=EmptyStateTone::Muted
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Tone / Align / Compact Comparison)"
                code_signal=state_matrix_code
                code_imports=empty_state_imports.clone()
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
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=empty_state_imports
            >
                <EmptyState
                    title="No incidents".to_string()
                    description="Everything is healthy. If this changes, add actions below.".to_string()
                    tone=EmptyStateTone::Default
                />
            </Playground>

            <section class="docs-card docs-prose" data-slot="empty-state-source-first-contract">
                <h3>"Source-first / Copy-Paste Ready Contract"</h3>
                <p>
                    "Open "
                    <code>"Show code"</code>
                    " in any playground, then use the code block copy action. Copied snippets are auto-normalized by "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    " so required imports are included."
                </p>
                <p>"Real component sources:"</p>
                <ul data-slot="empty-state-source-first-paths">
                    <li><code>"components/empty-state/src/mod.rs"</code></li>
                    <li><code>"components/empty-state/src/logic.rs"</code></li>
                    <li><code>"components/empty-state/src/view.rs"</code></li>
                    <li><code>"components/empty-state/src/styles.rs"</code></li>
                    <li><code>"components/empty-state/src/motion.rs"</code></li>
                </ul>
                <p>"Dependency baseline (Cargo.toml):"</p>
                <pre data-slot="empty-state-source-first-deps">
                    <code>
                        "[dependencies]\nui = { default-features = false, features = [\"component-empty_state\", \"inject-css\"] }\n# Mount under UiRoot to inject base/theme/components CSS."
                    </code>
                </pre>
            </section>
        </ComponentPage>
    }
    .into_any()
}
