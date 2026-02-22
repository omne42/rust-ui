use std::fs;
use std::path::Path;

fn workspace_dir() -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"))
        .to_path_buf()
}

fn load_source(rel_path: &str) -> String {
    if let Some(component_rel_path) = rel_path.strip_prefix("src/menu/") {
        let path = workspace_dir()
            .join("components/menu/src")
            .join(component_rel_path);
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }
    if let Some(component_rel_path) = rel_path.strip_prefix("src/popover/") {
        let path = workspace_dir()
            .join("components/popover/src")
            .join(component_rel_path);
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn action_menu_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/menu/action_menu/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ActionMenu internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn action_menu_module_exposes_slot_and_state_contracts() {
    let source = load_source("src/menu/action_menu/mod.rs");

    for needle in [
        "pub type MenuOpenFocusStrategy = ui_headless::MenuOpenFocusStrategy;",
        "pub enum ActionMenuDisabledState",
        "pub enum ActionMenuActionMode",
        "pub struct ActionMenuItemSpec",
        "pub struct ActionMenuIds",
        "pub enum ActionMenuSlot",
        "pub struct ActionMenuPartStateInput",
        "pub struct ActionMenuPartState",
        "DEFAULT_ID_BASE",
        "DEFAULT_TRIGGER_ARIA_LABEL",
        "DEFAULT_DISABLED",
        "DEFAULT_CLOSE_ON_ACTION",
        "DEFAULT_PLACEMENT",
        "pub use motion::ActionMenuMotion;",
    ] {
        assert!(
            source.contains(needle),
            "action_menu::mod should include `{needle}` contracts."
        );
    }
}

#[test]
fn action_menu_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/menu/action_menu/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::ActionMenu;"),
        "action_menu module should export `ActionMenu`."
    );
    assert!(
        crate_source
            .contains("pub use action_menu::{ActionMenu, ActionMenuItemSpec, ActionMenuMotion};"),
        "crate root should re-export action_menu contracts."
    );
}

#[test]
fn action_menu_logic_exposes_state_helpers() {
    let source = load_source("src/menu/action_menu/logic.rs");

    for needle in [
        "pub struct ActionMenuItemsInput",
        "pub struct ActionMenuItemsOutput",
        "pub struct ActionMenuDiscreteProps",
        "pub struct ActionMenuNormalizeInput",
        "pub struct ActionMenuNormalizedProps",
        "pub struct ActionMenuTriggerPressResult",
        "pub fn normalize_menu_items(input: ActionMenuItemsInput) -> ActionMenuItemsOutput",
        "pub fn state_attr(is_open: bool, trigger_disabled: bool, item_count: usize)",
        "pub fn item_attr(item_count: usize)",
        "pub fn action_attr(close_on_action: bool)",
        "pub fn open_mode_attr(is_controlled: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_id_base(id_base: String)",
        "pub fn resolve_ids(id_base: &str)",
        "pub fn normalize_disabled_indices(disabled_indices: Vec<usize>, item_count: usize)",
        "pub fn resolve_trigger_disabled(disabled: bool, item_count: usize)",
        "pub fn resolve_trigger_aria_label(",
        "fallback_aria_label: &str",
        "pub fn normalize_discrete_props(",
        "pub fn normalize_props(input: ActionMenuNormalizeInput) -> ActionMenuNormalizedProps",
        "pub fn resolve_trigger_press(",
        "pub fn resolve_action_open_change(action_mode: ActionMenuActionMode) -> Option<bool>",
        "pub fn resolve_state(input: ActionMenuPartStateInput) -> ActionMenuPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ActionMenuPartState)",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn action_menu_state_primitives_are_sourced_from_ui_state_primitives() {
    let source = load_source("src/menu/action_menu/logic.rs");

    for needle in [
        "use ui_state_primitives::action_menu as action_menu_state;",
        "pub const DEFAULT_ID_BASE: &str = action_menu_state::DEFAULT_ID_BASE;",
        "pub const DEFAULT_TRIGGER_ARIA_LABEL: &str = action_menu_state::DEFAULT_TRIGGER_ARIA_LABEL;",
        "action_menu_state::state_attr(",
        "action_menu_state::normalize_id_base(",
        "action_menu_state::resolve_id_pair(",
        "action_menu_state::normalize_disabled_indices(",
        "action_menu_state::resolve_trigger_disabled(",
        "action_menu_state::resolve_trigger_aria_label_with_fallback(",
        "action_menu_state::normalize_boolean_props(",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu state primitives should delegate to ui-state-primitives via `{needle}`."
        );
    }

    for forbidden in [
        "let mut unique = BTreeSet::new();",
        "unique.into_iter().collect()",
        "if let Some(label) = normalize_optional_text(value) {",
        "format!(\"{id_base}-trigger\")",
        "format!(\"{id_base}-menu\")",
    ] {
        assert!(
            !source.contains(forbidden),
            "ActionMenu should not reimplement state primitive logic in ui: `{forbidden}`."
        );
    }
}

#[test]
fn action_menu_view_uses_logic_contracts_and_source_markers() {
    let source = load_source("src/menu/action_menu/view.rs");

    for needle in [
        "logic::normalize_menu_items(logic::ActionMenuItemsInput {",
        "logic::normalize_props(logic::ActionMenuNormalizeInput {",
        "logic::resolve_state(ActionMenuPartStateInput {",
        "slot: ActionMenuSlot::Root",
        "logic::compose_class_name(class_name.get_value(), root_state_for_class.get())",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
        "data-items=move || root_state.get().item_attr",
        "data-action-mode=move || root_state.get().action_attr",
        "data-open-mode=move || root_state.get().open_mode_attr",
        "data-id-source=move || root_state.get().id_source_attr",
        "data-aria-label-source=move || root_state.get().aria_label_source_attr",
        "data-class-source=move || root_state.get().class_source_attr",
        "data-disabled-source=move || root_state.get().disabled_source_attr",
        "data-disabled-indices-source=move || root_state.get().disabled_indices_source_attr",
        "data-item-kinds-source=move || root_state.get().item_kinds_source_attr",
        "data-close-on-action-source=move || root_state.get().close_on_action_source_attr",
        "data-placement-source=move || root_state.get().placement_source_attr",
        "data-open-source=move || root_state.get().open_source_attr",
        "data-default-open-source=move || root_state.get().default_open_source_attr",
        "data-open-change-source=move || root_state.get().open_change_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "data-custom-id=move || root_state.get().has_custom_id_base.then_some(\"true\")",
        "data-custom-aria-label=move || root_state.get().has_custom_aria_label.then_some(\"true\")",
        "data-custom-class=move || root_state.get().has_custom_class_name.then_some(\"true\")",
        "data-custom-disabled=move || root_state.get().has_custom_disabled.then_some(\"true\")",
        "data-custom-disabled-indices=move ||",
        "data-custom-item-kinds=move || root_state.get().has_custom_item_kinds.then_some(\"true\")",
        "data-custom-close-on-action=move ||",
        "data-custom-placement=move || root_state.get().has_custom_placement.then_some(\"true\")",
        "data-custom-open=move || root_state.get().has_custom_open.then_some(\"true\")",
        "data-custom-default-open=move || root_state.get().has_custom_default_open.then_some(\"true\")",
        "data-custom-open-change=move || root_state.get().has_custom_on_open_change.then_some(\"true\")",
        "data-custom-motion=move || root_state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu view should include `{needle}` for stable state/source marker contracts."
        );
    }

    for forbidden in [
        "logic::normalize_id_base(id_base)",
        "logic::normalize_disabled_indices(disabled_indices, item_count)",
        "logic::resolve_trigger_aria_label(",
    ] {
        assert!(
            !source.contains(forbidden),
            "ActionMenu view should not duplicate state normalization primitives after centralization: `{forbidden}`."
        );
    }
}

#[test]
fn action_menu_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/menu/action_menu/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "let has_custom_open = open.is_some()",
        "let has_custom_default_open = default_open.is_some()",
        "let has_custom_on_open_change = on_open_change.is_some()",
        "overlay_open::use_controllable_open_state_traced(",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu should support `{needle}` for controllable open behavior."
        );
    }
}

