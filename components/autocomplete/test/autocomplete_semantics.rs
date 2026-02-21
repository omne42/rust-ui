use std::fs;
use std::path::Path;

fn resolve_path(rel_path: &str) -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    if let Some(suffix) = rel_path.strip_prefix("src/autocomplete/") {
        workspace_dir
            .join("components/autocomplete/src")
            .join(suffix)
    } else if rel_path == "src/lib.rs" {
        workspace_dir.join("crates/ui-components/src/lib.rs")
    } else if rel_path == "src/css.rs" {
        workspace_dir.join("crates/ui-components/src/css.rs")
    } else if rel_path == "Cargo.toml" {
        workspace_dir.join("crates/ui-components/Cargo.toml")
    } else if let Some(suffix) = rel_path.strip_prefix("../ui-state-primitives/") {
        workspace_dir
            .join("crates/ui-state-primitives")
            .join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("../ui-headless/") {
        workspace_dir.join("crates/ui-headless").join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("../ui-motion/") {
        workspace_dir.join("crates/ui-motion").join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("../../") {
        workspace_dir.join(suffix)
    } else {
        manifest_dir.join(rel_path)
    }
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_path(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn slice_between<'a>(source: &'a str, start: &str, end: &str) -> &'a str {
    let start_idx = source
        .find(start)
        .unwrap_or_else(|| panic!("source should contain start marker `{start}`"));
    let tail = &source[start_idx..];
    let end_idx = tail
        .find(end)
        .unwrap_or_else(|| panic!("source should contain end marker `{end}` after `{start}`"));
    &tail[..end_idx]
}

#[test]
fn autocomplete_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/autocomplete/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Autocomplete internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn autocomplete_uses_logic_state_model() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/autocomplete.rs");

    for needle in [
        "pub use ui_state_primitives::autocomplete::{",
        "AutocompleteStateInput",
        "AutocompleteState",
        "RootDataState",
        "resolve_root_data_state",
        "normalize_optional_text",
        "normalize_id_base",
        "normalize_disabled_indices",
        "resolve_state",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Autocomplete logic should include `{needle}` while consuming centralized ui-state-primitives."
        );
    }

    for needle in [
        "pub struct AutocompleteStateInput",
        "pub struct AutocompleteState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_disabled_indices(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Autocomplete primitive source should define `{needle}`."
        );
    }

    for needle in [
        "let root_state = logic::normalize_root_state(logic::RootStateInput {",
        "logic::resolve_root_data_state(is_open.get(), state.is_disabled).as_attr()",
        "let state = root_state.state;",
        "let class = root_state.class_name;",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn autocomplete_logic_does_not_reimplement_reusable_state_primitives() {
    let logic_source = load_source("src/autocomplete/logic.rs");

    for forbidden in [
        "pub fn normalize_disabled_indices(",
        "pub fn filter_indices(",
        "pub fn map_selected_to_filtered(",
        "pub fn map_filtered_to_original(",
        "pub fn resolve_state(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Autocomplete logic should not reimplement reusable primitive `{forbidden}`; it must consume ui-state-primitives instead.",
        );
    }
}

#[test]
fn autocomplete_component_has_store_adapter_boundary() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");

    for forbidden in ["GlobalState", "AppState", "Store<", "SignalStore", "apps::"] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Autocomplete should not bind app/business store type `{forbidden}` directly.",
        );
    }
}

#[test]
fn autocomplete_discrete_data_state_is_enum_backed() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");

    for needle in [
        "pub enum RootDataState",
        "Open,",
        "Disabled,",
        "Closed,",
        "pub fn resolve_root_data_state(",
        "pub const fn as_attr(self) -> &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "Autocomplete logic should model discrete data-state with `{needle}`."
        );
    }

    assert!(
        view_source
            .contains("logic::resolve_root_data_state(is_open.get(), state.is_disabled).as_attr()"),
        "Autocomplete view should map data-state via typed enum contract instead of inline string branching."
    );
}

#[test]
fn autocomplete_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");

    for needle in [
        "is_open: Option<Signal<bool>>",
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "pub struct OpenStateInput",
        "pub struct OpenState",
        "pub fn normalize_open_state(",
    ] {
        assert!(
            source.contains(needle) || logic_source.contains(needle),
            "Autocomplete should accept `{needle}` for controlled/uncontrolled open state."
        );
    }
}

#[test]
fn autocomplete_wires_open_value_change_default_triplet_into_headless_state() {
    let source = load_source("src/autocomplete/view.rs");

    for needle in [
        "let normalized_open_state = logic::normalize_open_state(logic::OpenStateInput {",
        "let open_state = overlay_open::use_controllable_open_state_traced(",
        "\"autocomplete\",",
        "open,",
        "default_open,",
        "on_open_change,",
        "let is_open = open_state.open;",
        "let set_open = open_state.request_open_change;",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete open axis should wire `{needle}` for stable controlled/uncontrolled semantics.",
        );
    }
}

#[test]
fn autocomplete_supports_is_prefixed_boolean_props_with_legacy_aliases() {
    let source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");

    for needle in [
        "selected_index: Option<Signal<Option<usize>>>",
        "default_selected_index: Option<usize>",
        "on_selected_index_change: Option<Callback<Option<usize>>>",
        "set_selected_index: Option<WriteSignal<Option<usize>>>",
        "pub struct SelectionChangeInput",
        "pub struct SelectionChange",
        "pub fn normalize_selection_change(",
        "pub selected_index: Option<Signal<Option<usize>>>",
        "pub default_selected_index: Option<usize>",
        "pub is_controlled: bool",
        "pub selected_source: SelectedSource",
        "pub change_source: SelectedChangeSource",
        "pub enum SelectedSource",
        "pub enum SelectedChangeSource",
        "pub const fn as_attr(self) -> &'static str",
        "Self::SelectedIndex => \"selected_index\"",
        "Self::DefaultSelectedIndex => \"default_selected_index\"",
        "Self::OnSelectedIndexChange => \"on_selected_index_change\"",
        "Self::SetSelectedIndex => \"set_selected_index\"",
        "Self::None => \"none\"",
        "default_selected_index: Option<usize>",
        "item_count: usize",
        "let default_selected_index = input",
        "let is_controlled = input.selected_index.is_some();",
        "let selected_source = if is_controlled {",
        "SelectedSource::SelectedIndex",
        "SelectedSource::DefaultSelectedIndex",
        "SelectedChangeSource::OnSelectedIndexChange",
        "SelectedChangeSource::SetSelectedIndex",
        "SelectedChangeSource::None",
        "is_disabled: Option<bool>",
        "disabled: bool",
        "is_required: Option<Signal<bool>>",
        "required: Option<Signal<bool>>",
        "is_invalid: Option<Signal<bool>>",
        "invalid: Option<Signal<bool>>",
        "pub struct AccessibilityStateInput",
        "pub struct AccessibilityState",
        "pub fn normalize_accessibility_state(",
        "is_disabled: input.is_disabled.unwrap_or(input.disabled)",
        "let required = input",
        ".is_required",
        ".or(input.required)",
        "let invalid = input",
        ".is_invalid",
        ".or(input.invalid)",
    ] {
        assert!(
            source.contains(needle) || logic_source.contains(needle),
            "Autocomplete API naming contract should include `{needle}`."
        );
    }

    for needle in [
        "let accessibility_state =",
        "logic::normalize_accessibility_state(logic::AccessibilityStateInput {",
        "let selection_change = logic::normalize_selection_change(logic::SelectionChangeInput {",
        "selected_index,",
        "default_selected_index,",
        "let selected_source_attr = selection_change.selected_source.as_attr();",
        "let selected_change_source_attr = selection_change.change_source.as_attr();",
        "let selected_state = overlay_open::use_controllable_state(",
        "Some(selection_change.default_selected_index),",
        "selection_change.on_selected_index_change,",
        "let selected_index = selected_state.value;",
        "let request_selected_index_change = selected_state.request_change;",
        "let is_disabled = accessibility_state.is_disabled;",
        "let required = accessibility_state.required;",
        "let invalid = accessibility_state.invalid;",
        "data-selected-source=selected_source_attr",
        "data-selected-controlled=is_selected_controlled.then_some(\"true\")",
        "data-selected-uncontrolled=(!is_selected_controlled).then_some(\"true\")",
        "data-selected-change-source=selected_change_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete view should consume normalized accessibility state via `{needle}`."
        );
    }
}

#[test]
fn autocomplete_view_does_not_inline_default_fallback_rules() {
    let source = load_source("src/autocomplete/view.rs");

    for forbidden in [
        "is_disabled.unwrap_or(disabled)",
        "is_required.or(required)",
        "is_invalid.or(invalid)",
        "is_open.or(open)",
        "empty_message: empty_message",
        ".or_else(|| Some(common.autocomplete_empty_message.to_string()))",
        "unwrap_or_else(|| Signal::derive(|| false))",
        "logic::normalize_id_base(",
        "logic::normalize_label(",
        "logic::resolve_placeholder(",
        "logic::resolve_state(",
    ] {
        assert!(
            !source.contains(forbidden),
            "Autocomplete view.rs should not own fallback/priority rule `{forbidden}`; keep it in logic.rs.",
        );
    }
}

#[test]
fn autocomplete_centralizes_input_state_reduction_in_logic_layer() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");

    for needle in [
        "pub struct InputStateSource",
        "pub fn reduce_sync_from_selection(",
        "pub fn reduce_after_option_commit(",
        "pub fn reduce_after_input_blur(",
        "pub fn reduce_after_input_change(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Autocomplete logic should centralize typed reducer helper `{needle}`."
        );
    }

    for needle in [
        "logic::reduce_sync_from_selection(",
        "logic::reduce_after_option_commit(",
        "logic::reduce_after_input_blur(",
        "logic::reduce_after_input_change(",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should consume centralized reducer `{needle}`."
        );
    }

    for forbidden in [
        "logic::reduce_input_state(",
        "logic::AutocompleteInputEvent::",
        "logic::AutocompleteInputState {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Autocomplete view should not reconstruct reducer internals `{forbidden}`.",
        );
    }
}

#[test]
fn autocomplete_normalizes_label_placeholder_and_id_base() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/autocomplete.rs");

    for needle in [
        "normalize_label(",
        "resolve_placeholder(",
        "resolve_empty_message(",
        "normalize_id_base(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Autocomplete logic should use `{needle}` to keep text and id semantics stable."
        );
    }

    assert!(
        view_source.contains("logic::normalize_root_state(logic::RootStateInput {"),
        "Autocomplete view should delegate normalization to logic::normalize_root_state."
    );

    for needle in [
        "pub const DEFAULT_LABEL: &str = \"Options\"",
        "pub const DEFAULT_ID_BASE: &str = \"autocomplete\"",
        "pub const DEFAULT_PLACEHOLDER: &str = \"Type…\"",
        "pub const DEFAULT_EMPTY_MESSAGE: &str = \"No matches\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Autocomplete primitives should provide fallback semantics via `{needle}`."
        );
    }
}
#[test]
fn autocomplete_escape_stops_propagation_when_open() {
    let source = load_source("src/autocomplete/view.rs");

    for needle in [
        "let key_result = aria.handlers.on_input_key_down.run(key);",
        "if key_result.handled {",
        "if key_result.stop_propagation {",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete should consume typed headless keydown outcomes via `{needle}` to keep keyboard semantics out of view.rs."
        );
    }
}

#[test]
fn autocomplete_passes_lang_dir_and_headless_aria_controls_contract() {
    let source = load_source("src/autocomplete/view.rs");

    for needle in [
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
        "lang,",
        "dir,",
        "aria-controls=move || aria.input.aria_controls.get()",
        "lang=aria.input.lang.clone()",
        "dir=aria.input.dir",
        "lang=aria.listbox.lang.clone()",
        "dir=aria.listbox.dir",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete should wire `{needle}` so locale + aria-controls semantics come from ui-headless contract."
        );
    }
}

#[test]
fn autocomplete_panel_is_portaled_and_uses_popover_positioning() {
    let source = load_source("src/autocomplete/view.rs");

    for needle in [
        "<Portal>",
        "use_popover_position",
        "data-ui-overlay-portal",
        "--ui-popover-top",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete panel should include `{needle}` for baseline-style popover behavior."
        );
    }
}

#[test]
fn autocomplete_two_pass_geometry_rendering_contract_is_measure_then_guarded_rectification() {
    let view_source = load_source("src/autocomplete/view.rs");
    let headless_popover_source = load_source("../ui-headless/src/popover_position.rs");
    let headless_popover_test_source = load_source("../ui-headless/src/test/popover_position.rs");

    for needle in [
        "use_popover_position(PopoverPositionOptions {",
        "position.top_px.get()",
        "position.left_px.get()",
        "position.anchor_width_px.get()",
        "data-placement=move || position.placement.get().as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should keep measure-stage marker `{needle}`."
        );
    }

    for needle in [
        "fn compute_popover_position(",
        "const POSITION_EPSILON_PX: f64 = 0.01;",
        "fn should_update_scalar(current: f64, next: f64) -> bool",
        "if should_update_scalar(top_px.get_untracked(), computed.top) {",
        "if should_update_scalar(left_px.get_untracked(), computed.left) {",
        "if placement.get_untracked() != computed.placement {",
        "ResizeObserver",
    ] {
        assert!(
            headless_popover_source.contains(needle),
            "ui-headless popover position should keep guarded rectification marker `{needle}`."
        );
    }

    for needle in [
        "fn scalar_update_guard_ignores_sub_epsilon_noise()",
        "fn scalar_update_guard_accepts_meaningful_delta()",
    ] {
        assert!(
            headless_popover_test_source.contains(needle),
            "ui-headless popover tests should keep convergence regression `{needle}`."
        );
    }
}

#[test]
fn autocomplete_non_registered_collection_contract_uses_filtered_vec_order() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");

    for needle in [
        "let filtered_indices = Memo::new",
        "logic::filter_indices(",
        "logic::map_selected_to_filtered(selected_index.get(), &filtered_indices.get())",
        "logic::map_filtered_to_original(filtered_index, &indices)",
        "disabled_indices.contains(&original)",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete should keep deterministic collection-order marker `{needle}`."
        );
    }

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "for index in disabled_indices",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Autocomplete should avoid registration-protocol token `{forbidden}` for non-composite collection flow.",
        );
    }
}

#[test]
fn autocomplete_non_container_slot_projection_contract_is_not_applicable() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let motion_source = load_source("src/autocomplete/motion.rs");

    for needle in [
        "use_presence(is_open)",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete should keep predictable presence lifecycle marker `{needle}`."
        );
    }

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot_projection",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Autocomplete should avoid container slot-projection token `{forbidden}`.",
        );
    }
}

#[test]
fn autocomplete_panel_exposes_option_and_empty_state_slots() {
    let source = load_source("src/autocomplete/view.rs");

    for needle in [
        "data-slot=\"autocomplete-listbox\"",
        "data-empty=move || filtered_indices.get().is_empty().then_some(\"true\")",
        "data-slot=\"autocomplete-option\"",
        "let option_attrs = aria.option_attrs;",
        "Memo::new(move |_| option_attrs.run(filtered_index));",
        "data-focused=move || option_attrs.get().data_focused",
        "data-slot=\"autocomplete-empty\"",
        "{move || empty_message.get_value()}",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete panel should expose `{needle}` for baseline-style state styling and deterministic tests."
        );
    }
}

#[test]
fn autocomplete_has_a11y_i18n_and_locale_entrypoints_via_headless_contracts() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let module_source = load_source("src/autocomplete/mod.rs");
    let headless_a11y_source = load_source("../ui-headless/src/a11y.rs");
    let common_i18n_source = load_source("../ui-headless/src/i18n/common.rs");

    for needle in [
        "use ui_headless::{",
        "CommonStrings",
        "use_ui_i18n",
        "let i18n = use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "empty_message,",
        "i18n_empty_message: Some(common.autocomplete_empty_message.to_string()),",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete should include a11y/i18n/locale contract token `{needle}`."
        );
    }

    assert!(
        logic_source
            .contains("resolve_empty_message(input.empty_message.or(input.i18n_empty_message))"),
        "Autocomplete logic should centralize empty-message fallback priority."
    );

    assert!(
        common_i18n_source.contains("pub autocomplete_empty_message: Arc<str>,"),
        "CommonStrings should expose autocomplete empty-message i18n slot."
    );
    assert!(
        common_i18n_source.contains("autocomplete_empty_message: \"No matches\".into(),"),
        "CommonStrings default should provide autocomplete empty-message fallback."
    );
    assert!(
        !view_source.contains("\"No matches\""),
        "Autocomplete view.rs should not hardcode user-visible empty-state copy."
    );

    for needle in ["pub enum A11yDirection", "pub fn locale_attrs("] {
        assert!(
            headless_a11y_source.contains(needle),
            "ui-headless a11y source should include shared marker `{needle}`."
        );
    }

    for forbidden in ["mod a11y;", "fn locale_attrs("] {
        assert!(
            !module_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden),
            "Autocomplete component layer should avoid reimplementing shared a11y helper `{forbidden}`.",
        );
    }
}

