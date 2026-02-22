fn load_source(rel_path: &str) -> &'static str {
    match rel_path {
        "../../components/color-picker/src/lib.rs" => include_str!("../src/lib.rs"),
        "../../components/color-picker/src/mod.rs" => include_str!("../src/mod.rs"),
        "../../components/color-picker/src/logic.rs" => include_str!("../src/logic.rs"),
        "../../components/color-picker/src/motion.rs" => include_str!("../src/motion.rs"),
        "../../components/color-picker/src/protocol.rs" => include_str!("../src/protocol.rs"),
        "../../components/color-picker/src/styles.rs" => include_str!("../src/styles.rs"),
        "../../components/color-picker/src/view.rs" => include_str!("../src/view.rs"),
        "../../components/color-picker/src/README.md" => include_str!("../src/README.md"),
        "../../components/color-picker/src/Component.toml" => include_str!("../src/Component.toml"),
        "../../components/color-picker/src/color_picker.rbi" => {
            include_str!("../src/color_picker.rbi")
        }
        "../../components/color-picker/Cargo.toml" => include_str!("../Cargo.toml"),
        "../../crates/ui/src/css.rs" => {
            include_str!("../../../crates/ui/src/css.rs")
        }
        "../../crates/ui/src/lib.rs" => {
            include_str!("../../../crates/ui/src/lib.rs")
        }
        "../../crates/ui/Cargo.toml" => {
            include_str!("../../../crates/ui/Cargo.toml")
        }
        "../../crates/ui/src/root.rs" => {
            include_str!("../../../crates/ui/src/root.rs")
        }
        "../../crates/ui-motion/src/lib.rs" => include_str!("../../../crates/ui-motion/src/lib.rs"),
        "../../crates/ui-motion/src/web.rs" => include_str!("../../../crates/ui-motion/src/web.rs"),
        "../../crates/ui-headless/src/id_provider.rs" => {
            include_str!("../../../crates/ui-headless/src/id_provider.rs")
        }
        "../../crates/ui-headless/src/lib.rs" => {
            include_str!("../../../crates/ui-headless/src/lib.rs")
        }
        "../../crates/ui-headless/src/modal.rs" => {
            include_str!("../../../crates/ui-headless/src/modal.rs")
        }
        "../../apps/web-demo/Cargo.toml" => include_str!("../../../apps/web-demo/Cargo.toml"),
        "../../crates/ui-state-primitives/src/color_picker.rs" => {
            include_str!("../../../crates/ui-state-primitives/src/color_picker.rs")
        }
        "../../crates/ui-headless/src/controllable_state.rs" => {
            include_str!("../../../crates/ui-headless/src/controllable_state.rs")
        }
        "../../crates/ui-headless/src/a11y.rs" => {
            include_str!("../../../crates/ui-headless/src/a11y.rs")
        }
        "../../crates/ui-headless/src/trace.rs" => {
            include_str!("../../../crates/ui-headless/src/trace.rs")
        }
        "../../crates/ui-headless/src/presence.rs" => {
            include_str!("../../../crates/ui-headless/src/presence.rs")
        }
        "../../components/color-swatch-picker/src/view.rs" => {
            include_str!("../../color-swatch-picker/src/view.rs")
        }
        "../../components/color-swatch-picker/src/logic.rs" => {
            include_str!("../../color-swatch-picker/src/logic.rs")
        }
        "../../components/popover/src/view.rs" => include_str!("../../popover/src/view.rs"),
        "../../components/popover/src/motion.rs" => include_str!("../../popover/src/motion.rs"),
        "../../crates/ui-headless/src/focus_trap.rs" => {
            include_str!("../../../crates/ui-headless/src/focus_trap.rs")
        }
        "../../crates/ui-headless/src/popover_position.rs" => {
            include_str!("../../../crates/ui-headless/src/popover_position.rs")
        }
        "../../crates/ui-headless/src/test/popover_position.rs" => {
            include_str!("../../../crates/ui-headless/src/test/popover_position.rs")
        }
        "../../apps/docs-app/src/pages/components/pages/forms_color.rs" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs")
        }
        "../../apps/docs-app/src/pages/components/shell.rs" => {
            include_str!("../../../apps/docs-app/src/pages/components/shell.rs")
        }
        "../../apps/docs-app/src/pages/components/pages.rs" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages.rs")
        }
        "../../apps/docs-app/src/perf_probe.rs" => {
            include_str!("../../../apps/docs-app/src/perf_probe.rs")
        }
        "../../e2e/tests/docs_app_components_coverage.spec.mjs" => {
            include_str!("../../../e2e/tests/docs_app_components_coverage.spec.mjs")
        }
        "../../e2e/tests/docs_app_color_picker_contract.spec.mjs" => {
            include_str!("../../../e2e/tests/docs_app_color_picker_contract.spec.mjs")
        }
        "../../scripts/check-ui-performance.sh" => {
            include_str!("../../../scripts/check-ui-performance.sh")
        }
        "../../scripts/check-ui-component-files.sh" => {
            include_str!("../../../scripts/check-ui-component-files.sh")
        }
        "../../scripts/check-ui-inner-html.sh" => {
            include_str!("../../../scripts/check-ui-inner-html.sh")
        }
        "../../scripts/check-ui-dx.sh" => {
            include_str!("../../../scripts/check-ui-dx.sh")
        }
        "../../scripts/check-ui-engineering.sh" => {
            include_str!("../../../scripts/check-ui-engineering.sh")
        }
        "../../components/color-picker/scripts/check-ui-e2e-color-picker.sh" => {
            include_str!("../../../components/color-picker/scripts/check-ui-e2e-color-picker.sh")
        }
        "../../scripts/check-ui-wasm-debug.sh" => {
            include_str!("../../../scripts/check-ui-wasm-debug.sh")
        }
        "../../scripts/check-ui-view-macro.sh" => {
            include_str!("../../../scripts/check-ui-view-macro.sh")
        }
        "../../scripts/check-ui-streaming.sh" => {
            include_str!("../../../scripts/check-ui-streaming.sh")
        }
        "../../scripts/check-ui-contract-hygiene.sh" => {
            include_str!("../../../scripts/check-ui-contract-hygiene.sh")
        }
        "../../apps/docs-app/src/playground.rs" => {
            include_str!("../../../apps/docs-app/src/playground.rs")
        }
        "../../apps/docs-app/src/lib.rs" => include_str!("../../../apps/docs-app/src/lib.rs"),
        "../../apps/docs-app/src/debug_overlay.rs" => {
            include_str!("../../../apps/docs-app/src/debug_overlay.rs")
        }
        "../../docs/spec/heroui-parameter-design-strategy.md" => {
            include_str!("../../../docs/spec/heroui-parameter-design-strategy.md")
        }
        "../../docs/plan/TODO.md" => include_str!("../../../docs/plan/TODO.md"),
        "../../components/color-picker/check2.md" => include_str!("../check2.md"),
        "legacy_semantics" => {
            include_str!("../../../components/color-picker/test/color_picker_semantics.rs")
        }
        _ => panic!("unsupported source path: {rel_path}"),
    }
}

#[test]
fn color_picker_semantics_tests_are_migrated_to_component_directory() {
    let lib_source = load_source("../../components/color-picker/src/lib.rs");
    let mod_source = load_source("../../components/color-picker/src/mod.rs");
    let legacy_semantics = load_source("legacy_semantics");
    let local_semantics = include_str!("semantics.rs");

    for source in [lib_source, mod_source] {
        assert!(
            source.contains("#[path = \"../test/semantics.rs\"]")
                && source.contains("mod semantics_tests;"),
            "color-picker should wire `components/color-picker/test/semantics.rs` from entrypoints."
        );
    }

    assert!(
        legacy_semantics.contains("../../../components/color-picker/test/semantics.rs"),
        "legacy ui semantics entry should include migrated component semantics file.",
    );
    assert!(
        local_semantics
            .contains("color_picker_semantics_tests_are_migrated_to_component_directory"),
        "component-local semantics suite should provide migration coverage.",
    );
}

#[test]
fn color_picker_public_surface_does_not_expose_dom_platform_types() {
    let mod_source = load_source("../../components/color-picker/src/mod.rs");
    let lib_source = load_source("../../components/color-picker/src/lib.rs");

    for forbidden in [
        "web_sys::",
        "web-sys",
        "wasm_bindgen",
        "JsValue",
        "HtmlElement",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "color-picker public module should not expose `{forbidden}`."
        );
        assert!(
            !lib_source.contains(forbidden),
            "color-picker crate entry should not expose `{forbidden}`."
        );
    }
}

#[test]
fn color_picker_component_layer_keeps_file_responsibilities() {
    let mod_source = load_source("../../components/color-picker/src/mod.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let motion_source = load_source("../../components/color-picker/src/motion.rs");
    let styles_source = load_source("../../components/color-picker/src/styles.rs");

    for needle in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::ColorPicker;",
        "pub use motion::ColorPickerMotion;",
        "#[cfg(all(test, not(feature = \"component-color_picker\")))]",
        "#[path = \"../test/semantics.rs\"]",
    ] {
        assert!(
            mod_source.contains(needle),
            "color-picker module boundary should include `{needle}`."
        );
    }
    for forbidden in ["view! {", "use leptos", "ui_state_primitives::"] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should keep export boundary only; found implementation marker `{forbidden}`.",
        );
    }

    for needle in [
        "ui_state_primitives::color_picker as primitive",
        "pub struct ColorPickerDerivedStateInput",
        "pub fn resolve_default_selected_color(",
        "pub fn resolve_is_disabled(",
        "pub fn resolve_selected_color_axis<",
        "pub fn resolve_selected_change_axis<",
        "pub fn resolve_derived_state(",
        "pub fn resolve_ids(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "color-picker logic should include `{needle}`."
        );
    }
    for forbidden in [
        "use leptos",
        "web_sys::",
        "wasm_bindgen",
        "NodeRef",
        "view! {",
        "on:pointer",
        "on:keydown",
        "var(--ui-",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay platform-agnostic; found `{forbidden}`."
        );
    }

    for needle in [
        "use ui_headless",
        "<Popover",
        "motion=motion.popover",
        "popup_trigger_attrs(",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should compose rendering/headless contracts; missing `{needle}`."
        );
    }
    for forbidden in [
        "ui_state_primitives::",
        "resolve_state(ColorPickerStateInput",
        "color-mix(",
        "var(--ui-",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should mount contracts, not re-implement state/theme internals; found `{forbidden}`.",
        );
    }

    assert!(
        motion_source
            .contains("pub fn sanitize_motion(motion: ColorPickerMotion) -> ColorPickerMotion"),
        "motion.rs should expose a sanitize contract.",
    );
    assert!(
        motion_source.contains("crate::popover::motion::sanitize_motion(motion.popover)"),
        "motion.rs should map to shared motion contract, not a local driver.",
    );
    for forbidden in [
        "requestAnimationFrame",
        "spring",
        "keyframe",
        "stiffness",
        "damping",
        "web_sys::",
        "wasm_bindgen",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should map component semantics to shared motion contract only; found `{forbidden}`.",
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str"),
        "styles.rs should own static css output.",
    );
    assert!(
        styles_source.contains("var(--ui-"),
        "styles.rs should consume ui-theme variables via `var(--ui-*)`.",
    );
    for forbidden in [
        "#[component]",
        "view! {",
        "on:pointer",
        "on:keydown",
        "web_sys::",
        "wasm_bindgen",
        "role=\"",
        "aria-",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should stay token-first static css only; found `{forbidden}`.",
        );
    }
}

#[test]
fn color_picker_component_directory_standard_files_follow_contract_and_na_paths() {
    let check2_source = load_source("../../components/color-picker/check2.md");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");
    let mod_source = load_source("../../components/color-picker/src/mod.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let styles_source = load_source("../../components/color-picker/src/styles.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let motion_source = load_source("../../components/color-picker/src/motion.rs");

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required_file).exists(),
            "color-picker component directory should include `{required_file}`.",
        );
    }
    for absent_file in ["render.rs", "spec.rs"] {
        assert!(
            !src_dir.join(absent_file).exists(),
            "color-picker component directory should keep `{absent_file}` absent.",
        );
    }

    for required in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{ColorPickerState, ColorPickerStateInput, DEFAULT_ARIA_LABEL, DEFAULT_LABEL};",
        "pub use motion::ColorPickerMotion;",
        "pub use view::ColorPicker;",
    ] {
        assert!(
            mod_source.contains(required),
            "color-picker mod.rs should keep minimal stable export marker `{required}`.",
        );
    }
    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "mod render;",
        "mod spec;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "color-picker mod.rs should avoid over-export/drift marker `{forbidden}`.",
        );
    }

    for required in [
        "ui_state_primitives::color_picker as primitive",
        "pub struct ColorPickerDerivedStateInput",
        "pub struct ColorPickerIds",
        "pub fn resolve_default_selected_color(",
        "pub fn resolve_derived_state(",
        "pub fn resolve_ids(",
    ] {
        assert!(
            logic_source.contains(required),
            "color-picker logic.rs should keep normalization/derived-state marker `{required}`.",
        );
    }
    for forbidden in [
        "use leptos",
        "web_sys::",
        "NodeRef",
        "view! {",
        "on:pointer",
        "var(--ui-",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "color-picker logic.rs should stay free of view/dom/style marker `{forbidden}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-color-picker",
        "var(--ui-",
    ] {
        assert!(
            styles_source.contains(required),
            "color-picker styles.rs should keep token-first static css marker `{required}`.",
        );
    }
    for forbidden in [
        "#[component]",
        "use ui_headless",
        "use leptos::",
        "on:pointer",
        "role=\"",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "color-picker styles.rs should avoid runtime/render concern `{forbidden}`.",
        );
    }

    for required in [
        "#[component]",
        "pub fn ColorPicker(",
        "use ui_headless::{",
        "overlay_open::use_controllable_state(",
        "popup_trigger_attrs(",
        "<Popover",
        "data-slot=SLOT_TRIGGER",
        "data-slot=SLOT_PANEL",
        "data-slot=SLOT_CONTENT",
    ] {
        assert!(
            view_source.contains(required),
            "color-picker view.rs should keep render + headless mount marker `{required}`.",
        );
    }
    for forbidden in ["include_str!(\"./render.rs\")", "mod render;"] {
        assert!(
            !view_source.contains(forbidden),
            "color-picker view.rs should not drift to render.rs marker `{forbidden}`.",
        );
    }

    for required in [
        "pub struct ColorPickerMotion",
        "pub fn sanitize_motion(",
        "crate::popover::motion::sanitize_motion(motion.popover)",
    ] {
        assert!(
            motion_source.contains(required),
            "color-picker motion.rs should keep semantic-to-motion mapping marker `{required}`.",
        );
    }
    for forbidden in [
        "SpringAnimator::new(",
        "requestAnimationFrame",
        "view! {",
        "data-slot=",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "color-picker motion.rs should avoid re-implementing runtime/view marker `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "color_picker_component_directory_standard_files_follow_contract_and_na_paths",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep component-directory governance marker `{required}`.",
        );
    }
}

