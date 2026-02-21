fn load_source(rel_path: &str) -> &'static str {
    match rel_path {
        "../../components/color-editor/src/lib.rs" => include_str!("../src/lib.rs"),
        "../../components/color-editor/src/mod.rs" => include_str!("../src/mod.rs"),
        "../../components/color-editor/src/logic.rs" => include_str!("../src/logic.rs"),
        "../../components/color-editor/src/motion.rs" => include_str!("../src/motion.rs"),
        "../../components/color-editor/src/styles.rs" => include_str!("../src/styles.rs"),
        "../../components/color-editor/src/view.rs" => include_str!("../src/view.rs"),
        "../../components/color-editor/src/README.md" => include_str!("../src/README.md"),
        "../../components/color-editor/src/check2.md" => include_str!("../src/check2.md"),
        "../../components/color-editor/check2.md" => include_str!("../check2.md"),
        "../ui-state-primitives/src/color_editor.rs" => {
            include_str!("../../../crates/ui-state-primitives/src/color_editor.rs")
        }
        "../../apps/docs-app/src/pages/components/pages/forms_color.rs" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages/forms_color.rs")
        }
        "../../apps/docs-app/src/pages/components/pages.rs" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages.rs")
        }
        "../../apps/docs-app/src/pages/components/shell.rs" => {
            include_str!("../../../apps/docs-app/src/pages/components/shell.rs")
        }
        "../../apps/docs-app/src/perf_probe.rs" => {
            include_str!("../../../apps/docs-app/src/perf_probe.rs")
        }
        "../../e2e/tests/docs_app_components_coverage.spec.mjs" => {
            include_str!("../../../e2e/tests/docs_app_components_coverage.spec.mjs")
        }
        "../../e2e/tests/docs_app_color_editor_contract.spec.mjs" => {
            include_str!("../../../e2e/tests/docs_app_color_editor_contract.spec.mjs")
        }
        "../../scripts/check-ui-components-performance.sh" => {
            include_str!("../../../scripts/check-ui-components-performance.sh")
        }
        "../../scripts/check-ui-components-view-macro.sh" => {
            include_str!("../../../scripts/check-ui-components-view-macro.sh")
        }
        "../../scripts/check-ui-components-inner-html.sh" => {
            include_str!("../../../scripts/check-ui-components-inner-html.sh")
        }
        "../../scripts/check-ui-components-wasm-debug.sh" => {
            include_str!("../../../scripts/check-ui-components-wasm-debug.sh")
        }
        "../../scripts/check-ui-components-dx.sh" => {
            include_str!("../../../scripts/check-ui-components-dx.sh")
        }
        "../../scripts/check-ui-components-engineering.sh" => {
            include_str!("../../../scripts/check-ui-components-engineering.sh")
        }
        "../../scripts/check-ui-components-component-files.sh" => {
            include_str!("../../../scripts/check-ui-components-component-files.sh")
        }
        "../../scripts/check-ui-components-contract-hygiene.sh" => {
            include_str!("../../../scripts/check-ui-components-contract-hygiene.sh")
        }
        "../../scripts/check-ui-components-entrypoints.sh" => {
            include_str!("../../../scripts/check-ui-components-entrypoints.sh")
        }
        "../../scripts/check-ui-components-platforms.sh" => {
            include_str!("../../../scripts/check-ui-components-platforms.sh")
        }
        "../../scripts/check-ui-components-streaming.sh" => {
            include_str!("../../../scripts/check-ui-components-streaming.sh")
        }
        "../../scripts/check-ui-components-tree-shaking.sh" => {
            include_str!("../../../scripts/check-ui-components-tree-shaking.sh")
        }
        "../../scripts/check-ui-components-e2e-color-editor.sh" => {
            include_str!("../../../scripts/check-ui-components-e2e-color-editor.sh")
        }
        "../../scripts/check-rust-hygiene.sh" => {
            include_str!("../../../scripts/check-rust-hygiene.sh")
        }
        "../../apps/docs-app/src/playground.rs" => {
            include_str!("../../../apps/docs-app/src/playground.rs")
        }
        "../../docs/plan/TODO.md" => include_str!("../../../docs/plan/TODO.md"),
        "../../docs/spec/heroui-parameter-design-strategy.md" => {
            include_str!("../../../docs/spec/heroui-parameter-design-strategy.md")
        }
        "../../apps/docs-app/src/lib.rs" => include_str!("../../../apps/docs-app/src/lib.rs"),
        "../../apps/docs-app/src/debug_overlay.rs" => {
            include_str!("../../../apps/docs-app/src/debug_overlay.rs")
        }
        "../../crates/ui-headless/src/trace.rs" => {
            include_str!("../../../crates/ui-headless/src/trace.rs")
        }
        "../../crates/ui-motion/src/lib.rs" => {
            include_str!("../../../crates/ui-motion/src/lib.rs")
        }
        "../../crates/ui-motion/src/spring.rs" => {
            include_str!("../../../crates/ui-motion/src/spring.rs")
        }
        "../../components/color-slider/src/motion.rs" => {
            include_str!("../../../components/color-slider/src/motion.rs")
        }
        "../../crates/ui-components/src/lib.rs" => {
            include_str!("../../../crates/ui-components/src/lib.rs")
        }
        "../../crates/ui-components/src/css.rs" => {
            include_str!("../../../crates/ui-components/src/css.rs")
        }
        "../../crates/ui-components/src/root.rs" => {
            include_str!("../../../crates/ui-components/src/root.rs")
        }
        "../../components/color-editor/Cargo.toml" => {
            include_str!("../../../components/color-editor/Cargo.toml")
        }
        "Cargo.toml" => include_str!("../../../crates/ui-components/Cargo.toml"),
        "legacy_semantics" => {
            include_str!("../../../components/color-editor/test/color_editor_semantics.rs")
        }
        _ => panic!("unsupported source path: {rel_path}"),
    }
}

#[test]
fn color_editor_semantics_tests_are_migrated_to_component_directory() {
    let lib_source = load_source("../../components/color-editor/src/lib.rs");
    let mod_source = load_source("../../components/color-editor/src/mod.rs");
    let legacy_semantics = load_source("legacy_semantics");
    let local_semantics = include_str!("semantics.rs");

    for source in [lib_source, mod_source] {
        assert!(
            source.contains("#[path = \"../test/semantics.rs\"]")
                && source.contains("mod semantics_tests;"),
            "color-editor should wire `components/color-editor/test/semantics.rs` from both lib/mod entrypoints.",
        );
    }

    assert!(
        legacy_semantics.contains("../../../components/color-editor/test/semantics.rs"),
        "legacy ui-components semantics entry should include migrated component semantics file.",
    );
    assert!(
        local_semantics
            .contains("color_editor_semantics_tests_are_migrated_to_component_directory"),
        "component-local semantics suite should provide migration coverage.",
    );
}

#[test]
fn color_editor_public_surface_does_not_expose_dom_platform_types() {
    let mod_source = load_source("../../components/color-editor/src/mod.rs");
    let lib_source = load_source("../../components/color-editor/src/lib.rs");

    for forbidden in [
        "web_sys::",
        "web-sys",
        "wasm_bindgen",
        "JsValue",
        "HtmlElement",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "color-editor public module should not expose `{forbidden}`.",
        );
        assert!(
            !lib_source.contains(forbidden),
            "color-editor crate entry should not expose `{forbidden}`.",
        );
    }
}

#[test]
fn color_editor_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/color-editor/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorEditor internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_editor_uses_logic_state_model() {
    let logic_source = load_source("../../components/color-editor/src/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/color_editor.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");

    for needle in [
        "pub use ui_state_primitives::color_editor::{",
        "DEFAULT_LABEL",
        "sanitize_color",
        "sanitize_hue",
        "sanitize_alpha",
        "sanitize_area",
        "compose_color_from_hsb",
        "format_channel_preview",
        "resolve_state",
        "compose_class_name",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorEditor logic should re-export `{needle}` from ui-state-primitives."
        );
    }

    for needle in [
        "pub enum ColorEditorFormat",
        "pub struct ColorEditorStateInput",
        "pub struct ColorEditorState",
        "pub fn sanitize_color(",
        "pub fn sanitize_hue(",
        "pub fn sanitize_alpha(",
        "pub fn sanitize_area(",
        "pub fn hsb_to_rgb(",
        "pub fn hsb_to_hsl(",
        "pub fn compose_color_from_hsb(",
        "pub fn format_channel_preview(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ColorEditor primitives should include `{needle}` in ui-state-primitives."
        );
    }

    for needle in [
        "overlay_open::use_controllable_state(",
        "use_roving_tabindex(RovingTabIndexOptions {",
        "tabs_list_a11y_attrs(",
        "tabs_tab_a11y_attrs(",
        "let selected_state =",
        "let format_state =",
        "logic::resolve_state(ColorEditorStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "<ColorArea",
        "<ColorSlider",
        "<ColorField",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorEditor view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn color_editor_exposes_baseline_style_data_markers() {
    let source = load_source("../../components/color-editor/src/view.rs");

    for attr in [
        "data-slot=\"color-editor\"",
        "data-state=move || state.get().data_state_attr",
        "data-format=move || state.get().format_attr",
        "data-alpha=move || state.get().alpha_visibility_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-slot=\"color-editor-canvas\"",
        "data-slot=\"color-editor-sliders\"",
        "data-slot=\"color-editor-formats\"",
        "data-slot=\"color-editor-format-button\"",
        "data-slot=\"color-editor-channels\"",
        "data-slot=\"color-editor-channel-row\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorEditor should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn color_editor_semantics_contract_exposes_role_aria_and_source_markers() {
    let source = load_source("../../components/color-editor/src/view.rs");

    for attr in [
        "role=\"group\"",
        "aria-label=move || aria_label.get_value()",
        "aria-labelledby=label_id_for_root",
        "role=tabs_list_role",
        "aria-label=tabs_list_aria_label.clone()",
        "role=tab_role",
        "aria-controls=tab_aria_controls",
        "aria-selected=move || tab_aria_selected.get()",
        "aria-disabled=tab_aria_disabled",
        "role=\"tabpanel\"",
        "aria-labelledby=move || active_tab_id.get()",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "ColorEditor semantics contract should expose `{attr}`."
        );
    }
}

#[test]
fn color_editor_semantics_matrix_covers_state_and_interaction_branches() {
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "let selected_state = overlay_open::use_controllable_state(",
        "let format_state =",
        "selected_color: Option<Signal<Option<String>>>",
        "default_selected_color: Option<String>",
        "format: Option<Signal<ColorEditorFormat>>",
        "default_format: Option<ColorEditorFormat>",
        "is_disabled: bool,",
        "on:keydown=move |ev: ev::KeyboardEvent| {",
        "roving_on_key_down.run(ev.key())",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "<ColorArea",
        "<ColorSlider",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorEditor view should keep semantics matrix branch `{needle}`."
        );
    }

    for needle in [
        "title=\"Controlled Color + Controlled Format\"",
        "title=\"Disabled + Alpha Hidden + Reduced Motion\"",
        "selected_color=selected_color_signal",
        "default_selected_color=\"#0ea5e9\".to_string()",
        "is_disabled=true",
    ] {
        assert!(
            docs_source.contains(needle),
            "ColorEditor docs should keep matrix example `{needle}`."
        );
    }
}

