use std::fs;
use std::path::Path;

fn load_ui_components_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_alert_component_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let path = workspace_dir.join("components/alert").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_alert_test_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let path = workspace_dir.join("components/alert/test").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn ui_components_reexports_alert_component_crate() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-alert\")]")
            && lib_source.contains("pub use ui_alert as alert;"),
        "ui-components should re-export the external ui-alert crate as `alert`.",
    );
    assert!(
        cargo_source.contains("component-alert = [\"dep:ui-alert\"]"),
        "component-alert feature should depend on dep:ui-alert after extraction.",
    );
    assert!(
        cargo_source.contains("ui-alert = { path = \"../../components/alert\", optional = true }"),
        "ui-components Cargo.toml should include the optional ui-alert dependency.",
    );
}

#[test]
fn alert_component_module_exposes_unified_notification_contract() {
    let module_source = load_alert_component_source("src/mod.rs");

    for needle in [
        "pub use logic::{AlertFill, AlertLayout, AlertTone, AlertVariant};",
        "pub use motion::AlertMotion;",
        "pub use view::Alert;",
    ] {
        assert!(
            module_source.contains(needle),
            "alert component module should export `{needle}`."
        );
    }

    assert!(
        !module_source.contains("pub mod protocol;"),
        "Alert module should keep a minimal five-file boundary and must not re-export protocol.rs."
    );
}

#[test]
fn alert_component_manifest_defines_machine_readable_contract() {
    let manifest_source = load_alert_component_source("src/Component.toml");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Alert\"",
        "crate = \"ui-alert\"",
        "name = \"tone\"",
        "name = \"variant\"",
        "name = \"layout\"",
        "name = \"fill\"",
        "name = \"motion\"",
        "name = \"a11y_status_region\"",
        "name = \"ui-state-primitives\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "Alert component manifest should include `{needle}`."
        );
    }
}

#[test]
fn alert_component_rbi_projects_public_interface_signatures() {
    let rbi_source = load_alert_component_source("src/alert.rbi");

    for needle in [
        "pub enum AlertLayout",
        "pub enum AlertVariant",
        "pub type AlertTone = ui_state_primitives::alert_banner::AlertBannerTone;",
        "pub type AlertFill = ui_state_primitives::alert_banner::AlertBannerFill;",
        "pub struct AlertMotion",
        "pub fn Alert(",
        "-> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "Alert RBI projection should include `{needle}`."
        );
    }
}

#[test]
fn alert_view_accepts_tone_fill_layout_and_slot_props() {
    let view_source = load_alert_component_source("src/view.rs");

    for needle in [
        "#[prop(optional)] tone: Option<AlertTone>",
        "#[prop(optional)] variant: Option<AlertVariant>",
        "#[prop(optional)] layout: Option<AlertLayout>",
        "#[prop(optional)] fill: Option<AlertFill>",
        "#[prop(optional)] is_hide_icon: Option<bool>",
        "#[prop(optional)] hide_icon: Option<bool>",
        "#[prop(optional, into)] start_content: Option<ViewFn>",
        "#[prop(optional, into)] end_content: Option<ViewFn>",
        "#[prop(optional)] motion: AlertMotion",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert view should expose `{needle}` in public props."
        );
    }
}

#[test]
fn alert_api_naming_contract_keeps_is_prefix_with_legacy_alias_bridge() {
    let logic_source = load_alert_component_source("src/logic.rs");
    let view_source = load_alert_component_source("src/view.rs");

    assert!(
        logic_source.contains("pub fn resolve_hide_icon("),
        "Alert logic should provide a centralized hide_icon compatibility resolver."
    );
    for needle in [
        "#[prop(optional)] is_hide_icon: Option<bool>",
        "#[prop(optional)] hide_icon: Option<bool>",
        "let (hide_icon, hide_icon_source) = logic::resolve_hide_icon(is_hide_icon, hide_icon);",
        "data-hide-icon-source=hide_icon_source",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert API naming compatibility contract should include `{needle}`."
        );
    }
}

#[test]
fn alert_has_no_controlled_or_uncontrolled_state_axes() {
    let view_source = load_alert_component_source("src/view.rs");

    for forbidden in [
        "#[prop(optional)] value:",
        "#[prop(optional)] default_value:",
        "#[prop(optional)] is_open:",
        "#[prop(optional)] default_open:",
        "#[prop(optional, into)] on_open_change:",
        "#[prop(optional, into)] on_value_change:",
        "on_open_change",
        "on_value_change",
        "default_open",
        "default_value",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Alert should remain a stateless display component without controlled/uncontrolled axis token `{forbidden}`."
        );
    }
}

