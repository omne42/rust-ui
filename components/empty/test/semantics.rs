use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn component_assembly_uses_state_primitives_and_keeps_layer_boundary() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    assert!(
        logic_source.contains("pub use ui_state_primitives::empty::{"),
        "empty logic should only consume state contracts from ui-state-primitives."
    );

    for forbidden in ["ui_state_primitives::", "web_sys::", "HtmlElement"] {
        assert!(
            !view_source.contains(forbidden),
            "empty view should remain pure assembly without leaking platform details; found `{forbidden}`."
        );
    }
}

#[test]
fn component_exports_stable_public_api_without_dom_types() {
    let module_source = load_source("src/mod.rs");

    for required in [
        "pub use logic::{EmptyMediaVariant, EmptyPartState, EmptyPartStateInput, EmptySlot};",
        "pub use view::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle};",
    ] {
        assert!(
            module_source.contains(required),
            "empty module should export `{required}` as stable component API."
        );
    }

    for forbidden in ["web_sys::", "HtmlElement", "NodeRef<web_sys::"] {
        assert!(
            !module_source.contains(forbidden),
            "empty public API must not expose DOM platform types; found `{forbidden}`."
        );
    }
}

#[test]
fn component_view_mounts_semantic_markers() {
    let view_source = load_source("src/view.rs");

    for required in [
        "data-slot=state.slot_attr",
        "data-state=state.state_attr",
        "data-class-source=state.class_source_attr",
        "data-variant-source=state.variant_source_attr",
        "data-variant=variant_attr",
        "Some(state.media_variant_attr)",
    ] {
        assert!(
            view_source.contains(required),
            "empty view should mount semantic marker `{required}`."
        );
    }
}

#[test]
fn component_semantic_contract_covers_data_and_focus_aria_boundary_for_static_empty() {
    let view_source = load_source("src/view.rs");

    for required in [
        "data-slot=state.slot_attr",
        "data-state=state.state_attr",
        "data-class-source=state.class_source_attr",
        "data-variant-source=state.variant_source_attr",
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-action=agent_contract.action.as_attr()",
        "data-ui-source=agent_contract.source.as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "empty semantic contract should keep stable data markers `{required}`."
        );
    }

    // Empty is static display-only: no focus lifecycle or interactive aria wiring is expected.
    for forbidden in [
        "aria-",
        "role=",
        "tabindex",
        "autofocus",
        "on:focus=",
        "on:blur=",
        "on:focusin=",
        "on:focusout=",
        "on:keydown=",
        "on:keyup=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "empty should not drift into focus/interactive aria contracts; found `{forbidden}`."
        );
    }
}

#[test]
fn component_semantics_testing_priority_stays_contract_first_and_not_visual_snapshot_only() {
    let view_source = load_source("src/view.rs");
    let semantics_source = load_source("test/semantics.rs");

    for required in [
        "fn component_view_mounts_semantic_markers()",
        "fn component_semantic_contract_covers_data_and_focus_aria_boundary_for_static_empty()",
        "data-slot=state.slot_attr",
        "data-state=state.state_attr",
        "data-class-source=state.class_source_attr",
        "data-ui-source=agent_contract.source.as_attr()",
    ] {
        assert!(
            semantics_source.contains(required) || view_source.contains(required),
            "empty should keep semantics-first assertions for state/source contracts; missing `{required}`."
        );
    }

    // Empty is non-interactive: aria/role/focus keyboard path is explicitly constrained to N/A.
    for required in ["\"aria-\"", "\"role=\"", "\"on:keydown=\""] {
        assert!(
            semantics_source.contains(required),
            "empty semantics tests should keep explicit non-interactive aria/focus N/A boundary `{required}`."
        );
    }

    let snapshot_forbidden = [
        ["insta::assert", "_snapshot"].concat(),
        ["assert", "_snapshot!"].concat(),
        ["assert_yaml", "_snapshot!"].concat(),
        ["to_match", "_snapshot("].concat(),
        ["screen", "shot"].concat(),
    ];

    for forbidden in snapshot_forbidden {
        assert!(
            !semantics_source.contains(&forbidden),
            "empty semantics contract must not depend on visual snapshot-only assertions; found `{forbidden}`."
        );
    }
}

#[test]
fn component_public_prop_names_remain_stable_for_display_contract() {
    let view_source = load_source("src/view.rs");

    for required in [
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional, into)] dir: Option<String>",
        "#[prop(optional, into)] variant: Option<logic::EmptyMediaVariant>",
    ] {
        assert!(
            view_source.contains(required),
            "empty public props should include `{required}`."
        );
    }

    for forbidden in ["className:", "default_open", "on_open_change", "is_open"] {
        assert!(
            !view_source.contains(forbidden),
            "empty display API should not introduce unrelated controlled-state aliases; found `{forbidden}`."
        );
    }
}

