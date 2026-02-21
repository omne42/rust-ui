use std::fs;
use std::path::PathBuf;

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn load_source(rel_path: &str) -> String {
    let path = crate_dir().join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    crate_dir().join(rel_path).exists()
}

fn function_signature(source: &str, fn_name: &str) -> String {
    let start = source
        .find(&format!("pub fn {fn_name}("))
        .unwrap_or_else(|| panic!("missing function signature for `{fn_name}`"));
    let end = source[start..]
        .find(") -> impl IntoView {")
        .unwrap_or_else(|| panic!("missing IntoView return marker for `{fn_name}`"));
    source[start..start + end].to_string()
}

#[test]
fn carousel_component_keeps_hyper_structure_file_split() {
    for rel_path in [
        "src/i18n.rs",
        "src/logic.rs",
        "src/view.rs",
        "src/styles.rs",
        "src/motion.rs",
        "test/logic.rs",
        "test/motion.rs",
        "test/protocol.rs",
        "test/semantics.rs",
    ] {
        let path = crate_dir().join(rel_path);
        assert!(
            path.exists(),
            "Carousel component should keep `{rel_path}` in the hyper-structure layout."
        );
    }
}

#[test]
fn carousel_simple_component_does_not_introduce_spec_rs() {
    let spec_path = crate_dir().join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "Carousel is a simple component and should not introduce `src/spec.rs`."
    );

    let mod_source = load_source("src/mod.rs");
    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "Carousel module should not expose spec-layer entry `{forbidden}`."
        );
    }

    let readme_source = load_source("src/README.md");
    let check2_source = load_source("check2.md");
    assert!(
        !readme_source.contains("spec.rs"),
        "Carousel README should stay focused on usage and should not require a spec layer."
    );
    assert!(
        check2_source.contains("`spec.rs` 只用于少数复杂组件"),
        "Carousel check2 should document the spec.rs guardrail for this component."
    );
}

#[test]
fn carousel_token_first_static_styles_flow_through_css_aggregation_and_ui_root_injection() {
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let css_aggregator_source = load_source("../../crates/ui-components/src/css.rs");
    let ui_root_source = load_source("../../crates/ui-components/src/root.rs");

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-carousel {",
        "var(--ui-",
        "var(--ui-border)",
        "var(--ui-radius-lg)",
        "var(--ui-shadow-sm)",
    ] {
        assert!(
            styles_source.contains(required),
            "Carousel styles should keep token-first static css contract `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-carousel\")]",
        "out.push_str(crate::carousel::styles::CSS);",
    ] {
        assert!(
            css_aggregator_source.contains(required),
            "ui-components css aggregator should include carousel styles via `{required}`."
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            ui_root_source.contains(required),
            "UiRoot should inject aggregated component css through `{required}`."
        );
    }

    for forbidden in [
        "style=",
        "style:",
        "class=\"flex",
        "class=\"grid",
        "class=\"px-",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Carousel view should avoid runtime business styling and utility-first class leakage `{forbidden}`.",
        );
    }

    for forbidden in ["style!(", "css!(", "styled::", "stylist::"] {
        assert!(
            !styles_source.contains(forbidden),
            "Carousel component should not use CSS-in-Rust default path `{forbidden}`."
        );
    }
}

#[test]
fn carousel_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("src/styles.rs");
    let theme_css_source = load_source("../../crates/ui-theme/src/css.rs");

    for required in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-lg, var(--ui-fallback-space-lg))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-accent-fg, var(--ui-fallback-accent-fg))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
    ] {
        assert!(
            styles_source.contains(required),
            "carousel styles should keep defensive fallback chain marker `{required}`.",
        );
    }

    for required in [
        "--ui-fallback-space-2xs:",
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-space-lg:",
        "--ui-fallback-component-height-100:",
        "--ui-fallback-border-width:",
        "--ui-fallback-border:",
        "--ui-fallback-radius-sm:",
        "--ui-fallback-radius-lg:",
        "--ui-fallback-bg:",
        "--ui-fallback-bg-muted:",
        "--ui-fallback-fg:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-focus-ring:",
        "--ui-fallback-accent-fg:",
        "--ui-fallback-shadow-sm:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme css should provide fallback terminal `{required}`.",
        );
    }

    for forbidden in [
        "42rem",
        "10rem",
        "5.5rem",
        "2rem",
        "1.75rem",
        "0.5rem",
        "1px solid",
        "#",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "carousel styles should avoid raw terminal token `{forbidden}`.",
        );
    }
}

#[test]
fn carousel_defensive_variables_check_script_covers_style_fallback_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    let required = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(required),
        "contract-hygiene check script should enforce `{required}`.",
    );
}

#[test]
fn carousel_check2_marks_defensive_variables_contract_complete() {
    let check2_source = load_source("check2.md");

    assert!(
        check2_source.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "carousel check2 should mark defensive-variables gate complete.",
    );

    for required in [
        "carousel_styles_use_defensive_variable_fallback_chain",
        "carousel_defensive_variables_check_script_covers_style_fallback_contract",
        "scripts/check-ui-components-contract-hygiene.sh",
        "components/carousel/src/styles.rs",
        "crates/ui-theme/src/css.rs",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel check2 defensive-variables section should reference `{required}`.",
        );
    }
}

#[test]
fn carousel_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_entry_source = load_source("../../crates/ui-components/src/css.rs");
    let root_source = load_source("../../crates/ui-components/src/root.rs");
    let view_source = load_source("src/view.rs");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-carousel\")]",
        "out.push_str(crate::carousel::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(required),
            "ui-components css entry should keep cascade-layer marker `{required}`.",
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should keep centralized css injection contract `{required}`.",
        );
    }

    for forbidden in [
        " style=",
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
        "style:top=",
        "style:left=",
        "style:right=",
        "style:bottom=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "carousel view should not include plain inline style token `{forbidden}`.",
        );
    }

    for (line_index, line) in view_source.lines().enumerate() {
        if let Some(pos) = line.find("style:") {
            let key = line[pos + "style:".len()..]
                .split(|c: char| c == '=' || c.is_whitespace() || c == '>')
                .next()
                .unwrap_or_default()
                .trim();
            assert!(
                key.starts_with("--"),
                "carousel runtime style should only set css custom properties; found `style:{key}` at line {}.",
                line_index + 1
            );
        }
    }
}

#[test]
fn carousel_cascade_layer_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    let required = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(required),
        "contract-hygiene check script should enforce `{required}`.",
    );
}

#[test]
fn carousel_check2_marks_cascade_layer_contract_complete() {
    let check2_source = load_source("check2.md");

    assert!(
        check2_source.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "carousel check2 should mark cascade-layer gate complete.",
    );

    for required in [
        "carousel_cascade_layer_and_runtime_style_contract_is_enforced",
        "carousel_cascade_layer_check_script_covers_contract",
        "scripts/check-ui-components-contract-hygiene.sh",
        "crates/ui-components/src/css.rs",
        "crates/ui-components/src/root.rs",
        "components/carousel/src/view.rs",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel check2 cascade-layer section should reference `{required}`.",
        );
    }
}

#[test]
fn carousel_tree_shaking_contract_keeps_feature_gated_entrypoints() {
    let manifest_source = load_source("../../crates/ui-components/Cargo.toml");
    let lib_source = load_source("../../crates/ui-components/src/lib.rs");
    let css_source = load_source("../../crates/ui-components/src/css.rs");

    for required in [
        "component-carousel = [\"dep:ui-carousel\"]",
        "ui-carousel = { path = \"../../components/carousel\", optional = true }",
        "#[cfg(feature = \"component-carousel\")]\npub use ui_carousel as carousel;",
        "#[cfg(feature = \"component-carousel\")]\n    out.push_str(crate::carousel::styles::CSS);",
    ] {
        assert!(
            manifest_source.contains(required)
                || lib_source.contains(required)
                || css_source.contains(required),
            "Tree-shaking contract should keep feature-gated carousel entry `{required}`."
        );
    }

    for forbidden in [
        "component-carousel = [\"all-components\"]",
        "pub use ui_carousel as carousel;\n#[cfg(feature = \"component-carousel\")]",
        "out.push_str(crate::carousel::styles::CSS);\n#[cfg(feature = \"component-carousel\")]",
    ] {
        assert!(
            !manifest_source.contains(forbidden)
                && !lib_source.contains(forbidden)
                && !css_source.contains(forbidden),
            "Carousel tree-shaking path should avoid ungated/full-bundle coupling `{forbidden}`.",
        );
    }
}

#[test]
fn carousel_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let tree_shaking_script = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let tree_shaking_budget = load_source("../../scripts/tree_shaking_budget.env");

    for required in [
        "CAROUSEL_MIN_FEATURES=\"component-carousel,inject-css\"",
        "carousel_tree_shaking_contract_keeps_feature_gated_entrypoints",
        "carousel_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "carousel_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "CAROUSEL_TREE_OUTPUT",
        "if ! grep -q 'feature \"component-carousel\" (command-line)' <<<\"$CAROUSEL_TREE_OUTPUT\"; then",
        "if ! grep -q 'feature \"inject-css\" (command-line)' <<<\"$CAROUSEL_TREE_OUTPUT\"; then",
        "if grep -q 'all-components' <<<\"$CAROUSEL_TREE_OUTPUT\"; then",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$CAROUSEL_MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
    ] {
        assert!(
            tree_shaking_script.contains(required),
            "tree-shaking script should enforce `{required}`",
        );
    }

    for required in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            tree_shaking_budget.contains(required),
            "tree-shaking budget env should define `{required}`",
        );
    }
}

#[test]
fn carousel_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_source("check2.md");

    for required in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "component-carousel = [\"dep:ui-carousel\"]",
        "#[cfg(feature = \"component-carousel\")]",
        "pub use ui_carousel as carousel;",
        "out.push_str(crate::carousel::styles::CSS);",
        "carousel_tree_shaking_contract_keeps_feature_gated_entrypoints",
        "carousel_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "carousel_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_tree_shaking_contract_keeps_feature_gated_entrypoints",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-carousel,inject-css",
        "cargo tree -e features -i ui-components -p web-demo",
        "scripts/check-ui-components-tree-shaking.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel check2 tree-shaking section should reference `{required}`",
        );
    }
}

#[test]
fn carousel_platform_build_contract_uses_explicit_web_ssr_features() {
    let manifest_source = load_source("Cargo.toml");
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for required in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\", \"ui-headless/web\"]",
        "ssr = [\"leptos/ssr\", \"ui-headless/ssr\"]",
        "leptos = { version = \"0.8.15\", default-features = false }",
        "ui-headless = { path = \"../../crates/ui-headless\", default-features = false }",
    ] {
        assert!(
            manifest_source.contains(required),
            "Carousel Cargo platform feature contract should include `{required}`.",
        );
    }

    for forbidden in [
        "web-sys",
        "web_sys",
        "js-sys",
        "js_sys",
        "wasm-bindgen",
        "wasm_bindgen",
        "target_arch = \"wasm32\"",
        "target_arch=\"wasm32\"",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Carousel source should keep platform-agnostic component path without `{forbidden}`.",
        );
    }
}

#[test]
fn carousel_headless_feature_mutex_guard_is_preserved() {
    let carousel_manifest_source = load_source("Cargo.toml");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
        "web = [\"leptos/csr\", \"ui-headless/web\"]",
        "ssr = [\"leptos/ssr\", \"ui-headless/ssr\"]",
        "ui-headless = { path = \"../../crates/ui-headless\", default-features = false }",
    ] {
        assert!(
            headless_lib_source.contains(required) || carousel_manifest_source.contains(required),
            "Headless web/ssr mutex guard contract should include `{required}`.",
        );
    }

    for forbidden in [
        "web = [\"leptos/csr\", \"ui-headless/web\", \"ui-headless/ssr\"]",
        "ssr = [\"leptos/ssr\", \"ui-headless/web\", \"ui-headless/ssr\"]",
        "ui-headless = { path = \"../../crates/ui-headless\" }",
        "default = [\"web\", \"ssr\"]",
    ] {
        assert!(
            !carousel_manifest_source.contains(forbidden),
            "Carousel feature mapping should avoid breaking headless mutex guard via `{forbidden}`.",
        );
    }
}

#[test]
fn carousel_motion_non_wasm_noop_contract_is_preserved() {
    let ui_motion_source = load_source("../../crates/ui-motion/src/lib.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let carousel_motion_source = load_source("src/motion.rs");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_source.contains(required),
            "ui-motion should preserve non-wasm no-op backend contract `{required}`.",
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "active_highlight should provide non-wasm attach stub `{required}`.",
        );
    }

    for required in [
        "pub fn attach_carousel_indicator_motion(",
        "let motion = sanitize_motion(motion);",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            carousel_motion_source.contains(required),
            "Carousel motion adapter should keep deterministic delegation `{required}`.",
        );
    }

    for forbidden in ["panic!(", ".unwrap(", ".expect("] {
        assert!(
            !carousel_motion_source.contains(forbidden),
            "Carousel motion adapter should avoid panic-prone path `{forbidden}`.",
        );
    }
}

