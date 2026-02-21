use std::fs;
use std::path::Path;

fn workspace_dir() -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"))
        .to_path_buf()
}

fn load_ui_components_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_description_component_source(rel_path: &str) -> String {
    let path = workspace_dir()
        .join("components/description")
        .join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_workspace_source(rel_path: &str) -> String {
    let path = workspace_dir().join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn ui_components_reexports_description_component_crate() {
    let lib_source = load_ui_components_source("src/lib.rs");
    let cargo_source = load_ui_components_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-description\")]")
            && lib_source.contains("pub use ui_description as description;"),
        "ui-components should re-export the external ui-description crate as `description`.",
    );
    assert!(
        cargo_source.contains("component-description = [\"dep:ui-description\"]"),
        "component-description feature should depend on dep:ui-description after extraction.",
    );
    assert!(
        cargo_source.contains(
            "ui-description = { path = \"../../components/description\", optional = true }"
        ),
        "ui-components Cargo.toml should include the optional ui-description dependency.",
    );
}

#[test]
fn description_tree_shaking_feature_gating_contract_is_checked_and_documented() {
    let ui_components_cargo = load_ui_components_source("Cargo.toml");
    let ui_components_lib = load_ui_components_source("src/lib.rs");
    let ui_components_css = load_ui_components_source("src/css.rs");
    let web_demo_cargo = load_workspace_source("apps/web-demo/Cargo.toml");
    let check2_source = load_description_component_source("check2.md");

    for needle in [
        "component-description = [\"dep:ui-description\"]",
        "ui-description = { path = \"../../components/description\", optional = true }",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components Cargo should keep description tree-shaking marker `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-description\")]",
        "pub use ui_description as description;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui-components lib should keep feature gate marker `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-description\")]",
        "out.push_str(crate::description::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "ui-components css should keep feature gate marker `{needle}`.",
        );
    }

    assert!(
        web_demo_cargo.contains(
            "ui-components = { path = \"../../crates/ui-components\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }"
        ) && !web_demo_cargo.contains("all-components"),
        "web-demo should keep source-mode import without implicitly enabling all-components.",
    );

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "已满足（特性树注册）",
        "已满足（`lib.rs` feature 门控）",
        "已满足（`css.rs` feature 门控）",
        "已满足（禁止隐式全量拉起）",
        "已验证（最小特性树）",
        "已验证（web-demo 反向依赖）",
        "tree_shaking_feature_gating_contract_is_checked_and_documented_for_description",
        "description_tree_shaking_feature_gating_contract_is_checked_and_documented",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep tree-shaking gate marker `{needle}`.",
        );
    }
}

