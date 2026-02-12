use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn skeleton_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/skeleton_group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SkeletonGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn skeleton_group_uses_logic_state_model() {
    let mod_source = load_source("src/skeleton_group/mod.rs");
    let logic_source = load_source("src/skeleton_group/logic.rs");
    let view_source = load_source("src/skeleton_group/view.rs");

    for needle in [
        "pub struct SkeletonGroupStateInput",
        "pub struct SkeletonGroupState",
    ] {
        assert!(
            mod_source.contains(needle),
            "SkeletonGroup module should include `{needle}` state contracts."
        );
    }

    for needle in [
        "pub enum SkeletonGroupVariant",
        "pub enum SkeletonGroupLayout",
        "pub enum SkeletonGroupDensity",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "SkeletonGroup logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(SkeletonGroupStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "SkeletonGroup view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn skeleton_group_emits_spectrum_style_state_data_attributes() {
    let source = load_source("src/skeleton_group/view.rs");

    for attr in [
        "data-slot=\"skeleton-group\"",
        "data-state=state.state_attr",
        "data-visibility=state.visibility_attr",
        "data-loading-mode=state.loading_mode_attr",
        "data-variant=state.variant_attr",
        "data-layout=state.layout_attr",
        "data-density=state.density_attr",
        "data-label-source=state.label_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "aria-busy=state.is_loading.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "SkeletonGroup should expose `{attr}` for Spectrum-style state inspection and styling."
        );
    }
}

#[test]
fn skeleton_group_supports_skeleton_only_hidden_contract() {
    let source = load_source("src/skeleton_group/view.rs");

    for needle in ["state.should_hide_root", "hidden=state.should_hide_root"] {
        assert!(
            source.contains(needle),
            "SkeletonGroup should implement skeleton-only hidden contract (`{needle}`)."
        );
    }
}

#[test]
fn skeleton_group_styles_include_variant_and_layout_contracts() {
    let source = load_source("src/skeleton_group/styles.rs");

    for selector in [
        ".ui-skeleton-group",
        ".ui-skeleton-group--layout-horizontal",
        ".ui-skeleton-group[data-layout=\"vertical\"]",
        ".ui-skeleton-group--density-compact",
        ".ui-skeleton-group[data-variant=\"pulse\"] .ui-skeleton",
        ".ui-skeleton-group[data-variant=\"none\"] .ui-skeleton",
        ".ui-skeleton-group--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "SkeletonGroup styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn skeleton_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn skeleton_group() -> AnyView",
        r#"title="SkeletonGroup""#,
        r#"slug="skeleton-group""#,
        r#"description="Spectrum/HeroUI-style skeleton coordination container with centralized loading/layout/variant visibility contracts and stable slot/data-state markers.""#,
        r#"<Playground title="Shimmer + Pulse Layout" code=loading_code>"#,
        r#"<Playground title="Loaded + Skeleton Only" code=state_code>"#,
        "<SkeletonGroup",
    ] {
        assert!(
            source.contains(needle),
            "display_extra skeleton_group docs should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn skeleton_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        r#"title="Shimmer + Pulse Layout""#,
        "is_loading=true",
        "variant=SkeletonGroupVariant::Shimmer",
        "layout=SkeletonGroupLayout::Vertical",
        "density=SkeletonGroupDensity::Comfortable",
        "variant=SkeletonGroupVariant::Pulse",
        "layout=SkeletonGroupLayout::Horizontal",
        "density=SkeletonGroupDensity::Compact",
        r#"aria_label="Profile placeholders".to_string()"#,
        r#"class_name="docs-skeleton-group-custom".to_string()"#,
        "variant=SkeletonVariant::Circle",
        "shimmer=false",
        r#"title="Loaded + Skeleton Only""#,
        "is_loading=false",
        "is_skeleton_only=false",
        "variant=SkeletonGroupVariant::None",
        "is_skeleton_only=true",
        "\"Loaded content rendered by parent group.\"",
        "When `is_skeleton_only=true` and loading is finished, the skeleton group hides itself.",
    ] {
        assert!(
            source.contains(needle),
            "display_extra skeleton_group playgrounds should contain `{needle}` for state-matrix contracts.",
        );
    }
}
