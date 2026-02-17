use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn button_copy_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button/copy/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ButtonCopy internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn button_copy_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/button/copy/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::ButtonCopy;")
            && module_source.contains("pub use motion::ButtonCopyMotion;")
            && module_source.contains("pub use i18n::ButtonCopyStrings;"),
        "button_copy module should export `ButtonCopy`, `ButtonCopyMotion`, and `ButtonCopyStrings`."
    );
    assert!(
        crate_source
            .contains("pub use button::copy::{ButtonCopy, ButtonCopyMotion, ButtonCopyStrings};"),
        "crate root should re-export `ButtonCopy`, `ButtonCopyMotion`, and `ButtonCopyStrings` contracts."
    );
}

#[test]
fn button_copy_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::button::copy::styles::CSS);"),
        "ui-components css aggregator should include button_copy styles."
    );
}

#[test]
fn button_copy_docs_page_contains_playground_contracts() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn button_copy() -> AnyView",
        "title=\"ButtonCopy\"",
        "slug=\"button-copy\"",
        "Label + variant",
        "Disabled + empty matrix",
        "Mode matrix",
        "Copy-to-clipboard button with baseline-style disabled/empty semantics and live copied announcements.",
        "<ButtonCopy",
    ] {
        assert!(
            source.contains(needle),
            "button-copy docs page should contain `{needle}`."
        );
    }
}

#[test]
fn button_copy_uses_logic_state_model() {
    let view_source = load_source("src/button/copy/view.rs");
    let logic_source = load_source("src/button/copy/logic.rs");

    for needle in [
        "pub struct ButtonCopyViewState",
        "pub struct ButtonCopyTextContract",
        "pub enum ButtonCopyMode",
        "pub is_copyable: bool",
        "pub has_custom_label: bool",
        "pub fn normalize_optional_text(",
        "pub fn resolve_text_contract(",
        "pub fn resolve_view_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ButtonCopy logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "#[prop(optional, default = logic::ButtonCopyMode::default())] mode: logic::ButtonCopyMode",
        "let label = logic::normalize_optional_text(label);",
        "let copied_label = logic::normalize_optional_text(copied_label);",
        "logic::resolve_text_contract(",
        "label.or(default_label)",
        "copied_label.or(default_copied_label)",
        "let view_state = logic::resolve_view_state(",
        "let class = logic::compose_class_name(class_name, view_state);",
    ] {
        assert!(
            view_source.contains(needle),
            "ButtonCopy view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn button_copy_uses_snippet_logic_for_copy_behavior() {
    let source = load_source("src/button/copy/view.rs");

    for needle in [
        "crate::snippet::logic::use_snippet_logic(text.clone())",
        "on_press=logic.copy",
        "data-copied=move || logic.copied.get().then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy should delegate copy behavior via `{needle}`."
        );
    }
}

#[test]
fn button_copy_supports_i18n_and_locale_passthrough() {
    let source = load_source("src/button/copy/view.rs");

    for needle in [
        "let i18n = i18n::use_ui_i18n();",
        "strings::<super::i18n::ButtonCopyStrings>()",
        "copy_button_label",
        "copied_status_text",
        "copy_failed_status_text",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = ui_headless::a11y::locale_attrs(lang, dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy should include i18n/locale support via `{needle}`.",
        );
    }
}

#[test]
fn button_copy_forwards_button_contract_and_disabled_semantics() {
    let source = load_source("src/button/copy/view.rs");

    for needle in [
        "<Button",
        "variant=variant",
        "size=size",
        "motion=motion.button",
        "aria_label=aria_label.get_value()",
        "is_icon_only=view_state.is_icon_only",
        "is_loading=is_copying",
        "is_disabled=!view_state.is_copyable",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy should forward `{needle}` to the underlying Button."
        );
    }
}

#[test]
fn button_copy_emits_baseline_style_data_attributes() {
    let source = load_source("src/button/copy/view.rs");

    for needle in [
        "data-slot=\"button-copy\"",
        "data-state=if view_state.is_copyable {",
        "data-mode=view_state.mode_attr",
        "data-icon-only=view_state.is_icon_only.then_some(\"true\")",
        "data-with-icon=view_state.shows_icon.then_some(\"true\")",
        "data-with-text=view_state.shows_text.then_some(\"true\")",
        "data-copyable=view_state.is_copyable.then_some(\"true\")",
        "data-disabled=view_state.is_disabled.then_some(\"true\")",
        "data-empty=(!view_state.has_text).then_some(\"true\")",
        "data-label=if view_state.has_custom_label {",
        "data-copied-label=if view_state.has_custom_copied_label {",
        "data-copying=move || logic.is_copying.get().then_some(\"true\")",
        "data-copy-error=move || logic.has_copy_error.get().then_some(\"true\")",
        "data-copy-status=move || {",
        "data-motion-source=if motion == ButtonCopyMotion::default()",
        "data-custom-motion=(motion != ButtonCopyMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy should expose `{needle}` for baseline-style state inspection."
        );
    }
}

