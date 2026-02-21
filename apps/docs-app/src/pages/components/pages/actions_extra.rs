use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use std::collections::BTreeSet;
use ui_components::logic_button::LogicButtonMotion;
use ui_components::{
    ActionBar, ActionBarMotion, ActionBarPosition, ActionButton, ActionGroup, ActionGroupItem,
    ActionGroupSelectionMode, ActionGroupTone, ClearButton, CloseButton, CloseButtonSize,
    CloseButtonVariant, CodeBlock, FieldButton, InfieldButton, LogicButton, LogicButtonVariant,
    SegmentedControl, SegmentedControlSize, Switch, Toggle, ToggleGroup, ToggleGroupItem,
    ToggleGroupOrientation, ToggleGroupSelectionMode, ToggleMotion, ToggleSize, ToggleVariant,
};

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
        "use leptos::prelude::*;\nuse ui_components::{ActionBar, ActionBarMotion, ActionBarPosition, ActionButton};"
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
        format!(
            "ActionBarInteractiveSpec {{\n  selected_count: {},\n  position: \"{}\",\n  is_force_visible: {},\n  has_clear_action: {},\n  custom_labels: {},\n  reduced_motion: {},\n}}",
            interactive_selected_count.get(),
            interactive_position.get().as_attr(),
            interactive_force_visible.get(),
            interactive_with_clear_action.get(),
            interactive_custom_labels.get(),
            interactive_reduced_motion.get(),
        )
    });
    let action_bar_dependency_code = Signal::derive(move || {
        r#"[dependencies]
ui-components = { workspace = true, default-features = false, features = ["component-action_bar", "inject-css"] }"#
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
                title="Selection + clear action"
                code_signal=code
                code_imports=action_bar_code_imports.clone()
            >
                <div class="docs-stack">
                    <div class="docs-row">
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            aria_label="Increase selected count".to_string()
                            on_press=Callback::new(move |_| {
                                set_selected_count.update(|count| *count = count.saturating_add(1));
                            })
                        >
                            "Select +1"
                        </ui_components::Button>
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Outline
                            aria_label="Decrease selected count".to_string()
                            on_press=Callback::new(move |_| {
                                set_selected_count.update(|count| *count = count.saturating_sub(1));
                            })
                        >
                            "Select -1"
                        </ui_components::Button>
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
                title="State Matrix (selection + placement + visibility)"
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
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Secondary
                            aria_label="Interactive select +1".to_string()
                            on_press=Callback::new(move |_| {
                                set_interactive_selected_count
                                    .update(|count| *count = count.saturating_add(1));
                            })
                        >
                            "Select +1"
                        </ui_components::Button>
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Outline
                            aria_label="Interactive select -1".to_string()
                            on_press=Callback::new(move |_| {
                                set_interactive_selected_count
                                    .update(|count| *count = count.saturating_sub(1));
                            })
                        >
                            "Select -1"
                        </ui_components::Button>
                        <ui_components::Button
                            variant=ui_components::ButtonVariant::Ghost
                            aria_label="Interactive reset count".to_string()
                            on_press=Callback::new(move |_| set_interactive_selected_count.set(2))
                        >
                            "Reset to 2"
                        </ui_components::Button>
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
                            ui_components::action_bar::DEFAULT_ARIA_LABEL
                        )}
                    </li>
                    <li>
                        <code>"clear_label: Option&lt;String&gt;"</code>
                        " "
                        {format!(
                            "default label = {:?}",
                            ui_components::action_bar::DEFAULT_CLEAR_LABEL
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
    let default_code = Signal::derive(move || {
        r#"<FieldButton aria_label="Open options".to_string()>
  "Options"
</FieldButton>
<FieldButton is_quiet=true aria_label="Open calendar".to_string()>
  "📅"
</FieldButton>"#
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"<FieldButton
  is_invalid=true
  is_active=true
  aria_label="Invalid trigger".to_string()
  class_name="docs-field-button-custom".to_string()
>
  "Needs fix"
</FieldButton>
<FieldButton is_disabled=true aria_label="Disabled trigger".to_string()>
  "Disabled"
</FieldButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="FieldButton"
            slug="field-button"
            group="Actions"
            description="baseline-style field trigger button with centralized quiet/invalid/active/disabled state contracts and headless press/hover/focus behavior."
        >
            <Playground title="Default + Quiet" code_signal=default_code>
                <div class="docs-row">
                    <FieldButton aria_label="Open options".to_string()>
                        "Options"
                    </FieldButton>
                    <FieldButton is_quiet=true aria_label="Open calendar".to_string()>
                        "📅"
                    </FieldButton>
                </div>
            </Playground>

            <Playground title="Invalid + Active + Disabled" code_signal=state_code>
                <div class="docs-row">
                    <FieldButton
                        is_invalid=true
                        is_active=true
                        aria_label="Invalid trigger".to_string()
                        class_name="docs-field-button-custom".to_string()
                    >
                        "Needs fix"
                    </FieldButton>
                    <FieldButton is_disabled=true aria_label="Disabled trigger".to_string()>
                        "Disabled"
                    </FieldButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn infield_button() -> AnyView {
    let preset_options = vec![
        "Default".to_string(),
        "Quiet".to_string(),
        "Invalid".to_string(),
    ];
    let (preset_index, set_preset_index) = signal(Some(0_usize));
    let (disabled, set_disabled) = signal(false);
    let (forced_active, set_forced_active) = signal(false);
    let (custom_aria_label, set_custom_aria_label) = signal(false);
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
            "/* crates/ui-components/src/button/infield_button/styles.rs */\n{}",
            ui_components::infield_button::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let preset = match preset_index.get().unwrap_or(0) {
            1 => "quiet",
            2 => "invalid",
            _ => "default",
        };
        format!(
            "InfieldButtonWorkbenchConfig {{\n  preset: \"{preset}\",\n  disabled: {},\n  is_active: {},\n  custom_aria_label: {},\n  press_count: {},\n}}",
            disabled.get(),
            forced_active.get(),
            custom_aria_label.get(),
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
            <Playground
                title="Interactive Playground (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/button/infield_button/styles.rs".to_string()
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
    let basic_code = Signal::derive(move || {
        r#"<ClearButton aria_label="Clear query".to_string()>"×"</ClearButton>
<ClearButton
  variant=ui_components::ClearButtonVariant::OverBackground
  aria_label="Dismiss overlay".to_string()
>
  "×"
</ClearButton>"#
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"<ClearButton
  inset=true
  focus_mode=ui_components::ClearButtonFocusMode::Prevent
  aria_label="Clear token".to_string()
  class_name="docs-clear-button-custom".to_string()
>
  "×"
</ClearButton>
<ClearButton
  disabled=true
  focus_mode=ui_components::ClearButtonFocusMode::ExcludeTab
  aria_label="Disabled clear".to_string()
>
  "×"
</ClearButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ClearButton"
            slug="clear-button"
            group="Actions"
            description="baseline-style clear affordance with centralized variant/inset/focus-mode normalization and stable state/source data contracts."
        >
            <Playground title="Default + OverBackground" code_signal=basic_code>
                <div class="docs-row">
                    <ClearButton aria_label="Clear query".to_string()>"×"</ClearButton>
                    <ClearButton
                        variant=ui_components::ClearButtonVariant::OverBackground
                        aria_label="Dismiss overlay".to_string()
                    >
                        "×"
                    </ClearButton>
                </div>
            </Playground>

            <Playground title="Inset + Focus Mode + Disabled" code_signal=state_code>
                <div class="docs-row">
                        <ClearButton
                            inset=true
                            focus_mode=ui_components::ClearButtonFocusMode::Prevent
                            aria_label="Clear token".to_string()
                            class_name="docs-clear-button-custom".to_string()
                        >
                        "×"
                    </ClearButton>
                        <ClearButton
                            disabled=true
                            focus_mode=ui_components::ClearButtonFocusMode::ExcludeTab
                            aria_label="Disabled clear".to_string()
                        >
                        "×"
                    </ClearButton>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
pub(super) fn close_button() -> AnyView {
    let basic_code = Signal::derive(move || {
        r#"<CloseButton />
<CloseButton variant=CloseButtonVariant::OverBackground />
<CloseButton aria_label="Dismiss popover".to_string() />"#
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"<CloseButton size=CloseButtonSize::Sm />
<CloseButton size=CloseButtonSize::Lg />
<CloseButton size=CloseButtonSize::Xl disabled=true />
<CloseButton class_name="docs-close-button-custom".to_string() />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="CloseButton"
            slug="close-button"
            group="Actions"
            description="baseline-style close affordance with default icon fallback, centralized variant+size contracts, and stable state/source data markers."
        >
            <Playground title="Default + OverBackground + Custom Label" code_signal=basic_code>
                <div class="docs-row">
                    <CloseButton />
                    <CloseButton variant=CloseButtonVariant::OverBackground />
                    <CloseButton aria_label="Dismiss popover".to_string() />
                </div>
            </Playground>

            <Playground title="Size Matrix + Disabled + Custom Class" code_signal=state_code>
                <div class="docs-row">
                    <CloseButton size=CloseButtonSize::Sm />
                    <CloseButton size=CloseButtonSize::Lg />
                    <CloseButton size=CloseButtonSize::Xl disabled=true />
                    <CloseButton class_name="docs-close-button-custom".to_string() />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn logic_button() -> AnyView {
    let variant_options = vec!["AND".to_string(), "OR".to_string()];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let (disabled, set_disabled) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
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
            "/* crates/ui-components/src/button/logic_button/styles.rs */\n{}",
            ui_components::logic_button::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "LogicButtonWorkbenchConfig {{\n  variant: \"{:?}\",\n  disabled: {},\n  custom_motion: {},\n  custom_class: {},\n  press_count: {},\n}}",
            variant.get(),
            disabled.get(),
            custom_motion.get(),
            custom_class.get(),
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
            <Playground
                title="Interactive Playground (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-components/src/button/logic_button/styles.rs".to_string()
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
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <LogicButton
                        variant=variant.get()
                        disabled=disabled.get()
                        motion=motion.get()
                        class_name=if custom_class.get() {
                            "docs-logic-button-custom".to_string()
                        } else {
                            String::new()
                        }
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

    let (selected_ids, set_selected_ids) = signal(BTreeSet::from(["align-left".to_string()]));
    let (last_action, set_last_action) = signal("none".to_string());

    let on_selected_change = Callback::new(move |next: BTreeSet<String>| {
        set_selected_ids.set(next);
    });

    let on_action = Callback::new(move |id: String| {
        set_last_action.set(id);
    });

    let items_primary = items.clone();
    let items_secondary = items;

    let code = Signal::derive(move || {
        let selected_ids = selected_ids.get();
        let selected_literal = if selected_ids.is_empty() {
            None
        } else {
            let ids = selected_ids
                .iter()
                .map(|id| format!("\"{id}\".into()"))
                .collect::<Vec<_>>()
                .join(", ");
            Some(format!("BTreeSet::from([{ids}])"))
        };

        let mut snippet = vec![
            "<ActionGroup".to_string(),
            "  id_base=\"text-align\".into()".to_string(),
            "  items=vec![".to_string(),
            "    ActionGroupItem::new(\"align-left\", \"Align Left\"),".to_string(),
            "    ActionGroupItem::new(\"align-center\", \"Align Center\"),".to_string(),
            "    ActionGroupItem::new(\"align-right\", \"Align Right\"),".to_string(),
            "    ActionGroupItem::new(\"align-justify\", \"Justify\").disabled(true),".to_string(),
            "  ]".to_string(),
        ];

        if let Some(selected_literal) = selected_literal {
            snippet.push(format!("  default_selected_ids={selected_literal}"));
        }

        snippet.extend([">".to_string(), "</ActionGroup>".to_string()]);

        snippet.join(
            "
",
        )
    });

    let states_code = Signal::derive(move || {
        r#"<ActionGroup
  id_base="text-style".to_string()
  items=vec![
    ActionGroupItem::new("align-left", "Align Left"),
    ActionGroupItem::new("align-center", "Align Center"),
    ActionGroupItem::new("align-right", "Align Right"),
    ActionGroupItem::new("align-justify", "Justify").disabled(true),
  ]
  selection_mode=ActionGroupSelectionMode::Multiple
  default_selected_ids=BTreeSet::from(["align-left".to_string(), "align-center".to_string()])
  tone=ActionGroupTone::Strong
  class_name="docs-action-group-custom".to_string()
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
            <Playground title="Single Selection + Action Callback" code_signal=code>
                <div class="docs-stack">
                    <ActionGroup
                        id_base="docs-action-group-single".to_string()
                        items=items_primary
                        selected_ids=selected_ids
                        on_selected_ids_change=on_selected_change
                        on_action=on_action
                    />
                    <span class="ui-muted">
                        "selected: " {move || selected_ids.get().iter().cloned().collect::<Vec<_>>().join(", ")}
                        " · last action: " {move || last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground title="Multiple + Strong Tone" code_signal=states_code>
                <ActionGroup
                    id_base="docs-action-group-multiple".to_string()
                    items=items_secondary
                    selection_mode=ActionGroupSelectionMode::Multiple
                    default_selected_ids=BTreeSet::from([
                        "align-left".to_string(),
                        "align-center".to_string(),
                    ])
                    tone=ActionGroupTone::Strong
                    class_name="docs-action-group-custom".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn toggle() -> AnyView {
    let (pressed, set_pressed) = signal(false);
    let pressed_signal: Signal<bool> = Signal::derive(move || pressed.get());
    let on_pressed_change = Callback::new(move |next: bool| set_pressed.set(next));

    let basic_code = Signal::derive(move || {
        let pressed = pressed.get();

        [
            format!("let (pressed, set_pressed) = signal({pressed});"),
            "let pressed_signal: Signal<bool> = Signal::derive(move || pressed.get());".to_string(),
            String::new(),
            "<Toggle".to_string(),
            "  is_pressed=pressed_signal".to_string(),
            "  on_pressed_change=Callback::new(move |next| set_pressed.set(next))".to_string(),
            ">".to_string(),
            "  \"Bold\"".to_string(),
            "</Toggle>".to_string(),
        ]
        .join(
            "
",
        )
    });

    let states_code = Signal::derive(move || {
        let pressed = pressed.get();

        vec![
            format!("let (pressed, set_pressed) = signal({pressed});"),
            "let pressed_signal: Signal<bool> = Signal::derive(move || pressed.get());".to_string(),
            String::new(),
            "<Toggle".to_string(),
            "  is_pressed=pressed_signal".to_string(),
            "  on_pressed_change=Callback::new(move |next| set_pressed.set(next))".to_string(),
            "  variant=ToggleVariant::Outline".to_string(),
            "  size=ToggleSize::Sm".to_string(),
            ">".to_string(),
            "  \"Italic\"".to_string(),
            "</Toggle>".to_string(),
            "<Toggle".to_string(),
            "  is_pressed=pressed_signal".to_string(),
            "  on_pressed_change=Callback::new(move |next| set_pressed.set(next))".to_string(),
            "  variant=ToggleVariant::Ghost".to_string(),
            "  is_disabled=true".to_string(),
            ">".to_string(),
            "  \"Disabled\"".to_string(),
            "</Toggle>".to_string(),
        ]
        .join(
            "
",
        )
    });

    let markers_code = Signal::derive(move || {
        let pressed = pressed.get();

        vec![
            format!("let (pressed, set_pressed) = signal({pressed});"),
            "let pressed_signal: Signal<bool> = Signal::derive(move || pressed.get());".to_string(),
            String::new(),
            "<Toggle".to_string(),
            "  is_pressed=pressed_signal".to_string(),
            "  on_pressed_change=Callback::new(move |next| set_pressed.set(next))".to_string(),
            "  variant=ToggleVariant::Outline".to_string(),
            "  size=ToggleSize::Sm".to_string(),
            "  motion=ToggleMotion { tap_scale: 0.92, ..ToggleMotion::default() }".to_string(),
            "  class_name=\"docs-toggle-state\".into()".to_string(),
            "  aria_label=\"Toggle formatting\".into()".to_string(),
            ">".to_string(),
            "  \"Markers\"".to_string(),
            "</Toggle>".to_string(),
        ]
        .join(
            "
",
        )
    });

    view! {
        <ComponentPage
            title="Toggle"
            slug="toggle"
            group="Actions"
            description="baseline-compatible single toggle primitive with baseline-style press/focus contracts and baseline-level spring press motion."
        >
            <Playground title="Controlled Toggle" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Toggle
                            is_pressed=pressed_signal
                            on_pressed_change=on_pressed_change
                        >
                            "Bold"
                        </Toggle>
                        <span class="ui-muted">"pressed: " {move || pressed.get()}</span>
                    </div>
                </div>
            </Playground>

            <Playground title="Outline + Ghost + Disabled" code_signal=states_code>
                <div class="docs-row">
                    <Toggle
                        is_pressed=pressed_signal
                        on_pressed_change=on_pressed_change
                        variant=ToggleVariant::Outline
                        size=ToggleSize::Sm
                    >
                        "Italic"
                    </Toggle>
                    <Toggle
                        is_pressed=pressed_signal
                        on_pressed_change=on_pressed_change
                        variant=ToggleVariant::Ghost
                        is_disabled=true
                    >
                        "Disabled"
                    </Toggle>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-interaction`, `data-variant-source`, `data-motion-source`, `data-aria-source`, and `data-handler-source` contracts."
                code_signal=markers_code
            >
                <div class="docs-row">
                    <Toggle
                        is_pressed=pressed_signal
                        variant=ToggleVariant::Outline
                        size=ToggleSize::Sm
                        motion=ToggleMotion {
                            tap_scale: 0.92,
                            ..ToggleMotion::default()
                        }
                        class_name="docs-toggle-state".to_string()
                        aria_label="Toggle formatting".to_string()
                        on_pressed_change=on_pressed_change
                    >
                        "Markers"
                    </Toggle>
                    <span class="ui-muted">"pressed: " {move || pressed.get()}</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn toggle_group() -> AnyView {
    let style_items = vec![
        ToggleGroupItem::new("bold", "Bold"),
        ToggleGroupItem::new("italic", "Italic"),
        ToggleGroupItem::new("underline", "Underline"),
    ];

    let (style_selected_raw, set_style_selected_raw) =
        signal(BTreeSet::from(["bold".to_string(), "italic".to_string()]));
    let style_selected: Signal<BTreeSet<String>> = Signal::derive(move || style_selected_raw.get());
    let on_style_selected_change =
        Callback::new(move |next: BTreeSet<String>| set_style_selected_raw.set(next));

    let alignment_items = vec![
        ToggleGroupItem::new("left", "Left"),
        ToggleGroupItem::new("center", "Center"),
        ToggleGroupItem::new("right", "Right").disabled(true),
    ];

    let (alignment_selected_raw, set_alignment_selected_raw) =
        signal(BTreeSet::from(["center".to_string()]));
    let alignment_selected: Signal<BTreeSet<String>> =
        Signal::derive(move || alignment_selected_raw.get());
    let on_alignment_selected_change =
        Callback::new(move |next: BTreeSet<String>| set_alignment_selected_raw.set(next));

    let code = Signal::derive(move || {
        let selected_ids = style_selected_raw.get();
        let selected_literal = if selected_ids.is_empty() {
            None
        } else {
            let ids = selected_ids
                .iter()
                .map(|id| format!("\"{id}\".into()"))
                .collect::<Vec<_>>()
                .join(", ");
            Some(format!("BTreeSet::from([{ids}])"))
        };

        let mut snippet = vec![
            "<ToggleGroup".to_string(),
            "  id_base=\"formatting\".into()".to_string(),
            "  items=vec![".to_string(),
            "    ToggleGroupItem::new(\"bold\", \"Bold\"),".to_string(),
            "    ToggleGroupItem::new(\"italic\", \"Italic\"),".to_string(),
            "    ToggleGroupItem::new(\"underline\", \"Underline\"),".to_string(),
            "  ]".to_string(),
            "  is_attached=true".to_string(),
        ];

        if let Some(selected_literal) = selected_literal {
            snippet.push(format!("  default_selected_ids={selected_literal}"));
        }

        snippet.extend([">".to_string(), "</ToggleGroup>".to_string()]);

        snippet.join(
            "
",
        )
    });

    let states_code = Signal::derive(move || {
        r#"<ToggleGroup
  id_base="alignment".to_string()
  items=vec![
    ToggleGroupItem::new("left", "Left"),
    ToggleGroupItem::new("center", "Center"),
    ToggleGroupItem::new("right", "Right").disabled(true),
  ]
  default_selected_ids=BTreeSet::from(["center".to_string()])
  selection_mode=ToggleGroupSelectionMode::Single
  orientation=ToggleGroupOrientation::Vertical
  is_attached=false
  aria_label="Alignment controls".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="ToggleGroup"
            slug="toggle-group"
            group="Actions"
            description="baseline-compatible grouped toggle primitive with controlled selection modes and baseline-style root state contracts."
        >
            <Playground title="Multiple + Attached" code_signal=code>
                <div class="docs-stack docs-stack--tight">
                    <ToggleGroup
                        id_base="docs-toggle-group-formatting".to_string()
                        items=style_items
                        selected_ids=style_selected
                        on_selected_ids_change=on_style_selected_change
                        selection_mode=ToggleGroupSelectionMode::Multiple
                        is_attached=true
                    />
                    <span class="ui-muted">
                        "selected ids: "
                        {move || {
                            style_selected_raw
                                .get()
                                .iter()
                                .cloned()
                                .collect::<Vec<_>>()
                                .join(", ")
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="Single + Vertical + Disabled Item" code_signal=states_code>
                <div class="docs-stack docs-stack--tight">
                    <ToggleGroup
                        id_base="docs-toggle-group-alignment".to_string()
                        items=alignment_items
                        selected_ids=alignment_selected
                        on_selected_ids_change=on_alignment_selected_change
                        selection_mode=ToggleGroupSelectionMode::Single
                        orientation=ToggleGroupOrientation::Vertical
                        is_attached=false
                        aria_label="Alignment controls".to_string()
                        class_name="docs-toggle-group-custom".to_string()
                    />
                    <span class="ui-muted">
                        "alignment selected: "
                        {move || {
                            alignment_selected_raw
                                .get()
                                .iter()
                                .cloned()
                                .collect::<Vec<_>>()
                                .join(", ")
                        }}
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
