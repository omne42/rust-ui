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
fn skeleton_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/skeleton/group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "SkeletonGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn skeleton_group_public_api_stays_component_layer_and_hides_dom_details() {
    let mod_source = load_source("src/skeleton/group/mod.rs");
    let view_source = load_source("src/skeleton/group/view.rs");

    for needle in [
        "pub use logic::{",
        "DEFAULT_ARIA_LABEL",
        "pub use view::SkeletonGroup;",
        "pub struct SkeletonGroupStateInput",
        "pub struct SkeletonGroupState",
    ] {
        assert!(
            mod_source.contains(needle),
            "SkeletonGroup public contract should expose `{needle}` inside component-layer boundary."
        );
    }

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "HtmlElement",
        "NodeRef<web_sys",
        "EventTarget",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "SkeletonGroup API should not leak DOM/web-sys details; found `{forbidden}`.",
        );
    }
}

#[test]
fn skeleton_group_public_props_follow_is_on_default_prefix_contract() {
    let view_source = load_source("src/skeleton/group/view.rs");
    let logic_source = load_source("src/skeleton/group/logic.rs");

    for needle in [
        "#[prop(optional)] is_loading: Option<bool>",
        "#[prop(optional)] is_skeleton_only: Option<bool>",
    ] {
        assert!(
            view_source.contains(needle),
            "SkeletonGroup boolean public prop should use `is_*` prefix; missing `{needle}`.",
        );
    }
    for needle in [
        "pub const DEFAULT_IS_LOADING: bool = true;",
        "pub const DEFAULT_IS_SKELETON_ONLY: bool = false;",
    ] {
        assert!(
            logic_source.contains(needle),
            "SkeletonGroup defaults should be centralized in logic.rs; missing `{needle}`.",
        );
    }
}

#[test]
fn skeleton_group_has_no_half_controlled_state_axis_contract() {
    let view_source = load_source("src/skeleton/group/view.rs");
    let logic_source = load_source("src/skeleton/group/logic.rs");

    assert!(
        view_source.contains("logic::normalize_state_input(logic::SkeletonGroupViewInput {"),
        "SkeletonGroup should derive render state from props in one pass."
    );
    for forbidden in [
        "create_signal(",
        "RwSignal",
        "ReadSignal",
        "WriteSignal",
        "on_loading_change",
        "default_loading",
        "on_skeleton_only_change",
        "default_skeleton_only",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "SkeletonGroup should not carry half-controlled local state contract; found `{forbidden}`.",
        );
    }
}

#[test]
fn skeleton_group_uses_logic_state_model() {
    let mod_source = load_source("src/skeleton/group/mod.rs");
    let logic_source = load_source("src/skeleton/group/logic.rs");
    let view_source = load_source("src/skeleton/group/view.rs");

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
        "pub struct SkeletonGroupViewInput",
        "pub enum SkeletonGroupVariant",
        "pub enum SkeletonGroupLayout",
        "pub enum SkeletonGroupDensity",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_state_input(",
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
        "logic::normalize_state_input(logic::SkeletonGroupViewInput {",
        "let state = logic::resolve_state(state_input);",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "SkeletonGroup view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn skeleton_group_defaults_are_normalized_only_in_logic() {
    let view_source = load_source("src/skeleton/group/view.rs");
    let logic_source = load_source("src/skeleton/group/logic.rs");

    assert!(
        !view_source.contains("default ="),
        "SkeletonGroup view.rs must not own default value branches.",
    );
    assert!(
        !view_source.contains("unwrap_or("),
        "SkeletonGroup view.rs should consume normalized outputs instead of fallback branching.",
    );
    for needle in [
        "pub const DEFAULT_IS_LOADING: bool = true;",
        "pub const DEFAULT_IS_SKELETON_ONLY: bool = false;",
        "pub fn normalize_state_input(",
        "input.is_loading.unwrap_or(DEFAULT_IS_LOADING)",
        "input.is_skeleton_only.unwrap_or(DEFAULT_IS_SKELETON_ONLY)",
    ] {
        assert!(
            logic_source.contains(needle),
            "SkeletonGroup defaults should be centralized in logic.rs; missing `{needle}`.",
        );
    }
}

#[test]
fn skeleton_group_state_normalization_is_centralized_in_logic() {
    let view_source = load_source("src/skeleton/group/view.rs");
    let logic_source = load_source("src/skeleton/group/logic.rs");

    for forbidden in [
        "if is_loading",
        "if is_skeleton_only",
        "match is_loading",
        "match is_skeleton_only",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "SkeletonGroup view.rs should not rebuild state decisions; found `{forbidden}`.",
        );
    }

    for needle in [
        "pub fn normalize_state_input(",
        "pub fn resolve_state(",
        "let state_attr = if input.is_loading {",
        "let loading_mode_attr = if input.is_skeleton_only {",
    ] {
        assert!(
            logic_source.contains(needle),
            "SkeletonGroup logic.rs should centralize state normalization; missing `{needle}`.",
        );
    }
}

