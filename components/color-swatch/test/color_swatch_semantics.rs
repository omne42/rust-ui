use std::fs;
use std::path::Path;

#[path = "../../../components/color-swatch/test/semantics.rs"]
mod color_swatch_local_semantics;

fn resolve_source_path(rel_path: &str) -> Option<std::path::PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let mut candidates = vec![manifest_dir.join(rel_path)];

    if let Some(component_rel_path) = rel_path.strip_prefix("../../components/") {
        let direct = workspace_dir.join("components").join(component_rel_path);
        candidates.push(direct.clone());

        let parts: Vec<&str> = component_rel_path.split('/').collect();
        if parts.len() > 3 && parts.get(1) == Some(&"src") && parts.get(2) == parts.first() {
            let collapsed = workspace_dir
                .join("components")
                .join(parts[0])
                .join("src")
                .join(parts[3..].join("/"));
            candidates.push(collapsed);
        }
    }

    if let Some(src_rel_path) = rel_path.strip_prefix("src/") {
        let segments: Vec<&str> = src_rel_path.split('/').collect();
        let components_root = workspace_dir.join("components");

        if let Ok(entries) = fs::read_dir(&components_root) {
            let component_dirs: Vec<String> = entries
                .flatten()
                .filter_map(|entry| {
                    let path = entry.path();
                    path.is_dir()
                        .then(|| entry.file_name().to_string_lossy().to_string())
                })
                .collect();

            for component_dir in component_dirs {
                for start in 0..segments.len() {
                    for end in start..segments.len() {
                        let name = segments[start..=end]
                            .iter()
                            .map(|segment| segment.replace('_', "-"))
                            .collect::<Vec<_>>()
                            .join("-");

                        if name != component_dir {
                            continue;
                        }

                        if end + 1 >= segments.len() {
                            candidates
                                .push(components_root.join(&component_dir).join("src/mod.rs"));
                            candidates
                                .push(components_root.join(&component_dir).join("src/check2.md"));
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                            continue;
                        }

                        let suffix = segments[end + 1..].join("/");
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("src")
                                .join(&suffix),
                        );
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("test")
                                .join(&suffix),
                        );

                        if suffix == "check2.md" {
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                        }
                    }
                }
            }
        }
    }

    candidates.into_iter().find(|path| path.exists())
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_source_path(rel_path)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path));

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}
#[test]
fn color_swatch_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/color-swatch/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorSwatch internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_swatch_uses_logic_state_model() {
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/color_swatch.rs");
    let headless_source = load_source("../../crates/ui-headless/src/color_swatch.rs");

    for needle in [
        "pub enum ColorSwatchSize",
        "pub enum ColorSwatchRounding",
        "pub enum ColorSwatchShape",
        "pub enum ColorSwatchAlpha",
        "pub fn sanitize_color_value(",
        "pub fn resolve_alpha(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_is_bordered(",
        "pub fn normalize_is_decorative(",
        "pub fn resolve_state(",
        "pub enum ColorSwatchBoolSource",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ColorSwatch primitives should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "pub use ui_state_primitives::color_swatch::{",
        "ColorSwatchStateInput",
        "ColorSwatchRenderInput",
        "ColorSwatchRenderState",
        "pub fn resolve_render_state(",
        "pub fn compose_class_name(",
        "pub fn compose_inline_style(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorSwatch logic should include `{needle}` for primitive composition."
        );
    }

    for needle in [
        "pub struct ColorSwatchA11yOptions",
        "pub struct ColorSwatchA11yAttrs",
        "pub struct ColorSwatchA11yHandlers",
        "pub struct ColorSwatchA11yState",
        "pub struct ColorSwatchA11yContract",
        "pub fn use_color_swatch_a11y(",
        "let locale = locale_attrs(options.lang, options.dir);",
        "role: exposes_image_role.then_some(\"img\")",
    ] {
        assert!(
            headless_source.contains(needle),
            "ColorSwatch headless contract should include `{needle}` for A11y semantics."
        );
    }

    for needle in [
        "pub struct ColorSwatchMotion",
        "default_swatch_motion_tokens()",
        "ui_motion::spring::sanitize_config(motion.spring, default.spring)",
        "ui_motion::spring::SpringAnimator::new",
        "ui_motion::web::prefers_reduced_motion()",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "ColorSwatch motion contract should include `{needle}` for ui-motion mapping."
        );
    }

    let forbidden = "crate::illustrated_message::motion::";
    assert!(
        !motion_source.contains(forbidden),
        "ColorSwatch motion should not depend on other component motion modules: `{forbidden}`."
    );

    for needle in [
        "let render_state = logic::resolve_render_state(logic::ColorSwatchRenderInput {",
        "use ui_headless::{A11yDirection, ColorSwatchA11yOptions, use_color_swatch_a11y};",
        "let a11y = use_color_swatch_a11y(ColorSwatchA11yOptions {",
        "role=a11y.attrs.role",
        "aria-label=a11y.attrs.aria_label.clone()",
        "aria-hidden=a11y.attrs.aria_hidden",
        "lang=a11y.attrs.lang.clone()",
        "dir=a11y.attrs.dir",
        "class=render_state.class_name",
        "style=render_state.inline_style",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorSwatch view should derive state via logic helpers; missing `{needle}`."
        );
    }
    for forbidden in [
        "logic::sanitize_color_value(",
        "logic::resolve_alpha(",
        "logic::normalize_aria_label(",
        "logic::normalize_is_bordered(",
        "logic::normalize_is_decorative(",
        "logic::resolve_state(ColorSwatchStateInput {",
        "logic::compose_class_name(",
        "style=logic::resolve_inline_style(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ColorSwatch view should not re-run normalization branches directly: `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_emits_baseline_style_state_data_attributes() {
    let source = load_source("../../components/color-swatch/src/view.rs");

    for attr in [
        "const SLOT_COLOR_SWATCH: &str = \"color-swatch\";",
        "const SLOT_COLOR_SWATCH_CHECKER: &str = \"color-swatch-checker\";",
        "const SLOT_COLOR_SWATCH_SAMPLE: &str = \"color-swatch-sample\";",
        "const SLOT_COLOR_SWATCH_SLASH: &str = \"color-swatch-slash\";",
        "data-slot=SLOT_COLOR_SWATCH",
        "data-size=state.size_attr",
        "data-rounding=state.rounding_attr",
        "data-shape=state.shape_attr",
        "data-alpha=state.alpha_attr",
        "data-state=state.data_state_attr",
        "data-has-color=state.has_color.then_some(\"true\")",
        "data-bordered=state.is_bordered.then_some(\"true\")",
        "data-bordered-source=render_state.bordered_source.as_attr()",
        "data-decorative-source=render_state.decorative_source.as_attr()",
        "data-aria-source=state.aria_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "data-ui-schema=\"ui.color-swatch.agent-contract\"",
        "data-ui-schema-version=\"1\"",
        "data-ui-intent=\"color-preview\"",
        "data-ui-action=\"render\"",
        "data-ui-state=state.data_state_attr",
        "data-ui-source=state.aria_source_attr",
        "data-ui-stream-support=\"optional\"",
        "data-ui-stream-fallback=\"snapshot\"",
        "data-ui-output-status=\"verified\"",
        "data-slot=SLOT_COLOR_SWATCH_CHECKER",
        "data-slot=SLOT_COLOR_SWATCH_SAMPLE",
        "data-slot=SLOT_COLOR_SWATCH_SLASH",
    ] {
        assert!(
            source.contains(attr),
            "ColorSwatch should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn color_swatch_discrete_state_axes_use_type_constraints() {
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/color_swatch.rs");

    for needle in [
        "#[prop(optional)] size: ColorSwatchSize",
        "#[prop(optional)] rounding: ColorSwatchRounding",
        "#[prop(optional)] shape: ColorSwatchShape",
        "pub enum ColorSwatchSize",
        "pub enum ColorSwatchRounding",
        "pub enum ColorSwatchShape",
        "pub enum ColorSwatchAlpha",
        "pub size: ColorSwatchSize",
        "pub rounding: ColorSwatchRounding",
        "pub shape: ColorSwatchShape",
        "pub alpha: ColorSwatchAlpha",
    ] {
        let found = view_source.contains(needle)
            || logic_source.contains(needle)
            || primitive_source.contains(needle);
        assert!(
            found,
            "discrete ColorSwatch state axis should stay type-constrained via `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] size: Option<String>",
        "#[prop(optional, into)] rounding: Option<String>",
        "#[prop(optional, into)] shape: Option<String>",
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "discrete axes should not regress to stringly-typed inputs: `{forbidden}`."
        );
    }

    assert_eq!(
        view_source.matches("Option<bool>").count(),
        2,
        "ColorSwatch view should only expose two orthogonal bool props, not bool explosion."
    );
}

#[test]
fn color_swatch_machine_readable_state_contract_is_type_constrained_and_semantic() {
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/color_swatch.rs");
    let local_semantics = include_str!("../../../components/color-swatch/test/semantics.rs");
    let legacy_semantics =
        load_source("../../components/color-swatch/test/color_swatch_semantics.rs");

    for typed_axis in [
        "#[prop(optional)] size: ColorSwatchSize",
        "#[prop(optional)] rounding: ColorSwatchRounding",
        "#[prop(optional)] shape: ColorSwatchShape",
        "pub enum ColorSwatchSize",
        "pub enum ColorSwatchRounding",
        "pub enum ColorSwatchShape",
        "pub enum ColorSwatchAlpha",
        "pub struct ColorSwatchStateInput {",
    ] {
        assert!(
            view_source.contains(typed_axis)
                || logic_source.contains(typed_axis)
                || primitive_source.contains(typed_axis),
            "key state inputs should stay type-constrained via `{typed_axis}`."
        );
    }
    for forbidden in [
        "#[prop(optional, into)] size: Option<String>",
        "#[prop(optional, into)] rounding: Option<String>",
        "#[prop(optional, into)] shape: Option<String>",
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "stringly-typed state axis should stay forbidden: `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains("let state = resolve_state(ColorSwatchStateInput {"),
        "state normalization should be centralized in logic.rs."
    );
    assert!(
        !view_source.contains("resolve_state(ColorSwatchStateInput {"),
        "view.rs should consume normalized state and not rebuild state machine rules."
    );

    for marker in [
        "data-size=state.size_attr",
        "data-rounding=state.rounding_attr",
        "data-shape=state.shape_attr",
        "data-alpha=state.alpha_attr",
        "data-state=state.data_state_attr",
        "data-aria-source=state.aria_source_attr",
        "data-ui-state=state.data_state_attr",
        "data-ui-source=state.aria_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "machine-readable semantic marker should be exposed via `{marker}`."
        );
    }
    for finite_mapping in [
        "ColorSwatchSize::Xs => \"xs\"",
        "ColorSwatchSize::Sm => \"sm\"",
        "ColorSwatchSize::Md => \"md\"",
        "ColorSwatchSize::Lg => \"lg\"",
        "ColorSwatchAlpha::None => \"none\"",
        "ColorSwatchAlpha::Opaque => \"opaque\"",
        "ColorSwatchAlpha::Translucent => \"translucent\"",
        "ColorSwatchAlpha::Transparent => \"transparent\"",
    ] {
        assert!(
            primitive_source.contains(finite_mapping),
            "semantic marker values should remain enumerable via `{finite_mapping}`."
        );
    }

    for contract_test in [
        "fn color_swatch_discrete_state_axes_use_type_constraints()",
        "fn color_swatch_state_markers_are_observable_queryable_and_enumerable()",
        "fn color_swatch_state_primitives_source_boundary_is_enforced()",
        "fn color_swatch_default_value_resolution_stays_in_logic()",
    ] {
        assert!(
            local_semantics.contains(contract_test) && legacy_semantics.contains(contract_test),
            "compiler+tests feedback loop should keep semantic contract test `{contract_test}`."
        );
    }
}

#[test]
fn color_swatch_state_primitives_source_boundary_is_enforced() {
    let mod_source = load_source("../../components/color-swatch/src/mod.rs");
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/color_swatch.rs");

    for needle in [
        "pub use ui_state_primitives::color_swatch::{",
        "ColorSwatchStateInput",
        "normalize_is_bordered",
        "normalize_is_decorative",
        "resolve_state",
        "let state = resolve_state(ColorSwatchStateInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should consume primitive capability from ui-state-primitives via `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_is_bordered(",
        "pub fn normalize_is_decorative(",
        "pub fn resolve_state(",
        "pub struct ColorSwatchStateInput {",
        "pub struct ColorSwatchState {",
    ] {
        assert!(
            primitive_source.contains(needle),
            "state primitive definitions should stay in ui-state-primitives: `{needle}`."
        );
    }

    for forbidden in [
        "pub fn normalize_is_bordered(",
        "pub fn normalize_is_decorative(",
        "pub fn resolve_state(",
        "pub struct ColorSwatchStateInput {",
        "pub struct ColorSwatchState {",
        "store",
        "Store",
        "use_store",
        "global_state",
        "redux",
        "zustand",
        "pinia",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !mod_source.contains(forbidden),
            "component layer should not re-implement primitive state machines or bind business store `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_has_no_async_interaction_protocol() {
    let mod_source = load_source("../../components/color-swatch/src/mod.rs");
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let readme_source = load_source("../../components/color-swatch/src/README.md");

    for forbidden in [
        "#[prop(optional, into)] is_loading",
        "is_loading:",
        "on_retry",
        "retry:",
        "error:",
        "aria-busy",
        "data-loading",
        "data-error",
        "use_async_action",
        "spawn_local(",
        "tokio::spawn",
        "async fn",
        "Future<",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "ColorSwatch has no async interaction state axis; found forbidden token `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_styles_include_size_shape_and_alpha_markers() {
    let source = load_source("../../components/color-swatch/src/styles.rs");

    for selector in [
        "--ui-color-swatch-size: var(--ui-color-swatch-size-md, var(--ui-fallback-color-swatch-size-md));",
        "--ui-color-swatch-radius-default",
        "--ui-color-swatch-checker-size",
        "--ui-color-swatch-slash-width",
        "--ui-color-swatch-wide-multiplier",
        "--ui-color-swatch-border-width",
        "opacity: var(--ui-color-swatch-opacity, 1);",
        "transform: translateY(var(--ui-color-swatch-y, var(--ui-fallback-color-swatch-y)));",
        ".ui-color-swatch--size-xs",
        ".ui-color-swatch[data-size=\"lg\"]",
        ".ui-color-swatch--rounding-default",
        ".ui-color-swatch[data-rounding=\"full\"]",
        ".ui-color-swatch--shape-wide",
        ".ui-color-swatch[data-shape=\"square\"]",
        ".ui-color-swatch--bordered",
        ".ui-color-swatch[data-bordered=\"true\"]",
        ".ui-color-swatch--alpha-translucent .ui-color-swatch__checker",
        ".ui-color-swatch[data-alpha=\"transparent\"] .ui-color-swatch__sample",
        ".ui-color-swatch--alpha-none .ui-color-swatch__slash",
        ".ui-color-swatch--custom-class",
        ".ui-color-swatch[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorSwatch styles should include `{selector}` as stable state-marker contracts."
        );
    }

    for forbidden in ["0.875rem", "1rem", "1.25rem", "1.5rem", "999px"] {
        assert!(
            !source.contains(forbidden),
            "ColorSwatch styles should consume ui-theme tokens instead of hardcoded `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_token_first_styles_are_aggregated_and_uiroot_injected() {
    let mod_source = load_source("../../components/color-swatch/src/mod.rs");
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let styles_source = load_source("../../components/color-swatch/src/styles.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let css_source = load_source("../../crates/ui/src/css.rs");
    let root_source = load_source("../../crates/ui/src/root.rs");

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "styles.rs should be the static token-first style source of truth."
    );
    for declaration in styles_source
        .lines()
        .map(str::trim_start)
        .filter(|line| line.starts_with("--"))
    {
        assert!(
            declaration.starts_with("--ui-"),
            "custom property declarations should stay in `--ui-*` namespace: `{declaration}`."
        );
    }
    for required_token_usage in [
        "var(--ui-color-swatch-size-",
        "--ui-color-swatch-radius-default",
        "var(--ui-color-swatch-checker-size",
        "var(--ui-color-swatch-slash-width",
        "var(--ui-color-swatch-wide-multiplier",
        "var(--ui-color-swatch-border-width",
        "var(--ui-bg-muted)",
        "var(--ui-fg)",
        "var(--ui-danger)",
    ] {
        assert!(
            styles_source.contains(required_token_usage),
            "styles.rs should consume ui-theme variables via `{required_token_usage}`."
        );
    }
    for forbidden_private_token in ["var(--color-swatch-", "var(--swatch-"] {
        assert!(
            !styles_source.contains(forbidden_private_token),
            "styles.rs should not define private token namespaces like `{forbidden_private_token}`."
        );
    }

    assert!(
        css_source.contains("#[cfg(feature = \"component-color_swatch\")]")
            && css_source.contains("out.push_str(crate::color::swatch::styles::CSS);"),
        "ui css aggregator should register color-swatch styles behind component feature gate."
    );
    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should inject component CSS through centralized style output `{needle}`."
        );
    }

    assert!(
        view_source.contains("style=render_state.inline_style")
            && logic_source
                .contains("color.map(|color| format!(\"--ui-color-swatch-color: {color};\"))"),
        "runtime styling should only pass required CSS custom properties."
    );
    for forbidden_inline_style in [
        "style=\"background:",
        "style=\"color:",
        "style=\"top:",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
    ] {
        assert!(
            !view_source.contains(forbidden_inline_style),
            "view.rs should not carry business style declarations `{forbidden_inline_style}`."
        );
    }

    for forbidden in [
        "tailwind",
        "utility-first",
        "class=\"flex",
        "class=\"grid",
        "class=\"p-",
        "class=\"m-",
        "class=\"text-",
        "class=\"bg-",
        "stylist::",
        "Style::new(",
        "css!(",
        "styled_components",
        "emotion::",
        "stylex",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "component layer should not depend on utility-first/CSS-in-Rust default contract token `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_visual_desire_gate_reuses_theme_visual_baseline_and_heroui_contracts() {
    let color_swatch_docs =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_page =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let e2e_spec = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_doc = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "title=\"ColorSwatch\"",
        "slug=\"color-swatch\"",
        "title=\"Hello World (Default Path)\"",
        "title=\"Comparison Matrix (Size / Alpha / Shape / Empty)\"",
    ] {
        assert!(
            color_swatch_docs.contains(needle),
            "ColorSwatch docs should keep baseline visual entry `{needle}`."
        );
    }

    for needle in ["\"ThemeVisualBaseline\"", "\"theme-visual-baseline\""] {
        assert!(
            pages_registry.contains(needle),
            "docs pages registry should expose theme visual baseline entry `{needle}`."
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "hover/active/focus",
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
            "ThemeVisualBaseline page should keep visual-quality contract token `{needle}`."
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
            "theme visual baseline e2e contract should include `{needle}`."
        );
    }

    for needle in ["# HeroUI 参数设计风格对齐策略", "HeroUI 对齐结论"] {
        assert!(
            heroui_doc.contains(needle),
            "HeroUI strategy doc should keep visual-alignment contract token `{needle}`."
        );
    }
}

#[test]
fn color_swatch_tree_shaking_contract_enforces_component_feature_gates_and_budgeted_ci() {
    let check_source = load_source("../../components/color-swatch/check2.md");
    let cargo_source = load_source("../../crates/ui/Cargo.toml");
    let lib_source = load_source("../../crates/ui/src/lib.rs");
    let css_source = load_source("../../crates/ui/src/css.rs");
    let tree_shaking_script = load_source("../../scripts/check-ui-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "web-demo-components = [",
        "all-components = [",
        "\"component-color_swatch\"",
        "component-color_swatch = [\"component-illustrated_message\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui feature graph should keep ColorSwatch tree-shaking token `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-color_swatch\")]\n#[path = \"../../../components/color-swatch/src/mod.rs\"]\npub mod color_swatch;",
        "#[cfg(feature = \"component-color_swatch\")]\n    pub use crate::color_swatch as swatch;",
        "pub use color::swatch::{",
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]",
        "#[cfg(feature = \"all-components\")]",
    ] {
        assert!(
            lib_source.contains(needle),
            "lib.rs should keep feature-bounded ColorSwatch export token `{needle}`.",
        );
    }

    assert!(
        css_source.contains(
            "#[cfg(feature = \"component-color_swatch\")]\n    out.push_str(crate::color::swatch::styles::CSS);"
        ),
        "css.rs should aggregate ColorSwatch CSS behind component-color_swatch gate."
    );
    assert_eq!(
        css_source
            .matches("out.push_str(crate::color::swatch::styles::CSS);")
            .count(),
        1,
        "ColorSwatch CSS should have a single feature-gated aggregation site."
    );

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "cargo tree -e features -i ui -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_shaking_script.contains(needle) || budget_source.contains(needle),
            "tree-shaking CI gate should include `{needle}`.",
        );
    }

    for needle in [
        "- [x] Tree Shaking 是一等能力：",
        "cargo tree -e features -p ui --no-default-features --features component-accordion,inject-css",
        "cargo tree -e features -i ui -p web-demo",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-accordion,inject-css",
    ] {
        assert!(
            check_source.contains(needle),
            "ColorSwatch check2 tree-shaking checklist should keep token `{needle}`."
        );
    }
}

#[test]
fn color_swatch_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn color_swatch() -> AnyView",
        "title=\"ColorSwatch\"",
        "slug=\"color-swatch\"",
        "title=\"Hello World (Default Path)\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "title=\"Comparison Matrix (Size / Alpha / Shape / Empty)\"",
        "title=\"Rounded Large + Custom Label/Class\"",
    ] {
        assert!(
            source.contains(needle),
            "color-swatch docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn color_swatch_dx_paradox_keeps_default_path_simple() {
    let readme_source = load_source("../../components/color-swatch/src/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "## Hello World（最小可用）",
        "<ColorSwatch color=\"#2663eb\".to_string() />",
    ] {
        assert!(
            readme_source.contains(needle),
            "README should provide minimal copy-paste path via `{needle}`."
        );
    }

    for forbidden in ["ui-state-primitives", "ui_headless", "state="] {
        assert!(
            !readme_source.contains(forbidden),
            "README minimal path should not require internal wiring token `{forbidden}`."
        );
    }

    for needle in [
        "title=\"Hello World (Default Path)\"",
        "r##\"<ColorSwatch color=\"#2663eb\".to_string() />\"##.to_string()",
        "<ColorSwatch color=\"#2663eb\".to_string() />",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app should expose default minimal path via `{needle}`."
        );
    }
}

#[test]
fn color_swatch_non_composite_api_stays_explicit_and_simple() {
    let mod_source = load_source("../../components/color-swatch/src/mod.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let readme_source = load_source("../../components/color-swatch/src/README.md");

    assert!(
        view_source.contains("pub fn ColorSwatch(")
            && readme_source.contains("<ColorSwatch color=\"#2663eb\".to_string() />"),
        "ColorSwatch should remain a leaf component with direct explicit usage."
    );

    for forbidden in [
        "#[prop(optional, into)] labels",
        "#[prop(optional, into)] titles",
        "#[prop(optional, into)] panels",
        "#[prop(optional, into)] children",
        "#[prop(optional, into)] items",
        "labels + children",
        "titles + panels",
        "ItemSpec",
        "item_specs",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "ColorSwatch is not a composite Parent/Item API and should reject `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_has_no_drag_macro_micro_state_machine() {
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");
    let readme_source = load_source("../../components/color-swatch/src/README.md");

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "on_drag",
        "on_drag_end",
        "is_dragging",
        "pointermove",
        "mousemove",
        "touchmove",
        "drag_loop",
        "request_animation_frame",
        "raf",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "ColorSwatch has no drag macro/micro state machine; found forbidden token `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_has_no_two_pass_geometry_rendering_pipeline() {
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");
    let readme_source = load_source("../../components/color-swatch/src/README.md");

    for forbidden in [
        "Intent -> Measure(view) -> Rectification(logic)",
        "Rectification",
        "geometry",
        "measure(",
        "get_bounding_client_rect",
        "BoundingClientRect",
        "client_width",
        "client_height",
        "offset_width",
        "offset_height",
        "ResizeObserver",
        "Tooltip",
        "Popover",
        "Menu",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "ColorSwatch is not a geometry-dependent overlay component; found forbidden token `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_has_no_overlay_focus_stack_restore_contract() {
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");
    let readme_source = load_source("../../components/color-swatch/src/README.md");

    assert!(
        view_source.contains("let root_ref: NodeRef<html::Div> = NodeRef::new();")
            && view_source.contains("motion::attach_motion(root_ref, motion);"),
        "ColorSwatch NodeRef should stay motion-only, not focus-stack recovery state."
    );

    for forbidden in [
        "Overlay",
        "overlay",
        "focus stack",
        "Focus Stack",
        "focus manager",
        "use_focus_trap",
        "use_overlay_stack",
        "RestorePolicy",
        "FallbackTo",
        "Selector",
        "focus_manager",
        "restore_focus",
        "document.body",
        "on_close",
        "on_exit_complete",
        "aria_labelledby",
        "aria_describedby",
        "focus()",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "ColorSwatch is not a layered overlay and should not carry focus-stack contract token `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_has_no_registration_protocol_for_dynamic_collections() {
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let mod_source = load_source("../../components/color-swatch/src/mod.rs");
    let readme_source = load_source("../../components/color-swatch/src/README.md");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "register_item",
        "unregister_item",
        "items_order",
        "HashSet",
        "Accordion",
        "Tabs",
        "Menu",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "ColorSwatch is not a dynamic collection container; found forbidden registration token `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_has_no_slot_projection_strategy_contract() {
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let mod_source = load_source("../../components/color-swatch/src/mod.rs");
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");
    let readme_source = load_source("../../components/color-swatch/src/README.md");

    for forbidden in [
        "Slot Projection",
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "projection_mode",
        "lazy_mount",
        "keep_alive",
        "eager_mount",
        "pause_polling",
        "resume_polling",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "ColorSwatch is not a container with slot projection lifecycle; found forbidden token `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_has_no_environment_stream_subscription_pipeline() {
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let mod_source = load_source("../../components/color-swatch/src/mod.rs");
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");
    let readme_source = load_source("../../components/color-swatch/src/README.md");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "Action::BreakpointChanged",
        "ThemeChanged",
        "Action::ThemeChanged",
        "Action::IntersectionChanged",
        "on_resize",
        "on_intersection",
        "debounce(",
        "throttle(",
        "match_media",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "ColorSwatch has no Env Stream pipeline; found forbidden token `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_has_no_event_light_cone_for_large_collections() {
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let mod_source = load_source("../../components/color-swatch/src/mod.rs");
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");
    let readme_source = load_source("../../components/color-swatch/src/README.md");

    for forbidden in [
        "Event Light Cone",
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "selection_state",
        "batch_select",
        "select_all",
        "prop drilling",
        "Table",
        "Grid",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "ColorSwatch has no large-collection event-light-cone contract; found forbidden token `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_has_no_causality_bus_trace_chain_contract() {
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let mod_source = load_source("../../components/color-swatch/src/mod.rs");
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");
    let readme_source = load_source("../../components/color-swatch/src/README.md");

    for forbidden in [
        "Causality Bus",
        "TraceId",
        "trace_id",
        "dispatch",
        "broadcast",
        "subscriber",
        "event_bus",
        "command_bus",
        "derived_command",
        "causal_chain",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "ColorSwatch has no causality-bus trace-chain contract; found forbidden token `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_has_no_foreign_zone_escape_hatch_contract() {
    let lib_source = load_source("../../components/color-swatch/src/lib.rs");
    let mod_source = load_source("../../components/color-swatch/src/mod.rs");
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");
    let readme_source = load_source("../../components/color-swatch/src/README.md");
    let check_source = load_source("../../components/color-swatch/check2.md");

    for forbidden in [
        "ECharts",
        "echarts",
        "mapbox",
        "leaflet",
        "google.maps",
        "ol.Map",
        "Foreign Zone",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "third_party_instance",
        "foreign_instance",
        "on_foreign_ready",
    ] {
        assert!(
            !lib_source.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "ColorSwatch should not integrate imperative third-party instances; found `{forbidden}`."
        );
    }

    assert!(
        check_source.contains("- [x] 受控外交特区（Escape Hatches）："),
        "check2 should mark Escape Hatches as completed with explicit N/A rationale."
    );
}

#[test]
fn color_swatch_has_no_hydration_discontinuity_id_generation_contract() {
    let lib_source = load_source("../../components/color-swatch/src/lib.rs");
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");
    let check_source = load_source("../../components/color-swatch/check2.md");

    for forbidden in [
        "SystemTime::now",
        "UNIX_EPOCH",
        "Instant::now",
        "Date::now",
        "performance.now",
        "rand::",
        "thread_rng",
        "Uuid",
        "new_v4",
        "uuid::",
        "use_ui_id_provider(",
        "provide_ui_id_provider(",
    ] {
        assert!(
            !lib_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "ColorSwatch should not introduce non-deterministic hydration init token `{forbidden}`."
        );
    }

    for forbidden in ["id=", "aria-labelledby=", "aria-describedby="] {
        assert!(
            !view_source.contains(forbidden),
            "ColorSwatch should not emit runtime-generated id linkage `{forbidden}`."
        );
    }

    assert!(
        check_source.contains("- [x] SSR 时空断裂治理（Hydration Discontinuity）："),
        "check2 should mark hydration discontinuity contract as completed with explicit N/A rationale."
    );
}

#[test]
fn color_swatch_platform_contract_uses_cfg_and_non_wasm_browser_free_path() {
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");
    let check_source = load_source("../../components/color-swatch/check2.md");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should expose explicit platform split token `{needle}`."
        );
    }

    let wasm_section = motion_source
        .split("#[cfg(target_arch = \"wasm32\")]")
        .nth(1)
        .unwrap_or_default()
        .split("#[cfg(not(target_arch = \"wasm32\"))]")
        .next()
        .unwrap_or_default();
    let non_wasm_section = motion_source
        .split("#[cfg(not(target_arch = \"wasm32\"))]")
        .nth(1)
        .unwrap_or_default();

    for wasm_only in [
        "use leptos::wasm_bindgen::JsCast;",
        "leptos::web_sys::HtmlElement",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            wasm_section.contains(wasm_only),
            "wasm branch should contain browser/runtime binding `{wasm_only}`."
        );
    }

    for forbidden in ["web_sys::", "wasm_bindgen", "JsCast", "window", "document"] {
        assert!(
            !non_wasm_section.contains(forbidden),
            "non-wasm branch must stay browser-free; found `{forbidden}`."
        );
    }

    assert!(
        check_source.contains("- [x] SSR 与跨平台检查："),
        "check2 should mark SSR/platform compile contract as completed with evidence."
    );
}

#[test]
fn color_swatch_preserves_ui_headless_web_ssr_compile_error_contract() {
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let headless_cargo_source = load_source("../../crates/ui-headless/Cargo.toml");
    let ui_components_cargo_source = load_source("../../crates/ui/Cargo.toml");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let check_source = load_source("../../components/color-swatch/check2.md");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless mutual exclusion guard should include `{needle}`."
        );
    }

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            headless_cargo_source.contains(needle),
            "ui-headless feature definitions should include `{needle}`."
        );
    }

    assert!(
        view_source.contains(
            "use ui_headless::{A11yDirection, ColorSwatchA11yOptions, use_color_swatch_a11y};"
        ),
        "ColorSwatch should consume ui-headless contracts through typed API imports."
    );

    assert!(
        ui_components_cargo_source.contains("ui-headless = { path = \"../ui-headless\" }"),
        "ui should depend on ui-headless without forcing conflicting web/ssr flags."
    );
    for forbidden in [
        "ui-headless/web",
        "ui-headless/ssr",
        "features = [\"web\", \"ssr\"]",
    ] {
        assert!(
            !ui_components_cargo_source.contains(forbidden),
            "ui should not hardwire conflicting ui-headless feature token `{forbidden}`."
        );
    }

    assert!(
        check_source.contains("- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护"),
        "check2 should mark ui-headless web/ssr mutual exclusion contract as completed."
    );
}

#[test]
fn color_swatch_motion_contract_preserves_non_wasm_ui_motion_noop_backend() {
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let check_source = load_source("../../components/color-swatch/check2.md");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]\npub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion should provide deterministic non-wasm noop token `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new",
        "ui_motion::web::prefers_reduced_motion()",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "ColorSwatch motion contract should contain `{needle}`."
        );
    }

    assert!(
        view_source.contains("motion::attach_motion(root_ref, motion);"),
        "view.rs should attach motion through contract without assuming backend animation handles."
    );

    for forbidden in ["unwrap()", "expect(", "panic!("] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs downgrade path should avoid hard failure token `{forbidden}`."
        );
    }

    assert!(
        check_source.contains("- [x] `ui-motion` 非 wasm 提供 no-op/stub"),
        "check2 should mark ui-motion non-wasm noop contract as completed."
    );
}

#[test]
fn color_swatch_covers_reduced_motion_ssr_wasm_without_semantic_split() {
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let check_source = load_source("../../components/color-swatch/check2.md");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "let initial_progress = if motion.enabled && !ui_motion::web::prefers_reduced_motion() {",
        "spring.set_target(1.0);",
        "spring.set_target(initial_progress);",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should cover reduced-motion + wasm/non-wasm branch token `{needle}`."
        );
    }

    for needle in [
        "role=a11y.attrs.role",
        "aria-label=a11y.attrs.aria_label.clone()",
        "aria-hidden=a11y.attrs.aria_hidden",
        "data-state=state.data_state_attr",
        "data-ui-state=state.data_state_attr",
        "data-ui-source=state.aria_source_attr",
        "data-ui-schema=\"ui.color-swatch.agent-contract\"",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep platform-neutral semantic marker `{needle}`."
        );
    }

    for forbidden in [
        "#[cfg(",
        "prefers_reduced_motion()",
        "web_sys::",
        "wasm_bindgen",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs semantics should not split by runtime/platform token `{forbidden}`."
        );
    }

    assert!(
        check_source.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。"),
        "check2 should mark reduced-motion/SSR/wasm coverage contract as completed."
    );
}

#[test]
fn color_swatch_a11y_i18n_l10n_contract_is_headless_driven() {
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let headless_source = load_source("../../crates/ui-headless/src/color_swatch.rs");
    let shared_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "use ui_headless::{A11yDirection, ColorSwatchA11yOptions, use_color_swatch_a11y};",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "let a11y = use_color_swatch_a11y(ColorSwatchA11yOptions {",
        "role=a11y.attrs.role",
        "aria-label=a11y.attrs.aria_label.clone()",
        "aria-hidden=a11y.attrs.aria_hidden",
        "lang=a11y.attrs.lang.clone()",
        "dir=a11y.attrs.dir",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should mount headless A11y + locale contract via `{needle}`."
        );
    }

    assert!(
        logic_source.contains(
            "normalize_aria_label(input.aria_label, input.color_name, color.as_deref(), alpha);"
        ),
        "logic.rs should centralize aria-label source resolution instead of hardcoding copy in view.rs."
    );

    for needle in [
        "use crate::a11y::{A11yDirection, locale_attrs};",
        "let locale = locale_attrs(options.lang, options.dir);",
        "role: exposes_image_role.then_some(\"img\")",
        "aria_hidden: options.is_decorative.then_some(\"true\")",
    ] {
        assert!(
            headless_source.contains(needle),
            "headless color_swatch contract should include `{needle}`."
        );
    }

    assert!(
        shared_a11y_source.contains("pub fn locale_attrs("),
        "shared locale bridge should come from crates/ui-headless/src/a11y.rs."
    );

    for forbidden in ["role=\"img\"", "aria-label=\"", "on:keydown", "tabindex="] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not hardcode interaction copy/keyboard contract token `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_state_markers_are_observable_queryable_and_enumerable() {
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let styles_source = load_source("../../components/color-swatch/src/styles.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/color_swatch.rs");

    for marker in [
        "role=a11y.attrs.role",
        "aria-label=a11y.attrs.aria_label.clone()",
        "aria-hidden=a11y.attrs.aria_hidden",
        "data-size=state.size_attr",
        "data-rounding=state.rounding_attr",
        "data-shape=state.shape_attr",
        "data-alpha=state.alpha_attr",
        "data-state=state.data_state_attr",
        "data-bordered=state.is_bordered.then_some(\"true\")",
        "data-bordered-source=render_state.bordered_source.as_attr()",
        "data-decorative-source=render_state.decorative_source.as_attr()",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "view.rs should expose stable semantic marker `{marker}` for automation and audits."
        );
    }

    for selector in [
        "[data-size=\"xs\"]",
        "[data-size=\"sm\"]",
        "[data-size=\"md\"]",
        "[data-size=\"lg\"]",
        "[data-rounding=\"default\"]",
        "[data-rounding=\"none\"]",
        "[data-rounding=\"full\"]",
        "[data-shape=\"square\"]",
        "[data-shape=\"wide\"]",
        "[data-alpha=\"opaque\"]",
        "[data-alpha=\"translucent\"]",
        "[data-alpha=\"transparent\"]",
        "[data-alpha=\"none\"]",
        "[data-bordered=\"true\"]",
        "[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "styles.rs should consume explicit semantic selector `{selector}`."
        );
    }
    assert!(
        !styles_source.contains(":nth-child"),
        "styles.rs should not rely on DOM-order guessing selectors."
    );

    for finite_mapping in [
        "ColorSwatchSize::Xs => \"xs\"",
        "ColorSwatchSize::Sm => \"sm\"",
        "ColorSwatchSize::Md => \"md\"",
        "ColorSwatchSize::Lg => \"lg\"",
        "ColorSwatchRounding::Default => \"default\"",
        "ColorSwatchRounding::None => \"none\"",
        "ColorSwatchRounding::Full => \"full\"",
        "ColorSwatchShape::Square => \"square\"",
        "ColorSwatchShape::Wide => \"wide\"",
        "ColorSwatchAlpha::None => \"none\"",
        "ColorSwatchAlpha::Opaque => \"opaque\"",
        "ColorSwatchAlpha::Translucent => \"translucent\"",
        "ColorSwatchAlpha::Transparent => \"transparent\"",
        "Self::IsProp => \"is-prop\"",
        "Self::Default => \"default\"",
        "ColorSwatchAlpha::None => \"empty\"",
        "ColorSwatchAlpha::Transparent => \"transparent\"",
        "ColorSwatchAlpha::Translucent => \"translucent\"",
        "ColorSwatchAlpha::Opaque if input.bordered => \"framed\"",
        "ColorSwatchAlpha::Opaque => \"default\"",
        "aria_source_attr: if input.has_custom_aria_label {",
        "class_source_attr: if input.has_custom_class_name {",
    ] {
        assert!(
            primitive_source.contains(finite_mapping),
            "state marker values should be finite and traceable in primitives via `{finite_mapping}`."
        );
    }
}