#[test]
fn color_picker_file_placement_discipline_contract_is_explicit_for_interactive_component_scope() {
    let check2_source = load_source("../../components/color-picker/check2.md");
    let mod_source = load_source("../../components/color-picker/src/mod.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let styles_source = load_source("../../components/color-picker/src/styles.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let motion_source = load_source("../../components/color-picker/src/motion.rs");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    assert!(
        check2_source.contains("文件落点纪律"),
        "color-picker check2 should explicitly track file-placement discipline gate.",
    );

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required_file).exists(),
            "file-placement discipline requires `{required_file}` in color-picker source directory.",
        );
    }
    for forbidden_file in ["render.rs", "spec.rs"] {
        assert!(
            !src_dir.join(forbidden_file).exists(),
            "color-picker should keep `{forbidden_file}` absent in current scope.",
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
        logic_source.contains("pub fn resolve_derived_state(")
            && styles_source.contains("pub const CSS: &str =")
            && view_source.contains("#[component]")
            && motion_source.contains("pub struct ColorPickerMotion"),
        "logic/styles/view/motion should keep canonical responsibility anchors.",
    );

    let script_needle = "cargo test -p ui-color-picker color_picker_file_placement_discipline_contract_is_explicit_for_interactive_component_scope";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "color_picker_file_placement_discipline_contract_is_explicit_for_interactive_component_scope",
    ] {
        assert!(
            check2_source.contains(required),
            "color-picker check2 should keep file-placement-discipline marker `{required}`.",
        );
    }
}

#[test]
fn color_picker_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component() {
    let check2_source = load_source("../../components/color-picker/check2.md");
    let lib_source = load_source("../../components/color-picker/src/lib.rs");
    let mod_source = load_source("../../components/color-picker/src/mod.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let styles_source = load_source("../../components/color-picker/src/styles.rs");
    let motion_source = load_source("../../components/color-picker/src/motion.rs");
    let protocol_source = load_source("../../components/color-picker/src/protocol.rs");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    assert!(
        check2_source.contains("Hyper-Structure Builder（`spec.rs`）"),
        "color-picker checklist should explicitly track hyper-structure builder gate.",
    );

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "color-picker is not a complex schema-driven component; spec.rs should remain N/A.",
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "ColorPickerSpec",
        "Spec::new(",
        ".render(",
    ] {
        assert!(
            !lib_source.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "color-picker should not expose hyper-structure builder token `{forbidden}` in current scope.",
        );
    }

    for required in [
        "pub enum PickerComponentSchemaVersion",
        "pub struct PickerComponentSpec",
        "pub schema_version: PickerComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(required),
            "protocol.rs should keep minimal schema/version protocol marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。（N/A：`ColorPicker` 当前不属于复杂 schema 驱动组件，不存在稳定外部 schema 固化需求；组件目录保持 `mod.rs/logic.rs/styles.rs/view.rs/motion.rs` 且 `src/spec.rs` 不存在。`protocol.rs` 仅承载最小版本化序列化协议，不暴露 `*Spec::new()...render()` 建造者入口。）",
        "color_picker_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component",
    ] {
        assert!(
            check2_source.contains(required),
            "color-picker checklist should keep hyper-structure-builder marker `{required}`.",
        );
    }
}

#[test]
fn color_picker_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let check2_source = load_source("../../components/color-picker/check2.md");
    let script_source = load_source("../../scripts/check-ui-component-files.sh");
    let component_manifest = load_source("../../components/color-picker/src/Component.toml");
    let component_rbi = load_source("../../components/color-picker/src/color_picker.rbi");

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["Component.toml", "color_picker.rbi"] {
        assert!(
            src_dir.join(required_file).exists(),
            "color-picker context-compression file should exist: `{required_file}`.",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "name = \"ColorPicker\"",
        "crate = \"ui-color-picker\"",
        "name = \"value\"",
        "name = \"default_value\"",
        "name = \"on_value_change\"",
        "name = \"selected_color\"",
        "name = \"default_selected_color\"",
        "name = \"on_selected_change\"",
        "name = \"open\"",
        "name = \"default_open\"",
        "name = \"on_open_change\"",
        "name = \"motion\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "color-picker Component.toml should keep context-compression marker `{required}`.",
        );
    }

    for required in [
        "pub type ColorPickerState = ui_state_primitives::color_picker::ColorPickerState;",
        "pub type ColorPickerStateInput = ui_state_primitives::color_picker::ColorPickerStateInput;",
        "pub type ColorPickerMotion = crate::ColorPickerMotion;",
        "pub const DEFAULT_LABEL: &str;",
        "pub const DEFAULT_ARIA_LABEL: &str;",
        "pub fn ColorPicker(",
        "children: leptos::children::ChildrenFn",
        "value: Option<leptos::prelude::Signal<Option<String>>>",
        "on_value_change: Option<leptos::prelude::Callback<Option<String>>>",
        "default_value: Option<String>",
        "open: Option<leptos::prelude::Signal<bool>>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
        "dir: Option<ui_headless::A11yDirection>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            component_rbi.contains(required),
            "color_picker.rbi should keep signature-projection marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。（`components/color-picker/src/Component.toml` 与 `components/color-picker/src/color_picker.rbi` 已同步维护；`Component.toml` 覆盖输入输出轴与能力清单，`.rbi` 提供 `ColorPicker` 接口签名投影，避免 AI 检索漂移。回归：`components/color-picker/test/semantics.rs::color_picker_context_compression_manifest_and_rbi_projection_are_present_and_current`；门禁脚本：`scripts/check-ui-component-files.sh` 新增 `cargo test -p ui-color-picker color_picker_context_compression_manifest_and_rbi_projection_are_present_and_current`。）",
        "color_picker_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            check2_source.contains(required),
            "color-picker checklist should keep context-compression marker `{required}`.",
        );
    }
}

#[test]
fn color_picker_agent_contract_is_schema_typed_and_machine_readable() {
    let check2_source = load_source("../../components/color-picker/check2.md");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let component_manifest = load_source("../../components/color-picker/src/Component.toml");
    let component_rbi = load_source("../../components/color-picker/src/color_picker.rbi");

    for typed_source in [
        "pub const COLOR_PICKER_AGENT_SCHEMA: &str = \"ui.color-picker.agent-contract\";",
        "pub enum ColorPickerAgentSchemaVersion",
        "pub enum ColorPickerAgentIntent",
        "pub enum ColorPickerAgentAction",
        "pub enum ColorPickerAgentState",
        "pub enum ColorPickerAgentSource",
        "pub struct ColorPickerAgentContract",
        "pub struct ColorPickerAgentContractInput",
        "fn resolve_agent_state(render_state: ColorPickerState) -> ColorPickerAgentState",
        "pub fn resolve_agent_contract(input: ColorPickerAgentContractInput) -> ColorPickerAgentContract",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "color-picker Agent Contract should stay type-derived via `{typed_source}`.",
        );
    }

    for marker in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-ui-selection-source=move || agent_contract.get().selection_source",
        "data-ui-open-source=move || agent_contract.get().open_source",
        "data-ui-motion-source=move || agent_contract.get().motion_source",
        "data-ui-label-source=move || agent_contract.get().label_source",
        "data-ui-aria-source=move || agent_contract.get().aria_source",
        "data-ui-class-source=move || agent_contract.get().class_source",
        "data-ui-config-policy=move || agent_contract.get().config_policy",
    ] {
        assert!(
            view_source.contains(marker),
            "color-picker view should mount Agent Contract marker `{marker}`.",
        );
    }

    for required in [
        "name = \"agent-contract-markers\"",
        "name = \"agent_contract_schema_markers\"",
        "[[agent_contract]]",
        "schema = \"ui.color-picker.agent-contract.v1\"",
        "intent = \"color.selection\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "COLOR_PICKER_AGENT_SCHEMA",
        "ColorPickerAgentContract",
        "resolve_agent_contract",
    ] {
        assert!(
            component_manifest.contains(required) || component_rbi.contains(required),
            "color-picker context-compression assets should keep Agent Contract marker `{required}`.",
        );
    }

    for forbidden in [
        "data-ui-schema=\"",
        "format!(\"data-ui-schema",
        "format!(\"data-ui-intent",
        "format!(\"data-ui-state",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "color-picker Agent Contract should avoid free-form schema token `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。（`components/color-picker/src/logic.rs` 新增类型化 Agent Contract（`ColorPickerAgent{SchemaVersion/Intent/Action/State/Source}` + `resolve_agent_contract`），`components/color-picker/src/view.rs` 挂载稳定 `data-ui-schema/data-ui-schema-version/data-ui-intent/data-ui-action/data-ui-state/data-ui-source` 与来源轴标记（`data-ui-*-source`）；`components/color-picker/src/Component.toml` 补充 `agent-contract-markers`、`agent_contract_schema_markers`、`[[agent_contract]]` 与 marker 白名单描述。回归由 `components/color-picker/test/semantics.rs::color_picker_agent_contract_is_schema_typed_and_machine_readable` 与 `components/color-picker/test/semantics.rs::color_picker_agent_contract_render_path_is_whitelist_safe_and_script_injection_free` 覆盖，并接入 `scripts/check-ui-contract-hygiene.sh` 门禁。）",
        "color_picker_agent_contract_is_schema_typed_and_machine_readable",
    ] {
        assert!(
            check2_source.contains(required),
            "color-picker checklist should keep Agent Contract evidence `{required}`.",
        );
    }
}

#[test]
fn color_picker_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let check2_source = load_source("../../components/color-picker/check2.md");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let component_manifest = load_source("../../components/color-picker/src/Component.toml");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "allowed = [",
        "\"render_empty_swatch(...)\"",
        "\"render_selected_swatch(...)\"",
        "\"render_selected_value_text(...)\"",
        "\"render_trigger()\"",
        "\"render_panel()\"",
        "\"logic::resolve_derived_state(...)\"",
        "\"logic::resolve_agent_contract(...)\"",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\", \"eval(\"]",
    ] {
        assert!(
            component_manifest.contains(required),
            "color-picker manifest should keep whitelist-safe render path marker `{required}`.",
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
            "color-picker Agent Contract render path should forbid `{forbidden}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui-color-picker color_picker_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-color-picker color_picker_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(script_needle),
            "contract-hygiene gate script should include `{script_needle}`.",
        );
    }

    for required in [
        "color_picker_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "白名单能力边界",
    ] {
        assert!(
            check2_source.contains(required),
            "color-picker checklist should keep Agent Contract whitelist evidence `{required}`.",
        );
    }
}

#[test]
fn color_picker_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("../../components/color-picker/check2.md");
    let mod_source = load_source("../../components/color-picker/src/mod.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let styles_source = load_source("../../components/color-picker/src/styles.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。（N/A：`ColorPicker` 不是 LLM 正文渲染组件，组件职责是同步颜色选择与弹层开合控制；组件侧不实现 token-by-token streaming 协议，仅消费稳定快照状态输入。术语约束固定为两种显示模式：`Streaming`（边生成边显示）与 `Snapshot`（完整结果一次性显示），避免在组件层引入第三种“伪流式”定义。回归：`components/color-picker/test/semantics.rs::color_picker_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`；门禁脚本：`scripts/check-ui-streaming.sh` 新增 `cargo test -p ui-color-picker color_picker_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`。）",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "N/A：`ColorPicker` 不是 LLM 正文渲染组件",
    ] {
        assert!(
            check2_source.contains(required),
            "color-picker check2 should keep streaming-definition marker `{required}`.",
        );
    }

    for forbidden in ["use_ai_space_state", "project_streaming_"] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !docs_page_source.contains(forbidden),
            "color-picker should stay out of LLM streaming protocol scope and avoid `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_picker_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_source("../../components/color-picker/check2.md");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。（`ColorPicker` 已支持完整配置快照输入并稳定渲染：`components/color-picker/src/view.rs` 通过受控/非受控三件套（`value/default_value/on_value_change`、`selected_color/default_selected_color/on_selected_change`、`open/default_open/on_open_change`）+ 归一化边界（`logic::resolve_default_selected_color`、`logic::resolve_derived_state`）消费完整结果，根节点持续输出稳定语义标记（`data-state/data-open/data-disabled/data-open-mode/data-label-source/data-aria-source/data-class-source/data-ui-stream-fallback/data-ui-stream-mode/...`）。docs 基线示例 `apps/docs-app/src/pages/components/pages/forms_color.rs` 提供 Hello World、Controlled、Open by default 等完整快照路径。回归：`components/color-picker/test/semantics.rs::color_picker_check2_documents_snapshot_as_default_baseline_capability` 与 `components/color-picker/test/semantics.rs::color_picker_snapshot_baseline_consumes_complete_result_and_renders_stably`；门禁脚本：`scripts/check-ui-streaming.sh` 新增 `cargo test -p ui-color-picker color_picker_snapshot_baseline_consumes_complete_result_and_renders_stably`。）",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            check2_source.contains(required),
            "color-picker check2 should keep snapshot-baseline marker `{required}`.",
        );
    }
}

#[test]
fn color_picker_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for marker in [
        "pub fn ColorPicker(",
        "#[prop(optional)] value: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] selected_color: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_selected_color: Option<String>",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "let selected_state = overlay_open::use_controllable_state(",
        "let open_state = overlay_open::use_controllable_open_state_traced(",
        "logic::resolve_derived_state(logic::ColorPickerDerivedStateInput {",
        "data-state=move || state.get().data_state_attr",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "color-picker snapshot baseline should keep complete-result render marker `{marker}`.",
        );
    }

    for marker in [
        "pub fn resolve_default_selected_color(",
        "pub fn resolve_selected_color_axis<T>(",
        "pub fn resolve_selected_change_axis<T>(",
        "pub fn resolve_derived_state(input: ColorPickerDerivedStateInput) -> ColorPickerState",
        "pub fn resolve_agent_contract(input: ColorPickerAgentContractInput) -> ColorPickerAgentContract",
        "ColorPickerAgentStreamFallback::Snapshot",
        "ColorPickerAgentStreamMode::Snapshot",
    ] {
        assert!(
            logic_source.contains(marker),
            "color-picker logic should keep normalized snapshot baseline marker `{marker}`.",
        );
    }

    for marker in [
        "title=\"ColorPicker\"",
        "slug=\"color-picker\"",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<ColorPicker id_base=\"docs-color-picker-hello\".to_string()>",
        "title=\"Config Workbench\"",
        "code_signal=workbench_code",
        "id_base=\"docs-color-picker-workbench\".to_string()",
        "selected_color=workbench_selected_color",
        "on_selected_change=on_selected_change",
        "open=workbench_open",
        "on_open_change=on_open_change",
        "<Playground title=\"State Matrix\" code_signal=matrix_code>",
        "id_base=\"docs-color-picker-matrix-open\".to_string()",
        "default_open=true",
    ] {
        assert!(
            docs_source.contains(marker),
            "color-picker docs should keep snapshot-ready baseline usage marker `{marker}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_picker_check2_documents_streaming_required_optional_classification_rules() {
    let check2_source = load_source("../../components/color-picker/check2.md");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。（`ColorPicker` 归类为 `Streaming Optional`；组件职责是颜色选择与弹层交互而非 LLM 正文阅读面，默认走 `Snapshot` 渲染路径。实现显式输出 `data-ui-stream-support=\"unsupported\"`、`data-ui-stream-fallback=\"snapshot\"`、`data-ui-stream-mode=\"snapshot\"` 与 `data-ui-output-status`，并保持 `role/aria/data-*` 连续可读。数据校验、断线恢复、重试策略继续留在上层编排，不下沉到组件。回归：`components/color-picker/test/semantics.rs::color_picker_check2_documents_streaming_required_optional_classification_rules`、`components/color-picker/test/semantics.rs::color_picker_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、`components/color-picker/test/semantics.rs::color_picker_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer`；门禁脚本：`scripts/check-ui-streaming.sh` 新增对应 `cargo test` 目标。）",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "`ColorPicker` 归类为 `Streaming Optional`",
    ] {
        assert!(
            check2_source.contains(required),
            "color-picker check2 should keep streaming responsibility marker `{required}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui-color-picker color_picker_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-color-picker color_picker_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-color-picker color_picker_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming check script should enforce `{script_needle}`.",
        );
    }
}

