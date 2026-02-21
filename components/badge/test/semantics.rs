use std::fs;
use std::path::Path;

fn load_badge_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn badge_component_layer_split_is_stable() {
    let mod_source = load_badge_source("src/mod.rs");

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "pub mod motion;",
        "mod view;",
    ] {
        assert!(
            mod_source.contains(needle),
            "badge module boundary should include `{needle}`"
        );
    }

    for needle in [
        "pub use logic::BadgeVariant;",
        "pub use motion::BadgeMotion;",
        "pub use view::Badge;",
    ] {
        assert!(
            mod_source.contains(needle),
            "badge public API should include `{needle}`"
        );
    }
}

#[test]
fn badge_view_mounts_headless_locale_and_semantic_markers() {
    let source = load_badge_source("src/view.rs");

    for needle in [
        "use ui_headless::{A11yDirection, locale_attrs};",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang",
        "dir=locale.dir",
        "data-slot=\"badge\"",
        "data-state=render_state.state.fill_attr",
        "data-ui-schema=render_state.agent_contract.schema_name",
        "data-ui-output-status=render_state.agent_contract.output_status.as_attr()",
    ] {
        assert!(
            source.contains(needle),
            "badge view should mount semantic contract marker `{needle}`"
        );
    }
}

#[test]
fn badge_public_surface_does_not_expose_dom_specific_types() {
    for rel_path in [
        "src/mod.rs",
        "src/logic.rs",
        "src/view.rs",
        "src/styles.rs",
        "src/motion.rs",
    ] {
        let source = load_badge_source(rel_path);
        for forbidden in ["web_sys", "wasm_bindgen", "HtmlElement"] {
            assert!(
                !source.contains(forbidden),
                "badge source `{rel_path}` should not leak DOM-specific public dependency `{forbidden}`"
            );
        }
    }
}

#[test]
fn badge_non_test_source_avoids_forbidden_hygiene_patterns() {
    for rel_path in [
        "src/mod.rs",
        "src/logic.rs",
        "src/view.rs",
        "src/styles.rs",
        "src/motion.rs",
        "src/protocol.rs",
    ] {
        let source = load_badge_source(rel_path);
        for forbidden in [".unwrap(", ".expect(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "badge non-test source `{rel_path}` should avoid forbidden hygiene pattern `{forbidden}`"
            );
        }
    }
}

#[test]
fn badge_logic_uses_cow_for_class_name_composition() {
    let source = load_badge_source("src/logic.rs");

    for needle in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![",
        "Cow::Borrowed(\"ui-badge\")",
        "Cow::Borrowed(\"ui-badge--custom-class\")",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            source.contains(needle),
            "badge logic should include `{needle}` for Cow-based string composition"
        );
    }

    for forbidden in [
        "\"ui-badge\".to_string()",
        "\"ui-badge--custom-class\".to_string()",
    ] {
        assert!(
            !source.contains(forbidden),
            "badge logic should not keep string clone hotspot `{forbidden}`"
        );
    }
}

#[test]
fn badge_api_stays_display_only_without_controlled_axes() {
    let source = load_badge_source("src/view.rs");

    for required in [
        "#[prop(optional, into)] variant: Option<BadgeVariant>,",
        "let render_state = logic::resolve_render_state(variant, class_name);",
        "#[prop(optional, into)] class_name: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
    ] {
        assert!(
            source.contains(required),
            "badge API should keep stable display props `{required}`"
        );
    }

    for forbidden in [
        "value:",
        "on_value_change",
        "default_value",
        "unwrap_or",
        "variant: String",
        "variant: Option<String>",
        "is_open",
        "on_open_change",
        "BadgeStateInput {",
        "logic::resolve_state(",
    ] {
        assert!(
            !source.contains(forbidden),
            "badge API should not introduce controlled/uncontrolled axis marker `{forbidden}`"
        );
    }
}

#[test]
fn badge_styles_use_defensive_variable_fallback_chains() {
    let source = load_badge_source("src/styles.rs");

    for needle in [
        "gap: var(--ui-space-xs, var(--ui-fallback-space-xs));",
        "var(--ui-space-sm, var(--ui-fallback-space-sm));",
        "border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));",
        "var(--ui-border-width, var(--ui-fallback-border-width)) solid transparent;",
        "font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
        "line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));",
        "background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
        "background: var(--ui-accent, var(--ui-fallback-accent));",
        "color: var(--ui-accent-fg, var(--ui-fallback-accent-fg));",
        "background: var(--ui-danger, var(--ui-fallback-danger));",
        "color: var(--ui-danger-fg, var(--ui-fallback-danger-fg));",
    ] {
        assert!(
            source.contains(needle),
            "badge styles should include defensive fallback chain `{needle}`"
        );
    }
}

#[test]
fn badge_component_manifest_declares_context_compression_contract() {
    let source = load_badge_source("src/Component.toml");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Badge\"",
        "crate = \"ui-badge\"",
        "name = \"variant\"",
        "name = \"class_name\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"agent_contract_schema_markers\"",
        "name = \"snapshot_rendering\"",
    ] {
        assert!(
            source.contains(needle),
            "badge manifest should include `{needle}`"
        );
    }
}

#[test]
fn badge_rbi_tracks_public_signature_projection() {
    let source = load_badge_source("src/badge.rbi");

    for needle in [
        "pub type BadgeVariant = ui_state_primitives::badge::BadgeVariant;",
        "pub struct BadgeMotion {",
        "pub enter_ms: u16,",
        "pub exit_ms: u16,",
        "pub reduced_ms: u16,",
        "pub fn Badge(",
        "variant: Option<BadgeVariant>,",
        "class_name: Option<String>,",
        "lang: Option<String>,",
        "dir: Option<ui_headless::A11yDirection>,",
        "children: leptos::children::Children,",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            source.contains(needle),
            "badge RBI should include `{needle}`"
        );
    }
}