#[test]
fn color_swatch_styles_depend_on_explicit_state_markers_only() {
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let styles_source = load_source("../../components/color-swatch/src/styles.rs");

    for selector in [
        ".ui-color-swatch[data-size=\"xs\"]",
        ".ui-color-swatch[data-size=\"sm\"]",
        ".ui-color-swatch[data-size=\"md\"]",
        ".ui-color-swatch[data-size=\"lg\"]",
        ".ui-color-swatch[data-rounding=\"default\"]",
        ".ui-color-swatch[data-rounding=\"none\"]",
        ".ui-color-swatch[data-rounding=\"full\"]",
        ".ui-color-swatch[data-shape=\"square\"]",
        ".ui-color-swatch[data-shape=\"wide\"]",
        ".ui-color-swatch[data-bordered=\"true\"]",
        ".ui-color-swatch[data-alpha=\"opaque\"] .ui-color-swatch__sample",
        ".ui-color-swatch[data-alpha=\"transparent\"] .ui-color-swatch__sample",
        ".ui-color-swatch[data-alpha=\"none\"] .ui-color-swatch__slash",
    ] {
        assert!(
            styles_source.contains(selector),
            "styles.rs should express visual state via explicit selector `{selector}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":first-child", ":last-child"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not guess state from fragile DOM structure `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("style=render_state.inline_style"),
        "view.rs should consume normalized runtime style payload from logic.rs."
    );
    assert!(
        logic_source.contains("color.map(|color| format!(\"--ui-color-swatch-color: {color};\"))"),
        "runtime style should only pass necessary CSS custom properties."
    );
    for forbidden in [
        "style=\"background:",
        "style=\"color:",
        "style=\"top:",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not carry business style logic in inline declarations `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_tests_prioritize_semantic_contracts_over_visual_snapshots() {
    let local_semantics = include_str!("../../../components/color-swatch/test/semantics.rs");
    let legacy_semantics = include_str!("color_swatch_semantics.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");

    for required_semantic_contract in [
        "fn color_swatch_state_markers_are_observable_queryable_and_enumerable()",
        "fn color_swatch_a11y_i18n_l10n_contract_is_headless_driven()",
        "fn color_swatch_has_no_controlled_uncontrolled_state_axis()",
        "fn color_swatch_has_no_async_interaction_protocol()",
    ] {
        assert!(
            local_semantics.contains(required_semantic_contract)
                && legacy_semantics.contains(required_semantic_contract),
            "semantic contract coverage should include `{required_semantic_contract}` in both local and aggregated suites."
        );
    }

    for required_marker in [
        "role=a11y.attrs.role",
        "aria-label=a11y.attrs.aria_label.clone()",
        "data-state=state.data_state_attr",
        "data-aria-source=state.aria_source_attr",
    ] {
        assert!(
            local_semantics.contains(required_marker),
            "semantic tests should assert key marker `{required_marker}`."
        );
    }

    let snapshot_token = ["assert", "_snapshot!"].concat();
    let debug_snapshot_token = ["assert_debug", "_snapshot!"].concat();
    let json_snapshot_token = ["assert_json", "_snapshot!"].concat();
    let to_match_snap_token = ["to_match", "_snapshot"].concat();
    let snapshot_ns_token = ["snapshot", "::"].concat();
    for snapshot_only_pattern in [
        snapshot_token.as_str(),
        debug_snapshot_token.as_str(),
        json_snapshot_token.as_str(),
        to_match_snap_token.as_str(),
        snapshot_ns_token.as_str(),
    ] {
        assert!(
            !local_semantics.contains(snapshot_only_pattern)
                && !legacy_semantics.contains(snapshot_only_pattern),
            "snapshot-only assertion `{snapshot_only_pattern}` should not replace semantic-contract tests."
        );
    }

    for n_a_branch in ["is_disabled", "on:keydown", "on:pointerdown", "on:click"] {
        assert!(
            !view_source.contains(n_a_branch),
            "ColorSwatch has no interactive `{n_a_branch}` branch; matrix should treat it as N/A."
        );
    }

    for wasm_ssr_marker in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(wasm_ssr_marker),
            "SSR/wasm semantic-difference path should be explicitly guarded via `{wasm_ssr_marker}`."
        );
    }
}

#[test]
fn color_swatch_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only() {
    let local_semantics = include_str!("../../../components/color-swatch/test/semantics.rs");
    let legacy_semantics = include_str!("color_swatch_semantics.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");
    let check2_source = load_source("../../components/color-swatch/check2.md");

    for required_semantic_contract in [
        "fn color_swatch_state_markers_are_observable_queryable_and_enumerable()",
        "fn color_swatch_a11y_i18n_l10n_contract_is_headless_driven()",
        "fn color_swatch_tests_prioritize_semantic_contracts_over_visual_snapshots()",
    ] {
        assert!(
            local_semantics.contains(required_semantic_contract)
                && legacy_semantics.contains(required_semantic_contract),
            "semantic-priority contract should keep `{required_semantic_contract}` in local + aggregated suites."
        );
    }

    for required_marker in [
        "role=a11y.attrs.role",
        "aria-label=a11y.attrs.aria_label.clone()",
        "data-state=state.data_state_attr",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            view_source.contains(required_marker) && local_semantics.contains(required_marker),
            "semantic-priority contract should assert marker `{required_marker}` in view + tests."
        );
    }

    let snapshot_token = ["assert", "_snapshot!"].concat();
    let debug_snapshot_token = ["assert_debug", "_snapshot!"].concat();
    let json_snapshot_token = ["assert_json", "_snapshot!"].concat();
    let to_match_snap_token = ["to_match", "_snapshot"].concat();
    let snapshot_ns_token = ["snapshot", "::"].concat();
    for snapshot_only_pattern in [
        snapshot_token.as_str(),
        debug_snapshot_token.as_str(),
        json_snapshot_token.as_str(),
        to_match_snap_token.as_str(),
        snapshot_ns_token.as_str(),
    ] {
        assert!(
            !local_semantics.contains(snapshot_only_pattern)
                && !legacy_semantics.contains(snapshot_only_pattern),
            "snapshot-only assertion `{snapshot_only_pattern}` should not replace semantic-contract tests."
        );
    }

    let script_needle = "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate should include `{script_needle}`."
    );

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "color_swatch_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 semantics-priority evidence should include `{required}`."
        );
    }
}

#[test]
fn color_swatch_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let check2_source = load_source("../../components/color-swatch/check2.md");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let styles_source = load_source("../../components/color-swatch/src/styles.rs");

    for needle in [
        "component_doc!(",
        "\"ColorSwatch\"",
        "\"color-swatch\"",
        "display_extra::color_swatch",
    ] {
        assert!(
            pages_source.contains(needle),
            "ColorSwatch docs page should stay in component coverage traversal via `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn color_swatch() -> AnyView",
        "title=\"ColorSwatch\"",
        "slug=\"color-swatch\"",
        "<ComponentPage",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "ColorSwatch docs page should mount through ComponentPage contract `{needle}`."
        );
    }

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "max_update_ms: Some(10.0),",
        "\"color-swatch\" => UiPerfBudget {",
        "max_mount_ms: 22.0,",
        "max_update_ms: Some(6.0),",
        "max_heap_kb: Some(320.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "component shell should keep performance budget token `{needle}`."
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
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose performance regression marker `{needle}`."
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "docs coverage e2e should enforce perf regression guard `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance follow-up plan should keep `{needle}`."
        );
    }

    for needle in [
        "data-state=state.data_state_attr",
        "data-alpha=state.alpha_attr",
        "data-aria-source=state.aria_source_attr",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
        "data-ui-schema=\"ui.color-swatch.agent-contract\"",
        "transform: translateY(var(--ui-color-swatch-y, var(--ui-fallback-color-swatch-y)));",
        "opacity: var(--ui-color-swatch-opacity, 1);",
        "will-change: transform, opacity;",
    ] {
        assert!(
            view_source.contains(needle) || styles_source.contains(needle),
            "ColorSwatch should expose performance attribution marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_performance_governance_contract_is_budgeted_traceable_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`."
    );
    assert!(
        script_source.contains(
            "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
        ),
        "performance gate script should keep render_count follow-up blocker."
    );

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "渲染次数预算为 `1`",
        "render_count",
        "等价证据",
        "color_swatch_performance_governance_contract_is_budgeted_traceable_and_blocking",
    ] {
        assert!(
            check2_source.contains(needle),
            "ColorSwatch check2 should include performance governance evidence token `{needle}`."
        );
    }
}

