use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join(rel_path).exists()
}

#[test]
fn surface_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/surface/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Surface internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn surface_uses_primitive_and_headless_contract_layers() {
    let logic_source = load_source("src/surface/logic.rs");
    let view_source = load_source("src/surface/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/surface.rs");
    let headless_source = load_source("../ui-headless/src/surface.rs");

    for needle in [
        "pub use ui_state_primitives::surface::{",
        "pub struct SurfaceControlInput",
        "pub struct SurfaceRootInput",
        "pub fn normalize_control_state(",
        "pub fn normalize_root_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Surface logic should include `{needle}` for layered normalization."
        );
    }

    for needle in [
        "pub struct SurfaceStateInput",
        "pub struct SurfaceState",
        "pub fn resolve_state(input: SurfaceStateInput) -> SurfaceState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Surface state primitive should include `{needle}`."
        );
    }

    for needle in [
        "pub struct SurfaceOptions",
        "pub struct SurfaceContract",
        "pub fn use_surface(options: SurfaceOptions) -> SurfaceContract",
    ] {
        assert!(
            headless_source.contains(needle),
            "Surface headless contract should include `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, SurfaceOptions, use_surface};",
        "logic::normalize_root_state(logic::SurfaceRootInput {",
        "control: logic::SurfaceControlInput {",
        "let semantics = use_surface(SurfaceOptions {",
        "data-bordered-source=bordered_source_attr",
        "data-padded-source=padded_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Surface view should mount normalized/headless outputs via `{needle}`."
        );
    }
}

#[test]
fn surface_component_files_follow_layered_responsibilities() {
    let mod_source = load_source("src/surface/mod.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let styles_source = load_source("src/surface/styles.rs");
    let view_source = load_source("src/surface/view.rs");
    let motion_source = load_source("src/surface/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::SurfaceMotion;",
        "pub use view::Surface;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Surface mod boundary should include `{needle}`."
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;"] {
        assert!(
            !mod_source.contains(forbidden),
            "Surface mod boundary should keep internals private; found `{forbidden}`."
        );
    }

    for forbidden in ["view! {", "NodeRef<", "on:click", "style="] {
        assert!(
            !logic_source.contains(forbidden),
            "Surface logic should avoid DOM/view/runtime style detail `{forbidden}`."
        );
    }

    for forbidden in ["view! {", "on:click", "on:keydown", "NodeRef<"] {
        assert!(
            !styles_source.contains(forbidden),
            "Surface styles should stay static and avoid `{forbidden}`."
        );
    }

    for needle in [
        "logic::normalize_root_state(logic::SurfaceRootInput {",
        "let semantics = use_surface(SurfaceOptions {",
        "super::motion::attach_motion(node_ref, motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "Surface view should assemble logic/headless/motion via `{needle}`."
        );
    }

    for forbidden in ["pub fn resolve_state(", "pub struct SurfaceStateInput"] {
        assert!(
            !view_source.contains(forbidden),
            "Surface view should not reimplement state primitives via `{forbidden}`."
        );
    }

    for needle in [
        "pub struct SurfaceMotion",
        "pub fn sanitize_motion(motion: SurfaceMotion) -> SurfaceMotion",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "Surface motion layer should include `{needle}`."
        );
    }

    for forbidden in ["view! {", "on:click", "on:keydown", "SpringAnimator::new"] {
        assert!(
            !motion_source.contains(forbidden),
            "Surface motion layer should remain contract+attach only; found `{forbidden}`."
        );
    }
}

#[test]
fn surface_spec_boundary_reuses_button_spec_without_local_spec_file() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let button_spec_path = manifest_dir.join("../ui-components/src/button/spec.rs");
    let button_mod_source = load_source("../ui-components/src/button/mod.rs");
    let surface_mod_source = load_source("src/surface/mod.rs");

    assert!(
        button_spec_path.exists(),
        "button should keep canonical spec.rs boundary for complex schema contracts."
    );
    assert!(
        !manifest_dir.join("src/surface/spec.rs").exists(),
        "Surface should not introduce local spec.rs."
    );

    for needle in [
        "pub mod spec;",
        "pub use spec::{ButtonA11y, ButtonAction, ButtonIntent, ButtonSchema, ButtonSpec, ButtonText};",
    ] {
        assert!(
            button_mod_source.contains(needle),
            "button module should keep canonical spec export `{needle}`."
        );
    }

    for forbidden in ["mod spec;", "pub mod spec;", "SurfaceSpec", "SurfaceSchema"] {
        assert!(
            !surface_mod_source.contains(forbidden),
            "Surface module should avoid local spec boundary token `{forbidden}`."
        );
    }
}

#[test]
fn surface_api_naming_keeps_is_prefixed_props_with_compatibility_path() {
    let view_source = load_source("src/surface/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");

    for needle in [
        "#[prop(optional)] is_bordered: Option<bool>",
        "#[prop(optional)] is_padded: Option<bool>",
    ] {
        assert!(
            view_source.contains(needle),
            "Surface props should expose migration-compatible naming contract `{needle}`."
        );
    }

    for needle in ["is_bordered=true", "is_padded=false"] {
        assert!(
            docs_source.contains(needle),
            "Surface docs should demonstrate `is_*` naming contract via `{needle}`."
        );
    }
}

#[test]
fn surface_emits_baseline_style_and_a11y_data_markers() {
    let source = load_source("src/surface/view.rs");

    for attr in [
        "data-slot=\"surface\"",
        "data-tone=semantics.attrs.data_tone",
        "data-elevation=semantics.attrs.data_elevation",
        "data-state=semantics.attrs.data_state",
        "data-bordered=semantics.attrs.data_bordered",
        "data-padded=semantics.attrs.data_padded",
        "data-plain=semantics.attrs.data_plain",
        "data-aria-source=semantics.attrs.data_aria_source",
        "data-custom-class=semantics.attrs.data_custom_class",
        "data-class-source=semantics.attrs.data_class_source",
        "data-motion-source=motion_source",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "lang=semantics.attrs.lang",
        "dir=semantics.attrs.dir",
    ] {
        assert!(
            source.contains(attr),
            "Surface should expose semantic marker `{attr}`."
        );
    }
}

#[test]
fn surface_styles_include_state_markers_and_token_variables() {
    let source = load_source("src/surface/styles.rs");

    for selector in [
        ".ui-surface--tone-default",
        ".ui-surface[data-tone=\"subtle\"]",
        ".ui-surface--elevation-raised",
        ".ui-surface[data-elevation=\"floating\"]",
        ".ui-surface--bordered",
        ".ui-surface[data-bordered=\"true\"]",
        ".ui-surface--padded",
        ".ui-surface[data-state=\"framed\"]",
        ".ui-surface--custom-class",
        ".ui-surface[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Surface styles should include `{selector}` as stable contracts."
        );
    }

    for token in [
        "var(--ui-radius-md)",
        "var(--ui-bg)",
        "var(--ui-fg)",
        "var(--ui-shadow-sm)",
        "var(--ui-shadow-md)",
    ] {
        assert!(
            source.contains(token),
            "Surface styles should remain token-first and include `{token}`."
        );
    }
}

