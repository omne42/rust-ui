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
fn tag_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/tag/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Tag internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn tag_component_layers_follow_file_responsibility_contract() {
    let mod_source = load_source("src/tag/mod.rs");
    let logic_source = load_source("src/tag/logic.rs");
    let styles_source = load_source("src/tag/styles.rs");
    let view_source = load_source("src/tag/view.rs");

    for forbidden in ["pub mod logic", "pub mod view", "resolve_state("] {
        assert!(
            !mod_source.contains(forbidden),
            "Tag mod.rs should keep minimal exports and avoid implementation token `{forbidden}`."
        );
    }

    for forbidden in ["view! {", "<span", "on:click", "web_sys::"] {
        assert!(
            !logic_source.contains(forbidden),
            "Tag logic.rs should stay pure normalization/derivation and avoid `{forbidden}`."
        );
    }

    for forbidden in ["view! {", "on:click", "Callback::new(", "use "] {
        assert!(
            !styles_source.contains(forbidden),
            "Tag styles.rs should remain static CSS and avoid runtime token `{forbidden}`."
        );
    }

    for forbidden in ["TagStateInput {", "normalize_optional_text(class_name)"] {
        assert!(
            !view_source.contains(forbidden),
            "Tag view.rs should not reconstruct state primitives via `{forbidden}`."
        );
    }
}

#[test]
fn tag_does_not_introduce_component_spec_module() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/tag/spec.rs");
    assert!(
        !spec_path.exists(),
        "Tag is a simple component and should not introduce `src/tag/spec.rs`."
    );
}

#[test]
fn tag_uses_logic_state_model() {
    let logic_source = load_source("src/tag/logic.rs");
    let view_source = load_source("src/tag/view.rs");

    for needle in [
        "pub use ui_state_primitives::tag::{",
        "TagVariant",
        "TagSize",
        "TagStateInput",
        "normalize_optional_text",
        "normalize_remove_aria_label",
        "resolve_state",
        "pub struct TagNormalizedInput",
        "pub fn normalize_tag_input(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tag logic should include `{needle}` for primitive-backed state derivation."
        );
    }

    for needle in [
        "logic::normalize_tag_input(",
        "let state = normalized.state;",
        "logic::compose_class_name(normalized.class_name, state)",
        "ui_headless::{A11yDirection, OnPress, locale_attrs}",
        "lang=locale.lang",
        "dir=locale.dir",
    ] {
        assert!(
            view_source.contains(needle),
            "Tag view should derive state via logic helpers; missing `{needle}`."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_remove_aria_label(remove_aria_label)",
        "logic::resolve_state(TagStateInput {",
    ] {
        assert!(
            !view_source.contains(needle),
            "Tag view should not rebuild normalization/state rules; found `{needle}`."
        );
    }
}

#[test]
fn tag_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/tag/view.rs");

    for attr in [
        "data-slot=\"tag\"",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-state=state.state_attr",
        "data-enabled=state.is_enabled.then_some(\"true\")",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-removable=state.is_removable.then_some(\"true\")",
        "data-static=state.is_static.then_some(\"true\")",
        "data-has-remove-handler=state.has_remove_handler.then_some(\"true\")",
        "data-remove-label-source=state.remove_label_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
        "const TAG_CONTENT_SLOT: &str = \"tag-content\";",
        "const TAG_REMOVE_SLOT: &str = \"tag-remove-button\";",
        "data-slot=TAG_CONTENT_SLOT",
        "data-slot=TAG_REMOVE_SLOT",
        "data-label-source=state.remove_label_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Tag should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn tag_styles_include_variant_size_and_state_markers() {
    let source = load_source("src/tag/styles.rs");

    for selector in [
        ".ui-tag--size-sm",
        ".ui-tag[data-size=\"md\"]",
        ".ui-tag--variant-default",
        ".ui-tag[data-variant=\"surface\"]",
        ".ui-tag--enabled",
        ".ui-tag[data-state=\"disabled\"]",
        ".ui-tag[data-state=\"static\"]",
        ".ui-tag[data-state=\"removable\"]",
        ".ui-tag--custom-class",
        ".ui-tag[data-custom-class=\"true\"]",
        ".ui-tag[data-class-source=\"custom\"]",
        ".ui-tag__remove[data-disabled=\"true\"]",
        ".ui-tag__remove[data-label-source=\"custom\"]",
    ] {
        assert!(
            source.contains(selector),
            "Tag styles should include `{selector}` as stable state-marker contracts."
        );
    }

    for token_var in [
        "var(--ui-bg-muted",
        "var(--ui-border",
        "var(--ui-fg",
        "var(--ui-focus-ring",
    ] {
        assert!(
            source.contains(token_var),
            "Tag styles should stay token-first and include `{token_var}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":first-child", ":last-child"] {
        assert!(
            !source.contains(forbidden),
            "Tag styles should not guess state with brittle selector `{forbidden}`."
        );
    }

    for forbidden in ["view! {", "on:click", "Callback::new(", "style="] {
        assert!(
            !source.contains(forbidden),
            "Tag styles should stay static and avoid runtime token `{forbidden}`."
        );
    }
}

#[test]
fn tag_machine_readable_marker_values_are_closed_sets() {
    let primitive_source = load_source("../../crates/ui-state-primitives/src/tag.rs");

    for needle in [
        "pub enum TagVariant",
        "pub enum TagSize",
        "\"default\"",
        "\"surface\"",
        "\"sm\"",
        "\"md\"",
        "\"lg\"",
        "\"disabled\"",
        "\"removable\"",
        "\"static\"",
        "\"custom\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Tag primitive should keep closed marker value token `{needle}`."
        );
    }
}

#[test]
fn tag_tree_shaking_feature_wiring_is_component_scoped() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "component-tag = []",
        "#[cfg(feature = \"component-tag\")]\npub mod tag;",
        "#[cfg(feature = \"component-tag\")]\n    out.push_str(crate::tag::styles::CSS);",
    ] {
        assert!(
            cargo_source.contains(needle)
                || lib_source.contains(needle)
                || css_source.contains(needle),
            "Tag tree-shaking contract should include `{needle}`."
        );
    }
}