#[test]
fn alert_defaults_and_state_normalization_stay_in_logic_layer() {
    let logic_source = load_alert_component_source("src/logic.rs");
    let view_source = load_alert_component_source("src/view.rs");

    for needle in [
        "pub fn normalize_fill(fill: Option<AlertFill>) -> AlertFill",
        "pub fn normalize_layout(layout: Option<AlertLayout>) -> AlertLayout",
        "pub fn resolve_hide_icon(",
        "pub fn resolve_icon_label(",
        "AlertIconLabelSource",
        "pub fn resolve_state(input: AlertStateInput) -> AlertState",
    ] {
        assert!(
            logic_source.contains(needle),
            "Alert logic should own normalization/default contract `{needle}`."
        );
    }

    for needle in [
        "let title = logic::normalize_optional_text(title);",
        "let description = logic::normalize_optional_text(description);",
        "let class_name = logic::normalize_optional_text(class_name);",
        "let (hide_icon, hide_icon_source) = logic::resolve_hide_icon(is_hide_icon, hide_icon);",
        "let (icon_label, icon_label_source) = logic::resolve_icon_label(icon_label, state.tone);",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert view should consume logic-normalized values via `{needle}`."
        );
    }
    assert!(
        !view_source.contains("match (is_hide_icon, hide_icon)"),
        "Alert view should not duplicate hide_icon default logic outside logic.rs."
    );
    assert!(
        !view_source.contains("unwrap_or_default()"),
        "Alert view should not apply local fallback defaults outside logic.rs."
    );
    assert!(
        !view_source.contains(".default_icon_label()"),
        "Alert view should not compute icon-label defaults outside logic.rs."
    );
}

#[test]
fn alert_logic_uses_notification_primitives_and_variant_mapping() {
    let logic_source = load_alert_component_source("src/logic.rs");

    for needle in [
        "AlertBannerFill as AlertFill",
        "AlertBannerTone as AlertTone",
        "pub enum AlertLayout",
        "pub enum AlertVariant",
        "pub fn as_tone(self) -> AlertTone",
        "pub fn resolve_state(input: AlertStateInput) -> AlertState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: AlertState) -> String",
    ] {
        assert!(
            logic_source.contains(needle),
            "Alert logic should include `{needle}`."
        );
    }
}

#[test]
fn alert_view_derives_state_from_logic_layer() {
    let view_source = load_alert_component_source("src/view.rs");

    for needle in [
        "let state = logic::resolve_state(AlertStateInput {",
        "let class = logic::compose_class_name(class_name, state);",
        "let motion = crate::motion::sanitize_motion(motion);",
        "alert_motion::attach_motion(node_ref, motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert view should derive behavior via `{needle}`."
        );
    }
}

#[test]
fn alert_emits_unified_state_markers() {
    let view_source = load_alert_component_source("src/view.rs");

    for needle in [
        "data-slot=\"alert\"",
        "data-layout=state.layout_attr",
        "data-tone=state.tone_attr",
        "data-fill=state.fill_attr",
        "data-icon=state.icon_attr",
        "data-variant-source=state.variant_source_attr",
        "data-hide-icon-source=hide_icon_source",
        "data-icon-label-source=icon_label_source.as_attr()",
        "role=state.role_attr",
        "aria-live=state.live_attr",
        "data-motion-source=motion_source.as_attr()",
        "data-custom-motion=(motion_source == logic::AlertMotionSource::Custom).then_some(\"true\")",
        "data-ui-schema=logic::AlertAgentSchema::V1.as_attr()",
        "data-ui-intent=logic::AlertAgentIntent::StatusRegion.as_attr()",
        "data-ui-action=logic::AlertAgentAction::Announce.as_attr()",
        "data-ui-state=logic::AlertAgentState::Snapshot.as_attr()",
        "data-ui-source=agent_source.as_attr()",
        "data-ui-streaming=logic::AlertStreamingPolicy::Optional.as_attr()",
        "data-ui-fallback=logic::AlertStreamingFallback::Snapshot.as_attr()",
        "data-ui-output-status=logic::AlertOutputStatus::Verified.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert view should expose `{needle}` for semantic/state inspection."
        );
    }
}