#[test]
fn surface_token_first_styles_flow_through_css_pipeline_and_ui_root_injection() {
    let styles_source = load_source("src/surface/styles.rs");
    let css_source = load_source("src/css.rs");
    let lib_source = load_source("src/lib.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-bg)",
        "var(--ui-fg)",
        "var(--ui-radius-md)",
    ] {
        assert!(
            styles_source.contains(needle),
            "Surface styles should remain token-first and include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-surface\")]",
        "out.push_str(crate::surface::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "Surface styles should be aggregated in css.rs via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-layout public css bridge should include `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should inject aggregated component css via `{needle}`."
        );
    }
}

#[test]
fn surface_component_contract_avoids_utility_first_and_css_in_rust_patterns() {
    let styles_source = load_source("src/surface/styles.rs");
    let view_source = load_source("src/surface/view.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let motion_source = load_source("src/surface/motion.rs");

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"p-",
        "class=\"m-",
        "class=\"text-",
        "class=\"bg-",
        "@apply ",
        "tailwind",
    ] {
        assert!(
            !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Surface component sources should avoid utility-first contract leakage `{forbidden}`."
        );
    }

    for forbidden in [
        "css!(", "styled!(", "stylex", "emotion", "stylist", "linaria",
    ] {
        assert!(
            !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Surface component sources should avoid CSS-in-Rust default pattern `{forbidden}`."
        );
    }
}

#[test]
fn surface_styles_avoid_dom_guessing_and_view_avoids_business_inline_styles() {
    let styles_source = load_source("src/surface/styles.rs");
    let view_source = load_source("src/surface/view.rs");

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        ":first-child",
        ":last-child",
        " > ",
        "+ .",
        "~ .",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Surface styles should avoid DOM-structure guessing selector `{forbidden}`."
        );
    }

    assert!(
        !view_source.contains("style="),
        "Surface view should not include business inline style logic."
    );
}

#[test]
fn surface_docs_page_covers_primary_playgrounds() {
    let layout_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let docs =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");

    assert!(
        layout_extra.contains("pub(super) fn surface() -> AnyView"),
        "layout_extra should expose surface route entry.",
    );

    for needle in [
        "pub(super) fn surface() -> AnyView",
        "title=\"Surface\"",
        "slug=\"surface\"",
        "description=\"baseline-style foundational container surface with centralized tone/elevation/frame/source contracts and stable data markers.\"",
        "<Playground title=\"Tone + Elevation + Frame\" code_signal=tone_code>",
        "<Playground title=\"Custom Aria + Class\" code_signal=custom_code>",
        "<Surface",
    ] {
        assert!(
            docs.contains(needle),
            "layout_extra_surface docs should include `{needle}`."
        );
    }
}

#[test]
fn surface_visual_desire_reuses_theme_visual_baseline_and_heroui_contracts() {
    let baseline_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let baseline_e2e_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "mod theme_visual_baseline;",
        "\"theme-visual-baseline\",",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            baseline_registry_source.contains(needle),
            "docs component registry should include visual baseline route marker `{needle}`."
        );
    }

    for needle in [
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline",
        "Checks first-impression quality: hierarchy, spacing rhythm, contrast layers, and interactive feedback (hover/active/focus).",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            baseline_page_source.contains(needle),
            "theme visual baseline docs page should include `{needle}`."
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "[data-slot=\"theme-visual-baseline\"]",
        "[data-slot=\"theme-visual-baseline-button\"]",
        "[data-slot=\"theme-visual-baseline-input\"]",
        "[data-slot=\"theme-visual-baseline-overlay\"]",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            baseline_e2e_source.contains(needle),
            "theme visual baseline e2e regression contract should include `{needle}`."
        );
    }

    for needle in [
        "# HeroUI 参数设计风格对齐策略",
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
        "HeroUI 对齐结论",
    ] {
        assert!(
            heroui_strategy_source.contains(needle),
            "HeroUI strategy should include alignment constraint `{needle}`."
        );
    }
}

#[test]
fn surface_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_layout_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "component-surface = []",
        "\"component-surface\"",
        "inject-css = []",
    ] {
        assert!(
            ui_layout_cargo.contains(needle),
            "ui-layout Cargo features should include `{needle}` for tree-shaking boundaries."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-surface\")]\npub mod surface;"),
        "lib.rs should feature-gate surface module export for tree-shaking.",
    );

    for needle in [
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]",
        "#[cfg(feature = \"all-components\")]",
        "pub use web_demo_components::*;",
        "pub use all_components::*;",
    ] {
        assert!(
            lib_source.contains(needle),
            "lib.rs should keep feature-bounded export surface token `{needle}`."
        );
    }

    assert!(
        css_source.contains("#[cfg(feature = \"component-surface\")]")
            && css_source.contains("out.push_str(crate::surface::styles::CSS);"),
        "css.rs should gate surface CSS aggregation behind component-surface feature."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("pub fn push_components_css(out: &mut String)"),
        "css.rs should keep top-level inject-css gate for component CSS injection."
    );

    for forbidden in [
        "static ALL_COMPONENTS",
        "const ALL_COMPONENTS",
        "HashMap<&'static str, fn",
    ] {
        assert!(
            !lib_source.contains(forbidden) && !css_source.contains(forbidden),
            "global registry pattern that defeats DCE should stay absent `{forbidden}`."
        );
    }

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("web-demo-components")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui-layout via web-demo-components, not all-components."
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components instead of implicit default pull-up."
    );
}

#[test]
fn surface_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-layout-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui-layout -p ui-layout --no-default-features --features \"$MIN_FEATURES\"",
        "cargo tree -e features -i ui-layout -p web-demo",
        "cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui-layout --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "size regression",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking check script should include `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`."
        );
    }
}

#[test]
fn surface_type_system_and_semantic_markers_form_machine_readable_contract() {
    let primitives_source = load_source("../ui-state-primitives/src/surface.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let headless_source = load_source("../ui-headless/src/surface.rs");
    let view_source = load_source("src/surface/view.rs");

    for required in [
        "pub enum SurfaceTone",
        "pub enum SurfaceElevation",
        "pub struct SurfaceStateInput",
        "pub struct SurfaceState",
        "pub fn resolve_state(input: SurfaceStateInput) -> SurfaceState",
    ] {
        assert!(
            primitives_source.contains(required),
            "Surface primitive contract should keep type-constrained state token `{required}`."
        );
    }

    for required in [
        "pub struct SurfaceControlInput",
        "pub struct SurfaceControlState",
        "pub struct SurfaceRootInput",
        "pub struct SurfaceRootState",
        "pub fn normalize_control_state(",
        "pub fn normalize_root_state(",
    ] {
        assert!(
            logic_source.contains(required),
            "Surface logic should keep normalized typed state token `{required}`."
        );
    }

    for required in [
        "pub struct SurfaceAttrs",
        "pub struct SurfaceSemanticState",
        "pub struct SurfaceContract",
        "pub struct SurfaceOptions",
        "pub fn use_surface(options: SurfaceOptions) -> SurfaceContract",
    ] {
        assert!(
            headless_source.contains(required),
            "Surface headless semantic contract should keep typed output token `{required}`."
        );
    }

    for forbidden in [
        "tone: Option<String>",
        "elevation: Option<String>",
        "tone: String",
        "elevation: String",
        "data_tone: String",
        "data_elevation: String",
    ] {
        assert!(
            !primitives_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !headless_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "Surface should avoid stringly-typed discrete state axis `{forbidden}`."
        );
    }

    for marker in [
        "data-tone=semantics.attrs.data_tone",
        "data-elevation=semantics.attrs.data_elevation",
        "data-state=semantics.attrs.data_state",
        "data-bordered=semantics.attrs.data_bordered",
        "data-padded=semantics.attrs.data_padded",
        "data-plain=semantics.attrs.data_plain",
        "data-aria-source=semantics.attrs.data_aria_source",
        "data-class-source=semantics.attrs.data_class_source",
        "data-bordered-source=bordered_source_attr",
        "data-padded-source=padded_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "Surface machine-readable semantic contract should expose marker `{marker}`."
        );
    }
}

#[test]
fn surface_closed_state_sets_and_tests_make_contract_regressions_locatable() {
    let primitives_source = load_source("../ui-state-primitives/src/surface.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let headless_source = load_source("../ui-headless/src/surface.rs");

    for closed_value in [
        "\"default\"",
        "\"subtle\"",
        "\"strong\"",
        "\"flat\"",
        "\"raised\"",
        "\"floating\"",
        "\"framed\"",
        "\"bordered\"",
        "\"padded\"",
        "\"plain\"",
        "\"custom\"",
    ] {
        assert!(
            primitives_source.contains(closed_value) || headless_source.contains(closed_value),
            "Surface state/source markers should use closed enumerable values; missing `{closed_value}`."
        );
    }

    for test_name in [
        "fn resolve_state_tracks_state_and_source_markers()",
        "fn normalize_root_state_centralizes_defaults_and_sources()",
        "fn use_surface_maps_region_locale_and_state_markers()",
    ] {
        assert!(
            primitives_source.contains(test_name)
                || logic_source.contains(test_name)
                || headless_source.contains(test_name),
            "Surface state contract should keep targeted regression test `{test_name}`."
        );
    }
}

#[test]
fn surface_platform_guards_keep_non_wasm_files_web_sys_free() {
    let mod_source = load_source("src/surface/mod.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let styles_source = load_source("src/surface/styles.rs");
    let view_source = load_source("src/surface/view.rs");

    for forbidden in [
        "web_sys",
        "wasm_bindgen",
        "window()",
        "document()",
        "js_sys",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "non-wasm Surface files should stay browser-object free; found `{forbidden}`."
        );
    }
}

#[test]
fn surface_motion_covers_wasm_and_non_wasm_contract_paths() {
    let motion_source = load_source("src/surface/motion.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "Surface motion should keep explicit platform contract token `{needle}`."
        );
    }
}

#[test]
fn surface_platform_check_script_covers_default_ssr_wasm_compile_paths() {
    let script_source = load_source("../../scripts/check-ui-layout-platforms.sh");

    for needle in [
        "echo \"[platform] compile-only: default native path\"",
        "cargo check -p ui-layout",
        "echo \"[platform] compile-only: ssr native path\"",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "echo \"[platform] compile-only: web wasm path (ui-headless)\"",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "echo \"[platform] compile-only: web wasm path\"",
        "cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css",
        "echo \"[platform] source guard: non-wasm button files must not reference web_sys\"",
        "if rg -n \"web_sys\" \"$file\" >/dev/null; then",
    ] {
        assert!(
            script_source.contains(needle),
            "platform check script should include `{needle}` for native/ssr/wasm compile-only evidence."
        );
    }
}

#[test]
fn surface_ui_headless_web_ssr_mutex_is_compile_error_guarded() {
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should keep web/ssr mutex compile guard `{needle}`."
        );
    }
}

#[test]
fn surface_platform_script_enforces_ui_headless_web_ssr_mutex() {
    let script_source = load_source("../../scripts/check-ui-layout-platforms.sh");

    for needle in [
        "[platform] compile-only: ssr native path",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "[platform] compile-only: web wasm path (ui-headless)",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "[platform] compile guard: ui-headless web+ssr must fail",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "mutually exclusive",
    ] {
        assert!(
            script_source.contains(needle),
            "platform guard script should enforce ui-headless feature mutex via `{needle}`."
        );
    }
}

#[test]
fn surface_ui_motion_non_wasm_stub_contract_is_explicit_and_predictable() {
    let motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "Compile on non-wasm targets (no-op stubs) to support SSR/tooling builds.",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`."
        );
    }
}