#[test]
fn description_does_not_expose_logic_or_view_modules() {
    let source = load_description_component_source("src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Description internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn description_uses_logic_state_model() {
    let logic_source = load_description_component_source("src/logic.rs");
    let view_source = load_description_component_source("src/view.rs");
    let primitive_source = load_workspace_source("crates/ui-state-primitives/src/description.rs");

    for needle in [
        "pub use ui_state_primitives::description::{",
        "pub use ui_headless::A11yDirection;",
        "DescriptionState",
        "DescriptionStateInput",
        "DescriptionTone",
        "normalize_optional_text",
        "normalize_content",
        "normalize_aria_label",
        "resolve_state",
        "pub enum DescriptionElement",
        "pub fn resolve_locale_attrs(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Description logic should include `{needle}` for primitive-consumption state derivation."
        );
    }

    for needle in [
        "pub enum DescriptionTone",
        "pub struct DescriptionStateInput",
        "pub struct DescriptionState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_content(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Description primitive should define `{needle}` in ui-state-primitives."
        );
    }

    for needle in [
        "logic::normalize_content(Some(text))",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_locale_attrs(lang, dir)",
        "logic::resolve_state(DescriptionStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "lang=move || lang.get_value()",
        "dir=move || dir.get_value()",
    ] {
        assert!(
            view_source.contains(needle),
            "Description view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn description_emits_baseline_style_state_data_attributes() {
    let source = load_description_component_source("src/view.rs");

    for attr in [
        "data-slot=\"description\"",
        "slot=\"description\"",
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-truncate=move || state.get().is_truncated.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Description should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn description_styles_include_tone_state_and_markers() {
    let source = load_description_component_source("src/styles.rs");

    for selector in [
        ".ui-description--tone-default",
        ".ui-description[data-tone=\"default\"]",
        ".ui-description--tone-muted",
        ".ui-description[data-tone=\"muted\"]",
        ".ui-description--tone-negative",
        ".ui-description[data-tone=\"negative\"]",
        ".ui-description--disabled",
        ".ui-description[data-disabled=\"true\"]",
        ".ui-description--truncate",
        ".ui-description[data-truncate=\"true\"]",
        ".ui-description--custom-class",
        ".ui-description[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Description styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn description_docs_page_covers_primary_playgrounds() {
    let source = load_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "pub(super) fn description() -> AnyView",
        "title=\"Description\"",
        "slug=\"description\"",
        "description=\"baseline-style form description primitive with centralized tone/state/source contracts and stable slot semantics.\"",
        "<Playground title=\"Hello World\" code_signal=hello_world_code>",
        "text=\"This appears below the field.\".to_string()",
        "<Playground title=\"Tone Variants\" code_signal=tone_code>",
        "<Playground title=\"Truncate + Element + Disabled\" code_signal=truncate_code>",
        "<Description",
        "DescriptionTone::Negative",
        "DescriptionElement::Span",
        "is_truncated=true",
    ] {
        assert!(
            source.contains(needle),
            "forms_extra docs page should include `{needle}` for description primary coverage.",
        );
    }
}

#[test]
fn description_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");

    for needle in [
        "text=\"This appears below the field as guidance.\".to_string()",
        "tone=DescriptionTone::Default",
        "aria_label=\"Name helper\".to_string()",
        "text=\"Optional details are only visible to admins.\".to_string()",
        "tone=DescriptionTone::Muted",
        "text=\"Two-factor code expired. Request a new one.\".to_string()",
        "tone=DescriptionTone::Negative",
        "text=\"A very long assistant text that should truncate in constrained layouts to avoid breaking form rhythm.\".to_string()",
        "element=DescriptionElement::Span",
        "is_truncated=true",
        "class_name=\"docs-description-custom\".to_string()",
        "text=\"Disabled helper text\".to_string()",
        "is_disabled=true",
        "class=\"docs-stack docs-description-limit\"",
    ] {
        assert!(
            source.contains(needle),
            "description docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn description_performance_governance_contract_is_mount_only_traceable_and_blocking() {
    let shell_source = load_workspace_source("apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_workspace_source("apps/docs-app/src/perf_probe.rs");
    let e2e_source = load_workspace_source("e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_workspace_source("docs/plan/TODO.md");
    let script_source = load_workspace_source("scripts/check-ui-components-performance.sh");
    let check2_source = load_workspace_source("components/description/check2.md");
    let view_source = load_description_component_source("src/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "_ => UiPerfBudget::mount_only(120.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep performance budget marker `{needle}`.",
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
            "UiPerfProbe should expose repeatable perf marker `{needle}`.",
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
            "docs e2e should keep blocking perf assertion `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance follow-up plan should keep marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "渲染次数预算为 `1`",
        "render_count",
        "N/A（本组件精确 `render_count`）",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep performance governance marker `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui-components --test description_semantics description_performance_governance_contract_is_mount_only_traceable_and_blocking",
        "cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`.",
        );
    }

    for needle in [
        "data-tone=move || state.get().tone_attr",
        "data-state=move || state.get().data_state_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Description view should expose perf attribution marker `{needle}`.",
        );
    }
}

#[test]
fn description_semantic_and_performance_regression_contract_is_covered_beyond_snapshots() {
    let check2_source = load_description_component_source("check2.md");
    let view_source = load_description_component_source("src/view.rs");
    let component_semantics_source = load_description_component_source("test/semantics.rs");

    for needle in [
        "aria-label=aria_label",
        "data-state=move || state.get().data_state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "description view should keep semantic marker `{needle}` for non-snapshot assertions.",
        );
    }

    for forbidden in [
        "on:keydown",
        "on:keyup",
        "on:keypress",
        "on:click",
        "on:pointerdown",
        "on:pointerup",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "description focus/interaction path should stay N/A and avoid `{forbidden}`.",
        );
    }

    for needle in [
        "fn performance_governance_contract_is_mount_only_traceable_and_blocking()",
        "fn view_mounts_stable_semantic_markers()",
        "fn focus_stack_overlay_gc_is_explicitly_na_for_non_overlay_description_component()",
        "render_count",
        "N/A（本组件精确 `render_count`）",
    ] {
        assert!(
            component_semantics_source.contains(needle) || check2_source.contains(needle),
            "description semantic/perf regression contract should keep marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "已满足（语义断言覆盖）",
        "N/A（焦点流转）",
        "已满足（性能回归与阻断）",
        "semantic_and_performance_regression_contract_is_covered_beyond_snapshots_for_description",
        "description_semantic_and_performance_regression_contract_is_covered_beyond_snapshots",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep semantic/perf regression marker `{needle}`.",
        );
    }
}

#[test]
fn description_view_macro_complexity_is_guarded_by_shallow_blocks() {
    let view_source = load_description_component_source("src/view.rs");
    let check2_source = load_description_component_source("check2.md");

    assert!(
        view_source.contains("match element {"),
        "Description view should split by semantic element branch.",
    );

    assert!(
        view_source.contains("fn render_span(")
            && view_source.contains("fn render_paragraph(")
            && view_source.contains("fn render_div("),
        "Description view should keep local rendering split into plain functions.",
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        3,
        "Description should keep three shallow render blocks for span/p/div.",
    );
    for forbidden in [
        "<section", "<article", "<header", "<footer", "<main", "<aside", "<nav", "<ul", "<ol",
        "<li", "<table", "<tbody", "<tr", "<td",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view should not introduce deep semantic container `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "view_macro_complexity_is_controlled_by_shallow_semantic_blocks",
        "description_view_macro_complexity_is_guarded_by_shallow_blocks",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep macro-complexity marker `{needle}`.",
        );
    }
}

#[test]
fn description_view_prefers_functional_split_over_extra_components() {
    let view_source = load_description_component_source("src/view.rs");
    let check2_source = load_description_component_source("check2.md");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Description view should keep one component entrypoint.",
    );
    assert!(
        view_source.contains("DescriptionElement::Span => render_span(")
            && view_source.contains("DescriptionElement::Paragraph => {")
            && view_source.contains("render_paragraph(class, state, aria_label, lang, dir, text)")
            && view_source.contains("DescriptionElement::Div => render_div("),
        "Description should dispatch render branches through plain helper functions.",
    );
    for helper in ["fn render_span(", "fn render_paragraph(", "fn render_div("] {
        assert!(
            view_source.contains(helper),
            "Description view should keep helper `{helper}`.",
        );
    }

    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "view_prefers_functional_split_over_extra_components",
        "description_view_prefers_functional_split_over_extra_components",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep functional-split marker `{needle}`.",
        );
    }
}

#[test]
fn description_static_fragment_constantization_is_explicitly_scoped_and_accessible() {
    let view_source = load_description_component_source("src/view.rs");
    let check2_source = load_description_component_source("check2.md");

    assert_eq!(
        view_source.matches("{text.get_value()}").count(),
        3,
        "Description should render dynamic text leaf in each element branch rather than duplicating static long-copy fragments.",
    );
    for forbidden in [
        "inner_html",
        "<svg",
        "<footer",
        "<path",
        "<defs",
        "include_str!",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Description view should not introduce heavy static fragment `{forbidden}` in current scope.",
        );
    }
    for helper in ["fn render_span(", "fn render_paragraph(", "fn render_div("] {
        assert!(
            view_source.contains(helper),
            "Description should keep static structure concentrated via helper `{helper}`.",
        );
    }
    for needle in [
        "aria-label=aria_label",
        "lang=move || lang.get_value()",
        "dir=move || dir.get_value()",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "static fragment governance should not regress semantic/a11y marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "N/A（无重静态片段）",
        "static_fragment_constantization_is_explicitly_scoped_and_accessible",
        "description_static_fragment_constantization_is_explicitly_scoped_and_accessible",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep static-fragment marker `{needle}`.",
        );
    }
}

#[test]
fn description_inner_html_usage_is_forbidden_and_safe_text_rendering_is_enforced() {
    let mod_source = load_description_component_source("src/mod.rs");
    let logic_source = load_description_component_source("src/logic.rs");
    let view_source = load_description_component_source("src/view.rs");
    let styles_source = load_description_component_source("src/styles.rs");
    let check2_source = load_description_component_source("check2.md");

    for source in [&mod_source, &logic_source, &view_source, &styles_source] {
        for forbidden in [
            "inner_html",
            "innerHTML",
            "dangerously_set_inner_html",
            "set_inner_html(",
            "insert_adjacent_html(",
            "outer_html",
            "document.write(",
        ] {
            assert!(
                !source.contains(forbidden),
                "Description should not expose HTML injection surface `{forbidden}`.",
            );
        }
    }

    assert_eq!(
        view_source.matches("{text.get_value()}").count(),
        3,
        "Description should keep rendering text through escaped text nodes.",
    );
    for needle in [
        "aria-label=aria_label",
        "lang=move || lang.get_value()",
        "dir=move || dir.get_value()",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "safe text rendering should keep semantic/a11y marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "N/A（无 `inner_html` 节点）",
        "inner_html_usage_is_forbidden_and_safe_text_rendering_is_enforced",
        "description_inner_html_usage_is_forbidden_and_safe_text_rendering_is_enforced",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep inner_html marker `{needle}`.",
        );
    }
}

#[test]
fn description_wasm_debug_contract_is_explicitly_na_and_feature_isolated() {
    let mod_source = load_description_component_source("src/mod.rs");
    let logic_source = load_description_component_source("src/logic.rs");
    let view_source = load_description_component_source("src/view.rs");
    let styles_source = load_description_component_source("src/styles.rs");
    let description_cargo_source = load_description_component_source("Cargo.toml");
    let ui_components_cargo_source = load_workspace_source("crates/ui-components/Cargo.toml");
    let wasm_debug_script_source =
        load_workspace_source("scripts/check-ui-components-wasm-debug.sh");
    let debug_overlay_source = load_workspace_source("apps/docs-app/src/debug_overlay.rs");
    let check2_source = load_description_component_source("check2.md");

    for source in [&mod_source, &logic_source, &view_source, &styles_source] {
        for forbidden in [
            "use_ui_trace(",
            "UiTrace",
            "trace.emit(",
            "trace_id",
            "TraceId",
            "replay",
            "debug-overlay",
            "cfg(target_arch = \"wasm32\")",
        ] {
            assert!(
                !source.contains(forbidden),
                "Description should not host local wasm-debug tracing surface `{forbidden}`.",
            );
        }
    }

    assert!(
        description_cargo_source.contains("[features]\ndefault = []"),
        "Description crate should keep feature surface minimal and default-empty.",
    );
    assert!(
        !description_cargo_source.contains("wasm-debug")
            && !ui_components_cargo_source.contains("description-wasm-debug"),
        "Description should not expose a dedicated wasm-debug feature gate.",
    );
    assert!(
        ui_components_cargo_source.contains("component-description = [\"dep:ui-description\"]"),
        "ui-components should keep description behind component feature isolation.",
    );
    assert!(
        !wasm_debug_script_source.contains("description_semantics")
            && !wasm_debug_script_source.contains("component-description"),
        "wasm-debug gate script should not require a dedicated description debug contract.",
    );

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "shared debug overlay should keep global trace/replay marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "N/A（静态非交互组件）",
        "wasm_debug_contract_is_explicitly_na_and_feature_isolated",
        "description_wasm_debug_contract_is_explicitly_na_and_feature_isolated",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep wasm-debug marker `{needle}`.",
        );
    }
}

#[test]
fn description_motion_contract_is_explicitly_na_for_static_description_scope() {
    let description_cargo_source = load_description_component_source("Cargo.toml");
    let mod_source = load_description_component_source("src/mod.rs");
    let view_source = load_description_component_source("src/view.rs");
    let logic_source = load_description_component_source("src/logic.rs");
    let motion_lib_source = load_workspace_source("crates/ui-motion/src/lib.rs");
    let check2_source = load_description_component_source("check2.md");
    let motion_path = workspace_dir().join("components/description/src/motion.rs");

    assert!(
        !description_cargo_source.contains("ui-motion"),
        "description should not pull ui-motion dependency when no component motion contract is needed.",
    );
    assert!(
        !motion_path.exists(),
        "description should keep src/motion.rs absent for static text scope.",
    );
    for source in [&mod_source, &view_source, &logic_source] {
        for forbidden in [
            "mod motion",
            "pub mod motion",
            "attach_motion",
            "stiffness",
            "damping",
            "MotionOptions",
            "prefers_reduced_motion(",
        ] {
            assert!(
                !source.contains(forbidden),
                "description static scope should not expose motion contract marker `{forbidden}`.",
            );
        }
    }

    for needle in [
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion should keep reduced-motion and non-wasm no-op marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "N/A（静态文本组件）",
        "已满足（全局能力不回退）",
        "已满足（组件边界）",
        "motion_contract_is_explicitly_na_for_static_description_scope",
        "description_motion_contract_is_explicitly_na_for_static_description_scope",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep motion-contract governance marker `{needle}`.",
        );
    }
}

#[test]
fn description_dx_workbench_contract_provides_fast_css_feedback_and_explicit_persistence_na() {
    let forms_extra_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = load_description_component_source("check2.md");

    let description_start = forms_extra_source
        .find("pub(super) fn description() -> AnyView {")
        .expect("forms_extra.rs should define description docs page");
    let description_end = forms_extra_source[description_start..]
        .find("pub(super) fn fieldset() -> AnyView {")
        .map(|offset| description_start + offset)
        .expect("description docs section should end before fieldset page");
    let description_docs = &forms_extra_source[description_start..description_end];

    for needle in [
        "title=\"Workbench\"",
        "description=\"Interactive display/config/code/css-test playground for Description state contracts.\"",
        "code_signal=workbench_code",
        "test_css_source=test_css_source",
        "test_source_path=\"components/description/src/styles.rs\".to_string()",
        "test_config_signal=actual_config",
        "ui_components::description::styles::CSS",
        "let (tone_index, set_tone_index) = signal(Some(0_usize));",
        "let (is_disabled, set_is_disabled) = signal(false);",
        "let (is_truncated, set_is_truncated) = signal(false);",
        "SegmentedControl",
        "Switch checked=is_disabled set_checked=set_is_disabled",
        "Switch checked=is_truncated set_checked=set_is_truncated",
        "Switch checked=custom_aria_label set_checked=set_custom_aria_label",
        "Switch checked=custom_class set_checked=set_custom_class",
    ] {
        assert!(
            description_docs.contains(needle),
            "description docs workbench should keep DX contract marker `{needle}`.",
        );
    }

    assert!(
        !description_docs.contains("Persist workbench state")
            && !description_docs.contains("localStorage")
            && !description_docs.contains("load_calendar_workbench_state()")
            && !description_docs.contains("save_calendar_workbench_state("),
        "description DX scope keeps persistence explicitly N/A and avoids unnecessary local storage coupling.",
    );

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "已满足（样式快速反馈）",
        "已满足（上下文保持）",
        "N/A（可选状态保留）",
        "已满足（隔离画布）",
        "dx_workbench_contract_provides_fast_css_feedback_and_explicit_persistence_na",
        "description_dx_workbench_contract_provides_fast_css_feedback_and_explicit_persistence_na",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep DX governance marker `{needle}`.",
        );
    }
}

#[test]
fn description_documentation_as_product_copy_paste_ready_contract_is_implemented() {
    let forms_extra_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = load_description_component_source("check2.md");

    let description_start = forms_extra_source
        .find("pub(super) fn description() -> AnyView {")
        .expect("forms_extra.rs should define description docs page.");
    let description_end = forms_extra_source[description_start..]
        .find("pub(super) fn fieldset() -> AnyView {")
        .map(|offset| description_start + offset)
        .expect("description docs section should end before fieldset page.");
    let description_docs = &forms_extra_source[description_start..description_end];

    for needle in [
        "let description_imports =",
        "use ui_components::{Description, DescriptionElement, DescriptionTone};",
        "title=\"Hello World\"",
        "title=\"State Matrix (Tone / Disabled / Truncate)\"",
        "title=\"Controlled vs Uncontrolled (Stateless Contract)\"",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "code_imports=description_imports.clone()",
        "Snapshot: email is required",
        "Streaming fallback=snapshot: waiting for final validation",
        "data-slot=\"description-source-first\"",
        "Source-first / Copy-Paste Ready",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "docs-description-source-copy",
        "components/description/src/mod.rs",
        "components/description/src/logic.rs",
        "components/description/src/view.rs",
        "components/description/src/styles.rs",
        "component-description",
        "inject-css",
    ] {
        assert!(
            description_docs.contains(needle),
            "description docs should keep copy-paste-ready marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "已满足（Playground 覆盖）",
        "已满足（Streaming/Snapshot 展现）",
        "已满足（Source-first 一键复制）",
        "已满足（源码与依赖可追溯）",
        "documentation_as_product_copy_paste_ready_contract_is_implemented_for_description",
        "description_documentation_as_product_copy_paste_ready_contract_is_implemented",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep documentation-as-product marker `{needle}`.",
        );
    }
}

#[test]
fn description_docs_examples_and_matrices_are_synced_with_logic_contract() {
    let forms_extra_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let logic_source = load_description_component_source("src/logic.rs");
    let primitive_source = load_workspace_source("crates/ui-state-primitives/src/description.rs");
    let check2_source = load_description_component_source("check2.md");

    let description_start = forms_extra_source
        .find("pub(super) fn description() -> AnyView {")
        .expect("forms_extra.rs should define description docs page.");
    let description_end = forms_extra_source[description_start..]
        .find("pub(super) fn fieldset() -> AnyView {")
        .map(|offset| description_start + offset)
        .expect("description docs section should end before fieldset page.");
    let description_docs = &forms_extra_source[description_start..description_end];

    for needle in [
        "title=\"Hello World\"",
        "title=\"State Matrix (Tone / Disabled / Truncate)\"",
        "title=\"Controlled vs Uncontrolled (Stateless Contract)\"",
        "title=\"Workbench\"",
        "title=\"Tone Variants\"",
        "title=\"Truncate + Element + Disabled\"",
        "DescriptionActualConfig {",
    ] {
        assert!(
            description_docs.contains(needle),
            "description docs should keep synchronized example/matrix marker `{needle}`.",
        );
    }

    for needle in [
        "let (tone_index, set_tone_index) = signal(Some(0_usize));",
        "let (element_index, set_element_index) = signal(Some(0_usize));",
        "let (is_disabled, set_is_disabled) = signal(false);",
        "let (is_truncated, set_is_truncated) = signal(false);",
        "1 => DescriptionTone::Muted,",
        "2 => DescriptionTone::Negative,",
        "_ => DescriptionTone::Default,",
        "1 => DescriptionElement::Span,",
        "2 => DescriptionElement::Div,",
        "_ => DescriptionElement::Paragraph,",
        "if tone != DescriptionTone::Default {",
        "if element != DescriptionElement::Paragraph {",
        "if is_disabled.get() {",
        "if is_truncated.get() {",
    ] {
        assert!(
            description_docs.contains(needle),
            "description docs should keep logic-aligned default/api marker `{needle}`.",
        );
    }

    for needle in [
        "pub struct DescriptionViewModelInput {",
        "pub text: String,",
        "pub tone: DescriptionTone,",
        "pub is_disabled: bool,",
        "pub is_truncated: bool,",
        "pub aria_label: Option<String>,",
        "pub class_name: Option<String>,",
        "pub lang: Option<String>,",
        "pub dir: Option<A11yDirection>,",
        "let state = resolve_state(DescriptionStateInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic should keep api/default-alignment marker `{needle}`.",
        );
    }

    for needle in [
        "#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]",
        "pub enum DescriptionTone {",
        "#[default]",
        "Default,",
        "pub fn resolve_state(input: DescriptionStateInput) -> DescriptionState {",
        "let data_state_attr = if input.disabled {",
    ] {
        assert!(
            primitive_source.contains(needle),
            "state primitive should keep defaults and state-matrix marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "已满足（文档与示例同步）",
        "已满足（状态矩阵覆盖）",
        "已满足（参数矩阵可检视）",
        "已满足（API/默认值对齐 logic）",
        "docs_examples_and_matrices_are_synced_with_description_logic_contract",
        "description_docs_examples_and_matrices_are_synced_with_logic_contract",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep docs/matrix sync marker `{needle}`.",
        );
    }
}

#[test]
fn description_documentation_entry_is_beginner_friendly_with_default_first_and_advanced_later() {
    let readme_source = load_description_component_source("src/README.md");
    let forms_extra_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let check2_source = load_description_component_source("check2.md");

    for needle in [
        "## Quick Start (Use First)",
        "Start with the default API path first. Move to advanced props only when needed.",
        "### Hello World",
        "<Description text=\"This appears below the field.\".to_string() />",
        "### Common Usage",
        "tone=DescriptionTone::Muted",
        "## Advanced Controls (Use When Needed)",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs::description()",
    ] {
        assert!(
            readme_source.contains(needle),
            "README should keep beginner-friendly documentation marker `{needle}`.",
        );
    }

    let quick_start_idx = readme_source
        .find("## Quick Start (Use First)")
        .expect("README should define quick-start section.");
    let advanced_idx = readme_source
        .find("## Advanced Controls (Use When Needed)")
        .expect("README should define advanced section.");
    assert!(
        quick_start_idx < advanced_idx,
        "README should keep default API path before advanced controls.",
    );

    for needle in [
        "pub(super) fn description() -> AnyView {",
        "title=\"Description\"",
        "slug=\"description\"",
    ] {
        assert!(
            forms_extra_source.contains(needle),
            "docs-app should keep accessible description documentation entry `{needle}`.",
        );
    }

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "已满足（文档入口存在）",
        "已满足（零门槛 + 常见用法）",
        "已满足（先用后进阶）",
        "documentation_entry_is_beginner_friendly_with_default_first_and_advanced_later_for_description",
        "description_documentation_entry_is_beginner_friendly_with_default_first_and_advanced_later",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep beginner-friendly documentation marker `{needle}`.",
        );
    }
}

#[test]
fn description_interactive_playground_contract_is_available_with_reproducible_flow() {
    let forms_extra_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let e2e_source = load_workspace_source("e2e/tests/docs_app_description_contract.spec.mjs");
    let check2_source = load_description_component_source("check2.md");

    let description_start = forms_extra_source
        .find("pub(super) fn description() -> AnyView {")
        .expect("forms_extra.rs should define description docs page.");
    let description_end = forms_extra_source[description_start..]
        .find("pub(super) fn fieldset() -> AnyView {")
        .map(|offset| description_start + offset)
        .expect("description docs section should end before fieldset page.");
    let description_docs = &forms_extra_source[description_start..description_end];

    for needle in [
        "title=\"Workbench\"",
        "description=\"Interactive display/config/code/css-test playground for Description state contracts.\"",
        "code_signal=workbench_code",
        "test_config_signal=actual_config",
        "DescriptionActualConfig {",
        "SegmentedControl",
        "selected_index=tone_index",
        "selected_index=element_index",
        "Switch checked=is_disabled set_checked=set_is_disabled",
        "Switch checked=is_truncated set_checked=set_is_truncated",
        "Switch checked=custom_aria_label set_checked=set_custom_aria_label",
        "Switch checked=custom_class set_checked=set_custom_class",
    ] {
        assert!(
            description_docs.contains(needle),
            "description docs should keep interactive playground marker `{needle}`.",
        );
    }

    assert!(
        e2e_source.contains(
            "test(\"docs-app description key flow remains repeatable with semantic ready checkpoints\"",
        ),
        "description e2e suite should keep repeatable key-flow regression for playground acceptance.",
    );

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "已满足（交互工作台）",
        "已满足（props/状态/反馈可观测）",
        "N/A（AI Spec 联动示例）",
        "已满足（可重复关键路径）",
        "interactive_playground_contract_is_available_with_reproducible_flow_for_description",
        "description_interactive_playground_contract_is_available_with_reproducible_flow",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep interactive-playground marker `{needle}`.",
        );
    }
}

#[test]
fn description_source_first_docs_are_copy_paste_ready_with_imports_and_real_source_paths() {
    let forms_extra_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let playground_source = load_workspace_source("apps/docs-app/src/playground.rs");
    let check2_source = load_description_component_source("check2.md");

    let description_start = forms_extra_source
        .find("pub(super) fn description() -> AnyView {")
        .expect("forms_extra.rs should define description docs page.");
    let description_end = forms_extra_source[description_start..]
        .find("pub(super) fn fieldset() -> AnyView {")
        .map(|offset| description_start + offset)
        .expect("description docs section should end before fieldset page.");
    let description_docs = &forms_extra_source[description_start..description_end];

    for needle in [
        "let description_imports =",
        "use ui_components::{Description, DescriptionElement, DescriptionTone};",
        "code_imports=description_imports.clone()",
        "data-slot=\"description-source-first\"",
        "Source-first / Copy-Paste Ready",
        "<code>\"Show code\"</code>",
        "copy button. Snippets are import-ready via ",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "label=\"Copy starter\".to_string()",
        "copyable=true",
        "class_name=\"docs-description-source-copy\".to_string()",
        "components/description/src/mod.rs",
        "components/description/src/logic.rs",
        "components/description/src/view.rs",
        "components/description/src/styles.rs",
        "component-description",
        "inject-css",
    ] {
        assert!(
            description_docs.contains(needle),
            "description docs should keep source-first copy-paste marker `{needle}`.",
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "fn missing_import_lines(raw: &str, imports: &str) -> Vec<String>",
        "if missing_imports.is_empty() {",
        "format!(\"{}\\n\\n{raw}\", missing_imports.join(\"\\n\"))",
        "code_imports: Option<String>",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground pipeline should keep copy-ready import merge marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "已满足（复制按钮 + import-ready）",
        "已满足（源码落点与依赖前提）",
        "已满足（文档与实现同步）",
        "source_first_docs_are_copy_paste_ready_with_imports_and_real_source_paths_for_description",
        "description_source_first_docs_are_copy_paste_ready_with_imports_and_real_source_paths",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep source-first copy-paste marker `{needle}`.",
        );
    }
}

#[test]
fn description_heroui_alignment_strategy_and_docs_entry_are_synced_for_parameter_changes() {
    let heroui_strategy_source =
        load_workspace_source("docs/spec/heroui-parameter-design-strategy.md");
    let forms_extra_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/forms_extra.rs");
    let readme_source = load_description_component_source("src/README.md");
    let check2_source = load_description_component_source("check2.md");

    for needle in [
        "### Description 同步记录（2026-02-20）",
        "参数模型同步：`Description` 参数主轴保持 `text/tone/is_disabled/is_truncated/element/aria_label/class_name/lang/dir`",
        "docs 入口同步：`apps/docs-app/src/pages/components/pages/forms_extra.rs` 通过 `description()` 暴露 `slug=\"description\"` 页面入口",
        "研究文档补充判定：本轮为 Description 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。",
        "HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。",
    ] {
        assert!(
            heroui_strategy_source.contains(needle),
            "HeroUI alignment strategy should keep description sync marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn description() -> AnyView {",
        "title=\"Description\"",
        "slug=\"description\"",
    ] {
        assert!(
            forms_extra_source.contains(needle),
            "docs entry should keep description accessibility marker `{needle}`.",
        );
    }

    for needle in [
        "# Description",
        "## Quick Start (Use First)",
        "apps/docs-app/src/pages/components/pages/forms_extra.rs::description()",
    ] {
        assert!(
            readme_source.contains(needle),
            "README should keep equivalent documentation entry marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "已满足（参数模型同步策略文档）",
        "已满足（组件文档入口可访问）",
        "已满足（实现-文档同步约束）",
        "N/A（研究文档补充）",
        "heroui_alignment_strategy_and_description_docs_entry_are_synced_for_parameter_changes",
        "description_heroui_alignment_strategy_and_docs_entry_are_synced_for_parameter_changes",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep HeroUI alignment governance marker `{needle}`.",
        );
    }
}

#[test]
fn description_engineering_capability_unification_is_structured_and_runtime_agnostic() {
    let description_cargo_source = load_description_component_source("Cargo.toml");
    let protocol_source = load_description_component_source("src/protocol.rs");
    let protocol_test_source = load_description_component_source("test/protocol.rs");
    let mod_source = load_description_component_source("src/mod.rs");
    let logic_source = load_description_component_source("src/logic.rs");
    let view_source = load_description_component_source("src/view.rs");
    let styles_source = load_description_component_source("src/styles.rs");
    let check2_source = load_description_component_source("check2.md");

    assert!(
        description_cargo_source.contains("serde = { version = \"1.0\", features = [\"derive\"] }"),
        "description crate should keep serde dependency for structured protocol contracts.",
    );

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "pub enum DescriptionComponentSchemaVersion",
        "V1,",
        "pub struct DescriptionComponentSpec",
        "#[serde(default)]",
        "pub schema_version: DescriptionComponentSchemaVersion,",
        "Serialize, Deserialize, Default",
    ] {
        assert!(
            protocol_source.contains(needle),
            "description protocol should keep structured serde schema marker `{needle}`.",
        );
    }

    for needle in [
        "fn protocol_types_implement_serde_contract()",
        "assert_serde::<DescriptionComponentSchemaVersion>();",
        "assert_serde::<DescriptionComponentSpec>();",
    ] {
        assert!(
            protocol_test_source.contains(needle),
            "description protocol test should keep serde contract marker `{needle}`.",
        );
    }

    for source in [&mod_source, &logic_source, &view_source, &styles_source] {
        for forbidden in [
            "use tracing::",
            "tracing::info!",
            "tracing::warn!",
            "tracing::error!",
            "tracing::debug!",
            "tokio::",
            "async_std::",
            "async-std",
            "spawn_local(",
            "spawn(",
            "pub async fn",
        ] {
            assert!(
                !source.contains(forbidden),
                "description component should keep runtime-agnostic non-async boundary: forbidden `{forbidden}`.",
            );
        }
    }

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "已满足（serde 结构化协议）",
        "已满足（协议回归）",
        "N/A（spec/config 运行时输入）",
        "N/A（tracing 组件埋点）",
        "N/A（async runtime 绑定）",
        "engineering_capability_unification_is_structured_and_runtime_agnostic",
        "description_engineering_capability_unification_is_structured_and_runtime_agnostic",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep engineering-unification marker `{needle}`.",
        );
    }
}

#[test]
fn description_version_deprecation_migration_contract_is_explicitly_na_without_breaking_upgrade() {
    let protocol_source = load_description_component_source("src/protocol.rs");
    let mod_source = load_description_component_source("src/mod.rs");
    let logic_source = load_description_component_source("src/logic.rs");
    let view_source = load_description_component_source("src/view.rs");
    let styles_source = load_description_component_source("src/styles.rs");
    let check2_source = load_description_component_source("check2.md");

    for needle in [
        "pub enum DescriptionComponentSchemaVersion",
        "V1,",
        "pub struct DescriptionComponentSpec",
        "pub schema_version: DescriptionComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "description protocol should keep stable v1 marker `{needle}`.",
        );
    }

    assert!(
        !protocol_source.contains("V2")
            && !protocol_source.contains("Breaking")
            && !protocol_source.contains("Deprecated"),
        "description protocol should not claim a breaking schema upgrade when none exists.",
    );

    for source in [&mod_source, &logic_source, &view_source, &styles_source] {
        assert!(
            !source.contains("migrate_v1_to_v2")
                && !source.contains("schema_registry")
                && !source.contains("codemod"),
            "description component source should not add fake migration scaffolding without a real breaking upgrade.",
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A（当前变更范围）",
        "已满足（现状可证）",
        "已满足（迁移层不应虚构）",
        "升级触发条件（后续约束）",
        "version_deprecation_migration_contract_is_explicitly_na_without_breaking_upgrade",
        "description_version_deprecation_migration_contract_is_explicitly_na_without_breaking_upgrade",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep deprecation/migration marker `{needle}`.",
        );
    }
}