#[test]
fn skeleton_group_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/skeleton/group/view.rs");

    for attr in [
        "data-slot=\"skeleton-group\"",
        "data-state=state.state_attr",
        "data-visibility=state.visibility_attr",
        "data-loading-mode=state.loading_mode_attr",
        "data-loading-source=state.loading_source_attr",
        "data-skeleton-only-source=state.skeleton_only_source_attr",
        "data-variant=state.variant_attr",
        "data-variant-source=state.variant_source_attr",
        "data-layout=state.layout_attr",
        "data-layout-source=state.layout_source_attr",
        "data-density=state.density_attr",
        "data-density-source=state.density_source_attr",
        "data-label-source=state.label_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "aria-busy=state.is_loading.then_some(\"true\")",
    ] {
        assert!(
            source.contains(attr),
            "SkeletonGroup should expose `{attr}` for baseline-style state inspection and styling."
        );
    }
}

#[test]
fn skeleton_group_supports_skeleton_only_hidden_contract() {
    let source = load_source("src/skeleton/group/view.rs");

    for needle in ["state.should_hide_root", "hidden=state.should_hide_root"] {
        assert!(
            source.contains(needle),
            "SkeletonGroup should implement skeleton-only hidden contract (`{needle}`)."
        );
    }
}

#[test]
fn skeleton_group_remains_non_interactive_without_headless_handler_contract() {
    let source = load_source("src/skeleton/group/view.rs");

    for forbidden in [
        "on:click=",
        "on:keydown=",
        "on:pointer",
        "tabindex=",
        "use_button(",
        "use_focus_ring(",
        "use_hover(",
        "use_press(",
    ] {
        assert!(
            !source.contains(forbidden),
            "SkeletonGroup currently serves as a non-interactive loading container; `{forbidden}` indicates headless interaction semantics leaking into this component.",
        );
    }
}

#[test]
fn skeleton_group_does_not_define_motion_file_or_custom_animation_driver() {
    let mod_source = load_source("src/skeleton/group/mod.rs");
    let view_source = load_source("src/skeleton/group/view.rs");
    let logic_source = load_source("src/skeleton/group/logic.rs");

    for forbidden in [
        "mod motion;",
        "attach_motion(",
        "ui_motion::",
        "SpringAnimator",
        "request_animation_frame",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden),
            "SkeletonGroup should keep motion concern out of component-local implementation; found `{forbidden}`.",
        );
    }
}