#[test]
fn autocomplete_has_no_async_loading_protocol_and_keeps_sync_input_contract() {
    let view_source = load_source("src/autocomplete/view.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "on_retry",
        "retry",
        "use_async_action",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Autocomplete should not define per-component async protocol token `{forbidden}` when interaction is sync-only."
        );
    }

    for required in [
        "let on_action = Callback::new(move |filtered_index: usize| {",
        "request_selected_index_change.run(Some(original_index));",
        "logic::reduce_after_option_commit(",
        "set_query.set(next.query);",
        "set_has_typed.set(next.has_typed);",
    ] {
        assert!(
            view_source.contains(required),
            "Autocomplete should keep synchronous selection-action flow `{required}`."
        );
    }
}

#[test]
fn autocomplete_uses_presence_for_motion_safe_unmounting() {
    let source = load_source("src/autocomplete/view.rs");

    for needle in [
        "use_presence(is_open)",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete should use `{needle}` so popover exit motion can finish before unmount."
        );
    }
}

#[test]
fn autocomplete_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/autocomplete/view.rs");

    for attr in [
        "data-slot=\"autocomplete\"",
        "data-state=move ||",
        "data-open=move || is_open.get().then_some(\"true\")",
        "data-closed=move || (!is_open.get()).then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-empty=move || (filtered_count.get() == 0).then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-has-filtered-items=move || (filtered_count.get() > 0).then_some(\"true\")",
        "data-selection-empty=move || selected_index.get().is_none().then_some(\"true\")",
        "data-has-selection=move || selected_index.get().is_some().then_some(\"true\")",
        "data-selected-source=selected_source_attr",
        "data-selected-controlled=is_selected_controlled.then_some(\"true\")",
        "data-selected-uncontrolled=(!is_selected_controlled).then_some(\"true\")",
        "data-invalid=move || invalid.get().then_some(\"true\")",
        "data-valid=move || (!invalid.get()).then_some(\"true\")",
        "data-required=move || required.get().then_some(\"true\")",
        "data-optional=move || (!required.get()).then_some(\"true\")",
        "data-has-description=state.has_description.then_some(\"true\")",
        "data-has-error=state.has_error.then_some(\"true\")",
        "data-has-disabled-options=state.has_disabled_options.then_some(\"true\")",
        "data-controlled=state.is_controlled.then_some(\"true\")",
        "data-uncontrolled=state.is_uncontrolled.then_some(\"true\")",
        "data-label-source=state.label_source_attr",
        "data-description-source=state.description_source_attr",
        "data-error-source=state.error_source_attr",
        "data-placeholder-source=state.placeholder_source_attr",
        "data-id-source=state.id_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-custom-label=state.has_custom_label.then_some(\"true\")",
        "data-custom-description=state.has_custom_description.then_some(\"true\")",
        "data-custom-error=state.has_custom_error.then_some(\"true\")",
        "data-custom-placeholder=state.has_custom_placeholder.then_some(\"true\")",
        "data-custom-id=state.has_custom_id_base.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-custom-motion=state.has_custom_motion.then_some(\"true\")",
        "data-typed=move || has_typed.get().then_some(\"true\")",
        "data-count=state.item_count.to_string()",
        "data-filtered-count=move || filtered_count.get().to_string()",
        "data-disabled-option-count=state.disabled_option_count.to_string()",
    ] {
        assert!(
            source.contains(attr),
            "Autocomplete should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}
#[test]
fn autocomplete_panel_styles_use_fixed_positioning_and_transform_origin_by_placement() {
    let source = load_source("src/autocomplete/styles.rs");

    for needle in [
        "position: fixed;",
        "var(--ui-popover-top",
        "data-placement=\"bottom-start\"",
        ".ui-autocomplete__empty",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete styles should include `{needle}` for popover layout and empty-state rendering."
        );
    }
}

#[test]
fn autocomplete_styles_include_controlled_and_disabled_option_markers() {
    let source = load_source("src/autocomplete/styles.rs");

    for needle in [
        ".ui-autocomplete--controlled",
        ".ui-autocomplete[data-controlled=\"true\"]",
        ".ui-autocomplete--has-disabled-options",
        ".ui-autocomplete[data-has-disabled-options=\"true\"]",
        ".ui-autocomplete--empty",
        ".ui-autocomplete[data-empty=\"true\"]",
        ".ui-autocomplete[data-label-source=\"custom\"]",
        ".ui-autocomplete[data-custom-label=\"true\"]",
        ".ui-autocomplete--custom-label",
        ".ui-autocomplete[data-description-source=\"custom\"]",
        ".ui-autocomplete[data-custom-description=\"true\"]",
        ".ui-autocomplete--custom-description",
        ".ui-autocomplete[data-error-source=\"custom\"]",
        ".ui-autocomplete[data-custom-error=\"true\"]",
        ".ui-autocomplete--custom-error",
        ".ui-autocomplete[data-placeholder-source=\"custom\"]",
        ".ui-autocomplete[data-custom-placeholder=\"true\"]",
        ".ui-autocomplete--custom-placeholder",
        ".ui-autocomplete[data-id-source=\"custom\"]",
        ".ui-autocomplete[data-custom-id=\"true\"]",
        ".ui-autocomplete--custom-id",
        ".ui-autocomplete[data-class-source=\"custom\"]",
        ".ui-autocomplete[data-custom-class=\"true\"]",
        ".ui-autocomplete--custom-class",
        ".ui-autocomplete[data-motion-source=\"custom\"]",
        ".ui-autocomplete[data-custom-motion=\"true\"]",
        ".ui-autocomplete--custom-motion",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete styles should include `{needle}` for stable state-marker contracts."
        );
    }
}

#[test]
fn autocomplete_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("src/autocomplete/styles.rs");
    let theme_css_source = load_source("../ui-theme/src/css.rs");

    for needle in [
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
        "var(--ui-shadow-md, var(--ui-fallback-shadow-md))",
        "var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))",
        "var(--ui-overlay-viewport-inset, var(--ui-fallback-overlay-viewport-inset))",
        "var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index))",
        "var(--ui-overlay-enter-offset-y, var(--ui-fallback-overlay-enter-offset-y))",
        "var(--ui-overlay-enter-scale, var(--ui-fallback-overlay-enter-scale))",
    ] {
        assert!(
            styles_source.contains(needle),
            "Autocomplete styles should keep defensive fallback chain marker `{needle}`."
        );
    }

    for needle in [
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-space-md:",
        "--ui-fallback-font-size-150:",
        "--ui-fallback-line-height-150:",
        "--ui-fallback-font-size-100:",
        "--ui-fallback-line-height-100:",
        "--ui-fallback-border-width:",
        "--ui-fallback-border:",
        "--ui-fallback-radius-md:",
        "--ui-fallback-radius-lg:",
        "--ui-fallback-bg:",
        "--ui-fallback-fg:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-focus-ring:",
        "--ui-fallback-danger:",
        "--ui-fallback-accent-soft:",
        "--ui-fallback-shadow-md:",
        "--ui-fallback-overlay-panel-min-width:",
        "--ui-fallback-overlay-viewport-inset:",
        "--ui-fallback-overlay-z-index:",
        "--ui-fallback-overlay-enter-offset-y:",
        "--ui-fallback-overlay-enter-scale:",
    ] {
        assert!(
            theme_css_source.contains(needle),
            "ui-theme css should provide fallback terminal `{needle}`."
        );
    }

    for forbidden in [
        "14px", "20px", "12px", "16px", "240px", "0px", "1px", "2px", "3px",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Autocomplete styles should avoid raw terminal token `{forbidden}`."
        );
    }
}

#[test]
fn autocomplete_defensive_variables_check_script_covers_style_fallback_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    let needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn autocomplete_check2_marks_defensive_variables_contract_complete() {
    let source = load_source("../../components/autocomplete/check2.md");

    assert!(
        source.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "autocomplete check2 should mark defensive-variables gate complete."
    );

    for needle in [
        "autocomplete_styles_use_defensive_variable_fallback_chain",
        "autocomplete_defensive_variables_check_script_covers_style_fallback_contract",
        "scripts/check-ui-components-contract-hygiene.sh",
        "components/autocomplete/src/styles.rs",
        "crates/ui-theme/src/css.rs",
    ] {
        assert!(
            source.contains(needle),
            "autocomplete check2 defensive-variables section should reference `{needle}`."
        );
    }
}

#[test]
fn autocomplete_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_entry_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let view_source = load_source("src/autocomplete/view.rs");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-autocomplete\")]",
        "out.push_str(crate::autocomplete::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(needle),
            "ui-components css entry should enforce cascade-layer contract `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized css injection contract `{needle}`."
        );
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
        "style=\"position:",
        "style:top=",
        "style:left=",
        "style:right=",
        "style:bottom=",
        "style:width=",
        "style:height=",
        "style:position=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "autocomplete view should not include plain inline style token `{forbidden}`."
        );
    }

    let style_lines: Vec<&str> = view_source
        .lines()
        .filter(|line| line.contains("style="))
        .collect();
    assert_eq!(
        style_lines.len(),
        1,
        "autocomplete view should keep a single runtime style binding for css vars."
    );
    assert!(
        style_lines[0].contains("style=panel_vars"),
        "autocomplete runtime style binding should route through `panel_vars`."
    );

    for needle in [
        "let panel_vars = move || {",
        "--ui-popover-top: {}px;",
        "--ui-popover-left: {}px;",
        "--ui-popover-anchor-width: {}px;",
        "position.top_px.get()",
        "position.left_px.get()",
        "position.anchor_width_px.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "autocomplete runtime style payload should stay css-custom-property-only via `{needle}`."
        );
    }
}

#[test]
fn autocomplete_cascade_layer_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    let needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn autocomplete_check2_marks_cascade_layer_contract_complete() {
    let source = load_source("../../components/autocomplete/check2.md");

    assert!(
        source.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "autocomplete check2 should mark cascade-layer gate complete."
    );

    for needle in [
        "autocomplete_cascade_layer_and_runtime_style_contract_is_enforced",
        "autocomplete_cascade_layer_check_script_covers_contract",
        "scripts/check-ui-components-contract-hygiene.sh",
        "crates/ui-components/src/css.rs",
        "crates/ui-components/src/root.rs",
        "components/autocomplete/src/view.rs",
    ] {
        assert!(
            source.contains(needle),
            "autocomplete check2 cascade-layer section should reference `{needle}`."
        );
    }
}

#[test]
fn autocomplete_motion_contract_exposes_popover_and_highlight_customization() {
    let mod_source = load_source("src/autocomplete/mod.rs");
    let motion_source = load_source("src/autocomplete/motion.rs");
    let motion_test_source = load_source("../../components/autocomplete/test/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::AutocompleteMotion;",
        "pub struct AutocompleteMotion",
        "pub popover: PopoverMotion",
        "pub highlight: ActiveHighlightMotion",
        "fn default_motion_uses_default_popover_and_highlight_motion()",
        "fn supports_custom_popover_and_highlight_motion_contracts()",
    ] {
        assert!(
            mod_source.contains(needle)
                || motion_source.contains(needle)
                || motion_test_source.contains(needle),
            "Autocomplete motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
fn autocomplete_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/autocomplete/motion.rs");
    let motion_test_source = load_source("../../components/autocomplete/test/motion.rs");
    let view_source = load_source("src/autocomplete/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: AutocompleteMotion) -> AutocompleteMotion",
        "popover: sanitize_popover_motion(motion.popover)",
        "highlight: sanitize_highlight(motion.highlight)",
        "fn sanitize_motion_falls_back_for_invalid_nested_values()",
    ] {
        assert!(
            motion_source.contains(needle) || motion_test_source.contains(needle),
            "Autocomplete motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::motion::sanitize_motion(motion);"),
        "Autocomplete view should sanitize motion before attaching popover and active-highlight motion.",
    );
}

#[test]
fn autocomplete_motion_layer_stays_contract_mapped_and_platform_gated() {
    let motion_source = load_source("src/autocomplete/motion.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");

    for needle in [
        "pub struct PopoverMotion",
        "pub struct AutocompleteMotion",
        "ui_motion::spring::SpringConfig",
        "ui_motion::spring::SpringAnimator",
        "pub fn sanitize_motion(motion: AutocompleteMotion) -> AutocompleteMotion",
        "pub fn attach_popover_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(needle),
            "Autocomplete motion layer should keep contract mapping token `{needle}`."
        );
    }

    for forbidden in [
        "view! {",
        "role=",
        "aria-",
        "on:keydown",
        "on:click",
        "data-slot",
        "request_animation_frame(",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "Autocomplete motion.rs should avoid view/a11y/driver internals token `{forbidden}`."
        );
    }

    for needle in [
        "pub mod keyframes;",
        "pub mod options;",
        "pub mod spring;",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion should preserve shared backend and non-wasm no-op token `{needle}`."
        );
    }
}

#[test]
fn autocomplete_theme_contract_is_token_first_and_ui_theme_backed() {
    let styles_source = load_source("src/autocomplete/styles.rs");
    let tokens_source = load_source("../ui-theme/src/tokens.rs");
    let theme_source = load_source("../ui-theme/src/theme.rs");
    let css_source = load_source("../ui-theme/src/css.rs");
    let styling_spec_source = load_source("../../docs/spec/styling.md");
    let token_baseline_test_source = load_source("../ui-theme/tests/token_scale_baseline.rs");
    let wcag_test_source = load_source("../ui-theme/tests/wcag_contrast.rs");

    for needle in [
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-shadow-md, var(--ui-fallback-shadow-md))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
        "var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))",
    ] {
        assert!(
            styles_source.contains(needle),
            "Autocomplete styles should consume ui-theme variable `{needle}`."
        );
    }

    for forbidden in ["oklch(", "rgb(", "hsl("] {
        assert!(
            !styles_source.contains(forbidden),
            "Autocomplete styles should not hardcode color literal `{forbidden}` outside ui-theme."
        );
    }

    for needle in [
        "pub enum ThemeSystem",
        "pub enum ThemeColor",
        "pub enum ThemeScale",
        "pub struct ThemeContext",
        "pub struct Theme",
    ] {
        assert!(
            theme_source.contains(needle),
            "ui-theme theme axis contract should include `{needle}`."
        );
    }

    for needle in [
        "pub fn theme_to_css_variables(theme: &Theme) -> String",
        "pub const BASE_CSS: &str",
        "--ui-system",
        "--ui-color",
        "--ui-scale",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-theme css output contract should include `{needle}`."
        );
    }

    for needle in [
        "pub struct ThemeTokens",
        "pub struct SemanticColorTokens",
        "pub struct LayoutTokens",
        "pub struct TypographyTokens",
    ] {
        assert!(
            tokens_source.contains(needle),
            "ui-theme token taxonomy should include `{needle}`."
        );
    }

    assert!(
        styling_spec_source.contains("Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量"),
        "styling spec should document ui-theme token->theme->css SSOT path."
    );
    assert!(
        token_baseline_test_source.contains("fn token_scale_baselines_are_regression_testable()"),
        "ui-theme should keep regression baseline test for scale tokens."
    );
    assert!(
        wcag_test_source.contains("WCAG 2.1 AA contrast failed"),
        "ui-theme should keep WCAG contrast regression guard."
    );
}

#[test]
fn autocomplete_token_first_style_contract_is_aggregated_and_ui_root_injected() {
    let styles_source = load_source("src/autocomplete/styles.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let combined_component_sources = format!(
        "{}\n{}\n{}\n{}",
        load_source("src/autocomplete/logic.rs"),
        load_source("src/autocomplete/view.rs"),
        styles_source,
        load_source("src/autocomplete/motion.rs")
    );

    for needle in ["pub const CSS: &str", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "Autocomplete styles should keep token-first static CSS marker `{needle}`."
        );
    }

    assert!(
        css_source.contains("out.push_str(crate::autocomplete::styles::CSS);"),
        "ui-components css aggregator should include autocomplete styles CSS constant."
    );

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should inject component CSS through `{needle}`."
        );
    }

    for forbidden in [
        "tailwind",
        "utility-first",
        "styled_components",
        "emotion",
        "css!(",
    ] {
        assert!(
            !combined_component_sources.contains(forbidden),
            "Autocomplete component sources should not depend on css-framework default token `{forbidden}`.",
        );
    }
}

#[test]
fn autocomplete_visual_desire_default_theme_baseline_contract_is_backed_by_docs_and_e2e() {
    let docs_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_theme_baseline_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let e2e_theme_baseline_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for needle in [
        "component_doc!(",
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
    ] {
        assert!(
            docs_registry_source.contains(needle),
            "docs registry should include theme visual baseline entry `{needle}`."
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "Default theme should feel trustworthy at first glance",
    ] {
        assert!(
            docs_theme_baseline_source.contains(needle),
            "theme visual baseline docs page should include `{needle}`."
        );
    }

    for needle in [
        "docs-app: theme visual baseline renders button/input/overlay",
        "docs-app: theme visual baseline screenshots",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            e2e_theme_baseline_source.contains(needle),
            "theme visual baseline e2e should include `{needle}`."
        );
    }
}

