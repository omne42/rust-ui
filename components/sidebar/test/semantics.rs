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

#[test]
fn sidebar_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/sidebar/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Sidebar internals should stay private; found `{needle}`.",
        );
    }
}

#[test]
fn sidebar_is_exported_from_module_and_crate_root() {
    let module_source = load_source("../../components/sidebar/src/mod.rs");
    let crate_source = load_source("../../crates/ui/src/lib.rs");

    assert!(
        module_source.contains("pub use view::Sidebar;"),
        "sidebar module should export `Sidebar`.",
    );
    assert!(
        module_source.contains("pub use logic::{SidebarCollapsible, SidebarSide, SidebarVariant};"),
        "sidebar module should export sidebar state enums.",
    );
    assert!(
        crate_source.contains("pub use sidebar::{"),
        "crate root should re-export sidebar contracts.",
    );
}

#[test]
fn sidebar_uses_logic_state_model() {
    let logic_source = load_source("../../components/sidebar/src/logic.rs");
    let view_source = load_source("../../components/sidebar/src/view.rs");

    for needle in [
        "pub enum SidebarSide",
        "pub enum SidebarVariant",
        "pub enum SidebarCollapsible",
        "pub struct SidebarStateInput",
        "pub struct SidebarState",
        "pub fn normalize_shortcut_key(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Sidebar logic should include `{needle}`.",
        );
    }

    for needle in [
        "headless::use_controllable_open_state_traced(",
        "logic::normalize_shortcut_key(shortcut_key, is_shortcut_enabled)",
        "motion::sanitize_motion(motion)",
        "motion::source_attr(motion)",
        "motion::attach_motion(motion)",
        "logic::resolve_state(SidebarStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "headless::use_sidebar_root(",
        "headless::sidebar_toggle_button_a11y_attrs(",
        "SidebarKeyDownInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "Sidebar view should derive behavior via logic helpers; missing `{needle}`.",
        );
    }
}

#[test]
fn sidebar_api_naming_contract_prefers_prefixed_names_with_compatibility_bridge() {
    let sidebar_view = load_source("../../components/sidebar/src/view.rs");
    let menu_view = load_source("../../components/sidebar/src/menu/view.rs");
    let group_view = load_source("../../components/sidebar/src/group/view.rs");
    let content_view = load_source("../../components/sidebar/src/content/view.rs");
    let footer_view = load_source("../../components/sidebar/src/footer/view.rs");
    let header_view = load_source("../../components/sidebar/src/header/view.rs");
    let inset_view = load_source("../../components/sidebar/src/inset/view.rs");
    let rail_view = load_source("../../components/sidebar/src/rail/view.rs");
    let trigger_view = load_source("../../components/sidebar/src/trigger/view.rs");
    let menu_action_view = load_source("../../components/sidebar/src/menu_action/view.rs");
    let menu_badge_view = load_source("../../components/sidebar/src/menu_badge/view.rs");

    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] is_trigger_visible: Option<bool>",
        "#[prop(optional)] is_shortcut_enabled: Option<bool>",
        "logic::resolve_disabled(is_disabled, disabled)",
        "logic::resolve_trigger_visibility(is_trigger_visible, show_trigger)",
        "logic::resolve_shortcut_enabled(is_shortcut_enabled, enable_shortcut)",
        "on_open_change: Option<Callback<bool>>",
        "default_open: Option<bool>",
    ] {
        assert!(
            sidebar_view.contains(needle),
            "Sidebar API naming contract should include `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] is_badges_visible: Option<bool>",
        "#[prop(optional)] is_actions_visible: Option<bool>",
        "#[prop(optional)] is_submenu_collapse_allowed: Option<bool>",
        "#[prop(optional)] is_keyboard_shortcut_enabled: Option<bool>",
        "logic::resolve_disabled(is_disabled, disabled)",
        "logic::resolve_show_badges(is_badges_visible, show_badges)",
        "logic::resolve_show_actions(is_actions_visible, show_actions)",
        "logic::resolve_allow_submenu_collapse(",
        "logic::resolve_keyboard_shortcut_enabled(",
        "on_active_id_change: Option<Callback<Option<String>>>",
        "default_active_id: Option<String>",
    ] {
        assert!(
            menu_view.contains(needle),
            "SidebarMenu API naming contract should include `{needle}`.",
        );
    }

    for (source, needle) in [
        (
            group_view.as_str(),
            "#[prop(optional)] is_collapsible: Option<bool>",
        ),
        (
            group_view.as_str(),
            "#[prop(optional)] is_label_visible: Option<bool>",
        ),
        (
            group_view.as_str(),
            "#[prop(optional)] is_action_visible: Option<bool>",
        ),
        (
            group_view.as_str(),
            "logic::resolve_collapsible(is_collapsible, collapsible)",
        ),
        (
            group_view.as_str(),
            "logic::resolve_label_visibility(is_label_visible, show_label)",
        ),
        (
            group_view.as_str(),
            "logic::resolve_action_visibility(is_action_visible, show_action)",
        ),
        (
            content_view.as_str(),
            "#[prop(optional)] is_padded: Option<bool>",
        ),
        (
            content_view.as_str(),
            "#[prop(optional)] is_scrollable: Option<bool>",
        ),
        (
            content_view.as_str(),
            "logic::resolve_padded(is_padded, padded)",
        ),
        (
            content_view.as_str(),
            "logic::resolve_scrollable(is_scrollable, scrollable)",
        ),
        (
            footer_view.as_str(),
            "#[prop(optional)] is_bordered: Option<bool>",
        ),
        (
            footer_view.as_str(),
            "logic::resolve_bordered(is_bordered, bordered)",
        ),
        (
            header_view.as_str(),
            "#[prop(optional)] is_disabled: Option<bool>",
        ),
        (
            inset_view.as_str(),
            "#[prop(optional)] is_padded: Option<bool>",
        ),
        (
            inset_view.as_str(),
            "#[prop(optional)] is_recessed: Option<bool>",
        ),
        (
            inset_view.as_str(),
            "logic::resolve_recessed(is_recessed, recessed)",
        ),
        (
            rail_view.as_str(),
            "#[prop(optional)] is_disabled: Option<bool>",
        ),
        (
            trigger_view.as_str(),
            "#[prop(optional)] is_disabled: Option<bool>",
        ),
        (
            menu_action_view.as_str(),
            "#[prop(optional)] is_hover_only: Option<bool>",
        ),
        (
            menu_action_view.as_str(),
            "logic::resolve_hover_only(is_hover_only, hover_only)",
        ),
        (
            menu_badge_view.as_str(),
            "#[prop(optional)] is_muted: Option<bool>",
        ),
        (
            menu_badge_view.as_str(),
            "logic::resolve_muted(is_muted, muted)",
        ),
    ] {
        assert!(
            source.contains(needle),
            "Sidebar subcomponent API naming contract should include `{needle}`.",
        );
    }
}

#[test]
fn sidebar_default_priority_is_centralized_in_logic_modules() {
    let sidebar_logic = load_source("../../components/sidebar/src/logic.rs");
    let menu_logic = load_source("../../components/sidebar/src/menu/logic.rs");
    let group_logic = load_source("../../components/sidebar/src/group/logic.rs");
    let content_logic = load_source("../../components/sidebar/src/content/logic.rs");
    let footer_logic = load_source("../../components/sidebar/src/footer/logic.rs");
    let header_logic = load_source("../../components/sidebar/src/header/logic.rs");
    let inset_logic = load_source("../../components/sidebar/src/inset/logic.rs");
    let rail_logic = load_source("../../components/sidebar/src/rail/logic.rs");
    let trigger_logic = load_source("../../components/sidebar/src/trigger/logic.rs");
    let menu_action_logic = load_source("../../components/sidebar/src/menu_action/logic.rs");
    let menu_badge_logic = load_source("../../components/sidebar/src/menu_badge/logic.rs");

    for (source, needle) in [
        (
            sidebar_logic.as_str(),
            "pub fn resolve_disabled(is_disabled: Option<bool>, disabled: bool) -> bool",
        ),
        (sidebar_logic.as_str(), "pub fn resolve_trigger_visibility("),
        (sidebar_logic.as_str(), "pub fn resolve_shortcut_enabled("),
        (
            sidebar_logic.as_str(),
            "pub fn normalize_trigger_label(value: Option<String>) -> String",
        ),
        (menu_logic.as_str(), "pub fn resolve_disabled("),
        (menu_logic.as_str(), "pub fn resolve_show_badges("),
        (menu_logic.as_str(), "pub fn resolve_show_actions("),
        (
            menu_logic.as_str(),
            "pub fn resolve_allow_submenu_collapse(",
        ),
        (
            menu_logic.as_str(),
            "pub fn resolve_keyboard_shortcut_enabled(",
        ),
        (
            menu_logic.as_str(),
            "pub fn normalize_keyboard_shortcut_key(",
        ),
        (group_logic.as_str(), "pub fn resolve_collapsible("),
        (group_logic.as_str(), "pub fn resolve_label_visibility("),
        (group_logic.as_str(), "pub fn resolve_action_visibility("),
        (content_logic.as_str(), "pub fn resolve_padded("),
        (content_logic.as_str(), "pub fn resolve_scrollable("),
        (footer_logic.as_str(), "pub fn resolve_bordered("),
        (header_logic.as_str(), "pub fn resolve_disabled("),
        (inset_logic.as_str(), "pub fn resolve_recessed("),
        (rail_logic.as_str(), "pub fn resolve_disabled("),
        (trigger_logic.as_str(), "pub fn resolve_disabled("),
        (menu_action_logic.as_str(), "pub fn resolve_hover_only("),
        (menu_badge_logic.as_str(), "pub fn resolve_muted("),
    ] {
        assert!(
            source.contains(needle),
            "default-priority helper should exist: `{needle}`."
        );
    }

    let views = [
        load_source("../../components/sidebar/src/view.rs"),
        load_source("../../components/sidebar/src/menu/view.rs"),
        load_source("../../components/sidebar/src/group/view.rs"),
        load_source("../../components/sidebar/src/content/view.rs"),
        load_source("../../components/sidebar/src/footer/view.rs"),
        load_source("../../components/sidebar/src/header/view.rs"),
        load_source("../../components/sidebar/src/inset/view.rs"),
        load_source("../../components/sidebar/src/rail/view.rs"),
        load_source("../../components/sidebar/src/trigger/view.rs"),
        load_source("../../components/sidebar/src/menu_action/view.rs"),
        load_source("../../components/sidebar/src/menu_badge/view.rs"),
    ];

    for view_source in views {
        assert!(
            !view_source.contains(".unwrap_or("),
            "view.rs should not decide default priority via `unwrap_or`; move it into logic.rs.",
        );
        assert!(
            !view_source.contains(".unwrap_or_else("),
            "view.rs should not decide default priority via `unwrap_or_else`; move it into logic.rs.",
        );
    }
}