#[test]
fn alert_agent_contract_schema_is_typed_and_whitelisted() {
    let logic_source = load_alert_component_source("src/logic.rs");
    let view_source = load_alert_component_source("src/view.rs");
    let manifest_source = load_alert_component_source("src/Component.toml");

    for needle in [
        "pub enum AlertAgentSchema",
        "pub enum AlertAgentIntent",
        "pub enum AlertAgentAction",
        "pub enum AlertAgentState",
        "pub enum AlertAgentSource",
        "pub fn resolve_agent_source(variant_source_attr: &'static str) -> AlertAgentSource",
    ] {
        assert!(
            logic_source.contains(needle),
            "Alert logic should type Agent contract field via `{needle}`."
        );
    }

    for needle in [
        "let motion_source = logic::resolve_motion_source(motion == AlertMotion::default());",
        "let agent_source = logic::resolve_agent_source(state.variant_source_attr);",
        "data-ui-schema=logic::AlertAgentSchema::V1.as_attr()",
        "data-ui-intent=logic::AlertAgentIntent::StatusRegion.as_attr()",
        "data-ui-action=logic::AlertAgentAction::Announce.as_attr()",
        "data-ui-state=logic::AlertAgentState::Snapshot.as_attr()",
        "data-ui-source=agent_source.as_attr()",
        "data-ui-streaming=logic::AlertStreamingPolicy::Optional.as_attr()",
        "data-ui-fallback=logic::AlertStreamingFallback::Snapshot.as_attr()",
        "data-ui-output-status=logic::AlertOutputStatus::Verified.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert view should mount typed Agent contract field `{needle}`."
        );
    }

    assert!(
        manifest_source.contains("name = \"agent-contract\""),
        "Alert Component.toml should project agent-contract capability for context compression."
    );
    assert!(
        manifest_source.contains("data-ui-streaming + data-ui-fallback + data-ui-output-status"),
        "Alert Component.toml agent-contract projection should include streaming policy/fallback/output-status fields."
    );

    for forbidden in ["inner_html", "set_inner_html", "javascript:"] {
        assert!(
            !view_source.contains(forbidden),
            "Alert view should keep whitelist-only render boundary and avoid `{forbidden}`."
        );
    }
}

#[test]
fn alert_llm_render_mode_is_snapshot_only_without_streaming_surface() {
    let logic_source = load_alert_component_source("src/logic.rs");
    let view_source = load_alert_component_source("src/view.rs");

    assert!(
        logic_source.contains("pub enum AlertAgentState")
            && logic_source.contains("Snapshot")
            && !logic_source.contains("Streaming"),
        "Alert should keep render-mode state in snapshot-only contract and avoid local streaming branch."
    );

    assert!(
        view_source.contains("data-ui-state=logic::AlertAgentState::Snapshot.as_attr()"),
        "Alert view should expose snapshot render-mode marker for Agent consumption."
    );

    for forbidden in [
        "data-streaming",
        "data-ui-stream",
        "streaming=true",
        "partial-output",
        "delta_chunk",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Alert should not expose ad-hoc streaming protocol token `{forbidden}`."
        );
    }
}

#[test]
fn alert_snapshot_mode_supports_complete_payload_rendering() {
    let logic_source = load_alert_component_source("src/logic.rs");
    let view_source = load_alert_component_source("src/view.rs");

    for needle in [
        "let title = logic::normalize_optional_text(title);",
        "let description = logic::normalize_optional_text(description);",
        "has_title: title.is_some()",
        "has_description: description.is_some()",
        "data-title=state.title_attr",
        "data-description=state.description_attr",
        "data-ui-state=logic::AlertAgentState::Snapshot.as_attr()",
        "{title.clone().filter(|_| state.show_title).map(|title| {",
        "{description.clone().filter(|_| state.show_description).map(|description| {",
        "{children()}",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "Alert snapshot baseline should preserve complete payload render contract `{needle}`."
        );
    }
}