#[test]
fn description_defensive_variables_use_theme_fallback_chain_without_component_terminal_literals() {
    let styles_source = load_description_component_source("src/styles.rs");
    let theme_css_source = load_workspace_source("crates/ui-theme/src/css.rs");
    let check2_source = load_description_component_source("check2.md");
    let css_literal_start = styles_source
        .find("r#\"")
        .map(|idx| idx + 3)
        .expect("description styles.rs should contain raw CSS literal start.");
    let css_literal_end = styles_source
        .rfind("\"#;")
        .expect("description styles.rs should contain raw CSS literal end.");
    let css_body = &styles_source[css_literal_start..css_literal_end];

    for needle in [
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-checkbox-disabled-opacity, var(--ui-fallback-checkbox-disabled-opacity))",
        "var(--ui-button-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset))",
        "var(--ui-border-width, var(--ui-fallback-border-width)) solid color-mix(",
    ] {
        assert!(
            styles_source.contains(needle),
            "description styles should keep defensive variable chain marker `{needle}`.",
        );
    }

    for forbidden in ["0.68", "2px", "outline: 1px solid"] {
        assert!(
            !css_body.contains(forbidden),
            "description styles should not keep component terminal literal `{forbidden}`.",
        );
    }
    assert!(
        !css_body.contains('#'),
        "description CSS body should not hardcode hex literals.",
    );

    for needle in [
        "--ui-fallback-checkbox-disabled-opacity:",
        "--ui-fallback-button-focus-outline-offset:",
        "--ui-fallback-border-width:",
    ] {
        assert!(
            theme_css_source.contains(needle),
            "ui-theme should remain SSOT for fallback terminal variable `{needle}`.",
        );
    }

    for needle in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "已满足（双层回退链）",
        "已满足（移除组件终值）",
        "已满足（SSOT 来源）",
        "defensive_variables_use_theme_fallback_chain_without_component_terminal_literals",
        "description_defensive_variables_use_theme_fallback_chain_without_component_terminal_literals",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep defensive-variables governance marker `{needle}`.",
        );
    }
}

