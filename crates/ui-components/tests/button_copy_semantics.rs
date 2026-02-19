use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn button_copy_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button/copy/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ButtonCopy internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn button_copy_agent_contract_is_schema_typed_and_mounted() {
    let view_source = load_source("src/button/copy/view.rs");
    let logic_source = load_source("src/button/copy/logic.rs");

    for needle in [
        "pub enum ButtonCopyAgentSchemaVersion",
        "pub enum ButtonCopyAgentIntent",
        "pub enum ButtonCopyAgentAction",
        "pub enum ButtonCopyAgentStateAxis",
        "pub enum ButtonCopyAgentOutputStatus",
        "pub struct ButtonCopyAgentCapabilities",
        "pub struct ButtonCopyAgentContract",
        "pub fn resolve_agent_contract(",
        "pub fn resolve_agent_output_status(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ButtonCopy agent contract typing should include `{needle}`."
        );
    }

    for needle in [
        "let agent_contract = logic::resolve_agent_contract(view_state);",
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-schema-version=agent_contract.schema_version.as_str()",
        "data-ui-agent-schema=agent_contract.schema_name",
        "data-ui-agent-schema-version=agent_contract.schema_version.as_str()",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-action=agent_contract.action.as_str()",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-output-status=move || {",
        "data-ui-capability-copy=agent_contract.capabilities.can_copy.then_some(\"true\")",
        "data-ui-capability-visual-feedback=agent_contract",
        "data-ui-capability-announce-feedback=agent_contract",
    ] {
        assert!(
            view_source.contains(needle),
            "ButtonCopy view should mount schemaized agent contract field `{needle}`."
        );
    }
}

#[test]
fn button_copy_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/button/copy/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::ButtonCopy;")
            && module_source.contains("pub use motion::ButtonCopyMotion;")
            && module_source.contains("pub use i18n::ButtonCopyStrings;"),
        "button_copy module should export `ButtonCopy`, `ButtonCopyMotion`, and `ButtonCopyStrings`."
    );
    assert!(
        crate_source
            .contains("pub use button::copy::{ButtonCopy, ButtonCopyMotion, ButtonCopyStrings};"),
        "crate root should re-export `ButtonCopy`, `ButtonCopyMotion`, and `ButtonCopyStrings` contracts."
    );
}

#[test]
fn button_copy_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::button::copy::styles::CSS);"),
        "ui-components css aggregator should include button_copy styles."
    );
}

#[test]
fn button_copy_docs_page_contains_playground_contracts() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn button_copy() -> AnyView",
        "title=\"ButtonCopy\"",
        "slug=\"button-copy\"",
        "Label + variant",
        "Disabled + empty matrix",
        "Mode matrix",
        "Copy-to-clipboard button with baseline-style disabled/empty semantics and live copied announcements.",
        "<ButtonCopy",
    ] {
        assert!(
            source.contains(needle),
            "button-copy docs page should contain `{needle}`."
        );
    }
}