#[test]
fn carousel_reduced_motion_ssr_wasm_branch_contract_is_preserved() {
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let spring_source = load_source("../../crates/ui-motion/src/spring.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for required in [
        "if crate::web::prefers_reduced_motion() {",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
        "on_rest();",
    ] {
        assert!(
            spring_source.contains(required),
            "Spring animator should preserve reduced-motion fast path `{required}`.",
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(required),
            "ui-motion platform branch contract should include `{required}`.",
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "Active highlight motion should preserve wasm/non-wasm split `{required}`.",
        );
    }

    for forbidden in [
        "cfg(target_arch = \"wasm32\")",
        "cfg(not(target_arch = \"wasm32\"))",
        "web_sys",
        "web-sys",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Carousel semantic contract path should stay platform-agnostic in view/logic `{forbidden}`.",
        );
    }
}

#[test]
fn carousel_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let motion_source = load_source("src/motion.rs");
    let view_source = load_source("src/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");
    let spring_source = load_source("../../crates/ui-motion/src/spring.rs");
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");

    for required in [
        "pub fn sanitize_motion(motion: super::CarouselMotion) -> super::CarouselMotion",
        "ui_motion::spring::sanitize_config(motion.spring, default.spring)",
        "pub fn attach_carousel_indicator_motion(",
        "let motion = sanitize_motion(motion);",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            motion_source.contains(required),
            "carousel motion contract should keep `{required}` in motion.rs.",
        );
    }

    for required in [
        "crate::motion::attach_carousel_indicator_motion(",
        "let mut marker_motion = ui_components::CarouselMotion::default();",
        "marker_motion.spring.stiffness = 250.0",
        "marker_motion.spring.damping = 22.0",
        "motion=marker_motion",
    ] {
        assert!(
            view_source.contains(required) || docs_source.contains(required),
            "carousel motion contract should expose/consume configurable spring marker `{required}`.",
        );
    }

    for required in [
        "if crate::web::prefers_reduced_motion() {",
        "(self.inner.apply.borrow_mut())(target);",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            spring_source.contains(required)
                || ui_motion_lib_source.contains(required)
                || active_highlight_source.contains(required),
            "carousel motion path should preserve reduced-motion/non-wasm safety marker `{required}`.",
        );
    }

    for forbidden in [
        "web_sys",
        "wasm_bindgen",
        "requestAnimationFrame",
        "panic!(",
        ".unwrap(",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "carousel motion adapter should avoid runtime-specific/panic token `{forbidden}`.",
        );
    }
}

#[test]
fn carousel_motion_contract_platform_script_covers_guard() {
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    let required = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        platform_script_source.contains(required),
        "platform script should enforce `{required}`.",
    );
}

#[test]
fn carousel_check2_marks_motion_contract_complete() {
    let check2_source = load_source("check2.md");

    for required in [
        "carousel_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
        "carousel_motion_contract_platform_script_covers_guard",
        "scripts/check-ui-components-platforms.sh",
        "components/carousel/src/motion.rs",
        "components/carousel/src/view.rs",
        "crates/ui-motion/src/spring.rs",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel check2 motion section should reference `{required}`.",
        );
    }
}

#[test]
fn carousel_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let lib_source = load_source("../../crates/ui-components/src/lib.rs");
    let css_source = load_source("../../crates/ui-components/src/css.rs");
    let root_source = load_source("../../crates/ui-components/src/root.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-carousel\")]",
        "pub use ui_carousel as carousel;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep marker `{needle}`."
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
            "ui-components lib entry should not leak platform/internal marker `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-carousel\")]",
        "out.push_str(crate::carousel::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css registry should keep feature-gated marker `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n, provide_ui_id_provider};",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if let Some(overrides) = semantic_overrides.get_value() {",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
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

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "Carousel",
        "Button",
        "Accordion",
        "aria-",
        "data-slot",
        "on:click",
    ] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic and avoid component business marker `{forbidden}`."
        );
    }

    for forbidden in [
        "../../crates/ui-components/src/overlay_open.rs",
        "../../crates/ui-components/src/presence.rs",
        "../../crates/ui-components/src/a11y.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "ui-components forbidden entrypoint file should not exist: `{forbidden}`."
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
fn carousel_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-components-entrypoints.sh");
    let needle = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_ui_components_fixed_entry_files_follow_layered_boundaries";

    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn carousel_check2_marks_ui_components_fixed_entry_files_complete() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] `ui-components` 固定入口文件落点正确。"),
        "carousel check2 should mark fixed-entrypoint gate complete."
    );

    for required in [
        "`crates/ui-components/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui-components/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui-components/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui-components/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui-components/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui-components/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
    ] {
        assert!(
            source.contains(required),
            "carousel check2 fixed-entrypoint section should retain rule `{required}`."
        );
    }

    for needle in [
        "components/carousel/test/semantics.rs::carousel_ui_components_fixed_entry_files_follow_layered_boundaries",
        "components/carousel/test/semantics.rs::carousel_entrypoints_check_script_covers_fixed_entrypoint_contract",
        "components/carousel/test/semantics.rs::carousel_check2_marks_ui_components_fixed_entry_files_complete",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_ui_components_fixed_entry_files_follow_layered_boundaries",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_entrypoints_check_script_covers_fixed_entrypoint_contract",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_check2_marks_ui_components_fixed_entry_files_complete",
        "scripts/check-ui-components-entrypoints.sh",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_ui_components_fixed_entry_files_follow_layered_boundaries",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "carousel check2 fixed-entrypoint section should reference `{needle}`."
        );
    }
}

#[test]
fn carousel_component_directory_standard_files_follow_contract_and_na_paths() {
    let module_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let checklist_source = load_source("check2.md");
    let component_src_dir = crate_dir().join("src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            component_src_dir.join(required).exists(),
            "carousel component should keep required standard file `{required}`.",
        );
    }
    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !component_src_dir.join(forbidden).exists(),
            "carousel simple component should not include `{forbidden}`.",
        );
    }

    for required in [
        "mod i18n;",
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use ui_visual_primitive::active_highlight::ActiveHighlightMotion as CarouselMotion;",
        "pub use view::Carousel;",
    ] {
        assert!(
            module_source.contains(required),
            "mod.rs should keep minimal stable export marker `{required}`.",
        );
    }
    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub mod motion;",
        "mod spec;",
        "pub mod spec;",
        "pub use logic::*;",
        "pub use view::*;",
        "pub use motion::*;",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "mod.rs should not over-export internals or wire spec for simple component (`{forbidden}`).",
        );
    }

    for forbidden in [
        "view! {",
        "#[component]",
        "on:click",
        "on:keydown",
        "class=",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay in normalization/derivation scope (`{forbidden}`).",
        );
    }

    for required in ["pub const CSS: &str", ".ui-carousel", "var(--ui-"] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep static token-first css marker `{required}`.",
        );
    }
    for forbidden in [
        "#[component]",
        "on:click",
        "use_controllable_state",
        "use_carousel_root",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not mix rendering/logic token `{forbidden}`.",
        );
    }

    for required in [
        "#[component]",
        "pub fn Carousel(",
        "use_carousel_root",
        "labeled_toolbar_attrs",
        "labeled_group_attrs",
        "logic::resolve_state(",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep render + headless mounting marker `{required}`.",
        );
    }
    {
        let forbidden = "ui_state_primitives::carousel";
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not bypass logic boundary for primitive state mapping (`{forbidden}`).",
        );
    }

    for required in [
        "pub fn sanitize_motion(motion: super::CarouselMotion) -> super::CarouselMotion",
        "pub fn attach_carousel_indicator_motion(",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should keep motion contract + attach marker `{required}`.",
        );
    }
    for forbidden in ["#[component]", "view! {", "on:click", "role="] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not host view/semantic business token `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "文件存在证据：`components/carousel/src/` 保持 `mod.rs/logic.rs/styles.rs/view.rs/motion.rs` 五文件结构；`render.rs` 不存在。",
        "导出边界证据：`mod.rs` 维持最小稳定导出（`Carousel/CarouselMotion` 与必要常量、类型）；未出现 `pub mod logic/view/motion` 过度导出。",
        "职责证据：`logic.rs` 仅做 props 归一化、状态派生与来源标记；`styles.rs` 仅承载 token-first 静态 CSS；`view.rs` 仅做 Leptos 结构渲染 + headless 语义挂载；`motion.rs` 仅做 `CarouselMotion + attach_carousel_indicator_motion` 合同映射与挂载。",
        "spec N/A 证据：`components/carousel/src/spec.rs` 不存在，`mod.rs` 未声明 `mod spec;`；简单组件不引入 spec。",
        "回归覆盖：`components/carousel/test/semantics.rs::carousel_component_directory_standard_files_follow_contract_and_na_paths`、`crates/ui-components/tests/carousel_semantics.rs::carousel_component_directory_standard_files_follow_contract_and_na_paths`。",
        "门禁证据：`scripts/check-ui-components-component-files.sh` 新增 `carousel_component_directory_standard_files_follow_contract_and_na_paths` 命令，阻断目录落点回归。",
    ] {
        assert!(
            checklist_source.contains(required),
            "carousel checklist should document component-directory standard-file evidence `{required}`.",
        );
    }
}

#[test]
fn carousel_component_files_check_script_covers_standard_layout_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`.",
    );
}

#[test]
fn carousel_file_placement_discipline_is_strict_for_component_scope() {
    for required in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "carousel file placement should include required file `{required}`.",
        );
    }

    {
        let forbidden = "src/render.rs";
        assert!(
            !path_exists(forbidden),
            "carousel file placement should forbid `{forbidden}`.",
        );
    }

    assert!(
        !path_exists("src/spec.rs"),
        "carousel is a simple component; `src/spec.rs` should remain absent in this scope.",
    );

    assert!(
        path_exists("src/protocol.rs"),
        "carousel keeps `src/protocol.rs` as sidecar contract file and should not regress.",
    );
}

#[test]
fn carousel_component_files_script_covers_file_placement_discipline() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_file_placement_discipline_is_strict_for_component_scope";

    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`.",
    );
}

#[test]
fn carousel_check2_marks_file_placement_discipline_complete() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
        "carousel check2 should mark file-placement discipline gate complete.",
    );

    for needle in [
        "components/carousel/src/mod.rs",
        "components/carousel/src/logic.rs",
        "components/carousel/src/styles.rs",
        "components/carousel/src/view.rs",
        "components/carousel/src/motion.rs",
        "components/carousel/src/render.rs`（不存在）",
        "components/carousel/src/spec.rs`（不存在）",
        "components/carousel/src/protocol.rs`（sidecar 保留）",
        "components/carousel/test/semantics.rs::carousel_file_placement_discipline_is_strict_for_component_scope",
        "components/carousel/test/semantics.rs::carousel_component_files_script_covers_file_placement_discipline",
        "components/carousel/test/semantics.rs::carousel_check2_marks_file_placement_discipline_complete",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_file_placement_discipline_is_strict_for_component_scope",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_component_files_script_covers_file_placement_discipline",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_check2_marks_file_placement_discipline_complete",
        "scripts/check-ui-components-component-files.sh",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_file_placement_discipline_is_strict_for_component_scope",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "carousel check2 file-placement section should reference `{needle}`.",
        );
    }
}

#[test]
fn carousel_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    assert!(
        !path_exists("src/spec.rs"),
        "carousel is a simple component; `src/spec.rs` should remain absent for Hyper-Structure Builder gate.",
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "CarouselSpec",
        "spec::",
        "Spec::new(",
        ".render()",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "carousel simple scope should not expose Hyper-Structure Builder marker `{forbidden}`.",
        );
    }
}