#[test]
fn component_exposes_locale_hooks_without_hardcoded_user_copy() {
    let view_source = load_source("src/view.rs");

    for required in ["lang=lang", "dir=dir"] {
        assert!(
            view_source.contains(required),
            "empty root should expose locale hook `{required}`."
        );
    }

    for forbidden in [
        "\"No results\"",
        "\"Try adjusting filters.\"",
        "\"Open search\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "empty view should not hardcode user-visible copy; found `{forbidden}`."
        );
    }
}

#[test]
fn component_defaults_are_normalized_in_logic_layer() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    assert!(
        logic_source.contains("pub fn normalize_part(")
            && logic_source.contains("media_variant: media_variant.unwrap_or_default()"),
        "empty logic should own default variant normalization."
    );
    assert!(
        !view_source.contains("EmptyMediaVariant::default()"),
        "empty view should not fallback defaults directly."
    );
}

#[test]
fn component_state_normalization_is_centralized_in_logic() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");

    assert!(
        logic_source.contains("pub fn normalize_part(")
            && logic_source.contains("resolve_state(EmptyPartStateInput {"),
        "empty logic should own state derivation pipeline."
    );

    for forbidden in [
        "resolve_state(EmptyPartStateInput {",
        "compose_class_name(",
        "normalize_optional_text(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "empty view should not rebuild state logic; found `{forbidden}`."
        );
    }

    assert!(
        !styles_source.contains("resolve_state(") && !styles_source.contains("normalize_part("),
        "empty styles should only consume state markers, not derive state."
    );
}

#[test]
fn component_does_not_expose_partial_controlled_state_contracts() {
    let view_source = load_source("src/view.rs");

    for forbidden in [
        "default_value",
        "on_value_change",
        "#[prop(optional)] value:",
        "default_open",
        "on_open_change",
        "#[prop(optional)] open:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "empty should not expose controlled/uncontrolled state APIs without a real state axis; found `{forbidden}`."
        );
    }
}

#[test]
fn component_hydration_init_stays_deterministic_without_time_or_random_sources() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/empty.rs");

    for forbidden in [
        "now(",
        "Date::now",
        "SystemTime",
        "Instant::now",
        "rand::",
        "random(",
        "Uuid",
        "uuid::",
        "new_v4",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !primitive_source.contains(forbidden),
            "empty hydration/init contract must stay deterministic; found `{forbidden}`."
        );
    }
}

#[test]
fn component_platform_surface_stays_non_wasm_safe_and_cfg_stable() {
    let module_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");

    for forbidden in [
        "web_sys::",
        "js_sys::",
        "wasm_bindgen",
        "window.",
        "document.",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "empty platform contract should stay non-wasm safe; found `{forbidden}`."
        );
    }

    for forbidden in [
        "cfg(target_arch = \"wasm32\")",
        "cfg(feature = \"ssr\")",
        "cfg(feature = \"web\")",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "empty has no platform split and should not drift into implicit cfg branches; found `{forbidden}`."
        );
    }
}

#[test]
fn component_performance_path_stays_static_and_hotspot_free() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");

    for forbidden in [
        "create_effect(",
        "spawn_local(",
        "set_interval(",
        "set_timeout(",
        "request_animation_frame",
        "requestAnimationFrame",
        "ui_motion::",
        "attach_motion(",
        "Arc<Mutex",
        "Rc<RefCell",
        "on:mousemove=",
        "on:pointermove=",
        "on:scroll=",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "empty performance contract should stay static and predictable; found `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains("pub fn normalize_part("),
        "empty should keep a single pure normalization entry for predictable render-time cost."
    );
}

#[test]
fn component_render_count_budget_is_na_but_static_render_path_is_guarded() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    // Empty is not a high-frequency/heavy component; lock static render path to justify render_count N/A.
    for forbidden in [
        "create_signal(",
        "create_rw_signal(",
        "create_memo(",
        "Memo::new(",
        "Signal::derive(",
        "create_effect(",
        "spawn_local(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "empty should remain static and avoid reactive churn constructs; found `{forbidden}`."
        );
    }
}

#[test]
fn component_view_macro_complexity_stays_semantically_split_and_shallow() {
    let view_source = load_source("src/view.rs");

    let view_macro_count = view_source.matches("view! {").count();
    let opening_div_count = view_source.matches("<div").count();
    let closing_div_count = view_source.matches("</div>").count();
    let children_count = view_source.matches("{children()}").count();

    assert!(
        view_macro_count <= 6,
        "empty should keep view macro blocks small and split by semantic parts; found {view_macro_count} blocks."
    );
    assert_eq!(
        opening_div_count, view_macro_count,
        "each view! block should stay shallow with one root div to avoid deep nesting."
    );
    assert_eq!(
        closing_div_count, view_macro_count,
        "each view! block should close exactly one root div."
    );
    assert_eq!(
        children_count, view_macro_count,
        "each view! block should keep a single semantic child projection point."
    );
}

