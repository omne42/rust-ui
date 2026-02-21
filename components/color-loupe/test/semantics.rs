fn load_source(path: &str) -> &'static str {
    match path {
        "lib" => include_str!("../src/lib.rs"),
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "protocol" => include_str!("../src/protocol.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "readme" => include_str!("../src/README.md"),
        "cargo" => include_str!("../Cargo.toml"),
        "check2" => include_str!("../check2.md"),
        "component_manifest" => include_str!("../Component.toml"),
        "component_rbi" => include_str!("../Component.rbi"),
        "docs_forms_color" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs")
        }
        "docs_pages" => include_str!("../../../apps/docs-app/src/pages/components/pages.rs"),
        "docs_playground" => include_str!("../../../apps/docs-app/src/playground.rs"),
        "docs_shell" => include_str!("../../../apps/docs-app/src/pages/components/shell.rs"),
        "heroui_strategy_doc" => {
            include_str!("../../../docs/spec/heroui-parameter-design-strategy.md")
        }
        "perf_check_script" => include_str!("../../../scripts/check-ui-components-performance.sh"),
        "e2e_color_loupe_contract" => {
            include_str!("../../../e2e/tests/docs_app_color_loupe_contract.spec.mjs")
        }
        "ui_components_lib" => include_str!("../../../crates/ui-components/src/lib.rs"),
        "ui_components_cargo" => include_str!("../../../crates/ui-components/Cargo.toml"),
        "ui_components_css" => include_str!("../../../crates/ui-components/src/css.rs"),
        "ui_components_root" => include_str!("../../../crates/ui-components/src/root.rs"),
        "ui_visual_active_highlight" => {
            include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs")
        }
        "ui_headless_lib" => include_str!("../../../crates/ui-headless/src/lib.rs"),
        "ui_headless_cargo" => include_str!("../../../crates/ui-headless/Cargo.toml"),
        "ui_motion_lib" => include_str!("../../../crates/ui-motion/src/lib.rs"),
        "code_block_view" => include_str!("../../../components/code-block/src/view.rs"),
        "legacy_semantics" => {
            include_str!("../../../components/color-loupe/test/color_loupe_semantics.rs")
        }
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn color_loupe_semantics_tests_are_migrated_to_component_directory() {
    let lib_source = load_source("lib");
    let mod_source = load_source("mod");
    let legacy_semantics = load_source("legacy_semantics");
    let local_semantics = include_str!("semantics.rs");

    for source in [lib_source, mod_source] {
        assert!(
            source.contains("#[path = \"../test/semantics.rs\"]")
                && source.contains("mod semantics_tests;"),
            "color-loupe should wire `components/color-loupe/test/semantics.rs` from both lib/mod entrypoints.",
        );
    }

    assert!(
        legacy_semantics.contains("color_loupe_"),
        "legacy ui-components semantics suite should still be readable during migration.",
    );
    assert!(
        local_semantics.contains("color_loupe_semantics_tests_are_migrated_to_component_directory"),
        "component-local semantics suite should provide migration coverage.",
    );
}

#[test]
fn color_loupe_ui_components_layer_boundaries_are_explicit() {
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let styles_source = load_source("styles");

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{",
        "ColorLoupeOutputState",
        "ColorLoupeState",
        "ColorLoupeStateInput",
        "DEFAULT_ARIA_LABEL",
        "DEFAULT_COLOR",
        "pub use view::ColorLoupe;",
    ] {
        assert!(
            mod_source.contains(required),
            "color-loupe mod.rs should keep ui-components export boundary `{required}`.",
        );
    }

    assert!(
        logic_source.contains("pub use ui_state_primitives::color_loupe::{"),
        "logic.rs should consume ui-state-primitives instead of redefining state machines.",
    );
    assert!(
        logic_source.contains("pub fn resolve_component_state(input: ColorLoupeLogicInput)")
            && logic_source.contains("normalize_position_percent(input.x_percent)")
            && logic_source.contains("normalize_position_percent(input.y_percent)"),
        "logic.rs should own default position normalization before mapping into ui-state-primitives.",
    );
    assert!(
        view_source.contains("#[prop(optional)] is_open: bool")
            && view_source.contains("#[prop(optional)] is_disabled: bool")
            && view_source.contains("#[prop(optional, into)] lang: Option<String>")
            && view_source.contains("#[prop(optional)] dir: Option<A11yDirection>"),
        "view.rs public API should follow `is_*` bool naming contract.",
    );
    assert!(
        !view_source.contains("#[prop(optional)] open: bool")
            && !view_source.contains("#[prop(optional)] disabled: bool"),
        "view.rs should not keep legacy bool aliases `open/disabled`.",
    );
    assert!(
        view_source.contains("logic::resolve_component_state(ColorLoupeLogicInput {")
            && view_source.contains("role=\"img\"")
            && view_source
                .contains("let locale = locale_attrs(logic::normalize_optional_text(lang), dir);")
            && view_source.contains("lang=locale.lang")
            && view_source.contains("dir=locale.dir")
            && view_source.contains("data-state=move || state.get().data_state_attr")
            && view_source
                .contains("data-disabled=move || state.get().is_disabled.then_some(\"true\")")
            && view_source.contains("data-aria-source=move || state.get().aria_source_attr")
            && view_source.contains("data-class-source=move || state.get().class_source_attr"),
        "view.rs should assemble semantics from logic/headless-compatible markers.",
    );
    assert!(
        !view_source.contains("#[prop(optional, default = logic::DEFAULT_POSITION_PERCENT)]"),
        "view.rs should not own x/y default fallback; defaults must be normalized in logic.rs.",
    );
    assert!(
        styles_source.contains("pub const CSS: &str =") && styles_source.contains("var(--ui-"),
        "styles.rs should remain token-first static CSS.",
    );
}

#[test]
fn color_loupe_public_surface_does_not_expose_dom_platform_types() {
    let mod_source = load_source("mod");
    let lib_source = load_source("lib");

    for forbidden in [
        "web_sys::",
        "web-sys",
        "wasm_bindgen",
        "JsValue",
        "HtmlElement",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "color-loupe ui-components public module should not expose `{forbidden}`.",
        );
        assert!(
            !lib_source.contains(forbidden),
            "color-loupe crate public entry should not expose `{forbidden}`.",
        );
    }
}

#[test]
fn color_loupe_hydration_id_is_deterministic_and_time_free() {
    let view_source = load_source("view");
    let logic_source = load_source("logic");

    assert!(
        view_source.contains("id_base: String") && view_source.contains("id=id_base"),
        "color-loupe should use caller-provided `id_base` directly to keep SSR/hydration id stable.",
    );

    for forbidden in [
        "now(",
        "SystemTime",
        "Instant::now",
        "rand::",
        "random(",
        "Uuid",
        "uuid::",
        "Date::now",
        "Math::random",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not use nondeterministic id source `{forbidden}`.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not use nondeterministic id source `{forbidden}`.",
        );
    }
}

#[test]
fn color_loupe_platform_contract_is_explicit_and_non_wasm_safe() {
    let mod_source = load_source("mod");
    let lib_source = load_source("lib");
    let logic_source = load_source("logic");
    let protocol_source = load_source("protocol");
    let view_source = load_source("view");
    let styles_source = load_source("styles");
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");

    for source in [
        mod_source,
        lib_source,
        logic_source,
        protocol_source,
        view_source,
        styles_source,
    ] {
        for forbidden in [
            "web_sys::",
            "web-sys",
            "js_sys::",
            "wasm_bindgen",
            "window(",
            "document(",
            "HtmlElement",
        ] {
            assert!(
                !source.contains(forbidden),
                "color-loupe source should remain non-wasm-safe and must not depend on `{forbidden}`.",
            );
        }
    }

    assert!(
        ui_components_lib.contains("#[cfg(feature = \"component-color_loupe\")]")
            && ui_components_lib.contains("pub mod color_loupe;"),
        "ui-components lib.rs should gate color-loupe module behind component feature.",
    );
    assert!(
        ui_components_css.contains("#[cfg(feature = \"component-color_loupe\")]")
            && ui_components_css.contains("out.push_str(crate::color::loupe::styles::CSS);"),
        "ui-components css.rs should gate color-loupe CSS aggregation behind component feature.",
    );
}