#[test]
fn carousel_component_files_script_covers_hyper_structure_builder_na_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_hyper_structure_builder_spec_is_not_applicable_for_simple_component";

    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`.",
    );
}

#[test]
fn carousel_check2_marks_hyper_structure_builder_contract_complete() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
        "carousel check2 should mark hyper-structure-builder gate complete.",
    );

    for needle in [
        "N/A：`carousel` 为简单组件",
        "components/carousel/src/spec.rs",
        "CarouselSpec",
        "spec::",
        ".render()",
        "components/carousel/test/semantics.rs::carousel_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "components/carousel/test/semantics.rs::carousel_component_files_script_covers_hyper_structure_builder_na_contract",
        "components/carousel/test/semantics.rs::carousel_check2_marks_hyper_structure_builder_contract_complete",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_component_files_script_covers_hyper_structure_builder_na_contract",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_check2_marks_hyper_structure_builder_contract_complete",
        "scripts/check-ui-components-component-files.sh",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "carousel check2 hyper-structure-builder section should reference `{needle}`.",
        );
    }
}

#[test]
fn carousel_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    for required_file in ["src/Component.toml", "src/carousel.rbi"] {
        assert!(
            path_exists(required_file),
            "carousel context-compression artifact should exist: `{required_file}`.",
        );
    }

    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/carousel.rbi");
    let view_source = load_source("src/view.rs");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Carousel\"",
        "crate = \"ui-carousel\"",
        "rbi = \"carousel.rbi\"",
        "name = \"id_base\"",
        "name = \"items\"",
        "name = \"selected_index\"",
        "name = \"default_selected_index\"",
        "name = \"on_selected_index_change\"",
        "name = \"orientation\"",
        "name = \"is_loop_navigation\"",
        "name = \"motion\"",
        "name = \"aria_label\"",
        "name = \"controls_aria_label\"",
        "name = \"indicators_aria_label\"",
        "name = \"previous_label\"",
        "name = \"next_label\"",
        "name = \"indicator_aria_label_template\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"class_name\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "carousel Component.toml should include context-compression marker `{needle}`.",
        );
    }

    for needle in [
        "pub use crate::CarouselMotion;",
        "pub use crate::{",
        "pub use ui_headless::A11yDirection;",
        "pub const DEFAULT_ID_BASE: &str;",
        "pub const DEFAULT_ARIA_LABEL: &str;",
        "pub const DEFAULT_LOOP_NAVIGATION: bool;",
        "pub fn sanitize_motion(motion: crate::CarouselMotion) -> crate::CarouselMotion;",
        "pub fn attach_carousel_indicator_motion(",
        "pub fn Carousel(",
        "id_base: String,",
        "items: Vec<crate::CarouselItem>,",
        "selected_index: Option<leptos::prelude::Signal<Option<usize>>>",
        "default_selected_index: Option<usize>",
        "on_selected_index_change: Option<leptos::prelude::Callback<Option<usize>>>",
        "orientation: crate::CarouselOrientation,",
        "is_loop_navigation: bool,",
        "motion: crate::CarouselMotion,",
        "aria_label: Option<String>",
        "controls_aria_label: Option<String>",
        "indicators_aria_label: Option<String>",
        "previous_label: Option<String>",
        "next_label: Option<String>",
        "indicator_aria_label_template: Option<String>",
        "lang: Option<String>",
        "dir: Option<ui_headless::A11yDirection>",
        "class_name: Option<String>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "carousel RBI projection should keep signature marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn Carousel(",
        "id_base: String,",
        "items: Vec<CarouselItem>,",
        "#[prop(optional)] selected_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] default_selected_index: Option<usize>",
        "#[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>",
        "#[prop(optional)] orientation: CarouselOrientation,",
        "#[prop(default = true)] is_loop_navigation: bool,",
        "#[prop(optional)] motion: ActiveHighlightMotion,",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] controls_aria_label: Option<String>",
        "#[prop(optional, into)] indicators_aria_label: Option<String>",
        "#[prop(optional, into)] previous_label: Option<String>",
        "#[prop(optional, into)] next_label: Option<String>",
        "#[prop(optional, into)] indicator_aria_label_template: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional, into)] class_name: Option<String>",
    ] {
        assert!(
            view_source.contains(needle),
            "carousel view signature should include `{needle}` for manifest/rbi drift detection.",
        );
    }
}

#[test]
fn carousel_component_files_check_script_covers_context_compression_manifest_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_context_compression_manifest_and_rbi_projection_are_present_and_current";

    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`.",
    );
}

#[test]
fn carousel_check2_marks_context_compression_manifest_and_rbi_contract_complete() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "carousel check2 should mark context-compression manifest/rbi gate complete.",
    );

    for needle in [
        "components/carousel/src/Component.toml",
        "components/carousel/src/carousel.rbi",
        "components/carousel/test/semantics.rs::carousel_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "components/carousel/test/semantics.rs::carousel_component_files_check_script_covers_context_compression_manifest_contract",
        "components/carousel/test/semantics.rs::carousel_check2_marks_context_compression_manifest_and_rbi_contract_complete",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_component_files_check_script_covers_context_compression_manifest_contract",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_check2_marks_context_compression_manifest_and_rbi_contract_complete",
        "scripts/check-ui-components-component-files.sh",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "carousel check2 context-compression section should reference `{needle}`.",
        );
    }
}

#[test]
fn carousel_performance_governance_contract_is_mount_only_traceable_and_blocking() {
    let check2_source = load_source("check2.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_carousel_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_source("src/view.rs");

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "N/A：`Carousel` 暂未接入精确 `render_count` 自动化计数",
        "render_count",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "Carousel checklist should include performance governance evidence token `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"Carousel\"",
        "\"carousel\"",
        "\"Collections\"",
        "collections_command::carousel",
    ] {
        assert!(
            pages_source.contains(needle),
            "Carousel docs catalog should keep marker `{needle}` for perf coverage traversal.",
        );
    }

    for needle in ["title=\"Carousel\"", "slug=\"carousel\"", "<ComponentPage"] {
        assert!(
            docs_carousel_page_source.contains(needle),
            "Carousel docs page should mount through ComponentPage contract `{needle}`.",
        );
    }

    for needle in [
        "_ => UiPerfBudget::mount_only(120.0),",
        "let perf_budget = component_page_perf_budget(slug);",
        "let perf_name = format!(\"ComponentPage::{slug}\");",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep mount-only fallback/perf probe wiring via `{needle}`.",
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
        "\"mount-plus-budget\"",
        "\"mount-only\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose stable perf observability marker `{needle}`.",
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
            "docs coverage e2e should keep blocking perf assertion `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "perf governance should keep explicit render_count follow-up marker `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_performance_governance_contract_is_mount_only_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || root_state.get().state_attr",
        "data-selected-index=move || root_state.get().selected_index",
        "data-class-source=move || root_state.get().class_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Carousel should expose state attribution marker `{needle}` for perf triage.",
        );
    }
}

#[test]
fn carousel_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let local_semantics = load_source("test/semantics.rs");
    let aggregated_semantics =
        load_source("../../crates/ui-components/tests/carousel_semantics.rs");
    let view_source = load_source("src/view.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let check2_source = load_source("check2.md");

    for required_test in [
        "fn carousel_semantic_contract_matrix_covers_state_interaction_and_non_snapshot_policy()",
        "fn carousel_performance_governance_contract_is_mount_only_traceable_and_blocking()",
        "fn carousel_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            local_semantics.contains(required_test) && aggregated_semantics.contains(required_test),
            "semantic/performance regression suite should include `{required_test}` in local and aggregated tests.",
        );
    }

    for marker in [
        "role=root_a11y.attrs.role",
        "aria-label=root_a11y.attrs.aria_label.clone()",
        "aria-disabled=controls_a11y.aria_disabled",
        "data-state=move || root_state.get().state_attr",
        "data-selection=move || root_state.get().selected_attr",
        "data-focus=move || root_state.get().focus_attr",
        "data-selected-index-source=move || root_state.get().selected_index_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "on:keydown=on_key_down",
        "on:focus=on_focus",
    ] {
        assert!(
            view_source.contains(marker),
            "carousel view should expose aria/data/focus semantic marker `{marker}`.",
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "render_count follow-up governance should include `{marker}`.",
        );
    }

    for marker in [
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_performance_governance_contract_is_mount_only_traceable_and_blocking",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            script_source.contains(marker),
            "performance gate script should include `{marker}`.",
        );
    }

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "carousel_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "carousel_performance_governance_contract_is_mount_only_traceable_and_blocking",
    ] {
        assert!(
            check2_source.contains(marker),
            "checklist should keep semantics/performance governance marker `{marker}`.",
        );
    }
}

#[test]
fn carousel_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("src/view.rs");
    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    for required in [
        "let viewport_view = view! {",
        "let controls_view = view! {",
        "let indicators_view = view! {",
        "{viewport_view}",
        "{controls_view}",
        "{indicators_view}",
    ] {
        assert!(
            view_source.contains(required),
            "carousel view macro split should include `{required}`.",
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "carousel should keep a single public component boundary.",
    );

    assert!(
        view_source.matches("view! {").count() <= 6,
        "carousel view should keep macro count bounded after semantic subrender split.",
    );

    assert!(
        view_source.lines().count() <= 560,
        "carousel view.rs should stay bounded; split further if this grows significantly.",
    );

    let script_needle = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "carousel_view_macro_complexity_is_split_into_semantic_subrenders",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel check2 should keep view-macro governance marker `{required}`.",
        );
    }
}

#[test]
fn carousel_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("src/view.rs");
    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    for required in [
        "struct CarouselSlideRenderInput {",
        "fn render_carousel_slide(input: CarouselSlideRenderInput) -> impl IntoView {",
        "struct CarouselIndicatorRenderInput {",
        "fn render_carousel_indicator(input: CarouselIndicatorRenderInput) -> impl IntoView {",
        "let render_slide = move |index: usize| {",
        "render_carousel_slide(CarouselSlideRenderInput {",
        "let render_indicator = move |index: usize| {",
        "render_carousel_indicator(CarouselIndicatorRenderInput {",
    ] {
        assert!(
            view_source.contains(required),
            "carousel functional split should include `{required}`.",
        );
    }

    for forbidden in [
        "#[component]\nfn render_carousel_slide(",
        "#[component]\nfn render_carousel_indicator(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "carousel local fragments should stay plain functions, not extra components `{forbidden}`.",
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "carousel should keep one public component boundary.",
    );

    for required in [
        "data-slot=item_slot.as_attr()",
        "data-state=move || item_state.get().status.as_attr()",
        "data-selected=move || item_state.get().selected_attr",
        "data-focused=move || item_state.get().focused_attr",
        "data-disabled=move || item_state.get().disabled_attr",
        "data-slot=indicator_slot.as_attr()",
        "data-slot=indicator_dot_slot.as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "carousel semantic markers should stay stable after function split `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "carousel_view_functional_split_prefers_plain_functions_over_local_components",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel check2 should keep function-split governance marker `{required}`.",
        );
    }
}

#[test]
fn carousel_static_fragments_are_constantized_or_absent_for_simple_layout() {
    let view_source = load_source("src/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2_source = load_source("check2.md");

    for required in [
        "fn render_carousel_indicator_dot() -> impl IntoView {",
        "let indicator_dot_slot = CarouselSlot::IndicatorDot;",
        "{render_carousel_indicator_dot()}",
        "data-slot=indicator_dot_slot.as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "carousel static fragment contract should keep constantized token `{required}`.",
        );
    }

    for forbidden in [
        "inner_html=",
        "dangerously_set_inner_html",
        "<svg",
        "<path",
        "<footer",
        "<article class=\"docs",
        "let long_text",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "carousel simple layout should avoid heavy static fragment token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_static_fragments_are_constantized_or_absent_for_simple_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "可判定为纯静态的片段应避免重复动态构造。",
        "常量化后仍需维持可访问语义（title/aria-label/role 等）。",
        "静态资源变更路径要清晰，避免散落在多个 `view!` 片段中。",
        "carousel_static_fragments_are_constantized_or_absent_for_simple_layout",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel check2 should keep static-fragment governance marker `{required}`.",
        );
    }
}

#[test]
fn carousel_inner_html_usage_is_explicitly_na_and_guarded() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");
    let view_source = load_source("src/view.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");
    let docs_shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-components-inner-html.sh");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "markdown_to_html(",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !docs_page_source.contains(forbidden),
            "carousel component/docs page should stay free from html injection token `{forbidden}`.",
        );
    }

    assert!(
        docs_shell_source.contains("<div data-slot=\"component-readme\" inner_html=html></div>"),
        "shared docs shell should keep the single trusted inner_html mount for readme rendering.",
    );
    assert!(
        !docs_shell_source.contains("\"carousel\" => Some("),
        "carousel should stay out of docs-shell inner_html whitelist mapping.",
    );

    for required in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "仅允许编译期常量或明确白名单内容进入 `inner_html`。",
        "严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。",
        "使用 `inner_html` 的节点必须补语义测试与安全回归说明。",
        "carousel_inner_html_usage_is_explicitly_na_and_guarded",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel checklist should keep inner_html safety governance marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_inner_html_usage_is_explicitly_na_and_guarded";
    assert!(
        script_source.contains(script_needle),
        "inner-html gate script should include `{script_needle}`.",
    );
}