#[test]
fn action_menu_api_naming_uses_prefixed_props_with_legacy_alias_migration_path() {
    let view_source = load_source("src/menu/action_menu/view.rs");
    let logic_source = load_source("src/menu/action_menu/logic.rs");
    let state_primitives_source = load_source("../ui-state-primitives/src/action_menu.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "#[prop(optional)] disabled_state: Option<ActionMenuDisabledState>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: Option<bool>",
        "#[prop(optional)] action_mode: Option<ActionMenuActionMode>",
        "#[prop(optional)] is_close_on_action: Option<bool>",
        "#[prop(optional)] close_on_action: Option<bool>",
        "logic::normalize_props(",
        "pub fn normalize_discrete_props(",
        "is_disabled.or(disabled).unwrap_or(DEFAULT_DISABLED)",
        "is_close_on_action",
        ".or(close_on_action)",
        "is_close_on_action=false",
        "is_disabled=true",
        "action_menu_state::normalize_boolean_props(",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || state_primitives_source.contains(needle)
                || docs_source.contains(needle),
            "ActionMenu API naming contract should include `{needle}`."
        );
    }
}

#[test]
fn action_menu_default_values_have_single_source_in_logic_layer() {
    let logic_source = load_source("src/menu/action_menu/logic.rs");
    let view_source = load_source("src/menu/action_menu/view.rs");

    for needle in [
        "pub fn normalize_discrete_props(",
        "action_menu_state::normalize_boolean_props(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionMenu logic should own default normalization rule `{needle}`."
        );
    }

    for forbidden in [
        "is_disabled.or(disabled).unwrap_or(",
        "is_close_on_action\n        .or(close_on_action)\n        .unwrap_or(",
        ".unwrap_or(logic::DEFAULT_DISABLED)",
        ".unwrap_or(logic::DEFAULT_CLOSE_ON_ACTION)",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ActionMenu view should not contain default fallback logic: `{forbidden}`."
        );
    }
}