#[test]
fn sidebar_state_normalization_is_centralized_in_logic_modules() {
    let sidebar_view = load_source("../../components/sidebar/src/view.rs");
    let menu_view = load_source("../../components/sidebar/src/menu/view.rs");
    let group_view = load_source("../../components/sidebar/src/group/view.rs");
    let content_view = load_source("../../components/sidebar/src/content/view.rs");
    let footer_view = load_source("../../components/sidebar/src/footer/view.rs");
    let header_view = load_source("../../components/sidebar/src/header/view.rs");
    let inset_view = load_source("../../components/sidebar/src/inset/view.rs");
    let rail_view = load_source("../../components/sidebar/src/rail/view.rs");
    let trigger_view = load_source("../../components/sidebar/src/trigger/view.rs");
    let menu_action_view = load_source("../../components/sidebar/src/menu_action/view.rs");
    let menu_badge_view = load_source("../../components/sidebar/src/menu_badge/view.rs");
    let group_logic = load_source("../../components/sidebar/src/group/logic.rs");

    for (source, needle) in [
        (
            sidebar_view.as_str(),
            "logic::resolve_state(SidebarStateInput {",
        ),
        (
            menu_view.as_str(),
            "logic::resolve_state(SidebarMenuStateInput {",
        ),
        (
            group_view.as_str(),
            "logic::resolve_state(SidebarGroupStateInput {",
        ),
        (
            content_view.as_str(),
            "logic::resolve_state(SidebarContentStateInput {",
        ),
        (
            footer_view.as_str(),
            "logic::resolve_state(SidebarFooterStateInput {",
        ),
        (
            header_view.as_str(),
            "logic::resolve_state(SidebarHeaderStateInput {",
        ),
        (
            inset_view.as_str(),
            "logic::resolve_state(SidebarInsetStateInput {",
        ),
        (
            rail_view.as_str(),
            "logic::resolve_state(SidebarRailStateInput {",
        ),
        (
            trigger_view.as_str(),
            "logic::resolve_state(SidebarTriggerStateInput {",
        ),
        (
            menu_action_view.as_str(),
            "logic::resolve_state(SidebarMenuActionStateInput {",
        ),
        (
            menu_badge_view.as_str(),
            "logic::resolve_state(SidebarMenuBadgeStateInput {",
        ),
    ] {
        assert!(
            source.contains(needle),
            "state normalization should be derived in logic.rs via `{needle}`.",
        );
    }

    for needle in [
        "pub fn resolve_effective_open(open: bool, collapsible: bool) -> bool",
        "pub fn can_toggle_open(disabled: bool, collapsible: bool) -> bool",
        "pub fn next_toggled_open(open: bool) -> bool",
    ] {
        assert!(
            group_logic.contains(needle),
            "SidebarGroup state/toggle rules should be owned by logic.rs; missing `{needle}`.",
        );
    }

    for needle in [
        "open: logic::resolve_effective_open(open.get(), is_collapsible),",
        "if !logic::can_toggle_open(is_disabled, is_collapsible) {",
        "request_open_change.run(logic::next_toggled_open(open.get_untracked()));",
    ] {
        assert!(
            group_view.contains(needle),
            "SidebarGroup view should consume centralized state rule `{needle}`.",
        );
    }

    for forbidden in [
        "open: if is_collapsible { open.get() } else { true },",
        "if is_disabled || !is_collapsible {",
    ] {
        assert!(
            !group_view.contains(forbidden),
            "SidebarGroup view should not embed state-machine branching `{forbidden}`.",
        );
    }

    assert!(
        menu_view
            .contains("logic::toggle_open_sub_ids(open_sub_ids, &id, items.get_value().as_ref())"),
        "SidebarMenu event path should delegate open-sub derivation to logic helper."
    );
}

#[test]
fn sidebar_discrete_state_axes_are_modeled_by_enums() {
    let sidebar_logic = load_source("../../components/sidebar/src/logic.rs");
    let sidebar_view = load_source("../../components/sidebar/src/view.rs");
    let inset_view = load_source("../../components/sidebar/src/inset/view.rs");
    let rail_view = load_source("../../components/sidebar/src/rail/view.rs");

    for needle in [
        "pub enum SidebarSide",
        "pub enum SidebarVariant",
        "pub enum SidebarCollapsible",
        "pub struct SidebarStateInput",
        "pub side: SidebarSide,",
        "pub variant: SidebarVariant,",
        "pub collapsible: SidebarCollapsible,",
        "match input.side",
        "match input.variant",
        "match input.collapsible",
    ] {
        assert!(
            sidebar_logic.contains(needle),
            "discrete axis typing should be owned by enum contract `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional)] side: SidebarSide,",
        "#[prop(optional)] variant: SidebarVariant,",
        "#[prop(optional)] collapsible: SidebarCollapsible,",
    ] {
        assert!(
            sidebar_view.contains(needle),
            "Sidebar root should expose discrete axes through typed enum prop `{needle}`.",
        );
    }

    assert!(
        inset_view.contains("#[prop(optional)] side: SidebarSide,"),
        "SidebarInset should consume `SidebarSide` enum prop."
    );
    assert!(
        rail_view.contains("#[prop(optional)] side: SidebarSide,"),
        "SidebarRail should consume `SidebarSide` enum prop."
    );

    for forbidden in [
        "side: Option<String>",
        "variant: Option<String>",
        "collapsible: Option<String>",
        "is_sidebar: Option<bool>",
        "is_floating: Option<bool>",
        "is_inset: Option<bool>",
        "is_offcanvas: Option<bool>",
        "is_icon: Option<bool>",
    ] {
        assert!(
            !sidebar_view.contains(forbidden) && !sidebar_logic.contains(forbidden),
            "discrete state axes must not regress to string/multi-bool encodings: `{forbidden}`.",
        );
    }
}

#[test]
fn sidebar_state_primitive_origin_contract_is_respected() {
    let menu_logic = load_source("../../components/sidebar/src/menu/logic.rs");
    let menu_view = load_source("../../components/sidebar/src/menu/view.rs");
    let root_view = load_source("../../components/sidebar/src/view.rs");
    let group_view = load_source("../../components/sidebar/src/group/view.rs");
    let rail_view = load_source("../../components/sidebar/src/rail/view.rs");
    let trigger_view = load_source("../../components/sidebar/src/trigger/view.rs");

    for needle in [
        "use ui_state_primitives::sidebar_menu as primitives;",
        "pub fn default_open_sub_id_set(items: &[SidebarMenuItem]) -> BTreeSet<String> {",
        "primitives::default_open_sub_id_set(items)",
        "pub fn default_active_id(items: &[SidebarMenuItem], requested: Option<String>) -> Option<String> {",
        "primitives::default_active_id(items, requested)",
        "pub fn toggle_open_sub_ids(",
        "primitives::toggle_open_sub_id(open_sub_ids, id, items)",
    ] {
        assert!(
            menu_logic.contains(needle),
            "SidebarMenu logic should consume ui-state-primitives for reusable selection/expansion primitive `{needle}`.",
        );
    }

    for needle in [
        "logic::default_open_sub_id_set(items.as_ref())",
        "logic::default_active_id(items.as_ref(), default_active_id)",
        "logic::toggle_open_sub_ids(open_sub_ids, &id, items.get_value().as_ref())",
        "headless::use_controllable_state(active_id, Some(default_active_id), on_active_id_change)",
    ] {
        assert!(
            menu_view.contains(needle),
            "SidebarMenu view should only assemble primitive outputs and contracts via `{needle}`.",
        );
    }

    for (source, label) in [
        (root_view.as_str(), "Sidebar"),
        (group_view.as_str(), "SidebarGroup"),
        (rail_view.as_str(), "SidebarRail"),
        (trigger_view.as_str(), "SidebarTrigger"),
    ] {
        assert!(
            source.contains("use_controllable_open_state_traced("),
            "{label} should consume shared controllable primitive instead of re-implementing controlled/uncontrolled state machine.",
        );
    }

    for source in [
        menu_logic.as_str(),
        menu_view.as_str(),
        root_view.as_str(),
        group_view.as_str(),
        rail_view.as_str(),
        trigger_view.as_str(),
    ] {
        for forbidden in [
            "AppStore",
            "GlobalStore",
            "BusinessStore",
            "use_app_store(",
            "use_business_store(",
            "use_global_store(",
        ] {
            assert!(
                !source.contains(forbidden),
                "sidebar component should not directly depend on business store type `{forbidden}`.",
            );
        }
    }
}

#[test]
fn sidebar_async_interaction_contract_is_not_applicable() {
    let sources = [
        load_source("../../components/sidebar/src/view.rs"),
        load_source("../../components/sidebar/src/logic.rs"),
        load_source("../../components/sidebar/src/group/view.rs"),
        load_source("../../components/sidebar/src/group/logic.rs"),
        load_source("../../components/sidebar/src/menu/view.rs"),
        load_source("../../components/sidebar/src/menu/logic.rs"),
        load_source("../../components/sidebar/src/content/view.rs"),
        load_source("../../components/sidebar/src/content/logic.rs"),
        load_source("../../components/sidebar/src/footer/view.rs"),
        load_source("../../components/sidebar/src/footer/logic.rs"),
        load_source("../../components/sidebar/src/header/view.rs"),
        load_source("../../components/sidebar/src/header/logic.rs"),
        load_source("../../components/sidebar/src/inset/view.rs"),
        load_source("../../components/sidebar/src/inset/logic.rs"),
        load_source("../../components/sidebar/src/rail/view.rs"),
        load_source("../../components/sidebar/src/rail/logic.rs"),
        load_source("../../components/sidebar/src/trigger/view.rs"),
        load_source("../../components/sidebar/src/trigger/logic.rs"),
        load_source("../../components/sidebar/src/menu_action/view.rs"),
        load_source("../../components/sidebar/src/menu_action/logic.rs"),
        load_source("../../components/sidebar/src/menu_badge/view.rs"),
        load_source("../../components/sidebar/src/menu_badge/logic.rs"),
    ];

    for source in sources {
        for forbidden in [
            "use_async_action(",
            "is_loading",
            "aria-busy",
            "on_retry",
            "data-loading",
            "retry_count",
            "load_error",
        ] {
            assert!(
                !source.contains(forbidden),
                "Sidebar has no async interaction flow; unexpected async protocol marker `{forbidden}` found.",
            );
        }
    }
}

#[test]
fn sidebar_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("../../components/sidebar/src/view.rs");

    for needle in [
        "open: Option<Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "let is_controlled = open.is_some()",
        "headless::use_controllable_open_state_traced(",
    ] {
        assert!(
            source.contains(needle),
            "Sidebar should support `{needle}` for controllable open-state flow.",
        );
    }
}