#[test]
fn skeleton_group_styles_include_variant_and_layout_contracts() {
    let source = load_source("src/skeleton/group/styles.rs");

    for selector in [
        ".ui-skeleton-group",
        ".ui-skeleton-group--layout-horizontal",
        ".ui-skeleton-group[data-layout=\"vertical\"]",
        ".ui-skeleton-group--density-compact",
        ".ui-skeleton-group[data-loading-source=\"prop\"]",
        ".ui-skeleton-group[data-skeleton-only-source=\"prop\"]",
        ".ui-skeleton-group[data-variant-source=\"prop\"]",
        ".ui-skeleton-group[data-layout-source=\"prop\"]",
        ".ui-skeleton-group[data-density-source=\"prop\"]",
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
fn skeleton_group_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let styles_source = load_source("src/skeleton/group/styles.rs");
    let view_source = load_source("src/skeleton/group/view.rs");

    for needle in [
        ".ui-skeleton-group[data-layout=\"vertical\"]",
        ".ui-skeleton-group[data-layout=\"horizontal\"]",
        ".ui-skeleton-group[data-density=\"compact\"]",
        ".ui-skeleton-group[data-loading-source=\"prop\"]",
        ".ui-skeleton-group[data-skeleton-only-source=\"prop\"]",
        ".ui-skeleton-group[data-variant-source=\"prop\"]",
        ".ui-skeleton-group[data-layout-source=\"prop\"]",
        ".ui-skeleton-group[data-density-source=\"prop\"]",
        ".ui-skeleton-group[data-variant=\"pulse\"] .ui-skeleton",
        ".ui-skeleton-group[data-state=\"loaded\"]",
        ".ui-skeleton-group[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "SkeletonGroup styles should key off explicit semantic state markers `{needle}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", "style="] {
        assert!(
            !styles_source.contains(forbidden) && !view_source.contains(forbidden),
            "SkeletonGroup should not rely on fragile DOM-structure/inline-style contract `{forbidden}`.",
        );
    }
}

#[test]
fn skeleton_group_semantic_contract_covers_role_aria_and_state_source_markers() {
    let source = load_source("src/skeleton/group/view.rs");

    for attr in [
        "role=\"group\"",
        "aria-busy=state.is_loading.then_some(\"true\")",
        "data-state=state.state_attr",
        "data-loading-mode=state.loading_mode_attr",
        "data-loading-source=state.loading_source_attr",
        "data-skeleton-only-source=state.skeleton_only_source_attr",
        "data-variant-source=state.variant_source_attr",
        "data-layout-source=state.layout_source_attr",
        "data-density-source=state.density_source_attr",
        "data-label-source=state.label_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "SkeletonGroup semantic contract should expose `{attr}` for a11y + machine-readable state/source checks.",
        );
    }
}

#[test]
fn skeleton_group_component_files_follow_responsibility_boundaries() {
    let mod_source = load_source("src/skeleton/group/mod.rs");
    let logic_source = load_source("src/skeleton/group/logic.rs");
    let styles_source = load_source("src/skeleton/group/styles.rs");
    let view_source = load_source("src/skeleton/group/view.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let motion_path = manifest_dir.join("src/skeleton/group/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::SkeletonGroup;",
    ] {
        assert!(
            mod_source.contains(needle),
            "skeleton/group/mod.rs should keep minimal stable exports; missing `{needle}`.",
        );
    }
    for forbidden in ["#[component]", "view! {", "pub const CSS", "mod motion;"] {
        assert!(
            !mod_source.contains(forbidden),
            "skeleton/group/mod.rs should not carry implementation details (`{forbidden}`).",
        );
    }

    for forbidden in [
        "#[component]",
        "view! {",
        "<div",
        "data-slot=",
        "var(--ui-",
        "color-mix(",
        "web_sys::",
        "wasm_bindgen::",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "skeleton/group/logic.rs should only normalize/derive state, not view/style/platform details (`{forbidden}`).",
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-space-sm)",
        "var(--ui-accent)",
    ] {
        assert!(
            styles_source.contains(needle),
            "skeleton/group/styles.rs should be token-first static CSS; missing `{needle}`.",
        );
    }
    for forbidden in [
        "#[component]",
        "view! {",
        "pub fn normalize_state_input",
        "on:click=",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "skeleton/group/styles.rs should not include component logic/event handling (`{forbidden}`).",
        );
    }

    for needle in [
        "#[component]",
        "view! {",
        "data-state=state.state_attr",
        "role=\"group\"",
    ] {
        assert!(
            view_source.contains(needle),
            "skeleton/group/view.rs should render structure and mount semantic contract; missing `{needle}`.",
        );
    }
    for forbidden in ["unwrap_or(", "pub const CSS", "web_sys::", "wasm_bindgen::"] {
        assert!(
            !view_source.contains(forbidden),
            "skeleton/group/view.rs should not own defaults/style constants/platform details (`{forbidden}`).",
        );
    }

    assert!(
        !motion_path.exists() && !mod_source.contains("mod motion;"),
        "SkeletonGroup is non-interactive and should keep motion layer out of this component.",
    );
}

#[test]
fn skeleton_group_simple_component_does_not_define_spec_file_or_module() {
    let mod_source = load_source("src/skeleton/group/mod.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/skeleton/group/spec.rs");

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "SkeletonGroup is a simple component and should not expose spec module contract (`{forbidden}`).",
        );
    }

    assert!(
        !spec_path.exists(),
        "SkeletonGroup should not define `src/skeleton/group/spec.rs` without stable external schema/versioning requirements.",
    );
}

#[test]
fn skeleton_group_theme_contract_is_token_first_and_ui_theme_owned() {
    let source = load_source("src/skeleton/group/styles.rs");

    for needle in [
        "var(--ui-space-sm)",
        "var(--ui-space-xs)",
        "var(--ui-accent)",
        "color-mix(",
    ] {
        assert!(
            source.contains(needle),
            "SkeletonGroup styles should consume theme tokens via `{needle}`.",
        );
    }

    for forbidden in ["--skeleton-group-", "#fff", "#000", "rgb(", "hsl("] {
        assert!(
            !source.contains(forbidden),
            "SkeletonGroup styles should not introduce private color/token system (`{forbidden}`).",
        );
    }
}

#[test]
fn skeleton_group_token_first_styles_are_static_and_aggregated_via_ui_root_css_pipeline() {
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let styles_source = load_source("src/skeleton/group/styles.rs");
    let view_source = load_source("src/skeleton/group/view.rs");
    let logic_source = load_source("src/skeleton/group/logic.rs");

    for required in [
        "#[cfg(feature = \"component-skeleton_group\")]",
        "out.push_str(crate::skeleton::group::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "css.rs should aggregate SkeletonGroup styles via feature-gated contract `{required}`.",
        );
    }

    for required in [
        "#[prop(optional)] inject_components_css: bool",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should stay as centralized CSS injection boundary via `{required}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-space-sm)",
        "var(--ui-space-xs)",
        "var(--ui-accent)",
        "color-mix(",
    ] {
        assert!(
            styles_source.contains(required),
            "SkeletonGroup styles should stay token-first/static and include `{required}`.",
        );
    }

    for forbidden in [
        "--skeleton-group-",
        "@apply",
        "tailwind",
        "tw-",
        "styled(",
        "stylex",
        "emotion",
        "css!(",
        "style!(",
        "format!(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "SkeletonGroup styles should not adopt utility-first/CSS-in-Rust/runtime style token `{forbidden}`.",
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"px-",
        "class=\"py-",
        "class=\"rounded-",
        "class=\"bg-",
        "class=\"text-",
        "tailwind",
        "tw!",
        "css!(",
        "style!(",
        "styled!(",
        "emotion",
        "style=",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "SkeletonGroup component layer should not depend on utility-first/CSS-in-Rust marker `{forbidden}`.",
        );
    }
}