#[test]
fn carousel_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = load_source("../../crates/ui-components/Cargo.toml");
    let crate_root_source = load_source("../../crates/ui-components/src/lib.rs");
    let button_view_source = load_source("../../components/button/src/view.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let check2_source = load_source("check2.md");

    assert!(
        cargo_source.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "wasm debug capability should stay feature-gated via `button-wasm-debug`."
    );

    let all_components_start = cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = cargo_source[all_components_start..]
        .find("\n\ncomponent-")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before component feature declarations");
    let all_components_block = &cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "wasm debug feature must not be pulled into all-components production path."
    );

    for required in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
        "#[cfg(target_arch = \"wasm32\")]\nmod observability;",
    ] {
        assert!(
            crate_root_source.contains(required),
            "ui-components root should keep wasm debug isolation marker `{required}`.",
        );
    }

    for required in [
        "data-debug-source=source.clone()",
        "data-debug-before=before_attr",
        "data-debug-after=after_attr",
        "data-debug-timestamp-ms=format!(\"{:.0}\", event.timestamp_ms)",
        "data-slot=\"button-debug-replay\"",
        "request_replay.run(event.source)",
    ] {
        assert!(
            button_view_source.contains(required),
            "shared button wasm debug path should keep trace/replay marker `{required}`.",
        );
    }

    for required in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(required),
            "docs debug visual entry should keep `{required}`.",
        );
    }

    for required in [
        "events.push(event);",
        ".into_iter()",
        ".take(40)",
        "let ts_ms = event.ts_ms;",
        "UiTraceEventKind::Note",
        "UiTraceEventKind::Inspect",
        "trace.emit(",
    ] {
        assert!(
            trace_source.contains(required) || debug_overlay_source.contains(required),
            "global trace timeline/replay evidence should keep marker `{required}`.",
        );
    }

    for required in [
        "data-selected-index=move || root_state.get().selected_index",
        "data-focused-index=move || root_state.get().focused_index",
        "data-selected-index-source=move || root_state.get().selected_index_source_attr",
        "data-default-selected-index-source=move || root_state.get().default_selected_index_source_attr",
        "data-selected-index-change-source=move || root_state.get().selected_index_change_source_attr",
        "data-class-source=move || root_state.get().class_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "on:keydown=on_key_down",
        "on:click=on_prev",
        "on:click=on_next",
        "on:click=on_click",
        "on:focus=on_focus",
        "request_selected_index_change.run(Some(index));",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "carousel should keep machine-readable state/source/interaction marker `{required}` for debug attribution.",
        );
    }

    for forbidden in [
        "carousel-wasm-debug",
        "wasm_debug",
        "render_debug_panel(",
        "data-debug-source",
        "request_replay.run(",
        "trace.emit(",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "carousel should not duplicate shared wasm debug runtime token `{forbidden}`.",
        );
    }

    for required in [
        "WASM 调试要求：关键状态可追踪",
        "开发模式下至少能追踪关键状态变更来源与前后值",
        "关键交互链路应支持最小可复现记录",
        "调试开关默认不进入生产包体与公共 API",
        "carousel_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel checklist should keep wasm-debug governance contract marker `{required}`.",
        );
    }
}

#[test]
fn carousel_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");

    let needle = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm debug check script should enforce `{needle}`.",
    );
}

#[test]
fn carousel_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let source = load_source("../../apps/docs-app/src/playground.rs");

    for required in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Show settings\"",
        "\"Show code\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            source.contains(required),
            "playground should keep CSS hot-reload contract marker `{required}`.",
        );
    }
}

#[test]
fn carousel_dx_workbench_supports_optional_state_persistence_and_isolated_canvas() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");
    let check2_source = load_source("check2.md");

    for required in [
        "pub(super) fn carousel() -> AnyView",
        "title=\"Interactive Playground\"",
        "description=\"Workbench canvas: scoped CSS live-edit + optional selected-index context persistence across scenario switches.\"",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/carousel/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "<Switch",
        "checked=workbench_preserve_context",
        "set_checked=set_workbench_preserve_context",
        "if !workbench_preserve_context.get() {",
        "reset_workbench_selected.set(Some(0));",
        "data-slot=\"carousel-workbench-controls\"",
        "data-slot=\"carousel-workbench\"",
        "data-slot=\"carousel-workbench-canvas\"",
        "data-slot=\"carousel-workbench-last-selected\"",
    ] {
        assert!(
            source.contains(required),
            "carousel workbench should keep DX marker `{required}`.",
        );
    }

    for required in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "carousel_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "carousel_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel check2 should keep DX governance marker `{required}`.",
        );
    }
}

#[test]
fn carousel_dx_check_script_covers_hot_reload_and_workbench_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for required in [
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should enforce `{required}`.",
        );
    }
}

#[test]
fn carousel_docs_product_copy_paste_ready_contract_is_documented_and_scripted_locally() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_view = load_source("../../components/code-block/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");
    let check2_source = load_source("check2.md");

    for required in [
        "pub(super) fn carousel() -> AnyView",
        "title=\"Hello World (Minimal)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "const CAROUSEL_DOC_IMPORTS: &str =",
        "use ui_components::{Carousel, CarouselItem, CarouselOrientation};",
        "let carousel_imports = CAROUSEL_DOC_IMPORTS.to_string();",
        "code_imports=carousel_imports.clone()",
        "data-slot=\"carousel-streaming-policy\"",
        "Streaming Optional; fallback=snapshot.",
        "data-slot=\"carousel-source-first\"",
        "data-slot=\"carousel-copy-ready-hint\"",
        "data-slot=\"carousel-source-paths\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
    ] {
        assert!(
            docs_source.contains(required),
            "carousel docs-product surface should include `{required}`.",
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "data-slot=\"playground-toggle-code\"",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground copy-ready pipeline should keep `{required}`.",
        );
    }

    for required in [
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view.contains(required),
            "CodeBlock one-click copy affordance should keep `{required}`.",
        );
    }

    for required in [
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_docs_product_copy_paste_ready_rules",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_dx_check_script_covers_docs_product_copy_paste_ready_contract",
    ] {
        assert!(
            script_source.contains(required),
            "DX gate script should include docs-product command `{required}`.",
        );
    }

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "carousel_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "carousel_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync",
        "carousel_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "carousel_docs_product_copy_paste_ready_contract_is_documented_and_scripted_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel check2 docs-product section should include `{required}`.",
        );
    }
}

#[test]
fn carousel_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_source("check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel check2 should keep docs-sync/state-matrix rule `{required}`."
        );
    }
}

#[test]
fn carousel_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let check2_source = load_source("check2.md");

    for needle in [
        "pub const DEFAULT_ID_BASE: &str = carousel_primitives::DEFAULT_ID_BASE;",
        "pub const DEFAULT_ORIENTATION: CarouselOrientation = CarouselOrientation::Horizontal;",
        "pub const DEFAULT_LOOP_NAVIGATION: bool = true;",
        "#[prop(optional)] selected_index: Option<Signal<Option<usize>>>,",
        "#[prop(optional)] default_selected_index: Option<usize>,",
        "#[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>",
        "#[prop(optional)] orientation: CarouselOrientation,",
        "#[prop(default = true)] is_loop_navigation: bool,",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "carousel API/default contract should keep marker `{needle}` for docs sync."
        );
    }

    for needle in [
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"carousel-state-matrix\"",
        "data-slot=\"carousel-controlled-uncontrolled\"",
        "description=\"Side-by-side compare `selected_index + on_selected_index_change` versus `default_selected_index` paths.\"",
        "selected_index=controlled_selected.clone()",
        "on_selected_index_change=on_controlled_selected_change.clone()",
        "default_selected_index=Some(1)",
        "orientation=state_matrix_orientation.get()",
        "is_loop_navigation=state_matrix_is_loop.get()",
    ] {
        assert!(
            docs_source.contains(needle),
            "carousel docs should keep synced example/matrix/default marker `{needle}`."
        );
    }

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "apps/docs-app/src/pages/components/pages/collections_command.rs::carousel",
        "carousel_check2_documents_docs_sync_and_state_matrix_rules",
        "carousel_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "components/carousel/check2.md should keep docs-sync evidence marker `{needle}`."
        );
    }
}

#[test]
fn carousel_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: carousel docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include docs-sync/state-matrix marker `{needle}`."
        );
    }
}

#[test]
fn carousel_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "carousel check2 should mark docs-sync/state-matrix checklist item complete."
    );

    for needle in [
        "apps/docs-app/src/pages/components/pages/collections_command.rs::carousel",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "DEFAULT_ORIENTATION",
        "DEFAULT_LOOP_NAVIGATION",
        "carousel_check2_documents_docs_sync_and_state_matrix_rules",
        "carousel_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "carousel_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "carousel check2 docs-sync/state-matrix section should reference `{needle}`."
        );
    }
}

#[test]
fn carousel_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "carousel check2 documentation-as-product section should include `{needle}`.",
        );
    }
}

#[test]
fn carousel_documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = load_source("src/README.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "# Carousel",
        "## Hello World（最小可用）",
        "## 先用起来，再进阶",
        "## 常见用法",
        "默认路径：先用 `id_base + items`，不用先理解底层分层。",
        "进阶控制：再启用 `selected_index + on_selected_index_change + default_selected_index` 受控轴。",
        "### Controlled Example（高级入口）",
    ] {
        assert!(
            readme_source.contains(needle),
            "carousel README should include beginner-first marker `{needle}`.",
        );
    }

    let readme_hello = readme_source
        .find("## Hello World（最小可用）")
        .expect("carousel README should include hello-world section");
    let readme_beginner = readme_source
        .find("## 先用起来，再进阶")
        .expect("carousel README should include beginner-first progression section");
    let readme_common = readme_source
        .find("## 常见用法")
        .expect("carousel README should include common-usage section");
    let readme_advanced = readme_source
        .find("### Controlled Example（高级入口）")
        .expect("carousel README should include controlled advanced section");
    assert!(
        readme_hello < readme_beginner
            && readme_beginner < readme_common
            && readme_common < readme_advanced,
        "carousel README should keep beginner-first progression order (hello -> beginner -> common -> advanced).",
    );

    for needle in [
        "component_doc!(",
        "\"Carousel\"",
        "\"carousel\"",
        "collections_command::carousel",
        "pub(super) fn carousel() -> AnyView",
        "title=\"Carousel\"",
        "slug=\"carousel\"",
        "title=\"Hello World (Minimal)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
    ] {
        assert!(
            pages_source.contains(needle) || docs_source.contains(needle),
            "carousel docs entry should include `{needle}`.",
        );
    }
}

#[test]
fn carousel_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: carousel documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include documentation-as-product marker `{needle}`.",
        );
    }
}

#[test]
fn carousel_check2_marks_documentation_as_product_item_complete() {
    let check2_source = load_source("check2.md");

    assert!(
        check2_source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "carousel check2 should mark documentation-as-product item complete.",
    );

    for needle in [
        "components/carousel/src/README.md",
        "apps/docs-app/src/pages/components/pages.rs",
        "apps/docs-app/src/pages/components/pages/collections_command.rs",
        "## Hello World（最小可用）",
        "## 先用起来，再进阶",
        "## 常见用法",
        "carousel_check2_documents_documentation_as_product_rules",
        "carousel_documentation_entry_exists_with_beginner_first_progression",
        "carousel_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "carousel check2 documentation-as-product section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn carousel_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(needle),
            "carousel check2 interactive-playground section should include `{needle}`.",
        );
    }
}

#[test]
fn carousel_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"Interactive Playground\"",
        "description=\"Workbench canvas: scoped CSS live-edit + optional selected-index context persistence across scenario switches.\"",
        "code_signal=workbench_code",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/carousel/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "controls=move || view!",
        "selected_index=workbench_index",
        "set_selected_index=set_workbench_index",
        "checked=workbench_preserve_context",
        "set_checked=set_workbench_preserve_context",
        "data-slot=\"carousel-workbench-controls\"",
        "data-slot=\"carousel-workbench\"",
        "data-slot=\"carousel-workbench-actions\"",
        "data-slot=\"carousel-workbench-select-0\"",
        "data-slot=\"carousel-workbench-select-1\"",
        "data-slot=\"carousel-workbench-clear\"",
        "data-slot=\"carousel-workbench-canvas\"",
        "data-slot=\"carousel-workbench-last-selected\"",
        "CarouselWorkbenchConfig {",
    ] {
        assert!(
            docs_source.contains(needle),
            "carousel docs interactive playground should include `{needle}`.",
        );
    }
}

#[test]
fn carousel_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_carousel_contract.spec.mjs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "docs-app carousel key flow is repeatable with semantic breakpoints",
        "[data-slot=\"carousel-workbench\"]",
        "[data-slot=\"carousel-workbench-select-0\"]",
        "[data-slot=\"carousel-workbench-select-1\"]",
        "[data-slot=\"carousel-workbench-clear\"]",
        "for (const cycle of [1, 2]) {",
        "carousel key flow cycle ${cycle}",
        "data-selection-mode\", \"controlled\"",
        "data-selected-index-source\", \"external\"",
        "data-selected-index-change-source\", \"custom\"",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "carousel interactive e2e flow should include `{needle}`.",
        );
    }

    for needle in [
        "data-slot=\"carousel-workbench\"",
        "data-slot=\"carousel-workbench-select-0\"",
        "data-slot=\"carousel-workbench-select-1\"",
        "data-slot=\"carousel-workbench-clear\"",
        "data-slot=\"carousel-workbench-canvas\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "carousel docs should expose stable interactive anchor `{needle}` for repeatable e2e replay.",
        );
    }
}

#[test]
fn carousel_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: carousel interactive playground docs acceptance surface\"",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_interactive_playground_rules",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include interactive-playground marker `{needle}`.",
        );
    }
}

#[test]
fn carousel_check2_marks_interactive_playground_item_complete() {
    let check2_source = load_source("check2.md");

    assert!(
        check2_source.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "carousel check2 should mark interactive-playground item complete.",
    );

    for needle in [
        "title=\"Interactive Playground\"",
        "data-slot=\"carousel-workbench-controls\"",
        "data-slot=\"carousel-workbench-select-0\"",
        "data-slot=\"carousel-workbench-canvas\"",
        "CarouselWorkbenchConfig {",
        "N/A：`Carousel` 非 AI Spec 组件",
        "carousel_check2_documents_interactive_playground_rules",
        "carousel_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "carousel_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "carousel_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "carousel check2 interactive-playground section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn carousel_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(needle),
            "carousel check2 source-first section should include `{needle}`.",
        );
    }
}