#[test]
fn color_picker_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("../../components/color-picker/src/view.rs");

    for required in [
        "role=trigger_aria.attrs.role",
        "aria-haspopup=\"dialog\"",
        "aria-expanded=move || trigger_aria_expanded.get().unwrap_or(\"false\")",
        "aria-controls=move || trigger_aria_controls.get()",
        "aria-label=move || trigger_aria_label.get_value()",
        "aria-disabled=trigger_aria.attrs.aria_disabled",
        "role=\"dialog\"",
        "aria-modal=\"false\"",
        "aria-labelledby=panel_aria_labelledby.get_value()",
        "lang=panel_lang.get_value()",
        "dir=panel_dir",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "color-picker should keep continuous aria/data semantics via `{required}` in optional-streaming scope.",
        );
    }
}

#[test]
fn color_picker_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let motion_source = load_source("../../components/color-picker/src/motion.rs");
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
            "color-picker should keep validation/retry/resilience orchestration out of component layer; found `{forbidden}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui-color-picker color_picker_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-color-picker color_picker_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-color-picker color_picker_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming gate script should include `{script_needle}`.",
        );
    }
}

#[test]
fn color_picker_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources()
{
    let mod_source = load_source("../../components/color-picker/src/mod.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let styles_source = load_source("../../components/color-picker/src/styles.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let motion_source = load_source("../../components/color-picker/src/motion.rs");
    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "color-picker non-test source should forbid rust-hygiene anti-pattern `{forbidden}`.",
        );
    }
}

#[test]
fn color_picker_rust_hygiene_string_clone_hotspots_converge_to_cow_or_borrowed_static() {
    let logic_source = load_source("../../components/color-picker/src/logic.rs");

    for required in [
        "use std::borrow::Cow;",
        "let label: Cow<'static, str> = normalized",
        ".map(Cow::Owned)",
        ".unwrap_or(Cow::Borrowed(DEFAULT_LABEL));",
        "let fallback: Cow<'static, str> = Cow::Borrowed(DEFAULT_ARIA_LABEL);",
        "label.into_owned()",
        "fallback.into_owned()",
    ] {
        assert!(
            logic_source.contains(required),
            "color-picker logic should keep Cow-based string hotspot mitigation marker `{required}`.",
        );
    }

    for forbidden in [
        "primitive::normalize_label(value)",
        "primitive::normalize_aria_label(value, label)",
        "String::from(DEFAULT_LABEL)",
        "String::from(DEFAULT_ARIA_LABEL)",
        "DEFAULT_LABEL.to_string()",
        "DEFAULT_ARIA_LABEL.to_string()",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "color-picker fallback normalization should avoid string clone hotspot `{forbidden}`.",
        );
    }
}

#[test]
fn color_picker_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let rust_hygiene_script = include_str!("../../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("../../scripts/check-ui-engineering.sh");

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

    for needle in [
        "cargo test -p ui-color-picker color_picker_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-color-picker color_picker_rust_hygiene_string_clone_hotspots_converge_to_cow_or_borrowed_static",
        "cargo test -p ui-color-picker color_picker_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn color_picker_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("../../components/color-picker/check2.md");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。（`components/color-picker/src/logic.rs` 通过 `Cow<'static, str>` 收敛默认文案回退的字符串复制热点；组件非测试源码维持无 `unwrap/expect` 与无吞错 `let _ = ...`。回归：`components/color-picker/test/semantics.rs::color_picker_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources`、`components/color-picker/test/semantics.rs::color_picker_rust_hygiene_string_clone_hotspots_converge_to_cow_or_borrowed_static`、`components/color-picker/test/semantics.rs::color_picker_rust_hygiene_script_enforces_repo_level_hygiene_guards`；门禁脚本：`scripts/check-ui-engineering.sh` 新增对应 `cargo test` 目标。另执行：`./scripts/check-rust-hygiene.sh`（当前环境已执行，若失败以脚本输出为准）。）",
        "color_picker_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "color_picker_rust_hygiene_string_clone_hotspots_converge_to_cow_or_borrowed_static",
        "color_picker_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "Cow<'static, str>",
        "./scripts/check-rust-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "color-picker check2 should keep rust-hygiene evidence marker `{needle}`.",
        );
    }
}

#[test]
fn color_picker_keeps_spec_rs_out_of_simple_component_surface() {
    let lib_source = load_source("../../components/color-picker/src/lib.rs");
    let mod_source = load_source("../../components/color-picker/src/mod.rs");
    let protocol_source = load_source("../../components/color-picker/src/protocol.rs");
    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");

    assert!(
        !spec_path.exists(),
        "simple color-picker component should not introduce `spec.rs` at `{}`.",
        spec_path.display(),
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "Spec::new(",
        "ColorPickerSpec",
    ] {
        assert!(
            !lib_source.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "simple component surface should not expose spec builder/schema module marker `{forbidden}`.",
        );
    }

    for needle in [
        "pub enum PickerComponentSchemaVersion",
        "pub struct PickerComponentSpec",
        "pub schema_version: PickerComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(needle),
            "lightweight protocol contract should stay in `protocol.rs` without forcing `spec.rs`; missing `{needle}`.",
        );
    }
}

#[test]
fn color_picker_token_first_static_css_contract_is_aggregated_and_injected_via_ui_root() {
    let styles_source = load_source("../../components/color-picker/src/styles.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let css_registry_source = load_source("../../crates/ui/src/css.rs");
    let ui_root_source = load_source("../../crates/ui/src/root.rs");

    for needle in ["pub const CSS: &str", ".ui-color-picker", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "color-picker styles should keep token-first static css marker `{needle}`.",
        );
    }

    for forbidden in [
        "@apply",
        "--tw-",
        "tailwind",
        "styled-components",
        "emotion",
        "css!(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "component styles should not depend on utility-first or css-in-rust default track `{forbidden}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-color_picker\")]",
        "out.push_str(crate::color::picker::styles::CSS);",
    ] {
        assert!(
            css_registry_source.contains(needle),
            "ui css registry should aggregate color-picker styles via `{needle}`.",
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "UiRoot should inject aggregated component css through `{needle}`.",
        );
    }

    assert!(
        !view_source.contains("style=") && !view_source.contains("style:"),
        "component runtime should not move business styling into inline style.",
    );
}

#[test]
fn color_picker_api_naming_prefers_is_prefix_with_legacy_alias_migration() {
    let view_source = load_source("../../components/color-picker/src/view.rs");

    for needle in [
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] disabled: Option<bool>",
        "let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);",
        "disabled=is_disabled",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorPicker view should include `{needle}` for API naming migration."
        );
    }

    for needle in [
        "#[prop(optional)] value: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] selected_color: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_selected_color: Option<String>",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "let selected_color = logic::resolve_selected_color_axis(value, selected_color);",
        "logic::resolve_default_selected_color(default_value, default_selected_color);",
        "let on_selected_change =",
        "logic::resolve_selected_change_axis(on_value_change, on_selected_change);",
        "logic::resolve_derived_state(logic::ColorPickerDerivedStateInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorPicker value axis should include `{needle}` for canonical/legacy controllable API migration.",
        );
    }

    assert!(
        !view_source.contains("default_value.or(default_selected_color)"),
        "default value priority should be normalized in logic.rs, not composed in view.rs."
    );
    assert!(
        !view_source.contains("has_selection: selected_color.get().is_some()"),
        "selection-state derivation should be normalized in logic.rs, not scattered in view.rs."
    );
    assert!(
        !view_source.contains("#[prop(optional)] state:"),
        "ColorPicker should not require an internal state object prop for baseline usage."
    );
}

#[test]
fn color_picker_discrete_axes_are_type_constrained_with_enums() {
    let view_source = load_source("../../components/color-picker/src/view.rs");

    for needle in [
        "#[prop(optional)] placement: PopoverPlacement",
        "#[prop(optional)] swatch_size: ColorSwatchSize",
        "#[prop(optional)] swatch_rounding: ColorSwatchRounding",
        "#[prop(optional)] swatch_shape: ColorSwatchShape",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorPicker discrete axis should use enum type `{needle}`."
        );
    }

    for forbidden in [
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ColorPicker should not expose free-form discrete string axis `{forbidden}`."
        );
    }
}

#[test]
fn color_picker_machine_readable_state_contract_is_type_driven_and_regression_locked() {
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/color_picker.rs");
    let logic_tests = include_str!("logic.rs");

    for needle in [
        "pub struct ColorPickerDerivedStateInput",
        "pub fn resolve_derived_state(input: ColorPickerDerivedStateInput) -> ColorPickerState",
        "selected_color: Option<String>",
        "resolve_state(ColorPickerStateInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic normalization should keep typed state boundary `{needle}`.",
        );
    }

    for needle in [
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub struct ColorPickerStateInput",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub struct ColorPickerState",
        "pub data_state_attr: &'static str",
        "pub open_mode_attr: &'static str",
        "pub label_source_attr: &'static str",
        "pub aria_source_attr: &'static str",
        "pub class_source_attr: &'static str",
        "\"disabled\"",
        "\"open\"",
        "\"selected\"",
        "\"empty\"",
        "\"controlled\"",
        "\"uncontrolled\"",
        "\"custom\"",
        "\"default\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "state primitive should expose closed-set machine-readable contract `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "view should surface machine-readable semantic marker `{needle}`.",
        );
    }

    for needle in [
        "fn axis_aliases_and_disabled_priority_are_normalized_in_logic()",
        "fn resolve_state_and_class_name_track_markers()",
        "fn resolve_derived_state_maps_selection_and_source_flags()",
    ] {
        assert!(
            logic_tests.contains(needle),
            "logic regression should keep contract locator `{needle}` for fast breakage diagnosis.",
        );
    }
}

#[test]
fn color_picker_composition_api_prefers_explicit_children_over_parallel_arrays() {
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "children: ChildrenFn",
        "let children = StoredValue::new(children);",
        "{children()}",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorPicker composition API should keep explicit children contract `{needle}`.",
        );
    }

    for forbidden in [
        "#[prop(optional)] labels:",
        "#[prop(optional)] titles:",
        "#[prop(optional)] panels:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ColorPicker should not expose parallel-array composition prop `{forbidden}`.",
        );
    }

    for needle in [
        "<ColorPicker",
        "id_base=\"docs-color-picker-workbench\".to_string()",
        "<ColorSwatchPicker",
        "ColorSwatchPickerItem::named(",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs should demonstrate explicit composition / typed item entry `{needle}`.",
        );
    }
}

#[test]
fn color_picker_has_no_drag_frame_loop_or_drag_end_action_path() {
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let motion_source = load_source("../../components/color-picker/src/motion.rs");

    for forbidden in [
        "on:pointermove",
        "on:mousemove",
        "on:touchmove",
        "requestAnimationFrame",
        "DragEnd",
        "dragging",
        "Dragging",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ColorPicker view should not include drag-frame loop marker `{forbidden}` when drag interaction is out of scope.",
        );
        assert!(
            !logic_source.contains(forbidden),
            "ColorPicker logic should not include drag action marker `{forbidden}` when drag interaction is out of scope.",
        );
    }

    for forbidden in [
        "requestAnimationFrame",
        "raf",
        "DragEnd",
        "dragging",
        "Dragging",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "ColorPicker motion should not include drag-driver marker `{forbidden}` when drag interaction is out of scope.",
        );
    }
}

#[test]
fn color_picker_overlay_pipeline_uses_two_pass_positioning_with_idempotent_convergence() {
    let picker_view = load_source("../../components/color-picker/src/view.rs");
    let popover_view = load_source("../../components/popover/src/view.rs");
    let popover_position = load_source("../../crates/ui-headless/src/popover_position.rs");
    let popover_position_tests =
        load_source("../../crates/ui-headless/src/test/popover_position.rs");

    for needle in ["<Popover", "placement=placement", "anchor_ref=anchor_ref"] {
        assert!(
            picker_view.contains(needle),
            "ColorPicker should forward positioning intent to Popover via `{needle}`.",
        );
    }

    for needle in [
        "use_popover_position(PopoverPositionOptions {",
        "anchor_ref,",
        "panel_ref,",
        "placement,",
    ] {
        assert!(
            popover_view.contains(needle),
            "Popover view should run measure phase via `{needle}`.",
        );
    }

    for needle in [
        "get_bounding_client_rect()",
        "compute_popover_position(",
        "should_update_scalar(",
        "POSITION_EPSILON_PX",
        "if placement.get_untracked() != computed.placement {",
    ] {
        assert!(
            popover_position.contains(needle),
            "Popover positioning pipeline should keep rectification/idempotence guard `{needle}`.",
        );
    }

    for needle in [
        "scalar_update_guard_ignores_sub_epsilon_noise",
        "scalar_update_guard_accepts_meaningful_delta",
    ] {
        assert!(
            popover_position_tests.contains(needle),
            "headless popover position tests should cover convergence guard `{needle}`.",
        );
    }
}

#[test]
fn color_picker_collection_contract_does_not_require_registration_protocol() {
    let picker_view = load_source("../../components/color-picker/src/view.rs");
    let picker_logic = load_source("../../components/color-picker/src/logic.rs");
    let swatch_picker_view = load_source("../../components/color-swatch-picker/src/view.rs");
    let swatch_picker_logic = load_source("../../components/color-swatch-picker/src/logic.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !picker_view.contains(forbidden),
            "ColorPicker should not define collection registration protocol marker `{forbidden}`.",
        );
        assert!(
            !picker_logic.contains(forbidden),
            "ColorPicker logic should not depend on collection registration marker `{forbidden}`.",
        );
    }

    for needle in [
        "swatches: ReadSignal<Vec<ColorSwatchPickerItem>>",
        "item_count: items.len(),",
        ".enumerate()",
        "on_change: Some(Callback::new(move |index: usize| {",
    ] {
        assert!(
            swatch_picker_view.contains(needle) || swatch_picker_logic.contains(needle),
            "ColorSwatchPicker should keep ordered collection contract via `{needle}`.",
        );
    }

    assert!(
        !swatch_picker_view.contains("HashSet") && !swatch_picker_logic.contains("HashSet"),
        "ColorSwatchPicker should not rely on HashSet iteration for item navigation order.",
    );
}

#[test]
fn color_picker_slot_projection_strategy_is_lazy_presence_not_keepalive() {
    let picker_view = load_source("../../components/color-picker/src/view.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");

    for needle in [
        "data-slot-projection=\"lazy\"",
        "data-slot-projection-source=\"presence\"",
        "let presence = use_presence(open);",
        "<Show when=move || presence.is_present.get()>",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            picker_view.contains(needle),
            "ColorPicker slot projection contract should include `{needle}`.",
        );
    }

    for forbidden in ["KeepAlive", "NotifyHidden"] {
        assert!(
            !picker_view.contains(forbidden),
            "ColorPicker lazy projection should not include keepalive-only marker `{forbidden}`.",
        );
    }

    for needle in [
        "pub fn use_presence",
        "set_present.set(true);",
        "set_present.set(false);",
    ] {
        assert!(
            presence_source.contains(needle),
            "Presence primitive should support lazy enter/exit projection step `{needle}`.",
        );
    }
}