#[test]
fn alert_streaming_policy_is_optional_with_snapshot_fallback_and_verified_status() {
    let logic_source = load_alert_component_source("src/logic.rs");
    let view_source = load_alert_component_source("src/view.rs");

    for needle in [
        "pub enum AlertStreamingPolicy",
        "pub enum AlertStreamingFallback",
        "pub enum AlertOutputStatus",
        "AlertStreamingPolicy::Optional => \"optional\"",
        "AlertStreamingFallback::Snapshot => \"snapshot\"",
        "AlertOutputStatus::Verified => \"verified\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "Alert logic should type streaming/status contract via `{needle}`."
        );
    }

    for needle in [
        "data-ui-streaming=logic::AlertStreamingPolicy::Optional.as_attr()",
        "data-ui-fallback=logic::AlertStreamingFallback::Snapshot.as_attr()",
        "data-ui-output-status=logic::AlertOutputStatus::Verified.as_attr()",
        "data-ui-state=logic::AlertAgentState::Snapshot.as_attr()",
        "role=state.role_attr",
        "aria-live=state.live_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert view should expose optional-streaming snapshot contract via `{needle}`."
        );
    }

    assert!(
        !view_source.contains("AlertStreamingPolicy::Required"),
        "Alert should not claim required streaming for a non-body snapshot component."
    );
}

#[test]
fn alert_rust_hygiene_contract_avoids_unwrap_expect_and_underscore_swallowing() {
    let logic_source = load_alert_component_source("src/logic.rs");
    let view_source = load_alert_component_source("src/view.rs");
    let motion_source = load_alert_component_source("src/motion.rs");
    let styles_source = load_alert_component_source("src/styles.rs");
    let combined = format!("{logic_source}\n{view_source}\n{motion_source}\n{styles_source}");

    for forbidden in [".unwrap(", ".unwrap_err(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "Alert non-test source should not include hygiene-forbidden token `{forbidden}`."
        );
    }

    for needle in [
        "use std::borrow::Cow;",
        "Vec<Cow<'static, str>>",
        "Cow::Borrowed(\"ui-alert\")",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Alert logic should keep string hotspot mitigation marker `{needle}`."
        );
    }
}

#[test]
fn alert_a11y_and_i18n_contract_is_mounted_from_headless_locale_bridge() {
    let view_source = load_alert_component_source("src/view.rs");

    for needle in [
        "#[prop(optional, into)] icon_label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang",
        "dir=locale.dir",
        "role=state.role_attr",
        "aria-live=state.live_attr",
        "data-icon-label-source=icon_label_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "Alert A11y/i18n contract should include `{needle}`."
        );
    }
}

#[test]
fn alert_styles_cover_tone_fill_layout_and_motion_contracts() {
    let styles_source = load_alert_component_source("src/styles.rs");

    for needle in [
        "--ui-alert-opacity",
        "--ui-alert-translate-y",
        "--ui-alert-scale",
        ".ui-alert[data-motion-source=\"custom\"]",
        ".ui-alert--layout-inline",
        ".ui-alert[data-layout=\"inline\"]",
        ".ui-alert--tone-info",
        ".ui-alert[data-tone=\"negative\"]",
        ".ui-alert--fill-border",
        ".ui-alert[data-fill=\"bold\"]",
        ".ui-alert__icon",
        ".ui-alert__content",
    ] {
        assert!(
            styles_source.contains(needle),
            "Alert styles should include `{needle}`."
        );
    }
}

#[test]
fn alert_styles_use_defensive_variable_fallback_chains() {
    let styles_source = load_alert_component_source("src/styles.rs");

    for required in [
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-alert-opacity, var(--ui-fallback-alert-opacity))",
        "var(--ui-alert-translate-y, var(--ui-fallback-alert-translate-y))",
        "var(--ui-alert-scale, var(--ui-fallback-alert-scale))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-danger, var(--ui-fallback-danger))",
    ] {
        assert!(
            styles_source.contains(required),
            "Alert styles should use defensive fallback chain `{required}`."
        );
    }

    for forbidden in ["border: 1px solid", "var(--ui-alert-opacity, 1);", "#"] {
        assert!(
            !styles_source.contains(forbidden),
            "Alert styles should not keep raw terminal style literal `{forbidden}`."
        );
    }
}

#[test]
fn alert_styles_depend_on_explicit_state_selectors() {
    let styles_source = load_alert_component_source("src/styles.rs");
    let view_source = load_alert_component_source("src/view.rs");

    for selector in [
        ".ui-alert[data-layout=\"inline\"]",
        ".ui-alert[data-tone=\"negative\"]",
        ".ui-alert[data-fill=\"bold\"]",
        ".ui-alert[data-icon=\"hidden\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "Alert styles should include explicit state selector `{selector}`."
        );
    }

    for bad_selector in [":nth-child", ":nth-of-type"] {
        assert!(
            !styles_source.contains(bad_selector),
            "Alert styles should avoid fragile structural selector `{bad_selector}`."
        );
    }

    assert!(
        !view_source.contains("style=") && !view_source.contains("style:"),
        "Alert view should not inject inline style properties."
    );
}