#[test]
fn autocomplete_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "pub(super) fn autocomplete() -> AnyView",
        "title=\"Autocomplete\"",
        "slug=\"autocomplete\"",
        "description=\"Combobox-like autocomplete with baseline-style root attrs, controlled/uncontrolled open state, and baseline-level active highlight motion.\"",
        "title=\"Selection + Validation\"",
        "code_signal=code",
        "title=\"Controlled Open State\"",
        "code_signal=controlled_code",
        "title=\"Disabled + Empty\"",
        "code_signal=states_code",
        "<Autocomplete",
        "is_open=controlled_open",
        "is_disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "collections docs page should include `{needle}` for autocomplete coverage.",
        );
    }
}

#[test]
fn autocomplete_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "id_base=\"docs-autocomplete\".to_string()",
        "label=\"City\".to_string()",
        "disabled_indices=vec![3]",
        "description=\"Search and pick one city\".to_string()",
        "error=\"City is required\".to_string()",
        "placeholder=\"Type…\".to_string()",
        "on_press=Callback::new(move |_| set_invalid.update(|value| *value = !*value))",
        "\"selected: \"",
        "id_base=\"docs-autocomplete-controlled\".to_string()",
        "on_open_change=on_open_change",
        "description=\"Open state is externally controlled\".to_string()",
        "\"open: \"",
        "id_base=\"docs-autocomplete-disabled\".to_string()",
        "id_base=\"docs-autocomplete-empty\".to_string()",
        "placeholder=\"No options\".to_string()",
        "\"disabled selected: \"",
        "\"empty selected: \"",
    ] {
        assert!(
            source.contains(needle),
            "autocomplete docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_docs_page_includes_hello_world_entrypoint() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "data-slot=\"autocomplete-hello-world\"",
        "id_base=\"docs-autocomplete-hello\".to_string()",
        "items=hello_items",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete docs should keep zero-threshold hello-world marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_dx_hello_world_keeps_zero_wiring_default_path() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let readme_source = load_source("src/autocomplete/README.md");

    let docs_section = slice_between(
        &docs_source,
        "pub(super) fn autocomplete() -> AnyView {",
        "pub(super) fn dropdown_menu() -> AnyView {",
    );
    let hello_code_section = slice_between(
        docs_section,
        "let hello_code = Signal::derive(move || {",
        "let code = Signal::derive(move || {",
    );
    let readme_hello_section = slice_between(&readme_source, "## Hello World", "## 受控 open 示例");

    for needle in [
        "id_base=\"city\".to_string()",
        "label=\"City\".to_string()",
        "items=vec![\"Sydney\".to_string(), \"Melbourne\".to_string()]",
    ] {
        assert!(
            hello_code_section.contains(needle),
            "Autocomplete docs hello snippet should keep minimal marker `{needle}`.",
        );
    }

    for forbidden in [
        "signal(",
        "selected_index=selected",
        "set_selected_index=set_selected",
        "on_selected_index_change",
    ] {
        assert!(
            !hello_code_section.contains(forbidden),
            "Autocomplete docs hello path should not require state-machine wiring token `{forbidden}`.",
        );
    }
    for forbidden in [
        "let (hello_selected, set_hello_selected) = signal(",
        "selected_index=hello_selected",
        "set_selected_index=set_hello_selected",
    ] {
        assert!(
            !docs_section.contains(forbidden),
            "Autocomplete docs hello playground should avoid manual wiring token `{forbidden}`.",
        );
    }

    for needle in [
        "id_base=\"city\".to_string()",
        "label=\"City\".to_string()",
        "items=vec![\"Tokyo\".to_string(), \"Osaka\".to_string()]",
    ] {
        assert!(
            readme_hello_section.contains(needle),
            "Autocomplete README hello section should include minimal marker `{needle}`.",
        );
    }
    for forbidden in ["signal(", "selected_index=", "set_selected_index="] {
        assert!(
            !readme_hello_section.contains(forbidden),
            "Autocomplete README hello section should stay zero-wiring and avoid `{forbidden}`.",
        );
    }
}

#[test]
fn autocomplete_docs_playground_exposes_semantic_selector_anchors() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "data-slot=\"autocomplete-validation-playground\"",
        "data-slot=\"autocomplete-controlled-playground\"",
        "data-slot=\"autocomplete-controlled-open\"",
        "data-slot=\"autocomplete-controlled-selected\"",
        "data-slot=\"autocomplete-states-playground\"",
        "data-slot=\"autocomplete-disabled-playground\"",
        "data-slot=\"autocomplete-empty-playground\"",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete docs should expose semantic anchor `{needle}` for e2e and contract tests.",
        );
    }
}

#[test]
fn autocomplete_docs_entry_has_readme_streaming_policy_and_source_paths() {
    let readme_source = load_source("src/autocomplete/README.md");

    for needle in [
        "# Autocomplete",
        "## Streaming 策略",
        "Snapshot",
        "Streaming Optional",
        "fallback=snapshot",
        "## Hello World",
        "## Source-first",
        "components/autocomplete/src/{mod,logic,view,styles,motion}.rs",
        "crates/ui-state-primitives/src/autocomplete.rs",
    ] {
        assert!(
            readme_source.contains(needle),
            "Autocomplete README should include `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let check2_source = load_source("../../components/autocomplete/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "pub(super) fn autocomplete() -> AnyView {",
        "title=\"Hello World\"",
        "code_imports=autocomplete_code_imports.clone()",
        "title=\"Controlled Open State\"",
        "title=\"Disabled + Empty\"",
        "data-slot=\"autocomplete-state-matrix\"",
        "状态矩阵 State Matrix（受控 / 非受控）",
        "data-slot=\"autocomplete-state-rows\"",
        "title=\"Streaming/Snapshot Display\"",
        "data-slot=\"autocomplete-streaming-snapshot\"",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "data-ui-output-state=\"snapshot\"",
        "data-ui-output-state=\"streaming\"",
        "id_base=\"docs-autocomplete-snapshot\".to_string()",
        "id_base=\"docs-autocomplete-streaming\".to_string()",
        "label=\"Snapshot mode\".to_string()",
        "label=\"Streaming preview\".to_string()",
        "data-slot=\"autocomplete-source-first\"",
        "<Snippet",
        "label=\"Copy starter\".to_string()",
        "copyable=true",
        "class_name=\"docs-autocomplete-source-copy\".to_string()",
        "use leptos::prelude::*;\\nuse ui_components::Autocomplete;",
        "data-slot=\"autocomplete-source-paths\"",
        "data-slot=\"autocomplete-source-prerequisites\"",
        "<code>\"component-autocomplete\"</code>",
        "<code>\"inject-css\"</code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "Autocomplete docs should keep docs-product marker `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let missing_imports = missing_import_lines(&raw, &imports);",
        "return compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value());",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground copy-ready pipeline should keep `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";
    assert!(
        script_source.contains(script_needle),
        "dx check script should enforce `{script_needle}`."
    );

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "autocomplete_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "apps/docs-app/src/pages/components/pages/collections.rs",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "components/autocomplete/check2.md should keep docs-product marker `{needle}`."
        );
    }
}

#[test]
fn autocomplete_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_source("src/autocomplete/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "已同步补齐示例与双矩阵",
        "状态矩阵为 `section[data-slot=\"autocomplete-state-matrix\"]`；参数矩阵为 `section[data-slot=\"autocomplete-parameter-matrix\"]`",
        "文档参数矩阵默认值与实现一致",
    ] {
        assert!(
            check2_source.contains(required),
            "components/autocomplete/check2.md should keep docs-sync/state-matrix rule `{required}`.",
        );
    }
}

#[test]
fn autocomplete_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let view_source = load_source("../../components/autocomplete/src/view.rs");
    let logic_source = load_source("../../components/autocomplete/src/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/autocomplete.rs");
    let check2_source = load_source("../../components/autocomplete/check2.md");

    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>,",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional, into)] selected_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] default_selected_index: Option<usize>",
        "#[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>",
        "#[prop(optional)] set_selected_index: Option<WriteSignal<Option<usize>>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "pub fn normalize_open_state(input: OpenStateInput) -> OpenState",
        "pub fn normalize_selection_change(input: SelectionChangeInput) -> SelectionChange",
        "pub fn normalize_accessibility_state(input: AccessibilityStateInput) -> AccessibilityState",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "Autocomplete API/default contract should keep marker `{needle}` for docs sync.",
        );
    }

    for needle in [
        "pub const DEFAULT_LABEL: &str = \"Options\";",
        "pub const DEFAULT_ID_BASE: &str = \"autocomplete\";",
        "pub const DEFAULT_PLACEHOLDER: &str = \"Type…\";",
        "pub const DEFAULT_EMPTY_MESSAGE: &str = \"No matches\";",
        "pub fn resolve_placeholder(placeholder: Option<String>) -> String",
        "pub fn resolve_empty_message(value: Option<String>) -> String",
    ] {
        assert!(
            primitive_source.contains(needle) || logic_source.contains(needle),
            "Autocomplete primitive/default source should keep marker `{needle}`.",
        );
    }

    for needle in [
        "title=\"Hello World\"",
        "title=\"Selection + Validation\"",
        "title=\"Controlled Open State\"",
        "title=\"Disabled + Empty\"",
        "data-slot=\"autocomplete-state-matrix\"",
        "data-slot=\"autocomplete-state-rows\"",
        "data-slot=\"autocomplete-parameter-matrix\"",
        "data-slot=\"autocomplete-parameter-rows\"",
        "<code>\"is_open + on_open_change + default_open\"</code>",
        "<code>\"selected_index + on_selected_index_change + default_selected_index\"</code>",
        "<code>\"set_selected_index\"</code>",
        "<code>\"is_disabled / is_required / is_invalid\"</code>",
        "<code>\"label / id_base / placeholder / empty_message\"</code>",
        "is_open=controlled_open",
        "on_open_change=on_open_change",
        "selected_index=controlled_selected",
        "set_selected_index=set_controlled_selected",
        "is_disabled=true",
        "selected_index=empty_selected",
        "set_selected_index=set_empty_selected",
    ] {
        assert!(
            docs_source.contains(needle),
            "Autocomplete docs should keep synced example/matrix marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "apps/docs-app/src/pages/components/pages/collections.rs::autocomplete",
        "autocomplete_check2_documents_docs_sync_and_state_matrix_rules",
        "autocomplete_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "components/autocomplete/check2.md should keep docs-sync evidence marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: autocomplete docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include docs-sync/state-matrix marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let check2_source = load_source("src/autocomplete/check2.md");

    assert!(
        check2_source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "Autocomplete check2 should mark docs-sync/state-matrix checklist item complete.",
    );

    for needle in [
        "apps/docs-app/src/pages/components/pages/collections.rs::autocomplete",
        "data-slot=\"autocomplete-state-matrix\"",
        "data-slot=\"autocomplete-parameter-matrix\"",
        "DEFAULT_LABEL",
        "DEFAULT_ID_BASE",
        "DEFAULT_PLACEHOLDER",
        "DEFAULT_EMPTY_MESSAGE",
        "autocomplete_check2_documents_docs_sync_and_state_matrix_rules",
        "autocomplete_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "autocomplete_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete check2 docs-sync/state-matrix section should reference `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("src/autocomplete/check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "已形成新手优先路径：`Hello World` 零门槛示例 + `常见用法`（默认 API）在前，`受控 open 示例`（进阶控制）在后",
        "`apps/docs-app/src/pages/components/pages.rs` 继续保留 `component_doc!(\"Autocomplete\", \"autocomplete\", \"Collections\", collections::autocomplete)` 文档入口",
        "验证记录：`bash -n scripts/check-ui-components-dx.sh` 通过",
    ] {
        assert!(
            check2_source.contains(required),
            "components/autocomplete/check2.md should keep documentation-as-product rule `{required}`.",
        );
    }
}

#[test]
fn autocomplete_documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = load_source("../../components/autocomplete/src/README.md");
    let docs_catalog_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");

    assert!(
        readme_source.contains("# Autocomplete"),
        "Autocomplete should keep a discoverable README entry.",
    );

    for needle in [
        "## Hello World",
        "先跑默认路径，不需要先理解分层细节。",
        "## 常见用法",
        "非受控 open：仅传 `default_open`",
        "受控 open：传 `is_open + on_open_change`",
        "## 受控 open 示例",
        "进阶控制路径：当你需要把开合状态与上层流程同步时，再使用受控 open。",
        "## Architecture Layers",
        "## Source-first",
    ] {
        assert!(
            readme_source.contains(needle),
            "Autocomplete README should keep beginner-first marker `{needle}`.",
        );
    }

    let hello_pos = readme_source.find("## Hello World");
    let common_pos = readme_source.find("## 常见用法");
    let advanced_pos = readme_source.find("## 受控 open 示例");
    assert!(
        hello_pos.is_some() && common_pos.is_some() && advanced_pos.is_some(),
        "Autocomplete README should keep hello/common/advanced sections.",
    );
    assert!(
        hello_pos < common_pos && common_pos < advanced_pos,
        "Autocomplete README should keep beginner path before advanced control.",
    );

    for needle in [
        "component_doc!(",
        "\"Autocomplete\",",
        "\"autocomplete\",",
        "\"Collections\",",
        "collections::autocomplete",
    ] {
        assert!(
            docs_catalog_source.contains(needle),
            "docs-app catalog should keep Autocomplete entrypoint marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: autocomplete documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include documentation-as-product marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_check2_marks_documentation_as_product_item_complete() {
    let check2_source = load_source("src/autocomplete/check2.md");

    assert!(
        check2_source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "Autocomplete check2 should mark documentation-as-product item complete.",
    );

    for needle in [
        "components/autocomplete/src/README.md",
        "apps/docs-app/src/pages/components/pages.rs",
        "autocomplete_check2_documents_documentation_as_product_rules",
        "autocomplete_documentation_entry_exists_with_beginner_first_progression",
        "autocomplete_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete check2 documentation-as-product section should reference `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("src/autocomplete/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
        "AI Spec 子条款对 `Autocomplete` 为 N/A（组件不承载 Spec 输入协议，交互验收以 props/state Workbench + Streaming/Snapshot 契约为准）",
        "autocomplete_check2_documents_interactive_playground_rules",
        "autocomplete_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "autocomplete_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            check2_source.contains(required),
            "components/autocomplete/check2.md should keep interactive-playground rule `{required}`.",
        );
    }
}

#[test]
fn autocomplete_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for marker in [
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "test_config_signal=workbench_actual_config",
        "test_css_source=workbench_test_css",
        "controls=move || view! {",
        "data-slot=\"autocomplete-workbench-controls\"",
        "data-slot=\"autocomplete-workbench-canvas\"",
        "Persist selected index (optional)",
        "\"Toggle open\"",
        "title=\"Streaming/Snapshot Display\"",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "<AiSpace mode=snapshot_mode output_status=verified_output>",
        "<AiSpace mode=streaming_mode output_status=draft_output>",
    ] {
        assert!(
            docs_page_source.contains(marker),
            "autocomplete docs interactive playground should include `{marker}`.",
        );
    }

    for marker in [
        "<div data-playground-scope=scope_id.clone()>",
        "<Card class_name=\"playground__preview\".to_string()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<div attr:data-slot=\"playground-controls\">",
        "Card class_name=\"playground__panel playground__controls\".to_string()",
    ] {
        assert!(
            playground_source.contains(marker),
            "docs-app Playground should keep interactive preview marker `{marker}`.",
        );
    }
}

#[test]
fn autocomplete_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_autocomplete_contract.spec.mjs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for marker in [
        "docs-app autocomplete key flow is repeatable with semantic contract breakpoints",
        "docs-app autocomplete high-risk overlay/focus/keyboard path is replayable with semantic breakpoints",
        "#docs-autocomplete-controlled-input",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"data-closed\", \"true\")",
        "toHaveText(\"selected: 3\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(marker),
            "autocomplete interactive-playground e2e flow should include `{marker}`.",
        );
    }

    for marker in [
        "data-slot=\"autocomplete-controlled-playground\"",
        "data-slot=\"autocomplete-controlled-open\"",
        "data-slot=\"autocomplete-controlled-selected\"",
        "data-slot=\"autocomplete-workbench-controls\"",
        "data-slot=\"autocomplete-workbench-canvas\"",
    ] {
        assert!(
            docs_page_source.contains(marker),
            "autocomplete docs should expose stable interactive anchor `{marker}` for repeatable e2e replay.",
        );
    }
}

#[test]
fn autocomplete_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for marker in [
        "echo \"[dx] contract: autocomplete interactive playground docs acceptance surface\"",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_interactive_playground_rules",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(marker),
            "DX check script should include interactive-playground marker `{marker}`.",
        );
    }
}

#[test]
fn autocomplete_check2_marks_interactive_playground_item_complete() {
    let check2_source = load_source("src/autocomplete/check2.md");

    assert!(
        check2_source.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "autocomplete check2 should mark interactive-playground item complete.",
    );

    for marker in [
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"autocomplete-workbench-controls\"",
        "docs-app autocomplete key flow is repeatable with semantic contract breakpoints",
        "AI Spec 子条款对 `Autocomplete` 为 N/A（组件不承载 Spec 输入协议，交互验收以 props/state Workbench + Streaming/Snapshot 契约为准）",
        "autocomplete_check2_documents_interactive_playground_rules",
        "autocomplete_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "autocomplete_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "autocomplete_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-components-dx.sh",
        "e2e/tests/docs_app_autocomplete_contract.spec.mjs",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "autocomplete check2 interactive-playground section should include `{marker}`.",
        );
    }
}