#[test]
fn carousel_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "data-slot=\"carousel-source-first\"",
        "data-slot=\"carousel-source-paths\"",
        "<code>\"Show code\"</code>",
        "CAROUSEL_DOC_IMPORTS",
        "compose_copy_ready_code",
        "component-carousel",
        "inject-css",
        "components/carousel/src/mod.rs",
        "components/carousel/src/logic.rs",
        "components/carousel/src/view.rs",
        "components/carousel/src/styles.rs",
        "components/carousel/src/motion.rs",
    ] {
        assert!(
            docs_source.contains(needle),
            "carousel source-first docs should include `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "code_imports",
        "data-slot=\"playground-toggle-code\"",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground copy pipeline should include `{needle}`.",
        );
    }
}

#[test]
fn carousel_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: carousel source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include source-first marker `{needle}`.",
        );
    }
}

#[test]
fn carousel_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_source("check2.md");

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "carousel check2 should mark source-first copy-paste-ready item complete."
    );

    for needle in [
        "apps/docs-app/src/pages/components/pages/collections_command.rs::carousel",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "carousel_check2_documents_source_first_copy_paste_ready_rules",
        "carousel_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "carousel_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "carousel check2 source-first section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn carousel_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "carousel check2 heroui-benchmark docs-sync section should include `{needle}`.",
        );
    }
}

#[test]
fn carousel_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");
    let readme_source = load_source("src/README.md");

    for needle in [
        "### Carousel 同步记录（2026-02-20）",
        "参数模型同步：`Carousel` 参数主轴保持 `selected_index/default_selected_index/on_selected_index_change`",
        "component_doc!(\"Carousel\", \"carousel\", \"Collections\", collections_command::carousel)",
        "`apps/docs-app/src/pages/components/pages/collections_command.rs::carousel()`",
        "`components/carousel/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "heroui strategy doc should include carousel synchronization marker `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"Carousel\"",
        "\"carousel\"",
        "collections_command::carousel",
    ] {
        assert!(
            pages_source.contains(needle),
            "component docs index should expose carousel entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn carousel() -> AnyView {",
        "title=\"Carousel\"",
        "slug=\"carousel\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app carousel page should stay indexable via marker `{needle}`.",
        );
    }

    for needle in ["# Carousel", "## Hello World（最小可用）"] {
        assert!(
            readme_source.contains(needle),
            "carousel README should remain an equivalent component doc entry via `{needle}`.",
        );
    }
}

#[test]
fn carousel_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: carousel heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce heroui-benchmark docs-sync contract `{needle}`.",
        );
    }
}

#[test]
fn carousel_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "carousel_check2_documents_heroui_benchmark_docs_sync_rules",
        "carousel_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "carousel_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "carousel check2 should keep heroui-benchmark docs-sync evidence marker `{needle}`.",
        );
    }
}

#[test]
fn carousel_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let component_cargo = load_source("Cargo.toml");
    let ui_components_cargo = load_source("../../crates/ui-components/Cargo.toml");
    let mod_source = load_source("src/mod.rs");
    let i18n_source = load_source("src/i18n.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");
    let check2_source = load_source("check2.md");
    let spec_path = crate_dir().join("src/spec.rs");
    let protocol_path = crate_dir().join("src/protocol.rs");

    assert!(
        !spec_path.exists() && !protocol_path.exists(),
        "carousel simple component scope should keep spec/protocol serde path as explicit N/A.",
    );
    assert!(
        ui_components_cargo.contains("component-carousel = [\"dep:ui-carousel\"]"),
        "component-carousel feature should stay minimal and avoid schema/runtime fan-out.",
    );
    assert!(
        !component_cargo.contains("serde =") && !component_cargo.contains("serde_json"),
        "carousel should not pull serde dependencies without a spec/protocol contract.",
    );

    let combined = format!(
        "{mod_source}\n{i18n_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}"
    );
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "schema_version",
        "SchemaError",
        "from_json(",
        "to_json_result(",
        "spec::",
        "protocol::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "carousel engineering serde/spec N/A path should avoid `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
        "carousel_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel check2 should keep engineering governance marker `{required}`.",
        );
    }
}