#[test]
fn button_copy_uses_logic_state_model() {
    let view_source = load_source("src/button/copy/view.rs");
    let logic_source = load_source("src/button/copy/logic.rs");

    for needle in [
        "pub struct ButtonCopyViewState",
        "pub struct ButtonCopyTextContract",
        "pub enum ButtonCopyMode",
        "pub is_copyable: bool",
        "pub has_custom_label: bool",
        "pub fn normalize_optional_text(",
        "pub fn resolve_text_contract(",
        "pub fn resolve_view_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ButtonCopy logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "#[prop(optional, default = logic::ButtonCopyMode::default())] mode: logic::ButtonCopyMode",
        "let label = logic::normalize_optional_text(label);",
        "let copied_label = logic::normalize_optional_text(copied_label);",
        "logic::resolve_text_contract(",
        "label.or(default_label)",
        "copied_label.or(default_copied_label)",
        "let view_state = logic::resolve_view_state(",
        "let class = logic::compose_class_name(class_name, view_state);",
    ] {
        assert!(
            view_source.contains(needle),
            "ButtonCopy view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn button_copy_uses_snippet_logic_for_copy_behavior() {
    let source = load_source("src/button/copy/view.rs");

    for needle in [
        "crate::snippet::logic::use_snippet_logic(text.clone())",
        "on_press=logic.copy",
        "data-copied=move || logic.copied.get().then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy should delegate copy behavior via `{needle}`."
        );
    }
}

#[test]
fn button_copy_supports_i18n_and_locale_passthrough() {
    let source = load_source("src/button/copy/view.rs");

    for needle in [
        "let i18n = i18n::use_ui_i18n();",
        "strings::<super::i18n::ButtonCopyStrings>()",
        "copy_button_label",
        "copied_status_text",
        "copy_failed_status_text",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = ui_headless::a11y::locale_attrs(lang, dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy should include i18n/locale support via `{needle}`.",
        );
    }
}

#[test]
fn button_copy_forwards_button_contract_and_disabled_semantics() {
    let source = load_source("src/button/copy/view.rs");

    for needle in [
        "<Button",
        "variant=variant",
        "size=size",
        "motion=motion.button",
        "aria_label=aria_label.get_value()",
        "is_icon_only=view_state.is_icon_only",
        "is_loading=is_copying",
        "is_disabled=!view_state.is_copyable",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy should forward `{needle}` to the underlying Button."
        );
    }
}

#[test]
fn button_copy_defaults_align_with_base_button_contract() {
    let source = load_source("src/button/copy/view.rs");

    for needle in [
        "#[prop(optional, default = ButtonVariant::default())] variant: ButtonVariant",
        "#[prop(optional, default = ButtonSize::default())] size: ButtonSize",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy should keep `{needle}` so default visual contract matches Button."
        );
    }
}

#[test]
fn button_copy_emits_baseline_style_data_attributes() {
    let source = load_source("src/button/copy/view.rs");

    for needle in [
        "data-slot=\"button-copy\"",
        "data-state=view_state.state_attr",
        "data-mode=view_state.mode_attr",
        "data-icon-only=view_state.is_icon_only.then_some(\"true\")",
        "data-with-icon=view_state.shows_icon.then_some(\"true\")",
        "data-with-text=view_state.shows_text.then_some(\"true\")",
        "data-copyable=view_state.is_copyable.then_some(\"true\")",
        "data-disabled=view_state.is_disabled.then_some(\"true\")",
        "data-empty=(!view_state.has_text).then_some(\"true\")",
        "data-label=if view_state.has_custom_label {",
        "data-copied-label=if view_state.has_custom_copied_label {",
        "data-copying=move || logic.is_copying.get().then_some(\"true\")",
        "data-copy-error=move || logic.has_copy_error.get().then_some(\"true\")",
        "data-copy-status=move || {",
        "data-motion-source=if motion == ButtonCopyMotion::default()",
        "data-custom-motion=(motion != ButtonCopyMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy should expose `{needle}` for baseline-style state inspection."
        );
    }
}

#[test]
fn button_copy_announces_copy_result_for_assistive_tech() {
    let source = load_source("src/button/copy/view.rs");

    for needle in [
        "data-slot=\"button-copy-status\"",
        "aria-live=\"polite\"",
        "aria-atomic=\"true\"",
        "copy_failed_status_text.get_value()",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy a11y status element should include `{needle}`."
        );
    }
}