#[test]
fn action_menu_state_normalization_is_centralized_in_logic_layer() {
    let logic_source = load_source("src/menu/action_menu/logic.rs");
    let view_source = load_source("src/menu/action_menu/view.rs");

    for needle in [
        "pub struct ActionMenuNormalizeInput",
        "pub struct ActionMenuNormalizedProps",
        "pub fn normalize_props(input: ActionMenuNormalizeInput) -> ActionMenuNormalizedProps",
        "pub fn resolve_trigger_press(",
        "pub fn resolve_action_open_change(action_mode: ActionMenuActionMode) -> Option<bool>",
        "let id_base = normalize_id_base(input.id_base);",
        "let disabled_indices = normalize_disabled_indices(input.disabled_indices, input.item_count);",
        "let (aria_label, has_custom_aria_label) =",
        "resolve_trigger_aria_label(input.aria_label, input.fallback_aria_label.as_str());",
    ] {
        assert!(
            logic_source.contains(needle),
            "ActionMenu logic should centralize state normalization via `{needle}`."
        );
    }

    for needle in [
        "logic::normalize_props(logic::ActionMenuNormalizeInput {",
        "if let Some(result) = logic::resolve_trigger_press(",
        "if let Some(next_open) = logic::resolve_action_open_change(action_mode)",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionMenu view should consume centralized logic contracts via `{needle}`."
        );
    }

    for forbidden in [
        "let has_custom_id_base =",
        "let has_disabled_items = !disabled_indices.is_empty();",
        "let has_custom_disabled_indices = has_disabled_items;",
        "let has_item_kinds = !item_kinds.is_empty();",
        "let has_custom_item_kinds = has_item_kinds;",
        "let has_custom_class_name = class_name.is_some();",
        "let (aria_label, has_custom_aria_label) = logic::resolve_trigger_aria_label(",
        "if trigger_disabled {",
        "let next_open = !open.get_untracked();",
        "if is_close_on_action {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ActionMenu view should not rebuild normalization/state-machine branches: `{forbidden}`."
        );
    }
}

#[test]
fn action_menu_discrete_state_axes_use_enum_contracts() {
    let mod_source = load_source("src/menu/action_menu/mod.rs");
    let view_source = load_source("src/menu/action_menu/view.rs");
    let logic_source = load_source("src/menu/action_menu/logic.rs");

    for needle in [
        "pub enum ActionMenuDisabledState",
        "pub enum ActionMenuActionMode",
        "pub fn from_bool(",
        "pub fn is_disabled(self) -> bool",
        "pub fn is_close_on_action(self) -> bool",
        "#[prop(optional)] disabled_state: Option<ActionMenuDisabledState>",
        "#[prop(optional)] action_mode: Option<ActionMenuActionMode>",
        "disabled_state: Option<ActionMenuDisabledState>",
        "action_mode: Option<ActionMenuActionMode>",
        "normalize_discrete_props(",
        "ActionMenuDisabledState::from_bool(",
        "ActionMenuActionMode::from_bool(",
    ] {
        assert!(
            mod_source.contains(needle)
                || view_source.contains(needle)
                || logic_source.contains(needle),
            "ActionMenu discrete-state typing contract should include `{needle}`."
        );
    }
}

#[test]
fn action_menu_component_uses_state_primitive_boundary_without_business_store_binding() {
    let logic_source = load_source("src/menu/action_menu/logic.rs");
    let view_source = load_source("src/menu/action_menu/view.rs");

    assert!(
        logic_source.contains("use ui_state_primitives::action_menu as action_menu_state;"),
        "ActionMenu logic should consume ui-state-primitives as the state source boundary."
    );

    for forbidden in [
        "crate::app::",
        "crate::store::",
        "AppStore",
        "GlobalStore",
        "AppState",
        "use_store(",
        "leptos_reactive::store",
        "redux",
        "zustand",
        "RwSignal<",
        "Signal<",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "ActionMenu logic must not bind business/global store or framework signal containers: `{forbidden}`."
        );
    }

    for forbidden in [
        "crate::app::",
        "crate::store::",
        "AppStore",
        "GlobalStore",
        "AppState",
        "use_store(",
        "leptos_reactive::store",
        "redux",
        "zustand",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ActionMenu view must keep app-store adaptation boundaries outside the component: `{forbidden}`."
        );
    }
}

#[test]
fn action_menu_has_no_async_loading_protocol_and_keeps_sync_action_contract() {
    let view_source = load_source("src/menu/action_menu/view.rs");
    let logic_source = load_source("src/menu/action_menu/logic.rs");

    for needle in [
        "on_action: Callback<usize>",
        "let on_action_wrapped = Callback::new(move |index: usize| {",
        "on_action.run(index);",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionMenu should keep a synchronous action contract via `{needle}`."
        );
    }

    for forbidden in [
        "is_loading",
        "aria-busy",
        "aria_busy",
        "retry",
        "on_error",
        "use_async_action",
        "spawn_local(",
        "Future",
        "tokio::",
        "async fn",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "ActionMenu should not define per-component async loading/error protocol: `{forbidden}`."
        );
    }
}

#[test]
fn action_menu_open_axis_is_complete_controllable_contract_without_half_controlled_state() {
    let view_source = load_source("src/menu/action_menu/view.rs");
    let headless_controllable_source = load_source("../ui-headless/src/controllable_state.rs");
    let headless_controllable_test_source =
        load_source("../ui-headless/src/test/controllable_state.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "let has_custom_open = open.is_some();",
        "let has_custom_default_open = default_open.is_some();",
        "let has_custom_on_open_change = on_open_change.is_some();",
        "let open_state = overlay_open::use_controllable_open_state_traced(",
        "let open = open_state.open;",
        "let request_open_change = open_state.request_open_change;",
        "request_open_change.run(next_open);",
        "let on_close: OnPress = Callback::new(move |_| request_open_change.run(false));",
        "controlled_open_does_not_update_internal_state",
        "uncontrolled_open_updates_state_and_calls_on_change",
        "request_open_change_ignores_noop_updates",
    ] {
        assert!(
            view_source.contains(needle)
                || headless_controllable_source.contains(needle)
                || headless_controllable_test_source.contains(needle),
            "ActionMenu controllable contract should include `{needle}`."
        );
    }

    for forbidden in [
        "let (open, set_open) = signal(",
        "set_open.set(",
        "set_open.update(",
        "open.set(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ActionMenu should not maintain an extra local open source-of-truth in component view: `{forbidden}`."
        );
    }
}