#[test]
fn color_swatch_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let local_semantics = include_str!("../../../components/color-swatch/test/semantics.rs");
    let legacy_semantics = load_source("legacy_semantics");
    let check2_source = load_source("../../components/color-swatch/check2.md");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-performance.sh");

    for required_test in [
        "fn color_swatch_tests_prioritize_semantic_contracts_over_visual_snapshots()",
        "fn color_swatch_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn color_swatch_has_no_overlay_focus_stack_restore_contract()",
        "fn color_swatch_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            local_semantics.contains(required_test) && legacy_semantics.contains(required_test),
            "semantic/performance regression suite should include `{required_test}` in local and aggregated tests."
        );
    }

    for marker in [
        "role=a11y.attrs.role",
        "aria-label=a11y.attrs.aria_label.clone()",
        "data-state=state.data_state_attr",
        "data-aria-source=state.aria_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "ColorSwatch view should expose semantic marker `{marker}`."
        );
    }

    for non_focus_branch in ["on:focus=", "on:keydown", "on:pointerdown", "on:click"] {
        assert!(
            !view_source.contains(non_focus_branch),
            "ColorSwatch is a non-interactive leaf; focus-flow branch `{non_focus_branch}` must stay N/A."
        );
    }

    for required in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(required),
            "render_count tracking contract should keep `{required}`."
        );
    }

    for script_needle in [
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(script_needle),
            "performance check script should include `{script_needle}`."
        );
    }

    for check_needle in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "color_swatch_tests_prioritize_semantic_contracts_over_visual_snapshots",
        "color_swatch_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "color_swatch_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "render_count",
        "mount-only 等价证据",
    ] {
        assert!(
            check2_source.contains(check_needle),
            "ColorSwatch check2 semantic/performance entry should keep `{check_needle}`."
        );
    }
}