#[test]
fn color_loupe_headless_feature_exclusion_contract_is_guarded() {
    let view_source = load_source("view");
    let ui_headless_lib = load_source("ui_headless_lib");
    let ui_headless_cargo = load_source("ui_headless_cargo");

    assert!(
        view_source.contains("use ui_headless::a11y::{A11yDirection, locale_attrs};")
            && view_source
                .contains("let locale = locale_attrs(logic::normalize_optional_text(lang), dir);"),
        "color-loupe should consume a11y contract from ui-headless instead of redefining locale semantics.",
    );

    assert!(
        ui_headless_lib.contains("#[cfg(all(feature = \"web\", feature = \"ssr\"))]")
            && ui_headless_lib.contains(
                "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
            ),
        "ui-headless must keep explicit compile_error guard for web/ssr mutual exclusion.",
    );

    assert!(
        ui_headless_cargo.contains("web = [\"leptos/csr\"]")
            && ui_headless_cargo.contains("ssr = [\"leptos/ssr\"]"),
        "ui-headless feature matrix should keep separate web/ssr feature paths.",
    );
}

#[test]
fn color_loupe_motion_non_wasm_noop_contract_is_guarded() {
    let mod_source = load_source("mod");
    let lib_source = load_source("lib");
    let logic_source = load_source("logic");
    let protocol_source = load_source("protocol");
    let view_source = load_source("view");
    let styles_source = load_source("styles");
    let ui_motion_lib = load_source("ui_motion_lib");

    for source in [
        mod_source,
        lib_source,
        logic_source,
        protocol_source,
        view_source,
        styles_source,
    ] {
        for forbidden in [
            "ui_motion::",
            "MotionHandle",
            "AnimationHandle",
            "attach_motion(",
        ] {
            assert!(
                !source.contains(forbidden),
                "color-loupe snapshot component should not assume runtime motion handle `{forbidden}`.",
            );
        }
    }

    assert!(
        ui_motion_lib.contains("#[cfg(target_arch = \"wasm32\")]")
            && ui_motion_lib.contains("pub mod web;")
            && ui_motion_lib.contains("#[cfg(not(target_arch = \"wasm32\"))]")
            && ui_motion_lib.contains("pub fn prefers_reduced_motion() -> bool {")
            && ui_motion_lib.contains("pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}"),
        "ui-motion should keep explicit non-wasm no-op/stub backend.",
    );
    assert!(
        ui_motion_lib.contains("#[cfg(all(test, not(target_arch = \"wasm32\")))]")
            && ui_motion_lib.contains("fn non_wasm_web_backend_is_predictable_noop()"),
        "ui-motion should keep predictable non-wasm no-op regression test.",
    );
}

#[test]
fn color_loupe_motion_contract_item_is_n_a_and_safe() {
    let mod_source = load_source("mod");
    let view_source = load_source("view");
    let styles_source = load_source("styles");
    let ui_motion_lib = load_source("ui_motion_lib");

    assert!(
        !mod_source.contains("mod motion;") && !mod_source.contains("pub mod motion;"),
        "snapshot color-loupe should not expose a runtime motion.rs module.",
    );
    assert!(
        !view_source.contains("attach_motion(")
            && !view_source.contains("stiffness")
            && !view_source.contains("damping"),
        "view.rs should not bind runtime spring contract fields for snapshot-only motion path.",
    );
    assert!(
        styles_source.contains("@media (prefers-reduced-motion: reduce)")
            && styles_source.contains("animation: none;"),
        "styles.rs should keep reduced-motion downgrade even without runtime motion attach.",
    );
    assert!(
        ui_motion_lib.contains("#[cfg(not(target_arch = \"wasm32\"))]")
            && ui_motion_lib.contains("pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}"),
        "ui-motion should preserve non-wasm no-op backend for SSR/tooling safety.",
    );
}

#[test]
fn color_loupe_reduced_motion_and_ssr_wasm_semantics_are_stable() {
    let mod_source = load_source("mod");
    let lib_source = load_source("lib");
    let logic_source = load_source("logic");
    let protocol_source = load_source("protocol");
    let view_source = load_source("view");
    let styles_source = load_source("styles");

    assert!(
        styles_source.contains("@media (prefers-reduced-motion: reduce)")
            && styles_source.contains(".ui-color-loupe[data-state=\"open\"]")
            && styles_source.contains("animation: none;"),
        "styles.rs should downgrade open animation under reduced-motion.",
    );

    for source in [
        mod_source,
        lib_source,
        logic_source,
        protocol_source,
        view_source,
        styles_source,
    ] {
        for forbidden in [
            "target_arch = \"wasm32\"",
            "feature = \"ssr\"",
            "feature = \"web\"",
            "cfg!(target_arch",
        ] {
            assert!(
                !source.contains(forbidden),
                "color-loupe component implementation should not split SSR/wasm semantics with `{forbidden}`.",
            );
        }
    }

    assert!(
        view_source.contains("role=\"img\"")
            && view_source.contains("data-state=move || state.get().data_state_attr")
            && view_source.contains("data-open=move || state.get().is_open.then_some(\"true\")")
            && view_source
                .contains("data-disabled=move || state.get().is_disabled.then_some(\"true\")"),
        "view.rs should expose identical machine-readable semantics for SSR and wasm paths.",
    );
}

#[test]
fn color_loupe_styles_use_defensive_variable_fallback_chains() {
    let styles_source = load_source("styles");

    for required in [
        "--ui-color-loupe-space-2xs: var(--ui-space-2xs, var(--ui-fallback-space-2xs));",
        "--ui-color-loupe-space-xs: var(--ui-space-xs, var(--ui-fallback-space-xs));",
        "--ui-color-loupe-space-sm: var(--ui-space-sm, var(--ui-fallback-space-sm));",
        "--ui-color-loupe-space-md: var(--ui-space-md, var(--ui-fallback-space-md));",
        "--ui-color-loupe-space-xl: var(--ui-space-xl, var(--ui-fallback-space-xl));",
        "--ui-color-loupe-radius-sm: var(--ui-radius-sm, var(--ui-fallback-radius-sm));",
        "--ui-color-loupe-radius-lg: var(--ui-radius-lg, var(--ui-fallback-radius-lg));",
        "--ui-color-loupe-border-width: var(--ui-border-width, var(--ui-fallback-border-width));",
        "--ui-color-loupe-fg: var(--ui-fg, var(--ui-fallback-fg));",
        "--ui-color-loupe-bg: var(--ui-bg, var(--ui-fallback-bg));",
        "--ui-color-loupe-border: var(--ui-border, var(--ui-fallback-border));",
        "--ui-color-loupe-z-index: var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index));",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep defensive variable fallback chain `{required}`.",
        );
    }

    assert!(
        !styles_source.contains("rem"),
        "styles.rs should avoid raw rem terminal sizes and derive sizes from theme variables.",
    );

    let has_hex_literal = styles_source
        .as_bytes()
        .windows(2)
        .any(|pair| pair[0] == b'#' && (pair[1] as char).is_ascii_hexdigit());
    assert!(
        !has_hex_literal,
        "styles.rs should not hardcode hex colors; use theme variables and color-mix instead.",
    );
}