#[test]
fn alert_focus_path_is_neutral_without_focus_trap_or_keyboard_contract() {
    let view_source = load_alert_component_source("src/view.rs");

    for required in [
        "role=state.role_attr",
        "aria-live=state.live_attr",
        "data-slot=\"alert\"",
        "{children()}",
    ] {
        assert!(
            view_source.contains(required),
            "Alert focus/a11y baseline should include `{required}`."
        );
    }

    for forbidden in [
        "tabindex",
        "autofocus",
        "focus_trap",
        "on:focus",
        "on:blur",
        "on:keydown",
        "on:keyup",
        "on:keypress",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Alert should stay focus-neutral and must not mount focus-management token `{forbidden}`."
        );
    }
}

#[test]
fn alert_render_count_equivalent_evidence_keeps_stateless_single_pass_path() {
    let logic_source = load_alert_component_source("src/logic.rs");
    let view_source = load_alert_component_source("src/view.rs");
    let combined = format!("{logic_source}\n{view_source}");

    // Equivalent evidence for render_count budget in source-level contract tests:
    // no local reactive signals/effects/events => render work follows parent input changes only.
    for forbidden in [
        "create_signal",
        "create_rw_signal",
        "RwSignal",
        "ReadSignal",
        "create_memo",
        "Memo::new",
        "create_effect",
        "Effect::new",
        "watch(",
        "spawn_local",
        "request_animation_frame",
        "on:click",
        "on:input",
        "on:change",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Alert render path should remain stateless; found forbidden token `{forbidden}`."
        );
    }

    for required in [
        "let state = logic::resolve_state(AlertStateInput {",
        "let class = logic::compose_class_name(class_name, state);",
        "let node_ref: NodeRef<html::Section> = NodeRef::new();",
        "alert_motion::attach_motion(node_ref, motion);",
    ] {
        assert!(
            view_source.contains(required),
            "Alert render-path contract should keep `{required}`."
        );
    }
}

#[test]
fn alert_motion_contract_remains_spring_based() {
    let motion_source = load_alert_component_source("src/motion.rs");
    let motion_checks_source = load_alert_test_source("motion.rs");
    let combined = format!("{motion_source}\n{motion_checks_source}");

    for needle in [
        "pub struct AlertMotion",
        "SpringAnimator",
        "pub fn sanitize_motion(motion: AlertMotion) -> AlertMotion",
        "fn default_motion_matches_alert_spring_contract()",
        "fn sanitize_motion_falls_back_for_invalid_values()",
    ] {
        assert!(
            combined.contains(needle),
            "Alert motion should include `{needle}`."
        );
    }
}

#[test]
fn alert_component_file_responsibilities_match_standard_layout() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let alert_src = workspace_dir.join("components/alert/src");

    for required in ["mod.rs", "logic.rs", "view.rs", "styles.rs", "motion.rs"] {
        assert!(
            alert_src.join(required).exists(),
            "Alert component should include `{required}`."
        );
    }

    for forbidden in ["spec.rs", "render.rs"] {
        assert!(
            !alert_src.join(forbidden).exists(),
            "Alert component should not include `{forbidden}` for current complexity tier."
        );
    }
}

#[test]
fn alert_tree_shaking_contract_is_feature_gated_for_module_and_css() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let css_source = load_ui_components_source("src/css.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-alert\")]")
            && lib_source.contains("pub use ui_alert as alert;"),
        "Alert module export should stay under component-alert feature gate."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-alert\")]")
            && css_source.contains("out.push_str(crate::alert::styles::CSS);"),
        "Alert CSS aggregation should stay under component-alert feature gate."
    );
    assert!(
        cargo_source.contains("component-alert = [\"dep:ui-alert\"]"),
        "component-alert feature should map to dep:ui-alert in Cargo.toml."
    );
}