#[test]
fn action_menu_open_key_model_is_delegated_to_ui_headless() {
    let mod_source = load_source("src/menu/action_menu/mod.rs");
    let view_source = load_source("src/menu/action_menu/view.rs");

    for needle in [
        "ui_headless::menu_trigger_open_focus_strategy_for_key(key)",
        "if let Some(strategy) = ui_headless::menu_trigger_open_focus_strategy(",
        "set_open_focus.set(strategy);",
    ] {
        assert!(
            mod_source.contains(needle) || view_source.contains(needle),
            "ActionMenu open key model should delegate to ui-headless via `{needle}`."
        );
    }

    for forbidden in [
        "if let Some(strategy) = crate::action_menu::focus_strategy_for_open_key(&key)",
        "\"ArrowDown\" => Some(MenuOpenFocusStrategy::First)",
        "\"ArrowUp\" => Some(MenuOpenFocusStrategy::Last)",
    ] {
        assert!(
            !view_source.contains(forbidden) && !mod_source.contains(forbidden),
            "ActionMenu should not reimplement open key semantics in ui: `{forbidden}`."
        );
    }
}

#[test]
fn action_menu_a11y_i18n_and_locale_contract_is_wired() {
    let view_source = load_source("src/menu/action_menu/view.rs");
    let logic_source = load_source("src/menu/action_menu/logic.rs");
    let i18n_source = load_source("../ui-headless/src/i18n/common.rs");
    let a11y_source = load_source("../ui-headless/src/a11y.rs");

    for needle in [
        "use ui_headless::{",
        "A11yDirection",
        "CommonStrings",
        "locale_attrs",
        "use_ui_i18n",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let i18n = use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "let locale = locale_attrs(lang, dir);",
        "fallback_aria_label: common.action_menu_trigger_aria_label.to_string()",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "resolve_trigger_aria_label(input.aria_label, input.fallback_aria_label.as_str())",
        "pub action_menu_trigger_aria_label: Arc<str>,",
        "action_menu_trigger_aria_label: \"More actions\".into(),",
        "pub fn locale_attrs(lang: Option<String>, dir: Option<A11yDirection>)",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || i18n_source.contains(needle)
                || a11y_source.contains(needle),
            "ActionMenu A11y/i18n/locale contract should include `{needle}`."
        );
    }

    for forbidden in [
        "aria_label=\"More actions\"",
        "aria_label=\"Workspace actions\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ActionMenu view should not hardcode user-facing aria copy: `{forbidden}`."
        );
    }
}

#[test]
fn action_menu_trigger_uses_action_button_with_overlay_aria_contract() {
    let source = load_source("src/menu/action_menu/view.rs");

    for needle in [
        "<ActionButton",
        "aria_haspopup=\"menu\"",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
        "aria_label=aria_label.get_value()",
        "disabled=trigger_disabled",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu should wire its trigger via `{needle}` for baseline overlay semantics."
        );
    }
}

#[test]
fn action_menu_renders_menu_inside_popover_with_presence() {
    let source = load_source("src/menu/action_menu/view.rs");

    for needle in [
        "use_presence(open)",
        "<Popover",
        "<Menu",
        "aria_labelledby=trigger_id.get_value()",
        "on_exit_complete=presence.finish_exit",
        "motion=motion.popover",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu should compose menu/popover/presence via `{needle}` for motion-safe unmounting."
        );
    }
}

#[test]
fn action_menu_styles_include_state_and_source_markers() {
    let source = load_source("src/menu/action_menu/styles.rs");

    for needle in [
        ".ui-action-menu {",
        ".ui-action-menu--open",
        ".ui-action-menu[data-state=\"open\"]",
        ".ui-action-menu--persistent",
        ".ui-action-menu[data-action-mode=\"keep-open\"]",
        ".ui-action-menu[data-open-mode=\"controlled\"]",
        ".ui-action-menu[data-id-source=\"custom\"]",
        ".ui-action-menu[data-custom-id=\"true\"]",
        ".ui-action-menu--custom-id",
        ".ui-action-menu[data-aria-label-source=\"custom\"]",
        ".ui-action-menu[data-class-source=\"custom\"]",
        ".ui-action-menu[data-custom-class=\"true\"]",
        ".ui-action-menu--custom-class",
        ".ui-action-menu[data-disabled-source=\"custom\"]",
        ".ui-action-menu[data-custom-disabled=\"true\"]",
        ".ui-action-menu--custom-disabled",
        ".ui-action-menu[data-close-on-action-source=\"custom\"]",
        ".ui-action-menu[data-custom-close-on-action=\"true\"]",
        ".ui-action-menu--custom-close-on-action",
        ".ui-action-menu[data-placement-source=\"custom\"]",
        ".ui-action-menu[data-custom-placement=\"true\"]",
        ".ui-action-menu--custom-placement",
        ".ui-action-menu[data-default-open-source=\"custom\"]",
        ".ui-action-menu[data-custom-default-open=\"true\"]",
        ".ui-action-menu--custom-default-open",
        ".ui-action-menu[data-custom-aria-label=\"true\"]",
        ".ui-action-menu--custom-aria-label",
        ".ui-action-menu[data-disabled-indices-source=\"custom\"]",
        ".ui-action-menu[data-custom-disabled-indices=\"true\"]",
        ".ui-action-menu--custom-disabled-indices",
        ".ui-action-menu[data-item-kinds-source=\"custom\"]",
        ".ui-action-menu[data-custom-item-kinds=\"true\"]",
        ".ui-action-menu--custom-item-kinds",
        ".ui-action-menu[data-open-source=\"custom\"]",
        ".ui-action-menu[data-custom-open=\"true\"]",
        ".ui-action-menu--custom-open",
        ".ui-action-menu[data-open-change-source=\"custom\"]",
        ".ui-action-menu[data-custom-open-change=\"true\"]",
        ".ui-action-menu--custom-open-change",
        ".ui-action-menu[data-motion-source=\"custom\"]",
        ".ui-action-menu--custom-motion",
        ".ui-action-menu[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu styles should include `{needle}` for stable state/source contracts."
        );
    }
}