#[test]
fn component_prefers_functional_split_for_lightweight_fragments() {
    let view_source = load_source("src/view.rs");

    let component_count = view_source.matches("#[component]").count();
    let view_macro_count = view_source.matches("view! {").count();

    assert!(
        view_source.contains("fn render_part("),
        "empty should keep lightweight render assembly in plain Rust function helpers."
    );
    assert!(
        view_macro_count < component_count,
        "empty should avoid per-fragment component-level macro duplication; view macro count ({view_macro_count}) should stay below component count ({component_count})."
    );

    for required in [
        "render_part(class_name, state, lang, dir, None, children)",
        "render_part(class_name, state, None, None, None, children)",
        "Some(state.media_variant_attr)",
    ] {
        assert!(
            view_source.contains(required),
            "empty should route lightweight fragment rendering through shared function path `{required}`."
        );
    }
}

#[test]
fn component_static_fragments_stay_templated_and_centralized() {
    let view_source = load_source("src/view.rs");

    let view_macro_count = view_source.matches("view! {").count();
    assert_eq!(
        view_macro_count, 1,
        "empty should keep a single static render template to avoid duplicated view! generation."
    );
    assert!(
        view_source.contains("fn render_part("),
        "empty static structure should be centralized in render_part template path."
    );

    for forbidden in ["<svg", "<footer", "inner_html="] {
        assert!(
            !view_source.contains(forbidden),
            "empty should not scatter heavy static fragments in view.rs; found `{forbidden}`."
        );
    }
}

#[test]
fn component_inner_html_contract_stays_safe_and_semantic() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for forbidden in [
        "inner_html=",
        "dangerously_set_inner_html",
        ".set_inner_html(",
        "innerHTML",
        "<script",
        "onerror=",
        "format!(\"<",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "empty should reject inner_html and dynamic html injection paths; found `{forbidden}`."
        );
    }

    for required in [
        "data-slot=state.slot_attr",
        "data-state=state.state_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "empty should keep semantic markers stable while rejecting inner_html path `{required}`."
        );
    }
}

#[test]
fn component_wasm_debug_contract_stays_na_without_surface_pollution() {
    let cargo_source = load_source("Cargo.toml");
    let module_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for forbidden in [
        "feature = \"wasm-debug\"",
        "feature = \"debug-replay\"",
        "cfg(debug_assertions)",
        "tracing::",
        "tracing_wasm",
        "web_sys::console",
        "console_log",
        "TraceId",
        "replay",
    ] {
        assert!(
            !cargo_source.contains(forbidden)
                && !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "empty should not leak wasm debug/replay capability into production component surface; found `{forbidden}`."
        );
    }

    assert!(
        !view_source.contains("on:"),
        "empty has no keyboard/pointer interaction chain and should not carry replay-oriented event handlers."
    );
}

#[test]
fn component_dx_contract_stays_css_first_with_workbench_entry() {
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_empty.rs");

    assert!(
        styles_source.contains("pub const CSS: &str"),
        "empty should keep style iteration on CSS-first static source for fast feedback loops."
    );
    for forbidden in ["style=", "style:"] {
        assert!(
            !view_source.contains(forbidden),
            "empty view should avoid inline runtime style branches that force wasm-heavy iteration; found `{forbidden}`."
        );
    }

    for required in [
        "Interactive Playground (展示 / Config / Code / CSS Test)",
        "test_css_source=empty_test_css_source",
        "test_source_path=\"components/empty/src/styles.rs\".to_string()",
        "let (workbench_media_index, set_workbench_media_index) = signal(Some(0));",
    ] {
        assert!(
            docs_source.contains(required),
            "empty docs should expose a stable workbench/isolation entry for DX contract; missing `{required}`."
        );
    }
}

#[test]
fn component_docs_contract_stays_copy_paste_ready_with_matrix_and_snapshot_policy() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_empty.rs");

    for required in [
        "let empty_code_imports =",
        "use leptos::prelude::*;\\nuse ui::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};",
        "title=\"Hello World (Default Path)\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "title=\"Parameter Matrix (variant / class_name / content)\"",
        "title=\"State Matrix (Header / Action / Source Markers)\"",
        "title=\"Controlled vs Uncontrolled (N/A for Empty)\"",
        "title=\"Streaming/Snapshot Display\"",
        "Streaming Optional; fallback=snapshot.",
        "code_imports=empty_code_imports.clone()",
        "code_imports=empty_code_imports",
    ] {
        assert!(
            docs_source.contains(required),
            "empty docs must stay copy-paste ready with required playground coverage; missing `{required}`."
        );
    }
}

