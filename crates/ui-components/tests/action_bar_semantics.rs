use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(component_path) = rel_path.strip_prefix("src/") {
        let mut parts = component_path.splitn(2, '/');
        let component = parts.next().unwrap_or_default();
        let Some(suffix) = parts.next() else {
            return fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        };
        let component_dir = component.replace('_', "-");
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/{component_dir}/src/{suffix}"));
        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn workspace_root(manifest_dir: &Path) -> &Path {
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"))
}

#[test]
fn action_bar_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/action_bar/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ActionBar internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn action_bar_component_files_follow_layered_responsibilities() {
    let mod_source = load_source("src/action_bar/mod.rs");
    let logic_source = load_source("src/action_bar/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/action_bar.rs");
    let styles_source = load_source("src/action_bar/styles.rs");
    let view_source = load_source("src/action_bar/view.rs");
    let motion_source = load_source("src/action_bar/motion.rs");

    for needle in [
        "mod i18n;",
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::ActionBar;",
        "pub use motion::ActionBarMotion;",
    ] {
        assert!(
            mod_source.contains(needle),
            "ActionBar module boundary should include `{needle}`."
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;", "resolve_view_state("] {
        assert!(
            !mod_source.contains(forbidden),
            "ActionBar mod.rs should keep minimal exports and avoid implementation detail `{forbidden}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::action_bar::{",
        "pub fn resolve_view_state(input: ActionBarViewStateInput) -> ActionBarState",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionBar logic layer should include `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_state(input: ActionBarStateInput)",
        "pub struct ActionBarState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ActionBar state primitive layer should include `{needle}`."
        );
    }

    for forbidden in ["view! {", "<div", "NodeRef", "web_sys::"] {
        assert!(
            !logic_source.contains(forbidden),
            "ActionBar logic layer should not contain view or platform details `{forbidden}`."
        );
    }

    for needle in [
        "pub const CSS: &str",
        "var(--ui-fg)",
        "var(--ui-border)",
        "var(--ui-shadow-md)",
    ] {
        assert!(
            styles_source.contains(needle),
            "ActionBar styles layer should include token-first static css signal `{needle}`."
        );
    }

    for forbidden in ["view! {", "Callback::new(", "on:click", "on:keydown"] {
        assert!(
            !styles_source.contains(forbidden),
            "ActionBar styles layer should stay static and avoid runtime logic `{forbidden}`."
        );
    }

    for needle in [
        "logic::resolve_view_state(logic::ActionBarViewStateInput {",
        "use ui_button::{Button, ButtonSize, ButtonVariant};",
        "<Button",
        "variant=ButtonVariant::Link",
        "motion::attach_motion(root_ref, visible, motion);",
        "data-state=move || state.get().phase_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar view layer should compose structure + headless + motion via `{needle}`."
        );
    }

    for forbidden in [
        "pub fn resolve_state(",
        "pub struct ActionBarState",
        "ui_motion::spring::SpringAnimator::new",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ActionBar view layer should avoid lower-layer reimplementation `{forbidden}`."
        );
    }

    for needle in [
        "pub struct ActionBarMotion",
        "pub fn sanitize_motion(motion: ActionBarMotion) -> ActionBarMotion",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new",
    ] {
        assert!(
            motion_source.contains(needle),
            "ActionBar motion layer should include `{needle}`."
        );
    }

    for forbidden in [
        "view! {",
        "role=",
        "aria-",
        "use_button(",
        "use_focus_ring(",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "ActionBar motion layer should stay mapping/attach only and avoid `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_spec_boundary_reuses_button_spec_without_local_spec_file() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let button_mod_source = load_source("src/button/mod.rs");
    let action_bar_mod_source = load_source("src/action_bar/mod.rs");
    let workspace_dir = workspace_root(manifest_dir);
    let button_spec_in_ui_components = manifest_dir.join("src/button/spec.rs");
    let button_spec_in_components = workspace_dir.join("components/button/src/spec.rs");
    let button_spec_path = if button_spec_in_ui_components.exists() {
        button_spec_in_ui_components.clone()
    } else {
        button_spec_in_components.clone()
    };

    assert!(
        button_spec_in_ui_components.exists() || button_spec_in_components.exists(),
        "button should keep canonical spec.rs boundary for complex schema contract."
    );
    let button_spec_source = fs::read_to_string(&button_spec_path)
        .unwrap_or_else(|e| panic!("read_to_string failed for {button_spec_path:?}: {e}"));
    assert!(
        !manifest_dir.join("src/action_bar/spec.rs").exists(),
        "ActionBar should not introduce a local spec.rs file."
    );

    for needle in [
        "pub mod spec;",
        "pub use spec::{ButtonA11y, ButtonAction, ButtonIntent, ButtonSchema, ButtonSpec, ButtonText};",
    ] {
        assert!(
            button_mod_source.contains(needle),
            "button module should keep canonical spec export `{needle}`."
        );
    }

    for needle in [
        "pub struct ButtonSpec",
        "impl ButtonSpec {",
        "pub fn new() -> Self",
        "pub fn render(self) -> impl IntoView",
    ] {
        assert!(
            button_spec_source.contains(needle),
            "complex-component builder contract should remain in button spec via `{needle}`."
        );
    }

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "ActionBarSpec",
        "ActionBarSchema",
    ] {
        assert!(
            !action_bar_mod_source.contains(forbidden),
            "ActionBar module should stay lightweight and avoid local spec boundary token `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_file_placement_discipline_is_enforced() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = workspace_root(manifest_dir);
    let action_bar_src_dir = workspace_dir.join("components/action-bar/src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        let path = action_bar_src_dir.join(required);
        assert!(
            path.exists(),
            "ActionBar component directory should contain required file `{required}`."
        );
    }

    let forbidden = "render.rs";
    let path = action_bar_src_dir.join(forbidden);
    assert!(
        !path.exists(),
        "ActionBar component directory should forbid legacy file `{forbidden}`."
    );

    let spec_path = action_bar_src_dir.join("spec.rs");
    assert!(
        !spec_path.exists(),
        "ActionBar should keep lightweight boundary and avoid local `spec.rs`."
    );
}

#[test]
fn action_bar_manifest_and_rbi_projection_are_present_and_in_sync() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = workspace_root(manifest_dir);
    let action_bar_src_dir = workspace_dir.join("components/action-bar/src");

    let manifest_path = action_bar_src_dir.join("Component.toml");
    let rbi_path = action_bar_src_dir.join("action_bar.rbi");

    assert!(
        manifest_path.exists(),
        "ActionBar context manifest should exist at {:?}.",
        manifest_path
    );
    assert!(
        rbi_path.exists(),
        "ActionBar RBI projection should exist at {:?}.",
        rbi_path
    );

    let manifest_source = load_source("src/action_bar/Component.toml");
    let rbi_source = load_source("src/action_bar/action_bar.rbi");

    for needle in [
        "name = \"ActionBar\"",
        "name = \"selected_count\"",
        "name = \"default_selected_count\"",
        "name = \"on_selected_count_change\"",
        "name = \"is_force_visible\"",
        "name = \"data-state\"",
        "name = \"data-selection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "ActionBar Component.toml should keep capability/signature token `{needle}`."
        );
    }

    for needle in [
        "pub enum ActionBarPosition",
        "pub enum ActionBarPhase",
        "pub enum ActionBarSelectionKind",
        "pub struct ActionBarMotion",
        "pub fn ActionBar(",
        "is_force_visible: bool",
        "selected_count: Option<leptos::prelude::Signal<usize>>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "ActionBar RBI projection should keep interface token `{needle}`."
        );
    }
}

#[test]
fn action_bar_stays_in_ui_components_assembly_layer_and_public_api_boundary_is_stable() {
    let mod_source = load_source("src/action_bar/mod.rs");
    let logic_source = load_source("src/action_bar/logic.rs");
    let view_source = load_source("src/action_bar/view.rs");
    let styles_source = load_source("src/action_bar/styles.rs");
    let motion_source = load_source("src/action_bar/motion.rs");
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use view::ActionBar;",
        "pub use motion::ActionBarMotion;",
    ] {
        assert!(
            mod_source.contains(needle),
            "ActionBar module should keep ui-components assembly layout token `{needle}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::action_bar::{",
        "pub fn resolve_selection_text(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionBar logic should stay at normalization/mapping layer via `{needle}`."
        );
    }

    for needle in [
        "logic::resolve_view_state(logic::ActionBarViewStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "use ui_button::{Button, ButtonSize, ButtonVariant};",
        "motion::attach_motion(root_ref, visible, motion)",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar view should mount structure/headless/motion assembly via `{needle}`."
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str ="),
        "ActionBar styles should remain static CSS contract in styles.rs."
    );
    assert!(
        motion_source.contains("pub fn attach_motion("),
        "ActionBar motion should expose attach contract in motion.rs."
    );

    for needle in [
        "#[cfg(feature = \"component-action_bar\")]",
        "pub use ui_action_bar as action_bar;",
        "pub use action_bar::{ActionBar, ActionBarMotion, ActionBarPosition};",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components public API should expose ActionBar through feature-gated stable export `{needle}`."
        );
    }

    for forbidden in ["web_sys", "HtmlElement", "JsCast"] {
        assert!(
            !mod_source.contains(forbidden),
            "ActionBar public module boundary must not expose DOM detail token `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_api_naming_uses_is_on_default_prefix_contract() {
    let view_source = load_source("src/action_bar/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "#[prop(optional)] selected_count: Option<Signal<usize>>",
        "#[prop(optional)] default_selected_count: Option<usize>",
        "#[prop(optional)] on_selected_count_change: Option<Callback<usize>>",
        "#[prop(optional)] on_clear_selection: Option<Callback<()>>",
        "#[prop(optional)] is_force_visible: bool",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar API naming contract should include `{needle}`."
        );
    }

    assert!(
        !view_source.contains("#[prop(optional)] force_visible: bool"),
        "ActionBar bool props should use `is_*` prefix; legacy `force_visible` should not remain."
    );

    for needle in [
        "is_force_visible=true",
        "on_selected_count_change=on_selected_count_change",
        "on_clear_selection=clear_selection",
        "default_selected_count=5",
    ] {
        assert!(
            docs_source.contains(needle),
            "ActionBar docs examples should follow naming contract with `{needle}`."
        );
    }
}

#[test]
fn action_bar_selected_count_supports_controlled_and_uncontrolled_contract() {
    let view_source = load_source("src/action_bar/view.rs");

    for needle in [
        "#[prop(optional)] selected_count: Option<Signal<usize>>",
        "#[prop(optional)] default_selected_count: Option<usize>",
        "#[prop(optional)] on_selected_count_change: Option<Callback<usize>>",
        "let selected_count_state = use_controllable_state(",
        "selected_count,",
        "Some(default_selected_count),",
        "on_selected_count_change,",
        "let selected_count = selected_count_state.value;",
        "let request_selected_count_change = selected_count_state.request_change;",
        "request_selected_count_change.run(0);",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar selected_count axis should keep controlled/uncontrolled contract token `{needle}`."
        );
    }

    assert!(
        !view_source.contains("selected_count: Signal<usize>"),
        "ActionBar selected_count should not remain controlled-only."
    );
}

#[test]
fn action_bar_uses_logic_state_model() {
    let logic_source = load_source("src/action_bar/logic.rs");
    let view_source = load_source("src/action_bar/view.rs");

    for needle in [
        "pub use ui_state_primitives::action_bar::{",
        "pub struct ActionBarViewStateInput",
        "pub fn normalize_default_selected_count(",
        "pub fn resolve_view_state(input: ActionBarViewStateInput) -> ActionBarState",
        "normalize_optional_text",
        "normalize_aria_label",
        "normalize_clear_label",
        "normalize_selection_text",
        "resolve_state",
        "pub fn resolve_selection_text(",
        "pub fn compose_class_name(",
        "selection_source_attr",
        "clear_label_source_attr",
        "motion_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionBar logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "i18n::use_ui_i18n()",
        "i18n.strings::<ActionBarStrings>()",
        "let default_selected_count = logic::normalize_default_selected_count(default_selected_count);",
        "let selected_count_state = use_controllable_state(",
        "logic::normalize_aria_label(aria_label, strings.aria_label.as_ref())",
        "logic::normalize_clear_label(clear_label, strings.clear_label.as_ref())",
        "logic::normalize_selection_text(selection_text)",
        "logic::resolve_view_state(logic::ActionBarViewStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "resolve_selection_text(",
        "motion::attach_motion(root_ref, visible, motion)",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn action_bar_state_normalization_is_centralized_in_logic() {
    let logic_source = load_source("src/action_bar/logic.rs");
    let view_source = load_source("src/action_bar/view.rs");
    let styles_source = load_source("src/action_bar/styles.rs");

    for needle in [
        "pub struct ActionBarViewStateInput",
        "pub fn resolve_view_state(input: ActionBarViewStateInput) -> ActionBarState",
        "resolve_state(ActionBarStateInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionBar logic should centralize state normalization via `{needle}`."
        );
    }

    for needle in [
        "logic::resolve_view_state(logic::ActionBarViewStateInput {",
        "selected_count: selected_count.get(),",
        "is_force_visible,",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar view should pass typed state input boundary into logic via `{needle}`."
        );
    }

    for forbidden in [
        "ActionBarStateInput {",
        "logic::resolve_state(ActionBarStateInput {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ActionBar view should not reconstruct primitive state-machine input `{forbidden}`."
        );
    }

    for forbidden in ["resolve_state(", "resolve_view_state("] {
        assert!(
            !styles_source.contains(forbidden),
            "ActionBar styles should consume semantic markers only, not state resolution `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_discrete_state_axes_are_enum_typed() {
    let mod_source = load_source("src/action_bar/mod.rs");
    let view_source = load_source("src/action_bar/view.rs");
    let logic_source = load_source("src/action_bar/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/action_bar.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "pub enum ActionBarPosition",
        "pub enum ActionBarPhase",
        "pub enum ActionBarSelectionKind",
        "pub fn resolve_selection_kind(selected_count: usize) -> ActionBarSelectionKind",
        "position: ActionBarPosition",
        "phase: ActionBarPhase",
        "selection_kind: ActionBarSelectionKind",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ActionBar discrete state primitives should be enum-typed via `{needle}`."
        );
    }

    for needle in [
        "ActionBarPhase, ActionBarPosition, ActionBarSelectionKind",
        "#[prop(optional)] position: ActionBarPosition,",
        "logic::resolve_view_state(logic::ActionBarViewStateInput {",
        "position,",
        "position=ActionBarPosition::Top",
    ] {
        assert!(
            mod_source.contains(needle)
                || view_source.contains(needle)
                || docs_source.contains(needle),
            "ActionBar component/docs should consume enum discrete axes via `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] position: Option<String>",
        "#[prop(optional)] position: String",
        "#[prop(optional)] status: Option<String>",
        "#[prop(optional)] mode: Option<String>",
        "#[prop(optional)] is_top: bool",
        "#[prop(optional)] is_bottom: bool",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ActionBar view should not model mutually-exclusive discrete axes via `{forbidden}`."
        );
    }

    for forbidden in [
        "position: String",
        "phase: String",
        "selection_kind: String",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "ActionBar logic should not downgrade discrete axes into string protocol `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_machine_readable_contract_uses_typed_inputs_and_semantic_markers() {
    let view_source = load_source("src/action_bar/view.rs");
    let logic_source = load_source("src/action_bar/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/action_bar.rs");

    for needle in [
        "pub enum ActionBarPosition",
        "pub enum ActionBarPhase",
        "pub enum ActionBarSelectionKind",
        "pub struct ActionBarStateInput",
        "pub struct ActionBarState",
        "pub struct ActionBarViewStateInput",
        "pub fn resolve_view_state(input: ActionBarViewStateInput) -> ActionBarState",
        "pub fn resolve_selection_kind(selected_count: usize) -> ActionBarSelectionKind",
        "pub fn normalize_default_selected_count(value: Option<usize>) -> usize",
        "#[prop(optional)] selected_count: Option<Signal<usize>>",
        "#[prop(optional)] default_selected_count: Option<usize>",
        "#[prop(optional)] on_selected_count_change: Option<Callback<usize>>",
        "#[prop(optional)] position: ActionBarPosition,",
    ] {
        assert!(
            primitive_source.contains(needle)
                || logic_source.contains(needle)
                || view_source.contains(needle),
            "ActionBar machine-readable contract should keep typed input/model token `{needle}`."
        );
    }

    for needle in [
        "data-state=move || state.get().phase_attr",
        "data-position=move || state.get().position_attr",
        "data-selection=move || state.get().selection_attr",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-selected-count-source=move || state.get().selected_count_source_attr",
        "data-default-selected-count-source=move || state.get().default_selected_count_source_attr",
        "data-selected-count-change-source=move || state.get().selected_count_change_source_attr",
        "data-clear-action-source=move || state.get().clear_action_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar machine-readable contract should expose semantic marker `{needle}`."
        );
    }

    for needle in [
        "match selected_count {",
        "0 => ActionBarSelectionKind::Empty",
        "1 => ActionBarSelectionKind::Single",
        "_ => ActionBarSelectionKind::Multiple",
        "value.unwrap_or_default()",
    ] {
        assert!(
            primitive_source.contains(needle) || logic_source.contains(needle),
            "ActionBar invalid-state normalization should remain centralized via `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] position: Option<String>",
        "#[prop(optional)] position: String",
        "#[prop(optional)] is_top: bool",
        "#[prop(optional)] is_bottom: bool",
        "data-state=\"",
        "data-position=\"",
        "data-selection=\"",
        "data-control-mode=\"",
        "format!(\"data-",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "ActionBar should avoid string-protocol/boolean-explosion marker `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_agent_contract_schema_is_typed_and_whitelisted() {
    let protocol_source = load_source("src/action_bar/protocol.rs");
    let view_source = load_source("src/action_bar/view.rs");
    let manifest_source = load_source("src/action_bar/Component.toml");

    for needle in [
        "#[serde(deny_unknown_fields)]",
        "pub enum ActionBarRenderCapability",
        "pub struct ActionBarRenderPolicy",
        "pub fn render_policy(&self) -> ActionBarRenderPolicy",
        "pub enum ActionBarAgentIntent",
        "pub enum ActionBarAgentAction",
        "pub struct ActionBarAgentDataAttrs",
        "pub const ACTION_BAR_AGENT_SCHEMA: &str = \"ui.action-bar.contract.v1\";",
        "pub fn agent_data_attrs(state: ActionBarState) -> ActionBarAgentDataAttrs",
        "ActionBarAgentIntent::BulkSelection.as_attr()",
        "ActionBarAgentAction::ClearSelection.as_attr()",
    ] {
        assert!(
            protocol_source.contains(needle),
            "ActionBar protocol schema should keep typed contract token `{needle}`."
        );
    }

    for needle in [
        "let render_policy = protocol::ActionBarComponentSpec::default().render_policy();",
        "let on_clear_selection = if allow_clear_action {",
        "let children = if allow_children_slot { children } else { None };",
        "let agent_attrs = Signal::derive(move || protocol::agent_data_attrs(state.get()));",
        "data-ui-schema=move || agent_attrs.get().schema",
        "data-ui-intent=move || agent_attrs.get().intent",
        "data-ui-action=move || agent_attrs.get().action",
        "data-ui-state-phase=move || agent_attrs.get().state_phase",
        "data-ui-state-position=move || agent_attrs.get().state_position",
        "data-ui-state-selection=move || agent_attrs.get().state_selection",
        "data-ui-source-selected-count=move || agent_attrs.get().source_selected_count",
        "data-ui-source-clear-action=move || agent_attrs.get().source_clear_action",
        "data-ui-source-motion=move || agent_attrs.get().source_motion",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar view should mount typed agent-contract marker `{needle}`."
        );
    }

    for forbidden in [
        "format!(\"data-ui-",
        "inner_html=",
        "dangerously_set_inner_html",
        "eval(",
        "new Function(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !protocol_source.contains(forbidden),
            "ActionBar agent-contract path should forbid unsafe/script-like injection token `{forbidden}`."
        );
    }

    for needle in [
        "name = \"data-ui-schema\"",
        "name = \"data-ui-intent\"",
        "name = \"data-ui-action\"",
        "name = \"agent_contract_schema_v1_with_typed_state_source_markers\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "ActionBar manifest should keep traceable agent-contract declaration `{needle}`."
        );
    }
}

#[test]
fn action_bar_default_values_have_single_logic_source() {
    let logic_source = load_source("src/action_bar/logic.rs");
    let view_source = load_source("src/action_bar/view.rs");

    for needle in [
        "pub fn normalize_default_selected_count(value: Option<usize>) -> usize",
        "value.unwrap_or_default()",
        "let default_selected_count = logic::normalize_default_selected_count(default_selected_count);",
        "use_controllable_state(",
        "Some(default_selected_count),",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "ActionBar default normalization contract should include `{needle}`."
        );
    }

    for forbidden in [
        "default_selected_count.unwrap_or(",
        "default_selected_count.unwrap_or_default(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ActionBar view should not contain secondary default fallback `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_status_primitives_are_sourced_from_ui_state_primitives() {
    let source = load_source("src/action_bar/logic.rs");

    for needle in [
        "pub use ui_state_primitives::action_bar::{",
        "ActionBarPhase",
        "ActionBarPosition",
        "ActionBarSelectionKind",
        "ActionBarState",
        "ActionBarStateInput",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar should consume state primitives from ui-state-primitives; missing `{needle}`."
        );
    }
}