#[test]
fn action_menu_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let view_source = load_source("src/menu/action_menu/view.rs");
    let styles_source = load_source("src/menu/action_menu/styles.rs");

    for needle in [
        "data-state=move || root_state.get().state_attr",
        "data-items=move || root_state.get().item_attr",
        "data-action-mode=move || root_state.get().action_attr",
        "data-open-mode=move || root_state.get().open_mode_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "data-custom-motion=move || root_state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "ActionMenu view should expose explicit style state marker `{needle}`."
        );
    }

    for selector in [
        ".ui-action-menu[data-state=\"open\"]",
        ".ui-action-menu[data-state=\"closed\"]",
        ".ui-action-menu[data-state=\"empty\"]",
        ".ui-action-menu[data-state=\"disabled\"]",
        ".ui-action-menu[data-items=\"populated\"]",
        ".ui-action-menu[data-open-mode=\"controlled\"]",
        ".ui-action-menu[data-action-mode=\"keep-open\"]",
        ".ui-action-menu[data-motion-source=\"custom\"]",
        ".ui-action-menu[data-custom-motion=\"true\"]",
        ".ui-action-menu[data-action-mode=\"keep-open\"] .ui-action-button",
    ] {
        assert!(
            styles_source.contains(selector),
            "ActionMenu styles should consume explicit marker selector `{selector}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":first-child", ":last-child"] {
        assert!(
            !styles_source.contains(forbidden),
            "ActionMenu styles should not use brittle structural selector `{forbidden}` to infer state."
        );
    }

    assert!(
        !view_source.contains(" style="),
        "ActionMenu view should avoid inline style branches for business state."
    );
}

#[test]
fn action_menu_semantics_suite_prioritizes_contract_assertions_over_snapshots() {
    let suite_source = load_source("tests/action_menu_semantics.rs");

    for semantic_signal in [
        "action_menu_state_markers_are_observable_and_closed_set_contracts",
        "action_menu_styles_depend_on_explicit_state_markers_not_dom_guessing",
        "action_menu_supports_controlled_and_uncontrolled_open_state",
        "action_menu_open_axis_is_complete_controllable_contract_without_half_controlled_state",
        "action_menu_open_key_model_is_delegated_to_ui_headless",
        "action_menu_trigger_uses_action_button_with_overlay_aria_contract",
        "action_menu_motion_layering_delegates_to_popover_and_ui_motion_backends",
        "data-state=move || root_state.get().state_attr",
        "data-open-mode=move || root_state.get().open_mode_attr",
        "data-open-source=move || root_state.get().open_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
    ] {
        assert!(
            suite_source.contains(semantic_signal),
            "ActionMenu semantic suite should keep contract assertion signal `{semantic_signal}`."
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
            "ActionMenu semantic suite should not rely on visual snapshot token `{forbidden}`."
        );
    }
}

#[test]
fn action_menu_state_markers_are_observable_and_closed_set_contracts() {
    let view_source = load_source("src/menu/action_menu/view.rs");
    let menu_view_source = load_source("src/menu/view.rs");
    let logic_source = load_source("src/menu/action_menu/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/action_menu.rs");

    for needle in [
        "data-state=move || root_state.get().state_attr",
        "data-items=move || root_state.get().item_attr",
        "data-action-mode=move || root_state.get().action_attr",
        "data-open-mode=move || root_state.get().open_mode_attr",
        "data-open=move || root_state.get().open_attr",
        "data-disabled=move || root_state.get().is_trigger_disabled.then_some(\"true\")",
        "data-id-source=move || root_state.get().id_source_attr",
        "data-aria-label-source=move || root_state.get().aria_label_source_attr",
        "data-open-source=move || root_state.get().open_source_attr",
        "data-default-open-source=move || root_state.get().default_open_source_attr",
        "data-open-change-source=move || root_state.get().open_change_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "aria_haspopup=\"menu\"",
        "aria_expanded=open",
        "aria_controls_signal=aria_controls",
        "aria_label=aria_label.get_value()",
        "role=aria.attrs.role",
        "aria-disabled=aria.attrs.aria_disabled",
        "aria-label=aria_label.get_value()",
        "aria-labelledby=aria_labelledby.get_value()",
    ] {
        assert!(
            view_source.contains(needle) || menu_view_source.contains(needle),
            "ActionMenu should expose observable semantic marker `{needle}`."
        );
    }

    for closed_set_case in [
        "pub fn state_attr(is_open: bool, trigger_disabled: bool, item_count: usize) -> &'static str",
        "\"open\"",
        "\"disabled\"",
        "\"empty\"",
        "\"closed\"",
        "pub fn item_attr(item_count: usize) -> &'static str",
        "\"populated\"",
        "pub fn action_attr(close_on_action: bool) -> &'static str",
        "\"close\"",
        "\"keep-open\"",
        "pub fn open_mode_attr(is_controlled: bool) -> &'static str",
        "\"controlled\"",
        "\"uncontrolled\"",
        "fn source_attr(is_custom: bool) -> &'static str",
        "\"custom\"",
        "\"default\"",
    ] {
        assert!(
            primitive_source.contains(closed_set_case) || logic_source.contains(closed_set_case),
            "ActionMenu marker values should be enumerable closed sets; missing `{closed_set_case}`."
        );
    }
}

