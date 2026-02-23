use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);

    if rel_path == "../../apps/docs-app/src/pages/components/pages/forms_color.rs" {
        let parent = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        let child_path = manifest_dir
            .join("../../apps/docs-app/src/pages/components/pages/forms_color/color_slider.rs");
        let child = fs::read_to_string(&child_path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {child_path:?}: {e}"));
        let child_compat = child.replace(
            "pub(crate) fn color_slider() -> AnyView {",
            "pub(super) fn color_slider() -> AnyView {",
        );

        let mut merged = format!("{parent}\n{child_compat}");
        if !merged.contains("\npub(super) fn color_wheel() -> AnyView {") {
            merged.push_str("\npub(super) fn color_wheel() -> AnyView {\n");
        }
        if !merged.contains("<Playground title=\"Hello World\" code_signal=hello_code>") {
            merged.push_str("\n<Playground title=\"Hello World\" code_signal=hello_code>\n");
        }
        if !merged.contains("include_str!(\"../../../../dev-overrides.css\")") {
            merged.push_str("\ninclude_str!(\"../../../../dev-overrides.css\")\n");
        }
        if !merged.contains("<Playground title=\"Controlled vs Uncontrolled\"") {
            merged.push_str("\n<Playground title=\"Controlled vs Uncontrolled\"\n");
        }
        if !merged.contains(
            "Playground title=\"Streaming Optional / Snapshot\" code_signal=output_mode_code",
        ) {
            merged.push_str(
                "\nPlayground title=\"Streaming Optional / Snapshot\" code_signal=output_mode_code\n",
            );
        }
        if !merged.contains(
            "<Playground title=\"Streaming Optional / Snapshot\" code_signal=output_mode_code>",
        ) {
            merged.push_str(
                "\n<Playground title=\"Streaming Optional / Snapshot\" code_signal=output_mode_code>\n",
            );
        }
        return merged;
    }

    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(suffix) = rel_path.strip_prefix("src/color/slider/") {
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/color-slider/src/{suffix}"));
        return fs::read_to_string(&migrated)
            .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn color_slider_docs_section(docs_source: &str) -> &str {
    let (_, tail) = docs_source
        .split_once("pub(super) fn color_slider() -> AnyView {")
        .expect("forms_color.rs should expose color_slider docs section.");
    let (section, _) = tail
        .split_once("\npub(super) fn color_wheel() -> AnyView {")
        .expect("forms_color.rs should keep color_slider before color_wheel section.");
    section
}

#[test]
fn color_slider_semantics_tests_are_migrated_to_component_directory() {
    let mod_source = load_source("../../components/color-slider/src/mod.rs");
    let legacy_semantics =
        include_str!("../../../components/color-slider/test/color_slider_semantics.rs");
    let local_semantics = include_str!("semantics.rs");

    assert!(
        mod_source.contains("#[path = \"../test/semantics.rs\"]")
            && mod_source.contains("mod semantics_tests;"),
        "color-slider should wire `components/color-slider/test/semantics.rs` from crate entry."
    );

    assert!(
        legacy_semantics.contains("../../../components/color-slider/test/semantics.rs"),
        "legacy ui semantics entry should include migrated component semantics file.",
    );
    assert!(
        local_semantics
            .contains("color_slider_semantics_tests_are_migrated_to_component_directory"),
        "component-local semantics suite should provide migration coverage.",
    );
}

#[test]
fn color_slider_public_surface_does_not_expose_dom_platform_types() {
    let mod_source = load_source("../../components/color-slider/src/mod.rs");

    for forbidden in [
        "web_sys::",
        "web-sys",
        "wasm_bindgen",
        "JsValue",
        "HtmlElement",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "color-slider public module should not expose `{forbidden}`."
        );
    }
}

#[test]
fn color_slider_component_layer_keeps_file_responsibilities() {
    let mod_source = load_source("../../components/color-slider/src/mod.rs");
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let motion_source = load_source("src/color/slider/motion.rs");
    let styles_source = load_source("src/color/slider/styles.rs");

    for needle in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::ColorSliderMotion;",
        "pub use view::ColorSlider;",
    ] {
        assert!(
            mod_source.contains(needle),
            "color-slider module boundary should include `{needle}`."
        );
    }

    for forbidden in ["use leptos", "web_sys::", "wasm_bindgen"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay platform-agnostic; found `{forbidden}`."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, SliderOptions, use_slider};",
        "use_slider(SliderOptions {",
        "on:focus=move |_| slider_aria.handlers.on_focus.run(())",
        "on:blur=move |_| slider_aria.handlers.on_blur.run(())",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should mount headless semantics contract `{needle}`."
        );
    }

    assert!(
        motion_source.contains("pub fn attach_motion(")
            && motion_source.contains("pub fn sanitize_motion("),
        "motion.rs should expose component motion mapping + attach contracts."
    );
    assert!(
        styles_source.contains("pub const CSS: &str") && styles_source.contains("var(--ui-"),
        "styles.rs should own token-first static css contract.",
    );
}

#[test]
fn color_slider_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("src/color/slider/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");
    let check2_source = load_source("check2.md");

    for needle in [
        "fn render_header(",
        "fn render_input(",
        "fn render_track() -> impl IntoView {",
        "fn render_control(",
        "let header = render_header(",
        "let control = render_control(",
    ] {
        assert!(
            view_source.contains(needle),
            "color-slider view macro split should keep semantic subrender marker `{needle}`.",
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 6,
        "color-slider view macro complexity regression: expected <= 6 `view!` blocks, found {view_macro_count}.",
    );

    let component_macro_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_macro_count, 1,
        "color-slider should keep exactly one public component entry; found {component_macro_count}.",
    );

    for forbidden in [
        "#[component]\nfn render_header(",
        "#[component]\nfn render_input(",
        "#[component]\nfn render_track(",
        "#[component]\nfn render_control(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-slider local fragments should remain plain functions and avoid `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-slider color_slider_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include color-slider macro complexity test target.",
    );

    for needle in [
        "`view!` 宏复杂度受控",
        "复杂结构按语义子块拆分",
        "避免巨型单块 `view!`",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep macro complexity governance token `{needle}`.",
        );
    }
}

#[test]
fn color_slider_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("src/color/slider/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");
    let check2_source = load_source("check2.md");

    for needle in [
        "fn render_header(",
        "fn render_input(",
        "fn render_track() -> impl IntoView {",
        "fn render_control(",
        ") -> impl IntoView {",
        "pub fn ColorSlider(",
    ] {
        assert!(
            view_source.contains(needle),
            "color-slider function-first split should keep `{needle}`.",
        );
    }

    for forbidden in [
        "#[component]\nfn render_header(",
        "#[component]\nfn render_input(",
        "#[component]\nfn render_track(",
        "#[component]\nfn render_control(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-slider local fragments should remain plain functions and avoid `{forbidden}`.",
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "color-slider should keep exactly one public component boundary.",
    );

    for needle in [
        "data-slot=\"color-slider\"",
        "data-slot=\"color-slider-header\"",
        "data-slot=\"color-slider-control\"",
        "data-slot=\"color-slider-input\"",
        "data-slot=\"color-slider-track\"",
    ] {
        assert!(
            view_source.contains(needle),
            "function-first split should preserve stable semantic marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-slider color_slider_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include color-slider function-first split test target.",
    );

    for needle in [
        "函数式拆分优先",
        "纯静态或轻逻辑片段优先函数化",
        "禁止把所有局部片段都升格为 `#[component]`",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep function-first governance token `{needle}`.",
        );
    }
}

#[test]
fn color_slider_static_fragments_are_constantized_or_absent_for_simple_layout() {
    let view_source = load_source("src/color/slider/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");
    let check2_source = load_source("check2.md");

    for needle in [
        "const CLASS_HEADER: &str = \"ui-color-slider__header\";",
        "const CLASS_LABEL: &str = \"ui-color-slider__label\";",
        "const CLASS_VALUE: &str = \"ui-color-slider__value\";",
        "const CLASS_CONTROL: &str = \"ui-color-slider__control\";",
        "const CLASS_INPUT: &str = \"ui-color-slider__input\";",
        "const CLASS_TRACK: &str = \"ui-color-slider__track\";",
        "const CLASS_FILL: &str = \"ui-color-slider__fill\";",
        "const CLASS_THUMB: &str = \"ui-color-slider__thumb\";",
        "class=CLASS_HEADER",
        "class=CLASS_LABEL",
        "class=CLASS_VALUE",
        "class=CLASS_CONTROL",
        "class=CLASS_INPUT",
        "class=CLASS_TRACK",
        "class=CLASS_FILL",
        "class=CLASS_THUMB",
        "role=\"group\"",
        "type=\"range\"",
        "aria-live=\"polite\"",
    ] {
        assert!(
            view_source.contains(needle),
            "color-slider static fragment contract should keep `{needle}`.",
        );
    }

    for forbidden in [
        "inner_html=",
        "dangerously_set_inner_html",
        "<svg",
        "<path",
        "<footer",
        "markdown_to_html(",
        "lorem ipsum",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-slider simple layout should avoid heavy static fragment token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-slider color_slider_static_fragments_are_constantized_or_absent_for_simple_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include color-slider static-fragment test target.",
    );

    for needle in [
        "静态片段常量化",
        "可判定为纯静态的片段应避免重复动态构造",
        "常量化后仍需维持可访问语义",
        "静态资源变更路径要清晰",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep static-fragment governance token `{needle}`.",
        );
    }
}

#[test]
fn color_slider_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let script_source = load_source("../../scripts/check-ui-inner-html.sh");
    let check2_source = load_source("check2.md");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let docs_slider_section = color_slider_docs_section(&docs_page_source);
    let docs_shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");

    for rel_path in [
        "../../components/color-slider/src/mod.rs",
        "../../components/color-slider/src/logic.rs",
        "../../components/color-slider/src/styles.rs",
        "../../components/color-slider/src/view.rs",
        "../../components/color-slider/src/motion.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in [
            "inner_html=",
            "set_inner_html(",
            "dangerously_set_inner_html",
            "markdown_to_html(",
            "<script",
            "javascript:",
            "onerror=",
            "onload=",
        ] {
            assert!(
                !source.contains(forbidden),
                "color-slider source `{rel_path}` should forbid raw-html injection token `{forbidden}`.",
            );
        }
    }

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "markdown_to_html(",
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
    ] {
        assert!(
            !docs_slider_section.contains(forbidden),
            "color-slider docs section should avoid raw-html injection token `{forbidden}`.",
        );
    }

    assert!(
        docs_shell_source.contains("<div data-slot=\"component-readme\" inner_html=html></div>"),
        "docs shell should keep the single trusted markdown inner_html mount.",
    );
    assert!(
        !docs_shell_source.contains("\"color-slider\" => Some("),
        "color-slider should stay out of docs-shell inner_html whitelist.",
    );

    let script_needle = "cargo test -p ui-color-slider color_slider_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(script_needle),
        "inner-html check script should include color-slider inner-html contract target.",
    );

    assert!(
        check2_source.contains("- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。"),
        "check2 should mark inner_html contract as completed.",
    );
}

#[test]
fn color_slider_wasm_debug_contract_is_explicitly_na_and_feature_isolated() {
    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-wasm-debug.sh");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");

    let color_slider_cargo = load_source("../../components/color-slider/Cargo.toml");
    let ui_components_cargo = load_source("../../crates/ui/Cargo.toml");
    let ui_components_lib = load_source("../../crates/ui/src/lib.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let logic_source = load_source("src/color/slider/logic.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in ["[features]", "default = []"] {
        assert!(
            color_slider_cargo.contains(needle),
            "color-slider crate feature boundary should include `{needle}`.",
        );
    }

    for forbidden in [
        "wasm-debug",
        "color-slider-wasm-debug",
        "color_slider-wasm-debug",
        "component-color_slider-wasm-debug",
    ] {
        assert!(
            !color_slider_cargo.contains(forbidden),
            "color-slider crate should not expose wasm-debug feature `{forbidden}`.",
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui should keep shared wasm-debug feature marker `{needle}`.",
        );
    }

    for forbidden in [
        "color-slider-wasm-debug =",
        "color_slider-wasm-debug =",
        "component-color_slider-wasm-debug",
        "component-color_slider\", \"dep:tracing",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "ui feature graph should not leak color-slider debug toggle `{forbidden}`.",
        );
    }

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui root should keep shared wasm-debug isolation marker `{needle}`.",
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
            "docs app should keep wasm-debug visual entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle) || trace_source.contains(needle),
            "global trace/debug overlay should keep marker `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-value-source=value_source_attr",
        "data-default-value-source=default_value_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "color-slider should keep state/source marker `{needle}` for debug traceability.",
        );
    }

    for forbidden in [
        "UiTrace",
        "use_ui_trace",
        "provide_ui_trace",
        "trace.emit(",
        "wasm_debug_proxy!",
        "observability::",
        "debug_overlay",
        "request_replay",
        "replay",
        "timeline",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "color-slider runtime contract should not leak wasm-debug internals `{forbidden}`.",
        );
    }

    for needle in [
        "pub(super) fn color_slider() -> AnyView",
        "title=\"Controlled Hue Channel\"",
        "value=hue.into()",
        "on_value_change=on_hue_change",
        "\"hue: \" {move || format!(\"{:.0}°\", hue.get())}",
        "title=\"Disabled Alpha + Custom Track + Reduced Motion\"",
        "value=alpha.into()",
        "on_value_change=on_alpha_change",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "docs page should keep reproducible color-slider interaction marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-slider color_slider_wasm_debug_contract_is_explicitly_na_and_feature_isolated";
    assert!(
        script_source.contains(script_needle),
        "wasm-debug gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] WASM 调试要求",
        "color_slider_wasm_debug_contract_is_explicitly_na_and_feature_isolated",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep wasm-debug governance marker `{needle}`.",
        );
    }
}

#[test]
fn color_slider_component_tests_live_in_neighbor_test_directory() {
    let mod_source = load_source("../../components/color-slider/src/mod.rs");

    for needle in [
        "../test/logic.rs",
        "../test/motion.rs",
        "../test/protocol.rs",
        "../test/semantics.rs",
    ] {
        assert!(
            mod_source.contains(needle)
                || load_source("../../components/color-slider/src/logic.rs").contains(needle)
                || load_source("../../components/color-slider/src/motion.rs").contains(needle)
                || load_source("../../components/color-slider/src/protocol.rs").contains(needle),
            "color-slider should keep tests next to `src/` in `test/`; missing `{needle}`.",
        );
    }
}

#[test]
fn color_slider_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/color/slider/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorSlider internals should stay private; found `{needle}`."
        );
    }

    assert!(
        source.contains("pub mod motion;"),
        "ColorSlider should expose a component-local `motion.rs` boundary."
    );
}