#[test]
fn tag_platform_guards_keep_non_wasm_paths_browser_object_free() {
    let mod_source = load_source("src/tag/mod.rs");
    let logic_source = load_source("src/tag/logic.rs");
    let styles_source = load_source("src/tag/styles.rs");
    let view_source = load_source("src/tag/view.rs");

    for forbidden in ["web_sys", "js_sys", "wasm_bindgen", "leptos::web_sys"] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "non-wasm Tag files should stay browser-object free; found `{forbidden}`."
        );
    }

    for forbidden in ["target_arch = \"wasm32\"", "cfg(target_arch"] {
        assert!(
            !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "Tag should keep platform-agnostic component files and avoid ad-hoc platform branch `{forbidden}`."
        );
    }
}

#[test]
fn tag_platform_check_script_covers_native_ssr_and_wasm_compile_paths() {
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "cargo check -p ui-components",
        "cargo check -p ui-components --no-default-features --features component-tag,inject-css",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-tag,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "crates/ui-components/src/tag/view.rs",
        "crates/ui-components/src/tag/logic.rs",
    ] {
        assert!(
            script_source.contains(needle),
            "platform check script should include `{needle}`."
        );
    }
}

#[test]
fn tag_ui_headless_feature_mutex_compile_error_guard_is_present() {
    let headless_source = load_source("../ui-headless/src/lib.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_source.contains(needle),
            "ui-headless should keep feature mutex compile guard `{needle}`."
        );
    }
}

#[test]
fn tag_platform_script_enforces_ui_headless_web_ssr_mutex() {
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "mutually exclusive",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            script_source.contains(needle),
            "platform guard script should enforce ui-headless feature mutex via `{needle}`."
        );
    }
}

#[test]
fn tag_motion_contract_uses_ui_motion_non_wasm_stub_and_keeps_component_safe_without_motion() {
    let ui_motion_source = load_source("../../crates/ui-motion/src/lib.rs");
    let non_wasm_stub_test = load_source("../../crates/ui-motion/tests/non_wasm_stub.rs");
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");
    let view_source = load_source("src/tag/view.rs");
    let logic_source = load_source("src/tag/logic.rs");

    for needle in [
        "Compile on non-wasm targets (no-op stubs) to support SSR/tooling builds.",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion should keep non-wasm no-op/stub contract `{needle}`."
        );
    }

    for needle in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
    ] {
        assert!(
            non_wasm_stub_test.contains(needle),
            "ui-motion non-wasm stub test should include `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep ui-motion compile/stub guard `{needle}`."
        );
    }

    for forbidden in ["ui_motion", "attach_motion", "motion::"] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Tag has no motion contract and should avoid motion binding token `{forbidden}`."
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let motion_path = manifest_dir.join("src/tag/motion.rs");
    assert!(
        !motion_path.exists(),
        "Tag currently has no component motion contract and should not require `src/tag/motion.rs`."
    );
}