#[test]
fn action_menu_theme_layering_uses_ui_theme_tokens_without_local_theme_rebuild() {
    let action_menu_styles_source = load_source("src/menu/action_menu/styles.rs");
    let action_menu_logic_source = load_source("src/menu/action_menu/logic.rs");
    let action_menu_view_source = load_source("src/menu/action_menu/view.rs");
    let ui_root_source = load_source("src/root.rs");
    let ui_theme_tokens_source = load_source("../ui-theme/src/tokens.rs");
    let ui_theme_theme_source = load_source("../ui-theme/src/theme.rs");
    let ui_theme_css_source = load_source("../ui-theme/src/css.rs");
    let ui_theme_scale_checks = load_source("../ui-theme/tests/token_scale_baseline.rs");
    let ui_theme_wcag_checks = load_source("../ui-theme/tests/wcag_contrast.rs");
    let styling_spec_source = load_source("../../docs/spec/styling.md");

    for needle in [
        "This file is the single source of truth for token taxonomy and baselines.",
        "pub enum ThemeSystem",
        "pub enum ThemeColor",
        "pub enum ThemeScale",
        "--ui-system",
        "--ui-color",
        "--ui-scale",
        "theme.get().to_css_variables()",
        "data-theme-system",
        "data-theme-color",
        "data-theme-scale",
        "token_scale_baselines_are_regression_testable",
        "semantic_colors_meet_wcag_21_aa_for_text_pairs",
        "Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量；组件只在 `crates/ui/src/<component>/styles.rs` 消费。",
    ] {
        assert!(
            ui_theme_tokens_source.contains(needle)
                || ui_theme_theme_source.contains(needle)
                || ui_theme_css_source.contains(needle)
                || ui_root_source.contains(needle)
                || ui_theme_scale_checks.contains(needle)
                || ui_theme_wcag_checks.contains(needle)
                || styling_spec_source.contains(needle),
            "Theme layering contract should include `{needle}`."
        );
    }

    assert!(
        action_menu_styles_source.contains("var(--ui-shadow-sm)"),
        "ActionMenu styles should consume ui-theme CSS variables via `var(--ui-*)`."
    );

    for forbidden in [
        "ui_theme::Theme",
        "Theme::new(",
        "theme_to_css_variables(",
        "oklch(",
        "rgb(",
        "hsl(",
    ] {
        assert!(
            !action_menu_styles_source.contains(forbidden)
                && !action_menu_logic_source.contains(forbidden)
                && !action_menu_view_source.contains(forbidden),
            "ActionMenu should not rebuild theme tokens or hardcode theme colors locally: `{forbidden}`."
        );
    }
}

#[test]
fn action_menu_ui_components_layer_assembles_four_layers_without_public_dom_leakage() {
    let mod_source = load_source("src/menu/action_menu/mod.rs");
    let logic_source = load_source("src/menu/action_menu/logic.rs");
    let view_source = load_source("src/menu/action_menu/view.rs");
    let styles_source = load_source("src/menu/action_menu/styles.rs");
    let motion_source = load_source("src/menu/action_menu/motion.rs");
    let crate_source = load_source("src/lib.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::ActionMenu;",
        "pub use motion::ActionMenuMotion;",
        "use ui_state_primitives::action_menu as action_menu_state;",
        "overlay_open::use_controllable_open_state_traced(",
        "ui_headless::menu_trigger_open_focus_strategy(",
        "let motion = crate::action_menu::motion::sanitize_motion(motion);",
        "motion=motion.popover",
        "box-shadow: var(--ui-shadow-sm);",
        "pub use action_menu::{ActionMenu, ActionMenuItemSpec, ActionMenuMotion};",
    ] {
        assert!(
            mod_source.contains(needle)
                || logic_source.contains(needle)
                || view_source.contains(needle)
                || styles_source.contains(needle)
                || motion_source.contains(needle)
                || crate_source.contains(needle),
            "ActionMenu assembly contract should include `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "web_sys::",
        "NodeRef<",
        "HtmlElement",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "ActionMenu public API surface should not leak platform details: `{forbidden}`."
        );
    }
}

