use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(rel_path)
        .exists()
}

#[test]
fn separator_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/separator/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Separator internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn separator_uses_logic_state_model() {
    let view_source = load_source("src/separator/view.rs");
    let logic_source = load_source("src/separator/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/separator.rs");
    let headless_source = load_source("../ui-headless/src/separator.rs");

    for needle in [
        "pub struct SeparatorStateInput",
        "pub use crate::button::normalize_optional_text;",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Separator state primitives should define `{needle}` in ui-state-primitives."
        );
    }

    for needle in [
        "pub use ui_state_primitives::separator::{",
        "SeparatorNormalizeInput",
        "SeparatorNormalizedProps",
        "SeparatorStateInput",
        "normalize_orientation",
        "normalize_is_decorative",
        "normalize_element_type",
        "normalize_props",
        "resolve_state",
        "compose_class_name",
    ] {
        assert!(
            logic_source.contains(needle),
            "Separator logic should consume state primitives via `{needle}`."
        );
    }

    for needle in [
        "logic::normalize_props(SeparatorNormalizeInput {",
        "let state = logic::resolve_state(normalized.state_input);",
        "let class = logic::compose_class_name(normalized.class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "Separator view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }

    for needle in [
        "pub struct SeparatorOptions",
        "pub struct SeparatorHandlers",
        "pub struct SeparatorAttrs",
        "pub struct SeparatorSemanticState",
        "pub struct SeparatorContract",
        "pub fn use_separator(options: SeparatorOptions) -> SeparatorContract",
        "pub lang: Option<String>",
        "pub dir: Option<A11yDirection>",
    ] {
        assert!(
            headless_source.contains(needle),
            "Separator headless contract should include `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, SeparatorOptions, use_separator};",
        "let separator_a11y = use_separator(SeparatorOptions { state, lang, dir });",
        "let role = separator_a11y.attrs.role;",
        "let aria_orientation = separator_a11y.attrs.aria_orientation;",
        "let aria_hidden = separator_a11y.attrs.aria_hidden;",
    ] {
        assert!(
            view_source.contains(needle),
            "Separator view should mount headless output via `{needle}`."
        );
    }

    for forbidden in [
        "let role = state.is_semantic.then_some(\"separator\");",
        "let aria_hidden = state.is_decorative.then_some(\"true\");",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Separator view should not inline headless A11y derivation `{forbidden}`."
        );
    }
}

#[test]
fn separator_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/separator/view.rs");

    for attr in [
        "data-slot=\"separator\"",
        "data-state=state.state_attr",
        "data-state-source=state.state_source_attr",
        "data-ui-schema=state.ui_schema_attr",
        "data-ui-intent=state.intent_attr",
        "data-ui-action=state.action_attr",
        "data-output-mode=state.output_mode_attr",
        "data-streaming-fallback=state.streaming_fallback_attr",
        "data-output-status=state.output_status_attr",
        "data-orientation=state.orientation_attr",
        "data-element=state.element_attr",
        "data-decorative=state.is_decorative.then_some(\"true\")",
        "data-semantic=state.is_semantic.then_some(\"true\")",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
        "role=role",
        "aria-orientation=aria_orientation",
    ] {
        assert!(
            source.contains(attr),
            "Separator should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn separator_state_markers_are_observable_searchable_and_enumerated() {
    let view_source = load_source("src/separator/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/separator.rs");

    for needle in [
        "data-state=state.state_attr",
        "data-state-source=state.state_source_attr",
        "data-ui-schema=state.ui_schema_attr",
        "data-ui-intent=state.intent_attr",
        "data-ui-action=state.action_attr",
        "data-output-mode=state.output_mode_attr",
        "data-streaming-fallback=state.streaming_fallback_attr",
        "data-output-status=state.output_status_attr",
        "data-orientation=state.orientation_attr",
        "data-element=state.element_attr",
        "data-motion-source=motion_source",
        "role=role",
        "aria-orientation=aria_orientation",
        "aria-hidden=aria_hidden",
    ] {
        assert!(
            view_source.contains(needle),
            "Separator should expose stable semantic selectors via `{needle}`."
        );
    }

    for needle in [
        "pub const SEPARATOR_UI_SCHEMA: &str = \"ui.separator.v1\";",
        "state_attr: if input.decorative {\n            \"decorative\"\n        } else {\n            \"semantic\"\n        },",
        "state_source_attr: \"props-static\",",
        "ui_schema_attr: SEPARATOR_UI_SCHEMA,",
        "intent_attr: \"separate-content\",",
        "action_attr: \"none\",",
        "output_mode_attr: \"snapshot\",",
        "streaming_fallback_attr: \"snapshot\",",
        "output_status_attr: \"verified\",",
        "SeparatorOrientation::Horizontal => \"horizontal\"",
        "SeparatorOrientation::Vertical => \"vertical\"",
        "SeparatorElementType::Div => \"div\"",
        "SeparatorElementType::Hr => \"hr\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Separator state marker values should stay closed/enumerated via `{needle}`."
        );
    }
}

#[test]
fn separator_agent_contract_schema_is_typed_and_traceable() {
    let view_source = load_source("src/separator/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/separator.rs");

    for marker in [
        "data-ui-schema=state.ui_schema_attr",
        "data-ui-intent=state.intent_attr",
        "data-ui-action=state.action_attr",
        "data-state=state.state_attr",
        "data-state-source=state.state_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "Separator should expose typed Agent Contract marker `{marker}`."
        );
    }

    for marker in [
        "pub const SEPARATOR_UI_SCHEMA: &str = \"ui.separator.v1\";",
        "ui_schema_attr: SEPARATOR_UI_SCHEMA,",
        "intent_attr: \"separate-content\",",
        "action_attr: \"none\",",
    ] {
        assert!(
            primitive_source.contains(marker),
            "Separator state primitive should own Agent Contract value `{marker}`."
        );
    }

    assert!(
        !view_source.contains("data-ui-schema=\"ui.separator.v1\""),
        "Separator view should not hardcode schema strings; schema must come from typed primitive output."
    );
}

#[test]
fn separator_streaming_scope_is_snapshot_only_and_machine_readable() {
    let view_source = load_source("src/separator/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/separator.rs");

    for marker in [
        "data-output-mode=state.output_mode_attr",
        "data-streaming-fallback=state.streaming_fallback_attr",
        "data-output-status=state.output_status_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "Separator should expose streaming-mode marker `{marker}`."
        );
    }

    for marker in [
        "output_mode_attr: \"snapshot\",",
        "streaming_fallback_attr: \"snapshot\",",
        "output_status_attr: \"verified\",",
    ] {
        assert!(
            primitive_source.contains(marker),
            "Separator primitive should keep closed-set snapshot marker `{marker}`."
        );
    }

    for forbidden in [
        "is_streaming",
        "on_stream_chunk",
        "on_streaming_change",
        "streaming_state",
        "stream_delta",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Separator should stay snapshot-only and avoid streaming-only API token `{forbidden}`."
        );
    }
}

#[test]
fn separator_styles_include_state_marker_contracts() {
    let source = load_source("src/separator/styles.rs");

    for selector in [
        ".ui-separator[data-motion-source=\"custom\"]",
        ".ui-separator[data-custom-motion=\"true\"]",
        ".ui-separator--horizontal",
        ".ui-separator[data-orientation=\"vertical\"]",
        ".ui-separator--element-hr",
        ".ui-separator[data-element=\"div\"]",
        ".ui-separator--semantic",
        ".ui-separator[data-state=\"semantic\"]",
        ".ui-separator--decorative",
        ".ui-separator[data-state=\"decorative\"]",
        ".ui-separator[data-decorative=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Separator styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn separator_styles_depend_on_explicit_state_markers_not_dom_shape() {
    let styles_source = load_source("src/separator/styles.rs");
    let view_source = load_source("src/separator/view.rs");

    for needle in [
        ".ui-separator[data-orientation=\"horizontal\"]",
        ".ui-separator[data-orientation=\"vertical\"]",
        ".ui-separator[data-element=\"div\"]",
        ".ui-separator[data-element=\"hr\"]",
        ".ui-separator[data-state=\"semantic\"]",
        ".ui-separator[data-state=\"decorative\"]",
        ".ui-separator[data-decorative=\"true\"]",
        ".ui-separator[data-custom-motion=\"true\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "Separator styles should branch from explicit semantic markers `{needle}`."
        );
    }

    for forbidden in [":nth-child", ":first-child", ":last-child", "+", "~"] {
        assert!(
            !styles_source.contains(forbidden),
            "Separator styles should not depend on fragile DOM-shape selector token `{forbidden}`."
        );
    }

    assert!(
        !view_source.contains("style="),
        "Separator view should not push business style logic through inline style."
    );
}