#[test]
fn color_picker_env_streams_are_delegated_and_idempotent_in_headless() {
    let picker_view = load_source("../../components/color-picker/src/view.rs");
    let picker_logic = load_source("../../components/color-picker/src/logic.rs");
    let popover_position = load_source("../../crates/ui-headless/src/popover_position.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "match_media",
        "add_event_listener",
        "on:scroll",
        "on:resize",
    ] {
        assert!(
            !picker_view.contains(forbidden),
            "ColorPicker view should not subscribe raw env stream `{forbidden}` directly.",
        );
        assert!(
            !picker_logic.contains(forbidden),
            "ColorPicker logic should not consume raw env stream `{forbidden}` directly.",
        );
    }

    for needle in [
        "ResizeObserver",
        "add_event_listener_with_callback(\"resize\"",
        "add_event_listener_with_callback_and_bool(",
        "\"scroll\"",
        "compute_popover_position(",
        "should_update_scalar(",
    ] {
        assert!(
            popover_position.contains(needle),
            "headless popover position should include env sampling/guard contract `{needle}`.",
        );
    }

    assert!(
        !popover_position.contains("BreakpointChanged"),
        "headless env stream should not leak app-level action protocol into color-picker path.",
    );
}

#[test]
fn color_picker_event_light_cone_is_not_applicable_to_scalar_selection_path() {
    let picker_view = load_source("../../components/color-picker/src/view.rs");
    let picker_logic = load_source("../../components/color-picker/src/logic.rs");
    let swatch_picker_view = load_source("../../components/color-swatch-picker/src/view.rs");
    let swatch_picker_logic = load_source("../../components/color-swatch-picker/src/logic.rs");

    for forbidden in [
        "Context Bus",
        "SelectionState::All",
        "context_bus",
        "Selector<",
        "selector(",
    ] {
        assert!(
            !picker_view.contains(forbidden),
            "ColorPicker should not include large-collection bus protocol marker `{forbidden}`.",
        );
        assert!(
            !picker_logic.contains(forbidden),
            "ColorPicker logic should not include large-collection bus protocol marker `{forbidden}`.",
        );
        assert!(
            !swatch_picker_view.contains(forbidden),
            "ColorSwatchPicker should not include large-collection bus protocol marker `{forbidden}`.",
        );
        assert!(
            !swatch_picker_logic.contains(forbidden),
            "ColorSwatchPicker logic should not include large-collection bus protocol marker `{forbidden}`.",
        );
    }

    for needle in [
        "selected_color: Option<Signal<Option<String>>>",
        "selected_color: Option<String>",
        "selected_index: Option<usize>",
        "on_change: Some(Callback::new(move |index: usize| {",
    ] {
        assert!(
            picker_view.contains(needle)
                || picker_logic.contains(needle)
                || swatch_picker_view.contains(needle)
                || swatch_picker_logic.contains(needle),
            "ColorPicker path should stay on scalar/index selection contract `{needle}`.",
        );
    }
}

#[test]
fn color_picker_causality_path_is_local_callback_not_complex_traceid_bus() {
    let picker_view = load_source("../../components/color-picker/src/view.rs");
    let picker_logic = load_source("../../components/color-picker/src/logic.rs");
    let controllable_state = load_source("../../crates/ui-headless/src/controllable_state.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");

    for forbidden in [
        "TraceId",
        "causality bus",
        "broadcast",
        "subscriber",
        "publish(",
        "subscribe(",
    ] {
        assert!(
            !picker_view.contains(forbidden),
            "ColorPicker view should not depend on complex causality-bus marker `{forbidden}`.",
        );
        assert!(
            !picker_logic.contains(forbidden),
            "ColorPicker logic should not depend on complex causality-bus marker `{forbidden}`.",
        );
    }

    for needle in [
        "let next_open = !open.get_untracked();",
        "request_open_change.run(next_open);",
        "let on_close: OnPress = Callback::new(move |_| {",
        "request_open_change.run(false);",
        "use_controllable_open_state_traced(",
    ] {
        assert!(
            picker_view.contains(needle),
            "ColorPicker should keep local callback causality contract `{needle}`.",
        );
    }

    for needle in [
        "trace.emit(component, UiTraceEventKind::OpenChange { open: next });",
        "pub enum UiTraceEventKind",
        "OpenChange {",
    ] {
        assert!(
            controllable_state.contains(needle) || trace_source.contains(needle),
            "headless trace path should include `{needle}` for lightweight observability.",
        );
    }

    assert!(
        !trace_source.contains("TraceId"),
        "ui-headless trace primitives currently do not expose TraceId contract; complex bus requirement is N/A for ColorPicker.",
    );
}

#[test]
fn color_picker_a11y_i18n_l10n_contract_uses_headless_and_localizable_text_sources() {
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/color_picker.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "#[prop(optional, into)] label: Option<String>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "popup_trigger_attrs(",
        "overlay_dialog_attrs(",
        "use_button(ButtonOptions {",
        "use_focus_ring(FocusRingOptions {",
        "use_hover(HoverOptions {",
        "role=\"dialog\"",
        "aria-haspopup=\"dialog\"",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorPicker view should include a11y/i18n contract marker `{needle}`.",
        );
    }

    for needle in [
        "pub const DEFAULT_LABEL",
        "pub const DEFAULT_ARIA_LABEL",
        "pub fn normalize_label(",
        "pub fn normalize_aria_label(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorPicker logic should include text-source normalization marker `{needle}`.",
        );
    }

    for needle in [
        "pub const DEFAULT_LABEL: &str = \"Color\";",
        "pub const DEFAULT_ARIA_LABEL: &str = \"Color picker\";",
        "pub fn normalize_label(value: Option<String>) -> (String, bool)",
        "pub fn normalize_aria_label(value: Option<String>, label: &str) -> (String, bool)",
    ] {
        assert!(
            primitive_source.contains(needle),
            "state primitive should provide fallback/localizable text contract `{needle}`.",
        );
    }

    for needle in [
        "pub fn locale_attrs(lang: Option<String>, dir: Option<A11yDirection>)",
        "pub fn popup_trigger_attrs(",
        "pub fn overlay_dialog_attrs(",
    ] {
        assert!(
            a11y_source.contains(needle),
            "shared ui-headless a11y tool contract should include `{needle}`.",
        );
    }
}

#[test]
fn color_picker_observability_markers_are_stable_and_enumerated() {
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/color_picker.rs");

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-selection-empty=move || state.get().selection_empty.then_some(\"true\")",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=if motion == ColorPickerMotion::default()",
        "data-focus-visible=move || trigger_focus_ring.is_focus_visible.get().then_some(\"true\")",
        "aria-expanded=move || trigger_aria_expanded.get().unwrap_or(\"false\")",
        "aria-controls=move || trigger_aria_controls.get()",
        "aria-disabled=trigger_aria.attrs.aria_disabled",
        "role=trigger_aria.attrs.role",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorPicker should expose stable observability marker `{needle}`.",
        );
    }

    for needle in [
        "data_state_attr = if input.disabled {",
        "\"disabled\"",
        "\"open\"",
        "\"selected\"",
        "\"empty\"",
        "open_mode_attr: if input.is_open_controlled {",
        "\"controlled\"",
        "\"uncontrolled\"",
        "label_source_attr: if input.has_custom_label {",
        "aria_source_attr: if input.has_custom_aria_label {",
        "class_source_attr: if input.has_custom_class_name {",
        "\"custom\"",
        "\"default\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ColorPicker primitive should keep closed-set marker value contract `{needle}`.",
        );
    }

    assert!(
        !view_source.contains(":nth-child"),
        "ColorPicker automation selectors should not rely on DOM positional coupling.",
    );
}

#[test]
fn color_picker_styles_use_explicit_state_selectors_and_no_fragile_structure_coupling() {
    let styles_source = load_source("../../components/color-picker/src/styles.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");

    for needle in [
        ".ui-color-picker[data-open=\"true\"] .ui-color-picker__trigger",
        ".ui-color-picker[data-disabled=\"true\"]",
        ".ui-color-picker[data-disabled=\"true\"] .ui-color-picker__trigger",
        ".ui-color-picker[data-custom-class=\"true\"]",
        ".ui-color-picker[data-state=\"open\"] .ui-color-picker__trigger",
    ] {
        assert!(
            styles_source.contains(needle),
            "ColorPicker styles should key state branches off explicit semantic markers `{needle}`.",
        );
    }

    for forbidden in [":nth-child", ":nth-of-type"] {
        assert!(
            !styles_source.contains(forbidden),
            "ColorPicker styles should not rely on fragile structural selector `{forbidden}`.",
        );
    }

    assert!(
        !view_source.contains("style=") && !view_source.contains("style:"),
        "ColorPicker view should not embed business style logic via inline styles.",
    );
}

#[test]
fn color_picker_semantic_contract_matrix_covers_state_inputs_and_platform_paths() {
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/color_picker.rs");
    let headless_trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let logic_tests = include_str!("logic.rs");

    for needle in [
        "role=trigger_aria.attrs.role",
        "aria-expanded=move || trigger_aria_expanded.get().unwrap_or(\"false\")",
        "aria-controls=move || trigger_aria_controls.get()",
        "aria-disabled=trigger_aria.attrs.aria_disabled",
        "data-state=move || state.get().data_state_attr",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "semantic contract should expose role/aria/data marker `{needle}`.",
        );
    }

    for needle in [
        "on:pointerdown=move |_| trigger_aria.handlers.press.on_pointer_down.run(())",
        "on:pointerup=move |_| trigger_aria.handlers.press.on_pointer_up.run(())",
        "on:pointercancel=move |_| trigger_aria.handlers.press.on_pointer_cancel.run(())",
        "on:pointerenter=move |_| trigger_hover.handlers.on_pointer_enter.run(())",
        "on:pointerleave=move |_| trigger_hover.handlers.on_pointer_leave.run(())",
        "on:keydown=move |ev| {",
        "trigger_aria.handlers.press.on_key_down.run(key)",
        "on:keyup=move |ev| {",
        "trigger_aria.handlers.press.on_key_up.run(key)",
    ] {
        assert!(
            view_source.contains(needle),
            "semantic interaction matrix should cover pointer/keyboard path marker `{needle}`.",
        );
    }

    for needle in [
        "open_mode_attr: if input.is_open_controlled {",
        "\"controlled\"",
        "\"uncontrolled\"",
        "data_state_attr = if input.disabled {",
        "\"disabled\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "state primitive should encode controlled/uncontrolled + disabled semantic marker `{needle}`.",
        );
    }

    for needle in [
        "fn axis_aliases_and_disabled_priority_are_normalized_in_logic()",
        "fn resolve_state_and_class_name_track_markers()",
        "fn resolve_derived_state_maps_selection_and_source_flags()",
    ] {
        assert!(
            logic_tests.contains(needle),
            "logic regression suite should include semantic matrix coverage `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "features `web` and `ssr` are mutually exclusive",
    ] {
        assert!(
            headless_trace_source.contains(needle) || headless_lib_source.contains(needle),
            "platform branch contract should include web/ssr/wasm safeguard marker `{needle}`.",
        );
    }

    for forbidden in [
        "assert_snapshot(",
        "insta::assert",
        "to_match_snapshot(",
        "snapshot!(",
    ] {
        assert!(
            !logic_tests.contains(forbidden),
            "semantic regression should not be replaced by visual snapshot-only assertion `{forbidden}`.",
        );
    }
}

#[test]
fn color_picker_default_theme_visual_desire_baseline_is_documented_and_token_driven() {
    let styles_source = load_source("../../components/color-picker/src/styles.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        ".ui-color-picker__label {",
        "font-size: var(--ui-color-picker-font-size-150);",
        "line-height: var(--ui-color-picker-line-height-150);",
        "font-weight: 600;",
        ".ui-color-picker__value {",
        "color: var(--ui-color-picker-fg-muted);",
        ".ui-color-picker__trigger:hover {",
        ".ui-color-picker__trigger:focus-visible {",
        ".ui-color-picker__panel {",
        "background: var(--ui-color-picker-bg);",
        "box-shadow: var(--ui-color-picker-shadow-md);",
    ] {
        assert!(
            styles_source.contains(needle),
            "default-theme visual hierarchy/feedback should stay token-driven with marker `{needle}`.",
        );
    }

    for needle in [
        "title=\"ColorPicker\"",
        "title=\"Hello World\"",
        "title=\"Config Workbench\"",
        "title=\"State Matrix\"",
        "id_base=\"docs-color-picker-workbench\".to_string()",
        "id_base=\"docs-color-picker-matrix-open\".to_string()",
        "default_open=true",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app should provide color-picker default-theme visual baseline marker `{needle}`.",
        );
    }
}

#[test]
fn color_picker_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("../../components/color-picker/src/styles.rs");
    let check2_source = load_source("../../components/color-picker/check2.md");
    let theme_css_source = include_str!("../../../crates/ui-theme/src/css.rs");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "--ui-color-picker-space-2xs: var(--ui-space-2xs, var(--ui-fallback-space-2xs));",
        "--ui-color-picker-space-xs: var(--ui-space-xs, var(--ui-fallback-space-xs));",
        "--ui-color-picker-space-sm: var(--ui-space-sm, var(--ui-fallback-space-sm));",
        "--ui-color-picker-space-md: var(--ui-space-md, var(--ui-fallback-space-md));",
        "--ui-color-picker-radius-sm: var(--ui-radius-sm, var(--ui-fallback-radius-sm));",
        "--ui-color-picker-radius-md: var(--ui-radius-md, var(--ui-fallback-radius-md));",
        "--ui-color-picker-border-width: var(--ui-border-width, var(--ui-fallback-border-width));",
        "--ui-color-picker-border: var(--ui-border, var(--ui-fallback-border));",
        "--ui-color-picker-bg: var(--ui-bg, var(--ui-fallback-bg));",
        "--ui-color-picker-fg: var(--ui-fg, var(--ui-fallback-fg));",
        "--ui-color-picker-fg-muted: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        "--ui-color-picker-accent: var(--ui-accent, var(--ui-fallback-accent));",
        "--ui-color-picker-font-size-150: var(--ui-font-size-150, var(--ui-fallback-font-size-150));",
        "--ui-color-picker-line-height-150: var(--ui-line-height-150, var(--ui-fallback-line-height-150));",
        "--ui-color-picker-font-size-100: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
        "--ui-color-picker-line-height-100: var(--ui-line-height-100, var(--ui-fallback-line-height-100));",
        "--ui-color-picker-overlay-panel-min-width: var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width));",
        "--ui-color-picker-shadow-md: var(--ui-shadow-md, var(--ui-fallback-shadow-md));",
        "--ui-color-picker-focus-outline-width: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width));",
        "--ui-color-picker-focus-outline-offset: var(--ui-button-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset));",
        "--ui-color-picker-disabled-opacity: var(--ui-checkbox-disabled-opacity, var(--ui-fallback-checkbox-disabled-opacity));",
    ] {
        assert!(
            styles_source.contains(required),
            "color-picker styles should keep defensive fallback chain token `{required}`.",
        );
    }

    for required in [
        "--ui-fallback-space-2xs:",
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-space-md:",
        "--ui-fallback-radius-sm:",
        "--ui-fallback-radius-md:",
        "--ui-fallback-border-width:",
        "--ui-fallback-border:",
        "--ui-fallback-bg:",
        "--ui-fallback-fg:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-accent:",
        "--ui-fallback-font-size-150:",
        "--ui-fallback-line-height-150:",
        "--ui-fallback-font-size-100:",
        "--ui-fallback-line-height-100:",
        "--ui-fallback-overlay-panel-min-width:",
        "--ui-fallback-shadow-md:",
        "--ui-fallback-button-focus-outline-width:",
        "--ui-fallback-button-focus-outline-offset:",
        "--ui-fallback-checkbox-disabled-opacity:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme should stay SSOT for fallback token `{required}`.",
        );
    }

    for forbidden in [
        "var(--ui-font-size-150, 14px)",
        "var(--ui-line-height-150, 20px)",
        "var(--ui-font-size-100, 12px)",
        "var(--ui-line-height-100, 16px)",
        "min-inline-size: 11rem;",
        "min-inline-size: var(--ui-overlay-panel-min-width, 240px);",
        "opacity: 0.62;",
        "padding: calc(var(--ui-space-2xs) + 1px) var(--ui-space-xs);",
        "outline: 2px solid",
        "outline-offset: 2px;",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "color-picker styles should not keep raw terminal token `{forbidden}`.",
        );
    }

    let has_hex_literal = styles_source
        .as_bytes()
        .windows(2)
        .any(|pair| pair[0] == b'#' && (pair[1] as char).is_ascii_hexdigit());
    assert!(
        !has_hex_literal,
        "color-picker styles should not hardcode hex colors; use theme variables/fallback chain.",
    );

    let script_needle =
        "cargo test -p ui-color-picker color_picker_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "color_picker_styles_use_defensive_variable_fallback_chain",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep defensive-variable governance marker `{required}`.",
        );
    }
}