#[test]
fn color_loupe_css_cascade_layer_contract_is_explicit() {
    let ui_components_css = load_source("ui_components_css");
    let view_source = load_source("view");

    assert!(
        ui_components_css.contains("out.push_str(\"\\n@layer ui {\\n\");")
            && ui_components_css.contains("out.push_str(\"\\n}\\n\");"),
        "ui-components css aggregator should wrap component styles in explicit `@layer ui`.",
    );
    assert!(
        ui_components_css.contains("#[cfg(feature = \"component-color_loupe\")]")
            && ui_components_css.contains("out.push_str(crate::color::loupe::styles::CSS);"),
        "color-loupe styles should be injected only through feature-gated css aggregation.",
    );
    assert!(
        !view_source.contains("style=\"")
            && !view_source.contains("style =\"")
            && !view_source.contains("style=\"top:")
            && !view_source.contains("style=\"left:"),
        "view.rs should not rely on raw inline style declarations for runtime numeric adjustments.",
    );
}

#[test]
fn color_loupe_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("view");

    assert!(
        view_source.contains("const COLOR_LOUPE_SWATCH_CLASS: &str = \"ui-color-loupe__swatch\";")
            && view_source.contains("fn render_loupe_fill(color: Option<String>) -> impl IntoView")
            && view_source
                .contains("fn render_loupe_bubble(color: Option<String>) -> impl IntoView"),
        "view.rs should extract repeated loupe fill rendering into a dedicated helper and stable class constant.",
    );
    assert!(
        view_source.contains("{move || render_loupe_bubble(color.get_value())}"),
        "main ColorLoupe view! should delegate bubble/fill rendering to helper instead of nesting duplicate branches.",
    );
    assert!(
        !view_source.contains("if let Some(color) = color.get_value()"),
        "view.rs should avoid inline duplicated conditional rendering branches in the main view! block.",
    );
}

#[test]
fn color_loupe_prefers_function_split_over_extra_component() {
    let view_source = load_source("view");
    let component_count = view_source.matches("#[component]").count();

    assert!(
        view_source.contains("fn render_loupe_fill(color: Option<String>) -> impl IntoView"),
        "lightweight loupe fill fragment should be a plain Rust function returning `impl IntoView`.",
    );
    assert!(
        component_count == 1,
        "view.rs should keep only the main ColorLoupe as component; found {component_count} component macros.",
    );
    assert!(
        !view_source.contains("#[component]\nfn render_loupe_fill"),
        "render_loupe_fill should not be upgraded to a standalone #[component].",
    );
    assert!(
        !view_source.contains("#[component]\nfn render_loupe_bubble")
            && !view_source.contains("#[component]\nfn render_loupe_tail"),
        "loupe static shell fragments should remain plain functions instead of extra component macros.",
    );
}

#[test]
fn color_loupe_static_fragments_are_templated_with_constants() {
    let view_source = load_source("view");

    for required in [
        "const COLOR_LOUPE_BUBBLE_CLASS: &str = \"ui-color-loupe__bubble\";",
        "const COLOR_LOUPE_BUBBLE_SLOT: &str = \"color-loupe-bubble\";",
        "const COLOR_LOUPE_CHECKER_CLASS: &str = \"ui-color-loupe__checker\";",
        "const COLOR_LOUPE_CHECKER_SLOT: &str = \"color-loupe-checker\";",
        "const COLOR_LOUPE_FILL_CLASS: &str = \"ui-color-loupe__fill\";",
        "const COLOR_LOUPE_FILL_SLOT: &str = \"color-loupe-fill\";",
        "const COLOR_LOUPE_TAIL_CLASS: &str = \"ui-color-loupe__tail\";",
        "const COLOR_LOUPE_TAIL_SLOT: &str = \"color-loupe-tail\";",
        "const ARIA_HIDDEN_TRUE: &str = \"true\";",
        "fn render_loupe_bubble(color: Option<String>) -> impl IntoView",
        "fn render_loupe_tail() -> impl IntoView",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep static loupe shell fragment contract `{required}` centralized.",
        );
    }

    for forbidden in [
        "class=\"ui-color-loupe__bubble\"",
        "class=\"ui-color-loupe__checker\"",
        "class=\"ui-color-loupe__fill\"",
        "class=\"ui-color-loupe__tail\"",
        "data-slot=\"color-loupe-bubble\"",
        "data-slot=\"color-loupe-checker\"",
        "data-slot=\"color-loupe-fill\"",
        "data-slot=\"color-loupe-tail\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should avoid scattering inline static fragment literal `{forbidden}`.",
        );
    }

    assert!(
        view_source.contains("aria-hidden=ARIA_HIDDEN_TRUE"),
        "templated static fragments should preserve aria-hidden semantics.",
    );
}

#[test]
fn color_loupe_does_not_use_inner_html_injection_paths() {
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");

    for source in [view_source, logic_source, styles_source] {
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "innerHTML",
        ] {
            assert!(
                !source.contains(forbidden),
                "color-loupe should not introduce HTML injection primitive `{forbidden}`.",
            );
        }
    }
}

#[test]
fn color_loupe_wasm_debug_contract_is_non_polluting_for_snapshot_component() {
    let lib_source = load_source("lib");
    let mod_source = load_source("mod");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let cargo_source = load_source("cargo");

    for forbidden in [
        "on:click",
        "on:keydown",
        "on:pointerdown",
        "on:pointerup",
        "on:input",
        "on:change",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "snapshot color-loupe should not introduce interactive event replay surface `{forbidden}`.",
        );
    }

    assert!(
        view_source.contains("data-state=move || state.get().data_state_attr")
            && view_source.contains("data-aria-source=move || state.get().aria_source_attr")
            && view_source.contains("data-class-source=move || state.get().class_source_attr"),
        "state/source markers should remain machine-readable for debug tracing without runtime debug hooks.",
    );

    for source in [lib_source, mod_source, logic_source] {
        for forbidden in ["pub mod debug", "pub use crate::debug", "TraceId", "replay"] {
            assert!(
                !source.contains(forbidden),
                "public API should not leak debug surface `{forbidden}` in default build.",
            );
        }
    }

    assert!(
        cargo_source.contains("[features]") && cargo_source.contains("default = []"),
        "crate features should keep default artifact free of optional debug capability.",
    );
    for forbidden in ["wasm-debug", "debug-trace", "debug-replay"] {
        assert!(
            !cargo_source.contains(forbidden),
            "Cargo features should not accidentally ship debug toggle `{forbidden}`.",
        );
    }
}

#[test]
fn color_loupe_dx_workbench_supports_style_iteration_and_context_retention() {
    let docs_forms_color = load_source("docs_forms_color");

    for required in [
        "title=\"ColorLoupe\"",
        "slug=\"color-loupe\"",
        "title=\"Interactive Playground\"",
        "test_source_path=\"components/color-loupe/src/styles.rs\".to_string()",
        "id_base=\"docs-color-loupe-workbench-main\".to_string()",
        "id_base=\"docs-color-loupe-workbench-compare\".to_string()",
        "id_base=\"docs-color-loupe-workbench-color\".to_string()",
        "id_base=\"docs-color-loupe-workbench-position\".to_string()",
        "Switch checked=is_open set_checked=set_is_open",
        "Switch checked=is_disabled set_checked=set_is_disabled",
        "Switch checked=custom_aria set_checked=set_custom_aria",
        "Switch checked=custom_class set_checked=set_custom_class",
    ] {
        assert!(
            docs_forms_color.contains(required),
            "docs workbench should preserve DX contract marker `{required}`.",
        );
    }
}

