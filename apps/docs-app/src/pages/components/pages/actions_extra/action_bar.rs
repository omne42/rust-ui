use super::*;

pub(crate) fn action_bar() -> AnyView {
    // Keep the first playground visible by default so docs + E2E coverage can assert presence.
    let (selected_count, set_selected_count) = signal(2_usize);
    let selected_count_signal = Signal::derive(move || selected_count.get());
    let on_selected_count_change = Callback::new(move |next: usize| set_selected_count.set(next));

    let clear_selection = Callback::new(move |_| set_selected_count.set(0));
    let action_bar_code_imports =
        "use leptos::prelude::*;\nuse ui::{ActionBar, ActionBarMotion, ActionBarPosition, ActionButton};"
            .to_string();
    let interactive_position_options = vec!["Bottom".to_string(), "Top".to_string()];
    let (interactive_selected_count, set_interactive_selected_count) = signal(2_usize);
    let interactive_selected_count_signal =
        Signal::derive(move || interactive_selected_count.get());
    let interactive_on_selected_count_change =
        Callback::new(move |next: usize| set_interactive_selected_count.set(next));
    let interactive_on_clear_selection =
        Callback::new(move |_| set_interactive_selected_count.set(0));
    let (interactive_position_index, set_interactive_position_index) = signal(Some(0_usize));
    let interactive_position = Signal::derive(move || {
        if interactive_position_index.get().unwrap_or(0) == 1 {
            ActionBarPosition::Top
        } else {
            ActionBarPosition::Bottom
        }
    });
    let (interactive_force_visible, set_interactive_force_visible) = signal(false);
    let (interactive_with_clear_action, set_interactive_with_clear_action) = signal(true);
    let (interactive_custom_labels, set_interactive_custom_labels) = signal(false);
    let (interactive_reduced_motion, set_interactive_reduced_motion) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<ActionBar default_selected_count=1>
  <ActionButton>"Archive"</ActionButton>
</ActionBar>"#
            .to_string()
    });

    let code = Signal::derive(move || {
        let selected_count = selected_count.get();

        [
            "<ActionBar".to_string(),
            format!("  selected_count=Signal::derive(move || {selected_count}_usize)"),
            "  on_selected_count_change=Callback::new(move |next: usize| { drop(next); })"
                .to_string(),
            "  on_clear_selection=Callback::new(move |_| {})".to_string(),
            "  aria_label=\"Bulk actions\".into()".to_string(),
            "  class_name=\"docs-action-bar\".into()".to_string(),
            ">".to_string(),
            "  <ActionButton>\"Delete\"</ActionButton>".to_string(),
            "  <ActionButton is_quiet=true>\"Archive\"</ActionButton>".to_string(),
            "</ActionBar>".to_string(),
        ]
        .join("\n")
    });

    let control_mode_code = Signal::derive(move || {
        let selected_count = selected_count.get();

        [
            "// Controlled".to_string(),
            "<ActionBar".to_string(),
            format!("  selected_count=Signal::derive(move || {selected_count}_usize)"),
            "  on_selected_count_change=Callback::new(move |_next: usize| {})".to_string(),
            "  on_clear_selection=Callback::new(move |_| {})".to_string(),
            ">".to_string(),
            "  <ActionButton>\"Delete\"</ActionButton>".to_string(),
            "  <ActionButton is_quiet=true>\"Archive\"</ActionButton>".to_string(),
            "</ActionBar>".to_string(),
            "".to_string(),
            "// Uncontrolled".to_string(),
            "<ActionBar default_selected_count=2>".to_string(),
            "  <ActionButton>\"Tag\"</ActionButton>".to_string(),
            "  <ActionButton is_quiet=true>\"Assign\"</ActionButton>".to_string(),
            "</ActionBar>".to_string(),
        ]
        .join("\n")
    });

    let state_code = Signal::derive(move || {
        [
            "<ActionBar".to_string(),
            "  default_selected_count=5".to_string(),
            "  position=ActionBarPosition::Top".to_string(),
            "  is_force_visible=true".to_string(),
            "  selection_text=\"Rows selected\".into()".to_string(),
            "  clear_label=\"Clear all\".into()".to_string(),
            "  motion=ActionBarMotion::disabled()".to_string(),
            ">".to_string(),
            "  <ActionButton is_quiet=true>\"Tag\"</ActionButton>".to_string(),
            "  <ActionButton is_quiet=true>\"Assign\"</ActionButton>".to_string(),
            "</ActionBar>".to_string(),
        ]
        .join(
            "
",
        )
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<ActionBar default_selected_count=0>
  <ActionButton is_quiet=true>"Hidden when empty"</ActionButton>
</ActionBar>
<ActionBar default_selected_count=1>
  <ActionButton>"Bottom / single"</ActionButton>
</ActionBar>
<ActionBar default_selected_count=4 position=ActionBarPosition::Top is_force_visible=true>
  <ActionButton>"Top / forced visible"</ActionButton>
</ActionBar>"#
            .to_string()
    });

    let motion_code = Signal::derive(move || {
        let selected_count = selected_count.get();

        vec![
            "<ActionBar".to_string(),
            format!("  selected_count=Signal::derive(move || {selected_count}_usize)"),
            "  is_force_visible=true".to_string(),
            "  motion=ActionBarMotion {".to_string(),
            "    hidden_translate_px: 44.0,".to_string(),
            "    hidden_opacity: 0.22,".to_string(),
            "    ..ActionBarMotion::default()".to_string(),
            "  }".to_string(),
            ">".to_string(),
            "  <ActionButton is_quiet=true>\"Sync\"</ActionButton>".to_string(),
            "  <ActionButton is_quiet=true>\"Share\"</ActionButton>".to_string(),
            "</ActionBar>".to_string(),
            "<ActionBar".to_string(),
            format!("  selected_count=Signal::derive(move || {selected_count}_usize)"),
            "  is_force_visible=true".to_string(),
            "  motion=ActionBarMotion::disabled()".to_string(),
            ">".to_string(),
            "  <ActionButton is_quiet=true>\"Sync\"</ActionButton>".to_string(),
            "  <ActionButton is_quiet=true>\"Share\"</ActionButton>".to_string(),
            "</ActionBar>".to_string(),
        ]
        .join(
            "
",
        )
    });

    let snapshot_streaming_code = Signal::derive(move || {
        r#"<ActionBar default_selected_count=2 is_force_visible=true>
  <ActionButton>"Snapshot baseline"</ActionButton>
</ActionBar>
// ActionBar is not an LLM body reader surface.
// Streaming policy: optional; fallback: snapshot."#
            .to_string()
    });
    let interactive_playground_code = Signal::derive(move || {
        let position_literal = match interactive_position.get() {
            ActionBarPosition::Top => "ActionBarPosition::Top",
            ActionBarPosition::Bottom => "ActionBarPosition::Bottom",
        };
        let selected_count = interactive_selected_count.get();
        let with_clear_action = interactive_with_clear_action.get();
        let custom_labels = interactive_custom_labels.get();
        let reduced_motion = interactive_reduced_motion.get();

        let mut lines = vec![
            format!("let (selected_count, set_selected_count) = signal({selected_count}_usize);"),
            "let selected_count_signal = Signal::derive(move || selected_count.get());".to_string(),
            "let on_selected_count_change = Callback::new(move |next: usize| set_selected_count.set(next));"
                .to_string(),
        ];
        if with_clear_action {
            lines.push(
                "let on_clear_selection = Callback::new(move |_| set_selected_count.set(0));"
                    .to_string(),
            );
        }
        lines.push(String::new());
        lines.push("<ActionBar".to_string());
        lines.push("  selected_count=selected_count_signal".to_string());
        lines.push("  on_selected_count_change=on_selected_count_change".to_string());
        if with_clear_action {
            lines.push("  on_clear_selection=on_clear_selection".to_string());
        }
        lines.push(format!("  position={position_literal}"));
        lines.push(format!(
            "  is_force_visible={}",
            interactive_force_visible.get()
        ));
        if custom_labels {
            lines.push("  selection_text=\"Rows selected\".into()".to_string());
            lines.push("  clear_label=\"Clear rows\".into()".to_string());
        }
        if reduced_motion {
            lines.push("  motion=ActionBarMotion::disabled()".to_string());
        }
        lines.push("  aria_label=\"Interactive bulk actions\".into()".to_string());
        lines.push(">".to_string());
        lines.push("  <ActionButton>\"Delete\"</ActionButton>".to_string());
        lines.push("  <ActionButton is_quiet=true>\"Archive\"</ActionButton>".to_string());
        lines.push("</ActionBar>".to_string());

        lines.join("\n")
    });
    let interactive_spec_preview = Signal::derive(move || {
        let has_clear_action = interactive_with_clear_action.get();
        let custom_labels = interactive_custom_labels.get();
        let reduced_motion = interactive_reduced_motion.get();
        let rtl = interactive_position.get() == ActionBarPosition::Top;

        format!(
            "ActionBarActualConfig {{\n  selected_count: Some(Signal<usize>({})),\n  default_selected_count: None,\n  on_selected_count_change: Some(\"Callback<usize>\"),\n  on_clear_selection: {},\n  position: \"{}\",\n  is_force_visible: {},\n  aria_label: Some(\"Interactive bulk actions\"),\n  clear_label: {},\n  selection_text: {},\n  lang: {},\n  dir: {},\n  motion: {},\n  class_name: Some(\"docs-action-bar-interactive\"),\n}}",
            interactive_selected_count.get(),
            if has_clear_action {
                "Some(\"Callback<()>\" )"
            } else {
                "None"
            },
            interactive_position.get().as_attr(),
            interactive_force_visible.get(),
            if custom_labels {
                "Some(\"Clear rows\")"
            } else {
                "None"
            },
            if custom_labels {
                "Some(\"Rows selected\")"
            } else {
                "None"
            },
            if rtl { "Some(\"ar\")" } else { "None" },
            if rtl { "Some(\"rtl\")" } else { "None" },
            if reduced_motion {
                "ActionBarMotion::disabled()"
            } else {
                "ActionBarMotion::default()"
            },
        )
    });
    let action_bar_dependency_code = Signal::derive(move || {
        r#"[dependencies]
ui = { workspace = true, default-features = false, features = ["component-action_bar", "inject-css"] }"#
            .to_string()
    });

    let mut custom_motion = ActionBarMotion::default();
    custom_motion.spring.stiffness = 280.0;
    custom_motion.spring.damping = 24.0;
    custom_motion.spring.mass = 1.0;
    custom_motion.spring.precision = 0.002;
    custom_motion.hidden_translate_px = 44.0;
    custom_motion.hidden_opacity = 0.22;

    view! {
        <ComponentPage
            title="ActionBar"
            slug="action-bar"
            group="Actions"
            description="Bulk-action surface with baseline-style selection contracts and baseline-level spring visibility motion."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <ActionBar default_selected_count=1>
                    <ActionButton>"Archive"</ActionButton>
                </ActionBar>
            </Playground>

            <Playground
                title="Interactive Playground (Props + State + Spec Preview)"
                code_signal=interactive_playground_code
                code_imports=action_bar_code_imports.clone()
                test_config_signal=interactive_spec_preview
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="action-bar-interactive-controls">
                        <div class="docs-search__label">"Position"</div>
                        <SegmentedControl
                            id_base="docs-action-bar-interactive-position".to_string()
                            options=interactive_position_options.clone()
                            selected_index=interactive_position_index
                            set_selected_index=set_interactive_position_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionBar interactive position".to_string()
                        />
                        <Switch checked=interactive_force_visible set_checked=set_interactive_force_visible>
                            "Force visible"
                        </Switch>
                        <Switch checked=interactive_with_clear_action set_checked=set_interactive_with_clear_action>
                            "Enable clear action"
                        </Switch>
                        <Switch checked=interactive_custom_labels set_checked=set_interactive_custom_labels>
                            "Use custom labels"
                        </Switch>
                        <Switch checked=interactive_reduced_motion set_checked=set_interactive_reduced_motion>
                            "Reduced motion"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="action-bar-interactive-preview">
                    <div class="docs-row" data-slot="action-bar-interactive-actions">
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            aria_label="Interactive select +1".to_string()
                            on_press=Callback::new(move |_| {
                                set_interactive_selected_count
                                    .update(|count| *count = count.saturating_add(1));
                            })
                        >
                            "Select +1"
                        </ui::Button>
                        <ui::Button
                            variant=ui::ButtonVariant::Outline
                            aria_label="Interactive select -1".to_string()
                            on_press=Callback::new(move |_| {
                                set_interactive_selected_count
                                    .update(|count| *count = count.saturating_sub(1));
                            })
                        >
                            "Select -1"
                        </ui::Button>
                        <ui::Button
                            variant=ui::ButtonVariant::Ghost
                            aria_label="Interactive reset count".to_string()
                            on_press=Callback::new(move |_| set_interactive_selected_count.set(2))
                        >
                            "Reset to 2"
                        </ui::Button>
                        <span class="ui-muted">
                            "selected: " {move || interactive_selected_count.get()}
                        </span>
                    </div>
                    <span class="ui-muted">
                        "Repeatable flow: Select +1 -> Clear selection -> Select +1."
                    </span>
                    {move || {
                        let position = interactive_position.get();
                        let is_force_visible = interactive_force_visible.get();
                        let selection_text = if interactive_custom_labels.get() {
                            "Rows selected".to_string()
                        } else {
                            String::new()
                        };
                        let clear_label = if interactive_custom_labels.get() {
                            "Clear rows".to_string()
                        } else {
                            String::new()
                        };
                        let motion = if interactive_reduced_motion.get() {
                            ActionBarMotion::disabled()
                        } else {
                            ActionBarMotion::default()
                        };

                        if interactive_with_clear_action.get() {
                            view! {
                                <ActionBar
                                    selected_count=interactive_selected_count_signal
                                    on_selected_count_change=interactive_on_selected_count_change
                                    on_clear_selection=interactive_on_clear_selection
                                    position=position
                                    is_force_visible=is_force_visible
                                    selection_text=selection_text
                                    clear_label=clear_label
                                    motion=motion
                                    aria_label="Interactive bulk actions".to_string()
                                    class_name="docs-action-bar-interactive".to_string()
                                >
                                    <ActionButton>"Delete"</ActionButton>
                                    <ActionButton is_quiet=true>"Archive"</ActionButton>
                                </ActionBar>
                            }
                                .into_any()
                        } else {
                            view! {
                                <ActionBar
                                    selected_count=interactive_selected_count_signal
                                    on_selected_count_change=interactive_on_selected_count_change
                                    position=position
                                    is_force_visible=is_force_visible
                                    selection_text=selection_text
                                    clear_label=clear_label
                                    motion=motion
                                    aria_label="Interactive bulk actions".to_string()
                                    class_name="docs-action-bar-interactive".to_string()
                                >
                                    <ActionButton>"Delete"</ActionButton>
                                    <ActionButton is_quiet=true>"Archive"</ActionButton>
                                </ActionBar>
                            }
                                .into_any()
                        }
                    }}
                </div>
            </Playground>



            <Playground
                title="Selection + clear action"
                code_signal=code
                code_imports=action_bar_code_imports.clone()
            >
                <div class="docs-stack">
                    <div class="docs-row">
                        <ui::Button
                            variant=ui::ButtonVariant::Secondary
                            aria_label="Increase selected count".to_string()
                            on_press=Callback::new(move |_| {
                                set_selected_count.update(|count| *count = count.saturating_add(1));
                            })
                        >
                            "Select +1"
                        </ui::Button>
                        <ui::Button
                            variant=ui::ButtonVariant::Outline
                            aria_label="Decrease selected count".to_string()
                            on_press=Callback::new(move |_| {
                                set_selected_count.update(|count| *count = count.saturating_sub(1));
                            })
                        >
                            "Select -1"
                        </ui::Button>
                        <span class="ui-muted">
                            "selected: " {move || selected_count.get()}
                        </span>
                    </div>

                    <ActionBar
                        selected_count=selected_count_signal
                        on_selected_count_change=on_selected_count_change
                        on_clear_selection=clear_selection
                        aria_label="Bulk actions".to_string()
                        class_name="docs-action-bar".to_string()
                    >
                        <ActionButton>"Delete"</ActionButton>
                        <ActionButton is_quiet=true>"Archive"</ActionButton>
                    </ActionBar>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                code_signal=control_mode_code
                code_imports=action_bar_code_imports.clone()
            >
                <div class="docs-stack">
                    <div class="docs-row">
                        <span class="ui-muted">"Controlled"</span>
                    </div>
                    <ActionBar
                        selected_count=selected_count_signal
                        on_selected_count_change=on_selected_count_change
                        on_clear_selection=clear_selection
                    >
                        <ActionButton>"Delete"</ActionButton>
                        <ActionButton is_quiet=true>"Archive"</ActionButton>
                    </ActionBar>

                    <div class="docs-row">
                        <span class="ui-muted">"Uncontrolled"</span>
                    </div>
                    <ActionBar default_selected_count=2>
                        <ActionButton>"Tag"</ActionButton>
                        <ActionButton is_quiet=true>"Assign"</ActionButton>
                    </ActionBar>
                </div>
            </Playground>

            <Playground
                title="State Scenarios (selection + placement + visibility)"
                code_signal=state_matrix_code
                code_imports=action_bar_code_imports.clone()
            >
                <div class="docs-stack">
                    <ActionBar default_selected_count=0>
                        <ActionButton is_quiet=true>"Hidden when empty"</ActionButton>
                    </ActionBar>
                    <ActionBar default_selected_count=1>
                        <ActionButton>"Bottom / single"</ActionButton>
                    </ActionBar>
                    <ActionBar default_selected_count=4 position=ActionBarPosition::Top is_force_visible=true>
                        <ActionButton>"Top / forced visible"</ActionButton>
                    </ActionBar>
                </div>
            </Playground>

            <Playground
                title="Top placement + custom text + reduced motion"
                code_signal=state_code
                code_imports=action_bar_code_imports.clone()
            >
                <div class="docs-stack">
                    <ActionBar
                        default_selected_count=5
                        position=ActionBarPosition::Top
                        is_force_visible=true
                        selection_text="Rows selected".to_string()
                        clear_label="Clear all".to_string()
                        motion=ActionBarMotion::disabled()
                    >
                        <ActionButton is_quiet=true>"Tag"</ActionButton>
                        <ActionButton is_quiet=true>"Assign"</ActionButton>
                    </ActionBar>
                    <span class="ui-muted">
                        "Top placement + custom labels + motion disabled."
                    </span>
                </div>
            </Playground>

            <Playground
                title="Custom Motion Contract"
                code_signal=motion_code
                code_imports=action_bar_code_imports.clone()
            >
                <div class="docs-stack">
                    <ActionBar
                        selected_count=selected_count_signal
                        is_force_visible=true
                        motion=custom_motion
                    >
                        <ActionButton is_quiet=true>"Sync"</ActionButton>
                        <ActionButton is_quiet=true>"Share"</ActionButton>
                    </ActionBar>
                    <ActionBar
                        selected_count=selected_count_signal
                        is_force_visible=true
                        motion=ActionBarMotion::disabled()
                    >
                        <ActionButton is_quiet=true>"Sync"</ActionButton>
                        <ActionButton is_quiet=true>"Share"</ActionButton>
                    </ActionBar>
                </div>
            </Playground>

            <Playground
                title="Snapshot baseline + Streaming optional fallback"
                code_signal=snapshot_streaming_code
                code_imports=action_bar_code_imports.clone()
            >
                <div class="docs-stack">
                    <ActionBar default_selected_count=2 is_force_visible=true>
                        <ActionButton>"Snapshot baseline"</ActionButton>
                    </ActionBar>
                    <span class="ui-muted">
                        "ActionBar is not an LLM body reader surface. Streaming policy: optional; fallback: snapshot."
                    </span>
                </div>
            </Playground>



            <Playground
                title="State Matrix (Single + Multi + Forced Visible)"
                code_signal=state_matrix_code
                code_imports=action_bar_code_imports.clone()
            >
                <div class="docs-stack">
                    <ActionBar default_selected_count=0>
                        <ActionButton is_quiet=true>"Hidden when empty"</ActionButton>
                    </ActionBar>
                    <ActionBar default_selected_count=1>
                        <ActionButton>"Bottom / single"</ActionButton>
                    </ActionBar>
                    <ActionBar default_selected_count=4 position=ActionBarPosition::Top is_force_visible=true>
                        <ActionButton>"Top / forced visible"</ActionButton>
                    </ActionBar>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="action-bar-source-first">
                <h3>"Source-first Copy-Paste"</h3>
                <p>
                    "Use any ActionBar Playground's "
                    <code>"Show code"</code>
                    " panel and the built-in copy button to grab a runnable snippet with imports."
                </p>
                <ul data-slot="action-bar-source-first-paths">
                    <li>
                        <code>"components/action-bar/src/mod.rs"</code>
                        " (public exports)"
                    </li>
                    <li>
                        <code>"components/action-bar/src/view.rs"</code>
                        " (Leptos structure + semantics mount)"
                    </li>
                    <li>
                        <code>"components/action-bar/src/logic.rs"</code>
                        " (state normalization)"
                    </li>
                    <li>
                        <code>"components/action-bar/src/styles.rs"</code>
                        " (token-first CSS contract)"
                    </li>
                    <li>
                        <code>"components/action-bar/src/motion.rs"</code>
                        " (motion contract mapping)"
                    </li>
                </ul>
                <div class="docs-search__label">"Dependency prerequisites"</div>
                <CodeBlock code=action_bar_dependency_code.get() />
                <p class="ui-muted">
                    "If you copy from docs, keep "
                    <code>"code_imports"</code>
                    " output as-is and enable ActionBar feature flags above to avoid compile errors."
                </p>
            </section>

            <section class="docs-card docs-prose" data-slot="action-bar-api-matrix">
                <h3>"API Matrix"</h3>
                <ul data-slot="action-bar-api-rows">
                    <li>
                        <code>"selected_count: Option&lt;Signal&lt;usize&gt;&gt;"</code>
                        " default = None (uncontrolled path)"
                    </li>
                    <li>
                        <code>"default_selected_count: Option&lt;usize&gt;"</code>
                        " default = implicit 0 via logic::normalize_default_selected_count"
                    </li>
                    <li>
                        <code>"on_selected_count_change: Option&lt;Callback&lt;usize&gt;&gt;"</code>
                        " default = None"
                    </li>
                    <li>
                        <code>"on_clear_selection: Option&lt;Callback&lt;()&gt;&gt;"</code>
                        " default = None"
                    </li>
                    <li>
                        <code>"position: ActionBarPosition"</code>
                        " "
                        {format!(
                            "default = ActionBarPosition::{:?} ({})",
                            ActionBarPosition::default(),
                            ActionBarPosition::default().as_attr()
                        )}
                    </li>
                    <li>
                        <code>"is_force_visible: bool"</code>
                        " default = false"
                    </li>
                    <li>
                        <code>"aria_label: Option&lt;String&gt;"</code>
                        " "
                        {format!(
                            "default label = {:?}",
                            ui::action_bar::DEFAULT_ARIA_LABEL
                        )}
                    </li>
                    <li>
                        <code>"clear_label: Option&lt;String&gt;"</code>
                        " "
                        {format!(
                            "default label = {:?}",
                            ui::action_bar::DEFAULT_CLEAR_LABEL
                        )}
                    </li>
                    <li>
                        <code>"selection_text: Option&lt;String&gt;"</code>
                        " default = None (derived from selected_count)"
                    </li>
                    <li>
                        <code>"lang: Option&lt;String&gt;, dir: Option&lt;A11yDirection&gt;"</code>
                        " default = None (inherits app locale context)"
                    </li>
                    <li>
                        <code>"motion: ActionBarMotion"</code>
                        " default = ActionBarMotion::default()"
                    </li>
                    <li>
                        <code>"class_name: Option&lt;String&gt;"</code>
                        " default = None"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="action-bar-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="action-bar-state-rows">
                    <li>
                        <code>"control mode"</code>
                        " = controlled | uncontrolled"
                    </li>
                    <li>
                        <code>"data-state"</code>
                        " = visible | hidden"
                    </li>
                    <li>
                        <code>"data-position"</code>
                        " = top | bottom"
                    </li>
                    <li>
                        <code>"data-selection"</code>
                        " = empty | single | multiple"
                    </li>
                    <li>
                        <code>"data-selected-count-source"</code>
                        " = external | default"
                    </li>
                    <li>
                        <code>"data-default-selected-count-source"</code>
                        " = provided | implicit"
                    </li>
                    <li>
                        <code>"data-selected-count-change-source / data-clear-action-source"</code>
                        " = provided | none"
                    </li>
                    <li>
                        <code>"data-label-source / data-selection-source / data-clear-label-source / data-class-source / data-motion-source"</code>
                        " = default | custom"
                    </li>
                    <li>
                        <code>"disabled / size / variant"</code>
                        " = N/A on ActionBar root (these axes belong to child actions such as ActionButton)"
                    </li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