#[test]
fn surface_platform_script_covers_ui_motion_native_wasm_and_stub_paths() {
    let script_source = load_source("../../scripts/check-ui-layout-platforms.sh");

    for needle in [
        "echo \"[platform] compile-only: ui-motion native path\"",
        "cargo check -p ui-motion",
        "echo \"[platform] compile-only: ui-motion wasm path\"",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "echo \"[platform] ui-motion non-wasm stub tests\"",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            script_source.contains(needle),
            "platform script should cover ui-motion stub/compile path token `{needle}`."
        );
    }
}

#[test]
fn surface_reduced_motion_ssr_wasm_branches_are_covered_without_semantic_split() {
    let view_source = load_source("src/surface/view.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let styles_source = load_source("src/surface/styles.rs");
    let motion_source = load_source("src/surface/motion.rs");
    let ui_motion_spring_source = load_source("../ui-motion/src/spring.rs");
    let ui_motion_spring_tests_source = load_source("../ui-motion/tests/spring.rs");
    let platform_script_source = load_source("../../scripts/check-ui-layout-platforms.sh");

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            ui_motion_spring_source.contains(needle),
            "ui-motion spring should keep reduced-motion downgrade behavior token `{needle}`.",
        );
    }

    for needle in [
        "fn reduced_motion_set_target_applies_immediately()",
        "fn reduced_motion_set_target_triggers_on_rest_synchronously()",
        "fn reduced_motion_clear_on_rest_stops_triggering()",
    ] {
        assert!(
            ui_motion_spring_tests_source.contains(needle),
            "ui-motion reduced-motion regression tests should include `{needle}`.",
        );
    }

    for needle in [
        "data-slot=\"surface\"",
        "data-tone=semantics.attrs.data_tone",
        "data-elevation=semantics.attrs.data_elevation",
        "data-state=semantics.attrs.data_state",
        "data-motion-source=motion_source",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "lang=semantics.attrs.lang",
        "dir=semantics.attrs.dir",
    ] {
        assert!(
            view_source.contains(needle),
            "Surface should keep hydration-stable semantic marker `{needle}` across SSR/wasm paths.",
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "cfg!(target_arch = \"wasm32\")",
        "prefers_reduced_motion(",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "Surface semantic surface should not split by platform/reduced-motion token `{forbidden}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "Surface motion adapter should keep wasm/non-wasm split token `{needle}`.",
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep reduced-motion/SSR/wasm verification token `{needle}`.",
        );
    }
}

#[test]
fn surface_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../crates/ui-headless/src/perf.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/surface/check2.md");
    let script_source = load_source("../../scripts/check-ui-layout-performance.sh");
    let view_source = load_source("src/surface/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "max_update_ms: Some(10.0),",
        "_ => UiPerfBudget::mount_only(120.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "component shell should keep performance budget contract token `{needle}`."
        );
    }

    {
        let needle = "component_doc!(\"Surface\", \"surface\", \"Layout\", layout_extra::surface)";
        assert!(
            pages_source.contains(needle),
            "Surface docs page should remain in coverage traversal via `{needle}`."
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
        "\"mount-only\"",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose repeatable perf regression marker `{needle}`."
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_source.contains(needle),
            "docs coverage e2e should enforce perf regression guard `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep explicit render_count follow-up marker `{needle}`."
        );
    }

    for needle in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep perf budget/follow-up governance token `{needle}`."
        );
    }

    for needle in [
        "cargo test -p ui-layout --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-layout --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui-layout --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }

    for needle in [
        "data-state=semantics.attrs.data_state",
        "data-tone=semantics.attrs.data_tone",
        "data-elevation=semantics.attrs.data_elevation",
        "data-aria-source=semantics.attrs.data_aria_source",
        "data-class-source=semantics.attrs.data_class_source",
        "data-motion-source=motion_source",
    ] {
        assert!(
            view_source.contains(needle),
            "Surface view should expose attribution marker `{needle}` for perf triage."
        );
    }
}

#[test]
fn surface_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders() {
    let view_source = load_source("src/surface/view.rs");

    assert!(
        view_source.contains("view! {"),
        "Surface should keep a single explicit render block in view.rs."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        1,
        "Surface should keep one small `view!` block; no giant macro split needed for current scope."
    );
    assert!(
        view_source.lines().count() <= 120,
        "Surface view.rs should stay compact; if this grows significantly, split into semantic subrenders."
    );

    for forbidden in [
        "for item in",
        "collect::<Vec<_>>()",
        ".map(|",
        "match children",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Surface view should avoid loop-heavy/expansion-heavy rendering token `{forbidden}`."
        );
    }
}

#[test]
fn surface_view_functional_split_prefers_no_extra_local_components_for_simple_layout() {
    let view_source = load_source("src/surface/view.rs");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Surface should keep a single public component boundary for current simple layout."
    );

    for forbidden in [
        "#[component]\nfn render_",
        "#[component]\nfn surface_",
        "pub fn render_",
        "fn render_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Surface should not introduce extra local components/render API noise for simple layout `{forbidden}`."
        );
    }

    for needle in ["children: Children", "{children()}"] {
        assert!(
            view_source.contains(needle),
            "Surface should keep explicit simple composition marker `{needle}`."
        );
    }
}

#[test]
fn surface_static_fragments_are_constantized_or_absent_for_simple_container_layout() {
    let view_source = load_source("src/surface/view.rs");

    for forbidden in [
        "inner_html=",
        "<svg",
        "<path",
        "<footer",
        "<article",
        "<aside",
        "let markdown",
        "let long_text",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Surface view should avoid heavy inline static fragments for simple container layout `{forbidden}`."
        );
    }

    assert_eq!(
        view_source.matches("data-slot=\"surface\"").count(),
        1,
        "Surface should keep a single stable static slot literal for traceable static fragment scope."
    );

    for needle in [
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "lang=semantics.attrs.lang",
        "dir=semantics.attrs.dir",
    ] {
        assert!(
            view_source.contains(needle),
            "Surface should keep centralized static a11y semantics via `{needle}`."
        );
    }
}

#[test]
fn surface_check2_marks_static_fragment_constantization_complete_with_na_evidence() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "可判定为纯静态的片段应避免重复动态构造。",
        "常量化后仍需维持可访问语义（title/aria-label/role 等）。",
        "静态资源变更路径要清晰，避免散落在多个 `view!` 片段中。",
        "`surface/view.rs` 当前仅保留单一 `section` 容器 + `children()` 槽位",
        "surface_static_fragments_are_constantized_or_absent_for_simple_container_layout",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep static-fragment constantization completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_inner_html_usage_is_absent_and_untrusted_html_paths_are_blocked() {
    let mod_source = load_source("src/surface/mod.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let view_source = load_source("src/surface/view.rs");
    let styles_source = load_source("src/surface/styles.rs");
    let motion_source = load_source("src/surface/motion.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "markdown_to_html(",
        "format!(\"<",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Surface component implementation should forbid HTML injection path `{forbidden}`."
        );
    }

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Surface docs should not introduce untrusted HTML injection path `{forbidden}`."
        );
    }

    for needle in [
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "lang=semantics.attrs.lang",
        "dir=semantics.attrs.dir",
    ] {
        assert!(
            view_source.contains(needle),
            "Surface should keep semantic mounting without inner_html fallback via `{needle}`."
        );
    }
}