#[test]
fn color_loupe_docs_product_copy_paste_ready_contract_is_complete() {
    let docs_forms_color = load_source("docs_forms_color");
    let docs_playground = load_source("docs_playground");

    for required in [
        "pub(super) fn color_loupe() -> AnyView",
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled（N/A）\"",
        "title=\"Streaming Optional / Snapshot\"",
        "title=\"Interactive Playground\"",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "data-ui-output-state=\"snapshot\"",
        "data-slot=\"color-loupe-copy-ready\"",
        "\"Source-first / Copy-Paste Ready\"",
        "components/color-loupe/src/view.rs",
        "components/color-loupe/src/logic.rs",
        "components/color-loupe/src/styles.rs",
    ] {
        assert!(
            docs_forms_color.contains(required),
            "color-loupe docs page should keep copy-ready contract marker `{required}`.",
        );
    }

    assert!(
        docs_forms_color.contains("受控/非受控切换语义 N/A"),
        "docs should explicitly explain controlled/uncontrolled comparison as N/A for snapshot-only color-loupe.",
    );
    assert!(
        docs_playground.contains("fn compose_copy_ready_code(raw: &str, imports: &str) -> String")
            && docs_playground.contains("const DEFAULT_PLAYGROUND_IMPORTS: &str")
            && docs_playground.contains("use ui_components::*;"),
        "docs playground should keep copy-ready import injection pipeline.",
    );
}

#[test]
fn color_loupe_docs_interactive_playground_contract_is_online_and_reproducible() {
    let docs_forms_color = load_source("docs_forms_color");
    let e2e_contract = load_source("e2e_color_loupe_contract");
    let component_manifest = load_source("component_manifest");

    for required in [
        "title=\"Interactive Playground\"",
        "code_signal=workbench_code",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"color-loupe-workbench-controls\"",
        "data-slot=\"color-loupe-workbench-color\"",
        "data-slot=\"color-loupe-workbench-position\"",
        "data-slot=\"color-loupe-workbench-open\"",
        "data-slot=\"color-loupe-workbench-disabled\"",
        "data-slot=\"color-loupe-workbench-custom-aria\"",
        "data-slot=\"color-loupe-workbench-custom-class\"",
        "id_base=\"docs-color-loupe-workbench-main\".to_string()",
        "id_base=\"docs-color-loupe-workbench-compare\".to_string()",
    ] {
        assert!(
            docs_forms_color.contains(required),
            "docs should keep interactive-playground contract marker `{required}`.",
        );
    }

    for required in [
        "workbench flow uses semantic ready/settled breakpoints",
        "color-loupe-workbench-position",
        "color-loupe-workbench-custom-aria",
        "color-loupe-workbench-custom-class",
        "color-loupe-workbench-disabled",
        "color-loupe-workbench-open",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-state\", \"open\")",
    ] {
        assert!(
            e2e_contract.contains(required),
            "interactive playground should keep reproducible e2e flow marker `{required}`.",
        );
    }

    assert!(
        component_manifest.contains("spec_builder = false"),
        "ColorLoupe should keep AI-spec playground input as N/A by declaring spec_builder=false in manifest.",
    );
}

#[test]
fn color_loupe_source_first_copy_paste_ready_contract_is_enforced() {
    let docs_forms_color = load_source("docs_forms_color");
    let docs_playground = load_source("docs_playground");
    let code_block_view = load_source("code_block_view");

    for required in [
        "data-slot=\"color-loupe-copy-ready\"",
        "\"Source-first / Copy-Paste Ready\"",
        "compose_copy_ready_code",
        "component-color_loupe + inject-css",
        "\"UiRoot\"",
        "components/color-loupe/src/view.rs",
        "components/color-loupe/src/logic.rs",
        "components/color-loupe/src/styles.rs",
    ] {
        assert!(
            docs_forms_color.contains(required),
            "color-loupe docs should keep source-first copy-ready marker `{required}`.",
        );
    }

    for required in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            docs_playground.contains(required),
            "docs playground should keep copy-ready pipeline marker `{required}`.",
        );
    }

    for required in [
        "data-slot=\"code-block-header\"",
        "class_name=\"ui-code-block__copy-button\"",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view.contains(required),
            "code-block should keep one-click copy affordance marker `{required}`.",
        );
    }
}

#[test]
fn color_loupe_heroui_alignment_and_docs_entry_stay_in_sync() {
    let heroui_strategy_doc = load_source("heroui_strategy_doc");
    let docs_pages = load_source("docs_pages");
    let docs_forms_color = load_source("docs_forms_color");
    let readme = load_source("readme");

    for required in [
        "### ColorLoupe 同步记录（2026-02-20）",
        "`ColorLoupe` 参数主轴保持 `id_base/color/is_open/is_disabled/x_percent/y_percent/aria_label/class_name/lang/dir/output_state`",
        "component_doc!(\"ColorLoupe\", \"color-loupe\", \"Forms\", forms_color::color_loupe)",
        "`apps/docs-app/src/pages/components/pages/forms_color.rs::color_loupe()` 已覆盖",
        "`Open + Position Buckets`",
        "`Interactive Playground`，并与当前参数语义保持一致。",
        "`component-color_loupe + inject-css`、`UiRoot`",
    ] {
        assert!(
            heroui_strategy_doc.contains(required),
            "heroui strategy doc should keep color-loupe sync marker `{required}`.",
        );
    }

    assert!(
        docs_pages.contains(
            "component_doc!(\n        \"ColorLoupe\",\n        \"color-loupe\",\n        \"Forms\",\n        forms_color::color_loupe"
        ),
        "docs catalog should keep indexable ColorLoupe component entry.",
    );
    assert!(
        docs_forms_color.contains("title=\"ColorLoupe\"")
            && docs_forms_color.contains("slug=\"color-loupe\""),
        "forms_color docs page should keep stable title+slug entry for color-loupe.",
    );
    assert!(
        readme.contains("# ColorLoupe") && readme.contains("## 快速开始（先用起来）"),
        "component README should remain an equivalent documentation entry for color-loupe.",
    );
}

#[test]
fn color_loupe_semantics_first_contract_prioritizes_data_aria_role_and_source_markers() {
    let view_source = load_source("view");
    let local_semantics = include_str!("semantics.rs");
    let legacy_semantics = load_source("legacy_semantics");

    for marker in [
        "role=\"img\"",
        "aria-label=move || aria_label.get_value()",
        "data-state=move || state.get().data_state_attr",
        "data-output-state=output_state.as_attr()",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "view.rs should expose semantic marker `{marker}`.",
        );
    }

    for required_test in [
        "fn color_loupe_ui_components_layer_boundaries_are_explicit()",
        "fn color_loupe_snapshot_mode_supports_complete_configuration_render_contract()",
        "fn color_loupe_semantics_and_performance_regression_contract_is_covered()",
    ] {
        assert!(
            local_semantics.contains(required_test),
            "component semantics suite should keep semantic-contract test `{required_test}`.",
        );
    }
    assert!(
        legacy_semantics.contains("fn color_loupe_exposes_baseline_style_data_markers()"),
        "legacy ui-components semantics suite should keep baseline data-marker regression coverage.",
    );

    assert!(
        !view_source.contains("on:keydown")
            && !view_source.contains("on:focus")
            && !view_source.contains("tabindex"),
        "snapshot color-loupe keeps keyboard/focus interaction path as N/A; semantics still must stay test-first.",
    );
}