#[test]
fn carousel_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let ui_components_cargo = load_source("../../crates/ui-components/Cargo.toml");
    let button_view_source = load_source("../../components/button/src/view.rs");
    let combined = [
        load_source("src/mod.rs"),
        load_source("src/i18n.rs"),
        load_source("src/logic.rs"),
        load_source("src/view.rs"),
        load_source("src/styles.rs"),
        load_source("src/motion.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_components::button::state_change\"",
    ] {
        assert!(
            ui_components_cargo.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`.",
        );
    }

    assert!(
        !ui_components_cargo.contains("carousel-wasm-debug"),
        "carousel should not define component-local tracing/debug feature without replay contract.",
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::carousel::",
        "const CAROUSEL_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "carousel should avoid ad-hoc tracing semantic drift token `{forbidden}`.",
        );
    }
}

#[test]
fn carousel_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let component_cargo = load_source("Cargo.toml");
    let mod_source = load_source("src/mod.rs");
    let i18n_source = load_source("src/i18n.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");

    for source in [
        &component_cargo,
        &mod_source,
        &i18n_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
    ] {
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
                "carousel should not leak runtime marker `{forbidden}`.",
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "carousel public module boundary should not leak web_sys types.",
    );
}

#[test]
fn carousel_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    for required in [
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            script_source.contains(required),
            "engineering check script should enforce `{required}`.",
        );
    }
}

#[test]
fn carousel_view_composes_logic_headless_and_motion_contracts() {
    let source = load_source("src/view.rs");

    assert!(
        source.contains("#[prop(default = true)] is_loop_navigation: bool,"),
        "Carousel bool prop should use `is_*` naming."
    );
    assert!(
        !source.contains("#[prop(default = true)] loop_navigation: bool,"),
        "Legacy bool prop name should be removed to prevent naming alias drift."
    );

    for needle in [
        "logic::resolve_state(CarouselPartStateInput {",
        "logic::resolve_item_state_attrs(",
        "logic::can_item_receive_selection(item_disabled)",
        "use_controllable_state(",
        "use_carousel_root(CarouselRootOptions {",
        "crate::motion::attach_carousel_indicator_motion(",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
    ] {
        assert!(
            source.contains(needle),
            "Carousel view should compose shell contracts via `{needle}`."
        );
    }

    for forbidden in [
        "if item_disabled {",
        "} else if selected_index.get() == Some(index) {",
        "} else if focused_index.get() == Some(index) {",
    ] {
        assert!(
            !source.contains(forbidden),
            "Carousel view should not inline state machine branching `{forbidden}`."
        );
    }
}

#[test]
fn carousel_logic_delegates_state_machine_to_primitives() {
    let source = load_source("src/logic.rs");

    for needle in [
        "use ui_state_primitives::carousel as carousel_primitives;",
        "carousel_primitives::resolve_state_core(",
        "carousel_primitives::step_selected_index(",
        "carousel_primitives::can_step_selection(",
    ] {
        assert!(
            source.contains(needle),
            "Carousel logic should delegate state primitives via `{needle}`."
        );
    }
}

#[test]
fn carousel_public_api_does_not_expose_dom_detail_types() {
    let source = load_source("src/mod.rs");

    for forbidden in ["web_sys", "web-sys", "NodeRef<html", "HtmlElement"] {
        assert!(
            !source.contains(forbidden),
            "Carousel public module should not expose DOM/platform detail `{forbidden}`."
        );
    }

    for needle in [
        "pub use view::Carousel;",
        "pub struct CarouselItem",
        "pub enum CarouselOrientation",
        "pub enum CarouselItemStatus",
    ] {
        assert!(
            source.contains(needle),
            "Carousel public module should expose stable API contract `{needle}`."
        );
    }
}

#[test]
fn carousel_selection_axis_keeps_controlled_uncontrolled_triplet_contract() {
    let source = load_source("src/view.rs");

    for needle in [
        "#[prop(optional)] selected_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] default_selected_index: Option<usize>",
        "#[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>",
        "let has_custom_selected_index = selected_index.is_some()",
        "let has_custom_default_selected_index = default_selected_index.is_some()",
        "let has_custom_on_selected_index_change = on_selected_index_change.is_some()",
        "let is_controlled = has_custom_selected_index;",
        "let selected_state = use_controllable_state(",
        "selected_index,",
        "Some(default_selected_index),",
        "on_selected_index_change,",
        "selected_state.request_change.run(next);",
        "data-selection-mode=move || root_state.get().selection_mode_attr",
    ] {
        assert!(
            source.contains(needle),
            "Carousel selection axis must keep controlled/uncontrolled triplet via `{needle}`."
        );
    }
}

#[test]
fn carousel_view_uses_logic_as_single_default_source() {
    let source = load_source("src/view.rs");

    assert!(
        source.contains("logic::resolve_default_selected_index(default_selected_index, items.get_value().as_ref())"),
        "Carousel view should consume default selected index from a single logic entrypoint."
    );
    assert!(
        !source.contains("logic::sanitize_index(default_selected_index, item_count)"),
        "Carousel view must not perform default-index priority normalization inline."
    );
    assert!(
        !source.contains("logic::resolve_initial_selected_index("),
        "Carousel view must not apply secondary default fallback logic."
    );
}

#[test]
fn carousel_item_api_avoids_parallel_array_contracts() {
    let view_source = load_source("src/view.rs");
    let readme_source = load_source("src/README.md");

    assert!(
        view_source.contains("items: Vec<CarouselItem>,"),
        "Carousel API should model item input as `Vec<CarouselItem>`."
    );
    for forbidden in [
        "labels: Vec<",
        "titles: Vec<",
        "panels: Vec<",
        "children: Vec<",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Carousel API should not expose parallel-array props via `{forbidden}`."
        );
    }

    assert!(
        readme_source.contains("items=vec![CarouselItem::new(\"welcome\", \"Welcome\")]"),
        "README should keep the default path bound to a typed item structure."
    );
    for forbidden in [
        "labels=vec![",
        "titles=vec![",
        "panels=vec![",
        "children=vec![",
    ] {
        assert!(
            !readme_source.contains(forbidden),
            "README should not recommend parallel-array usage via `{forbidden}`."
        );
    }
}

#[test]
fn carousel_does_not_define_dragging_state_machine_paths() {
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for forbidden in [
        "Dragging",
        "Action::DragEnd",
        "on:pointermove",
        "on:pointerdown",
        "on:pointerup",
        "on:touchmove",
        "on:mousemove",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Carousel view should not define drag state-machine hooks via `{forbidden}`."
        );
    }

    for forbidden in ["drag", "DragEnd", "requestAnimationFrame", "raf"] {
        assert!(
            !motion_source.contains(forbidden),
            "Carousel motion should not encode drag-loop runtime via `{forbidden}`."
        );
    }
}

#[test]
fn carousel_does_not_inline_geometry_two_pass_rectification() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for forbidden in [
        "getBoundingClientRect",
        "clientWidth",
        "clientHeight",
        "offsetWidth",
        "offsetHeight",
        "ResizeObserver",
        "IntersectionObserver",
        "Rectification",
        "Intent",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Carousel should not inline geometry two-pass logic via `{forbidden}`."
        );
    }

    assert!(
        motion_source.contains("attach_active_highlight_motion("),
        "Carousel motion should delegate runtime measurement/motion plumbing to shared primitive."
    );
}

#[test]
fn carousel_does_not_require_dynamic_registration_protocol() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    assert!(
        view_source.contains("items: Vec<CarouselItem>,"),
        "Carousel should consume a deterministic item list input."
    );
    assert!(
        logic_source.contains("pub fn resolve_items(id_base: &str, items: Vec<CarouselItem>)"),
        "Carousel logic should resolve items from a typed Vec input."
    );

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Carousel should not depend on dynamic registration protocol via `{forbidden}`."
        );
    }
}

#[test]
fn carousel_uses_explicit_eager_slot_projection_strategy() {
    let view_source = load_source("src/view.rs");

    assert!(
        view_source.contains(
            "<For each=move || indicator_indices.get_value() key=|index| *index children=render_slide />",
        ),
        "Carousel viewport should eagerly render slide slots from the full item index list."
    );

    for forbidden in ["KeepAlive", "Lazy", "NotifyHidden"] {
        assert!(
            !view_source.contains(forbidden),
            "Carousel should not expose untracked slot-projection mode `{forbidden}`."
        );
    }
}

#[test]
fn carousel_does_not_subscribe_env_streams_or_emit_env_actions() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "addEventListener(\"resize\"",
        "BreakpointChanged",
        "debounce",
        "throttle",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Carousel should not implement env-stream subscriptions via `{forbidden}`."
        );
    }
}

#[test]
fn carousel_does_not_use_bulk_collection_light_cone_protocols() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for forbidden in [
        "ContextBus",
        "Context Bus",
        "SelectionState::All",
        "select_all",
        "bulk_select",
        "Selector<",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Carousel should not rely on bulk collection event-light-cone protocol `{forbidden}`."
        );
    }
}

#[test]
fn carousel_does_not_require_causality_bus_trace_forwarding() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for forbidden in [
        "TraceId",
        "trace_id",
        "broadcast",
        "subscriber",
        "publish",
        "dispatch_bus",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Carousel should not depend on causality-bus trace forwarding via `{forbidden}`."
        );
    }
}

#[test]
fn carousel_does_not_implement_overlay_focus_stack_restore_paths() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for forbidden in [
        "Overlay",
        "overlay_open",
        "FocusManager",
        "focus_manager",
        "focus_stack",
        "FallbackTo",
        "fallback_to",
        "return_focus",
        "restore_focus",
        "document.body",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Carousel should not implement overlay focus-stack restoration path `{forbidden}`.",
        );
    }
}

#[test]
fn carousel_does_not_integrate_imperative_third_party_foreign_zone_paths() {
    let mod_source = load_source("src/mod.rs");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "YieldControl",
        "CleanupForeign",
        "Foreign Zone",
        "foreign_zone",
        "third_party",
        "external_instance",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Carousel should not integrate imperative third-party foreign-zone path `{forbidden}`.",
        );
    }

    for forbidden_public_type in [
        "pub type CarouselECharts",
        "pub type CarouselMap",
        "pub struct CarouselECharts",
        "pub struct CarouselMap",
        "pub enum CarouselForeign",
        "pub fn mount_echarts",
        "pub fn mount_map",
    ] {
        assert!(
            !mod_source.contains(forbidden_public_type),
            "Carousel public API should not expose third-party instance type `{forbidden_public_type}`.",
        );
    }
}

#[test]
fn carousel_hydration_ids_are_deterministic_without_time_or_random_sources() {
    let mod_source = load_source("src/mod.rs");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for forbidden in [
        "SystemTime::now",
        "Instant::now",
        "Date::now",
        "now()",
        "Uuid",
        "uuid",
        "rand::",
        "random(",
        "thread_rng",
        "getrandom",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden),
            "Carousel should not build hydration IDs from nondeterministic source `{forbidden}`.",
        );
    }

    for required in [
        "id_base: String,",
        "let id_base = logic::normalize_id_base(id_base);",
        "let has_custom_id_base = id_base != logic::DEFAULT_ID_BASE;",
        "let items = logic::resolve_items(&id_base.get_value(), items);",
        "pub fn normalize_id_base(id_base: String) -> String",
        "carousel_primitives::normalize_id_base(id_base)",
        "pub fn resolve_items(id_base: &str, items: Vec<CarouselItem>) -> Vec<CarouselItemResolved>",
        "carousel_primitives::resolve_items(id_base, primitive_items)",
    ] {
        assert!(
            view_source.contains(required)
                || logic_source.contains(required)
                || mod_source.contains(required),
            "Carousel should keep deterministic ID derivation contract `{required}`.",
        );
    }
}

#[test]
fn carousel_state_markers_are_observable_and_selector_friendly() {
    let view_source = load_source("src/view.rs");

    for required in [
        "data-state=move || root_state.get().state_attr",
        "data-items=move || root_state.get().item_attr",
        "data-selection=move || root_state.get().selected_attr",
        "data-focus=move || root_state.get().focus_attr",
        "data-has-disabled-items=move || root_state.get().has_disabled_items.then_some(\"true\")",
        "data-selection-mode=move || root_state.get().selection_mode_attr",
        "data-id-source=move || root_state.get().id_source_attr",
        "data-aria-label-source=move || root_state.get().aria_label_source_attr",
        "data-orientation-source=move || root_state.get().orientation_source_attr",
        "data-loop-navigation-source=move || root_state.get().loop_navigation_source_attr",
        "data-selected-index-source=move || root_state.get().selected_index_source_attr",
        "data-default-selected-index-source=move || root_state.get().default_selected_index_source_attr",
        "data-selected-index-change-source=move || root_state.get().selected_index_change_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "data-state=move || item_state.get().status.as_attr()",
        "data-selected=move || item_state.get().selected_attr",
        "data-focused=move || item_state.get().focused_attr",
        "data-disabled=move || item_state.get().disabled_attr",
        "role=root_a11y.attrs.role",
        "aria-label=root_a11y.attrs.aria_label.clone()",
        "aria-hidden=move || carousel_slide_a11y_attrs(item_state.get().is_selected).aria_hidden",
    ] {
        assert!(
            view_source.contains(required),
            "Carousel should expose stable semantic marker `{required}`."
        );
    }

    for forbidden in [
        "data-state=format!",
        "data-selection=format!",
        "data-focus=format!",
        "data-id-source=format!",
        "data-aria-label-source=format!",
        "data-orientation-source=format!",
        "data-loop-navigation-source=format!",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Carousel marker values should stay closed-set and not be free-form via `{forbidden}`."
        );
    }
}

#[test]
fn carousel_styles_depend_on_explicit_semantic_state_not_dom_guessing() {
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");

    for required in [
        ".ui-carousel__slide[data-selected=\"true\"]",
        ".ui-carousel__slide[data-state=\"selected\"]",
        ".ui-carousel__slide[data-disabled=\"true\"]",
        ".ui-carousel__indicator[data-state=\"selected\"] .ui-carousel__indicator-dot",
        ".ui-carousel__indicator[data-disabled=\"true\"]",
        ".ui-carousel[data-state=\"empty\"] .ui-carousel__viewport",
        ".ui-carousel[data-state=\"selected\"] .ui-carousel__viewport",
    ] {
        assert!(
            styles_source.contains(required),
            "Carousel visual state switching should be explained by semantic selectors `{required}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":has(", " + ", " ~ "] {
        assert!(
            !styles_source.contains(forbidden),
            "Carousel styles should not use fragile structural guessing selector `{forbidden}`."
        );
    }

    for forbidden in ["style=", "style:"] {
        assert!(
            !view_source.contains(forbidden),
            "Carousel view should avoid inline business style logic `{forbidden}`."
        );
    }
}

#[test]
fn carousel_a11y_i18n_l10n_contract_is_headless_driven_and_not_hardcoded_in_view() {
    let view_source = load_source("src/view.rs");
    let mod_source = load_source("src/mod.rs");
    let i18n_source = load_source("src/i18n.rs");
    let logic_source = load_source("src/logic.rs");
    let headless_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");
    let signature = function_signature(&view_source, "Carousel");

    for required in [
        "use ui_headless::{",
        "i18n, labeled_group_attrs, labeled_toolbar_attrs,",
        "let i18n = i18n::use_ui_i18n();",
        "let strings = i18n.strings::<CarouselStrings>();",
        "logic::resolve_aria_label_with_fallback(aria_label, strings.aria_label.as_ref())",
        "logic::resolve_label_with_fallback(previous_label, strings.previous_label.as_ref())",
        "logic::resolve_label_with_fallback(next_label, strings.next_label.as_ref())",
        "logic::resolve_label_with_fallback(",
        "strings.indicator_aria_label_template.as_ref()",
        "logic::resolve_indicator_aria_label(",
        "role=root_a11y.attrs.role",
        "lang=root_a11y.attrs.lang.clone()",
        "dir=root_a11y.attrs.dir",
        "role=controls_a11y.role",
        "aria-orientation=controls_a11y.aria_orientation",
        "role=indicators_a11y.role",
    ] {
        assert!(
            view_source.contains(required),
            "Carousel should wire a11y/i18n/l10n contract via `{required}`."
        );
    }

    for required in [
        "mod i18n;",
        "pub use i18n::CarouselStrings;",
        "DEFAULT_PREVIOUS_LABEL",
        "DEFAULT_NEXT_LABEL",
        "DEFAULT_INDICATOR_ARIA_LABEL_TEMPLATE",
    ] {
        assert!(
            mod_source.contains(required),
            "Carousel public module should export i18n/default label contract `{required}`."
        );
    }

    for required in [
        "pub struct CarouselStrings",
        "pub aria_label: Arc<str>",
        "pub controls_aria_label: Arc<str>",
        "pub indicators_aria_label: Arc<str>",
        "pub previous_label: Arc<str>",
        "pub next_label: Arc<str>",
        "pub indicator_aria_label_template: Arc<str>",
    ] {
        assert!(
            i18n_source.contains(required),
            "Carousel i18n bundle should include `{required}`."
        );
    }

    for required in [
        "pub fn resolve_aria_label_with_fallback(",
        "pub fn resolve_label_with_fallback(",
        "pub fn resolve_indicator_aria_label(",
    ] {
        assert!(
            logic_source.contains(required),
            "Carousel logic should centralize label fallback normalization via `{required}`."
        );
    }

    for required in [
        "pub fn labeled_group_attrs(",
        "pub fn labeled_toolbar_attrs(",
        "pub fn locale_attrs(lang: Option<String>, dir: Option<A11yDirection>)",
    ] {
        assert!(
            headless_a11y_source.contains(required),
            "Shared a11y tools should come from ui-headless via `{required}`."
        );
    }

    for required in [
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
        "controls_aria_label: Option<String>",
        "indicators_aria_label: Option<String>",
        "previous_label: Option<String>",
        "next_label: Option<String>",
        "indicator_aria_label_template: Option<String>",
    ] {
        assert!(
            signature.contains(required),
            "Carousel public API should expose locale/i18n entrypoint `{required}`."
        );
    }

    for forbidden in [
        "\"Previous\"",
        "\"Next\"",
        "format!(\"Go to {}\"",
        "role=\"toolbar\"",
        "role=\"group\"",
        "dir=\"ltr\"",
        "dir=\"rtl\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Carousel view should not hardcode user-visible copy or duplicated locale/a11y literals `{forbidden}`."
        );
    }
}

#[test]
fn carousel_files_keep_single_responsibility_boundaries() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for required in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Carousel;",
    ] {
        assert!(
            mod_source.contains(required),
            "mod.rs should keep export boundary contract `{required}`."
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "pub const CSS:",
        "use_carousel_root(",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not carry implementation detail `{forbidden}`."
        );
    }

    for required in [
        "use ui_state_primitives::carousel as carousel_primitives;",
        "pub fn resolve_state(input: CarouselPartStateInput) -> CarouselPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: CarouselPartState) -> String",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalization/derivation contract `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "NodeRef<",
        "web_sys",
        "web-sys",
        "on:click=",
        "on:keydown=",
        "aria-label=",
        ".ui-carousel",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not contain view/dom/style detail `{forbidden}`."
        );
    }

    for required in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first static css contract `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "use_carousel_root(",
        "on:click=",
        "on:keydown=",
        "NodeRef<",
        "web_sys",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not carry logic/view behavior `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "use ui_headless::{",
        "logic::resolve_state(CarouselPartStateInput {",
        "crate::motion::attach_carousel_indicator_motion(",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep structure + headless mount contract `{required}`."
        );
    }

    for forbidden in [
        "pub const CSS: &str",
        "ui_motion::spring::sanitize_config(",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not carry style/motion-engine detail `{forbidden}`."
        );
    }

    for required in [
        "pub fn sanitize_motion(motion: super::CarouselMotion) -> super::CarouselMotion",
        "ui_motion::spring::sanitize_config(motion.spring, default.spring)",
        "pub fn attach_carousel_indicator_motion(",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should keep motion-contract mapping `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "use_carousel_root(",
        "on:click=",
        "on:keydown=",
        "pub const CSS:",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not carry view/style/interaction decision detail `{forbidden}`."
        );
    }
}