#[test]
fn autocomplete_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("src/autocomplete/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
        "autocomplete_check2_documents_source_first_copy_paste_ready_rules",
        "autocomplete_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            check2_source.contains(required),
            "components/autocomplete/check2.md should include source-first marker `{required}`.",
        );
    }
}

#[test]
fn autocomplete_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    autocomplete_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot();

    for marker in [
        "data-slot=\"autocomplete-source-first\"",
        "label=\"Copy starter\".to_string()",
        "copyable=true",
        "class_name=\"docs-autocomplete-source-copy\".to_string()",
        "code_imports=autocomplete_code_imports.clone()",
        "use leptos::prelude::*;\\nuse ui_components::Autocomplete;",
        "data-slot=\"autocomplete-source-paths\"",
        "components/autocomplete/src/mod.rs",
        "components/autocomplete/src/logic.rs",
        "components/autocomplete/src/view.rs",
        "components/autocomplete/src/styles.rs",
        "components/autocomplete/src/motion.rs",
        "data-slot=\"autocomplete-source-prerequisites\"",
        "<code>\"component-autocomplete\"</code>",
        "<code>\"inject-css\"</code>",
    ] {
        assert!(
            docs_source.contains(marker),
            "autocomplete source-first docs should include `{marker}`.",
        );
    }

    for marker in [
        "code_imports: Option<String>",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "missing_import_lines(&raw, &imports)",
        "<CodeBlock code=resolved_code.get() />",
        "attr:data-slot=\"playground-code\"",
    ] {
        assert!(
            playground_source.contains(marker),
            "playground source-first copy path should include `{marker}`.",
        );
    }
}

#[test]
fn autocomplete_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for marker in [
        "echo \"[dx] contract: autocomplete source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(marker),
            "DX check script should include source-first marker `{marker}`.",
        );
    }
}

#[test]
fn autocomplete_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_source("src/autocomplete/check2.md");

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        ),
        "autocomplete check2 should mark source-first copy-paste-ready item complete.",
    );

    for marker in [
        "data-slot=\"autocomplete-source-first\"",
        "Snippet(label=\"Copy starter\", copyable=true)",
        "code_imports=autocomplete_code_imports",
        "components/autocomplete/src/mod.rs",
        "components/autocomplete/src/logic.rs",
        "components/autocomplete/src/view.rs",
        "components/autocomplete/src/styles.rs",
        "components/autocomplete/src/motion.rs",
        "autocomplete_check2_documents_source_first_copy_paste_ready_rules",
        "autocomplete_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "autocomplete_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "autocomplete check2 source-first section should include `{marker}`.",
        );
    }
}

#[test]
fn autocomplete_non_composite_api_avoids_parallel_slot_convention() {
    let view_source = load_source("src/autocomplete/view.rs");
    let readme_source = load_source("src/autocomplete/README.md");

    for needle in ["#[component]\npub fn Autocomplete(", "items: Vec<String>"] {
        assert!(
            view_source.contains(needle),
            "Autocomplete should keep non-composite API marker `{needle}`."
        );
    }

    for forbidden in [
        "labels:",
        "titles:",
        "panels:",
        "children: Children",
        "AutocompleteItem",
        "ItemSpec",
    ] {
        assert!(
            !view_source.contains(forbidden) && !readme_source.contains(forbidden),
            "Autocomplete should avoid parallel-slot convention token `{forbidden}`."
        );
    }
}

#[test]
fn autocomplete_non_drag_component_avoids_drag_macro_micro_state_machine_contract() {
    let logic_source = load_source("src/autocomplete/logic.rs");
    let view_source = load_source("src/autocomplete/view.rs");
    let motion_source = load_source("src/autocomplete/motion.rs");

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "on:drag",
        "ondrag",
        "draggable=",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Autocomplete should not expose drag macro/micro state-machine token `{forbidden}`.",
        );
    }
}

#[test]
fn autocomplete_env_streams_are_delegated_to_headless_without_component_raw_event_flood() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let headless_popover_source = load_source("../ui-headless/src/popover_position.rs");

    for needle in [
        "use_popover_position(PopoverPositionOptions {",
        "position.top_px.get()",
        "position.left_px.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should consume headless env stream output `{needle}`."
        );
    }

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "match_media",
        "on:resize",
        "on:scroll",
        "BreakpointChanged",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Autocomplete component layer should avoid raw env stream token `{forbidden}`.",
        );
    }

    for needle in [
        "web_sys::ResizeObserver",
        "add_event_listener_with_callback(\"resize\",",
        "add_event_listener_with_callback_and_bool(",
        "\"scroll\"",
        "if should_update_scalar(top_px.get_untracked(), computed.top) {",
    ] {
        assert!(
            headless_popover_source.contains(needle),
            "ui-headless popover position should keep env stream sampling/guard marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_event_light_cone_contract_is_not_introduced_for_non_grid_scope() {
    let logic_source = load_source("src/autocomplete/logic.rs");
    let view_source = load_source("src/autocomplete/view.rs");
    let motion_source = load_source("src/autocomplete/motion.rs");

    for needle in [
        "items: Vec<String>",
        "let filtered_indices = Memo::new",
        "logic::filter_indices(",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete should keep single-list filtering flow marker `{needle}`."
        );
    }

    for forbidden in [
        "SelectionState::All",
        "ContextBus",
        "context_bus",
        "provide_context(",
        "use_context(",
        "selector(",
        "Selector<",
        "prop_drilling",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Autocomplete should avoid event-light-cone token `{forbidden}` for non-grid scope.",
        );
    }
}

#[test]
fn autocomplete_causality_bus_contract_is_not_introduced_for_local_interaction_scope() {
    let logic_source = load_source("src/autocomplete/logic.rs");
    let view_source = load_source("src/autocomplete/view.rs");
    let motion_source = load_source("src/autocomplete/motion.rs");

    for needle in [
        "let on_option_click = aria.handlers.on_option_click;",
        "request_selected_index_change.run(Some(original_index));",
        "set_open,",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete should keep direct local interaction flow marker `{needle}`."
        );
    }

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "causality_bus",
        "broadcast(",
        "subscribe(",
        "subscriber",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Autocomplete should avoid causality-bus token `{forbidden}` for local interaction scope.",
        );
    }
}

#[test]
fn autocomplete_styles_depend_on_explicit_state_markers_only() {
    let styles_source = load_source("src/autocomplete/styles.rs");
    let view_source = load_source("src/autocomplete/view.rs");

    for required in [
        ".ui-autocomplete[data-empty=\"true\"] .ui-autocomplete__input",
        ".ui-autocomplete[data-controlled=\"true\"] .ui-autocomplete__control",
        ".ui-autocomplete[data-has-disabled-options=\"true\"] .ui-autocomplete__listbox",
        ".ui-autocomplete__panel[data-placement=\"bottom-start\"]",
        ".ui-autocomplete__option[data-selected=\\\"true\\\"]",
        ".ui-autocomplete__option[data-focused=\\\"true\\\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "Autocomplete styles should use explicit semantic marker `{required}`.",
        );
    }

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        " > * > * > ",
        " + div + div",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Autocomplete styles should avoid brittle structural selector `{forbidden}`.",
        );
    }

    for needle in [
        "let panel_vars = move || {",
        "\"--ui-popover-top: {}px; --ui-popover-left: {}px; --ui-popover-anchor-width: {}px;\"",
        "style=panel_vars",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should keep runtime style variable marker `{needle}`.",
        );
    }

    let inline_style_count = view_source.matches("style=").count();
    assert_eq!(
        inline_style_count, 1,
        "Autocomplete view should keep exactly one inline style binding for required css variables."
    );
    assert!(
        !view_source.contains("style=\""),
        "Autocomplete view should avoid hardcoded inline style literals."
    );
}

#[test]
fn autocomplete_semantics_suite_is_contract_first_not_snapshot_only() {
    let source = load_source("tests/autocomplete_semantics.rs");
    let has_rust_snapshot_macro = source
        .lines()
        .any(|line| line.trim_start().starts_with("assert_snapshot!("));
    let has_js_snapshot_matcher = source
        .lines()
        .any(|line| line.trim_start().starts_with("toMatchSnapshot("));

    assert!(
        !has_rust_snapshot_macro && !has_js_snapshot_matcher,
        "Autocomplete semantics suite should stay contract-first and avoid snapshot-only assertions."
    );
}

#[test]
fn autocomplete_semantic_contract_matrix_covers_core_branches_and_platform_paths() {
    let source = load_source("tests/autocomplete_semantics.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_autocomplete_contract.spec.mjs");

    for needle in [
        "fn autocomplete_supports_controlled_and_uncontrolled_open_state()",
        "fn autocomplete_emits_baseline_style_state_data_attributes()",
        "fn autocomplete_escape_stops_propagation_when_open()",
        "fn autocomplete_non_registered_collection_flow_uses_filtered_vec_order_not_hashset_iteration()",
        "fn autocomplete_platform_cross_target_compile_only_contract_is_explicit_and_non_wasm_safe()",
        "fn autocomplete_headless_web_ssr_mutex_contract_is_enforced_by_compile_error_and_platform_probe()",
        "fn autocomplete_ui_motion_non_wasm_stub_contract_is_explicit_and_predictable()",
        "fn autocomplete_reduced_motion_ssr_wasm_branches_keep_semantics_consistent()",
        "fn autocomplete_view_macro_complexity_is_split_into_semantic_subrenders()",
        "fn autocomplete_view_functional_split_prefers_plain_functions_over_local_components()",
        "fn autocomplete_static_fragments_are_constantized_or_absent_for_simple_combobox_layout()",
        "fn autocomplete_inner_html_usage_is_forbidden_in_component_and_docs_examples()",
        "fn autocomplete_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated()",
        "fn autocomplete_dx_playground_supports_css_hot_reload_without_wasm_rebuild()",
        "fn autocomplete_dx_workbench_supports_optional_state_persistence_and_isolated_canvas()",
        "fn autocomplete_performance_governance_budget_is_defined_and_blocking()",
        "fn autocomplete_platform_contract_preserves_headless_mutex_and_motion_stub_references()",
        "fn autocomplete_streaming_term_is_limited_to_llm_output_render_modes()",
        "fn autocomplete_snapshot_is_foundational_and_complete_config_renders_stably()",
        "fn autocomplete_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status()",
        "fn autocomplete_semantics_suite_is_contract_first_not_snapshot_only()",
    ] {
        assert!(
            source.contains(needle),
            "Autocomplete semantic matrix should include branch guard `{needle}`.",
        );
    }

    for needle in [
        "await controlledInput.fill(\"Shen\")",
        "await option.click()",
        "await expect(controlledRoot).toHaveAttribute(\"data-open\", \"true\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Autocomplete e2e should include interaction path marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_component_files_are_layered_and_spec_file_is_absent() {
    let module_source = load_source("src/autocomplete/mod.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let styles_source = load_source("src/autocomplete/styles.rs");
    let view_source = load_source("src/autocomplete/view.rs");
    let motion_source = load_source("src/autocomplete/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::AutocompleteMotion;",
        "pub use view::Autocomplete;",
    ] {
        assert!(
            module_source.contains(needle),
            "Autocomplete mod.rs should keep layered export marker `{needle}`.",
        );
    }

    assert!(
        !module_source.contains("pub mod logic") && !module_source.contains("pub mod view"),
        "Autocomplete should not expose internal implementation modules."
    );
    assert!(
        logic_source.contains("pub fn normalize_root_state("),
        "Autocomplete logic.rs should keep normalization entrypoint."
    );
    assert!(
        styles_source.contains("pub const CSS: &str"),
        "Autocomplete styles.rs should keep static css contract."
    );
    assert!(
        view_source.contains("#[component]\npub fn Autocomplete("),
        "Autocomplete view.rs should keep component entrypoint."
    );
    assert!(
        motion_source.contains("pub struct AutocompleteMotion"),
        "Autocomplete motion.rs should keep motion contract."
    );
    assert!(
        !resolve_path("src/autocomplete/spec.rs").exists(),
        "Autocomplete should not add spec.rs for this component scope."
    );
}

#[test]
fn autocomplete_component_directory_standard_files_follow_contract_and_na_spec() {
    let module_source = load_source("src/autocomplete/mod.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let styles_source = load_source("src/autocomplete/styles.rs");
    let view_source = load_source("src/autocomplete/view.rs");
    let motion_source = load_source("src/autocomplete/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::AutocompleteMotion;",
        "pub use view::Autocomplete;",
    ] {
        assert!(
            module_source.contains(needle),
            "Autocomplete mod.rs should keep minimal stable export marker `{needle}`."
        );
    }

    for forbidden in ["pub mod logic", "pub mod view"] {
        assert!(
            !module_source.contains(forbidden),
            "Autocomplete mod.rs should avoid over-export `{forbidden}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::autocomplete::{",
        "pub fn normalize_root_state(",
        "pub fn resolve_root_data_state(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Autocomplete logic.rs should keep derivation marker `{needle}`."
        );
    }
    for forbidden in ["view! {", "NodeRef<", "web_sys::"] {
        assert!(
            !logic_source.contains(forbidden),
            "Autocomplete logic.rs should avoid view/platform token `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "Autocomplete styles.rs should keep token-first marker `{needle}`."
        );
    }
    for forbidden in ["on:click", "on:keydown", "style=\"", "style:top="] {
        assert!(
            !styles_source.contains(forbidden),
            "Autocomplete styles.rs should avoid runtime/event token `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "pub fn Autocomplete(",
        "use_combo_box(ComboBoxOptions {",
        "use_text_field(TextFieldOptions {",
        "let root_state = logic::normalize_root_state(logic::RootStateInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view.rs should keep headless/render assembly marker `{needle}`."
        );
    }

    for needle in [
        "pub struct AutocompleteMotion",
        "pub fn attach_popover_motion(",
        "pub fn sanitize_motion(motion: AutocompleteMotion) -> AutocompleteMotion",
    ] {
        assert!(
            motion_source.contains(needle),
            "Autocomplete motion.rs should keep motion-contract mapping marker `{needle}`."
        );
    }
    for forbidden in ["role=", "aria-", "use_combo_box(", "view! {"] {
        assert!(
            !motion_source.contains(forbidden),
            "Autocomplete motion.rs should avoid semantic/view token `{forbidden}`."
        );
    }

    for absent in ["src/autocomplete/spec.rs", "src/autocomplete/render.rs"] {
        let path = resolve_path(absent);
        assert!(
            !path.exists(),
            "Autocomplete simple component scope should not include `{}`.",
            path.display()
        );
    }
}

#[test]
fn autocomplete_component_directory_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_component_directory_standard_files_follow_contract_and_na_spec";
    assert!(
        script_source.contains(needle),
        "component-files script should cover autocomplete directory contract `{needle}`."
    );
}

#[test]
fn autocomplete_check2_marks_component_directory_standard_files_contract_complete() {
    let check2_source = load_source("src/autocomplete/check2.md");
    assert!(
        check2_source.contains("- [x] 组件目录标准文件落点正确。"),
        "autocomplete check2 should mark component-directory item complete."
    );
    for needle in [
        "component_directory_standard_files_follow_contract_and_na_spec",
        "autocomplete_component_directory_standard_files_follow_contract_and_na_spec",
        "component_directory_check_script_covers_contract",
        "autocomplete_component_directory_check_script_covers_contract",
        "components/autocomplete/src/mod.rs",
        "components/autocomplete/src/logic.rs",
        "components/autocomplete/src/styles.rs",
        "components/autocomplete/src/view.rs",
        "components/autocomplete/src/motion.rs",
        "components/autocomplete/src/spec.rs",
        "components/autocomplete/src/render.rs",
        "scripts/check-ui-components-component-files.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "autocomplete check2 component-directory evidence should reference `{needle}`."
        );
    }
}

#[test]
fn autocomplete_file_placement_discipline_is_strict_for_component_scope() {
    let component_src_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/autocomplete/src");
    let mod_source = load_source("../../components/autocomplete/src/mod.rs");
    let logic_source = load_source("../../components/autocomplete/src/logic.rs");
    let styles_source = load_source("../../components/autocomplete/src/styles.rs");
    let view_source = load_source("../../components/autocomplete/src/view.rs");
    let motion_source = load_source("../../components/autocomplete/src/motion.rs");
    let protocol_source = load_source("../../components/autocomplete/src/protocol.rs");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        let path = component_src_dir.join(required);
        assert!(
            path.exists(),
            "autocomplete file-placement discipline requires `{}`.",
            path.display()
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        let path = component_src_dir.join(forbidden);
        assert!(
            !path.exists(),
            "autocomplete should not introduce forbidden file `{}`.",
            path.display()
        );
    }

    assert!(
        component_src_dir.join("protocol.rs").exists(),
        "autocomplete keeps protocol.rs as schema/projection sidecar."
    );
    for needle in [
        "pub enum AutocompleteComponentSchemaVersion",
        "pub struct AutocompleteComponentSpec",
        "#[serde(default)]",
    ] {
        assert!(
            protocol_source.contains(needle),
            "protocol.rs should stay schema-only via `{needle}`."
        );
    }

    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");
    for needle in [
        "pub use view::Autocomplete;",
        "pub fn normalize_root_state(",
        "pub const CSS: &str",
        "view! {",
        "pub struct AutocompleteMotion",
    ] {
        assert!(
            combined.contains(needle),
            "autocomplete file-placement discipline should keep marker `{needle}`."
        );
    }
}

#[test]
fn autocomplete_file_placement_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_file_placement_discipline_is_strict_for_component_scope";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn autocomplete_check2_marks_file_placement_discipline_contract_complete() {
    let check2_root = load_source("../../components/autocomplete/check2.md");
    let check2_src = load_source("../../components/autocomplete/src/check2.md");

    for source in [check2_root, check2_src] {
        assert!(
            source.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
            "autocomplete check2 should mark file-placement-discipline item complete."
        );

        for needle in [
            "components/autocomplete/src/mod.rs",
            "components/autocomplete/src/logic.rs",
            "components/autocomplete/src/styles.rs",
            "components/autocomplete/src/view.rs",
            "components/autocomplete/src/motion.rs",
            "components/autocomplete/src/protocol.rs",
            "render.rs",
            "spec.rs",
            "components/autocomplete/test/semantics.rs::file_placement_discipline_is_strict_for_component_scope",
            "components/autocomplete/test/semantics.rs::file_placement_check_script_covers_contract",
            "components/autocomplete/test/autocomplete_semantics.rs::autocomplete_file_placement_discipline_is_strict_for_component_scope",
            "components/autocomplete/test/autocomplete_semantics.rs::autocomplete_file_placement_check_script_covers_contract",
            "components/autocomplete/test/autocomplete_semantics.rs::autocomplete_check2_marks_file_placement_discipline_contract_complete",
            "scripts/check-ui-components-component-files.sh",
            "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_file_placement_discipline_is_strict_for_component_scope",
        ] {
            assert!(
                source.contains(needle),
                "autocomplete check2 file-placement section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn autocomplete_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let component_src_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/autocomplete/src");
    let mod_source = load_source("../../components/autocomplete/src/mod.rs");
    let logic_source = load_source("../../components/autocomplete/src/logic.rs");
    let view_source = load_source("../../components/autocomplete/src/view.rs");
    let motion_source = load_source("../../components/autocomplete/src/motion.rs");
    let protocol_source = load_source("../../components/autocomplete/src/protocol.rs");
    let readme_source = load_source("../../components/autocomplete/src/README.md");
    let button_spec_source = load_source("../../components/button/src/spec.rs");

    assert!(
        !component_src_dir.join("spec.rs").exists(),
        "autocomplete simple component scope should keep spec.rs absent."
    );

    let combined = format!(
        "{mod_source}\n{logic_source}\n{view_source}\n{motion_source}\n{protocol_source}\n{readme_source}"
    );
    for forbidden in ["AutocompleteSpec", "spec::", "Spec::new()", ".render()"] {
        assert!(
            !combined.contains(forbidden),
            "autocomplete should not expose hyper-structure builder token `{forbidden}`."
        );
    }

    for needle in [
        "pub struct ButtonSpec",
        "impl ButtonSpec",
        "pub fn new() -> Self",
        "pub fn render(self) -> impl IntoView",
    ] {
        assert!(
            button_spec_source.contains(needle),
            "complex-component baseline should remain in button spec via `{needle}`."
        );
    }
}

#[test]
fn autocomplete_hyper_structure_builder_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn autocomplete_check2_marks_hyper_structure_builder_item_complete() {
    let check2_root = load_source("../../components/autocomplete/check2.md");
    let check2_src = load_source("../../components/autocomplete/src/check2.md");

    for source in [check2_root, check2_src] {
        assert!(
            source.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
            "autocomplete check2 should mark hyper-structure-builder item complete."
        );

        for needle in [
            "components/autocomplete/src/spec.rs",
            "components/button/src/spec.rs",
            "components/autocomplete/test/semantics.rs::{hyper_structure_builder_spec_is_not_applicable_for_simple_component",
            "hyper_structure_builder_check_script_covers_contract",
            "check2_marks_hyper_structure_builder_item_complete",
            "components/autocomplete/test/autocomplete_semantics.rs::{autocomplete_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
            "autocomplete_hyper_structure_builder_check_script_covers_contract",
            "autocomplete_check2_marks_hyper_structure_builder_item_complete",
            "scripts/check-ui-components-component-files.sh",
            "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        ] {
            assert!(
                source.contains(needle),
                "autocomplete check2 hyper-structure-builder section should reference `{needle}`."
            );
        }
    }
}

#[test]
fn autocomplete_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    for required_file in [
        "src/autocomplete/Component.toml",
        "src/autocomplete/autocomplete.rbi",
    ] {
        assert!(
            resolve_path(required_file).exists(),
            "autocomplete context-compression artifact should exist: `{required_file}`."
        );
    }

    let manifest_source = load_source("src/autocomplete/Component.toml");
    let rbi_source = load_source("src/autocomplete/autocomplete.rbi");
    let view_source = load_source("../../components/autocomplete/src/view.rs");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Autocomplete\"",
        "crate = \"ui-autocomplete\"",
        "rbi = \"autocomplete.rbi\"",
        "name = \"id_base\"",
        "name = \"label\"",
        "name = \"items\"",
        "name = \"selected_index\"",
        "name = \"default_selected_index\"",
        "name = \"on_selected_index_change\"",
        "name = \"set_selected_index\"",
        "name = \"is_open\"",
        "name = \"open\"",
        "name = \"default_open\"",
        "name = \"on_open_change\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "autocomplete Component.toml should keep context-compression marker `{needle}`."
        );
    }

    for needle in [
        "pub use crate::motion::AutocompleteMotion;",
        "pub fn sanitize_motion(",
        "pub fn sanitize_popover_motion(",
        "pub fn Autocomplete(",
        "id_base: String",
        "label: String",
        "items: Vec<String>",
        "selected_index: Option<leptos::prelude::Signal<Option<usize>>>",
        "default_selected_index: Option<usize>",
        "on_selected_index_change: Option<leptos::prelude::Callback<Option<usize>>>",
        "set_selected_index: Option<leptos::prelude::WriteSignal<Option<usize>>>",
        "is_open: Option<leptos::prelude::Signal<bool>>",
        "open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
        "motion: crate::motion::AutocompleteMotion",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "autocomplete RBI projection should keep signature marker `{needle}`."
        );
    }

    for needle in [
        "pub fn Autocomplete(",
        "id_base: String,",
        "label: String,",
        "items: Vec<String>,",
        "selected_index: Option<Signal<Option<usize>>>",
        "default_selected_index: Option<usize>",
        "on_selected_index_change: Option<Callback<Option<usize>>>",
        "set_selected_index: Option<WriteSignal<Option<usize>>>",
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] motion: AutocompleteMotion",
    ] {
        assert!(
            view_source.contains(needle),
            "autocomplete view signature should include `{needle}` for manifest/rbi drift detection."
        );
    }
}