#[test]
fn surface_check2_marks_inner_html_guardrails_complete_with_security_evidence() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "仅允许编译期常量或明确白名单内容进入 `inner_html`。",
        "严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。",
        "使用 `inner_html` 的节点必须补语义测试与安全回归说明。",
        "surface_inner_html_usage_is_absent_and_untrusted_html_paths_are_blocked",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep inner_html safety completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_wasm_debug_capability_stays_feature_isolated_and_non_polluting() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let mod_source = load_source("src/surface/mod.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let styles_source = load_source("src/surface/styles.rs");
    let view_source = load_source("src/surface/view.rs");
    let motion_source = load_source("src/surface/motion.rs");

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "#[cfg(target_arch = \"wasm32\")]\nmod observability;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui-layout should keep wasm debug capability isolated via `{needle}`."
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-layout Cargo features should keep explicit wasm-debug opt-in marker `{needle}`."
        );
    }

    assert!(
        !cargo_source.contains("surface-wasm-debug"),
        "Surface should not expose a dedicated wasm-debug feature because it is a non-interactive container."
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");
    for forbidden in [
        "wasm_debug_proxy!",
        "observability::",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
        "request_replay",
        "tracing::",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Surface production contract should not leak wasm-debug internals `{forbidden}`."
        );
    }
}

#[test]
fn surface_wasm_debug_observability_reuses_global_trace_overlay_with_timestamped_events() {
    let view_source = load_source("src/surface/view.rs");
    let docs_lib_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");

    for marker in [
        "data-tone=semantics.attrs.data_tone",
        "data-elevation=semantics.attrs.data_elevation",
        "data-state=semantics.attrs.data_state",
        "data-aria-source=semantics.attrs.data_aria_source",
        "data-class-source=semantics.attrs.data_class_source",
        "data-bordered-source=bordered_source_attr",
        "data-padded-source=padded_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "Surface should expose stable semantic/source markers for wasm-debug observability via `{marker}`."
        );
    }

    for forbidden in [
        "on:click=",
        "on:keydown=",
        "on:pointerdown=",
        "on:pointerup=",
        "request_replay",
        "data-slot=\"button-debug-replay\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Surface has no interactive replay path; non-applicable interaction token `{forbidden}` should remain absent."
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_lib_source.contains(needle),
            "docs-app should keep wasm dev visual-entry gate `{needle}`."
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "ui_headless::UiTraceEventKind::Inspect",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "docs debug overlay should keep wasm dev visual-entry/timeline marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "Inspect {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "let event = UiTraceEvent {",
        "ts_ms: now_ms(),",
        "if events.len() > MAX_EVENTS {",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace contract should keep timestamped/source event marker `{needle}`."
        );
    }
}

#[test]
fn surface_check2_marks_wasm_debug_governance_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "开发模式下至少能追踪关键状态变更来源与前后值。",
        "关键交互链路应支持最小可复现记录（事件顺序/状态转移）。",
        "调试开关默认不进入生产包体与公共 API。",
        "surface_wasm_debug_capability_stays_feature_isolated_and_non_polluting",
        "surface_wasm_debug_observability_reuses_global_trace_overlay_with_timestamped_events",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep wasm-debug completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn surface() -> AnyView",
        "<Playground title=\"Tone + Elevation + Frame\" code_signal=tone_code>",
        "<Playground title=\"Custom Aria + Class\" code_signal=custom_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "Surface docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
fn surface_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");

    for needle in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas contract marker `{needle}`."
        );
    }

    for forbidden in [
        "SURFACE_WORKBENCH_STORAGE_KEY",
        "load_surface_workbench_state(",
        "save_surface_workbench_state(",
        "clear_surface_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Surface is non-interactive; optional persisted state is N/A for this component scope, so `{forbidden}` should remain absent."
        );
    }
}

#[test]
fn surface_check2_marks_dx_governance_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
        "surface_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "surface_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep DX completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/surface/mod.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let view_source = load_source("src/surface/view.rs");
    let styles_source = load_source("src/surface/styles.rs");
    let motion_source = load_source("src/surface/motion.rs");
    let check2_source = load_source("src/surface/check2.md");

    assert!(
        !manifest_dir.join("src/surface/spec.rs").exists(),
        "Surface should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-surface = []"),
        "Surface feature should stay lightweight without serde/spec dependency fan-out."
    );
    assert!(
        !cargo_source.contains("component-surface = [\"dep:serde\"")
            && !cargo_source.contains("component-surface = [\"dep:serde_json\""),
        "Surface should not opt into serde/spec migration dependencies without an explicit schema contract."
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "schema_version",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Surface engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            check2_source.contains(required),
            "Surface checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
fn surface_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("../ui-components/src/button/view.rs");
    let combined = [
        load_source("src/surface/mod.rs"),
        load_source("src/surface/logic.rs"),
        load_source("src/surface/view.rs"),
        load_source("src/surface/styles.rs"),
        load_source("src/surface/motion.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_layout::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("surface-wasm-debug"),
        "Surface should not define component-local tracing feature when no local debug event/replay contract exists."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_layout::surface::",
        "const SURFACE_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Surface should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn surface_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/surface/mod.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let view_source = load_source("src/surface/view.rs");
    let styles_source = load_source("src/surface/styles.rs");
    let motion_source = load_source("src/surface/motion.rs");

    let sources = [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
    ];
    for source in sources {
        for forbidden in [
            "tokio",
            "tokio::",
            "async_std",
            "async_std::",
            "async-std",
            "runtime::Handle",
            "smol::",
            "spawn_blocking(",
        ] {
            assert!(
                !source.contains(forbidden),
                "Surface engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "Surface public module boundary should not leak web_sys types."
    );
}

#[test]
fn surface_check2_marks_engineering_governance_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
        "surface_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "surface_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "surface_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep engineering completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_check2_marks_ui_layout_entrypoint_rules_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] `ui-layout` 固定入口文件落点正确。",
        "`crates/ui-layout/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui-layout/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui-layout/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui-layout/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui-layout/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui-layout/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
        "surface_ui_layout_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks",
        "surface_ui_layout_css_registry_remains_feature_gated_and_non_global",
        "surface_ui_root_centralizes_theme_injection_and_i18n_context",
        "surface_active_highlight_stays_shared_motion_primitive_without_component_semantics",
        "surface_ui_layout_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep ui-layout entrypoint completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_component_directory_standard_files_are_present_and_no_render_rs_drift() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let surface_dir = manifest_dir.join("src/surface");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            surface_dir.join(required).exists(),
            "Surface directory should include `{required}`."
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !surface_dir.join(forbidden).exists(),
            "Surface directory should avoid drift file `{forbidden}`."
        );
    }
}

#[test]
fn surface_check2_marks_component_directory_standard_files_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] 组件目录标准文件落点正确。",
        "<component>/mod.rs`：最小稳定导出面，存在且无过度导出。",
        "<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。",
        "<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。",
        "<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。",
        "<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。",
        "<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。",
        "surface_component_files_follow_layered_responsibilities",
        "surface_component_directory_standard_files_are_present_and_no_render_rs_drift",
        "surface_spec_boundary_reuses_button_spec_without_local_spec_file",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep component-directory completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_check2_marks_agent_contract_schema_governance_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "surface_agent_contract_markers_are_schema_like_and_machine_readable",
        "surface_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "surface_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep Agent Contract completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_agent_contract_markers_are_schema_like_and_machine_readable() {
    let view_source = load_source("src/surface/view.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/surface.rs");
    let headless_source = load_source("../../crates/ui-headless/src/surface.rs");
    let combined = format!("{view_source}\n{logic_source}\n{primitives_source}\n{headless_source}");

    for marker in [
        "data-tone=semantics.attrs.data_tone",
        "data-elevation=semantics.attrs.data_elevation",
        "data-state=semantics.attrs.data_state",
        "data-aria-source=semantics.attrs.data_aria_source",
        "data-class-source=semantics.attrs.data_class_source",
        "data-bordered-source=bordered_source_attr",
        "data-padded-source=padded_source_attr",
        "data-motion-source=motion_source",
    ] {
        assert!(
            view_source.contains(marker),
            "Surface should expose agent-readable machine marker `{marker}`."
        );
    }

    for typed_source in [
        "pub enum SurfaceTone",
        "pub enum SurfaceElevation",
        "pub struct SurfaceStateInput",
        "pub struct SurfaceState",
        "pub struct SurfaceOptions",
        "pub struct SurfaceContract",
        "pub struct SurfaceSemanticState",
        "pub fn as_attr(self) -> &'static str",
        "pub fn resolve_state(input: SurfaceStateInput) -> SurfaceState",
    ] {
        assert!(
            combined.contains(typed_source),
            "Surface Agent Contract values should remain type-derived via `{typed_source}`."
        );
    }
}