#[test]
fn action_menu_file_roles_are_explicit_and_non_overlapping() {
    let mod_source = load_source("src/menu/action_menu/mod.rs");
    let logic_source = load_source("src/menu/action_menu/logic.rs");
    let styles_source = load_source("src/menu/action_menu/styles.rs");
    let view_source = load_source("src/menu/action_menu/view.rs");
    let motion_source = load_source("src/menu/action_menu/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::ActionMenuMotion;",
        "pub use view::ActionMenu;",
    ] {
        assert!(
            mod_source.contains(needle),
            "action_menu::mod should keep export-boundary contract `{needle}`."
        );
    }

    for forbidden in [
        "view! {",
        "pub const CSS:",
        "pub fn normalize_props(",
        "pub fn resolve_state(",
        "SpringAnimator::new(",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "action_menu::mod should not carry implementation details: `{forbidden}`."
        );
    }

    for needle in [
        "pub fn normalize_props(input: ActionMenuNormalizeInput) -> ActionMenuNormalizedProps",
        "pub fn resolve_state(input: ActionMenuPartStateInput) -> ActionMenuPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ActionMenuPartState)",
        "use ui_state_primitives::action_menu as action_menu_state;",
    ] {
        assert!(
            logic_source.contains(needle),
            "action_menu::logic should own normalization/derivation contracts via `{needle}`."
        );
    }

    for forbidden in [
        "view! {",
        "<ActionButton",
        "<Popover",
        "NodeRef<",
        "on:keydown",
        "box-shadow:",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "action_menu::logic should not include view/style/runtime DOM details: `{forbidden}`."
        );
    }

    for needle in [
        ".ui-action-menu {",
        ".ui-action-menu[data-state=\"open\"]",
        ".ui-action-menu[data-action-mode=\"keep-open\"]",
        "var(--ui-shadow-sm)",
    ] {
        assert!(
            styles_source.contains(needle),
            "action_menu::styles should expose token-first static CSS selector contract `{needle}`."
        );
    }

    for forbidden in [
        "view! {",
        "on:keydown",
        "aria_haspopup",
        "Callback<",
        "fn normalize_props(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "action_menu::styles should not include logic/view behavior: `{forbidden}`."
        );
    }

    for needle in [
        "view! {",
        "logic::normalize_props(logic::ActionMenuNormalizeInput {",
        "logic::resolve_state(ActionMenuPartStateInput {",
        "overlay_open::use_controllable_open_state_traced(",
        "if let Some(strategy) = ui_headless::menu_trigger_open_focus_strategy(",
        "<ActionButton",
        "<Popover",
        "<Menu",
    ] {
        assert!(
            view_source.contains(needle),
            "action_menu::view should keep rendering/headless mount contract `{needle}`."
        );
    }

    for forbidden in [
        "use ui_state_primitives::action_menu as action_menu_state;",
        "pub const CSS: &str",
        "SpringAnimator::new(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "action_menu::view should not absorb primitive/style/engine internals: `{forbidden}`."
        );
    }

    for needle in [
        "pub struct ActionMenuMotion",
        "pub fn sanitize_motion(motion: ActionMenuMotion) -> ActionMenuMotion",
        "crate::popover::motion::sanitize_motion(motion.popover)",
    ] {
        assert!(
            motion_source.contains(needle),
            "action_menu::motion should map semantic motion contract via `{needle}`."
        );
    }

    for forbidden in [
        "SpringAnimator::new(",
        "request_animation_frame(",
        "view! {",
        "<Popover",
        "aria_",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "action_menu::motion should not embed runtime driver/view/a11y logic: `{forbidden}`."
        );
    }
}

#[test]
fn action_menu_exposes_motion_contract_and_internal_module() {
    let mod_source = load_source("src/menu/action_menu/mod.rs");
    let motion_source = load_source("src/menu/action_menu/motion.rs");

    for needle in [
        "pub mod motion;",
        "pub use motion::ActionMenuMotion;",
        "pub struct ActionMenuMotion",
        "pub popover: PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || motion_source.contains(needle),
            "ActionMenu motion contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
fn action_menu_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn action_menu() -> AnyView",
        "title=\"ActionMenu\"",
        "slug=\"action-menu\"",
        "State + Source Markers",
        "data-id-source",
        "data-aria-label-source",
        "data-disabled-indices-source",
        "data-item-kinds-source",
        "data-open-source",
        "data-open-change-source",
        "data-motion-source",
        "<ActionMenu",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu docs page should contain `{needle}`."
        );
    }
}

#[test]
fn action_menu_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/menu/action_menu/motion.rs");
    let view_source = load_source("src/menu/action_menu/view.rs");
    let motion_test_source = load_source("../../components/menu/test/action_menu/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ActionMenuMotion) -> ActionMenuMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "fn sanitize_motion_delegates_to_popover_contract()",
    ] {
        assert!(
            motion_source.contains(needle) || motion_test_source.contains(needle),
            "ActionMenu motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::action_menu::motion::sanitize_motion(motion);"),
        "ActionMenu view should sanitize motion before forwarding to Popover.",
    );
}