#[test]
fn color_loupe_e2e_semantic_selectors_are_stable_and_sleep_free() {
    let docs_forms_color = load_source("docs_forms_color");
    let e2e_contract = load_source("e2e_color_loupe_contract");

    for required in [
        "data-slot=\"color-loupe-workbench-controls\"",
        "data-slot=\"color-loupe-workbench-open\"",
        "data-slot=\"color-loupe-workbench-disabled\"",
        "data-slot=\"color-loupe-workbench-custom-aria\"",
        "data-slot=\"color-loupe-workbench-custom-class\"",
        "data-slot=\"color-loupe-workbench-position\"",
        "data-slot=\"color-loupe-workbench-canvas\"",
        "data-slot=\"color-loupe-workbench-surface\"",
    ] {
        assert!(
            docs_forms_color.contains(required),
            "color-loupe docs controls should expose stable data-slot selector `{required}`.",
        );
    }

    for required in [
        "body:not(:has(#boot))",
        "[data-component=\"color-loupe\"]",
        "#docs-color-loupe-workbench-main[data-slot=\"color-loupe\"]",
        "[data-slot=\"color-loupe-output-mode\"]",
        "[data-slot=\"color-loupe-workbench-position\"] [data-slot=\"segmented-control\"]",
        "[data-slot=\"color-loupe-workbench-open\"] [data-slot=\"switch\"]",
        "[data-slot=\"color-loupe-workbench-disabled\"] [data-slot=\"switch\"]",
        "[data-slot=\"color-loupe-workbench-custom-aria\"] [data-slot=\"switch\"]",
        "[data-slot=\"color-loupe-workbench-custom-class\"] [data-slot=\"switch\"]",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-state\", \"open\")",
    ] {
        assert!(
            e2e_contract.contains(required),
            "color-loupe e2e contract should use semantic selector/wait marker `{required}`.",
        );
    }

    for forbidden in ["waitForTimeout", "setTimeout", "sleep("] {
        assert!(
            !e2e_contract.contains(forbidden),
            "color-loupe e2e contract should avoid fixed-time wait primitive `{forbidden}`.",
        );
    }
}

#[test]
fn color_loupe_docs_examples_and_state_matrix_are_synced_with_logic_defaults() {
    let docs_forms_color = load_source("docs_forms_color");
    let logic_source = load_source("logic");
    let color_loupe_start = docs_forms_color
        .find("pub(super) fn color_loupe() -> AnyView {")
        .expect("docs forms page should keep color_loupe entrypoint");
    let color_loupe_docs = &docs_forms_color[color_loupe_start..];

    for required in [
        "pub(super) fn color_loupe() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Open + Position Buckets\"",
        "title=\"Disabled + Custom Label + Custom Class\"",
        "title=\"Controlled vs Uncontrolled（N/A）\"",
        "title=\"State Matrix\"",
        "title=\"Streaming Optional / Snapshot\"",
        "title=\"Interactive Playground\"",
        "id_base=\"docs-color-loupe-matrix-default\".to_string()",
    ] {
        assert!(
            color_loupe_docs.contains(required),
            "color-loupe docs should keep synchronized example/state-matrix marker `{required}`.",
        );
    }

    for required in [
        "is_open=true",
        "is_disabled=true",
        "x_percent=18.0",
        "y_percent=74.0",
        "aria_label=\"Accent loupe\".to_string()",
        "class_name=\"docs-color-loupe-custom\".to_string()",
        "output_state=ColorLoupeOutputState::Verified",
    ] {
        assert!(
            color_loupe_docs.contains(required),
            "docs examples should use current public API token `{required}`.",
        );
    }

    for forbidden in [
        "\n    open=",
        "\n    disabled=",
        "\n    default_open=",
        "\n    default_disabled=",
    ] {
        assert!(
            !color_loupe_docs.contains(forbidden),
            "docs examples should not drift to legacy or unsupported API token `{forbidden}`.",
        );
    }

    for required in [
        "pub fn normalize_position_percent(value: Option<f32>) -> f32",
        "DEFAULT_POSITION_PERCENT",
        "pub fn normalize_output_state(value: Option<ColorLoupeOutputState>) -> ColorLoupeOutputState",
        "None => ColorLoupeOutputState::Verified",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep default normalization rule `{required}`.",
        );
    }
}

#[test]
fn color_loupe_documentation_entry_is_beginner_friendly() {
    let readme = load_source("readme");
    let docs_forms_color = load_source("docs_forms_color");

    for required in [
        "# ColorLoupe",
        "## 快速开始（先用起来）",
        "### Hello World（最小可用）",
        "### 常见用法",
        "## 进阶（理解实现与契约）",
        "默认先用 `id_base + color + is_open` 跑通；需要时再增加",
    ] {
        assert!(
            readme.contains(required),
            "color-loupe README should keep beginner-friendly marker `{required}`.",
        );
    }

    let quick_start_pos = readme
        .find("## 快速开始（先用起来）")
        .expect("README should expose a quick-start section");
    let advanced_pos = readme
        .find("## 进阶（理解实现与契约）")
        .expect("README should expose an advanced section");
    assert!(
        quick_start_pos < advanced_pos,
        "README should keep quick-start before advanced details.",
    );

    assert!(
        docs_forms_color.contains("slug=\"color-loupe\"")
            && docs_forms_color.contains("title=\"Hello World\"")
            && docs_forms_color.contains("title=\"Interactive Playground\""),
        "docs-app should keep indexable color-loupe entry with beginner examples.",
    );
}

#[test]
fn color_loupe_engineering_contract_is_runtime_and_trace_free() {
    let lib_source = load_source("lib");
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let styles_source = load_source("styles");
    let cargo_source = load_source("cargo");

    assert!(
        !lib_source.contains("protocol")
            && !mod_source.contains("protocol")
            && !lib_source.contains("Spec")
            && !mod_source.contains("Spec"),
        "public API should not expose spec/protocol surface for snapshot-only color-loupe component.",
    );

    for source in [
        lib_source,
        mod_source,
        logic_source,
        view_source,
        styles_source,
    ] {
        for forbidden in [
            "tokio::",
            "async_std::",
            "async-std",
            "Runtime",
            "JoinHandle",
            "Future",
            "tracing::",
            "span!(",
            "event!(",
            "#[instrument",
            "async fn",
        ] {
            assert!(
                !source.contains(forbidden),
                "color-loupe engineering boundary should stay runtime/trace-free: `{forbidden}`.",
            );
        }
    }

    for forbidden in ["tokio", "async-std", "tracing"] {
        assert!(
            !cargo_source.contains(forbidden),
            "component Cargo manifest should not bind to runtime/tracing dependency `{forbidden}`.",
        );
    }
}

#[test]
fn color_loupe_ui_components_entrypoints_are_located_and_scoped() {
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");
    let ui_components_root = load_source("ui_components_root");
    let ui_visual_active_highlight = load_source("ui_visual_active_highlight");

    assert!(
        ui_components_lib.contains("#[cfg(feature = \"component-color_loupe\")]")
            && ui_components_lib.contains("pub mod color_loupe;")
            && ui_components_lib.contains("pub mod color {")
            && ui_components_lib.contains("pub use crate::color_loupe as loupe;"),
        "ui-components lib.rs should expose color-loupe only behind component feature and stable color namespace.",
    );
    assert!(
        ui_components_css.contains("out.push_str(\"\\n@layer ui {\\n\");")
            && ui_components_css.contains("out.push_str(crate::color::loupe::styles::CSS);")
            && ui_components_css.contains("out.push_str(\"\\n}\\n\");"),
        "ui-components css.rs should aggregate color-loupe CSS inside `@layer ui` with explicit close.",
    );
    assert!(
        ui_components_root.contains("out.push_str(css::BASE_CSS);")
            && ui_components_root.contains("out.push_str(&theme.get().to_css_variables());")
            && ui_components_root.contains("crate::css::push_components_css(&mut out);")
            && ui_components_root.contains("provide_ui_i18n(i18n);"),
        "ui-components root.rs should centralize base css/theme vars/components css injection and i18n context.",
    );
    assert!(
        ui_visual_active_highlight.contains("pub const CSS: &str =")
            && ui_visual_active_highlight.contains("pub struct ActiveHighlightMotion")
            && ui_visual_active_highlight.contains("ui_motion::spring::SpringConfig")
            && !ui_visual_active_highlight.contains("ColorLoupe"),
        "ui-visual-primitive active_highlight should remain shared motion primitive, not component-specific logic.",
    );

    let ui_components_src =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui-components/src");
    for forbidden in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src.join(forbidden).exists(),
            "ui-components source root should not contain deprecated entry file `{forbidden}`.",
        );
    }
}