#[test]
fn alert_version_migration_not_required_without_major_breaking_upgrade() {
    let component_manifest = load_alert_component_source("src/Component.toml");
    let mod_source = load_alert_component_source("src/mod.rs");
    let rbi_source = load_alert_component_source("src/alert.rbi");
    let logic_source = load_alert_component_source("src/logic.rs");
    let view_source = load_alert_component_source("src/view.rs");
    let combined = format!("{mod_source}\n{rbi_source}\n{logic_source}\n{view_source}");

    assert!(
        component_manifest.contains("schema_version = \"1\""),
        "Alert schema version should remain at v1 for non-breaking iteration."
    );
    assert!(
        !component_manifest.contains("schema_version = \"2\""),
        "Alert should not claim v2 schema without a registered breaking migration plan."
    );

    for required in [
        "pub use logic::{AlertFill, AlertLayout, AlertTone, AlertVariant};",
        "pub use motion::AlertMotion;",
        "pub use view::Alert;",
    ] {
        assert!(
            combined.contains(required),
            "Alert stable public surface should keep `{required}` while migration is N/A."
        );
    }

    assert!(
        !combined.contains("migrate_v1_to_v2"),
        "Alert should not ship v1->v2 migration shim when no major-breaking upgrade exists."
    );
}

#[test]
fn alert_css_is_aggregated_under_ui_layer_without_plain_inline_styles() {
    let css_registry = load_ui_components_source("src/css.rs");
    let view_source = load_alert_component_source("src/view.rs");

    assert!(
        css_registry.contains("out.push_str(\"\\n@layer ui {\\n\");"),
        "ui-components css registry should aggregate component styles under `@layer ui`."
    );
    assert!(
        css_registry.contains("#[cfg(feature = \"component-alert\")]")
            && css_registry.contains("out.push_str(crate::alert::styles::CSS);"),
        "Alert styles should be feature-gated and injected through the centralized ui layer registry."
    );

    for line in view_source.lines() {
        let trimmed = line.trim_start();

        assert!(
            !trimmed.starts_with("style="),
            "Alert view should not use plain inline `style=...`; found `{trimmed}`.",
        );

        if trimmed.contains("style:") {
            assert!(
                trimmed.contains("style:--"),
                "Alert runtime style mutation must use CSS custom properties only; found `{trimmed}`.",
            );
        }
    }
}