#[test]
fn sidebar_controllable_axes_are_paired_and_state_updates_follow_control_mode() {
    let sidebar_view = load_source("../../components/sidebar/src/view.rs");
    let group_view = load_source("../../components/sidebar/src/group/view.rs");
    let rail_view = load_source("../../components/sidebar/src/rail/view.rs");
    let trigger_view = load_source("../../components/sidebar/src/trigger/view.rs");
    let menu_view = load_source("../../components/sidebar/src/menu/view.rs");
    let headless_controllable = load_source("../../crates/ui-headless/src/controllable_state.rs");

    for (source, needle) in [
        (sidebar_view.as_str(), "open: Option<Signal<bool>>"),
        (sidebar_view.as_str(), "default_open: Option<bool>"),
        (
            sidebar_view.as_str(),
            "on_open_change: Option<Callback<bool>>",
        ),
        (group_view.as_str(), "open: Option<Signal<bool>>"),
        (group_view.as_str(), "default_open: Option<bool>"),
        (
            group_view.as_str(),
            "on_open_change: Option<Callback<bool>>",
        ),
        (rail_view.as_str(), "open: Option<Signal<bool>>"),
        (rail_view.as_str(), "default_open: Option<bool>"),
        (rail_view.as_str(), "on_open_change: Option<Callback<bool>>"),
        (trigger_view.as_str(), "open: Option<Signal<bool>>"),
        (trigger_view.as_str(), "default_open: Option<bool>"),
        (
            trigger_view.as_str(),
            "on_open_change: Option<Callback<bool>>",
        ),
        (
            menu_view.as_str(),
            "active_id: Option<Signal<Option<String>>>",
        ),
        (menu_view.as_str(), "default_active_id: Option<String>"),
        (
            menu_view.as_str(),
            "on_active_id_change: Option<Callback<Option<String>>>",
        ),
    ] {
        assert!(
            source.contains(needle),
            "controllable axis contract should contain `{needle}`."
        );
    }

    for needle in [
        "let (uncontrolled_value, set_uncontrolled_value) = signal(default_value.unwrap_or_default());",
        "if !is_controlled {",
        "set_uncontrolled_value.set(next);",
    ] {
        assert!(
            headless_controllable.contains(needle),
            "headless controllable primitive should enforce `{needle}` to avoid semi-controlled updates.",
        );
    }
}

#[test]
fn sidebar_emits_baseline_root_state_data_attributes() {
    let source = load_source("../../components/sidebar/src/view.rs");

    for needle in [
        "data-slot=\"sidebar\"",
        "data-side=move || state.get().side_attr",
        "data-variant=move || state.get().variant_attr",
        "data-collapsible=move || state.get().collapsible_attr",
        "data-state=move || state.get().state_attr",
        "data-open=move || state.get().open.then_some(\"true\")",
        "data-disabled=move || state.get().disabled.then_some(\"true\")",
        "data-motion-source=motion_source_attr",
        "data-custom-motion=(motion_source_attr == \"custom\").then_some(\"true\")",
        "data-controlled=move || state.get().is_controlled.then_some(\"true\")",
        "data-controls=move || state.get().control_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Sidebar should expose `{needle}` for stable styling/testing contracts.",
        );
    }
}

#[test]
fn sidebar_styles_include_panel_and_state_markers() {
    let source = load_source("../../components/sidebar/src/styles.rs");

    for needle in [
        ".ui-sidebar {",
        ".ui-sidebar__panel",
        ".ui-sidebar__trigger",
        ".ui-sidebar__rail",
        ".ui-sidebar[data-state=\"closed\"][data-collapsible=\"offcanvas\"] .ui-sidebar__panel",
        ".ui-sidebar[data-state=\"closed\"][data-collapsible=\"icon\"] .ui-sidebar__panel",
        "--ui-sidebar-motion-duration",
        "--ui-sidebar-motion-easing",
        "--ui-sidebar-motion-runtime-duration",
        ".ui-sidebar--disabled",
        ".ui-sidebar--custom-class",
    ] {
        assert!(
            source.contains(needle),
            "Sidebar styles should include `{needle}` marker contracts.",
        );
    }
}

#[test]
fn sidebar_styles_are_theme_token_first() {
    let source = load_source("../../components/sidebar/src/styles.rs");

    for needle in [
        "var(--ui-space-",
        "var(--ui-fallback-space-",
        "var(--ui-radius-",
        "var(--ui-fallback-radius-",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-disabled-opacity,",
        "var(--ui-fallback-disabled-opacity)",
    ] {
        assert!(
            source.contains(needle),
            "Sidebar styles should consume theme token chain `{needle}`.",
        );
    }

    for forbidden in [
        "opacity: 0.62;",
        "var(--ui-bg-canvas, white)",
        "--ui-bg-surface",
        "--ui-border-subtle",
        "--ui-accent-solid",
    ] {
        assert!(
            !source.contains(forbidden),
            "Sidebar styles should not rely on legacy/non-theme token `{forbidden}`.",
        );
    }
}

#[test]
fn sidebar_docs_page_exists_in_layout_extra() {
    let docs =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra/sidebar.rs");

    for needle in [
        "pub(crate) fn sidebar() -> AnyView",
        "<ComponentPage",
        "title=\"Sidebar\"",
        "slug=\"sidebar\"",
        "<Sidebar",
    ] {
        assert!(
            docs.contains(needle),
            "Sidebar docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn sidebar_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra/sidebar.rs");

    for needle in [
        "pub(crate) fn sidebar() -> AnyView",
        r#"title="Sidebar""#,
        r#"slug="sidebar""#,
        r#"description="baseline-compatible sidebar primitive with controlled/uncontrolled open state, side+variant+collapsible contracts, keyboard shortcut toggle, and baseline-style data markers.""#,
        r#"<Playground title="Hello World (Default Sidebar)" code_signal=showcase_code>"#,
        r#"title="Workbench (All API + Actual Config)""#,
        r#"<Playground title="State Matrix (Left / Right / Disabled)" code_signal=matrix_code>"#,
        "<Sidebar",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra sidebar docs should include `{needle}` for sidebar primary playground coverage.",
        );
    }
}

#[test]
fn sidebar_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra/sidebar.rs");

    for needle in [
        r#"title="Hello World (Default Sidebar)""#,
        r#"<Sidebar>"#,
        r#""Dashboard""#,
        r#"title="Workbench (All API + Actual Config)""#,
        "open=workbench_open",
        "default_open=workbench_default_open.get()",
        "on_open_change=on_workbench_open_change",
        "side=workbench_side.get()",
        "variant=workbench_variant.get()",
        "collapsible=workbench_collapsible.get()",
        "disabled=workbench_disabled.get()",
        "show_trigger=workbench_show_trigger.get()",
        "enable_shortcut=workbench_enable_shortcut.get()",
        "shortcut_key=if workbench_custom_shortcut.get() {",
        "trigger_label=if workbench_custom_trigger_label.get() {",
        r#"aria_label="Project navigation sidebar".to_string()"#,
        r#"title="State Matrix (Left / Right / Disabled)""#,
        "side=SidebarSide::Left",
        "variant=SidebarVariant::Sidebar",
        "collapsible=SidebarCollapsible::Offcanvas",
        "side=SidebarSide::Right",
        "variant=SidebarVariant::Floating",
        "collapsible=SidebarCollapsible::Icon",
        "show_trigger=false",
        "variant=SidebarVariant::Inset",
        "collapsible=SidebarCollapsible::None",
        "disabled=true",
        "enable_shortcut=false",
        r#"trigger_label="Disabled".to_string()"#,
        r#"aria_label="Disabled sidebar".to_string()"#,
    ] {
        assert!(
            source.contains(needle),
            "layout_extra sidebar playgrounds should contain `{needle}` for sidebar contracts.",
        );
    }
}