#[test]
fn separator_theme_tokens_are_defined_mapped_and_consumed_via_css_variables() {
    let styles_source = load_source("src/separator/styles.rs");
    let theme_tokens_source = load_source("../ui-theme/src/tokens.rs");
    let theme_mapping_source = load_source("../ui-theme/src/theme.rs");
    let theme_css_source = load_source("../ui-theme/src/css.rs");
    let theme_baseline_test_source = load_source("../ui-theme/tests/token_scale_baseline.rs");
    let styling_spec_source = load_source("../../docs/spec/styling.md");

    assert!(
        theme_tokens_source.contains("pub separator_decorative_opacity_percent: u8,"),
        "ui-theme tokens.rs should define separator decorative opacity token taxonomy."
    );
    assert!(
        theme_mapping_source.contains("separator_decorative_opacity_percent: 72,"),
        "ui-theme theme.rs should map separator decorative opacity baseline."
    );
    assert!(
        theme_css_source.contains("--ui-separator-decorative-opacity:"),
        "ui-theme css.rs should emit separator decorative opacity variable."
    );
    assert!(
        theme_baseline_test_source.contains("--ui-separator-decorative-opacity:"),
        "ui-theme token scale baseline tests should guard separator decorative opacity emission."
    );
    assert!(
        styles_source.contains("opacity: var(--ui-separator-decorative-opacity);"),
        "separator styles should consume the theme variable directly."
    );
    assert!(
        !styles_source.contains("0.72"),
        "separator styles should not keep hardcoded decorative opacity literals."
    );
    assert!(
        styling_spec_source.contains("--ui-separator-decorative-opacity"),
        "styling spec should document separator token flow in tokens -> theme -> css chain."
    );
}

#[test]
fn separator_token_first_static_styles_are_aggregated_and_injected_via_uiroot() {
    let styles_source = load_source("src/separator/styles.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let view_source = load_source("src/separator/view.rs");

    assert!(
        styles_source.contains("pub const CSS: &str"),
        "Separator should define static css contract in styles.rs."
    );
    for needle in [
        "var(--ui-border)",
        "var(--ui-separator-opacity, 1)",
        "var(--ui-separator-scale-x, 1)",
        "var(--ui-separator-scale-y, 1)",
        "var(--ui-separator-decorative-opacity)",
    ] {
        assert!(
            styles_source.contains(needle),
            "Separator styles should consume tokenized css variables via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-separator\")]",
        "out.push_str(crate::separator::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css aggregator should feature-gate separator styles via `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should be the centralized component css injection point via `{needle}`."
        );
    }

    for forbidden in [
        "class=\"flex ",
        "class=\"grid ",
        "class=\"p-",
        "class=\"m-",
        "stylist::",
    ] {
        assert!(
            !styles_source.contains(forbidden) && !view_source.contains(forbidden),
            "Separator component contract should avoid utility-first/css-in-rust leakage `{forbidden}`."
        );
    }
}

#[test]
fn separator_visual_desire_is_scoped_and_must_not_regress_theme_quality() {
    let styles_source = load_source("src/separator/styles.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let check2_source = load_source("src/separator/check2.md");

    for needle in [
        "background: var(--ui-border);",
        "opacity: var(--ui-separator-opacity, 1);",
        "opacity: var(--ui-separator-decorative-opacity);",
    ] {
        assert!(
            styles_source.contains(needle),
            "Separator default visual baseline should stay tokenized via `{needle}`."
        );
    }

    for forbidden in [
        "rgb(",
        "rgba(",
        "hsl(",
        "hsla(",
        "box-shadow: 0",
        "border-radius:",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Separator should avoid hardcoded decorative styling drift `{forbidden}`."
        );
    }

    for needle in [
        "title=\"Separator\"",
        "Playground title=\"Semantic + Element Type\"",
        "Playground title=\"Decorative + Custom Class\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app should keep a visible separator baseline entry via `{needle}`."
        );
    }

    assert!(
        check2_source.contains("N/A：Button/Input/Overlay 的截图基线属于仓库级视觉回归门禁"),
        "Separator check2 should explicitly scope cross-component screenshot baseline as repo-level gate."
    );
}

#[test]
fn separator_tree_shaking_contract_is_feature_gated_and_budget_guarded() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo_source = load_source("../../apps/web-demo/Cargo.toml");
    let tree_script_source = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");
    let check2_source = load_source("src/separator/check2.md");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "component-separator = []",
        "all-components = [",
        "\"component-separator\",",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components Cargo feature graph should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-separator\")]",
        "pub mod separator;",
    ] {
        assert!(
            lib_source.contains(needle),
            "lib.rs should gate separator module by `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-separator\")]",
        "out.push_str(crate::separator::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "css.rs should gate separator CSS aggregation by `{needle}`."
        );
    }

    assert!(
        web_demo_cargo_source.contains("default-features = false"),
        "web-demo should consume ui-components without default all-components."
    );
    assert!(
        web_demo_cargo_source.contains("features = [\"inject-css\", \"web-demo-components\"]"),
        "web-demo should opt into `web-demo-components` bundle explicitly."
    );
    assert!(
        !web_demo_cargo_source.contains("\"all-components\""),
        "web-demo dependency contract should not explicitly pull all-components."
    );

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$MIN_FEATURES\"",
        "cargo tree -e features -i ui-components -p web-demo",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_script_source.contains(needle),
            "tree-shaking gate script should include `{needle}`."
        );
    }
    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget env should include `{needle}`."
        );
    }

    for needle in [
        "MIN_TREE_HAS_ALL_COMPONENTS=no",
        "MIN_TREE_HAS_COMPONENT_SEPARATOR=yes",
        "MIN_TREE_HAS_INJECT_CSS=yes",
        "WEB_TREE_HAS_ALL_COMPONENTS=no",
        "WEB_TREE_HAS_WEB_DEMO_COMPONENTS=yes",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-separator,inject-css",
        "CURRENT_BYTES=1210354",
        "MAX_BYTES=2000000",
        "SEPARATOR_BUDGET_STATUS=pass",
    ] {
        assert!(
            check2_source.contains(needle),
            "separator check2 should preserve executable tree-shaking evidence `{needle}`."
        );
    }
}

#[test]
fn separator_type_system_and_semantic_markers_form_machine_readable_contract() {
    let primitive_source = load_source("../ui-state-primitives/src/separator.rs");
    let logic_source = load_source("src/separator/logic.rs");
    let view_source = load_source("src/separator/view.rs");
    let tests_source = load_source("tests/separator_semantics.rs");

    for needle in [
        "pub enum SeparatorOrientation",
        "pub enum SeparatorElementType",
        "pub struct SeparatorStateInput",
        "pub struct SeparatorState",
        "pub fn resolve_state(input: SeparatorStateInput) -> SeparatorState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Separator primitive type system should include `{needle}`."
        );
    }

    for needle in [
        "pub struct SeparatorNormalizeInput",
        "pub struct SeparatorNormalizedProps",
        "normalize_orientation(value: Option<SeparatorOrientation>)",
        "normalize_element_type(value: Option<SeparatorElementType>)",
        "normalize_props(input: SeparatorNormalizeInput)",
        "resolve_state(normalized.state_input)",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "Separator logic normalization contract should include `{needle}`."
        );
    }
    for forbidden in [
        "orientation: Option<String>",
        "element_type: Option<String>",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Separator should avoid free-form string protocol for discrete axes `{forbidden}`."
        );
    }

    for marker in [
        "data-state=state.state_attr",
        "data-state-source=state.state_source_attr",
        "data-orientation=state.orientation_attr",
        "data-element=state.element_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "Separator should expose machine-readable semantic marker `{marker}`."
        );
    }

    for needle in [
        "fn separator_discrete_state_axes_are_type_constrained_with_enums()",
        "fn separator_state_markers_are_observable_searchable_and_enumerated()",
        "fn separator_state_normalization_is_centralized_in_logic()",
    ] {
        assert!(
            tests_source.contains(needle),
            "Separator contract feedback loop should keep dedicated regression `{needle}`."
        );
    }
}

#[test]
fn separator_cross_platform_compile_contract_has_explicit_cfg_and_no_non_wasm_web_sys_usage() {
    let mod_source = load_source("src/separator/mod.rs");
    let logic_source = load_source("src/separator/logic.rs");
    let view_source = load_source("src/separator/view.rs");
    let motion_source = load_source("src/separator/motion.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let check2_source = load_source("src/separator/check2.md");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion<E>(",
    ] {
        assert!(
            motion_source.contains(needle),
            "Separator motion should keep explicit platform cfg boundary via `{needle}`."
        );
    }

    for forbidden in ["web_sys", "wasm_bindgen", "window.", "document."] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "Separator non-motion component files should stay free of browser type leakage `{forbidden}`."
        );
    }

    let non_wasm_section = motion_source
        .split("#[cfg(not(target_arch = \"wasm32\"))]")
        .nth(1)
        .unwrap_or_default();
    for forbidden in ["web_sys", "wasm_bindgen", "HtmlElement"] {
        assert!(
            !non_wasm_section.contains(forbidden),
            "Separator non-wasm motion branch should avoid browser-only type `{forbidden}`."
        );
    }

    assert!(
        headless_lib_source
            .contains("#[cfg(all(feature = \"web\", feature = \"ssr\"))]\ncompile_error!"),
        "ui-headless should keep web/ssr compile_error mutex guard."
    );

    for needle in [
        "cargo check -p ui-components --no-default-features --features component-separator,inject-css",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-separator,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "SEPARATOR_NATIVE_CHECK=pass",
        "SEPARATOR_WASM_CHECK=pass",
        "HEADLESS_SSR_CHECK=pass",
    ] {
        assert!(
            check2_source.contains(needle),
            "Separator check2 should preserve cross-platform compile evidence `{needle}`."
        );
    }
}

#[test]
fn separator_headless_web_ssr_mutex_contract_is_preserved() {
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let check2_source = load_source("src/separator/check2.md");

    assert!(
        headless_lib_source
            .contains("#[cfg(all(feature = \"web\", feature = \"ssr\"))]\ncompile_error!"),
        "ui-headless should keep web/ssr compile_error mutex guard."
    );

    for needle in [
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "HEADLESS_WEB_CHECK=pass",
        "HEADLESS_SSR_CHECK=pass",
        "HEADLESS_WEB_SSR_MUTEX=pass",
    ] {
        assert!(
            check2_source.contains(needle),
            "Separator check2 should preserve ui-headless mutex evidence `{needle}`."
        );
    }
}