#[test]
fn component_docs_examples_and_matrices_stay_synced_with_logic_api_defaults() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_empty.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for required in [
        "media_variant: media_variant.unwrap_or_default()",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] variant: Option<logic::EmptyMediaVariant>",
    ] {
        assert!(
            logic_source.contains(required) || view_source.contains(required),
            "empty logic/view API contract should keep `{required}`."
        );
    }

    for required in [
        "signal(Some(0))",
        "workbench_media_index.get().unwrap_or(0)",
        "title=\"Parameter Matrix (variant / class_name / content)\"",
        "title=\"State Matrix (Header / Action / Source Markers)\"",
        "variant=EmptyMediaVariant::Icon",
        "class_name=\"docs-empty-custom\".to_string()",
        "<EmptyMedia>\"📦\"</EmptyMedia>",
    ] {
        assert!(
            docs_source.contains(required),
            "empty docs should stay synced with API names/defaults and matrix coverage; missing `{required}`."
        );
    }

    for forbidden in ["className=", "mediaVariant=", "default_variant"] {
        assert!(
            !docs_source.contains(forbidden),
            "empty docs must not drift to non-canonical API naming `{forbidden}`."
        );
    }
}

#[test]
fn component_docs_interactive_playground_stays_live_with_spec_linkage_and_repeatable_flow() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_empty.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_empty_contract.spec.mjs");

    for required in [
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "description=\"可在线调 media/content/class/source 并实时预览；同面板提供 code + config + AI Spec 输入联动 + scoped css test，作为可重复验收面。\"",
        "test_config_signal=workbench_config",
        "EmptyAgentSpecInput {",
        "PreviewLinkage {",
        "preview_action:",
        "render-snapshot",
        "<SegmentedControl",
        "<Switch checked=workbench_show_content",
        "<Switch checked=workbench_custom_class",
        "<Switch",
    ] {
        assert!(
            docs_source.contains(required),
            "empty docs interactive playground should keep live prop/state controls with spec-linkage evidence `{required}`."
        );
    }

    for required in [
        "async function runEmptyCriticalFlow(docsRoot)",
        "test(\"docs-app empty key flow is repeatable with semantic breakpoints\"",
        "await page.reload();",
        "await runEmptyCriticalFlow(reloadedDocsRoot);",
    ] {
        assert!(
            e2e_source.contains(required),
            "empty interactive playground acceptance should keep repeatable key-flow replay evidence `{required}`."
        );
    }
}

#[test]
fn component_docs_source_first_copy_paste_ready_contract_stays_runnable_and_traceable() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_empty.rs");

    for required in [
        "data-slot=\"empty-source-first\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "data-slot=\"empty-source-prerequisites\"",
        "component-empty",
        "UiRoot",
        "inject-css",
        "label=\"Copy empty starter\".to_string()",
        "copyable=true",
        "class_name=\"docs-empty-source-copy\".to_string()",
        "data-slot=\"empty-source-paths\"",
        "\"components/empty/src/mod.rs\"",
        "\"components/empty/src/logic.rs\"",
        "\"components/empty/src/view.rs\"",
        "\"components/empty/src/styles.rs\"",
        "use ui::{Empty, EmptyHeader, EmptyTitle};",
    ] {
        assert!(
            docs_source.contains(required),
            "empty source-first docs contract should keep copy-ready runtime and path traceability `{required}`."
        );
    }
}

#[test]
fn component_heroui_alignment_docs_stay_synced_and_docs_entry_indexable() {
    let heroui_spec_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let empty_docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra_empty.rs");

    for required in [
        "### Empty 同步记录（2026-02-20）",
        "display_extra_empty_catalog::{EMPTY_DOC, EMPTY_HEADER_DOC, EMPTY_MEDIA_DOC, EMPTY_TITLE_DOC, EMPTY_DESCRIPTION_DOC, EMPTY_CONTENT_DOC}",
        "apps/docs-app/src/pages/components/pages/display_extra_empty.rs",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "component-empty",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            heroui_spec_source.contains(required),
            "empty HeroUI alignment spec should keep synced documentation evidence `{required}`."
        );
    }

    for required in [
        "display_extra_empty_catalog::EMPTY_DOC",
        "display_extra_empty_catalog::EMPTY_HEADER_DOC",
        "display_extra_empty_catalog::EMPTY_MEDIA_DOC",
        "display_extra_empty_catalog::EMPTY_TITLE_DOC",
        "display_extra_empty_catalog::EMPTY_DESCRIPTION_DOC",
        "display_extra_empty_catalog::EMPTY_CONTENT_DOC",
    ] {
        assert!(
            docs_pages_source.contains(required),
            "empty docs entry should remain indexable from docs-app pages registry `{required}`."
        );
    }

    for required in [
        "title=\"Empty\"",
        "slug=\"empty\"",
        "Source-first / Copy-Paste Ready",
    ] {
        assert!(
            empty_docs_source.contains(required),
            "empty component docs page should stay accessible and aligned `{required}`."
        );
    }
}