#[test]
fn action_bar_status_primitive_boundary_avoids_local_state_machine_and_business_store_coupling() {
    let logic_source = load_source("src/action_bar/logic.rs");
    let view_source = load_source("src/action_bar/view.rs");

    for needle in [
        "pub use ui_state_primitives::action_bar::{",
        "pub fn resolve_view_state(input: ActionBarViewStateInput) -> ActionBarState",
        "resolve_state(ActionBarStateInput {",
        "use ui_headless::use_controllable_state;",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "ActionBar should keep status-primitive boundary token `{needle}`."
        );
    }

    for forbidden in [
        "pub fn resolve_state(",
        "pub fn resolve_selection_kind(",
        "pub struct ActionBarStateInput {",
        "pub enum ActionBarSelectionKind",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "ActionBar logic should not reimplement status primitives locally via `{forbidden}`."
        );
    }

    for forbidden in [
        "GlobalStore",
        "AppStore",
        "BusinessStore",
        "use crate::store",
        "use app::store",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "ActionBar component boundary should not directly couple to business store token `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_has_no_async_loading_protocol_and_keeps_sync_clear_contract() {
    let view_source = load_source("src/action_bar/view.rs");
    let logic_source = load_source("src/action_bar/logic.rs");

    for needle in [
        "#[prop(optional)] on_clear_selection: Option<Callback<()>>",
        "let request_selected_count_change = selected_count_state.request_change;",
        "let on_press = Callback::new(move |_| {",
        "request_selected_count_change.run(0);",
        "on_clear_selection.run(());",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar should keep synchronous clear interaction contract via `{needle}`."
        );
    }

    for forbidden in [
        "use_async_action",
        "is_loading",
        "aria-busy",
        "retry",
        "async fn",
        ".await",
        "Future<",
        "spawn_local",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "ActionBar has no async workflow; forbidden async/loading token `{forbidden}` should be absent."
        );
    }
}