#[test]
fn button_copy_announces_copy_result_for_assistive_tech() {
    let source = load_source("src/button/copy/view.rs");

    for needle in [
        "data-slot=\"button-copy-status\"",
        "aria-live=\"polite\"",
        "aria-atomic=\"true\"",
        "copy_failed_status_text.get_value()",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy a11y status element should include `{needle}`."
        );
    }
}

#[test]
fn button_copy_styles_include_motion_marker_contracts() {
    let source = load_source("src/button/copy/styles.rs");

    for selector in [
        ".ui-button-copy[data-motion-source=\"custom\"]",
        ".ui-button-copy[data-custom-motion=\"true\"]",
        ".ui-button-copy[data-mode=\"icon-only\"] .ui-button-copy__button",
        ".ui-button-copy[data-copied=\"true\"] .ui-button-copy__button",
    ] {
        assert!(
            source.contains(selector),
            "ButtonCopy styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn button_copy_motion_contract_exposes_default_and_custom_tests() {
    let source = load_source("src/button/copy/motion.rs");

    for needle in [
        "pub struct ButtonCopyMotion",
        "pub fn attach_motion(",
        "fn default_motion_matches_button_contract_defaults()",
        "fn supports_custom_button_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy motion module should include `{needle}` for baseline-level motion contract coverage."
        );
    }
}

#[test]
fn button_copy_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/button/copy/motion.rs");
    let view_source = load_source("src/button/copy/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ButtonCopyMotion) -> ButtonCopyMotion",
        "button: crate::button::motion::sanitize_motion(motion.button)",
        "copied_feedback_spring: sanitize_spring(motion.copied_feedback_spring)",
        "fn sanitize_motion_clamps_feedback_values()",
        "fn sanitize_motion_delegates_to_button_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "ButtonCopy motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = super::motion::sanitize_motion(motion);"),
        "ButtonCopy view should sanitize motion before forwarding to Button.",
    );
    assert!(
        view_source.contains("super::motion::attach_motion(root_ref, logic.copied, motion);"),
        "ButtonCopy should attach copied-feedback motion on wrapper state.",
    );
}

#[test]
fn button_copy_docs_default_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn button_copy() -> AnyView",
        "<Playground title=\"Label + variant\" code_signal=code>",
        "text=\"cargo add ui-components\".to_string()",
        "label=\"Copy install command\".to_string()",
        "copied_label=\"Copied!\".to_string()",
        "text=\"https://github.com/openai\".to_string()",
        "variant=ButtonVariant::Outline",
        "label=\"Copy URL\".to_string()",
        "copied_label=\"URL copied\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "button_copy docs default playground should contain `{needle}`.",
        );
    }
}

#[test]
fn button_copy_docs_state_matrix_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<Playground title=\"Disabled + empty matrix\" code_signal=states_code>",
        "text=\"https://example.com/docs\".to_string()",
        "variant=ButtonVariant::Outline",
        "text=\"   \".to_string()",
        "label=\"Nothing to copy\".to_string()",
        "text=\"token\".to_string()",
        "is_disabled=true",
        "Blank text and explicit disabled state both force non-copyable semantics.",
    ] {
        assert!(
            source.contains(needle),
            "button_copy docs state matrix playground should contain `{needle}`.",
        );
    }
}

#[test]
fn button_copy_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn button_copy() -> AnyView",
        "title=\"ButtonCopy\"",
        "slug=\"button-copy\"",
        "Label + variant",
        "Disabled + empty matrix",
        "Mode matrix",
    ] {
        assert!(
            source.contains(needle),
            "button_copy docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn button_copy_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<Playground title=\"Label + variant\" code_signal=code>",
        "text=\"cargo add ui-components\".to_string()",
        "label=\"Copy install command\".to_string()",
        "copied_label=\"Copied!\".to_string()",
        "<Playground title=\"Disabled + empty matrix\" code_signal=states_code>",
        "<Playground title=\"Mode matrix\" code_signal=modes_code>",
        "text=\"   \".to_string()",
        "label=\"Nothing to copy\".to_string()",
        "is_disabled=true",
        "mode=ButtonCopyMode::TextOnly",
        "mode=ButtonCopyMode::IconOnly",
        "mode=ButtonCopyMode::IconAndText",
    ] {
        assert!(
            source.contains(needle),
            "button_copy docs playground should contain `{needle}`.",
        );
    }
}