#[test]
fn alert_ui_components_entrypoint_layout_and_headless_boundaries_are_correct() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let css_source = load_ui_components_source("src/css.rs");
    let root_source = load_ui_components_source("src/root.rs");
    let active_highlight_source =
        load_ui_components_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable_state_source =
        load_ui_components_source("../../crates/ui-headless/src/controllable_state.rs");
    let headless_presence_source =
        load_ui_components_source("../../crates/ui-headless/src/presence.rs");
    let headless_a11y_source = load_ui_components_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "#[cfg(feature = \"component-alert\")]",
        "pub use ui_alert as alert;",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entrypoint should keep `{needle}`."
        );
    }

    for needle in [
        "pub fn push_components_css(out: &mut String)",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-alert\")]",
        "out.push_str(crate::alert::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css entrypoint should keep `{needle}`."
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot entrypoint should keep `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str",
        "pub struct ActiveHighlightMotion",
        "attach_active_highlight_motion",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep `{needle}`."
        );
    }

    assert!(
        headless_controllable_state_source.contains("pub fn use_controllable_state"),
        "headless controllable-state primitive should stay in `crates/ui-headless/src/controllable_state.rs`."
    );
    assert!(
        headless_presence_source.contains("pub fn use_presence"),
        "headless presence primitive should stay in `crates/ui-headless/src/presence.rs`."
    );
    assert!(
        headless_a11y_source.contains("pub fn aria_controls_when_open"),
        "headless a11y helper should stay in `crates/ui-headless/src/a11y.rs`."
    );

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !manifest_dir.join(forbidden).exists(),
            "ui-components forbidden entrypoint file should not exist: `{forbidden}`."
        );
    }
}

#[test]
fn alert_dom_environment_contracts_cover_cfg_and_hydration_boundaries() {
    let logic_source = load_alert_component_source("src/logic.rs");
    let view_source = load_alert_component_source("src/view.rs");
    let motion_source = load_alert_component_source("src/motion.rs");
    let headless_lib_source = load_ui_components_source("../../crates/ui-headless/src/lib.rs");

    assert!(
        headless_lib_source.contains("features `web` and `ssr` are mutually exclusive"),
        "ui-headless should keep compile_error guard for web/ssr mutual exclusion."
    );

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if ui_motion::web::prefers_reduced_motion()",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "Alert motion should include platform/reduced-motion guard `{needle}`."
        );
    }

    for forbidden in [
        "now()",
        "Uuid::new_v4",
        "rand::",
        "inner_html",
        "set_inner_html",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Alert should not depend on forbidden hydration/unsafe HTML token `{forbidden}`."
        );
    }
}

#[test]
fn alert_view_structure_avoids_macro_bloat_and_prefers_function_split() {
    let view_source = load_alert_component_source("src/view.rs");

    assert!(
        view_source
            .contains("fn render_alert_icon(layout: AlertLayout, tone: AlertTone) -> AnyView"),
        "Alert view should split icon rendering into plain function helper."
    );
    assert!(
        view_source.contains("{render_alert_icon(state.layout, state.tone)}"),
        "Alert main view! block should consume icon helper instead of inlining giant match."
    );
}

#[test]
fn docs_page_uses_unified_alert_api() {
    let display_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn alert() -> AnyView",
        "title=\"Alert\"",
        "slug=\"alert\"",
        "tone=AlertTone::Info",
        "fill=AlertFill::Border",
        "layout=AlertLayout::Inline",
        "is_hide_icon=true",
        "variant=AlertVariant::Danger",
        "motion=AlertMotion {",
    ] {
        assert!(
            display_source.contains(needle),
            "Alert docs should include `{needle}`."
        );
    }
}

#[test]
fn docs_page_hello_world_stays_minimal_and_copy_paste_ready() {
    let display_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    assert!(
        display_source.contains(
            "r#\"<Alert>\n  \\\"Install now to keep your workspace secure.\\\"\n</Alert>\"#"
        ),
        "Alert docs Hello World should remain a 3-line minimal example."
    );
    assert!(
        display_source.contains("test_source_path=\"components/alert/src/view.rs\".to_string()"),
        "Alert docs playground should point to the alert source path."
    );
}

#[test]
fn alert_readme_is_beginner_friendly_with_progressive_path() {
    let readme_source = load_alert_component_source("src/README.md");

    for needle in [
        "# Alert",
        "## Hello World（先用起来）",
        "## 常见用法",
        "## 进阶用法（需要时再看）",
        "## docs-app 入口",
        "use ui_components::Alert;",
        "use ui_components::{Alert, AlertFill, AlertTone};",
        "use ui_components::{Alert, AlertFill, AlertLayout, AlertTone};",
        "use ui_components::{Alert, AlertMotion};",
        "apps/docs-app/src/pages/components/pages/display.rs",
        "/#/components/alert",
    ] {
        assert!(
            readme_source.contains(needle),
            "Alert README beginner contract should include `{needle}`."
        );
    }

    let hello_index = readme_source
        .find("## Hello World（先用起来）")
        .expect("Hello World section should exist");
    let common_index = readme_source
        .find("## 常见用法")
        .expect("Common usage section should exist");
    let advanced_index = readme_source
        .find("## 进阶用法（需要时再看）")
        .expect("Advanced section should exist");

    assert!(
        hello_index < common_index && common_index < advanced_index,
        "README should guide beginners with progressive order: Hello World -> common -> advanced."
    );
}

#[test]
fn docs_page_alert_exposes_interactive_playground_contract() {
    let display_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let e2e_source = load_ui_components_source("../../e2e/tests/docs_app_alert_contract.spec.mjs");

    for needle in [
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "data-slot=\"alert-workbench-controls\"",
        "data-slot=\"alert-workbench-preview\"",
        "id_base=\"docs-alert-workbench-tone\".to_string()",
        "id_base=\"docs-alert-workbench-fill\".to_string()",
        "id_base=\"docs-alert-workbench-layout\".to_string()",
        "\"is_hide_icon\"",
        "\"Show title\"",
        "\"Show description\"",
        "\"Custom class_name\"",
        "\"RTL direction\"",
        "test_source_path=\"components/alert/src/styles.rs\".to_string()",
        "test_config_signal=workbench_config",
        "code_signal=workbench_code",
        "Acknowledge",
    ] {
        assert!(
            display_source.contains(needle),
            "Alert docs interactive playground contract should include `{needle}`."
        );
    }

    assert!(
        e2e_source.contains(
            "docs-app alert key flow is repeatable with focus+keyboard semantic breakpoints"
        ),
        "Alert key interaction should stay in repeatable E2E regression set."
    );
}

#[test]
fn docs_page_alert_source_first_copy_paste_ready_contract_is_stable() {
    let display_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "data-slot=\"alert-source-first\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "label=\"Copy alert starter\".to_string()",
        "copyable=true",
        "class_name=\"docs-alert-source-copy\".to_string()",
        "use leptos::prelude::*;\\nuse ui_components::{Alert, AlertFill, AlertTone};",
        "data-slot=\"alert-source-paths\"",
        "components/alert/src/mod.rs",
        "components/alert/src/logic.rs",
        "components/alert/src/view.rs",
        "components/alert/src/styles.rs",
        "components/alert/src/motion.rs",
        "data-slot=\"alert-source-prerequisites\"",
        "component-alert",
        "UiRoot",
        "inject-css",
    ] {
        assert!(
            display_source.contains(needle),
            "Alert source-first contract should include `{needle}`."
        );
    }
}

#[test]
fn alert_heroui_strategy_doc_and_component_docs_entry_stay_in_sync() {
    let strategy_source =
        load_ui_components_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages.rs");
    let readme_source = load_alert_component_source("src/README.md");

    for needle in [
        "### Alert 同步记录（2026-02-20）",
        "`Alert` 参数主轴已统一为 `tone/fill/layout`",
        "component_doc!(\"Alert\", \"alert\", \"Display\", display::alert)",
        "`#/components/alert` 可索引访问",
        "`apps/docs-app/src/pages/components/pages/display.rs::alert()`",
        "`component-alert`、`UiRoot`、`inject-css`",
    ] {
        assert!(
            strategy_source.contains(needle),
            "HeroUI strategy doc should include synchronized Alert record `{needle}`."
        );
    }

    assert!(
        pages_source.contains("component_doc!(\"Alert\", \"alert\", \"Display\", display::alert)"),
        "components catalog should keep Alert docs entry indexed in pages.rs."
    );
    assert!(
        readme_source.contains("/#/components/alert"),
        "Alert README should keep docs-app entry route for discoverability."
    );
}