#[test]
fn carousel_semantic_contract_matrix_covers_state_interaction_and_non_snapshot_policy() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let semantics_source = load_source("test/semantics.rs");
    let workspace_semantics_source =
        load_source("../../crates/ui-components/tests/carousel_semantics.rs");

    for required in [
        "role=root_a11y.attrs.role",
        "aria-label=root_a11y.attrs.aria_label.clone()",
        "data-state=move || root_state.get().state_attr",
        "data-selected-index-source=move || root_state.get().selected_index_source_attr",
        "data-disabled=move || item_state.get().disabled_attr",
        "aria-disabled=controls_a11y.aria_disabled",
        "on:keydown=on_key_down",
        "on:click=on_prev",
        "on:click=on_next",
        "on:click=on_click",
        "on:focus=on_focus",
        "logic::can_item_receive_selection(item_disabled)",
    ] {
        assert!(
            view_source.contains(required),
            "Carousel view should expose semantic matrix contract `{required}`.",
        );
    }

    for required in [
        "fn carousel_selection_axis_keeps_controlled_uncontrolled_triplet_contract()",
        "fn carousel_state_markers_are_observable_and_selector_friendly()",
        "fn carousel_agent_contract_schema_markers_are_typed_traceable_and_whitelist_rendered()",
        "fn carousel_contract_hygiene_check_script_covers_agent_contract_schema_gate()",
        "fn carousel_check2_marks_agent_contract_schema_item_complete()",
        "fn carousel_check2_documents_streaming_definition_is_llm_output_only_with_two_modes()",
        "fn carousel_stays_snapshot_only_and_does_not_mount_stream_contract_fields()",
        "fn carousel_check2_documents_snapshot_as_default_baseline_capability()",
        "fn carousel_snapshot_baseline_consumes_complete_result_and_renders_stably()",
        "fn carousel_check2_documents_streaming_required_optional_classification_rules()",
        "fn carousel_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous()",
        "fn carousel_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer()",
        "fn carousel_streaming_check_script_covers_required_optional_classification_contract()",
        "fn carousel_streaming_check_script_covers_snapshot_only_contract()",
        "fn carousel_platform_build_contract_uses_explicit_web_ssr_features()",
        "fn carousel_headless_feature_mutex_guard_is_preserved()",
        "fn carousel_motion_non_wasm_noop_contract_is_preserved()",
        "fn carousel_reduced_motion_ssr_wasm_branch_contract_is_preserved()",
        "fn carousel_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe()",
        "fn carousel_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()",
        "fn carousel_version_deprecation_migration_script_covers_engineering_gate()",
        "fn carousel_performance_governance_contract_is_mount_only_traceable_and_blocking()",
        "fn carousel_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
        "fn carousel_view_macro_complexity_is_split_into_semantic_subrenders()",
        "fn carousel_view_functional_split_prefers_plain_functions_over_local_components()",
        "fn carousel_static_fragments_are_constantized_or_absent_for_simple_layout()",
        "fn carousel_inner_html_usage_is_explicitly_na_and_guarded()",
        "fn carousel_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated()",
        "fn carousel_dx_playground_supports_css_hot_reload_without_wasm_rebuild()",
        "fn carousel_dx_workbench_supports_optional_state_persistence_and_isolated_canvas()",
        "fn carousel_exposes_keyboard_and_control_contracts()",
        "fn carousel_styles_depend_on_explicit_semantic_state_not_dom_guessing()",
    ] {
        assert!(
            semantics_source.contains(required),
            "Component-level semantics suite should include `{required}`.",
        );
    }

    for required in [
        "fn carousel_supports_controlled_and_uncontrolled_selection_state()",
        "fn carousel_view_uses_logic_contracts_and_source_markers()",
        "fn carousel_agent_contract_schema_markers_are_typed_traceable_and_whitelist_rendered()",
        "fn carousel_contract_hygiene_check_script_covers_agent_contract_schema_gate()",
        "fn carousel_check2_marks_agent_contract_schema_item_complete()",
        "fn carousel_check2_documents_streaming_definition_is_llm_output_only_with_two_modes()",
        "fn carousel_stays_snapshot_only_and_does_not_mount_stream_contract_fields()",
        "fn carousel_check2_documents_snapshot_as_default_baseline_capability()",
        "fn carousel_snapshot_baseline_consumes_complete_result_and_renders_stably()",
        "fn carousel_check2_documents_streaming_required_optional_classification_rules()",
        "fn carousel_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous()",
        "fn carousel_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer()",
        "fn carousel_streaming_check_script_covers_required_optional_classification_contract()",
        "fn carousel_streaming_check_script_covers_snapshot_only_contract()",
        "fn carousel_platform_build_contract_uses_explicit_web_ssr_features()",
        "fn carousel_headless_feature_mutex_guard_is_preserved()",
        "fn carousel_motion_non_wasm_noop_contract_is_preserved()",
        "fn carousel_reduced_motion_ssr_wasm_branch_contract_is_preserved()",
        "fn carousel_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe()",
        "fn carousel_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()",
        "fn carousel_version_deprecation_migration_script_covers_engineering_gate()",
        "fn carousel_performance_governance_contract_is_mount_only_traceable_and_blocking()",
        "fn carousel_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
        "fn carousel_view_macro_complexity_is_split_into_semantic_subrenders()",
        "fn carousel_view_functional_split_prefers_plain_functions_over_local_components()",
        "fn carousel_static_fragments_are_constantized_or_absent_for_simple_layout()",
        "fn carousel_inner_html_usage_is_explicitly_na_and_guarded()",
        "fn carousel_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated()",
        "fn carousel_dx_playground_supports_css_hot_reload_without_wasm_rebuild()",
        "fn carousel_dx_workbench_supports_optional_state_persistence_and_isolated_canvas()",
        "fn carousel_exposes_keyboard_and_control_contracts()",
        "fn carousel_styles_avoid_fragile_dom_structure_guessing_and_view_inline_business_styles()",
    ] {
        assert!(
            workspace_semantics_source.contains(required),
            "Workspace semantics suite should include `{required}`.",
        );
    }

    for (prefix, suffix) in [
        ("insta::assert_", "snapshot!"),
        ("assert_", "snapshot!"),
        ("assert_debug_", "snapshot!"),
        ("snapshot", "("),
    ] {
        let forbidden = format!("{prefix}{suffix}");
        assert!(
            !semantics_source.contains(&forbidden)
                && !workspace_semantics_source.contains(&forbidden),
            "Semantic contract validation should not rely on snapshot-only assertion `{forbidden}`.",
        );
    }

    for forbidden in [
        "cfg(target_arch = \"wasm32\")",
        "cfg(not(target_arch = \"wasm32\"))",
        "web_sys",
        "web-sys",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Semantic contract path should stay target-agnostic without platform split `{forbidden}`.",
        );
    }
}

#[test]
fn carousel_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let view_source = load_source("src/view.rs");
    let local_semantics_source = load_source("test/semantics.rs");
    let workspace_semantics_source =
        load_source("../../crates/ui-components/tests/carousel_semantics.rs");
    let perf_script_source = load_source("../../scripts/check-ui-components-performance.sh");

    assert!(
        path_exists("test/semantics.rs")
            && path_exists("../../crates/ui-components/tests/carousel_semantics.rs"),
        "Carousel semantic-priority gate requires component/local `*_semantics.rs` suites.",
    );

    for marker in [
        "role=root_a11y.attrs.role",
        "aria-label=root_a11y.attrs.aria_label.clone()",
        "role=controls_a11y.role",
        "role=indicators_a11y.role",
        "data-state=move || root_state.get().state_attr",
        "data-selection=move || root_state.get().selected_attr",
        "data-focus=move || root_state.get().focus_attr",
        "data-selected-index-source=move || root_state.get().selected_index_source_attr",
        "data-selected-index-change-source=move || root_state.get().selected_index_change_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "on:keydown=on_key_down",
        "on:focus=on_focus",
        "on:click=on_prev",
        "on:click=on_next",
        "on:click=on_click",
    ] {
        assert!(
            view_source.contains(marker),
            "Carousel semantic-priority contract should keep marker `{marker}`.",
        );
    }

    for marker in [
        "fn carousel_semantic_contract_matrix_covers_state_interaction_and_non_snapshot_policy()",
        "fn carousel_state_markers_are_observable_and_selector_friendly()",
        "for (prefix, suffix) in [",
        "Semantic contract validation should not rely on snapshot-only assertion",
    ] {
        assert!(
            local_semantics_source.contains(marker),
            "Carousel local semantics suite should keep semantic-priority marker `{marker}`.",
        );
    }

    for marker in [
        "fn carousel_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()",
        "fn carousel_semantic_contract_matrix_is_asserted_by_semantics_instead_of_snapshots()",
    ] {
        assert!(
            workspace_semantics_source.contains(marker),
            "Workspace carousel semantics suite should keep semantic-priority marker `{marker}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script_source.contains(script_needle),
        "performance script should include carousel semantic-priority gate `{script_needle}`.",
    );
}

#[test]
fn carousel_performance_script_covers_semantic_test_priority_contract() {
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for marker in [
        "echo \"[perf] contract: carousel semantic test priority\"",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
    ] {
        assert!(
            script_source.contains(marker),
            "performance script should include carousel semantic-priority marker `{marker}`.",
        );
    }
}

#[test]
fn carousel_check2_marks_semantic_test_priority_contract_complete() {
    let source = load_source("check2.md");

    for marker in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "carousel_semantic_contract_matrix_covers_state_interaction_and_non_snapshot_policy",
        "carousel_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "carousel_performance_script_covers_semantic_test_priority_contract",
        "scripts/check-ui-components-performance.sh",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(marker),
            "carousel check2 semantic-priority section should include `{marker}`.",
        );
    }
}

#[test]
fn carousel_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel check2 e2e selector stability section should include `{required}`.",
        );
    }
}