#[test]
fn sidebar_dx_paradox_api_is_simple_and_docs_have_minimal_path() {
    let sidebar_view = load_source("../../components/sidebar/src/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra/sidebar.rs");
    let signature = sidebar_view
        .split(") -> impl IntoView")
        .next()
        .unwrap_or_default();

    for needle in [
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            signature.contains(needle),
            "Sidebar basic API should expose `{needle}` for uncontrolled/controlled ease-of-use.",
        );
    }

    for forbidden in [
        "#[prop(optional)] state:",
        "state: Signal<",
        "state: ReadSignal<",
        "state: RwSignal<",
    ] {
        assert!(
            !signature.contains(forbidden),
            "Sidebar basic API must not require internal state object `{forbidden}`.",
        );
    }

    for needle in [
        r#"<Playground title="Hello World (Default Sidebar)" code_signal=showcase_code>"#,
        r#"<Sidebar>
  <div class="ui-sidebar__content"><span>"Dashboard"</span></div>
</Sidebar>"#,
        r#"title="Workbench (All API + Actual Config)""#,
    ] {
        assert!(
            docs_source.contains(needle),
            "Sidebar docs should include DX-paradox proof marker `{needle}`.",
        );
    }

    for forbidden in ["ui_state_primitives", "ui_headless", "state="] {
        assert!(
            !docs_source.contains(forbidden),
            "Sidebar hello-world docs path should not expose low-level wiring marker `{forbidden}`.",
        );
    }
}

#[test]
fn sidebar_composite_api_prefers_explicit_composition_or_typed_itemspec() {
    let sidebar_view = load_source("../../components/sidebar/src/view.rs");
    let menu_view = load_source("../../components/sidebar/src/menu/view.rs");
    let menu_logic = load_source("../../components/sidebar/src/menu/logic.rs");
    let menu_primitives = load_source("../../crates/ui-state-primitives/src/sidebar_menu.rs");
    let sidebar_docs =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra/sidebar.rs");
    let sidebar_menu_docs =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra/sidebar_menu.rs");

    assert!(
        sidebar_view.contains("pub fn Sidebar(") && sidebar_view.contains("children: Children,"),
        "Sidebar composite root API should prefer explicit composition via `children: Children`.",
    );
    assert!(
        sidebar_docs.contains("<Sidebar>"),
        "Sidebar docs should show explicit parent-child composition path.",
    );

    assert!(
        menu_view.contains("pub fn SidebarMenu(")
            && menu_view.contains("items: Vec<SidebarMenuItem>,"),
        "SidebarMenu config input should be typed `Vec<SidebarMenuItem>` item-spec.",
    );
    assert!(
        menu_logic.contains("pub use primitives::{SidebarMenuItem, SidebarMenuSubItem};"),
        "SidebarMenu item-spec should come from shared primitives and stay strongly typed.",
    );

    for needle in [
        "pub struct SidebarMenuItem {",
        "pub id: String,",
        "pub label: String,",
        "pub sub_items: Vec<SidebarMenuSubItem>,",
        "pub struct SidebarMenuSubItem {",
    ] {
        assert!(
            menu_primitives.contains(needle),
            "typed item-spec contract should include `{needle}`.",
        );
    }

    for forbidden in [
        "labels:",
        "titles:",
        "panels:",
        "labels=",
        "titles=",
        "panels=",
        "items_labels",
        "items_titles",
    ] {
        assert!(
            !menu_view.contains(forbidden)
                && !menu_logic.contains(forbidden)
                && !sidebar_menu_docs.contains(forbidden),
            "Sidebar composite API must not regress to parallel-array convention `{forbidden}`.",
        );
    }

    for needle in ["SidebarMenuItem {", "SidebarMenuSubItem {"] {
        assert!(
            sidebar_menu_docs.contains(needle),
            "SidebarMenu docs should build examples from typed item-spec `{needle}`.",
        );
    }
}

#[test]
fn sidebar_macro_micro_duality_is_not_applicable_without_drag_interaction() {
    let sources = [
        load_source("../../components/sidebar/src/view.rs"),
        load_source("../../components/sidebar/src/logic.rs"),
        load_source("../../components/sidebar/src/group/view.rs"),
        load_source("../../components/sidebar/src/group/logic.rs"),
        load_source("../../components/sidebar/src/menu/view.rs"),
        load_source("../../components/sidebar/src/menu/logic.rs"),
        load_source("../../components/sidebar/src/rail/view.rs"),
        load_source("../../components/sidebar/src/rail/logic.rs"),
        load_source("../../components/sidebar/src/trigger/view.rs"),
        load_source("../../components/sidebar/src/trigger/logic.rs"),
        load_source("../../components/sidebar/src/motion.rs"),
        load_source("../../components/sidebar/src/trigger/motion.rs"),
        load_source("../../components/sidebar/src/footer/motion.rs"),
    ];

    for source in sources {
        for forbidden in [
            "on:pointermove",
            "on:mousemove",
            "on:touchmove",
            "on:drag",
            "DragEnd",
            "drag_state",
            "data-dragging",
            "requestAnimationFrame(",
            "Action::DragEnd",
        ] {
            assert!(
                !source.contains(forbidden),
                "Sidebar has no drag interaction axis; unexpected macro/micro drag marker `{forbidden}` found.",
            );
        }
    }
}

#[test]
fn sidebar_two_pass_rendering_is_n_a_for_overlay_geometry_with_idempotent_visual_measurement() {
    let menu_view = load_source("../../components/sidebar/src/menu/view.rs");
    let menu_logic = load_source("../../components/sidebar/src/menu/logic.rs");
    let root_view = load_source("../../components/sidebar/src/view.rs");
    let root_logic = load_source("../../components/sidebar/src/logic.rs");
    let highlight_primitive =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");

    for needle in [
        "let legend_ref: NodeRef<html::Div> = NodeRef::new();",
        "let highlight_ref: NodeRef<html::Div> = NodeRef::new();",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            menu_view.contains(needle),
            "SidebarMenu visual measurement path should be explicitly delegated via `{needle}`.",
        );
    }

    for forbidden in [
        "getBoundingClientRect",
        "get_bounding_client_rect",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "ResizeObserver",
    ] {
        assert!(
            !menu_view.contains(forbidden)
                && !menu_logic.contains(forbidden)
                && !root_view.contains(forbidden)
                && !root_logic.contains(forbidden),
            "Sidebar component layer should not implement geometry measurement primitive `{forbidden}` directly.",
        );
    }

    for needle in [
        "pub fn attach_active_highlight_motion(",
        "fn sync_measured_layout(&mut self)",
        "let unchanged =",
        "if unchanged {",
    ] {
        assert!(
            highlight_primitive.contains(needle),
            "shared visual primitive should provide idempotent measurement convergence marker `{needle}`.",
        );
    }
}

#[test]
fn sidebar_registration_protocol_is_n_a_and_navigation_order_is_vec_based() {
    let menu_view = load_source("../../components/sidebar/src/menu/view.rs");
    let menu_logic = load_source("../../components/sidebar/src/menu/logic.rs");
    let menu_primitives = load_source("../../crates/ui-state-primitives/src/sidebar_menu.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !menu_view.contains(forbidden)
                && !menu_logic.contains(forbidden)
                && !menu_primitives.contains(forbidden),
            "SidebarMenu should not depend on dynamic registration protocol marker `{forbidden}`.",
        );
    }

    for needle in [
        "items: Vec<SidebarMenuItem>,",
        ".iter()",
        ".enumerate()",
        "logic::next_enabled_id(items.get_value().as_ref(), active_id.get(), 1)",
        "logic::next_enabled_id(items.get_value().as_ref(), active_id.get(), -1)",
    ] {
        assert!(
            menu_view.contains(needle),
            "SidebarMenu navigation/render order should be driven by Vec sequence marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn linear_enabled_ids(items: &[SidebarMenuItem]) -> Vec<String> {",
        "for item in items {",
        "for sub_item in &item.sub_items {",
        "pub fn next_enabled_id(",
        "let linear_ids = linear_enabled_ids(items);",
    ] {
        assert!(
            menu_primitives.contains(needle),
            "shared primitive should derive navigation order from item Vec traversal `{needle}`.",
        );
    }
}

#[test]
fn sidebar_slot_projection_policy_is_n_a_without_keepalive_lifecycle_contract() {
    let root_view = load_source("../../components/sidebar/src/view.rs");
    let content_view = load_source("../../components/sidebar/src/content/view.rs");
    let group_view = load_source("../../components/sidebar/src/group/view.rs");
    let menu_view = load_source("../../components/sidebar/src/menu/view.rs");
    let root_logic = load_source("../../components/sidebar/src/logic.rs");
    let group_logic = load_source("../../components/sidebar/src/group/logic.rs");
    let menu_logic = load_source("../../components/sidebar/src/menu/logic.rs");

    for source in [
        root_view.as_str(),
        content_view.as_str(),
        group_view.as_str(),
        menu_view.as_str(),
        root_logic.as_str(),
        group_logic.as_str(),
        menu_logic.as_str(),
    ] {
        for forbidden in [
            "KeepAlive",
            "Lazy",
            "Eager",
            "NotifyHidden",
            "notify_hidden",
            "keep_alive",
            "projection_mode",
            "slot_projection",
            "pause_polling",
            "resume_polling",
        ] {
            assert!(
                !source.contains(forbidden),
                "Sidebar slot projection contract is N/A; unexpected lifecycle/projection marker `{forbidden}` found.",
            );
        }
    }

    assert!(
        group_view.contains("hidden=move || !state.get().open"),
        "SidebarGroup may toggle plain visibility via `hidden` attribute."
    );
    assert!(
        !group_view.contains("NotifyHidden")
            && !group_view.contains("on_hidden")
            && !group_view.contains("on:transitionend"),
        "SidebarGroup hidden path should remain a simple visibility toggle, not KeepAlive lifecycle notification."
    );
}