#[test]
fn action_bar_docs_expose_hello_world_path_without_state_machine_wiring() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "let hello_code = Signal::derive(move || {",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<ActionBar default_selected_count=1>",
        "<ActionButton>\"Archive\"</ActionButton>",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar docs should keep DX minimal path `{needle}`."
        );
    }

    for forbidden in ["<ActionBar state=", "ui_state_primitives", "ui-headless"] {
        assert!(
            !source.contains(forbidden),
            "ActionBar docs minimal usage should not require internal wiring token `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_documentation_as_product_keeps_readme_and_beginner_first_flow() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = workspace_root(manifest_dir);
    let readme_path = workspace_dir.join("components/action-bar/src/README.md");
    let readme_source = fs::read_to_string(&readme_path)
        .unwrap_or_else(|e| panic!("read_to_string failed for {readme_path:?}: {e}"));
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "# ActionBar",
        "## Start Here (Hello World)",
        "<ActionBar default_selected_count=1>",
        "## Common Usage",
        "### 1) Controlled",
        "### 2) Uncontrolled",
        "## Learn In Order",
        "default_selected_count",
        "selected_count + on_selected_count_change",
        "advanced props",
        "## Docs Entry",
        "#/components/action-bar",
    ] {
        assert!(
            readme_source.contains(needle),
            "ActionBar README should keep beginner-first documentation token `{needle}`."
        );
    }

    for forbidden in [
        "ui-state-primitives",
        "ui-headless",
        "must wire state primitive first",
    ] {
        assert!(
            !readme_source.contains(forbidden),
            "ActionBar beginner docs should avoid internal-first onboarding token `{forbidden}`."
        );
    }

    for needle in [
        "Playground title=\"Hello World\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Top placement + custom text + reduced motion\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "ActionBar docs-app page should keep beginner->advanced progression token `{needle}`."
        );
    }
}

#[test]
fn action_bar_is_not_llm_output_surface_and_has_no_streaming_snapshot_modes() {
    let view_source = load_source("src/action_bar/view.rs");
    let protocol_source = load_source("src/action_bar/protocol.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for forbidden in [
        "#[prop(optional)] is_streaming: bool",
        "#[prop(optional)] streaming: bool",
        "#[prop(optional)] mode: OutputMode",
        "OutputMode::Streaming",
        "token_chunk",
        "on_chunk",
        "data-stream-state",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !protocol_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "ActionBar should not expose LLM output rendering mode token `{forbidden}`."
        );
    }

    for needle in [
        "#[prop(optional)] selected_count: Option<Signal<usize>>",
        "#[prop(optional)] default_selected_count: Option<usize>",
        "#[prop(optional)] selection_text: Option<String>",
        "data-selected-count=move || state.get().selected_count.to_string()",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar should keep full-state snapshot-style input contract `{needle}`."
        );
    }
}

#[test]
fn action_bar_streaming_is_optional_with_snapshot_fallback_and_status_markers() {
    let protocol_source = load_source("src/action_bar/protocol.rs");
    let view_source = load_source("src/action_bar/view.rs");
    let manifest_source = load_source("src/action_bar/Component.toml");

    for needle in [
        "pub enum ActionBarStreamingPolicy",
        "ActionBarStreamingPolicy::Optional => \"optional\"",
        "pub enum ActionBarStreamingFallback",
        "ActionBarStreamingFallback::Snapshot => \"snapshot\"",
        "pub enum ActionBarOutputMode",
        "ActionBarOutputMode::Snapshot => \"snapshot\"",
        "pub enum ActionBarOutputStatus",
        "ActionBarOutputStatus::Validated => \"validated\"",
        "streaming_policy: ActionBarStreamingPolicy::Optional.as_attr()",
        "streaming_fallback: ActionBarStreamingFallback::Snapshot.as_attr()",
        "output_mode: ActionBarOutputMode::Snapshot.as_attr()",
        "output_status: ActionBarOutputStatus::Validated.as_attr()",
    ] {
        assert!(
            protocol_source.contains(needle),
            "ActionBar protocol should keep streaming-optional contract token `{needle}`."
        );
    }

    for needle in [
        "data-ui-streaming-policy=move || agent_attrs.get().streaming_policy",
        "data-ui-streaming-fallback=move || agent_attrs.get().streaming_fallback",
        "data-ui-output-mode=move || agent_attrs.get().output_mode",
        "data-ui-output-status=move || agent_attrs.get().output_status",
        "role=\"toolbar\"",
        "aria-hidden=move || state.get().is_hidden.then_some(\"true\")",
        "data-state=move || state.get().phase_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar view should expose streaming optional/snapshot status marker `{needle}`."
        );
    }

    for forbidden in [
        "ActionBarStreamingPolicy::Required",
        "streaming_policy: ActionBarStreamingPolicy::Required",
        "ActionBarOutputMode::Streaming",
    ] {
        assert!(
            !protocol_source.contains(forbidden),
            "ActionBar should not claim streaming-required contract token `{forbidden}`."
        );
    }

    for needle in [
        "name = \"data-ui-streaming-policy\"",
        "name = \"data-ui-streaming-fallback\"",
        "name = \"data-ui-output-mode\"",
        "name = \"data-ui-output-status\"",
        "name = \"streaming_optional_with_snapshot_fallback_and_output_status_markers\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "ActionBar manifest should declare streaming optional capability token `{needle}`."
        );
    }
}

#[test]
fn action_bar_rust_hygiene_avoids_unwrap_expect_let_underscore_and_string_clone_churn() {
    let logic_source = load_source("src/action_bar/logic.rs");
    let view_source = load_source("src/action_bar/view.rs");
    let motion_source = load_source("src/action_bar/motion.rs");
    let i18n_source = load_source("src/action_bar/i18n.rs");
    let protocol_source = load_source("src/action_bar/protocol.rs");

    for source in [
        &logic_source,
        &view_source,
        &motion_source,
        &i18n_source,
        &protocol_source,
    ] {
        for forbidden in [".unwrap(", ".expect(", ".unwrap_err(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "ActionBar non-test source must not contain forbidden hygiene token `{forbidden}`."
            );
        }
    }

    for needle in [
        "use std::borrow::Cow;",
        "Cow::Borrowed(\"ui-action-bar\")",
        "Cow::Borrowed(\"ui-action-bar--clearable\")",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionBar class assembly should keep Cow-based string hygiene token `{needle}`."
        );
    }

    for forbidden in [
        "\"ui-action-bar\".to_string()",
        "\"ui-action-bar--clearable\".to_string()",
        "\"ui-action-bar--label-custom\".to_string()",
        "\"ui-action-bar--selection-custom\".to_string()",
        "\"ui-action-bar--clear-label-custom\".to_string()",
        "\"ui-action-bar--motion-custom\".to_string()",
        "\"ui-action-bar--custom-class\".to_string()",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "ActionBar logic should avoid avoidable string clone hotspot `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_snapshot_mode_can_consume_complete_input_and_render_stably() {
    let view_source = load_source("src/action_bar/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "#[prop(optional)] selected_count: Option<Signal<usize>>",
        "#[prop(optional)] default_selected_count: Option<usize>",
        "#[prop(optional)] on_selected_count_change: Option<Callback<usize>>",
        "#[prop(optional)] on_clear_selection: Option<Callback<()>>",
        "#[prop(optional)] position: ActionBarPosition,",
        "#[prop(optional)] is_force_visible: bool",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] clear_label: Option<String>",
        "#[prop(optional, into)] selection_text: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional)] motion: ActionBarMotion",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] children: Option<Children>",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar snapshot baseline should accept complete config field `{needle}`."
        );
    }

    for needle in [
        "data-state=move || state.get().phase_attr",
        "data-position=move || state.get().position_attr",
        "data-selection=move || state.get().selection_attr",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-selected-count-source=move || state.get().selected_count_source_attr",
        "data-ui-schema=move || agent_attrs.get().schema",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar snapshot baseline should emit stable semantic marker `{needle}`."
        );
    }

    for needle in [
        "selected_count=selected_count_signal",
        "on_selected_count_change=on_selected_count_change",
        "on_clear_selection=clear_selection",
        "aria_label=\"Bulk actions\".to_string()",
        "class_name=\"docs-action-bar\".to_string()",
        "default_selected_count=5",
        "position=ActionBarPosition::Top",
        "is_force_visible=true",
        "selection_text=\"Rows selected\".to_string()",
        "clear_label=\"Clear all\".to_string()",
        "motion=ActionBarMotion::disabled()",
    ] {
        assert!(
            docs_source.contains(needle),
            "ActionBar docs should provide complete snapshot-style config example `{needle}`."
        );
    }
}