#[test]
fn button_copy_styles_include_motion_marker_contracts() {
    let source = load_source("src/button/copy/styles.rs");

    for selector in [
        ".ui-button-copy[data-motion-source=\"custom\"]",
        ".ui-button-copy[data-custom-motion=\"true\"]",
        ".ui-button-copy[data-mode=\"icon-only\"] .ui-button-copy__button",
        ".ui-button-copy[data-copied=\"true\"] .ui-button-copy__button",
    ] {
        assert!(
            source.contains(selector),
            "ButtonCopy styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn button_copy_motion_contract_exposes_default_and_custom_tests() {
    let source = load_source("src/button/copy/motion.rs");

    for needle in [
        "pub struct ButtonCopyMotion",
        "pub fn attach_motion(",
        "fn default_motion_matches_button_contract_defaults()",
        "fn supports_custom_button_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy motion module should include `{needle}` for baseline-level motion contract coverage."
        );
    }
}

#[test]
fn button_copy_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/button/copy/motion.rs");
    let view_source = load_source("src/button/copy/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ButtonCopyMotion) -> ButtonCopyMotion",
        "button: crate::button::motion::sanitize_motion(motion.button)",
        "copied_feedback_spring: sanitize_spring(motion.copied_feedback_spring)",
        "fn sanitize_motion_clamps_feedback_values()",
        "fn sanitize_motion_delegates_to_button_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "ButtonCopy motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = super::motion::sanitize_motion(motion);"),
        "ButtonCopy view should sanitize motion before forwarding to Button.",
    );
    assert!(
        view_source.contains("super::motion::attach_motion(root_ref, logic.copied, motion);"),
        "ButtonCopy should attach copied-feedback motion on wrapper state.",
    );
}

#[test]
fn button_copy_reduced_motion_ssr_wasm_branches_are_covered_via_button_contract() {
    let copy_styles_source = load_source("src/button/copy/styles.rs");
    let button_styles_source = load_source("src/button/styles.rs");
    let copy_motion_source = load_source("src/button/copy/motion.rs");
    let button_motion_source = load_source("src/button/motion.rs");
    let copy_view_source = load_source("src/button/copy/view.rs");

    for needle in [
        "@media (prefers-reduced-motion: reduce)",
        ".ui-button-copy {",
        "transform: none;",
        ".ui-button-copy__icon {",
        "transition: none;",
    ] {
        assert!(
            copy_styles_source.contains(needle),
            "button_copy reduced-motion path should include `{needle}`."
        );
    }

    assert!(
        button_styles_source.contains("@media (prefers-reduced-motion: reduce)"),
        "base Button reduced-motion contract should stay present for ButtonCopy reuse."
    );

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "button: crate::button::motion::sanitize_motion(motion.button)",
    ] {
        assert!(
            copy_motion_source.contains(needle),
            "button_copy motion should keep platform split/reuse marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            button_motion_source.contains(needle),
            "base Button motion should keep wasm/non-wasm marker `{needle}` for ButtonCopy inheritance."
        );
    }

    for needle in [
        "<Button",
        "motion=motion.button",
        "data-copy-status=move || {",
        "data-ui-output-status=move || {",
    ] {
        assert!(
            copy_view_source.contains(needle),
            "button_copy view should keep SSR/hydration-stable semantic marker `{needle}`."
        );
    }
}

#[test]
fn button_copy_docs_default_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn button_copy() -> AnyView",
        "<Playground title=\"Label + variant\" code_signal=code>",
        "text=\"cargo add ui-components\".to_string()",
        "label=\"Copy install command\".to_string()",
        "copied_label=\"Copied!\".to_string()",
        "text=\"https://github.com/openai\".to_string()",
        "variant=ButtonVariant::Outline",
        "label=\"Copy URL\".to_string()",
        "copied_label=\"URL copied\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "button_copy docs default playground should contain `{needle}`.",
        );
    }
}

#[test]
fn button_copy_docs_entry_is_beginner_friendly_with_hello_world_first() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "let hello_world_code = Signal::derive(move || {",
        "<ButtonCopy text=\"cargo add ui-components\".to_string() />",
        "Start simple, then move to advanced controls.",
    ] {
        assert!(
            source.contains(needle),
            "button_copy docs beginner-friendly entry should include `{needle}`."
        );
    }

    let hello_world_pos = source
        .find("<Playground title=\"Hello World\" code_signal=hello_world_code>")
        .expect("button_copy docs should include Hello World playground");
    let workbench_pos = source
        .find("title=\"Workbench (Isolated Canvas + Optional Persist)\"")
        .expect("button_copy docs should include Workbench playground");
    assert!(
        hello_world_pos < workbench_pos,
        "button_copy docs should place beginner hello-world path before advanced workbench."
    );
}