#[test]
fn sidebar_env_streams_are_not_modeled_in_component_layer() {
    let root_view = load_source("../../components/sidebar/src/view.rs");
    let menu_view = load_source("../../components/sidebar/src/menu/view.rs");
    let group_view = load_source("../../components/sidebar/src/group/view.rs");
    let content_view = load_source("../../components/sidebar/src/content/view.rs");
    let root_logic = load_source("../../components/sidebar/src/logic.rs");
    let menu_logic = load_source("../../components/sidebar/src/menu/logic.rs");

    for source in [
        root_view.as_str(),
        menu_view.as_str(),
        group_view.as_str(),
        content_view.as_str(),
        root_logic.as_str(),
        menu_logic.as_str(),
    ] {
        for forbidden in [
            "on:resize",
            "on:scroll",
            "on:intersection",
            "ResizeObserver",
            "IntersectionObserver",
            "matchMedia",
            "window().",
            "BreakpointChanged",
            "debounce",
            "throttle",
            "set_interval",
            "set_timeout",
        ] {
            assert!(
                !source.contains(forbidden),
                "Sidebar component layer should not implement raw env-stream plumbing `{forbidden}`.",
            );
        }
    }

    assert!(
        menu_view.contains("attach_active_highlight_motion("),
        "SidebarMenu should delegate visual resize reaction to shared visual primitive attach API.",
    );
    assert!(
        menu_view.contains("logic::active_index_for_current("),
        "SidebarMenu should map derived view updates through logic helper instead of env event flooding.",
    );
}

#[test]
fn sidebar_event_light_cone_is_n_a_without_large_collection_bus_selector_contract() {
    let menu_view = load_source("../../components/sidebar/src/menu/view.rs");
    let menu_logic = load_source("../../components/sidebar/src/menu/logic.rs");
    let menu_primitives = load_source("../../crates/ui-state-primitives/src/sidebar_menu.rs");
    let docs_menu =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra/sidebar_menu.rs");

    for source in [
        menu_view.as_str(),
        menu_logic.as_str(),
        menu_primitives.as_str(),
        docs_menu.as_str(),
    ] {
        for forbidden in [
            "ContextBus",
            "Context Bus",
            "Selector",
            "SelectionState",
            "SelectionState::All",
            "select_all",
            "bulk_select",
            "batch_action",
            "provide_context(",
            "use_context(",
        ] {
            assert!(
                !source.contains(forbidden),
                "SidebarMenu should not expose large-collection bus/selector contract marker `{forbidden}`.",
            );
        }
    }

    for needle in [
        "items: Vec<SidebarMenuItem>,",
        "items.get_value().iter().enumerate()",
        "logic::next_enabled_id(items.get_value().as_ref(), active_id.get(), 1)",
        "logic::next_enabled_id(items.get_value().as_ref(), active_id.get(), -1)",
    ] {
        assert!(
            menu_view.contains(needle),
            "SidebarMenu interaction path should remain per-item Vec traversal `{needle}` for current scale.",
        );
    }

    assert!(
        menu_primitives
            .contains("pub fn linear_enabled_ids(items: &[SidebarMenuItem]) -> Vec<String> {")
            && menu_primitives.contains("for item in items {"),
        "SidebarMenu primitive navigation should remain Vec-linear rather than bus-compressed state for this component scope.",
    );
}

#[test]
fn sidebar_causality_bus_is_n_a_without_trace_id_pipeline() {
    let root_view = load_source("../../components/sidebar/src/view.rs");
    let menu_view = load_source("../../components/sidebar/src/menu/view.rs");
    let group_view = load_source("../../components/sidebar/src/group/view.rs");
    let menu_logic = load_source("../../components/sidebar/src/menu/logic.rs");
    let root_logic = load_source("../../components/sidebar/src/logic.rs");

    for source in [
        root_view.as_str(),
        menu_view.as_str(),
        group_view.as_str(),
        menu_logic.as_str(),
        root_logic.as_str(),
    ] {
        for forbidden in [
            "TraceId",
            "trace_id",
            "CausalityBus",
            "causality_bus",
            "event_bus",
            "command_bus",
            "publish(",
            "broadcast(",
            "subscribe(",
        ] {
            assert!(
                !source.contains(forbidden),
                "Sidebar should not expose causality bus marker `{forbidden}` for current component scope.",
            );
        }
    }

    for needle in [
        "if let Some(callback) = on_action.get_value() {",
        "callback.run(id);",
        "if let Some(callback) = on_item_action.get_value() {",
        "if let Some(on_action) = on_action.get_value() {",
        "on_action.run(());",
        "request_open_change.run(!open.get_untracked());",
    ] {
        assert!(
            menu_view.contains(needle) || group_view.contains(needle) || root_view.contains(needle),
            "Sidebar causality path should remain direct callback dispatch via `{needle}`.",
        );
    }
}

#[test]
fn sidebar_a11y_i18n_contract_is_headless_driven_and_text_is_overrideable() {
    let root_view = load_source("../../components/sidebar/src/view.rs");
    let group_view = load_source("../../components/sidebar/src/group/view.rs");
    let group_logic = load_source("../../components/sidebar/src/group/logic.rs");
    let menu_view = load_source("../../components/sidebar/src/menu/view.rs");
    let menu_logic = load_source("../../components/sidebar/src/menu/logic.rs");
    let headless_a11y = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "headless::use_sidebar_root(",
        "headless::sidebar_toggle_button_a11y_attrs(",
        "role=root_role",
        "aria-label=root_aria_label",
        "lang=root_lang",
        "dir=root_dir",
    ] {
        assert!(
            root_view.contains(needle),
            "Sidebar root should mount headless a11y contract `{needle}`.",
        );
    }

    for needle in [
        "headless::navigation_attrs(",
        "role=\"navigation\"",
        "aria-label=nav_aria_label",
        "lang=nav_lang",
        "dir=nav_dir",
        "#[prop(optional, into)] submenu_toggle_label: Option<String>",
        "logic::normalize_submenu_toggle_label(submenu_toggle_label)",
    ] {
        assert!(
            menu_view.contains(needle),
            "SidebarMenu should expose localized a11y/text override path `{needle}`.",
        );
    }

    for needle in [
        "overlay_open::labeled_group_attrs(aria_label.clone(), lang, dir)",
        "#[prop(optional, into)] toggle_label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "logic::normalize_toggle_label(toggle_label)",
        "lang=move || group_lang.get_value()",
        "dir=move || group_dir.get_value()",
    ] {
        assert!(
            group_view.contains(needle),
            "SidebarGroup should consume shared headless a11y locale/group contract `{needle}`.",
        );
    }

    for forbidden in [
        r#"aria-label="toggle submenu""#,
        r#"aria-label="Toggle group""#,
        r#""toggle sidebar""#,
    ] {
        assert!(
            !root_view.contains(forbidden)
                && !menu_view.contains(forbidden)
                && !group_view.contains(forbidden),
            "Sidebar view layer should not hardcode user-visible/a11y label `{forbidden}`.",
        );
    }

    for needle in [
        "pub fn normalize_toggle_label(value: Option<String>) -> String {",
        "pub fn normalize_submenu_toggle_label(value: Option<String>) -> String {",
    ] {
        assert!(
            group_logic.contains(needle) || menu_logic.contains(needle),
            "Sidebar logic layer should provide fallback+override normalization `{needle}`.",
        );
    }

    for needle in [
        "pub fn labeled_group_attrs(",
        "pub fn navigation_attrs(",
        "pub fn locale_attrs(",
    ] {
        assert!(
            headless_a11y.contains(needle),
            "ui-headless shared a11y toolkit should expose `{needle}` for component reuse.",
        );
    }
}