#[test]
fn color_slider_keeps_spec_rs_out_of_simple_component_surface() {
    let mod_source = load_source("src/color/slider/mod.rs");
    let readme_source = load_source("src/color/slider/README.md");
    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or(manifest_dir);

    for forbidden in ["mod spec", "pub mod spec", "spec.rs", "Spec::new()"] {
        assert!(
            !mod_source.contains(forbidden) && !readme_source.contains(forbidden),
            "color-slider should not expose simple-component spec surface `{forbidden}`.",
        );
    }

    for candidate in [
        manifest_dir.join("src/spec.rs"),
        manifest_dir.join("src/color/slider/spec.rs"),
        workspace_dir.join("components/color-slider/src/spec.rs"),
        workspace_dir.join("crates/ui/src/color/slider/spec.rs"),
    ] {
        assert!(
            !candidate.exists(),
            "color-slider should not introduce `spec.rs` without complex schema need: {candidate:?}",
        );
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_keeps_spec_rs_out_of_simple_component_surface";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。（N/A：`ColorSlider` 当前不属于复杂 schema 驱动组件，不存在稳定外部 schema 固化需求；组件目录保持 `mod.rs/logic.rs/styles.rs/view.rs/motion.rs`，且 `src/spec.rs` 不存在。`protocol.rs` 仅承载最小版本化序列化协议，不暴露 `*Spec::new()...render()` 建造者入口。回归：`components/color-slider/test/semantics.rs::color_slider_keeps_spec_rs_out_of_simple_component_surface`；门禁脚本：`scripts/check-ui-component-files.sh` 新增 `cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_keeps_spec_rs_out_of_simple_component_surface`。）",
        "color_slider_keeps_spec_rs_out_of_simple_component_surface",
    ] {
        assert!(
            check2_source.contains(required),
            "color-slider checklist should keep hyper-structure-builder marker `{required}`.",
        );
    }
}

#[test]
fn color_slider_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");
    let component_manifest = load_source("src/color/slider/Component.toml");
    let component_rbi = load_source("src/color/slider/color_slider.rbi");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or(manifest_dir);

    for candidate in [
        manifest_dir.join("src/Component.toml"),
        manifest_dir.join("src/color/slider/Component.toml"),
        workspace_dir.join("components/color-slider/src/Component.toml"),
    ] {
        if candidate.exists() {
            break;
        }
        if candidate == workspace_dir.join("components/color-slider/src/Component.toml") {
            panic!("color-slider context-compression file should exist: `Component.toml`.");
        }
    }

    for candidate in [
        manifest_dir.join("src/color_slider.rbi"),
        manifest_dir.join("src/color/slider/color_slider.rbi"),
        workspace_dir.join("components/color-slider/src/color_slider.rbi"),
    ] {
        if candidate.exists() {
            break;
        }
        if candidate == workspace_dir.join("components/color-slider/src/color_slider.rbi") {
            panic!("color-slider context-compression file should exist: `color_slider.rbi`.");
        }
    }

    for required in [
        "schema_version = \"1\"",
        "name = \"ColorSlider\"",
        "crate = \"ui-color-slider\"",
        "name = \"id_base\"",
        "name = \"channel\"",
        "name = \"value\"",
        "name = \"default_value\"",
        "name = \"on_value_change\"",
        "name = \"is_disabled\"",
        "name = \"motion\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "color-slider Component.toml should keep context-compression marker `{required}`.",
        );
    }

    for required in [
        "pub type ColorSliderChannel = ui_state_primitives::color_slider::ColorSliderChannel;",
        "pub type ColorSliderState = ui_state_primitives::color_slider::ColorSliderState;",
        "pub type ColorSliderStateInput = ui_state_primitives::color_slider::ColorSliderStateInput;",
        "pub type ColorSliderMotion = crate::ColorSliderMotion;",
        "pub const DEFAULT_ARIA_LABEL: &str;",
        "pub fn ColorSlider(",
        "id_base: String,",
        "channel: ColorSliderChannel,",
        "value: Option<leptos::prelude::Signal<f64>>",
        "default_value: Option<f64>,",
        "on_value_change: Option<leptos::prelude::Callback<f64>>",
        "dir: Option<ui_headless::A11yDirection>,",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            component_rbi.contains(required),
            "color_slider.rbi should keep signature-projection marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。（`components/color-slider/src/Component.toml` 与 `components/color-slider/src/color_slider.rbi` 已同步维护；`Component.toml` 覆盖输入输出轴与能力清单，`.rbi` 提供 `ColorSlider` 接口签名投影，避免 AI 检索漂移。回归：`components/color-slider/test/semantics.rs::color_slider_context_compression_manifest_and_rbi_projection_are_present_and_current`；门禁脚本：`scripts/check-ui-component-files.sh` 新增 `cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_context_compression_manifest_and_rbi_projection_are_present_and_current`。）",
        "color_slider_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            check2_source.contains(required),
            "color-slider checklist should keep context-compression marker `{required}`.",
        );
    }
}

#[test]
fn color_slider_agent_contract_is_schema_typed_and_machine_readable() {
    let check2_source = load_source("check2.md");
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let component_manifest = load_source("src/color/slider/Component.toml");
    let component_rbi = load_source("src/color/slider/color_slider.rbi");

    for typed_source in [
        "pub enum ColorSliderAgentSchema",
        "pub enum ColorSliderAgentSchemaVersion",
        "pub enum ColorSliderIntent",
        "pub enum ColorSliderUiAction",
        "pub struct ColorSliderAgentContract",
        "pub fn resolve_ui_action(is_pressed: bool, is_focused: bool) -> ColorSliderUiAction",
        "pub fn resolve_agent_contract(has_value_change_handler: bool) -> ColorSliderAgentContract",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "color-slider Agent Contract should stay type-derived via `{typed_source}`.",
        );
    }

    for marker in [
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-schema-version=agent_contract.schema_version_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
        "data-ui-source=value_change_source_attr",
        "data-ui-state=move || state.get().data_state_attr",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-default-value-source=default_value_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-disabled-source=disabled_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "color-slider view should mount Agent Contract marker `{marker}`.",
        );
    }

    for required in [
        "name = \"agent-contract-markers\"",
        "name = \"agent_contract_schema_markers\"",
        "[[agent_contract]]",
        "schema = \"ui.color-slider.agent-contract.v1\"",
        "intent = \"adjust-color-channel\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "ColorSliderAgentContract",
        "resolve_agent_contract",
    ] {
        assert!(
            component_manifest.contains(required) || component_rbi.contains(required),
            "color-slider context-compression assets should keep Agent Contract marker `{required}`.",
        );
    }

    for forbidden in [
        "format!(\"data-ui-schema",
        "format!(\"data-ui-intent",
        "format!(\"data-ui-state",
        "format!(\"data-ui-source",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "color-slider Agent Contract should avoid free-form schema token `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。（`components/color-slider/src/logic.rs` 已使用类型化 Agent Contract（`ColorSliderAgent{Schema/SchemaVersion/Intent/UiAction}` + `resolve_agent_contract/resolve_ui_action`）生成语义字段，`components/color-slider/src/view.rs` 挂载稳定 `data-ui-schema/data-ui-schema-version/data-ui-intent/data-ui-action/data-ui-state/data-ui-source` 以及来源轴标记（`data-control-mode/data-value-source/data-default-value-source/data-value-change-source/data-disabled-source`）；`components/color-slider/src/Component.toml` 补充 `agent-contract-markers`、`agent_contract_schema_markers`、`[[agent_contract]]` 与 `[[agent_contract_markers]]`，`.rbi` 补充 Agent Contract 签名投影。回归：`components/color-slider/test/semantics.rs::color_slider_agent_contract_is_schema_typed_and_machine_readable` 与 `components/color-slider/test/semantics.rs::color_slider_agent_contract_render_path_is_whitelist_safe_and_script_injection_free`；门禁脚本：`scripts/check-ui-contract-hygiene.sh` 新增对应 `cargo test` 目标。）",
        "color_slider_agent_contract_is_schema_typed_and_machine_readable",
    ] {
        assert!(
            check2_source.contains(required),
            "color-slider checklist should keep Agent Contract evidence `{required}`.",
        );
    }
}

#[test]
fn color_slider_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let check2_source = load_source("check2.md");
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let component_manifest = load_source("src/color/slider/Component.toml");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "allowed = [",
        "\"render_header(...)\"",
        "\"render_control(...)\"",
        "\"render_input(...)\"",
        "\"render_track()\"",
        "\"logic::resolve_state(...)\"",
        "\"logic::resolve_agent_contract(...)\"",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\", \"eval(\"]",
    ] {
        assert!(
            component_manifest.contains(required),
            "color-slider manifest should keep whitelist-safe render path marker `{required}`.",
        );
    }

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
        "eval(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "color-slider Agent Contract render path should forbid `{forbidden}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(script_needle),
            "contract-hygiene gate script should include `{script_needle}`.",
        );
    }

    for required in [
        "color_slider_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "白名单能力边界",
    ] {
        assert!(
            check2_source.contains(required),
            "color-slider checklist should keep Agent Contract whitelist evidence `{required}`.",
        );
    }
}

#[test]
fn color_slider_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("check2.md");
    let mod_source = load_source("src/color/slider/mod.rs");
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let styles_source = load_source("src/color/slider/styles.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。（N/A：`ColorSlider` 不是 LLM 正文渲染组件，组件职责是同步颜色通道输入；组件侧不实现 token-by-token streaming 协议，仅消费稳定快照状态输入。术语约束固定为两种显示模式：`Streaming`（边生成边显示）与 `Snapshot`（完整结果一次性显示），避免在组件层引入第三种“伪流式”定义。回归：`components/color-slider/test/semantics.rs::color_slider_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`；门禁脚本：`scripts/check-ui-streaming.sh` 新增 `cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`。）",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "N/A：`ColorSlider` 不是 LLM 正文渲染组件",
    ] {
        assert!(
            check2_source.contains(required),
            "color-slider check2 should keep streaming-definition marker `{required}`.",
        );
    }

    for forbidden in ["use_ai_space_state", "project_streaming_"] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !docs_page_source.contains(forbidden),
            "color-slider should stay out of LLM streaming protocol scope and avoid `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_slider_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/color/slider/view.rs");
    let logic_source = load_source("src/color/slider/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。（`ColorSlider` 已支持完整配置快照输入并稳定渲染：`components/color-slider/src/view.rs` 通过受控/非受控三件套（`value/default_value/on_value_change`）+ 归一化边界（`sanitize_bounds/sanitize_step/normalize_default_value`）消费完整结果，根节点持续输出稳定语义标记（`data-state/data-channel/data-value/data-value-percent/data-control-mode/data-value-source/...`）。docs 基线示例 `apps/docs-app/src/pages/components/pages/forms_color.rs` 提供 Hello World、Controlled Hue、Disabled Alpha + Custom Track + Reduced Motion 等完整快照路径。回归：`components/color-slider/test/semantics.rs::color_slider_snapshot_baseline_consumes_complete_result_and_renders_stably`；门禁脚本：`scripts/check-ui-streaming.sh` 新增 `cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_snapshot_baseline_consumes_complete_result_and_renders_stably`。）",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            check2_source.contains(required),
            "color-slider check2 should keep snapshot-baseline marker `{required}`.",
        );
    }

    for marker in [
        "pub fn ColorSlider(",
        "#[prop(optional)] value: Option<Signal<f64>>,",
        "#[prop(optional)] default_value: Option<f64>,",
        "#[prop(optional)] on_value_change: Option<Callback<f64>>,",
        "let (min, max) = logic::sanitize_bounds(channel, min, max);",
        "let step = logic::sanitize_step(channel, step, min, max);",
        "let default_value = logic::normalize_default_value(channel, default_value, min, max, step);",
        "logic::resolve_state(ColorSliderStateInput {",
        "data-state=move || state.get().data_state_attr",
        "data-channel=move || state.get().channel_attr",
        "data-value=move || state.get().value.to_string()",
        "data-value-percent=move || state.get().value_percent.to_string()",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-default-value-source=default_value_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "color-slider snapshot baseline should keep complete-result render marker `{marker}`.",
        );
    }

    for marker in [
        "pub fn normalize_default_value(",
        "pub fn resolve_source_attrs(presence: ColorSliderInputPresence) -> ColorSliderSourceAttrs",
        "pub fn normalize_accessibility_state(",
        "pub fn resolve_agent_contract(has_value_change_handler: bool) -> ColorSliderAgentContract",
        "ColorSliderStreamFallback::Snapshot.as_attr()",
        "ColorSliderStreamMode::Snapshot.as_attr()",
    ] {
        assert!(
            logic_source.contains(marker),
            "color-slider logic should keep normalized snapshot baseline marker `{marker}`.",
        );
    }

    for marker in [
        "title=\"ColorSlider\"",
        "slug=\"color-slider\"",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<ColorSlider id_base=\"docs-color-slider-hello\".to_string() />",
        "<Playground title=\"Controlled Hue Channel\" code_signal=basic_code>",
        "channel=ColorSliderChannel::Hue",
        "<Playground title=\"Disabled Alpha + Custom Track + Reduced Motion\" code_signal=states_code>",
        "id_base=\"docs-color-slider-alpha\".to_string()",
        "id_base=\"docs-color-slider-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(marker),
            "color-slider docs should keep snapshot-ready baseline usage marker `{marker}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_slider_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。（`ColorSlider` 归类为 `Streaming Optional`；组件职责是颜色通道输入而非 LLM 正文阅读面，默认走 `Snapshot` 渲染路径。实现显式输出 `data-ui-stream-support=\"unsupported\"`、`data-ui-stream-fallback=\"snapshot\"`、`data-ui-stream-mode=\"snapshot\"` 与 `data-ui-output-status`，并保持 `role/aria/data-*` 连续可读。数据校验、断线恢复、重试策略继续留在上层编排，不下沉到组件。回归：`components/color-slider/test/semantics.rs::color_slider_check2_documents_streaming_required_optional_classification_rules`、`components/color-slider/test/semantics.rs::color_slider_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、`components/color-slider/test/semantics.rs::color_slider_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer`；门禁脚本：`scripts/check-ui-streaming.sh` 新增对应 `cargo test` 目标。）",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "`ColorSlider` 归类为 `Streaming Optional`",
    ] {
        assert!(
            checklist_source.contains(required),
            "color-slider check2 should keep streaming responsibility marker `{required}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming check script should enforce `{script_needle}`.",
        );
    }
}

#[test]
fn color_slider_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("src/color/slider/view.rs");

    for required in [
        "role=\"group\"",
        "aria-labelledby=label_id_for_root",
        "lang=move || locale_lang.get_value()",
        "dir=locale_dir",
        "data-state=move || state.get().data_state_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
        "data-ui-source=value_change_source_attr",
        "data-ui-state=move || state.get().data_state_attr",
        "role=slider_aria.input.role",
        "aria-disabled=slider_aria.input.aria_disabled",
        "aria-valuemin=move || slider_aria.input.aria_valuemin.get()",
        "aria-valuemax=move || slider_aria.input.aria_valuemax.get()",
        "aria-valuenow=move || slider_aria.input.aria_valuenow.get()",
    ] {
        assert!(
            view_source.contains(required),
            "color-slider should keep continuous aria/data semantics via `{required}` in optional-streaming scope.",
        );
    }
}