#[test]
fn carousel_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_carousel_contract.spec.mjs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for marker in [
        "docs-app carousel contract uses semantic selectors with wasm-stable ready waits",
        "docs-app carousel motion interaction uses semantic ready and settled breakpoints",
        "await page.goto(\"/#/components/carousel\");",
        "await waitForWasmReady(page);",
        "[data-component=\"carousel\"]",
        "[data-slot=\"carousel-state-matrix\"]",
        "[data-slot=\"carousel-controlled-uncontrolled\"]",
        "[data-slot=\"carousel-e2e-markers\"]",
        "[data-slot=\"carousel-e2e-select-overview\"]",
        "[data-slot=\"carousel-e2e-select-analytics\"]",
        "[data-slot=\"carousel-e2e-clear\"]",
        "[data-slot=\"carousel-controls\"]",
        "[data-slot=\"carousel-indicators\"]",
    ] {
        assert!(
            e2e_source.contains(marker),
            "carousel e2e selector contract should include `{marker}`.",
        );
    }

    for marker in [
        "data-slot=\"carousel-state-matrix\"",
        "data-slot=\"carousel-controlled-uncontrolled\"",
        "data-slot=\"carousel-e2e-markers\"",
        "data-slot=\"carousel-e2e-marker-actions\"",
        "data-slot=\"carousel-e2e-select-overview\"",
        "data-slot=\"carousel-e2e-select-analytics\"",
        "data-slot=\"carousel-e2e-clear\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "carousel docs should expose stable e2e semantic anchor `{marker}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        ":nth-child(",
        "getByText(",
        "toHaveScreenshot(",
        "toMatchSnapshot(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "carousel e2e selector contract should avoid brittle wait/selector token `{forbidden}`.",
        );
    }
}

#[test]
fn carousel_e2e_contract_covers_ready_and_settled_conditions_for_motion_paths() {
    let e2e_source = load_source("../../e2e/tests/docs_app_carousel_contract.spec.mjs");

    for marker in [
        "const WASM_READY_SELECTOR = \"body:not(:has(#boot))\";",
        "await expect(carouselRoot).toHaveAttribute(\"data-ui-schema\", \"ui.carousel.agent\");",
        "await expect(carouselRoot).toHaveAttribute(\"data-ui-output-status\", \"verified\");",
        "await expect(markerCarousel).toHaveAttribute(\"data-motion-source\", \"custom\");",
        "await expect(markerCarousel).toHaveAttribute(\"data-selection-mode\", \"controlled\");",
        "await expect(markerCarousel).toHaveAttribute(\"data-selected-index-source\", \"external\");",
        "await expectCarouselSettledSelection(markerCarousel, 0);",
        "await expectCarouselSettledSelection(markerCarousel, 1);",
        "await expect(markerCarousel).not.toHaveAttribute(\"data-selected-index\", /[0-9]+/);",
        "await expect(markerCarousel).toHaveAttribute(\"data-selection\", \"idle\");",
        "await expect(selectedIndicator).toHaveAttribute(\"data-selected\", \"true\");",
    ] {
        assert!(
            e2e_source.contains(marker),
            "carousel e2e ready/settled contract should include `{marker}`.",
        );
    }
}

#[test]
fn carousel_e2e_check_script_covers_selector_and_settled_wait_contract() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-carousel.sh");

    for marker in [
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_e2e_contract_covers_ready_and_settled_conditions_for_motion_paths",
    ] {
        assert!(
            script_source.contains(marker),
            "carousel e2e check script should include `{marker}`.",
        );
    }
}

#[test]
fn carousel_check2_marks_e2e_selector_stability_item_complete() {
    let check2_source = load_source("check2.md");

    assert!(
        check2_source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "carousel check2 should mark e2e selector stability item complete.",
    );

    for marker in [
        "components/carousel/test/semantics.rs::carousel_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/carousel/test/semantics.rs::carousel_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "components/carousel/test/semantics.rs::carousel_e2e_contract_covers_ready_and_settled_conditions_for_motion_paths",
        "components/carousel/test/semantics.rs::carousel_e2e_check_script_covers_selector_and_settled_wait_contract",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_check2_documents_e2e_selector_and_stable_wait_rules",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_e2e_contract_covers_ready_and_settled_conditions_for_motion_paths",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_e2e_check_script_covers_selector_and_settled_wait_contract",
        "e2e/tests/docs_app_carousel_contract.spec.mjs",
        "scripts/check-ui-components-e2e-carousel.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "carousel check2 e2e selector stability section should include `{marker}`.",
        );
    }
}

#[test]
fn carousel_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = load_source("check2.md");

    for marker in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(marker),
            "carousel check2 repeatable-key-flow section should include `{marker}`.",
        );
    }
}

#[test]
fn carousel_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_carousel_contract.spec.mjs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for marker in [
        "docs-app carousel key flow is repeatable with semantic breakpoints",
        "for (const cycle of [1, 2])",
        "carousel key flow cycle ${cycle}",
        "[data-slot=\"carousel-workbench\"]",
        "[data-slot=\"carousel-workbench-select-0\"]",
        "[data-slot=\"carousel-workbench-select-1\"]",
        "[data-slot=\"carousel-workbench-clear\"]",
        "[data-slot=\"carousel-prev\"]",
        "[data-slot=\"carousel-next\"]",
        "await expectCarouselSettledSelection(workbenchCarousel, 0);",
        "await expectCarouselSettledSelection(workbenchCarousel, 1);",
        "data-selection-mode\", \"controlled\"",
        "data-selected-index-source\", \"external\"",
        "data-selected-index-change-source\", \"custom\"",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(marker),
            "carousel repeatable key-flow contract should include `{marker}`.",
        );
    }

    for marker in [
        "data-slot=\"carousel-workbench\"",
        "data-slot=\"carousel-workbench-actions\"",
        "data-slot=\"carousel-workbench-select-0\"",
        "data-slot=\"carousel-workbench-select-1\"",
        "data-slot=\"carousel-workbench-clear\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "carousel docs should expose repeatable key-flow anchor `{marker}`.",
        );
    }
}

#[test]
fn carousel_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_carousel_contract.spec.mjs");

    for marker in [
        "docs-app carousel high-risk paths keep focus keyboard and settled semantic breakpoints",
        "[data-slot=\"carousel-e2e-markers\"]",
        "[data-slot=\"carousel-indicator\"][data-index=\"0\"]",
        "[data-slot=\"carousel-indicator\"][data-index=\"1\"]",
        "await expect(indicator0).toHaveAttribute(\"data-focused\", \"true\");",
        "await expect(indicator1).toHaveAttribute(\"data-focused\", \"true\");",
        "await markerCarousel.focus();",
        "await expect(markerCarousel).toBeFocused();",
        "await page.keyboard.press(\"ArrowLeft\");",
        "await page.keyboard.press(\"ArrowRight\");",
        "data-motion-source\", \"custom\"",
        "data-selection-mode\", \"controlled\"",
        "data-selected-index-source\", \"external\"",
        "data-selected-index-change-source\", \"custom\"",
    ] {
        assert!(
            e2e_source.contains(marker),
            "carousel high-risk e2e path should include semantic breakpoint `{marker}`.",
        );
    }
}

#[test]
fn carousel_e2e_check_script_covers_repeatable_key_flow_contract() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-carousel.sh");

    for marker in [
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(marker),
            "carousel e2e check script should include repeatable/high-risk marker `{marker}`.",
        );
    }
}

#[test]
fn carousel_check2_marks_e2e_repeatable_key_flow_item_complete() {
    let check2_source = load_source("check2.md");

    assert!(
        check2_source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "carousel check2 should mark repeatable key-flow item complete.",
    );

    for marker in [
        "components/carousel/test/semantics.rs::carousel_check2_documents_e2e_repeatable_key_flow_rules",
        "components/carousel/test/semantics.rs::carousel_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "components/carousel/test/semantics.rs::carousel_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "components/carousel/test/semantics.rs::carousel_check2_marks_e2e_repeatable_key_flow_item_complete",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_check2_documents_e2e_repeatable_key_flow_rules",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "crates/ui-components/tests/carousel_semantics.rs::carousel_check2_marks_e2e_repeatable_key_flow_item_complete",
        "scripts/check-ui-components-e2e-carousel.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "carousel check2 repeatable key-flow section should include `{marker}`.",
        );
    }
}

#[test]
fn carousel_agent_contract_schema_markers_are_typed_traceable_and_whitelist_rendered() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for required in [
        "pub enum CarouselAgentSchemaVersion",
        "pub enum CarouselAgentIntent",
        "pub enum CarouselAgentAction",
        "pub enum CarouselAgentStateAxis",
        "pub enum CarouselAgentSourceAxis",
        "pub enum CarouselAgentConfigPolicy",
        "pub struct CarouselAgentContract",
        "pub fn resolve_agent_contract(state: CarouselPartState) -> CarouselAgentContract",
        "schema_name: \"ui.carousel.agent\"",
        "intent: CarouselAgentIntent::NavigateSlides",
        "config_policy: CarouselAgentConfigPolicy::WhitelistOnly",
        "pub enum CarouselAgentOutputStatus",
        "output_status: CarouselAgentOutputStatus::Verified",
    ] {
        assert!(
            logic_source.contains(required),
            "Carousel agent contract should stay type-derived via `{required}`.",
        );
    }

    for required in [
        "let agent_contract =",
        "logic::resolve_agent_contract(root_state_for_agent.get())",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "Carousel view should expose typed Agent Contract marker `{required}`.",
        );
    }

    for forbidden in [
        "format!(\"data-ui",
        "inner_html=",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Carousel agent contract render path should stay whitelist-only and reject `{forbidden}`.",
        );
    }
}

#[test]
fn carousel_contract_hygiene_check_script_covers_agent_contract_schema_gate() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");
    let required = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_agent_contract_schema_markers_are_typed_traceable_and_whitelist_rendered";

    assert!(
        script_source.contains(required),
        "contract-hygiene check script should enforce `{required}`.",
    );
}

#[test]
fn carousel_check2_marks_agent_contract_schema_item_complete() {
    let check2_source = load_source("check2.md");

    assert!(
        check2_source.contains("- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。"),
        "carousel check2 should mark Agent Contract schema checklist item complete.",
    );

    for required in [
        "components/carousel/src/logic.rs",
        "components/carousel/src/view.rs",
        "data-ui-schema",
        "data-ui-intent",
        "data-ui-action",
        "data-ui-state",
        "data-ui-source",
        "data-ui-config-policy",
        "data-ui-output-status",
        "carousel_agent_contract_schema_markers_are_typed_traceable_and_whitelist_rendered",
        "carousel_contract_hygiene_check_script_covers_agent_contract_schema_gate",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel check2 Agent Contract section should reference `{required}`.",
        );
    }
}

#[test]
fn carousel_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let checklist_source = load_source("check2.md");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "Carousel 不是 LLM 正文渲染组件，当前实现按 `snapshot-only` 归类；该术语约束仅用于防止误把普通交互组件当作流式文本承载层。",
        "carousel_stays_snapshot_only_and_does_not_mount_stream_contract_fields",
    ] {
        assert!(
            checklist_source.contains(required),
            "Carousel checklist should keep streaming-definition marker `{required}`.",
        );
    }
}

#[test]
fn carousel_stays_snapshot_only_and_does_not_mount_stream_contract_fields() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let mod_source = load_source("src/mod.rs");
    let motion_source = load_source("src/motion.rs");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "streaming",
        "data-ui-stream",
        "data-stream",
        "data-draft",
        "data-verified",
        "data-commit-ready",
        "fallback=snapshot",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Carousel is snapshot-only for LLM output semantics; forbidden marker `{forbidden}` should remain absent.",
        );
    }
}

#[test]
fn carousel_check2_documents_snapshot_as_default_baseline_capability() {
    let checklist_source = load_source("check2.md");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "carousel_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            checklist_source.contains(required),
            "Carousel checklist should keep snapshot-baseline marker `{required}`.",
        );
    }
}

#[test]
fn carousel_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for required in [
        "items: Vec<CarouselItem>,",
        "let items = logic::resolve_items(&id_base.get_value(), items);",
        "let item_count = items.get_value().len();",
        "let indicator_indices: StoredValue<Vec<usize>> = StoredValue::new((0..item_count).collect());",
        "<For each=move || indicator_indices.get_value() key=|index| *index children=render_slide />",
        "<For each=move || indicator_indices.get_value() key=|index| *index children=render_indicator />",
        "render_carousel_slide(CarouselSlideRenderInput {",
        "render_carousel_indicator(CarouselIndicatorRenderInput {",
    ] {
        assert!(
            view_source.contains(required),
            "Carousel snapshot baseline should keep complete-result render path marker `{required}`.",
        );
    }

    for required in [
        "pub fn resolve_items(id_base: &str, items: Vec<CarouselItem>) -> Vec<CarouselItemResolved>",
        "carousel_primitives::resolve_items(id_base, primitive_items)",
    ] {
        assert!(
            logic_source.contains(required),
            "Carousel logic should keep stable item normalization marker `{required}`.",
        );
    }
}

#[test]
fn carousel_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("check2.md");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "Carousel 归类为 `Streaming Optional`；当前实现为 snapshot-only，显式声明 `fallback=snapshot`，并通过 `data-ui-output-status=\"verified\"` 输出当前状态。",
        "carousel_check2_documents_streaming_required_optional_classification_rules",
        "carousel_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "carousel_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "Carousel checklist should keep streaming required/optional marker `{required}`.",
        );
    }
}

#[test]
fn carousel_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("src/view.rs");

    for marker in [
        "role=root_a11y.attrs.role",
        "aria-label=root_a11y.attrs.aria_label.clone()",
        "lang=root_a11y.attrs.lang.clone()",
        "dir=root_a11y.attrs.dir",
        "data-state=move || root_state.get().state_attr",
        "data-selection=move || root_state.get().selected_attr",
        "data-focus=move || root_state.get().focus_attr",
        "data-selected-index-source=move || root_state.get().selected_index_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "Carousel optional-streaming scope should keep role/aria/data continuity marker `{marker}`.",
        );
    }
}

#[test]
fn carousel_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let mod_source = load_source("src/mod.rs");
    let motion_source = load_source("src/motion.rs");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "validate_stream",
        "stream_error",
        "network_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Carousel should keep validation/retry/resilience policy outside component layer; found `{forbidden}`.",
        );
    }
}

#[test]
fn carousel_streaming_check_script_covers_required_optional_classification_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn carousel_streaming_check_script_covers_snapshot_only_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_stays_snapshot_only_and_does_not_mount_stream_contract_fields",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn carousel_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let mod_source = load_source("src/mod.rs");
    let motion_source = load_source("src/motion.rs");
    let styles_source = load_source("src/styles.rs");
    let combined =
        format!("{logic_source}\n{view_source}\n{mod_source}\n{motion_source}\n{styles_source}");

    for forbidden in [".unwrap(", ".unwrap_err(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "Carousel non-test sources should forbid `{forbidden}`.",
        );
    }
}

#[test]
fn carousel_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for forbidden in [
        "fallback.trim().to_string()",
        "String::from(\"Carousel\")",
        "controls_aria_label.to_string()",
        "previous_label.to_string()",
        "next_label.to_string()",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Carousel string hotspot contract should avoid `{forbidden}`.",
        );
    }

    assert!(
        logic_source.contains("Vec<Cow<'static, str>>"),
        "Carousel class-name composition should converge hot path allocations to Cow.",
    );
}

#[test]
fn carousel_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let rust_hygiene_script = load_source("../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("../../scripts/check-ui-components-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            rust_hygiene_script.contains(required),
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }

    for required in [
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(required),
            "engineering check script should enforce `{required}`.",
        );
    }
}

#[test]
fn carousel_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("check2.md");

    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "carousel_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "carousel_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "carousel_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "RUST_HYGIENE_SCOPE=\"components/carousel\" ./scripts/check-rust-hygiene.sh",
        "scripts/check-ui-components-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "carousel check2 rust-hygiene section should reference `{required}`.",
        );
    }
}

#[test]
fn carousel_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
{
    let manifest_source = load_source("src/Component.toml");
    let protocol_source = load_source("src/protocol.rs");
    let rbi_source = load_source("src/carousel.rbi");
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");
    let i18n_source = load_source("src/i18n.rs");
    let check2_source = load_source("check2.md");

    for marker in [
        "schema_version = \"1\"",
        "name = \"Carousel\"",
        "crate = \"ui-carousel\"",
    ] {
        assert!(
            manifest_source.contains(marker),
            "Carousel manifest should keep stable v1 schema marker `{marker}`.",
        );
    }

    for marker in [
        "pub enum CarouselComponentSchemaVersion",
        "V1",
        "pub struct CarouselComponentSpec",
    ] {
        assert!(
            protocol_source.contains(marker),
            "Carousel protocol should keep stable v1 schema marker `{marker}`.",
        );
    }

    for marker in [
        "pub fn Carousel(",
        "selected_index: Option<leptos::prelude::Signal<Option<usize>>>",
        "default_selected_index: Option<usize>",
        "on_selected_index_change: Option<leptos::prelude::Callback<Option<usize>>>",
    ] {
        assert!(
            rbi_source.contains(marker),
            "Carousel RBI should keep stable public API marker `{marker}`.",
        );
    }

    let combined = format!(
        "{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}\n{i18n_source}\n{protocol_source}"
    );
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "contract.v2",
        "SchemaRegistry",
        "schema_registry",
        "codemod",
    ] {
        assert!(
            !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !combined.contains(forbidden),
            "Carousel should not introduce major-version migration marker `{forbidden}` in current scope.",
        );
    }

    for required in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `Carousel` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "CarouselComponentSchemaVersion::V1",
        "carousel_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
        "scripts/check-ui-components-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "Carousel check2 should keep version-migration governance marker `{required}`.",
        );
    }
}

#[test]
fn carousel_version_deprecation_migration_script_covers_engineering_gate() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");
    let marker = "cargo test -p ui-components --test carousel_semantics --no-default-features --features component-carousel,inject-css carousel_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";

    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`.",
    );
}