#[test]
fn color_loupe_component_directory_layout_is_standardized() {
    use std::path::Path;

    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");

    let component_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs"] {
        assert!(
            component_src.join(required).exists(),
            "color-loupe component directory should include `{required}`.",
        );
    }
    let forbidden = "render.rs";
    assert!(
        !component_src.join(forbidden).exists(),
        "color-loupe component directory should not drift into `{forbidden}`.",
    );

    assert!(
        !component_src.join("spec.rs").exists(),
        "snapshot color-loupe should not add `spec.rs` without schema-level necessity.",
    );
    assert!(
        !component_src.join("motion.rs").exists(),
        "snapshot color-loupe is non-interactive and should keep `motion.rs` as N/A.",
    );

    assert!(
        mod_source.contains("pub(crate) mod logic;")
            && mod_source.contains("pub mod styles;")
            && mod_source.contains("mod view;")
            && mod_source.contains("pub use logic::{")
            && mod_source.contains("ColorLoupeOutputState")
            && mod_source.contains("ColorLoupeState")
            && mod_source.contains("ColorLoupeStateInput")
            && mod_source.contains("DEFAULT_ARIA_LABEL")
            && mod_source.contains("DEFAULT_COLOR")
            && mod_source.contains("pub use view::ColorLoupe;")
            && !mod_source.contains("pub mod protocol;")
            && !mod_source.contains("pub use protocol::"),
        "mod.rs should keep minimal stable exports and avoid over-exporting internal protocols.",
    );

    assert!(
        logic_source.contains("pub use ui_state_primitives::color_loupe::{")
            && logic_source.contains("pub fn resolve_component_state(input: ColorLoupeLogicInput)")
            && logic_source.contains("pub fn compose_class_name(")
            && !logic_source.contains("view!")
            && !logic_source.contains("leptos::"),
        "logic.rs should normalize/derive state via ui-state-primitives without render concerns.",
    );

    assert!(
        styles_source.contains("pub const CSS: &str =") && styles_source.contains("var(--ui-"),
        "styles.rs should remain token-first static css without hardcoded hex theme constants.",
    );
    for forbidden in [
        " #fff",
        " #000",
        "#fff;",
        "#000;",
        "background: #",
        "color: #",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should avoid hardcoded hex-like token `{forbidden}`.",
        );
    }

    assert!(
        view_source.contains("#[component]")
            && view_source.contains("pub fn ColorLoupe(")
            && view_source.contains("logic::resolve_component_state(ColorLoupeLogicInput {")
            && view_source.contains("let locale = locale_attrs(")
            && view_source.contains("role=\"img\"")
            && view_source.contains("data-state=move || state.get().data_state_attr")
            && !view_source.contains("attach_motion")
            && !view_source.contains("mod render;"),
        "view.rs should stay focused on leptos structure + a11y mount and avoid hidden state/motion wiring.",
    );
}

#[test]
fn color_loupe_hyper_structure_builder_item_is_n_a_for_snapshot_component() {
    use std::path::Path;

    let lib_source = load_source("lib");
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let protocol_source = load_source("protocol");

    let component_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    assert!(
        !component_src.join("spec.rs").exists(),
        "snapshot color-loupe should not introduce spec.rs builder surface.",
    );

    for source in [
        lib_source,
        mod_source,
        logic_source,
        view_source,
        protocol_source,
    ] {
        for forbidden in [
            "struct ColorLoupeSpec",
            "enum ColorLoupeSpec",
            "ColorLoupeSpec::new(",
            "Spec::new(",
            ".render(",
        ] {
            assert!(
                !source.contains(forbidden),
                "snapshot color-loupe should not expose hyper-structure builder token `{forbidden}`.",
            );
        }
    }
}

#[test]
fn color_loupe_context_compression_manifest_and_rbi_are_synced() {
    let manifest_source = load_source("component_manifest");
    let rbi_source = load_source("component_rbi");
    let view_source = load_source("view");
    let mod_source = load_source("mod");

    for required in [
        "id = \"ui-color-loupe\"",
        "name = \"ColorLoupe\"",
        "kind = \"snapshot\"",
        "rbi = \"Component.rbi\"",
        "mod_rs = \"src/mod.rs\"",
        "logic_rs = \"src/logic.rs\"",
        "styles_rs = \"src/styles.rs\"",
        "view_rs = \"src/view.rs\"",
        "snapshot = true",
        "streaming = false",
        "required = false",
        "fallback = \"snapshot\"",
        "owner = \"upstream\"",
        "spec_builder = false",
        "motion_runtime = false",
        "role = \"img\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "component manifest should keep context-compression contract token `{required}`.",
        );
    }

    for required in [
        "component \"ui-color-loupe\"",
        "signature ColorLoupe(",
        "id_base: String",
        "is_open?: bool",
        "is_disabled?: bool",
        "x_percent?: Option<f32>",
        "y_percent?: Option<f32>",
        "aria_label?: Option<String>",
        "class_name?: Option<String>",
        "output_state?: Option<ColorLoupeOutputState>",
        "lang?: Option<String>",
        "dir?: Option<A11yDirection>",
        "required: false",
        "fallback: \"snapshot\"",
        "owner: \"upstream\"",
        "\"data-state\"",
        "\"data-output-state\"",
        "\"data-aria-source\"",
        "\"data-class-source\"",
    ] {
        assert!(
            rbi_source.contains(required),
            "component rbi should keep interface projection token `{required}`.",
        );
    }

    assert!(
        view_source.contains("#[prop(optional)] is_open: bool")
            && view_source.contains("#[prop(optional)] is_disabled: bool")
            && view_source
                .contains("#[prop(optional)] output_state: Option<ColorLoupeOutputState>")
            && !rbi_source.contains("\n    open?: bool,\n")
            && !rbi_source.contains("\n    disabled?: bool,\n"),
        "rbi projection should stay aligned with view.rs `is_*` API naming.",
    );

    assert!(
        mod_source.contains("pub use view::ColorLoupe;"),
        "component export surface should match rbi projected primary export.",
    );
}

#[test]
fn color_loupe_agent_contract_schema_is_machine_readable_and_whitelisted() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let manifest_source = load_source("component_manifest");
    let rbi_source = load_source("component_rbi");

    for required in [
        "pub enum AgentContractIntent",
        "pub enum AgentContractAction",
        "pub enum AgentContractState",
        "pub enum AgentContractSource",
        "pub enum AgentContractBucket",
        "pub enum ColorLoupeOutputState",
        "pub struct ColorLoupeAgentContract",
        "pub fn resolve_agent_contract(",
        "output_state: ColorLoupeOutputState",
        "pub fn agent_contract_schema_attr(",
        "v=1;intent={};action={};state={};output_state={};source=aria:{},class:{};x_bucket={};y_bucket={}",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep typed agent-contract schema token `{required}`.",
        );
    }

    assert!(
        view_source.contains("data-ui-schema=move || ui_schema.get()")
            && view_source.contains("data-output-state=output_state.as_attr()")
            && view_source.contains(
                "let ui_schema =\n        Memo::new(move |_| logic::agent_contract_schema_attr(state.get(), output_state));"
            )
            && !view_source.contains("data-ui-schema=\"")
            && !view_source.contains("format!(\"v=1;intent="),
        "view.rs should mount machine-readable agent schema from logic, not ad-hoc string concatenation.",
    );

    for required in [
        "\"data-ui-schema\"",
        "\"data-output-state\"",
        "schema = \"ui-color-loupe/v1\"",
        "\"intent\"",
        "\"action\"",
        "\"state\"",
        "\"output_state\"",
        "\"source.aria\"",
        "\"source.class\"",
        "\"x_bucket\"",
        "\"y_bucket\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "Component.toml should project agent-contract schema token `{required}`.",
        );
    }

    for required in [
        "\"data-ui-schema\"",
        "\"data-output-state\"",
        "agent_contract_schema \"ui-color-loupe/v1\"",
        "\"intent\"",
        "\"action\"",
        "\"state\"",
        "\"output_state\"",
        "\"source.aria\"",
        "\"source.class\"",
        "\"x_bucket\"",
        "\"y_bucket\"",
    ] {
        assert!(
            rbi_source.contains(required),
            "Component.rbi should project agent-contract schema token `{required}`.",
        );
    }
}