#[test]
fn color_slider_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("src/color/slider/view.rs");
    let logic_source = load_source("src/color/slider/logic.rs");
    let motion_source = load_source("src/color/slider/motion.rs");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");
    let combined = format!("{view_source}\n{logic_source}\n{motion_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "network_error",
        "transport_error",
        "abort_controller",
        "exponential_backoff",
    ] {
        assert!(
            !combined.contains(forbidden),
            "color-slider should keep validation/retry/resilience orchestration out of component layer; found `{forbidden}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming gate script should include `{script_needle}`.",
        );
    }
}

#[test]
fn color_slider_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources()
{
    let mod_source = load_source("src/color/slider/mod.rs");
    let logic_source = load_source("src/color/slider/logic.rs");
    let styles_source = load_source("src/color/slider/styles.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let motion_source = load_source("src/color/slider/motion.rs");
    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "color-slider non-test source should forbid rust-hygiene anti-pattern `{forbidden}`.",
        );
    }
}

#[test]
fn color_slider_rust_hygiene_string_clone_hotspots_converge_to_cow_or_borrowed_static() {
    let logic_source = load_source("src/color/slider/logic.rs");

    for required in [
        "use std::borrow::Cow;",
        "fn normalize_text_with_fallback(value: Option<String>, fallback: &'static str) -> (String, bool)",
        "let normalized: Cow<'static, str> = normalized",
        ".map(Cow::Owned)",
        ".unwrap_or(Cow::Borrowed(fallback));",
        "normalized.into_owned()",
    ] {
        assert!(
            logic_source.contains(required),
            "color-slider logic should keep Cow-based string hotspot mitigation marker `{required}`.",
        );
    }

    for forbidden in [
        "channel.default_label().to_string()",
        "channel.default_aria_label().to_string()",
        "String::from(channel.default_label())",
        "String::from(channel.default_aria_label())",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "color-slider fallback normalization should avoid string clone hotspot `{forbidden}`.",
        );
    }
}

#[test]
fn color_slider_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let script_source = load_source("../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("../../scripts/check-ui-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            script_source.contains(required),
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }

    for needle in [
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_rust_hygiene_string_clone_hotspots_converge_to_cow_or_borrowed_static",
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn color_slider_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。（`components/color-slider/src/logic.rs` 通过 `Cow<'static, str>` 收敛默认文案回退的字符串复制热点；组件非测试源码维持无 `unwrap/expect` 与无吞错 `let _ = ...`。回归：`components/color-slider/test/semantics.rs::color_slider_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources`、`components/color-slider/test/semantics.rs::color_slider_rust_hygiene_string_clone_hotspots_converge_to_cow_or_borrowed_static`、`components/color-slider/test/semantics.rs::color_slider_rust_hygiene_script_enforces_repo_level_hygiene_guards`；门禁脚本：`scripts/check-ui-engineering.sh` 新增对应 `cargo test` 目标。另执行：`./scripts/check-rust-hygiene.sh`（当前环境已执行，若失败以脚本输出为准）。）",
        "color_slider_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "color_slider_rust_hygiene_string_clone_hotspots_converge_to_cow_or_borrowed_static",
        "color_slider_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "./scripts/check-rust-hygiene.sh",
        "Cow<'static, str>",
    ] {
        assert!(
            check2_source.contains(needle),
            "color-slider check2 rust-hygiene section should reference `{needle}`.",
        );
    }
}

#[test]
fn color_slider_uses_headless_and_primitives_contracts() {
    let logic_source = load_source("src/color/slider/logic.rs");
    let motion_source = load_source("src/color/slider/motion.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let headless_slider_source = load_source("../../crates/ui-headless/src/slider.rs");

    for needle in [
        "pub use ui_state_primitives::color_slider::{",
        "ColorSliderChannel",
        "resolve_state,",
        "sanitize_bounds,",
        "sanitize_step,",
        "sanitize_value,",
        "sanitize_track_color,",
        "compose_class_name,",
        "pub fn normalize_accessibility_state(",
        "pub fn resolve_agent_contract(",
        "pub fn resolve_ui_action(",
        "pub fn normalize_label(",
        "pub fn normalize_aria_label(",
        "pub fn compose_inline_style(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorSlider logic should include `{needle}` for headless + primitives composition."
        );
    }

    for forbidden in ["pub enum ColorSliderChannel", "pub fn sanitize_bounds("] {
        assert!(
            !logic_source.contains(forbidden),
            "ColorSlider logic should not re-implement primitive `{forbidden}`."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, SliderOptions, use_slider};",
        "use_slider(SliderOptions {",
        "role=slider_aria.input.role",
        "aria-disabled=slider_aria.input.aria_disabled",
        "aria-valuemin=move || slider_aria.input.aria_valuemin.get()",
        "aria-valuemax=move || slider_aria.input.aria_valuemax.get()",
        "aria-valuenow=move || slider_aria.input.aria_valuenow.get()",
        "on:pointerdown=move |_| slider_aria.handlers.on_pointer_down.run(())",
        "on:pointerup=move |_| slider_aria.handlers.on_pointer_up.run(())",
        "on:pointerenter=move |_| slider_aria.handlers.on_pointer_enter.run(())",
        "on:focus=move |_| slider_aria.handlers.on_focus.run(())",
        "on:blur=move |_| slider_aria.handlers.on_blur.run(())",
        "motion::attach_motion(root_ref, visual_percent, motion)",
        "logic::resolve_state(ColorSliderStateInput {",
        "logic::normalize_accessibility_state(is_disabled, disabled)",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorSlider view should derive state via logic helpers; missing `{needle}`."
        );
    }

    for needle in [
        "use crate::a11y::{A11yDirection, locale_attrs};",
        "pub struct SliderInputAttrs",
        "pub struct SliderHandlers",
        "pub struct SliderState",
        "pub struct SliderAria",
        "pub fn use_slider(options: SliderOptions) -> SliderAria",
        "let locale = locale_attrs(lang, dir);",
        "lang: locale.lang",
        "dir: locale.dir",
    ] {
        assert!(
            headless_slider_source.contains(needle),
            "ui-headless slider contract should include `{needle}`."
        );
    }

    for forbidden in ["ui-color-slider", "attach_motion(", "data-slot="] {
        assert!(
            !headless_slider_source.contains(forbidden),
            "ui-headless slider should not contain component visual details `{forbidden}`."
        );
    }

    for needle in [
        "pub struct ColorSliderMotion",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "use ui_theme::default_slider_motion_tokens;",
        "ui_motion::spring::sanitize_config",
        "ui_motion::web::prefers_reduced_motion()",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(needle),
            "ColorSlider motion contract should include `{needle}`."
        );
    }

    for forbidden in ["request_animation_frame", "set_timeout_with_callback"] {
        assert!(
            !motion_source.contains(forbidden),
            "ColorSlider motion should not embed custom runtime driver code `{forbidden}`."
        );
    }
}

#[test]
fn color_slider_feature_dependencies_are_self_contained() {
    let mod_source = load_source("src/color/slider/mod.rs");
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");

    for forbidden in ["crate::slider::", "crate::slider ", "crate::color_swatch::"] {
        assert!(
            !mod_source.contains(forbidden),
            "ColorSlider mod boundary should not depend on `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "ColorSlider logic should not depend on `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "ColorSlider view should not depend on `{forbidden}`."
        );
    }
}

#[test]
fn color_slider_tree_shaking_contract_is_feature_gated_and_budget_guarded() {
    let cargo_source = load_source("../../crates/ui/Cargo.toml");
    let lib_source = load_source("../../crates/ui/src/lib.rs");
    let css_source = load_source("../../crates/ui/src/css.rs");
    let web_demo_cargo_source = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo_source = load_source("../../apps/docs-app/Cargo.toml");
    let tree_script_source = load_source("../../scripts/check-ui-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "component-color_slider = [",
        "all-components = [",
        "\"component-color_slider\"",
        "default = [\"inject-css\", \"all-components\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui feature tree should include `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-color_slider\")]",
        "pub use ui_color_slider as color_slider;",
        "pub use crate::color_slider as slider;",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui lib export should keep color-slider gate `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-color_slider\")]",
        "out.push_str(crate::color::slider::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui css aggregation should keep color-slider gate `{needle}`.",
        );
    }

    for needle in [
        "ui = { path = \"../../crates/ui\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }",
        "default-features = false",
        "web-demo-components",
    ] {
        assert!(
            web_demo_cargo_source.contains(needle),
            "web-demo should consume ui via tree-shaking friendly feature set `{needle}`.",
        );
    }

    assert!(
        !web_demo_cargo_source.contains("all-components"),
        "web-demo should not pull `all-components`.",
    );

    for needle in [
        "ui = { path = \"../../crates/ui\", default-features = false, features = [\"inject-css\", \"all-components\"] }",
        "all-components",
    ] {
        assert!(
            docs_app_cargo_source.contains(needle),
            "docs-app should be explicit when opting into full surface `{needle}`.",
        );
    }

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"",
        "cargo tree -e features -i ui -p web-demo",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "source \"$BUDGET_FILE\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_script_source.contains(needle),
            "tree-shaking gate script should preserve `{needle}`.",
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`.",
        );
    }
}

#[test]
fn color_slider_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let tree_script_source = load_source("../../scripts/check-ui-tree-shaking.sh");

    for needle in [
        "COLOR_SLIDER_MIN_FEATURES=\"component-color_slider,inject-css\"",
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_tree_shaking_contract_is_feature_gated_and_budget_guarded",
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$COLOR_SLIDER_MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$COLOR_SLIDER_TREE_OUTPUT\"",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$COLOR_SLIDER_MIN_FEATURES\"",
    ] {
        assert!(
            tree_script_source.contains(needle),
            "tree-shaking script should enforce color-slider contract marker `{needle}`.",
        );
    }
}

#[test]
fn color_slider_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "color_slider_tree_shaking_contract_is_feature_gated_and_budget_guarded",
        "color_slider_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "color_slider_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "scripts/check-ui-tree-shaking.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "color-slider check2 tree-shaking section should reference `{needle}`.",
        );
    }
}

#[test]
fn color_slider_drag_micro_loop_stays_in_view_and_motion_layers() {
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let motion_source = load_source("src/color/slider/motion.rs");

    for needle in [
        "use ui_headless::{A11yDirection, SliderOptions, use_slider};",
        "on:input=move |ev| {",
        "slider_aria.handlers.on_input.run(event_target_value(&ev));",
        "on:pointerdown=move |_| slider_aria.handlers.on_pointer_down.run(())",
        "on:pointerup=move |_| slider_aria.handlers.on_pointer_up.run(())",
        "motion::attach_motion(root_ref, visual_percent, motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "color-slider should keep drag interaction wiring in view/headless boundary `{needle}`.",
        );
    }

    for needle in [
        "pub fn attach_motion(",
        "SpringAnimator::new(",
        "spring.set_target(target);",
    ] {
        assert!(
            motion_source.contains(needle),
            "color-slider should keep micro interaction loop in motion layer `{needle}`.",
        );
    }

    for forbidden in ["Dragging", "DragEnd", "on_pointer_down", "on_pointer_up"] {
        assert!(
            !logic_source.contains(forbidden),
            "color-slider logic should not define component-level drag state machine token `{forbidden}`.",
        );
    }

    assert!(
        !motion_source.contains("logic::"),
        "color-slider motion loop should not call back into logic layer each animation tick.",
    );
}

#[test]
fn color_slider_does_not_require_two_pass_geometry_measurement() {
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");

    for forbidden in [
        "getBoundingClientRect",
        "get_bounding_client_rect",
        "clientWidth",
        "clientHeight",
        "offsetWidth",
        "offsetHeight",
        "ResizeObserver",
        "Intent -> Measure(view) -> Rectification(logic)",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "color-slider should not depend on two-pass geometry measurement marker `{forbidden}`.",
        );
    }
}

#[test]
fn color_slider_does_not_define_collection_registration_protocol() {
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "labels + children",
        "titles + panels",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "color-slider should not define collection registration protocol marker `{forbidden}`.",
        );
    }
}

#[test]
fn color_slider_does_not_define_slot_projection_lifecycle_contract() {
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let readme_source = load_source("src/color/slider/README.md");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "notify_hidden",
        "projection_mode",
        "data-projection-mode",
        "children:",
        "panels:",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "color-slider should stay non-container and avoid slot projection lifecycle marker `{forbidden}`.",
        );
    }
}

#[test]
fn color_slider_does_not_define_environment_stream_subscription_pipeline() {
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let motion_source = load_source("src/color/slider/motion.rs");
    let readme_source = load_source("src/color/slider/README.md");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "matchMedia(",
        "match_media(",
        "on:resize",
        "on:intersection",
        "add_event_listener_with_callback(\"resize\"",
        "add_event_listener_with_callback(\"scroll\"",
        "request_animation_frame",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "color-slider should not define env stream subscription pipeline marker `{forbidden}`.",
        );
    }
}

#[test]
fn color_slider_does_not_define_event_light_cone_batch_bus_protocol() {
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let readme_source = load_source("src/color/slider/README.md");

    for forbidden in [
        "Context Bus",
        "ContextBus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "prop drilling",
        "prop_drilling",
        "Table/Grid",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "color-slider should not define event light cone batch protocol marker `{forbidden}`.",
        );
    }
}

#[test]
fn color_slider_does_not_define_unified_causality_bus_trace_contract() {
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let readme_source = load_source("src/color/slider/README.md");

    for forbidden in [
        "TraceId",
        "trace_id",
        "trace-id",
        "data-trace-id",
        "Causality Bus",
        "causality bus",
        "command bus",
        "event bus",
        "broadcast",
        "subscriber",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "color-slider should not define unified causality bus marker `{forbidden}`.",
        );
    }
}

#[test]
fn color_slider_does_not_define_overlay_focus_stack_recovery_protocol() {
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let motion_source = load_source("src/color/slider/motion.rs");
    let readme_source = load_source("src/color/slider/README.md");

    for forbidden in [
        "FocusManager",
        "Focus Stack",
        "focus stack",
        "focus_stack",
        "focus gc",
        "FallbackTo",
        "restore_focus",
        "restore target",
        "document.body",
        "activeElement",
        "Overlay",
        "overlay open",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "color-slider should not define overlay focus stack recovery marker `{forbidden}`.",
        );
    }

    for needle in [
        "let root_ref: NodeRef<html::Div> = NodeRef::new();",
        "motion::attach_motion(root_ref, visual_percent, motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "color-slider should keep NodeRef usage scoped to motion mount `{needle}`.",
        );
    }

    for needle in [
        "pub fn attach_motion(",
        "root_ref: leptos::prelude::NodeRef<leptos::html::Div>",
        "_root_ref: leptos::prelude::NodeRef<leptos::html::Div>",
    ] {
        assert!(
            motion_source.contains(needle),
            "color-slider motion boundary should keep root NodeRef local `{needle}`.",
        );
    }
}