#[test]
fn component_readme_stays_beginner_friendly_with_default_path_first() {
    let readme_source = load_source("src/README.md");

    for required in [
        "# Empty",
        "## Quick Start (Hello World)",
        "## Common Usage",
        "## Advanced (Use Only When Needed)",
        "## Learn In Order",
        "## Docs Entry",
        "use ui::{Empty, EmptyHeader, EmptyTitle};",
        "No state machine wiring is required.",
        "has no controlled/uncontrolled state axis",
        "display_extra_empty.rs",
    ] {
        assert!(
            readme_source.contains(required),
            "empty README should keep beginner-facing entry and progressive guidance `{required}`."
        );
    }

    let hello_idx = readme_source
        .find("## Quick Start (Hello World)")
        .expect("README must contain Hello World section.");
    let common_idx = readme_source
        .find("## Common Usage")
        .expect("README must contain Common Usage section.");
    let advanced_idx = readme_source
        .find("## Advanced (Use Only When Needed)")
        .expect("README must contain Advanced section.");

    assert!(
        hello_idx < common_idx && common_idx < advanced_idx,
        "README must keep default path first, then common usage, then advanced usage."
    );
}

#[test]
fn component_e2e_selector_contract_stays_semantic_and_wasm_stable() {
    let e2e_source = load_source("../../e2e/tests/docs_app_empty_contract.spec.mjs");

    for required in [
        "await page.goto(\"/#/components/empty\");",
        "async function runEmptyCriticalFlow(docsRoot)",
        "body:not(:has(#boot))",
        "[data-component=\"empty\"]",
        "[data-slot=\"empty\"][data-state=\"root\"]",
        "[data-slot=\"empty-header\"][data-state=\"header\"]",
        "[data-slot=\"empty-title\"][data-state=\"title\"]",
        "[data-slot=\"empty-description\"][data-state=\"description\"]",
        "[data-slot=\"empty-icon\"][data-state=\"media\"]",
        "[data-slot=\"playground-controls\"]",
        "[data-slot=\"segmented-control-option\"][data-index=\"0\"]",
        "[data-slot=\"segmented-control-option\"][data-index=\"1\"]",
        "test(\"docs-app empty key flow is repeatable with semantic breakpoints\"",
        "await page.reload();",
        "await runEmptyCriticalFlow(docsRoot);",
        "await runEmptyCriticalFlow(reloadedDocsRoot);",
    ] {
        assert!(
            e2e_source.contains(required),
            "empty e2e contract should keep semantic-selector and settled-wait checkpoints `{required}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "getByText(",
        "locator(\"text=",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "empty e2e contract should avoid brittle selectors and fixed-sleep waits; found `{forbidden}`."
        );
    }
}

#[test]
fn component_engineering_contract_stays_na_without_runtime_leaks() {
    let cargo_source = load_source("Cargo.toml");
    let module_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for forbidden in [
        "serde",
        "Serialize",
        "Deserialize",
        "tracing::",
        "tracing-",
        "tokio::",
        "async_std::",
        "async-std",
        "runtime",
        "JoinHandle",
        "async fn",
    ] {
        assert!(
            !cargo_source.contains(forbidden)
                && !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "empty should not leak serde/tracing/async-runtime engineering surface; found `{forbidden}`."
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(
        !manifest_dir.join("src/spec.rs").exists(),
        "empty has no spec/config serialization contract and should not introduce spec.rs."
    );
}

#[test]
fn component_defensive_variables_stay_token_fallback_first() {
    let styles_source = load_source("src/styles.rs");

    for required in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
    ] {
        assert!(
            styles_source.contains(required),
            "empty styles should use defensive token fallback chain `{required}`."
        );
    }

    for forbidden in [
        "px;",
        "rem;",
        "em;",
        "vh;",
        "vw;",
        "border-radius: inherit;",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "empty styles should avoid hardcoded terminal style values in component island; found `{forbidden}`."
        );
    }
}

#[test]
fn component_cascade_layer_contract_stays_ui_scoped_and_inline_free() {
    let css_aggregator_source = load_source("../../crates/ui/src/css.rs");
    let view_source = load_source("src/view.rs");

    for required in [
        "@layer ui {",
        "#[cfg(feature = \"component-empty\")]",
        "out.push_str(crate::empty::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_aggregator_source.contains(required),
            "empty css should be aggregated inside ui cascade layer with feature gate; missing `{required}`."
        );
    }

    for forbidden in ["style=\"", "style='", "style=", "style:top", "style:left"] {
        assert!(
            !view_source.contains(forbidden),
            "empty view should avoid normal inline style injection path; found `{forbidden}`."
        );
    }
}