#[test]
fn separator_ui_motion_non_wasm_stub_contract_is_preserved() {
    let ui_motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let separator_motion_source = load_source("src/separator/motion.rs");
    let check2_source = load_source("src/separator/check2.md");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web;",
        "pub mod spring;",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion should keep non-wasm stub compatibility contract `{needle}`."
        );
    }

    assert!(
        separator_motion_source.contains("#[cfg(not(target_arch = \"wasm32\"))]"),
        "Separator motion should preserve non-wasm safe degrade branch."
    );

    for needle in [
        "cargo test -p ui-motion --test non_wasm_stub",
        "cargo test -p ui-components --test separator_semantics separator_motion_stays_ui_motion_driven_and_semantic_free --no-default-features --features component-separator,inject-css",
        "UI_MOTION_NON_WASM_STUB=pass",
        "SEPARATOR_MOTION_NON_WASM_CONTRACT=pass",
    ] {
        assert!(
            check2_source.contains(needle),
            "Separator check2 should preserve ui-motion non-wasm evidence `{needle}`."
        );
    }
}

#[test]
fn separator_reduced_motion_ssr_wasm_contract_is_preserved() {
    let separator_motion_source = load_source("src/separator/motion.rs");
    let check2_source = load_source("src/separator/check2.md");

    for needle in [
        "ui_motion::web::prefers_reduced_motion()",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            separator_motion_source.contains(needle),
            "Separator motion should keep reduced-motion/ssr/wasm branch contract `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui-components --no-default-features --features component-separator,inject-css",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-separator,inject-css",
        "cargo test -p ui-components --test separator_semantics separator_motion_stays_ui_motion_driven_and_semantic_free --no-default-features --features component-separator,inject-css",
        "SEPARATOR_NATIVE_CHECK=pass",
        "SEPARATOR_WASM_CHECK=pass",
        "SEPARATOR_REDUCED_MOTION_CONTRACT=pass",
    ] {
        assert!(
            check2_source.contains(needle),
            "Separator check2 should preserve reduced-motion/ssr/wasm evidence `{needle}`."
        );
    }
}

#[test]
fn separator_wasm_debug_capability_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let check2_source = load_source("src/separator/check2.md");
    let view_source = load_source("src/separator/view.rs");
    let logic_source = load_source("src/separator/logic.rs");
    let motion_source = load_source("src/separator/motion.rs");
    let styles_source = load_source("src/separator/styles.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "#[cfg(target_arch = \"wasm32\")]\nmod observability;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui-components should keep wasm debug capability isolated via `{needle}`."
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components Cargo features should keep explicit wasm-debug opt-in marker `{needle}`."
        );
    }
    assert!(
        !cargo_source.contains("separator-wasm-debug"),
        "Separator should not expose dedicated wasm-debug feature; debug runtime stays global and opt-in."
    );

    for marker in [
        "data-state=state.state_attr",
        "data-state-source=state.state_source_attr",
        "data-orientation=state.orientation_attr",
        "data-element=state.element_attr",
        "data-motion-source=motion_source",
    ] {
        assert!(
            view_source.contains(marker),
            "Separator should expose stable state/source marker `{marker}` for wasm debug attribution."
        );
    }

    let combined = format!("{view_source}\n{logic_source}\n{motion_source}\n{styles_source}");
    for forbidden in [
        "wasm_debug_proxy!",
        "observability::",
        "trace.emit(",
        "request_replay.run(",
        "data-slot=\"button-debug-replay\"",
        "#[prop(optional)] debug",
        "data-debug-source",
        "data-debug-before",
        "data-debug-after",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Separator production contract should not leak wasm-debug internals `{forbidden}`."
        );
    }

    for forbidden in ["on:click", "on:keydown", "on:pointerdown", "on:pointerup"] {
        assert!(
            !view_source.contains(forbidden),
            "Separator has no interactive replay path; token `{forbidden}` should stay absent."
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
            "docs debug visual entry should keep `{needle}`."
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
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
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "ts_ms: now_ms(),",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace contract should keep timestamped/source event marker `{needle}`."
        );
    }

    for needle in [
        "WASM 调试要求：关键状态可追踪",
        "开发模式下至少能追踪关键状态变更来源与前后值",
        "关键交互链路应支持最小可复现记录",
        "调试开关默认不进入生产包体与公共 API",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features inject-css,button-wasm-debug",
        "cargo test -p ui-components --test separator_semantics --no-default-features --features component-separator,inject-css separator_wasm_debug_capability_reuses_global_trace_and_stays_feature_isolated",
        "SEPARATOR_WASM_DEBUG=pass",
    ] {
        assert!(
            check2_source.contains(needle),
            "Separator checklist should keep wasm-debug governance marker `{needle}`."
        );
    }
}

#[test]
fn separator_performance_governance_contract_is_mount_only_traceable_and_blocking() {
    let check2_source = load_source("src/separator/check2.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_source("../../crates/ui-headless/src/perf.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let view_source = load_source("src/separator/view.rs");
    let motion_source = load_source("src/separator/motion.rs");

    for needle in [
        "- [x] 性能治理：关键路径有预算",
        "N/A：`Separator` 为静态分隔原语，无交互状态机与异步更新链路",
        "render_count",
        "等价证据",
        "separator_performance_governance_contract_is_mount_only_traceable_and_blocking",
        "SEPARATOR_PERF_GOVERNANCE=pass",
    ] {
        assert!(
            check2_source.contains(needle),
            "Separator checklist should keep performance governance evidence token `{needle}`."
        );
    }

    assert!(
        pages_source.contains(
            "component_doc!(\"Separator\", \"separator\", \"Layout\", layout::separator)"
        ),
        "Separator docs page should stay in components coverage traversal."
    );

    for needle in [
        "_ => UiPerfBudget::mount_only(120.0),",
        "let perf_budget = component_page_perf_budget(slug);",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "Docs shell should keep mount-only fallback/perf probe wiring via `{needle}`."
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "\"mount-plus-budget\"",
        "\"mount-only\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose stable perf observability marker `{needle}`."
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
            "Docs coverage e2e should keep blocking perf assertion `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "Perf governance should keep explicit render_count follow-up marker `{needle}`."
        );
    }

    for needle in [
        "data-state=state.state_attr",
        "data-state-source=state.state_source_attr",
        "data-orientation=state.orientation_attr",
        "data-element=state.element_attr",
        "data-motion-source=motion_source",
    ] {
        assert!(
            view_source.contains(needle),
            "Separator should expose state attribution marker `{needle}` for perf triage."
        );
    }

    for forbidden in [
        "Signal::derive",
        "Memo::new",
        "create_effect(",
        "spawn_local",
    ] {
        assert!(
            !view_source.contains(forbidden) && !motion_source.contains(forbidden),
            "Separator should avoid local reactive loop token `{forbidden}` to keep update budget predictable."
        );
    }

    let motion_effect_count = motion_source.matches("Effect::new").count();
    assert!(
        motion_effect_count <= 1,
        "Separator motion reactive budget exceeded: expected <= 1 `Effect::new`, found {motion_effect_count}."
    );
}

#[test]
fn separator_ui_components_layer_stays_assembly_only_and_platform_agnostic() {
    let mod_source = load_source("src/separator/mod.rs");
    let logic_source = load_source("src/separator/logic.rs");
    let view_source = load_source("src/separator/view.rs");
    let motion_source = load_source("src/separator/motion.rs");
    let styles_source = load_source("src/separator/styles.rs");

    for needle in [
        "pub use logic::{SeparatorElementType, SeparatorOrientation};",
        "pub use motion::SeparatorMotion;",
        "pub use view::Separator;",
    ] {
        assert!(
            mod_source.contains(needle),
            "separator public API surface should expose `{needle}`."
        );
    }

    for forbidden in ["pub mod logic", "pub mod view", "web_sys", "wasm_bindgen"] {
        assert!(
            !mod_source.contains(forbidden),
            "separator module boundary should not leak `{forbidden}`."
        );
    }
    assert!(
        !mod_source.contains("pub use styles::"),
        "separator module should not expose styles as public API surface."
    );

    assert!(
        logic_source.contains("pub use ui_state_primitives::separator::{"),
        "separator logic should only assemble by consuming ui-state-primitives."
    );
    assert!(
        view_source.contains("use ui_headless::{A11yDirection, SeparatorOptions, use_separator};"),
        "separator view should mount ui-headless contract instead of rewriting semantics."
    );
    assert!(
        view_source
            .contains("let separator_a11y = use_separator(SeparatorOptions { state, lang, dir });"),
        "separator view should wire headless attrs/handlers/state from use_separator."
    );
    assert!(
        motion_source.contains("ui_motion::spring::SpringAnimator"),
        "separator motion should delegate runtime animation execution to ui-motion."
    );
    assert!(
        styles_source.contains("var(--ui-"),
        "separator styles should stay token-first and consume theme css variables."
    );
}