#[test]
fn color_editor_styles_include_format_disabled_alpha_and_custom_contracts() {
    let source = load_source("../../components/color-editor/src/styles.rs");

    for selector in [
        ".ui-color-editor",
        ".ui-color-editor__canvas",
        ".ui-color-editor__sliders",
        ".ui-color-editor__format-button",
        ".ui-color-editor__channels",
        ".ui-color-editor--format-hex .ui-color-editor__channels",
        ".ui-color-editor--disabled",
        ".ui-color-editor[data-disabled=\"true\"]",
        ".ui-color-editor--alpha-hidden .ui-color-editor__slider--alpha",
        ".ui-color-editor[data-alpha=\"hidden\"] .ui-color-editor__slider--alpha",
        ".ui-color-editor--custom-class",
        ".ui-color-editor[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ColorEditor styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn color_editor_styles_consume_ui_theme_tokens_with_component_scoped_aliases() {
    let source = load_source("../../components/color-editor/src/styles.rs");

    for token in [
        "var(--ui-space-sm)",
        "var(--ui-font-size-150",
        "var(--ui-fg)",
        "var(--ui-fg-muted)",
        "var(--ui-border)",
        "var(--ui-accent)",
        "var(--ui-radius-sm)",
    ] {
        assert!(
            source.contains(token),
            "ColorEditor styles should consume ui-theme token `{token}`.",
        );
    }

    for required in [
        "--ui-color-editor-space-sm: var(--ui-space-sm, var(--ui-fallback-space-sm));",
        "--ui-color-editor-fg: var(--ui-fg, var(--ui-fallback-fg));",
        "--ui-color-editor-accent: var(--ui-accent, var(--ui-fallback-accent));",
    ] {
        assert!(
            source.contains(required),
            "ColorEditor should keep component-scoped alias sourced from ui-theme token `{required}`.",
        );
    }

    for forbidden in ["--color-editor-token-", "var(--color-editor-token-"] {
        assert!(
            !source.contains(forbidden),
            "ColorEditor should not introduce parallel token namespace `{forbidden}`.",
        );
    }
}

#[test]
fn color_editor_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("../../components/color-editor/src/styles.rs");
    let check2_source = load_source("../../components/color-editor/check2.md");
    let theme_css_source = include_str!("../../../crates/ui-theme/src/css.rs");
    let script_source = include_str!("../../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "--ui-color-editor-space-sm: var(--ui-space-sm, var(--ui-fallback-space-sm));",
        "--ui-color-editor-border-width: var(--ui-border-width, var(--ui-fallback-border-width));",
        "--ui-color-editor-border: var(--ui-border, var(--ui-fallback-border));",
        "--ui-color-editor-bg: var(--ui-bg, var(--ui-fallback-bg));",
        "--ui-color-editor-fg: var(--ui-fg, var(--ui-fallback-fg));",
        "--ui-color-editor-fg-muted: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        "--ui-color-editor-accent: var(--ui-accent, var(--ui-fallback-accent));",
        "--ui-color-editor-font-size-150: var(--ui-font-size-150, var(--ui-fallback-font-size-150));",
        "--ui-color-editor-line-height-150: var(--ui-line-height-150, var(--ui-fallback-line-height-150));",
        "--ui-color-editor-font-size-100: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
        "--ui-color-editor-line-height-100: var(--ui-line-height-100, var(--ui-fallback-line-height-100));",
        "--ui-color-editor-disabled-opacity: var(--ui-checkbox-disabled-opacity, var(--ui-fallback-checkbox-disabled-opacity));",
        "--ui-color-editor-focus-outline-width: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width));",
        "--ui-color-editor-letter-spacing: var(--ui-command-group-heading-letter-spacing, var(--ui-fallback-command-group-heading-letter-spacing));",
    ] {
        assert!(
            styles_source.contains(required),
            "color-editor styles should keep defensive fallback chain token `{required}`.",
        );
    }

    for required in [
        "--ui-fallback-space-sm:",
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
        "--ui-fallback-checkbox-disabled-opacity:",
        "--ui-fallback-button-focus-outline-width:",
        "--ui-fallback-command-group-heading-letter-spacing:",
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
        "gap: 2px;",
        "minmax(11rem, 12rem)",
        "minmax(4.5rem, 1fr)",
        "minmax(8rem, 1fr)",
        "@media (max-width: 48rem)",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "color-editor styles should not keep raw terminal token `{forbidden}`.",
        );
    }

    let has_hex_literal = styles_source
        .as_bytes()
        .windows(2)
        .any(|pair| pair[0] == b'#' && (pair[1] as char).is_ascii_hexdigit());
    assert!(
        !has_hex_literal,
        "color-editor styles should not hardcode hex colors; use theme variables/fallback chain.",
    );

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "color_editor_styles_use_defensive_variable_fallback_chain",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep defensive-variable governance marker `{required}`.",
        );
    }
}

#[test]
fn color_editor_cascade_layer_and_runtime_style_contract_is_enforced() {
    let check2_source = load_source("../../components/color-editor/check2.md");
    let script_source = include_str!("../../../scripts/check-ui-components-contract-hygiene.sh");
    let css_source = load_source("../../crates/ui-components/src/css.rs");
    let root_source = load_source("../../crates/ui-components/src/root.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");
    let motion_source = load_source("../../components/color-editor/src/motion.rs");

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-color_editor\")]",
        "out.push_str(crate::color::editor::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_source.contains(required),
            "ui-components css aggregation should keep cascade-layer marker `{required}`.",
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

    for source in [view_source, logic_source, motion_source] {
        for forbidden in [
            " style=",
            "\tstyle=",
            "\nstyle=",
            "style=move ||",
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
        ] {
            assert!(
                !source.contains(forbidden),
                "ColorEditor runtime style path should avoid non-variable inline style marker `{forbidden}`.",
            );
        }
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "color_editor_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep cascade-layer governance marker `{required}`.",
        );
    }
}

#[test]
fn color_editor_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "pub(super) fn color_editor() -> AnyView",
        "title=\"ColorEditor\"",
        "slug=\"color-editor\"",
        "Playground title=\"Hello World（默认路径）\" code_signal=hello_code",
        "Playground title=\"State Matrix\" code_signal=state_matrix_code",
        "title=\"Controlled vs Uncontrolled\"",
        "code_signal=controlled_vs_uncontrolled_code",
        "Playground title=\"Streaming Optional / Snapshot\" code_signal=output_mode_code",
        "data-ui-streaming=\"optional\"",
        "data-ui-fallback=\"snapshot\"",
        "data-ui-output-state=\"snapshot\"",
        "title=\"Interactive Playground\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "title=\"Controlled Color + Controlled Format\"",
        "title=\"Disabled + Alpha Hidden + Reduced Motion\"",
        "Source-first / Copy-Paste Ready",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "\"code_imports\"",
        "components/color-editor/src/view.rs",
        "components/color-editor/src/logic.rs",
        "components/color-editor/src/styles.rs",
    ] {
        assert!(
            source.contains(needle),
            "color-editor docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn color_editor_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "data-slot=\"color-editor-state-matrix\"",
        "id_base=\"docs-color-editor-matrix-ready\".to_string()",
        "id_base=\"docs-color-editor-matrix-empty\".to_string()",
        "id_base=\"docs-color-editor-matrix-disabled\".to_string()",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"color-editor-controlled-vs-uncontrolled\"",
        "id_base=\"docs-color-editor-controlled\".to_string()",
        "id_base=\"docs-color-editor-uncontrolled\".to_string()",
        "title=\"Streaming Optional / Snapshot\"",
        "data-slot=\"color-editor-output-mode\"",
        "id_base=\"docs-color-editor-snapshot\".to_string()",
        "label=\"Snapshot fallback\".to_string()",
        "data-slot=\"color-editor-copy-ready\"",
        "Source-first / Copy-Paste Ready",
        "id_base=\"docs-color-editor-workbench\".to_string()",
        "id_base=\"docs-color-editor-workbench-compare\".to_string()",
        "options=workbench_format_options.clone()",
        "<Switch checked=workbench_disabled set_checked=set_workbench_disabled>",
        "<Switch checked=workbench_hide_alpha set_checked=set_workbench_hide_alpha>",
        "<Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>",
        "<Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>",
        "<Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>",
        "Comparison (Disabled + Alpha Hidden)",
        "<Playground title=\"Controlled Color + Controlled Format\" code_signal=basic_code>",
        "id_base=\"docs-color-editor-basic\".to_string()",
        "selected_color=selected_color_signal",
        "on_selected_change=on_selected_change",
        "format=format_signal",
        "on_format_change=on_format_change",
        "<Playground title=\"Disabled + Alpha Hidden + Reduced Motion\" code_signal=states_code>",
        "id_base=\"docs-color-editor-disabled\".to_string()",
        "default_selected_color=\"#0ea5e9\".to_string()",
        "default_format=ColorEditorFormat::Rgb",
        "is_alpha_channel_hidden=true",
        "is_disabled=true",
        "class_name=\"docs-color-editor-custom\".to_string()",
        "id_base=\"docs-color-editor-motion\".to_string()",
        "default_format=ColorEditorFormat::Hsb",
        "default_hue=282.0",
        "default_alpha=64.0",
        "default_area=(0.46, 0.88)",
        "motion=reduced_motion",
    ] {
        assert!(
            source.contains(needle),
            "color-editor docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn color_editor_api_naming_contract_uses_is_on_default_prefixes() {
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let readme_source = load_source("../../components/color-editor/src/README.md");

    for required in [
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional)] is_alpha_channel_hidden: bool,",
        "#[prop(optional, into)] default_selected_color: Option<String>,",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] on_format_change: Option<Callback<ColorEditorFormat>>",
    ] {
        assert!(
            view_source.contains(required),
            "ColorEditor API should keep naming contract marker `{required}`.",
        );
    }

    for forbidden in [
        "#[prop(optional)] disabled: bool,",
        "#[prop(optional)] hide_alpha_channel: bool,",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ColorEditor API should not keep legacy bool alias `{forbidden}`.",
        );
    }

    for required in ["is_disabled=true", "is_alpha_channel_hidden=true"] {
        assert!(
            docs_source.contains(required),
            "ColorEditor docs examples should use normalized prop `{required}`.",
        );
    }

    for required in [
        "| `is_disabled` | `bool` | `false` |",
        "| `is_alpha_channel_hidden` | `bool` | `false` |",
        "## API 命名迁移",
    ] {
        assert!(
            readme_source.contains(required),
            "ColorEditor README should document naming contract marker `{required}`.",
        );
    }
}