#[test]
fn docs_page_alert_has_productized_playgrounds_and_source_first_copy_ready_contract() {
    let display_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional / Snapshot\"",
        "code_imports=\"use leptos::prelude::*;\\nuse ui_components::{Alert, AlertFill, AlertTone};\".to_string()",
        "data-slot=\"alert-state-matrix\"",
        "data-slot=\"alert-parameter-matrix\"",
        "data-slot=\"alert-parameter-rows\"",
        "data-slot=\"alert-streaming-modes\"",
        "data-slot=\"alert-source-first\"",
        "Alert has no disabled prop in API",
        "default = None -> normalize to banner",
        "default = None -> normalize to border",
        "default = None/None -> hide_icon=false (source=default)",
        "default = AlertMotion::default()",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "label=\"Copy alert starter\".to_string()",
        "copyable=true",
        "class_name=\"docs-alert-source-copy\".to_string()",
    ] {
        assert!(
            display_source.contains(needle),
            "Alert docs product contract should include `{needle}`."
        );
    }
}

#[test]
fn docs_navigation_no_longer_lists_alert_banner_or_inline_alert() {
    let pages_source =
        load_ui_components_source("../../apps/docs-app/src/pages/components/pages.rs");

    assert!(
        !pages_source.contains("\"alert-banner\""),
        "components pages should no longer expose a separate alert-banner route."
    );
    assert!(
        !pages_source.contains("\"inline-alert\""),
        "components pages should no longer expose a separate inline-alert route."
    );
}

#[test]
fn alert_e2e_contract_prefers_semantic_selectors_and_settled_waits() {
    let e2e_source = load_ui_components_source("../../e2e/tests/docs_app_alert_contract.spec.mjs");

    for needle in [
        "page.goto(\"/#/components/alert\")",
        "body:not(:has(#boot))",
        "[data-slot=\"alert\"]",
        "data-ui-output-status=\"verified\"",
        "data-ui-state=\"snapshot\"",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "docs-app alert key flow is repeatable with focus+keyboard semantic breakpoints",
        "toBeFocused()",
        "page.keyboard.press(\"Enter\")",
        "data-custom-class=\"true\"",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Alert E2E contract should include semantic-ready marker `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        ":nth-child",
        ":nth-of-type",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "Alert E2E contract should avoid brittle/unstable token `{forbidden}`."
        );
    }
}

#[test]
fn alert_has_no_async_loading_protocol() {
    let logic_source = load_alert_component_source("src/logic.rs");
    let view_source = load_alert_component_source("src/view.rs");

    for forbidden in [
        "use_async_action",
        "is_loading",
        "aria-busy",
        "retry",
        "spawn_local",
        "create_resource",
        "async fn",
        ".await",
        "Future<",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Alert has no async workflow; forbidden token `{forbidden}` should be absent."
        );
    }
}