#[test]
fn separator_api_naming_contract_uses_is_prefix_for_boolean_props() {
    let view_source = load_source("src/separator/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    assert!(
        view_source.contains("#[prop(optional)] is_decorative: Option<bool>"),
        "Separator public boolean prop should use `is_*` prefix (`is_decorative`)."
    );
    assert!(
        !view_source.contains("#[prop(optional)] decorative: bool"),
        "Separator should not keep legacy boolean prop name `decorative`."
    );
    assert!(
        docs_source.contains("<Separator is_decorative=true"),
        "Docs should use `is_decorative` naming path consistently."
    );
    assert!(
        !docs_source.contains("<Separator decorative=true"),
        "Docs should not drift to legacy alias naming."
    );
}

#[test]
fn separator_default_values_are_single_sourced_in_logic() {
    let view_source = load_source("src/separator/view.rs");
    let logic_source = load_source("src/separator/logic.rs");

    for needle in [
        "pub fn normalize_orientation(value: Option<SeparatorOrientation>)",
        "pub fn normalize_is_decorative(value: Option<bool>)",
        "pub fn normalize_element_type(value: Option<SeparatorElementType>)",
        "pub fn normalize_props(input: SeparatorNormalizeInput)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Separator logic should centralize defaults via `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] orientation: Option<SeparatorOrientation>",
        "#[prop(optional)] is_decorative: Option<bool>",
        "#[prop(optional)] element_type: Option<SeparatorElementType>",
        "let normalized = logic::normalize_props(SeparatorNormalizeInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "Separator view should forward raw inputs to logic normalization via `{needle}`."
        );
    }

    for forbidden in ["unwrap_or(", "SeparatorStateInput {"] {
        assert!(
            !view_source.contains(forbidden),
            "Separator view should not apply local default/state-input branching `{forbidden}`."
        );
    }
}

#[test]
fn separator_state_normalization_is_centralized_in_logic() {
    let view_source = load_source("src/separator/view.rs");
    let logic_source = load_source("src/separator/logic.rs");

    assert!(
        view_source.contains("let state = logic::resolve_state(normalized.state_input);"),
        "Separator view should consume normalized state_input from logic."
    );
    assert!(
        !view_source.contains("logic::resolve_state(SeparatorStateInput {"),
        "Separator view should not rebuild state_input rules inline."
    );
    assert!(
        logic_source.contains("SeparatorNormalizedProps"),
        "Separator logic should expose a typed normalized output contract."
    );
}

#[test]
fn separator_discrete_state_axes_are_type_constrained_with_enums() {
    let view_source = load_source("src/separator/view.rs");
    let logic_source = load_source("src/separator/logic.rs");

    for needle in [
        "Option<SeparatorOrientation>",
        "Option<SeparatorElementType>",
        "normalize_orientation(value: Option<SeparatorOrientation>)",
        "normalize_element_type(value: Option<SeparatorElementType>)",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "Separator should use typed enum axis contract `{needle}`."
        );
    }

    for forbidden in [
        "orientation: Option<String>",
        "element_type: Option<String>",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Separator should not use free-form string axis `{forbidden}`."
        );
    }
}

#[test]
fn separator_state_primitives_source_is_ui_state_primitives_only() {
    let logic_source = load_source("src/separator/logic.rs");
    let view_source = load_source("src/separator/view.rs");

    assert!(
        logic_source.contains("pub use ui_state_primitives::separator::{"),
        "Separator logic should source state primitives from ui-state-primitives."
    );
    for forbidden in ["RwSignal", "Store", "use_context", "create_resource"] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Separator should not bind component state directly to business/global store path `{forbidden}`."
        );
    }
}

#[test]
fn separator_async_contract_is_explicitly_na_without_loading_protocol() {
    let check2_source = load_source("src/separator/check2.md");
    let view_source = load_source("src/separator/view.rs");
    let logic_source = load_source("src/separator/logic.rs");

    assert!(
        check2_source.contains("N/A：`Separator` 无远程请求与异步状态"),
        "Separator check2 should explicitly document async N/A reason."
    );

    for forbidden in [
        "is_loading",
        "aria-busy",
        "retry",
        "use_async_action",
        "async fn",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Separator should not implement ad-hoc async protocol token `{forbidden}`."
        );
    }
}

#[test]
fn separator_controlled_uncontrolled_triplet_is_na_without_state_axis() {
    let view_source = load_source("src/separator/view.rs");
    let logic_source = load_source("src/separator/logic.rs");

    for forbidden in [
        "use_controllable_state(",
        "use_controllable_open_state_traced(",
        "#[prop(optional)] value:",
        "#[prop(optional)] default_value:",
        "#[prop(optional)] on_value_change:",
        "on_open_change",
        "default_open",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Separator should not expose half-controlled API fragment `{forbidden}` when no controllable axis exists."
        );
    }

    assert!(
        logic_source.contains("pub use ui_state_primitives::separator::{"),
        "Separator logic should stay assembly-only without local controlled/uncontrolled state machine."
    );
}

#[test]
fn separator_dx_paradox_keeps_basic_usage_simple() {
    let view_source = load_source("src/separator/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    assert!(
        view_source.contains("pub fn Separator("),
        "Separator should expose a direct component API surface."
    );
    for needle in [
        "#[prop(optional)] orientation: Option<SeparatorOrientation>",
        "#[prop(optional)] is_decorative: Option<bool>",
        "#[prop(optional)] element_type: Option<SeparatorElementType>",
    ] {
        assert!(
            view_source.contains(needle),
            "Separator should keep simple optional props for default usage path: `{needle}`."
        );
    }
    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(optional)] machine:",
        "#[prop(optional)] on_state_change:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Separator should not require internal state objects in public API `{forbidden}`."
        );
    }

    let separator_start = docs_source
        .find("pub(super) fn separator()")
        .expect("separator docs section should exist");
    let spacer_start = docs_source[separator_start..]
        .find("pub(super) fn spacer()")
        .map(|offset| separator_start + offset)
        .expect("separator docs section should have a bounded end");
    let separator_docs = &docs_source[separator_start..spacer_start];

    assert!(
        separator_docs.contains("r#\"<Separator />\n<Separator element_type=SeparatorElementType::Hr />\n<Separator orientation=SeparatorOrientation::Vertical class_name=\"docs-separator-rail\".to_string() />\"#.to_string()"),
        "Separator docs should provide a copy-ready hello-world-first path within 5 lines."
    );
    for forbidden in ["state=", "ui_state_primitives", "ui_headless::"] {
        assert!(
            !separator_docs.contains(forbidden),
            "Separator docs basic path should not require internal wiring token `{forbidden}`."
        );
    }
}

#[test]
fn separator_composition_api_rule_is_explicitly_na_for_non_composite_component() {
    let check2_source = load_source("src/separator/check2.md");
    let view_source = load_source("src/separator/view.rs");

    assert!(
        check2_source.contains("N/A：`Separator` 为单节点分隔原语，不存在 Parent/Item 组合语义"),
        "Separator check2 should explicitly document why composition Parent/Item rule is N/A."
    );

    for forbidden in [
        "children: Children",
        "labels",
        "titles",
        "panels",
        "ItemSpec",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Separator should not expose composite-array or implicit-pairing API token `{forbidden}`."
        );
    }
}

#[test]
fn separator_a11y_i18n_l10n_contract_is_headless_backed_and_locale_ready() {
    let view_source = load_source("src/separator/view.rs");
    let headless_source = load_source("../ui-headless/src/separator.rs");

    for needle in [
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let separator_a11y = use_separator(SeparatorOptions { state, lang, dir });",
        "let role = separator_a11y.attrs.role;",
        "let aria_orientation = separator_a11y.attrs.aria_orientation;",
        "let aria_hidden = separator_a11y.attrs.aria_hidden;",
        "let lang = separator_a11y.attrs.lang;",
        "let dir = separator_a11y.attrs.dir;",
        "role=role",
        "aria-orientation=aria_orientation",
        "aria-hidden=aria_hidden",
        "lang=lang",
        "dir=dir",
    ] {
        assert!(
            view_source.contains(needle),
            "Separator view should mount headless/localized a11y contract token `{needle}`."
        );
    }

    for needle in [
        "use crate::a11y::{A11yDirection, locale_attrs};",
        "let locale = locale_attrs(lang, dir);",
        "pub lang: Option<String>",
        "pub dir: Option<&'static str>",
    ] {
        assert!(
            headless_source.contains(needle),
            "Separator headless contract should source shared a11y locale helpers via `{needle}`."
        );
    }

    for forbidden in [
        "fn locale_attrs(",
        "\"Above\"",
        "\"Below\"",
        "\"Left\"",
        "\"Right\"",
    ] {
        assert!(
            !view_source.contains(forbidden) && !headless_source.contains(forbidden),
            "Separator should not hardcode user-facing copy or duplicate shared a11y helpers `{forbidden}`."
        );
    }

    for forbidden in ["on:click", "on:keydown", "tabindex"] {
        assert!(
            !view_source.contains(forbidden),
            "Separator is non-interactive; keyboard handler assumptions should not appear via `{forbidden}`."
        );
    }
}