#[test]
fn surface_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let view_source = load_source("src/surface/view.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/surface.rs");
    let headless_source = load_source("../../crates/ui-headless/src/surface.rs");
    let combined = format!("{view_source}\n{logic_source}\n{primitives_source}\n{headless_source}");

    // Surface is non-interactive/simple, so schema-like markers are state/source-only.
    // `data-ui-schema` is optional and should not be faked by ad-hoc free-form strings.
    for forbidden in [
        "data-ui-schema=",
        "data-ui-schema-version=",
        "data-ui-intent=",
        "data-ui-action=",
        "data-ui-state=",
        "data-ui-source=",
        "intent=\"",
        "action=\"",
        "format!(\"data-",
        "format!(\"{",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Surface should avoid free-form/fake schema field token `{forbidden}` in non-interactive scope."
        );
    }

    for forbidden_interaction in ["on:click", "on:keydown", "on:pointerdown", "on:pointerup"] {
        assert!(
            !view_source.contains(forbidden_interaction),
            "Surface has no interactive intent/action axis; token `{forbidden_interaction}` should remain absent."
        );
    }
}

#[test]
fn surface_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let mod_source = load_source("src/surface/mod.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let styles_source = load_source("src/surface/styles.rs");
    let view_source = load_source("src/surface/view.rs");
    let motion_source = load_source("src/surface/motion.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");
    let combined = format!(
        "{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}\n{docs_source}"
    );

    for forbidden in [
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Surface Agent Contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn surface_ui_layout_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks() {
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-surface\")]",
        "pub mod surface;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-layout lib entry should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod css;",
        "leptos::web_sys",
        "web_sys::",
        "wasm_bindgen",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-layout lib entry should not leak platform/internal marker `{forbidden}`."
        );
    }
}

#[test]
fn surface_ui_layout_css_registry_remains_feature_gated_and_non_global() {
    let css_source = load_source("src/css.rs");

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-surface\")]",
        "out.push_str(crate::surface::styles::CSS);",
        "#[cfg(feature = \"component-active_highlight\")]",
        "out.push_str(ui_visual_primitive::active_highlight::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-layout css registry should keep feature-gated marker `{needle}`."
        );
    }
}

#[test]
fn surface_ui_root_centralizes_theme_injection_and_i18n_context() {
    let root_source = load_source("src/root.rs");

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n};",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if let Some(overrides) = semantic_overrides.get_value() {",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "data-theme-scheme",
        "data-theme-color",
        "data-theme-system",
        "data-theme-scale",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized theme/i18n marker `{needle}`."
        );
    }
}

#[test]
fn surface_active_highlight_stays_shared_motion_primitive_without_component_semantics() {
    let source = load_source("../ui-visual-primitive/src/active_highlight.rs");

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "Accordion",
        "Button",
        "Sidebar",
        "aria-",
        "data-slot",
        "on:click",
    ] {
        assert!(
            !source.contains(forbidden),
            "active_highlight should stay generic and avoid component business marker `{forbidden}`."
        );
    }
}

#[test]
fn surface_ui_layout_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present() {
    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden),
            "ui-layout forbidden entrypoint file should not exist: `{forbidden}`."
        );
    }

    for required in [
        "../../crates/ui-headless/src/controllable_state.rs",
        "../../crates/ui-headless/src/presence.rs",
        "../../crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            path_exists(required),
            "ui-headless canonical primitive file should exist: `{required}`."
        );
    }

    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            controllable_state_source.contains(needle)
                || presence_source.contains(needle)
                || a11y_source.contains(needle),
            "headless canonical primitive files should keep marker `{needle}`."
        );
    }
}

#[test]
fn surface_semantics_suite_prioritizes_contract_assertions_over_snapshots() {
    let source = load_source("tests/surface_semantics.rs");

    let forbidden = [
        ["assert", "_snapshot!"].concat(),
        ["insta::", "assert", "_snapshot!"].concat(),
        ["toMatch", "Snapshot"].concat(),
        ["to_match_", "snapshot"].concat(),
    ];

    for forbidden in forbidden {
        assert!(
            !source.contains(&forbidden),
            "Surface semantic suite should not depend on visual snapshot assertion `{forbidden}`."
        );
    }
}

#[test]
fn surface_component_is_non_interactive_and_relies_on_semantic_markers_only() {
    let view_source = load_source("src/surface/view.rs");

    for forbidden in [
        "on:click",
        "on:keydown",
        "on:keyup",
        "on:pointerdown",
        "on:pointerup",
        "on:pointermove",
        "on:focus",
        "on:blur",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Surface is non-interactive; should not include interaction handler `{forbidden}`."
        );
    }

    for required in [
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
    ] {
        assert!(
            view_source.contains(required),
            "Surface should expose semantic contract marker `{required}`."
        );
    }
}

#[test]
fn surface_check2_marks_streaming_definition_complete_with_llm_mode_scope() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "surface_check2_marks_streaming_definition_complete_with_llm_mode_scope",
        "surface_streaming_protocol_tokens_are_absent_for_non_interactive_snapshot_container",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep streaming-definition completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_streaming_protocol_tokens_are_absent_for_non_interactive_snapshot_container() {
    let view_source = load_source("src/surface/view.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let motion_source = load_source("src/surface/motion.rs");
    let mod_source = load_source("src/surface/mod.rs");
    let combined = format!("{view_source}\n{logic_source}\n{motion_source}\n{mod_source}");

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "streaming",
        "fallback=snapshot",
        "data-stream",
        "data-output-status",
        "data-draft",
        "partial",
        "delta",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Surface is a non-interactive snapshot container and should not include streaming token `{forbidden}`."
        );
    }
}

#[test]
fn surface_check2_marks_snapshot_baseline_capability_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "surface_check2_marks_snapshot_baseline_capability_complete",
        "surface_snapshot_baseline_consumes_complete_configuration_and_renders_stably",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep snapshot-baseline completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_snapshot_baseline_consumes_complete_configuration_and_renders_stably() {
    let view_source = load_source("src/surface/view.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");
    let combined = format!("{view_source}\n{logic_source}\n{docs_source}");

    for required in [
        "children: Children",
        "{children()}",
        "#[prop(optional)] tone: SurfaceTone",
        "#[prop(optional)] elevation: SurfaceElevation",
        "logic::normalize_root_state(logic::SurfaceRootInput {",
        "Surface tone=SurfaceTone::Default elevation=SurfaceElevation::Raised",
        "Surface tone=SurfaceTone::Subtle elevation=SurfaceElevation::Flat is_bordered=true",
        "Surface tone=SurfaceTone::Strong elevation=SurfaceElevation::Floating is_padded=false",
    ] {
        assert!(
            combined.contains(required),
            "Surface should keep snapshot-baseline render path marker `{required}`."
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "data-stream",
        "data-output-status",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Surface snapshot baseline should stay protocol-agnostic without `{forbidden}`."
        );
    }
}