#[test]
fn component_motion_contract_stays_na_with_no_motion_surface() {
    let cargo_source = load_source("Cargo.toml");
    let module_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for forbidden in [
        "ui-motion",
        "ui_motion::",
        "attach_motion(",
        "XxxMotion",
        "stiffness",
        "damping",
        "prefers-reduced-motion",
        "prefers_reduced_motion",
    ] {
        assert!(
            !cargo_source.contains(forbidden)
                && !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "empty should not expose motion contract surface in static component path; found `{forbidden}`."
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(
        !manifest_dir.join("src/motion.rs").exists(),
        "empty has no motion semantics and should not introduce motion.rs contract file."
    );
}

#[test]
fn component_ui_components_entrypoints_stay_canonical_for_empty_integration() {
    let lib_source = load_source("../../crates/ui/src/lib.rs");
    let css_source = load_source("../../crates/ui/src/css.rs");
    let root_source = load_source("../../crates/ui/src/root.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");

    for required in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-empty\")]",
        "pub use ui_empty as empty;",
    ] {
        assert!(
            lib_source.contains(required),
            "ui lib entry should expose canonical empty/root/css boundaries; missing `{required}`."
        );
    }

    for required in [
        "@layer ui {",
        "#[cfg(feature = \"component-empty\")]",
        "out.push_str(crate::empty::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(required),
            "ui css entry should keep feature-gated layer aggregation contract; missing `{required}`."
        );
    }

    for required in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot entry should centralize theme/css/i18n injection strategy; missing `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str",
        "pub struct ActiveHighlightMotion",
        "ui_motion::spring::SpringConfig",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "active_highlight should remain shared visual primitive capability, not component business logic; missing `{required}`."
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    for forbidden_rel in [
        "../../crates/ui/src/overlay_open.rs",
        "../../crates/ui/src/presence.rs",
        "../../crates/ui/src/a11y.rs",
    ] {
        assert!(
            !manifest_dir.join(forbidden_rel).exists(),
            "ui should not re-home headless primitives; forbidden file exists: `{forbidden_rel}`."
        );
    }
}

#[test]
fn component_directory_layout_stays_canonical_for_empty_static_component() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs"] {
        assert!(
            src_dir.join(required).exists(),
            "empty component directory must keep canonical file `{required}`."
        );
    }

    for forbidden in ["render.rs", "motion.rs", "spec.rs"] {
        assert!(
            !src_dir.join(forbidden).exists(),
            "empty static component should not introduce `{forbidden}` in directory layout."
        );
    }

    let module_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{EmptyMediaVariant, EmptyPartState, EmptyPartStateInput, EmptySlot};",
        "pub use view::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle};",
    ] {
        assert!(
            module_source.contains(required),
            "empty module boundary should keep minimal canonical exports; missing `{required}`."
        );
    }
    for forbidden in ["pub mod logic;", "pub mod view;"] {
        assert!(
            !module_source.contains(forbidden),
            "empty module should avoid over-exporting internal implementation unit `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_part(",
        "resolve_state(EmptyPartStateInput {",
    ] {
        assert!(
            logic_source.contains(required),
            "empty logic should keep normalization/derivation in logic.rs; missing `{required}`."
        );
    }
    for forbidden in ["view! {", "data-slot=", "style="] {
        assert!(
            !logic_source.contains(forbidden),
            "empty logic.rs should not drift into view/style assembly; found `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str"),
        "empty styles.rs should keep static CSS contract constant."
    );
    for forbidden in ["#[component]", "view! {", "normalize_part("] {
        assert!(
            !styles_source.contains(forbidden),
            "empty styles.rs should stay CSS-only and avoid logic/view concerns; found `{forbidden}`."
        );
    }

    for required in [
        "fn render_part(",
        "logic::normalize_part(",
        "data-slot=state.slot_attr",
    ] {
        assert!(
            view_source.contains(required),
            "empty view.rs should keep structure rendering + logic output mounting; missing `{required}`."
        );
    }
    for forbidden in ["resolve_state(EmptyPartStateInput {", "compose_class_name("] {
        assert!(
            !view_source.contains(forbidden),
            "empty view.rs should not re-implement logic derivation; found `{forbidden}`."
        );
    }
}

#[test]
fn component_hyper_structure_builder_contract_stays_na_for_empty_static_component() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");
    let module_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    assert!(
        !src_dir.join("spec.rs").exists(),
        "empty is not a complex schema-driven component and must not introduce spec.rs."
    );

    for forbidden in [
        "Spec::new(",
        "EmptySpec",
        ".render()",
        "builder",
        "component-empty-spec",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "empty should not expose Hyper-Structure builder surface; found `{forbidden}`."
        );
    }
}

#[test]
fn component_context_compression_manifest_and_rbi_projection_stay_present_and_aligned() {
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/empty.rbi");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    assert!(
        manifest_dir.join("src/Component.toml").exists(),
        "empty must keep context manifest file for AI context compression protocol."
    );
    assert!(
        manifest_dir.join("src/empty.rbi").exists(),
        "empty must keep RBI signature projection file for interface indexing."
    );

    for required in [
        "schema_version = \"1\"",
        "name = \"Empty\"",
        "crate = \"ui-empty\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"root.class_name\"",
        "name = \"media.variant\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "empty Component.toml should keep manifest contract entry `{required}`."
        );
    }

    for required in [
        "pub enum EmptyMediaVariant",
        "pub enum EmptySlot",
        "pub struct EmptyPartStateInput",
        "pub struct EmptyPartState",
        "pub fn Empty(",
        "pub fn EmptyHeader(",
        "pub fn EmptyTitle(",
        "pub fn EmptyDescription(",
        "pub fn EmptyContent(",
        "pub fn EmptyMedia(",
    ] {
        assert!(
            rbi_source.contains(required),
            "empty RBI should project stable API signature `{required}`."
        );
    }
}

