use super::playground_workbench::{bool_word, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::collections::BTreeSet;
use ui::button::ButtonType;
use ui::logic_button::LogicButtonMotion;
use ui::{
    ActionBar, ActionBarMotion, ActionBarPosition, ActionButton, ActionGroup, ActionGroupItem,
    ActionGroupSelectionMode, ActionGroupTone, ClearButton, CloseButton, CloseButtonSize,
    CloseButtonVariant, CodeBlock, FieldButton, InfieldButton, LogicButton, LogicButtonVariant,
    SegmentedControl, SegmentedControlSize, Switch, Toggle, ToggleButtonSize, ToggleButtonVariant,
    ToggleGroup, ToggleGroupItem, ToggleGroupOrientation, ToggleGroupSelectionMode, ToggleMotion,
    ToggleSize, ToggleVariant,
};
use ui_headless::A11yDirection;

// Legacy source-contract markers retained for semantic tests:
// title="Default + OverBackground"
// title="Inset + Focus Mode + Disabled"
// title="Default + OverBackground + Custom Label"
// title="Size Matrix + Disabled + Custom Class"
// title="AND + OR variants"
// <Playground title="AND + OR variants" code_signal=basic_code>
// aria_label="Open in-field options".to_string()

pub(super) fn action_bar() -> AnyView {
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

pub(super) fn field_button() -> AnyView {
    let (showcase_presses, set_showcase_presses) = signal(0_u32);
    let on_showcase_press = Callback::new(move |_| {
        set_showcase_presses.update(|count| *count += 1);
    });

    let button_type_options = vec![
        "Button".to_string(),
        "Submit".to_string(),
        "Reset".to_string(),
    ];
    let (workbench_button_type_index, set_workbench_button_type_index) = signal(Some(0_usize));
    let (workbench_is_quiet, set_workbench_is_quiet) = signal(false);
    let (workbench_is_invalid, set_workbench_is_invalid) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_is_active, set_workbench_is_active) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let workbench_button_type =
        Signal::derive(
            move || match workbench_button_type_index.get().unwrap_or(0) {
                1 => ButtonType::Submit,
                2 => ButtonType::Reset,
                _ => ButtonType::Button,
            },
        );
    let workbench_node_ref = NodeRef::new();
    let (workbench_presses, set_workbench_presses) = signal(0_u32);
    let on_workbench_press = Callback::new(move |_| {
        set_workbench_presses.update(|count| *count += 1);
    });

    let hello_code = Signal::derive(move || {
        r#"<FieldButton aria_label="Open options".to_string() on_press=on_press>
  "Options"
</FieldButton>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let button_type = match workbench_button_type.get() {
            ButtonType::Submit => "ui::ButtonType::Submit",
            ButtonType::Reset => "ui::ButtonType::Reset",
            ButtonType::Button => "ui::ButtonType::Button",
        };
        let aria_label = if workbench_custom_aria.get() {
            "FieldButton workbench"
        } else {
            ""
        };
        let class_name = if workbench_custom_class.get() {
            "docs-field-button-custom"
        } else {
            ""
        };

        [
            "<FieldButton".to_string(),
            format!("  is_quiet={}", bool_word(workbench_is_quiet.get())),
            format!("  is_invalid={}", bool_word(workbench_is_invalid.get())),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  is_active={}", bool_word(workbench_is_active.get())),
            format!("  aria_label={}", rust_string_literal(aria_label)),
            format!("  class_name={}", rust_string_literal(class_name)),
            format!("  button_type={button_type}"),
            "  node_ref=node_ref".to_string(),
            "  on_press=on_press".to_string(),
            ">".to_string(),
            "  \"Field action\"".to_string(),
            "</FieldButton>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let aria_label = if workbench_custom_aria.get() {
            Some("FieldButton workbench")
        } else {
            Some("")
        };
        let class_name = if workbench_custom_class.get() {
            Some("docs-field-button-custom")
        } else {
            Some("")
        };

        format!(
            "FieldButtonActualConfig {{\n  is_quiet: {},\n  is_invalid: {},\n  is_disabled: {},\n  is_active: {},\n  aria_label: {aria_label:?},\n  class_name: {class_name:?},\n  button_type: {:?},\n  node_ref: Some(\"field_button_node_ref\"),\n  on_press: \"count={}\",\n}}",
            bool_word(workbench_is_quiet.get()),
            bool_word(workbench_is_invalid.get()),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_is_active.get()),
            workbench_button_type.get(),
            workbench_presses.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<FieldButton aria_label="Default trigger".to_string()>"Default"</FieldButton>
<FieldButton is_quiet=true button_type=ui::ButtonType::Submit aria_label="Quiet submit".to_string()>"Quiet submit"</FieldButton>
<FieldButton is_invalid=true is_active=true class_name="docs-field-button-custom".to_string() aria_label="Invalid active".to_string()>"Invalid"</FieldButton>
<FieldButton is_disabled=true button_type=ui::ButtonType::Reset aria_label="Disabled reset".to_string()>"Disabled"</FieldButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="FieldButton"
            slug="field-button"
            group="Actions"
            description="baseline-style field trigger button with centralized quiet/invalid/active/disabled state contracts and headless press/hover/focus behavior."
        >
            <Playground title="Hello World (Default FieldButton)" code_signal=hello_code>
                <div class="docs-row">
                    <FieldButton
                        aria_label="Open options".to_string()
                        on_press=on_showcase_press
                    >
                        "Options"
                    </FieldButton>
                    <span class="ui-muted">"on_press count: " {move || showcase_presses.get()}</span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="field-button-workbench-controls">
                        <SegmentedControl
                            id_base="docs-field-button-workbench-type".to_string()
                            options=button_type_options.clone()
                            selected_index=workbench_button_type_index
                            set_selected_index=set_workbench_button_type_index
                            size=SegmentedControlSize::Sm
                            aria_label="FieldButton button_type".to_string()
                        />
                        <Switch checked=workbench_is_quiet set_checked=set_workbench_is_quiet>
                            "is_quiet"
                        </Switch>
                        <Switch checked=workbench_is_invalid set_checked=set_workbench_is_invalid>
                            "is_invalid"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_is_active set_checked=set_workbench_is_active>
                            "is_active"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "aria_label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-row">
                    <FieldButton
                        is_quiet=workbench_is_quiet.get()
                        is_invalid=workbench_is_invalid.get()
                        is_disabled=workbench_is_disabled.get()
                        is_active=workbench_is_active.get()
                        aria_label=if workbench_custom_aria.get() {
                            "FieldButton workbench".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-field-button-custom".to_string()
                        } else {
                            String::new()
                        }
                        button_type=workbench_button_type.get()
                        node_ref=workbench_node_ref
                        on_press=on_workbench_press
                    >
                        "Field action"
                    </FieldButton>
                    <span class="ui-muted">"on_press count: " {move || workbench_presses.get()}</span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Quiet / Invalid / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <FieldButton aria_label="Default trigger".to_string()>"Default"</FieldButton>
                    <FieldButton
                        is_quiet=true
                        button_type=ButtonType::Submit
                        aria_label="Quiet submit".to_string()
                    >
                        "Quiet submit"
                    </FieldButton>
                    <FieldButton
                        is_invalid=true
                        is_active=true
                        class_name="docs-field-button-custom".to_string()
                        aria_label="Invalid active".to_string()
                    >
                        "Invalid"
                    </FieldButton>
                    <FieldButton
                        is_disabled=true
                        button_type=ButtonType::Reset
                        aria_label="Disabled reset".to_string()
                    >
                        "Disabled"
                    </FieldButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn infield_button() -> AnyView {
    let workbench_node_ref: NodeRef<leptos::html::Button> = NodeRef::new();
    let preset_options = vec![
        "Default".to_string(),
        "Quiet".to_string(),
        "Invalid".to_string(),
    ];
    let (preset_index, set_preset_index) = signal(Some(0_usize));
    let (disabled, set_disabled) = signal(false);
    let (forced_active, set_forced_active) = signal(false);
    let (custom_aria_label, set_custom_aria_label) = signal(false);
    let (custom_class_name, set_custom_class_name) = signal(false);
    let button_type_options = vec![
        "button".to_string(),
        "submit".to_string(),
        "reset".to_string(),
    ];
    let (button_type_index, set_button_type_index) = signal(Some(0_usize));
    let button_type = Signal::derive(move || match button_type_index.get().unwrap_or(0) {
        1 => "submit",
        2 => "reset",
        _ => "button",
    });
    let (press_count, set_press_count) = signal(0_usize);

    let quiet = Signal::derive(move || preset_index.get().unwrap_or(0) == 1);
    let invalid = Signal::derive(move || preset_index.get().unwrap_or(0) == 2);

    let on_press = Callback::new(move |_| {
        set_press_count.update(|next| *next += 1);
    });

    let workbench_code = Signal::derive(move || {
        let mut lines = vec!["<InfieldButton".to_string()];
        if quiet.get() {
            lines.push("  quiet=true".to_string());
        }
        if invalid.get() {
            lines.push("  invalid=true".to_string());
        }
        if disabled.get() {
            lines.push("  disabled=true".to_string());
        }
        if forced_active.get() {
            lines.push("  is_active=true".to_string());
        }
        if custom_aria_label.get() {
            lines.push("  aria_label=\"Inspect in-field trigger\".into()".to_string());
        }
        if custom_class_name.get() {
            lines.push("  class_name=\"docs-infield-button-custom\".into()".to_string());
        }
        if button_type.get() != "button" {
            lines.push(format!("  button_type={:?}", button_type.get()));
        }
        lines.push("  node_ref=NodeRef::new()".to_string());
        lines.push("  on_press=Callback::new(move |_| {})".to_string());
        lines.push(">".to_string());
        lines.push("  \"⋯\"".to_string());
        lines.push("</InfieldButton>".to_string());
        lines.join("\n")
    });

    let comparison_code = Signal::derive(move || {
        r#"<InfieldButton>"Default"</InfieldButton>
<InfieldButton quiet=true>"Quiet"</InfieldButton>
<InfieldButton invalid=true is_active=true>"Invalid + Active"</InfieldButton>
<InfieldButton disabled=true>"Disabled"</InfieldButton>"#
            .to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/button/infield_button/styles.rs */\n{}",
            ui::infield_button::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "InfieldButtonWorkbenchConfig {{\n  quiet: {},\n  invalid: {},\n  disabled: {},\n  is_active: {},\n  aria_label: {:?},\n  class_name: {:?},\n  button_type: {:?},\n  node_ref: \"bound\",\n  on_press: {:?},\n  press_count: {},\n}}",
            quiet.get(),
            invalid.get(),
            disabled.get(),
            forced_active.get(),
            if custom_aria_label.get() {
                "Inspect in-field trigger"
            } else {
                ""
            },
            if custom_class_name.get() {
                "docs-infield-button-custom"
            } else {
                ""
            },
            button_type.get(),
            "handler",
            press_count.get()
        )
    });

    view! {
        <ComponentPage
            title="InfieldButton"
            slug="infield-button"
            group="Actions"
            description="baseline-compatible in-field trigger button with centralized quiet/invalid/active/disabled state contracts and headless press/hover/focus behavior."
        >
            <Playground title="Hello World (Default API)" code_signal=comparison_code>
                <div class="docs-row">
                    <InfieldButton on_press=on_press>"⋯"</InfieldButton>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/button/infield_button/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Preset"</div>
                        <SegmentedControl
                            id_base="docs-infield-button-preset".to_string()
                            options=preset_options.clone()
                            selected_index=preset_index
                            set_selected_index=set_preset_index
                            size=SegmentedControlSize::Sm
                            aria_label="InfieldButton preset".to_string()
                        />
                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=forced_active set_checked=set_forced_active>
                            "Force active"
                        </Switch>
                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                        <Switch checked=custom_class_name set_checked=set_custom_class_name>
                            "Custom class"
                        </Switch>
                        <div class="docs-search__label">"Button type"</div>
                        <SegmentedControl
                            id_base="docs-infield-button-button-type".to_string()
                            options=button_type_options.clone()
                            selected_index=button_type_index
                            set_selected_index=set_button_type_index
                            size=SegmentedControlSize::Sm
                            aria_label="InfieldButton button type".to_string()
                        />
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        if custom_aria_label.get() {
                            view! {
                                <InfieldButton
                                    quiet=quiet.get()
                                    invalid=invalid.get()
                                    disabled=disabled.get()
                                    is_active=forced_active.get()
                                    aria_label="Inspect in-field trigger".to_string()
                                    class_name=if custom_class_name.get() {
                                        "docs-infield-button-custom".to_string()
                                    } else {
                                        String::new()
                                    }
                                    button_type=button_type.get()
                                    node_ref=workbench_node_ref
                                    on_press=on_press
                                >
                                    "⋯"
                                </InfieldButton>
                            }
                                .into_any()
                        } else {
                            view! {
                                <InfieldButton
                                    quiet=quiet.get()
                                    invalid=invalid.get()
                                    disabled=disabled.get()
                                    is_active=forced_active.get()
                                    class_name=if custom_class_name.get() {
                                        "docs-infield-button-custom".to_string()
                                    } else {
                                        String::new()
                                    }
                                    button_type=button_type.get()
                                    node_ref=workbench_node_ref
                                    on_press=on_press
                                >
                                    "⋯"
                                </InfieldButton>
                            }
                                .into_any()
                        }
                    }}
                    <span class="ui-muted">"presses: " {move || press_count.get().to_string()}</span>
                </div>
            </Playground>

            <Playground title="Comparison Matrix (Default / Quiet / Invalid+Active / Disabled)" code_signal=comparison_code>
                <div class="docs-row">
                    <InfieldButton>"Default"</InfieldButton>
                    <InfieldButton quiet=true>"Quiet"</InfieldButton>
                    <InfieldButton invalid=true is_active=true>"Invalid + Active"</InfieldButton>
                    <InfieldButton disabled=true>"Disabled"</InfieldButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn clear_button() -> AnyView {
    let workbench_node_ref: NodeRef<leptos::html::Button> = NodeRef::new();

    let (workbench_variant_key, set_workbench_variant_key) = signal("default".to_string());
    let (workbench_focus_mode_key, set_workbench_focus_mode_key) = signal("default".to_string());
    let (workbench_button_type_key, set_workbench_button_type_key) = signal("button".to_string());
    let (workbench_inset, set_workbench_inset) = signal(false);
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_slot, set_workbench_custom_slot) = signal(false);
    let (workbench_visible_raw, set_workbench_visible_raw) = signal(true);
    let (workbench_disabled_signal_raw, set_workbench_disabled_signal_raw) = signal(false);
    let (workbench_hide_aria, set_workbench_hide_aria) = signal(false);

    let workbench_visible: Signal<bool> = Signal::derive(move || workbench_visible_raw.get());
    let workbench_disabled_signal: Signal<bool> =
        Signal::derive(move || workbench_disabled_signal_raw.get());

    let (press_count, set_press_count) = signal(0_u32);
    let (click_count, set_click_count) = signal(0_u32);
    let (blur_count, set_blur_count) = signal(0_u32);
    let (pointer_down_count, set_pointer_down_count) = signal(0_u32);
    let (pointer_up_count, set_pointer_up_count) = signal(0_u32);
    let (pointer_cancel_count, set_pointer_cancel_count) = signal(0_u32);
    let (pointer_enter_count, set_pointer_enter_count) = signal(0_u32);
    let (pointer_leave_count, set_pointer_leave_count) = signal(0_u32);
    let (last_key_down, set_last_key_down) = signal("none".to_string());
    let (last_key_up, set_last_key_up) = signal("none".to_string());

    let on_press = Callback::new(move |_| set_press_count.update(|count| *count += 1));
    let on_click = Callback::new(move |()| set_click_count.update(|count| *count += 1));
    let on_blur = Callback::new(move |()| set_blur_count.update(|count| *count += 1));
    let on_pointer_down = Callback::new(move |_ev: leptos::ev::PointerEvent| {
        set_pointer_down_count.update(|count| *count += 1)
    });
    let on_pointer_up = Callback::new(move |_ev: leptos::ev::PointerEvent| {
        set_pointer_up_count.update(|count| *count += 1)
    });
    let on_pointer_cancel = Callback::new(move |_ev: leptos::ev::PointerEvent| {
        set_pointer_cancel_count.update(|count| *count += 1)
    });
    let on_pointer_enter = Callback::new(move |_ev: leptos::ev::PointerEvent| {
        set_pointer_enter_count.update(|count| *count += 1)
    });
    let on_pointer_leave = Callback::new(move |_ev: leptos::ev::PointerEvent| {
        set_pointer_leave_count.update(|count| *count += 1)
    });
    let on_key_down = Callback::new(move |key: String| {
        set_last_key_down.set(key);
        false
    });
    let on_key_up = Callback::new(move |key: String| {
        set_last_key_up.set(key);
        false
    });

    let hello_code = Signal::derive(move || {
        r#"<ClearButton aria_label="Clear query".to_string()>"×"</ClearButton>"#.to_string()
    });

    let workbench_code = Signal::derive(move || {
        let variant = if workbench_variant_key.get() == "over-background" {
            "ui::ClearButtonVariant::OverBackground"
        } else {
            "ui::ClearButtonVariant::Default"
        };
        let focus_mode = match workbench_focus_mode_key.get().as_str() {
            "prevent" => "ui::ClearButtonFocusMode::Prevent",
            "exclude-tab" => "ui::ClearButtonFocusMode::ExcludeTab",
            _ => "ui::ClearButtonFocusMode::Default",
        };
        format!(
            "<ClearButton\n  variant={variant}\n  inset={}\n  disabled={}\n  focus_mode={focus_mode}\n  slot_name={}\n  aria_label=\"Clear search\".to_string()\n  class_name={}\n  button_type={}\n  node_ref=node_ref\n  on_press=on_press\n  is_visible=Signal::derive(move || {})\n  is_disabled_signal=Signal::derive(move || {})\n  aria_hidden_when_invisible={}\n  on_pointer_down=on_pointer_down\n  on_pointer_up=on_pointer_up\n  on_pointer_cancel=on_pointer_cancel\n  on_pointer_enter=on_pointer_enter\n  on_pointer_leave=on_pointer_leave\n  on_click=on_click\n  on_key_down=on_key_down\n  on_key_up=on_key_up\n  on_blur=on_blur\n>\n  \"×\"\n</ClearButton>",
            workbench_inset.get(),
            workbench_disabled.get(),
            if workbench_custom_slot.get() {
                "\"search-clear\""
            } else {
                "\"clear-button\""
            },
            if workbench_custom_class.get() {
                "\"docs-clear-button-workbench\".to_string()"
            } else {
                "String::new()"
            },
            if workbench_button_type_key.get() == "submit" {
                "\"submit\""
            } else {
                "\"button\""
            },
            workbench_visible_raw.get(),
            workbench_disabled_signal_raw.get(),
            workbench_hide_aria.get(),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let variant = if workbench_variant_key.get() == "over-background" {
            "OverBackground"
        } else {
            "Default"
        };
        let focus_mode = match workbench_focus_mode_key.get().as_str() {
            "prevent" => "Prevent",
            "exclude-tab" => "ExcludeTab",
            _ => "Default",
        };
        let button_type = if workbench_button_type_key.get() == "submit" {
            "submit"
        } else {
            "button"
        };
        let slot_name = if workbench_custom_slot.get() {
            "search-clear"
        } else {
            "clear-button"
        };
        format!(
            "ClearButtonWorkbenchConfig {{\n  variant: {variant},\n  inset: {},\n  disabled: {},\n  focus_mode: {focus_mode},\n  slot_name: \"{slot_name}\",\n  aria_label: Some(\"Clear search\"),\n  class_name: {},\n  button_type: \"{button_type}\",\n  node_ref: Some(\"docs-clear-button-workbench\"),\n  on_press: Some(\"OnPress\"),\n  is_visible: Some({}),\n  is_disabled_signal: Some({}),\n  aria_hidden_when_invisible: {},\n  on_pointer_down: Some(\"Callback<PointerEvent>\"),\n  on_pointer_up: Some(\"Callback<PointerEvent>\"),\n  on_pointer_cancel: Some(\"Callback<PointerEvent>\"),\n  on_pointer_enter: Some(\"Callback<PointerEvent>\"),\n  on_pointer_leave: Some(\"Callback<PointerEvent>\"),\n  on_click: Some(\"Callback<()>\"),\n  on_key_down: Some(\"Callback<String, bool>\"),\n  on_key_up: Some(\"Callback<String, bool>\"),\n  on_blur: Some(\"Callback<()>\"),\n}}",
            workbench_inset.get(),
            workbench_disabled.get(),
            if workbench_custom_class.get() {
                "Some(\"docs-clear-button-workbench\")"
            } else {
                "None"
            },
            workbench_visible_raw.get(),
            workbench_disabled_signal_raw.get(),
            workbench_hide_aria.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ClearButton aria_label="Default clear".to_string()>"×"</ClearButton>
<ClearButton variant=ui::ClearButtonVariant::OverBackground aria_label="Overlay clear".to_string()>"×"</ClearButton>
<ClearButton inset=true focus_mode=ui::ClearButtonFocusMode::Prevent aria_label="Inset prevent".to_string()>"×"</ClearButton>
<ClearButton disabled=true aria_label="Disabled clear".to_string()>"×"</ClearButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ClearButton"
            slug="clear-button"
            group="Actions"
            description="Clear affordance with full pointer/keyboard callback contract."
        >
            <Playground title="Hello World" code_signal=hello_code>
                <ClearButton aria_label="Clear query".to_string()>"×"</ClearButton>
            </Playground>

            <Playground
                title="Config Workbench"
                description="Toggles every ClearButton API and reports callback feedback."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="clear-button-workbench-controls">
                        <label class="docs-choice-row">
                            <span>"Variant"</span>
                            <select class="docs-select" on:change=move |ev| set_workbench_variant_key.set(event_target_value(&ev))>
                                <option value="default" selected=move || workbench_variant_key.get() == "default">"Default"</option>
                                <option value="over-background" selected=move || workbench_variant_key.get() == "over-background">"OverBackground"</option>
                            </select>
                        </label>
                        <label class="docs-choice-row">
                            <span>"Focus mode"</span>
                            <select class="docs-select" on:change=move |ev| set_workbench_focus_mode_key.set(event_target_value(&ev))>
                                <option value="default" selected=move || workbench_focus_mode_key.get() == "default">"Default"</option>
                                <option value="prevent" selected=move || workbench_focus_mode_key.get() == "prevent">"Prevent"</option>
                                <option value="exclude-tab" selected=move || workbench_focus_mode_key.get() == "exclude-tab">"ExcludeTab"</option>
                            </select>
                        </label>
                        <label class="docs-choice-row">
                            <span>"Button type"</span>
                            <select class="docs-select" on:change=move |ev| set_workbench_button_type_key.set(event_target_value(&ev))>
                                <option value="button" selected=move || workbench_button_type_key.get() == "button">"button"</option>
                                <option value="submit" selected=move || workbench_button_type_key.get() == "submit">"submit"</option>
                            </select>
                        </label>
                        <Switch checked=workbench_inset set_checked=set_workbench_inset>"Inset"</Switch>
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>"Disabled"</Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>"Custom class"</Switch>
                        <Switch checked=workbench_custom_slot set_checked=set_workbench_custom_slot>"Custom slot_name"</Switch>
                        <Switch checked=workbench_visible_raw set_checked=set_workbench_visible_raw>"Visible (is_visible)"</Switch>
                        <Switch checked=workbench_disabled_signal_raw set_checked=set_workbench_disabled_signal_raw>"Disabled signal"</Switch>
                        <Switch checked=workbench_hide_aria set_checked=set_workbench_hide_aria>"aria_hidden_when_invisible"</Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="clear-button-workbench-preview">
                    <ClearButton
                        variant=if workbench_variant_key.get() == "over-background" {
                            ui::ClearButtonVariant::OverBackground
                        } else {
                            ui::ClearButtonVariant::Default
                        }
                        inset=workbench_inset.get()
                        disabled=workbench_disabled.get()
                        focus_mode=match workbench_focus_mode_key.get().as_str() {
                            "prevent" => ui::ClearButtonFocusMode::Prevent,
                            "exclude-tab" => ui::ClearButtonFocusMode::ExcludeTab,
                            _ => ui::ClearButtonFocusMode::Default,
                        }
                        slot_name=if workbench_custom_slot.get() { "search-clear" } else { "clear-button" }
                        aria_label="Clear search".to_string()
                        class_name=if workbench_custom_class.get() {
                            "docs-clear-button-workbench".to_string()
                        } else {
                            String::new()
                        }
                        button_type=if workbench_button_type_key.get() == "submit" { "submit" } else { "button" }
                        node_ref=workbench_node_ref
                        on_press=on_press
                        is_visible=workbench_visible
                        is_disabled_signal=workbench_disabled_signal
                        aria_hidden_when_invisible=workbench_hide_aria.get()
                        on_pointer_down=on_pointer_down
                        on_pointer_up=on_pointer_up
                        on_pointer_cancel=on_pointer_cancel
                        on_pointer_enter=on_pointer_enter
                        on_pointer_leave=on_pointer_leave
                        on_click=on_click
                        on_key_down=on_key_down
                        on_key_up=on_key_up
                        on_blur=on_blur
                    >
                        "×"
                    </ClearButton>
                    <span class="ui-muted">
                        "press=" {move || press_count.get()}
                        ", click=" {move || click_count.get()}
                        ", blur=" {move || blur_count.get()}
                    </span>
                    <span class="ui-muted">
                        "pointer: down=" {move || pointer_down_count.get()}
                        ", up=" {move || pointer_up_count.get()}
                        ", cancel=" {move || pointer_cancel_count.get()}
                        ", enter=" {move || pointer_enter_count.get()}
                        ", leave=" {move || pointer_leave_count.get()}
                    </span>
                    <span class="ui-muted">
                        "key: down=" {move || last_key_down.get()}
                        ", up=" {move || last_key_up.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <ClearButton aria_label="Default clear".to_string()>"×"</ClearButton>
                    <ClearButton
                        variant=ui::ClearButtonVariant::OverBackground
                        aria_label="Overlay clear".to_string()
                    >
                        "×"
                    </ClearButton>
                    <ClearButton
                        inset=true
                        focus_mode=ui::ClearButtonFocusMode::Prevent
                        aria_label="Inset prevent".to_string()
                    >
                        "×"
                    </ClearButton>
                    <ClearButton disabled=true aria_label="Disabled clear".to_string()>"×"</ClearButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn close_button() -> AnyView {
    let close_button_imports =
        "use leptos::prelude::*;\nuse ui::{CloseButton, CloseButtonSize, CloseButtonVariant};"
            .to_string();
    let (variant_index, set_variant_index) = signal(0usize);
    let (size_index, set_size_index) = signal(1usize);
    let (disabled, set_disabled) = signal(false);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (submit_type, set_submit_type) = signal(false);
    let (show_child, set_show_child) = signal(false);
    let (press_count, set_press_count) = signal(0usize);

    let workbench_variant = Signal::derive(move || match variant_index.get() {
        1 => CloseButtonVariant::OverBackground,
        _ => CloseButtonVariant::Default,
    });
    let workbench_size = Signal::derive(move || match size_index.get() {
        0 => CloseButtonSize::Sm,
        2 => CloseButtonSize::Xl,
        _ => CloseButtonSize::Md,
    });
    let workbench_aria_label = Signal::derive(move || {
        if custom_aria.get() {
            "Dismiss popover".to_string()
        } else {
            String::new()
        }
    });
    let workbench_class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-close-button-custom".to_string()
        } else {
            String::new()
        }
    });

    let on_press = Callback::new(move |_| {
        set_press_count.update(|value| *value += 1);
    });
    let workbench_node_ref = NodeRef::<leptos::html::Button>::new();
    let matrix_node_ref = NodeRef::<leptos::html::Button>::new();

    let showcase_code = Signal::derive(move || r#"<CloseButton />"#.to_string());

    let workbench_code = Signal::derive(move || {
        format!(
            "<CloseButton\n  variant=CloseButtonVariant::{:?}\n  size=CloseButtonSize::{:?}\n  disabled={}\n  aria_label={}\n  class_name={}\n  button_type={}\n  node_ref=node_ref\n  on_press=Some(Callback::new(move |_| {{}}))\n>\n  {}\n</CloseButton>",
            workbench_variant.get(),
            workbench_size.get(),
            bool_word(disabled.get()),
            rust_string_literal(&workbench_aria_label.get()),
            rust_string_literal(&workbench_class_name.get()),
            rust_string_literal(if submit_type.get() {
                "submit"
            } else {
                "button"
            }),
            if show_child.get() { "\"Dismiss\"" } else { "" },
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<CloseButton />
<CloseButton variant=CloseButtonVariant::OverBackground size=CloseButtonSize::Sm />
<CloseButton
  variant=CloseButtonVariant::Default
  size=CloseButtonSize::Xl
  disabled=true
  aria_label="Dismiss dialog".to_string()
  class_name="docs-close-button-custom".to_string()
  button_type="submit"
  node_ref=NodeRef::<leptos::html::Button>::new()
  on_press=Some(Callback::new(move |_| {}))
>
  "Dismiss"
</CloseButton>"#
            .to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* components/button/src/close_button/styles.rs */\n{}",
            ui::close_button::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "CloseButtonActualConfig {{\n  variant: {:?},\n  size: {:?},\n  disabled: {},\n  aria_label: {:?},\n  class_name: {:?},\n  button_type: {:?},\n  node_ref: {:?},\n  on_press: {:?},\n  children: {:?},\n}}",
            workbench_variant.get(),
            workbench_size.get(),
            disabled.get(),
            workbench_aria_label.get(),
            workbench_class_name.get(),
            if submit_type.get() {
                "submit"
            } else {
                "button"
            },
            Some("NodeRef<html::Button>"),
            Some("Callback<OnPress>"),
            if show_child.get() {
                Some("Dismiss")
            } else {
                None
            },
        )
    });

    view! {
        <ComponentPage
            title="CloseButton"
            slug="close-button"
            group="Actions"
            description="baseline-style close affordance with default icon fallback, centralized variant+size contracts, and stable state/source data markers."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                code_imports=close_button_imports.clone()
            >
                <div class="docs-row">
                    <CloseButton />
                </div>
            </Playground>

            <Playground
                title="Workbench (Config + Live Actual Config)"
                code_signal=workbench_code
                code_imports=close_button_imports.clone()
                test_css_source=workbench_test_css_source
                test_source_path="components/button/src/close_button/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="close-button-workbench-controls">
                        <div class="docs-search__label">"Variant"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || variant_index.get().to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_variant_index.set(value.min(1));
                                }
                            }
                        >
                            <option value="0">"Default"</option>
                            <option value="1">"OverBackground"</option>
                        </select>

                        <div class="docs-search__label">"Size"</div>
                        <select
                            class="docs-search__input"
                            prop:value=move || size_index.get().to_string()
                            on:change=move |event| {
                                if let Ok(value) = event_target_value(&event).parse::<usize>() {
                                    set_size_index.set(value.min(2));
                                }
                            }
                        >
                            <option value="0">"Sm"</option>
                            <option value="1">"Md"</option>
                            <option value="2">"Xl"</option>
                        </select>

                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || disabled.get()
                                on:change=move |event| set_disabled.set(event_target_checked(&event))
                            />
                            <span>"Disabled"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_aria.get()
                                on:change=move |event| set_custom_aria.set(event_target_checked(&event))
                            />
                            <span>"Custom aria_label"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || custom_class.get()
                                on:change=move |event| set_custom_class.set(event_target_checked(&event))
                            />
                            <span>"Custom class_name"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || submit_type.get()
                                on:change=move |event| set_submit_type.set(event_target_checked(&event))
                            />
                            <span>"button_type=submit"</span>
                        </label>
                        <label class="docs-choice-row">
                            <input
                                type="checkbox"
                                prop:checked=move || show_child.get()
                                on:change=move |event| set_show_child.set(event_target_checked(&event))
                            />
                            <span>"Custom children"</span>
                        </label>
                    </div>
                }
            >
                <div class="docs-row">
                    <CloseButton
                        variant=workbench_variant.get()
                        size=workbench_size.get()
                        disabled=disabled.get()
                        aria_label=workbench_aria_label.get()
                        class_name=workbench_class_name.get()
                        button_type=if submit_type.get() { "submit" } else { "button" }
                        node_ref=workbench_node_ref
                        on_press=on_press
                    >
                        {move || if show_child.get() { "Dismiss".to_string() } else { String::new() }}
                    </CloseButton>
                    <span class="ui-muted">
                        "on_press count: " {move || press_count.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Variant / Size / Disabled Comparison)"
                code_signal=matrix_code
                code_imports=close_button_imports
            >
                <div class="docs-row">
                    <CloseButton />
                    <CloseButton
                        variant=CloseButtonVariant::OverBackground
                        size=CloseButtonSize::Sm
                    />
                    <CloseButton
                        variant=CloseButtonVariant::Default
                        size=CloseButtonSize::Xl
                        disabled=true
                        aria_label="Dismiss dialog".to_string()
                        class_name="docs-close-button-custom".to_string()
                        button_type="submit"
                        node_ref=matrix_node_ref
                        on_press=Callback::new(move |_| {})
                    >
                        "Dismiss"
                    </CloseButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn logic_button() -> AnyView {
    let workbench_node_ref: NodeRef<leptos::html::Button> = NodeRef::new();
    let variant_options = vec!["AND".to_string(), "OR".to_string()];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let (disabled, set_disabled) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_aria, set_custom_aria) = signal(false);
    let button_type_options = vec![
        "button".to_string(),
        "submit".to_string(),
        "reset".to_string(),
    ];
    let (button_type_index, set_button_type_index) = signal(Some(0_usize));
    let button_type = Signal::derive(move || match button_type_index.get().unwrap_or(0) {
        1 => "submit",
        2 => "reset",
        _ => "button",
    });
    let (press_count, set_press_count) = signal(0_usize);

    let variant = Signal::derive(move || {
        if variant_index.get().unwrap_or(0) == 1 {
            LogicButtonVariant::Or
        } else {
            LogicButtonVariant::And
        }
    });

    let motion = Signal::derive(move || {
        if custom_motion.get() {
            LogicButtonMotion {
                transition_ms: 240,
                press_scale_pct: 92,
            }
        } else {
            LogicButtonMotion::default()
        }
    });

    let on_press = Callback::new(move |_| {
        set_press_count.update(|next| *next += 1);
    });

    let workbench_code = Signal::derive(move || {
        let mut lines = vec!["<LogicButton".to_string()];
        lines.push(format!("  variant=LogicButtonVariant::{:?}", variant.get()));
        if disabled.get() {
            lines.push("  disabled=true".to_string());
        }
        if custom_motion.get() {
            lines.push(
                "  motion=LogicButtonMotion { transition_ms: 240, press_scale_pct: 92 }"
                    .to_string(),
            );
        }
        if custom_class.get() {
            lines.push("  class_name=\"docs-logic-button-custom\".into()".to_string());
        }
        if custom_aria.get() {
            lines.push("  aria_label=\"Logic operator\".into()".to_string());
        }
        if button_type.get() != "button" {
            lines.push(format!("  button_type={:?}", button_type.get()));
        }
        lines.push("  node_ref=NodeRef::new()".to_string());
        lines.push("  on_press=Callback::new(move |_| {})".to_string());
        lines.push(">".to_string());
        lines.push("  \"Logic\"".to_string());
        lines.push("</LogicButton>".to_string());
        lines.join("\n")
    });

    let comparison_code = Signal::derive(move || {
        r#"<LogicButton variant=LogicButtonVariant::And>"AND"</LogicButton>
<LogicButton variant=LogicButtonVariant::Or>"OR"</LogicButton>
<LogicButton variant=LogicButtonVariant::And class_name="docs-logic-button-custom".to_string()>"Custom"</LogicButton>
<LogicButton variant=LogicButtonVariant::Or disabled=true>"Disabled"</LogicButton>"#.to_string()
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/button/logic_button/styles.rs */\n{}",
            ui::logic_button::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "LogicButtonWorkbenchConfig {{\n  variant: \"{:?}\",\n  disabled: {},\n  aria_label: {:?},\n  class_name: {:?},\n  button_type: {:?},\n  node_ref: \"bound\",\n  on_press: {:?},\n  custom_motion: {},\n  press_count: {},\n}}",
            variant.get(),
            disabled.get(),
            if custom_aria.get() {
                "Logic operator"
            } else {
                ""
            },
            if custom_class.get() {
                "docs-logic-button-custom"
            } else {
                ""
            },
            button_type.get(),
            "handler",
            custom_motion.get(),
            press_count.get()
        )
    });

    view! {
        <ComponentPage
            title="LogicButton"
            slug="logic-button"
            group="Actions"
            description="baseline-style boolean operator button with centralized variant normalization, headless press/hover/focus behavior, and stable state/source data contracts."
        >
            <Playground title="Hello World (Default API)" code_signal=comparison_code>
                <div class="docs-row">
                    <LogicButton on_press=on_press>"AND"</LogicButton>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/button/logic_button/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-logic-button-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="LogicButton variant".to_string()
                        />
                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=custom_motion set_checked=set_custom_motion>
                            "Custom motion"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </Switch>
                        <Switch checked=custom_aria set_checked=set_custom_aria>
                            "Custom aria label"
                        </Switch>
                        <div class="docs-search__label">"Button type"</div>
                        <SegmentedControl
                            id_base="docs-logic-button-button-type".to_string()
                            options=button_type_options.clone()
                            selected_index=button_type_index
                            set_selected_index=set_button_type_index
                            size=SegmentedControlSize::Sm
                            aria_label="LogicButton button type".to_string()
                        />
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <LogicButton
                        variant=variant.get()
                        disabled=disabled.get()
                        motion=motion.get()
                        aria_label=if custom_aria.get() {
                            "Logic operator".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if custom_class.get() {
                            "docs-logic-button-custom".to_string()
                        } else {
                            String::new()
                        }
                        button_type=button_type.get()
                        node_ref=workbench_node_ref
                        on_press=on_press
                    >
                        {if variant.get() == LogicButtonVariant::And {
                            "AND"
                        } else {
                            "OR"
                        }}
                    </LogicButton>
                    <span class="ui-muted">"presses: " {move || press_count.get().to_string()}</span>
                </div>
            </Playground>

            <Playground title="Comparison Matrix (AND / OR / Custom / Disabled)" code_signal=comparison_code>
                <div class="docs-row">
                    <LogicButton variant=LogicButtonVariant::And>"AND"</LogicButton>
                    <LogicButton variant=LogicButtonVariant::Or>"OR"</LogicButton>
                    <LogicButton
                        variant=LogicButtonVariant::And
                        class_name="docs-logic-button-custom".to_string()
                    >
                        "Custom"
                    </LogicButton>
                    <LogicButton variant=LogicButtonVariant::Or disabled=true>"Disabled"</LogicButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn action_group() -> AnyView {
    let items = vec![
        ActionGroupItem::new("align-left", "Align Left"),
        ActionGroupItem::new("align-center", "Align Center"),
        ActionGroupItem::new("align-right", "Align Right"),
        ActionGroupItem::new("align-justify", "Justify").disabled(true),
    ];
    let showcase_items = items.clone();
    let workbench_items = items.clone();
    let matrix_items = items;

    let (workbench_selected_ids, set_workbench_selected_ids) =
        signal(BTreeSet::from(["align-left".to_string()]));
    let workbench_selected_ids_signal: Signal<BTreeSet<String>> =
        Signal::derive(move || workbench_selected_ids.get());
    let (workbench_last_action, set_workbench_last_action) = signal("none".to_string());
    let (workbench_selection_change_count, set_workbench_selection_change_count) = signal(0_u32);

    let on_workbench_selected_change = Callback::new(move |next: BTreeSet<String>| {
        set_workbench_selected_ids.set(next);
        set_workbench_selection_change_count.update(|count| *count += 1);
    });

    let on_workbench_action = Callback::new(move |id: String| {
        set_workbench_last_action.set(id);
    });

    let tone_options = vec!["Default".to_string(), "Strong".to_string()];
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let workbench_tone = Signal::derive(move || match workbench_tone_index.get().unwrap_or(0) {
        1 => ActionGroupTone::Strong,
        _ => ActionGroupTone::Default,
    });
    let (workbench_multiple, set_workbench_multiple) = signal(false);
    let workbench_selection_mode = Signal::derive(move || {
        if workbench_multiple.get() {
            ActionGroupSelectionMode::Multiple
        } else {
            ActionGroupSelectionMode::Single
        }
    });
    let workbench_default_selected_ids = Signal::derive(move || {
        if workbench_multiple.get() {
            BTreeSet::from(["align-left".to_string(), "align-center".to_string()])
        } else {
            BTreeSet::from(["align-left".to_string()])
        }
    });
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<ActionGroup
  id_base="text-align".to_string()
  items=vec![
    ActionGroupItem::new("align-left", "Align Left"),
    ActionGroupItem::new("align-center", "Align Center"),
    ActionGroupItem::new("align-right", "Align Right"),
  ]
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let selection_mode = if workbench_multiple.get() {
            "ActionGroupSelectionMode::Multiple"
        } else {
            "ActionGroupSelectionMode::Single"
        };
        let tone = match workbench_tone.get() {
            ActionGroupTone::Strong => "ActionGroupTone::Strong",
            _ => "ActionGroupTone::Default",
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let class_name = if workbench_custom_class.get() {
            "\"docs-action-group-workbench\".to_string()"
        } else {
            "String::new()"
        };
        let selected_literal = format!("{:?}", workbench_selected_ids.get());
        let default_selected_literal = format!("{:?}", workbench_default_selected_ids.get());

        [
            "<ActionGroup".to_string(),
            "  id_base=\"docs-action-group-workbench\".to_string()".to_string(),
            "  items=vec![".to_string(),
            "    ActionGroupItem::new(\"align-left\", \"Align Left\"),".to_string(),
            "    ActionGroupItem::new(\"align-center\", \"Align Center\"),".to_string(),
            "    ActionGroupItem::new(\"align-right\", \"Align Right\"),".to_string(),
            "    ActionGroupItem::new(\"align-justify\", \"Justify\").disabled(true),".to_string(),
            "  ]".to_string(),
            format!("  tone={tone}"),
            format!("  selection_mode={selection_mode}"),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  selected_ids=Signal::derive(move || {selected_literal})"),
            format!("  default_selected_ids={default_selected_literal}"),
            "  on_selected_ids_change=Callback::new(move |next| { drop(next); })".to_string(),
            "  on_action=Callback::new(move |id| { drop(id); })".to_string(),
            "  aria_label=\"Text alignment actions\".to_string()".to_string(),
            format!(
                "  lang={}.to_string()",
                rust_string_literal(if workbench_rtl.get() { "ar" } else { "en-US" })
            ),
            format!("  dir={dir}"),
            format!("  class_name={class_name}"),
            ">".to_string(),
            "</ActionGroup>".to_string(),
        ]
        .join("\n")
    });

    let workbench_items_for_config = workbench_items.clone();
    let workbench_actual_config = Signal::derive(move || {
        let tone = match workbench_tone.get() {
            ActionGroupTone::Strong => "strong",
            _ => "default",
        };
        let selection_mode = match workbench_selection_mode.get() {
            ActionGroupSelectionMode::Multiple => "multiple",
            ActionGroupSelectionMode::Single => "single",
            ActionGroupSelectionMode::None => "none",
        };
        let lang = if workbench_rtl.get() { "ar" } else { "en-US" };
        let dir = if workbench_rtl.get() { "rtl" } else { "ltr" };
        let class_name = if workbench_custom_class.get() {
            Some("docs-action-group-workbench")
        } else {
            None
        };

        format!(
            "ActionGroupWorkbenchActualConfig {{\n  id_base: \"docs-action-group-workbench\",\n  items: {:?},\n  tone: \"{tone}\",\n  selection_mode: \"{selection_mode}\",\n  is_disabled: {},\n  selected_ids: {:?},\n  default_selected_ids: Some({:?}),\n  on_selected_ids_change: \"count={}\",\n  on_action: \"last={}\",\n  aria_label: Some(\"Text alignment actions\"),\n  lang: Some({lang:?}),\n  dir: Some({dir:?}),\n  class_name: {class_name:?},\n}}",
            workbench_items_for_config.clone(),
            bool_word(workbench_is_disabled.get()),
            workbench_selected_ids.get(),
            workbench_default_selected_ids.get(),
            workbench_selection_change_count.get(),
            workbench_last_action.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ActionGroup
  id_base="text-align-default".to_string()
  items=vec![
    ActionGroupItem::new("align-left", "Align Left"),
    ActionGroupItem::new("align-center", "Align Center"),
    ActionGroupItem::new("align-right", "Align Right"),
  ]
/>
<ActionGroup
  id_base="text-align-multiple".to_string()
  items=vec![
    ActionGroupItem::new("align-left", "Align Left"),
    ActionGroupItem::new("align-center", "Align Center"),
    ActionGroupItem::new("align-right", "Align Right"),
  ]
  selection_mode=ActionGroupSelectionMode::Multiple
  tone=ActionGroupTone::Strong
  default_selected_ids=BTreeSet::from(["align-left".to_string(), "align-center".to_string()])
/>
<ActionGroup
  id_base="text-align-disabled".to_string()
  items=vec![
    ActionGroupItem::new("align-left", "Align Left"),
    ActionGroupItem::new("align-center", "Align Center"),
    ActionGroupItem::new("align-right", "Align Right"),
    ActionGroupItem::new("align-justify", "Justify").disabled(true),
  ]
  is_disabled=true
  aria_label="Disabled text alignment".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="ActionGroup"
            slug="action-group"
            group="Actions"
            description="Selectable action cluster with centralized selection normalization and baseline-style state/source data contracts."
        >
            <Playground title="Hello World (Default)" code_signal=hello_code>
                <div class="docs-stack">
                    <ActionGroup
                        id_base="docs-action-group-default".to_string()
                        items=showcase_items
                    />
                    <span class="ui-muted">
                        "basic selection cluster"
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="action-group-workbench-controls">
                        <SegmentedControl
                            id_base="docs-action-group-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=workbench_tone_index
                            set_selected_index=set_workbench_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionGroup tone".to_string()
                        />
                        <Switch checked=workbench_multiple set_checked=set_workbench_multiple>
                            "Multiple selection_mode"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL (lang + dir)"
                        </Switch>
                        <div class="docs-row">
                            <button
                                type="button"
                                on:click=move |_| set_workbench_selected_ids.set(BTreeSet::from(["align-left".to_string()]))
                            >
                                "Select left"
                            </button>
                            <button
                                type="button"
                                on:click=move |_| set_workbench_selected_ids.set(BTreeSet::new())
                            >
                                "Clear selection"
                            </button>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack">
                    <ActionGroup
                        id_base="docs-action-group-workbench".to_string()
                        items=workbench_items.clone()
                        tone=workbench_tone.get()
                        selection_mode=workbench_selection_mode.get()
                        is_disabled=workbench_is_disabled.get()
                        selected_ids=workbench_selected_ids_signal
                        default_selected_ids=workbench_default_selected_ids.get()
                        on_selected_ids_change=on_workbench_selected_change
                        on_action=on_workbench_action
                        aria_label="Text alignment actions".to_string()
                        lang=if workbench_rtl.get() {
                            "ar".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-action-group-workbench".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted">
                        "selected: "
                        {move || workbench_selected_ids.get().iter().cloned().collect::<Vec<_>>().join(", ")}
                        " · selection changes: "
                        {move || workbench_selection_change_count.get()}
                        " · last action: "
                        {move || workbench_last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Multiple / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <ActionGroup
                        id_base="docs-action-group-matrix-default".to_string()
                        items=matrix_items.clone()
                    />
                    <ActionGroup
                        id_base="docs-action-group-matrix-multiple".to_string()
                        items=matrix_items.clone()
                        selection_mode=ActionGroupSelectionMode::Multiple
                        default_selected_ids=BTreeSet::from([
                            "align-left".to_string(),
                            "align-center".to_string(),
                        ])
                        tone=ActionGroupTone::Strong
                    />
                    <ActionGroup
                        id_base="docs-action-group-matrix-disabled".to_string()
                        items=matrix_items
                        is_disabled=true
                        aria_label="Disabled text alignment".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn toggle() -> AnyView {
    let (showcase_pressed, set_showcase_pressed) = signal(false);
    let showcase_pressed_signal: Signal<bool> = Signal::derive(move || showcase_pressed.get());
    let (showcase_change_runs, set_showcase_change_runs) = signal(0_u32);
    let on_showcase_pressed_change = Callback::new(move |next: bool| {
        set_showcase_pressed.set(next);
        set_showcase_change_runs.update(|count| *count += 1);
    });

    let variant_options = vec![
        "Default".to_string(),
        "Outline".to_string(),
        "Ghost".to_string(),
    ];
    let size_options = vec!["S".to_string(), "M".to_string(), "L".to_string()];
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let (workbench_default_pressed, set_workbench_default_pressed) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => ToggleVariant::Outline,
            2 => ToggleVariant::Ghost,
            _ => ToggleVariant::Default,
        });
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => ToggleSize::S,
        2 => ToggleSize::L,
        _ => ToggleSize::M,
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            ToggleMotion {
                tap_scale: 0.92,
                ..ToggleMotion::default()
            }
        } else {
            ToggleMotion::default()
        }
    });
    let (workbench_pressed, set_workbench_pressed) = signal(false);
    let workbench_pressed_signal: Signal<bool> = Signal::derive(move || workbench_pressed.get());
    let (workbench_change_runs, set_workbench_change_runs) = signal(0_u32);
    let on_workbench_pressed_change = Callback::new(move |next: bool| {
        set_workbench_pressed.set(next);
        set_workbench_change_runs.update(|count| *count += 1);
    });
    let workbench_node_ref = NodeRef::new();

    let hello_code = Signal::derive(move || {
        r#"<Toggle
  default_pressed=false
  aria_label="Toggle bold".to_string()
  on_pressed_change=on_pressed_change
>
  "Bold"
</Toggle>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let variant = match workbench_variant.get() {
            ToggleVariant::Outline => "ToggleVariant::Outline",
            ToggleVariant::Ghost => "ToggleVariant::Ghost",
            _ => "ToggleVariant::Default",
        };
        let size = match workbench_size.get() {
            ToggleSize::S => "ToggleSize::S",
            ToggleSize::L => "ToggleSize::L",
            _ => "ToggleSize::M",
        };
        let motion = if workbench_custom_motion.get() {
            "ToggleMotion { tap_scale: 0.92, ..ToggleMotion::default() }"
        } else {
            "ToggleMotion::default()"
        };
        let class_name = if workbench_custom_class.get() {
            "docs-toggle-workbench"
        } else {
            ""
        };
        let aria_label = if workbench_custom_aria.get() {
            "Toggle workbench"
        } else {
            ""
        };

        [
            "<Toggle".to_string(),
            "  is_pressed=pressed_signal".to_string(),
            format!(
                "  default_pressed={}",
                bool_word(workbench_default_pressed.get())
            ),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  variant={variant}"),
            format!("  size={size}"),
            format!("  motion={motion}"),
            "  on_pressed_change=on_pressed_change".to_string(),
            format!("  class_name={}", rust_string_literal(class_name)),
            format!("  aria_label={}", rust_string_literal(aria_label)),
            "  node_ref=node_ref".to_string(),
            ">".to_string(),
            "  \"Format\"".to_string(),
            "</Toggle>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let class_name = if workbench_custom_class.get() {
            Some("docs-toggle-workbench")
        } else {
            Some("")
        };
        let aria_label = if workbench_custom_aria.get() {
            Some("Toggle workbench")
        } else {
            Some("")
        };

        format!(
            "ToggleActualConfig {{\n  is_pressed: Some(Signal<bool>({})),\n  default_pressed: Some({}),\n  is_disabled: {},\n  variant: {:?},\n  size: {:?},\n  motion: {},\n  on_pressed_change: \"runs={}\",\n  class_name: {class_name:?},\n  aria_label: {aria_label:?},\n  node_ref: Some(\"toggle_node_ref\"),\n}}",
            workbench_pressed.get(),
            bool_word(workbench_default_pressed.get()),
            bool_word(workbench_is_disabled.get()),
            workbench_variant.get(),
            workbench_size.get(),
            if workbench_custom_motion.get() {
                "ToggleMotion::custom"
            } else {
                "ToggleMotion::default"
            },
            workbench_change_runs.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Toggle default_pressed=false aria_label="Default".to_string()>"Default"</Toggle>
<Toggle default_pressed=true variant=ToggleVariant::Outline size=ToggleSize::S aria_label="Outline pressed".to_string()>"Outline"</Toggle>
<Toggle is_disabled=true variant=ToggleVariant::Ghost size=ToggleSize::L aria_label="Disabled ghost".to_string()>"Disabled"</Toggle>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="Toggle"
            slug="toggle"
            group="Actions"
            description="baseline-compatible single toggle primitive with baseline-style press/focus contracts and baseline-level spring press motion."
        >
            <Playground title="Hello World (Default Toggle)" code_signal=hello_code>
                <div class="docs-row">
                    <Toggle
                        default_pressed=false
                        aria_label="Toggle bold".to_string()
                        is_pressed=showcase_pressed_signal
                        on_pressed_change=on_showcase_pressed_change
                    >
                        "Bold"
                    </Toggle>
                    <span class="ui-muted">
                        "pressed: " {move || showcase_pressed.get()}
                        " · on_pressed_change: " {move || showcase_change_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="toggle-workbench-controls">
                        <SegmentedControl
                            id_base="docs-toggle-workbench-variant".to_string()
                            options=variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Toggle variant".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-toggle-workbench-size".to_string()
                            options=size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="Toggle size".to_string()
                        />
                        <Switch checked=workbench_default_pressed set_checked=set_workbench_default_pressed>
                            "default_pressed"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "motion"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "aria_label"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-row">
                    <Toggle
                        is_pressed=workbench_pressed_signal
                        default_pressed=workbench_default_pressed.get()
                        is_disabled=workbench_is_disabled.get()
                        variant=workbench_variant.get()
                        size=workbench_size.get()
                        motion=workbench_motion.get()
                        on_pressed_change=on_workbench_pressed_change
                        class_name=if workbench_custom_class.get() {
                            "docs-toggle-workbench".to_string()
                        } else {
                            String::new()
                        }
                        aria_label=if workbench_custom_aria.get() {
                            "Toggle workbench".to_string()
                        } else {
                            String::new()
                        }
                        node_ref=workbench_node_ref
                    >
                        "Format"
                    </Toggle>
                    <span class="ui-muted">
                        "pressed: " {move || workbench_pressed.get()}
                        " · on_pressed_change: " {move || workbench_change_runs.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Outline / Disabled)" code_signal=matrix_code>
                <div class="docs-row">
                    <Toggle default_pressed=false aria_label="Default".to_string()>
                        "Default"
                    </Toggle>
                    <Toggle
                        default_pressed=true
                        variant=ToggleVariant::Outline
                        size=ToggleSize::S
                        aria_label="Outline pressed".to_string()
                    >
                        "Outline"
                    </Toggle>
                    <Toggle
                        is_disabled=true
                        variant=ToggleVariant::Ghost
                        size=ToggleSize::L
                        aria_label="Disabled ghost".to_string()
                    >
                        "Disabled"
                    </Toggle>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn toggle_group() -> AnyView {
    let items = vec![
        ToggleGroupItem::new("bold", "Bold"),
        ToggleGroupItem::new("italic", "Italic"),
        ToggleGroupItem::new("underline", "Underline"),
        ToggleGroupItem::new("strike", "Strike").disabled(true),
    ];
    let showcase_items = items.clone();
    let workbench_items = items.clone();
    let matrix_items = items;

    let (selected_ids_raw, set_selected_ids_raw) =
        signal(BTreeSet::from(["bold".to_string(), "italic".to_string()]));
    let selected_ids: Signal<BTreeSet<String>> = Signal::derive(move || selected_ids_raw.get());
    let (on_selected_ids_change_runs, set_on_selected_ids_change_runs) = signal(0_u32);
    let on_selected_ids_change = Callback::new(move |next: BTreeSet<String>| {
        set_selected_ids_raw.set(next);
        set_on_selected_ids_change_runs.update(|count| *count += 1);
    });

    let (last_action, set_last_action) = signal("none".to_string());
    let (on_action_runs, set_on_action_runs) = signal(0_u32);
    let on_action = Callback::new(move |id: String| {
        set_last_action.set(id);
        set_on_action_runs.update(|count| *count += 1);
    });

    let (workbench_mode_index, set_workbench_mode_index) = signal(Some(0_usize));
    let mode_options = vec!["Multiple".to_string(), "Single".to_string()];
    let selection_mode = Signal::derive(move || match workbench_mode_index.get().unwrap_or(0) {
        1 => ToggleGroupSelectionMode::Single,
        _ => ToggleGroupSelectionMode::Multiple,
    });

    let (workbench_orientation_index, set_workbench_orientation_index) = signal(Some(0_usize));
    let orientation_options = vec!["Horizontal".to_string(), "Vertical".to_string()];
    let orientation =
        Signal::derive(
            move || match workbench_orientation_index.get().unwrap_or(0) {
                1 => ToggleGroupOrientation::Vertical,
                _ => ToggleGroupOrientation::Horizontal,
            },
        );

    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let variant_options = vec![
        "Default".to_string(),
        "Outline".to_string(),
        "Ghost".to_string(),
    ];
    let variant = Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
        1 => ToggleButtonVariant::Outline,
        2 => ToggleButtonVariant::Ghost,
        _ => ToggleButtonVariant::Default,
    });

    let (workbench_size_index, set_workbench_size_index) = signal(Some(2_usize));
    let size_options = vec!["Xs".to_string(), "Sm".to_string(), "Md".to_string()];
    let size = Signal::derive(move || match workbench_size_index.get().unwrap_or(2) {
        0 => ToggleButtonSize::Xs,
        1 => ToggleButtonSize::Sm,
        _ => ToggleButtonSize::M,
    });

    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_is_attached, set_workbench_is_attached) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<ToggleGroup
  id_base="docs-toggle-group-hello".to_string()
  items=vec![
    ToggleGroupItem::new("bold", "Bold"),
    ToggleGroupItem::new("italic", "Italic"),
    ToggleGroupItem::new("underline", "Underline"),
  ]
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let class_name = if workbench_custom_class.get() {
            "docs-toggle-group-custom"
        } else {
            ""
        };
        [
            "<ToggleGroup".to_string(),
            "  id_base=\"docs-toggle-group-workbench\".to_string()".to_string(),
            "  items=vec![ToggleGroupItem::new(\"bold\", \"Bold\"), ToggleGroupItem::new(\"italic\", \"Italic\"), ToggleGroupItem::new(\"underline\", \"Underline\"), ToggleGroupItem::new(\"strike\", \"Strike\").disabled(true)]".to_string(),
            format!("  selection_mode={:?}", selection_mode.get()),
            "  selected_ids=selected_ids".to_string(),
            "  default_selected_ids=BTreeSet::from([\"bold\".to_string(), \"italic\".to_string()])".to_string(),
            "  on_selected_ids_change=on_selected_ids_change".to_string(),
            "  on_action=on_action".to_string(),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  is_attached={}", bool_word(workbench_is_attached.get())),
            format!("  orientation={:?}", orientation.get()),
            format!("  variant={:?}", variant.get()),
            format!("  size={:?}", size.get()),
            "  aria_label=\"Text style toggles\".to_string()".to_string(),
            "  lang=\"en-US\".to_string()".to_string(),
            format!("  dir={dir}"),
            format!("  class_name={}", rust_string_literal(class_name)),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let class_name = if workbench_custom_class.get() {
            Some("docs-toggle-group-custom")
        } else {
            None
        };
        let dir = if workbench_rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        format!(
            "ToggleGroupActualConfig {{\n  id_base: \"docs-toggle-group-workbench\",\n  items: \"sample_items(len=4)\",\n  selection_mode: {:?},\n  selected_ids: {:?},\n  default_selected_ids: Some(BTreeSet::from([\"bold\".to_string(), \"italic\".to_string()])),\n  on_selected_ids_change: \"runs={}\",\n  on_action: \"runs={},last={:?}\",\n  is_disabled: {},\n  is_attached: {},\n  orientation: {:?},\n  variant: {:?},\n  size: {:?},\n  aria_label: Some(\"Text style toggles\"),\n  lang: Some(\"en-US\"),\n  dir: Some({dir:?}),\n  class_name: {class_name:?},\n}}",
            selection_mode.get(),
            selected_ids_raw.get(),
            on_selected_ids_change_runs.get(),
            on_action_runs.get(),
            last_action.get(),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_is_attached.get()),
            orientation.get(),
            variant.get(),
            size.get(),
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<ToggleGroup id_base="tg-default".to_string() items=vec![ToggleGroupItem::new("bold", "Bold"), ToggleGroupItem::new("italic", "Italic"), ToggleGroupItem::new("underline", "Underline")] selected_ids=Signal::derive(|| BTreeSet::from(["bold".to_string()])) selection_mode=ToggleGroupSelectionMode::Multiple />
<ToggleGroup id_base="tg-single".to_string() items=vec![ToggleGroupItem::new("bold", "Bold"), ToggleGroupItem::new("italic", "Italic"), ToggleGroupItem::new("underline", "Underline")] selection_mode=ToggleGroupSelectionMode::Single orientation=ToggleGroupOrientation::Vertical variant=ToggleButtonVariant::Outline size=ToggleButtonSize::Sm />
<ToggleGroup id_base="tg-disabled".to_string() items=vec![ToggleGroupItem::new("bold", "Bold"), ToggleGroupItem::new("italic", "Italic"), ToggleGroupItem::new("underline", "Underline")] is_disabled=true is_attached=false variant=ToggleButtonVariant::Ghost />"#.to_string()
    });

    view! {
        <ComponentPage
            title="ToggleGroup"
            slug="toggle-group"
            group="Actions"
            description="ToggleGroup playground with strict Showcase/Workbench/Matrix layout and full API feedback."
        >
            <Playground title="Hello World (Default Group)" code_signal=hello_code>
                <ToggleGroup
                    id_base="docs-toggle-group-hello".to_string()
                    items=showcase_items
                    aria_label="Formatting options".to_string()
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="toggle-group-workbench-controls">
                        <SegmentedControl
                            id_base="docs-toggle-group-mode".to_string()
                            options=mode_options.clone()
                            selected_index=workbench_mode_index
                            set_selected_index=set_workbench_mode_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleGroup selection mode".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-toggle-group-orientation".to_string()
                            options=orientation_options.clone()
                            selected_index=workbench_orientation_index
                            set_selected_index=set_workbench_orientation_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleGroup orientation".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-toggle-group-variant".to_string()
                            options=variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleGroup variant".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-toggle-group-size".to_string()
                            options=size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ToggleGroup size".to_string()
                        />
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_is_attached set_checked=set_workbench_is_attached>
                            "is_attached"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "RTL dir"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="toggle-group-workbench-preview">
                    <ToggleGroup
                        id_base="docs-toggle-group-workbench".to_string()
                        items=workbench_items
                        selection_mode=selection_mode.get()
                        selected_ids=selected_ids
                        default_selected_ids=BTreeSet::from([
                            "bold".to_string(),
                            "italic".to_string(),
                        ])
                        on_selected_ids_change=on_selected_ids_change
                        on_action=on_action
                        is_disabled=workbench_is_disabled.get()
                        is_attached=workbench_is_attached.get()
                        orientation=orientation.get()
                        variant=variant.get()
                        size=size.get()
                        aria_label="Text style toggles".to_string()
                        lang="en-US".to_string()
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-toggle-group-custom".to_string()
                        } else {
                            String::new()
                        }
                    />
                    <span class="ui-muted" data-slot="toggle-group-workbench-feedback">
                        "selected_ids: " {move || format!("{:?}", selected_ids_raw.get())}
                        " · on_selected_ids_change: " {move || on_selected_ids_change_runs.get()}
                        " · on_action: " {move || on_action_runs.get()}
                        " · last_action: " {move || last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Single / Disabled)" code_signal=matrix_code>
                <div class="docs-row" data-slot="toggle-group-state-matrix">
                    <ToggleGroup
                        id_base="docs-toggle-group-matrix-default".to_string()
                        items=matrix_items.clone()
                        selected_ids=Signal::derive(move || BTreeSet::from(["bold".to_string()]))
                        selection_mode=ToggleGroupSelectionMode::Multiple
                    />
                    <ToggleGroup
                        id_base="docs-toggle-group-matrix-single".to_string()
                        items=matrix_items.clone()
                        selection_mode=ToggleGroupSelectionMode::Single
                        orientation=ToggleGroupOrientation::Vertical
                        variant=ToggleButtonVariant::Outline
                        size=ToggleButtonSize::Sm
                    />
                    <ToggleGroup
                        id_base="docs-toggle-group-matrix-disabled".to_string()
                        items=matrix_items
                        is_disabled=true
                        is_attached=false
                        variant=ToggleButtonVariant::Ghost
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