#[test]
fn surface_check2_marks_streaming_requirement_by_component_scope_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "surface_check2_marks_streaming_requirement_by_component_scope_complete",
        "surface_streaming_requirement_is_optional_with_snapshot_fallback_and_semantic_continuity",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep streaming-requirement completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_streaming_requirement_is_optional_with_snapshot_fallback_and_semantic_continuity() {
    let view_source = load_source("src/surface/view.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let motion_source = load_source("src/surface/motion.rs");
    let mod_source = load_source("src/surface/mod.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");
    let combined =
        format!("{view_source}\n{logic_source}\n{motion_source}\n{mod_source}\n{docs_source}");

    for required in [
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "data-state=semantics.attrs.data_state",
        "data-aria-source=semantics.attrs.data_aria_source",
        "data-class-source=semantics.attrs.data_class_source",
    ] {
        assert!(
            view_source.contains(required),
            "Surface should keep semantic continuity marker `{required}` when running as snapshot fallback container."
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "streaming",
        "data-stream",
        "data-output-status",
        "data-draft",
        "retry",
        "reconnect",
        "断线恢复",
        "fallback=snapshot",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Surface should keep streaming optional boundary in component scope without `{forbidden}`."
        );
    }
}

#[test]
fn surface_check2_marks_semantic_contract_first_testing_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
        "surface_emits_baseline_style_and_a11y_data_markers",
        "surface_type_system_and_semantic_markers_form_machine_readable_contract",
        "surface_semantics_suite_prioritizes_contract_assertions_over_snapshots",
        "surface_component_is_non_interactive_and_relies_on_semantic_markers_only",
        "surface_semantic_contract_markers_in_view_are_backed_by_semantics_suite_assertions",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep semantic-first testing completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_semantic_contract_markers_in_view_are_backed_by_semantics_suite_assertions() {
    let view_source = load_source("src/surface/view.rs");
    let semantics_test_source = load_source("tests/surface_semantics.rs");

    for marker in [
        "data-tone=semantics.attrs.data_tone",
        "data-elevation=semantics.attrs.data_elevation",
        "data-state=semantics.attrs.data_state",
        "data-bordered=semantics.attrs.data_bordered",
        "data-padded=semantics.attrs.data_padded",
        "data-plain=semantics.attrs.data_plain",
        "data-aria-source=semantics.attrs.data_aria_source",
        "data-custom-class=semantics.attrs.data_custom_class",
        "data-class-source=semantics.attrs.data_class_source",
        "data-bordered-source=bordered_source_attr",
        "data-padded-source=padded_source_attr",
        "data-motion-source=motion_source",
        "role=semantics.attrs.role",
        "aria-label=semantics.attrs.aria_label",
        "lang=semantics.attrs.lang",
        "dir=semantics.attrs.dir",
    ] {
        assert!(
            view_source.contains(marker),
            "Surface view should expose semantic marker `{marker}`."
        );
        assert!(
            semantics_test_source.contains(marker),
            "Surface semantics suite should assert semantic marker `{marker}` to prevent contract drift."
        );
    }
}

#[test]
fn surface_check2_marks_e2e_selector_stability_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
        "surface_e2e_surface_contract_uses_semantic_selectors_and_stable_wasm_wait_strategy",
        "surface_e2e_scope_marks_async_motion_ready_settled_as_na_for_non_interactive_container",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep E2E-stability completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_e2e_surface_contract_uses_semantic_selectors_and_stable_wasm_wait_strategy() {
    let e2e_source = load_source("../../e2e/tests/docs_app_surface.spec.mjs");

    for required in [
        "await page.goto(\"/#/components/surface\");",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "const surfaces = page.locator('section[data-slot=\"surface\"][data-state]');",
        "await expect(surfaces.first()).toHaveAttribute(\"data-slot\", \"surface\");",
        "await expect(first).toHaveAttribute(\"data-state\", \"padded\");",
        "await expect(second).toHaveAttribute(\"data-bordered-source\", \"is-prop\");",
        "await expect(third).toHaveAttribute(\"data-padded-source\", \"is-prop\");",
        "await expect(custom).toHaveAttribute(\"data-class-source\", \"custom\");",
    ] {
        assert!(
            e2e_source.contains(required),
            "Surface E2E contract should keep semantic selector/wait marker `{required}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        ".docs-page-title",
        ":nth-child(",
        "getByText(",
        "text=",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "Surface E2E contract should avoid fragile selector/wait token `{forbidden}`."
        );
    }
}

#[test]
fn surface_e2e_scope_marks_async_motion_ready_settled_as_na_for_non_interactive_container() {
    let e2e_source = load_source("../../e2e/tests/docs_app_surface.spec.mjs");
    let view_source = load_source("src/surface/view.rs");
    let motion_source = load_source("src/surface/motion.rs");

    for forbidden in [
        "data-loading",
        "aria-busy",
        "retry",
        "ready",
        "settled",
        "await expect(",
        "toHaveAttribute(\"data-motion-state\"",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "Surface motion scope should stay no-op/non-async and avoid token `{forbidden}`."
        );
    }

    for forbidden_interaction in ["on:click", "on:keydown", "on:pointerdown", "on:pointerup"] {
        assert!(
            !view_source.contains(forbidden_interaction),
            "Surface is non-interactive; async/ready-settled E2E branch is N/A for `{forbidden_interaction}`."
        );
    }

    for forbidden_e2e in ["waitForTimeout(", "setTimeout("] {
        assert!(
            !e2e_source.contains(forbidden_e2e),
            "Surface E2E should not fake async/motion settled flow token `{forbidden_e2e}`."
        );
    }
}

#[test]
fn surface_check2_marks_repeatable_e2e_key_flow_regression_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
        "surface_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "surface_e2e_high_risk_paths_are_na_for_non_interactive_container_scope",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep repeatable E2E key-flow completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_surface.spec.mjs");

    for required in [
        "test(\"docs-app: surface contract markers are observable\"",
        "await test.step(\"route-open-and-wasm-ready\"",
        "await test.step(\"surface-list-visible\"",
        "await test.step(\"checkpoint-default-raised\"",
        "await test.step(\"checkpoint-subtle-bordered\"",
        "await test.step(\"checkpoint-strong-plain\"",
        "await test.step(\"checkpoint-custom-aria-and-class\"",
        "test(\"docs-app: surface key flow is repeatable after reload\"",
        "await page.reload();",
        "await expect(reloadedFirstSurface).toHaveAttribute(\"data-state\", \"padded\");",
        "await expect(reloadedFirstSurface).toHaveAttribute(\"data-tone\", \"default\");",
    ] {
        assert!(
            e2e_source.contains(required),
            "Surface E2E should keep repeatable key-flow semantic checkpoint `{required}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "waitForTimeout(", "setTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "Surface E2E key flow should avoid non-semantic/flaky token `{forbidden}`."
        );
    }
}

#[test]
fn surface_e2e_high_risk_paths_are_na_for_non_interactive_container_scope() {
    let view_source = load_source("src/surface/view.rs");
    let motion_source = load_source("src/surface/motion.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_surface.spec.mjs");

    for forbidden in [
        "on:click",
        "on:keydown",
        "on:pointerdown",
        "on:pointerup",
        "on:focus",
        "on:blur",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Surface high-risk input path should stay N/A in non-interactive scope `{forbidden}`."
        );
    }

    for forbidden in ["aria-busy", "data-loading", "retry", "reconnect"] {
        assert!(
            !motion_source.contains(forbidden),
            "Surface high-risk async path should stay N/A in no-op motion scope `{forbidden}`."
        );
    }

    for forbidden in [
        "keyboard",
        "pointer",
        "overlay",
        "focus trap",
        "data-ready",
        "data-settled",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "Surface E2E key-flow should not fake unsupported high-risk path `{forbidden}`."
        );
    }
}

#[test]
fn surface_docs_examples_sync_with_logic_api_names_and_default_matrix() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let view_source = load_source("src/surface/view.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_surface.spec.mjs");

    for required in [
        "<Playground title=\"Tone + Elevation + Frame\" code_signal=tone_code>",
        "<Surface tone=SurfaceTone::Default elevation=SurfaceElevation::Raised>",
        "<Surface tone=SurfaceTone::Subtle elevation=SurfaceElevation::Flat is_bordered=true>",
        "<Surface tone=SurfaceTone::Strong elevation=SurfaceElevation::Floating is_padded=false>",
        "<Playground title=\"Custom Aria + Class\" code_signal=custom_code>",
        "is_bordered=true",
        "is_padded=false",
        "aria_label=\"Deployment summary\".to_string()",
        "class_name=\"docs-surface-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(required),
            "Surface docs should keep state/params matrix marker `{required}`."
        );
    }

    for forbidden in [" bordered=true", " padded=false"] {
        assert!(
            !docs_source.contains(forbidden),
            "Surface docs should avoid legacy API marker `{forbidden}` and prefer is_* props."
        );
    }

    for required in [
        "let (bordered, bordered_source_attr) = input",
        ".is_bordered",
        ".unwrap_or((false, \"default\"));",
        "let (padded, padded_source_attr) = input",
        ".is_padded",
        ".unwrap_or((true, \"default\"));",
    ] {
        assert!(
            logic_source.contains(required),
            "Surface logic should keep API compatibility/default precedence marker `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] is_bordered: Option<bool>",
        "#[prop(optional)] is_padded: Option<bool>",
    ] {
        assert!(
            view_source.contains(required),
            "Surface view should keep public API/default marker `{required}`."
        );
    }

    for required in [
        "await expect(first).toHaveAttribute(\"data-state\", \"padded\");",
        "await expect(second).toHaveAttribute(\"data-bordered-source\", \"is-prop\");",
        "await expect(third).toHaveAttribute(\"data-padded-source\", \"is-prop\");",
    ] {
        assert!(
            e2e_source.contains(required),
            "Surface docs regression should lock semantic matrix outcome `{required}`."
        );
    }
}