#[test]
fn color_swatch_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders() {
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let check2_source = load_source("../../components/color-swatch/check2.md");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");

    assert!(
        view_source.contains("view! {"),
        "ColorSwatch should keep explicit render block in view.rs."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        1,
        "ColorSwatch should keep one bounded `view!` block for this simple leaf component."
    );
    assert!(
        view_source.lines().count() <= 120,
        "ColorSwatch view.rs should stay compact; split semantic subrenders if this grows significantly."
    );

    for needle in [
        "const SLOT_COLOR_SWATCH: &str = \"color-swatch\";",
        "const SLOT_COLOR_SWATCH_CHECKER: &str = \"color-swatch-checker\";",
        "const SLOT_COLOR_SWATCH_SAMPLE: &str = \"color-swatch-sample\";",
        "const SLOT_COLOR_SWATCH_SLASH: &str = \"color-swatch-slash\";",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorSwatch view should keep static slot constant marker `{needle}`."
        );
    }

    for needle in [
        "data-slot=SLOT_COLOR_SWATCH\n            data-size=state.size_attr",
        "data-slot=SLOT_COLOR_SWATCH_CHECKER",
        "data-slot=SLOT_COLOR_SWATCH_SAMPLE",
        "data-slot=SLOT_COLOR_SWATCH_SLASH",
    ] {
        assert_eq!(
            view_source.matches(needle).count(),
            1,
            "ColorSwatch semantic marker `{needle}` should stay singular to avoid repeated deep fragments."
        );
    }

    for forbidden in [
        "<header",
        "<section",
        "<article",
        "<footer",
        "<nav",
        "<ul",
        "<li",
        "for item in",
        ".map(|",
        "collect::<Vec<_>>()",
        "while let Some(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ColorSwatch view should avoid expansion-heavy nesting token `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "color_swatch_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders",
    ] {
        assert!(
            check2_source.contains(needle),
            "ColorSwatch check2 should include view-macro complexity evidence token `{needle}`."
        );
    }
}