#[test]
fn action_bar_composition_api_prefers_explicit_parent_item_structure() {
    let view_source = load_source("src/action_bar/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "#[prop(optional)] children: Option<Children>",
        "{children.map(|children| children())}",
        "<ActionBar default_selected_count=1>",
        "<ActionButton>\"Archive\"</ActionButton>",
        "<ActionButton>\"Delete\"</ActionButton>",
        "<ActionButton is_quiet=true>\"Archive\"</ActionButton>",
    ] {
        assert!(
            view_source.contains(needle) || docs_source.contains(needle),
            "ActionBar composition contract should keep explicit parent/item structure via `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] labels: Vec<",
        "#[prop(optional)] titles: Vec<",
        "#[prop(optional)] panels: Vec<",
        "#[prop(optional)] items: Vec<String>",
        "labels=vec![",
        "titles=vec![",
        "panels=vec![",
        "ItemSpec",
    ] {
        assert!(
            !view_source.contains(forbidden) && !docs_source.contains(forbidden),
            "ActionBar should not use parallel-array/config sugar composition `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_a11y_i18n_and_locale_contract_is_wired() {
    let view_source = load_source("src/action_bar/view.rs");
    let i18n_source = load_source("src/action_bar/i18n.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "use ui_headless::i18n;",
        "use ui_headless::{A11yDirection, locale_attrs};",
        "let i18n = i18n::use_ui_i18n();",
        "let strings = i18n.strings::<ActionBarStrings>();",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] clear_label: Option<String>,",
        "#[prop(optional, into)] selection_text: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "logic::normalize_aria_label(aria_label, strings.aria_label.as_ref())",
        "logic::normalize_clear_label(clear_label, strings.clear_label.as_ref())",
        "logic::normalize_selection_text(selection_text)",
        "let locale = locale_attrs(lang, dir);",
        "role=\"toolbar\"",
        "aria-label=aria_label",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "<Button",
        "variant=ButtonVariant::Link",
        "aria_label=clear_label_attr",
        "on_press=on_press",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar A11y/i18n/locale wiring should include `{needle}`."
        );
    }

    for needle in [
        "pub struct ActionBarStrings",
        "pub aria_label: Arc<str>",
        "pub clear_label: Arc<str>",
        "pub selection_empty_label: Arc<str>",
        "pub selection_single_label: Arc<str>",
        "pub selection_multiple_template: Arc<str>",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_CLEAR_LABEL",
        "DEFAULT_SELECTION_EMPTY_LABEL",
        "DEFAULT_SELECTION_SINGLE_LABEL",
        "DEFAULT_SELECTION_MULTIPLE_SUFFIX",
    ] {
        assert!(
            i18n_source.contains(needle),
            "ActionBar i18n strings contract should include `{needle}`."
        );
    }

    assert!(
        a11y_source
            .contains("pub fn locale_attrs(lang: Option<String>, dir: Option<A11yDirection>)"),
        "ui-headless shared a11y utilities should provide locale_attrs."
    );

    for forbidden in [
        "\"Actions\"",
        "\"Clear selection\"",
        "\"No items selected\"",
        "\"items selected\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ActionBar view should not hardcode user-facing fallback text `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_mounts_headless_contract_in_view_not_logic_layer() {
    let view_source = load_source("src/action_bar/view.rs");
    let logic_source = load_source("src/action_bar/logic.rs");

    for needle in [
        "use ui_headless::i18n;",
        "use ui_button::{Button, ButtonSize, ButtonVariant};",
        "<Button",
        "variant=ButtonVariant::Link",
        "size=ButtonSize::S",
        "class_name=\"ui-action-bar__clear\".to_string()",
        "aria_label=clear_label_attr",
        "on_press=on_press",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar view should reuse Button contract for clear action; missing `{needle}`."
        );
    }

    for forbidden in [
        "use_button(",
        "use_focus_ring(",
        "use_hover(",
        "ButtonOptions {",
        "FocusRingOptions::default()",
        "HoverOptions::default()",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "ActionBar logic should remain pure mapping and must not mount headless hooks `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_emits_toolbar_semantics_and_state_attributes() {
    let source = load_source("src/action_bar/view.rs");

    for needle in [
        "data-slot=\"action-bar\"",
        "data-state=move || state.get().phase_attr",
        "data-position=move || state.get().position_attr",
        "data-selection=move || state.get().selection_attr",
        "data-selected-count=move || state.get().selected_count.to_string()",
        "data-visible=move || state.get().is_visible.then_some(\"true\")",
        "data-hidden=move || state.get().is_hidden.then_some(\"true\")",
        "data-top=move || state.get().is_top.then_some(\"true\")",
        "data-bottom=move || state.get().is_bottom.then_some(\"true\")",
        "data-controlled=move || state.get().is_controlled_selected_count.then_some(\"true\")",
        "data-uncontrolled=move || state.get().is_uncontrolled_selected_count.then_some(\"true\")",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-selected-count-source=move || state.get().selected_count_source_attr",
        "data-default-selected-count-source=move || state.get().default_selected_count_source_attr",
        "data-selected-count-change-source=move || state.get().selected_count_change_source_attr",
        "data-has-clear=move || state.get().has_clear_action.then_some(\"true\")",
        "data-clear-action-source=move || state.get().clear_action_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-selection-source=move || state.get().selection_source_attr",
        "data-clear-label-source=move || state.get().clear_label_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "role=\"toolbar\"",
        "aria-hidden=move || state.get().is_hidden.then_some(\"true\")",
        "data-slot=\"action-bar-clear\"",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar should emit `{needle}` for baseline-style contract and tooling."
        );
    }
}

#[test]
fn action_bar_state_markers_are_observable_and_closed_set_contracts() {
    let view_source = load_source("src/action_bar/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/action_bar.rs");

    for needle in [
        "data-state=move || state.get().phase_attr",
        "data-position=move || state.get().position_attr",
        "data-selection=move || state.get().selection_attr",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-selected-count-source=move || state.get().selected_count_source_attr",
        "data-default-selected-count-source=move || state.get().default_selected_count_source_attr",
        "data-selected-count-change-source=move || state.get().selected_count_change_source_attr",
        "data-clear-action-source=move || state.get().clear_action_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-selection-source=move || state.get().selection_source_attr",
        "data-clear-label-source=move || state.get().clear_label_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar should expose observable semantic marker `{needle}`."
        );
    }

    for closed_set_case in [
        "\"controlled\"",
        "\"uncontrolled\"",
        "\"external\"",
        "\"default\"",
        "\"provided\"",
        "\"implicit\"",
        "\"none\"",
        "ActionBarPhase::Visible => \"visible\"",
        "ActionBarPhase::Hidden => \"hidden\"",
        "ActionBarPosition::Top => \"top\"",
        "ActionBarPosition::Bottom => \"bottom\"",
        "ActionBarSelectionKind::Empty => \"empty\"",
        "ActionBarSelectionKind::Single => \"single\"",
        "ActionBarSelectionKind::Multiple => \"multiple\"",
    ] {
        assert!(
            primitive_source.contains(closed_set_case),
            "ActionBar marker values should be enumerable closed sets; missing `{closed_set_case}`."
        );
    }

    for forbidden in [
        "data-control-mode=\"",
        "data-selected-count-source=\"",
        "data-default-selected-count-source=\"",
        "data-selected-count-change-source=\"",
        "data-clear-action-source=\"",
        "format!(\"data-",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ActionBar markers should come from typed derived state, not ad-hoc literals `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_styles_include_position_state_and_source_contracts() {
    let source = load_source("src/action_bar/styles.rs");

    for selector in [
        ".ui-action-bar--position-bottom",
        ".ui-action-bar[data-position=\"bottom\"]",
        ".ui-action-bar--position-top",
        ".ui-action-bar[data-position=\"top\"]",
        ".ui-action-bar--state-hidden",
        ".ui-action-bar[data-state=\"hidden\"]",
        ".ui-action-bar[data-hidden=\"true\"]",
        ".ui-action-bar--selection-custom",
        ".ui-action-bar[data-selection-source=\"custom\"]",
        ".ui-action-bar--clear-label-custom",
        ".ui-action-bar[data-clear-label-source=\"custom\"]",
        ".ui-action-bar--motion-custom",
        ".ui-action-bar[data-motion-source=\"custom\"]",
        ".ui-action-bar--custom-class",
        ".ui-action-bar[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ActionBar styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn action_bar_token_first_styles_are_static_and_aggregated_via_ui_root() {
    let styles_source = load_source("src/action_bar/styles.rs");
    let view_source = load_source("src/action_bar/view.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-fg)",
        "var(--ui-border)",
        "var(--ui-shadow-md)",
        ".ui-action-bar[data-state=\"hidden\"]",
        ".ui-action-bar[data-selection=\"multiple\"] .ui-action-bar__selection",
    ] {
        assert!(
            styles_source.contains(needle),
            "ActionBar styles should remain token-first/static and include `{needle}`.",
        );
    }

    for forbidden in [
        "view! {",
        "Callback::new",
        "format!(",
        "@apply",
        "styled(",
        "css!(",
        "tailwind",
        "tw-",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "ActionBar styles should avoid runtime or utility/CSS-in-Rust default patterns `{forbidden}`.",
        );
    }

    assert!(
        !view_source.contains(" style="),
        "ActionBar view should avoid inline business style logic and rely on static styles.rs contracts.",
    );
    assert!(
        !view_source.contains("style:"),
        "ActionBar view should avoid inline style directives and keep runtime numeric updates in motion CSS custom properties.",
    );

    for needle in [
        "@layer ui {",
        "#[cfg(feature = \"component-action_bar\")]",
        "out.push_str(crate::action_bar::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ActionBar styles must be aggregated via css.rs feature-gated registry token `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional)] inject_components_css: bool",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should remain the centralized component-css injection path via `{needle}`.",
        );
    }
}