#[test]
fn action_menu_motion_layering_delegates_to_popover_and_ui_motion_backends() {
    let action_menu_motion_source = load_source("src/menu/action_menu/motion.rs");
    let action_menu_view_source = load_source("src/menu/action_menu/view.rs");
    let popover_motion_source = load_source("src/popover/motion.rs");
    let ui_motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let ui_motion_spring_source = load_source("../ui-motion/src/spring.rs");

    for needle in [
        "use crate::popover::PopoverMotion;",
        "pub struct ActionMenuMotion",
        "pub popover: PopoverMotion",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "let motion = crate::action_menu::motion::sanitize_motion(motion);",
        "motion=motion.popover",
        "ui_motion::spring::SpringAnimator::new(",
        "if crate::web::prefers_reduced_motion() {",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            action_menu_motion_source.contains(needle)
                || action_menu_view_source.contains(needle)
                || popover_motion_source.contains(needle)
                || ui_motion_lib_source.contains(needle)
                || ui_motion_spring_source.contains(needle),
            "ActionMenu motion layering should include `{needle}`."
        );
    }

    for forbidden in [
        "SpringAnimator::new(",
        "request_animation_frame(",
        "aria_",
        "KeyboardEvent",
    ] {
        assert!(
            !action_menu_motion_source.contains(forbidden),
            "ActionMenu motion contract should not implement runtime driver/A11y logic directly: `{forbidden}`."
        );
    }
}

#[test]
fn action_menu_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "let marker_motion = ui::ActionMenuMotion {",
        "popover: ui::PopoverMotion {",
        "initial_scale: 0.93",
        "offset_y_px: 8.0",
        "title=\"State + Source Markers\"",
        "is_close_on_action=false",
        "open=marker_open",
        "default_open=true",
        "on_open_change=on_marker_open_change",
        "aria_label=\"Workspace actions\".to_string()",
        "class_name=\"docs-action-menu-custom\".to_string()",
        "motion=marker_motion",
    ] {
        assert!(
            source.contains(needle),
            "action-menu docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn action_menu_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn action_menu() -> AnyView",
        "title=\"ActionMenu\"",
        "slug=\"action-menu\"",
        "Playground title=\"Hello World\"",
        "Playground title=\"Default\"",
        "Playground title=\"Controlled + persistent open\"",
        "Playground title=\"State + Source Markers\"",
        "Playground title=\"Disabled + Empty\"",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should contain `{needle}` for ActionMenu.",
        );
    }
}

#[test]
fn action_menu_docs_expose_hello_world_path_without_state_machine_wiring() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    let block_start = source
        .find("let hello_code = Signal::derive(move || {")
        .expect("hello_code block should exist");
    let block_end = source[block_start..]
        .find("let code = Signal::derive(move || {")
        .map(|offset| block_start + offset)
        .expect("hello_code block should appear before default code block");
    let hello_block = &source[block_start..block_end];

    assert!(
        source.contains("Playground title=\"Hello World\""),
        "ActionMenu docs should expose a Hello World playground path."
    );
    for needle in [
        "<ActionMenu",
        "id_base=\"action-menu-hello\".to_string()",
        "item_specs=vec![ActionMenuItemSpec::action(\"Profile\")]",
        "on_action=Callback::new(|_| {})",
    ] {
        assert!(
            hello_block.contains(needle),
            "ActionMenu hello path should include `{needle}`."
        );
    }

    let snippet_start = hello_block
        .find("r#\"")
        .map(|idx| idx + 3)
        .expect("hello snippet should be embedded as raw string");
    let snippet_end = hello_block[snippet_start..]
        .find("\"#")
        .map(|offset| snippet_start + offset)
        .expect("hello snippet should terminate raw string");
    let hello_snippet = &hello_block[snippet_start..snippet_end];
    let meaningful_lines = hello_snippet
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();

    assert!(
        meaningful_lines <= 5,
        "ActionMenu Hello World snippet should be <= 5 lines, got {meaningful_lines} lines:\n{hello_snippet}"
    );

    for forbidden in [
        "open=",
        "default_open=",
        "on_open_change=",
        "state=",
        "ui_state_primitives",
        "ui_headless",
    ] {
        assert!(
            !hello_snippet.contains(forbidden),
            "ActionMenu Hello World path should not require state-machine wiring via `{forbidden}`."
        );
    }
}

#[test]
fn action_menu_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "title=\"Default\"",
        "id_base=\"docs-action-menu\".to_string()",
        "last action:",
        "title=\"Controlled + persistent open\"",
        "id_base=\"docs-action-menu-controlled\".to_string()",
        "is_close_on_action=false",
        "ActionMenuItemSpec::action(\"Duplicate project\").with_disabled(true)",
        "open=controlled_open",
        "on_open_change=on_open_change",
        "title=\"State + Source Markers\"",
        "id_base=\"docs-action-menu-markers\".to_string()",
        "default_open=true",
        "aria_label=\"Workspace actions\".to_string()",
        "class_name=\"docs-action-menu-custom\".to_string()",
        "motion=marker_motion",
        "title=\"Disabled + Empty\"",
        "id_base=\"docs-action-menu-disabled\".to_string()",
        "is_disabled=true",
        "id_base=\"docs-action-menu-empty\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "action-menu docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn action_menu_docs_prefer_typed_item_specs_over_parallel_arrays() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "ActionMenuItemSpec::action(\"Profile\")",
        "item_specs=default_items",
        "item_specs=controlled_items",
        "item_specs=marker_items",
        "item_specs=disabled_items",
    ] {
        assert!(
            source.contains(needle),
            "ActionMenu docs should prefer typed item specs via `{needle}`.",
        );
    }

    for forbidden in [
        "disabled_indices=vec![",
        "item_kinds=vec![",
        "labels + children",
        "titles + panels",
    ] {
        assert!(
            !source.contains(forbidden),
            "ActionMenu docs should not recommend parallel/implicit item API: `{forbidden}`.",
        );
    }
}