#[test]
fn color_editor_docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults() {
    let check2_source = load_source("../../components/color-editor/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let script_source = include_str!("../../../scripts/check-ui-components-contract-hygiene.sh");

    assert!(
        check2_source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "checklist should mark docs/examples/parameter-matrix/state-matrix sync item complete.",
    );

    for required in [
        "Playground title=\"Hello World（默认路径）\" code_signal=hello_code",
        "Playground title=\"State Matrix\" code_signal=state_matrix_code",
        "data-slot=\"color-editor-state-matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"color-editor-controlled-vs-uncontrolled\"",
        "title=\"Interactive Playground\"",
        "id_base=\"docs-color-editor-workbench-format\".to_string()",
        "<Switch checked=workbench_disabled set_checked=set_workbench_disabled>",
        "<Switch checked=workbench_hide_alpha set_checked=set_workbench_hide_alpha>",
        "<Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>",
        "<Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>",
        "<Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>",
        "selected_color=workbench_selected_color_signal",
        "on_selected_change=on_workbench_selected_change",
        "format=workbench_format_signal",
        "on_format_change=on_workbench_format_change",
        "is_alpha_channel_hidden=workbench_hide_alpha.get()",
        "is_disabled=workbench_disabled.get()",
    ] {
        assert!(
            docs_source.contains(required),
            "docs page should keep docs/examples/parameter-matrix marker `{required}`.",
        );
    }

    for required in [
        "pub struct ColorEditorDefaultInput {",
        "pub fn normalize_default_inputs(input: ColorEditorDefaultInput) -> ColorEditorDefaultState {",
        "let default_selected_color = sanitize_color(input.default_selected_color);",
        "let default_format = input.default_format.unwrap_or_default();",
        "let default_hue = sanitize_hue(input.default_hue.unwrap_or(DEFAULT_HUE));",
        "let default_alpha = sanitize_alpha(input.default_alpha.unwrap_or(DEFAULT_ALPHA));",
        "let default_area = sanitize_area(input.default_area.unwrap_or(DEFAULT_AREA));",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep default/normalization contract marker `{required}`.",
        );
    }

    for required in [
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional, into)] default_selected_color: Option<String>,",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>,",
        "#[prop(optional)] format: Option<Signal<ColorEditorFormat>>,",
        "#[prop(optional)] default_format: Option<ColorEditorFormat>,",
        "#[prop(optional)] on_format_change: Option<Callback<ColorEditorFormat>>,",
        "#[prop(optional)] is_alpha_channel_hidden: bool,",
        "let normalized_defaults = logic::normalize_default_inputs(logic::ColorEditorDefaultInput {",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep API + default normalization wiring marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_docs_examples_parameter_and_state_matrix_stay_synced_with_logic_defaults";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_editor_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("../../components/color-editor/check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "color-editor checklist should keep documentation-as-product rule `{required}`.",
        );
    }
}

#[test]
fn color_editor_docs_entry_exists_as_readme_or_equivalent_docs_app_page() {
    let readme_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../components/color-editor/src/README.md");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    assert!(
        readme_path.exists(),
        "color-editor should provide README as documentation entry.",
    );
    assert!(
        docs_page_source.contains("pub(super) fn color_editor() -> AnyView"),
        "docs-app should expose color_editor docs entry function.",
    );
}

#[test]
fn color_editor_docs_are_beginner_friendly_with_default_then_advanced_path() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let readme_source = load_source("../../components/color-editor/src/README.md");
    let script_source = include_str!("../../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "title=\"ColorEditor\"",
        "slug=\"color-editor\"",
        "Playground title=\"Hello World（默认路径）\" code_signal=hello_code",
        "Playground title=\"State Matrix\" code_signal=state_matrix_code",
        "Playground title=\"Controlled vs Uncontrolled\" code_signal=controlled_vs_uncontrolled_code",
        "Playground title=\"Interactive Playground\"",
    ] {
        assert!(
            docs_source.contains(required),
            "color-editor docs should keep beginner-to-advanced marker `{required}`.",
        );
    }

    let hello_pos = docs_source
        .find("Playground title=\"Hello World（默认路径）\" code_signal=hello_code")
        .expect("docs should include hello-world playground for zero-threshold path.");
    let matrix_pos = docs_source
        .find("Playground title=\"State Matrix\" code_signal=state_matrix_code")
        .expect("docs should include state-matrix playground as common usage.");
    let controlled_pos = docs_source
        .find("Playground title=\"Controlled vs Uncontrolled\" code_signal=controlled_vs_uncontrolled_code")
        .expect("docs should include controlled-vs-uncontrolled playground.");
    let interactive_pos = docs_source
        .find("Playground title=\"Interactive Playground\"")
        .expect("docs should include interactive playground for advanced controls.");
    assert!(
        hello_pos < matrix_pos && matrix_pos < controlled_pos && controlled_pos < interactive_pos,
        "docs should present default usage before advanced controls.",
    );

    for required in [
        "## Hello World（最小可用）",
        "默认路径无需手动接线 `ui-state-primitives` / `ui-headless`。",
        "进阶需求再按需开启受控值、格式切换、alpha 隐藏、motion 与 locale。",
        "## Docs Playground（展示 / Config / Code / CSS Test）",
    ] {
        assert!(
            readme_source.contains(required),
            "README should keep beginner-friendly guidance marker `{required}`.",
        );
    }

    let readme_hello_pos = readme_source
        .find("## Hello World（最小可用）")
        .expect("README should include hello-world section.");
    let readme_advanced_pos = readme_source
        .find("## Docs Playground（展示 / Config / Code / CSS Test）")
        .expect("README should include advanced playground section.");
    assert!(
        readme_hello_pos < readme_advanced_pos,
        "README should present default path before advanced playground details.",
    );

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_docs_are_beginner_friendly_with_default_then_advanced_path";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_editor_readme_documents_docs_workbench_contract() {
    let source = load_source("../../components/color-editor/src/README.md");

    for needle in [
        "## Docs Playground（展示 / Config / Code / CSS Test）",
        "forms_color.rs` 中 `color_editor()`",
        "展示（Preview）",
        "Config：`test_config_signal`",
        "Code：`code_signal`",
        "CSS Test：`test_css_source`",
        "Controlled Color + Controlled Format",
        "Disabled + Alpha Hidden + Reduced Motion",
    ] {
        assert!(
            source.contains(needle),
            "color_editor README should include docs-playground marker `{needle}`.",
        );
    }
}

#[test]
fn color_editor_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("../../components/color-editor/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(required),
            "color-editor checklist should keep interactive-playground rule `{required}`.",
        );
    }
}

#[test]
fn color_editor_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for marker in [
        "pub(super) fn color_editor() -> AnyView",
        "title=\"Interactive Playground\"",
        "description=\"展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "controls=move || view!",
        "<Switch checked=workbench_disabled set_checked=set_workbench_disabled>",
        "<Switch checked=workbench_hide_alpha set_checked=set_workbench_hide_alpha>",
        "<Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>",
        "<Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>",
        "<Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>",
        "id_base=\"docs-color-editor-workbench\".to_string()",
        "id_base=\"docs-color-editor-workbench-compare\".to_string()",
        "on_selected_change=on_workbench_selected_change",
        "on_format_change=on_workbench_format_change",
        "workbench_format.get().as_attr()",
    ] {
        assert!(
            docs_source.contains(marker),
            "color-editor docs should keep interactive playground marker `{marker}`.",
        );
    }

    for marker in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(marker),
            "docs-app Playground should keep interactive preview contract marker `{marker}`.",
        );
    }
}

#[test]
fn color_editor_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_editor_contract.spec.mjs");

    for marker in [
        "docs-app color-editor key flow is repeatable and failures map to semantic breakpoints",
        "await page.goto(COLOR_EDITOR_PAGE);",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-component=\"color-editor\"] #docs-color-editor-controlled[data-slot=\"color-editor\"]",
        "await page.keyboard.press(\"ArrowRight\");",
        "await expect(root).toHaveAttribute(\"data-format\", \"hsb\");",
        "await expect(root).toHaveAttribute(\"data-ui-output-status\", \"submittable\");",
        "await page.reload();",
        "await expect(root).toHaveAttribute(\"data-ui-action\", \"snapshot-render\");",
        "await expect(root).toHaveAttribute(\"data-ui-output-status\", \"verified\");",
    ] {
        assert!(
            e2e_source.contains(marker),
            "color-editor interactive playground should keep repeatable semantic e2e marker `{marker}`.",
        );
    }
}

#[test]
fn color_editor_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for marker in [
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(marker),
            "DX check script should enforce interactive-playground contract marker `{marker}`.",
        );
    }
}

#[test]
fn color_editor_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("../../components/color-editor/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(required),
            "color-editor checklist should keep source-first copy-paste-ready rule `{required}`.",
        );
    }
}

#[test]
fn color_editor_docs_are_copy_paste_ready_with_imports_copy_button_and_sync() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_logic_source = include_str!("../../../components/code-block/src/logic.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");

    for marker in [
        "pub(super) fn color_editor() -> AnyView",
        "data-slot=\"color-editor-copy-ready\"",
        "<h3>\"Source-first / Copy-Paste Ready\"</h3>",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "\"code_imports\"",
        "components/color-editor/src/mod.rs",
        "components/color-editor/src/logic.rs",
        "components/color-editor/src/view.rs",
        "components/color-editor/src/styles.rs",
        "components/color-editor/src/motion.rs",
        "data-slot=\"color-editor-source-prerequisites\"",
        "\"component-color_editor\"",
        "\"inject-css\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "color-editor docs should keep source-first copy-ready marker `{marker}`.",
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
        "#[prop(optional, into)] default_selected_color: Option<String>,",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>,",
        "#[prop(optional)] default_format: Option<ColorEditorFormat>,",
        "#[prop(optional)] on_format_change: Option<Callback<ColorEditorFormat>>,",
        "pub fn normalize_default_inputs(input: ColorEditorDefaultInput) -> ColorEditorDefaultState {",
    ] {
        assert!(
            view_source.contains(marker) || logic_source.contains(marker),
            "color-editor source-first snippets should stay synced with implementation marker `{marker}`.",
        );
    }
}

#[test]
fn color_editor_contract_hygiene_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_docs_are_copy_paste_ready_with_imports_copy_button_and_sync",
    ] {
        assert!(
            script_source.contains(marker),
            "contract-hygiene script should enforce source-first copy-ready marker `{marker}`.",
        );
    }
}