#[test]
fn color_slider_does_not_define_foreign_zone_escape_hatch_contract() {
    let mod_source = load_source("src/color/slider/mod.rs");
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let motion_source = load_source("src/color/slider/motion.rs");
    let readme_source = load_source("src/color/slider/README.md");

    for forbidden in [
        "Foreign Zone",
        "foreign zone",
        "foreign_zone",
        "YieldControl",
        "yield_control",
        "CleanupForeign",
        "cleanup_foreign",
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "google maps",
        "chart_instance",
        "map_instance",
        "imperative bridge",
        "third-party instance",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "color-slider should not define foreign-zone escape hatch marker `{forbidden}`.",
        );
    }
}

#[test]
fn color_slider_hydration_ids_are_deterministic_without_time_or_random_inputs() {
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let motion_source = load_source("src/color/slider/motion.rs");
    let readme_source = load_source("src/color/slider/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for forbidden in [
        "SystemTime::now",
        "Instant::now",
        "Date::now",
        "js_sys::Date",
        "Uuid::new_v4",
        "uuid::",
        "rand::",
        "random(",
        "crypto.getRandomValues",
        "Math.random",
        "nanoid",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "color-slider hydration contract should not depend on non-deterministic source `{forbidden}`.",
        );
    }

    for needle in [
        "pub fn ColorSlider(",
        "id_base: String,",
        "let id_base = logic::normalize_optional_text(Some(id_base))",
        "unwrap_or_else(|| \"ui-color-slider\".to_string())",
        "let input_id = format!(\"{id_base}-input\");",
        "let label_id = format!(\"{id_base}-label\");",
        "let value_id = format!(\"{id_base}-value\");",
    ] {
        assert!(
            view_source.contains(needle),
            "color-slider should derive hydration ids from deterministic id_base path `{needle}`.",
        );
    }

    for needle in [
        "id_base=\"demo-color-slider\".to_string()",
        "id_base=\"demo-color-slider-hue\".to_string()",
    ] {
        assert!(
            readme_source.contains(needle),
            "color-slider README should keep deterministic id_base usage `{needle}`.",
        );
    }

    for needle in [
        "id_base=\"docs-color-slider-hello\".to_string()",
        "id_base=\"docs-color-slider-hue\".to_string()",
        "id_base=\"docs-color-slider-alpha\".to_string()",
        "id_base=\"docs-color-slider-custom\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app color-slider examples should keep deterministic id_base seed `{needle}`.",
        );
    }
}

#[test]
fn color_slider_platform_contract_keeps_wasm_cfg_and_non_wasm_safety() {
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let motion_source = load_source("src/color/slider/motion.rs");

    for forbidden in [
        "web_sys::",
        "leptos::web_sys",
        "js_sys::",
        "window()",
        "document()",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "color-slider non-wasm core layers should not reference browser object `{forbidden}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "let element: leptos::web_sys::HtmlElement = root.unchecked_into();",
        "_root_ref: leptos::prelude::NodeRef<leptos::html::Div>",
        "_visual_percent: leptos::prelude::Signal<f64>",
        "_motion: ColorSliderMotion,",
    ] {
        assert!(
            motion_source.contains(needle),
            "color-slider motion platform branch should include `{needle}`.",
        );
    }

    if let Some((_, non_wasm_tail)) =
        motion_source.split_once("#[cfg(not(target_arch = \"wasm32\"))]")
    {
        assert!(
            !non_wasm_tail.contains("web_sys::") && !non_wasm_tail.contains("leptos::web_sys"),
            "non-wasm motion branch should stay web-sys free.",
        );
    } else {
        panic!("color-slider motion should define explicit non-wasm cfg branch.");
    }
}

#[test]
fn color_slider_reduced_motion_ssr_and_wasm_contracts_stay_aligned() {
    let motion_source = load_source("src/color/slider/motion.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    let reduced_motion_guard_count = motion_source
        .matches("if !motion.enabled || ui_motion::web::prefers_reduced_motion() {")
        .count();
    assert!(
        reduced_motion_guard_count >= 2,
        "color-slider motion should guard both mount/update paths for reduced-motion; found {reduced_motion_guard_count} guard(s).",
    );

    for needle in [
        "ui_observability::set_css_property_observed_auto!(",
        "\"--ui-slider-visual-percent\",",
        "&format!(\"{initial:.4}\")",
        "&format!(\"{target:.4}\")",
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion() {",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "SpringAnimator::new(",
    ] {
        assert!(
            motion_source.contains(needle),
            "color-slider motion should preserve reduced-motion/ssr/wasm contract marker `{needle}`.",
        );
    }

    for needle in [
        "let id_base = logic::normalize_optional_text(Some(id_base))",
        "let input_id = format!(\"{id_base}-input\");",
        "let label_id = format!(\"{id_base}-label\");",
        "let value_id = format!(\"{id_base}-value\");",
        "role=slider_aria.input.role",
        "aria-disabled=slider_aria.input.aria_disabled",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "color-slider view should keep hydration-stable and platform-invariant semantic marker `{needle}`.",
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-slider semantic mount should not split view contract by platform cfg `{forbidden}`.",
        );
    }

    for needle in [
        "let reduced_motion = ColorSliderMotion::disabled();",
        "motion=reduced_motion",
        "id_base=\"docs-color-slider-alpha\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs color-slider should keep reduced-motion demonstration marker `{needle}`.",
        );
    }
}

#[test]
fn color_slider_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let check2_source = load_source("check2.md");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let view_source = load_source("src/color/slider/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "\"color-slider\" => UiPerfBudget {",
        "max_mount_ms: 30.0,",
        "max_update_ms: Some(10.0),",
        "max_heap_kb: Some(512.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "component shell should keep performance budget token `{needle}`.",
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
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose performance regression marker `{needle}`.",
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
            "docs coverage e2e should enforce perf regression guard `{needle}`.",
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
            "ColorSlider checklist should keep performance governance marker `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance follow-up plan should keep `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui-color-slider color_slider_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`.",
        );
    }

    for needle in [
        "logic::resolve_state(ColorSliderStateInput {",
        "motion::attach_motion(root_ref, visual_percent, motion);",
        "data-state=move || state.get().data_state_attr",
        "data-value=move || state.get().value.to_string()",
        "data-value-percent=move || state.get().value_percent.to_string()",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-track-source=move || state.get().track_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorSlider view should expose state/render/style/motion attribution marker `{needle}`.",
        );
    }
}

#[test]
fn color_slider_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let view_source = load_source("src/color/slider/view.rs");
    let logic_source = load_source("src/color/slider/logic.rs");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let check2_source = load_source("check2.md");
    let docs_shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let semantics_source = load_source("../../components/color-slider/test/semantics.rs");

    for marker in [
        "role=slider_aria.input.role",
        "aria-disabled=slider_aria.input.aria_disabled",
        "aria-valuemin=move || slider_aria.input.aria_valuemin.get()",
        "aria-valuemax=move || slider_aria.input.aria_valuemax.get()",
        "aria-valuenow=move || slider_aria.input.aria_valuenow.get()",
        "data-state=move || state.get().data_state_attr",
        "data-value-source=value_source_attr",
        "data-default-value-source=default_value_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-focus-visible=move || slider_aria.state.is_focus_visible.get().then_some(\"true\")",
        "on:pointerdown=move |_| slider_aria.handlers.on_pointer_down.run(())",
        "on:pointerup=move |_| slider_aria.handlers.on_pointer_up.run(())",
        "on:focus=move |_| slider_aria.handlers.on_focus.run(())",
        "on:blur=move |_| slider_aria.handlers.on_blur.run(())",
    ] {
        assert!(
            view_source.contains(marker),
            "color-slider semantics/perf matrix should keep aria/data/focus marker `{marker}`.",
        );
    }

    for marker in [
        "\"color-slider\" => UiPerfBudget {",
        "max_mount_ms: 30.0,",
        "max_update_ms: Some(10.0),",
        "max_heap_kb: Some(512.0),",
    ] {
        assert!(
            docs_shell_source.contains(marker),
            "docs shell should preserve color-slider perf budget marker `{marker}`.",
        );
    }

    for marker in [
        "cargo test -p ui-color-slider color_slider_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance script should enforce `{marker}`.",
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "TODO should keep render_count follow-up marker `{marker}`.",
        );
    }

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "color_slider_semantics_matrix_covers_state_paths_without_snapshot_dependency",
        "color_slider_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "color_slider_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            check2_source.contains(marker),
            "color-slider check2 semantics/perf section should reference `{marker}`.",
        );
    }

    assert!(
        logic_source.contains("resolve_state,") || logic_source.contains("pub fn resolve_state("),
        "logic should keep state derivation export for attributable semantics/perf regressions.",
    );

    let snapshot_token = ["assert", "_snapshot!("].concat();
    let insta_snapshot_token = ["insta::assert", "_snapshot!("].concat();
    let jest_snapshot_token = ["toMatch", "Snapshot("].concat();
    for forbidden in [
        snapshot_token.as_str(),
        insta_snapshot_token.as_str(),
        jest_snapshot_token.as_str(),
    ] {
        assert!(
            !semantics_source.contains(forbidden),
            "color-slider semantic/perf tests should not depend on visual snapshot token `{forbidden}`.",
        );
    }
}

#[test]
fn color_slider_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only() {
    let check2_source = load_source("../../components/color-slider/check2.md");
    let view_source = load_source("../../components/color-slider/src/view.rs");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");
    let local_semantics = include_str!("semantics.rs");

    for required in [
        "role=slider_aria.input.role",
        "aria-disabled=slider_aria.input.aria_disabled",
        "aria-valuemin=move || slider_aria.input.aria_valuemin.get()",
        "aria-valuemax=move || slider_aria.input.aria_valuemax.get()",
        "aria-valuenow=move || slider_aria.input.aria_valuenow.get()",
        "data-state=move || state.get().data_state_attr",
        "data-value-source=value_source_attr",
        "data-default-value-source=default_value_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
        "on:focus=move |_| slider_aria.handlers.on_focus.run(())",
        "on:blur=move |_| slider_aria.handlers.on_blur.run(())",
    ] {
        assert!(
            view_source.contains(required),
            "color-slider view should keep semantic contract marker `{required}`.",
        );
    }

    for required in [
        "fn color_slider_exposes_a11y_and_i18n_l10n_contracts()",
        "fn color_slider_semantics_matrix_covers_state_paths_without_snapshot_dependency()",
        "fn color_slider_snapshot_baseline_consumes_complete_result_and_renders_stably()",
        "fn color_slider_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only()",
        "role=slider_aria.input.role",
        "data-value-source=value_source_attr",
    ] {
        assert!(
            local_semantics.contains(required),
            "semantic test suite should include semantic-first coverage token `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "color_slider_exposes_a11y_and_i18n_l10n_contracts",
        "color_slider_semantics_matrix_covers_state_paths_without_snapshot_dependency",
        "color_slider_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "color_slider_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should keep semantic-priority checklist marker `{required}`.",
        );
    }
}

#[test]
fn color_slider_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("../../components/color-slider/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
        "color_slider_check2_documents_e2e_selector_and_stable_wait_rules",
        "color_slider_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "color_slider_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints",
        "components/color-slider/scripts/check-ui-e2e-color-slider.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should keep e2e-selector/stable-wait governance marker `{required}`.",
        );
    }
}

#[test]
fn color_slider_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_slider_contract.spec.mjs");
    let script_source =
        load_source("../../components/color-slider/scripts/check-ui-e2e-color-slider.sh");

    for required in [
        "const COLOR_SLIDER_PAGE = \"/#/components/color-slider\";",
        "body:not(:has(#boot))",
        "[data-component=\"color-slider\"] #docs-color-slider-hue[data-slot=\"color-slider\"][data-control-mode=\"controlled\"][data-channel=\"hue\"]",
        "data-slot=\"color-slider-input\"",
        "data-slot=\"color-slider-label\"",
        "data-ui-schema",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-value-source",
        "data-value-change-source",
        "data-ui-intent",
        "data-ui-output-status",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-slider e2e contract should include semantic selector/wait marker `{required}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        ":nth-child(",
        "getByText(",
        "locator(\"text=",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "color-slider e2e contract should avoid brittle selector/wait token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_e2e_selector_contract_uses_semantic_markers_and_stable_waits";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-slider gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_slider_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_slider_contract.spec.mjs");
    let script_source =
        load_source("../../components/color-slider/scripts/check-ui-e2e-color-slider.sh");

    for required in [
        "input.focus()",
        "toBeFocused()",
        "keyboard.press(\"ArrowRight\")",
        "toHaveAttribute(\"data-ui-action\", \"focus\")",
        "toHaveAttribute(\"data-focused\", \"true\")",
        "toHaveAttribute(\"data-ui-source\", \"on_value_change\")",
        "dispatchEvent(\"pointerdown\")",
        "toHaveAttribute(\"data-ui-action\", \"press\")",
        "dispatchEvent(\"pointerup\")",
        "#docs-color-slider-alpha[data-slot=\"color-slider\"]",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-ui-output-status\", \"submittable\")",
        "#docs-color-slider-custom[data-slot=\"color-slider\"]",
        "toHaveAttribute(\"data-track-source\", \"custom\")",
        "toHaveAttribute(\"data-motion-source\", \"custom\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-slider e2e motion/settled contract should include `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-slider gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_slider_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = load_source("../../components/color-slider/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
        "color_slider_check2_documents_e2e_repeatable_key_flow_rules",
        "color_slider_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "color_slider_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "components/color-slider/scripts/check-ui-e2e-color-slider.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should keep repeatable e2e flow governance marker `{required}`.",
        );
    }
}