#[test]
fn autocomplete_component_files_check_script_covers_context_compression_manifest_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    let needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`.",
    );
}

#[test]
fn autocomplete_check2_marks_context_compression_manifest_and_rbi_contract_complete() {
    let source = load_source("../../components/autocomplete/check2.md");

    assert!(
        source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "autocomplete check2 should mark context-compression manifest/rbi gate complete.",
    );

    for needle in [
        "components/autocomplete/src/Component.toml",
        "components/autocomplete/src/autocomplete.rbi",
        "context_compression_manifest_and_rbi_projection_are_present_and_current",
        "autocomplete_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "component_files_check_script_covers_context_compression_manifest_contract",
        "scripts/check-ui-components-component-files.sh",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            source.contains(needle),
            "autocomplete check2 context-compression section should reference `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_tree_shaking_feature_gates_are_explicit() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let tree_shaking_script_source =
        load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let tree_shaking_budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "component-autocomplete = [\"component-active_highlight\", \"component-popover\", \"dep:ui-autocomplete\"]",
        "#[cfg(feature = \"component-autocomplete\")]\npub use ui_autocomplete as autocomplete;",
        "#[cfg(feature = \"component-autocomplete\")]\n    out.push_str(crate::autocomplete::styles::CSS);",
    ] {
        assert!(
            cargo_source.contains(needle)
                || lib_source.contains(needle)
                || css_source.contains(needle),
            "Autocomplete tree-shaking contract should include `{needle}`.",
        );
    }

    for needle in [
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "cargo tree -e features -i ui-components -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_shaking_script_source.contains(needle)
                || tree_shaking_budget_source.contains(needle),
            "Tree-shaking CI pipeline should include `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let tree_shaking_script_source =
        load_source("../../scripts/check-ui-components-tree-shaking.sh");

    for needle in [
        "AUTOCOMPLETE_MIN_FEATURES=\"component-autocomplete,inject-css\"",
        "autocomplete_tree_shaking_feature_gates_are_explicit",
        "autocomplete_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "autocomplete_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "AUTOCOMPLETE_TREE_OUTPUT=\"$(cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$AUTOCOMPLETE_MIN_FEATURES\")\"",
        "missing command-line feature: component-autocomplete",
        "missing command-line feature: inject-css for autocomplete minimal tree",
        "autocomplete minimal feature tree should not pull all-components",
        "[tree-shaking] autocomplete minimal wasm check",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$AUTOCOMPLETE_MIN_FEATURES\"",
    ] {
        assert!(
            tree_shaking_script_source.contains(needle),
            "autocomplete tree-shaking script should include `{needle}`."
        );
    }
}

#[test]
fn autocomplete_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_source("src/autocomplete/check2.md");

    assert!(
        check2_source.contains("- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。"),
        "autocomplete check2 should mark tree-shaking feature-pruning item complete."
    );

    for needle in [
        "component-autocomplete = [\"component-active_highlight\", \"component-popover\", \"dep:ui-autocomplete\"]",
        "#[cfg(feature = \"component-autocomplete\")] pub use ui_autocomplete as autocomplete;",
        "#[cfg(feature = \"component-autocomplete\")] out.push_str(crate::autocomplete::styles::CSS);",
        "tree_shaking_contract_keeps_feature_gates_and_ci_budget_pipeline_explicit",
        "tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "autocomplete_tree_shaking_feature_gates_are_explicit",
        "autocomplete_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "autocomplete_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "scripts/check-ui-components-tree-shaking.sh",
        "cargo tree -e features -p ui-components --no-default-features --features component-autocomplete,inject-css",
        "cargo tree -e features -i ui-components -p web-demo",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-autocomplete,inject-css",
    ] {
        assert!(
            check2_source.contains(needle),
            "autocomplete tree-shaking check2 section should reference `{needle}`."
        );
    }
}