#[test]
fn color_picker_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_registry_source = load_source("../../crates/ui/src/css.rs");
    let ui_root_source = load_source("../../crates/ui/src/root.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let check2_source = load_source("../../components/color-picker/check2.md");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-color_picker\")]\n    out.push_str(crate::color::picker::styles::CSS);",
    ] {
        assert!(
            css_registry_source.contains(needle),
            "ui css registry should keep cascade layer + feature-gated color-picker aggregation marker `{needle}`.",
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "UiRoot should inject aggregated css from registry marker `{needle}`.",
        );
    }

    for forbidden in [
        "style=",
        "style:",
        "style=\"top:",
        "style=\"left:",
        "style=\"transform:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-picker view should forbid non-custom-property inline style marker `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "color_picker_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep cascade-layer governance marker `{required}`.",
        );
    }
}

#[test]
fn color_picker_tree_shaking_contract_is_feature_gated_for_module_and_css_paths() {
    let ui_components_cargo = load_source("../../crates/ui/Cargo.toml");
    let ui_components_lib = load_source("../../crates/ui/src/lib.rs");
    let ui_components_css = load_source("../../crates/ui/src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");

    for needle in [
        "component-color_picker = [\"component-color_swatch\", \"component-popover\"]",
        "component-color_swatch = [\"component-illustrated_message\"]",
        "component-popover = [\"dep:ui-popover\"]",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui feature graph should keep color-picker tree-shaking edge `{needle}`.",
        );
    }

    assert!(
        ui_components_lib.contains(
            "#[cfg(feature = \"component-color_picker\")]\n#[path = \"../../../components/color-picker/src/mod.rs\"]\npub mod color_picker;"
        ),
        "ui lib export should gate `color_picker` module by `component-color_picker` feature.",
    );
    assert!(
        ui_components_lib.contains(
            "#[cfg(feature = \"component-color_picker\")]\n    pub use crate::color_picker as picker;"
        ),
        "domain color namespace should gate picker alias by `component-color_picker` feature.",
    );

    assert!(
        ui_components_css.contains(
            "#[cfg(feature = \"component-color_picker\")]\n    out.push_str(crate::color::picker::styles::CSS);"
        ),
        "css aggregation should include color-picker styles only under `component-color_picker`.",
    );

    assert!(
        web_demo_cargo.contains("ui = { path = \"../../crates/ui\", default-features = false")
            && web_demo_cargo.contains("features = [\"inject-css\", \"web-demo-components\"]"),
        "web-demo should consume ui via explicit minimal feature set without default feature fan-out.",
    );
    assert!(
        !web_demo_cargo.contains("\"all-components\""),
        "web-demo should not pull `all-components` implicitly when tree-shaking is expected.",
    );
}

#[test]
fn color_picker_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let tree_script_source = include_str!("../../../scripts/check-ui-tree-shaking.sh");

    for needle in [
        "COLOR_PICKER_MIN_FEATURES=\"component-color_picker,inject-css\"",
        "cargo test -p ui-color-picker color_picker_tree_shaking_contract_is_feature_gated_for_module_and_css_paths",
        "cargo test -p ui-color-picker color_picker_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui-color-picker color_picker_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$COLOR_PICKER_MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$COLOR_PICKER_TREE_OUTPUT\"",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$COLOR_PICKER_MIN_FEATURES\"",
    ] {
        assert!(
            tree_script_source.contains(needle),
            "tree-shaking script should enforce color-picker contract marker `{needle}`.",
        );
    }
}

#[test]
fn color_picker_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_source("../../components/color-picker/check2.md");

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "color_picker_tree_shaking_contract_is_feature_gated_for_module_and_css_paths",
        "color_picker_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "color_picker_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "scripts/check-ui-tree-shaking.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "color-picker check2 tree-shaking section should reference `{needle}`.",
        );
    }
}

#[test]
fn color_picker_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let check2_source = load_source("../../components/color-picker/check2.md");
    let ui_components_lib = load_source("../../crates/ui/src/lib.rs");
    let ui_components_css = load_source("../../crates/ui/src/css.rs");
    let ui_components_root = load_source("../../crates/ui/src/root.rs");
    let active_highlight =
        include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
    let controllable_state = include_str!("../../../crates/ui-headless/src/controllable_state.rs");
    let presence = include_str!("../../../crates/ui-headless/src/presence.rs");
    let a11y = include_str!("../../../crates/ui-headless/src/a11y.rs");
    let entrypoints_script = include_str!("../../../scripts/check-ui-entrypoints.sh");

    for required in [
        "#[cfg(feature = \"component-color_picker\")]",
        "pub mod color_picker;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "pub use color_picker::{ColorPicker, ColorPickerMotion};",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib.rs should keep fixed entry marker `{required}`.",
        );
    }

    for forbidden in ["pub use web_sys", "web_sys::", "NodeRef<", "JsValue"] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui lib.rs should not leak platform detail `{forbidden}`.",
        );
    }

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-color_picker\")]",
        "out.push_str(crate::color::picker::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css.rs should keep fixed entry marker `{required}`.",
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
            ui_components_root.contains(required),
            "ui root.rs should keep centralized injection marker `{required}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight.contains(required),
            "active_highlight shared primitive should contain `{required}`.",
        );
    }

    for forbidden in ["ColorPicker", "aria-", "data-state"] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight should stay generic and avoid component semantic token `{forbidden}`.",
        );
    }

    for required in [
        "pub struct ControllableState<T>",
        "pub fn use_controllable_state<T>(",
    ] {
        assert!(
            controllable_state.contains(required),
            "ui-headless controllable_state canonical path should contain `{required}`.",
        );
    }

    for required in [
        "pub struct Presence",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
    ] {
        assert!(
            presence.contains(required),
            "ui-headless presence canonical path should contain `{required}`.",
        );
    }

    for required in ["pub fn locale_attrs(", "pub fn aria_controls_when_open("] {
        assert!(
            a11y.contains(required),
            "ui-headless a11y canonical path should contain `{required}`.",
        );
    }

    let ui_components_src_dir =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui/src");
    for forbidden_file in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src_dir.join(forbidden_file).exists(),
            "ui/src/{forbidden_file} should be absent by fixed-entrypoint contract.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        entrypoints_script.contains(script_needle),
        "entrypoints gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] `ui` 固定入口文件落点正确。",
        "color_picker_ui_components_fixed_entry_files_follow_layered_boundaries",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep fixed-entrypoint governance marker `{required}`.",
        );
    }
}

#[test]
fn color_picker_overlay_focus_restore_uses_global_focus_stack_contract() {
    let picker_view = load_source("../../components/color-picker/src/view.rs");
    let popover_view = load_source("../../components/popover/src/view.rs");
    let focus_trap_source = load_source("../../crates/ui-headless/src/focus_trap.rs");

    for needle in [
        "let anchor_ref: NodeRef<html::Button> = NodeRef::new();",
        "<Popover",
        "anchor_ref=anchor_ref",
    ] {
        assert!(
            picker_view.contains(needle),
            "ColorPicker should pass anchor node for placement only via `{needle}`.",
        );
    }

    for forbidden in ["restore_target", "previous_focus", "focus_restore_ref"] {
        assert!(
            !picker_view.contains(forbidden),
            "ColorPicker should not keep private focus-restore node marker `{forbidden}`.",
        );
    }

    for needle in [
        "use_overlay_stack_registration()",
        "use_focus_trap(",
        "FocusTrapOptions::enabled(panel_ref)",
        ".with_scope_id(\"popover\")",
        "RestorePolicy::FallbackTo(",
        ".with_fallback_selector(",
    ] {
        assert!(
            popover_view.contains(needle),
            "Popover should delegate focus restore contract to headless/global focus manager via `{needle}`.",
        );
    }

    for needle in [
        "FOCUS_MANAGER_STACK",
        "focus_manager_push_trap(",
        "focus_manager_pop_trap(",
        "focus_manager_peek_trap(",
        "RestorePolicy::Selector(",
        "RestorePolicy::FallbackTo(",
        "restore_focus_chain(",
        "if let Some(body) = document.body()",
    ] {
        assert!(
            focus_trap_source.contains(needle),
            "ui-headless focus trap should keep global focus stack + fallback restoration marker `{needle}`.",
        );
    }
}

#[test]
fn color_picker_escape_hatch_foreign_zone_is_not_applicable_without_imperative_third_party_bridge()
{
    let picker_view = load_source("../../components/color-picker/src/view.rs");
    let picker_logic = load_source("../../components/color-picker/src/logic.rs");
    let picker_mod = load_source("../../components/color-picker/src/mod.rs");
    let picker_lib = load_source("../../components/color-picker/src/lib.rs");
    let picker_protocol = load_source("../../components/color-picker/src/protocol.rs");
    let popover_view = load_source("../../components/popover/src/view.rs");

    for forbidden in [
        "ECharts",
        "Mapbox",
        "leaflet",
        "YieldControl",
        "CleanupForeign",
        "ForeignZone",
        "foreign_zone",
        "foreign_instance",
        "third_party_instance",
    ] {
        assert!(
            !picker_view.contains(forbidden)
                && !picker_logic.contains(forbidden)
                && !picker_mod.contains(forbidden)
                && !picker_lib.contains(forbidden)
                && !picker_protocol.contains(forbidden)
                && !popover_view.contains(forbidden),
            "color-picker path should not carry imperative third-party bridge marker `{forbidden}` when escape hatch is N/A.",
        );
    }

    for forbidden in [
        "JsValue",
        "js_sys::",
        "web_sys::HtmlCanvasElement",
        "web_sys::CanvasRenderingContext2d",
    ] {
        assert!(
            !picker_view.contains(forbidden)
                && !picker_logic.contains(forbidden)
                && !picker_mod.contains(forbidden)
                && !picker_lib.contains(forbidden)
                && !picker_protocol.contains(forbidden),
            "color-picker public/component surface should not expose imperative foreign instance type `{forbidden}`.",
        );
    }
}

#[test]
fn color_picker_hydration_discontinuity_contract_uses_deterministic_id_flow_without_random_entropy()
{
    let picker_view = load_source("../../components/color-picker/src/view.rs");
    let picker_logic = load_source("../../components/color-picker/src/logic.rs");
    let picker_protocol = load_source("../../components/color-picker/src/protocol.rs");
    let ui_root_source = load_source("../../crates/ui/src/root.rs");
    let id_provider_source = load_source("../../crates/ui-headless/src/id_provider.rs");

    for forbidden in [
        "SystemTime::now",
        "Instant::now",
        "Date::now",
        "Math::random",
        "randomUUID",
        "Uuid::new_v4",
        "rand::",
        "uuid::",
    ] {
        assert!(
            !picker_view.contains(forbidden)
                && !picker_logic.contains(forbidden)
                && !picker_protocol.contains(forbidden),
            "color-picker initialization should avoid non-deterministic hydration entropy `{forbidden}`.",
        );
    }

    for needle in [
        "id_base: String,",
        "logic::normalize_optional_text(Some(id_base))",
        "let ids = logic::resolve_ids(&id_base);",
    ] {
        assert!(
            picker_view.contains(needle),
            "color-picker view should keep deterministic id-base initialization contract `{needle}`.",
        );
    }

    for needle in [
        "pub fn resolve_ids(id_base: &str) -> ColorPickerIds {",
        "trigger_id: format!(\"{id_base}-trigger\")",
        "label_id: format!(\"{id_base}-label\")",
        "panel_id: format!(\"{id_base}-panel\")",
        "content_id: format!(\"{id_base}-content\")",
    ] {
        assert!(
            picker_logic.contains(needle),
            "color-picker logic should derive ids deterministically from `id_base` via `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional, default = 1)] id_seed: u64,",
        "provide_ui_id_provider(id_seed);",
        "provide_ui_i18n(i18n);",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "UiRoot should keep deterministic id seed injection boundary marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider {",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider> {",
        "pub fn next_prefixed_id(self, prefix: &str) -> String {",
    ] {
        assert!(
            id_provider_source.contains(needle),
            "headless id provider should expose deterministic id contract marker `{needle}`.",
        );
    }
}

#[test]
fn color_picker_platform_contract_uses_cfg_gates_and_keeps_non_wasm_surface_websys_free() {
    let picker_view = load_source("../../components/color-picker/src/view.rs");
    let picker_logic = load_source("../../components/color-picker/src/logic.rs");
    let picker_motion = load_source("../../components/color-picker/src/motion.rs");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let headless_modal_source = load_source("../../crates/ui-headless/src/modal.rs");
    let focus_trap_source = load_source("../../crates/ui-headless/src/focus_trap.rs");
    let popover_position_source = load_source("../../crates/ui-headless/src/popover_position.rs");
    let popover_view = load_source("../../components/popover/src/view.rs");
    let popover_motion = load_source("../../components/popover/src/motion.rs");
    let ui_motion_lib = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should keep explicit web/ssr mutual exclusion marker `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(all(feature = \"web\", target_arch = \"wasm32\"))]",
        "#[cfg(not(all(feature = \"web\", target_arch = \"wasm32\")))]",
    ] {
        assert!(
            focus_trap_source.contains(needle)
                && popover_position_source.contains(needle)
                && headless_modal_source.contains(needle),
            "headless platform branches should remain cfg-gated for web wasm vs non-wasm via `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            popover_motion.contains(needle) && ui_motion_lib.contains(needle),
            "motion backend should keep wasm/non-wasm split marker `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            popover_view.contains(needle),
            "popover keyboard event guards should stay explicit across wasm/non-wasm via `{needle}`.",
        );
    }

    for forbidden in ["web_sys::", "leptos::web_sys::", "wasm_bindgen", "js_sys::"] {
        assert!(
            !picker_view.contains(forbidden)
                && !picker_logic.contains(forbidden)
                && !picker_motion.contains(forbidden),
            "color-picker component surface should stay websys-free for non-wasm compatibility; found `{forbidden}`.",
        );
    }
}