#[test]
fn action_bar_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let view_source = load_source("src/action_bar/view.rs");
    let styles_source = load_source("src/action_bar/styles.rs");
    let motion_source = load_source("src/action_bar/motion.rs");

    for needle in [
        "data-state=move || state.get().phase_attr",
        "data-position=move || state.get().position_attr",
        "data-selection=move || state.get().selection_attr",
        "data-hidden=move || state.get().is_hidden.then_some(\"true\")",
        "data-has-clear=move || state.get().has_clear_action.then_some(\"true\")",
        "data-selection-source=move || state.get().selection_source_attr",
        "data-clear-label-source=move || state.get().clear_label_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "<Button",
        "variant=ButtonVariant::Link",
        "class_name=\"ui-action-bar__clear\".to_string()",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar view should expose explicit style marker `{needle}`."
        );
    }

    for selector in [
        ".ui-action-bar[data-position=\"bottom\"]",
        ".ui-action-bar[data-position=\"top\"]",
        ".ui-action-bar[data-state=\"hidden\"]",
        ".ui-action-bar[data-hidden=\"true\"]",
        ".ui-action-bar[data-selection=\"empty\"] .ui-action-bar__selection",
        ".ui-action-bar[data-selection=\"single\"] .ui-action-bar__selection",
        ".ui-action-bar[data-selection=\"multiple\"] .ui-action-bar__selection",
        ".ui-action-bar[data-has-clear=\"true\"]",
        ".ui-action-bar[data-label-source=\"custom\"]",
        ".ui-action-bar[data-selection-source=\"custom\"]",
        ".ui-action-bar[data-clear-label-source=\"custom\"]",
        ".ui-action-bar[data-motion-source=\"custom\"]",
        ".ui-action-bar[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "ActionBar styles should consume explicit marker selector `{selector}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":first-child", ":last-child"] {
        assert!(
            !styles_source.contains(forbidden),
            "ActionBar styles should not guess state from brittle structural selector `{forbidden}`."
        );
    }

    assert!(
        !view_source.contains(" style="),
        "ActionBar view should not inline business style decisions."
    );

    for needle in [
        "set_property(\"--ui-action-bar-translate-y\"",
        "set_property(\"--ui-action-bar-opacity\"",
    ] {
        assert!(
            motion_source.contains(needle),
            "ActionBar runtime style updates should be CSS-variable-only via `{needle}`."
        );
    }

    for forbidden in [
        "set_property(\"color\"",
        "set_property(\"background\"",
        "set_property(\"display\"",
        "set_property(\"border\"",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "ActionBar motion runtime should not inject business style property `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_semantics_suite_prioritizes_contract_assertions_over_snapshots() {
    let suite_source = load_source("tests/action_bar_semantics.rs");

    for semantic_signal in [
        "action_bar_emits_toolbar_semantics_and_state_attributes",
        "action_bar_state_markers_are_observable_and_closed_set_contracts",
        "action_bar_styles_depend_on_explicit_state_markers_not_dom_guessing",
        "action_bar_selected_count_supports_controlled_and_uncontrolled_contract",
        "action_bar_mounts_headless_contract_in_view_not_logic_layer",
    ] {
        assert!(
            suite_source.contains(semantic_signal),
            "ActionBar semantic suite should keep contract assertion signal `{semantic_signal}`."
        );
    }

    let forbidden_assert_snapshot = ["assert", "_", "snapshot", "!"].concat();
    let forbidden_insta = ["in", "sta", "::"].concat();
    let forbidden_match_snapshot = ["to", "_", "match", "_", "snapshot"].concat();

    for forbidden in [
        forbidden_assert_snapshot,
        forbidden_insta,
        forbidden_match_snapshot,
    ] {
        assert!(
            !suite_source.contains(&forbidden),
            "ActionBar semantic suite should not rely on visual snapshot token `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_semantics_cover_data_aria_and_interaction_matrix() {
    let suite_source = load_source("tests/action_bar_semantics.rs");
    let view_source = load_source("src/action_bar/view.rs");
    let motion_source = load_source("src/action_bar/motion.rs");

    for semantic_test in [
        "action_bar_selected_count_supports_controlled_and_uncontrolled_contract",
        "action_bar_mounts_headless_contract_in_view_not_logic_layer",
        "action_bar_emits_toolbar_semantics_and_state_attributes",
        "action_bar_state_markers_are_observable_and_closed_set_contracts",
        "action_bar_motion_stays_as_mapping_and_attach_layer",
        "action_bar_semantics_suite_prioritizes_contract_assertions_over_snapshots",
    ] {
        assert!(
            suite_source.contains(semantic_test),
            "ActionBar semantic suite should include `{semantic_test}`."
        );
    }

    for semantic_marker in [
        "role=\"toolbar\"",
        "aria-hidden=move || state.get().is_hidden.then_some(\"true\")",
        "data-slot=\"action-bar-clear\"",
        "data-state=move || state.get().phase_attr",
        "data-selection=move || state.get().selection_attr",
        "data-controlled=move || state.get().is_controlled_selected_count.then_some(\"true\")",
        "data-uncontrolled=move || state.get().is_uncontrolled_selected_count.then_some(\"true\")",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-selected-count-source=move || state.get().selected_count_source_attr",
        "<Button",
        "variant=ButtonVariant::Link",
        "on_press=on_press",
    ] {
        assert!(
            view_source.contains(semantic_marker),
            "ActionBar semantic contract should expose `{semantic_marker}`."
        );
    }

    for platform_marker in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            motion_source.contains(platform_marker),
            "ActionBar motion contract should keep wasm/non-wasm branch marker `{platform_marker}`."
        );
    }
}

#[test]
fn action_bar_theme_contract_is_token_first_and_ui_theme_owned() {
    let styles_source = load_source("src/action_bar/styles.rs");
    let tokens_source = load_source("../../crates/ui-theme/src/tokens.rs");
    let theme_source = load_source("../../crates/ui-theme/src/theme.rs");
    let css_source = load_source("../../crates/ui-theme/src/css.rs");
    let styling_spec = load_source("../../docs/spec/styling.md");

    for needle in [
        "var(--ui-space-sm)",
        "var(--ui-space-md)",
        "var(--ui-space-xl)",
        "var(--ui-border)",
        "var(--ui-bg-muted)",
        "var(--ui-bg)",
        "var(--ui-fg)",
        "var(--ui-accent)",
        "var(--ui-accent-soft)",
        "var(--ui-focus-ring)",
        "var(--ui-radius-sm)",
        "var(--ui-radius-lg)",
        "var(--ui-shadow-md)",
    ] {
        assert!(
            styles_source.contains(needle),
            "ActionBar styles should consume ui-theme CSS variables via `{needle}`."
        );
    }

    for fallback_chain in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-xl, var(--ui-fallback-space-xl))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-shadow-md, var(--ui-fallback-shadow-md))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
    ] {
        assert!(
            styles_source.contains(fallback_chain),
            "ActionBar styles should keep defensive fallback chains via `{fallback_chain}`."
        );
    }

    for needle in [
        "pub enum ThemeSystem",
        "pub enum ThemeColor",
        "pub enum ThemeScale",
        "pub struct ThemeContext",
    ] {
        assert!(
            theme_source.contains(needle),
            "ui-theme should own the three-axis theme context contract via `{needle}`."
        );
    }

    for needle in [
        "pub struct ThemeTokens",
        "pub struct LayoutTokens",
        "pub struct SemanticColorTokens",
    ] {
        assert!(
            tokens_source.contains(needle),
            "ui-theme token taxonomy should be defined in tokens.rs via `{needle}`."
        );
    }

    for needle in [
        "--ui-system:",
        "--ui-color:",
        "--ui-scale:",
        "--ui-space-sm:",
        "--ui-space-md:",
        "--ui-radius-lg:",
        "--ui-shadow-md:",
        "--ui-space-xl:",
        "--ui-fallback-space-xl:",
        "--ui-fallback-bg:",
        "--ui-fallback-shadow-md:",
        "--ui-fallback-focus-ring:",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-theme css emitter should output theme/token variables including `{needle}`."
        );
    }

    for needle in [
        "`crates/ui-theme/src/tokens.rs`",
        "`crates/ui-theme/src/theme.rs`",
        "`crates/ui-theme/src/css.rs`",
    ] {
        assert!(
            styling_spec.contains(needle),
            "styling spec should keep traceability to ui-theme source of truth `{needle}`."
        );
    }
}

#[test]
fn action_bar_visual_desire_reuses_theme_visual_baseline_gate() {
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let baseline_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_e2e_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let action_bar_docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "component_doc!(\n        \"ThemeVisualBaseline\",",
        "\"theme-visual-baseline\",",
        "pub(super) fn theme_visual_baseline() -> AnyView",
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline",
        "hierarchy, spacing rhythm, contrast layers, and interactive feedback",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button variant=ButtonVariant::Accent>",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            baseline_registry_source.contains(needle) || baseline_page_source.contains(needle),
            "theme visual baseline docs gate should include `{needle}`.",
        );
    }

    for needle in [
        "E2E_VISUAL_BASELINE",
        "/#/components/theme-visual-baseline",
        "[data-slot=\"theme-visual-baseline\"]",
        "[data-slot=\"theme-visual-baseline-button\"]",
        "[data-slot=\"theme-visual-baseline-input\"]",
        "[data-slot=\"theme-visual-baseline-overlay\"]",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
    ] {
        assert!(
            baseline_e2e_source.contains(needle),
            "theme visual baseline e2e regression gate should include `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn action_bar() -> AnyView",
        "title=\"ActionBar\"",
        "slug=\"action-bar\"",
        "description=\"Bulk-action surface with baseline-style selection contracts and baseline-level spring visibility motion.\"",
    ] {
        assert!(
            action_bar_docs_source.contains(needle),
            "ActionBar docs entry should stay under default-theme quality gate `{needle}`.",
        );
    }
}

#[test]
fn action_bar_visual_desire_heroui_alignment_targets_experience_not_api_copy() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let action_bar_docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "# HeroUI 参数设计风格对齐策略",
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
        "保持简洁参数面与显式槽位组合",
        "避免并行数组式隐式约定",
    ] {
        assert!(
            strategy_source.contains(needle),
            "HeroUI strategy should include alignment constraint `{needle}`.",
        );
    }

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<ActionBar default_selected_count=1>",
        "<ActionButton>\"Archive\"</ActionButton>",
        "<Playground title=\"Top placement + custom text + reduced motion\" code_signal=state_code>",
        "position=ActionBarPosition::Top",
        "motion=ActionBarMotion::disabled()",
    ] {
        assert!(
            action_bar_docs_source.contains(needle),
            "ActionBar docs should keep simple-first + advanced-on-demand progression token `{needle}`.",
        );
    }

    for forbidden in ["Bootstrap", "btn-default", "panel-default", "well well-"] {
        assert!(
            !action_bar_docs_source.contains(forbidden),
            "ActionBar docs should avoid legacy visual-regression token `{forbidden}`.",
        );
    }
}