#[test]
fn color_editor_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");

    for marker in [
        "### ColorEditor 同步记录（2026-02-20）",
        "selected_color + on_selected_change + default_selected_color",
        "format + on_format_change + default_format",
        "is_disabled / is_alpha_channel_hidden",
        "component_doc!(\"ColorEditor\", \"color-editor\", \"Forms\", forms_color::color_editor)",
        "apps/docs-app/src/pages/components/pages/forms_color.rs::color_editor()",
        "Hello World（默认路径）",
        "State Matrix",
        "Controlled vs Uncontrolled",
        "Interactive Playground",
        "Source-first / Copy-Paste Ready",
        "compose_copy_ready_code",
        "参数语义若变更，必须先同步本策略文档与 docs 入口",
    ] {
        assert!(
            strategy_source.contains(marker) || docs_index_source.contains(marker),
            "color-editor HeroUI/doc sync record should include `{marker}`.",
        );
    }

    for marker in [
        "pub(super) fn color_editor() -> AnyView",
        "title=\"ColorEditor\"",
        "slug=\"color-editor\"",
        "data-slot=\"color-editor-copy-ready\"",
    ] {
        assert!(
            docs_page_source.contains(marker),
            "color-editor docs entry should keep indexable marker `{marker}`.",
        );
    }

    for marker in [
        "#[prop(optional)] selected_color: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_selected_color: Option<String>,",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] format: Option<Signal<ColorEditorFormat>>",
        "#[prop(optional)] default_format: Option<ColorEditorFormat>",
        "#[prop(optional)] on_format_change: Option<Callback<ColorEditorFormat>>",
        "#[prop(optional)] is_disabled: bool,",
        "#[prop(optional)] is_alpha_channel_hidden: bool,",
        "pub fn normalize_default_inputs(input: ColorEditorDefaultInput) -> ColorEditorDefaultState {",
    ] {
        assert!(
            view_source.contains(marker) || logic_source.contains(marker),
            "color-editor parameter model marker `{marker}` should remain in implementation.",
        );
    }
}

#[test]
fn color_editor_check2_marks_heroui_strategy_and_component_docs_sync_complete() {
    let check2_source = load_source("../../components/color-editor/check2.md");

    for marker in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        "color_editor_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
    ] {
        assert!(
            check2_source.contains(marker),
            "color-editor checklist should keep HeroUI/doc sync completion evidence `{marker}`.",
        );
    }
}

#[test]
fn color_editor_contract_hygiene_script_covers_heroui_strategy_doc_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_check2_marks_heroui_strategy_and_component_docs_sync_complete",
    ] {
        assert!(
            script_source.contains(marker),
            "contract-hygiene script should enforce HeroUI/doc sync marker `{marker}`.",
        );
    }
}

#[test]
fn color_editor_feature_dependency_chain_covers_composed_children() {
    let source = load_source("Cargo.toml");

    assert!(
        source.contains("component-color_editor = ["),
        "ColorEditor feature should use an explicit dependency list."
    );

    for dependency in [
        "\"component-color_area\"",
        "\"component-color_field\"",
        "\"component-color_slider\"",
        "\"component-color_swatch\"",
        "\"component-slider\"",
    ] {
        assert!(
            source.contains(dependency),
            "ColorEditor feature dependency chain should include `{dependency}`."
        );
    }
}

#[test]
fn color_editor_tree_shaking_feature_pruning_is_gated_in_lib_and_css() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("../../crates/ui-components/src/lib.rs");
    let css_source = load_source("../../crates/ui-components/src/css.rs");

    assert!(
        cargo_source.contains("component-color_editor = ["),
        "ui-components feature graph should register component-color_editor explicitly.",
    );
    assert!(
        cargo_source.contains("all-components = [")
            && cargo_source.contains("\"component-color_editor\","),
        "all-components aggregate should include color_editor only through explicit feature list.",
    );

    let color_export_idx = lib_source
        .find("pub use crate::color_editor as editor;")
        .expect("ui-components lib.rs should expose color::editor namespace when feature enabled.");
    let export_prefix_start = color_export_idx.saturating_sub(128);
    let export_prefix = &lib_source[export_prefix_start..color_export_idx];
    assert!(
        export_prefix.contains("#[cfg(feature = \"component-color_editor\")]"),
        "color::editor re-export in lib.rs must stay behind `component-color_editor` gate.",
    );

    let module_idx = lib_source
        .find("pub mod color_editor;")
        .expect("ui-components lib.rs should declare color_editor module.");
    let module_prefix_start = module_idx.saturating_sub(128);
    let module_prefix = &lib_source[module_prefix_start..module_idx];
    assert!(
        module_prefix.contains("#[cfg(feature = \"component-color_editor\")]"),
        "color_editor module in lib.rs must stay behind `component-color_editor` gate.",
    );

    let color_css_idx = css_source
        .find("out.push_str(crate::color::editor::styles::CSS);")
        .expect("ui-components css.rs should aggregate color_editor CSS when feature is enabled.");
    let css_prefix_start = color_css_idx.saturating_sub(128);
    let css_prefix = &css_source[css_prefix_start..color_css_idx];
    assert!(
        css_prefix.contains("#[cfg(feature = \"component-color_editor\")]"),
        "color_editor CSS aggregation in css.rs must stay behind `component-color_editor` gate.",
    );

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "css aggregation entry should preserve tree-shaking guard marker `{needle}`.",
        );
    }
}

#[test]
fn color_editor_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let script_source = load_source("../../scripts/check-ui-components-tree-shaking.sh");

    for needle in [
        "COLOR_EDITOR_MIN_FEATURES=\"component-color_editor,inject-css\"",
        "COLOR_EDITOR_TREE_OUTPUT=\"$(cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$COLOR_EDITOR_MIN_FEATURES\")\"",
        "missing command-line feature: component-color_editor",
        "missing command-line feature: inject-css for color-editor minimal tree",
        "color-editor minimal feature tree should not pull all-components",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$COLOR_EDITOR_MIN_FEATURES\"",
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_tree_shaking_feature_pruning_is_gated_in_lib_and_css",
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking script should enforce `{needle}`.",
        );
    }
}

#[test]
fn color_editor_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_source("../../components/color-editor/check2.md");

    assert!(
        check2_source.contains("- [x] Tree Shaking & 特性剪裁："),
        "color-editor/check2.md should mark tree-shaking feature-pruning item complete.",
    );

    for needle in [
        "component-color_editor",
        "#[cfg(feature = \"component-color_editor\")] pub use crate::color_editor as editor;",
        "out.push_str(crate::color::editor::styles::CSS);",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-color_editor,inject-css",
        "scripts/check-ui-components-tree-shaking.sh",
        "color_editor_tree_shaking_feature_pruning_is_gated_in_lib_and_css",
    ] {
        assert!(
            check2_source.contains(needle),
            "color-editor/check2.md tree-shaking evidence should reference `{needle}`.",
        );
    }
}

#[test]
fn color_editor_component_has_motion_contract_module() {
    let mod_source = load_source("../../components/color-editor/src/mod.rs");
    let motion_source = load_source("../../components/color-editor/src/motion.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");

    for needle in ["pub mod motion;", "pub use motion::ColorEditorMotion;"] {
        assert!(
            mod_source.contains(needle),
            "ColorEditor mod.rs should export motion contract marker `{needle}`."
        );
    }

    for needle in [
        "pub use crate::color_slider::ColorSliderMotion as ColorEditorMotion;",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "pub fn source_attr(",
    ] {
        assert!(
            motion_source.contains(needle),
            "ColorEditor motion contract should contain `{needle}`."
        );
    }

    assert!(
        view_source.contains("motion_contract::attach_motion(motion)"),
        "ColorEditor view should consume motion contract attach path."
    );
}

#[test]
fn color_editor_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let motion_source = load_source("../../components/color-editor/src/motion.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let slider_motion_source = load_source("../../components/color-slider/src/motion.rs");
    let ui_motion_spring_source = load_source("../../crates/ui-motion/src/spring.rs");
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let platform_script = load_source("../../scripts/check-ui-components-platforms.sh");
    let check2_source = load_source("../../components/color-editor/check2.md");

    for required in [
        "pub use crate::color_slider::ColorSliderMotion as ColorEditorMotion;",
        "pub fn sanitize_motion(motion: ColorEditorMotion) -> ColorEditorMotion {",
        "crate::color_slider::motion::sanitize_motion(motion)",
        "pub fn attach_motion(motion: ColorEditorMotion) -> ColorEditorMotion {",
        "sanitize_motion(motion)",
    ] {
        assert!(
            motion_source.contains(required),
            "color-editor motion contract should include `{required}`."
        );
    }

    assert!(
        view_source
            .contains("let motion = StoredValue::new(motion_contract::attach_motion(motion));"),
        "color-editor view should wire motion via component motion contract attach path.",
    );

    for required in [
        "pub struct ColorSliderMotion {",
        "stiffness: tokens.spring.stiffness,",
        "damping: tokens.spring.damping,",
        "pub fn sanitize_motion(motion: ColorSliderMotion) -> ColorSliderMotion {",
        "#[cfg(target_arch = \"wasm32\")]",
        "pub fn attach_motion(",
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion() {",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "_motion: ColorSliderMotion,",
    ] {
        assert!(
            slider_motion_source.contains(required),
            "color-slider motion backend contract should keep `{required}` for color-editor composition.",
        );
    }

    for required in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
    ] {
        assert!(
            ui_motion_spring_source.contains(required),
            "ui-motion spring should keep reduced-motion contract marker `{required}`.",
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
            "ui-motion wasm/non-wasm split should keep marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        platform_script.contains(script_needle),
        "platform gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "color_editor_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep motion contractualization marker `{required}`.",
        );
    }
}

#[test]
fn color_editor_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let check2_source = load_source("../../components/color-editor/check2.md");
    let ui_components_lib = load_source("../../crates/ui-components/src/lib.rs");
    let ui_components_css = load_source("../../crates/ui-components/src/css.rs");
    let ui_components_root = load_source("../../crates/ui-components/src/root.rs");
    let active_highlight =
        include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
    let controllable_state = include_str!("../../../crates/ui-headless/src/controllable_state.rs");
    let presence = include_str!("../../../crates/ui-headless/src/presence.rs");
    let a11y = include_str!("../../../crates/ui-headless/src/a11y.rs");
    let entrypoints_script = load_source("../../scripts/check-ui-components-entrypoints.sh");

    for required in [
        "#[cfg(feature = \"component-color_editor\")]",
        "pub mod color_editor;",
        "pub mod root;",
        "pub use root::UiRoot;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui-components lib.rs should keep fixed entry marker `{required}`.",
        );
    }

    for forbidden in ["pub use web_sys", "web_sys::", "NodeRef<", "JsValue"] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui-components lib.rs should not leak platform detail `{forbidden}`.",
        );
    }

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-color_editor\")]",
        "out.push_str(crate::color::editor::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui-components css.rs should keep fixed entry marker `{required}`.",
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
            "ui-components root.rs should keep centralized injection marker `{required}`.",
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

    for forbidden in ["ColorEditor", "aria-", "data-state"] {
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
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui-components/src");
    for forbidden_file in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src_dir.join(forbidden_file).exists(),
            "ui-components/src/{forbidden_file} should be absent by fixed-entrypoint contract.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        entrypoints_script.contains(script_needle),
        "entrypoints gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "color_editor_ui_components_fixed_entry_files_follow_layered_boundaries",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep fixed-entrypoint governance marker `{required}`.",
        );
    }
}

