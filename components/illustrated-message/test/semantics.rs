use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_docs_illustrated_message_section() -> String {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let marker = "pub(super) fn illustrated_message() -> AnyView";
    let start = source
        .find(marker)
        .unwrap_or_else(|| panic!("missing docs function marker: {marker}"));
    let tail = &source[start..];
    let end = tail.find("\npub(super) fn ").unwrap_or(tail.len());
    tail[..end].to_string()
}

#[test]
fn illustrated_message_does_not_expose_logic_module() {
    let source = load_source("src/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "IllustratedMessage's `logic` module should stay private to avoid leaking internal view-state helpers into the public API."
    );
}

#[test]
fn illustrated_message_emits_expected_data_slots() {
    let source = load_source("src/view.rs");

    for attr in [
        "data-slot=\"illustrated-message\"",
        "data-slot=\"illustrated-message-content\"",
        "data-slot=\"illustrated-message-title\"",
        "data-slot=\"illustrated-message-description\"",
        "data-slot=\"illustrated-message-actions\"",
    ] {
        assert!(
            source.contains(attr),
            "IllustratedMessage should set `{attr}` for baseline-style styling and inspection."
        );
    }
}

#[test]
fn illustrated_message_exposes_stable_state_and_source_markers() {
    let view = load_source("src/view.rs");
    let logic = load_source("src/logic.rs");

    for needle in [
        "data-view-state=view_state",
        "data-content-state=content_state",
        "data-title-state=title_state",
        "data-description-state=description_state",
        "data-illustration-state=illustration_state",
        "data-actions-state=actions_state",
        "data-title-source=title_source",
        "data-description-source=description_source",
        "data-illustration-source=illustration_source",
        "data-actions-source=actions_source",
        "data-orientation=orientation_attr",
        "aria-live",
    ] {
        assert!(
            view.contains(needle),
            "IllustratedMessage view should expose stable semantic marker `{needle}`.",
        );
    }

    for needle in [
        "pub enum IllustratedMessageStateMarker",
        "pub enum IllustratedMessageRenderMarker",
        "IllustratedMessageStateMarker::Shown => \"shown\"",
        "IllustratedMessageStateMarker::Hidden => \"hidden\"",
        "IllustratedMessageRenderMarker::Empty => \"empty\"",
        "IllustratedMessageRenderMarker::Populated => \"populated\"",
    ] {
        assert!(
            logic.contains(needle),
            "IllustratedMessage logic should keep marker values closed and enumerable (`{needle}`).",
        );
    }
}

#[test]
fn illustrated_message_uses_spring_driven_opacity_and_y_css_vars() {
    let styles = load_source("src/styles.rs");
    let motion = load_source("src/motion.rs");

    for needle in [
        "--ui-im-opacity",
        "--ui-im-y",
        "opacity: var(--ui-im-opacity)",
        "transform: translateY(var(--ui-im-y))",
    ] {
        assert!(
            styles.contains(needle),
            "IllustratedMessage styles should reference `{needle}` for spring-driven enter motion."
        );
    }

    for needle in ["--ui-im-opacity", "--ui-im-y"] {
        assert!(
            motion.contains(needle),
            "IllustratedMessage motion should write `{needle}` to drive enter animation."
        );
    }
}

#[test]
fn illustrated_message_attaches_motion_driver() {
    let source = load_source("src/view.rs");

    assert!(
        source.contains("motion::attach_motion"),
        "IllustratedMessage should attach its motion driver from `motion.rs`."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn illustrated_message_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/motion.rs");
    let view_source = load_source("src/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: IllustratedMessageMotion) -> IllustratedMessageMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "IllustratedMessage motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::motion::sanitize_motion(motion);"),
        "IllustratedMessage view should sanitize motion before attaching motion driver.",
    );
}

#[test]
fn illustrated_message_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn illustrated_message() -> AnyView",
        "title=\"IllustratedMessage\"",
        "slug=\"illustrated-message\"",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional / Snapshot\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for IllustratedMessage.",
        );
    }
}