#[test]
fn color_swatch_view_functional_split_prefers_no_extra_local_components_for_simple_layout() {
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let check2_source = load_source("../../components/color-swatch/check2.md");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "ColorSwatch should keep a single public component boundary for current simple indicator layout."
    );

    for forbidden in [
        "#[component]\nfn render_",
        "#[component]\nfn color_swatch_",
        "pub fn render_",
        "fn render_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ColorSwatch should not introduce extra local component/render API noise for simple layout `{forbidden}`."
        );
    }

    for needle in [
        "const SLOT_COLOR_SWATCH: &str = \"color-swatch\";",
        "const SLOT_COLOR_SWATCH_CHECKER: &str = \"color-swatch-checker\";",
        "const SLOT_COLOR_SWATCH_SAMPLE: &str = \"color-swatch-sample\";",
        "const SLOT_COLOR_SWATCH_SLASH: &str = \"color-swatch-slash\";",
        "data-slot=SLOT_COLOR_SWATCH",
        "data-slot=SLOT_COLOR_SWATCH_CHECKER",
        "data-slot=SLOT_COLOR_SWATCH_SAMPLE",
        "data-slot=SLOT_COLOR_SWATCH_SLASH",
        "data-state=state.data_state_attr",
        "data-aria-source=state.aria_source_attr",
        "data-ui-schema=\"ui.color-swatch.agent-contract\"",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorSwatch should keep stable semantic selector marker `{needle}` after function-split constraints."
        );
    }

    let script_needle = "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_view_functional_split_prefers_no_extra_local_components_for_simple_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "color_swatch_view_functional_split_prefers_no_extra_local_components_for_simple_layout",
    ] {
        assert!(
            check2_source.contains(needle),
            "ColorSwatch check2 should include functional-split evidence token `{needle}`."
        );
    }
}

#[test]
fn color_swatch_static_fragments_are_constantized_or_absent_for_simple_indicator_layout() {
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let check2_source = load_source("../../components/color-swatch/check2.md");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");

    for forbidden in [
        "inner_html=",
        "<header",
        "<section",
        "<article",
        "<footer",
        "<nav",
        "<ul",
        "<li",
        "<path",
        "let markdown",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ColorSwatch view should avoid heavy inline static fragments for simple indicator layout `{forbidden}`."
        );
    }

    for needle in [
        "const SLOT_COLOR_SWATCH: &str = \"color-swatch\";",
        "const SLOT_COLOR_SWATCH_CHECKER: &str = \"color-swatch-checker\";",
        "const SLOT_COLOR_SWATCH_SAMPLE: &str = \"color-swatch-sample\";",
        "const SLOT_COLOR_SWATCH_SLASH: &str = \"color-swatch-slash\";",
        "const CLASS_COLOR_SWATCH_CHECKER: &str = \"ui-color-swatch__checker\";",
        "const CLASS_COLOR_SWATCH_SAMPLE: &str = \"ui-color-swatch__sample\";",
        "const CLASS_COLOR_SWATCH_SLASH: &str = \"ui-color-swatch__slash\";",
        "const BOOL_TRUE: &str = \"true\";",
        "data-slot=SLOT_COLOR_SWATCH",
        "data-slot=SLOT_COLOR_SWATCH_CHECKER",
        "data-slot=SLOT_COLOR_SWATCH_SAMPLE",
        "data-slot=SLOT_COLOR_SWATCH_SLASH",
        "class=CLASS_COLOR_SWATCH_CHECKER",
        "class=CLASS_COLOR_SWATCH_SAMPLE",
        "class=CLASS_COLOR_SWATCH_SLASH",
        "aria-hidden=BOOL_TRUE",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorSwatch should keep static fragment constantization marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_static_fragments_are_constantized_or_absent_for_simple_indicator_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "color_swatch_static_fragments_are_constantized_or_absent_for_simple_indicator_layout",
    ] {
        assert!(
            check2_source.contains(needle),
            "ColorSwatch check2 should include static-fragment evidence token `{needle}`."
        );
    }
}

#[test]
fn color_swatch_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    for rel_path in [
        "../../components/color-swatch/src/mod.rs",
        "../../components/color-swatch/src/logic.rs",
        "../../components/color-swatch/src/styles.rs",
        "../../components/color-swatch/src/motion.rs",
        "../../components/color-swatch/src/view.rs",
        "../../apps/docs-app/src/pages/components/pages/display_extra.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "dangerouslySetInnerHTML",
        ] {
            assert!(
                !source.contains(forbidden),
                "ColorSwatch path `{rel_path}` must not inject raw html; found `{forbidden}`."
            );
        }
    }

    let check2_source = load_source("../../components/color-swatch/check2.md");
    for needle in [
        "`inner_html` 使用约束",
        "仅允许编译期常量或明确白名单内容进入 `inner_html`",
        "使用 `inner_html` 的节点必须补语义测试与安全回归说明",
    ] {
        assert!(
            check2_source.contains(needle),
            "ColorSwatch checklist should keep inner_html security contract marker `{needle}`."
        );
    }
}

#[test]
fn color_swatch_inner_html_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-inner-html.sh");

    let needle = "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(needle),
        "inner-html check script should enforce ColorSwatch contract marker `{needle}`."
    );
}