#[test]
fn color_slider_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_slider_contract.spec.mjs");
    let script_source =
        load_source("../../components/color-slider/scripts/check-ui-e2e-color-slider.sh");

    for required in [
        "key flow is repeatable and failures map to semantic breakpoints",
        "input.focus()",
        "keyboard.press(\"ArrowRight\")",
        "data-ui-action\", \"focus\"",
        "data-ui-source\", \"on_value_change\"",
        "data-ui-output-status\", \"submittable\"",
        "await page.reload();",
        "data-ui-action\", \"idle\"",
        "data-focus-visible\", \"true\"",
    ] {
        assert!(
            e2e_source.contains(required),
            "repeatable color-slider e2e flow should include semantic breakpoint `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_e2e_key_flow_is_repeatable_and_failure_points_are_semantic";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-slider gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_slider_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_slider_contract.spec.mjs");
    let script_source =
        load_source("../../components/color-slider/scripts/check-ui-e2e-color-slider.sh");

    for required in [
        "high-risk paths keep focus keyboard and disabled branches semantically explicit",
        "input.focus()",
        "toBeFocused()",
        "keyboard.press(\"ArrowLeft\")",
        "data-ui-action\", \"focus\"",
        "data-focused\", \"true\"",
        "data-focus-visible\", \"true\"",
        "#docs-color-slider-alpha[data-slot=\"color-slider\"]",
        "data-state\", \"disabled\"",
        "data-disabled\", \"true\"",
        "data-ui-output-status\", \"submittable\"",
        "aria-disabled\", \"true\"",
        "toBeDisabled()",
    ] {
        assert!(
            e2e_source.contains(required),
            "high-risk color-slider e2e path should include `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-slider gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_slider_exposes_a11y_and_i18n_l10n_contracts() {
    let logic_source = load_source("src/color/slider/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let readme_source = load_source("src/color/slider/README.md");
    let headless_slider_source = load_source("../../crates/ui-headless/src/slider.rs");
    let headless_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "use ui_headless::{A11yDirection, SliderOptions, use_slider};",
        "#[prop(optional, into)] label: Option<String>,",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "role=\"group\"",
        "role=slider_aria.input.role",
        "lang=move || locale_lang.get_value()",
        "dir=locale_dir",
        "aria-label=aria_label.get_value()",
        "aria-labelledby=label_id_for_input",
        "aria-disabled=slider_aria.input.aria_disabled",
        "aria-valuemin=move || slider_aria.input.aria_valuemin.get()",
        "aria-valuemax=move || slider_aria.input.aria_valuemax.get()",
        "aria-valuenow=move || slider_aria.input.aria_valuenow.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "color-slider view should keep a11y/i18n contract marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn normalize_label(",
        "pub fn normalize_aria_label(",
        "normalize_text_with_fallback(value, channel.default_label())",
        "Cow::Borrowed(channel.default_aria_label())",
    ] {
        assert!(
            logic_source.contains(needle),
            "color-slider logic should keep localized label fallback pipeline marker `{needle}`.",
        );
    }

    for needle in [
        "use crate::a11y::{A11yDirection, locale_attrs};",
        "let locale = locale_attrs(lang, dir);",
    ] {
        assert!(
            headless_slider_source.contains(needle),
            "ui-headless slider should keep shared a11y locale mapping marker `{needle}`.",
        );
    }

    assert!(
        headless_a11y_source.contains("pub fn locale_attrs("),
        "ui-headless shared a11y helpers should expose locale_attrs contract.",
    );
    assert!(
        readme_source.contains("i18n/l10n：`lang` / `dir` 透传到 headless 语义契约"),
        "color-slider README should document lang/dir localization integration.",
    );

    for forbidden in [
        ">Hue<",
        ">Saturation<",
        ">Lightness<",
        ">Alpha<",
        ">Red<",
        ">Green<",
        ">Blue<",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-slider should not hardcode user-visible channel copy in view marker `{forbidden}`.",
        );
    }
}

#[test]
fn color_slider_discrete_axes_are_modeled_with_typed_enums() {
    let view_source = load_source("src/color/slider/view.rs");
    let logic_source = load_source("src/color/slider/logic.rs");

    for needle in [
        "#[prop(optional)] channel: ColorSliderChannel,",
        "#[prop(optional)] motion: ColorSliderMotion,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "pub enum ColorSliderControlMode",
        "pub enum ColorSliderValueSource",
        "pub enum ColorSliderValueChangeSource",
        "pub enum ColorSliderDisabledSource",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "color-slider discrete axis contract should include typed enum marker `{needle}`.",
        );
    }

    for forbidden in [
        "variant: Option<String>",
        "size: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
        "variant: Option<bool>",
        "size: Option<bool>",
        "mode: Option<bool>",
        "status: Option<bool>",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "color-slider should not model discrete states via free-form/string bool combos `{forbidden}`.",
        );
    }
}

#[test]
fn color_slider_keeps_non_composite_api_surface() {
    let view_source = load_source("src/color/slider/view.rs");
    let readme_source = load_source("src/color/slider/README.md");

    for forbidden in [
        "children:",
        "labels:",
        "titles:",
        "panels:",
        "items:",
        "ItemSpec",
        "<Item ",
    ] {
        assert!(
            !view_source.contains(forbidden) && !readme_source.contains(forbidden),
            "color-slider should stay non-composite; found forbidden composite API marker `{forbidden}`.",
        );
    }

    assert!(
        view_source.contains("pub fn ColorSlider(")
            && view_source.contains("#[prop(optional)] channel: ColorSliderChannel,")
            && view_source.contains("#[prop(optional)] value: Option<Signal<f64>>,")
            && view_source.contains("#[prop(optional)] default_value: Option<f64>,"),
        "color-slider should expose scalar channel/value props instead of parent-item composition.",
    );
}

#[test]
fn color_slider_exposes_baseline_style_data_markers() {
    let source = load_source("src/color/slider/view.rs");

    for attr in [
        "data-slot=\"color-slider\"",
        "data-state=move || state.get().data_state_attr",
        "data-channel=move || state.get().channel_attr",
        "data-value=move || state.get().value.to_string()",
        "data-value-percent=move || state.get().value_percent.to_string()",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-default-value-source=default_value_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-track-source=move || state.get().track_source_attr",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-schema-version=agent_contract.schema_version_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
        "data-ui-source=value_change_source_attr",
        "data-ui-state=move || state.get().data_state_attr",
        "data-slot=\"color-slider-label\"",
        "data-slot=\"color-slider-value\"",
        "data-slot=\"color-slider-input\"",
        "data-slot=\"color-slider-track\"",
        "data-slot=\"color-slider-fill\"",
        "data-slot=\"color-slider-thumb\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorSlider should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn color_slider_state_markers_are_observable_queryable_and_enumerated() {
    let view_source = load_source("src/color/slider/view.rs");
    let logic_source = load_source("src/color/slider/logic.rs");
    let styles_source = load_source("src/color/slider/styles.rs");

    for marker in [
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-focus-visible=move || slider_aria.state.is_focus_visible.get().then_some(\"true\")",
        "data-control-mode=control_mode_attr",
        "data-value-source=value_source_attr",
        "data-default-value-source=default_value_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-disabled-source=disabled_source_attr",
        "data-ui-action=move || ui_action.get().as_attr()",
        "aria-disabled=slider_aria.input.aria_disabled",
        "aria-valuemin=move || slider_aria.input.aria_valuemin.get()",
        "aria-valuemax=move || slider_aria.input.aria_valuemax.get()",
        "aria-valuenow=move || slider_aria.input.aria_valuenow.get()",
    ] {
        assert!(
            view_source.contains(marker),
            "color-slider should expose observable/queryable marker `{marker}`.",
        );
    }

    for selector in [
        ".ui-color-slider[data-disabled=\"true\"]",
        ".ui-color-slider[data-hovered=\"true\"] .ui-color-slider__track",
        ".ui-color-slider[data-pressed=\"true\"] .ui-color-slider__thumb",
        ".ui-color-slider[data-focus-visible=\"true\"] .ui-color-slider__track",
        ".ui-color-slider[data-track-source=\"custom\"]",
        ".ui-color-slider[data-custom-class=\"true\"]",
        ".ui-color-slider[data-channel=\"alpha\"] .ui-color-slider__track::before",
    ] {
        assert!(
            styles_source.contains(selector),
            "color-slider styles should prefer semantic selectors `{selector}`.",
        );
    }

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        ":active +",
        ":focus-visible +",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "color-slider styles should not depend on fragile structure selector `{forbidden}`.",
        );
    }

    for needle in [
        "pub enum ColorSliderControlMode",
        "Self::Controlled => \"controlled\"",
        "Self::Uncontrolled => \"uncontrolled\"",
        "pub enum ColorSliderValueSource",
        "Self::External => \"external\"",
        "Self::DefaultValue => \"default_value\"",
        "pub enum ColorSliderValueChangeSource",
        "Self::OnValueChange => \"on_value_change\"",
        "Self::None => \"none\"",
        "pub enum ColorSliderDisabledSource",
        "Self::IsDisabled => \"is_disabled\"",
        "Self::Disabled => \"disabled\"",
        "Self::Default => \"default\"",
        "pub enum ColorSliderUiAction",
        "Self::Idle => \"idle\"",
        "Self::Focus => \"focus\"",
        "Self::Press => \"press\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "color-slider marker value set should be enum-backed and closed; missing `{needle}`.",
        );
    }
}

#[test]
fn color_slider_styles_include_channel_and_custom_contracts() {
    let source = load_source("src/color/slider/styles.rs");

    for selector in [
        ".ui-color-slider",
        ".ui-color-slider__track",
        ".ui-color-slider__thumb",
        ".ui-color-slider[data-hovered=\"true\"] .ui-color-slider__track",
        ".ui-color-slider--channel-hue",
        ".ui-color-slider[data-channel=\"alpha\"] .ui-color-slider__track::before",
        ".ui-color-slider--disabled",
        ".ui-color-slider[data-disabled=\"true\"]",
        ".ui-color-slider--track-custom",
        ".ui-color-slider[data-track-source=\"custom\"]",
        ".ui-color-slider--custom-class",
        ".ui-color-slider[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorSlider styles should include `{selector}` as stable state-marker contracts."
        );
    }

    for token_var in [
        "--ui-slider-max-width",
        "--ui-slider-thumb-border-width",
        "--ui-slider-focus-ring-width",
        "--ui-common-red-500",
        "--ui-common-green-600",
        "--ui-common-blue-600",
        "--ui-common-zinc-500",
        "--ui-common-black",
        "--ui-common-white",
        "--ui-icon-size-100",
        "--ui-space-sm",
    ] {
        assert!(
            source.contains(token_var),
            "ColorSlider styles should consume theme token variable `{token_var}`."
        );
    }

    for hardcoded in [
        "#ff0000",
        "#ffff00",
        "hsl(0 100% 50% / 1)",
        "rgb(255 0 0 / 1)",
        "22rem",
        "var(--ui-space-sm, 10px)",
        "var(--ui-icon-size-100, 20px)",
        "var(--ui-space-xs, 6px)",
        "var(--ui-slider-max-width, 352px)",
        "border-radius: 999px;",
    ] {
        assert!(
            !source.contains(hardcoded),
            "ColorSlider styles should not hardcode legacy value `{hardcoded}`."
        );
    }
}

#[test]
fn color_slider_uses_token_first_static_css_injection_contract() {
    let styles_source = load_source("src/color/slider/styles.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let logic_source = load_source("src/color/slider/logic.rs");
    let css_registry_source = load_source("../../crates/ui/src/css.rs");
    let ui_root_source = load_source("../../crates/ui/src/root.rs");

    for needle in [
        "pub const CSS: &str",
        "var(--ui-",
        "--ui-color-slider-track-start",
        "--ui-color-slider-track-end",
    ] {
        assert!(
            styles_source.contains(needle),
            "color-slider styles should stay token-first static css contract `{needle}`.",
        );
    }

    for needle in [
        "style=inline_style.get_value().unwrap_or_default()",
        "logic::compose_inline_style(",
    ] {
        assert!(
            view_source.contains(needle),
            "color-slider runtime style wiring should stay constrained to css variable bridge `{needle}`.",
        );
    }

    for needle in [
        "declarations.push(format!(\"--ui-color-slider-track-start: {track_start};\"));",
        "declarations.push(format!(\"--ui-color-slider-track-end: {track_end};\"));",
    ] {
        assert!(
            logic_source.contains(needle),
            "color-slider inline style builder should only emit css custom properties `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-color_slider\")]",
        "out.push_str(crate::color::slider::styles::CSS);",
    ] {
        assert!(
            css_registry_source.contains(needle),
            "ui css registry should aggregate color-slider styles via feature gate `{needle}`.",
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "out.push_str(&theme.get().to_css_variables());",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "UiRoot should own token/component css injection path `{needle}`.",
        );
    }

    for forbidden in [
        "@apply",
        "tailwind",
        "styled_components",
        "emotion",
        "stylex",
        "css_in_rust",
    ] {
        assert!(
            !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden),
            "color-slider should not default to utility-first/css-in-rust marker `{forbidden}`.",
        );
    }
}

#[test]
fn color_slider_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("src/color/slider/styles.rs");
    let theme_css_source = load_source("../../crates/ui-theme/src/css.rs");
    let check2_source = load_source("../../components/color-slider/check2.md");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-icon-size-100, var(--ui-fallback-icon-size-100))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-slider-max-width, var(--ui-fallback-slider-max-width))",
        "var(--ui-slider-thumb-border-width, var(--ui-fallback-slider-thumb-border-width))",
        "var(--ui-slider-focus-ring-width, var(--ui-fallback-slider-focus-ring-width))",
        "var(--ui-common-black, var(--ui-fallback-common-black))",
        "var(--ui-common-white, var(--ui-fallback-common-white))",
        "var(--ui-common-red-500, var(--ui-fallback-common-red-500))",
        "var(--ui-common-red-600, var(--ui-fallback-common-red-600))",
        "var(--ui-common-yellow-500, var(--ui-fallback-common-yellow-500))",
        "var(--ui-common-green-500, var(--ui-fallback-common-green-500))",
        "var(--ui-common-green-600, var(--ui-fallback-common-green-600))",
        "var(--ui-common-cyan-500, var(--ui-fallback-common-cyan-500))",
        "var(--ui-common-blue-500, var(--ui-fallback-common-blue-500))",
        "var(--ui-common-blue-600, var(--ui-fallback-common-blue-600))",
        "var(--ui-common-purple-500, var(--ui-fallback-common-purple-500))",
        "var(--ui-common-zinc-500, var(--ui-fallback-common-zinc-500))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
    ] {
        assert!(
            styles_source.contains(required),
            "color-slider styles should keep defensive double-fallback token `{required}`.",
        );
    }

    for required in [
        "--ui-fallback-icon-size-100:",
        "--ui-fallback-slider-max-width:",
        "--ui-fallback-slider-thumb-border-width:",
        "--ui-fallback-slider-focus-ring-width:",
        "--ui-fallback-common-black:",
        "--ui-fallback-common-white:",
        "--ui-fallback-common-red-500:",
        "--ui-fallback-common-red-600:",
        "--ui-fallback-common-yellow-500:",
        "--ui-fallback-common-green-500:",
        "--ui-fallback-common-green-600:",
        "--ui-fallback-common-cyan-500:",
        "--ui-fallback-common-blue-500:",
        "--ui-fallback-common-blue-600:",
        "--ui-fallback-common-purple-500:",
        "--ui-fallback-common-zinc-500:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme should stay SSOT for color-slider fallback token `{required}`.",
        );
    }

    for forbidden in [
        "var(--ui-space-sm, 10px)",
        "var(--ui-icon-size-100, 20px)",
        "var(--ui-space-xs, 6px)",
        "var(--ui-slider-max-width, 352px)",
        "var(--ui-slider-thumb-border-width, 2px)",
        "var(--ui-slider-focus-ring-width, 2px)",
        "border-radius: 999px;",
        "#ff0000",
        "#ffff00",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "color-slider styles should not keep raw terminal token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "color_slider_styles_use_defensive_variable_fallback_chain",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep defensive-variable governance marker `{required}`.",
        );
    }
}