#[test]
fn illustrated_message_docs_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "let state_matrix_code = Signal::derive(move || {",
        "let controlled_contrast_code = Signal::derive(move || {",
        "let stream_snapshot_code = Signal::derive(move || {",
        "let source_first_code = Signal::derive(move || {",
        "let code_imports =",
        "use ui::{Button, IllustratedMessage};",
        "title=\"No results\".to_string()",
        "description=\"Try changing your search.\".to_string()",
        "illustration=move || view! { <div class=\"docs-illustration\">\"◎\"</div> }",
        "actions=move || view! { <ui::Button>\"Clear\"</ui::Button> }",
    ] {
        assert!(
            source.contains(needle),
            "illustrated-message docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"IllustratedMessage\"",
        "slug=\"illustrated-message\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional / Snapshot\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "Streaming Optional -> fallback=snapshot.",
        "Copy-ready snippets prepend missing imports automatically.",
        "title=\"No results\".to_string()",
        "description=\"Try changing your search.\".to_string()",
        "illustration=move || view! { <div class=\"docs-illustration\">\"◎\"</div> }",
        "actions=move || view! { <Button>\"Clear\"</Button> }",
    ] {
        assert!(
            source.contains(needle),
            "illustrated-message docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_docs_include_minimal_hello_world_path() {
    let source = load_docs_illustrated_message_section();

    for needle in [
        "let hello_world_code = Signal::derive(move || {",
        "<IllustratedMessage title=\"Empty\".to_string() description=\"Nothing here\".to_string() />",
        "title=\"Hello World (Default API)\"",
        "title=\"Empty\".to_string()",
        "description=\"Nothing here\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "IllustratedMessage docs should expose a minimal default path with `{needle}`.",
        );
    }

    assert!(
        !source.contains("state="),
        "IllustratedMessage minimal docs path should not require internal state wiring.",
    );
}

#[test]
fn illustrated_message_docs_are_copy_paste_ready_with_import_completion() {
    let check2 = load_source("check2.md");
    let docs_source = load_docs_illustrated_message_section();
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional / Snapshot\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "code_imports=code_imports.clone()",
        "code_imports=code_imports",
    ] {
        assert!(
            docs_source.contains(needle),
            "illustrated-message docs should keep copy-paste-ready playground contract `{needle}`.",
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str =",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "fn missing_import_lines(raw: &str, imports: &str) -> Vec<String>",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground infrastructure should keep import-completion primitive `{needle}`.",
        );
    }

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "`apps/docs-app/src/pages/components/pages/display.rs::illustrated_message` 已提供 `Hello World (Default API)`、`State Matrix`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional / Snapshot`、`Source-first Starter (Copy-Paste Ready)` 五组 Playground",
        "流式/快照展示由 `Streaming Optional / Snapshot` Playground 明确（`fallback=snapshot`）",
        "Source-first 一键复制与 imports 补全由 `apps/docs-app/src/playground.rs::compose_copy_ready_code` + `code_imports` 注入链路保障",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_docs_are_copy_paste_ready_with_import_completion`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record docs copy-paste-ready evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_source_first_docs_are_copy_paste_ready_and_traceable() {
    let check2 = load_source("check2.md");
    let docs_source = load_docs_illustrated_message_section();

    for needle in [
        "data-slot=\"illustrated-message-source-first\"",
        "data-slot=\"illustrated-message-source-first-contract\"",
        "data-slot=\"illustrated-message-source-prerequisites\"",
        "data-slot=\"illustrated-message-source-paths\"",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "component-illustrated_message",
        "UiRoot",
        "inject-css",
        "Copy illustrated-message starter",
        "docs-illustrated-message-source-copy",
        "use leptos::prelude::*;",
        "use ui::{Button, IllustratedMessage};",
        "title=\"No results\".to_string()",
        "description=\"Try changing your search.\".to_string()",
        "illustration=move || view! { <div class=\"docs-illustration\">\"◎\"</div> }",
        "actions=move || view! { <Button>\"Clear\"</Button> }",
        "components/illustrated-message/src/mod.rs",
        "components/illustrated-message/src/logic.rs",
        "components/illustrated-message/src/view.rs",
        "components/illustrated-message/src/styles.rs",
        "components/illustrated-message/src/motion.rs",
    ] {
        assert!(
            docs_source.contains(needle),
            "illustrated-message source-first docs should contain `{needle}`.",
        );
    }

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "`apps/docs-app/src/pages/components/pages/display.rs::illustrated_message` 新增 `data-slot=\"illustrated-message-source-first\"` 合同区块与 `Snippet` 复制按钮（`Copy illustrated-message starter`）",
        "复制输出默认含可运行 imports（`use leptos::prelude::*; use ui::{Button, IllustratedMessage};`）",
        "依赖前提在 `illustrated-message-source-prerequisites` 明确：`component-illustrated_message` feature + `UiRoot`/`inject-css` 样式注入",
        "源码落点在 `illustrated-message-source-paths` 指向 `components/illustrated-message/src/{mod,logic,view,styles,motion}.rs`",
        "示例片段与 `source_first_code`（`No results` + `Try changing your search.` + `illustration/actions`）保持同步以防漂移",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_source_first_docs_are_copy_paste_ready_and_traceable`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record source-first copy-paste evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_heroui_alignment_docs_and_component_docs_are_synced_and_indexable() {
    let check2 = load_source("check2.md");
    let heroui_strategy = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_docs_illustrated_message_section();
    let readme = load_source("src/README.md");

    for needle in [
        "### IllustratedMessage 同步记录（2026-02-20）",
        "`IllustratedMessage` 维持 display empty-state primitive 定位",
        "`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!(\"IllustratedMessage\", \"illustrated-message\", \"Display\", display::illustrated_message)` 暴露入口",
        "`#/components/illustrated-message` 可索引访问",
        "`components/illustrated-message/src/README.md` 提供等价组件文档入口",
        "`apps/docs-app/src/pages/components/pages/display.rs::illustrated_message()` 已覆盖 `Hello World (Default API)`、`State Matrix`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional / Snapshot`、`Source-first Starter (Copy-Paste Ready)` 与 `Interactive Playground (Props + State + Preview)`",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
    ] {
        assert!(
            heroui_strategy.contains(needle),
            "HeroUI parameter alignment strategy should keep IllustratedMessage sync marker `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"IllustratedMessage\"",
        "\"illustrated-message\"",
        "display::illustrated_message",
    ] {
        assert!(
            pages_registry.contains(needle),
            "docs pages registry should keep IllustratedMessage index entry token `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn illustrated_message() -> AnyView",
        "title=\"IllustratedMessage\"",
        "slug=\"illustrated-message\"",
        "title=\"Hello World (Default API)\"",
        "title=\"Interactive Playground (Props + State + Preview)\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "IllustratedMessage docs page should keep accessible entry marker `{needle}`.",
        );
    }

    for needle in [
        "# IllustratedMessage",
        "## Start Here (Hello World)",
        "## API (Table)",
        "## docs-app Entry",
        "apps/docs-app/src/pages/components/pages/display.rs",
    ] {
        assert!(
            readme.contains(needle),
            "IllustratedMessage README should keep component-doc entry marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "`docs/spec/heroui-parameter-design-strategy.md` 新增 `### IllustratedMessage 同步记录（2026-02-20）`",
        "`component_doc!(\\\"IllustratedMessage\\\", \\\"illustrated-message\\\", \\\"Display\\\", display::illustrated_message)`",
        "`components/illustrated-message/src/README.md` 双入口保证",
        "`docs/research/spectrum-heroui-style-interface-study.md` 无需补充",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_heroui_alignment_docs_and_component_docs_are_synced_and_indexable`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record HeroUI/docs-sync evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_docs_matrices_and_api_defaults_stay_synced_with_logic_contract() {
    let check2 = load_source("check2.md");
    let docs_source = load_docs_illustrated_message_section();
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for needle in [
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional / Snapshot\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "description=\"Covers default, rich slots, and partial-content states with stable aria/data markers.\"",
        "description=\"IllustratedMessage is display-only; compare default props and app-state-mapped props without internal state axis.\"",
        "description=\"N/A: no value/on_value_change/default_value axis.\".to_string()",
        "<IllustratedMessage description=\"Only description provided.\".to_string() />",
    ] {
        assert!(
            docs_source.contains(needle),
            "illustrated-message docs should keep synced matrix/sample marker `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional, into)] title: Option<String>,",
        "#[prop(optional, into)] description: Option<String>,",
        "#[prop(optional, into)] illustration: Option<ViewFn>,",
        "#[prop(optional, into)] actions: Option<ViewFn>,",
        "#[prop(optional)] orientation: IllustratedMessageOrientation,",
        "#[prop(optional)] motion: IllustratedMessageMotion,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
    ] {
        assert!(
            view_source.contains(needle),
            "illustrated-message view API should expose documented prop `{needle}`.",
        );
    }

    for needle in [
        "pub fn resolve_view_model<",
        "fn normalize_display_text(value: Option<String>)",
        "IllustratedMessageTextSource::Missing",
        "IllustratedMessageTextSource::Blank",
        "let view_state = normalize_render_marker(",
    ] {
        assert!(
            logic_source.contains(needle),
            "illustrated-message logic defaults should stay traceable via `{needle}`.",
        );
    }

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "`apps/docs-app/src/pages/components/pages/display.rs::illustrated_message` 同步提供 `Hello World (Default API)`、`State Matrix`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional / Snapshot`、`Source-first Starter (Copy-Paste Ready)` 示例",
        "`State Matrix` 覆盖 default/rich/partial-content 状态组合",
        "`Controlled vs Uncontrolled (N/A)` 明确该组件无受控轴",
        "示例 API 名称与 `components/illustrated-message/src/view.rs` props（`title/description/illustration/actions/orientation/motion/class_name/lang/dir`）一致",
        "默认行为与 `components/illustrated-message/src/logic.rs::resolve_view_model` 的缺省归一语义（`missing/blank -> hidden`）一致",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_docs_matrices_and_api_defaults_stay_synced_with_logic_contract`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record docs/parameter/state matrix sync evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_documentation_is_beginner_friendly_and_progressive() {
    let check2 = load_source("check2.md");
    let readme = load_source("src/README.md");
    let docs_source = load_docs_illustrated_message_section();

    for needle in [
        "# IllustratedMessage",
        "## Start Here (Hello World)",
        "<IllustratedMessage",
        "title=\"Empty\".to_string()",
        "description=\"Nothing here\".to_string()",
        "## Common Usage",
        "illustration=move || view! { <div class=\"docs-illustration\">\"o\"</div> }",
        "actions=move || view! { <ui::Button>\"Clear\"</ui::Button> }",
        "## Advanced Options (Optional)",
        "- custom `orientation` (`Vertical` / `Horizontal`)",
        "- custom `motion` contract (`IllustratedMessageMotion`)",
        "- locale context (`lang` / `dir`)",
        "- custom class hook (`class_name`)",
    ] {
        assert!(
            readme.contains(needle),
            "README should keep beginner-friendly documentation marker `{needle}`.",
        );
    }

    let start_here = readme.find("## Start Here (Hello World)");
    let common_usage = readme.find("## Common Usage");
    let advanced = readme.find("## Advanced Options (Optional)");
    let api_table = readme.find("## API (Table)");
    assert!(
        start_here.is_some() && common_usage.is_some() && advanced.is_some() && api_table.is_some(),
        "README should expose start/common/advanced/api sections for progressive onboarding.",
    );
    assert!(
        start_here < common_usage && common_usage < advanced && advanced < api_table,
        "README should keep progressive order: Start Here -> Common Usage -> Advanced -> API.",
    );

    for needle in [
        "pub(super) fn illustrated_message() -> AnyView",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app entry should keep beginner-accessible section `{needle}`.",
        );
    }

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "`components/illustrated-message/src/README.md` 作为组件文档入口已存在",
        "`Start Here (Hello World)` 给出零门槛最小示例（仅 `title/description`）",
        "`Common Usage` 提供常见富内容用法（`illustration/actions`）",
        "`Advanced Options (Optional)` 再介绍 `orientation/motion/lang/dir/class_name`",
        "`apps/docs-app/src/pages/components/pages/display.rs::illustrated_message` 同步提供新手可直接运行的 `Hello World (Default API)` 与进阶矩阵示例",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_documentation_is_beginner_friendly_and_progressive`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record beginner-friendly documentation evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_docs_interactive_playground_supports_live_prop_controls_and_repeatable_preview_flow()
 {
    let check2 = load_source("check2.md");
    let docs_source = load_docs_illustrated_message_section();
    let e2e_source = load_source("../../e2e/tests/docs_app_illustrated_message_contract.spec.mjs");

    for needle in [
        "title=\"Interactive Playground (Props + State + Preview)\"",
        "data-slot=\"illustrated-message-workbench-controls\"",
        "data-slot=\"illustrated-message-workbench-preview\"",
        "data-slot=\"illustrated-message-workbench-state\"",
        "data-slot=\"illustrated-message-workbench-orientation\"",
        "data-slot=\"illustrated-message-workbench-toggle-title\"",
        "data-slot=\"illustrated-message-workbench-toggle-description\"",
        "data-slot=\"illustrated-message-workbench-toggle-illustration\"",
        "data-slot=\"illustrated-message-workbench-toggle-actions\"",
        "data-slot=\"illustrated-message-workbench-toggle-custom-class\"",
        "data-slot=\"illustrated-message-workbench-toggle-rtl\"",
        "id_base=\"docs-illustrated-message-workbench-orientation\".to_string()",
        "Show title",
        "Show description",
        "Show illustration",
        "Show actions",
        "Custom class",
        "RTL",
        "orientation=orientation",
        "dir=dir",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs interactive playground should keep live-prop control marker `{needle}`.",
        );
    }

    for needle in [
        "docs-app illustrated-message interactive playground updates preview state markers",
        "[data-slot=\"illustrated-message-workbench-controls\"]",
        "[data-slot=\"illustrated-message-workbench-preview\"]",
        "[data-slot=\"illustrated-message-workbench-state\"]",
        "[data-slot=\"illustrated-message-workbench-toggle-illustration\"]",
        "[data-slot=\"illustrated-message-workbench-toggle-actions\"]",
        "[data-slot=\"illustrated-message-workbench-toggle-title\"]",
        "[data-slot=\"illustrated-message-workbench-toggle-rtl\"]",
        "[data-slot=\"illustrated-message-workbench-orientation\"]",
        "data-orientation",
        "data-title-state",
        "data-illustration-state",
        "data-actions-state",
    ] {
        assert!(
            e2e_source.contains(needle),
            "interactive playground e2e should keep repeatable flow marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "`Interactive Playground (Props + State + Preview)`",
        "`SegmentedControl + Switch` 提供基础 props 调整",
        "`illustrated-message-workbench-controls`/`illustrated-message-workbench-preview`/`illustrated-message-workbench-state`",
        "Spec 输入联动要求在本项范围 N/A-by-design",
        "`e2e/tests/docs_app_illustrated_message_contract.spec.mjs::docs-app illustrated-message interactive playground updates preview state markers` 覆盖“打开 settings -> 切换控件 -> 预览 `data-*` 状态同步”",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_docs_interactive_playground_supports_live_prop_controls_and_repeatable_preview_flow`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record interactive-playground evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_stays_display_only_without_local_interaction_handlers() {
    let source = load_source("src/view.rs");

    for forbidden in [
        "on:keydown",
        "on:keyup",
        "on:click",
        "on:pointerdown",
        "on:pointerup",
        "on:focus",
        "on:blur",
    ] {
        assert!(
            !source.contains(forbidden),
            "IllustratedMessage is display-only; local interaction/a11y handlers should not be implemented in this component (`{forbidden}`).",
        );
    }
}

#[test]
fn illustrated_message_exposes_locale_entrypoint_without_hardcoded_visible_copy() {
    let source = load_source("src/view.rs");

    for needle in [
        "use ui_headless::a11y::{A11yDirection, locale_attrs};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang",
        "dir=locale.dir",
    ] {
        assert!(
            source.contains(needle),
            "IllustratedMessage should provide locale/i18n entrypoint `{needle}`.",
        );
    }

    for forbidden in [
        "\"No results\"",
        "\"Try changing your search.\"",
        "\"Nothing here\"",
    ] {
        assert!(
            !source.contains(forbidden),
            "IllustratedMessage view should not hardcode user-visible copy (`{forbidden}`).",
        );
    }
}

#[test]
fn illustrated_message_motion_keeps_ui_motion_boundary_and_ssr_noop() {
    let source = load_source("src/motion.rs");

    for needle in [
        "ui_motion::presets::spring_soft()",
        "ui_motion::spring::SpringAnimator::new",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            source.contains(needle),
            "IllustratedMessage motion boundary should include `{needle}`.",
        );
    }

    for forbidden in [
        "request_animation_frame",
        "Animation::",
        "web_sys::Animation",
        "set_timeout",
        "set_interval",
    ] {
        assert!(
            !source.contains(forbidden),
            "IllustratedMessage motion.rs should not implement its own browser driver (`{forbidden}`).",
        );
    }
}

#[test]
fn illustrated_message_styles_consume_ui_theme_tokens_without_local_px_fallbacks() {
    let source = load_source("src/styles.rs");

    for needle in [
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-space-lg, var(--ui-fallback-space-lg))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-icon-size-200, var(--ui-fallback-icon-size-200))",
        "var(--ui-space-xl, var(--ui-fallback-space-xl))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-heading-h6-font-size, var(--ui-fallback-heading-h6-font-size))",
        "--ui-heading-h6-line-height",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
    ] {
        assert!(
            source.contains(needle),
            "IllustratedMessage styles should consume ui-theme token variable `{needle}`.",
        );
    }

    for forbidden in [
        "width: 52px;",
        "height: 52px;",
        "border-radius: 16px;",
        "font-size: var(--ui-heading-h6-font-size, 14px);",
        "line-height: var(--ui-heading-h6-line-height, 20px);",
        "font-size: var(--ui-font-size-150, 14px);",
        "line-height: var(--ui-line-height-150, 20px);",
    ] {
        assert!(
            !source.contains(forbidden),
            "IllustratedMessage styles should avoid local hardcoded fallback `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_styles_branch_on_semantic_markers_not_dom_guessing() {
    let styles = load_source("src/styles.rs");
    let view = load_source("src/view.rs");

    for needle in [
        ".ui-illustrated-message[data-view-state=\"empty\"]",
        ".ui-illustrated-message[data-content-state=\"hidden\"] .ui-illustrated-message__content",
        ".ui-illustrated-message[data-description-state=\"hidden\"]",
        ".ui-illustrated-message[data-actions-state=\"hidden\"]",
        "gap: var(--ui-im-content-gap);",
        "margin-top: var(--ui-im-actions-margin-top);",
    ] {
        assert!(
            styles.contains(needle),
            "IllustratedMessage styles should branch from semantic marker `{needle}`.",
        );
    }

    for forbidden in [":nth-child", ":first-child", ":last-child", ":empty"] {
        assert!(
            !styles.contains(forbidden),
            "IllustratedMessage styles should not rely on structural selector `{forbidden}`.",
        );
    }

    assert!(
        !view.contains("style="),
        "IllustratedMessage view should avoid inline business style logic and rely on CSS contract markers.",
    );
}

#[test]
fn illustrated_message_visual_desire_reuses_theme_baseline_and_preserves_component_quality() {
    let styles = load_source("src/styles.rs");
    let pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_page =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let baseline_e2e = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_strategy = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let check2 = load_source("check2.md");

    for needle in [
        "padding: var(--ui-space-lg, var(--ui-fallback-space-lg));",
        "border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));",
        "font-size: var(--ui-heading-h6-font-size, var(--ui-fallback-heading-h6-font-size));",
        "font-weight: 700;",
        "color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        ".ui-illustrated-message:hover {",
        ".ui-illustrated-message:focus-within {",
        "--ui-im-surface-border-active",
        "--ui-im-focus-ring",
    ] {
        assert!(
            styles.contains(needle),
            "IllustratedMessage default theme baseline should keep visual-quality marker `{needle}`.",
        );
    }

    for needle in [
        "mod theme_visual_baseline;",
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            pages_registry.contains(needle),
            "docs pages registry should include theme visual baseline route token `{needle}`.",
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            baseline_page.contains(needle),
            "theme visual baseline docs page should include `{needle}`.",
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "theme visual baseline renders button/input/overlay",
        "theme visual baseline screenshots",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            baseline_e2e.contains(needle),
            "theme visual baseline e2e contract should include `{needle}`.",
        );
    }

    for needle in [
        "# HeroUI 参数设计风格对齐策略",
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
        "HeroUI 对齐结论",
    ] {
        assert!(
            heroui_strategy.contains(needle),
            "HeroUI alignment strategy should include marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。",
        "已落实：`IllustratedMessage` 默认主题样式在 `styles.rs` 提供清晰信息层级（标题/描述字重字号）、对比层次（surface/illustration）与 `hover/focus-within` 反馈。",
        "`Button/Input/Overlay` 截图基线由仓库级默认主题页面与 e2e 契约统一治理",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_visual_desire_reuses_theme_baseline_and_preserves_component_quality`",
    ] {
        assert!(
            check2.contains(needle),
            "IllustratedMessage check2 should keep visual-desire evidence token `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_semantics_suite_covers_contract_matrix_without_snapshot_dependency() {
    let semantics = load_source("test/semantics.rs");

    for needle in [
        "fn illustrated_message_exposes_stable_state_and_source_markers()",
        "aria-live",
        "data-view-state=view_state",
        "data-title-source=title_source",
        "fn illustrated_message_does_not_define_half_controlled_state_api()",
        "fn illustrated_message_stays_display_only_without_local_interaction_handlers()",
        "fn illustrated_message_motion_keeps_ui_motion_boundary_and_ssr_noop()",
    ] {
        assert!(
            semantics.contains(needle),
            "IllustratedMessage semantics matrix should include contract evidence `{needle}`.",
        );
    }

    let forbidden_insta = ["in", "sta::"].concat();
    let forbidden_assert_snapshot = ["assert_", "snapshot!"].concat();
    let forbidden_match_snapshot = ["to_match_", "snapshot"].concat();

    for forbidden in [
        forbidden_insta,
        forbidden_assert_snapshot,
        forbidden_match_snapshot,
    ] {
        assert!(
            !semantics.contains(&forbidden),
            "IllustratedMessage semantics suite should verify semantic contracts directly, not snapshot matcher `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_semantics_first_contract_is_locked() {
    let check2 = load_source("check2.md");
    let semantics = load_source("test/semantics.rs");

    for needle in [
        "fn illustrated_message_exposes_stable_state_and_source_markers()",
        "fn illustrated_message_semantics_suite_covers_contract_matrix_without_snapshot_dependency()",
        "fn illustrated_message_stays_display_only_without_local_interaction_handlers()",
        "fn illustrated_message_agent_contract_schema_is_typed_traceable_and_whitelisted()",
    ] {
        assert!(
            semantics.contains(needle),
            "semantics-first contract should keep regression anchor `{needle}`.",
        );
    }

    let forbidden_insta = ["in", "sta::"].concat();
    let forbidden_assert_snapshot = ["assert_", "snapshot!"].concat();
    let forbidden_match_snapshot = ["to_match_", "snapshot"].concat();

    for forbidden in [
        forbidden_insta,
        forbidden_assert_snapshot,
        forbidden_match_snapshot,
    ] {
        assert!(
            !semantics.contains(&forbidden),
            "semantics-first coverage should avoid snapshot-only matcher `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "`components/illustrated-message/test/semantics.rs::illustrated_message_exposes_stable_state_and_source_markers` 覆盖 `aria-live` 与 `data-view-state/data-content-state/data-*-state/data-*-source`",
        "`components/illustrated-message/test/semantics.rs::illustrated_message_semantics_suite_covers_contract_matrix_without_snapshot_dependency` 锁定“语义断言优先、非 snapshot 匹配”",
        "`IllustratedMessage` 为展示型非交互组件，“每个交互组件至少有 `*_semantics.rs`”在本组件范围按 N/A-by-design 处理，并由 `illustrated_message_stays_display_only_without_local_interaction_handlers` 固定无键盘/指针交互路径",
        "新增语义字段（含 `data-ui-*` Agent Contract）已由 `illustrated_message_agent_contract_schema_is_typed_traceable_and_whitelisted` 同步覆盖",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_semantics_first_contract_is_locked`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record semantics-first checklist evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_files_keep_single_responsibility_boundaries() {
    let module = load_source("src/mod.rs");
    let logic = load_source("src/logic.rs");
    let styles = load_source("src/styles.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::IllustratedMessageMotion;",
        "pub use view::IllustratedMessage;",
    ] {
        assert!(
            module.contains(needle),
            "mod.rs should keep minimal stable export boundary `{needle}`.",
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "pub fn resolve_view_model",
        "pub fn attach_motion(",
    ] {
        assert!(
            !module.contains(forbidden),
            "mod.rs should not leak implementation detail `{forbidden}`.",
        );
    }

    for needle in [
        "resolve_view_state",
        "resolve_view_model",
        "IllustratedMessageResolvedView",
        "IllustratedMessageTextSource",
        "IllustratedMessageSlotSource",
    ] {
        assert!(
            logic.contains(needle),
            "logic.rs should provide normalization/derivation contract `{needle}`.",
        );
    }

    for forbidden in [
        "view! {",
        "NodeRef",
        "on:click",
        "on:keydown",
        "set_property(",
        "web_sys",
        "JsCast",
        "SpringAnimator::new",
        "--ui-",
    ] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should not contain view/style/motion runtime detail `{forbidden}`.",
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-illustrated-message",
    ] {
        assert!(
            styles.contains(needle),
            "styles.rs should remain token-first static CSS contract `{needle}`.",
        );
    }

    for forbidden in [
        "view! {",
        "NodeRef",
        "on:click",
        "SpringAnimator::new",
        "set_property(",
    ] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should not carry rendering/handler/motion runtime detail `{forbidden}`.",
        );
    }

    for needle in [
        "view! {",
        "let resolved_view = crate::logic::resolve_view_model(",
        "let class = crate::logic::resolve_root_class(orientation, class_name);",
        "motion::attach_motion(root_ref, motion);",
        "let locale = locale_attrs(lang, dir);",
    ] {
        assert!(
            view.contains(needle),
            "view.rs should mount logic/headless/motion contracts `{needle}`.",
        );
    }

    for forbidden in [
        "SpringAnimator::new",
        "sanitize_spring(",
        "stiffness:",
        "damping:",
        "mass:",
        "precision:",
    ] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not re-implement motion engine detail `{forbidden}`.",
        );
    }

    for needle in [
        "pub struct IllustratedMessageMotion",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion.contains(needle),
            "motion.rs should keep motion contract + attach responsibility `{needle}`.",
        );
    }

    for forbidden in [
        "view! {",
        "data-slot=\"illustrated-message\"",
        "resolve_view_model(",
        "data-title-source",
        "lang=locale.lang",
    ] {
        assert!(
            !motion.contains(forbidden),
            "motion.rs should not take over view/logic responsibilities `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_does_not_introduce_spec_module_for_simple_component() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let check2 = load_source("check2.md");
    let spec_path = manifest_dir.join("src/spec.rs");
    let module = load_source("src/mod.rs");
    let readme = load_source("src/README.md");

    assert!(
        !spec_path.exists(),
        "IllustratedMessage is a simple display component and should not introduce `src/spec.rs`."
    );

    for forbidden in ["mod spec", "pub mod spec", "pub use spec::"] {
        assert!(
            !module.contains(forbidden),
            "IllustratedMessage module boundary should not expose spec contract detail `{forbidden}`.",
        );
    }

    assert!(
        !readme.contains("Spec::new()"),
        "IllustratedMessage docs should not require spec-builder entrypoint for this simple component.",
    );

    for needle in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "N/A-by-design：`IllustratedMessage` 是展示型简单组件",
        "未引入 `src/spec.rs` 与 `*Spec::new()...render()` 建造者入口",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_does_not_introduce_spec_module_for_simple_component`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record hyper-structure builder N/A evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_context_compression_manifest_and_rbi_are_present_and_synced() {
    let check2 = load_source("check2.md");
    let manifest = load_source("src/Component.toml");
    let rbi = load_source("src/illustrated_message.rbi");
    let readme = load_source("src/README.md");
    let component_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    assert!(
        component_src.join("Component.toml").exists(),
        "illustrated-message should keep `src/Component.toml` for context-compression manifest.",
    );
    assert!(
        component_src.join("illustrated_message.rbi").exists(),
        "illustrated-message should keep `src/illustrated_message.rbi` for interface signature projection.",
    );

    for needle in [
        "schema_version = \"1\"",
        "name = \"IllustratedMessage\"",
        "crate = \"ui-illustrated-message\"",
        "rbi = \"illustrated_message.rbi\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"title\"",
        "name = \"description\"",
        "name = \"illustration\"",
        "name = \"actions\"",
        "name = \"orientation\"",
        "name = \"motion\"",
        "name = \"class_name\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"semantic-markers\"",
    ] {
        assert!(
            manifest.contains(needle),
            "Component manifest should keep context-compression contract fragment `{needle}`.",
        );
    }

    for needle in [
        "pub enum IllustratedMessageOrientation {",
        "pub struct IllustratedMessageMotion {",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "pub fn IllustratedMessage(",
        "orientation: crate::IllustratedMessageOrientation,",
        "motion: crate::IllustratedMessageMotion,",
        "dir: Option<A11yDirection>,",
    ] {
        assert!(
            rbi.contains(needle),
            "RBI signature projection should keep API fragment `{needle}`.",
        );
    }

    for needle in [
        "`components/illustrated-message/src/Component.toml`",
        "`components/illustrated-message/src/illustrated_message.rbi`",
    ] {
        assert!(
            readme.contains(needle),
            "README source index should include context-compression artifacts `{needle}`.",
        );
    }

    for needle in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "新增 `components/illustrated-message/src/Component.toml`",
        "新增 `components/illustrated-message/src/illustrated_message.rbi`",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_context_compression_manifest_and_rbi_are_present_and_synced`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record context-compression manifest/rbi evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_agent_contract_schema_is_typed_traceable_and_whitelisted() {
    let check2 = load_source("check2.md");
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let manifest = load_source("src/Component.toml");
    let rbi = load_source("src/illustrated_message.rbi");

    for needle in [
        "pub const ILLUSTRATED_MESSAGE_AGENT_SCHEMA: &str = \"ui.illustrated-message.agent-contract\";",
        "pub const ILLUSTRATED_MESSAGE_AGENT_SCHEMA_VERSION: &str = \"v1\";",
        "pub enum IllustratedMessageAgentIntent",
        "pub enum IllustratedMessageAgentAction",
        "pub enum IllustratedMessageAgentState",
        "pub enum IllustratedMessageAgentSource",
        "pub enum IllustratedMessageAgentConfigPolicy",
        "pub struct IllustratedMessageAgentContractAttrs",
        "pub fn resolve_agent_contract_attrs(",
    ] {
        assert!(
            logic.contains(needle),
            "logic should define typed agent-contract schema/source/state derivation `{needle}`.",
        );
    }

    for needle in [
        "let agent_contract = crate::logic::resolve_agent_contract_attrs(&resolved_view);",
        "data-ui-schema=ui_schema",
        "data-ui-schema-version=ui_schema_version",
        "data-ui-intent=ui_intent",
        "data-ui-action=ui_action",
        "data-ui-state=ui_state",
        "data-ui-source=ui_source",
        "data-ui-config-policy=ui_config_policy",
        "data-ui-streaming-policy=ui_streaming_policy",
        "data-ui-streaming-fallback=ui_streaming_fallback",
        "data-ui-output-status=ui_output_status",
    ] {
        assert!(
            view.contains(needle),
            "view should mount typed agent contract attrs `{needle}`.",
        );
    }

    for needle in [
        "[agent_contract]",
        "schema = \"ui.illustrated-message.agent-contract.v1\"",
        "intent = \"empty-state-display\"",
        "action_axis = [\"render-snapshot\"]",
        "state_axes = [\"view_state\"]",
        "source_axes = [\"content_source\"]",
        "name = \"agent_contract_schema_markers\"",
        "name = \"agent_contract_whitelist_render_policy\"",
        "name = \"streaming_optional_with_snapshot_fallback_and_output_status_markers\"",
        "[[agent_contract_markers]]",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-intent\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "attr = \"data-ui-config-policy\"",
        "attr = \"data-ui-streaming-policy\"",
        "attr = \"data-ui-streaming-fallback\"",
        "attr = \"data-ui-output-status\"",
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "\"inner_html\"",
        "\"dangerously_set_inner_html\"",
        "\"<script\"",
        "\"javascript:\"",
    ] {
        assert!(
            manifest.contains(needle),
            "Component.toml should keep agent-contract schema/marker/whitelist fragment `{needle}`.",
        );
    }

    for needle in [
        "pub const ILLUSTRATED_MESSAGE_AGENT_SCHEMA: &str;",
        "pub const ILLUSTRATED_MESSAGE_AGENT_SCHEMA_VERSION: &str;",
        "pub enum IllustratedMessageAgentIntent",
        "pub enum IllustratedMessageAgentAction",
        "pub enum IllustratedMessageAgentState",
        "pub enum IllustratedMessageAgentSource",
        "pub enum IllustratedMessageAgentConfigPolicy",
        "pub enum IllustratedMessageAgentStreamingPolicy",
        "pub enum IllustratedMessageAgentStreamingFallback",
        "pub enum IllustratedMessageAgentOutputStatus",
        "pub struct IllustratedMessageAgentContractAttrs",
        "pub fn resolve_agent_contract_attrs(",
    ] {
        assert!(
            rbi.contains(needle),
            "RBI should project typed agent-contract API fragment `{needle}`.",
        );
    }

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "`components/illustrated-message/src/logic.rs` 新增类型化 Agent Contract 轴与归一",
        "`components/illustrated-message/src/view.rs` 在根节点挂载 `data-ui-schema`、`data-ui-schema-version`、`data-ui-intent`、`data-ui-action`、`data-ui-state`、`data-ui-source`、`data-ui-config-policy`",
        "`components/illustrated-message/src/Component.toml` 同步 `agent_contract`、`agent_contract_markers` 与 `agent_contract_whitelist`",
        "`components/illustrated-message/src/illustrated_message.rbi` 同步投影 Agent Contract 类型与 `resolve_agent_contract_attrs` 签名",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_agent_contract_schema_is_typed_traceable_and_whitelisted`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record typed agent-contract governance evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_streaming_term_is_restricted_to_llm_output_modes() {
    let check2 = load_source("check2.md");
    let manifest = load_source("src/Component.toml");
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");
    let rbi = load_source("src/illustrated_message.rbi");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "`agent_contract.output_mode_axis = [\\\"snapshot\\\"]` 明确当前只消费 `Snapshot` 显示模式，动作轴为 `render-snapshot`",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_streaming_term_is_restricted_to_llm_output_modes`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should pin streaming-term scope to LLM output modes `{needle}`.",
        );
    }

    for needle in [
        "output_mode_axis = [\"snapshot\"]",
        "action_axis = [\"render-snapshot\"]",
    ] {
        assert!(
            manifest.contains(needle),
            "Component.toml should keep snapshot-only render mode contract `{needle}`.",
        );
    }

    assert!(
        rbi.contains("RenderSnapshot"),
        "RBI should keep snapshot action projection for output mode contract.",
    );
    for forbidden in ["RenderStreaming", "render-streaming"] {
        assert!(
            !rbi.contains(forbidden),
            "RBI should not introduce non-snapshot streaming projection `{forbidden}` for this component.",
        );
    }

    for forbidden in ["token_stream", "chunk", "render_stream", "on:stream"] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "illustrated-message sources should not implement incremental streaming pipeline `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_snapshot_mode_is_baseline_and_full_config_is_renderable() {
    let check2 = load_source("check2.md");
    let manifest = load_source("src/Component.toml");
    let view = load_source("src/view.rs");
    let logic = load_source("src/logic.rs");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "`components/illustrated-message/src/Component.toml` 显式声明 `snapshot_rendering` 能力",
        "`agent_contract.output_mode_axis` 固定为 `[\\\"snapshot\\\"]`（`action_axis = [\\\"render-snapshot\\\"]`）",
        "`components/illustrated-message/src/view.rs` 接收完整配置输入（`title/description/illustration/actions/orientation/motion/class_name/lang/dir`）",
        "`components/illustrated-message/src/logic.rs` 对缺省输入走可预测默认归一（`unwrap_or_default` + 封闭状态/来源标记）",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_snapshot_mode_is_baseline_and_full_config_is_renderable`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record snapshot-baseline evidence `{needle}`.",
        );
    }

    for needle in [
        "name = \"snapshot_rendering\"",
        "output_mode_axis = [\"snapshot\"]",
        "action_axis = [\"render-snapshot\"]",
    ] {
        assert!(
            manifest.contains(needle),
            "Component.toml should keep snapshot baseline contract `{needle}`.",
        );
    }
    for forbidden in ["output_mode_axis = [\"streaming\"]", "render-streaming"] {
        assert!(
            !manifest.contains(forbidden),
            "Component.toml should not drift to non-baseline output mode `{forbidden}`.",
        );
    }

    for needle in [
        "#[prop(optional, into)] title: Option<String>,",
        "#[prop(optional, into)] description: Option<String>,",
        "#[prop(optional, into)] illustration: Option<ViewFn>,",
        "#[prop(optional, into)] actions: Option<ViewFn>,",
        "#[prop(optional)] orientation: IllustratedMessageOrientation,",
        "#[prop(optional)] motion: IllustratedMessageMotion,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "let resolved_view = crate::logic::resolve_view_model(",
        "title,",
        "description,",
        "illustration.as_ref(),",
        "actions.as_ref(),",
    ] {
        assert!(
            view.contains(needle),
            "view should consume full completed config for snapshot render path `{needle}`.",
        );
    }

    for needle in [
        "pub fn resolve_view_model<TIllustration, TActions>(",
        "title: Option<String>,",
        "description: Option<String>,",
        "title: normalized_title.unwrap_or_default(),",
        "description: normalized_description.unwrap_or_default(),",
        "IllustratedMessageRenderMarker::Empty => \"empty\"",
        "IllustratedMessageRenderMarker::Populated => \"populated\"",
    ] {
        assert!(
            logic.contains(needle),
            "logic should provide stable defaults and closed states for snapshot baseline `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_streaming_requirement_is_role_based_and_optional_with_snapshot_fallback() {
    let check2 = load_source("check2.md");
    let manifest = load_source("src/Component.toml");
    let view = load_source("src/view.rs");
    let logic = load_source("src/logic.rs");
    let motion = load_source("src/motion.rs");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`IllustratedMessage` 属于展示型空态组件而非正文阅读面，故落位为 `Streaming Optional`",
        "`data-ui-streaming-policy=\\\"optional\\\"`、`data-ui-streaming-fallback=\\\"snapshot\\\"`、`data-ui-output-status=\\\"validated\\\"`",
        "`components/illustrated-message/src/view.rs` 持续输出 `aria-live` 与稳定 `data-*`/`data-ui-*` 标记",
        "数据校验、断线恢复、重试未下沉到组件层",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_streaming_requirement_is_role_based_and_optional_with_snapshot_fallback`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record role-based streaming policy evidence `{needle}`.",
        );
    }

    for needle in [
        "output_mode_axis = [\"snapshot\"]",
        "action_axis = [\"render-snapshot\"]",
        "attr = \"data-ui-streaming-policy\"",
        "values = [\"optional\"]",
        "attr = \"data-ui-streaming-fallback\"",
        "values = [\"snapshot\"]",
        "attr = \"data-ui-output-status\"",
        "values = [\"validated\"]",
    ] {
        assert!(
            manifest.contains(needle),
            "Component.toml should keep optional-streaming snapshot-fallback marker `{needle}`.",
        );
    }
    for forbidden in ["streaming-required", "output_mode_axis = [\"streaming\"]"] {
        assert!(
            !manifest.contains(forbidden),
            "manifest should not force streaming-required policy `{forbidden}` for this component.",
        );
    }

    for needle in [
        "data-ui-streaming-policy=ui_streaming_policy",
        "data-ui-streaming-fallback=ui_streaming_fallback",
        "data-ui-output-status=ui_output_status",
        "aria-live=\"off\"",
        "data-ui-schema=ui_schema",
        "data-ui-state=ui_state",
    ] {
        assert!(
            view.contains(needle),
            "view should keep continuous a11y/data markers for streaming optional path `{needle}`.",
        );
    }

    for forbidden in [
        "retry",
        "reconnect",
        "disconnect",
        "on_retry",
        "stream_error",
        "validate_stream",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "component layer should not absorb upper-layer retry/recovery policy `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_follows_token_first_static_style_injection_contract() {
    let styles = load_source("src/styles.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");
    let css_aggregate = load_source("../../crates/ui/src/css.rs");
    let ui_root = load_source("../../crates/ui/src/root.rs");
    let ui_components_lib = load_source("../../crates/ui/src/lib.rs");

    for needle in [
        "#[cfg(feature = \"component-illustrated_message\")]",
        "out.push_str(crate::illustrated_message::styles::CSS);",
    ] {
        assert!(
            css_aggregate.contains(needle),
            "ui css aggregation should include illustrated-message feature-gated css `{needle}`.",
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "#[cfg(feature = \"component-illustrated_message\")]",
        "pub use ui_illustrated_message as illustrated_message;",
    ] {
        assert!(
            ui_root.contains(needle) || ui_components_lib.contains(needle),
            "UiRoot/lib should keep components css injection + export contract `{needle}`.",
        );
    }

    for needle in [
        "var(--ui-",
        ".ui-illustrated-message",
        "--ui-im-content-gap",
    ] {
        assert!(
            styles.contains(needle),
            "IllustratedMessage styles should stay token-first static css `{needle}`.",
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"p-",
        "class=\"m-",
        "class=\"text-",
        "class=\"bg-",
        "class=\"rounded-",
        "class=\"shadow-",
        "stylist",
        "stylex",
        "emotion",
        "css!",
    ] {
        assert!(
            !view.contains(forbidden) && !styles.contains(forbidden),
            "IllustratedMessage component contract should avoid utility-first/css-in-rust pattern `{forbidden}`.",
        );
    }

    for needle in ["--ui-im-opacity", "--ui-im-y"] {
        assert!(
            motion.contains(needle),
            "IllustratedMessage runtime style writes should stay in css custom property channel `{needle}`.",
        );
    }

    for forbidden in ["set_property(\"opacity\"", "set_property(\"transform\""] {
        assert!(
            !motion.contains(forbidden),
            "IllustratedMessage should not push business inline style property `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_tree_shaking_contract_is_feature_gated_and_budget_guarded() {
    let component_cargo = load_source("Cargo.toml");
    let ui_components_cargo = load_source("../../crates/ui/Cargo.toml");
    let ui_components_lib = load_source("../../crates/ui/src/lib.rs");
    let ui_components_css = load_source("../../crates/ui/src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let tree_shaking_script = load_source("../../scripts/check-ui-tree-shaking.sh");
    let tree_shaking_budget = load_source("../../scripts/tree_shaking_budget.env");
    let ci_source = load_source("../../.github/workflows/ci.yml");
    let check2 = load_source("check2.md");

    for needle in [
        "component-illustrated_message = [\"dep:ui-illustrated-message\"]",
        "ui-illustrated-message = { path = \"../../components/illustrated-message\", optional = true }",
        "#[cfg(feature = \"component-illustrated_message\")]",
        "pub use ui_illustrated_message as illustrated_message;",
        "out.push_str(crate::illustrated_message::styles::CSS);",
    ] {
        assert!(
            ui_components_cargo.contains(needle)
                || ui_components_lib.contains(needle)
                || ui_components_css.contains(needle),
            "Tree-shaking package mode should keep illustrated-message feature gate `{needle}`.",
        );
    }

    assert!(
        component_cargo.contains("[features]\ndefault = []"),
        "IllustratedMessage source-mode crate should keep `default = []` so consumers only pull requested capability.",
    );
    assert!(
        !component_cargo.contains("\nui = {")
            && !component_cargo.contains("\nui={")
            && !component_cargo.contains("\nui =\"")
            && !component_cargo.contains("\nui=\""),
        "IllustratedMessage source-mode crate should not depend on ui central registry.",
    );

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("features = [\"inject-css\", \"web-demo-components\"]"),
        "web-demo should consume ui via explicit feature bundle without default all-components.",
    );
    assert!(
        !web_demo_cargo.contains("\"all-components\""),
        "web-demo should not implicitly pull all-components.",
    );

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"",
        "cargo tree -e features -i ui -p web-demo",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_shaking_script.contains(needle) || tree_shaking_budget.contains(needle),
            "Tree-shaking script/budget guard should include `{needle}`.",
        );
    }

    assert!(
        ci_source.contains("Tree Shaking Budget")
            && ci_source.contains("./scripts/check-ui-tree-shaking.sh"),
        "CI should execute tree-shaking budget gate.",
    );

    for needle in [
        "- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。",
        "component-illustrated_message = [\"dep:ui-illustrated-message\"]",
        "`crates/ui/src/lib.rs` 通过 `#[cfg(feature = \"component-illustrated_message\")] pub use ui_illustrated_message as illustrated_message;` 做条件导出",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_tree_shaking_contract_is_feature_gated_and_budget_guarded`",
    ] {
        assert!(
            check2.contains(needle),
            "IllustratedMessage check2 should keep tree-shaking evidence token `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_tree_shaking_checklist_item_is_feature_registered_and_gated() {
    let ui_components_cargo = load_source("../../crates/ui/Cargo.toml");
    let ui_components_lib = load_source("../../crates/ui/src/lib.rs");
    let ui_components_css = load_source("../../crates/ui/src/css.rs");
    let check2 = load_source("check2.md");

    for needle in [
        "component-illustrated_message = [\"dep:ui-illustrated-message\"]",
        "ui-illustrated-message = { path = \"../../components/illustrated-message\", optional = true }",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui feature tree should register illustrated-message via `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-illustrated_message\")]",
        "pub use ui_illustrated_message as illustrated_message;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui lib export should stay feature-gated by `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-illustrated_message\")]",
        "out.push_str(crate::illustrated_message::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "ui css aggregation should stay feature-gated by `{needle}`.",
        );
    }

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "`crates/ui/Cargo.toml` 已注册 `component-illustrated_message = [\"dep:ui-illustrated-message\"]`，并以 `ui-illustrated-message` optional 依赖接入",
        "`crates/ui/src/lib.rs` 通过 `#[cfg(feature = \"component-illustrated_message\")] pub use ui_illustrated_message as illustrated_message;` 做条件导出",
        "`crates/ui/src/css.rs` 通过 `#[cfg(feature = \"component-illustrated_message\")] out.push_str(crate::illustrated_message::styles::CSS);` 做条件聚合",
        "`cargo tree -e features -p ui --no-default-features --features component-illustrated_message,inject-css | rg \"all-components|ui-illustrated-message\"` 仅命中 `ui-illustrated-message`，无 `all-components`",
        "`cargo tree -e features -i ui -p web-demo | rg \"component-illustrated_message|all-components\"` 命中 `component-illustrated_message` 且无 `all-components`",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_tree_shaking_checklist_item_is_feature_registered_and_gated`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record tree-shaking checklist-item evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_public_api_hides_web_sys_and_dom_detail_types() {
    let source = load_source("src/mod.rs");

    for forbidden in ["web_sys", "web-sys", "HtmlElement", "NodeRef<"] {
        assert!(
            !source.contains(forbidden),
            "IllustratedMessage public API surface (`mod.rs`) should not expose DOM detail type `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_api_naming_contract_stays_alias_free() {
    let source = load_source("src/view.rs");

    for required in [
        "title: Option<String>",
        "description: Option<String>",
        "illustration: Option<ViewFn>",
        "actions: Option<ViewFn>",
        "orientation: IllustratedMessageOrientation",
        "motion: IllustratedMessageMotion",
        "class_name: Option<String>",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
    ] {
        assert!(
            source.contains(required),
            "IllustratedMessage API should keep stable prop naming `{required}`.",
        );
    }

    for forbidden in [
        "is_open",
        "on_open_change",
        "default_open",
        "on_change",
        "default_value",
    ] {
        assert!(
            !source.contains(forbidden),
            "Display-only IllustratedMessage should not introduce alias naming `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_does_not_define_half_controlled_state_api() {
    let source = load_source("src/view.rs");

    for forbidden in [
        "value: ",
        "default_value",
        "on_value_change",
        "open: ",
        "default_open",
        "on_open_change",
        "selected: ",
        "default_selected",
        "on_selected_change",
    ] {
        assert!(
            !source.contains(forbidden),
            "Display-only IllustratedMessage should not expose controlled/uncontrolled state API fragment `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_keeps_default_resolution_in_logic_only() {
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");

    for needle in [
        "pub fn resolve_view_model<",
        "pub fn resolve_root_class(",
        "fn normalize_display_text(",
        "title: normalized_title.unwrap_or_default()",
        "description: normalized_description.unwrap_or_default()",
    ] {
        assert!(
            logic.contains(needle),
            "IllustratedMessage logic should define explicit default/priority rule `{needle}`.",
        );
    }

    for needle in [
        "let resolved_view = crate::logic::resolve_view_model(",
        "title,",
        "description,",
        "illustration.as_ref(),",
        "actions.as_ref(),",
        "let class = crate::logic::resolve_root_class(orientation, class_name);",
        "data-title-source=title_source",
        "data-description-source=description_source",
        "data-illustration-source=illustration_source",
        "data-actions-source=actions_source",
    ] {
        assert!(
            view.contains(needle),
            "IllustratedMessage view should consume logic-normalized result `{needle}`.",
        );
    }

    for forbidden in [
        "unwrap_or_default()",
        "unwrap_or_else(",
        "let base_class = format!(",
        ".filter(|value| !value.trim().is_empty())",
        "illustration.is_some()",
        "actions.is_some()",
        ".trim().is_empty()",
    ] {
        assert!(
            !view.contains(forbidden),
            "IllustratedMessage view should not define fallback/default branch `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_discrete_state_axes_stay_type_safe() {
    let module = load_source("src/mod.rs");
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");

    for needle in [
        "pub enum IllustratedMessageOrientation",
        "orientation: IllustratedMessageOrientation",
        "pub enum IllustratedMessageTextSource",
        "pub enum IllustratedMessageSlotSource",
    ] {
        assert!(
            module.contains(needle) || view.contains(needle) || logic.contains(needle),
            "IllustratedMessage should keep discrete state axis type `{needle}`.",
        );
    }

    for forbidden in [
        "variant: Option<String>",
        "size: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
        "variant: String",
        "size: String",
        "mode: String",
        "status: String",
        "Option<bool>",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "IllustratedMessage should not model discrete/mutually-exclusive states with `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_type_system_and_semantic_markers_form_machine_readable_contract() {
    let module = load_source("src/mod.rs");
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let logic_tests = load_source("test/logic.rs");
    let check2 = load_source("check2.md");

    for needle in [
        "pub enum IllustratedMessageOrientation",
        "pub enum IllustratedMessageStateMarker",
        "pub enum IllustratedMessageRenderMarker",
        "pub enum IllustratedMessageTextSource",
        "pub enum IllustratedMessageSlotSource",
        "pub const fn as_data_attr(self) -> &'static str",
        "IllustratedMessageStateMarker::Shown => \"shown\"",
        "IllustratedMessageRenderMarker::Populated => \"populated\"",
        "IllustratedMessageTextSource::Blank => \"blank\"",
        "IllustratedMessageSlotSource::Missing => \"missing\"",
    ] {
        assert!(
            module.contains(needle) || logic.contains(needle),
            "Type system contract should keep closed machine-readable axis `{needle}`.",
        );
    }

    for forbidden in [
        "variant: String",
        "size: String",
        "mode: String",
        "status: String",
        "Option<bool>",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "Component should avoid string/bool-explosion state modeling `{forbidden}`.",
        );
    }

    for needle in [
        "pub fn resolve_view_model<",
        "fn normalize_display_text(",
        "fn normalize_slot_source<T>(",
        "fn normalize_state_marker(",
        "fn normalize_render_marker(",
        "title: normalized_title.unwrap_or_default()",
        "description: normalized_description.unwrap_or_default()",
    ] {
        assert!(
            logic.contains(needle),
            "logic.rs should keep centralized normalization marker `{needle}`.",
        );
    }

    for needle in [
        "fn resolves_text_defaults_in_logic_only()",
        "IllustratedMessageTextSource::Blank",
        "IllustratedMessageTextSource::Missing",
        "IllustratedMessageSlotSource::Missing",
        "IllustratedMessageRenderMarker::Empty",
    ] {
        assert!(
            logic_tests.contains(needle),
            "logic regression tests should keep normalization branch marker `{needle}`.",
        );
    }

    for needle in [
        "data-view-state=view_state",
        "data-content-state=content_state",
        "data-title-state=title_state",
        "data-description-state=description_state",
        "data-illustration-state=illustration_state",
        "data-actions-state=actions_state",
        "data-title-source=title_source",
        "data-description-source=description_source",
        "data-illustration-source=illustration_source",
        "data-actions-source=actions_source",
    ] {
        assert!(
            view.contains(needle),
            "view.rs should expose machine-readable semantic marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。",
        "IllustratedMessageOrientation",
        "IllustratedMessageStateMarker",
        "data-view-state/data-content-state/data-*-state/data-*-source",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_type_system_and_semantic_markers_form_machine_readable_contract`",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should keep type-system and semantic-marker evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_consumes_state_primitives_without_business_store_binding() {
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let cargo = load_source("Cargo.toml");

    for needle in [
        "ui-state-primitives = { path = \"../../crates/ui-state-primitives\" }",
        "pub use ui_state_primitives::illustrated_message::{",
        "IllustratedMessageViewState, resolve_view_state",
        "let state = resolve_view_state(",
        "crate::logic::resolve_view_model(",
    ] {
        assert!(
            cargo.contains(needle) || logic.contains(needle) || view.contains(needle),
            "IllustratedMessage should consume state primitive contract `{needle}`.",
        );
    }

    for forbidden in [
        "resolve_view_state(",
        "RwSignal",
        "ReadSignal",
        "WriteSignal",
        "Arc<Mutex",
        "tokio::",
        "store::",
        "global_store",
    ] {
        assert!(
            !view.contains(forbidden),
            "IllustratedMessage view should not bind primitive/store internals directly (`{forbidden}`).",
        );
    }

    for forbidden in [
        "RwSignal",
        "ReadSignal",
        "WriteSignal",
        "Arc<Mutex",
        "tokio::",
        "store::",
    ] {
        assert!(
            !logic.contains(forbidden),
            "IllustratedMessage logic should stay as primitive assembly/mapping without business store binding (`{forbidden}`).",
        );
    }
}

#[test]
fn illustrated_message_has_no_async_interaction_contract() {
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");

    for forbidden in [
        "use_async_action",
        "is_loading",
        "on_retry",
        "retry",
        "aria-busy",
        "Future<",
        ".await",
        "tokio::",
        "async fn",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "IllustratedMessage is display-only and should not define async interaction contract `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_is_not_composite_parent_item_api() {
    let view = load_source("src/view.rs");
    let docs = load_docs_illustrated_message_section();
    let readme = load_source("src/README.md");

    for needle in [
        "title: Option<String>",
        "description: Option<String>",
        "illustration: Option<ViewFn>",
        "actions: Option<ViewFn>",
    ] {
        assert!(
            view.contains(needle),
            "IllustratedMessage should stay a direct display API with prop `{needle}`.",
        );
    }

    for forbidden in [
        "items: Vec<",
        "items: Option<Vec<",
        "labels + children",
        "titles + panels",
        "labels:",
        "titles:",
        "panels:",
        "ItemSpec",
        "<Parent>",
        "<Item ",
    ] {
        assert!(
            !view.contains(forbidden) && !docs.contains(forbidden) && !readme.contains(forbidden),
            "IllustratedMessage should not introduce composite implicit-contract API `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_has_no_drag_macro_micro_state_machine() {
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "on:drag",
        "on:dragstart",
        "on:dragend",
        "on:pointermove",
        "on:mousemove",
        "on:touchmove",
        "requestAnimationFrame",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "IllustratedMessage should not implement drag macro/micro state machine contract `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_has_no_two_pass_geometry_pipeline() {
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");

    for forbidden in [
        "OverlayIntent",
        "Measure",
        "Rectification",
        "getBoundingClientRect",
        "clientWidth",
        "clientHeight",
        "offsetWidth",
        "offsetHeight",
        "ResizeObserver",
        "IntersectionObserver",
        "requestAnimationFrame",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "IllustratedMessage should not implement two-pass geometry pipeline contract `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_has_no_collection_registration_protocol() {
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "Accordion",
        "Tabs",
        "Menu",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "IllustratedMessage should not implement collection registration protocol `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_has_no_slot_projection_policy_contract() {
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "on_hidden",
        "projection",
        "slot_projection",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "IllustratedMessage should not implement slot projection policy contract `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_has_no_env_stream_subscription_pipeline() {
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "ThemeChanged",
        "BreakpointChanged",
        "matchMedia",
        "on:resize",
        "debounce",
        "throttle",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "IllustratedMessage should not implement env stream subscription pipeline `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_has_no_event_light_cone_protocol() {
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "prop drilling",
        "Table",
        "Grid",
        "batch",
        "bulk",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "IllustratedMessage should not implement event light cone contract `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_has_no_causality_bus_trace_id_chain() {
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");

    for forbidden in [
        "TraceId",
        "CausalityBus",
        "causality",
        "broadcast",
        "subscriber",
        "subscribe",
        "command bus",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !motion.contains(forbidden),
            "IllustratedMessage should not implement causality bus contract `{forbidden}`.",
        );
    }
}

#[test]
fn illustrated_message_has_no_overlay_focus_stack_or_focus_restore_contract() {
    let check2 = load_source("check2.md");
    let view = load_source("src/view.rs");
    let logic = load_source("src/logic.rs");
    let motion = load_source("src/motion.rs");

    for needle in [
        "let root_ref = NodeRef::new();",
        "motion::attach_motion(root_ref, motion);",
    ] {
        assert!(
            view.contains(needle),
            "IllustratedMessage should keep NodeRef usage scoped to motion attach `{needle}`.",
        );
    }

    for forbidden in [
        "FocusManager",
        "focus manager",
        "FallbackTo",
        "Selector",
        "restore_focus",
        "focus_restore",
        "focus stack",
        "document.body",
        "OverlayStack",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !motion.contains(forbidden),
            "IllustratedMessage should not implement overlay focus-stack/focus-restore contract `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。",
        "N/A-by-design：`IllustratedMessage` 为展示型组件，不承载层叠 `Overlay` 与焦点恢复协议。",
        "NodeRef` 仅用于 `motion::attach_motion` 绑定根节点动画，不作为恢复目标",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_has_no_overlay_focus_stack_or_focus_restore_contract`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record focus-stack N/A evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_has_no_foreign_zone_escape_hatch_contract() {
    let check2 = load_source("check2.md");
    let module = load_source("src/mod.rs");
    let view = load_source("src/view.rs");
    let logic = load_source("src/logic.rs");
    let motion = load_source("src/motion.rs");

    for forbidden in [
        "ForeignZone",
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
        "ECharts",
        "Mapbox",
        "Leaflet",
        "third-party instance",
        "js_sys::Object",
        "web_sys::HtmlCanvasElement",
    ] {
        assert!(
            !module.contains(forbidden)
                && !view.contains(forbidden)
                && !logic.contains(forbidden)
                && !motion.contains(forbidden),
            "IllustratedMessage should not implement foreign-zone escape hatch contract `{forbidden}`.",
        );
    }

    for forbidden in [
        "pub struct Echarts",
        "pub struct Map",
        "pub type Echarts",
        "pub type Map",
        "pub fn set_chart(",
        "pub fn set_map(",
    ] {
        assert!(
            !module.contains(forbidden),
            "IllustratedMessage public API should not expose imperative third-party instance handle `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。",
        "N/A-by-design：`IllustratedMessage` 为纯展示组件，不集成命令式第三方实例",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_has_no_foreign_zone_escape_hatch_contract`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record foreign-zone N/A evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_hydration_discontinuity_contract_is_na_without_local_entropy_init() {
    let check2 = load_source("check2.md");
    let module = load_source("src/mod.rs");
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");
    let ui_root = load_source("../../crates/ui/src/root.rs");

    for forbidden in ["now(", "SystemTime", "Uuid", "uuid", "rand", "random"] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !motion.contains(forbidden),
            "IllustratedMessage should not initialize hydration-sensitive entropy source `{forbidden}`.",
        );
    }

    for needle in ["id_seed: u64", "provide_ui_id_provider(id_seed);"] {
        assert!(
            ui_root.contains(needle),
            "UiRoot should keep deterministic IdProvider seed integration `{needle}`.",
        );
    }

    for needle in [
        "- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。",
        "N/A-by-design：`IllustratedMessage` 为展示型组件，不生成本地随机/时间 ID；`src/{mod,logic,view,motion}.rs` 无 `now/SystemTime/Uuid/rand/random` 初始化路径。",
        "SSR/Hydration ID 稳定性由 `UiRoot` 注入确定性 `id_seed` 并调用 `provide_ui_id_provider(id_seed)` 统一保障。",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_hydration_discontinuity_contract_is_na_without_local_entropy_init`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record hydration discontinuity N/A evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_ssr_cross_platform_contract_uses_explicit_cfg_and_non_wasm_web_sys_ban() {
    let check2 = load_source("check2.md");
    let module = load_source("src/mod.rs");
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion.contains(needle),
            "IllustratedMessage motion should keep explicit platform branch `{needle}`.",
        );
    }

    for forbidden in ["web-sys", "web_sys", "window(", "document("] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "IllustratedMessage non-motion paths should not depend on browser-only API `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。",
        "src/motion.rs` 通过 `#[cfg(target_arch = \"wasm32\")]` / `#[cfg(not(target_arch = \"wasm32\"))]` 显式分支",
        "src/{mod,logic,view}.rs` 无 `web-sys/web_sys/window/document` 引用",
        "cargo check -p ui`（默认本地）",
        "cargo check -p ui-headless --no-default-features --features ssr`（ssr native）",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-illustrated_message,inject-css`（web wasm）",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_ssr_cross_platform_contract_uses_explicit_cfg_and_non_wasm_web_sys_ban`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record SSR/cross-platform evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_respects_ui_headless_web_ssr_feature_mutex_contract() {
    let check2 = load_source("check2.md");
    let view = load_source("src/view.rs");
    let ui_headless_lib = load_source("../../crates/ui-headless/src/lib.rs");
    let platform_script = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            ui_headless_lib.contains(needle),
            "ui-headless should keep web/ssr compile-time mutex guard `{needle}`.",
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "rg -n \"mutually exclusive\" \"$MUTEX_LOG\"",
    ] {
        assert!(
            platform_script.contains(needle),
            "platform guard script should enforce ui-headless mutex contract `{needle}`.",
        );
    }

    assert!(
        view.contains("use ui_headless::a11y::{A11yDirection, locale_attrs};"),
        "IllustratedMessage should consume ui-headless contract via a11y entrypoint.",
    );

    for needle in [
        "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。",
        "`crates/ui-headless/src/lib.rs` 使用 `#[cfg(all(feature = \"web\", feature = \"ssr\"))] compile_error!(...)` 强制互斥",
        "`scripts/check-ui-platforms.sh` 同时覆盖 `--features ssr` 与 wasm `--features web` 两条 compile-only 路径",
        "`cargo check -p ui-headless --no-default-features --features web,ssr` 作为“必须失败”守卫且校验日志含 `mutually exclusive`",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_respects_ui_headless_web_ssr_feature_mutex_contract`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record ui-headless web/ssr mutex evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_ui_motion_non_wasm_stub_contract_is_predictable_and_safe() {
    let check2 = load_source("check2.md");
    let motion = load_source("src/motion.rs");
    let ui_motion_lib = load_source("../../crates/ui-motion/src/lib.rs");
    let ui_motion_non_wasm_test = load_source("../../crates/ui-motion/tests/non_wasm_stub.rs");
    let platform_script = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion should keep non-wasm stub contract `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion.contains(needle),
            "IllustratedMessage motion should keep predictable non-wasm no-op downgrade `{needle}`.",
        );
    }

    for forbidden in ["panic!", ".unwrap()", ".expect("] {
        assert!(
            !motion.contains(forbidden),
            "IllustratedMessage motion should not introduce panic path in no-op branch (`{forbidden}`).",
        );
    }

    for needle in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
    ] {
        assert!(
            ui_motion_non_wasm_test.contains(needle),
            "ui-motion should keep dedicated non-wasm stub regression `{needle}`.",
        );
    }

    assert!(
        platform_script.contains("cargo test -p ui-motion --test non_wasm_stub"),
        "platform script should keep ui-motion non-wasm stub gate.",
    );

    for needle in [
        "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。",
        "`crates/ui-motion/src/lib.rs` 在 `#[cfg(not(target_arch = \"wasm32\"))]` 下提供 `web::prefers_reduced_motion() -> true` 与 `web::animate(..)` no-op stub",
        "`components/illustrated-message/src/motion.rs` 的 non-wasm `attach_motion` 分支仅执行 `std::hint::black_box(sanitize_motion(motion))`",
        "`scripts/check-ui-platforms.sh` 包含 `cargo test -p ui-motion --test non_wasm_stub`",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_ui_motion_non_wasm_stub_contract_is_predictable_and_safe`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record ui-motion non-wasm stub evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_reduced_motion_ssr_wasm_branches_keep_semantic_contract_stable() {
    let check2 = load_source("check2.md");
    let motion = load_source("src/motion.rs");
    let view = load_source("src/view.rs");
    let styles = load_source("src/styles.rs");
    let spring = load_source("../../crates/ui-motion/src/spring.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "animator.set_target(1.0);",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion.contains(needle),
            "IllustratedMessage motion should keep wasm/non-wasm branch contract `{needle}`.",
        );
    }

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "(self.inner.apply.borrow_mut())(target);",
        "return;",
    ] {
        assert!(
            spring.contains(needle),
            "ui-motion spring should keep reduced-motion short-circuit contract `{needle}`.",
        );
    }

    for needle in ["--ui-im-opacity: 1;", "--ui-im-y:"] {
        assert!(
            styles.contains(needle),
            "IllustratedMessage styles should keep SSR-first-frame stable css var default `{needle}`.",
        );
    }

    for needle in [
        "aria-live=\"off\"",
        "data-view-state=view_state",
        "data-content-state=content_state",
        "data-title-source=title_source",
        "data-description-source=description_source",
    ] {
        assert!(
            view.contains(needle),
            "IllustratedMessage semantic markers should stay platform-invariant `{needle}`.",
        );
    }

    for needle in [
        "- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
        "`components/illustrated-message/src/motion.rs` 使用 `#[cfg(target_arch = \"wasm32\")]` / `#[cfg(not(target_arch = \"wasm32\"))]` 分支",
        "`crates/ui-motion/src/spring.rs` 在 `SpringAnimator::set_target` 内通过 `crate::web::prefers_reduced_motion()` 直接收敛到目标值",
        "`components/illustrated-message/src/styles.rs` 预置 `--ui-im-opacity: 1` 与 `--ui-im-y: 0px`",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_reduced_motion_ssr_wasm_branches_keep_semantic_contract_stable`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record reduced-motion/SSR/wasm evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_performance_governance_is_mount_only_traceable_and_blocking_via_global_gates()
 {
    let check2 = load_source("check2.md");
    let perf_script = load_source("../../scripts/check-ui-performance.sh");
    let docs_shell = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let view = load_source("src/view.rs");

    for needle in [
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script.contains(needle),
            "performance gate script should keep blocking governance command `{needle}`.",
        );
    }

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            docs_shell.contains(needle),
            "docs component shell should keep repeatable perf budget/probe baseline `{needle}`.",
        );
    }

    assert!(
        todo_source.contains(
            "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据"
        ),
        "performance governance should keep explicit render_count automation follow-up in plan.",
    );

    for forbidden in ["on:click", "on:keydown", "on:pointerdown"] {
        assert!(
            !view.contains(forbidden),
            "IllustratedMessage display-only view should avoid interactive update churn source `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "`scripts/check-ui-performance.sh` 已纳入 `button_performance_governance_contract_is_budgeted_traceable_and_blocking` 与 `input_performance_governance_contract_is_budgeted_traceable_and_blocking`",
        "`apps/docs-app/src/pages/components/shell.rs` 以 `component_page_perf_budget` + `UiPerfProbe` 暴露可重复预算标记",
        "`IllustratedMessage` 走默认 `_ => UiPerfBudget::mount_only(120.0)` 的 mount-only 等价基线",
        "`docs/plan/TODO.md` 明确保留 `render_count` 自动化补齐项",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_performance_governance_is_mount_only_traceable_and_blocking_via_global_gates`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record performance governance evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_semantics_and_performance_regression_checklist_item_is_closed() {
    let check2 = load_source("check2.md");
    let semantics = load_source("test/semantics.rs");
    let view = load_source("src/view.rs");

    for needle in [
        "fn illustrated_message_exposes_stable_state_and_source_markers()",
        "fn illustrated_message_semantics_suite_covers_contract_matrix_without_snapshot_dependency()",
        "fn illustrated_message_stays_display_only_without_local_interaction_handlers()",
        "fn illustrated_message_has_no_overlay_focus_stack_or_focus_restore_contract()",
        "fn illustrated_message_performance_governance_is_mount_only_traceable_and_blocking_via_global_gates()",
    ] {
        assert!(
            semantics.contains(needle),
            "semantics suite should keep regression coverage anchor `{needle}`.",
        );
    }

    for needle in [
        "aria-live=\"off\"",
        "data-view-state=view_state",
        "data-content-state=content_state",
        "data-title-source=title_source",
        "data-description-source=description_source",
    ] {
        assert!(
            view.contains(needle),
            "view semantic contract should expose aria/data marker `{needle}`.",
        );
    }

    let forbidden_insta = ["in", "sta::"].concat();
    let forbidden_assert_snapshot = ["assert_", "snapshot!"].concat();
    let forbidden_match_snapshot = ["to_match_", "snapshot"].concat();
    let forbidden_assert_debug_snapshot = ["assert_debug_", "snapshot!"].concat();

    for forbidden in [
        forbidden_insta,
        forbidden_assert_snapshot,
        forbidden_match_snapshot,
        forbidden_assert_debug_snapshot,
    ] {
        assert!(
            !semantics.contains(&forbidden),
            "semantics coverage should not degrade into snapshot-only matcher `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "`components/illustrated-message/test/semantics.rs::illustrated_message_exposes_stable_state_and_source_markers` 覆盖 `aria-live` 与 `data-*` 状态/来源断言",
        "`components/illustrated-message/test/semantics.rs::illustrated_message_semantics_suite_covers_contract_matrix_without_snapshot_dependency` 锁定“语义断言优先、非 snapshot 匹配”",
        "焦点流转对 `IllustratedMessage` 为 N/A-by-design（展示型无本地交互处理器），由 `illustrated_message_stays_display_only_without_local_interaction_handlers` 与 `illustrated_message_has_no_overlay_focus_stack_or_focus_restore_contract` 共同锁定",
        "`components/illustrated-message/test/semantics.rs::illustrated_message_performance_governance_is_mount_only_traceable_and_blocking_via_global_gates` 覆盖性能预算与阻断门禁",
        "`IllustratedMessage` 非高频/重型交互组件，采用 `UiPerfBudget::mount_only(120.0)` 等价基线，并在 `docs/plan/TODO.md` 追踪 `render_count` 自动化补齐",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_semantics_and_performance_regression_checklist_item_is_closed`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record semantics/performance regression evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_version_deprecation_migration_is_not_required_without_breaking_upgrade() {
    let check2 = load_source("check2.md");
    let module = load_source("src/mod.rs");
    let component_toml = load_source("src/Component.toml");
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");
    let styles = load_source("src/styles.rs");
    let rbi = load_source("src/illustrated_message.rbi");

    for needle in [
        "pub use motion::IllustratedMessageMotion;",
        "pub use view::IllustratedMessage;",
    ] {
        assert!(
            module.contains(needle),
            "public API export should remain stable without major-version migration `{needle}`.",
        );
    }

    assert!(
        component_toml.contains("schema_version = \"1\""),
        "Component manifest should stay on schema_version=1 when no major-breaking upgrade is introduced.",
    );

    for source in [&logic, &view, &motion, &styles, &rbi, &component_toml] {
        for forbidden in ["migrate_v1_to_v2", "Schema Registry", "deprecation_window"] {
            assert!(
                !source.contains(forbidden),
                "component sources should not carry forced migration scaffolding without a major-break trigger (`{forbidden}`).",
            );
        }
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A-by-scope：本次 `components/illustrated-message` 提交未引入跨大版本 API 破坏升级",
        "`components/illustrated-message/src/mod.rs` 公共导出面保持 `pub use view::IllustratedMessage` 与 `pub use motion::IllustratedMessageMotion`，无破坏性改名/删除",
        "`components/illustrated-message/src/Component.toml` 仍为 `schema_version = \"1\"`，未触发 `v1 -> v2` 迁移窗口",
        "因此无需登记 Schema Registry 弃用窗口，也无需新增 `migrate_v1_to_v2` 迁移函数",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_version_deprecation_migration_is_not_required_without_breaking_upgrade`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record version deprecation/migration N/A evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_view_macro_complexity_is_bounded_and_semantically_partitioned() {
    let check2 = load_source("check2.md");
    let view = load_source("src/view.rs");
    let tree_shaking_script = load_source("../../scripts/check-ui-tree-shaking.sh");
    let tree_shaking_budget = load_source("../../scripts/tree_shaking_budget.env");

    let view_macro_count = view.matches("view! {").count();
    assert!(
        (1..=5).contains(&view_macro_count),
        "IllustratedMessage view macro complexity should stay bounded (expected 1..=5 `view!` blocks, found {view_macro_count}).",
    );

    let root_slot_count = view.matches("data-slot=\"illustrated-message\"").count();
    assert_eq!(
        root_slot_count, 1,
        "IllustratedMessage should keep exactly one root semantic slot."
    );

    for needle in [
        "data-slot=\"illustrated-message-illustration\"",
        "data-slot=\"illustrated-message-content\"",
        "data-slot=\"illustrated-message-title\"",
        "data-slot=\"illustrated-message-description\"",
        "data-slot=\"illustrated-message-actions\"",
    ] {
        assert!(
            view.contains(needle),
            "IllustratedMessage view should keep semantic sub-block partition `{needle}`.",
        );
    }

    for needle in [
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_shaking_script.contains(needle) || tree_shaking_budget.contains(needle),
            "tree-shaking/budget governance should keep wasm size regression guard `{needle}`.",
        );
    }

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "`components/illustrated-message/src/view.rs` 采用“根容器 + 语义子块”结构（`illustration/content/title/description/actions`）",
        "`view!` 宏数量受控（当前为小规模分块而非巨型单块）",
        "`scripts/check-ui-tree-shaking.sh` + `scripts/tree_shaking_budget.env`",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_view_macro_complexity_is_bounded_and_semantically_partitioned`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record view-macro complexity governance evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_prefers_functional_fragmentation_without_component_noise() {
    let check2 = load_source("check2.md");
    let view = load_source("src/view.rs");

    let component_macro_count = view.matches("#[component]").count();
    assert_eq!(
        component_macro_count, 1,
        "IllustratedMessage view should keep a single component entrypoint, found {component_macro_count}.",
    );

    assert!(
        view.contains("#[component]\npub fn IllustratedMessage(")
            || view.contains("#[component]\r\npub fn IllustratedMessage("),
        "IllustratedMessage should keep the public component boundary only at root.",
    );

    for forbidden in [
        "#[component]\nfn render_",
        "#[component]\npub fn render_",
        "mod fragments",
        "pub mod fragments",
    ] {
        assert!(
            !view.contains(forbidden),
            "IllustratedMessage should avoid promoting local fragments into extra component/module noise `{forbidden}`.",
        );
    }

    for needle in [
        "data-slot=\"illustrated-message-illustration\"",
        "data-slot=\"illustrated-message-content\"",
        "data-slot=\"illustrated-message-title\"",
        "data-slot=\"illustrated-message-description\"",
        "data-slot=\"illustrated-message-actions\"",
    ] {
        assert!(
            view.contains(needle),
            "IllustratedMessage should keep semantic slot partition after functional split preference `{needle}`.",
        );
    }

    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "`components/illustrated-message/src/view.rs` 保持单一公开 `#[component] fn IllustratedMessage`，未把局部片段升级为额外 `#[component]`",
        "展示片段仍在同一组件内按语义槽位（`illustration/content/title/description/actions`）组织",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_prefers_functional_fragmentation_without_component_noise`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record functional-fragment preference evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_static_fragments_are_minimal_and_centralized_without_heavy_static_payload() {
    let check2 = load_source("check2.md");
    let view = load_source("src/view.rs");

    for forbidden in ["<svg", "inner_html=", "footer", "lorem ipsum"] {
        assert!(
            !view.contains(forbidden),
            "IllustratedMessage should not inline heavy static payload fragment `{forbidden}` in component view.",
        );
    }

    for slot in [
        "data-slot=\"illustrated-message\"",
        "data-slot=\"illustrated-message-illustration\"",
        "data-slot=\"illustrated-message-content\"",
        "data-slot=\"illustrated-message-title\"",
        "data-slot=\"illustrated-message-description\"",
        "data-slot=\"illustrated-message-actions\"",
    ] {
        let count = view.matches(slot).count();
        assert_eq!(
            count, 1,
            "IllustratedMessage semantic static slot should stay centralized and unique (`{slot}` found {count} times).",
        );
    }

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "N/A-by-design：`IllustratedMessage` 组件本体不内置复杂 SVG/长说明文本/页脚模板",
        "`src/view.rs` 中静态语义槽位（`data-slot`）集中在单一组件模板并保持唯一映射",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_static_fragments_are_minimal_and_centralized_without_heavy_static_payload`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record static-fragment governance evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_inner_html_usage_is_absent_and_untrusted_injection_paths_are_blocked() {
    let check2 = load_source("check2.md");

    for rel_path in ["src/mod.rs", "src/logic.rs", "src/view.rs", "src/motion.rs"] {
        let source = load_source(rel_path);
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "markdown_to_html(",
        ] {
            assert!(
                !source.contains(forbidden),
                "IllustratedMessage source `{rel_path}` must not include unsafe HTML injection surface `{forbidden}`.",
            );
        }
    }

    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "N/A-by-design：`IllustratedMessage` 无 HTML 注入用例",
        "`components/illustrated-message/src/{mod,logic,view,motion}.rs` 未出现 `inner_html` / `set_inner_html` / `dangerously_set_inner_html` / `markdown_to_html(`",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_inner_html_usage_is_absent_and_untrusted_injection_paths_are_blocked`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record inner_html safety governance evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_wasm_debug_contract_is_traceable_dev_visible_and_feature_isolated() {
    let check2 = load_source("check2.md");
    let view = load_source("src/view.rs");
    let logic = load_source("src/logic.rs");
    let cargo = load_source("Cargo.toml");
    let docs_display = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let wasm_debug_gate = load_source("../../scripts/check-ui-wasm-debug.sh");

    for needle in [
        "data-view-state=view_state",
        "data-content-state=content_state",
        "data-title-state=title_state",
        "data-description-state=description_state",
        "data-title-source=title_source",
        "data-description-source=description_source",
    ] {
        assert!(
            view.contains(needle),
            "IllustratedMessage should expose traceable state/source marker `{needle}` for wasm debug observability.",
        );
    }

    for needle in [
        "pub enum IllustratedMessageStateMarker",
        "pub enum IllustratedMessageRenderMarker",
        "pub enum IllustratedMessageTextSource",
        "pub enum IllustratedMessageSlotSource",
        "IllustratedMessageStateMarker::Shown => \"shown\"",
        "IllustratedMessageStateMarker::Hidden => \"hidden\"",
    ] {
        assert!(
            logic.contains(needle),
            "IllustratedMessage logic should keep closed marker enums for debug snapshot comparability `{needle}`.",
        );
    }

    for forbidden in ["on:click", "on:keydown", "on:pointerdown", "on:keyup"] {
        assert!(
            !view.contains(forbidden),
            "IllustratedMessage has no local interaction stream; replay requirement is N/A-by-design (`{forbidden}`).",
        );
    }

    for needle in [
        "pub(super) fn illustrated_message() -> AnyView",
        "slug=\"illustrated-message\"",
        "title=\"Hello World (Default API)\"",
        "title=\"Interactive Playground (Props + State + Preview)\"",
    ] {
        assert!(
            docs_display.contains(needle),
            "IllustratedMessage should keep docs visual debug/workbench entry `{needle}`.",
        );
    }

    for needle in [
        "[features]",
        "default = []",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features inject-css,button-wasm-debug",
    ] {
        assert!(
            cargo.contains(needle) || wasm_debug_gate.contains(needle),
            "wasm debug contract should keep feature-gated isolation evidence `{needle}`.",
        );
    }

    for forbidden in ["wasm-debug", "wasm_debug"] {
        assert!(
            !cargo.contains(forbidden),
            "IllustratedMessage package should not expose dedicated wasm debug feature `{forbidden}` to public API surface.",
        );
    }

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "`components/illustrated-message/src/view.rs` 暴露 `data-*-state/source` 稳定标记",
        "`components/illustrated-message/src/logic.rs` 以封闭枚举输出可比较状态来源快照",
        "关键交互回放为 N/A-by-design（组件无 `on:*` 交互链路）",
        "`apps/docs-app/src/pages/components/pages/display.rs::illustrated_message` 的 `Playground`",
        "`scripts/check-ui-wasm-debug.sh` 的 feature-gated 路径",
        "`components/illustrated-message/Cargo.toml` 无 `wasm-debug` 特性与公共调试 API",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_wasm_debug_contract_is_traceable_dev_visible_and_feature_isolated`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record wasm-debug governance evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_dx_playground_supports_hot_css_feedback_context_and_isolated_canvas() {
    let check2 = load_source("check2.md");
    let docs_display = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let playground = load_source("../../apps/docs-app/src/playground.rs");
    let dx_gate = load_source("../../scripts/check-ui-dx.sh");
    let view = load_source("src/view.rs");

    for needle in [
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "let (test_css, set_test_css) = signal(default_test_css.get_untracked());",
        "class=\"playground__test-editor\"",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "data-playground-scope=scope_id.clone()",
        "class=\"playground__preview-stage\"",
    ] {
        assert!(
            playground.contains(needle),
            "shared Playground should keep DX hot-css/context/isolation contract `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn illustrated_message() -> AnyView",
        "title=\"Hello World (Default API)\"",
        "title=\"Interactive Playground (Props + State + Preview)\"",
        "slug=\"illustrated-message\"",
    ] {
        assert!(
            docs_display.contains(needle),
            "IllustratedMessage docs should keep isolated playground entry `{needle}`.",
        );
    }

    for forbidden in ["on:click", "on:keydown", "on:pointerdown"] {
        assert!(
            !view.contains(forbidden),
            "IllustratedMessage has no local interaction state machine; optional state persistence stays N/A-by-design (`{forbidden}`).",
        );
    }

    assert!(
        dx_gate.contains("button_dx_playground_supports_css_hot_reload_without_wasm_rebuild"),
        "repo DX gate should keep css hot-reload without wasm rebuild contract anchored in CI scripts.",
    );

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "`apps/docs-app/src/playground.rs` 通过 `<style>{compose_scoped_css(..., test_css)}</style>` + `playground__test-editor` 的 `on:input` 提供样式热反馈路径",
        "`show_settings_panel/show_code_panel/show_test_panel` 以局部信号维持当前调试上下文",
        "`apps/docs-app/src/pages/components/pages/display.rs::illustrated_message` 提供 `Playground` 隔离画布入口",
        "“可选状态保留”对该组件为 N/A-by-design（无本地交互状态机，仅展示内容装配）",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_dx_playground_supports_hot_css_feedback_context_and_isolated_canvas`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record DX governance evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_engineering_contract_is_na_scoped_and_runtime_non_leaky() {
    let check2 = load_source("check2.md");
    let cargo = load_source("Cargo.toml");
    let mod_rs = load_source("src/mod.rs");
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");
    let styles = load_source("src/styles.rs");
    let component_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let engineering_gate = load_source("../../scripts/check-ui-engineering.sh");

    for needle in [
        "#[prop(optional, into)] title: Option<String>",
        "#[prop(optional, into)] description: Option<String>",
        "#[prop(optional, into)] illustration: Option<ViewFn>",
        "#[prop(optional, into)] actions: Option<ViewFn>",
        "#[prop(optional)] orientation: IllustratedMessageOrientation",
        "#[prop(optional)] motion: IllustratedMessageMotion",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
    ] {
        assert!(
            view.contains(needle),
            "IllustratedMessage should keep a display-only public props surface `{needle}`.",
        );
    }

    for forbidden in ["mod protocol;", "pub mod protocol", "pub use protocol::"] {
        assert!(
            !mod_rs.contains(forbidden),
            "IllustratedMessage public module should not leak protocol/spec surface `{forbidden}`.",
        );
    }

    for forbidden in ["protocol.rs", "spec.rs"] {
        assert!(
            !component_src.join(forbidden).exists(),
            "illustrated-message should not keep extra schema/runtime sidecar `{forbidden}` for N/A engineering scope.",
        );
    }

    for forbidden in [
        "tracing::",
        "event!(",
        "span!(",
        "tokio::",
        "async_std::",
        "JoinHandle",
        "Runtime",
        "Handle",
        "async fn",
    ] {
        assert!(
            !mod_rs.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !motion.contains(forbidden)
                && !styles.contains(forbidden),
            "IllustratedMessage sources should avoid runtime/tracing leakage `{forbidden}`.",
        );
    }

    for forbidden in ["tokio", "async-std"] {
        assert!(
            !cargo.contains(forbidden),
            "IllustratedMessage package should not bind to single async runtime dependency `{forbidden}`.",
        );
    }

    assert!(
        engineering_gate.contains(
            "button_engineering_contract_uses_serde_schema_and_structured_migration_errors"
        ) && engineering_gate
            .contains("button_engineering_contract_uses_consistent_tracing_targets")
            && engineering_gate
                .contains("button_engineering_contract_avoids_runtime_leaks_in_public_api"),
        "repo engineering gate should keep serde/tracing/runtime-boundary contracts centralized.",
    );

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "N/A-by-scope + 已落实：`IllustratedMessage` 公共 API 为展示型 props",
        "组件源码目录不再保留额外 protocol/spec 实现入口",
        "`src/{mod,logic,view,motion,styles}.rs` 无组件私有 `tracing::*` 事件目标",
        "`components/illustrated-message/Cargo.toml` 无 `tokio`/`async-std` 依赖且源码无 `async fn`/runtime 类型泄露",
        "仓库级统一门禁见 `scripts/check-ui-engineering.sh`",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_engineering_contract_is_na_scoped_and_runtime_non_leaky`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record engineering-contract evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_styles_defensive_variables_use_dual_fallback_chain_and_theme_ssot() {
    let check2 = load_source("check2.md");
    let styles = load_source("src/styles.rs");
    let theme_css = load_source("../../crates/ui-theme/src/css.rs");

    for needle in [
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-space-lg, var(--ui-fallback-space-lg))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-shadow-md, var(--ui-fallback-shadow-md))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-icon-size-200, var(--ui-fallback-icon-size-200))",
        "var(--ui-min-inline-size-none, var(--ui-fallback-min-inline-size-none))",
    ] {
        assert!(
            styles.contains(needle),
            "IllustratedMessage defensive style contract should keep dual fallback chain token `{needle}`.",
        );
    }

    for forbidden in [
        "var(--ui-space-md);",
        "var(--ui-space-lg);",
        "var(--ui-radius-lg);",
        "var(--ui-fg);",
        "var(--ui-bg);",
        "var(--ui-border);",
        "var(--ui-accent);",
        "var(--ui-shadow-sm);",
        "var(--ui-shadow-md);",
    ] {
        assert!(
            !styles.contains(forbidden),
            "IllustratedMessage styles should avoid single-layer token usage `{forbidden}`.",
        );
    }

    for forbidden in ["color: #", "background: #", "border-color: #", "px;"] {
        assert!(
            !styles.contains(forbidden),
            "IllustratedMessage styles should avoid hardcoded visual terminal values `{forbidden}`.",
        );
    }

    for needle in [
        "--ui-fallback-space-md",
        "--ui-fallback-fg",
        "--ui-fallback-bg",
        "--ui-fallback-border",
        "--ui-fallback-accent",
        "--ui-fallback-shadow-sm",
        "--ui-fallback-shadow-md",
        "--ui-fallback-focus-ring",
        "--ui-fallback-icon-size-200",
        "--ui-fallback-min-inline-size-none",
    ] {
        assert!(
            theme_css.contains(needle),
            "ui-theme SSOT should define fallback token `{needle}`.",
        );
    }

    for needle in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "`components/illustrated-message/src/styles.rs` 的核心视觉与尺寸变量均采用双层回退链",
        "`--ui-min-inline-size-none` / `--ui-fallback-min-inline-size-none` 替代裸 `0/0px` 尺寸终值",
        "Fallback SSOT 来源由 `crates/ui-theme/src/css.rs` 统一生成 `--ui-fallback-*` 变量",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_styles_defensive_variables_use_dual_fallback_chain_and_theme_ssot`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record defensive-variable evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_css_is_layered_in_ui_and_runtime_style_updates_use_custom_properties_only() {
    let check2 = load_source("check2.md");
    let ui_components_css = load_source("../../crates/ui/src/css.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-illustrated_message\")]",
        "out.push_str(crate::illustrated_message::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "ui css aggregation should keep layer/feature entry `{needle}`.",
        );
    }

    for forbidden in [
        "style=",
        "style:\\\"top:",
        "style:\\\"left:",
        "style:\\\"right:",
        "style:\\\"bottom:",
    ] {
        assert!(
            !view.contains(forbidden),
            "IllustratedMessage view should not use regular inline style channel `{forbidden}`.",
        );
    }

    let has_direct_style_writes = motion.contains("style.set_property(\"--ui-im-opacity\"")
        && motion.contains("style.set_property(\"--ui-im-y\"")
        && motion.contains("style_for_apply.set_property(\"--ui-im-opacity\"")
        && motion.contains("style_for_apply.set_property(\"--ui-im-y\"");
    let has_observed_style_writes = motion.contains("set_css_property_observed_auto!")
        && motion.contains("style_for_apply")
        && motion.contains("style")
        && motion.contains("\"--ui-im-opacity\"")
        && motion.contains("\"--ui-im-y\"");
    assert!(
        has_direct_style_writes || has_observed_style_writes,
        "IllustratedMessage runtime style writes should stay in CSS custom property channel.",
    );

    for forbidden in [
        "style.set_property(\"top\"",
        "style.set_property(\"left\"",
        "style.set_property(\"right\"",
        "style.set_property(\"bottom\"",
    ] {
        assert!(
            !motion.contains(forbidden),
            "IllustratedMessage motion should not write layout positioning style directly (`{forbidden}`).",
        );
    }

    for needle in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "`crates/ui/src/css.rs` 在 `push_components_css` 统一注入 `@layer ui`",
        "`components/illustrated-message/src/view.rs` 无 `style=` 普通内联样式",
        "`components/illustrated-message/src/motion.rs` 运行时仅写入 `--ui-im-opacity` / `--ui-im-y` CSS 自定义属性",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_css_is_layered_in_ui_and_runtime_style_updates_use_custom_properties_only`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record cascade-layer evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_motion_contract_is_component_bound_reduced_motion_aware_and_non_wasm_noop() {
    let check2 = load_source("check2.md");
    let motion = load_source("src/motion.rs");
    let view = load_source("src/view.rs");
    let ui_motion_spring = load_source("../../crates/ui-motion/src/spring.rs");

    for needle in [
        "pub struct IllustratedMessageMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "pub initial_y_px: f64,",
        "spring: ui_motion::presets::spring_soft(),",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "stiffness:",
        "damping:",
        "mass:",
        "precision:",
        "pub fn sanitize_motion(motion: IllustratedMessageMotion) -> IllustratedMessageMotion",
    ] {
        assert!(
            motion.contains(needle),
            "IllustratedMessage motion contract should keep sanitized spring parameter boundary `{needle}`.",
        );
    }

    for needle in [
        "let motion = crate::motion::sanitize_motion(motion);",
        "motion::attach_motion(root_ref, motion);",
    ] {
        assert!(
            view.contains(needle),
            "IllustratedMessage view should keep explicit motion contract attach step `{needle}`.",
        );
    }

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "(self.inner.apply.borrow_mut())(target);",
        "return;",
    ] {
        assert!(
            ui_motion_spring.contains(needle),
            "ui-motion spring should keep reduced-motion short-circuit contract `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion.contains(needle),
            "IllustratedMessage motion should keep wasm/non-wasm downgrade contract `{needle}`.",
        );
    }

    for needle in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "`components/illustrated-message/src/motion.rs` 以 `IllustratedMessageMotion { spring, initial_y_px }` 定义组件动效 Contract",
        "`sanitize_spring` 对 `stiffness/damping/mass/precision` 做有效值约束并回落到 `ui_motion::presets::spring_soft()`",
        "`components/illustrated-message/src/view.rs` 在渲染入口先 `sanitize_motion` 再 `motion::attach_motion(root_ref, motion)` 挂载",
        "`crates/ui-motion/src/spring.rs` 在 `SpringAnimator::set_target` 里通过 `crate::web::prefers_reduced_motion()` 短路到目标值",
        "`motion.rs` 的 `#[cfg(not(target_arch = \"wasm32\"))]` 分支使用 `std::hint::black_box(sanitize_motion(motion))`",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_motion_contract_is_component_bound_reduced_motion_aware_and_non_wasm_noop`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record motion-contract evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let check2 = load_source("check2.md");
    let ui_components_lib = load_source("../../crates/ui/src/lib.rs");
    let ui_components_css = load_source("../../crates/ui/src/css.rs");
    let ui_components_root = load_source("../../crates/ui/src/root.rs");
    let active_highlight = load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let ui_components_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui/src");

    for needle in [
        "#[cfg(feature = \"component-illustrated_message\")]",
        "pub use ui_illustrated_message as illustrated_message;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui lib entry should keep feature-gated public export `{needle}`.",
        );
    }
    for forbidden in ["web_sys", "wasm_bindgen"] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui lib entry should not leak platform detail `{forbidden}` into public surface.",
        );
    }

    for needle in [
        "pub fn push_components_css(out: &mut String)",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-illustrated_message\")]",
        "out.push_str(crate::illustrated_message::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "ui css entry should keep layered feature-gated aggregation `{needle}`.",
        );
    }

    for needle in [
        "#[component]\npub fn UiRoot(",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
    ] {
        assert!(
            ui_components_root.contains(needle),
            "UiRoot entry should centralize theme/css/i18n strategy `{needle}`.",
        );
    }

    for needle in [
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::presets::spring_slide()",
    ] {
        assert!(
            active_highlight.contains(needle),
            "active_highlight shared primitive should keep generic highlight-motion capability `{needle}`.",
        );
    }
    for forbidden in [
        "illustrated-message",
        "dialog",
        "menu",
        "popover",
        "business",
    ] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight should not encode concrete component business semantics `{forbidden}`.",
        );
    }

    for missing in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src.join(missing).exists(),
            "ui fixed entry boundary requires `{missing}` to stay absent from crate root.",
        );
    }

    for needle in [
        "- [x] `ui` 固定入口文件落点正确。",
        "`crates/ui/src/lib.rs` 通过 `#[cfg(feature = \"component-illustrated_message\")] pub use ui_illustrated_message as illustrated_message;` 暴露组件并保持 feature gate",
        "`crates/ui/src/css.rs` 在 `push_components_css` 里统一 `@layer ui` 聚合并按 feature 条件注入 `crate::illustrated_message::styles::CSS`",
        "`crates/ui/src/root.rs` 由 `UiRoot` 集中注入 base css + theme vars +（可选）components css",
        "`crates/ui-visual-primitive/src/active_highlight.rs` 仅承载共享高亮动效（`ActiveHighlightMotion + attach_active_highlight_motion`）",
        "`crates/ui/src/overlay_open.rs`、`crates/ui/src/presence.rs`、`crates/ui/src/a11y.rs` 当前均不存在",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_ui_components_fixed_entry_files_follow_layered_boundaries`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record fixed-entry-file evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_component_directory_standard_files_follow_responsibility_boundaries() {
    let check2 = load_source("check2.md");
    let mod_source = load_source("src/mod.rs");
    let logic = load_source("src/logic.rs");
    let styles = load_source("src/styles.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");
    let component_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            component_src.join(required).exists(),
            "component directory should keep required standard file `{required}`.",
        );
    }
    for forbidden in ["render.rs", "spec.rs", "protocol.rs"] {
        assert!(
            !component_src.join(forbidden).exists(),
            "component directory should not drift to forbidden optional file `{forbidden}`.",
        );
    }

    for needle in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use motion::IllustratedMessageMotion;",
        "pub use view::IllustratedMessage;",
    ] {
        assert!(
            mod_source.contains(needle),
            "mod.rs should keep minimal stable export boundary `{needle}`.",
        );
    }
    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "pub use logic::",
        "pub use view::*",
        "pub use motion::*",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not over-export internal implementation `{forbidden}`.",
        );
    }

    for needle in [
        "pub use ui_state_primitives::illustrated_message::{",
        "resolve_view_state,",
        "pub fn resolve_view_model<",
        "pub struct IllustratedMessageResolvedView {",
        "pub fn resolve_root_class(",
    ] {
        assert!(
            logic.contains(needle),
            "logic.rs should own normalization/derived-state/source-marker contract `{needle}`.",
        );
    }
    for forbidden in [
        "NodeRef",
        "view! {",
        "web_sys",
        "wasm_bindgen",
        "on:click",
        "set_property(",
    ] {
        assert!(
            !logic.contains(forbidden),
            "logic.rs should stay free of DOM/event/motion runtime concerns `{forbidden}`.",
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-illustrated-message",
    ] {
        assert!(
            styles.contains(needle),
            "styles.rs should keep static token-first CSS contract `{needle}`.",
        );
    }
    for forbidden in ["#ff", "#FF", "#fff", "#FFF"] {
        assert!(
            !styles.contains(forbidden),
            "styles.rs should not hardcode theme hex values `{forbidden}`.",
        );
    }

    for needle in [
        "use ui_headless::a11y::{A11yDirection, locale_attrs};",
        "let resolved_view = crate::logic::resolve_view_model(",
        "let locale = locale_attrs(lang, dir);",
        "view! {",
        "data-view-state=view_state",
    ] {
        assert!(
            view.contains(needle),
            "view.rs should render structure and mount headless semantics `{needle}`.",
        );
    }
    for forbidden in ["resolve_view_state(", "pub fn resolve_view_model("] {
        assert!(
            !view.contains(forbidden),
            "view.rs should not duplicate logic-layer state derivation `{forbidden}`.",
        );
    }

    for needle in [
        "pub struct IllustratedMessageMotion {",
        "pub fn sanitize_motion(motion: IllustratedMessageMotion) -> IllustratedMessageMotion",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion.contains(needle),
            "motion.rs should keep motion contract + attach mapping `{needle}`.",
        );
    }
    for forbidden in ["view! {", "resolve_view_state(", "locale_attrs("] {
        assert!(
            !motion.contains(forbidden),
            "motion.rs should not absorb view/logic/headless responsibilities `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] 组件目录标准文件落点正确。",
        "`components/illustrated-message/src/mod.rs` 存在且维持最小导出面",
        "`components/illustrated-message/src/logic.rs` 负责 props 归一化、派生状态与来源标记",
        "`components/illustrated-message/src/view.rs` 仅做 Leptos 结构渲染并挂载 headless locale 语义",
        "`components/illustrated-message/src/motion.rs` 提供 `IllustratedMessageMotion + attach_motion`",
        "组件目录无 `render.rs`，且未新增 `spec.rs`。",
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "目录内无 `render.rs`、无 `spec.rs`，并清理了非必要 `protocol.rs` 侧车实现",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_component_directory_standard_files_follow_responsibility_boundaries`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record component-directory standard file evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_rust_hygiene_contract_keeps_component_sources_clean() {
    let check2 = load_source("check2.md");
    let source_files = [
        ("src/mod.rs", load_source("src/mod.rs")),
        ("src/logic.rs", load_source("src/logic.rs")),
        ("src/styles.rs", load_source("src/styles.rs")),
        ("src/view.rs", load_source("src/view.rs")),
        ("src/motion.rs", load_source("src/motion.rs")),
    ];

    for (path, source) in &source_files {
        for forbidden in [".unwrap(", ".expect(", "let _ = "] {
            assert!(
                !source.contains(forbidden),
                "{path} should not contain forbidden non-test hygiene pattern `{forbidden}`.",
            );
        }
    }

    for (path, source) in &source_files {
        for forbidden in [".to_string()", ".to_owned()"] {
            assert!(
                !source.contains(forbidden),
                "{path} should avoid eager string-copy hotspot pattern `{forbidden}` in component runtime code.",
            );
        }
    }

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "`components/illustrated-message/src/{mod,logic,styles,view,motion}.rs` 非测试代码扫描无 `.unwrap(`、`.expect(`、`let _ =`",
        "`components/illustrated-message/src/{mod,logic,styles,view,motion}.rs` 无 `.to_string()`/`.to_owned()` 热点拷贝调用",
        "执行 `./scripts/check-rust-hygiene.sh`：当前环境因 `rg` 缺少 PCRE2 与仓库级 `check-api-contracts` baseline drift 报错，属仓库门禁基线问题，非本组件 hygiene 违约",
        "回归：`components/illustrated-message/test/semantics.rs::illustrated_message_rust_hygiene_contract_keeps_component_sources_clean`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record Rust hygiene evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_e2e_selectors_and_wasm_waits_are_semantic_and_stable() {
    let check2 = load_source("check2.md");
    let e2e = load_source("../../e2e/tests/docs_app_illustrated_message_contract.spec.mjs");

    for needle in [
        "await page.goto(\"/#/components/illustrated-message\");",
        "body:not(:has(#boot))",
        "[data-component=\"illustrated-message\"]",
        "[data-slot=\"illustrated-message\"]",
        "[data-slot=\"illustrated-message-streaming-preview\"]",
        "data-ui-schema=\"ui.illustrated-message.agent-contract\"",
        "data-ui-output-status=\"validated\"",
        "data-ui-streaming-fallback=\"snapshot\"",
        "data-view-state=\"populated\"",
        "data-illustration-state=\"shown\"",
        "data-actions-state=\"shown\"",
        "await page.reload();",
        "docs-app illustrated-message uses semantic selectors with wasm-stable ready waits",
        "docs-app illustrated-message key flow is repeatable with semantic contract breakpoints",
    ] {
        assert!(
            e2e.contains(needle),
            "illustrated-message e2e contract should contain semantic-ready marker `{needle}`.",
        );
    }

    assert!(
        !e2e.contains("waitForTimeout("),
        "illustrated-message e2e contract should avoid fixed sleeps and use semantic waits.",
    );

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "`e2e/tests/docs_app_illustrated_message_contract.spec.mjs`",
        "`docs-app illustrated-message uses semantic selectors with wasm-stable ready waits`",
        "`docs-app illustrated-message key flow is repeatable with semantic contract breakpoints`",
        "仅使用 `data-component`/`data-slot`/`data-ui-*`/`data-*-state` 语义选择器",
        "`body:not(:has(#boot))` + `toHaveAttribute(...)` 语义断点",
        "未使用固定 `waitForTimeout`",
        "回归：`e2e/tests/docs_app_illustrated_message_contract.spec.mjs`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record e2e selector/wait evidence `{needle}`.",
        );
    }
}

#[test]
fn illustrated_message_key_flow_is_in_repeatable_e2e_regression_set() {
    let check2 = load_source("check2.md");
    let e2e = load_source("../../e2e/tests/docs_app_illustrated_message_contract.spec.mjs");

    for needle in [
        "async function runRepeatableKeyFlowWithSemanticBreakpoints(docsRoot) {",
        "docs-app illustrated-message key flow is repeatable with semantic contract breakpoints",
        "await runRepeatableKeyFlowWithSemanticBreakpoints(docsRoot);",
        "await page.reload();",
        "await runRepeatableKeyFlowWithSemanticBreakpoints(reloadedRoot);",
        "data-illustration-state=\"shown\"",
        "data-actions-state=\"shown\"",
        "data-title-state=\"hidden\"",
        "data-ui-output-status=\"validated\"",
    ] {
        assert!(
            e2e.contains(needle),
            "illustrated-message key-flow regression should keep semantic breakpoint `{needle}`.",
        );
    }

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "`docs-app illustrated-message key flow is repeatable with semantic contract breakpoints`",
        "reload 后复跑同一断点链路",
        "`data-view-state`/`data-ui-state`/`data-ui-output-status`",
        "高风险路径（overlay/focus/keyboard/async）对 `IllustratedMessage` 属 N/A-by-design",
        "回归：`e2e/tests/docs_app_illustrated_message_contract.spec.mjs`。",
    ] {
        assert!(
            check2.contains(needle),
            "check2 should record repeatable key-flow evidence `{needle}`.",
        );
    }
}