#[test]
fn sidebar_state_markers_are_observable_queryable_and_closed_set() {
    let root_view = load_source("../../components/sidebar/src/view.rs");
    let menu_view = load_source("../../components/sidebar/src/menu/view.rs");
    let trigger_view = load_source("../../components/sidebar/src/trigger/view.rs");
    let rail_view = load_source("../../components/sidebar/src/rail/view.rs");
    let group_view = load_source("../../components/sidebar/src/group/view.rs");
    let menu_styles = load_source("../../components/sidebar/src/menu/styles.rs");

    for needle in [
        "data-state=move || state.get().state_attr",
        "data-open=move || state.get().open.then_some(\"true\")",
        "data-closed=move || state.get().closed.then_some(\"true\")",
        "data-disabled=move || state.get().disabled.then_some(\"true\")",
        "data-controlled=move || state.get().is_controlled.then_some(\"true\")",
        "data-uncontrolled=move || state.get().is_uncontrolled.then_some(\"true\")",
        "data-controls=move || state.get().control_attr",
        "data-shortcut-source=root_shortcut_source",
        "data-has-shortcut=move || shortcut_key.get_value().as_ref().map(|_| \"true\")",
        "aria-expanded=move || trigger_aria_expanded.get()",
        "aria-expanded=move || rail_aria_expanded.get()",
    ] {
        assert!(
            root_view.contains(needle),
            "Sidebar root should expose stable state/source/a11y marker `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || state.get().state_attr",
        "data-selection=move || logic::selection_state_attr(active_id.get())",
        "data-has-selection=move || active_id.get().as_ref().map(|_| \"true\")",
        "data-control-mode=move || state.get().control_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-shortcut-source=menu_keyboard_shortcut_source",
        "data-active=item_active",
        "aria-current=item_aria_current",
    ] {
        assert!(
            menu_view.contains(needle),
            "SidebarMenu should expose stable state/source marker `{needle}`.",
        );
    }

    for needle in [
        "data-control-mode=move || state.get().control_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "aria-expanded=move || if state.get().open { \"true\" } else { \"false\" }",
    ] {
        assert!(
            trigger_view.contains(needle)
                || rail_view.contains(needle)
                || group_view.contains(needle),
            "Interactive sidebar controls should expose source/expanded semantics via `{needle}`.",
        );
    }

    for forbidden in ["data-shortcut=move ||", "data-active-id=move ||"] {
        assert!(
            !root_view.contains(forbidden) && !menu_view.contains(forbidden),
            "State marker should avoid free-text drift field `{forbidden}`.",
        );
    }

    for needle in [
        ".ui-sidebar-menu__button:focus-visible",
        ".ui-sidebar-menu__sub-button:focus-visible",
        ".ui-sidebar-menu__action:focus-visible",
        ".ui-sidebar-menu__toggle:focus-visible",
    ] {
        assert!(
            menu_styles.contains(needle),
            "Focus-visible interaction should stay queryable by stable selector `{needle}`.",
        );
    }
}

#[test]
fn sidebar_styles_use_explicit_state_markers_and_css_var_runtime_styles_only() {
    let root_styles = load_source("../../components/sidebar/src/styles.rs");
    let content_styles = load_source("../../components/sidebar/src/content/styles.rs");
    let footer_styles = load_source("../../components/sidebar/src/footer/styles.rs");
    let group_styles = load_source("../../components/sidebar/src/group/styles.rs");
    let header_styles = load_source("../../components/sidebar/src/header/styles.rs");
    let inset_styles = load_source("../../components/sidebar/src/inset/styles.rs");
    let menu_styles = load_source("../../components/sidebar/src/menu/styles.rs");
    let menu_action_styles = load_source("../../components/sidebar/src/menu_action/styles.rs");
    let menu_badge_styles = load_source("../../components/sidebar/src/menu_badge/styles.rs");
    let rail_styles = load_source("../../components/sidebar/src/rail/styles.rs");
    let trigger_styles = load_source("../../components/sidebar/src/trigger/styles.rs");
    let root_view = load_source("../../components/sidebar/src/view.rs");
    let menu_view = load_source("../../components/sidebar/src/menu/view.rs");
    let group_view = load_source("../../components/sidebar/src/group/view.rs");
    let content_view = load_source("../../components/sidebar/src/content/view.rs");
    let footer_view = load_source("../../components/sidebar/src/footer/view.rs");
    let header_view = load_source("../../components/sidebar/src/header/view.rs");
    let inset_view = load_source("../../components/sidebar/src/inset/view.rs");
    let rail_view = load_source("../../components/sidebar/src/rail/view.rs");
    let trigger_view = load_source("../../components/sidebar/src/trigger/view.rs");
    let menu_action_view = load_source("../../components/sidebar/src/menu_action/view.rs");
    let menu_badge_view = load_source("../../components/sidebar/src/menu_badge/view.rs");
    let motion_logic = load_source("../../components/sidebar/src/motion.rs");

    for source in [
        root_styles.as_str(),
        content_styles.as_str(),
        footer_styles.as_str(),
        group_styles.as_str(),
        header_styles.as_str(),
        inset_styles.as_str(),
        menu_styles.as_str(),
        menu_action_styles.as_str(),
        menu_badge_styles.as_str(),
        rail_styles.as_str(),
        trigger_styles.as_str(),
    ] {
        for forbidden in [":nth-child(", ":nth-of-type(", ":has("] {
            assert!(
                !source.contains(forbidden),
                "Sidebar styles should not infer state via fragile structural selector `{forbidden}`.",
            );
        }
    }

    for needle in [
        ".ui-sidebar[data-state=\"closed\"][data-collapsible=\"offcanvas\"] .ui-sidebar__panel",
        ".ui-sidebar-group[data-state=\"closed\"] .ui-sidebar-group__content",
        ".ui-sidebar-menu__item[data-active=\"true\"] .ui-sidebar-menu__button",
        ".ui-sidebar-menu__sub-button[data-active=\"true\"]",
        ".ui-sidebar-rail[data-closed=\"true\"]",
        ".ui-sidebar-trigger[data-open=\"true\"]",
    ] {
        assert!(
            root_styles.contains(needle)
                || group_styles.contains(needle)
                || menu_styles.contains(needle)
                || rail_styles.contains(needle)
                || trigger_styles.contains(needle),
            "Sidebar visual state branches should be driven by explicit marker selector `{needle}`.",
        );
    }

    let style_bindings = [
        root_view.as_str(),
        menu_view.as_str(),
        group_view.as_str(),
        content_view.as_str(),
        footer_view.as_str(),
        header_view.as_str(),
        inset_view.as_str(),
        rail_view.as_str(),
        trigger_view.as_str(),
        menu_action_view.as_str(),
        menu_badge_view.as_str(),
    ]
    .iter()
    .map(|source| source.matches("style=").count())
    .sum::<usize>();

    assert_eq!(
        style_bindings, 1,
        "Sidebar component views should keep runtime style bindings minimal and centralized.",
    );
    assert!(
        root_view.contains("style=move || motion_style.get_value()"),
        "Sidebar root should use a single runtime style binding for motion CSS variables.",
    );

    for forbidden in ["style:top=", "style:left=", "style:width=", "style:height="] {
        for source in [
            root_view.as_str(),
            menu_view.as_str(),
            group_view.as_str(),
            content_view.as_str(),
            footer_view.as_str(),
            header_view.as_str(),
            inset_view.as_str(),
            rail_view.as_str(),
            trigger_view.as_str(),
            menu_action_view.as_str(),
            menu_badge_view.as_str(),
        ] {
            assert!(
                !source.contains(forbidden),
                "Sidebar view should avoid inline business styling `{forbidden}`.",
            );
        }
    }

    for needle in [
        "--ui-sidebar-motion-duration:",
        "--ui-sidebar-motion-reduced-duration:",
        "--ui-sidebar-motion-runtime-duration:",
    ] {
        assert!(
            motion_logic.contains(needle),
            "Sidebar motion runtime style should pass CSS custom properties `{needle}`.",
        );
    }
    for forbidden in [
        "top:",
        "left:",
        "width:",
        "height:",
        "display:",
        "position:",
    ] {
        assert!(
            !motion_logic.contains(forbidden),
            "Sidebar motion attachment should avoid business CSS declaration `{forbidden}`.",
        );
    }
}

#[test]
fn sidebar_semantic_tests_cover_contract_matrix_and_do_not_depend_on_snapshot_only() {
    let root_view = load_source("../../components/sidebar/src/view.rs");
    let menu_view = load_source("../../components/sidebar/src/menu/view.rs");
    let semantics_test = load_source("../../components/sidebar/test/semantics.rs");
    let menu_semantics_test =
        load_source("../../components/sidebar/test/sidebar_menu_semantics.rs");
    let group_semantics_test =
        load_source("../../components/sidebar/test/sidebar_group_semantics.rs");
    let motion_test = load_source("../../components/sidebar/test/motion.rs");

    for needle in [
        "role=root_role",
        "aria-label=root_aria_label",
        "data-state=move || state.get().state_attr",
        "data-controlled=move || state.get().is_controlled.then_some(\"true\")",
        "data-disabled=move || state.get().disabled.then_some(\"true\")",
    ] {
        assert!(
            root_view.contains(needle),
            "sidebar semantic contract marker should be present in root view `{needle}`.",
        );
    }

    for needle in [
        "role=\"navigation\"",
        "aria-label=nav_aria_label",
        "on:keydown=on_key_down",
        "on:pointerenter=on_pointer_enter",
        "data-shortcut-source=menu_keyboard_shortcut_source",
    ] {
        assert!(
            menu_view.contains(needle),
            "sidebar menu should expose semantic interaction path `{needle}`.",
        );
    }

    for needle in [
        "fn sidebar_supports_controlled_and_uncontrolled_open_state()",
        "fn sidebar_controllable_axes_are_paired_and_state_updates_follow_control_mode()",
        "fn sidebar_state_markers_are_observable_queryable_and_closed_set()",
        "fn sidebar_a11y_i18n_contract_is_headless_driven_and_text_is_overrideable()",
    ] {
        assert!(
            semantics_test.contains(needle),
            "root semantics test matrix should include `{needle}`.",
        );
    }

    for needle in [
        "fn sidebar_menu_supports_controlled_and_uncontrolled_active_state()",
        "fn sidebar_group_supports_controlled_and_uncontrolled_open_state()",
        "data-disabled=move || state.get().disabled.then_some(\"true\")",
        "data-state=move || state.get().state_attr",
    ] {
        assert!(
            menu_semantics_test.contains(needle) || group_semantics_test.contains(needle),
            "sidebar semantic suite should keep branch/state marker coverage `{needle}`.",
        );
    }

    assert!(
        motion_test.contains("if cfg!(target_arch = \"wasm32\") { 320 } else { 8 }"),
        "sidebar test matrix should include wasm/non-wasm semantic contract branch coverage.",
    );

    for source in [
        semantics_test.as_str(),
        menu_semantics_test.as_str(),
        group_semantics_test.as_str(),
    ] {
        for forbidden in [
            concat!("assert_", "snapshot!"),
            concat!("to_match_", "snapshot"),
            concat!("in", "sta::"),
            concat!("gol", "den"),
            concat!("visual ", "snapshot"),
        ] {
            assert!(
                !source.contains(forbidden),
                "sidebar semantics verification should not rely on snapshot-only assertion `{forbidden}`.",
            );
        }
    }
}

#[test]
fn sidebar_component_file_responsibilities_are_separated_by_layer() {
    let module_source = load_source("../../components/sidebar/src/mod.rs");
    let logic_source = load_source("../../components/sidebar/src/logic.rs");
    let styles_source = load_source("../../components/sidebar/src/styles.rs");
    let view_source = load_source("../../components/sidebar/src/view.rs");
    let motion_source = load_source("../../components/sidebar/src/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{SidebarCollapsible, SidebarSide, SidebarVariant};",
        "pub use motion::SidebarMotion;",
        "pub use view::Sidebar;",
    ] {
        assert!(
            module_source.contains(needle),
            "sidebar mod boundary should expose only module/export surface `{needle}`.",
        );
    }

    for forbidden in [
        "pub const DEFAULT_ARIA_LABEL",
        "pub const DEFAULT_SHORTCUT_KEY",
        "pub fn ",
        "impl ",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "sidebar mod.rs should not carry implementation detail `{forbidden}`.",
        );
    }

    for needle in [
        "pub fn normalize_aria_label(",
        "pub fn normalize_trigger_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub struct SidebarStateInput",
        "pub struct SidebarState",
    ] {
        assert!(
            logic_source.contains(needle),
            "sidebar logic layer should own normalization/derivation contract `{needle}`.",
        );
    }

    for forbidden in [
        "view! {",
        "on:click=",
        "on:keydown=",
        "<aside",
        ".ui-sidebar",
        "ui_headless::",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "sidebar logic.rs should avoid view/css/headless implementation detail `{forbidden}`.",
        );
    }

    for needle in [
        "var(--ui-space-",
        "var(--ui-fallback-space-",
        ".ui-sidebar[data-state=",
        ".ui-sidebar__panel",
    ] {
        assert!(
            styles_source.contains(needle),
            "sidebar styles.rs should keep token-first static CSS contract `{needle}`.",
        );
    }

    for forbidden in [
        "Toggle sidebar",
        "use_sidebar_root(",
        "on:click=",
        "request_open_change",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "sidebar styles.rs should not carry behavior/text implementation `{forbidden}`.",
        );
    }

    for needle in [
        "logic::resolve_state(SidebarStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "headless::use_sidebar_root(",
        "headless::sidebar_toggle_button_a11y_attrs(",
        "view! {",
    ] {
        assert!(
            view_source.contains(needle),
            "sidebar view.rs should render structure and mount logic/headless contract `{needle}`.",
        );
    }

    for forbidden in [
        "match side",
        "match variant",
        "match collapsible",
        "pub fn resolve_state(",
        "pub struct SidebarState",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "sidebar view.rs should not re-implement state derivation `{forbidden}`.",
        );
    }

    for needle in [
        "pub fn sanitize_motion(",
        "pub fn source_attr(",
        "pub fn attach_motion(",
        "ui_motion::web::prefers_reduced_motion()",
        "--ui-sidebar-motion-duration:",
    ] {
        assert!(
            motion_source.contains(needle),
            "sidebar motion.rs should map semantic motion contract via `{needle}`.",
        );
    }

    for forbidden in [
        "view! {",
        "on:click=",
        "fn spring_solver",
        "requestAnimationFrame",
        "web_sys::",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "sidebar motion.rs should not embed view/event or custom animation engine `{forbidden}`.",
        );
    }
}