#[test]
fn autocomplete_platform_contract_preserves_headless_mutex_and_motion_stub_references() {
    let headless_lib = load_source("../ui-headless/src/lib.rs");
    let motion_lib = load_source("../ui-motion/src/lib.rs");
    let view_source = load_source("src/autocomplete/view.rs");

    for needle in [
        "feature = \"web\"",
        "feature = \"ssr\"",
        "compile_error!(",
        "features `web` and `ssr` are mutually exclusive; enable exactly one",
    ] {
        assert!(
            headless_lib.contains(needle),
            "ui-headless should preserve web/ssr mutex marker `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
    ] {
        assert!(
            motion_lib.contains(needle),
            "ui-motion should expose non-wasm fallback marker `{needle}`.",
        );
    }

    for forbidden in ["web_sys::window", "window()", "document()"] {
        assert!(
            !view_source.contains(forbidden),
            "Autocomplete view should not directly hard-bind browser-only API `{forbidden}`.",
        );
    }
}

#[test]
fn autocomplete_platform_cross_target_compile_only_contract_is_explicit_and_non_wasm_safe() {
    let platform_script = load_source("../../scripts/check-ui-components-platforms.sh");
    let headless_lib = load_source("../ui-headless/src/lib.rs");
    let module_source = load_source("src/autocomplete/mod.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let view_source = load_source("src/autocomplete/view.rs");
    let styles_source = load_source("src/autocomplete/styles.rs");
    let motion_source = load_source("src/autocomplete/motion.rs");
    let check2_source = load_source("src/autocomplete/check2.md");

    for needle in [
        "[platform] compile-only: default native path",
        "cargo check -p ui-components",
        "[platform] compile-only: ssr native path",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "[platform] compile-only: web wasm path (ui-headless)",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "[platform] compile-only: autocomplete native path",
        "cargo check -p ui-components --no-default-features --features component-autocomplete,inject-css",
        "[platform] compile-only: autocomplete wasm path",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-autocomplete,inject-css",
        "[platform] source guard: non-wasm autocomplete files must not reference web_sys",
        "components/autocomplete/src/view.rs",
    ] {
        assert!(
            platform_script.contains(needle),
            "platform compile-only/source-guard pipeline should include `{needle}`.",
        );
    }

    for needle in [
        "feature = \"web\"",
        "feature = \"ssr\"",
        "compile_error!(",
        "features `web` and `ssr` are mutually exclusive; enable exactly one",
    ] {
        assert!(
            headless_lib.contains(needle),
            "ui-headless should preserve explicit feature-gated mutex marker `{needle}`.",
        );
    }

    let combined = format!(
        "{}\n{}\n{}\n{}\n{}",
        module_source, logic_source, view_source, styles_source, motion_source
    );
    for forbidden in ["web_sys::window", "window()", "document()"] {
        assert!(
            !combined.contains(forbidden),
            "Autocomplete non-wasm component paths should avoid browser-only token `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。",
        "autocomplete_platform_cross_target_compile_only_contract_is_explicit_and_non_wasm_safe",
        "platform_compile_only_contract_covers_default_ssr_wasm_and_non_wasm_source_guard",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete check2 should keep platform-cross-target marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_headless_web_ssr_mutex_contract_is_enforced_by_compile_error_and_platform_probe() {
    let platform_script = load_source("../../scripts/check-ui-components-platforms.sh");
    let headless_lib = load_source("../ui-headless/src/lib.rs");
    let check2_source = load_source("src/autocomplete/check2.md");

    for needle in [
        "[platform] compile guard: ui-headless web+ssr must fail",
        "if cargo check -p ui-headless --no-default-features --features web,ssr >\"$MUTEX_LOG\" 2>&1; then",
        "expected ui-headless web+ssr to fail, but command succeeded",
        "if ! rg -n \"mutually exclusive\" \"$MUTEX_LOG\" >/dev/null; then",
        "ui-headless web+ssr failed for an unexpected reason",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            platform_script.contains(needle),
            "platform mutex guard pipeline should include `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib.contains(needle),
            "ui-headless should keep explicit feature mutex guard `{needle}`.",
        );
    }

    for needle in [
        "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。",
        "headless_web_ssr_mutex_contract_is_guarded_by_compile_error_and_failure_probe",
        "autocomplete_headless_web_ssr_mutex_contract_is_enforced_by_compile_error_and_platform_probe",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete check2 should keep ui-headless mutex marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_ui_motion_non_wasm_stub_contract_is_explicit_and_predictable() {
    let platform_script = load_source("../../scripts/check-ui-components-platforms.sh");
    let ui_motion_lib = load_source("../ui-motion/src/lib.rs");
    let ui_motion_non_wasm_stub_test = load_source("../ui-motion/tests/non_wasm_stub.rs");
    let autocomplete_motion = load_source("src/autocomplete/motion.rs");
    let check2_source = load_source("src/autocomplete/check2.md");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion should keep non-wasm no-op/stub marker `{needle}`.",
        );
    }

    for needle in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
    ] {
        assert!(
            ui_motion_non_wasm_stub_test.contains(needle),
            "ui-motion non-wasm stub regression should include `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_popover_motion(",
        "if !is_open.get() {",
        "on_exit_complete.run(())",
    ] {
        assert!(
            autocomplete_motion.contains(needle),
            "Autocomplete motion should keep predictable non-wasm fallback marker `{needle}`.",
        );
    }

    for forbidden in ["panic!(", "unreachable!(", "todo!(", "unimplemented!("] {
        assert!(
            !autocomplete_motion.contains(forbidden),
            "Autocomplete motion fallback should avoid crash-only placeholder `{forbidden}`.",
        );
    }

    for needle in [
        "echo \"[platform] compile-only: ui-motion native path\"",
        "cargo check -p ui-motion",
        "echo \"[platform] compile-only: ui-motion wasm path\"",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "echo \"[platform] ui-motion non-wasm stub tests\"",
        "cargo test -p ui-motion --test non_wasm_stub",
        "echo \"[platform] source guard: autocomplete motion must keep explicit wasm/non-wasm branches\"",
        "echo \"[platform] source guard: autocomplete non-wasm motion fallback must remain predictable\"",
    ] {
        assert!(
            platform_script.contains(needle),
            "platform pipeline should include ui-motion non-wasm fallback marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。",
        "motion_non_wasm_stub_contract_is_predictable_and_toolchain_safe",
        "autocomplete_ui_motion_non_wasm_stub_contract_is_explicit_and_predictable",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete check2 should keep ui-motion non-wasm marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let platform_script = load_source("../../scripts/check-ui-components-platforms.sh");
    let ui_motion_spring = load_source("../ui-motion/src/spring.rs");
    let autocomplete_motion = load_source("src/autocomplete/motion.rs");
    let autocomplete_view = load_source("src/autocomplete/view.rs");
    let check2_source = load_source("src/autocomplete/check2.md");

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
        "on_rest();",
    ] {
        assert!(
            ui_motion_spring.contains(needle),
            "ui-motion spring reduced-motion contract should include `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if !is_open.get() {",
        "on_exit_complete.run(())",
    ] {
        assert!(
            autocomplete_motion.contains(needle),
            "Autocomplete motion should keep wasm/non-wasm semantic branch marker `{needle}`.",
        );
    }

    for needle in [
        "let presence = use_presence(is_open);",
        "<Show when=move || presence.is_present.get()>",
        "on_exit_complete=presence.finish_exit",
        "let open_now = is_open.get_untracked();",
    ] {
        assert!(
            autocomplete_view.contains(needle) || autocomplete_motion.contains(needle),
            "Autocomplete should keep SSR/hydration-safe presence sequencing marker `{needle}`.",
        );
    }

    for needle in [
        "echo \"[platform] autocomplete reduced-motion/ssr/wasm contract\"",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
    ] {
        assert!(
            platform_script.contains(needle),
            "platform pipeline should include autocomplete reduced-motion/ssr/wasm contract marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
        "reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
        "autocomplete_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete check2 should keep reduced-motion/SSR/wasm marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop() {
    let contract_hygiene_script =
        load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    let autocomplete_motion = load_source("src/autocomplete/motion.rs");
    let autocomplete_view = load_source("src/autocomplete/view.rs");
    let ui_motion_spring = load_source("../ui-motion/src/spring.rs");

    for needle in [
        "pub struct PopoverMotion",
        "pub spring: ui_motion::spring::SpringConfig,",
        "stiffness: 300.0,",
        "damping: 25.0,",
        "mass: 1.0,",
        "pub fn sanitize_popover_motion(motion: PopoverMotion) -> PopoverMotion",
        "pub fn sanitize_motion(motion: AutocompleteMotion) -> AutocompleteMotion",
        "pub fn attach_popover_motion(",
    ] {
        assert!(
            autocomplete_motion.contains(needle),
            "Autocomplete motion contract should include `{needle}`."
        );
    }

    for needle in [
        "let motion = crate::motion::sanitize_motion(motion);",
        "crate::motion::attach_popover_motion(",
        "popover_motion=motion.popover",
        "motion=motion.highlight",
    ] {
        assert!(
            autocomplete_view.contains(needle),
            "Autocomplete view should mount motion contract via `{needle}`."
        );
    }

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "on_rest();",
    ] {
        assert!(
            ui_motion_spring.contains(needle),
            "ui-motion spring should keep reduced-motion contract marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if !is_open.get() {",
        "on_exit_complete.run(())",
    ] {
        assert!(
            autocomplete_motion.contains(needle),
            "Autocomplete motion should keep non-wasm no-op branch marker `{needle}`."
        );
    }

    for forbidden in ["panic!(", "todo!(", "unimplemented!("] {
        assert!(
            !autocomplete_motion.contains(forbidden),
            "Autocomplete motion fallback should avoid crash-only token `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop";
    assert!(
        contract_hygiene_script.contains(script_needle),
        "contract-hygiene check script should enforce `{script_needle}`."
    );
}

#[test]
fn autocomplete_check2_marks_motion_contractualization_complete() {
    let check2_source = load_source("src/autocomplete/check2.md");

    assert!(
        check2_source.contains("- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。"),
        "autocomplete check2 should mark motion-contract gate complete."
    );

    for needle in [
        "motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop",
        "autocomplete_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop",
        "scripts/check-ui-components-contract-hygiene.sh",
        "components/autocomplete/src/motion.rs",
        "crates/ui-motion/src/spring.rs",
    ] {
        assert!(
            check2_source.contains(needle),
            "autocomplete check2 motion-contract section should reference `{needle}`."
        );
    }
}

#[test]
fn autocomplete_ui_components_fixed_entry_files_are_correctly_located_and_scoped() {
    let ui_components_lib = load_source("src/lib.rs");
    let ui_components_css = load_source("src/css.rs");
    let ui_components_root = load_source("src/root.rs");
    let active_highlight = load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let headless_lib = load_source("../ui-headless/src/lib.rs");
    let headless_controllable_state = load_source("../ui-headless/src/controllable_state.rs");
    let headless_presence = load_source("../ui-headless/src/presence.rs");

    for needle in [
        "mod css;",
        "#[cfg(feature = \"component-autocomplete\")]",
        "pub use ui_autocomplete as autocomplete;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "pub use autocomplete::{Autocomplete, AutocompleteMotion};",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui-components lib entry should keep `{needle}`."
        );
    }

    for forbidden in [
        "pub use leptos::web_sys",
        "pub type HtmlElement",
        "pub type NodeRef",
    ] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui-components lib entry should not expose platform detail `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-autocomplete\")]",
        "out.push_str(crate::autocomplete::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "ui-components css entry should keep `{needle}`."
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root.contains(needle),
            "UiRoot entry should centralize css/theme/i18n contract `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "pub spring: ui_motion::spring::SpringConfig,",
        "measure_layout: impl FnMut() -> Option<HighlightLayout> + 'static,",
    ] {
        assert!(
            active_highlight.contains(needle),
            "active-highlight visual primitive should keep `{needle}`."
        );
    }

    for forbidden in [
        "Autocomplete",
        "autocomplete_empty_message",
        "use_combo_box(",
        "aria-autocomplete",
    ] {
        assert!(
            !active_highlight.contains(forbidden),
            "active-highlight primitive should avoid component business semantic `{forbidden}`."
        );
    }

    for absent in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        let path = resolve_path(absent);
        assert!(
            !path.exists(),
            "ui-components fixed-entry contract forbids `{}`.",
            path.display()
        );
    }

    for needle in [
        "pub mod controllable_state;",
        "pub mod presence;",
        "pub mod a11y;",
        "pub use controllable_state::{",
        "pub use presence::{Presence, use_presence};",
        "use_controllable_open_state_traced,",
    ] {
        assert!(
            headless_lib.contains(needle),
            "ui-headless should host fixed primitive entry `{needle}`."
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_controllable_state<T>(",
    ] {
        assert!(
            headless_controllable_state.contains(needle),
            "headless controllable-state primitive should keep `{needle}`."
        );
    }

    assert!(
        headless_presence.contains("pub fn use_presence(is_open: Signal<bool>) -> Presence"),
        "headless presence primitive should provide `use_presence` entry."
    );
}

#[test]
fn autocomplete_ui_components_fixed_entry_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    let needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_ui_components_fixed_entry_files_are_correctly_located_and_scoped";
    assert!(
        script_source.contains(needle),
        "contract-hygiene script should cover fixed-entry file contract `{needle}`."
    );
}

#[test]
fn autocomplete_check2_marks_ui_components_fixed_entry_files_contract_complete() {
    let check2_source = load_source("src/autocomplete/check2.md");

    assert!(
        check2_source.contains("- [x] `ui-components` 固定入口文件落点正确。"),
        "autocomplete check2 should mark ui-components fixed-entry file item complete."
    );

    for needle in [
        "ui_components_fixed_entry_files_are_correctly_located_and_scoped",
        "autocomplete_ui_components_fixed_entry_files_are_correctly_located_and_scoped",
        "autocomplete_ui_components_fixed_entry_check_script_covers_contract",
        "crates/ui-components/src/lib.rs",
        "crates/ui-components/src/css.rs",
        "crates/ui-components/src/root.rs",
        "crates/ui-visual-primitive/src/active_highlight.rs",
        "crates/ui-headless/src/controllable_state.rs",
        "crates/ui-headless/src/presence.rs",
        "crates/ui-headless/src/a11y.rs",
        "crates/ui-components/src/overlay_open.rs",
        "crates/ui-components/src/presence.rs",
        "crates/ui-components/src/a11y.rs",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "autocomplete check2 fixed-entry evidence should reference `{needle}`."
        );
    }
}

#[test]
fn autocomplete_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("src/autocomplete/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2_source = load_source("src/autocomplete/check2.md");

    for needle in [
        "struct AutocompleteOptionViewCtx {",
        "fn render_autocomplete_option(ctx: AutocompleteOptionViewCtx) -> impl IntoView",
        "fn render_autocomplete_description(",
        "fn render_autocomplete_error(",
        "render_autocomplete_option(AutocompleteOptionViewCtx {",
        "let description_view =",
        "render_autocomplete_description(description, text_field.description.id.clone())",
        "let error_view = render_autocomplete_error(error, text_field.error.id.clone(), invalid);",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should keep macro complexity split marker `{needle}`."
        );
    }

    for forbidden in [
        "let description_id = text_field.description.id.clone();\n                view! {",
        "let error_id = text_field.error.id.clone();\n                let error_id = StoredValue::new(error_id);",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Autocomplete view should not regress to nested optional-section macro token `{forbidden}`."
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 5,
        "Autocomplete view macro complexity regression: expected <= 5 `view!` blocks, found {view_macro_count}.",
    );

    let script_needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "view_macro_complexity_is_split_into_semantic_subblocks",
        "autocomplete_view_macro_complexity_is_split_into_semantic_subrenders",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete check2 should keep view-macro complexity marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("src/autocomplete/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2_source = load_source("src/autocomplete/check2.md");

    for needle in [
        "fn render_autocomplete_option(ctx: AutocompleteOptionViewCtx) -> impl IntoView",
        "fn render_autocomplete_description(",
        "fn render_autocomplete_error(",
        "render_autocomplete_option(AutocompleteOptionViewCtx {",
        "let description_view =",
        "let error_view = render_autocomplete_error(error, text_field.error.id.clone(), invalid);",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should keep function-first split marker `{needle}`."
        );
    }

    let component_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_count, 2,
        "Autocomplete should keep exactly two component boundaries (root + panel); found {component_count}.",
    );

    for needle in [
        "#[component]\nfn AutocompletePanel(",
        "#[component]\npub fn Autocomplete(",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should keep required component boundary `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "view_functional_split_prefers_plain_functions_over_local_components",
        "autocomplete_view_functional_split_prefers_plain_functions_over_local_components",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete check2 should keep function-first split marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_static_fragments_are_constantized_or_absent_for_simple_combobox_layout() {
    let view_source = load_source("src/autocomplete/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2_source = load_source("src/autocomplete/check2.md");

    for needle in [
        "class=\"ui-active-highlight\"",
        "data-slot=\"autocomplete-highlight\"",
        "class=\"ui-autocomplete__empty\" data-slot=\"autocomplete-empty\"",
        "empty_message.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should keep stable static-fragment anchor `{needle}`."
        );
    }

    for forbidden in [
        "<svg",
        "</svg>",
        "<footer",
        "</footer>",
        "include_str!(",
        "markdown_to_html",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Autocomplete should avoid heavy static fragment token `{forbidden}` in view layer.",
        );
    }

    let script_needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_static_fragments_are_constantized_or_absent_for_simple_combobox_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "static_fragments_are_constantized_or_absent_for_simple_combobox_layout",
        "autocomplete_static_fragments_are_constantized_or_absent_for_simple_combobox_layout",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete check2 should keep static-fragment governance marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/autocomplete/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_source("src/autocomplete/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "\"autocomplete\" => UiPerfBudget {",
        "max_mount_ms: 38.0,",
        "max_update_ms: Some(13.0),",
        "max_heap_kb: Some(768.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep autocomplete performance budget token `{needle}`."
        );
    }

    for needle in [
        "component_doc!(",
        "\"Autocomplete\"",
        "\"autocomplete\"",
        "collections::autocomplete",
    ] {
        assert!(
            pages_source.contains(needle),
            "Autocomplete docs page should remain in coverage traversal via `{needle}`.",
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
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
        "performance_governance_budget_is_defined_traceable_and_blocking",
        "autocomplete_performance_governance_budget_is_defined_and_blocking",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete checklist should keep performance governance marker `{needle}`."
        );
    }

    for needle in [
        "data-state=move || {",
        "data-label-source=state.label_source_attr",
        "data-placeholder-source=state.placeholder_source_attr",
        "data-id-source=state.id_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=state.motion_source_attr",
        "data-selected-source=selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should expose attribution marker `{needle}` for perf triage.",
        );
    }

    let script_needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_performance_governance_budget_is_defined_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`.",
    );
}

#[test]
fn autocomplete_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let view_source = load_source("src/autocomplete/view.rs");
    let check2_source = load_source("src/autocomplete/check2.md");
    let perf_script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let semantics_source = load_source("tests/autocomplete_semantics.rs");

    for needle in [
        "role=aria.input.role",
        "aria-controls=move || aria.input.aria_controls.get()",
        "aria-expanded=move || aria.input.aria_expanded.get()",
        "aria-selected=move || option_attrs.get().aria_selected",
        "aria-disabled=move || option_attrs.get().aria_disabled",
        "data-state=move ||",
        "data-selected-source=selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
        "data-focused=move || focus_ring.is_focused.get().then_some(\"true\")",
        "data-focus-visible=move || focus_ring.is_focus_visible.get().then_some(\"true\")",
        "on:focus=on_focus",
        "on:blur=on_blur",
        "let key_result = aria.handlers.on_input_key_down.run(key);",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should keep semantics/focus marker `{needle}`."
        );
    }

    let perf_gate_needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_performance_governance_budget_is_defined_and_blocking";
    assert!(
        perf_script_source.contains(perf_gate_needle),
        "performance gate script should include `{perf_gate_needle}`."
    );

    assert!(
        semantics_source
            .contains("fn autocomplete_performance_governance_budget_is_defined_and_blocking()"),
        "autocomplete semantics suite should keep a dedicated blocking performance governance test."
    );

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "render_count follow-up tracking should include `{needle}`."
        );
    }

    for needle in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "semantic_contract_matrix_covers_state_a11y_input_paths_and_platform_guards",
        "performance_governance_budget_is_defined_traceable_and_blocking",
        "semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "autocomplete_semantic_contract_matrix_covers_core_branches_and_platform_paths",
        "autocomplete_performance_governance_budget_is_defined_and_blocking",
        "autocomplete_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "`render_count` 状态：当前测试框架对该组件仍采用可重复 mount/perf trace 等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete checklist should keep semantics+performance marker `{needle}`."
        );
    }
}

#[test]
fn autocomplete_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let view_source = load_source("src/autocomplete/view.rs");
    let local_semantics_source = load_source("../../components/autocomplete/test/semantics.rs");
    let semantics_source = load_source("tests/autocomplete_semantics.rs");
    let perf_script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let check2_source = load_source("src/autocomplete/check2.md");

    for needle in [
        "role=aria.input.role",
        "aria-controls=move || aria.input.aria_controls.get()",
        "data-state=move ||",
        "data-label-source=state.label_source_attr",
        "data-description-source=state.description_source_attr",
        "data-error-source=state.error_source_attr",
        "data-placeholder-source=state.placeholder_source_attr",
        "data-id-source=state.id_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=state.motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete semantic-priority contract should keep `{needle}`."
        );
    }

    for needle in [
        "fn semantic_contract_matrix_covers_state_a11y_input_paths_and_platform_guards()",
        "fn performance_governance_budget_is_defined_traceable_and_blocking()",
        "fn semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            local_semantics_source.contains(needle),
            "local semantics suite should keep contract-focused assertion `{needle}`."
        );
    }

    for needle in [
        "fn autocomplete_semantics_suite_is_contract_first_not_snapshot_only()",
        "fn autocomplete_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
        "fn autocomplete_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()",
    ] {
        assert!(
            semantics_source.contains(needle),
            "workspace semantics suite should keep contract-focused assertion `{needle}`."
        );
    }

    let local_has_snapshot_assertion = local_semantics_source.lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with("assert_snapshot!(") || trimmed.starts_with("insta::assert")
    });
    assert!(
        !local_has_snapshot_assertion,
        "semantic-priority contract should avoid snapshot-only assertion calls in local semantics suite."
    );
    let workspace_has_snapshot_assertion = semantics_source.lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with("assert_snapshot!(") || trimmed.starts_with("insta::assert")
    });
    assert!(
        !workspace_has_snapshot_assertion,
        "semantic-priority contract should avoid snapshot-only assertion calls in workspace semantics suite."
    );

    let script_needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script_source.contains(script_needle),
        "performance gate script should include semantic-priority command `{script_needle}`."
    );

    for needle in [
        "语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "components/autocomplete/test/semantics.rs",
        "semantic_contract_matrix_covers_state_a11y_input_paths_and_platform_guards",
        "semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "autocomplete_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "scripts/check-ui-components-performance.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete check2 should keep semantic-test-priority marker `{needle}`."
        );
    }
}