#[test]
fn color_swatch_wasm_debug_contract_is_explicitly_na_and_feature_isolated() {
    let cargo_source = load_source("../../crates/ui/Cargo.toml");
    let crate_root_source = load_source("../../crates/ui/src/lib.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");
    let check2_source = load_source("../../components/color-swatch/check2.md");

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui Cargo features should keep shared wasm-debug marker `{needle}`."
        );
    }
    assert!(
        !cargo_source.contains("color-swatch-wasm-debug")
            && !cargo_source.contains("color_swatch-wasm-debug"),
        "ColorSwatch should not expose a component-local wasm-debug feature for a non-interactive display primitive."
    );

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui root should keep wasm-debug isolation marker `{needle}`."
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(needle),
            "docs-app should keep debug-only wasm trace visual entry marker `{needle}`."
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "global debug overlay should keep marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "Inspect {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "ts_ms: now_ms(),",
    ] {
        assert!(
            trace_source.contains(needle),
            "global trace model should keep typed source/timestamp marker `{needle}`."
        );
    }

    for needle in [
        "data-size=state.size_attr",
        "data-rounding=state.rounding_attr",
        "data-shape=state.shape_attr",
        "data-alpha=state.alpha_attr",
        "data-state=state.data_state_attr",
        "data-bordered-source=render_state.bordered_source.as_attr()",
        "data-decorative-source=render_state.decorative_source.as_attr()",
        "data-aria-source=state.aria_source_attr",
        "data-class-source=state.class_source_attr",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
        "data-ui-state=state.data_state_attr",
        "data-ui-source=state.aria_source_attr",
        "data-ui-schema=\"ui.color-swatch.agent-contract\"",
        "data-ui-action=\"render\"",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorSwatch should expose machine-readable trace markers via `{needle}`."
        );
    }

    for forbidden in [
        "on:click=",
        "on:keydown=",
        "on:keyup=",
        "on:input=",
        "on:pointerdown=",
        "on:pointerup=",
        "on:focus=",
        "on:blur=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ColorSwatch has no key interaction replay chain; token `{forbidden}` should remain absent."
        );
    }

    let combined = format!("{view_source}\n{logic_source}\n{motion_source}");
    for forbidden in [
        "wasm_debug_proxy!",
        "observability::",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
        "request_replay",
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ColorSwatch production contract should not leak wasm-debug internals `{forbidden}`."
        );
    }

    for needle in [
        "WASM 调试要求：关键状态可追踪",
        "开发模式下至少能追踪关键状态变更来源与前后值",
        "关键交互链路应支持最小可复现记录",
        "调试开关默认不进入生产包体与公共 API",
    ] {
        assert!(
            check2_source.contains(needle),
            "ColorSwatch checklist should keep wasm debug governance contract marker `{needle}`."
        );
    }
}

#[test]
fn color_swatch_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-wasm-debug.sh");

    let needle = "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_wasm_debug_contract_is_explicitly_na_and_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm-debug check script should enforce ColorSwatch contract marker `{needle}`."
    );
}

fn color_swatch_docs_section(source: &str) -> &str {
    let section_start = source
        .find("pub(super) fn color_swatch() -> AnyView {")
        .unwrap_or_else(|| panic!("display_extra docs should contain color_swatch section"));
    let section_tail = &source[section_start..];
    let section_end_rel = section_tail
        .find("\npub(super) fn color_swatch_picker() -> AnyView {")
        .unwrap_or_else(|| {
            panic!("display_extra docs should contain color_swatch_picker after color_swatch")
        });
    &section_tail[..section_end_rel]
}

#[test]
fn color_swatch_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let docs_section = color_swatch_docs_section(&docs_source);
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let dx_script_source = load_source("../../scripts/check-ui-dx.sh");
    let check2_source = load_source("../../components/color-swatch/check2.md");

    for required in [
        "let color_swatch_imports = \"use leptos::prelude::*;\\nuse ui::{ColorSwatch, ColorSwatchRounding, ColorSwatchShape, ColorSwatchSize};\".to_string();",
        "title=\"Hello World (Default Path)\"",
        "title=\"Comparison Matrix (Size / Alpha / Shape / Empty)\"",
        "title=\"Controlled vs Uncontrolled Contrast (N/A for ColorSwatch)\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=color_swatch_imports.clone()",
        "description=\"ColorSwatch has no controllable state axis; compare default rendering with upstream state mapped into plain props.\"",
        "description=\"ColorSwatch is a display leaf: streaming is optional and falls back to snapshot (`data-ui-stream-support=optional`, `data-ui-stream-fallback=snapshot`).\"",
        "description=\"Copy action auto-injects missing imports for direct run.\"",
        "effective component markers: data-ui-stream-support=optional data-ui-stream-fallback=snapshot data-ui-output-status=verified",
    ] {
        assert!(
            docs_section.contains(required),
            "ColorSwatch docs-as-product section should keep marker `{required}`."
        );
    }

    for required in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let missing_imports = missing_import_lines(&raw, &imports);",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(required),
            "Playground copy-ready pipeline should keep marker `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";
    assert!(
        dx_script_source.contains(script_needle),
        "DX gate script should include docs-as-product contract `{script_needle}`."
    );

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "受控/非受控对照对 `ColorSwatch` 为 N/A",
        "color_swatch_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "scripts/check-ui-dx.sh",
        "compose_copy_ready_code",
    ] {
        assert!(
            check2_source.contains(required),
            "ColorSwatch check2 docs-as-product evidence should include `{required}`."
        );
    }
}

#[test]
fn color_swatch_check2_documents_source_first_copy_paste_ready_rules() {
    let checklist_source = load_source("../../components/color-swatch/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            checklist_source.contains(required),
            "color-swatch checklist should keep source-first copy-paste-ready rule `{required}`.",
        );
    }
}

#[test]
fn color_swatch_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let docs_section = color_swatch_docs_section(&docs_source);
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let readme_source = load_source("../../components/color-swatch/src/README.md");

    for needle in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "data-slot=\"color-swatch-source-first-contract\"",
        "<h3>\"Source-first / Copy-Paste Ready Contract\"</h3>",
        "<code>\"Show code\"</code>",
        "code_imports=color_swatch_imports.clone()",
        "Dependency baseline (Cargo.toml): ",
        "component-color_swatch",
        "inject-css",
        "data-slot=\"color-swatch-source-paths\"",
        "components/color-swatch/src/mod.rs",
        "components/color-swatch/src/logic.rs",
        "components/color-swatch/src/view.rs",
        "components/color-swatch/src/styles.rs",
        "components/color-swatch/src/motion.rs",
    ] {
        assert!(
            docs_section.contains(needle),
            "color-swatch source-first docs should contain `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "<CodeBlock code=resolved_code.get() />",
        "missing_import_lines(&raw, &imports)",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should contain `{needle}`.",
        );
    }

    for needle in [
        "## Source-first",
        "组件源码：`components/color-swatch/src/{mod,logic,view,styles,motion}.rs`",
        "package feature：`component-color_swatch`（可选叠加 `inject-css`）",
    ] {
        assert!(
            readme_source.contains(needle),
            "color-swatch README should document source-first dependency/path marker `{needle}`.",
        );
    }
}

#[test]
fn color_swatch_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce source-first copy-paste-ready contract `{needle}`.",
        );
    }
}

#[test]
fn color_swatch_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_source("../../components/color-swatch/check2.md");

    for marker in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "color_swatch_check2_documents_source_first_copy_paste_ready_rules",
        "color_swatch_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "color_swatch_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "color-swatch checklist should keep source-first copy-paste-ready evidence marker `{marker}`.",
        );
    }
}

#[test]
fn color_swatch_check2_documents_heroui_benchmark_docs_sync_rules() {
    let checklist_source = load_source("../../components/color-swatch/check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            checklist_source.contains(required),
            "color-swatch checklist should keep heroui-benchmark docs-sync rule `{required}`.",
        );
    }
}

#[test]
fn color_swatch_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let readme_source = load_source("../../components/color-swatch/src/README.md");

    for needle in [
        "### ColorSwatch 同步记录（2026-02-20）",
        "参数模型同步：`ColorSwatch` 维持 display color preview primitive 定位",
        "component_doc!(\"ColorSwatch\", \"color-swatch\", \"Display\", display_extra::color_swatch)",
        "#/components/color-swatch",
        "`components/color-swatch/src/README.md` 提供等价文档入口",
        "display_extra.rs::color_swatch()",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "heroui strategy doc should include color-swatch synchronization marker `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"ColorSwatch\"",
        "\"color-swatch\"",
        "display_extra::color_swatch",
    ] {
        assert!(
            pages_source.contains(needle),
            "component docs index should expose color-swatch entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn color_swatch() -> AnyView {",
        "title=\"ColorSwatch\"",
        "slug=\"color-swatch\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app color-swatch page should stay indexable via marker `{needle}`.",
        );
    }

    assert!(
        readme_source.contains("# ColorSwatch"),
        "color-swatch README should remain an equivalent component doc entry.",
    );
}

#[test]
fn color_swatch_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce heroui-benchmark docs-sync contract `{needle}`.",
        );
    }
}

#[test]
fn color_swatch_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("../../components/color-swatch/check2.md");

    for marker in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "color_swatch_check2_documents_heroui_benchmark_docs_sync_rules",
        "color_swatch_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "color_swatch_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "color-swatch check2 should keep heroui-benchmark docs-sync evidence marker `{marker}`.",
        );
    }
}

#[test]
fn color_swatch_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_source("../../components/color-swatch/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(required),
            "ColorSwatch checklist should keep docs-sync/state-matrix rule `{required}`."
        );
    }

    for marker in [
        "color_swatch_check2_documents_docs_sync_and_state_matrix_rules",
        "color_swatch_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "color_swatch_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "color_swatch/check2.md should keep docs-sync evidence marker `{marker}`."
        );
    }
}

#[test]
fn color_swatch_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let docs_section = color_swatch_docs_section(&docs_source);
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");

    for marker in [
        "pub(super) fn color_swatch() -> AnyView {",
        "title=\"Hello World (Default Path)\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "title=\"Comparison Matrix (Size / Alpha / Shape / Empty)\"",
        "title=\"Controlled vs Uncontrolled Contrast (N/A for ColorSwatch)\"",
        "description=\"ColorSwatch has no controllable state axis; compare default rendering with upstream state mapped into plain props.\"",
        "<ColorSwatch color=\"#2663eb\".to_string() />",
        "size=ColorSwatchSize::Xs",
        "size=ColorSwatchSize::Sm",
        "shape=ColorSwatchShape::Wide",
        "is_bordered=true",
        "if size != ColorSwatchSize::Md {",
        "if rounding != ColorSwatchRounding::Default {",
        "if shape != ColorSwatchShape::Square {",
    ] {
        assert!(
            docs_section.contains(marker),
            "ColorSwatch docs examples should keep state-matrix/API sync marker `{marker}`."
        );
    }

    for marker in [
        "#[prop(optional, into)] color: Option<String>",
        "#[prop(optional, into)] color_name: Option<String>",
        "#[prop(optional)] size: ColorSwatchSize",
        "#[prop(optional)] rounding: ColorSwatchRounding",
        "#[prop(optional)] shape: ColorSwatchShape",
        "#[prop(optional, into)] is_bordered: Option<bool>",
        "#[prop(optional, into)] is_decorative: Option<bool>",
        "#[prop(optional)] motion: ColorSwatchMotion",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
    ] {
        assert!(
            view_source.contains(marker),
            "ColorSwatch view public API should keep `{marker}` for docs/runtime sync."
        );
    }

    for marker in [
        "normalize_aria_label",
        "normalize_is_bordered",
        "normalize_is_decorative",
        "resolve_alpha(color.as_deref())",
        "resolve_state(ColorSwatchStateInput {",
        "compose_inline_style(color).unwrap_or_default()",
    ] {
        assert!(
            logic_source.contains(marker),
            "ColorSwatch logic defaults should keep `{marker}` for docs consistency."
        );
    }

    for forbidden in [
        "variant=",
        "is_disabled=",
        "default_size",
        "on_size_change",
        "default_bordered",
        "on_bordered_change",
    ] {
        assert!(
            !docs_section.contains(forbidden),
            "ColorSwatch docs should avoid stale/aliased API token `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce docs-sync/state-matrix contract `{needle}`."
        );
    }
}