#[test]
fn sidebar_spec_rs_is_not_introduced_without_schema_contract_need() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let sidebar_src = manifest_dir.join("../../components/sidebar/src");
    let mut stack = vec![sidebar_src.clone()];
    let mut spec_paths = Vec::new();

    while let Some(dir) = stack.pop() {
        let entries =
            fs::read_dir(&dir).unwrap_or_else(|e| panic!("read_dir failed for {dir:?}: {e}"));
        for entry in entries {
            let entry = entry.unwrap_or_else(|e| panic!("read_dir entry failed in {dir:?}: {e}"));
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.file_name().and_then(|value| value.to_str()) == Some("spec.rs") {
                spec_paths.push(path);
            }
        }
    }

    assert!(
        spec_paths.is_empty(),
        "sidebar should not introduce spec.rs without stable schema contract need; found {spec_paths:?}",
    );

    let module_source = load_source("../../components/sidebar/src/mod.rs");
    let view_source = load_source("../../components/sidebar/src/view.rs");
    let docs_source = load_source("../../components/sidebar/check2.md");

    for source in [module_source.as_str(), view_source.as_str()] {
        for forbidden in [
            "mod spec;",
            "pub mod spec;",
            "pub use spec::",
            "SidebarSpec",
        ] {
            assert!(
                !source.contains(forbidden),
                "sidebar simple component surface should not expose spec builder contract `{forbidden}`.",
            );
        }
    }

    assert!(
        docs_source.contains("`spec.rs` 只用于少数复杂组件（如 button），避免泛滥。"),
        "sidebar checklist should preserve spec.rs scope note for simple component governance.",
    );
}

#[test]
fn sidebar_styles_follow_token_first_contract_and_ui_root_css_injection_path() {
    let ui_css_source = load_source("../../crates/ui/src/css.rs");
    let ui_root_source = load_source("../../crates/ui/src/root.rs");
    let root_styles = load_source("../../components/sidebar/src/styles.rs");
    let content_styles = load_source("../../components/sidebar/src/content/styles.rs");
    let footer_styles = load_source("../../components/sidebar/src/footer/styles.rs");
    let group_styles = load_source("../../components/sidebar/src/group/styles.rs");
    let header_styles = load_source("../../components/sidebar/src/header/styles.rs");
    let inset_styles = load_source("../../components/sidebar/src/inset/styles.rs");
    let menu_styles = load_source("../../components/sidebar/src/menu/styles.rs");
    let menu_action_styles = load_source("../../components/sidebar/src/menu_action/styles.rs");
    let menu_badge_styles = load_source("../../components/sidebar/src/menu_badge/styles.rs");
    let rail_styles = load_source("../../components/sidebar/src/rail/styles.rs");
    let trigger_styles = load_source("../../components/sidebar/src/trigger/styles.rs");
    let motion_source = load_source("../../components/sidebar/src/motion.rs");
    let view_source = load_source("../../components/sidebar/src/view.rs");

    for needle in [
        "#[cfg(feature = \"component-sidebar\")]",
        "out.push_str(crate::sidebar::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_content\")]",
        "out.push_str(crate::sidebar_content::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_footer\")]",
        "out.push_str(crate::sidebar_footer::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_header\")]",
        "out.push_str(crate::sidebar_header::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_inset\")]",
        "out.push_str(crate::sidebar_inset::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_group\")]",
        "out.push_str(crate::sidebar::group::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_menu\")]",
        "out.push_str(crate::sidebar_menu::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_menu_action\")]",
        "out.push_str(crate::sidebar_menu_action::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_menu_badge\")]",
        "out.push_str(crate::sidebar_menu_badge::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_rail\")]",
        "out.push_str(crate::sidebar_rail::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_trigger\")]",
        "out.push_str(crate::sidebar_trigger::styles::CSS);",
    ] {
        assert!(
            ui_css_source.contains(needle),
            "ui css aggregation should include sidebar styles contract `{needle}`.",
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "UiRoot should inject aggregated component css via `{needle}`.",
        );
    }

    for source in [
        root_styles.as_str(),
        content_styles.as_str(),
        footer_styles.as_str(),
        group_styles.as_str(),
        header_styles.as_str(),
        inset_styles.as_str(),
        menu_styles.as_str(),
        menu_action_styles.as_str(),
        menu_badge_styles.as_str(),
        rail_styles.as_str(),
        trigger_styles.as_str(),
    ] {
        assert!(
            source.contains("var(--ui-"),
            "sidebar styles should use ui token variables (`var(--ui-*)`).",
        );
        assert!(
            source.contains("var(--ui-fallback-") || source.contains("var(--ui-"),
            "sidebar styles should retain token fallback chain.",
        );
    }

    for source in [view_source.as_str(), motion_source.as_str()] {
        for forbidden in [
            "class=\"flex",
            "class=\"grid",
            "class=\"px-",
            "class=\"py-",
            "class=\"mx-",
            "class=\"my-",
            "class=\"text-",
            "class=\"bg-",
            "class=\"rounded-",
            "class=\"shadow-",
            "tailwind",
            "utility-first",
            "stylist::",
            "stylex",
            "emotion",
            "styled_components",
            "css!(",
            "styled!(",
        ] {
            assert!(
                !source.contains(forbidden),
                "sidebar component layer should not depend on utility-first/CSS-in-Rust marker `{forbidden}`.",
            );
        }
    }

    assert!(
        view_source.contains("style=move || motion_style.get_value()"),
        "sidebar runtime style path should be constrained to motion css variable binding.",
    );
    for needle in [
        "--ui-sidebar-motion-duration:",
        "--ui-sidebar-motion-reduced-duration:",
        "--ui-sidebar-motion-runtime-duration:",
    ] {
        assert!(
            motion_source.contains(needle),
            "sidebar runtime style should only expose motion css custom properties `{needle}`.",
        );
    }
}

#[test]
fn sidebar_visual_desire_reuses_theme_visual_baseline_and_heroui_contracts() {
    let baseline_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let baseline_e2e_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let sidebar_styles = load_source("../../components/sidebar/src/styles.rs");
    let sidebar_menu_styles = load_source("../../components/sidebar/src/menu/styles.rs");
    let sidebar_group_styles = load_source("../../components/sidebar/src/group/styles.rs");
    let sidebar_docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra/sidebar.rs");
    let heroui_strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "mod theme_visual_baseline;",
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            baseline_registry_source.contains(needle),
            "docs registry should expose visual baseline route token `{needle}`.",
        );
    }

    for needle in [
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "Checks first-impression quality: hierarchy, spacing rhythm, contrast layers, and interactive feedback (hover/active/focus).",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            baseline_page_source.contains(needle),
            "theme visual baseline page should include `{needle}`.",
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "docs-app: theme visual baseline renders button/input/overlay",
        "docs-app: theme visual baseline screenshots",
        concat!(
            "set E2E_VISUAL_BASELINE=on to run visual ",
            "snapshot regression"
        ),
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            baseline_e2e_source.contains(needle),
            "theme visual baseline e2e contract should include `{needle}`.",
        );
    }

    for needle in [
        "--ui-sidebar-line-height-100:",
        "--ui-sidebar-panel-gap:",
        "--ui-sidebar-panel-surface:",
        ".ui-sidebar__trigger:focus-visible",
        ".ui-sidebar__rail:hover",
        ".ui-sidebar[data-variant=\"floating\"] .ui-sidebar__panel",
        ".ui-sidebar[data-variant=\"inset\"] .ui-sidebar__panel",
    ] {
        assert!(
            sidebar_styles.contains(needle),
            "sidebar default visual style should include hierarchy/contrast/feedback token `{needle}`.",
        );
    }

    for needle in [
        ".ui-sidebar-menu__button:hover",
        ".ui-sidebar-menu__button:focus-visible",
        ".ui-sidebar-group__label:hover",
        ".ui-sidebar-group__label:focus-visible",
    ] {
        assert!(
            sidebar_menu_styles.contains(needle) || sidebar_group_styles.contains(needle),
            "sidebar interactive style should include visible feedback selector `{needle}`.",
        );
    }

    for forbidden in [
        "btn btn-",
        "panel-default",
        "Bootstrap",
        "bootstrap.min.css",
    ] {
        assert!(
            !sidebar_styles.contains(forbidden) && !sidebar_docs_source.contains(forbidden),
            "sidebar should avoid legacy coarse visual pattern marker `{forbidden}`.",
        );
    }

    for needle in [
        "# HeroUI 参数设计风格对齐策略",
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
    ] {
        assert!(
            heroui_strategy_source.contains(needle),
            "HeroUI strategy contract should include `{needle}`.",
        );
    }
}