#[test]
fn tag_reduced_motion_ssr_wasm_contract_is_n_a_but_semantics_stay_platform_stable() {
    let logic_source = load_source("src/tag/logic.rs");
    let view_source = load_source("src/tag/view.rs");
    let styles_source = load_source("src/tag/styles.rs");
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "cargo check -p ui-components --no-default-features --features component-tag,inject-css",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-tag,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "Tag platform compile-only coverage should include `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"tag\"",
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-remove-label-source=state.remove_label_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Tag should keep hydration-stable semantic marker `{needle}` across SSR/wasm paths."
        );
    }

    for forbidden in [
        "target_arch = \"wasm32\"",
        "cfg(target_arch",
        "prefers_reduced_motion",
        "ui_motion",
        "attach_motion",
        "animation:",
        "transition:",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "Tag has no motion contract; reduced-motion/platform split token `{forbidden}` should be absent."
        );
    }
}

#[test]
fn tag_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");

    for needle in [
        "pub(super) fn tag() -> AnyView",
        "title=\"Tag\"",
        "slug=\"tag\"",
        "title=\"Hello World\"",
        "title=\"Variant + Size Matrix\"",
        "title=\"Removable + Disabled + Custom Class\"",
        "code_signal=hello_world_code",
        "code_signal=matrix_code",
        "code_signal=states_code",
        "test_source_path=\"crates/ui-components/src/tag/view.rs\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "collections_groups docs page should contain `{needle}` for Tag.",
        );
    }
}

#[test]
fn tag_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");

    for needle in [
        "title=\"Variant + Size Matrix\"",
        "variant=TagVariant::Default size=TagSize::Sm",
        "variant=TagVariant::Surface size=TagSize::Lg",
        "title=\"Removable + Disabled + Custom Class\"",
        "remove_aria_label=\"Remove alpha release\".to_string()",
        "class_name=\"docs-tag-custom\".to_string()",
        "disabled=true removable=true",
        "remove count:",
    ] {
        assert!(
            source.contains(needle),
            "tag docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn tag_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../crates/ui-headless/src/perf.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/tag/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_source("src/tag/view.rs");

    for needle in [
        "\"tag\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "max_heap_kb: Some(384.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "Tag docs page should keep performance budget contract token `{needle}`."
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
            "UiPerfProbe should expose performance regression marker `{needle}`."
        );
    }

    let needle = "component_doc!(\"Tag\", \"tag\", \"Collections\", collections_groups::tag)";
    assert!(
        pages_source.contains(needle),
        "Tag docs page should remain in coverage traversal via `{needle}`."
    );

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_source.contains(needle),
            "docs coverage e2e should enforce repeatable perf regression guard `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`."
        );
    }

    for needle in [
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tag checklist should keep render-count baseline/follow-up token `{needle}`."
        );
    }

    for needle in [
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-class-source=state.class_source_attr",
        "data-remove-label-source=state.remove_label_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Tag view should expose attribution marker `{needle}` for perf triage."
        );
    }

    let script_needle = "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_performance_governance_budget_is_defined_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`."
    );
}

#[test]
fn tag_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("src/tag/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    for needle in [
        "fn render_tag_content(children: Children) -> impl IntoView",
        "fn render_remove_button(",
        "let content = render_tag_content(children);",
        "let remove_button = render_remove_button(state, agent_source, remove_aria_label, on_remove);",
        "{content}",
        "{remove_button}",
    ] {
        assert!(
            view_source.contains(needle),
            "Tag view should keep macro complexity split marker `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Tag should keep a single public component boundary."
    );

    let script_needle = "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
}

#[test]
fn tag_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("src/tag/view.rs");

    for needle in [
        "fn render_tag_content(children: Children) -> impl IntoView",
        "fn render_remove_button(",
        ") -> impl IntoView {",
        "pub fn Tag(",
    ] {
        assert!(
            view_source.contains(needle),
            "Tag view should keep function-first split marker `{needle}`."
        );
    }

    for forbidden in [
        "#[component]\nfn render_tag_content(",
        "#[component]\nfn render_remove_button(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Tag local fragments should stay plain functions, not extra components `{forbidden}`."
        );
    }
}