#[test]
fn description_cascade_layer_coverage_uses_ui_layer_and_rejects_plain_inline_styles() {
    let view_source = load_description_component_source("src/view.rs");
    let css_aggregate_source = load_ui_components_source("src/css.rs");
    let check2_source = load_description_component_source("check2.md");

    let layer_start = css_aggregate_source
        .find("out.push_str(\"\\n@layer ui {\\n\");")
        .expect("ui-components css aggregation should open @layer ui block.");
    let description_push = css_aggregate_source
        .find("out.push_str(crate::description::styles::CSS);")
        .expect("ui-components css aggregation should include description styles.");
    let layer_end = css_aggregate_source
        .rfind("out.push_str(\"\\n}\\n\");")
        .expect("ui-components css aggregation should close @layer ui block.");

    assert!(
        layer_start < description_push && description_push < layer_end,
        "description css should be aggregated inside @layer ui boundaries.",
    );
    assert!(
        !view_source.contains("style=") && !view_source.contains("style:"),
        "description view should not emit plain inline style or runtime style bindings.",
    );

    for needle in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "已满足（层级聚合）",
        "已满足（运行时样式边界）",
        "N/A（CSS 变量运行时调节）",
        "cascade_layer_coverage_uses_ui_layer_and_rejects_plain_inline_styles",
        "description_cascade_layer_coverage_uses_ui_layer_and_rejects_plain_inline_styles",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep cascade-layer governance marker `{needle}`.",
        );
    }
}