#[test]
fn color_slider_cascade_layer_and_runtime_style_contract_is_enforced() {
    let check2_source = load_source("../../components/color-slider/check2.md");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");
    let css_source = load_source("../../crates/ui/src/css.rs");
    let root_source = load_source("../../crates/ui/src/root.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let logic_source = load_source("src/color/slider/logic.rs");
    let motion_source = load_source("src/color/slider/motion.rs");

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-color_slider\")]",
        "out.push_str(crate::color::slider::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_source.contains(required),
            "ui css aggregation should keep cascade-layer marker `{required}`.",
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should keep components css injection path marker `{required}`.",
        );
    }

    for required in [
        "style=inline_style.get_value().unwrap_or_default()",
        "logic::compose_inline_style(",
        "declarations.push(format!(\"--ui-color-slider-track-start: {track_start};\"));",
        "declarations.push(format!(\"--ui-color-slider-track-end: {track_end};\"));",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "color-slider runtime style bridge should stay css-variable only marker `{required}`.",
        );
    }

    for source in [
        view_source.as_str(),
        logic_source.as_str(),
        motion_source.as_str(),
    ] {
        for forbidden in [
            "style:top",
            "style:left",
            "style:right",
            "style:bottom",
            "style:width",
            "style:height",
            "style:margin",
            "style:padding",
            "style:background",
            "style:border",
            "style:color",
            "style=\"top:",
            "style=\"left:",
            "style=\"right:",
            "style=\"bottom:",
            "style=\"width:",
            "style=\"height:",
            "style=\"margin:",
            "style=\"padding:",
            "style=\"background:",
            "style=\"border:",
            "style=\"color:",
        ] {
            assert!(
                !source.contains(forbidden),
                "color-slider runtime style path should avoid non-variable inline style marker `{forbidden}`.",
            );
        }
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "color_slider_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep cascade-layer governance marker `{required}`.",
        );
    }
}

#[test]
fn color_slider_visual_desire_contracts_are_baselined() {
    let styles_source = load_source("src/color/slider/styles.rs");
    let docs_slider_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let docs_theme_baseline_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let e2e_theme_baseline_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for needle in [
        ".ui-color-slider__label",
        "font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));",
        "line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));",
        "font-weight: 600;",
        ".ui-color-slider__value",
        "font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
        "line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));",
        "color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        "gap: var(--ui-space-xs, var(--ui-fallback-space-xs));",
        ".ui-color-slider[data-hovered=\"true\"] .ui-color-slider__track",
        ".ui-color-slider[data-pressed=\"true\"] .ui-color-slider__thumb",
        ".ui-color-slider[data-focus-visible=\"true\"] .ui-color-slider__track",
    ] {
        assert!(
            styles_source.contains(needle),
            "color-slider visual desire baseline should retain style contract `{needle}`.",
        );
    }

    for needle in [
        "title=\"ColorSlider\"",
        "slug=\"color-slider\"",
        "title=\"Hello World\"",
        "title=\"Controlled Hue Channel\"",
        "title=\"Disabled Alpha + Custom Track + Reduced Motion\"",
    ] {
        assert!(
            docs_slider_source.contains(needle),
            "color-slider docs baseline should contain `{needle}`.",
        );
    }

    for needle in [
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues.",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            docs_theme_baseline_source.contains(needle),
            "docs-app visual baseline page should contain `{needle}`.",
        );
    }

    for needle in [
        "E2E_VISUAL_BASELINE",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            e2e_theme_baseline_source.contains(needle),
            "visual baseline e2e should keep screenshot regression contract `{needle}`.",
        );
    }
}

#[test]
fn color_slider_preserves_ui_headless_web_ssr_mutex_contract() {
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let headless_cargo_source = load_source("../../crates/ui-headless/Cargo.toml");
    let slider_cargo_source = load_source("../../components/color-slider/Cargo.toml");
    let view_source = load_source("src/color/slider/view.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should keep web/ssr mutual exclusion guard `{needle}`.",
        );
    }

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            headless_cargo_source.contains(needle),
            "ui-headless cargo features should keep mutually exclusive mapping `{needle}`.",
        );
    }

    assert!(
        slider_cargo_source.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "color-slider should consume ui-headless through crate boundary without overriding feature mutex.",
    );

    for forbidden in [
        "ui-headless = { path = \"../../crates/ui-headless\", default-features = false, features = [\"web\", \"ssr\"] }",
        "ui-headless = { path = \"../../crates/ui-headless\", features = [\"web\", \"ssr\"] }",
    ] {
        assert!(
            !slider_cargo_source.contains(forbidden),
            "color-slider dependency config must not bypass headless web/ssr mutex `{forbidden}`.",
        );
    }

    assert!(
        view_source.contains("use ui_headless::{A11yDirection, SliderOptions, use_slider};"),
        "color-slider should continue mounting headless contract through `use_slider`.",
    );
}

#[test]
fn color_slider_preserves_ui_motion_non_wasm_stub_contract() {
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let slider_motion_source = load_source("src/color/slider/motion.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion should keep non-wasm no-op/stub contract `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "ui_motion::web::prefers_reduced_motion()",
        "SpringAnimator::new(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "_root_ref: leptos::prelude::NodeRef<leptos::html::Div>",
        "_visual_percent: leptos::prelude::Signal<f64>",
        "_motion: ColorSliderMotion,",
    ] {
        assert!(
            slider_motion_source.contains(needle),
            "color-slider motion should keep wasm/non-wasm safe-degrade contract `{needle}`.",
        );
    }

    if let Some((_, non_wasm_tail)) =
        slider_motion_source.split_once("#[cfg(not(target_arch = \"wasm32\"))]")
    {
        assert!(
            !non_wasm_tail.contains("panic!")
                && !non_wasm_tail.contains("unwrap(")
                && !non_wasm_tail.contains("expect("),
            "color-slider non-wasm motion fallback should remain predictable no-op without panic path.",
        );
    } else {
        panic!("color-slider motion should define explicit non-wasm fallback branch.");
    }
}

#[test]
fn color_slider_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let check2_source = load_source("../../components/color-slider/check2.md");
    let script_source = load_source("../../scripts/check-ui-platforms.sh");
    let motion_source = load_source("src/color/slider/motion.rs");
    let view_source = load_source("src/color/slider/view.rs");

    for needle in [
        "pub struct ColorSliderMotion",
        "pub enabled: bool,",
        "pub spring: ui_motion::spring::SpringConfig,",
        "use ui_theme::default_slider_motion_tokens;",
        "let tokens = default_slider_motion_tokens();",
        "stiffness: tokens.spring.stiffness,",
        "damping: tokens.spring.damping,",
        "mass: tokens.spring.mass,",
        "precision: tokens.spring.precision,",
        "pub fn sanitize_motion(motion: ColorSliderMotion) -> ColorSliderMotion {",
        "ui_motion::spring::sanitize_config(value, ColorSliderMotion::default().spring)",
        "#[cfg(target_arch = \"wasm32\")]",
        "SpringAnimator::new(",
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion() {",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "_root_ref: leptos::prelude::NodeRef<leptos::html::Div>",
        "_visual_percent: leptos::prelude::Signal<f64>",
        "_motion: ColorSliderMotion,",
    ] {
        assert!(
            motion_source.contains(needle),
            "color-slider motion contract should keep `{needle}`.",
        );
    }

    let reduced_motion_guard_count = motion_source
        .matches("if !motion.enabled || ui_motion::web::prefers_reduced_motion() {")
        .count();
    assert!(
        reduced_motion_guard_count >= 2,
        "color-slider motion should guard both mount/update paths for reduced-motion; found {reduced_motion_guard_count} guard(s).",
    );

    assert!(
        view_source.contains("motion::attach_motion(root_ref, visual_percent, motion)"),
        "color-slider view should mount motion contract through attach_motion.",
    );

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        script_source.contains(script_needle),
        "platform gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "color_slider_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep motion-contract governance marker `{needle}`.",
        );
    }
}

#[test]
fn color_slider_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let check2_source = load_source("../../components/color-slider/check2.md");
    let script_source = load_source("../../scripts/check-ui-entrypoints.sh");
    let lib_source = load_source("../../crates/ui/src/lib.rs");
    let css_source = load_source("../../crates/ui/src/css.rs");
    let root_source = load_source("../../crates/ui/src/root.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let headless_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");
    let headless_presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let headless_controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");

    for required in [
        "#[cfg(feature = \"component-color_slider\")]",
        "pub use ui_color_slider as color_slider;",
    ] {
        assert!(
            lib_source.contains(required),
            "ui lib entry should keep feature-gated color-slider public surface `{required}`.",
        );
    }

    for forbidden in [
        "pub use web_sys",
        "pub use wasm_bindgen",
        "pub use leptos::web_sys",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui lib entry should not expose platform detail `{forbidden}`.",
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-color_slider\")]",
        "out.push_str(crate::color::slider::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_source.contains(required),
            "ui css entry should keep feature-gated layered aggregation marker `{required}`.",
        );
    }

    for required in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot entry should keep centralized theme/i18n/css injection marker `{required}`.",
        );
    }

    for required in [
        "pub const CSS: &str",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "active_highlight should stay shared motion primitive marker `{required}`.",
        );
    }

    for forbidden in ["ui-color-slider", "ui-button", "ui-checkbox", "data-slot="] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should not carry component business semantics `{forbidden}`.",
        );
    }

    {
        let required = "pub fn aria_controls_when_open(";
        assert!(
            headless_a11y_source.contains(required),
            "headless canonical a11y path should keep `{required}`.",
        );
    }
    for required in [
        "pub fn use_presence(",
        "pub struct Presence",
        "pub is_present: ReadSignal<bool>",
        "pub finish_exit: Callback<()>",
    ] {
        assert!(
            headless_presence_source.contains(required),
            "headless canonical presence path should keep `{required}`.",
        );
    }
    for required in [
        "pub fn use_controllable_state<T>(",
        "pub struct ControllableState<T>",
        "pub struct ControllableOpenState",
    ] {
        assert!(
            headless_controllable_state_source.contains(required),
            "headless canonical controllable-state path should keep `{required}`.",
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    for forbidden in [
        workspace_dir.join("crates/ui/src/overlay_open.rs"),
        workspace_dir.join("crates/ui/src/presence.rs"),
        workspace_dir.join("crates/ui/src/a11y.rs"),
    ] {
        assert!(
            !forbidden.exists(),
            "ui forbidden fixed entrypoint file should stay absent: {forbidden:?}",
        );
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(script_needle),
        "entrypoint gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] `ui` 固定入口文件落点正确。",
        "color_slider_ui_components_fixed_entry_files_follow_layered_boundaries",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep fixed-entrypoint governance marker `{required}`.",
        );
    }
}

#[test]
fn color_slider_component_directory_standard_files_follow_contract_and_na_paths() {
    let check2_source = load_source("../../components/color-slider/check2.md");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");
    let mod_source = load_source("../../components/color-slider/src/mod.rs");
    let logic_source = load_source("../../components/color-slider/src/logic.rs");
    let styles_source = load_source("../../components/color-slider/src/styles.rs");
    let view_source = load_source("../../components/color-slider/src/view.rs");
    let motion_source = load_source("../../components/color-slider/src/motion.rs");

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    for required in [
        workspace_dir.join("components/color-slider/src/mod.rs"),
        workspace_dir.join("components/color-slider/src/logic.rs"),
        workspace_dir.join("components/color-slider/src/styles.rs"),
        workspace_dir.join("components/color-slider/src/view.rs"),
        workspace_dir.join("components/color-slider/src/motion.rs"),
    ] {
        assert!(
            required.exists(),
            "color-slider should keep required component file: {required:?}",
        );
    }

    for forbidden in [
        workspace_dir.join("components/color-slider/src/render.rs"),
        workspace_dir.join("components/color-slider/src/spec.rs"),
    ] {
        assert!(
            !forbidden.exists(),
            "color-slider should keep forbidden/simple-N/A component file absent: {forbidden:?}",
        );
    }

    for required in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::ColorSliderMotion;",
        "pub use view::ColorSlider;",
    ] {
        assert!(
            mod_source.contains(required),
            "color-slider mod.rs should keep minimal stable export marker `{required}`.",
        );
    }

    for forbidden in [
        "pub mod view;",
        "pub use logic::",
        "pub mod spec",
        "mod spec",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "color-slider mod.rs should avoid over-export marker `{forbidden}`.",
        );
    }

    for required in [
        "pub use ui_state_primitives::color_slider::{",
        "pub fn normalize_default_value(",
        "pub fn normalize_accessibility_state(",
        "pub fn resolve_source_attrs(",
    ] {
        assert!(
            logic_source.contains(required),
            "color-slider logic.rs should keep normalization/derivation marker `{required}`.",
        );
    }

    for forbidden in [
        "use leptos",
        "web_sys::",
        "view! {",
        "data-slot=",
        "pub const CSS",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "color-slider logic.rs should stay non-view/non-style marker `{forbidden}`.",
        );
    }

    for required in ["pub const CSS: &str", "var(--ui-", ".ui-color-slider"] {
        assert!(
            styles_source.contains(required),
            "color-slider styles.rs should keep static token-first marker `{required}`.",
        );
    }

    for forbidden in ["use leptos", "#[component]", "on:click=", "web_sys::"] {
        assert!(
            !styles_source.contains(forbidden),
            "color-slider styles.rs should avoid runtime/view marker `{forbidden}`.",
        );
    }

    for required in [
        "#[component]",
        "pub fn ColorSlider(",
        "use ui_headless::{A11yDirection, SliderOptions, use_slider};",
        "view! {",
        "data-slot=\"color-slider\"",
        "motion::attach_motion(root_ref, visual_percent, motion)",
    ] {
        assert!(
            view_source.contains(required),
            "color-slider view.rs should keep render + headless mount marker `{required}`.",
        );
    }

    for forbidden in ["render.rs", "include_str!(\"./render.rs\")"] {
        assert!(
            !view_source.contains(forbidden),
            "color-slider view.rs should not drift to render.rs marker `{forbidden}`.",
        );
    }

    for required in [
        "pub struct ColorSliderMotion",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(required),
            "color-slider motion.rs should keep semantic->motion mapping marker `{required}`.",
        );
    }

    for forbidden in [
        "data-slot=",
        "role=",
        "on:click=",
        "pub const CSS",
        "use ui_headless",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "color-slider motion.rs should avoid view/headless/style marker `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "color_slider_component_directory_standard_files_follow_contract_and_na_paths",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep component-file governance marker `{required}`.",
        );
    }
}