#[test]
fn separator_semantics_tests_prioritize_contract_over_snapshots() {
    let tests_source = load_source("tests/separator_semantics.rs");
    let check2_source = load_source("src/separator/check2.md");
    let tests_source_before_guard = tests_source
        .split("fn separator_semantics_tests_prioritize_contract_over_snapshots()")
        .next()
        .unwrap_or(&tests_source);

    for needle in [
        "fn separator_emits_baseline_style_state_data_attributes()",
        "fn separator_state_markers_are_observable_searchable_and_enumerated()",
        "fn separator_a11y_i18n_l10n_contract_is_headless_backed_and_locale_ready()",
        "fn separator_controlled_uncontrolled_triplet_is_na_without_state_axis()",
        "fn separator_motion_stays_ui_motion_driven_and_semantic_free()",
    ] {
        assert!(
            tests_source.contains(needle),
            "Separator semantics suite should keep contract assertions via `{needle}`."
        );
    }

    for needle in [
        "N/A：`Separator` 无可控状态轴，不存在受控/非受控与 disabled 交互分支",
        "N/A：`Separator` 非交互原语，无 keyboard/pointer 路径",
    ] {
        assert!(
            check2_source.contains(needle),
            "Separator check2 should explicitly record matrix applicability via `{needle}`."
        );
    }

    for forbidden in ["assert_snapshot!", "insta::", "to_match_snapshot"] {
        assert!(
            !tests_source_before_guard.contains(forbidden),
            "Separator semantics should not rely on visual snapshot assertion `{forbidden}`."
        );
    }
}

#[test]
fn separator_component_files_follow_responsibility_boundaries() {
    let lib_source = load_source("src/lib.rs");
    let mod_source = load_source("src/separator/mod.rs");
    let logic_source = load_source("src/separator/logic.rs");
    let styles_source = load_source("src/separator/styles.rs");
    let view_source = load_source("src/separator/view.rs");
    let motion_source = load_source("src/separator/motion.rs");

    for needle in [
        "#[cfg(feature = \"component-separator\")]",
        "pub mod separator;",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should gate separator module via `{needle}`."
        );
    }

    for needle in [
        "mod logic;",
        "pub(crate) mod motion;",
        "pub(crate) mod styles;",
        "mod view;",
        "pub use logic::{SeparatorElementType, SeparatorOrientation};",
        "pub use motion::SeparatorMotion;",
        "pub use view::Separator;",
    ] {
        assert!(
            mod_source.contains(needle),
            "separator mod.rs should keep minimal boundary/export via `{needle}`."
        );
    }
    for forbidden in ["pub mod logic", "pub mod view", "view!", "SpringAnimator"] {
        assert!(
            !mod_source.contains(forbidden),
            "separator mod.rs should avoid implementation leakage `{forbidden}`."
        );
    }

    for needle in [
        "SeparatorNormalizeInput",
        "SeparatorNormalizedProps",
        "normalize_props",
        "resolve_state",
    ] {
        assert!(
            logic_source.contains(needle),
            "separator logic.rs should focus on normalization/derivation `{needle}`."
        );
    }
    for forbidden in ["view!", "NodeRef<", "data-state=", "style=", "--ui-"] {
        assert!(
            !logic_source.contains(forbidden),
            "separator logic.rs should not contain DOM/style concerns `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str"),
        "separator styles.rs should expose static token-first CSS contract."
    );
    assert!(
        styles_source.contains("var(--ui-"),
        "separator styles.rs should consume theme tokens via css variables."
    );
    for forbidden in [
        "#[component]",
        "use ui_headless",
        "SpringAnimator",
        "on:click",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "separator styles.rs should not contain view/headless/motion runtime code `{forbidden}`."
        );
    }

    for needle in [
        "view! {",
        "use_separator(SeparatorOptions { state, lang, dir })",
    ] {
        assert!(
            view_source.contains(needle),
            "separator view.rs should render structure and mount headless contract via `{needle}`."
        );
    }
    for forbidden in [
        "ui_motion::spring::SpringAnimator",
        "spring_fast()",
        "prefers_reduced_motion()",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "separator view.rs should not reimplement motion driver `{forbidden}`."
        );
    }

    for needle in [
        "pub fn attach_motion<E>(",
        "ui_motion::spring::SpringAnimator",
        "ui_motion::presets::spring_fast()",
    ] {
        assert!(
            motion_source.contains(needle),
            "separator motion.rs should map semantics to ui-motion contract via `{needle}`."
        );
    }
    for forbidden in ["view! {", "use_separator(", "data-state=", "role=", "aria-"] {
        assert!(
            !motion_source.contains(forbidden),
            "separator motion.rs should not contain view/headless semantic markup `{forbidden}`."
        );
    }
}

#[test]
fn separator_view_macro_complexity_is_bounded() {
    let view_source = load_source("src/separator/view.rs");

    let view_macro_count = view_source.match_indices("view! {").count();
    assert_eq!(
        view_macro_count, 2,
        "Separator view should keep exactly two small semantic view! branches (hr/div)."
    );

    for block in view_source.split("view! {").skip(1) {
        let line_count_before_into_any = block
            .lines()
            .take_while(|line| !line.contains(".into_any()"))
            .count();
        assert!(
            line_count_before_into_any <= 40,
            "Separator view! branch should stay bounded; found {line_count_before_into_any} lines before `.into_any()`."
        );
    }
}

#[test]
fn separator_prefers_functional_split_without_component_overgrowth() {
    let view_source = load_source("src/separator/view.rs");

    assert_eq!(
        view_source.match_indices("#[component]").count(),
        1,
        "Separator should keep a single public component entry without local component overgrowth."
    );
    assert!(
        !view_source.contains("pub fn SeparatorItem(")
            && !view_source.contains("pub fn SeparatorPart("),
        "Separator should avoid introducing unnecessary nested component abstractions."
    );
}

#[test]
fn separator_static_fragment_constantization_is_not_applicable_for_simple_node_rendering() {
    let view_source = load_source("src/separator/view.rs");
    let check2_source = load_source("src/separator/check2.md");

    for forbidden in ["<svg", "<footer", "inner_html", "Lorem ipsum"] {
        assert!(
            !view_source.contains(forbidden),
            "Separator should not contain heavy static fragment token `{forbidden}`."
        );
    }
    assert!(
        check2_source.contains("N/A：`Separator` 不包含复杂 SVG/长文案/静态模板块"),
        "Separator check2 should explicitly record static-fragment constantization N/A reason."
    );
}

#[test]
fn separator_inner_html_usage_is_forbidden_in_component_implementation() {
    let check2_source = load_source("src/separator/check2.md");
    let view_source = load_source("src/separator/view.rs");
    let logic_source = load_source("src/separator/logic.rs");
    let motion_source = load_source("src/separator/motion.rs");
    let styles_source = load_source("src/separator/styles.rs");

    assert!(
        check2_source.contains("N/A：`Separator` 组件实现不使用 `inner_html`。"),
        "Separator check2 should explicitly record inner_html N/A reason."
    );
    for source in [&view_source, &logic_source, &motion_source, &styles_source] {
        assert!(
            !source.contains("inner_html"),
            "Separator implementation should not use inner_html."
        );
    }
}

#[test]
fn separator_spec_file_policy_remains_minimal_for_simple_component() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/separator/spec.rs");
    let mod_source = load_source("src/separator/mod.rs");
    let check2_source = load_source("src/separator/check2.md");

    assert!(
        !spec_path.exists(),
        "Separator should not introduce `spec.rs` for simple component contracts."
    );
    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "Separator module boundary should not expose spec module `{forbidden}`."
        );
    }
    assert!(
        check2_source
            .contains("N/A：`Separator` 为简单分隔原语，不存在独立 Schema/版本迁移契约需求"),
        "Separator check2 should explicitly document why spec.rs stays absent."
    );
}

#[test]
fn separator_does_not_ignore_motion_contract() {
    let source = load_source("src/separator/view.rs");

    assert!(
        !source.contains("let _ = motion"),
        "Separator should honor `SeparatorMotion` rather than ignoring it."
    );
}

#[test]
fn separator_attaches_motion_driver() {
    let source = load_source("src/separator/view.rs");

    assert!(
        source.contains("attach_motion"),
        "Separator should attach its motion driver when `SeparatorMotion` requests animation."
    );
}

#[test]
fn separator_styles_use_only_css_variables_for_motion() {
    let source = load_source("src/separator/styles.rs");

    for name in [
        "--ui-separator-scale-x",
        "--ui-separator-scale-y",
        "--ui-separator-opacity",
    ] {
        assert!(
            source.contains(name),
            "Separator styles should define `{name}` so motion updates only touch CSS variables."
        );
    }
}

#[test]
fn separator_motion_uses_spring_animator() {
    let source = load_source("src/separator/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Separator motion should animate via a spring to match the repo's motion spec."
    );
}

#[test]
fn separator_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/separator/motion.rs");
    let view_source = load_source("src/separator/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: SeparatorMotion) -> SeparatorMotion",
        "fn sanitize_motion_keeps_explicit_entry_flag()",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "Separator motion should include `{needle}` so runtime motion contracts stay sanitized across SSR/wasm boundaries.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::separator::motion::sanitize_motion(motion);"),
        "Separator view should sanitize motion before deriving motion source markers and attaching the driver.",
    );
}

#[test]
fn separator_motion_stays_ui_motion_driven_and_semantic_free() {
    let motion_source = load_source("src/separator/motion.rs");

    for needle in [
        "ui_motion::spring::SpringAnimator",
        "ui_motion::presets::spring_fast()",
        "ui_motion::web::prefers_reduced_motion()",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion<E>(",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "Separator motion should keep ui-motion/non-wasm-stub contract `{needle}`."
        );
    }

    for forbidden in ["data-slot=", "role=", "aria-"] {
        assert!(
            !motion_source.contains(forbidden),
            "Separator motion should not mix semantic/view-layer logic `{forbidden}`."
        );
    }
}

