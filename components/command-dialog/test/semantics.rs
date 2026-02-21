use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn command_dialog_directory_layout_matches_ui_components_contract() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for rel_path in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
        "test/logic.rs",
        "test/motion.rs",
        "test/protocol.rs",
        "test/semantics.rs",
    ] {
        assert!(
            manifest_dir.join(rel_path).exists(),
            "command-dialog should keep `{rel_path}` in the expected location."
        );
    }

    assert!(
        !manifest_dir.join("src/render.rs").exists(),
        "command-dialog should not drift to `render.rs`; rendering must stay in `view.rs`."
    );
}

#[test]
fn command_dialog_mod_exports_stable_api_and_wires_local_semantics_tests() {
    let source = load_source("src/mod.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::CommandDialogMotion;",
        "pub use view::CommandDialog;",
        "#[path = \"../test/semantics.rs\"]",
        "mod semantics_tests;",
    ] {
        assert!(
            source.contains(needle),
            "command-dialog mod should include `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub use web_sys",
        "pub use leptos::",
    ] {
        assert!(
            !source.contains(forbidden),
            "command-dialog mod should keep internals private and avoid DOM-type leakage `{forbidden}`."
        );
    }
}

#[test]
fn command_dialog_layer_responsibilities_stay_separated() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");

    for needle in [
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn normalize_id_base(",
        "pub fn normalize_title(",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep state normalization and derivation marker `{needle}`."
        );
    }

    for forbidden in ["view! {", "web_sys", "NodeRef<"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not include rendering/DOM details `{forbidden}`."
        );
    }

    for needle in [
        "logic::resolve_state(CommandDialogPartStateInput {",
        "logic::compose_class_name(",
        "motion::attach_motion(command_motion, overlay_motion)",
        "use_presence(open)",
        "<Modal",
        "<Command",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should assemble logic/headless/motion contracts via `{needle}`."
        );
    }

    assert!(
        !view_source.contains("web_sys"),
        "view.rs should avoid direct web-sys dependency in this component boundary."
    );

    assert!(
        styles_source.contains("var(--ui-"),
        "styles.rs should consume theme tokens via `var(--ui-*)`."
    );

    for forbidden in ["view! {", "use_presence(", "SpringAnimator::new"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should stay static and not include runtime behavior `{forbidden}`."
        );
    }

    for needle in [
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "overlay: crate::overlay::motion::sanitize_motion",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should keep contract mapping marker `{needle}`."
        );
    }

    for forbidden in ["view! {", "role=", "aria-", "web_sys"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not include view/a11y/dom tokens `{forbidden}`."
        );
    }
}

#[test]
fn command_dialog_api_naming_contract_uses_is_on_default_prefixes() {
    let view_source = load_source("src/view.rs");
    let readme_source = load_source("src/README.md");

    for needle in [
        "default_open: Option<bool>",
        "on_open_change: Option<Callback<bool>>",
        "on_action: Option<Callback<String>>",
        "is_disabled: Option<bool>",
        "let disabled = logic::normalize_is_disabled(is_disabled, disabled);",
    ] {
        assert!(
            view_source.contains(needle),
            "command-dialog API naming contract should include `{needle}`."
        );
    }

    for needle in [
        "打开状态轴：`open + on_open_change + default_open`",
        "禁用状态轴：`is_disabled`（规范） + `disabled`（兼容别名，`is_disabled` 优先）",
    ] {
        assert!(
            readme_source.contains(needle),
            "README should document naming compatibility and migration path via `{needle}`."
        );
    }
}

#[test]
fn command_dialog_default_values_are_normalized_in_logic_layer() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    assert!(
        logic_source.contains("pub fn resolve_text_with_empty_default("),
        "logic.rs should own text fallback defaults via `resolve_text_with_empty_default`."
    );

    for needle in [
        "let description_text = logic::resolve_text_with_empty_default(description.as_deref());",
        "let placeholder_text = logic::resolve_text_with_empty_default(placeholder.as_deref());",
        "let empty_label_text = logic::resolve_text_with_empty_default(empty_label.as_deref());",
        "let aria_label_text = logic::resolve_text_with_empty_default(aria_label.as_deref());",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should consume logic defaults via `{needle}`."
        );
    }

    for forbidden in ["unwrap_or_default()", "unwrap_or("] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not keep default fallback branch `{forbidden}`."
        );
    }
}