#[test]
fn action_bar_heroui_strategy_and_component_docs_stay_in_sync_for_parameter_changes() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let action_bar_docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "### ActionBar 同步记录（2026-02-20）",
        "selected_count + on_selected_count_change + default_selected_count",
        "on_clear_selection",
        "position",
        "is_force_visible",
        "selection_text",
        "clear_label",
        "component_doc!(\"ActionBar\", \"action-bar\", \"Actions\", ax::action_bar)",
        "#/components/action-bar",
        "参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "ActionBar HeroUI strategy sync record should contain `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"ActionBar\", \"action-bar\", \"Actions\", ax::action_bar)",
        "component_doc!(\"ActionButton\", \"action-button\", \"Actions\", a::action_button)",
    ] {
        assert!(
            pages_source.contains(needle),
            "components catalog should keep indexable ActionBar docs entry token `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn action_bar() -> AnyView",
        "title=\"ActionBar\"",
        "slug=\"action-bar\"",
    ] {
        assert!(
            action_bar_docs_source.contains(needle),
            "ActionBar docs page should remain accessible and indexable via `{needle}`."
        );
    }
}

#[test]
fn action_bar_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "component-action_bar = [\"component-button\", \"dep:ui-action-bar\"]",
        "inject-css = []",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components Cargo features should include `{needle}` for tree-shaking boundaries."
        );
    }

    assert!(
        lib_source.contains(
            "#[cfg(feature = \"component-action_bar\")]\npub use ui_action_bar as action_bar;"
        ),
        "lib.rs should feature-gate action_bar module export for tree-shaking.",
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-action_bar\")]")
            && css_source.contains("out.push_str(crate::action_bar::styles::CSS);"),
        "css.rs should gate action_bar CSS aggregation behind component-action_bar feature."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("pub fn push_components_css(out: &mut String)"),
        "css.rs should keep top-level inject-css gate for component CSS injection."
    );

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("web-demo-components")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui-components via web-demo-components, not all-components."
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components instead of implicit default pull-up."
    );
}

#[test]
fn action_bar_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features",
        "cargo tree -e features -i ui-components -p web-demo",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "size regression",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking check script should include `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`."
        );
    }
}

#[test]
fn action_bar_entrypoint_files_and_headless_boundaries_are_stable() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "#[cfg(feature = \"component-action_bar\")]",
        "pub use ui_action_bar as action_bar;",
        "pub use root::UiRoot;",
        "pub use action_bar::{ActionBar, ActionBarMotion, ActionBarPosition};",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep stable boundary token `{needle}`."
        );
    }

    for forbidden in ["web_sys::", "HtmlElement", "JsCast"] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-components public lib boundary should not expose platform detail `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "@layer ui {",
        "#[cfg(feature = \"component-action_bar\")]",
        "out.push_str(crate::action_bar::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css entry should keep feature-gated aggregation token `{needle}`."
        );
    }

    for needle in [
        "use ui_theme::{SemanticOverrides, Theme, css};",
        "use ui_headless::{UiI18n, provide_ui_i18n, provide_ui_id_provider};",
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot entry should centralize theme/css/i18n injection via `{needle}`."
        );
    }

    for needle in [
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringAnimator::new",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep generic motion capability token `{needle}`."
        );
    }

    for forbidden in ["ActionBar", "role=", "aria-", "on_press"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight shared primitive should not include component business semantics `{forbidden}`."
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace = workspace_root(manifest_dir);
    for forbidden_path in [
        "crates/ui-components/src/overlay_open.rs",
        "crates/ui-components/src/presence.rs",
        "crates/ui-components/src/a11y.rs",
    ] {
        let path = workspace.join(forbidden_path);
        assert!(
            !path.exists(),
            "ui-components should not host deprecated/shared primitive file `{forbidden_path}`."
        );
    }

    for expected_path in [
        "crates/ui-headless/src/controllable_state.rs",
        "crates/ui-headless/src/presence.rs",
        "crates/ui-headless/src/a11y.rs",
    ] {
        let path = workspace.join(expected_path);
        assert!(
            path.exists(),
            "headless shared primitive file should exist at `{expected_path}`."
        );
    }
}

#[test]
fn action_bar_platform_guards_keep_cfg_split_and_non_wasm_web_sys_free() {
    let motion_source = load_source("src/action_bar/motion.rs");
    let mod_source = load_source("src/action_bar/mod.rs");
    let i18n_source = load_source("src/action_bar/i18n.rs");
    let logic_source = load_source("src/action_bar/logic.rs");
    let styles_source = load_source("src/action_bar/styles.rs");
    let view_source = load_source("src/action_bar/view.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "let element: leptos::web_sys::HtmlElement = node.unchecked_into();",
    ] {
        assert!(
            motion_source.contains(needle),
            "ActionBar motion should keep explicit platform branch marker `{needle}`."
        );
    }

    let forbidden = "web_sys";
    assert!(
        !mod_source.contains(forbidden)
            && !i18n_source.contains(forbidden)
            && !logic_source.contains(forbidden)
            && !styles_source.contains(forbidden)
            && !view_source.contains(forbidden),
        "non-wasm ActionBar files should stay browser-object free; found `{forbidden}` outside motion.rs.",
    );
}

#[test]
fn action_bar_platform_check_script_covers_default_ssr_wasm_compile_paths() {
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "cargo check -p ui-components",
        "cargo check -p ui-components --no-default-features --features component-action_bar,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-action_bar,inject-css",
        "components/action-bar/src/view.rs",
        "components/action-bar/src/motion.rs",
        "cfg(target_arch = \"wasm32\")",
        "cfg(not(target_arch = \"wasm32\"))",
    ] {
        assert!(
            script_source.contains(needle),
            "platform check script should include `{needle}`."
        );
    }
}

#[test]
fn action_bar_ui_headless_feature_mutex_contract_is_guarded() {
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless feature mutex should be guarded in lib.rs by `{needle}`."
        );
    }

    for needle in [
        "echo \"[platform] compile guard: ui-headless web+ssr must fail\"",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "mutually exclusive",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            script_source.contains(needle),
            "platform guard script should enforce ui-headless feature mutex and dual compile paths via `{needle}`."
        );
    }
}

#[test]
fn action_bar_ui_motion_non_wasm_noop_stub_contract_is_guarded() {
    let motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let action_bar_motion_source = load_source("src/action_bar/motion.rs");
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion non-wasm no-op/stub contract should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "_node_ref: leptos::prelude::NodeRef<leptos::html::Div>",
        "_visible: leptos::prelude::Signal<bool>",
        "_motion: ActionBarMotion",
        ") {",
        "}",
    ] {
        assert!(
            action_bar_motion_source.contains(needle),
            "ActionBar non-wasm motion path should keep predictable safe degrade via `{needle}`."
        );
    }

    for needle in [
        "echo \"[platform] compile-only: ui-motion native path\"",
        "cargo check -p ui-motion",
        "echo \"[platform] ui-motion non-wasm stub tests\"",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            script_source.contains(needle),
            "platform script should keep ui-motion non-wasm compile/tooling guards via `{needle}`."
        );
    }
}

#[test]
fn action_bar_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let view_source = load_source("src/action_bar/view.rs");
    let motion_source = load_source("src/action_bar/motion.rs");
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "!motion.enabled || ui_motion::web::prefers_reduced_motion()",
        "translate.clear_on_rest();",
        "opacity.clear_on_rest();",
        "translate.set_target(0.0);",
        "opacity.set_target(1.0);",
        "translate.set_target(motion.hidden_translate_px);",
        "opacity.set_target(motion.hidden_opacity);",
    ] {
        assert!(
            motion_source.contains(needle),
            "reduced-motion path should keep deterministic minimal motion fallback via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion layer should keep explicit wasm/non-wasm branch split via `{needle}`."
        );
    }

    for forbidden in ["data-state", "aria-", "role=", "set_attribute(\"aria-\""] {
        assert!(
            !motion_source.contains(forbidden),
            "motion layer should not mutate semantic contract tokens `{forbidden}`."
        );
    }

    for needle in [
        "let state = Signal::derive(move || {",
        "motion::attach_motion(root_ref, visible, motion);",
        "role=\"toolbar\"",
        "data-state=move || state.get().phase_attr",
        "data-position=move || state.get().position_attr",
        "data-selection=move || state.get().selection_attr",
        "aria-label=aria_label",
        "aria-hidden=move || state.get().is_hidden.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "view layer should keep SSR/hydration semantic markers stable via `{needle}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "web_sys",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view layer semantics should not split by platform token `{forbidden}`."
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-action_bar,inject-css",
    ] {
        assert!(
            script_source.contains(needle),
            "platform check script should keep SSR/wasm compile-only guard `{needle}`."
        );
    }
}

#[test]
fn action_bar_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/action_bar/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_source("src/action_bar/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "\"action-bar\" => UiPerfBudget {",
        "max_mount_ms: 34.0,",
        "max_update_ms: Some(12.0),",
        "max_heap_kb: Some(640.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep performance budget contract token `{needle}`."
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose performance regression marker `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"ActionBar\", \"action-bar\", \"Actions\", ax::action_bar)",
        "\"action-bar\"",
    ] {
        assert!(
            pages_source.contains(needle),
            "ActionBar docs page should remain in coverage traversal via `{needle}`.",
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_source.contains(needle),
            "docs coverage e2e should enforce repeatable perf regression guard `{needle}`.",
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace-based perf attribution token `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`.",
        );
    }

    for needle in [
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "ActionBar checklist should keep render-count baseline/follow-up token `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || state.get().phase_attr",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-selection-source=move || state.get().selection_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar view should expose attribution marker `{needle}` for perf triage.",
        );
    }

    let script_needle = "cargo test -p ui-components --test action_bar_semantics --no-default-features --features component-action_bar,inject-css action_bar_performance_governance_budget_is_defined_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`.",
    );
}