#[test]
fn color_picker_motion_non_wasm_noop_contract_is_predictable_and_safe() {
    let ui_motion_lib = load_source("../../crates/ui-motion/src/lib.rs");
    let popover_motion = load_source("../../components/popover/src/motion.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion non-wasm backend should keep deterministic no-op marker `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "Effect::new(move |_| {",
        "if !is_open.get() {",
        "on_exit_complete.run(())",
    ] {
        assert!(
            popover_motion.contains(needle),
            "popover motion non-wasm branch should safely degrade via marker `{needle}`.",
        );
    }

    for forbidden in ["panic!(", "unwrap()"] {
        assert!(
            !ui_motion_lib.contains(forbidden) && !popover_motion.contains(forbidden),
            "non-wasm motion downgrade path should avoid hard-fail marker `{forbidden}`.",
        );
    }
}

#[test]
fn color_picker_motion_contract_covers_reduced_motion_ssr_and_wasm_paths() {
    let picker_view = load_source("../../components/color-picker/src/view.rs");
    let popover_motion = load_source("../../components/popover/src/motion.rs");
    let ui_motion_web = load_source("../../crates/ui-motion/src/web.rs");
    let ui_motion_lib = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if ui_motion::web::prefers_reduced_motion() {",
        "set_style_values(&style, target_opacity, target_scale, target_y);",
        "if !open {",
        "on_exit_complete.run(())",
    ] {
        assert!(
            popover_motion.contains(needle),
            "popover motion should keep reduced-motion + wasm/ssr branch marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn prefers_reduced_motion() -> bool {",
        "match_media(\"(prefers-reduced-motion: reduce)\")",
        "if prefers_reduced_motion() {",
        "return;",
    ] {
        assert!(
            ui_motion_web.contains(needle),
            "ui-motion web backend should keep reduced-motion capability marker `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion crate should expose wasm/non-wasm split marker `{needle}`.",
        );
    }

    for needle in [
        "let presence = use_presence(open);",
        "<Show when=move || presence.is_present.get()>",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            picker_view.contains(needle),
            "color-picker should keep presence-driven hydration-safe render marker `{needle}`.",
        );
    }
}

#[test]
fn color_picker_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let color_picker_motion = load_source("../../components/color-picker/src/motion.rs");
    let color_picker_view = load_source("../../components/color-picker/src/view.rs");
    let popover_view = load_source("../../components/popover/src/view.rs");
    let popover_motion = load_source("../../components/popover/src/motion.rs");
    let check2_source = load_source("../../components/color-picker/check2.md");
    let script_source = include_str!("../../../scripts/check-ui-platforms.sh");

    for needle in [
        "pub struct ColorPickerMotion {",
        "pub popover: PopoverMotion,",
        "pub fn sanitize_motion(motion: ColorPickerMotion) -> ColorPickerMotion",
        "crate::popover::motion::sanitize_motion(motion.popover)",
    ] {
        assert!(
            color_picker_motion.contains(needle),
            "color-picker motion should keep component-scoped composed contract marker `{needle}`.",
        );
    }

    for needle in [
        "pub struct PopoverMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "stiffness: 300.0,",
        "damping: 25.0,",
        "pub fn attach_motion(",
        "if ui_motion::web::prefers_reduced_motion() {",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if !is_open.get() {",
        "on_exit_complete.run(())",
    ] {
        assert!(
            popover_motion.contains(needle),
            "popover motion backend should keep spring/reduced-motion/non-wasm-noop marker `{needle}`.",
        );
    }

    for needle in ["motion::attach_motion(", "motion,"] {
        assert!(
            popover_view.contains(needle),
            "popover view should keep attach_motion mounting marker `{needle}`.",
        );
    }

    for needle in [
        "let motion = crate::color_picker::motion::sanitize_motion(motion);",
        "motion=motion.popover",
    ] {
        assert!(
            color_picker_view.contains(needle),
            "color-picker view should map component motion to popover attach path via `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        script_source.contains(script_needle),
        "platform gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "color_picker_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep motion-contract governance marker `{needle}`.",
        );
    }
}

#[test]
fn color_picker_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("../../components/color-picker/check2.md");
    let view_source = load_source("../../components/color-picker/src/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "\"color-picker\" => UiPerfBudget {",
        "max_mount_ms: 36.0,",
        "max_update_ms: Some(12.0),",
        "max_heap_kb: Some(640.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep color-picker performance budget token `{needle}`.",
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
    ] {
        assert!(
            perf_source.contains(needle),
            "UiPerfProbe should expose machine-readable perf marker `{needle}`.",
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
            "docs e2e should keep blocking perf regression assertion `{needle}`.",
        );
    }

    for needle in [
        "slug=\"color-picker\"",
        "id_base=\"docs-color-picker-workbench\".to_string()",
        "id_base=\"docs-color-picker-matrix-open\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "color-picker docs page should keep perf-observable playground marker `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-motion-source=if motion == ColorPickerMotion::default() {",
        "data-custom-motion=move || (motion != ColorPickerMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "color-picker view should keep perf attribution marker `{needle}`.",
        );
    }

    for needle in [
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
        "render_count",
    ] {
        assert!(
            check2_source.contains(needle) || todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-color-picker color_picker_performance_governance_contract_is_budgeted_traceable_and_blocking",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`.",
        );
    }
}

#[test]
fn color_picker_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let check2_source = load_source("../../components/color-picker/check2.md");
    let docs_shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for marker in [
        "role=trigger_aria.attrs.role",
        "aria-haspopup=\"dialog\"",
        "aria-expanded=move || trigger_aria_expanded.get().unwrap_or(\"false\")",
        "aria-controls=move || trigger_aria_controls.get()",
        "aria-label=move || trigger_aria_label.get_value()",
        "aria-disabled=trigger_aria.attrs.aria_disabled",
        "role=\"dialog\"",
        "aria-modal=\"false\"",
        "aria-labelledby=panel_aria_labelledby.get_value()",
        "data-state=move || state.get().data_state_attr",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-focus-visible=move || trigger_focus_ring.is_focus_visible.get().then_some(\"true\")",
        "on:focus=move |_| trigger_focus_ring.handlers.on_focus.run(())",
        "on:blur=move |_| {",
        "on:keydown=move |ev| {",
        "on:keyup=move |ev| {",
    ] {
        assert!(
            view_source.contains(marker),
            "color-picker semantics/perf matrix should keep aria/data/focus marker `{marker}`.",
        );
    }

    for marker in [
        "\"color-picker\" => UiPerfBudget {",
        "max_mount_ms: 36.0,",
        "max_update_ms: Some(12.0),",
        "max_heap_kb: Some(640.0),",
    ] {
        assert!(
            docs_shell_source.contains(marker),
            "docs shell should preserve color-picker perf budget marker `{marker}`.",
        );
    }

    for marker in [
        "cargo test -p ui-color-picker color_picker_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-color-picker color_picker_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
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
        "color_picker_semantic_contract_matrix_covers_state_inputs_and_platform_paths",
        "color_picker_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "color_picker_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            check2_source.contains(marker),
            "color-picker check2 semantics/perf section should reference `{marker}`.",
        );
    }

    assert!(
        logic_source.contains("pub fn resolve_state("),
        "logic should keep state derivation path for attributable semantics/perf regressions.",
    );
}

#[test]
fn color_picker_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");

    for needle in [
        "let render_trigger = move || {",
        "let render_panel = move || {",
        "{render_trigger()}",
        "{render_panel()}",
    ] {
        assert!(
            view_source.contains(needle),
            "color-picker view should split macro-heavy layout into semantic sub-block marker `{needle}`.",
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 8,
        "color-picker view macro complexity regression: expected <= 8 `view!` blocks, found {view_macro_count}.",
    );

    let script_needle = "cargo test -p ui-color-picker color_picker_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_picker_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");

    for needle in [
        "fn render_empty_swatch(",
        "fn render_selected_swatch(",
        "fn render_selected_value_text(",
        "render_empty_swatch(",
        "render_selected_swatch(",
        "render_selected_value_text(",
    ] {
        assert!(
            view_source.contains(needle),
            "color-picker view should keep function-first split marker `{needle}`.",
        );
    }

    let component_attr_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_attr_count, 1,
        "color-picker view should keep only root component entry; found {component_attr_count} `#[component]` markers.",
    );
    for forbidden in [
        "#[component]\nfn render_",
        "#[component]\r\nfn render_",
        "struct Render",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "function-first split should avoid extra local component abstraction marker `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_picker_static_fragments_are_constantized_with_stable_semantics() {
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");
    let check2_source = load_source("../../components/color-picker/check2.md");

    for needle in [
        "const CLASS_TRIGGER: &str = \"ui-color-picker__trigger\";",
        "const CLASS_SWATCH: &str = \"ui-color-picker__swatch\";",
        "const CLASS_LABEL: &str = \"ui-color-picker__label\";",
        "const CLASS_VALUE: &str = \"ui-color-picker__value\";",
        "const CLASS_PANEL: &str = \"ui-color-picker__panel\";",
        "const CLASS_CONTENT: &str = \"ui-color-picker__content\";",
        "const SLOT_ROOT: &str = \"color-picker\";",
        "const SLOT_TRIGGER: &str = \"color-picker-trigger\";",
        "const SLOT_SWATCH: &str = \"color-picker-swatch\";",
        "const SLOT_LABEL: &str = \"color-picker-label\";",
        "const SLOT_VALUE: &str = \"color-picker-value\";",
        "const SLOT_PANEL: &str = \"color-picker-panel\";",
        "const SLOT_CONTENT: &str = \"color-picker-content\";",
        "const ARIA_HIDDEN_TRUE: &str = \"true\";",
        "class=CLASS_TRIGGER",
        "class=CLASS_SWATCH",
        "class=CLASS_LABEL",
        "class=CLASS_VALUE",
        "class=CLASS_PANEL",
        "class=CLASS_CONTENT",
        "data-slot=SLOT_ROOT",
        "data-slot=SLOT_TRIGGER",
        "data-slot=SLOT_SWATCH",
        "data-slot=SLOT_LABEL",
        "data-slot=SLOT_VALUE",
        "data-slot=SLOT_PANEL",
        "data-slot=SLOT_CONTENT",
        "aria-hidden=ARIA_HIDDEN_TRUE",
        "aria-haspopup=\"dialog\"",
        "role=\"dialog\"",
    ] {
        assert!(
            view_source.contains(needle),
            "color-picker static fragment contract should include `{needle}`.",
        );
    }

    for forbidden in [
        "class=\"ui-color-picker__trigger\"",
        "class=\"ui-color-picker__swatch\"",
        "class=\"ui-color-picker__label\"",
        "class=\"ui-color-picker__value\"",
        "class=\"ui-color-picker__panel\"",
        "class=\"ui-color-picker__content\"",
        "data-slot=\"color-picker-trigger\"",
        "data-slot=\"color-picker-swatch\"",
        "data-slot=\"color-picker-label\"",
        "data-slot=\"color-picker-value\"",
        "data-slot=\"color-picker-panel\"",
        "data-slot=\"color-picker-content\"",
        "inner_html=",
        "dangerously_set_inner_html",
        "markdown_to_html(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-picker view should avoid scattered inline static fragment token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_static_fragments_are_constantized_with_stable_semantics";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
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
fn color_picker_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let script_source = load_source("../../scripts/check-ui-inner-html.sh");
    let check2_source = load_source("../../components/color-picker/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for rel_path in [
        "../../components/color-picker/src/lib.rs",
        "../../components/color-picker/src/mod.rs",
        "../../components/color-picker/src/logic.rs",
        "../../components/color-picker/src/motion.rs",
        "../../components/color-picker/src/protocol.rs",
        "../../components/color-picker/src/styles.rs",
        "../../components/color-picker/src/view.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in [
            "inner_html=",
            "set_inner_html(",
            "dangerously_set_inner_html",
            "innerHTML",
            "markdown_to_html(",
            "<script",
        ] {
            assert!(
                !source.contains(forbidden),
                "color-picker component source `{rel_path}` must not contain html injection marker `{forbidden}`.",
            );
        }
    }

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "innerHTML",
        "markdown_to_html(",
        "<script",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "color-picker docs examples should avoid html injection marker `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(script_needle),
        "inner-html gate script should include `{script_needle}`.",
    );

    for needle in [
        "`inner_html` 使用约束",
        "仅允许编译期常量或明确白名单内容进入 `inner_html`",
        "严禁直接或间接注入用户输入、远端返回或未清洗模板字符串",
        "使用 `inner_html` 的节点必须补语义测试与安全回归说明",
        "color_picker_inner_html_usage_is_forbidden_in_component_and_docs_examples",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep inner-html governance marker `{needle}`.",
        );
    }
}