#[test]
fn tag_static_fragments_are_constantized_with_stable_semantics() {
    let view_source = load_source("src/tag/view.rs");

    for needle in [
        "const TAG_CONTENT_CLASS: &str = \"ui-tag__content\";",
        "const TAG_CONTENT_SLOT: &str = \"tag-content\";",
        "const TAG_REMOVE_CLASS: &str = \"ui-tag__remove\";",
        "const TAG_REMOVE_SLOT: &str = \"tag-remove-button\";",
        "const TAG_REMOVE_GLYPH: &str = \"×\";",
        "{TAG_REMOVE_GLYPH}",
    ] {
        assert!(
            view_source.contains(needle),
            "Tag view should keep static fragment constants via `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("TAG_REMOVE_GLYPH").count(),
        2,
        "Tag remove glyph should keep a single constant source + one render usage."
    );
}

#[test]
fn tag_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    for rel_path in [
        "src/tag/mod.rs",
        "src/tag/logic.rs",
        "src/tag/styles.rs",
        "src/tag/view.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in ["inner_html", "set_inner_html", "dangerously_set_inner_html"] {
            assert!(
                !source.contains(forbidden),
                "Tag component source `{rel_path}` must not inject raw html; found `{forbidden}`."
            );
        }
    }

    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    for forbidden in ["inner_html", "set_inner_html", "dangerously_set_inner_html"] {
        assert!(
            !docs_source.contains(forbidden),
            "Tag docs examples must not demonstrate raw html injection token `{forbidden}`."
        );
    }
}

#[test]
fn tag_inner_html_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-components-inner-html.sh");

    let needle = "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(needle),
        "inner-html check script should enforce Tag contract marker `{needle}`."
    );
}

#[test]
fn tag_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let tag_view_source = load_source("src/tag/view.rs");
    let tag_logic_source = load_source("src/tag/logic.rs");

    assert!(
        cargo_source.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "wasm debug capability should stay feature-gated via `button-wasm-debug`."
    );

    let all_components_start = cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = cargo_source[all_components_start..]
        .find("\n\ncomponent-accordion")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before component feature declarations");
    let all_components_block = &cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "wasm debug feature must not be pulled into all-components production path."
    );

    for needle in [
        "data-debug-source=source.clone()",
        "data-debug-before=before_attr",
        "data-debug-after=after_attr",
        "data-debug-timestamp-ms=format!(\"{:.0}\", event.timestamp_ms)",
        "data-slot=\"button-debug-replay\"",
        "request_replay.run(event.source)",
    ] {
        assert!(
            button_view_source.contains(needle),
            "shared button wasm debug path should keep trace/replay marker `{needle}`."
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
        "events.push(event);",
        ".into_iter()",
        ".take(40)",
        "let ts_ms = event.ts_ms;",
        "UiTraceEventKind::Note",
        "UiTraceEventKind::Inspect",
    ] {
        assert!(
            trace_source.contains(needle) || debug_overlay_source.contains(needle),
            "global trace timeline/replay evidence should keep marker `{needle}`."
        );
    }

    for needle in [
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-class-source=state.class_source_attr",
        "data-remove-label-source=state.remove_label_source_attr",
    ] {
        assert!(
            tag_view_source.contains(needle),
            "Tag should keep machine-readable state/source marker `{needle}` for debug attribution."
        );
    }

    for forbidden in [
        "button-wasm-debug",
        "wasm_debug",
        "render_debug_panel(",
        "data-debug-source",
        "request_replay.run(",
    ] {
        assert!(
            !tag_view_source.contains(forbidden) && !tag_logic_source.contains(forbidden),
            "Tag should not duplicate shared wasm debug runtime token `{forbidden}`."
        );
    }
}

#[test]
fn tag_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");

    let needle = "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm debug check script should enforce `{needle}`."
    );
}

#[test]
fn tag_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/tag/mod.rs");
    let logic_source = load_source("src/tag/logic.rs");
    let view_source = load_source("src/tag/view.rs");
    let styles_source = load_source("src/tag/styles.rs");
    let checklist_source = load_source("src/tag/check2.md");

    assert!(
        !manifest_dir.join("src/tag/spec.rs").exists(),
        "Tag should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-tag = []"),
        "Tag feature should stay lightweight without serde/spec dependency fan-out."
    );
    assert!(
        !cargo_source.contains("component-tag = [\"dep:serde\"")
            && !cargo_source.contains("component-tag = [\"dep:serde_json\""),
        "Tag should not opt into serde/spec migration dependencies without explicit schema contract."
    );

    let combined = format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}");
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
            "Tag engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
        );
    }

    for required in [
        "- [ ] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Tag checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
fn tag_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("src/tag/mod.rs"),
        load_source("src/tag/logic.rs"),
        load_source("src/tag/view.rs"),
        load_source("src/tag/styles.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_components::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("tag-wasm-debug"),
        "Tag should not define component-local tracing feature when no local debug event/replay contract exists."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::tag::",
        "const TAG_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Tag should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn tag_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/tag/mod.rs");
    let logic_source = load_source("src/tag/logic.rs");
    let view_source = load_source("src/tag/view.rs");
    let styles_source = load_source("src/tag/styles.rs");

    let sources = [&mod_source, &logic_source, &view_source, &styles_source];
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
                "Tag engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "Tag public module boundary should not leak web_sys types."
    );
}