#[test]
fn color_editor_component_directory_standard_files_follow_contract_and_na_paths() {
    let check2_source = load_source("../../components/color-editor/check2.md");
    let mod_source = load_source("../../components/color-editor/src/mod.rs");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");
    let styles_source = load_source("../../components/color-editor/src/styles.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let motion_source = load_source("../../components/color-editor/src/motion.rs");
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required_file).exists(),
            "color-editor component directory should include `{required_file}`.",
        );
    }
    for absent_file in ["render.rs", "spec.rs"] {
        assert!(
            !src_dir.join(absent_file).exists(),
            "color-editor component directory should keep `{absent_file}` absent.",
        );
    }

    for required in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::ColorEditorMotion;",
        "pub use view::ColorEditor;",
    ] {
        assert!(
            mod_source.contains(required),
            "mod.rs should keep minimal stable export marker `{required}`.",
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
            "mod.rs should not over-export or drift to `{forbidden}`.",
        );
    }

    for required in [
        "pub struct ColorEditorDefaultInput",
        "pub struct ColorEditorDefaultState",
        "pub fn normalize_default_inputs(",
        "pub struct ColorEditorSelectionInput",
        "pub fn resolve_selected_color(",
        "pub fn resolve_area_change(",
        "pub fn resolve_hue_change(",
        "pub fn resolve_alpha_change(",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalized state derivation marker `{required}`.",
        );
    }
    for forbidden in [
        "web_sys::",
        "window()",
        "document()",
        "NodeRef",
        "HtmlElement",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should stay free of DOM/platform token `{forbidden}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-color-editor[data-disabled=\"true\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first CSS marker `{required}`.",
        );
    }
    for forbidden in ["#[component]", "use ui_headless", "use leptos::"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should avoid render/headless concern `{forbidden}`.",
        );
    }

    for required in [
        "#[component]",
        "overlay_open::use_controllable_state(",
        "use_roving_tabindex(RovingTabIndexOptions {",
        "tabs_list_a11y_attrs(",
        "tabs_tab_a11y_attrs(",
        "logic::resolve_state(ColorEditorStateInput {",
        "view! {",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep render + headless mount marker `{required}`.",
        );
    }
    assert!(
        !src_dir.join("render.rs").exists(),
        "view layer should not drift to render.rs.",
    );

    for required in [
        "pub use crate::color_slider::ColorSliderMotion as ColorEditorMotion;",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "crate::color_slider::motion::sanitize_motion(motion)",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should keep semantic-to-motion contract mapping marker `{required}`.",
        );
    }
    for forbidden in [
        "SpringAnimator::new(",
        "ui_motion::spring::sanitize_config(",
        "requestAnimationFrame",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should avoid re-implementing shared motion engine token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "color_editor_component_directory_standard_files_follow_contract_and_na_paths",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep component-directory governance marker `{required}`.",
        );
    }
}

#[test]
fn color_editor_file_placement_discipline_contract_is_explicit_for_interactive_component_scope() {
    let check2_source = load_source("../../components/color-editor/check2.md");
    let mod_source = load_source("../../components/color-editor/src/mod.rs");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");
    let styles_source = load_source("../../components/color-editor/src/styles.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let motion_source = load_source("../../components/color-editor/src/motion.rs");
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    assert!(
        check2_source.contains("文件落点纪律"),
        "color-editor check2 should explicitly track file-placement discipline gate.",
    );

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required_file).exists(),
            "file-placement discipline requires `{required_file}` in color-editor source directory.",
        );
    }
    for forbidden_file in ["render.rs", "spec.rs"] {
        assert!(
            !src_dir.join(forbidden_file).exists(),
            "color-editor should keep `{forbidden_file}` absent in current scope.",
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
        logic_source.contains("pub fn normalize_default_inputs(")
            && styles_source.contains("pub const CSS: &str =")
            && view_source.contains("#[component]")
            && motion_source
                .contains("pub use crate::color_slider::ColorSliderMotion as ColorEditorMotion;"),
        "logic/styles/view/motion should keep canonical responsibility anchors.",
    );

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_file_placement_discipline_contract_is_explicit_for_interactive_component_scope";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "color_editor_file_placement_discipline_contract_is_explicit_for_interactive_component_scope",
    ] {
        assert!(
            check2_source.contains(required),
            "color-editor check2 should keep file-placement-discipline marker `{required}`.",
        );
    }
}

#[test]
fn color_editor_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component() {
    let check2_source = load_source("../../components/color-editor/check2.md");
    let mod_source = load_source("../../components/color-editor/src/mod.rs");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let styles_source = load_source("../../components/color-editor/src/styles.rs");
    let motion_source = load_source("../../components/color-editor/src/motion.rs");
    let readme_source = load_source("../../components/color-editor/src/README.md");
    let protocol_source = include_str!("../src/protocol.rs");
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    assert!(
        check2_source.contains("Hyper-Structure Builder（`spec.rs`）"),
        "color-editor checklist should explicitly track hyper-structure builder gate.",
    );

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "color-editor is not a complex schema-driven component; spec.rs should remain N/A.",
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "ColorEditorSpec",
        "Spec::new(",
        ".render(",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "color-editor should not expose hyper-structure builder token `{forbidden}` in current scope.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。（N/A：`ColorEditor` 当前不属于复杂 schema 驱动组件，且不存在 `src/spec.rs`；组件仅保留 `mod.rs/logic.rs/styles.rs/view.rs/motion.rs` 职责分层。`protocol.rs` 仅承载最小版本化序列化类型，不暴露 builder API，不替代 `spec.rs`。）",
        "color_editor_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component",
    ] {
        assert!(
            check2_source.contains(required),
            "color-editor checklist should keep hyper-structure-builder marker `{required}`.",
        );
    }
}

#[test]
fn color_editor_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let check2_source = load_source("../../components/color-editor/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let component_manifest = include_str!("../src/Component.toml");
    let component_rbi = include_str!("../src/color_editor.rbi");

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["Component.toml", "color_editor.rbi"] {
        assert!(
            src_dir.join(required_file).exists(),
            "color-editor context-compression file should exist: `{required_file}`.",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "name = \"ColorEditor\"",
        "crate = \"ui-color-editor\"",
        "name = \"selected_color\"",
        "name = \"on_selected_change\"",
        "name = \"default_selected_color\"",
        "name = \"format\"",
        "name = \"on_format_change\"",
        "name = \"default_format\"",
        "name = \"is_disabled\"",
        "name = \"motion\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "color-editor Component.toml should keep context-compression marker `{required}`.",
        );
    }

    for required in [
        "pub type ColorEditorFormat = ui_state_primitives::color_editor::ColorEditorFormat;",
        "pub type ColorEditorState = ui_state_primitives::color_editor::ColorEditorState;",
        "pub type ColorEditorMotion = crate::color_slider::ColorSliderMotion;",
        "pub fn ColorEditor(",
        "selected_color: Option<leptos::prelude::Signal<Option<String>>>",
        "on_selected_change: Option<leptos::prelude::Callback<Option<String>>>",
        "default_selected_color: Option<String>",
        "format: Option<leptos::prelude::Signal<ColorEditorFormat>>",
        "on_format_change: Option<leptos::prelude::Callback<ColorEditorFormat>>",
        "default_format: Option<ColorEditorFormat>",
        "dir: Option<ui_headless::A11yDirection>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            component_rbi.contains(required),
            "color_editor.rbi should keep signature-projection marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。（`components/color-editor/src/Component.toml` 与 `components/color-editor/src/color_editor.rbi` 已同步维护；`Component.toml` 覆盖能力清单与输入输出轴，`.rbi` 提供 `ColorEditor` 接口签名投影，避免 AI 检索漂移。回归由 `components/color-editor/test/semantics.rs::color_editor_context_compression_manifest_and_rbi_projection_are_present_and_current` 覆盖，并接入 `scripts/check-ui-components-component-files.sh` 门禁。）",
        "color_editor_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            check2_source.contains(required),
            "color-editor checklist should keep context-compression marker `{required}`.",
        );
    }
}

#[test]
fn color_editor_agent_contract_is_schema_typed_and_machine_readable() {
    let check2_source = load_source("../../components/color-editor/check2.md");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let component_manifest = include_str!("../src/Component.toml");
    let component_rbi = include_str!("../src/color_editor.rbi");

    for typed_source in [
        "pub const COLOR_EDITOR_AGENT_SCHEMA: &str = \"ui.color-editor.agent-contract\";",
        "pub enum ColorEditorAgentSchemaVersion",
        "pub enum ColorEditorAgentIntent",
        "pub enum ColorEditorAgentAction",
        "pub enum ColorEditorAgentState",
        "pub enum ColorEditorAgentSource",
        "pub struct ColorEditorAgentContract",
        "pub struct ColorEditorAgentContractInput",
        "fn resolve_agent_state(render_state: ColorEditorState) -> ColorEditorAgentState",
        "pub fn resolve_agent_contract(input: ColorEditorAgentContractInput) -> ColorEditorAgentContract",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "color-editor Agent Contract should stay type-derived via `{typed_source}`.",
        );
    }

    for marker in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-selection-source=move || agent_contract.get().selection_source",
        "data-ui-format-source=move || agent_contract.get().format_source",
        "data-ui-motion-source=move || agent_contract.get().motion_source",
        "data-ui-label-source=move || agent_contract.get().label_source",
        "data-ui-aria-source=move || agent_contract.get().aria_source",
        "data-ui-class-source=move || agent_contract.get().class_source",
    ] {
        assert!(
            view_source.contains(marker),
            "color-editor view should mount Agent Contract marker `{marker}`.",
        );
    }

    for required in [
        "name = \"agent-contract-markers\"",
        "name = \"agent_contract_schema_markers\"",
        "[[agent_contract]]",
        "schema = \"ui.color-editor.agent-contract.v1\"",
        "intent = \"color.editing\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-state\"",
        "COLOR_EDITOR_AGENT_SCHEMA",
        "ColorEditorAgentContract",
    ] {
        assert!(
            component_manifest.contains(required) || component_rbi.contains(required),
            "color-editor context-compression assets should keep Agent Contract marker `{required}`.",
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
            "color-editor Agent Contract should avoid free-form schema token `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。（`components/color-editor/src/logic.rs` 新增类型化 Agent Contract（`ColorEditorAgent{SchemaVersion/Intent/Action/State/Source}` + `resolve_agent_contract`），`components/color-editor/src/view.rs` 挂载稳定 `data-ui-schema/data-ui-schema-version/data-ui-intent/data-ui-action/data-ui-state/data-ui-source` 与来源轴标记（`data-ui-*-source`），并在交互回调中更新 action 语义；`components/color-editor/src/Component.toml` 补充 `agent-contract-markers`、`agent_contract_schema_markers`、`[[agent_contract]]` 与 marker 白名单描述。回归由 `components/color-editor/test/semantics.rs::color_editor_agent_contract_is_schema_typed_and_machine_readable` 与 `components/color-editor/test/semantics.rs::color_editor_agent_contract_render_path_is_whitelist_safe_and_script_injection_free` 覆盖，并接入 `scripts/check-ui-components-contract-hygiene.sh` 门禁。）",
        "color_editor_agent_contract_is_schema_typed_and_machine_readable",
    ] {
        assert!(
            check2_source.contains(required),
            "color-editor checklist should keep Agent Contract evidence `{required}`.",
        );
    }
}