#[test]
fn surface_check2_marks_docs_examples_and_matrices_synced_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
        "surface_docs_page_covers_primary_playgrounds",
        "surface_docs_examples_sync_with_logic_api_names_and_default_matrix",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep docs-sync completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_check2_marks_interactive_playground_complete() {
    let check2_source = load_source("src/surface/check2.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_surface.spec.mjs");

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
        "tests/docs_app_surface.spec.mjs",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep interactive playground completion evidence `{needle}`."
        );
    }

    for needle in [
        "<Playground title=\"Tone + Elevation + Frame\" code_signal=tone_code>",
        "<Playground title=\"Custom Aria + Class\" code_signal=custom_code>",
        "let tone_code = Signal::derive(move || {",
        "let custom_code = Signal::derive(move || {",
    ] {
        assert!(
            docs_source.contains(needle),
            "Surface docs interactive playground should include `{needle}`."
        );
    }

    for needle in [
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground runtime should keep interactive controls/preview marker `{needle}`."
        );
    }

    for needle in [
        "tests/docs_app_surface.spec.mjs",
        "section[data-slot=\"surface\"][data-state]",
        "surface key flow is repeatable after reload",
    ] {
        assert!(
            check2_source.contains(needle) || e2e_source.contains(needle),
            "Interactive playground evidence should include `{needle}`."
        );
    }
}

#[test]
fn surface_docs_entry_exists_as_readme_or_equivalent_docs_app_page() {
    let has_readme = path_exists("src/surface/README.md");
    let has_docs_page =
        path_exists("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");

    assert!(
        has_readme || has_docs_page,
        "Surface must provide README or equivalent docs-app entry."
    );
    assert!(
        docs_source.contains("pub(super) fn surface() -> AnyView"),
        "Equivalent docs entry should expose surface page function."
    );
}

#[test]
fn surface_docs_are_beginner_friendly_with_default_then_advanced_path() {
    let readme_source = load_source("src/surface/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "组件文档必须对新手友好（Documentation as Product）",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法",
        "文档需明确“先用起来，再进阶”",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep documentation-as-product marker `{needle}`."
        );
    }

    for needle in [
        "## 先用起来（默认路径）",
        "### Hello World（最小可用）",
        "## 常见用法",
        "## 再进阶（高级控制）",
    ] {
        assert!(
            readme_source.contains(needle),
            "Surface README should include beginner-to-advanced marker `{needle}`."
        );
    }

    for needle in [
        "title=\"Surface\"",
        "slug=\"surface\"",
        "<Playground title=\"Tone + Elevation + Frame\" code_signal=tone_code>",
        "<Playground title=\"Custom Aria + Class\" code_signal=custom_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "Surface docs-app page should keep docs entry marker `{needle}`."
        );
    }

    let basic_pos = readme_source
        .find("## 先用起来（默认路径）")
        .expect("Surface README should include default path section");
    let common_pos = readme_source
        .find("## 常见用法")
        .expect("Surface README should include common usage section");
    let advanced_pos = readme_source
        .find("## 再进阶（高级控制）")
        .expect("Surface README should include advanced section");

    assert!(
        basic_pos < common_pos && common_pos < advanced_pos,
        "Surface README should present default usage before advanced controls."
    );
}

#[test]
fn surface_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring() {
    let readme_source = load_source("src/surface/README.md");
    let hello_marker = "### Hello World（最小可用）";
    let hello_start = readme_source
        .find(hello_marker)
        .expect("Surface README should include hello-world marker");
    let code_start = readme_source[hello_start..]
        .find("```rust")
        .map(|offset| hello_start + offset + "```rust".len())
        .expect("Surface README hello-world section should include rust snippet");
    let code_end = readme_source[code_start..]
        .find("```")
        .map(|offset| code_start + offset)
        .expect("Surface README hello-world snippet should terminate");
    let hello_snippet = &readme_source[code_start..code_end];
    let meaningful_lines = hello_snippet
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();

    assert!(
        meaningful_lines <= 5,
        "Surface Hello World snippet should stay <= 5 lines for beginner DX, got {meaningful_lines} lines:\n{hello_snippet}"
    );

    for forbidden in [
        "ui_state_primitives",
        "ui-headless",
        "ui_headless",
        "state=",
        "controller=",
        "Signal<",
    ] {
        assert!(
            !hello_snippet.contains(forbidden),
            "Surface Hello World path should not require architecture-level wiring `{forbidden}`."
        );
    }
}

#[test]
fn surface_check2_marks_documentation_as_product_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
        "surface_docs_entry_exists_as_readme_or_equivalent_docs_app_page",
        "surface_docs_are_beginner_friendly_with_default_then_advanced_path",
        "surface_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep documentation-as-product completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_docs_source_first_copy_paste_ready_with_imports_source_paths_and_sync() {
    let readme_source = load_source("src/surface/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_source = load_source("../ui-components/src/code_block/view.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let view_source = load_source("src/surface/view.rs");

    for needle in [
        "## Source-first Copy-Paste Ready",
        "component-surface",
        "crates/ui-layout/src/surface/mod.rs",
        "crates/ui-layout/src/surface/logic.rs",
        "crates/ui-layout/src/surface/view.rs",
        "crates/ui-layout/src/surface/styles.rs",
        "crates/ui-layout/src/surface/motion.rs",
    ] {
        assert!(
            readme_source.contains(needle),
            "Surface README should keep source-first copy-ready marker `{needle}`."
        );
    }

    for needle in [
        "<Playground title=\"Tone + Elevation + Frame\" code_signal=tone_code>",
        "<Playground title=\"Custom Aria + Class\" code_signal=custom_code>",
        "is_bordered=true",
        "is_padded=false",
    ] {
        assert!(
            docs_source.contains(needle),
            "Surface docs page should keep copy-paste example marker `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let resolved_code = Signal::derive(move || {",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground runtime should keep copy-paste-ready marker `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"code-block\"",
        "data-copyable=state.copyable.then_some(\"true\")",
        "data-slot=\"code-block-code\"",
        "class_name=\"ui-code-block__copy-button\".to_string()",
    ] {
        assert!(
            code_block_source.contains(needle),
            "CodeBlock should keep one-click copy marker `{needle}` for docs source-first flow."
        );
    }

    for needle in [
        "#[prop(optional)] is_bordered: Option<bool>",
        "#[prop(optional)] is_padded: Option<bool>",
        "let (bordered, bordered_source_attr) = input",
        "let (padded, padded_source_attr) = input",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "Surface source-first docs should stay synced with implementation marker `{needle}`."
        );
    }
}

#[test]
fn surface_check2_marks_source_first_copy_paste_ready_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
        "surface_docs_source_first_copy_paste_ready_with_imports_source_paths_and_sync",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep source-first copy-ready completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_entry_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let view_source = load_source("src/surface/view.rs");

    for needle in [
        "### Surface 同步记录（2026-02-17）",
        "参数模型同步：`Surface` 维持基础容器定位",
        "docs 入口同步：`apps/docs-app/src/pages/components/pages.rs`",
        "示例矩阵同步：`apps/docs-app/src/pages/components/pages/layout_extra_surface.rs`",
        "Copy-Paste Ready 同步：`Surface` 示例通过 `code_signal` 进入 `Playground`",
    ] {
        assert!(
            strategy_source.contains(needle),
            "HeroUI strategy should keep Surface sync marker `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"Surface\", \"surface\", \"Layout\", layout_extra::surface)",
        "pub(super) fn surface() -> AnyView",
        "slug=\"surface\"",
    ] {
        assert!(
            pages_index_source.contains(needle) || docs_entry_source.contains(needle),
            "Surface docs entry should remain indexable marker `{needle}`."
        );
    }

    for needle in [
        "tone: SurfaceTone",
        "elevation: SurfaceElevation",
        "is_bordered: Option<bool>",
        "is_padded: Option<bool>",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "Surface parameter model marker `{needle}` should stay in implementation."
        );
    }
}

#[test]
fn surface_check2_marks_heroui_strategy_and_component_docs_sync_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        "surface_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep HeroUI/docs-sync completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_anti_pattern_status_primitives_remains_dom_and_style_free() {
    let primitives_source = load_source("../ui-state-primitives/src/surface.rs");

    for forbidden in [
        "use leptos",
        "leptos::",
        "web_sys::",
        "wasm_bindgen",
        "view! {",
        "NodeRef<",
        "on:click",
        "style=",
    ] {
        assert!(
            !primitives_source.contains(forbidden),
            "ui-state-primitives surface contract should avoid DOM/style runtime dependency `{forbidden}`."
        );
    }
}

#[test]
fn surface_anti_pattern_ui_headless_remains_visual_and_motion_free() {
    let headless_source = load_source("../ui-headless/src/surface.rs");

    for forbidden in [
        ".ui-",
        "ui-surface",
        "class=",
        "var(--ui-",
        "Spring",
        "keyframe",
        "animate(",
        "request_animation_frame",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "ui-headless surface contract should avoid visual/motion orchestration token `{forbidden}`."
        );
    }

    for required in [
        "pub struct SurfaceAttrs",
        "pub struct SurfaceSemanticState",
        "pub struct SurfaceContract",
        "pub fn use_surface(options: SurfaceOptions) -> SurfaceContract",
    ] {
        assert!(
            headless_source.contains(required),
            "ui-headless surface contract should keep typed semantic output `{required}`."
        );
    }
}