#[test]
fn autocomplete_focus_stack_gc_contract_is_na_for_non_modal_combobox_overlay() {
    let view_source = load_source("src/autocomplete/view.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let motion_source = load_source("src/autocomplete/motion.rs");
    let focus_trap_source = load_source("../ui-headless/src/focus_trap.rs");

    for needle in [
        "on:focus=on_focus",
        "on:blur=on_blur",
        "aria.handlers.open.run(())",
        "aria.handlers.close.run(())",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete should keep input-centric focus lifecycle marker `{needle}`."
        );
    }

    for forbidden in [
        "use_focus_trap(",
        "FocusTrap",
        "RestorePolicy",
        "FallbackTo(",
        "Selector(",
        "document.body",
        "body.focus(",
        "previous_focus",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Autocomplete component layer should not own overlay focus-restore internals `{forbidden}`.",
        );
    }

    for needle in [
        "pub enum RestorePolicy",
        "FallbackTo(String)",
        "Selector(String)",
        "FOCUS_MANAGER_STACK",
        "fn restore_focus_chain(",
    ] {
        assert!(
            focus_trap_source.contains(needle),
            "ui-headless focus trap should provide shared focus manager primitive `{needle}`."
        );
    }
}

#[test]
fn autocomplete_hydration_discontinuity_contract_uses_seeded_id_provider_and_has_no_entropy_path() {
    let module_source = load_source("src/autocomplete/mod.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let view_source = load_source("src/autocomplete/view.rs");
    let root_source = load_source("src/root.rs");
    let id_provider_source = load_source("../ui-headless/src/id_provider.rs");
    let check2_source = load_source("src/autocomplete/check2.md");

    for needle in [
        "use_text_field, use_ui_i18n, use_ui_id_provider,",
        "let generated_id_base = use_ui_id_provider()",
        "next_prefixed_id(ui_state_primitives::autocomplete::DEFAULT_ID_BASE)",
        "let id_base = logic::resolve_id_base(id_base, generated_id_base);",
        "has_custom_id_base,",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete view should keep hydration-stable id bootstrap marker `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_id_base(id_base: String, generated_id_base: String) -> String",
        "normalize_optional_text(Some(id_base)).unwrap_or(generated_id_base)",
        "pub has_custom_id_base: bool,",
        "has_custom_id_base: input.has_custom_id_base,",
    ] {
        assert!(
            logic_source.contains(needle),
            "Autocomplete logic should keep deterministic id normalization marker `{needle}`."
        );
    }

    let combined = format!("{}\n{}\n{}", module_source, logic_source, view_source);
    for forbidden in [
        "now()",
        "Instant::now",
        "SystemTime::now",
        "Date::now",
        "Uuid",
        "uuid::",
        "new_v4",
        "rand::",
        "thread_rng",
        "random::<",
        "getrandom",
        "nanoid",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Autocomplete should remain deterministic across SSR/hydration and avoid entropy token `{forbidden}`.",
        );
    }

    assert!(
        root_source.contains("provide_ui_id_provider(id_seed);"),
        "UiRoot should wire deterministic id seed via provide_ui_id_provider(id_seed).",
    );

    for needle in [
        "pub struct UiIdProvider",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider>",
        "pub fn next_prefixed_id(self, prefix: &str) -> String",
    ] {
        assert!(
            id_provider_source.contains(needle),
            "ui-headless id provider contract should expose `{needle}`.",
        );
    }

    for needle in [
        "- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。",
        "hydrat",
        "autocomplete_hydration_discontinuity_contract_uses_seeded_id_provider_and_has_no_entropy_path",
        "hydration_discontinuity_contract_uses_seeded_id_provider_and_avoids_entropy_sources",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete check2 should keep hydration-discontinuity marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_escape_hatch_foreign_zone_contract_is_na_without_third_party_instances() {
    let module_source = load_source("src/autocomplete/mod.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let view_source = load_source("src/autocomplete/view.rs");
    let motion_source = load_source("src/autocomplete/motion.rs");
    let readme_source = load_source("src/autocomplete/README.md");
    let combined = format!(
        "{}\n{}\n{}\n{}\n{}",
        module_source, logic_source, view_source, motion_source, readme_source
    );

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "OpenLayers",
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Autocomplete should not introduce third-party imperative integration token `{forbidden}`.",
        );
    }

    for forbidden in [
        "pub struct EChart",
        "pub struct Mapbox",
        "pub enum ForeignZone",
        "pub type ForeignHandle",
        "pub third_party_instance:",
        "pub map_instance:",
        "pub chart_instance:",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Autocomplete public/component API should not expose third-party instance handle `{forbidden}`.",
        );
    }
}

#[test]
fn autocomplete_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let module_source = load_source("src/autocomplete/mod.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let styles_source = load_source("src/autocomplete/styles.rs");
    let view_source = load_source("src/autocomplete/view.rs");
    let motion_source = load_source("src/autocomplete/motion.rs");
    let readme_source = load_source("src/autocomplete/README.md");
    let check2_source = load_source("src/autocomplete/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-inner-html.sh");
    let combined =
        format!("{module_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "markdown_to_html(",
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Autocomplete component layer should not expose raw html injection token `{forbidden}`.",
        );
    }

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !readme_source.contains(forbidden),
            "Autocomplete README examples should not include raw html injection token `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(script_needle),
        "inner-html gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "N/A：`Autocomplete` 当前无 `inner_html` 使用点",
        "inner_html_usage_is_forbidden_in_component_and_docs_examples",
        "autocomplete_inner_html_usage_is_forbidden_in_component_and_docs_examples",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete checklist should keep inner_html safety marker `{needle}`."
        );
    }
}

#[test]
fn autocomplete_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let module_source = load_source("src/autocomplete/mod.rs");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let view_source = load_source("src/autocomplete/view.rs");
    let motion_source = load_source("src/autocomplete/motion.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_autocomplete_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");
    let check2_source = load_source("src/autocomplete/check2.md");

    assert!(
        cargo_source.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "shared wasm debug capability should remain feature-gated via `button-wasm-debug`."
    );

    let all_components_start = cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = cargo_source[all_components_start..]
        .find("\n\ncomponent-accordion")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before component feature declarations");
    let all_components_block = &cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "wasm debug feature must not be pulled into all-components production path."
    );
    assert!(
        !cargo_source.contains("autocomplete-wasm-debug"),
        "Autocomplete should not define component-local wasm debug feature when shared trace overlay is sufficient."
    );

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(needle),
            "docs app should keep wasm debug visual entry marker `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "let ts_ms = event.ts_ms;",
        "UiTraceEventKind::OpenChange",
        "UiTraceEventKind::Inspect",
        "UiTraceEventKind::Note",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "global trace timeline should keep marker `{needle}`."
        );
    }

    for needle in [
        "let open_state = overlay_open::use_controllable_open_state_traced(",
        "\"autocomplete\",",
        "let key_result = aria.handlers.on_input_key_down.run(key);",
        "data-state=move || {",
        "data-selected-source=selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Autocomplete should keep traceable state/interaction marker `{needle}`."
        );
    }

    for needle in [
        "docs-app autocomplete key flow is repeatable with semantic contract breakpoints",
        "await controlledInput.fill(\"Shen\")",
        "await option.click()",
        "await page.reload()",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Autocomplete e2e should keep replayable key flow marker `{needle}`."
        );
    }

    for forbidden in [
        "autocomplete-wasm-debug",
        "wasm_debug",
        "data-debug-source=",
        "request_replay.run(",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Autocomplete should not ship component-local wasm debug runtime token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated";
    assert!(
        script_source.contains(script_needle),
        "wasm-debug gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
        "autocomplete_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete checklist should keep wasm-debug marker `{needle}`."
        );
    }
}

#[test]
fn autocomplete_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Show settings\"",
        "\"Show code\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn autocomplete() -> AnyView {",
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "test_css_source=workbench_test_css",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/autocomplete/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"autocomplete-workbench\"",
        "data-slot=\"autocomplete-workbench-canvas\"",
        "ui_components::autocomplete::styles::CSS",
    ] {
        assert!(
            docs_source.contains(needle),
            "Autocomplete docs should keep DX CSS hot-reload marker `{needle}`."
        );
    }
}

#[test]
fn autocomplete_dx_workbench_supports_optional_state_persistence_and_isolated_canvas() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");
    let check2_source = load_source("src/autocomplete/check2.md");

    for needle in [
        "AUTOCOMPLETE_WORKBENCH_STORAGE_KEY",
        "fn load_autocomplete_workbench_selected() -> Option<usize>",
        "fn save_autocomplete_workbench_selected(selected_index: usize)",
        "fn clear_autocomplete_workbench_selected()",
        "let persisted_autocomplete_workbench_selected = load_autocomplete_workbench_selected();",
        "let (workbench_persist_state, set_workbench_persist_state) =",
        "save_autocomplete_workbench_selected(selected_index);",
        "clear_autocomplete_workbench_selected();",
        "\" Persist selected index (optional)\"",
        "\" · persist selected: \"",
        "data-slot=\"autocomplete-workbench-controls\"",
        "data-slot=\"autocomplete-workbench\"",
        "data-slot=\"autocomplete-workbench-canvas\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Autocomplete workbench should keep optional-persistence marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            docs_source.contains(needle),
            "Autocomplete workbench persistence should keep platform guard `{needle}`."
        );
    }

    for needle in [
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            script_source.contains(needle),
            "dx check script should enforce `{needle}`."
        );
    }

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "autocomplete_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "autocomplete_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            check2_source.contains(needle),
            "Autocomplete checklist should keep DX marker `{needle}`."
        );
    }
}

#[test]
fn autocomplete_engineering_contract_uses_serde_protocol_and_structured_schema_defaults() {
    let component_cargo_source = load_source("../../components/autocomplete/Cargo.toml");
    let protocol_source = load_source("../../components/autocomplete/src/protocol.rs");
    let checklist_source = load_source("../../components/autocomplete/check2.md");

    assert!(
        component_cargo_source.contains("serde = { version = \"1.0\", features = [\"derive\"] }"),
        "Autocomplete crate should keep serde derive dependency for structured protocol schema."
    );
    assert!(
        !component_cargo_source.contains("serde_json"),
        "Autocomplete should avoid serde_json fan-out when protocol only requires typed serde schema defaults."
    );

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "pub enum AutocompleteComponentSchemaVersion",
        "#[serde(rename_all = \"snake_case\")]",
        "pub struct AutocompleteComponentSpec",
        "#[serde(default)]",
        "pub schema_version: AutocompleteComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "Autocomplete protocol should keep structured serde schema marker `{needle}`."
        );
    }

    for forbidden in [
        "serde_json::",
        "SchemaError",
        "from_json(",
        "to_json_result(",
    ] {
        assert!(
            !protocol_source.contains(forbidden),
            "Autocomplete protocol should avoid unsupported migration/error token `{forbidden}` in current v1 scope."
        );
    }

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "autocomplete_engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
        "autocomplete_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "autocomplete_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            checklist_source.contains(needle),
            "Autocomplete checklist should keep engineering marker `{needle}`."
        );
    }
}

#[test]
fn autocomplete_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events()
 {
    let cargo_source = load_source("Cargo.toml");
    let combined = [
        load_source("src/autocomplete/mod.rs"),
        load_source("src/autocomplete/logic.rs"),
        load_source("src/autocomplete/view.rs"),
        load_source("src/autocomplete/motion.rs"),
    ]
    .join("\n");

    assert!(
        cargo_source.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "engineering baseline should keep canonical tracing feature gate."
    );
    assert!(
        !cargo_source.contains("autocomplete-wasm-debug")
            && !cargo_source.contains("component-autocomplete-wasm-debug"),
        "Autocomplete should not define component-local tracing debug feature in cargo feature graph."
    );
    assert!(
        combined.contains("overlay_open::use_controllable_open_state_traced(")
            && combined.contains("\"autocomplete\""),
        "Autocomplete should reuse shared traced controllable-state hook."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::autocomplete::",
        "const AUTOCOMPLETE_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Autocomplete should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn autocomplete_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let combined = [
        load_source("src/autocomplete/mod.rs"),
        load_source("src/autocomplete/logic.rs"),
        load_source("src/autocomplete/view.rs"),
        load_source("src/autocomplete/styles.rs"),
        load_source("src/autocomplete/motion.rs"),
        load_source("src/autocomplete/protocol.rs"),
        load_source("../../components/autocomplete/src/README.md"),
    ]
    .join("\n");

    for forbidden in [
        "tokio",
        "tokio::",
        "async_std",
        "async_std::",
        "async-std",
        "runtime::Handle",
        "smol::",
        "spawn_blocking(",
        "futures::executor",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Autocomplete engineering contract should not leak runtime marker `{forbidden}`."
        );
    }

    assert!(
        !load_source("src/autocomplete/mod.rs").contains("web_sys"),
        "Autocomplete public module boundary should not leak web_sys types."
    );
}

#[test]
fn autocomplete_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn autocomplete_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
 {
    let check2_source = load_source("../../components/autocomplete/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");
    let protocol_source = load_source("src/autocomplete/protocol.rs");
    let component_manifest = load_source("../../components/autocomplete/src/Component.toml");
    let rbi_source = load_source("../../components/autocomplete/src/autocomplete.rbi");
    let combined = [
        load_source("src/autocomplete/mod.rs"),
        load_source("src/autocomplete/logic.rs"),
        load_source("src/autocomplete/view.rs"),
        load_source("src/autocomplete/styles.rs"),
        load_source("src/autocomplete/motion.rs"),
        protocol_source.clone(),
        component_manifest.clone(),
        rbi_source.clone(),
    ]
    .join("\n");

    for needle in [
        "pub enum AutocompleteComponentSchemaVersion {",
        "V1,",
        "pub struct AutocompleteComponentSpec {",
        "pub schema_version: AutocompleteComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "Autocomplete protocol should keep v1 schema marker `{needle}` in non-breaking scope.",
        );
    }

    for needle in [
        "schema_version = \"1\"",
        "schema = \"ui.autocomplete.agent-contract.v1\"",
        "values = [\"v1\"]",
    ] {
        assert!(
            component_manifest.contains(needle),
            "Autocomplete Component.toml should keep v1 registration marker `{needle}` in non-breaking scope.",
        );
    }

    for needle in ["pub enum AutocompleteAgentSchemaVersion {", "V1,"] {
        assert!(
            rbi_source.contains(needle),
            "Autocomplete RBI should keep v1 marker `{needle}` in non-breaking scope.",
        );
    }

    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "SchemaRegistry",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "agent-contract.v2",
    ] {
        assert!(
            !combined.contains(forbidden),
            "without major breaking upgrade, autocomplete should not introduce migration token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(script_needle),
        "engineering check script should enforce `{script_needle}`.",
    );

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `Autocomplete` 改动未引入跨大版本 API 破坏升级",
        "AutocompleteComponentSchemaVersion::V1",
        "schema_version = \"1\"",
        "ui.autocomplete.agent-contract.v1",
        "AutocompleteAgentSchemaVersion::V1",
        "version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
        "autocomplete_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
        "scripts/check-ui-components-engineering.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "components/autocomplete/check2.md should keep version-deprecation marker `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("src/autocomplete/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "已统一使用语义选择器与稳定等待",
        "并移除文本定位依赖（不再使用 `hasText` 过滤）",
        "关键路径显式断言 ready/settled 断点",
    ] {
        assert!(
            check2_source.contains(required),
            "components/autocomplete/check2.md should keep e2e selector stability marker `{required}`.",
        );
    }
}

#[test]
fn autocomplete_e2e_selectors_are_semantic_and_wasm_wait_strategy_is_stable() {
    let e2e_source = load_source("../../e2e/tests/docs_app_autocomplete_contract.spec.mjs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "body:not(:has(#boot))",
        "#docs-autocomplete-controlled-input",
        "data-slot=\"autocomplete-controlled-open\"",
        "data-slot=\"autocomplete-controlled-selected\"",
        "[data-slot=\"autocomplete-option\"]",
        "toHaveAttribute(\"data-controlled\", \"true\")",
    ] {
        assert!(
            e2e_source.contains(needle) || docs_source.contains(needle),
            "Autocomplete e2e/docs contract should include `{needle}`.",
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "Autocomplete e2e should avoid brittle wait primitive `{forbidden}`.",
        );
    }
}

#[test]
fn autocomplete_e2e_contract_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_autocomplete_contract.spec.mjs");

    for needle in [
        "toHaveText(\"open: false\")",
        "toHaveText(\"open: true\")",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"data-closed\", \"true\")",
        "toHaveText(\"selected: 3\")",
        "await page.reload();",
        "toHaveText(\"selected: 2\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Autocomplete e2e ready/settled contract should include `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_e2e_key_flow_is_repeatable_with_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_autocomplete_contract.spec.mjs");

    for needle in [
        "docs-app autocomplete key flow is repeatable with semantic contract breakpoints",
        "await controlledInput.fill(\"Shen\")",
        "await option.click()",
        "await expect(selectedMarker).toHaveText(\"selected: 3\")",
        "await page.reload()",
        "selected: 2",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Autocomplete e2e repeatable flow should include `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_e2e_check_script_covers_selector_and_settled_wait_contract() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-autocomplete.sh");

    for needle in [
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_e2e_selectors_are_semantic_and_wasm_wait_strategy_is_stable",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_e2e_contract_covers_ready_and_settled_semantic_breakpoints",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_e2e_key_flow_is_repeatable_with_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(needle),
            "autocomplete e2e script should include `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = load_source("src/autocomplete/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "已纳入两条可重复关键流程并可回放",
        "失败可定位到具体语义断点而非笼统页面差异",
        "overlay/focus/keyboard 已进入回归集合",
    ] {
        assert!(
            check2_source.contains(required),
            "components/autocomplete/check2.md should keep repeatable-flow governance marker `{required}`.",
        );
    }
}

#[test]
fn autocomplete_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_autocomplete_contract.spec.mjs");

    for needle in [
        "docs-app autocomplete high-risk overlay/focus/keyboard path is replayable with semantic breakpoints",
        "await controlledInput.focus();",
        "await expect(controlledInput).toBeFocused();",
        "await controlledInput.press(\"Escape\");",
        "await controlledInput.press(\"ArrowDown\");",
        "await controlledInput.press(\"Enter\");",
        "const activeDescendant = await controlledInput.getAttribute(\"aria-activedescendant\");",
        "expect(activeDescendant).toBeTruthy();",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"data-closed\", \"true\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Autocomplete high-risk e2e flow should include `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_e2e_check_script_covers_repeatable_flow_and_high_risk_contract() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-autocomplete.sh");

    for needle in [
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_e2e_key_flow_is_repeatable_with_semantic_breakpoints",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(needle),
            "autocomplete e2e script should include repeatable-flow/high-risk contract `{needle}`.",
        );
    }
}