#[test]
fn color_loupe_streaming_and_snapshot_modes_are_explicit_for_llm_rendering() {
    let manifest_source = load_source("component_manifest");
    let rbi_source = load_source("component_rbi");
    let view_source = load_source("view");

    assert!(
        manifest_source.contains("snapshot = true")
            && manifest_source.contains("streaming = false")
            && manifest_source.contains("kind = \"snapshot\"")
            && manifest_source.contains("required = false")
            && manifest_source.contains("fallback = \"snapshot\""),
        "component manifest should explicitly model snapshot/streaming rendering modes.",
    );
    assert!(
        rbi_source.contains("mode: \"snapshot\"")
            && rbi_source.contains("required: false")
            && rbi_source.contains("fallback: \"snapshot\""),
        "component rbi should project snapshot mode for agent consumption.",
    );

    for forbidden in ["stream_chunk", "token_delta", "on_stream", "partial_text"] {
        assert!(
            !view_source.contains(forbidden),
            "snapshot color-loupe view should not implement streaming rendering primitive `{forbidden}`.",
        );
    }
}

#[test]
fn color_loupe_snapshot_mode_supports_complete_configuration_render_contract() {
    let manifest_source = load_source("component_manifest");
    let rbi_source = load_source("component_rbi");
    let view_source = load_source("view");
    let logic_source = load_source("logic");

    assert!(
        manifest_source.contains("kind = \"snapshot\"")
            && manifest_source.contains("snapshot = true")
            && manifest_source.contains("streaming = false"),
        "component manifest should declare snapshot as the default rendering capability.",
    );
    assert!(
        rbi_source.contains("mode: \"snapshot\"") && rbi_source.contains("signature ColorLoupe("),
        "component rbi should expose snapshot mode and full render signature projection.",
    );

    for required in [
        "id_base: String",
        "#[prop(optional, into)] color: Option<String>",
        "#[prop(optional)] is_open: bool",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] x_percent: Option<f32>",
        "#[prop(optional)] y_percent: Option<f32>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] output_state: Option<ColorLoupeOutputState>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let output_state = logic::normalize_output_state(output_state);",
        "logic::resolve_component_state(ColorLoupeLogicInput {",
        "data-state=move || state.get().data_state_attr",
        "data-output-state=output_state.as_attr()",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep complete snapshot configuration render token `{required}`.",
        );
    }

    for required in [
        "pub fn normalize_position_percent(value: Option<f32>) -> f32",
        "pub fn normalize_output_state(value: Option<ColorLoupeOutputState>) -> ColorLoupeOutputState",
        "pub fn resolve_component_state(input: ColorLoupeLogicInput) -> ColorLoupeState",
        "normalize_aria_label",
        "sanitize_color",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep deterministic normalization for complete snapshot input `{required}`.",
        );
    }
}

#[test]
fn color_loupe_streaming_requirement_is_component_scoped_with_snapshot_fallback() {
    let manifest_source = load_source("component_manifest");
    let rbi_source = load_source("component_rbi");
    let view_source = load_source("view");
    let logic_source = load_source("logic");

    assert!(
        manifest_source.contains("required = false")
            && manifest_source.contains("fallback = \"snapshot\"")
            && manifest_source.contains("owner = \"upstream\""),
        "manifest should define streaming as optional with explicit snapshot fallback and upstream ownership.",
    );
    assert!(
        rbi_source.contains("required: false")
            && rbi_source.contains("fallback: \"snapshot\"")
            && rbi_source.contains("owner: \"upstream\""),
        "rbi should project streaming-optional policy with snapshot fallback.",
    );
    assert!(
        view_source.contains("data-output-state=output_state.as_attr()")
            && view_source.contains("role=\"img\"")
            && view_source.contains("data-state=move || state.get().data_state_attr")
            && view_source.contains("data-aria-source=move || state.get().aria_source_attr"),
        "view should keep output-state/status + role/aria/data markers continuously readable.",
    );
    assert!(
        logic_source.contains("pub enum ColorLoupeOutputState")
            && logic_source.contains("Draft")
            && logic_source.contains("Verified")
            && logic_source.contains("Committable")
            && logic_source.contains("normalize_output_state"),
        "logic should keep typed output-state model for draft/verified/committable.",
    );
    for source in [view_source, logic_source] {
        for forbidden in [
            "retry",
            "reconnect",
            "backoff",
            "validate_stream",
            "network_error",
        ] {
            assert!(
                !source.contains(forbidden),
                "component layer should not own retry/reconnect/validation concern `{forbidden}`.",
            );
        }
    }
}

#[test]
fn color_loupe_rust_hygiene_component_source_is_clean() {
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let styles_source = load_source("styles");

    for source in [mod_source, logic_source, view_source, styles_source] {
        for forbidden in [".unwrap(", ".expect(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "component non-test source should not contain rust-hygiene forbidden token `{forbidden}`.",
            );
        }
    }

    assert!(
        logic_source.contains("use std::borrow::Cow;")
            && logic_source.contains("Vec<Cow<'static, str>>"),
        "logic.rs should use Cow<'static, str> to reduce string clone churn on static class tokens.",
    );
    assert_eq!(
        logic_source.matches(".to_string()").count(),
        0,
        "logic.rs should avoid direct `.to_string()` hotspots after Cow migration.",
    );
    assert!(
        view_source.matches(".to_string()").count() <= 1,
        "view.rs should keep unavoidable string conversion hotspots minimal.",
    );
}

#[test]
fn color_loupe_tree_shaking_feature_gates_are_component_scoped() {
    let ui_components_cargo = load_source("ui_components_cargo");
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");

    assert!(
        ui_components_cargo.contains("component-color_loupe = [\"component-color_swatch\"]"),
        "ui-components Cargo feature tree should register color-loupe as component-scoped feature dependency.",
    );
    assert!(
        ui_components_cargo.contains("all-components = [")
            && ui_components_cargo.contains("\"component-color_loupe\","),
        "all-components aggregate should include color-loupe only through explicit feature list, not implicit module reachability.",
    );
    assert!(
        ui_components_lib.contains("#[cfg(feature = \"component-color_loupe\")]")
            && ui_components_lib
                .contains("#[path = \"../../../components/color-loupe/src/mod.rs\"]")
            && ui_components_lib.contains("pub mod color_loupe;")
            && ui_components_lib.contains("pub use crate::color_loupe as loupe;"),
        "ui-components lib.rs should gate color-loupe module and namespace export by feature.",
    );
    assert!(
        ui_components_css.contains("#[cfg(feature = \"component-color_loupe\")]")
            && ui_components_css.contains("out.push_str(crate::color::loupe::styles::CSS);"),
        "ui-components css.rs should gate color-loupe css aggregation by component feature.",
    );
}