#[test]
fn tag_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn tag_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present() {
    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
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
fn tag_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-components-entrypoints.sh");

    let needle = "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present";
    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn tag_component_directory_has_standard_file_layout() {
    for required in [
        "src/tag/mod.rs",
        "src/tag/logic.rs",
        "src/tag/styles.rs",
        "src/tag/view.rs",
    ] {
        assert!(
            path_exists(required),
            "tag component directory should include required file `{required}`."
        );
    }

    assert!(
        !path_exists("src/tag/render.rs"),
        "tag component should not drift into `render.rs`; keep rendering in `view.rs`."
    );
    assert!(
        !path_exists("src/tag/motion.rs"),
        "Tag currently has no motion contract; keep `src/tag/motion.rs` absent until a reusable semantic motion mapping exists."
    );
    assert!(
        !path_exists("src/tag/spec.rs"),
        "Tag is a simple component and should not introduce `src/tag/spec.rs`."
    );
}

#[test]
fn tag_mod_rs_keeps_minimal_stable_exports() {
    let mod_source = load_source("src/tag/mod.rs");

    for needle in [
        "mod logic;",
        "mod view;",
        "pub mod styles;",
        "#[cfg(feature = \"component-tag_group\")]\npub mod group;",
        "pub use view::Tag;",
        "pub use logic::{DEFAULT_REMOVE_ARIA_LABEL, TagSize, TagState, TagStateInput, TagVariant};",
    ] {
        assert!(
            mod_source.contains(needle),
            "tag/mod.rs should include stable export marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "web_sys",
        "leptos::web_sys",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "tag/mod.rs should not leak internal/platform marker `{forbidden}`."
        );
    }
}

#[test]
fn tag_component_file_responsibilities_remain_scoped() {
    let logic_source = load_source("src/tag/logic.rs");
    let styles_source = load_source("src/tag/styles.rs");
    let view_source = load_source("src/tag/view.rs");

    for forbidden in [
        "view!",
        "on:click",
        "NodeRef<",
        "web_sys",
        "leptos::html",
        "role=",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "tag/logic.rs should stay normalization-only; found `{forbidden}`."
        );
    }

    for required in ["var(--ui-", "pub const CSS: &str ="] {
        assert!(
            styles_source.contains(required),
            "tag/styles.rs should keep token-first CSS marker `{required}`."
        );
    }

    for forbidden in ["#[component]", "use leptos::", "on:click=", "view!"] {
        assert!(
            !styles_source.contains(forbidden),
            "tag/styles.rs should stay static style contract; found `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn Tag(",
        "logic::normalize_tag_input(",
        "logic::compose_class_name(",
        "ui_headless::{A11yDirection, OnPress, locale_attrs}",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "tag/view.rs should keep rendering + semantics mount marker `{required}`."
        );
    }
}

#[test]
fn tag_component_files_check_script_covers_directory_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    for needle in [
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_component_directory_has_standard_file_layout",
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_mod_rs_keeps_minimal_stable_exports",
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_component_file_responsibilities_remain_scoped",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce `{needle}`."
        );
    }
}

#[test]
fn tag_agent_contract_is_schema_typed_and_machine_readable() {
    let view_source = load_source("src/tag/view.rs");
    let logic_source = load_source("src/tag/logic.rs");

    for needle in [
        "pub enum TagAgentSchemaVersion",
        "pub enum TagAgentIntent",
        "pub enum TagAgentAction",
        "pub enum TagAgentStateAxis",
        "pub enum TagAgentSource",
        "pub enum TagAgentOutputStatus",
        "pub struct TagAgentContract",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tag agent contract typing should include `{needle}`."
        );
    }

    for needle in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-capability-remove=move || {",
        "data-ui-capability-disable=move || {",
    ] {
        assert!(
            view_source.contains(needle),
            "Tag view should mount schemaized agent contract field `{needle}`."
        );
    }
}