#[test]
fn color_editor_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let check2_source = load_source("../../components/color-editor/check2.md");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let component_manifest = include_str!("../src/Component.toml");
    let script_source = include_str!("../../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "allowed = [",
        "\"render_editor_header()\"",
        "\"render_editor_canvas()\"",
        "\"render_editor_sliders()\"",
        "\"render_format_tabs()\"",
        "\"render_channel_rows()\"",
        "\"render_editor_controls()\"",
        "\"<ColorArea>\"",
        "\"<ColorSlider>\"",
        "\"<ColorField>\"",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\", \"eval(\"]",
    ] {
        assert!(
            component_manifest.contains(required),
            "color-editor manifest should keep whitelist-safe render path marker `{required}`.",
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
            "color-editor Agent Contract render path should forbid `{forbidden}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(script_needle),
            "contract-hygiene gate script should include `{script_needle}`.",
        );
    }

    for required in [
        "color_editor_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "白名单能力边界",
    ] {
        assert!(
            check2_source.contains(required),
            "color-editor checklist should keep Agent Contract whitelist evidence `{required}`.",
        );
    }
}

#[test]
fn color_editor_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("../../components/color-editor/check2.md");
    let mod_source = load_source("../../components/color-editor/src/mod.rs");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let styles_source = load_source("../../components/color-editor/src/styles.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。（N/A：`ColorEditor` 不是 LLM 正文渲染组件，组件职责是同步颜色编辑；组件侧不实现 token-by-token streaming 协议，仅消费稳定快照状态输入。术语约束仍固定为两种显示模式定义：`Streaming`（边生成边显示）与 `Snapshot`（完整结果一次性显示），避免在组件层引入第三种“伪流式”定义。回归由 `components/color-editor/test/semantics.rs::color_editor_check2_documents_streaming_definition_is_llm_output_only_with_two_modes` 覆盖，并接入 `scripts/check-ui-components-streaming.sh` 门禁。）",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "N/A：`ColorEditor` 不是 LLM 正文渲染组件",
    ] {
        assert!(
            check2_source.contains(required),
            "color-editor check2 should keep streaming-definition marker `{required}`.",
        );
    }

    for forbidden in ["use_ai_space_state", "project_streaming_"] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !docs_page_source.contains(forbidden),
            "color-editor should stay out of LLM streaming protocol scope and avoid `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_editor_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("../../components/color-editor/check2.md");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。（`ColorEditor` 归类为 `Streaming Optional`；组件职责是颜色编辑而非 LLM 正文阅读面，默认走 `Snapshot` 渲染路径。实现显式输出 `data-ui-stream-support=\"unsupported\"`、`data-ui-stream-fallback=\"full-snapshot\"`、`data-ui-stream-mode=\"snapshot\"` 与 `data-ui-output-status`，并保持 `role/aria/data-*` 连续可读。数据校验、断线恢复、重试策略继续留在上层编排，不下沉到组件。回归由 `components/color-editor/test/semantics.rs::color_editor_check2_documents_streaming_required_optional_classification_rules`、`components/color-editor/test/semantics.rs::color_editor_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、`components/color-editor/test/semantics.rs::color_editor_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer` 覆盖，并接入 `scripts/check-ui-components-streaming.sh` 门禁。）",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "`ColorEditor` 归类为 `Streaming Optional`",
    ] {
        assert!(
            checklist_source.contains(required),
            "color-editor check2 should keep streaming responsibility marker `{required}`.",
        );
    }
}

#[test]
fn color_editor_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("../../components/color-editor/src/view.rs");

    for required in [
        "role=\"group\"",
        "aria-label=move || aria_label.get_value()",
        "aria-labelledby=label_id_for_root",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "data-state=move || state.get().data_state_attr",
        "data-format=move || state.get().format_attr",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "color-editor should keep continuous aria/data semantics via `{required}` in optional-streaming scope.",
        );
    }
}

#[test]
fn color_editor_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");
    let combined = format!("{view_source}\n{logic_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "validate_stream",
        "stream_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "color-editor should keep validation/retry/resilience policy in upper layer; component must not include `{forbidden}`.",
        );
    }

    for needle in [
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn color_editor_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_source("../../components/color-editor/check2.md");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。（`ColorEditor` 已支持完整配置快照输入并稳定渲染：`components/color-editor/src/view.rs` 通过 `logic::normalize_default_inputs` + 双受控轴（`selected_color` 与 `format`）消费完整结果，根节点持续输出稳定语义标记（`data-state/data-format/data-alpha/...`）。docs 基线示例 `apps/docs-app/src/pages/components/pages/forms_color.rs` 提供 Hello World、Controlled、Disabled 等完整快照路径。回归由 `components/color-editor/test/semantics.rs::color_editor_check2_documents_snapshot_as_default_baseline_capability` 与 `components/color-editor/test/semantics.rs::color_editor_snapshot_baseline_consumes_complete_result_and_renders_stably` 覆盖，并接入 `scripts/check-ui-components-streaming.sh` 门禁。）",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            check2_source.contains(required),
            "color-editor check2 should keep snapshot-baseline marker `{required}`.",
        );
    }
}

#[test]
fn color_editor_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for marker in [
        "pub fn ColorEditor(",
        "#[prop(optional)] selected_color: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_selected_color: Option<String>",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] format: Option<Signal<ColorEditorFormat>>",
        "#[prop(optional)] default_format: Option<ColorEditorFormat>",
        "#[prop(optional)] on_format_change: Option<Callback<ColorEditorFormat>>",
        "let normalized_defaults = logic::normalize_default_inputs(",
        "let selected_state = overlay_open::use_controllable_state(",
        "let format_state = overlay_open::use_controllable_state(",
        "data-state=move || state.get().data_state_attr",
        "data-format=move || state.get().format_attr",
        "data-alpha=move || state.get().alpha_visibility_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "color-editor snapshot baseline should keep complete-result render marker `{marker}`.",
        );
    }

    for marker in [
        "pub struct ColorEditorDefaultInput",
        "pub struct ColorEditorDefaultState",
        "pub fn normalize_default_inputs(input: ColorEditorDefaultInput) -> ColorEditorDefaultState",
        "pub fn resolve_state(input: ColorEditorStateInput) -> ColorEditorState",
        "pub fn resolve_field_change(next: Option<String>) -> Option<String>",
    ] {
        assert!(
            logic_source.contains(marker),
            "color-editor logic should keep normalized snapshot baseline marker `{marker}`.",
        );
    }

    for marker in [
        "title=\"ColorEditor\"",
        "slug=\"color-editor\"",
        "<Playground title=\"Hello World（默认路径）\" code_signal=hello_code>",
        "<ColorEditor id_base=\"docs-color-editor-hello\".to_string() />",
        "<Playground title=\"Controlled Color + Controlled Format\" code_signal=basic_code>",
        "selected_color=selected_color_signal",
        "on_selected_change=on_selected_change",
        "format=format_signal",
        "on_format_change=on_format_change",
    ] {
        assert!(
            docs_source.contains(marker),
            "color-editor docs should keep snapshot-ready baseline usage marker `{marker}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_editor_check2_documents_docs_product_copy_paste_ready_contract() {
    let check2_source = load_source("../../components/color-editor/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let script_source = include_str!("../../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "apps/docs-app/src/pages/components/pages/forms_color.rs::color_editor()",
        "Hello World（默认路径）",
        "State Matrix",
        "Controlled vs Uncontrolled",
        "Streaming Optional / Snapshot",
        "Source-first / Copy-Paste Ready",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "code_imports",
        "color_editor_docs_page_covers_primary_playgrounds",
        "color_editor_docs_playgrounds_lock_state_matrix_contract_values",
        "color_editor_check2_documents_docs_product_copy_paste_ready_contract",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep docs-product copy-ready marker `{required}`.",
        );
    }

    for required in [
        "pub(super) fn color_editor() -> AnyView",
        "Playground title=\"Hello World（默认路径）\" code_signal=hello_code",
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

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_check2_documents_docs_product_copy_paste_ready_contract";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_editor_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only() {
    let check2_source = load_source("../../components/color-editor/check2.md");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let script_source = include_str!("../../../scripts/check-ui-components-contract-hygiene.sh");
    let local_semantics = include_str!("semantics.rs");

    for required in [
        "role=\"group\"",
        "aria-label=move || aria_label.get_value()",
        "aria-labelledby=label_id_for_root",
        "role=tab_role",
        "aria-controls=tab_aria_controls",
        "aria-selected=move || tab_aria_selected.get()",
        "data-state=move || state.get().data_state_attr",
        "data-format=move || state.get().format_attr",
        "data-ui-selection-source=move || agent_contract.get().selection_source",
        "data-ui-format-source=move || agent_contract.get().format_source",
        "data-ui-motion-source=move || agent_contract.get().motion_source",
        "data-ui-label-source=move || agent_contract.get().label_source",
        "data-ui-aria-source=move || agent_contract.get().aria_source",
        "data-ui-class-source=move || agent_contract.get().class_source",
    ] {
        assert!(
            view_source.contains(required),
            "color-editor view should keep semantic contract marker `{required}`.",
        );
    }

    for required in [
        "fn color_editor_semantics_contract_exposes_role_aria_and_source_markers()",
        "fn color_editor_semantics_matrix_covers_state_and_interaction_branches()",
        "fn color_editor_snapshot_baseline_consumes_complete_result_and_renders_stably()",
        "role=\"group\"",
        "aria-label=move || aria_label.get_value()",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            local_semantics.contains(required),
            "semantic test suite should include semantic-first coverage token `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "color_editor_semantics_contract_exposes_role_aria_and_source_markers",
        "color_editor_semantics_matrix_covers_state_and_interaction_branches",
        "color_editor_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "color_editor_semantics_priority_contract_prefers_semantic_assertions_over_snapshot_only",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should keep semantic-priority checklist marker `{required}`.",
        );
    }
}

#[test]
fn color_editor_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("../../components/color-editor/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
        "color_editor_check2_documents_e2e_selector_and_stable_wait_rules",
        "color_editor_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "color_editor_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints",
        "scripts/check-ui-components-e2e-color-editor.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should keep e2e-selector/stable-wait governance marker `{required}`.",
        );
    }
}