#[test]
fn color_loupe_semantics_and_performance_regression_contract_is_covered() {
    let view_source = load_source("view");
    let docs_shell = load_source("docs_shell");
    let perf_check_script = load_source("perf_check_script");

    for required in [
        "role=\"img\"",
        "aria-label=move || aria_label.get_value()",
        "data-state=move || state.get().data_state_attr",
        "data-output-state=output_state.as_attr()",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep semantic regression marker `{required}`.",
        );
    }

    for forbidden in ["tabindex", "on:focus", "on:blur", "on:keydown", "NodeRef"] {
        assert!(
            !view_source.contains(forbidden),
            "snapshot role=img component should not introduce focus-flow surface `{forbidden}`.",
        );
    }

    for required in [
        "\"color-loupe\" => UiPerfBudget {",
        "max_mount_ms: 20.0,",
        "max_update_ms: Some(6.0),",
        "max_heap_kb: Some(320.0),",
    ] {
        assert!(
            docs_shell.contains(required),
            "docs shell should keep color-loupe perf budget token `{required}`.",
        );
    }

    assert!(
        perf_check_script.contains(
            "cargo test -p ui-components --test color_loupe_semantics color_loupe_performance_governance_contract_is_budgeted_traceable_and_blocking"
        ),
        "performance check script should keep color-loupe governance test gate.",
    );
}

#[test]
fn color_loupe_checklist_marks_ui_components_definition_complete() {
    let check2 = load_source("check2");
    assert!(
        check2.contains("- [x] `ui-components` 定义"),
        "color-loupe check2 should mark ui-components definition as completed.",
    );
    assert!(
        check2.contains("- [x] API 命名契约统一"),
        "color-loupe check2 should mark API naming contract as completed.",
    );
    assert!(
        check2.contains("- [x] 受控/非受控必须成对"),
        "color-loupe check2 should mark controlled/uncontrolled checklist item as completed (N/A for snapshot-only API).",
    );
    assert!(
        check2.contains("- [x] 默认值单一来源"),
        "color-loupe check2 should mark default-value normalization ownership item as completed.",
    );
    assert!(
        check2.contains("- [x] SSR 时空断裂治理"),
        "color-loupe check2 should mark hydration discontinuity guard as completed.",
    );
    assert!(
        check2.contains("- [x] SSR 与跨平台检查"),
        "color-loupe check2 should mark cross-platform compile contract as completed.",
    );
    assert!(
        check2.contains("- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护"),
        "color-loupe check2 should mark ui-headless web/ssr exclusion guard as completed.",
    );
    assert!(
        check2.contains("- [x] `ui-motion` 非 wasm 提供 no-op/stub"),
        "color-loupe check2 should mark ui-motion non-wasm no-op/stub contract as completed.",
    );
    assert!(
        check2.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支"),
        "color-loupe check2 should mark reduced-motion + SSR/wasm coverage as completed.",
    );
    assert!(
        check2.contains("- [x] 性能治理：关键路径有预算"),
        "color-loupe check2 should mark performance governance budget item as completed.",
    );
    assert!(
        check2.contains("- [x] `view!` 宏复杂度受控"),
        "color-loupe check2 should mark view macro complexity governance item as completed.",
    );
    assert!(
        check2.contains("- [x] 函数式拆分优先"),
        "color-loupe check2 should mark function-first split governance item as completed.",
    );
    assert!(
        check2.contains("- [x] 静态片段常量化"),
        "color-loupe check2 should mark static fragment constantization governance item as completed.",
    );
    assert!(
        check2.contains("- [x] `inner_html` 使用约束"),
        "color-loupe check2 should mark inner_html safety governance item as completed.",
    );
    assert!(
        check2.contains("- [x] WASM 调试要求"),
        "color-loupe check2 should mark wasm debug governance item as completed.",
    );
    assert!(
        check2.contains("- [x] DX 要求"),
        "color-loupe check2 should mark DX governance item as completed.",
    );
    assert!(
        check2.contains("- [x] 工程能力统一"),
        "color-loupe check2 should mark engineering governance item as completed.",
    );
    assert!(
        check2.contains("- [x] 样式孤岛防御（Defensive Variables）"),
        "color-loupe check2 should mark defensive variable styling governance item as completed.",
    );
    assert!(
        check2.contains("- [x] 级联层覆盖（`@layer ui`）"),
        "color-loupe check2 should mark css cascade layer governance item as completed.",
    );
    assert!(
        check2.contains("- [x] Motion 合同化"),
        "color-loupe check2 should mark motion contract governance item as completed.",
    );
    assert!(
        check2.contains("- [x] `ui-components` 固定入口文件落点正确"),
        "color-loupe check2 should mark ui-components entrypoint layout governance item as completed.",
    );
    assert!(
        check2.contains("- [x] 组件目录标准文件落点正确"),
        "color-loupe check2 should mark component directory standard layout governance item as completed.",
    );
    assert!(
        check2.contains("- [x] 文件落点纪律"),
        "color-loupe check2 should mark file-placement discipline governance item as completed.",
    );
    assert!(
        check2.contains("- [x] Hyper-Structure Builder（`spec.rs`）"),
        "color-loupe check2 should mark hyper-structure builder governance item as completed.",
    );
    assert!(
        check2.contains("- [x] 上下文压缩协议（Manifest + RBI）"),
        "color-loupe check2 should mark context-compression manifest+rbi governance item as completed.",
    );
    assert!(
        check2.contains("- [x] 语义标记统一升级为 Agent Contract（Schema 化）"),
        "color-loupe check2 should mark agent-contract schema governance item as completed.",
    );
    assert!(
        check2.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）"),
        "color-loupe check2 should mark streaming-vs-snapshot mode-definition item as completed.",
    );
    assert!(
        check2.contains("- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）"),
        "color-loupe check2 should mark snapshot-baseline capability item as completed.",
    );
    assert!(
        check2.contains("- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）"),
        "color-loupe check2 should mark streaming requirement scoping item as completed.",
    );
    assert!(
        check2.contains("- [x] 代码卫生（Rust Hygiene）"),
        "color-loupe check2 should mark rust-hygiene governance item as completed.",
    );
    assert!(
        check2.contains("- [x] Tree Shaking & 特性剪裁"),
        "color-loupe check2 should mark tree-shaking feature-gating governance item as completed.",
    );
    assert!(
        check2.contains("- [x] 语义测试与性能回归"),
        "color-loupe check2 should mark semantics and performance regression item as completed.",
    );
    assert!(
        check2.contains("- [x] 版本弃用迁移（Codemod/Registry）"),
        "color-loupe check2 should mark version-deprecation migration item as completed (N/A without breaking API upgrade).",
    );
    assert!(
        check2.contains("- [x] 文档即产品（Copy-Paste Ready）"),
        "color-loupe check2 should mark docs-as-product copy-paste-ready item as completed.",
    );
    assert!(
        check2.contains("- [x] 语义测试优先"),
        "color-loupe check2 should mark semantics-first verification item as completed.",
    );
    assert!(
        check2.contains("- [x] E2E 选择器稳定"),
        "color-loupe check2 should mark e2e stable semantic-selector item as completed.",
    );
    assert!(
        check2.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）"),
        "color-loupe check2 should mark repeatable e2e key-flow regression item as completed.",
    );
    assert!(
        check2.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "color-loupe check2 should mark docs-app examples + parameter/state matrix sync item as completed.",
    );
    assert!(
        check2.contains("- [x] 组件文档必须对新手友好（Documentation as Product）"),
        "color-loupe check2 should mark beginner-friendly component documentation item as completed.",
    );
    assert!(
        check2.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground"),
        "color-loupe check2 should mark docs-app interactive playground item as completed.",
    );
    assert!(
        check2.contains("- [x] Source-first 文档必须 Copy-Paste Ready"),
        "color-loupe check2 should mark source-first copy-paste-ready item as completed.",
    );
    assert!(
        check2.contains("- [x] HeroUI 对标文档与组件文档同步"),
        "color-loupe check2 should mark HeroUI alignment and docs-entry synchronization item as completed.",
    );
}
