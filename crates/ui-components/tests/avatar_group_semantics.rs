use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn avatar_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/avatar_group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "AvatarGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn avatar_group_uses_logic_state_model() {
    let view_source = load_source("src/avatar_group/view.rs");
    let logic_source = load_source("src/avatar_group/logic.rs");

    for needle in [
        "pub struct AvatarGroupStateInput",
        "pub struct AvatarGroupState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_max_visible(",
        "pub fn resolve_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "state_class",
        "aria_label_source_class",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "AvatarGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_max_visible(max)",
        "logic::resolve_aria_label(aria_label)",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(AvatarGroupStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "AvatarGroup view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn avatar_group_emits_spectrum_style_root_data_attributes() {
    let source = load_source("src/avatar_group/view.rs");

    for attr in [
        "data-slot=\"avatar-group\"",
        "data-size=state.size_attr",
        "data-state=state.state_attr",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-has-overflow=state.has_overflow.then_some(\"true\")",
        "data-count=state.total_count.to_string()",
        "data-visible-count=state.visible_count.to_string()",
        "data-overflow-count=state.overflow_count.to_string()",
        "data-max-visible=state.max_visible.to_string()",
        "data-custom-aria-label=state.has_custom_aria_label.then_some(\"true\")",
        "data-aria-label-source=state.aria_label_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "AvatarGroup should set `{attr}` for Spectrum-style styling and state inspection."
        );
    }
}

#[test]
fn avatar_group_exposes_item_and_overflow_slots() {
    let source = load_source("src/avatar_group/view.rs");

    for attr in [
        "data-slot=\"avatar-group-item\"",
        "data-index=index",
        "data-has-src=has_src.then_some(\"true\")",
        "class_name=\"ui-avatar-group__avatar\"",
        "data-slot=\"avatar-group-overflow\"",
        "data-count=state.overflow_count.to_string()",
    ] {
        assert!(
            source.contains(attr),
            "AvatarGroup should expose `{attr}` for deterministic item/overflow hooks."
        );
    }
}

#[test]
fn avatar_group_styles_include_state_source_and_marker_contracts() {
    let source = load_source("src/avatar_group/styles.rs");

    for selector in [
        ".ui-avatar-group--size-sm",
        ".ui-avatar-group[data-size=\"md\"]",
        ".ui-avatar-group--size-lg",
        ".ui-avatar-group--stable",
        ".ui-avatar-group[data-state=\"overflow\"]",
        ".ui-avatar-group--overflow .ui-avatar-group__overflow",
        ".ui-avatar-group[data-has-overflow=\"true\"] .ui-avatar-group__overflow",
        ".ui-avatar-group[data-state=\"empty\"]",
        ".ui-avatar-group--aria-label-custom",
        ".ui-avatar-group[data-aria-label-source=\"custom\"]",
        ".ui-avatar-group--custom-class",
        ".ui-avatar-group[data-custom-class=\"true\"]",
        ".ui-avatar-group[data-class-source=\"custom\"]",
    ] {
        assert!(
            source.contains(selector),
            "AvatarGroup styles should include `{selector}` as stable state-marker contracts."
        );
    }
}