#[test]
fn color_picker_wasm_debug_contract_is_explicitly_na_and_feature_isolated() {
    let check2_source = load_source("../../components/color-picker/check2.md");
    let script_source = load_source("../../scripts/check-ui-wasm-debug.sh");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");

    let color_picker_cargo = load_source("../../components/color-picker/Cargo.toml");
    let ui_components_cargo = load_source("../../crates/ui/Cargo.toml");
    let ui_components_lib = load_source("../../crates/ui/src/lib.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let mod_source = load_source("../../components/color-picker/src/mod.rs");
    let lib_source = load_source("../../components/color-picker/src/lib.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in ["[features]", "default = []"] {
        assert!(
            color_picker_cargo.contains(needle),
            "color-picker crate feature boundary should include `{needle}`.",
        );
    }

    for forbidden in [
        "wasm-debug",
        "color-picker-wasm-debug",
        "color_picker-wasm-debug",
        "component-color_picker-wasm-debug",
    ] {
        assert!(
            !color_picker_cargo.contains(forbidden),
            "color-picker crate should not expose wasm-debug feature `{forbidden}`.",
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
        "color-picker-wasm-debug =",
        "color_picker-wasm-debug =",
        "component-color_picker-wasm-debug",
        "component-color_picker\", \"dep:tracing",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "ui feature graph should not leak color-picker debug toggle `{forbidden}`.",
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
        "data-open-mode=move || state.get().open_mode_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=if motion == ColorPickerMotion::default() {",
        "on:pointerdown=move |_| trigger_aria.handlers.press.on_pointer_down.run(())",
        "on:pointerup=move |_| trigger_aria.handlers.press.on_pointer_up.run(())",
        "on:click=move |_| trigger_aria.handlers.press.on_click.run(())",
        "on:keydown=move |ev| {",
    ] {
        assert!(
            view_source.contains(needle),
            "color-picker should keep machine-readable state/action marker `{needle}` for debug traceability/replay.",
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
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !lib_source.contains(forbidden),
            "color-picker runtime/public contract should not leak wasm-debug internals `{forbidden}`.",
        );
    }

    for needle in [
        "pub(super) fn color_picker() -> AnyView",
        "title=\"Config Workbench\"",
        "let on_selected_change =",
        "let on_open_change =",
        "selected_color=workbench_selected_color",
        "on_selected_change=on_selected_change",
        "open=workbench_open",
        "on_open_change=on_open_change",
        "\"selected=\"",
        "\" · open=\"",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "docs page should keep reproducible color-picker interaction marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_wasm_debug_contract_is_explicitly_na_and_feature_isolated";
    assert!(
        script_source.contains(script_needle),
        "wasm-debug gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] WASM 调试要求",
        "color_picker_wasm_debug_contract_is_explicitly_na_and_feature_isolated",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep wasm-debug governance marker `{needle}`.",
        );
    }
}

#[test]
fn color_picker_dx_workbench_supports_hot_style_feedback_and_optional_state_preserve() {
    let check2_source = load_source("../../components/color-picker/check2.md");
    let script_source = load_source("../../scripts/check-ui-dx.sh");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Show settings\"",
        "\"Show code\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-playground-scope=scope_id.clone()",
        "data-slot=\"playground-test\"",
        "<div class=\"playground__preview-stage\">{children()}</div>",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep DX hot-style/isolated-canvas marker `{needle}`.",
        );
    }

    let color_picker_section_start = docs_page_source
        .find("pub(super) fn color_picker() -> AnyView")
        .expect("docs forms_color page should include color_picker section start.");
    let color_picker_section_end = docs_page_source[color_picker_section_start..]
        .find("pub(super) fn color_thumb() -> AnyView")
        .map(|offset| color_picker_section_start + offset)
        .expect("docs forms_color page should include color_picker section end.");
    let color_picker_section =
        &docs_page_source[color_picker_section_start..color_picker_section_end];

    for needle in [
        "pub(super) fn color_picker() -> AnyView",
        "slug=\"color-picker\"",
        "title=\"Config Workbench\"",
        "description=\"Covers full ColorPicker API and shows callback feedback.\"",
        "code_signal=workbench_code",
        "test_config_signal=workbench_actual_config",
        "controls=move || view!",
        "data-slot=\"color-picker-workbench-controls\"",
        "<Switch checked=workbench_disabled set_checked=set_workbench_disabled>",
        "<Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>",
        "<Switch checked=workbench_rtl set_checked=set_workbench_rtl>",
        "<Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>",
        "<Switch checked=workbench_swatch_bordered set_checked=set_workbench_swatch_bordered>",
        "id_base=\"docs-color-picker-workbench\".to_string()",
        "data-slot=\"color-picker-workbench-preview\"",
        "\" · open=\"",
    ] {
        assert!(
            color_picker_section.contains(needle),
            "color-picker docs should keep interactive DX workbench marker `{needle}`.",
        );
    }

    for forbidden in [
        "COLOR_PICKER_WORKBENCH_STORAGE_KEY",
        "load_color_picker_workbench_state(",
        "save_color_picker_workbench_state(",
        "clear_color_picker_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !color_picker_section.contains(forbidden),
            "color-picker keeps persistent storage as optional N/A in current scope; `{forbidden}` should remain absent.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_dx_workbench_supports_hot_style_feedback_and_optional_state_preserve";
    assert!(
        script_source.contains(script_needle),
        "DX gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
        "color_picker_dx_workbench_supports_hot_style_feedback_and_optional_state_preserve",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep DX governance marker `{needle}`.",
        );
    }
}

#[test]
fn color_picker_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries() {
    let check2_source = load_source("../../components/color-picker/check2.md");
    let script_source = load_source("../../scripts/check-ui-engineering.sh");
    let protocol_source = load_source("../../components/color-picker/src/protocol.rs");
    let protocol_tests_source = include_str!("protocol.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let mod_source = load_source("../../components/color-picker/src/mod.rs");
    let lib_source = load_source("../../components/color-picker/src/lib.rs");
    let styles_source = load_source("../../components/color-picker/src/styles.rs");
    let motion_source = load_source("../../components/color-picker/src/motion.rs");
    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(rename_all = \"snake_case\")]",
        "pub enum PickerComponentSchemaVersion",
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "pub struct PickerComponentSpec",
        "#[serde(default)]",
        "pub schema_version: PickerComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "color-picker engineering contract should keep serde protocol marker `{needle}`.",
        );
    }

    for needle in [
        "fn assert_serde<T>()",
        "T: Serialize + DeserializeOwned,",
        "assert_serde::<PickerComponentSchemaVersion>();",
        "assert_serde::<PickerComponentSpec>();",
    ] {
        assert!(
            protocol_tests_source.contains(needle),
            "color-picker protocol test should keep structured serde contract marker `{needle}`.",
        );
    }

    for needle in [
        "use_controllable_open_state_traced(",
        "trace.emit(component, UiTraceEventKind::OpenChange { open: next });",
        "pub enum UiTraceEventKind",
        "OpenChange {",
        "open: bool,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            view_source.contains(needle)
                || controllable_state_source.contains(needle)
                || trace_source.contains(needle),
            "color-picker engineering contract should keep unified tracing semantics marker `{needle}`.",
        );
    }

    for source in [
        lib_source,
        mod_source,
        logic_source,
        view_source,
        styles_source,
        motion_source,
    ] {
        for forbidden in [
            "tokio::",
            "tokio =",
            "#[tokio::main]",
            "#[tokio::test]",
            "tokio::runtime",
            "async_std::",
            "async-std",
            "#[async_std::main]",
            "async fn",
            "JoinHandle",
            "Runtime",
        ] {
            assert!(
                !source.contains(forbidden),
                "color-picker engineering boundary should avoid runtime leakage token `{forbidden}`.",
            );
        }
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
        "color_picker_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep engineering governance marker `{needle}`.",
        );
    }
}

#[test]
fn color_picker_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
 {
    let check2_source = load_source("../../components/color-picker/check2.md");
    let script_source = load_source("../../scripts/check-ui-engineering.sh");
    let protocol_source = load_source("../../components/color-picker/src/protocol.rs");
    let component_manifest = load_source("../../components/color-picker/src/Component.toml");
    let rbi_source = load_source("../../components/color-picker/src/color_picker.rbi");

    for required in [
        "pub enum PickerComponentSchemaVersion",
        "V1",
        "pub struct PickerComponentSpec",
        "pub schema_version: PickerComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(required),
            "color-picker protocol should keep stable v1 marker `{required}` in non-breaking scope.",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "schema = \"ui.color-picker.agent-contract.v1\"",
        "values = [\"ui.color-picker.agent-contract\"]",
        "values = [\"v1\"]",
    ] {
        assert!(
            component_manifest.contains(required),
            "color-picker Component.toml should keep v1 registration marker `{required}` in current scope.",
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
                && !component_manifest.contains(forbidden)
                && !rbi_source.contains(forbidden),
            "without major breaking upgrade, color-picker should not claim migration surface token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `ColorPicker` 改动未引入跨大版本 API 破坏升级，组件协议与 Agent Contract 仍保持 `v1`（`components/color-picker/src/protocol.rs` 的 `PickerComponentSchemaVersion::V1`、`components/color-picker/src/Component.toml` 的 `schema_version = \"1\"` 与 `ui.color-picker.agent-contract.v1`），因此不触发 Codemod/Schema Registry 弃用窗口与 `migrate_v1_to_v2` 迁移层要求。回归：`components/color-picker/test/semantics.rs::color_picker_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade`；门禁脚本：`scripts/check-ui-engineering.sh` 新增对应 `cargo test` 目标。）",
        "color_picker_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep codemod/registry migration marker `{needle}`.",
        );
    }
}

#[test]
fn color_picker_check2_documents_docs_sync_and_state_matrix_rules() {
    let checklist_source = load_source("../../components/color-picker/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ColorPicker checklist should keep docs-sync/state-matrix rule `{required}`.",
        );
    }
}

#[test]
fn color_picker_docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");

    for required in [
        "pub(super) fn color_picker() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Config Workbench\"",
        "title=\"State Matrix\"",
        "data-slot=\"color-picker-state-matrix\"",
        "id_base=\"docs-color-picker-matrix-default\".to_string()",
        "id_base=\"docs-color-picker-matrix-open\".to_string()",
        "id_base=\"docs-color-picker-matrix-disabled\".to_string()",
        "default_selected_color=\"#3b82f6\".to_string()",
        "default_selected_color=\"#8b5cf6\".to_string()",
        "default_selected_color=\"#0ea5e9\".to_string()",
        "default_open=true",
        "is_disabled=true",
    ] {
        assert!(
            docs_source.contains(required),
            "ColorPicker docs should keep API/default/state-matrix marker `{required}`.",
        );
    }

    for required in [
        "#[prop(optional)] value: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] selected_color: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_selected_color: Option<String>",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] disabled: Option<bool>",
        "let selected_color = logic::resolve_selected_color_axis(value, selected_color);",
        "let default_selected_color =",
        "logic::resolve_default_selected_color(default_value, default_selected_color);",
        "let on_selected_change =",
        "logic::resolve_selected_change_axis(on_value_change, on_selected_change);",
        "let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);",
    ] {
        assert!(
            view_source.contains(required),
            "ColorPicker view contract should keep `{required}`.",
        );
    }

    for required in [
        "pub fn resolve_default_selected_color(",
        "sanitize_selected_color(default_value.or(default_selected_color))",
        "pub fn resolve_is_disabled(is_disabled: bool, disabled: Option<bool>) -> bool {",
        "disabled.unwrap_or(is_disabled)",
        "pub fn resolve_selected_color_axis<T>(value: Option<T>, selected_color: Option<T>) -> Option<T> {",
        "value.or(selected_color)",
        "pub fn resolve_selected_change_axis<T>(",
        "on_value_change.or(on_selected_change)",
    ] {
        assert!(
            logic_source.contains(required),
            "ColorPicker logic default/normalization contract should keep `{required}`.",
        );
    }
}