#[test]
fn component_version_deprecation_migration_contract_stays_na_without_breaking_upgrade() {
    let module_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/empty.rbi");

    // Empty has no major-version breaking API upgrade in this cycle.
    assert!(
        manifest_source.contains("schema_version = \"1\""),
        "empty must keep schema version baseline at v1 when no breaking upgrade exists."
    );
    assert!(
        logic_source.contains("pub const EMPTY_COMPONENT_SCHEMA_VERSION: &str = \"1\";"),
        "empty logic should stay aligned with v1 schema when no major migration is required."
    );

    for forbidden in [
        "migrate_v1_to_v2",
        "schema_registry",
        "SchemaRegistry",
        "deprecation_window",
        "deprecated_since",
        "breaking_upgrade",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden),
            "empty should not introduce codemod/registry migration surface without a real breaking change; found `{forbidden}`."
        );
    }
}

#[test]
fn component_agent_contract_schema_is_typed_and_whitelisted() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/empty.rbi");

    for required in [
        "pub const EMPTY_COMPONENT_SCHEMA_NAME: &str = \"ui-empty\";",
        "pub const EMPTY_COMPONENT_SCHEMA_VERSION: &str = \"1\";",
        "pub enum EmptyAgentIntent",
        "pub enum EmptyAgentAction",
        "pub enum EmptyAgentSource",
        "pub enum EmptyAgentOutputStatus",
        "pub enum EmptyAgentStreamSupport",
        "pub enum EmptyAgentStreamFallback",
        "pub struct EmptyAgentContract",
        "pub fn resolve_agent_contract(state: EmptyPartState) -> EmptyAgentContract",
    ] {
        assert!(
            logic_source.contains(required),
            "empty logic should keep typed agent contract generation entry `{required}`."
        );
    }

    for required in [
        "let agent_contract = logic::resolve_agent_contract(state);",
        "data-ui-schema=agent_contract.schema_name",
        "data-ui-schema-version=agent_contract.schema_version",
        "data-ui-intent=agent_contract.intent.as_attr()",
        "data-ui-action=agent_contract.action.as_attr()",
        "data-ui-state=agent_contract.state",
        "data-ui-source=agent_contract.source.as_attr()",
        "data-ui-stream-support=agent_contract.stream_support.as_attr()",
        "data-ui-stream-fallback=agent_contract.stream_fallback.as_attr()",
        "data-ui-output-status=agent_contract.output_status.as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "empty view should mount schema-derived agent marker `{required}`."
        );
    }

    for forbidden in [
        "\"empty-display\"",
        "\"render-snapshot\"",
        "\"draft\"",
        "\"verified\"",
        "\"submittable\"",
        "schema_json",
        "inner_html=",
        "dangerously_set_inner_html",
        "eval(",
        "<script",
        "onerror=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "empty view should avoid raw/scriptable contract payloads; found `{forbidden}`."
        );
    }

    for required in [
        "name = \"agent_contract_schema\"",
        "data-ui-schema",
        "data-ui-intent",
        "data-ui-action",
        "data-ui-state",
        "data-ui-source",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
    ] {
        assert!(
            manifest_source.contains(required),
            "empty Component.toml should project agent contract field `{required}`."
        );
    }

    for required in [
        "pub const EMPTY_COMPONENT_SCHEMA_NAME: &str;",
        "pub enum EmptyAgentIntent",
        "pub enum EmptyAgentAction",
        "pub enum EmptyAgentSource",
        "pub enum EmptyAgentOutputStatus",
        "pub enum EmptyAgentStreamSupport",
        "pub enum EmptyAgentStreamFallback",
        "pub struct EmptyAgentContract",
        "pub fn resolve_agent_contract(state: EmptyPartState) -> EmptyAgentContract;",
    ] {
        assert!(
            rbi_source.contains(required),
            "empty RBI should expose typed agent contract projection `{required}`."
        );
    }
}