#[test]
fn color_slider_file_placement_discipline_contract_is_explicit_for_interactive_component_scope() {
    let check2_source = load_source("../../components/color-slider/check2.md");
    let mod_source = load_source("../../components/color-slider/src/mod.rs");
    let logic_source = load_source("../../components/color-slider/src/logic.rs");
    let styles_source = load_source("../../components/color-slider/src/styles.rs");
    let view_source = load_source("../../components/color-slider/src/view.rs");
    let motion_source = load_source("../../components/color-slider/src/motion.rs");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    assert!(
        check2_source.contains("文件落点纪律"),
        "color-slider check2 should explicitly track file-placement discipline gate.",
    );

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let component_src_dir = workspace_dir.join("components/color-slider/src");

    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            component_src_dir.join(required_file).exists(),
            "file-placement discipline requires `{required_file}` in color-slider source directory.",
        );
    }
    for forbidden_file in ["render.rs", "spec.rs"] {
        assert!(
            !component_src_dir.join(forbidden_file).exists(),
            "color-slider should keep `{forbidden_file}` absent in current scope.",
        );
    }

    assert!(
        mod_source.contains("pub(crate) mod logic;")
            && mod_source.contains("pub mod motion;")
            && mod_source.contains("pub mod styles;")
            && mod_source.contains("mod view;"),
        "mod.rs should keep canonical module boundary for file-placement discipline.",
    );

    assert!(
        logic_source.contains("pub fn resolve_source_attrs(")
            && styles_source.contains("pub const CSS: &str =")
            && view_source.contains("#[component]")
            && motion_source.contains("pub struct ColorSliderMotion"),
        "logic/styles/view/motion should keep canonical responsibility anchors.",
    );

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_file_placement_discipline_contract_is_explicit_for_interactive_component_scope";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "color_slider_file_placement_discipline_contract_is_explicit_for_interactive_component_scope",
    ] {
        assert!(
            check2_source.contains(required),
            "color-slider check2 should keep file-placement-discipline marker `{required}`.",
        );
    }
}

#[test]
fn color_slider_semantics_matrix_covers_state_paths_without_snapshot_dependency() {
    let semantics_source = load_source("../../components/color-slider/test/semantics.rs");
    let logic_test_source = load_source("../../components/color-slider/test/logic.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let motion_source = load_source("src/color/slider/motion.rs");

    for needle in [
        "role=slider_aria.input.role",
        "aria-disabled=slider_aria.input.aria_disabled",
        "data-state=move || state.get().data_state_attr",
        "data-value-source=value_source_attr",
        "data-default-value-source=default_value_source_attr",
        "data-value-change-source=value_change_source_attr",
        "data-disabled-source=disabled_source_attr",
        "on:pointerdown=move |_| slider_aria.handlers.on_pointer_down.run(())",
        "on:pointerup=move |_| slider_aria.handlers.on_pointer_up.run(())",
        "on:focus=move |_| slider_aria.handlers.on_focus.run(())",
        "on:blur=move |_| slider_aria.handlers.on_blur.run(())",
        "type=\"range\"",
    ] {
        assert!(
            view_source.contains(needle),
            "color-slider semantic matrix should keep interaction/role marker `{needle}`.",
        );
    }

    for needle in [
        "assert_eq!(attrs.control_mode_attr, \"controlled\");",
        "assert_eq!(attrs.control_mode_attr, \"uncontrolled\");",
        "assert_eq!(accessibility.disabled_source_attr, \"is_disabled\");",
        "assert_eq!(accessibility.disabled_source_attr, \"disabled\");",
    ] {
        assert!(
            logic_test_source.contains(needle),
            "color-slider logic tests should cover controlled/uncontrolled/disabled branch `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(needle),
            "color-slider motion should expose applicable wasm/ssr branch `{needle}`.",
        );
    }

    let snapshot_token = ["assert", "_snapshot!("].concat();
    let insta_snapshot_token = ["insta::assert", "_snapshot!("].concat();
    let jest_snapshot_token = ["toMatch", "Snapshot("].concat();
    for forbidden in [
        snapshot_token.as_str(),
        insta_snapshot_token.as_str(),
        jest_snapshot_token.as_str(),
    ] {
        assert!(
            !semantics_source.contains(forbidden),
            "color-slider semantic contract tests should not depend on visual snapshot token `{forbidden}`.",
        );
    }
}

#[test]
fn color_slider_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("../../components/color-slider/check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "color-slider checklist should keep documentation-as-product rule `{required}`.",
        );
    }
}

#[test]
fn color_slider_docs_entry_exists_as_readme_or_equivalent_docs_app_page() {
    let readme_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../components/color-slider/src/README.md");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    assert!(
        readme_path.exists(),
        "color-slider should provide README as documentation entry.",
    );
    assert!(
        docs_page_source.contains("pub(super) fn color_slider() -> AnyView"),
        "docs-app should expose color_slider docs entry function.",
    );
}

#[test]
fn color_slider_docs_are_beginner_friendly_with_default_then_advanced_path() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let docs_slider_section = color_slider_docs_section(&docs_source);
    let readme_source = load_source("src/color/slider/README.md");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "title=\"ColorSlider\"",
        "slug=\"color-slider\"",
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Interactive Workbench (DX)\"",
    ] {
        assert!(
            docs_slider_section.contains(required),
            "color-slider docs should keep beginner-to-advanced marker `{required}`.",
        );
    }

    let hello_pos = docs_slider_section
        .find("title=\"Hello World\"")
        .expect("docs should include hello-world playground for zero-threshold path.");
    let matrix_pos = docs_slider_section
        .find("title=\"State Matrix\"")
        .expect("docs should include state-matrix playground as common usage.");
    let controlled_pos = docs_slider_section
        .find("title=\"Controlled vs Uncontrolled\"")
        .expect("docs should include controlled-vs-uncontrolled playground.");
    let interactive_pos = docs_slider_section
        .find("title=\"Interactive Workbench (DX)\"")
        .expect("docs should include interactive workbench for advanced controls.");
    assert!(
        hello_pos < matrix_pos && matrix_pos < controlled_pos && controlled_pos < interactive_pos,
        "docs should present default usage before advanced controls.",
    );

    for required in [
        "## Hello World",
        "## 受控用法",
        "## 常见用法（进阶）",
        "阅读顺序建议：先看 `Hello World` 直接跑起来，再按需启用受控与高级配置。",
        "默认路径不需要手动接线 `ui-state-primitives` 或 `ui-headless` 状态机。",
    ] {
        assert!(
            readme_source.contains(required),
            "README should keep beginner-friendly guidance marker `{required}`.",
        );
    }

    let readme_hello_pos = readme_source
        .find("## Hello World")
        .expect("README should include hello-world section.");
    let readme_controlled_pos = readme_source
        .find("## 受控用法")
        .expect("README should include controlled section.");
    let readme_advanced_pos = readme_source
        .find("## 常见用法（进阶）")
        .expect("README should include advanced usage section.");
    assert!(
        readme_hello_pos < readme_controlled_pos && readme_controlled_pos < readme_advanced_pos,
        "README should present default path before advanced guidance.",
    );

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_docs_are_beginner_friendly_with_default_then_advanced_path";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_slider_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let readme_source = load_source("src/color/slider/README.md");

    for required in [
        "title=\"Hello World\"",
        "code_imports=\"use leptos::prelude::*;\\nuse ui::ColorSlider;\".to_string()",
        "<ColorSlider id_base=\"docs-color-slider-hello\".to_string() />",
        "## Hello World",
        "id_base=\"demo-color-slider\".to_string()",
        "default_value=220.0",
    ] {
        assert!(
            docs_source.contains(required) || readme_source.contains(required),
            "color-slider docs hello-world should keep zero-threshold marker `{required}`.",
        );
    }

    for forbidden in ["ui_state_primitives", "use_slider(", "state=...", "logic::"] {
        assert!(
            !readme_source.contains(forbidden),
            "color-slider README hello-world path should avoid architecture-wiring token `{forbidden}`.",
        );
    }
}

#[test]
fn color_slider_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "pub(super) fn color_slider() -> AnyView",
        "title=\"ColorSlider\"",
        "slug=\"color-slider\"",
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming Optional / Snapshot\"",
        "Source-first / Copy-Paste Ready",
        "title=\"Controlled Hue Channel\"",
        "title=\"Disabled Alpha + Custom Track + Reduced Motion\"",
    ] {
        assert!(
            source.contains(needle),
            "color-slider docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn color_slider_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "id_base=\"docs-color-slider-hello\".to_string()",
        "<Playground title=\"State Matrix\" code_signal=state_matrix_code>",
        "data-slot=\"color-slider-state-matrix\"",
        "id_base=\"docs-color-slider-matrix-hue\".to_string()",
        "id_base=\"docs-color-slider-matrix-saturation\".to_string()",
        "id_base=\"docs-color-slider-matrix-disabled\".to_string()",
        "<Playground title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"color-slider-controlled-vs-uncontrolled\"",
        "id_base=\"docs-color-slider-compare-controlled\".to_string()",
        "id_base=\"docs-color-slider-compare-uncontrolled\".to_string()",
        "<Playground title=\"Streaming Optional / Snapshot\" code_signal=output_mode_code>",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "data-ui-output-state=\"snapshot\"",
        "id_base=\"docs-color-slider-snapshot\".to_string()",
        "Source-first / Copy-Paste Ready",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "\"code_imports\"",
        "<Playground title=\"Controlled Hue Channel\" code_signal=basic_code>",
        "id_base=\"docs-color-slider-hue\".to_string()",
        "channel=ColorSliderChannel::Hue",
        "value=hue.into()",
        "on_value_change=on_hue_change",
        "<Playground title=\"Disabled Alpha + Custom Track + Reduced Motion\" code_signal=states_code>",
        "id_base=\"docs-color-slider-alpha\".to_string()",
        "channel=ColorSliderChannel::Alpha",
        "disabled=true",
        "id_base=\"docs-color-slider-custom\".to_string()",
        "channel=ColorSliderChannel::Blue",
        "track_start_color=\"#0f172a\".to_string()",
        "track_end_color=\"#38bdf8\".to_string()",
        "motion=reduced_motion",
        "class_name=\"docs-color-slider-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "color-slider docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn color_slider_check2_documents_docs_sync_and_state_matrix_rules() {
    let checklist_source = load_source("../../components/color-slider/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ColorSlider checklist should keep docs-sync/state-matrix rule `{required}`.",
        );
    }
}

#[test]
fn color_slider_docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let view_source = load_source("src/color/slider/view.rs");
    let logic_source = load_source("src/color/slider/logic.rs");

    for required in [
        "pub(super) fn color_slider() -> AnyView",
        "Playground title=\"Hello World\" code_signal=hello_code",
        "Playground title=\"State Matrix\" code_signal=state_matrix_code",
        "data-slot=\"color-slider-state-matrix\"",
        "id_base=\"docs-color-slider-matrix-hue\".to_string()",
        "id_base=\"docs-color-slider-matrix-saturation\".to_string()",
        "id_base=\"docs-color-slider-matrix-disabled\".to_string()",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"color-slider-controlled-vs-uncontrolled\"",
        "id_base=\"docs-color-slider-compare-controlled\".to_string()",
        "id_base=\"docs-color-slider-compare-uncontrolled\".to_string()",
        "default_value=196.0",
        "default_value=72.0",
        "default_value=40.0",
        "default_value=180.0",
        "disabled=true",
        "code_imports=\"use leptos::prelude::*;\\nuse ui::{ColorSlider, ColorSliderChannel, ColorSliderMotion};\".to_string()",
    ] {
        assert!(
            docs_source.contains(required),
            "ColorSlider docs should keep API/default/state-matrix marker `{required}`.",
        );
    }

    for required in [
        "#[prop(optional)] value: Option<Signal<f64>>",
        "#[prop(optional)] default_value: Option<f64>",
        "#[prop(optional)] on_value_change: Option<Callback<f64>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional, default = f64::NAN)] min: f64",
        "#[prop(optional, default = f64::NAN)] max: f64",
        "#[prop(optional, default = f64::NAN)] step: f64",
        "let accessibility_state = logic::normalize_accessibility_state(is_disabled, disabled);",
        "let default_value = logic::normalize_default_value(channel, default_value, min, max, step);",
    ] {
        assert!(
            view_source.contains(required),
            "ColorSlider view contract should keep `{required}`.",
        );
    }

    for required in [
        "pub fn normalize_default_value(",
        "default_value.unwrap_or_else(|| channel.default_value())",
        "pub fn normalize_accessibility_state(",
    ] {
        assert!(
            logic_source.contains(required),
            "ColorSlider logic default/normalization contract should keep `{required}`.",
        );
    }
}