#[test]
fn button_copy_docs_state_matrix_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<Playground title=\"Disabled + empty matrix\" code_signal=states_code>",
        "text=\"https://example.com/docs\".to_string()",
        "variant=ButtonVariant::Outline",
        "text=\"   \".to_string()",
        "label=\"Nothing to copy\".to_string()",
        "text=\"token\".to_string()",
        "is_disabled=true",
        "Blank text and explicit disabled state both force non-copyable semantics.",
    ] {
        assert!(
            source.contains(needle),
            "button_copy docs state matrix playground should contain `{needle}`.",
        );
    }
}

#[test]
fn button_copy_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn button_copy() -> AnyView",
        "title=\"ButtonCopy\"",
        "slug=\"button-copy\"",
        "Label + variant",
        "Disabled + empty matrix",
        "Mode matrix",
    ] {
        assert!(
            source.contains(needle),
            "button_copy docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn button_copy_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<Playground title=\"Label + variant\" code_signal=code>",
        "text=\"cargo add ui-components\".to_string()",
        "label=\"Copy install command\".to_string()",
        "copied_label=\"Copied!\".to_string()",
        "<Playground title=\"Disabled + empty matrix\" code_signal=states_code>",
        "<Playground title=\"Mode matrix\" code_signal=modes_code>",
        "text=\"   \".to_string()",
        "label=\"Nothing to copy\".to_string()",
        "is_disabled=true",
        "mode=ButtonCopyMode::TextOnly",
        "mode=ButtonCopyMode::IconOnly",
        "mode=ButtonCopyMode::IconAndText",
    ] {
        assert!(
            source.contains(needle),
            "button_copy docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn button_copy_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for needle in [
        "\"button-copy\" => UiPerfBudget {",
        "max_mount_ms: 26.0,",
        "max_update_ms: Some(9.0),",
        "max_heap_kb: Some(448.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "button-copy page should keep performance budget contract `{needle}`."
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
            "docs e2e should keep perf guard `{needle}` for component pages including button-copy."
        );
    }

    let needle = "cargo test -p ui-components --test button_copy_semantics button_copy_performance_governance_budget_is_defined_and_blocking";
    assert!(
        script_source.contains(needle),
        "performance gate script should include `{needle}`."
    );
}

#[test]
fn button_copy_view_macro_complexity_is_split_into_semantic_subrenders() {
    let source = load_source("src/button/copy/view.rs");

    for needle in [
        "fn render_copy_content(",
        "fn render_copy_button(",
        "fn render_copy_status(",
        "{render_copy_button(",
        "{render_copy_status(status_logic, copied_label, copy_failed_status_text)}",
    ] {
        assert!(
            source.contains(needle),
            "button_copy view should keep semantic macro split marker `{needle}`."
        );
    }
}

#[test]
fn button_copy_static_icon_fragments_are_constantized() {
    let source = load_source("src/button/copy/view.rs");

    for needle in [
        "const ICON_VIEWBOX: &str = \"0 0 16 16\";",
        "const ICON_COPIED_PATH_D: &str = \"M3 8.5L6.25 11.5L13 4.5\";",
        "const ICON_IDLE_OFFSET_PATH_D: &str = \"M3 11V5.5C3 4.67 3.67 4 4.5 4H10\";",
        "fn render_icon_shape(copied: bool) -> AnyView",
        "let icon_shape = render_icon_shape(copied);",
    ] {
        assert!(
            source.contains(needle),
            "button_copy icon template should keep static fragment marker `{needle}`."
        );
    }

    let svg_occurrences = source.matches("class=\"ui-button-copy__icon-svg\"").count();
    assert_eq!(
        svg_occurrences, 1,
        "button_copy should keep a single svg shell template to reduce repeated view! static fragments."
    );
}