#[test]
fn tag_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("src/tag/view.rs");
    let logic_source = load_source("src/tag/logic.rs");
    let styles_source = load_source("src/tag/styles.rs");
    let mod_source = load_source("src/tag/mod.rs");
    let combined = format!("{view_source}\n{logic_source}\n{styles_source}\n{mod_source}");

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
            "Tag Agent Contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn tag_snapshot_baseline_and_streaming_fallback_contract_are_explicit() {
    let view_source = load_source("src/tag/view.rs");
    let logic_source = load_source("src/tag/logic.rs");
    let check2_source = load_source("src/tag/check2.md");

    for needle in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || {",
        "unwrap_or(\"snapshot\")",
        "data-ui-output-status=move || {",
    ] {
        assert!(
            view_source.contains(needle),
            "Tag view should expose explicit snapshot/fallback marker `{needle}`."
        );
    }

    for needle in [
        "TagAgentStreamSupport::Unsupported",
        "TagAgentStreamFallback::FullSnapshot",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tag logic should model stream N/A/fallback contract via `{needle}`."
        );
    }

    for needle in [
        "- [ ] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "- [ ] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
    ] {
        assert!(
            check2_source.contains(needle),
            "tag/check2.md should pin streaming baseline marker `{needle}`."
        );
    }
}

#[test]
fn tag_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("src/tag/check2.md");

    for required in [
        "- [ ] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "Tag/TagGroup 归类为 `Streaming Optional`；当前实现使用 `data-ui-stream-support=\\\"unsupported\\\" + data-ui-stream-fallback=\\\"full-snapshot\\\"`，并输出 `data-ui-output-status`。",
    ] {
        assert!(
            checklist_source.contains(required),
            "tag/check2.md should keep streaming responsibility marker `{required}`."
        );
    }
}

#[test]
fn tag_streaming_optional_scope_keeps_aria_and_data_markers_continuous() {
    let view_source = load_source("src/tag/view.rs");

    for required in [
        "data-slot=\"tag\"",
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-class-source=state.class_source_attr",
        "data-remove-label-source=state.remove_label_source_attr",
        "aria-label=move || remove_aria_label.get_value()",
        "data-ui-stream-mode=move || {",
        ".map(|state| state.get().mode.as_str())",
        "data-ui-output-status=move || {",
        ".map(|state| state.get().output_status.as_str())",
    ] {
        assert!(
            view_source.contains(required),
            "Tag should keep continuous aria/data semantics via `{required}` in optional-streaming scope."
        );
    }
}

#[test]
fn tag_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("src/tag/view.rs");
    let logic_source = load_source("src/tag/logic.rs");
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
            "Tag should keep validation/retry/resilience policy in upper layer; component must not include `{forbidden}`."
        );
    }
}

#[test]
fn tag_streaming_check_script_covers_snapshot_baseline_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_snapshot_baseline_and_streaming_fallback_contract_are_explicit",
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_streaming_optional_scope_keeps_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn tag_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_semantic_markers_changed_in_view_must_be_covered_by_semantics_tests",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn tag_check2_documents_semantics_first_testing_rules() {
    let checklist_source = load_source("src/tag/check2.md");

    for required in [
        "- [ ] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(required),
            "tag/check2.md should keep semantics-first testing rule `{required}`."
        );
    }
}

#[test]
fn tag_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("tests/tag_semantics.rs");

    for required in [
        "tag_emits_baseline_style_state_data_attributes",
        "tag_styles_include_variant_size_and_state_markers",
        "tag_machine_readable_marker_values_are_closed_sets",
        "tag_agent_contract_is_schema_typed_and_machine_readable",
        "tag_streaming_optional_scope_keeps_aria_and_data_markers_continuous",
    ] {
        assert!(
            semantics_source.contains(required),
            "Tag semantic test suite should assert contract marker `{required}`."
        );
    }

    let forbidden_terms = [
        ["assert", "_snapshot"].concat(),
        ["insta", "::"].concat(),
        ["image", "_snapshot"].concat(),
    ];

    for forbidden in forbidden_terms {
        assert!(
            !semantics_source.contains(forbidden.as_str()),
            "Tag semantic test suite should not rely on visual snapshot assertion `{forbidden}` as primary signal."
        );
    }
}