#[test]
fn color_swatch_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("../../components/color-swatch/check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
        "color_swatch_check2_documents_documentation_as_product_rules",
        "color_swatch_documentation_entry_exists_with_beginner_first_progression",
        "color_swatch_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "color-swatch check2 should keep documentation-as-product evidence `{required}`.",
        );
    }
}

#[test]
fn color_swatch_documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = load_source("../../components/color-swatch/src/README.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let docs_section = color_swatch_docs_section(&docs_source);

    for required in [
        "# ColorSwatch",
        "## Hello World（最小可用）",
        "<ColorSwatch color=\"#2663eb\".to_string() />",
        "## 常见用法",
        "## 新手路径（先用起来，再进阶）",
        "1. 先跑默认路径：`<ColorSwatch color=\"#2663eb\".to_string() />`",
        "2. 再加常见参数：`color_name`、`size`、`shape`、`is_bordered`",
        "3. 最后再用进阶参数：`is_decorative`、`aria_label`、`class_name`、`lang`、`dir`",
        "docs-app 页面：`apps/docs-app/src/pages/components/pages/display_extra.rs::color_swatch()`",
    ] {
        assert!(
            readme_source.contains(required),
            "color-swatch README should keep beginner-first doc marker `{required}`.",
        );
    }

    let hello_pos = docs_section
        .find("title=\"Hello World (Default Path)\"")
        .expect("docs should include color-swatch Hello World playground");
    let interactive_pos = docs_section
        .find("title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"")
        .expect("docs should include color-swatch Interactive Playground");
    assert!(
        hello_pos < interactive_pos,
        "docs should keep beginner-first order: Hello World before Interactive Playground.",
    );

    for required in [
        "pub(super) fn color_swatch() -> AnyView {",
        "title=\"Hello World (Default Path)\"",
        "title=\"Comparison Matrix (Size / Alpha / Shape / Empty)\"",
        "title=\"Controlled vs Uncontrolled Contrast (N/A for ColorSwatch)\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
    ] {
        assert!(
            docs_section.contains(required),
            "docs color-swatch entry should include `{required}`.",
        );
    }
}

#[test]
fn color_swatch_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_documentation_as_product_rules",
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "DX script should enforce documentation-as-product contract `{needle}`.",
        );
    }
}

#[test]
fn color_swatch_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("../../components/color-swatch/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
        "color_swatch_check2_documents_interactive_playground_rules",
        "color_swatch_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "color_swatch_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "color_swatch_dx_check_script_covers_interactive_playground_contract",
        "color_swatch_e2e_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-dx.sh",
        "components/color-swatch/scripts/check-ui-e2e-color-swatch.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "color-swatch checklist should keep interactive-playground evidence marker `{required}`."
        );
    }
}

#[test]
fn color_swatch_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let docs_section = color_swatch_docs_section(&docs_source);

    for marker in [
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "data-slot=\"color-swatch-workbench-controls\"",
        "data-slot=\"color-swatch-workbench-size-control\"",
        "data-slot=\"color-swatch-workbench-shape-control\"",
        "data-slot=\"color-swatch-workbench-rounding-control\"",
        "data-slot=\"color-swatch-workbench-alpha-control\"",
        "data-slot=\"color-swatch-workbench-bordered-switch\"",
        "data-slot=\"color-swatch-workbench-decorative-switch\"",
        "data-slot=\"color-swatch-workbench-custom-aria-switch\"",
        "data-slot=\"color-swatch-workbench-custom-class-switch\"",
        "data-slot=\"color-swatch-workbench-lang-switch\"",
        "data-slot=\"color-swatch-workbench-canvas\"",
        "test_css_source=workbench_test_css",
        "test_config_signal=workbench_config",
        "let workbench_code = Signal::derive(move || {",
        "let workbench_config = Signal::derive(move || {",
        "Switch checked=is_bordered set_checked=set_is_bordered",
        "Switch checked=is_decorative set_checked=set_is_decorative",
        "Switch checked=custom_aria set_checked=set_custom_aria",
        "Switch checked=custom_class set_checked=set_custom_class",
        "Switch checked=custom_lang set_checked=set_custom_lang",
    ] {
        assert!(
            docs_section.contains(marker),
            "color-swatch docs interactive playground should keep marker `{marker}`."
        );
    }
}

#[test]
fn color_swatch_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_swatch_contract.spec.mjs");

    for marker in [
        "docs-app color-swatch key flow is repeatable with semantic breakpoints",
        "data-slot=\"color-swatch-workbench-controls\"",
        "data-slot=\"color-swatch-workbench-canvas\"",
        "data-slot=\"color-swatch-workbench-alpha-control\"",
        "data-slot=\"color-swatch-workbench-decorative-switch\"",
        "data-slot=\"color-swatch-workbench-custom-aria-switch\"",
        "data-slot=\"color-swatch-workbench-lang-switch\"",
        "toHaveAttribute(\"data-alpha\", \"transparent\")",
        "toHaveAttribute(\"data-decorative\", \"true\")",
        "toHaveAttribute(\"data-aria-source\", \"custom\")",
        "toHaveAttribute(\"lang\", \"zh-CN\")",
        "toHaveAttribute(\"data-motion-source\", \"default\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(marker),
            "color-swatch interactive playground e2e flow should keep marker `{marker}`."
        );
    }
}

#[test]
fn color_swatch_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_interactive_playground_rules",
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_docs_app_provides_interactive_playground_for_props_state_and_preview",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce interactive-playground contract `{needle}`."
        );
    }
}

#[test]
fn color_swatch_e2e_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../components/color-swatch/scripts/check-ui-e2e-color-swatch.sh");

    for needle in [
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_e2e_check_script_covers_interactive_playground_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "e2e check script should enforce interactive-playground contract `{needle}`."
        );
    }
}

#[test]
fn color_swatch_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na()
{
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let dx_script_source = load_source("../../scripts/check-ui-dx.sh");
    let check2_source = load_source("../../components/color-swatch/check2.md");

    for required in [
        "let scope_selector = format!(\"[data-playground-scope=\\\"{scope_id}\\\"]\");",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "<div data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "let (test_css, set_test_css) = signal(default_test_css.get_untracked());",
        "<textarea",
        "class=\"playground__test-editor\"",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "Original CSS is loaded. Use :scope to target this playground only.",
        "on_press=on_reset_test_css",
        "\"Restore original CSS\"",
        "data-slot=\"playground-controls\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(required),
            "Playground should keep DX hot-style-feedback + isolated-canvas token `{required}`."
        );
    }

    let section = color_swatch_docs_section(&docs_source);

    for required in [
        "title=\"ColorSwatch\"",
        "slug=\"color-swatch\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "test_css_source=workbench_test_css",
        "test_source_path=\"crates/ui/src/color/swatch/styles.rs\".to_string()",
        "test_config_signal=workbench_config",
        "id_base=\"docs-color-swatch-workbench-size\".to_string()",
        "id_base=\"docs-color-swatch-workbench-shape\".to_string()",
        "id_base=\"docs-color-swatch-workbench-rounding\".to_string()",
        "id_base=\"docs-color-swatch-workbench-alpha\".to_string()",
        "Switch checked=is_bordered set_checked=set_is_bordered",
        "Switch checked=is_decorative set_checked=set_is_decorative",
        "Switch checked=custom_aria set_checked=set_custom_aria",
        "Switch checked=custom_class set_checked=set_custom_class",
        "Switch checked=custom_lang set_checked=set_custom_lang",
        "let workbench_code = Signal::derive(move || {",
        "let workbench_config = Signal::derive(move || {",
        "alpha={}, bordered={}, is_decorative={}",
    ] {
        assert!(
            section.contains(required),
            "ColorSwatch docs should provide isolated demo/workbench token `{required}`."
        );
    }

    for forbidden in [
        "Persist workbench state",
        "workbench_persist_state",
        "load_color_swatch_workbench_state",
        "save_color_swatch_workbench_state",
        "clear_color_swatch_workbench_state",
        "localStorage",
        "sessionStorage",
    ] {
        assert!(
            !section.contains(forbidden),
            "ColorSwatch keeps optional persisted workbench state as N/A; token `{forbidden}` should stay absent."
        );
    }

    let script_needle = "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na";
    assert!(
        dx_script_source.contains(
            "echo \"[dx] contract: color-swatch playground css hot-reload + isolated canvas\""
        ) && dx_script_source.contains(script_needle),
        "DX gate script should include ColorSwatch contract markers."
    );

    for required in [
        "DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
    ] {
        assert!(
            check2_source.contains(required),
            "ColorSwatch checklist should keep DX governance rule `{required}`."
        );
    }
}

#[test]
fn color_swatch_dx_check_script_covers_hot_reload_and_isolated_canvas_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    let needle = "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na";
    assert!(
        script_source.contains(needle),
        "DX check script should enforce ColorSwatch contract marker `{needle}`."
    );
}