#[test]
fn separator_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn separator() -> AnyView",
        "title=\"Separator\"",
        "slug=\"separator\"",
        "Playground title=\"Semantic + Element Type\"",
        "Playground title=\"Decorative + Custom Class\"",
    ] {
        assert!(
            source.contains(needle),
            "layout docs page should contain `{needle}` for Separator.",
        );
    }
}

#[test]
fn separator_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"Semantic + Element Type\"",
        "<Separator />",
        "element_type=SeparatorElementType::Hr",
        "orientation=SeparatorOrientation::Vertical",
        "class_name=\"docs-separator-rail\".to_string()",
        "title=\"Decorative + Custom Class\"",
        "<Separator is_decorative=true class_name=\"docs-separator-custom\".to_string() />",
        "class_name=\"docs-separator-rail docs-separator-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "separator docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn separator_docs_api_and_default_contract_matches_logic_normalization_rules() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let logic_source = load_source("src/separator/logic.rs");
    let view_source = load_source("src/separator/view.rs");

    assert!(
        docs_source
            .contains("r#\"<Separator />\n<Separator element_type=SeparatorElementType::Hr />"),
        "Separator docs should keep a default-call example plus typed element override."
    );
    assert!(
        docs_source.contains("<Separator is_decorative=true"),
        "Separator docs should use current boolean API name `is_decorative`."
    );
    assert!(
        !docs_source.contains("<Separator decorative=true"),
        "Separator docs should not drift back to legacy boolean prop name `decorative`."
    );

    for needle in [
        "pub fn normalize_orientation(value: Option<SeparatorOrientation>) -> SeparatorOrientation",
        "pub fn normalize_is_decorative(value: Option<bool>) -> bool",
        "pub fn normalize_element_type(value: Option<SeparatorElementType>) -> SeparatorElementType",
        "value.unwrap_or_default()",
        "value.unwrap_or(false)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Separator logic should keep explicit default normalization contract `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] orientation: Option<SeparatorOrientation>",
        "#[prop(optional)] is_decorative: Option<bool>",
        "#[prop(optional)] element_type: Option<SeparatorElementType>",
        "let normalized = logic::normalize_props(SeparatorNormalizeInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "Separator view/docs API contract should flow through logic normalization via `{needle}`."
        );
    }
}

#[test]
fn separator_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
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
        "pub(super) fn separator() -> AnyView",
        "<Playground title=\"Semantic + Element Type\" code_signal=semantic_code>",
        "<Playground title=\"Decorative + Custom Class\" code_signal=decorative_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "Separator docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
fn separator_dx_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let check2_source = load_source("src/separator/check2.md");

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

    for needle in [
        "<span>\"Above\"</span>",
        "<span>\"Below\"</span>",
        "<span>\"Left\"</span>",
        "<span>\"Right\"</span>",
        "<span>\"Start\"</span>",
        "<span>\"End\"</span>",
    ] {
        assert!(
            docs_source.contains(needle),
            "Separator docs should keep context-visible marker `{needle}`."
        );
    }

    for forbidden in [
        "SEPARATOR_WORKBENCH_STORAGE_KEY",
        "load_separator_workbench_state(",
        "save_separator_workbench_state(",
        "clear_separator_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Separator keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
        );
    }

    for required in [
        "DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
        "separator_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "separator_dx_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na",
        "SEPARATOR_DX_HOT_RELOAD=pass",
    ] {
        assert!(
            check2_source.contains(required),
            "Separator checklist should keep DX governance rule `{required}`."
        );
    }
}

#[test]
fn separator_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/separator/mod.rs");
    let logic_source = load_source("src/separator/logic.rs");
    let view_source = load_source("src/separator/view.rs");
    let styles_source = load_source("src/separator/styles.rs");
    let motion_source = load_source("src/separator/motion.rs");
    let checklist_source = load_source("src/separator/check2.md");

    assert!(
        !manifest_dir.join("src/separator/spec.rs").exists(),
        "Separator should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-separator = []"),
        "Separator feature should stay lightweight without serde/spec dependency fan-out."
    );
    assert!(
        !cargo_source.contains("component-separator = [\"dep:serde\"")
            && !cargo_source.contains("component-separator = [\"dep:serde_json\""),
        "Separator should not opt into serde/spec migration dependencies without an explicit schema contract."
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Separator engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
        "SEPARATOR_ENGINEERING_CONTRACT=pass",
    ] {
        assert!(
            checklist_source.contains(required),
            "Separator checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
fn separator_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let combined = [
        load_source("src/separator/mod.rs"),
        load_source("src/separator/logic.rs"),
        load_source("src/separator/view.rs"),
        load_source("src/separator/styles.rs"),
        load_source("src/separator/motion.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("separator-wasm-debug"),
        "Separator should not define component-local tracing feature when no local debug event/replay contract exists."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::separator::",
        "const SEPARATOR_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Separator should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn separator_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/separator/mod.rs");
    let logic_source = load_source("src/separator/logic.rs");
    let view_source = load_source("src/separator/view.rs");
    let styles_source = load_source("src/separator/styles.rs");
    let motion_source = load_source("src/separator/motion.rs");

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
                "Separator engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "Separator public module boundary should not leak web_sys types."
    );
}

#[test]
fn separator_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("src/active_highlight.rs");
    let headless_controllable = load_source("../../crates/ui-headless/src/controllable_state.rs");
    let headless_presence = load_source("../../crates/ui-headless/src/presence.rs");
    let headless_a11y = load_source("../../crates/ui-headless/src/a11y.rs");
    let checklist_source = load_source("src/separator/check2.md");

    for needle in [
        "#[cfg(feature = \"component-separator\")]\npub mod separator;",
        "pub use root::UiRoot;",
        "pub use separator::{Separator, SeparatorElementType, SeparatorMotion, SeparatorOrientation};",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep stable export/gate marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod overlay_open;",
        "pub mod presence;",
        "pub mod a11y;",
        "pub use leptos::web_sys",
        "pub use web_sys::",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-components lib entry should not expose internal platform/details marker `{forbidden}`."
        );
    }

    for needle in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-separator\")]\n    out.push_str(crate::separator::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]\npub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "css entry should keep feature-gated component aggregation marker `{needle}`."
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot entry should keep centralized theme/i18n/css injection marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight entry should keep shared style/motion contract marker `{needle}`."
        );
    }

    for forbidden in ["#[component]", "pub fn Separator(", "ui-separator"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should remain generic shared utility, not component-business implementation: `{forbidden}`."
        );
    }

    assert!(
        manifest_dir.join("src/active_highlight.rs").exists(),
        "ui-components should keep shared `src/active_highlight.rs` entry."
    );
    assert!(
        !manifest_dir.join("src/overlay_open.rs").exists(),
        "ui-components should not define `src/overlay_open.rs`; open-state primitive belongs to ui-headless."
    );
    assert!(
        !manifest_dir.join("src/presence.rs").exists(),
        "ui-components should not define `src/presence.rs`; presence primitive belongs to ui-headless."
    );
    assert!(
        !manifest_dir.join("src/a11y.rs").exists(),
        "ui-components should not define `src/a11y.rs`; shared a11y helpers belong to ui-headless."
    );

    assert!(
        path_exists("../../crates/ui-headless/src/controllable_state.rs")
            && path_exists("../../crates/ui-headless/src/presence.rs")
            && path_exists("../../crates/ui-headless/src/a11y.rs"),
        "ui-headless canonical primitive files should exist."
    );

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
        "pub fn aria_controls_when_open(open: Signal<bool>, controls_id: String) -> Signal<Option<String>>",
    ] {
        assert!(
            headless_controllable.contains(needle)
                || headless_presence.contains(needle)
                || headless_a11y.contains(needle),
            "headless layer should keep canonical primitive entry marker `{needle}`."
        );
    }

    for required in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "`crates/ui-components/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui-components/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui-components/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-components/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui-components/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui-components/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui-components/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
        "SEPARATOR_ENTRYPOINT_LAYOUT=pass",
    ] {
        assert!(
            checklist_source.contains(required),
            "Separator checklist should keep fixed-entry governance rule `{required}`."
        );
    }
}

#[test]
fn separator_e2e_selectors_and_repeatable_regression_are_semantic_and_wasm_stable() {
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let separator_e2e_source = load_source("../../e2e/tests/docs_app_separator_contract.spec.mjs");

    assert!(
        pages_source.contains(
            "component_doc!(\"Separator\", \"separator\", \"Layout\", layout::separator),"
        ),
        "Separator should be indexed by docs-app components page."
    );

    for needle in [
        "test(\"docs-app separator uses semantic selectors with wasm-stable waits\"",
        "test(\"docs-app separator key flow is repeatable with semantic contract breakpoints\"",
        "await page.goto(\"/#/components/separator\");",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-component=\"separator\"] [data-slot=\"separator\"][data-state=\"semantic\"]",
        "data-state-source",
        "data-output-mode",
        "data-streaming-fallback",
        "data-output-status",
        "await page.goto(\"/#/components/spacer\");",
    ] {
        assert!(
            separator_e2e_source.contains(needle),
            "separator e2e contract should keep semantic, wasm-stable selector marker `{needle}`."
        );
    }

    for needle in [
        "await page.goto(`/#/components/${slug}`);",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "await expect(page.locator(\"section.playground\").first()).toBeVisible();",
        "await expect(page.locator(`[data-slot=\"${slug}\"]`).first()).toBeVisible();",
        "test(\"docs-app components pages render playgrounds (all)\"",
    ] {
        assert!(
            coverage_source.contains(needle),
            "docs coverage e2e should keep semantic, wasm-stable selector contract `{needle}`."
        );
    }

    assert!(
        !coverage_source.contains("waitForTimeout(")
            && !separator_e2e_source.contains("waitForTimeout("),
        "E2E contract should avoid fixed sleeps and rely on semantic ready conditions."
    );
}