#[test]
fn skeleton_group_visual_desire_reuses_theme_visual_baseline_and_heroui_contracts() {
    let baseline_page =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let e2e_spec = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_doc = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "Checks first-impression quality: hierarchy, spacing rhythm, contrast layers, and interactive feedback (hover/active/focus).",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            baseline_page.contains(needle),
            "Theme visual baseline page should keep visual-quality contract token `{needle}`.",
        );
    }

    for needle in ["\"ThemeVisualBaseline\"", "\"theme-visual-baseline\""] {
        assert!(
            pages_registry.contains(needle),
            "Docs pages registry should expose theme visual baseline route token `{needle}`.",
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
    ] {
        assert!(
            e2e_spec.contains(needle),
            "Theme visual baseline e2e contract should include `{needle}`.",
        );
    }

    for needle in [
        "# HeroUI 参数设计风格对齐策略",
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
        "HeroUI 对齐结论",
    ] {
        assert!(
            heroui_doc.contains(needle),
            "HeroUI strategy doc should keep alignment constraint `{needle}`.",
        );
    }
}

#[test]
fn skeleton_group_type_system_and_semantic_markers_form_machine_readable_contract() {
    let mod_source = load_source("src/skeleton/group/mod.rs");
    let logic_source = load_source("src/skeleton/group/logic.rs");
    let view_source = load_source("src/skeleton/group/view.rs");

    for needle in [
        "pub struct SkeletonGroupStateInput {",
        "pub variant: SkeletonGroupVariant,",
        "pub layout: SkeletonGroupLayout,",
        "pub density: SkeletonGroupDensity,",
        "pub has_custom_is_loading: bool,",
        "pub has_custom_is_skeleton_only: bool,",
        "pub has_custom_variant: bool,",
        "pub has_custom_layout: bool,",
        "pub has_custom_density: bool,",
        "pub struct SkeletonGroupState {",
        "pub state_attr: &'static str,",
        "pub visibility_attr: &'static str,",
        "pub loading_mode_attr: &'static str,",
        "pub loading_source_attr: &'static str,",
        "pub skeleton_only_source_attr: &'static str,",
        "pub variant_source_attr: &'static str,",
        "pub layout_source_attr: &'static str,",
        "pub density_source_attr: &'static str,",
    ] {
        assert!(
            mod_source.contains(needle),
            "skeleton-group state contract should include `{needle}`."
        );
    }

    for needle in [
        "pub enum SkeletonGroupVariant {",
        "pub enum SkeletonGroupLayout {",
        "pub enum SkeletonGroupDensity {",
        "pub struct SkeletonGroupViewInput {",
        "pub variant: Option<SkeletonGroupVariant>,",
        "pub layout: Option<SkeletonGroupLayout>,",
        "pub density: Option<SkeletonGroupDensity>,",
        "pub fn normalize_state_input(input: SkeletonGroupViewInput) -> SkeletonGroupStateInput",
        "variant: input.variant.unwrap_or_default(),",
        "layout: input.layout.unwrap_or_default(),",
        "density: input.density.unwrap_or_default(),",
        "has_custom_is_loading: input.is_loading.is_some(),",
        "has_custom_is_skeleton_only: input.is_skeleton_only.is_some(),",
        "has_custom_variant: input.variant.is_some(),",
        "has_custom_layout: input.layout.is_some(),",
        "has_custom_density: input.density.is_some(),",
    ] {
        assert!(
            logic_source.contains(needle),
            "skeleton-group logic should keep typed input normalization contract `{needle}`."
        );
    }

    for forbidden in [
        "pub variant: Option<String>",
        "pub layout: Option<String>",
        "pub density: Option<String>",
        "variant: String",
        "layout: String",
        "density: String",
        "match variant.as_str()",
        "match layout.as_str()",
        "match density.as_str()",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "skeleton-group should not rely on string protocol for state typing/markers (`{forbidden}`).",
        );
    }

    for needle in [
        "data-state=state.state_attr",
        "data-visibility=state.visibility_attr",
        "data-loading-mode=state.loading_mode_attr",
        "data-loading-source=state.loading_source_attr",
        "data-skeleton-only-source=state.skeleton_only_source_attr",
        "data-variant=state.variant_attr",
        "data-variant-source=state.variant_source_attr",
        "data-layout=state.layout_attr",
        "data-layout-source=state.layout_source_attr",
        "data-density=state.density_attr",
        "data-density-source=state.density_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "skeleton-group view should expose machine-readable semantic marker `{needle}`.",
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
        r#"description="baseline-style skeleton coordination container with centralized loading/layout/variant visibility contracts and stable slot/data-state markers.""#,
        r#"title="Shimmer + Pulse Layout""#,
        r#"title="Loaded + Skeleton Only""#,
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
        "is_shimmer=false",
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