#[test]
fn color_swatch_e2e_contract_uses_semantic_selectors_and_stable_waits() {
    let source = load_source("../../e2e/tests/docs_app_color_swatch_contract.spec.mjs");

    for needle in [
        "/#/components/color-swatch",
        "body:not(:has(#boot))",
        "data-component=\"color-swatch\"",
        "data-slot=\"color-swatch-workbench-controls\"",
        "data-slot=\"color-swatch-workbench-canvas\"",
        "data-slot=\"color-swatch-workbench-alpha-control\"",
        "data-slot=\"color-swatch-workbench-decorative-switch\"",
        "data-slot=\"color-swatch-workbench-custom-aria-switch\"",
        "data-slot=\"color-swatch-workbench-lang-switch\"",
        "data-ui-output-status",
        "data-motion-source",
    ] {
        assert!(
            source.contains(needle),
            "color-swatch e2e contract should include semantic marker `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout",
        "setTimeout",
        "sleep(",
        "nth-child(",
        "getByText(",
        "locator(\"text=",
        ".filter({ hasText:",
        ".locator('[data-slot=\"segmented-control\"]').nth(",
    ] {
        assert!(
            !source.contains(forbidden),
            "color-swatch e2e contract should avoid unstable token `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_swatch_contract.spec.mjs");

    for needle in [
        "docs-app color-swatch contract uses semantic selectors with settled waits",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "component.locator('[data-slot=\"color-swatch\"][data-ui-output-status=\"verified\"]').first()",
        "component.locator('[data-slot=\"color-swatch-workbench-controls\"]').first()",
        "component.locator('[data-slot=\"color-swatch-workbench-canvas\"]').first()",
        "controls\n    .locator('[data-slot=\"color-swatch-workbench-alpha-control\"] [data-slot=\"segmented-control\"]')",
        "controls\n    .locator('[data-slot=\"color-swatch-workbench-decorative-switch\"] [data-slot=\"switch\"]')",
        "controls\n    .locator('[data-slot=\"color-swatch-workbench-custom-aria-switch\"] [data-slot=\"switch\"]')",
        "controls\n    .locator('[data-slot=\"color-swatch-workbench-lang-switch\"] [data-slot=\"switch\"]')",
        "await decorativeSwitch.focus();",
        "await expect(decorativeSwitch).toBeFocused();",
        "await decorativeSwitch.press(\"Space\");",
        "await langSwitch.focus();",
        "await expect(langSwitch).toBeFocused();",
        "await langSwitch.press(\"Enter\");",
        "toHaveAttribute(\"data-alpha\", \"transparent\")",
        "toHaveAttribute(\"data-state\", \"transparent\")",
        "toHaveAttribute(\"data-aria-source\", \"custom\")",
        "toHaveAttribute(\"data-motion-source\", \"default\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "color-swatch e2e semantic-selector contract should include `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
        "nth-child(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "color-swatch e2e selector contract should avoid unstable token `{forbidden}`."
        );
    }
}

#[test]
fn color_swatch_e2e_flow_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_swatch_contract.spec.mjs");

    for needle in [
        "docs-app color-swatch contract uses semantic selectors with settled waits",
        "docs-app color-swatch key flow is repeatable with semantic breakpoints",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "await decorativeSwitch.focus();",
        "await decorativeSwitch.press(\"Space\");",
        "await langSwitch.focus();",
        "await langSwitch.press(\"Enter\");",
        "toHaveAttribute(\"data-state\", \"transparent\")",
        "toHaveAttribute(\"data-decorative\", \"true\")",
        "toHaveAttribute(\"aria-hidden\", \"true\")",
        "toBeFocused()",
        "await page.reload();",
        "toHaveAttribute(\"data-aria-source\", \"default\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "color-swatch e2e flow should keep semantic ready/settled marker `{needle}`."
        );
    }
}

#[test]
fn color_swatch_e2e_regression_suite_includes_repeatable_key_flow_and_keyboard_focus_risk_path() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_swatch_contract.spec.mjs");

    for needle in [
        "docs-app color-swatch key flow is repeatable with semantic breakpoints",
        "await decorativeSwitch.focus();",
        "await expect(decorativeSwitch).toBeFocused();",
        "await decorativeSwitch.press(\"Space\");",
        "await langSwitch.focus();",
        "await expect(langSwitch).toBeFocused();",
        "await langSwitch.press(\"Enter\");",
        "toHaveAttribute(\"data-decorative\", \"true\")",
        "toHaveAttribute(\"aria-hidden\", \"true\")",
        "toHaveAttribute(\"lang\", \"zh-CN\")",
        "await page.reload();",
        "toHaveAttribute(\"data-aria-source\", \"default\")",
        "toHaveAttribute(\"data-motion-source\", \"default\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "color-swatch e2e regression suite should keep repeatable key-flow marker `{needle}`."
        );
    }
}

#[test]
fn color_swatch_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("../../components/color-swatch/check2.md");
    let script_source = load_source("../../components/color-swatch/scripts/check-ui-e2e-color-swatch.sh");

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "e2e/tests/docs_app_color_swatch_contract.spec.mjs",
        "body:not(:has(#boot))",
        "data-slot=\"color-swatch-workbench-controls\"",
        "ready/settled",
        "color_swatch_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "color_swatch_e2e_flow_covers_ready_and_settled_semantic_breakpoints",
        "components/color-swatch/scripts/check-ui-e2e-color-swatch.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "color_swatch/check2.md should keep e2e stability marker `{needle}`."
        );
    }

    for needle in [
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_e2e_contract_uses_semantic_selectors_and_stable_waits",
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_e2e_flow_covers_ready_and_settled_semantic_breakpoints",
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_e2e_selector_and_stable_wait_rules",
    ] {
        assert!(
            script_source.contains(needle),
            "color-swatch e2e check script should include `{needle}`."
        );
    }
}

#[test]
fn color_swatch_check2_documents_repeatable_e2e_regression_collection() {
    let check2_source = load_source("../../components/color-swatch/check2.md");
    let script_source = load_source("../../components/color-swatch/scripts/check-ui-e2e-color-swatch.sh");

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "docs-app color-swatch key flow is repeatable with semantic breakpoints",
        "data-decorative/data-aria-source/lang/data-motion-source",
        "keyboard/focus",
        "overlay/async N/A",
        "color_swatch_e2e_regression_suite_includes_repeatable_key_flow_and_keyboard_focus_risk_path",
        "components/color-swatch/scripts/check-ui-e2e-color-swatch.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "color_swatch/check2.md should keep repeatable e2e regression marker `{needle}`."
        );
    }

    for needle in [
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_e2e_regression_suite_includes_repeatable_key_flow_and_keyboard_focus_risk_path",
        "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_check2_documents_repeatable_e2e_regression_collection",
    ] {
        assert!(
            script_source.contains(needle),
            "color-swatch e2e check script should include repeatable flow guard `{needle}`."
        );
    }
}

#[test]
fn color_swatch_engineering_contract_marks_spec_serde_path_as_na_and_keeps_tracing_runtime_boundaries()
 {
    let mod_source = load_source("../../components/color-swatch/src/mod.rs");
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let view_source = load_source("../../components/color-swatch/src/view.rs");
    let styles_source = load_source("../../components/color-swatch/src/styles.rs");
    let motion_source = load_source("../../components/color-swatch/src/motion.rs");
    let readme_source = load_source("../../components/color-swatch/src/README.md");

    let cargo_source = load_source("../../crates/ui/Cargo.toml");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let check2_source = load_source("../../components/color-swatch/check2.md");

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let spec_path = workspace_dir.join("components/color-swatch/src/spec.rs");
    assert!(
        !spec_path.exists(),
        "ColorSwatch is a simple display component and should keep spec/schema boundary as N/A."
    );

    let combined = format!(
        "{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}\n{readme_source}"
    );

    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "#[serde(",
        "SchemaVersion",
        "migrate_v1_to_v2",
        "spec/config",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ColorSwatch engineering contract should keep spec/serde path as N/A and avoid `{forbidden}`."
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "pub enum UiTraceEventKind {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            cargo_source.contains(needle) || trace_source.contains(needle),
            "Engineering baseline should keep unified tracing marker `{needle}`."
        );
    }

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::color_swatch::",
        "const COLOR_SWATCH_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ColorSwatch should avoid component-local tracing semantic drift token `{forbidden}`."
        );
    }

    for forbidden in [
        "tokio",
        "tokio::",
        "async_std",
        "async_std::",
        "async-std",
        "smol::",
        "runtime::Handle",
        "spawn_blocking(",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ColorSwatch engineering contract should not leak runtime marker `{forbidden}`."
        );
    }

    for required in [
        "工程能力统一",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            check2_source.contains(required),
            "ColorSwatch checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
fn color_swatch_engineering_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-engineering.sh");
    let needle = "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_engineering_contract_marks_spec_serde_path_as_na_and_keeps_tracing_runtime_boundaries";

    assert!(
        script_source.contains(needle),
        "engineering check script should enforce ColorSwatch contract marker `{needle}`."
    );
}

#[test]
fn color_swatch_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
 {
    let check2_source = load_source("../../components/color-swatch/check2.md");
    let script_source = load_source("../../scripts/check-ui-engineering.sh");
    let readme_source = load_source("../../components/color-swatch/src/README.md");
    let logic_source = load_source("../../components/color-swatch/src/logic.rs");
    let component_manifest = load_source("../../components/color-swatch/src/Component.toml");

    for required in [
        "pub enum ColorSwatchAgentSchema",
        "pub enum ColorSwatchAgentSchemaVersion",
        "Self::V1 => \"ui.color-swatch.agent-contract.v1\"",
        "Self::V1 => \"1\"",
    ] {
        assert!(
            logic_source.contains(required),
            "color-swatch logic should keep stable v1 schema marker `{required}` in non-breaking scope.",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "schema = \"ui.color-swatch.agent-contract.v1\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "Component.toml should keep v1 schema marker `{required}` in current scope.",
        );
    }

    for forbidden in [
        "V2",
        "migrate_v1_to_v2",
        "SchemaRegistry",
        "deprecated_window",
        "codemod",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !readme_source.contains(forbidden),
            "without major breaking upgrade, color-swatch should not claim migration path token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_swatch_semantics --no-default-features --features component-color_swatch,inject-css color_swatch_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `ColorSwatch` 改动未引入跨大版本 API 破坏升级，组件语义契约仍保持 `v1`（`components/color-swatch/src/logic.rs` 的 `ColorSwatchAgentSchema::V1`/`ColorSwatchAgentSchemaVersion::V1`，`components/color-swatch/src/Component.toml` 的 `schema_version = \"1\"` 与 `ui.color-swatch.agent-contract.v1`），因此不触发 Codemod/Schema Registry 弃用窗口与 `migrate_v1_to_v2` 迁移层要求。回归：`components/color-swatch/test/semantics.rs::color_swatch_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade`；门禁脚本：`scripts/check-ui-engineering.sh` 新增对应 `cargo test` 目标。）",
        "color_swatch_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep codemod/registry migration marker `{needle}`.",
        );
    }
}

#[test]
fn color_swatch_keeps_spec_rs_out_of_simple_component_boundary() {
    let mod_source = load_source("../../components/color-swatch/src/mod.rs");
    let readme_source = load_source("../../components/color-swatch/src/README.md");
    let check_source = load_source("../../components/color-swatch/src/check2.md");

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let spec_path = workspace_dir.join("components/color-swatch/src/spec.rs");

    assert!(
        !spec_path.exists(),
        "simple ColorSwatch component must not introduce `src/spec.rs`."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "ColorSwatchSpec",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !readme_source.contains(forbidden),
            "ColorSwatch should not expose spec.rs contract token `{forbidden}`."
        );
    }

    assert!(
        readme_source.contains("## Hello World（最小可用）")
            && readme_source.contains("<ColorSwatch color=\"#2663eb\".to_string() />"),
        "simple-component usage guidance should remain in README."
    );
    assert!(
        check_source.contains("`spec.rs`"),
        "check2 should carry spec.rs applicability rules for simple components."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn color_swatch_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "test_source_path=\"components/color-swatch/src/styles.rs\".to_string()",
        "test_config_signal=workbench_config",
        "title=\"Comparison Matrix (Size / Alpha / Shape / Empty)\"",
        "size=ColorSwatchSize::Xs",
        "size=ColorSwatchSize::Sm",
        "ColorSwatchSize::Md",
        "size=ColorSwatchSize::Lg",
        "rounding=ColorSwatchRounding::Full",
        "color_name=\"Brand blue / 35%\".to_string()",
        "shape=ColorSwatchShape::Wide",
        "rounding=ColorSwatchRounding::Default",
        "class_name=\"docs-color-swatch-custom\".to_string()",
        "color_name=\"No fill\".to_string()",
        "is_bordered=true",
        "is_decorative=is_decorative",
    ] {
        assert!(
            source.contains(needle),
            "color-swatch docs playground should contain `{needle}`.",
        );
    }
}