#[test]
fn sidebar_tree_shaking_keeps_component_feature_and_css_boundaries_for_lib_target() {
    let ui_components_cargo = load_source("../../crates/ui/Cargo.toml");
    let ui_components_lib = load_source("../../crates/ui/src/lib.rs");
    let ui_components_css = load_source("../../crates/ui/src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "component-domain-sidebar = [",
        "component-sidebar = []",
        "component-sidebar_content = []",
        "component-sidebar_footer = []",
        "component-sidebar_group = [\"component-sidebar\"]",
        "component-sidebar_header = []",
        "component-sidebar_inset = []",
        "component-sidebar_menu = [\"component-active_highlight\"]",
        "component-sidebar_menu_action = []",
        "component-sidebar_menu_badge = []",
        "component-sidebar_rail = []",
        "component-sidebar_trigger = []",
        "inject-css = []",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui Cargo features should include sidebar tree-shaking marker `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-sidebar\")]",
        "#[path = \"../../../components/sidebar/src/mod.rs\"]",
        "pub mod sidebar;",
        "#[cfg(feature = \"component-sidebar_content\")]",
        "pub mod sidebar_content;",
        "#[cfg(feature = \"component-sidebar_footer\")]",
        "pub mod sidebar_footer;",
        "#[cfg(feature = \"component-sidebar_header\")]",
        "pub mod sidebar_header;",
        "#[cfg(feature = \"component-sidebar_inset\")]",
        "pub mod sidebar_inset;",
        "#[cfg(feature = \"component-sidebar_menu\")]",
        "pub mod sidebar_menu;",
        "#[cfg(feature = \"component-sidebar_menu_action\")]",
        "pub mod sidebar_menu_action;",
        "#[cfg(feature = \"component-sidebar_menu_badge\")]",
        "pub mod sidebar_menu_badge;",
        "#[cfg(feature = \"component-sidebar_rail\")]",
        "pub mod sidebar_rail;",
        "#[cfg(feature = \"component-sidebar_trigger\")]",
        "pub mod sidebar_trigger;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui lib.rs should keep sidebar export feature-gated via `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-sidebar\")]",
        "out.push_str(crate::sidebar::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_content\")]",
        "out.push_str(crate::sidebar_content::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_footer\")]",
        "out.push_str(crate::sidebar_footer::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_header\")]",
        "out.push_str(crate::sidebar_header::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_inset\")]",
        "out.push_str(crate::sidebar_inset::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_group\")]",
        "out.push_str(crate::sidebar::group::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_menu\")]",
        "out.push_str(crate::sidebar_menu::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_menu_action\")]",
        "out.push_str(crate::sidebar_menu_action::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_menu_badge\")]",
        "out.push_str(crate::sidebar_menu_badge::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_rail\")]",
        "out.push_str(crate::sidebar_rail::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_trigger\")]",
        "out.push_str(crate::sidebar_trigger::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "ui css aggregation should keep sidebar tree-shaking gate `{needle}`.",
        );
    }

    for forbidden in [
        "COMPONENT_REGISTRY",
        "ALL_COMPONENTS_REGISTRY",
        "GLOBAL_COMPONENT_MAP",
    ] {
        assert!(
            !ui_components_lib.contains(forbidden) && !ui_components_css.contains(forbidden),
            "tree-shaking boundary should avoid central reachability registry token `{forbidden}`.",
        );
    }

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("features = [\"inject-css\", \"web-demo-components\"]")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui via explicit web-demo-components bundle without all-components.",
    );
}

#[test]
fn sidebar_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget_for_lib_target() {
    let script_source = load_source("../../scripts/check-ui-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "SIDEBAR_MIN_FEATURES=\"component-sidebar,inject-css\"",
        "cargo test -p ui --test sidebar_semantics --no-default-features --features component-sidebar,inject-css sidebar_tree_shaking_keeps_component_feature_and_css_boundaries",
        "cargo test -p ui --test sidebar_semantics --no-default-features --features component-sidebar,inject-css sidebar_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "SIDEBAR_TREE_OUTPUT",
        "if grep -q 'all-components' <<<\"$SIDEBAR_TREE_OUTPUT\";",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$SIDEBAR_MIN_FEATURES\"",
        "cargo tree -e features -i ui -p web-demo",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$SIDEBAR_MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking check script should include `{needle}`.",
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`.",
        );
    }
}

#[test]
fn sidebar_check2_marks_tree_shaking_feature_pruning_contract_complete_for_lib_target() {
    let source = load_source("../../components/sidebar/check2.md");

    assert!(
        source.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
        "sidebar check2 should mark tree-shaking first-class ability item complete.",
    );

    for needle in [
        "sidebar_tree_shaking_keeps_component_feature_and_css_boundaries",
        "sidebar_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "sidebar_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui -p ui --no-default-features --features component-sidebar,inject-css",
        "cargo tree -e features -i ui -p web-demo",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-sidebar,inject-css",
        "scripts/check-ui-tree-shaking.sh",
        "scripts/tree_shaking_budget.env",
    ] {
        assert!(
            source.contains(needle),
            "sidebar check2 tree-shaking section should reference `{needle}`.",
        );
    }
}

#[test]
fn sidebar_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_components_cargo = load_source("../../crates/ui/Cargo.toml");
    let ui_components_lib = load_source("../../crates/ui/src/lib.rs");
    let ui_components_css = load_source("../../crates/ui/src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "component-domain-sidebar = [",
        "component-sidebar = []",
        "component-sidebar_content = []",
        "component-sidebar_footer = []",
        "component-sidebar_group = [\"component-sidebar\"]",
        "component-sidebar_header = []",
        "component-sidebar_inset = []",
        "component-sidebar_menu = [\"component-active_highlight\"]",
        "component-sidebar_menu_action = []",
        "component-sidebar_menu_badge = []",
        "component-sidebar_rail = []",
        "component-sidebar_trigger = []",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui Cargo feature map should keep sidebar tree-shaking marker `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-sidebar\")]",
        "#[path = \"../../../components/sidebar/src/mod.rs\"]",
        "pub mod sidebar;",
        "#[cfg(feature = \"component-sidebar_menu\")]",
        "#[path = \"../../../components/sidebar/src/menu/mod.rs\"]",
        "pub mod sidebar_menu;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui lib should gate sidebar exports behind component features via `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-sidebar\")]",
        "out.push_str(crate::sidebar::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_content\")]",
        "out.push_str(crate::sidebar_content::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_footer\")]",
        "out.push_str(crate::sidebar_footer::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_group\")]",
        "out.push_str(crate::sidebar::group::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_menu\")]",
        "out.push_str(crate::sidebar_menu::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_menu_action\")]",
        "out.push_str(crate::sidebar_menu_action::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_menu_badge\")]",
        "out.push_str(crate::sidebar_menu_badge::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_rail\")]",
        "out.push_str(crate::sidebar_rail::styles::CSS);",
        "#[cfg(feature = \"component-sidebar_trigger\")]",
        "out.push_str(crate::sidebar_trigger::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "ui css aggregation should keep sidebar feature-gated token `{needle}`.",
        );
    }

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("features = [\"inject-css\", \"web-demo-components\"]")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should depend on web-demo-components (not all-components) for source-mode pruning.",
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("features = [\"inject-css\", \"all-components\"]"),
        "docs-app should opt into all-components explicitly instead of implicit defaults.",
    );

    for forbidden in [
        "COMPONENT_REGISTRY",
        "ALL_COMPONENTS_REGISTRY",
        "GLOBAL_COMPONENT_MAP",
    ] {
        assert!(
            !ui_components_lib.contains(forbidden) && !ui_components_css.contains(forbidden),
            "tree-shaking should avoid central reachability registry token `{forbidden}`.",
        );
    }
}

#[test]
fn sidebar_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let script_source = load_source("../../scripts/check-ui-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "SIDEBAR_MIN_FEATURES=\"component-sidebar,inject-css\"",
        "cargo test -p ui --lib --no-default-features --features component-sidebar,inject-css sidebar_tree_shaking_keeps_component_feature_and_css_boundaries",
        "cargo test -p ui --lib --no-default-features --features component-sidebar,inject-css sidebar_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui --lib --no-default-features --features component-sidebar,inject-css sidebar_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "SIDEBAR_TREE_OUTPUT",
        "feature \"component-sidebar\" (command-line)",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$SIDEBAR_MIN_FEATURES\"",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$SIDEBAR_MIN_FEATURES\"",
        "cargo tree -e features -i ui -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "size regression",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking script should contain sidebar guard `{needle}`.",
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`.",
        );
    }
}

#[test]
fn sidebar_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let source = load_source("../../components/sidebar/check2.md");

    assert!(
        source.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
        "sidebar check2 should mark tree-shaking first-class item complete.",
    );

    for needle in [
        "sidebar_tree_shaking_keeps_component_feature_and_css_boundaries",
        "sidebar_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "sidebar_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui -p ui --no-default-features --features component-sidebar,inject-css",
        "cargo tree -e features -i ui -p web-demo",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-sidebar,inject-css",
        "scripts/check-ui-tree-shaking.sh",
        "scripts/tree_shaking_budget.env",
    ] {
        assert!(
            source.contains(needle),
            "sidebar check2 tree-shaking evidence should include `{needle}`.",
        );
    }
}