#[test]
fn surface_anti_pattern_view_keeps_decisions_in_logic_layer() {
    let view_source = load_source("src/surface/view.rs");
    let logic_source = load_source("src/surface/logic.rs");

    for required in [
        "logic::normalize_root_state(logic::SurfaceRootInput {",
        "let semantics = use_surface(SurfaceOptions {",
    ] {
        assert!(
            view_source.contains(required),
            "Surface view should consume centralized logic/headless output via `{required}`."
        );
    }

    for forbidden in [
        "resolve_state(SurfaceStateInput",
        "normalize_aria_label(",
        "normalize_optional_text(",
        "legacy-prop",
        "if let Some(value) = input.is_bordered",
        "if let Some(value) = input.is_padded",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Surface view should not hide key state-decision rule `{forbidden}`."
        );
    }

    for required in [
        "let (bordered, bordered_source_attr) = input",
        "let (padded, padded_source_attr) = input",
    ] {
        assert!(
            logic_source.contains(required),
            "Surface key decision rule should stay centralized in logic layer `{required}`."
        );
    }
}

#[test]
fn surface_anti_pattern_new_params_follow_naming_type_default_and_semantic_contract() {
    let view_source = load_source("src/surface/view.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let primitives_source = load_source("../ui-state-primitives/src/surface.rs");
    let semantics_test_source = load_source("tests/surface_semantics.rs");

    for required in [
        "#[prop(optional)] is_bordered: Option<bool>",
        "#[prop(optional)] is_padded: Option<bool>",
    ] {
        assert!(
            view_source.contains(required),
            "Surface public parameter naming/default contract should include `{required}`."
        );
    }

    for required in [
        "let (bordered, bordered_source_attr) = input",
        ".is_bordered",
        ".unwrap_or((false, \"default\"));",
        "let (padded, padded_source_attr) = input",
        ".is_padded",
        ".unwrap_or((true, \"default\"));",
        "pub struct SurfaceStateInput",
        "pub struct SurfaceState",
    ] {
        assert!(
            logic_source.contains(required) || primitives_source.contains(required),
            "Surface parameter contract should keep naming/type/default normalization marker `{required}`."
        );
    }

    for required in [
        "surface_api_naming_keeps_is_prefixed_props_with_compatibility_path",
        "surface_type_system_and_semantic_markers_form_machine_readable_contract",
        "surface_docs_examples_sync_with_logic_api_names_and_default_matrix",
    ] {
        assert!(
            semantics_test_source.contains(required),
            "Surface semantics suite should keep parameter-contract regression guard `{required}`."
        );
    }
}

#[test]
fn surface_anti_pattern_parallel_array_api_is_absent_for_surface_scope() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra_surface.rs");
    let readme_source = load_source("src/surface/README.md");
    let view_source = load_source("src/surface/view.rs");

    for forbidden in [
        "labels + children",
        "titles + panels",
        "labels:",
        "titles:",
        "panels:",
        "ItemSpec",
    ] {
        assert!(
            !docs_source.contains(forbidden)
                && !readme_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "Surface scope should avoid parallel-array/implicit semantic token `{forbidden}`."
        );
    }
}

#[test]
fn surface_anti_pattern_public_api_does_not_leak_platform_or_runtime_types() {
    let mod_source = load_source("src/surface/mod.rs");
    let lib_source = load_source("src/lib.rs");

    for forbidden in [
        "web_sys::",
        "leptos::web_sys",
        "wasm_bindgen",
        "tokio::",
        "async_std::",
        "runtime::Handle",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !lib_source.contains(forbidden),
            "Surface public API boundary should avoid leaking platform/runtime token `{forbidden}`."
        );
    }
}

#[test]
fn surface_anti_pattern_no_temporary_patch_contract_drift_tokens_in_surface_scope() {
    let mod_source = load_source("src/surface/mod.rs");
    let logic_source = load_source("src/surface/logic.rs");
    let view_source = load_source("src/surface/view.rs");
    let styles_source = load_source("src/surface/styles.rs");
    let motion_source = load_source("src/surface/motion.rs");
    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");

    for forbidden in [
        "TODO temporary",
        "TEMP FIX",
        "HACK",
        "workaround",
        "quick fix",
        "remove later",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Surface should avoid temporary patch contract-drift marker `{forbidden}`."
        );
    }
}

#[test]
fn surface_anti_pattern_reusable_state_invariants_are_sunk_to_primitives_or_headless() {
    let logic_source = load_source("src/surface/logic.rs");
    let primitives_source = load_source("../ui-state-primitives/src/surface.rs");
    let headless_source = load_source("../ui-headless/src/surface.rs");

    for required in [
        "pub use ui_state_primitives::surface::{",
        "resolve_state(SurfaceStateInput {",
        "pub struct SurfaceStateInput",
        "pub struct SurfaceState",
        "pub struct SurfaceOptions",
        "pub struct SurfaceContract",
    ] {
        assert!(
            logic_source.contains(required)
                || primitives_source.contains(required)
                || headless_source.contains(required),
            "Surface reusable state invariant should stay sunk to primitive/headless marker `{required}`."
        );
    }

    for forbidden in ["pub enum LocalSurfaceState", "pub enum SurfaceMachine"] {
        assert!(
            !logic_source.contains(forbidden),
            "Surface logic should not keep reusable state machine locally `{forbidden}`."
        );
    }
}

#[test]
fn surface_check2_marks_forbidden_anti_patterns_complete() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 在 `view` 层隐藏关键状态决策。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "- [x] 公共 API 泄露底层实现细节类型。",
        "- [x] 用临时补丁破坏跨组件一致性。",
        "- [x] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
        "surface_anti_pattern_status_primitives_remains_dom_and_style_free",
        "surface_anti_pattern_ui_headless_remains_visual_and_motion_free",
        "surface_anti_pattern_view_keeps_decisions_in_logic_layer",
        "surface_anti_pattern_new_params_follow_naming_type_default_and_semantic_contract",
        "surface_anti_pattern_parallel_array_api_is_absent_for_surface_scope",
        "surface_anti_pattern_public_api_does_not_leak_platform_or_runtime_types",
        "surface_anti_pattern_no_temporary_patch_contract_drift_tokens_in_surface_scope",
        "surface_anti_pattern_reusable_state_invariants_are_sunk_to_primitives_or_headless",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep forbidden anti-pattern completion evidence `{needle}`."
        );
    }
}

#[test]
fn surface_check2_marks_final_merge_gates_complete_with_full_gate_done() {
    let check2_source = load_source("src/surface/check2.md");

    for needle in [
        "- [x] 架构正确（边界不破）。",
        "- [x] 行为正确（状态与交互语义成立）。",
        "- [x] 可访问性达标（默认可用）。",
        "- [x] 默认主题美学质量达标（与可访问性同级门禁）。",
        "- [x] 可测试（契约可断言）。",
        "- [x] 可维护（命名和模式一致）。",
        "- [x] 可解释（人和自动化都能读懂）。",
        "- [x] 改动在正确层。",
        "- [x] 命名与全库一致。",
        "- [x] 无效状态被限制或归一化。",
        "- [x] 暴露必要语义标记。",
        "- [x] 覆盖 reduced-motion / SSR / wasm 分支。",
        "- [x] 文档与示例同步更新。",
        "surface_component_files_follow_layered_responsibilities",
        "surface_type_system_and_semantic_markers_form_machine_readable_contract",
        "surface_emits_baseline_style_and_a11y_data_markers",
        "surface_visual_desire_reuses_theme_visual_baseline_and_heroui_contracts",
        "surface_check2_marks_semantic_contract_first_testing_complete",
        "surface_api_naming_keeps_is_prefixed_props_with_compatibility_path",
        "surface_semantic_contract_markers_in_view_are_backed_by_semantics_suite_assertions",
        "surface_reduced_motion_ssr_wasm_branches_are_covered_without_semantic_split",
        "surface_check2_marks_docs_examples_and_matrices_synced_complete",
    ] {
        assert!(
            check2_source.contains(needle),
            "Surface checklist should keep final-merge-gate evidence marker `{needle}`."
        );
    }

    for done in [
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "/root/.cargo/bin/cargo clippy -p ui-layout --no-default-features --features component-surface,inject-css -- -D warnings",
        "TMPDIR=/root/autodl-tmp/zjj/p/rust-ui/.tmp/rust-tmp /root/.cargo/bin/cargo test -p ui-layout --test surface_semantics --no-default-features --features component-surface,inject-css",
        "npx playwright test tests/docs_app_surface.spec.mjs --config=playwright.config.mjs --project=chromium",
    ] {
        assert!(
            check2_source.contains(done),
            "Surface checklist should keep full-gate completion marker `{done}`."
        );
    }
}