#[test]
fn separator_check2_documents_e2e_selector_and_stable_wait_rules() {
    let checklist_source = load_source("src/separator/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Separator checklist should keep e2e selector/stable-wait rule `{required}`."
        );
    }
}

#[test]
fn separator_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_separator_contract.spec.mjs");

    for needle in [
        "/#/components/separator",
        "body:not(:has(#boot))",
        "[data-component=\"separator\"] [data-slot=\"separator\"][data-state=\"semantic\"]",
        "[data-component=\"separator\"] [data-slot=\"separator\"][data-state=\"decorative\"]",
        "toHaveAttribute(\"data-state-source\", \"props-static\")",
        "toHaveAttribute(\"data-output-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-streaming-fallback\", \"snapshot\")",
        "toHaveAttribute(\"data-output-status\", \"verified\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Separator e2e selector/stable-wait contract should include `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "Separator e2e contract should avoid unstable/non-semantic selector token `{forbidden}`."
        );
    }
}

#[test]
fn separator_check2_documents_e2e_repeatable_key_flow_rules() {
    let checklist_source = load_source("src/separator/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Separator checklist should keep repeatable-key-flow rule `{required}`."
        );
    }
}

#[test]
fn separator_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_separator_contract.spec.mjs");

    for needle in [
        "docs-app separator key flow is repeatable with semantic contract breakpoints",
        "await page.goto(\"/#/components/separator\");",
        "await expect(separatorNodes).toHaveCount(5);",
        "toHaveAttribute(\"data-state-source\", \"props-static\")",
        "await page.goto(\"/#/components/spacer\");",
        "await expect(page.locator(\".docs-page-title\")).toHaveText(\"Spacer\")",
        "await page.goto(\"/#/components/separator\");",
        "toHaveAttribute(\"data-output-status\", \"verified\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Separator e2e repeatable-flow contract should include `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "waitForTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "Separator e2e key flow should avoid non-semantic/flaky token `{forbidden}`."
        );
    }
}

#[test]
fn separator_e2e_high_risk_paths_are_explicitly_na_for_non_interactive_component() {
    let check2_source = load_source("src/separator/check2.md");
    let e2e_source = load_source("../../e2e/tests/docs_app_separator_contract.spec.mjs");

    assert!(
        check2_source.contains(
            "`Separator` 不涉及 overlay/focus/keyboard/async 高风险交互，交互链路为 N/A。"
        ) || check2_source
            .contains("`Separator` 不涉及 overlay/focus/async 高风险交互，交互链路为 N/A。"),
        "Separator check2 should explicitly document high-risk path N/A reason."
    );

    for forbidden in [
        "[data-slot=\"overlay\"]",
        "page.keyboard.press(",
        "aria-busy",
        "retry",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "Separator e2e should not claim unsupported high-risk path token `{forbidden}`."
        );
    }
}

#[test]
fn separator_e2e_check_script_covers_selector_and_key_flow_contracts() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-separator.sh");

    for needle in [
        "cargo test -p ui-components --test separator_semantics --no-default-features --features component-separator,inject-css separator_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-components --test separator_semantics --no-default-features --features component-separator,inject-css separator_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "cargo test -p ui-components --test separator_semantics --no-default-features --features component-separator,inject-css separator_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui-components --test separator_semantics --no-default-features --features component-separator,inject-css separator_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui-components --test separator_semantics --no-default-features --features component-separator,inject-css separator_e2e_high_risk_paths_are_explicitly_na_for_non_interactive_component",
    ] {
        assert!(
            script_source.contains(needle),
            "separator e2e check script should enforce `{needle}`."
        );
    }
}

#[test]
fn separator_docs_product_playground_and_copy_ready_contracts_are_present() {
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "component_doc!(\"Separator\", \"separator\", \"Layout\", layout::separator),",
        "pub(super) fn separator() -> AnyView",
        "r#\"<Separator />",
        "title=\"Semantic + Element Type\"",
        "title=\"Decorative + Custom Class\"",
        "description=\"Spring-enabled separator with centralized orientation/element/decorative state attrs.\"",
    ] {
        assert!(
            pages_source.contains(needle) || docs_source.contains(needle),
            "Separator docs contract should include `{needle}`."
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "CodeBlock code=resolved_code.get()",
        "{move || if show_code_panel.get() { \"Hide code\" } else { \"Show code\" }}",
        "data-slot=\"playground-code\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep source-first copy-ready path marker `{needle}`."
        );
    }
}

#[test]
fn separator_docs_product_is_beginner_friendly_hello_world_first_and_progressive() {
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    assert!(
        pages_source.contains(
            "component_doc!(\"Separator\", \"separator\", \"Layout\", layout::separator),"
        ),
        "Separator docs should be discoverable from docs-app navigation."
    );

    let separator_start = docs_source
        .find("pub(super) fn separator()")
        .expect("separator docs section should exist");
    let spacer_start = docs_source[separator_start..]
        .find("pub(super) fn spacer()")
        .map(|offset| separator_start + offset)
        .expect("separator docs section should have a bounded end");
    let separator_docs = &docs_source[separator_start..spacer_start];

    let semantic_title_index = separator_docs
        .find("Playground title=\"Semantic + Element Type\"")
        .expect("Separator docs should include semantic playground.");
    let decorative_title_index = separator_docs
        .find("Playground title=\"Decorative + Custom Class\"")
        .expect("Separator docs should include decorative playground.");
    assert!(
        semantic_title_index < decorative_title_index,
        "Separator docs should present default/semantic usage before advanced decorative examples."
    );

    let semantic_code_start = separator_docs
        .find("let semantic_code = Signal::derive")
        .expect("Separator docs should define semantic_code snippet.");
    let semantic_code_block = &separator_docs[semantic_code_start..];
    let literal_start = semantic_code_block
        .find("r#\"")
        .map(|idx| idx + 3)
        .expect("semantic snippet should start with raw string.");
    let literal_end = semantic_code_block
        .find("\"#.to_string()")
        .expect("semantic snippet should end with to_string.");
    let semantic_literal = &semantic_code_block[literal_start..literal_end];

    let snippet_lines: Vec<&str> = semantic_literal
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    assert!(
        !snippet_lines.is_empty(),
        "Separator docs semantic snippet should not be empty."
    );
    assert!(
        snippet_lines.first() == Some(&"<Separator />"),
        "Separator docs should start with zero-threshold hello world call `<Separator />`."
    );
    assert!(
        snippet_lines.len() <= 5,
        "Separator docs hello world path should remain concise (<=5 lines), got {} lines.",
        snippet_lines.len()
    );

    for token in [
        "element_type=SeparatorElementType::Hr",
        "orientation=SeparatorOrientation::Vertical",
        "is_decorative=true",
        "class_name=\"docs-separator-rail docs-separator-custom\".to_string()",
    ] {
        assert!(
            separator_docs.contains(token),
            "Separator docs should include progressive advanced token `{token}`."
        );
    }

    let default_call_index = separator_docs
        .find("<Separator />")
        .expect("Separator docs should include default call.");
    for advanced in [
        "element_type=SeparatorElementType::Hr",
        "orientation=SeparatorOrientation::Vertical",
        "is_decorative=true",
    ] {
        let advanced_index = separator_docs
            .find(advanced)
            .unwrap_or_else(|| panic!("Separator docs missing advanced token `{advanced}`."));
        assert!(
            default_call_index < advanced_index,
            "Separator docs should present default call before `{advanced}`."
        );
    }

    for forbidden in [
        "ui_state_primitives",
        "ui_headless::",
        "use_separator(",
        "state=",
        "machine=",
    ] {
        assert!(
            !separator_docs.contains(forbidden),
            "Separator docs should not force users to wire internal architecture token `{forbidden}`."
        );
    }
}

#[test]
fn separator_docs_interactive_playground_contract_is_editable_realtime_and_repeatable() {
    let check2_source = load_source("src/separator/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_separator_contract.spec.mjs");

    for needle in [
        "Playground title=\"Semantic + Element Type\" code_signal=semantic_code",
        "Playground title=\"Decorative + Custom Class\" code_signal=decorative_code",
        "<Separator element_type=SeparatorElementType::Hr />",
        "orientation=SeparatorOrientation::Vertical",
        "<Separator is_decorative=true class_name=\"docs-separator-custom\".to_string() />",
    ] {
        assert!(
            docs_source.contains(needle),
            "Separator docs should expose interactive state/prop matrix marker `{needle}`."
        );
    }

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "data-slot=\"playground-controls\"",
        "data-slot=\"playground-code\"",
        "data-slot=\"playground-test\"",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground infra should keep online-edit and realtime-preview marker `{needle}`."
        );
    }

    for needle in [
        "docs-app separator key flow is repeatable with semantic contract breakpoints",
        "await page.goto(\"/#/components/separator\");",
        "await page.goto(\"/#/components/spacer\");",
        "await expect(separatorNodes).toHaveCount(5);",
        "toHaveAttribute(\"data-state-source\", \"props-static\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Separator e2e should keep repeatable playground acceptance marker `{needle}`."
        );
    }

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
        "N/A：`Separator` 非 AI Spec 组件，不承载 Spec 输入与预览联动。",
        "separator_docs_interactive_playground_contract_is_editable_realtime_and_repeatable",
        "separator_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
    ] {
        assert!(
            check2_source.contains(required),
            "Separator check2 should preserve interactive-playground governance marker `{required}`."
        );
    }
}