#[test]
fn action_bar_semantic_and_performance_regression_gates_cover_aria_data_focus_and_render_count_path()
 {
    let suite_source = load_source("tests/action_bar_semantics.rs");
    let view_source = load_source("src/action_bar/view.rs");
    let button_semantics_source = load_source("../../components/button/test/semantics.rs");
    let perf_script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/action_bar/check2.md");

    for needle in [
        "fn action_bar_emits_toolbar_semantics_and_state_attributes()",
        "fn action_bar_semantics_cover_data_aria_and_interaction_matrix()",
        "fn action_bar_mounts_headless_contract_in_view_not_logic_layer()",
        "fn action_bar_performance_governance_budget_is_defined_and_blocking()",
    ] {
        assert!(
            suite_source.contains(needle),
            "semantic/perf gate should keep coverage test `{needle}`.",
        );
    }

    for needle in [
        "role=\"toolbar\"",
        "aria-label=aria_label",
        "aria-hidden=move || state.get().is_hidden.then_some(\"true\")",
        "data-state=move || state.get().phase_attr",
        "data-selection=move || state.get().selection_attr",
        "data-control-mode=move || state.get().control_mode_attr",
        "data-selected-count-source=move || state.get().selected_count_source_attr",
        "<Button",
        "on_press=on_press",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionBar view should expose semantic contract marker `{needle}`.",
        );
    }

    for needle in ["use_focus_ring", "data-focus-visible"] {
        assert!(
            button_semantics_source.contains(needle),
            "clear-action focus flow should stay delegated to Button semantics contract `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui-components --test action_bar_semantics --no-default-features --features component-action_bar,inject-css action_bar_performance_governance_budget_is_defined_and_blocking",
        "perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script_source.contains(needle),
            "performance gate script should keep `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
        "渲染次数预算为 `1`",
    ] {
        assert!(
            todo_source.contains(needle) || check2_source.contains(needle),
            "render-count regression evidence should keep marker `{needle}`.",
        );
    }
}

#[test]
fn action_bar_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let protocol_source = load_source("src/action_bar/protocol.rs");
    let manifest_source = load_source("src/action_bar/Component.toml");
    let rbi_source = load_source("src/action_bar/action_bar.rbi");
    let mod_source = load_source("src/action_bar/mod.rs");

    for needle in [
        "pub const ACTION_BAR_AGENT_SCHEMA: &str = \"ui.action-bar.contract.v1\";",
        "pub enum ActionBarComponentSchemaVersion",
        "V1",
        "schema_version = \"1\"",
        "ty = \"ui.action-bar.contract.v1\"",
    ] {
        assert!(
            protocol_source.contains(needle) || manifest_source.contains(needle),
            "ActionBar protocol/manifest should keep stable v1 contract token `{needle}`.",
        );
    }

    for needle in [
        "pub fn ActionBar(",
        "selected_count: Option<leptos::prelude::Signal<usize>>",
        "default_selected_count: Option<usize>",
        "on_selected_count_change: Option<leptos::prelude::Callback<usize>>",
        "on_clear_selection: Option<leptos::prelude::Callback<()>>",
        "is_force_visible: bool",
        "aria_label: Option<String>",
        "clear_label: Option<String>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "ActionBar RBI should keep stable public API token `{needle}`.",
        );
    }

    for forbidden in [
        "migrate_v1_to_v2",
        "deprecated",
        "deprecation_window",
        "schema_version = \"2\"",
        "contract.v2",
        "V2",
    ] {
        assert!(
            !protocol_source.contains(forbidden)
                && !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !mod_source.contains(forbidden),
            "ActionBar should not introduce breaking-upgrade migration token `{forbidden}` in this change.",
        );
    }
}

#[test]
fn action_bar_motion_contract_defaults_and_disabled_path_are_locked() {
    let source = load_source("src/action_bar/motion.rs");

    for needle in [
        "pub struct ActionBarMotion",
        "enabled: true",
        "hidden_translate_px: 28.0",
        "hidden_opacity: 0.0",
        "pub fn disabled() -> Self",
        "enabled: false",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar motion contract should include `{needle}` for baseline-level defaults/disabled stability."
        );
    }
}

#[test]
fn action_bar_motion_sanitization_and_reduced_motion_paths_are_locked() {
    let source = load_source("src/action_bar/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ActionBarMotion) -> ActionBarMotion",
        ".clamp(-400.0, 400.0)",
        ".clamp(0.0, 1.0)",
        "!motion.enabled || ui_motion::web::prefers_reduced_motion()",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn disabled_constructor_turns_motion_off()",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar motion implementation should include `{needle}` to avoid regressions."
        );
    }
}

#[test]
fn action_bar_motion_uses_spring_driver() {
    let source = load_source("src/action_bar/motion.rs");

    for needle in [
        "pub fn sanitize_motion(",
        "ui_motion::spring::SpringAnimator::new",
        "ui_motion::web::prefers_reduced_motion()",
        "--ui-action-bar-translate-y",
        "--ui-action-bar-opacity",
        "pub fn attach_motion(",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar motion should include `{needle}` for spring-driven visibility animation."
        );
    }
}

#[test]
fn action_bar_motion_stays_as_mapping_and_attach_layer() {
    let source = load_source("src/action_bar/motion.rs");

    for needle in [
        "pub struct ActionBarMotion",
        "pub spring: ui_motion::spring::SpringConfig,",
        "spring: ui_motion::presets::spring_soft(),",
        "pub fn sanitize_motion(motion: ActionBarMotion) -> ActionBarMotion",
        "ui_motion::spring::SpringAnimator::new",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar motion should stay as semantic-to-motion mapping and attach layer; missing `{needle}`."
        );
    }

    for forbidden in [
        "view! {",
        "role=",
        "aria-",
        "on:click",
        "on:keydown",
        "request_animation_frame(",
        "cancel_animation_frame(",
        "struct SpringState",
        "fn step(&mut self,",
    ] {
        assert!(
            !source.contains(forbidden),
            "ActionBar motion should not host view/a11y or reimplement driver internals `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_docs_page_includes_custom_motion_contract_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "title=\"Custom Motion Contract\"",
        "let mut custom_motion = ActionBarMotion::default();",
        "custom_motion.hidden_translate_px = 44.0;",
        "custom_motion.hidden_opacity = 0.22;",
        "motion=custom_motion",
        "motion=ActionBarMotion::disabled()",
    ] {
        assert!(
            source.contains(needle),
            "action_bar docs page should include `{needle}` for custom motion demos."
        );
    }
}

#[test]
fn action_bar_docs_default_and_state_playgrounds_lock_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "<Playground title=\"Selection + clear action\" code_signal=code>",
        "selected_count=selected_count_signal",
        "on_selected_count_change=on_selected_count_change",
        "on_clear_selection=clear_selection",
        "aria_label=\"Bulk actions\".to_string()",
        "class_name=\"docs-action-bar\".to_string()",
        "<ActionButton>\"Delete\"</ActionButton>",
        "<ActionButton is_quiet=true>\"Archive\"</ActionButton>",
        "<Playground title=\"Top placement + custom text + reduced motion\" code_signal=state_code>",
        "default_selected_count=5",
        "position=ActionBarPosition::Top",
        "is_force_visible=true",
        "selection_text=\"Rows selected\".to_string()",
        "clear_label=\"Clear all\".to_string()",
        "motion=ActionBarMotion::disabled()",
        "Top placement + custom labels + motion disabled.",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar docs default/state playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn action_bar_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "<Playground title=\"Custom Motion Contract\" code_signal=motion_code>",
        "let mut custom_motion = ActionBarMotion::default();",
        "custom_motion.spring.stiffness = 280.0;",
        "custom_motion.spring.damping = 24.0;",
        "custom_motion.spring.mass = 1.0;",
        "custom_motion.spring.precision = 0.002;",
        "custom_motion.hidden_translate_px = 44.0;",
        "custom_motion.hidden_opacity = 0.22;",
        "motion=custom_motion",
        "motion=ActionBarMotion::disabled()",
        "<ActionButton is_quiet=true>\"Sync\"</ActionButton>",
        "<ActionButton is_quiet=true>\"Share\"</ActionButton>",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar docs custom-motion playground should contain `{needle}`.",
        );
    }
}

#[test]
fn action_bar_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "pub(super) fn action_bar() -> AnyView",
        "title=\"ActionBar\"",
        "slug=\"action-bar\"",
        "Playground title=\"Hello World\"",
        "Playground title=\"Selection + clear action\"",
        "Playground title=\"Top placement + custom text + reduced motion\"",
        "Playground title=\"Custom Motion Contract\"",
    ] {
        assert!(
            source.contains(needle),
            "actions-extra docs page should contain `{needle}` for ActionBar.",
        );
    }
}

#[test]
fn action_bar_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "title=\"Selection + clear action\"",
        "selected_count=selected_count_signal",
        "on_selected_count_change=on_selected_count_change",
        "on_clear_selection=clear_selection",
        "aria_label=\"Bulk actions\".to_string()",
        "class_name=\"docs-action-bar\".to_string()",
        "title=\"Top placement + custom text + reduced motion\"",
        "default_selected_count=5",
        "position=ActionBarPosition::Top",
        "is_force_visible=true",
        "selection_text=\"Rows selected\".to_string()",
        "clear_label=\"Clear all\".to_string()",
        "motion=ActionBarMotion::disabled()",
        "title=\"Custom Motion Contract\"",
        "custom_motion.hidden_translate_px = 44.0;",
        "custom_motion.hidden_opacity = 0.22;",
    ] {
        assert!(
            source.contains(needle),
            "action-bar docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn action_bar_docs_copy_paste_ready_contract_covers_playgrounds_matrix_control_and_snapshot_modes()
{
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "Playground title=\"Hello World\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"State Matrix (selection + placement + visibility)\"",
        "title=\"Snapshot baseline + Streaming optional fallback\"",
        "selected_count=selected_count_signal",
        "default_selected_count=2",
        "Streaming policy: optional; fallback: snapshot.",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar docs should contain copy-ready playground contract token `{needle}`."
        );
    }

    for needle in [
        "let action_bar_code_imports =",
        "use leptos::prelude::*;\\nuse ui_components::{ActionBar, ActionBarMotion, ActionBarPosition, ActionButton};",
        "code_imports=action_bar_code_imports.clone()",
    ] {
        assert!(
            source.contains(needle),
            "ActionBar docs should wire import-complete copy contract token `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "format!(\"{}\\n\\n{raw}\", missing_imports.join(\"\\n\"))",
        "code_imports: Option<String>",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground infrastructure should keep copy-paste import completion token `{needle}`."
        );
    }
}

#[test]
fn action_bar_docs_api_and_state_matrix_track_logic_defaults_and_contract_axes() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");
    let logic_source = load_source("src/action_bar/logic.rs");

    for needle in [
        "data-slot=\"action-bar-api-matrix\"",
        "<h3>\"API Matrix\"</h3>",
        "selected_count: Option&lt;Signal&lt;usize&gt;&gt;",
        "default_selected_count: Option&lt;usize&gt;",
        "default = implicit 0 via logic::normalize_default_selected_count",
        "default = ActionBarPosition::",
        "ui_components::action_bar::DEFAULT_ARIA_LABEL",
        "ui_components::action_bar::DEFAULT_CLEAR_LABEL",
        "data-slot=\"action-bar-state-matrix\"",
        "<h3>\"State Matrix\"</h3>",
        "control mode",
        "controlled | uncontrolled",
        "data-state",
        "visible | hidden",
        "data-position",
        "top | bottom",
        "data-selection",
        "empty | single | multiple",
        "disabled / size / variant",
        "N/A on ActionBar root",
    ] {
        assert!(
            docs_source.contains(needle),
            "ActionBar docs API/State matrix should contain `{needle}`."
        );
    }

    for needle in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"Actions\";",
        "pub const DEFAULT_CLEAR_LABEL: &str = \"Clear selection\";",
        "pub fn normalize_default_selected_count(value: Option<usize>) -> usize",
        "value.unwrap_or_default()",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionBar logic defaults should keep `{needle}` for docs parity."
        );
    }
}