#[test]
fn tag_semantic_markers_changed_in_view_must_be_covered_by_semantics_tests() {
    let view_source = load_source("src/tag/view.rs");
    let semantics_source = load_source("tests/tag_semantics.rs");

    for marker in [
        "data-state=state.state_attr",
        "data-variant=state.variant_attr",
        "data-size=state.size_attr",
        "data-class-source=state.class_source_attr",
        "data-remove-label-source=state.remove_label_source_attr",
        "aria-label=move || remove_aria_label.get_value()",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || {",
    ] {
        assert!(
            view_source.contains(marker),
            "Tag view should keep semantic marker `{marker}`."
        );
        assert!(
            semantics_source.contains(marker),
            "Tag semantic marker `{marker}` changed without matching semantics assertion update."
        );
    }
}

#[test]
fn tag_check2_documents_e2e_selector_and_repeatable_flow_rules() {
    let check2_source = load_source("src/tag/check2.md");

    for needle in [
        "- [ ] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "- [ ] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
    ] {
        assert!(
            check2_source.contains(needle),
            "tag/check2.md should keep e2e selector/repeatable-flow rule `{needle}`."
        );
    }
}

#[test]
fn tag_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_tag_contract.spec.mjs");

    for needle in [
        "body:not(:has(#boot))",
        "[data-component=\"tag\"] [data-slot=\"tag\"]",
        "toHaveAttribute(\"data-ui-schema\", \"ui.tag.agent-contract\")",
        "toHaveAttribute(\"data-ui-stream-support\", \"unsupported\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"full-snapshot\")",
        "toHaveAttribute(\"data-ui-stream-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-output-status\", /verified|submittable/)",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Tag e2e selector/stable-wait contract should include `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "Tag e2e should avoid unstable fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn tag_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_tag_contract.spec.mjs");

    for needle in [
        "docs-app tag key flow is repeatable with semantic breakpoints",
        "page.keyboard.press(\"Enter\")",
        "toHaveAttribute(\"data-ui-action\", \"remove-pointer\")",
        "toHaveAttribute(\"data-ui-source\", \"remove-pointer\")",
        "toHaveAttribute(\"data-ui-output-status\", \"submittable\")",
        "await page.reload();",
        "toHaveAttribute(\"data-ui-action\", \"initialize\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Tag e2e repeatable-flow contract should include `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "waitForTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "Tag e2e key flow should avoid non-semantic/flaky token `{forbidden}`."
        );
    }
}

#[test]
fn tag_e2e_check_script_covers_selector_and_key_flow_contracts() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-tag.sh");

    for needle in [
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui-components --test tag_semantics --no-default-features --features component-tag,inject-css tag_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
    ] {
        assert!(
            script_source.contains(needle),
            "tag e2e check script should enforce `{needle}`."
        );
    }
}

#[test]
fn tag_docs_examples_sync_with_logic_api_names_and_default_matrix() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    let view_source = load_source("src/tag/view.rs");

    for needle in [
        "pub(super) fn tag() -> AnyView",
        "title=\"Tag\"",
        "slug=\"tag\"",
        "<Playground",
        "title=\"Hello World\"",
        "title=\"Variant + Size Matrix\"",
        "title=\"Removable + Disabled + Custom Class\"",
        "<Tag>\"Hello Tag\"</Tag>",
        "<Tag variant=TagVariant::Surface>\"Surface\"</Tag>",
        "variant=TagVariant::Default size=TagSize::Sm",
        "variant=TagVariant::Surface size=TagSize::Lg",
        "removable=true",
        "on_remove=on_remove_alpha",
        "disabled=true removable=true",
    ] {
        assert!(
            docs_source.contains(needle),
            "Tag docs examples should keep matrix/API marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] variant: TagVariant,",
        "#[prop(optional)] size: TagSize,",
        "#[prop(optional)] disabled: bool,",
        "#[prop(optional)] removable: bool,",
        "#[prop(optional)] on_remove: Option<OnPress>,",
        "#[prop(optional, into)] remove_aria_label: Option<String>,",
    ] {
        assert!(
            view_source.contains(needle),
            "Tag public API should keep marker `{needle}` for docs/runtime sync."
        );
    }
}