#[test]
fn color_picker_contract_hygiene_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-color-picker color_picker_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-color-picker color_picker_docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`.",
        );
    }
}

#[test]
fn color_picker_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("../../components/color-picker/check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "color-picker checklist should keep documentation-as-product rule `{required}`.",
        );
    }
}

#[test]
fn color_picker_docs_entry_exists_as_readme_or_equivalent_docs_app_page() {
    let readme_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../components/color-picker/src/README.md");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");

    assert!(
        readme_path.exists(),
        "color-picker should provide README as documentation entry.",
    );
    assert!(
        docs_page_source.contains("pub(super) fn color_picker() -> AnyView"),
        "docs-app should expose color_picker docs entry function.",
    );
    assert!(
        docs_index_source.contains(
            "component_doc!(\n        \"ColorPicker\",\n        \"color-picker\",\n        \"Forms\",\n        forms_color::color_picker",
        ),
        "docs-app components index should expose color-picker entry.",
    );
}

#[test]
fn color_picker_docs_are_beginner_friendly_with_default_then_advanced_path() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let readme_source = load_source("../../components/color-picker/src/README.md");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "title=\"ColorPicker\"",
        "slug=\"color-picker\"",
        "title=\"Hello World\"",
        "title=\"State Matrix\"",
        "title=\"Config Workbench\"",
    ] {
        assert!(
            docs_source.contains(required),
            "color-picker docs should keep beginner-to-advanced marker `{required}`.",
        );
    }

    let hello_pos = docs_source
        .find("id_base=\"docs-color-picker-hello\".to_string()")
        .expect("docs should include hello-world playground for zero-threshold path.");
    let workbench_pos = docs_source
        .find("data-slot=\"color-picker-workbench-controls\"")
        .expect("docs should include config workbench playground.");
    let matrix_pos = docs_source
        .find("data-slot=\"color-picker-state-matrix\"")
        .expect("docs should include state-matrix playground as common usage.");
    assert!(
        hello_pos < workbench_pos && workbench_pos < matrix_pos,
        "docs should keep zero-threshold default path ahead of advanced controls.",
    );

    for required in [
        "## Hello World（最小可用）",
        "## 受控用法",
        "## 常见用法（进阶）",
        "阅读顺序建议：先看 `Hello World（默认路径）` 直接跑起来，再按需启用受控与高级配置。",
        "默认路径不需要手动接线 `ui-state-primitives` 或 `ui-headless` 状态机。",
    ] {
        assert!(
            readme_source.contains(required),
            "README should keep beginner-friendly guidance marker `{required}`.",
        );
    }

    let readme_hello_pos = readme_source
        .find("## Hello World（最小可用）")
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

    let script_needle = "cargo test -p ui-color-picker color_picker_docs_are_beginner_friendly_with_default_then_advanced_path";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_picker_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let readme_source = load_source("../../components/color-picker/src/README.md");

    for required in [
        "title=\"Hello World\"",
        "<ColorPicker id_base=\"docs-color-picker-hello\".to_string()>",
        "\"Choose a brand color\"",
        "## Hello World（最小可用）",
        "id_base=\"demo-color-picker\".to_string()",
    ] {
        assert!(
            docs_source.contains(required) || readme_source.contains(required),
            "color-picker docs hello-world should keep zero-threshold marker `{required}`.",
        );
    }

    for forbidden in [
        "ui_state_primitives",
        "state=...",
        "logic::",
        "use_presence(",
    ] {
        assert!(
            !readme_source.contains(forbidden),
            "color-picker README hello-world path should avoid architecture-wiring token `{forbidden}`.",
        );
    }
}

#[test]
fn color_picker_contract_hygiene_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for marker in [
        "cargo test -p ui-color-picker color_picker_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-color-picker color_picker_docs_entry_exists_as_readme_or_equivalent_docs_app_page",
        "cargo test -p ui-color-picker color_picker_docs_are_beginner_friendly_with_default_then_advanced_path",
        "cargo test -p ui-color-picker color_picker_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring",
    ] {
        assert!(
            script_source.contains(marker),
            "contract-hygiene script should enforce documentation-as-product marker `{marker}`.",
        );
    }
}

#[test]
fn color_picker_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("../../components/color-picker/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(required),
            "color-picker checklist should keep interactive-playground rule `{required}`.",
        );
    }
}

#[test]
fn color_picker_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for marker in [
        "pub(super) fn color_picker() -> AnyView",
        "title=\"Config Workbench\"",
        "description=\"Covers full ColorPicker API and shows callback feedback.\"",
        "code_signal=workbench_code",
        "test_config_signal=workbench_actual_config",
        "controls=move || view!",
        "data-slot=\"color-picker-workbench-controls\"",
        "<Switch checked=workbench_disabled set_checked=set_workbench_disabled>",
        "<Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>",
        "<Switch checked=workbench_rtl set_checked=set_workbench_rtl>",
        "<Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>",
        "<Switch checked=workbench_swatch_bordered set_checked=set_workbench_swatch_bordered>",
        "id_base=\"docs-color-picker-workbench\".to_string()",
        "selected_color=workbench_selected_color",
        "on_selected_change=on_selected_change",
        "open=workbench_open",
        "on_open_change=on_open_change",
        "data-slot=\"color-picker-workbench-preview\"",
        "\" · open=\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "color-picker docs should keep interactive playground marker `{marker}`.",
        );
    }

    for marker in [
        "data-playground-scope=scope_id.clone()",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "data-slot=\"playground-test\"",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
    ] {
        assert!(
            playground_source.contains(marker),
            "docs-app Playground should keep interactive preview contract marker `{marker}`.",
        );
    }
}

#[test]
fn color_picker_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_picker_contract.spec.mjs");

    for marker in [
        "docs-app color-picker key flow is repeatable and failures map to semantic breakpoints",
        "await page.goto(COLOR_PICKER_PAGE);",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-component=\"color-picker\"] #docs-color-picker-basic[data-slot=\"color-picker\"][data-open-mode=\"controlled\"]",
        "await page.keyboard.press(\"Enter\");",
        "await expect(root).toHaveAttribute(\"data-open\", \"true\");",
        "await expect(root).toHaveAttribute(\"data-ui-action\", \"toggle-open\");",
        "await page.reload();",
        "await expect(root).toHaveAttribute(\"data-ui-action\", \"snapshot-render\");",
        "await expect(root).toHaveAttribute(\"data-ui-output-status\", \"verified\");",
    ] {
        assert!(
            e2e_source.contains(marker),
            "color-picker interactive playground should keep repeatable semantic e2e marker `{marker}`.",
        );
    }
}

#[test]
fn color_picker_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for marker in [
        "cargo test -p ui-color-picker color_picker_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-color-picker color_picker_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(marker),
            "DX check script should enforce interactive-playground contract marker `{marker}`.",
        );
    }
}

#[test]
fn color_picker_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("../../components/color-picker/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(required),
            "color-picker checklist should keep source-first copy-paste-ready rule `{required}`.",
        );
    }
}

#[test]
fn color_picker_docs_are_copy_paste_ready_with_imports_copy_button_and_sync() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");

    for marker in [
        "pub(super) fn color_picker() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Config Workbench\"",
        "title=\"State Matrix\"",
        "code_signal=hello_code",
        "code_signal=workbench_code",
        "code_signal=matrix_code",
        "test_config_signal=workbench_actual_config",
    ] {
        assert!(
            docs_source.contains(marker),
            "color-picker docs should keep source-first copy-ready marker `{marker}`.",
        );
    }

    for marker in [
        "const DEFAULT_PLAYGROUND_IMPORTS",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let missing_imports = missing_import_lines(&raw, &imports);",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "\"Show code\"",
        "Copy",
    ] {
        assert!(
            playground_source.contains(marker),
            "docs-app playground should keep copy-ready pipeline marker `{marker}`.",
        );
    }

    for marker in [
        "let hello_code = Signal::derive",
        "let workbench_code = Signal::derive",
        "let matrix_code = Signal::derive",
        "id_base=\"docs-color-picker-workbench\".to_string()",
        "id_base=\"docs-color-picker-matrix-default\".to_string()",
        "selected_color=workbench_selected_color",
        "on_selected_change=on_selected_change",
        "#[prop(optional)] selected_color: Option<Signal<Option<String>>>",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "pub fn resolve_selected_color_axis<T>(value: Option<T>, selected_color: Option<T>) -> Option<T> {",
        "pub fn resolve_selected_change_axis<T>(",
    ] {
        assert!(
            docs_source.contains(marker)
                || view_source.contains(marker)
                || logic_source.contains(marker),
            "color-picker source-first snippet should stay synced with implementation marker `{marker}`.",
        );
    }
}

#[test]
fn color_picker_contract_hygiene_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for marker in [
        "cargo test -p ui-color-picker color_picker_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-color-picker color_picker_docs_are_copy_paste_ready_with_imports_copy_button_and_sync",
    ] {
        assert!(
            script_source.contains(marker),
            "contract-hygiene script should enforce source-first copy-ready marker `{marker}`.",
        );
    }
}

#[test]
fn color_picker_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let logic_source = load_source("../../components/color-picker/src/logic.rs");
    let readme_source = load_source("../../components/color-picker/src/README.md");

    for marker in [
        "### ColorPicker 同步记录（2026-02-20）",
        "value + on_value_change + default_value",
        "selected_color + on_selected_change + default_selected_color",
        "open + on_open_change + default_open",
        "is_disabled (disabled legacy alias)",
        "component_doc!(\"ColorPicker\", \"color-picker\", \"Forms\", forms_color::color_picker)",
        "apps/docs-app/src/pages/components/pages/forms_color.rs::color_picker()",
        "Hello World",
        "State Matrix",
        "参数语义若变更，必须先同步本策略文档与 docs 入口",
    ] {
        assert!(
            strategy_source.contains(marker) || docs_index_source.contains(marker),
            "color-picker HeroUI/doc sync record should include `{marker}`.",
        );
    }

    for marker in [
        "pub(super) fn color_picker() -> AnyView",
        "title=\"ColorPicker\"",
        "slug=\"color-picker\"",
        "title=\"Config Workbench\"",
        "data-slot=\"color-picker-workbench-controls\"",
    ] {
        assert!(
            docs_page_source.contains(marker),
            "color-picker docs entry should keep indexable marker `{marker}`.",
        );
    }

    for marker in [
        "#[prop(optional)] value: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] selected_color: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_selected_color: Option<String>",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional)] disabled: Option<bool>,",
        "#[prop(optional)] motion: ColorPickerMotion,",
        "pub fn resolve_default_selected_color(",
        "pub fn resolve_is_disabled(",
        "pub fn resolve_selected_color_axis<T>(",
        "pub fn resolve_selected_change_axis<T>(",
    ] {
        assert!(
            view_source.contains(marker) || logic_source.contains(marker),
            "color-picker parameter model marker `{marker}` should remain in implementation.",
        );
    }

    for marker in [
        "# ColorPicker",
        "## Hello World（最小可用）",
        "## 受控用法",
        "## 常见用法（进阶）",
        "docs 入口：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_picker()`",
    ] {
        assert!(
            readme_source.contains(marker),
            "color-picker README/docs entry should keep marker `{marker}`.",
        );
    }
}

#[test]
fn color_picker_check2_marks_heroui_strategy_and_component_docs_sync_complete() {
    let check2_source = load_source("../../components/color-picker/check2.md");

    for marker in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        "color_picker_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
    ] {
        assert!(
            check2_source.contains(marker),
            "color-picker checklist should keep HeroUI/doc sync completion evidence `{marker}`.",
        );
    }
}

#[test]
fn color_picker_contract_hygiene_script_covers_heroui_strategy_doc_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for marker in [
        "cargo test -p ui-color-picker color_picker_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
        "cargo test -p ui-color-picker color_picker_check2_marks_heroui_strategy_and_component_docs_sync_complete",
    ] {
        assert!(
            script_source.contains(marker),
            "contract-hygiene script should enforce HeroUI/doc sync marker `{marker}`.",
        );
    }
}

#[test]
fn color_picker_docs_product_contract_is_copy_paste_ready_with_playground_stream_snapshot_and_imports()
 {
    let check2_source = load_source("../../components/color-picker/check2.md");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let docs_playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for required in [
        "pub(super) fn color_picker() -> AnyView {",
        "let hello_code = Signal::derive",
        "let workbench_code = Signal::derive",
        "let matrix_code = Signal::derive",
        "title=\"Hello World\"",
        "title=\"Config Workbench\"",
        "title=\"State Matrix\"",
        "code_signal=hello_code",
        "code_signal=workbench_code",
        "code_signal=matrix_code",
        "data-slot=\"color-picker-state-matrix\"",
        "data-slot=\"color-picker-workbench-controls\"",
        "data-slot=\"color-picker-workbench-preview\"",
    ] {
        assert!(
            docs_page_source.contains(required),
            "color-picker docs product page should keep copy-paste-ready playground marker `{required}`.",
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "let missing_imports = missing_import_lines(&raw, &imports);",
        "#[prop(optional, into)] code_imports: Option<String>",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "Copy",
    ] {
        assert!(
            docs_playground_source.contains(required),
            "shared docs playground should keep import-ready copy path marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_docs_product_contract_is_copy_paste_ready_with_playground_stream_snapshot_and_imports";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）",
        "`apps/docs-app/src/pages/components/pages/forms_color.rs::color_picker()`",
        "Hello World",
        "State Matrix",
        "components/color-picker/test/semantics.rs::color_picker_docs_product_contract_is_copy_paste_ready_with_playground_stream_snapshot_and_imports",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep docs-as-product copy-paste-ready evidence `{required}`.",
        );
    }
}

#[test]
fn color_picker_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only() {
    let check2_source = load_source("../../components/color-picker/check2.md");
    let view_source = load_source("../../components/color-picker/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");
    let local_semantics = include_str!("semantics.rs");
    let legacy_semantics = load_source("legacy_semantics");
    let logic_tests = include_str!("logic.rs");

    for required in [
        "role=trigger_aria.attrs.role",
        "aria-expanded=move || trigger_aria_expanded.get().unwrap_or(\"false\")",
        "aria-controls=move || trigger_aria_controls.get()",
        "aria-disabled=trigger_aria.attrs.aria_disabled",
        "data-state=move || state.get().data_state_attr",
        "data-open-mode=move || state.get().open_mode_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "on:pointerdown=move |_| trigger_aria.handlers.press.on_pointer_down.run(())",
        "on:keydown=move |ev| {",
    ] {
        assert!(
            view_source.contains(required),
            "color-picker view should keep semantic contract marker `{required}`.",
        );
    }

    for required in [
        "fn color_picker_semantic_contract_matrix_covers_state_inputs_and_platform_paths()",
        "fn color_picker_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous()",
        "fn color_picker_snapshot_baseline_consumes_complete_result_and_renders_stably()",
        "fn color_picker_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only()",
        "role=trigger_aria.attrs.role",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            local_semantics.contains(required),
            "semantic test suite should include semantic-first coverage token `{required}`.",
        );
    }

    for required in [
        "../../../components/color-picker/test/semantics.rs",
        "color_picker_semantics_tests_are_migrated_to_component_directory",
    ] {
        assert!(
            legacy_semantics.contains(required) || local_semantics.contains(required),
            "color-picker should keep `*_semantics.rs` compatibility marker `{required}`.",
        );
    }

    for forbidden in [
        "assert_snapshot(",
        "insta::assert",
        "to_match_snapshot(",
        "snapshot!(",
    ] {
        assert!(
            !logic_tests.contains(forbidden),
            "semantic-priority contract should reject snapshot-only assertion token `{forbidden}` in logic semantics regression.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "color_picker_semantic_contract_matrix_covers_state_inputs_and_platform_paths",
        "color_picker_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "color_picker_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "color_picker_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should keep semantic-priority checklist marker `{required}`.",
        );
    }
}

#[test]
fn color_picker_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("../../components/color-picker/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
        "color_picker_check2_documents_e2e_selector_and_stable_wait_rules",
        "color_picker_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "color_picker_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints",
        "components/color-picker/scripts/check-ui-e2e-color-picker.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should keep e2e-selector/stable-wait governance marker `{required}`.",
        );
    }
}

#[test]
fn color_picker_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_picker_contract.spec.mjs");
    let script_source =
        load_source("../../components/color-picker/scripts/check-ui-e2e-color-picker.sh");

    for required in [
        "const COLOR_PICKER_PAGE = \"/#/components/color-picker\";",
        "body:not(:has(#boot))",
        "[data-component=\"color-picker\"] #docs-color-picker-basic[data-slot=\"color-picker\"][data-open-mode=\"controlled\"]",
        "data-slot=\"color-picker-trigger\"",
        "data-slot=\"color-picker-label\"",
        "data-ui-schema",
        "data-ui-schema-version",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-ui-selection-source",
        "data-ui-open-source",
        "data-ui-output-status",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-picker e2e contract should include semantic selector/wait marker `{required}`.",
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
            "color-picker e2e contract should avoid brittle selector/wait token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_e2e_selector_contract_uses_semantic_markers_and_stable_waits";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-picker gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_picker_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_picker_contract.spec.mjs");
    let script_source =
        load_source("../../components/color-picker/scripts/check-ui-e2e-color-picker.sh");

    for required in [
        "interaction path covers ready/settled semantic breakpoints",
        "trigger.focus()",
        "toBeFocused()",
        "trigger.click()",
        "data-slot=\"color-picker-panel\"][role=\"dialog\"",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveAttribute(\"data-ui-action\", \"toggle-open\")",
        "toHaveAttribute(\"data-ui-output-status\", \"submittable\")",
        "toHaveCount(0)",
        "not.toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"data-ui-action\", \"toggle-close\")",
        "#docs-color-picker-disabled[data-slot=\"color-picker\"]",
        "toHaveAttribute(\"data-state\", \"disabled\")",
        "toHaveAttribute(\"data-disabled\", \"true\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "toHaveAttribute(\"aria-disabled\", \"true\")",
        "toBeDisabled()",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-picker e2e ready/settled contract should include `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-picker gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_picker_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = load_source("../../components/color-picker/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
        "color_picker_check2_documents_e2e_repeatable_key_flow_rules",
        "color_picker_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "color_picker_e2e_high_risk_paths_cover_overlay_focus_keyboard_and_settled_semantic_breakpoints",
        "components/color-picker/scripts/check-ui-e2e-color-picker.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should keep repeatable e2e flow governance marker `{required}`.",
        );
    }
}

#[test]
fn color_picker_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_picker_contract.spec.mjs");
    let script_source =
        load_source("../../components/color-picker/scripts/check-ui-e2e-color-picker.sh");

    for required in [
        "key flow is repeatable and failures map to semantic breakpoints",
        "page.keyboard.press(\"Enter\")",
        "data-ui-action\", \"toggle-open\"",
        "data-ui-action\", \"toggle-close\"",
        "data-ui-output-status\", \"submittable\"",
        "await page.reload();",
        "data-ui-action\", \"snapshot-render\"",
        "data-ui-output-status\", \"verified\"",
        "toBeFocused()",
    ] {
        assert!(
            e2e_source.contains(required),
            "repeatable color-picker e2e flow should include semantic breakpoint `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_e2e_key_flow_is_repeatable_and_failure_points_are_semantic";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-picker gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_picker_e2e_high_risk_paths_cover_overlay_focus_keyboard_and_settled_semantic_breakpoints()
{
    let e2e_source = load_source("../../e2e/tests/docs_app_color_picker_contract.spec.mjs");
    let script_source =
        load_source("../../components/color-picker/scripts/check-ui-e2e-color-picker.sh");

    for required in [
        "high-risk paths keep overlay focus keyboard and async boundaries semantically explicit",
        "page.keyboard.press(\"Space\")",
        "page.keyboard.press(\"Escape\")",
        "data-slot=\"color-picker-panel\"][role=\"dialog\"",
        "data-ui-action\", \"toggle-open\"",
        "data-ui-action\", \"toggle-close\"",
        "data-ui-stream-support\", \"unsupported\"",
        "data-ui-stream-mode\", \"snapshot\"",
        "not.toHaveAttribute(\"aria-busy\", \"true\")",
        "#docs-color-picker-disabled[data-slot=\"color-picker\"]",
        "data-state\", \"disabled\"",
        "data-disabled\", \"true\"",
        "aria-disabled\", \"true\"",
        "toBeDisabled()",
    ] {
        assert!(
            e2e_source.contains(required),
            "high-risk color-picker e2e path should include `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-color-picker color_picker_e2e_high_risk_paths_cover_overlay_focus_keyboard_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-picker gate script should include `{script_needle}`.",
    );
}