#[test]
fn button_copy_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let view_source = load_source("src/button/copy/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    assert!(
        !view_source.contains("inner_html"),
        "ButtonCopy component should not use `inner_html`; keep text/icon rendering explicit and safe."
    );
    assert!(
        !docs_source.contains("inner_html"),
        "ButtonCopy docs examples should not demonstrate `inner_html` injection."
    );
}

#[test]
fn button_copy_wasm_debug_contract_reuses_button_debug_and_keeps_feature_isolated() {
    let copy_view_source = load_source("src/button/copy/view.rs");
    let button_view_source = load_source("src/button/view.rs");
    let cargo_source = load_source("Cargo.toml");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");

    for needle in ["<Button", "on_press=logic.copy", "motion=motion.button"] {
        assert!(
            copy_view_source.contains(needle),
            "ButtonCopy should keep delegating debug-capable interaction path via `{needle}`."
        );
    }

    for needle in [
        "feature = \"button-wasm-debug\"",
        "debug_assertions",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
        "data-slot=\"button-debug-replay\"",
    ] {
        assert!(
            button_view_source.contains(needle),
            "Button wasm debug contract should keep `{needle}` for source/time/before/after and replay."
        );
    }

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components feature contract should keep `{needle}`."
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
            "docs visual debug entry should keep `{needle}`."
        );
    }
}

#[test]
fn button_copy_dx_workbench_supports_optional_state_persistence_and_isolated_canvas() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "BUTTON_COPY_WORKBENCH_STORAGE_KEY",
        "fn load_button_copy_workbench_state() -> Option<ButtonCopyWorkbenchState>",
        "fn save_button_copy_workbench_state(state: ButtonCopyWorkbenchState)",
        "fn clear_button_copy_workbench_state()",
        "title=\"Workbench (Isolated Canvas + Optional Persist)\"",
        "description=\"Workbench canvas: scoped CSS live-edit + optional state persistence across reload.\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "<Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>",
        "\"Persist workbench state\"",
        "Effect::new(move |_| {",
        "save_button_copy_workbench_state(ButtonCopyWorkbenchState {",
        "clear_button_copy_workbench_state();",
        "data-slot=\"button-copy-workbench\"",
        "data-slot=\"button-copy-workbench-canvas\"",
        "data-slot=\"button-copy-workbench-controls\"",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy workbench should keep DX marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            source.contains(needle),
            "ButtonCopy workbench persistence should keep platform guard `{needle}`."
        );
    }
}

#[test]
fn button_copy_dx_check_script_covers_workbench_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    let needle = "cargo test -p ui-components --test button_copy_semantics button_copy_dx_workbench_supports_optional_state_persistence_and_isolated_canvas";
    assert!(
        script_source.contains(needle),
        "DX check script should enforce `{needle}`."
    );
}

#[test]
fn button_copy_engineering_contract_reuses_button_tracing_and_avoids_runtime_leaks() {
    let cargo_source = load_source("Cargo.toml");
    let copy_mod_source = load_source("src/button/copy/mod.rs");
    let copy_view_source = load_source("src/button/copy/view.rs");
    let copy_logic_source = load_source("src/button/copy/logic.rs");
    let copy_motion_source = load_source("src/button/copy/motion.rs");
    let button_view_source = load_source("src/button/view.rs");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "button_copy engineering contract should keep feature boundary marker `{needle}`."
        );
    }

    for needle in [
        "<Button",
        "motion=motion.button",
        "on_press=logic.copy",
        "crate::button::motion::sanitize_motion(motion.button)",
    ] {
        assert!(
            copy_view_source.contains(needle) || copy_motion_source.contains(needle),
            "button_copy should reuse Button capability marker `{needle}`."
        );
    }

    for needle in [
        "target: \"ui_components::button::state_change\"",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
    ] {
        assert!(
            button_view_source.contains(needle),
            "button tracing/debug contract should provide `{needle}` for reused ButtonCopy interaction path."
        );
    }

    for needle in [
        "button.copy.motion.burst",
        "button.copy.motion.scale",
        "button.copy.motion.glow",
    ] {
        assert!(
            copy_motion_source.contains(needle),
            "button_copy motion observability contract should include `{needle}`."
        );
    }

    for source in [
        &copy_mod_source,
        &copy_view_source,
        &copy_logic_source,
        &copy_motion_source,
    ] {
        for forbidden in ["tokio", "tokio::", "async_std", "async-std", "async_std::"] {
            assert!(
                !source.contains(forbidden),
                "button_copy engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !copy_mod_source.contains("web_sys"),
        "button_copy public module boundary should not leak web_sys types."
    );
}

#[test]
fn button_copy_engineering_check_script_covers_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    let needle = "cargo test -p ui-components --test button_copy_semantics button_copy_engineering_contract_reuses_button_tracing_and_avoids_runtime_leaks";
    assert!(
        script_source.contains(needle),
        "engineering check script should enforce `{needle}`."
    );
}