#[test]
fn tag_docs_entry_exists_and_is_beginner_friendly_default_then_advanced() {
    let check2_source = load_source("src/tag/check2.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");

    for needle in [
        "- [ ] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tag checklist should keep documentation-as-product marker `{needle}`."
        );
    }

    for needle in [
        "title=\"Tag\"",
        "slug=\"tag\"",
        "title=\"Hello World\"",
        "title=\"Variant + Size Matrix\"",
        "title=\"Removable + Disabled + Custom Class\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Tag docs entry should include beginner-to-advanced marker `{needle}`."
        );
    }

    let hello_pos = docs_source
        .find("title=\"Hello World\"")
        .expect("Tag docs should include Hello World playground");
    let matrix_pos = docs_source
        .find("title=\"Variant + Size Matrix\"")
        .expect("Tag docs should include matrix playground");
    let advanced_pos = docs_source
        .find("title=\"Removable + Disabled + Custom Class\"")
        .expect("Tag docs should include advanced playground");

    assert!(
        hello_pos < matrix_pos && matrix_pos < advanced_pos,
        "Tag docs should keep default path before advanced controls."
    );
}

#[test]
fn tag_docs_app_provides_interactive_playground_with_live_props_and_state_preview() {
    let check2_source = load_source("src/tag/check2.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "- [ ] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tag checklist should keep interactive-playground marker `{needle}`."
        );
    }

    for needle in [
        "let (remove_count, set_remove_count) = signal(0_u32);",
        "let on_remove_alpha = Callback::new",
        "let on_remove_beta = Callback::new",
        "title=\"Hello World\"",
        "title=\"Variant + Size Matrix\"",
        "title=\"Removable + Disabled + Custom Class\"",
        "remove count:",
    ] {
        assert!(
            docs_source.contains(needle),
            "Tag docs interactive playground should include `{needle}`."
        );
    }

    for needle in [
        "pub fn Playground(",
        "#[prop(optional, into)] code_signal: Option<Signal<String>>",
        "children: Children,",
        "let resolved_code = Signal::derive(move || {",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground runtime should keep live-preview marker `{needle}`."
        );
    }
}

#[test]
fn tag_docs_source_first_copy_paste_ready_with_imports_source_paths_and_sync() {
    let check2_source = load_source("src/tag/check2.md");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_tag_contract.spec.mjs");

    for needle in [
        "- [ ] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tag checklist should keep source-first copy-ready marker `{needle}`."
        );
    }

    for needle in [
        "test_source_path=\"crates/ui-components/src/tag/view.rs\".to_string()",
        "title=\"Hello World\"",
        "title=\"Variant + Size Matrix\"",
        "title=\"Removable + Disabled + Custom Class\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Tag docs should keep source-first marker `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy pipeline should keep marker `{needle}`."
        );
    }

    for needle in [
        "docs-app tag playground source is copy-paste ready",
        "toHaveAttribute(\"data-copyable\", \"true\")",
        "toHaveAttribute(\"aria-label\", /Copy to clipboard/i)",
        "toContainText(\"use leptos::prelude::*;\")",
        "toContainText(\"use ui_components::*;\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Tag copy-flow e2e should keep marker `{needle}`."
        );
    }
}

#[test]
fn tag_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let check2_source = load_source("src/tag/check2.md");
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_groups.rs");
    let view_source = load_source("src/tag/view.rs");

    for needle in [
        "- [ ] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tag checklist should keep HeroUI/docs-sync marker `{needle}`."
        );
    }

    for needle in [
        "### Tag 同步记录（2026-02-17）",
        "`Tag` 维持 token primitive 定位",
        "component_doc!(\"Tag\", \"tag\", \"Collections\", collections_groups::tag)",
        "`#/components/tag` 可索引访问",
        "`Hello World`、`Variant + Size Matrix`、`Removable + Disabled + Custom Class`",
    ] {
        assert!(
            strategy_source.contains(needle),
            "HeroUI strategy should keep Tag sync marker `{needle}`."
        );
    }

    for needle in ["\"Tag\"", "\"tag\"", "collections_groups::tag"] {
        assert!(
            pages_source.contains(needle),
            "docs catalog should expose Tag token `{needle}`."
        );
    }

    for needle in [
        "title=\"Tag\"",
        "slug=\"tag\"",
        "title=\"Hello World\"",
        "title=\"Variant + Size Matrix\"",
        "title=\"Removable + Disabled + Custom Class\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Tag docs page should keep indexed/example marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] variant: TagVariant,",
        "#[prop(optional)] size: TagSize,",
        "#[prop(optional)] disabled: bool,",
        "#[prop(optional)] removable: bool,",
        "#[prop(optional)] on_remove: Option<OnPress>,",
    ] {
        assert!(
            view_source.contains(needle),
            "Tag API should keep marker `{needle}` for docs/runtime sync."
        );
    }
}