#[test]
fn color_editor_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_editor_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-e2e-color-editor.sh");

    for required in [
        "const COLOR_EDITOR_PAGE = \"/#/components/color-editor\";",
        "body:not(:has(#boot))",
        "[data-component=\"color-editor\"] #docs-color-editor-controlled[data-slot=\"color-editor\"]",
        "data-slot=\"color-editor-formats\"",
        "data-slot=\"color-editor-format-button\"",
        "data-slot=\"color-editor-channels\"",
        "data-ui-schema",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-ui-selection-source",
        "data-ui-format-source",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-editor e2e contract should include semantic selector/wait marker `{required}`.",
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
            "color-editor e2e contract should avoid brittle selector/wait token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_e2e_selector_contract_uses_semantic_markers_and_stable_waits";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-editor gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_editor_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_editor_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-e2e-color-editor.sh");

    for required in [
        "hslTab.focus()",
        "toBeFocused()",
        "hslTab.click()",
        "toHaveAttribute(\"data-format\", \"hsl\")",
        "toHaveAttribute(\"data-ui-action\", \"format-change\")",
        "toHaveAttribute(\"data-ui-output-status\", \"submittable\")",
        "#docs-color-editor-disabled[data-slot=\"color-editor\"]",
        "toHaveAttribute(\"data-motion-source\", \"custom\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "toHaveAttribute(\"data-state\", \"disabled\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "color-editor e2e motion/settled contract should include `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-editor gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_editor_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = load_source("../../components/color-editor/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
        "color_editor_check2_documents_e2e_repeatable_key_flow_rules",
        "color_editor_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "color_editor_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "scripts/check-ui-components-e2e-color-editor.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should keep repeatable e2e flow governance marker `{required}`.",
        );
    }
}

#[test]
fn color_editor_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_editor_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-e2e-color-editor.sh");

    for required in [
        "key flow is repeatable and failures map to semantic breakpoints",
        "hslTab.focus()",
        "page.keyboard.press(\"ArrowRight\")",
        "data-format\", \"hsb\"",
        "data-ui-action\", \"format-change\"",
        "data-ui-output-status\", \"submittable\"",
        "await page.reload();",
        "data-format\", \"hex\"",
        "data-ui-action\", \"snapshot-render\"",
        "data-ui-output-status\", \"verified\"",
    ] {
        assert!(
            e2e_source.contains(required),
            "repeatable color-editor e2e flow should include semantic breakpoint `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_e2e_key_flow_is_repeatable_and_failure_points_are_semantic";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-editor gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_editor_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_color_editor_contract.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-e2e-color-editor.sh");

    for required in [
        "high-risk paths keep focus keyboard and disabled branches semantically explicit",
        "rgbTab.focus()",
        "toBeFocused()",
        "page.keyboard.press(\"ArrowLeft\")",
        "data-format\", \"hex\"",
        "data-ui-action\", \"format-change\"",
        "#docs-color-editor-disabled[data-slot=\"color-editor\"]",
        "data-state\", \"disabled\"",
        "data-disabled\", \"true\"",
        "data-ui-action\", \"snapshot-render\"",
        "aria-disabled\", \"true\"",
        "toBeDisabled()",
    ] {
        assert!(
            e2e_source.contains(required),
            "high-risk color-editor e2e path should include `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "e2e-color-editor gate script should include `{script_needle}`.",
    );
}

#[test]
fn color_editor_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources()
{
    let lib_source = load_source("../../components/color-editor/src/lib.rs");
    let mod_source = load_source("../../components/color-editor/src/mod.rs");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");
    let styles_source = load_source("../../components/color-editor/src/styles.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let motion_source = load_source("../../components/color-editor/src/motion.rs");
    let combined = format!(
        "{lib_source}\n{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}"
    );

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "color-editor non-test source should forbid rust-hygiene anti-pattern `{forbidden}`.",
        );
    }
}

#[test]
fn color_editor_rust_hygiene_string_clone_hotspots_converge_to_cow_or_borrowed_static() {
    let logic_source = load_source("../../components/color-editor/src/logic.rs");

    for required in [
        "use std::borrow::Cow;",
        "fn normalize_text_with_fallback(value: Option<String>, fallback: &'static str) -> String",
        "let normalized: Cow<'static, str> = normalize_optional_text(value)",
        ".map(Cow::Owned)",
        ".unwrap_or(Cow::Borrowed(fallback));",
        "normalized.into_owned()",
    ] {
        assert!(
            logic_source.contains(required),
            "color-editor logic should keep Cow-based string hotspot mitigation marker `{required}`.",
        );
    }

    for forbidden in [
        "fallback.to_string()",
        "String::from(fallback)",
        "fallback.to_owned()",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "color-editor fallback normalization should avoid string clone hotspot `{forbidden}`.",
        );
    }
}

#[test]
fn color_editor_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let script_source = load_source("../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("../../scripts/check-ui-components-engineering.sh");

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
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_rust_hygiene_string_clone_hotspots_converge_to_cow_or_borrowed_static",
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn color_editor_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("../../components/color-editor/check2.md");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。（`components/color-editor/src/logic.rs` 通过 `Cow<'static, str>` 收敛默认文案回退的字符串复制热点；组件非测试源码维持无 `unwrap/expect` 与无吞错 `let _ = ...`。回归由 `components/color-editor/test/semantics.rs::color_editor_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources`、`components/color-editor/test/semantics.rs::color_editor_rust_hygiene_string_clone_hotspots_converge_to_cow_or_borrowed_static`、`components/color-editor/test/semantics.rs::color_editor_rust_hygiene_script_enforces_repo_level_hygiene_guards` 覆盖，并接入 `scripts/check-ui-components-engineering.sh` 门禁。）",
        "color_editor_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "color_editor_rust_hygiene_string_clone_hotspots_converge_to_cow_or_borrowed_static",
        "color_editor_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "./scripts/check-rust-hygiene.sh",
        "Cow<'static, str>",
    ] {
        assert!(
            check2_source.contains(needle),
            "color-editor check2 rust-hygiene section should reference `{needle}`.",
        );
    }
}

#[test]
fn color_editor_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("../../components/color-editor/check2.md");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget",
        "\"color-editor\" => UiPerfBudget {",
        "max_mount_ms: 40.0,",
        "max_update_ms: Some(14.0),",
        "max_heap_kb: Some(896.0),",
        "let perf_budget = component_page_perf_budget(slug);",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep color-editor perf governance token `{needle}`."
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "\"mount-only\"",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose repeatable perf marker `{needle}`."
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
            "docs coverage e2e should enforce perf blocking assertion `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_performance_governance_contract_is_budgeted_traceable_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include color-editor governance test command.",
    );

    assert!(
        script_source.contains("perf_render_count_follow_up_is_tracked_in_plan"),
        "performance gate script should retain render_count follow-up blocking guard.",
    );

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance TODO should track render_count follow-up token `{needle}`.",
        );
    }

    for needle in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep performance governance checklist token `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-format=move || state.get().format_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "view should expose perf-attribution marker `{needle}`.",
        );
    }

    for needle in [
        "pub fn normalize_default_inputs(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic should preserve attributable normalization path `{needle}`.",
        );
    }
}

#[test]
fn color_editor_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let check2_source = load_source("../../components/color-editor/check2.md");
    let docs_shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for marker in [
        "role=\"group\"",
        "aria-label=move || aria_label.get_value()",
        "aria-labelledby=label_id_for_root",
        "role=tab_role",
        "aria-controls=tab_aria_controls",
        "aria-selected=move || tab_aria_selected.get()",
        "data-state=move || state.get().data_state_attr",
        "data-format=move || state.get().format_attr",
        "data-alpha=move || state.get().alpha_visibility_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "tabindex=move || if roving_active_index.get() == index { 0 } else { -1 }",
        "on:focus=move |_| {",
        "on:keydown=move |ev: ev::KeyboardEvent| {",
        "use_roving_tabindex(RovingTabIndexOptions {",
    ] {
        assert!(
            view_source.contains(marker),
            "color-editor semantics/perf matrix should keep aria/data/focus marker `{marker}`.",
        );
    }

    for marker in [
        "\"color-editor\" => UiPerfBudget {",
        "max_mount_ms: 40.0,",
        "max_update_ms: Some(14.0),",
        "max_heap_kb: Some(896.0),",
    ] {
        assert!(
            docs_shell_source.contains(marker),
            "docs shell should preserve color-editor perf budget marker `{marker}`.",
        );
    }

    for marker in [
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
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
        "color_editor_semantics_contract_exposes_role_aria_and_source_markers",
        "color_editor_semantics_matrix_covers_state_and_interaction_branches",
        "color_editor_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "color_editor_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
    ] {
        assert!(
            check2_source.contains(marker),
            "color-editor check2 semantics/perf section should reference `{marker}`.",
        );
    }

    assert!(
        logic_source.contains("pub fn resolve_state("),
        "logic should keep state derivation path for attributable semantics/perf regressions.",
    );
}

#[test]
fn color_editor_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2_source = load_source("../../components/color-editor/check2.md");

    for needle in [
        "fn render_editor_header(",
        "fn render_editor_sliders(",
        "fn render_editor_canvas(",
        "fn render_format_tabs(",
        "fn render_channel_rows(",
        "fn render_editor_controls(",
        "let header = render_editor_header(",
        "let canvas = render_editor_canvas(",
        "let controls = render_editor_controls(",
    ] {
        assert!(
            view_source.contains(needle),
            "color-editor view macro split should keep semantic subrender marker `{needle}`.",
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 10,
        "color-editor view macro complexity regression: expected <= 10 `view!` blocks, found {view_macro_count}.",
    );

    let component_macro_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_macro_count, 1,
        "color-editor should keep exactly one public component entry; found {component_macro_count}.",
    );

    for forbidden in [
        "#[component]\nfn render_editor_header(",
        "#[component]\nfn render_editor_canvas(",
        "#[component]\nfn render_format_tabs(",
        "#[component]\nfn render_editor_controls(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-editor local fragments should remain plain functions and avoid `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include color-editor macro complexity test target.",
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
fn color_editor_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2_source = load_source("../../components/color-editor/check2.md");

    for needle in [
        "fn render_editor_header(",
        "fn render_editor_sliders(",
        "fn render_editor_canvas(",
        "fn render_format_tabs(",
        "fn render_channel_rows(",
        "fn render_editor_controls(",
        ") -> impl IntoView {",
        "pub fn ColorEditor(",
    ] {
        assert!(
            view_source.contains(needle),
            "color-editor function-first split should keep `{needle}`.",
        );
    }

    for forbidden in [
        "#[component]\nfn render_editor_header(",
        "#[component]\nfn render_editor_sliders(",
        "#[component]\nfn render_editor_canvas(",
        "#[component]\nfn render_format_tabs(",
        "#[component]\nfn render_channel_rows(",
        "#[component]\nfn render_editor_controls(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "color-editor local fragments should remain plain functions and avoid `{forbidden}`.",
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "color-editor should keep exactly one public component boundary.",
    );

    for needle in [
        "data-slot=\"color-editor\"",
        "data-slot=\"color-editor-header\"",
        "data-slot=\"color-editor-canvas\"",
        "data-slot=\"color-editor-controls\"",
    ] {
        assert!(
            view_source.contains(needle),
            "function-first split should preserve stable semantic marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include color-editor function-first split test target.",
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
fn color_editor_static_fragments_are_constantized_or_absent_for_simple_layout() {
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");
    let check2_source = load_source("../../components/color-editor/check2.md");

    for needle in [
        "const FORMAT_OPTIONS: [ColorEditorFormat; 4] = [",
        "ColorEditorFormat::Hex,",
        "ColorEditorFormat::Rgb,",
        "ColorEditorFormat::Hsl,",
        "ColorEditorFormat::Hsb,",
        "{FORMAT_OPTIONS",
        "fn render_channel_rows(channel_preview: Memo<Vec<(String, String)>>) -> impl IntoView {",
        "{channel_rows}",
        "data-slot=\"color-editor-channel-row\"",
        "data-slot=\"color-editor-channel-key\"",
        "data-slot=\"color-editor-channel-value\"",
        "role=\"tabpanel\"",
        "aria-labelledby=move || active_tab_id.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "color-editor static fragment contract should keep `{needle}`.",
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
            "color-editor simple layout should avoid heavy static fragment token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_static_fragments_are_constantized_or_absent_for_simple_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include color-editor static-fragment test target.",
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
fn color_editor_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let script_source = load_source("../../scripts/check-ui-components-inner-html.sh");
    let check2_source = load_source("../../components/color-editor/check2.md");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");
    let docs_shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");

    for rel_path in [
        "../../components/color-editor/src/mod.rs",
        "../../components/color-editor/src/logic.rs",
        "../../components/color-editor/src/styles.rs",
        "../../components/color-editor/src/view.rs",
        "../../components/color-editor/src/motion.rs",
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
                "color-editor source `{rel_path}` should forbid raw-html injection token `{forbidden}`.",
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
            !docs_page_source.contains(forbidden),
            "color-editor docs page should avoid raw-html injection token `{forbidden}`.",
        );
    }

    assert!(
        docs_shell_source.contains("<div data-slot=\"component-readme\" inner_html=html></div>"),
        "docs shell should keep the single trusted markdown inner_html mount.",
    );
    assert!(
        !docs_shell_source.contains("\"color-editor\" => Some("),
        "color-editor should stay out of docs-shell inner_html whitelist.",
    );

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(script_needle),
        "inner-html check script should include color-editor inner-html contract target.",
    );

    assert!(
        check2_source.contains("- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。"),
        "check2 should mark inner_html contract as completed.",
    );
}

#[test]
fn color_editor_wasm_debug_contract_is_explicitly_na_and_feature_isolated() {
    let check2_source = load_source("../../components/color-editor/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");

    let color_editor_cargo = load_source("../../components/color-editor/Cargo.toml");
    let ui_components_cargo = load_source("Cargo.toml");
    let ui_components_lib = load_source("../../crates/ui-components/src/lib.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in ["[features]", "default = []"] {
        assert!(
            color_editor_cargo.contains(needle),
            "color-editor crate feature boundary should include `{needle}`.",
        );
    }

    for forbidden in [
        "wasm-debug",
        "color-editor-wasm-debug",
        "color_editor-wasm-debug",
        "component-color_editor-wasm-debug",
    ] {
        assert!(
            !color_editor_cargo.contains(forbidden),
            "color-editor crate should not expose wasm-debug feature `{forbidden}`.",
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components should keep shared wasm-debug feature marker `{needle}`.",
        );
    }

    for forbidden in [
        "color-editor-wasm-debug =",
        "color_editor-wasm-debug =",
        "component-color_editor-wasm-debug",
        "component-color_editor\", \"dep:tracing",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "ui-components feature graph should not leak color-editor debug toggle `{forbidden}`.",
        );
    }

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui-components root should keep shared wasm-debug isolation marker `{needle}`.",
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
        "data-format=move || state.get().format_attr",
        "data-alpha=move || state.get().alpha_visibility_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "color-editor should keep state/source marker `{needle}` for debug traceability.",
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
            "color-editor runtime contract should not leak wasm-debug internals `{forbidden}`.",
        );
    }

    for needle in [
        "pub(super) fn color_editor() -> AnyView",
        "title=\"Interactive Playground\"",
        "id_base=\"docs-color-editor-workbench\".to_string()",
        "on_selected_change=on_workbench_selected_change",
        "on_format_change=on_workbench_format_change",
        "value: ",
        "workbench_format.get().as_attr()",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "docs page should keep reproducible color-editor interaction marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_wasm_debug_contract_is_explicitly_na_and_feature_isolated";
    assert!(
        script_source.contains(script_needle),
        "wasm-debug gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] WASM 调试要求",
        "color_editor_wasm_debug_contract_is_explicitly_na_and_feature_isolated",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep wasm-debug governance marker `{needle}`.",
        );
    }
}