#[test]
fn description_ui_components_entrypoints_layout_contract_is_correct_and_forbidden_files_absent() {
    let ui_components_lib_source = load_ui_components_source("src/lib.rs");
    let ui_components_css_source = load_ui_components_source("src/css.rs");
    let ui_components_root_source = load_ui_components_source("src/root.rs");
    let active_highlight_source =
        load_workspace_source("crates/ui-visual-primitive/src/active_highlight.rs");
    let check2_source = load_description_component_source("check2.md");

    assert!(
        ui_components_lib_source.contains("#[cfg(feature = \"component-description\")]")
            && ui_components_lib_source.contains("pub use ui_description as description;"),
        "ui-components lib.rs should keep feature-gated description re-export contract.",
    );
    assert!(
        !ui_components_lib_source.contains("pub use web_sys")
            && !ui_components_lib_source.contains("pub use leptos::web_sys"),
        "ui-components public entry should not leak web-sys detail types.",
    );

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "#[cfg(feature = \"component-description\")]",
        "out.push_str(crate::description::styles::CSS);",
    ] {
        assert!(
            ui_components_css_source.contains(needle),
            "ui-components css entry should keep `{needle}`.",
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root_source.contains(needle),
            "UiRoot should keep centralized injection marker `{needle}`.",
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`.",
        );
    }

    for forbidden in ["Accordion", "Description", "Menu"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic and avoid component-specific semantics `{forbidden}`.",
        );
    }

    for rel in [
        "crates/ui-components/src/overlay_open.rs",
        "crates/ui-components/src/presence.rs",
        "crates/ui-components/src/a11y.rs",
    ] {
        let path = workspace_dir().join(rel);
        assert!(
            !path.exists(),
            "forbidden ui-components entrypoint file should be absent: {}",
            path.display(),
        );
    }

    for needle in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "已满足（入口与导出边界）",
        "已满足（CSS 聚合边界）",
        "已满足（Root 注入集中）",
        "已满足（共享视觉原语落点）",
        "已满足（禁置文件不存在）",
        "ui_components_entrypoints_layout_contract_is_correct_and_forbidden_files_absent",
        "description_ui_components_entrypoints_layout_contract_is_correct_and_forbidden_files_absent",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep ui-components entrypoint governance marker `{needle}`.",
        );
    }
}

#[test]
fn description_component_directory_standard_layout_contract_is_correct() {
    let mod_source = load_description_component_source("src/mod.rs");
    let logic_source = load_description_component_source("src/logic.rs");
    let styles_source = load_description_component_source("src/styles.rs");
    let view_source = load_description_component_source("src/view.rs");
    let check2_source = load_description_component_source("check2.md");

    for required in ["src/mod.rs", "src/logic.rs", "src/styles.rs", "src/view.rs"] {
        let path = workspace_dir()
            .join("components/description")
            .join(required);
        assert!(
            path.exists(),
            "required description component file should exist: {}",
            path.display(),
        );
    }

    for forbidden in ["src/render.rs", "src/motion.rs", "src/spec.rs"] {
        let path = workspace_dir()
            .join("components/description")
            .join(forbidden);
        assert!(
            !path.exists(),
            "forbidden/non-applicable description component file should be absent: {}",
            path.display(),
        );
    }

    for needle in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Description;",
    ] {
        assert!(
            mod_source.contains(needle),
            "description mod.rs should keep minimal export marker `{needle}`.",
        );
    }
    assert!(
        !mod_source.contains("pub mod view")
            && !mod_source.contains("pub mod logic")
            && !mod_source.contains("fn "),
        "description mod.rs should not over-export internals or host implementation logic.",
    );

    for needle in [
        "pub struct DescriptionViewModelInput",
        "pub struct DescriptionViewModel",
        "pub fn resolve_view_model(",
        "pub fn compose_class_name(",
        "pub use ui_state_primitives::description::{",
    ] {
        assert!(
            logic_source.contains(needle),
            "description logic.rs should keep normalization/derivation marker `{needle}`.",
        );
    }
    assert!(
        !logic_source.contains("view!")
            && !logic_source.contains("<div")
            && !logic_source.contains("var(--"),
        "description logic.rs should not carry render/css concerns.",
    );

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
    ] {
        assert!(
            styles_source.contains(needle),
            "description styles.rs should keep static token CSS marker `{needle}`.",
        );
    }
    assert!(
        !styles_source.contains("view!")
            && !styles_source.contains("on:")
            && !styles_source.contains("spawn("),
        "description styles.rs should stay static and non-interactive.",
    );

    for needle in [
        "logic::resolve_view_model(logic::DescriptionViewModelInput",
        "fn render_span(",
        "fn render_paragraph(",
        "fn render_div(",
    ] {
        assert!(
            view_source.contains(needle),
            "description view.rs should keep render/mount marker `{needle}`.",
        );
    }
    assert!(
        !view_source.contains("mod render")
            && !view_source.contains("include!(\"render")
            && !view_source.contains("resolve_state(DescriptionStateInput {"),
        "description view.rs should not drift to render.rs or hide state normalization decisions.",
    );

    for needle in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "已满足（核心文件齐备）",
        "已满足（禁止 `render.rs` 漂移）",
        "N/A（`motion.rs`）",
        "N/A（`spec.rs`）",
        "- [x] 组件目录标准文件落点正确。",
        "已满足（mod.rs 导出面最小）",
        "已满足（logic.rs 归一派生边界）",
        "已满足（styles.rs 静态 token-first）",
        "已满足（view.rs 渲染与语义挂载）",
        "N/A（motion.rs）",
        "N/A（spec.rs）",
        "component_directory_standard_layout_contract_is_correct_for_description",
        "description_component_directory_standard_layout_contract_is_correct",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep component-directory marker `{needle}`.",
        );
    }
}

#[test]
fn description_spec_builder_contract_is_explicitly_na_for_description_scope() {
    let mod_source = load_description_component_source("src/mod.rs");
    let check2_source = load_description_component_source("check2.md");
    let spec_path = workspace_dir().join("components/description/src/spec.rs");

    assert!(
        !spec_path.exists(),
        "description should keep src/spec.rs absent when no complex spec builder is required.",
    );
    for forbidden in [
        "mod spec",
        "pub mod spec",
        "DescriptionSpec",
        "DescriptionComponentSpec::new(",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "description mod.rs should not expose spec-builder API marker `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "N/A（当前组件复杂度）",
        "已满足（禁置与边界）",
        "迁移预留（升级路径明确）",
        "spec_builder_contract_is_explicitly_na_for_description_scope",
        "description_spec_builder_contract_is_explicitly_na_for_description_scope",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep Hyper-Structure Builder marker `{needle}`.",
        );
    }
}