#[test]
fn action_bar_docs_interactive_playground_supports_props_state_and_repeatable_flow() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");

    for needle in [
        "title=\"Interactive Playground (Props + State + Spec Preview)\"",
        "controls=move || view! {",
        "data-slot=\"action-bar-interactive-controls\"",
        "data-slot=\"action-bar-interactive-preview\"",
        "data-slot=\"action-bar-interactive-actions\"",
        "test_config_signal=interactive_spec_preview",
        "ActionBarInteractiveSpec {",
        "SegmentedControl",
        "id_base=\"docs-action-bar-interactive-position\".to_string()",
        "Switch checked=interactive_force_visible",
        "Switch checked=interactive_with_clear_action",
        "Switch checked=interactive_custom_labels",
        "Switch checked=interactive_reduced_motion",
        "aria_label=\"Interactive select +1\".to_string()",
        "aria_label=\"Interactive select -1\".to_string()",
        "aria_label=\"Interactive reset count\".to_string()",
        "Repeatable flow: Select +1 -> Clear selection -> Select +1.",
    ] {
        assert!(
            docs_source.contains(needle),
            "ActionBar interactive playground contract should contain `{needle}`."
        );
    }

    for needle in [
        "selected_count=interactive_selected_count_signal",
        "on_selected_count_change=interactive_on_selected_count_change",
        "on_clear_selection=interactive_on_clear_selection",
        "position=position",
        "is_force_visible=is_force_visible",
        "selection_text=selection_text",
        "clear_label=clear_label",
        "motion=motion",
    ] {
        assert!(
            docs_source.contains(needle),
            "ActionBar interactive preview should map controls to props via `{needle}`."
        );
    }
}

#[test]
fn action_bar_source_first_docs_are_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_view_source = load_source("../../components/code-block/src/view.rs");

    for needle in [
        "data-slot=\"action-bar-source-first\"",
        "<h3>\"Source-first Copy-Paste\"</h3>",
        "Show code",
        "components/action-bar/src/mod.rs",
        "components/action-bar/src/view.rs",
        "components/action-bar/src/logic.rs",
        "components/action-bar/src/styles.rs",
        "components/action-bar/src/motion.rs",
        "Dependency prerequisites",
        "ui-components = { workspace = true, default-features = false, features = [\"component-action_bar\", \"inject-css\"] }",
        "code_imports=action_bar_code_imports.clone()",
    ] {
        assert!(
            docs_source.contains(needle),
            "ActionBar source-first docs should contain `{needle}`."
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "<CodeBlock code=resolved_code.get() />",
        "code_imports: Option<String>",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should contain `{needle}`."
        );
    }

    for needle in [
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view_source.contains(needle),
            "CodeBlock should keep one-click copy affordance token `{needle}`."
        );
    }
}

#[test]
fn action_bar_check2_documents_e2e_selector_and_stable_wait_rules() {
    let checklist_source = load_source("src/action_bar/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ActionBar checklist should keep e2e selector/stable-wait rule `{required}`."
        );
    }
}

#[test]
fn action_bar_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_action_bar.spec.mjs");

    for needle in [
        "/#/components/action-bar",
        "body:not(:has(#boot))",
        "[data-component=\"action-bar\"]",
        "[data-slot=\"action-bar\"][data-control-mode=\"controlled\"][data-has-clear=\"true\"]",
        "[data-slot=\"action-bar-selection-count\"]",
        "[data-slot=\"action-bar-clear\"] [data-slot=\"button\"]",
        "[data-slot=\"button\"][aria-label=\"Increase selected count\"]",
        "toHaveAttribute(\"role\", \"toolbar\")",
        "toHaveAttribute(\"data-state\", \"visible\")",
        "toHaveAttribute(\"data-state\", \"hidden\")",
        "toHaveAttribute(\"aria-hidden\", \"true\")",
        "toHaveAttribute(\"data-selected-count\", \"0\")",
        "toHaveAttribute(\"data-selected-count\", \"1\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "ActionBar e2e selector/stable-wait contract should include `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "ActionBar e2e selector contract should avoid unstable/non-semantic token `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_action_bar.spec.mjs");

    for needle in [
        "docs-app action-bar motion path uses semantic ready and settled breakpoints",
        "clearButton.focus();",
        "page.keyboard.press(\"Space\")",
        "toHaveAttribute(\"data-selected-count\", \"0\")",
        "toHaveAttribute(\"data-state\", \"hidden\")",
        "toHaveAttribute(\"aria-hidden\", \"true\")",
        "await incrementButton.click();",
        "toHaveAttribute(\"data-selected-count\", \"1\")",
        "toHaveAttribute(\"data-state\", \"visible\")",
        "not.toHaveAttribute(\"aria-hidden\", \"true\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "ActionBar e2e ready/settled semantic breakpoint contract should include `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "ActionBar e2e animation path should avoid unstable fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_check2_documents_e2e_repeatable_key_flow_rules() {
    let checklist_source = load_source("src/action_bar/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ActionBar checklist should keep repeatable-key-flow rule `{required}`."
        );
    }
}

#[test]
fn action_bar_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_action_bar.spec.mjs");

    for needle in [
        "docs-app action-bar key flow is repeatable with semantic breakpoints",
        "await runActionBarCriticalFlow(page, docsRoot);",
        "await page.reload();",
        "const reloadedDocsRoot = await openActionBarDocs(page);",
        "toHaveAttribute(\"data-selected-count\", \"2\")",
        "await runActionBarCriticalFlow(page, reloadedDocsRoot);",
    ] {
        assert!(
            e2e_source.contains(needle),
            "ActionBar e2e repeatable-flow contract should include `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "waitForTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "ActionBar e2e key flow should avoid non-semantic/flaky token `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_action_bar.spec.mjs");

    for needle in [
        "runActionBarCriticalFlow",
        "clearButton.focus();",
        "page.keyboard.press(\"Space\")",
        "toHaveAttribute(\"data-selected-count\", \"0\")",
        "toHaveAttribute(\"data-state\", \"hidden\")",
        "toHaveAttribute(\"aria-hidden\", \"true\")",
        "await incrementButton.click();",
        "toHaveAttribute(\"data-selected-count\", \"1\")",
        "toHaveAttribute(\"data-state\", \"visible\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "ActionBar e2e high-risk focus/keyboard semantic path should include `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "ActionBar high-risk e2e path should avoid unstable fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn action_bar_anti_patterns_are_blocked_by_contracts() {
    let suite_source = load_source("tests/action_bar_semantics.rs");
    let check2_source = load_source("src/action_bar/check2.md");

    for needle in [
        "fn action_bar_status_primitives_are_sourced_from_ui_state_primitives()",
        "fn action_bar_mounts_headless_contract_in_view_not_logic_layer()",
        "fn action_bar_state_normalization_is_centralized_in_logic()",
        "fn action_bar_api_naming_uses_is_on_default_prefix_contract()",
        "fn action_bar_composition_api_prefers_explicit_parent_item_structure()",
        "fn action_bar_platform_guards_keep_cfg_split_and_non_wasm_web_sys_free()",
        "fn action_bar_status_primitive_boundary_avoids_local_state_machine_and_business_store_coupling()",
    ] {
        assert!(
            suite_source.contains(needle),
            "ActionBar anti-pattern gate should keep backing contract test `{needle}`."
        );
    }

    for needle in [
        "### 8. 明确禁止的反模式",
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 在 `view` 层隐藏关键状态决策。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "- [x] 公共 API 泄露底层实现细节类型。",
        "- [x] 用临时补丁破坏跨组件一致性。",
        "- [x] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
    ] {
        assert!(
            check2_source.contains(needle),
            "ActionBar checklist anti-pattern section should keep checked governance marker `{needle}`."
        );
    }
}

#[test]
fn action_bar_check2_marks_final_merge_gates_complete() {
    let suite_source = load_source("tests/action_bar_semantics.rs");
    let check2_source = load_source("src/action_bar/check2.md");

    for needle in [
        "fn action_bar_stays_in_ui_components_assembly_layer_and_public_api_boundary_is_stable()",
        "fn action_bar_uses_logic_state_model()",
        "fn action_bar_emits_toolbar_semantics_and_state_attributes()",
        "fn action_bar_visual_desire_reuses_theme_visual_baseline_gate()",
        "fn action_bar_semantics_suite_prioritizes_contract_assertions_over_snapshots()",
        "fn action_bar_component_files_follow_layered_responsibilities()",
        "fn action_bar_machine_readable_contract_uses_typed_inputs_and_semantic_markers()",
        "fn action_bar_reduced_motion_ssr_wasm_branches_keep_semantics_consistent()",
        "fn action_bar_docs_page_covers_primary_playgrounds()",
    ] {
        assert!(
            suite_source.contains(needle),
            "ActionBar final merge gate should keep backing contract test `{needle}`."
        );
    }

    for needle in [
        "### 9. 合并门禁（最终裁决）",
        "- [x] 架构正确（边界不破）。",
        "- [x] 行为正确（状态与交互语义成立）。",
        "- [x] 可访问性达标（默认可用）。",
        "- [x] 默认主题美学质量达标（与可访问性同级门禁）。",
        "- [x] 可测试（契约可断言）。",
        "- [x] 可维护（命名和模式一致）。",
        "- [x] 可解释（人和自动化都能读懂）。",
        "- [x] 改动在正确层。",
        "- [x] 命名与全库一致。",
        "- [x] 无效状态被限制或归一化。",
        "- [x] 暴露必要语义标记。",
        "- [x] 覆盖 reduced-motion / SSR / wasm 分支。",
        "- [x] 文档与示例同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "ActionBar checklist final merge gate should keep checked governance marker `{needle}`."
        );
    }
}
