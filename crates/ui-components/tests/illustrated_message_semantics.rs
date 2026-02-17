use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn illustrated_message_does_not_expose_logic_module() {
    let source = load_source("src/illustrated_message/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "IllustratedMessage's `logic` module should stay private to avoid leaking internal view-state helpers into the public API."
    );
}

#[test]
fn illustrated_message_emits_expected_data_slots() {
    let source = load_source("src/illustrated_message/view.rs");

    for attr in [
        "data-slot=\"illustrated-message\"",
        "data-slot=\"illustrated-message-content\"",
        "data-slot=\"illustrated-message-title\"",
        "data-slot=\"illustrated-message-description\"",
        "data-slot=\"illustrated-message-actions\"",
    ] {
        assert!(
            source.contains(attr),
            "IllustratedMessage should set `{attr}` for baseline-style styling and inspection."
        );
    }
}

#[test]
fn illustrated_message_uses_spring_driven_opacity_and_y_css_vars() {
    let styles = load_source("src/illustrated_message/styles.rs");
    let motion = load_source("src/illustrated_message/motion.rs");

    for needle in [
        "--ui-im-opacity",
        "--ui-im-y",
        "opacity: var(--ui-im-opacity)",
        "transform: translateY(var(--ui-im-y))",
    ] {
        assert!(
            styles.contains(needle),
            "IllustratedMessage styles should reference `{needle}` for spring-driven enter motion."
        );
    }

    for needle in ["--ui-im-opacity", "--ui-im-y"] {
        assert!(
            motion.contains(needle),
            "IllustratedMessage motion should write `{needle}` to drive enter animation."
        );
    }
}

#[test]
fn illustrated_message_attaches_motion_driver() {
    let source = load_source("src/illustrated_message/view.rs");

    assert!(
        source.contains("motion::attach_motion"),
        "IllustratedMessage should attach its motion driver from `motion.rs`."
    );
}

#[test]
fn illustrated_message_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/illustrated_message/motion.rs");
    let view_source = load_source("src/illustrated_message/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: IllustratedMessageMotion) -> IllustratedMessageMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "IllustratedMessage motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source
            .contains("let motion = crate::illustrated_message::motion::sanitize_motion(motion);"),
        "IllustratedMessage view should sanitize motion before attaching motion driver.",
    );
}

#[test]
fn illustrated_message_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn illustrated_message() -> AnyView",
        "title=\"IllustratedMessage\"",
        "slug=\"illustrated-message\"",
        "title=\"Empty state\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for IllustratedMessage.",
        );
    }
}

#[test]
fn illustrated_message_docs_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "let code = Signal::derive(move || {",
        "title=\"No results\".to_string()",
        "description=\"Try changing your search.\".to_string()",
        "illustration=move || view! { <div class=\"docs-illustration\">\"◎\"</div> }",
        "actions=move || view! { <ui_components::Button>\"Clear\"</ui_components::Button> }",
    ] {
        assert!(
            source.contains(needle),
            "illustrated-message docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"IllustratedMessage\"",
        "slug=\"illustrated-message\"",
        "title=\"Empty state\"",
        "title=\"No results\".to_string()",
        "description=\"Try changing your search.\".to_string()",
        "illustration=move || view! { <div class=\"docs-illustration\">\"◎\"</div> }",
        "actions=move || view! { <ui_components::Button>\"Clear\"</ui_components::Button> }",
    ] {
        assert!(
            source.contains(needle),
            "illustrated-message docs playgrounds should contain `{needle}`.",
        );
    }
}