#[test]
fn description_context_compression_manifest_and_rbi_projection_are_present_and_synced() {
    let check2_source = load_description_component_source("check2.md");
    let manifest_source = load_description_component_source("src/Component.toml");
    let rbi_source = load_description_component_source("src/description.rbi");
    let manifest_path = workspace_dir().join("components/description/src/Component.toml");
    let rbi_path = workspace_dir().join("components/description/src/description.rbi");

    assert!(
        manifest_path.exists() && rbi_path.exists(),
        "description should provide both context-compression files: {} and {}",
        manifest_path.display(),
        rbi_path.display(),
    );

    for needle in [
        "schema_version = \"1\"",
        "name = \"Description\"",
        "crate = \"ui-description\"",
        "name = \"text\"",
        "name = \"tone\"",
        "name = \"is_disabled\"",
        "name = \"is_truncated\"",
        "name = \"element\"",
        "name = \"aria_label\"",
        "name = \"class_name\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "description Component.toml should keep manifest marker `{needle}`.",
        );
    }

    for needle in [
        "pub type DescriptionTone = ui_state_primitives::description::DescriptionTone;",
        "pub enum DescriptionElement {",
        "pub fn Description(",
        "text: String,",
        "tone: DescriptionTone,",
        "is_disabled: bool,",
        "is_truncated: bool,",
        "element: DescriptionElement,",
        "aria_label: Option<String>,",
        "class_name: Option<String>,",
        "lang: Option<String>,",
        "dir: Option<A11yDirection>,",
    ] {
        assert!(
            rbi_source.contains(needle),
            "description.rbi should keep interface projection marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "已满足（Manifest 落位）",
        "已满足（RBI 投影落位）",
        "已满足（Manifest/RBI 同步）",
        "context_compression_manifest_and_rbi_projection_are_present_and_synced",
        "description_context_compression_manifest_and_rbi_projection_are_present_and_synced",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep context-compression marker `{needle}`.",
        );
    }
}