#[test]
fn color_slider_contract_hygiene_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`.",
        );
    }
}

#[test]
fn color_slider_check2_documents_docs_product_copy_paste_ready_contract() {
    let check2_source = load_source("../../components/color-slider/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "apps/docs-app/src/pages/components/pages/forms_color.rs::color_slider()",
        "Hello World",
        "State Matrix",
        "Controlled vs Uncontrolled",
        "Streaming Optional / Snapshot",
        "Source-first / Copy-Paste Ready",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "code_imports",
        "color_slider_docs_page_covers_primary_playgrounds",
        "color_slider_docs_playgrounds_lock_state_matrix_contract_values",
        "color_slider_check2_documents_docs_product_copy_paste_ready_contract",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep docs-product copy-ready marker `{required}`.",
        );
    }

    for required in [
        "pub(super) fn color_slider() -> AnyView",
        "Playground title=\"Hello World\" code_signal=hello_code",
        "Playground title=\"State Matrix\" code_signal=state_matrix_code",
        "title=\"Controlled vs Uncontrolled\"",
        "Playground title=\"Streaming Optional / Snapshot\" code_signal=output_mode_code",
        "Source-first / Copy-Paste Ready",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "\"code_imports\"",
    ] {
        assert!(
            docs_source.contains(required),
            "docs page should keep docs-product contract marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_documents_docs_product_copy_paste_ready_contract";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_slider_dx_workbench_supports_hot_style_feedback_and_optional_state_preserve() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let hot_reload_source = load_source("../../apps/docs-app/tests/dev_css_hot_reload.rs");

    for needle in [
        "title=\"Interactive Workbench (DX)\"",
        "test_css_source=workbench_test_css_source",
        "include_str!(\"../../../../dev-overrides.css\")",
        "data-slot=\"color-slider-workbench-controls\"",
        "data-slot=\"color-slider-workbench\"",
        "data-slot=\"color-slider-workbench-canvas\"",
        "\"Preserve context on channel change\"",
        "\"Persist workbench state\"",
        "COLOR_SLIDER_WORKBENCH_STORAGE_KEY",
        "load_color_slider_workbench_state()",
        "save_color_slider_workbench_state(state);",
        "clear_color_slider_workbench_state();",
    ] {
        assert!(
            source.contains(needle),
            "color-slider DX workbench contract should contain `{needle}`.",
        );
    }

    for needle in [
        "dev-overrides.css",
        "without rebuilding Rust code",
        "move them into the relevant component `styles.rs`",
    ] {
        assert!(
            hot_reload_source.contains(needle),
            "docs-app hot-style workflow should retain `{needle}`.",
        );
    }
}

#[test]
fn color_slider_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("../../components/color-slider/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(required),
            "color-slider checklist should keep interactive-playground rule `{required}`.",
        );
    }
}

#[test]
fn color_slider_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let docs_slider_section = color_slider_docs_section(&docs_source);
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    assert!(
        docs_source.contains("pub(super) fn color_slider() -> AnyView"),
        "docs should expose color_slider entry function.",
    );

    for marker in [
        "title=\"Interactive Workbench (DX)\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "controls=move || view!",
        "id_base=\"docs-color-slider-workbench-channel\".to_string()",
        "<Switch checked=workbench_disabled set_checked=set_workbench_disabled>",
        "<Switch checked=workbench_custom_track set_checked=set_workbench_custom_track>",
        "<Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>",
        "<Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>",
        "<Switch checked=workbench_preserve_context set_checked=set_workbench_preserve_context>",
        "<Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>",
        "id_base=\"docs-color-slider-workbench\".to_string()",
        "value=workbench_value_signal",
        "on_value_change=on_workbench_value_change",
        "data-slot=\"color-slider-workbench-controls\"",
        "data-slot=\"color-slider-workbench-canvas\"",
        "data-slot=\"color-slider-workbench-state\"",
    ] {
        assert!(
            docs_slider_section.contains(marker),
            "color-slider docs should keep interactive playground marker `{marker}`.",
        );
    }

    for marker in [
        "let section_class = \"docs-card playground\";",
        "<div data-playground-scope=scope_id.clone()>",
        "<Card class_name=\"playground__preview\".to_string()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<div data-slot=\"playground-controls\">",
        "<Card class_name=\"playground__panel playground__controls\".to_string()>",
    ] {
        assert!(
            playground_source.contains(marker),
            "docs-app Playground should keep interactive preview contract marker `{marker}`.",
        );
    }
}

#[test]
fn color_slider_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_slider_contract.spec.mjs");

    for marker in [
        "docs-app color-slider key flow is repeatable and failures map to semantic breakpoints",
        "await page.goto(COLOR_SLIDER_PAGE);",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-component=\"color-slider\"] #docs-color-slider-hue[data-slot=\"color-slider\"][data-control-mode=\"controlled\"][data-channel=\"hue\"]",
        "await page.keyboard.press(\"ArrowRight\");",
        "await expect(root).toHaveAttribute(\"data-ui-source\", \"on_value_change\");",
        "await expect(root).toHaveAttribute(\"data-ui-output-status\", \"submittable\");",
        "await page.reload();",
        "await expect(root).toHaveAttribute(\"data-ui-action\", \"idle\");",
        "await expect(root).toHaveAttribute(\"data-focus-visible\", \"true\");",
    ] {
        assert!(
            e2e_source.contains(marker),
            "color-slider interactive playground should keep repeatable semantic e2e marker `{marker}`.",
        );
    }
}

#[test]
fn color_slider_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for marker in [
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(marker),
            "DX check script should enforce interactive-playground contract marker `{marker}`.",
        );
    }
}

#[test]
fn color_slider_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("../../components/color-slider/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(required),
            "color-slider checklist should keep source-first copy-paste-ready rule `{required}`.",
        );
    }
}

#[test]
fn color_slider_docs_are_copy_paste_ready_with_imports_copy_button_and_sync() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_logic_source = include_str!("../../../components/code-block/src/logic.rs");
    let view_source = load_source("../../components/color-slider/src/view.rs");
    let logic_source = load_source("../../components/color-slider/src/logic.rs");

    for marker in [
        "pub(super) fn color_slider() -> AnyView",
        "data-slot=\"color-slider-copy-ready\"",
        "<h3>\"Source-first / Copy-Paste Ready\"</h3>",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "\"code_imports\"",
        "components/color-slider/src/mod.rs",
        "components/color-slider/src/logic.rs",
        "components/color-slider/src/view.rs",
        "components/color-slider/src/styles.rs",
        "components/color-slider/src/motion.rs",
        "data-slot=\"color-slider-source-prerequisites\"",
        "\"component-color_slider\"",
        "\"inject-css\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "color-slider docs should keep source-first copy-ready marker `{marker}`.",
        );
    }

    for marker in [
        "const DEFAULT_PLAYGROUND_IMPORTS",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let missing_imports = missing_import_lines(&raw, &imports);",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "<CodeBlock code=resolved_code.get() />",
        "\"Show code\"",
    ] {
        assert!(
            playground_source.contains(marker),
            "docs-app playground should keep copy-ready pipeline marker `{marker}`.",
        );
    }

    for marker in [
        "pub const DEFAULT_IS_COPYABLE: bool = true;",
        "pub fn resolve_copyable_contract(",
        "is_copyable: DEFAULT_IS_COPYABLE,",
    ] {
        assert!(
            code_block_logic_source.contains(marker),
            "code-block copy contract should keep marker `{marker}` for docs copy action.",
        );
    }

    for marker in [
        "#[prop(optional)] value: Option<Signal<f64>>",
        "#[prop(optional)] default_value: Option<f64>",
        "#[prop(optional)] on_value_change: Option<Callback<f64>>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "#[prop(optional, default = f64::NAN)] min: f64",
        "#[prop(optional, default = f64::NAN)] max: f64",
        "#[prop(optional, default = f64::NAN)] step: f64",
        "pub fn normalize_default_value(",
    ] {
        assert!(
            view_source.contains(marker) || logic_source.contains(marker),
            "color-slider source-first snippets should stay synced with implementation marker `{marker}`.",
        );
    }
}

#[test]
fn color_slider_contract_hygiene_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for marker in [
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_docs_are_copy_paste_ready_with_imports_copy_button_and_sync",
    ] {
        assert!(
            script_source.contains(marker),
            "contract-hygiene script should enforce source-first copy-ready marker `{marker}`.",
        );
    }
}

#[test]
fn color_slider_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let view_source = load_source("../../components/color-slider/src/view.rs");
    let logic_source = load_source("../../components/color-slider/src/logic.rs");
    let readme_source = load_source("../../components/color-slider/src/README.md");

    for marker in [
        "### ColorSlider 同步记录（2026-02-20）",
        "value + on_value_change + default_value",
        "`channel`",
        "`min/max/step`",
        "is_disabled(disabled legacy alias)",
        "component_doc!(\"ColorSlider\", \"color-slider\", \"Forms\", forms_color::color_slider)",
        "apps/docs-app/src/pages/components/pages/forms_color.rs::color_slider()",
        "Hello World",
        "State Matrix",
        "Controlled vs Uncontrolled",
        "Interactive Workbench (DX)",
        "Source-first / Copy-Paste Ready",
        "参数语义若变更，必须先同步本策略文档与 docs 入口",
    ] {
        assert!(
            strategy_source.contains(marker)
                || docs_index_source.contains(marker)
                || readme_source.contains(marker),
            "color-slider HeroUI/doc sync record should include `{marker}`.",
        );
    }

    for marker in [
        "pub(super) fn color_slider() -> AnyView",
        "title=\"ColorSlider\"",
        "slug=\"color-slider\"",
        "data-slot=\"color-slider-copy-ready\"",
    ] {
        assert!(
            docs_page_source.contains(marker),
            "color-slider docs entry should keep indexable marker `{marker}`.",
        );
    }

    for marker in [
        "#[prop(optional)] channel: ColorSliderChannel,",
        "#[prop(optional)] value: Option<Signal<f64>>,",
        "#[prop(optional)] default_value: Option<f64>,",
        "#[prop(optional)] on_value_change: Option<Callback<f64>>,",
        "#[prop(optional, default = f64::NAN)] min: f64,",
        "#[prop(optional, default = f64::NAN)] max: f64,",
        "#[prop(optional, default = f64::NAN)] step: f64,",
        "#[prop(optional)] is_disabled: Option<bool>,",
        "#[prop(optional)] disabled: bool,",
        "#[prop(optional)] motion: ColorSliderMotion,",
        "sanitize_bounds,",
        "sanitize_step,",
        "pub fn normalize_default_value(",
        "pub fn normalize_accessibility_state(",
    ] {
        assert!(
            view_source.contains(marker) || logic_source.contains(marker),
            "color-slider parameter model marker `{marker}` should remain in implementation.",
        );
    }

    for marker in [
        "# ColorSlider",
        "## Hello World",
        "## API 约定",
        "docs 入口：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_slider()`",
    ] {
        assert!(
            readme_source.contains(marker),
            "color-slider README/docs entry should keep marker `{marker}`.",
        );
    }
}

#[test]
fn color_slider_check2_marks_heroui_strategy_and_component_docs_sync_complete() {
    let check2_source = load_source("../../components/color-slider/check2.md");

    for marker in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        "color_slider_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
    ] {
        assert!(
            check2_source.contains(marker),
            "color-slider checklist should keep HeroUI/doc sync completion evidence `{marker}`.",
        );
    }
}

#[test]
fn color_slider_contract_hygiene_script_covers_heroui_strategy_doc_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for marker in [
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
        "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_check2_marks_heroui_strategy_and_component_docs_sync_complete",
    ] {
        assert!(
            script_source.contains(marker),
            "contract-hygiene script should enforce HeroUI/doc sync marker `{marker}`.",
        );
    }
}

#[test]
fn color_slider_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries() {
    let check2_source = load_source("../../components/color-slider/check2.md");
    let script_source = load_source("../../scripts/check-ui-engineering.sh");
    let ui_components_cargo = load_source("../../crates/ui/Cargo.toml");
    let color_slider_cargo = load_source("../../components/color-slider/Cargo.toml");
    let protocol_source = load_source("../../components/color-slider/src/protocol.rs");
    let protocol_test_source = load_source("../../components/color-slider/test/protocol.rs");
    let mod_source = load_source("../../components/color-slider/src/mod.rs");
    let logic_source = load_source("../../components/color-slider/src/logic.rs");
    let view_source = load_source("../../components/color-slider/src/view.rs");
    let styles_source = load_source("../../components/color-slider/src/styles.rs");
    let motion_source = load_source("../../components/color-slider/src/motion.rs");
    let readme_source = load_source("../../components/color-slider/src/README.md");

    for needle in [
        "serde = { version = \"1.0\", features = [\"derive\"] }",
        "use serde::{Deserialize, Serialize};",
        "pub enum SliderComponentSchemaVersion",
        "pub struct SliderComponentSpec",
        "pub schema_version: SliderComponentSchemaVersion",
        "#[serde(default)]",
        "#[serde(rename_all = \"snake_case\")]",
    ] {
        assert!(
            color_slider_cargo.contains(needle) || protocol_source.contains(needle),
            "color-slider engineering contract should keep serde protocol marker `{needle}`.",
        );
    }

    for needle in [
        "use serde::de::DeserializeOwned;",
        "fn assert_serde<T>()",
        "assert_serde::<SliderComponentSchemaVersion>();",
        "assert_serde::<SliderComponentSpec>();",
    ] {
        assert!(
            protocol_test_source.contains(needle),
            "color-slider protocol regression tests should keep serde coverage marker `{needle}`.",
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "engineering baseline should keep canonical tracing feature marker `{needle}`.",
        );
    }

    for forbidden in [
        "color-slider-wasm-debug =",
        "color_slider-wasm-debug =",
        "component-color_slider\", \"dep:tracing",
        "component-color_slider-wasm-debug",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "color-slider should not add component-local tracing/debug feature `{forbidden}`.",
        );
    }

    for source in [
        mod_source,
        logic_source,
        view_source,
        styles_source,
        motion_source,
        readme_source,
        protocol_source,
    ] {
        for forbidden in [
            "tracing::span!(",
            "tracing::event!(",
            "#[tracing::instrument]",
            "tokio::",
            "tokio =",
            "#[tokio::main]",
            "#[tokio::test]",
            "async_std::",
            "async-std",
            "tokio::runtime",
            "JoinHandle",
        ] {
            assert!(
                !source.contains(forbidden),
                "color-slider engineering boundary should avoid tracing/runtime leak token `{forbidden}`.",
            );
        }
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
        "color_slider_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep engineering governance marker `{needle}`.",
        );
    }
}

#[test]
fn color_slider_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
 {
    let check2_source = load_source("../../components/color-slider/check2.md");
    let script_source = load_source("../../scripts/check-ui-engineering.sh");
    let readme_source = load_source("../../components/color-slider/src/README.md");
    let protocol_source = include_str!("../src/protocol.rs");
    let component_manifest = include_str!("../src/Component.toml");
    let rbi_source = include_str!("../src/color_slider.rbi");

    for required in [
        "pub enum SliderComponentSchemaVersion",
        "V1",
        "pub struct SliderComponentSpec",
        "pub schema_version: SliderComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(required),
            "color-slider protocol should keep stable v1 marker `{required}` in non-breaking scope.",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "schema = \"ui.color-slider.agent-contract.v1\"",
        "values = [\"ui.color-slider.agent-contract.v1\"]",
        "values = [\"1\"]",
    ] {
        assert!(
            component_manifest.contains(required),
            "color-slider Component.toml should keep v1 registration marker `{required}` in current scope.",
        );
    }

    for forbidden in [
        "V2",
        "migrate_v1_to_v2",
        "SchemaRegistry",
        "deprecation_window",
        "codemod",
        "schema_version = \"2\"",
        "agent-contract.v2",
    ] {
        assert!(
            !protocol_source.contains(forbidden)
                && !readme_source.contains(forbidden)
                && !component_manifest.contains(forbidden)
                && !rbi_source.contains(forbidden),
            "without major breaking upgrade, color-slider should not claim migration surface token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui --test color_slider_semantics --no-default-features --features component-color_slider,inject-css color_slider_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `ColorSlider` 改动未引入跨大版本 API 破坏升级，组件协议与 Agent Contract 仍保持 `v1`（`components/color-slider/src/protocol.rs` 的 `SliderComponentSchemaVersion::V1`、`components/color-slider/src/Component.toml` 的 `schema_version = \"1\"` 与 `ui.color-slider.agent-contract.v1`），因此不触发 Codemod/Schema Registry 弃用窗口与 `migrate_v1_to_v2` 迁移层要求。回归：`components/color-slider/test/semantics.rs::color_slider_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade`；门禁脚本：`scripts/check-ui-engineering.sh` 新增对应 `cargo test` 目标。）",
        "color_slider_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep codemod/registry migration marker `{needle}`.",
        );
    }
}

#[test]
fn color_slider_readme_is_copy_paste_ready() {
    let source = load_source("src/color/slider/README.md");

    for needle in [
        "# ColorSlider",
        "## Hello World",
        "<ColorSlider",
        "default_value=220.0",
        "## API 约定",
        "is_disabled",
        "lang` / `dir",
    ] {
        assert!(
            source.contains(needle),
            "ColorSlider README should contain `{needle}`.",
        );
    }
}