#[test]
fn separator_source_first_docs_are_copy_paste_ready_with_imports_and_sync() {
    let check2_source = load_source("src/separator/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_source = load_source("src/code_block/view.rs");

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "fn missing_import_lines(raw: &str, imports: &str) -> Vec<String> {",
        "CodeBlock code=resolved_code.get()",
        "data-slot=\"playground-code\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground copy-ready pipeline should keep import/runtime marker `{needle}`."
        );
    }

    for needle in [
        "pub fn CodeBlock(",
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "on_press=copy_logic.copy",
        "data-slot=\"code-block\"",
    ] {
        assert!(
            code_block_source.contains(needle),
            "CodeBlock should keep visible copy action marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn separator() -> AnyView",
        "Playground title=\"Semantic + Element Type\" code_signal=semantic_code",
        "Playground title=\"Decorative + Custom Class\" code_signal=decorative_code",
        "<Separator is_decorative=true />",
    ] {
        assert!(
            docs_source.contains(needle),
            "Separator docs should stay synced with current API/demo marker `{needle}`."
        );
    }

    assert!(
        !docs_source.contains("<Separator decorative=true"),
        "Separator docs should not regress to legacy prop alias `decorative`."
    );

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
        "N/A：`Separator` 文档形态为 playground-first（非独立 source-first 页面），源码落点/依赖前提不作为单页强制项。",
        "separator_source_first_docs_are_copy_paste_ready_with_imports_and_sync",
        "separator_docs_product_playground_and_copy_ready_contracts_are_present",
    ] {
        assert!(
            check2_source.contains(required),
            "Separator check2 should preserve source-first/copy-ready governance marker `{required}`."
        );
    }
}

#[test]
fn separator_heroui_strategy_and_component_docs_stay_synced_for_current_parameter_model() {
    let check2_source = load_source("src/separator/check2.md");
    let view_source = load_source("src/separator/view.rs");
    let docs_pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_layout_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let research_source =
        load_source("../../docs/research/spectrum-heroui-style-interface-study.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        "N/A：本轮 `Separator` 未引入参数语义变更，`orientation/element_type/is_decorative` 语义保持稳定；对标文档无需增量改写。",
        "separator_heroui_strategy_and_component_docs_stay_synced_for_current_parameter_model",
    ] {
        assert!(
            check2_source.contains(required),
            "Separator check2 should preserve HeroUI docs-sync governance marker `{required}`."
        );
    }

    for marker in [
        "component_doc!(\"Separator\", \"separator\", \"Layout\", layout::separator),",
        "pub(super) fn separator() -> AnyView",
    ] {
        assert!(
            docs_pages_source.contains(marker) || docs_layout_source.contains(marker),
            "Separator docs entry should remain indexed and navigable via `{marker}`."
        );
    }

    for marker in [
        "#[prop(optional)] orientation: Option<SeparatorOrientation>",
        "#[prop(optional)] is_decorative: Option<bool>",
        "#[prop(optional)] element_type: Option<SeparatorElementType>",
    ] {
        assert!(
            view_source.contains(marker),
            "Separator current parameter model marker should remain stable: `{marker}`."
        );
    }

    for marker in [
        "element_type=SeparatorElementType::Hr",
        "orientation=SeparatorOrientation::Vertical",
        "is_decorative=true",
    ] {
        assert!(
            docs_layout_source.contains(marker),
            "Separator docs should stay aligned with current parameter semantics via `{marker}`."
        );
    }

    assert!(
        !docs_layout_source.contains("<Separator decorative=true"),
        "Separator docs should not drift to legacy boolean prop alias `decorative`."
    );

    for marker in [
        "参数语义若变更，必须先同步本策略文档与 docs 入口，不允许实现先漂移文档后补。",
        "参数语义变更需先同步本策略文档与 docs 入口。",
    ] {
        assert!(
            strategy_source.contains(marker),
            "HeroUI strategy doc should retain anti-drift governance marker `{marker}`."
        );
    }

    assert!(
        research_source.contains("Spectrum × HeroUI 样式与接口综合学习"),
        "HeroUI research companion doc should remain available when strategy doc references it."
    );
}

#[test]
fn separator_forbidden_antipatterns_checklist_items_are_individually_enforced() {
    let check2_source = load_source("src/separator/check2.md");
    let primitive_source = load_source("../ui-state-primitives/src/separator.rs");
    let headless_source = load_source("../ui-headless/src/separator.rs");
    let logic_source = load_source("src/separator/logic.rs");
    let view_source = load_source("src/separator/view.rs");
    let mod_source = load_source("src/separator/mod.rs");
    let styles_source = load_source("src/separator/styles.rs");
    let motion_source = load_source("src/separator/motion.rs");
    let test_source = load_source("tests/separator_semantics.rs");

    for rule in [
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 在 `view` 层隐藏关键状态决策。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "- [x] 公共 API 泄露底层实现细节类型。",
        "- [x] 用临时补丁破坏跨组件一致性。",
        "- [x] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
        "separator_forbidden_antipatterns_checklist_items_are_individually_enforced",
    ] {
        assert!(
            check2_source.contains(rule),
            "Separator check2 should keep anti-pattern governance item `{rule}`."
        );
    }

    for forbidden in [
        "use leptos",
        "web_sys",
        "wasm_bindgen",
        "view! {",
        "data-slot=",
        "class=",
        "style=",
    ] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives separator should remain DOM/style free; found `{forbidden}`."
        );
    }

    for forbidden in [
        ".ui-",
        "class=",
        "style=",
        "SpringAnimator",
        "attach_motion(",
        "keyframes",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "ui-headless separator should remain semantic-only; found `{forbidden}`."
        );
    }

    for required in [
        "logic::normalize_props(SeparatorNormalizeInput {",
        "let state = logic::resolve_state(normalized.state_input);",
    ] {
        assert!(
            view_source.contains(required),
            "Separator view should consume normalized state output via `{required}`."
        );
    }
    for forbidden in [
        "SeparatorStateInput {",
        "unwrap_or(",
        "value.unwrap_or",
        "match orientation",
        "match is_decorative",
        "match element_type",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Separator view should not reimplement state/default decisions; found `{forbidden}`."
        );
    }

    for required in [
        "#[prop(optional)] orientation: Option<SeparatorOrientation>",
        "#[prop(optional)] is_decorative: Option<bool>",
        "#[prop(optional)] element_type: Option<SeparatorElementType>",
        "pub fn normalize_orientation(value: Option<SeparatorOrientation>) -> SeparatorOrientation",
        "pub fn normalize_is_decorative(value: Option<bool>) -> bool",
        "pub fn normalize_element_type(value: Option<SeparatorElementType>) -> SeparatorElementType",
        "separator_api_naming_contract_uses_is_prefix_for_boolean_props",
        "separator_discrete_state_axes_are_type_constrained_with_enums",
        "separator_default_values_are_single_sourced_in_logic",
    ] {
        assert!(
            view_source.contains(required)
                || logic_source.contains(required)
                || test_source.contains(required),
            "Separator parameter contract should remain unified via `{required}`."
        );
    }

    for forbidden in [
        "children: Children",
        "labels",
        "titles",
        "panels",
        "ItemSpec",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Separator should not use parallel-array/implicit-composition API `{forbidden}`."
        );
    }

    for forbidden in [
        "pub use leptos::web_sys",
        "pub use web_sys::",
        "web_sys::",
        "wasm_bindgen::",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "Separator public API should not leak platform-internal type `{forbidden}`."
        );
    }

    for forbidden in ["TODO", "FIXME", "HACK", "temporary patch", "quick fix"] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "Separator files should not carry temporary patch marker `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains("pub use ui_state_primitives::separator::{"),
        "Reusable separator state primitive should stay in ui-state-primitives."
    );
    for forbidden in [
        "pub struct SeparatorState {",
        "pub enum SeparatorOrientation {",
        "pub enum SeparatorElementType {",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Separator logic should not re-own reusable primitive definition `{forbidden}`."
        );
    }
}

#[test]
fn separator_forbidden_antipatterns_are_absent_across_layers() {
    let primitive_source = load_source("../ui-state-primitives/src/separator.rs");
    let headless_source = load_source("../ui-headless/src/separator.rs");
    let logic_source = load_source("src/separator/logic.rs");
    let view_source = load_source("src/separator/view.rs");

    for forbidden in [
        "use leptos",
        "web_sys",
        "wasm_bindgen",
        "view! {",
        "data-slot=",
        "class=",
        "style=",
    ] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives separator should not include DOM/style token `{forbidden}`."
        );
    }

    for forbidden in [
        ".ui-",
        "class_name=",
        "SpringAnimator",
        "attach_motion(",
        "keyframes",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "ui-headless separator should not include visual or animation token `{forbidden}`."
        );
    }

    for forbidden in ["TODO", "FIXME", "HACK", "temporary patch", "quick fix"] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Separator implementation should avoid temporary patch marker `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains("pub use ui_state_primitives::separator::{"),
        "Separator logic should keep reusable state primitive in ui-state-primitives."
    );
}