#[test]
fn description_agent_contract_schema_markers_are_typed_traceable_and_whitelisted() {
    let logic_source = load_description_component_source("src/logic.rs");
    let view_source = load_description_component_source("src/view.rs");
    let manifest_source = load_description_component_source("src/Component.toml");
    let rbi_source = load_description_component_source("src/description.rbi");
    let check2_source = load_description_component_source("check2.md");

    for needle in [
        "pub const DESCRIPTION_AGENT_SCHEMA: &str = \"ui.description.agent-contract.v1\";",
        "pub const DESCRIPTION_AGENT_SCHEMA_VERSION: &str = \"v1\";",
        "pub enum DescriptionAgentIntent",
        "pub enum DescriptionAgentAction",
        "pub enum DescriptionAgentSource",
        "pub struct DescriptionAgentContractAttrs",
        "pub fn resolve_agent_contract_attrs(state: DescriptionState) -> DescriptionAgentContractAttrs",
        "state_attr: state.data_state_attr,",
    ] {
        assert!(
            logic_source.contains(needle),
            "description logic should keep typed agent-contract marker `{needle}`.",
        );
    }
    assert!(
        !logic_source.contains("format!(\"data-ui-")
            && !logic_source.contains("format!(\"ui.description"),
        "description agent contract attrs should not be assembled via ad-hoc string formatting.",
    );

    for needle in [
        "data-ui-schema=move || agent_contract.get().schema_attr",
        "data-ui-schema-version=move || agent_contract.get().schema_version_attr",
        "data-ui-intent=move || agent_contract.get().intent_attr",
        "data-ui-action=move || agent_contract.get().action_attr",
        "data-ui-state=move || agent_contract.get().state_attr",
        "data-ui-source=move || agent_contract.get().source_attr",
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "description view should mount agent-contract semantic marker `{needle}`.",
        );
    }

    for needle in [
        "[agent_contract]",
        "schema = \"ui.description.agent-contract.v1\"",
        "intent = \"text-assistance\"",
        "[[agent_contract_markers]]",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-output-status\"",
        "[[agent_contract_whitelist]]",
        "typed_agent_contract_from_logic::resolve_agent_contract_attrs",
        "dangerously_set_inner_html",
        "<script",
    ] {
        assert!(
            manifest_source.contains(needle),
            "description Component.toml should keep agent-contract/whitelist marker `{needle}`.",
        );
    }

    for needle in [
        "pub const DESCRIPTION_AGENT_SCHEMA: &str;",
        "pub enum DescriptionAgentIntent",
        "pub enum DescriptionAgentAction",
        "pub enum DescriptionAgentSource",
        "pub struct DescriptionAgentContractAttrs",
        "pub fn resolve_agent_contract_attrs(state: DescriptionState) -> DescriptionAgentContractAttrs;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "description.rbi should expose typed agent-contract projection `{needle}`.",
        );
    }

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "已满足（Schema 化挂载）",
        "已满足（类型化生成）",
        "已满足（可追溯映射）",
        "已满足（白名单边界）",
        "agent_contract_schema_markers_are_typed_traceable_and_whitelisted",
        "description_agent_contract_schema_markers_are_typed_traceable_and_whitelisted",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep agent-contract governance marker `{needle}`.",
        );
    }
}

#[test]
fn description_streaming_definition_is_limited_to_llm_output_modes_and_snapshot_contract() {
    let logic_source = load_description_component_source("src/logic.rs");
    let view_source = load_description_component_source("src/view.rs");
    let manifest_source = load_description_component_source("src/Component.toml");
    let check2_source = load_description_component_source("check2.md");

    for needle in [
        "pub enum DescriptionAgentAction {",
        "RenderSnapshot,",
        "pub enum DescriptionAgentStreamSupport {",
        "Optional,",
        "pub enum DescriptionAgentStreamFallback {",
        "Snapshot,",
    ] {
        assert!(
            logic_source.contains(needle),
            "description logic should keep streaming/snapshot contract marker `{needle}`.",
        );
    }

    for needle in [
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "description view should keep stream contract marker `{needle}`.",
        );
    }
    assert!(
        !view_source.contains("data-ui-stream-mode")
            && !view_source.contains("data-ui-stream-state")
            && !view_source.contains("data-ui-stream-phase"),
        "description view should not expose undefined third streaming-mode axis.",
    );

    for needle in [
        "output_mode_axis = [\"snapshot\"]",
        "action_axis = [\"render-snapshot\"]",
        "attr = \"data-ui-stream-support\"",
        "attr = \"data-ui-stream-fallback\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "description Component.toml should keep snapshot-only output mode marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "已满足（术语收敛）",
        "已满足（契约落点）",
        "已满足（无第三模式漂移）",
        "streaming_definition_is_limited_to_llm_output_modes_and_snapshot_contract",
        "description_streaming_definition_is_limited_to_llm_output_modes_and_snapshot_contract",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep streaming-definition marker `{needle}`.",
        );
    }
}

#[test]
fn description_snapshot_is_base_capability_and_renders_complete_results_stably() {
    let logic_source = load_description_component_source("src/logic.rs");
    let view_source = load_description_component_source("src/view.rs");
    let check2_source = load_description_component_source("check2.md");

    for needle in [
        "pub fn resolve_view_model(input: DescriptionViewModelInput) -> DescriptionViewModel {",
        "let text = normalize_content(Some(input.text));",
        "let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);",
        "let class_name = normalize_optional_text(input.class_name);",
        "pub enum DescriptionAgentAction {",
        "RenderSnapshot,",
        "pub enum DescriptionAgentOutputStatus {",
        "Verified,",
        "action_attr: DescriptionAgentAction::RenderSnapshot.as_attr(),",
        "output_status_attr: DescriptionAgentOutputStatus::Verified.as_attr(),",
    ] {
        assert!(
            logic_source.contains(needle),
            "description logic should keep snapshot baseline marker `{needle}`.",
        );
    }

    for needle in [
        "DescriptionElement::Span => {",
        "DescriptionElement::Paragraph => {",
        "DescriptionElement::Div => {",
        "data-ui-action=move || agent_contract.get().action_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "description view should keep stable snapshot rendering marker `{needle}`.",
        );
    }
    assert!(
        view_source.matches("{text.get_value()}").count() == 3,
        "description should render complete snapshot text content in all element branches.",
    );

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "已满足（完整结果消费）",
        "已满足（稳定渲染）",
        "已满足（快照契约可读）",
        "snapshot_is_base_capability_and_renders_complete_results_stably",
        "description_snapshot_is_base_capability_and_renders_complete_results_stably",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep snapshot-baseline marker `{needle}`.",
        );
    }
}

#[test]
fn description_streaming_requirement_is_optional_for_description_with_snapshot_fallback_and_status_markers()
 {
    let logic_source = load_description_component_source("src/logic.rs");
    let view_source = load_description_component_source("src/view.rs");
    let manifest_source = load_description_component_source("src/Component.toml");
    let check2_source = load_description_component_source("check2.md");

    for needle in [
        "pub enum DescriptionAgentStreamSupport {",
        "Optional,",
        "pub enum DescriptionAgentStreamFallback {",
        "Snapshot,",
        "pub enum DescriptionAgentOutputStatus {",
        "Verified,",
    ] {
        assert!(
            logic_source.contains(needle),
            "description logic should keep streaming-optional marker `{needle}`.",
        );
    }
    assert!(
        !logic_source.contains("retry")
            && !logic_source.contains("reconnect")
            && !logic_source.contains("backoff")
            && !logic_source.contains("validate_remote"),
        "description logic should not own retry/recovery/validation policy.",
    );

    for needle in [
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
        "aria-label=aria_label",
        "data-state=move || state.get().data_state_attr",
        "data-ui-state=move || agent_contract.get().state_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "description view should keep readable stream/status marker `{needle}`.",
        );
    }
    assert!(
        !view_source.contains("retry")
            && !view_source.contains("reconnect")
            && !view_source.contains("on:error"),
        "description view should stay render-only without retry/recovery flow.",
    );

    for needle in [
        "output_mode_axis = [\"snapshot\"]",
        "attr = \"data-ui-stream-support\"",
        "values = [\"optional\"]",
        "attr = \"data-ui-stream-fallback\"",
        "values = [\"snapshot\"]",
        "attr = \"data-ui-output-status\"",
        "values = [\"verified\"]",
    ] {
        assert!(
            manifest_source.contains(needle),
            "description Component.toml should keep optional-streaming marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "已判定（职责分级）",
        "已满足（Optional + Fallback）",
        "已满足（输出状态连续可读）",
        "已满足（职责边界）",
        "streaming_requirement_is_optional_for_description_with_snapshot_fallback_and_status_markers",
        "description_streaming_requirement_is_optional_for_description_with_snapshot_fallback_and_status_markers",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep streaming-responsibility marker `{needle}`.",
        );
    }
}