#[test]
fn autocomplete_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = load_source("src/autocomplete/check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "autocomplete check2 heroui-benchmark docs-sync section should include `{required}`.",
        );
    }
}

#[test]
fn autocomplete_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for marker in [
        "### Autocomplete 同步记录（2026-02-18）",
        "参数模型同步：`Autocomplete` 保持 `is_open/open + on_open_change + default_open`、`is_disabled/disabled`、`is_required/required`、`is_invalid/invalid` 轴",
        "`apps/docs-app/src/pages/components/pages.rs` 继续通过 `component_doc!(\"Autocomplete\", \"autocomplete\", \"Collections\", collections::autocomplete)` 暴露入口",
        "`apps/docs-app/src/pages/components/pages/collections.rs::autocomplete()`",
        "Source-first / Copy-Paste Ready",
        "参数语义变更必须先同步本策略文档与 docs 页面",
    ] {
        assert!(
            strategy_source.contains(marker),
            "heroui strategy doc should include autocomplete sync marker `{marker}`.",
        );
    }

    for marker in [
        "component_doc!(",
        "\"Autocomplete\",",
        "\"autocomplete\",",
        "\"Collections\",",
        "collections::autocomplete",
    ] {
        assert!(
            pages_source.contains(marker),
            "component docs index should expose autocomplete entry marker `{marker}`.",
        );
    }

    for marker in [
        "pub(super) fn autocomplete() -> AnyView {",
        "title=\"Autocomplete\"",
        "slug=\"autocomplete\"",
    ] {
        assert!(
            docs_page_source.contains(marker),
            "autocomplete docs-app page should stay indexable via marker `{marker}`.",
        );
    }
}

#[test]
fn autocomplete_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for marker in [
        "echo \"[dx] contract: autocomplete heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(marker),
            "DX check script should include heroui-benchmark docs-sync marker `{marker}`.",
        );
    }
}

#[test]
fn autocomplete_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("src/autocomplete/check2.md");

    assert!(
        check2_source.contains("- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。"),
        "autocomplete check2 should mark heroui-benchmark docs-sync item complete.",
    );

    for marker in [
        "docs/spec/heroui-parameter-design-strategy.md",
        "apps/docs-app/src/pages/components/pages.rs",
        "apps/docs-app/src/pages/components/pages/collections.rs",
        "component_doc!(\"Autocomplete\", \"autocomplete\", \"Collections\", collections::autocomplete)",
        "autocomplete_check2_documents_heroui_benchmark_docs_sync_rules",
        "autocomplete_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "autocomplete_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "autocomplete check2 heroui-benchmark docs-sync section should include `{marker}`.",
        );
    }
}

#[test]
fn autocomplete_check2_documents_agent_contract_schema_governance_rules() {
    let check2_source = load_source("src/autocomplete/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "autocomplete_agent_contract_is_schema_typed_and_machine_readable",
        "autocomplete_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "autocomplete_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "Autocomplete check2 should keep Agent Contract governance marker `{required}`."
        );
    }

    for script_needle in [
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(script_needle),
            "contract-hygiene script should include `{script_needle}`."
        );
    }
}

#[test]
fn autocomplete_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("src/autocomplete/logic.rs");
    let view_source = load_source("src/autocomplete/view.rs");
    let component_manifest = load_source("src/autocomplete/Component.toml");
    let component_rbi = load_source("src/autocomplete/autocomplete.rbi");

    for typed_source in [
        "pub const AUTOCOMPLETE_AGENT_SCHEMA: &str = \"ui.autocomplete.agent-contract\";",
        "pub enum AutocompleteAgentSchemaVersion",
        "pub enum AutocompleteAgentIntent",
        "pub enum AutocompleteAgentAction",
        "pub enum AutocompleteAgentState",
        "pub enum AutocompleteAgentSource",
        "pub struct AutocompleteAgentContract",
        "pub struct AutocompleteAgentContractInput",
        "pub fn resolve_agent_contract(input: AutocompleteAgentContractInput) -> AutocompleteAgentContract",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "Autocomplete Agent Contract should stay type-derived via `{typed_source}`."
        );
    }

    for marker in [
        "let agent_contract = Signal::derive(move || {",
        "logic::resolve_agent_contract(logic::AutocompleteAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-ui-state-source=move || agent_contract.get().state_source",
        "data-ui-motion-source=move || agent_contract.get().motion_source",
        "data-ui-selected-source=move || agent_contract.get().selected_source",
        "data-ui-selected-change-source=move || agent_contract.get().selected_change_source",
        "data-ui-open-value-source=move || agent_contract.get().open_value_source",
        "data-ui-config-policy=move || agent_contract.get().config_policy",
    ] {
        assert!(
            view_source.contains(marker),
            "Autocomplete view should mount Agent Contract marker `{marker}`."
        );
    }

    for required in [
        "name = \"agent_contract_schema_markers\"",
        "name = \"agent_contract_whitelist_boundary\"",
        "[[agent_contract]]",
        "schema = \"ui.autocomplete.agent-contract.v1\"",
        "intent = \"autocomplete.suggest-and-select\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "AUTOCOMPLETE_AGENT_SCHEMA",
        "AutocompleteAgentContract",
        "resolve_agent_contract(",
    ] {
        assert!(
            component_manifest.contains(required) || component_rbi.contains(required),
            "Autocomplete context-compression assets should keep Agent Contract marker `{required}`."
        );
    }
}

#[test]
fn autocomplete_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let logic_source = load_source("src/autocomplete/logic.rs");
    let view_source = load_source("src/autocomplete/view.rs");

    for marker in [
        "Self::V1 => \"v1\"",
        "Self::SuggestAndSelect => \"autocomplete.suggest-and-select\"",
        "Self::Idle => \"idle\"",
        "Self::Query => \"query\"",
        "Self::CommitSelection => \"commit-selection\"",
        "Self::Open => \"open\"",
        "Self::Disabled => \"disabled\"",
        "Self::StatePrimitives => \"state-primitives\"",
    ] {
        assert!(
            logic_source.contains(marker),
            "Autocomplete Agent Contract should keep closed typed mapping marker `{marker}`."
        );
    }

    for forbidden in [
        "data-ui-schema=\"",
        "data-ui-intent=\"",
        "data-ui-action=\"",
        "data-ui-state=\"",
        "data-ui-source=\"",
        "format!(\"data-ui-",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Autocomplete view should avoid free-form Agent Contract marker `{forbidden}`."
        );
    }
}

#[test]
fn autocomplete_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("src/autocomplete/view.rs");
    let component_manifest = load_source("src/autocomplete/Component.toml");

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "\"typed_state_from_ui_state_primitives::autocomplete::resolve_state\"",
        "\"typed_semantics_from_logic::resolve_agent_contract\"",
        "\"typed_render_mount_from_view\"",
        "\"<script\"",
        "\"javascript:\"",
        "\"eval(\"",
        "name = \"agent_contract_whitelist_boundary\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "Autocomplete Component.toml should keep whitelist boundary marker `{required}`."
        );
    }

    for forbidden in [
        "inner_html=",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
        "eval(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Autocomplete render path should remain whitelist-safe; found `{forbidden}`."
        );
    }
}

#[test]
fn autocomplete_streaming_term_is_limited_to_llm_output_render_modes() {
    let check2_source = load_source("src/autocomplete/check2.md");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let view_source = load_source("src/autocomplete/view.rs");
    let component_manifest = load_source("src/autocomplete/Component.toml");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "streaming_term_is_limited_to_llm_output_render_modes",
        "autocomplete_streaming_term_is_limited_to_llm_output_render_modes",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "check2 should keep streaming-term governance marker `{marker}`."
        );
    }

    for marker in [
        "pub enum AutocompleteAgentStreamMode",
        "Self::Streaming => \"streaming\"",
        "Self::Snapshot => \"snapshot\"",
        "stream_support: AutocompleteAgentStreamSupport::Unsupported,",
        "stream_fallback: AutocompleteAgentStreamFallback::Snapshot,",
        "stream_mode: AutocompleteAgentStreamMode::Snapshot,",
    ] {
        assert!(
            logic_source.contains(marker),
            "logic.rs should keep LLM render mode marker `{marker}`."
        );
    }

    for marker in [
        "[streaming_policy]",
        "term_scope = \"llm-output-rendering\"",
        "defined_modes = [\"streaming\", \"snapshot\"]",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
    ] {
        assert!(
            component_manifest.contains(marker),
            "Component.toml should keep streaming-term scope marker `{marker}`."
        );
    }

    for marker in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "view.rs should expose machine-readable stream marker `{marker}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_streaming_term_is_limited_to_llm_output_render_modes";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );
}

#[test]
fn autocomplete_snapshot_is_foundational_and_complete_config_renders_stably() {
    let check2_source = load_source("src/autocomplete/check2.md");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let view_source = load_source("src/autocomplete/view.rs");
    let component_manifest = load_source("src/autocomplete/Component.toml");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "snapshot_is_foundational_and_complete_config_renders_stably",
        "autocomplete_snapshot_is_foundational_and_complete_config_renders_stably",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "check2 should keep snapshot-foundation marker `{marker}`."
        );
    }

    for marker in [
        "pub fn Autocomplete(",
        "id_base: String,",
        "label: String,",
        "items: Vec<String>,",
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool,",
        "#[prop(optional)] motion: AutocompleteMotion,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
    ] {
        assert!(
            view_source.contains(marker),
            "view.rs should keep complete-config snapshot render marker `{marker}`."
        );
    }

    for marker in [
        "Self::Verified => \"verified\"",
        "output_status: AutocompleteAgentOutputStatus::Verified,",
        "Self::Snapshot => \"snapshot\"",
        "stream_mode: AutocompleteAgentStreamMode::Snapshot,",
        "stream_fallback: AutocompleteAgentStreamFallback::Snapshot,",
    ] {
        assert!(
            logic_source.contains(marker),
            "logic.rs should keep snapshot/output-status marker `{marker}`."
        );
    }

    for marker in [
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "view.rs should expose snapshot output marker `{marker}`."
        );
    }

    for marker in [
        "[streaming_policy]",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
        "attr = \"data-ui-stream-fallback\"",
        "attr = \"data-ui-stream-mode\"",
        "values = [\"snapshot\"]",
        "attr = \"data-ui-output-status\"",
        "values = [\"verified\"]",
    ] {
        assert!(
            component_manifest.contains(marker),
            "Component.toml should keep snapshot-foundation marker `{marker}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_snapshot_is_foundational_and_complete_config_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );
}

#[test]
fn autocomplete_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status() {
    let check2_source = load_source("src/autocomplete/check2.md");
    let logic_source = load_source("src/autocomplete/logic.rs");
    let view_source = load_source("src/autocomplete/view.rs");
    let component_manifest = load_source("src/autocomplete/Component.toml");
    let readme_source = load_source("src/autocomplete/README.md");
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status",
        "autocomplete_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "check2 should keep streaming-requirement marker `{marker}`."
        );
    }

    for marker in [
        "## Streaming 策略",
        "`Snapshot`：默认路径，组件稳定消费完整配置并渲染。",
        "`Streaming Optional`：`Autocomplete` 不是 LLM 正文阅读面；若上层为流式容器，本组件按 `fallback=snapshot` 方式消费稳定配置。",
    ] {
        assert!(
            readme_source.contains(marker),
            "README should keep streaming optional policy marker `{marker}`."
        );
    }

    for marker in [
        "required = false",
        "owner = \"upstream\"",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
    ] {
        assert!(
            component_manifest.contains(marker),
            "Component.toml should keep streaming optional policy marker `{marker}`."
        );
    }

    for marker in [
        "output_status: AutocompleteAgentOutputStatus::Verified,",
        "stream_support: AutocompleteAgentStreamSupport::Unsupported,",
        "stream_fallback: AutocompleteAgentStreamFallback::Snapshot,",
        "stream_mode: AutocompleteAgentStreamMode::Snapshot,",
    ] {
        assert!(
            logic_source.contains(marker),
            "logic.rs should keep explicit output status marker `{marker}`."
        );
    }

    for marker in [
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "view.rs should keep explicit output marker `{marker}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );
}

#[test]
fn autocomplete_check2_has_no_unchecked_checklist_items() {
    let checklist = load_source("src/autocomplete/check2.md");
    assert!(
        checklist.contains("# 单组件 Check List"),
        "Autocomplete check2 checklist should keep the standard checklist template header.",
    );
    assert!(
        checklist.contains("- [ ]") || checklist.contains("- [x]"),
        "Autocomplete check2 checklist should keep markdown checkbox items for manual review.",
    );
}

#[test]
fn autocomplete_rust_hygiene_forbids_unwrap_expect_and_let_result_swallowing_in_non_test_sources() {
    for (path, source) in [
        (
            "src/autocomplete/mod.rs",
            load_source("src/autocomplete/mod.rs"),
        ),
        (
            "src/autocomplete/logic.rs",
            load_source("src/autocomplete/logic.rs"),
        ),
        (
            "src/autocomplete/view.rs",
            load_source("src/autocomplete/view.rs"),
        ),
        (
            "src/autocomplete/styles.rs",
            load_source("src/autocomplete/styles.rs"),
        ),
        (
            "src/autocomplete/motion.rs",
            load_source("src/autocomplete/motion.rs"),
        ),
        (
            "src/autocomplete/protocol.rs",
            load_source("src/autocomplete/protocol.rs"),
        ),
    ] {
        assert!(
            !source.contains("unwrap("),
            "{path} should not contain `unwrap(` in non-test code."
        );
        assert!(
            !source.contains("expect("),
            "{path} should not contain `expect(` in non-test code."
        );
        assert!(
            !source.contains("let _ ="),
            "{path} should not swallow side-effect results via `let _ = ...`."
        );
    }
}

#[test]
fn autocomplete_string_clone_hotspots_converge_to_cow_static_str_for_class_tokens() {
    let logic_source = load_source("src/autocomplete/logic.rs");

    for marker in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(\"ui-autocomplete\")];",
        "classes.push(Cow::Borrowed(\"ui-autocomplete--disabled\"));",
        "classes.push(Cow::Borrowed(\"ui-autocomplete--custom-class\"));",
        "classes.push(Cow::Owned(base_class_name));",
        "let mut composed = first.into_owned();",
        "composed.push_str(class.as_ref());",
    ] {
        assert!(
            logic_source.contains(marker),
            "logic.rs should keep Cow-based class token composition marker `{marker}`."
        );
    }

    for forbidden in [
        "vec![\"ui-autocomplete\".to_string()]",
        "classes.push(\"ui-autocomplete--disabled\".to_string())",
        "classes.push(\"ui-autocomplete--custom-class\".to_string())",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should remove String clone hotspot `{forbidden}`."
        );
    }
}

#[test]
fn autocomplete_check2_marks_rust_hygiene_item_complete_with_component_scope() {
    let check2_source = load_source("src/autocomplete/check2.md");
    for marker in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "rust_hygiene_forbids_unwrap_expect_and_let_result_swallowing_in_non_test_sources",
        "string_clone_hotspots_converge_to_cow_static_str_for_class_tokens",
        "autocomplete_rust_hygiene_forbids_unwrap_expect_and_let_result_swallowing_in_non_test_sources",
        "autocomplete_string_clone_hotspots_converge_to_cow_static_str_for_class_tokens",
        "scripts/check-ui-components-contract-hygiene.sh",
        "`./scripts/check-rust-hygiene.sh`",
    ] {
        assert!(
            check2_source.contains(marker),
            "check2 should keep rust-hygiene marker `{marker}`."
        );
    }
}

#[test]
fn autocomplete_contract_hygiene_script_covers_rust_hygiene_guards() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    for needle in [
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_rust_hygiene_forbids_unwrap_expect_and_let_result_swallowing_in_non_test_sources",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_string_clone_hotspots_converge_to_cow_static_str_for_class_tokens",
        "cargo test -p ui-components --test autocomplete_semantics --no-default-features --features component-autocomplete,inject-css autocomplete_check2_marks_rust_hygiene_item_complete_with_component_scope",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should include rust-hygiene guard `{needle}`."
        );
    }
}