#[test]
fn button_copy_e2e_flow_is_in_repeatable_regression_set() {
    let e2e_source = load_source("../../e2e/tests/docs_app_button_copy_contract.spec.mjs");

    for needle in [
        "docs-app button-copy flow uses semantic selectors with settled async copy states",
        "body:not(:has(#boot))",
        "section.playground",
        "[data-slot=\"button-copy\"]",
        "[data-slot=\"button\"]",
        "[data-slot=\"button-copy-status\"]",
        "toHaveAttribute(\"data-copy-status\", \"copied\")",
        "toHaveAttribute(\"data-copy-status\", \"idle\", { timeout: 3500 })",
        "poll(() => page.evaluate(() => window.__copiedText))",
    ] {
        assert!(
            e2e_source.contains(needle),
            "button-copy e2e flow contract should include `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "button-copy e2e flow contract should avoid unstable fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn button_copy_e2e_check_script_covers_repeatable_flow_contract() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-button.sh");

    let needle = "cargo test -p ui-components --test button_copy_semantics button_copy_e2e_flow_is_in_repeatable_regression_set";
    assert!(
        script_source.contains(needle),
        "button-copy e2e check script should enforce `{needle}`."
    );
}

#[test]
fn button_copy_heroui_strategy_doc_sync_tracks_params_and_docs_entrypoint() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_actions_source =
        load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "### ButtonCopy 同步记录（2026-02-17）",
        "Button` 特化定位",
        "text`、`label`、`copied_label`、`aria_label`、`mode`、`is_disabled`、`motion`、`lang/dir",
        "TextOnly` / `IconOnly` / `IconAndText",
        "apps/docs-app/src/pages/components/pages.rs",
        "apps/docs-app/src/pages/components/pages/actions.rs",
        "compose_copy_ready_code",
    ] {
        assert!(
            strategy_source.contains(needle),
            "HeroUI strategy document should include button-copy sync marker `{needle}`."
        );
    }

    assert!(
        docs_registry_source.contains(
            "component_doc!(\"ButtonCopy\", \"button-copy\", \"Actions\", actions::button_copy)"
        ),
        "docs component registry should expose button-copy entrypoint.",
    );

    for needle in [
        "title=\"ButtonCopy\"",
        "slug=\"button-copy\"",
        "title=\"Hello World\"",
        "title=\"Label + variant\"",
        "title=\"Disabled + empty matrix\"",
        "title=\"Mode matrix\"",
    ] {
        assert!(
            docs_actions_source.contains(needle),
            "docs button-copy page should keep synced example marker `{needle}`."
        );
    }
}

#[test]
fn button_copy_directory_layout_matches_component_contract() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dir = manifest_dir.join("src/button/copy");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        let path = dir.join(required);
        assert!(
            path.exists(),
            "button_copy directory contract should include `{required}`."
        );
    }

    for forbidden in ["spec.rs", "render.rs"] {
        let path = dir.join(forbidden);
        assert!(
            !path.exists(),
            "button_copy directory contract should not include `{forbidden}`."
        );
    }
}