#[test]
fn color_editor_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na()
{
    let check2_source = load_source("../../components/color-editor/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");
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

    for needle in [
        "pub(super) fn color_editor() -> AnyView",
        "slug=\"color-editor\"",
        "title=\"Interactive Playground\"",
        "description=\"展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。\"",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/color-editor/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "controls=move || view!",
        "<Switch checked=workbench_disabled set_checked=set_workbench_disabled>",
        "<Switch checked=workbench_hide_alpha set_checked=set_workbench_hide_alpha>",
        "<Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>",
        "<Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>",
        "<Switch checked=workbench_reduced_motion set_checked=set_workbench_reduced_motion>",
        "id_base=\"docs-color-editor-workbench\".to_string()",
        "id_base=\"docs-color-editor-workbench-compare\".to_string()",
        "on_selected_change=on_workbench_selected_change",
        "on_format_change=on_workbench_format_change",
        "value: ",
        "workbench_format.get().as_attr()",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "color-editor docs should keep interactive workbench/context marker `{needle}`.",
        );
    }

    for forbidden in [
        "COLOR_EDITOR_WORKBENCH_STORAGE_KEY",
        "load_color_editor_workbench_state(",
        "save_color_editor_workbench_state(",
        "clear_color_editor_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_page_source.contains(forbidden),
            "color-editor keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na";
    assert!(
        script_source.contains(script_needle),
        "DX gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
        "color_editor_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep DX governance marker `{needle}`.",
        );
    }
}

#[test]
fn color_editor_engineering_contract_marks_spec_serde_path_as_na_and_keeps_tracing_runtime_boundaries()
 {
    let check2_source = load_source("../../components/color-editor/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");
    let ui_components_cargo = load_source("Cargo.toml");
    let color_editor_cargo = load_source("../../components/color-editor/Cargo.toml");
    let mod_source = load_source("../../components/color-editor/src/mod.rs");
    let logic_source = load_source("../../components/color-editor/src/logic.rs");
    let view_source = load_source("../../components/color-editor/src/view.rs");
    let styles_source = load_source("../../components/color-editor/src/styles.rs");
    let motion_source = load_source("../../components/color-editor/src/motion.rs");
    let readme_source = load_source("../../components/color-editor/src/README.md");

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../components/color-editor/src/spec.rs");
    assert!(
        !spec_path.exists(),
        "color-editor should keep spec/serde migration path as N/A for simple component scope.",
    );

    for needle in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "engineering baseline should keep canonical tracing contract marker `{needle}`.",
        );
    }

    for forbidden in [
        "component-color_editor\", \"dep:tracing",
        "color-editor-wasm-debug =",
        "color_editor-wasm-debug =",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "color-editor should not define component-local tracing/debug feature `{forbidden}`.",
        );
    }

    for forbidden in [
        "serde =",
        "serde_json",
        "tracing",
        "tokio",
        "async-std",
        "async_std",
    ] {
        assert!(
            !color_editor_cargo.contains(forbidden),
            "color-editor manifest should avoid serde/tracing/runtime leak token `{forbidden}` in current scope.",
        );
    }

    for source in [
        mod_source,
        logic_source,
        view_source,
        styles_source,
        motion_source,
        readme_source,
    ] {
        for forbidden in [
            "serde::",
            "serde_json::",
            "tracing::span!(",
            "tracing::event!(",
            "#[tracing::instrument]",
            "tokio::",
            "tokio =",
            "#[tokio::main]",
            "#[tokio::test]",
            "async_std::",
            "async-std",
            "async fn",
            "tokio::runtime",
            "JoinHandle",
        ] {
            assert!(
                !source.contains(forbidden),
                "color-editor engineering boundary should avoid runtime/tracing/serde token `{forbidden}`.",
            );
        }
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_engineering_contract_marks_spec_serde_path_as_na_and_keeps_tracing_runtime_boundaries";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
        "color_editor_engineering_contract_marks_spec_serde_path_as_na_and_keeps_tracing_runtime_boundaries",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep engineering governance marker `{needle}`.",
        );
    }
}

#[test]
fn color_editor_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
 {
    let check2_source = load_source("../../components/color-editor/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");
    let readme_source = load_source("../../components/color-editor/src/README.md");
    let protocol_source = include_str!("../src/protocol.rs");
    let component_manifest = include_str!("../src/Component.toml");

    for required in [
        "pub enum EditorComponentSchemaVersion",
        "V1",
        "pub struct EditorComponentSpec",
        "pub schema_version: EditorComponentSchemaVersion",
    ] {
        assert!(
            protocol_source.contains(required),
            "color-editor protocol should keep stable v1 marker `{required}` in non-breaking scope.",
        );
    }

    assert!(
        component_manifest.contains("schema = \"ui.color-editor.agent-contract.v1\""),
        "Component.toml should keep v1 schema registration in current scope.",
    );

    for forbidden in [
        "V2",
        "migrate_v1_to_v2",
        "SchemaRegistry",
        "deprecated_window",
        "codemod",
    ] {
        assert!(
            !protocol_source.contains(forbidden) && !readme_source.contains(forbidden),
            "without major breaking upgrade, color-editor should not claim migration path token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test color_editor_semantics --no-default-features --features component-color_editor,inject-css color_editor_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `ColorEditor` 改动未引入跨大版本 API 破坏升级，也未将组件协议从 `v1` 升级到 `v2`；`components/color-editor/src/protocol.rs` 仍仅声明 `EditorComponentSchemaVersion::V1` 与最小 `EditorComponentSpec`，不存在 `migrate_v1_to_v2`/弃用窗口注册需求。回归由 `components/color-editor/test/semantics.rs::color_editor_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade` 覆盖，并接入 `scripts/check-ui-components-engineering.sh` 门禁。）",
        "color_editor_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            check2_source.contains(needle),
            "checklist should keep codemod/registry migration marker `{needle}`.",
        );
    }
}

#[test]
fn color_editor_check2_marks_all_items_completed() {
    let source = load_source("../../components/color-editor/src/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义",
        "- [x] `ui-headless` 定义",
        "- [x] `ui-motion` 定义",
        "- [x] Tree Shaking 是一等能力",
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            source.contains(needle),
            "color_editor/check2.md should keep completed marker `{needle}`."
        );
    }

    assert!(
        !source.contains("- [ ]"),
        "color_editor/check2.md should not contain unchecked items."
    );
}