#[test]
fn component_llm_render_mode_contract_stays_snapshot_only_for_empty() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/empty.rbi");

    for required in [
        "pub enum EmptyAgentAction",
        "pub enum EmptyAgentStreamSupport",
        "pub enum EmptyAgentStreamFallback",
        "RenderSnapshot",
        "EmptyAgentAction::RenderSnapshot => \"render-snapshot\"",
        "EmptyAgentStreamSupport::Optional => \"optional\"",
        "EmptyAgentStreamFallback::Snapshot => \"snapshot\"",
        "action: EmptyAgentAction::RenderSnapshot,",
        "let stream_support = EmptyAgentStreamSupport::Optional;",
        "stream_support,",
        "stream_fallback: EmptyAgentStreamFallback::Snapshot,",
        "data-ui-action=agent_contract.action.as_attr()",
        "data-ui-stream-support=agent_contract.stream_support.as_attr()",
        "data-ui-stream-fallback=agent_contract.stream_fallback.as_attr()",
    ] {
        assert!(
            logic_source.contains(required) || view_source.contains(required),
            "empty should keep snapshot render-mode contract marker `{required}`."
        );
    }

    for forbidden in ["RenderStreaming", "render-streaming"] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden),
            "empty should not expose streaming render-mode surface for snapshot-only contract; found `{forbidden}`."
        );
    }

    assert!(
        manifest_source.contains("name = \"streaming_optional_snapshot_fallback\""),
        "empty manifest should explicitly declare optional-streaming with snapshot fallback capability."
    );
    assert!(
        rbi_source.contains("pub enum EmptyAgentStreamSupport")
            && rbi_source.contains("pub enum EmptyAgentStreamFallback"),
        "empty RBI should expose stream support/fallback contract projection."
    );
}

#[test]
fn component_snapshot_capability_is_default_and_complete_input_surface_stays_renderable() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/empty.rbi");

    for required in [
        "action: EmptyAgentAction::RenderSnapshot,",
        "let stream_support = EmptyAgentStreamSupport::Optional;",
        "stream_support,",
        "stream_fallback: EmptyAgentStreamFallback::Snapshot,",
        "let output_status = EmptyAgentOutputStatus::Verified;",
        "output_status,",
        "data-ui-action=agent_contract.action.as_attr()",
        "data-ui-stream-support=agent_contract.stream_support.as_attr()",
        "data-ui-stream-fallback=agent_contract.stream_fallback.as_attr()",
        "data-ui-output-status=agent_contract.output_status.as_attr()",
    ] {
        assert!(
            logic_source.contains(required) || view_source.contains(required),
            "empty should keep snapshot as default render action with stable markers `{required}`."
        );
    }

    for required in [
        "name = \"snapshot_rendering\"",
        "name = \"streaming_optional_snapshot_fallback\"",
        "enabled = true",
        "name = \"root.class_name\"",
        "name = \"root.lang\"",
        "name = \"root.dir\"",
        "name = \"header.class_name\"",
        "name = \"title.class_name\"",
        "name = \"description.class_name\"",
        "name = \"content.class_name\"",
        "name = \"media.class_name\"",
        "name = \"media.variant\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "empty Component.toml should preserve snapshot-ready complete input contract `{required}`."
        );
    }

    for required in [
        "pub fn Empty(",
        "pub fn EmptyHeader(",
        "pub fn EmptyTitle(",
        "pub fn EmptyDescription(",
        "pub fn EmptyContent(",
        "pub fn EmptyMedia(",
        "pub enum EmptyAgentOutputStatus",
        "pub enum EmptyAgentStreamSupport",
        "pub enum EmptyAgentStreamFallback",
    ] {
        assert!(
            rbi_source.contains(required),
            "empty RBI should keep full component entry projection for complete snapshot config assembly `{required}`."
        );
    }

    for forbidden in ["RenderStreaming", "render-streaming"] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden),
            "empty snapshot baseline should not drift into streaming-only contract surface; found `{forbidden}`."
        );
    }
}

#[test]
fn component_rust_hygiene_stays_clean_for_non_test_source() {
    let module_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");

    for forbidden in [".unwrap(", ".unwrap_err(", ".expect(", "let _ ="] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "empty non-test source should keep rust hygiene without `{forbidden}`."
        );
    }

    // Empty is static display-only and currently has no string clone hotspots that require Cow.
    for hotspot in [".to_owned()", "String::from(", ".to_string("] {
        assert!(
            !logic_source.contains(hotspot) && !view_source.contains(hotspot),
            "empty should avoid string clone hotspots in logic/view path; found `{hotspot}`."
        );
    }
}

#[test]
fn component_tree_shaking_feature_contract_stays_component_scoped_for_empty() {
    let cargo_source = load_source("../../crates/ui/Cargo.toml");
    let lib_source = load_source("../../crates/ui/src/lib.rs");
    let css_source = load_source("../../crates/ui/src/css.rs");

    for required in [
        "component-empty = [\"dep:ui-empty\"]",
        "ui-empty = { path = \"../../components/empty\", optional = true }",
        "#[cfg(feature = \"component-empty\")]",
        "pub use ui_empty as empty;",
        "out.push_str(crate::empty::styles::CSS);",
    ] {
        assert!(
            cargo_source.contains(required)
                || lib_source.contains(required)
                || css_source.contains(required),
            "empty tree-shaking contract should keep feature-scoped registration and aggregation `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("component-empty = [\"dep:ui-empty\", \"all-components\"]"),
        "empty component feature must not be coupled to an all-components aggregate path."
    );
}
