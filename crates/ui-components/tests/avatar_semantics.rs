use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn avatar_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/avatar/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Avatar internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn avatar_uses_logic_state_model() {
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");

    for needle in [
        "pub struct AvatarStateInput",
        "pub struct AvatarState",
        "pub struct AvatarAccessibility",
        "pub fn normalize_optional_text(",
        "pub fn resolve_initials(",
        "pub fn resolve_accessibility(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "label_source_class",
        "ui-avatar--custom-class",
    ] {
        assert!(
            logic_source.contains(needle),
            "Avatar logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let name = logic::normalize_optional_text(name);",
        "let src = logic::normalize_optional_text(src);",
        "let alt = logic::normalize_optional_text(alt);",
        "let accessibility = logic::resolve_accessibility(name.as_deref(), alt.as_deref());",
        "let state = logic::resolve_state(logic::AvatarStateInput {",
        "let class = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "Avatar view should derive state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn avatar_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/avatar/view.rs");

    for attr in [
        "data-slot=\"avatar\"",
        "data-size=state.size_attr",
        "data-state=move || if show_image.get() { \"image\" } else { \"fallback\" }",
        "data-image=move || show_image.get().then_some(\"true\")",
        "data-fallback=move || (!show_image.get()).then_some(\"true\")",
        "data-has-name=state.has_name.then_some(\"true\")",
        "data-has-src=state.has_src.then_some(\"true\")",
        "data-has-alt=state.has_alt.then_some(\"true\")",
        "data-label-source=state.label_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "class:ui-avatar--image=move || show_image.get()",
        "class:ui-avatar--fallback=move || !show_image.get()",
    ] {
        assert!(
            source.contains(attr),
            "Avatar should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn avatar_fallback_wires_accessible_name_contract() {
    let source = load_source("src/avatar/view.rs");

    for needle in [
        "role=move || (!show_image.get()).then_some(\"img\".to_string())",
        "aria-label=move || (!show_image.get()).then_some(aria_label.get_value())",
        "data-slot=\"avatar-initials\"",
    ] {
        assert!(
            source.contains(needle),
            "Avatar fallback should include `{needle}` for accessible image semantics."
        );
    }
}

#[test]
fn avatar_image_slot_supports_error_fallback() {
    let source = load_source("src/avatar/view.rs");

    for needle in [
        "data-slot=\"avatar-img\"",
        "on:error=move |_| img_error.set(true)",
        "let show_image = Signal::derive(move || has_src && !img_error.get());",
    ] {
        assert!(
            source.contains(needle),
            "Avatar image rendering should include `{needle}` so broken images fall back to initials."
        );
    }
}

#[test]
fn avatar_styles_include_dual_state_marker_contracts() {
    let source = load_source("src/avatar/styles.rs");

    for selector in [
        ".ui-avatar--sm",
        ".ui-avatar[data-size=\"md\"]",
        ".ui-avatar--lg",
        ".ui-avatar--image",
        ".ui-avatar[data-state=\"fallback\"]",
        ".ui-avatar[data-image=\"true\"]",
        ".ui-avatar--has-src.ui-avatar--image",
        ".ui-avatar[data-has-src=\"true\"][data-state=\"image\"]",
        ".ui-avatar--label-alt",
        ".ui-avatar[data-label-source=\"name\"]",
        ".ui-avatar[data-label-source=\"fallback\"]",
        ".ui-avatar[data-has-alt=\"true\"][data-fallback=\"true\"]",
        ".ui-avatar--custom-class",
        ".ui-avatar[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Avatar styles should include `{selector}` for stable state-marker contracts."
        );
    }
}

#[test]
fn avatar_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn avatar() -> AnyView",
        "title=\"Avatar\"",
        "slug=\"avatar\"",
        "Playground title=\"Image + Fallback\"",
        "Playground title=\"Sizes + Label Sources\"",
        "Playground title=\"Custom Class + Normalized Props\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Avatar.",
        );
    }
}

#[test]
fn avatar_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Image + Fallback\"",
        "<Avatar name=\"Ada Lovelace\".to_string() src=src.to_string() size=AvatarSize::Md />",
        "<Avatar name=\"Grace Hopper\".to_string() size=AvatarSize::Md />",
        "title=\"Sizes + Label Sources\"",
        "alt=\"Profile photo\".to_string()",
        "<Avatar alt=\"Anonymous collaborator\".to_string() size=AvatarSize::Sm />",
        "<Avatar size=AvatarSize::Lg />",
        "title=\"Custom Class + Normalized Props\"",
        "class_name=\"docs-avatar-custom\".to_string()",
        "src=\"   \".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "avatar docs playgrounds should contain `{needle}`.",
        );
    }
}