#[test]
fn description_rust_hygiene_contract_is_clean_and_cow_based() {
    let logic_source = load_description_component_source("src/logic.rs");
    let check2_source = load_description_component_source("check2.md");

    for rel_path in ["src/mod.rs", "src/logic.rs", "src/styles.rs", "src/view.rs"] {
        let source = load_description_component_source(rel_path);
        assert!(
            !source.contains("unwrap(")
                && !source.contains("unwrap_err(")
                && !source.contains("expect("),
            "description non-test source `{rel_path}` must not contain unwrap/expect.",
        );
        assert!(
            !source.contains("let _ ="),
            "description non-test source `{rel_path}` must not swallow results with `let _ =`.",
        );
    }

    for needle in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> =",
        "Cow::Borrowed(\"ui-description\")",
        "Cow::Borrowed(state.tone_class)",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic_source.contains(needle),
            "description logic should keep Cow-based string composition marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "已满足（组件源码禁用危险模式）",
        "已满足（字符串复制热点收敛）",
        "已执行（仓库脚本）",
        "rust_hygiene_contract_for_description_is_clean_and_cow_based",
        "description_rust_hygiene_contract_is_clean_and_cow_based",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep rust-hygiene marker `{needle}`.",
        );
    }
}

#[test]
fn description_check2_marks_architecture_layer_definitions_complete() {
    let check2_source = load_description_component_source("src/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。",
        "- [x] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。",
        "- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。",
        "- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。",
        "- [x] `ui-components` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep architecture-layer marker `{needle}`.",
        );
    }
}

#[test]
fn description_check2_marks_semantics_first_testing_complete() {
    let view_source = load_description_component_source("src/view.rs");
    let component_semantics_source = load_description_component_source("test/semantics.rs");
    let check2_source = load_description_component_source("check2.md");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "已满足（语义测试落点）",
        "已满足（契约断言优先）",
        "N/A（显式 role）",
        "已满足（字段变更同步回归）",
        "semantics_first_contract_prioritizes_data_aria_role_and_state_source_over_snapshots",
        "description_semantics_first_contract_prioritizes_data_aria_role_and_state_source_over_snapshots",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep semantics-first marker `{needle}`.",
        );
    }

    for needle in [
        "aria-label=aria_label",
        "data-state=move || state.get().data_state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-ui-source=move || agent_contract.get().source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "description view should keep semantics-first marker `{needle}`.",
        );
    }
    assert!(
        !view_source.contains("role="),
        "description should keep native text semantics and avoid forced widget role override.",
    );

    for forbidden in [
        "assert_snapshot!",
        "insta::assert_snapshot",
        "to_match_snapshot",
    ] {
        assert!(
            !component_semantics_source.contains(forbidden),
            "description component semantics should not use visual snapshot assertion `{forbidden}` as primary gate.",
        );
    }
}

#[test]
fn description_semantics_first_contract_prioritizes_data_aria_role_and_state_source_over_snapshots()
{
    let check2_source = load_description_component_source("check2.md");
    let view_source = load_description_component_source("src/view.rs");
    let component_semantics_source = load_description_component_source("test/semantics.rs");
    let workspace_semantics_source = load_ui_components_source("tests/description_semantics.rs");

    for needle in [
        "aria-label=aria_label",
        "data-state=move || state.get().data_state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-ui-source=move || agent_contract.get().source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "description view should keep semantics-first marker `{needle}`.",
        );
    }
    assert!(
        !view_source.contains("role="),
        "description should keep native text semantics and avoid forced widget role override.",
    );

    for forbidden in [
        "assert_snapshot!",
        "insta::assert_snapshot",
        "to_match_snapshot",
    ] {
        assert!(
            !component_semantics_source.contains(forbidden)
                && !workspace_semantics_source.contains(forbidden),
            "semantics-first suites should not rely on visual snapshot assertion `{forbidden}`.",
        );
    }

    for needle in [
        "fn view_mounts_stable_semantic_markers()",
        "fn type_system_and_semantic_markers_form_machine_readable_state_contract()",
        "fn semantic_and_performance_regression_contract_is_covered_beyond_snapshots_for_description()",
    ] {
        assert!(
            component_semantics_source.contains(needle),
            "description component semantics suite should keep contract test `{needle}`.",
        );
    }

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "已满足（语义测试落点）",
        "已满足（契约断言优先）",
        "N/A（显式 role）",
        "已满足（字段变更同步回归）",
        "semantics_first_contract_prioritizes_data_aria_role_and_state_source_over_snapshots",
        "description_semantics_first_contract_prioritizes_data_aria_role_and_state_source_over_snapshots",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep semantics-first marker `{needle}`.",
        );
    }
}

#[test]
fn description_e2e_selector_contract_uses_semantic_markers_and_wasm_stable_waits() {
    let e2e_source = load_workspace_source("e2e/tests/docs_app_description_contract.spec.mjs");
    let check2_source = load_description_component_source("check2.md");

    for needle in [
        "const DESCRIPTION_PAGE = \"/#/components/description\";",
        "body:not(:has(#boot))",
        "[data-component=\"description\"][data-slot=\"description\"]",
        "[data-slot=\"description\"][data-tone=\"default\"][data-state=\"default\"]",
        "[data-slot=\"description\"][data-aria-source=\"custom\"]",
        "[data-slot=\"description\"][data-class-source=\"custom\"][data-custom-class=\"true\"]",
        "toHaveAttribute(\"data-ui-schema\", \"ui.description.agent-contract.v1\")",
        "toHaveAttribute(\"data-ui-stream-support\", \"optional\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "not.toHaveAttribute(\"role\", /.+/)",
        "toHaveAttribute(\"data-state\", \"default\")",
        "toHaveAttribute(\"data-ui-state\", \"default\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "description e2e should keep semantic selector/wait marker `{needle}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        ".docs-page-title",
        "section.playground",
        "nth-child",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "description e2e should avoid brittle selector/sleep marker `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "已满足（语义选择器优先）",
        "已满足（WASM 稳定等待）",
        "N/A（async/motion ready-settled）",
        "e2e_selector_contract_uses_semantic_markers_and_wasm_stable_waits",
        "description_e2e_selector_contract_uses_semantic_markers_and_wasm_stable_waits",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep e2e selector stability marker `{needle}`.",
        );
    }
}

#[test]
fn description_repeatable_key_flow_is_in_e2e_regression_set_with_semantic_breakpoints() {
    let e2e_source = load_workspace_source("e2e/tests/docs_app_description_contract.spec.mjs");
    let check2_source = load_description_component_source("check2.md");

    for needle in [
        "test(\"docs-app description key flow remains repeatable with semantic ready checkpoints\"",
        "await page.goto(\"/#/components/error-message\");",
        "await expect(reloadedDefaultDescription).toHaveAttribute(\"data-state\", \"default\");",
        "await expect(reloadedDefaultDescription).toHaveAttribute(\"data-ui-state\", \"default\");",
        "await expect(reloadedDefaultDescription).toHaveAttribute(\"data-ui-output-status\", \"verified\");",
        "toHaveAttribute(\"data-ui-action\", \"render-snapshot\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "description repeatable e2e flow should keep semantic breakpoint marker `{needle}`.",
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "description repeatable e2e flow should not use unstable wait marker `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "已满足（可重复关键流程）",
        "已满足（可定位语义断点）",
        "N/A（高风险交互路径）",
        "repeatable_key_flow_is_in_e2e_regression_set_with_semantic_breakpoints",
        "description_repeatable_key_flow_is_in_e2e_regression_set_with_semantic_breakpoints",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep repeatable key-flow e2e marker `{needle}`.",
        );
    }
}

#[test]
fn description_check2_marks_final_merge_gates_complete() {
    let check2_source = load_description_component_source("src/check2.md");

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
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "description/check2.md should keep final-gate marker `{needle}`.",
        );
    }
}

#[test]
fn description_check2_has_no_remaining_unchecked_items() {
    let check2_source = load_description_component_source("src/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "description/check2.md should not keep unchecked checklist items once governance is complete."
    );
}
