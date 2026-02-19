use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn spacer_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/spacer/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Spacer internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn spacer_uses_logic_state_model() {
    let view_source = load_source("src/spacer/view.rs");
    let logic_source = load_source("src/spacer/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/spacer.rs");
    let headless_source = load_source("../ui-headless/src/spacer.rs");

    for needle in [
        "pub enum SpacerAxis",
        "pub enum SpacerSize",
        "pub struct SpacerStateInput",
        "pub struct SpacerState",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Spacer state primitive should include `{needle}` in ui-state-primitives."
        );
    }

    for needle in [
        "pub use ui_state_primitives::spacer::{",
        "pub fn normalize_optional_text(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Spacer logic should include `{needle}` for primitive consumption and assembly mapping."
        );
    }

    for needle in [
        "pub struct SpacerOptions",
        "pub struct SpacerAttrs",
        "pub struct SpacerHandlers",
        "pub struct SpacerSemanticState",
        "pub struct SpacerContract",
        "pub fn use_spacer(",
        "pub lang: Option<String>",
        "pub dir: Option<A11yDirection>",
    ] {
        assert!(
            headless_source.contains(needle),
            "Spacer headless contract should include `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, SpacerOptions, use_spacer};",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(SpacerStateInput {",
        "let spacer = use_spacer(SpacerOptions { state, lang, dir });",
        "let motion = super::motion::sanitize_motion(motion);",
        "let motion_source = super::motion::source_attr(motion);",
        "super::motion::attach_motion(node_ref, motion);",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer view should consume logic + headless contracts; missing `{needle}`."
        );
    }
}

#[test]
fn spacer_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/spacer/view.rs");

    for attr in [
        "role=attrs.role",
        "aria-hidden=attrs.aria_hidden",
        "lang=attrs.lang.clone()",
        "dir=attrs.dir",
        "data-slot=attrs.data_slot",
        "data-axis=attrs.data_axis",
        "data-size=attrs.data_size",
        "data-state=attrs.data_state",
        "data-vertical=attrs.data_vertical",
        "data-horizontal=attrs.data_horizontal",
        "data-custom-class=attrs.data_custom_class",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
    ] {
        assert!(
            source.contains(attr),
            "Spacer should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn spacer_motion_contract_stays_mapping_only() {
    let source = load_source("src/spacer/motion.rs");

    for needle in [
        "pub struct SpacerMotion",
        "pub fn sanitize_motion(",
        "pub fn source_attr(",
        "pub fn attach_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            source.contains(needle),
            "Spacer motion contract should include `{needle}`."
        );
    }

    for forbidden in ["ui_motion::spring", "MotionKeyframe", "SpringAnimator"] {
        assert!(
            !source.contains(forbidden),
            "Spacer motion layer should stay mapping-only and avoid runtime engine code; found `{forbidden}`."
        );
    }
}

#[test]
fn spacer_styles_include_axis_and_size_state_markers() {
    let source = load_source("src/spacer/styles.rs");

    for selector in [
        ".ui-spacer--size-xs",
        ".ui-spacer[data-size=\"md\"]",
        ".ui-spacer--size-xl",
        ".ui-spacer--axis-vertical",
        ".ui-spacer[data-axis=\"horizontal\"]",
        ".ui-spacer[data-state=\"vertical\"]",
        ".ui-spacer[data-horizontal=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Spacer styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn spacer_theme_tokens_are_consumed_via_ui_theme_pipeline() {
    let style_source = load_source("src/spacer/styles.rs");
    let tokens_source = load_source("../ui-theme/src/tokens.rs");
    let theme_source = load_source("../ui-theme/src/theme.rs");
    let css_source = load_source("../ui-theme/src/css.rs");
    let styling_spec_source = load_source("../../docs/spec/styling.md");

    for needle in [
        "var(--ui-space-xs)",
        "var(--ui-space-sm)",
        "var(--ui-space-md)",
        "var(--ui-space-lg)",
    ] {
        assert!(
            style_source.contains(needle),
            "Spacer styles should consume `{needle}` from theme tokens."
        );
    }

    for needle in [
        "pub struct SpaceTokens",
        "pub space_3xs_px: u16",
        "pub space_2xs_px: u16",
        "pub xs_px: u16",
        "pub sm_px: u16",
        "pub md_px: u16",
        "pub lg_px: u16",
    ] {
        assert!(
            tokens_source.contains(needle),
            "ui-theme token taxonomy should include `{needle}`."
        );
    }

    for needle in [
        "pub enum ThemeSystem",
        "pub enum ThemeColor",
        "pub enum ThemeScale",
    ] {
        assert!(
            theme_source.contains(needle),
            "theme axis definition should include `{needle}`."
        );
    }

    for needle in [
        "--ui-space-3xs:",
        "--ui-space-2xs:",
        "--ui-space-xs:",
        "--ui-space-sm:",
        "--ui-space-md:",
        "--ui-space-lg:",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-theme css emission should include `{needle}`."
        );
    }

    assert!(
        styling_spec_source.contains("间距基线（`space-3xs/2xs/xs/sm/md/lg`）由 `ui-theme` 统一定义并输出 CSS 变量，组件只消费。"),
        "styling spec should document the shared space token baseline."
    );
}

#[test]
fn spacer_docs_page_covers_primary_playgrounds() {
    let check2_source = load_source("src/spacer/check2.md");
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "回归：`crates/ui-layout/tests/spacer_semantics.rs::spacer_docs_page_covers_primary_playgrounds`",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep documentation-product governance token `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn spacer() -> AnyView",
        "title=\"Spacer\"",
        "slug=\"spacer\"",
        "Playground title=\"Hello World\"",
        "Playground title=\"Axis + Size\"",
        "Playground title=\"Custom Class Marker\"",
    ] {
        assert!(
            source.contains(needle),
            "layout docs page should contain `{needle}` for Spacer.",
        );
    }

    let spacer_start = source
        .find("pub(super) fn spacer() -> AnyView {")
        .expect("Spacer docs entry should exist.");
    let spacer_end = source[spacer_start..]
        .find("pub(super) fn well() -> AnyView {")
        .map(|offset| spacer_start + offset)
        .expect("Spacer docs entry should end before next component section.");
    let spacer_section = &source[spacer_start..spacer_end];

    let hello_idx = spacer_section
        .find("Playground title=\"Hello World\"")
        .expect("Spacer docs should define Hello World playground.");
    let axis_idx = spacer_section
        .find("Playground title=\"Axis + Size\"")
        .expect("Spacer docs should define Axis + Size playground.");
    let custom_idx = spacer_section
        .find("Playground title=\"Custom Class Marker\"")
        .expect("Spacer docs should define Custom Class Marker playground.");

    assert!(
        hello_idx < axis_idx && axis_idx < custom_idx,
        "Spacer docs should present default path first, then advanced examples."
    );

    for forbidden in [
        "ui_state_primitives::",
        "ui_headless::",
        "state=",
        "use_spacer(",
    ] {
        assert!(
            !spacer_section.contains(forbidden),
            "Beginner-facing docs should avoid requiring internal layering wiring token `{forbidden}`."
        );
    }
}

#[test]
fn spacer_docs_interactive_playground_contract_is_stable() {
    let check2_source = load_source("src/spacer/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "回归：`crates/ui-layout/tests/spacer_semantics.rs::spacer_docs_page_covers_primary_playgrounds` + `spacer_docs_interactive_playground_contract_is_stable`",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep interactive-playground governance token `{needle}`."
        );
    }

    let spacer_start = docs_source
        .find("pub(super) fn spacer() -> AnyView {")
        .expect("Spacer docs entry should exist.");
    let spacer_end = docs_source[spacer_start..]
        .find("pub(super) fn well() -> AnyView {")
        .map(|offset| spacer_start + offset)
        .expect("Spacer docs entry should end before next component section.");
    let spacer_section = &docs_source[spacer_start..spacer_end];

    for needle in [
        "let hello_code = Signal::derive(move || r#\"<Spacer />\"#.to_string());",
        "let axis_and_size_code = Signal::derive(move || {",
        "let custom_class_code = Signal::derive(move || {",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Axis + Size\" code_signal=axis_and_size_code>",
        "<Playground title=\"Custom Class Marker\" code_signal=custom_class_code>",
        "axis=SpacerAxis::Vertical",
        "size=SpacerSize::Sm",
        "size=SpacerSize::Lg",
        "axis=SpacerAxis::Horizontal",
        "class_name=\"docs-spacer-guide\".to_string()",
    ] {
        assert!(
            spacer_section.contains(needle),
            "Spacer docs interactive playground should contain `{needle}`."
        );
    }

    assert!(
        pages_source
            .contains("component_doc!(\"Spacer\", \"spacer\", \"Layout\", layout::spacer),"),
        "docs-app catalog should expose Spacer docs entry for discoverable playground access."
    );

    for needle in [
        "test(\"docs-app components pages render playgrounds (sample)\"",
        "test(\"docs-app components pages render playgrounds (all)\"",
        "await page.goto(`/#/components/${slug}`);",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "await expect(page.locator(\"section.playground\").first()).toBeVisible();",
        "await expect(page.locator(`[data-slot=\"${slug}\"]`).first()).toBeVisible();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Coverage E2E should keep repeatable playground verification token `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "Playground verification should avoid fixed-delay wait token `{forbidden}`."
        );
    }
}

#[test]
fn spacer_docs_playgrounds_lock_state_matrix_contract_values() {
    let check2_source = load_source("src/spacer/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let view_source = load_source("src/spacer/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/spacer.rs");

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "回归：`crates/ui-layout/tests/spacer_semantics.rs::spacer_docs_playgrounds_lock_state_matrix_contract_values` + `spacer_docs_are_synced_and_copy_paste_ready`",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep docs-sync governance token `{needle}`."
        );
    }

    for needle in [
        "let hello_code = Signal::derive(move || r#\"<Spacer />\"#.to_string());",
        "title=\"Hello World\"",
        "<Spacer />",
        "title=\"Axis + Size\"",
        "<Spacer axis=SpacerAxis::Vertical size=SpacerSize::Sm />",
        "<Spacer axis=SpacerAxis::Vertical size=SpacerSize::Lg />",
        "<Spacer axis=SpacerAxis::Horizontal size=SpacerSize::Md />",
        "title=\"Custom Class Marker\"",
        "axis=SpacerAxis::Vertical",
        "size=SpacerSize::Md",
        "axis=SpacerAxis::Horizontal",
        "size=SpacerSize::Lg",
        "class_name=\"docs-spacer-guide\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "spacer docs playgrounds should contain `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional)] axis: SpacerAxis",
        "#[prop(optional)] size: SpacerSize",
        "#[prop(optional, into)] class_name: Option<String>",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer docs API examples should stay aligned with current public prop `{needle}`."
        );
    }

    for needle in [
        "#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]",
        "pub enum SpacerAxis",
        "#[default]",
        "Vertical",
        "pub enum SpacerSize",
        "Md",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Spacer default behavior should stay aligned with primitive default token `{needle}`."
        );
    }
}

#[test]
fn spacer_dx_paradox_keeps_hello_world_path_simple() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let view_source = load_source("src/spacer/view.rs");
    let mod_source = load_source("src/spacer/mod.rs");

    assert!(
        docs_source
            .contains("let hello_code = Signal::derive(move || r#\"<Spacer />\"#.to_string());"),
        "Spacer docs should provide a copy-paste ready Hello World snippet."
    );

    for forbidden in [
        "ui_state_primitives::spacer::",
        "ui_headless::use_spacer",
        "state=",
        "on_open_change",
        "default_open",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Spacer Hello World path should not require internal state/headless wiring; found `{forbidden}`."
        );
    }

    for needle in [
        "#[prop(optional)] axis: SpacerAxis",
        "#[prop(optional)] size: SpacerSize",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional)] motion: super::motion::SpacerMotion",
        "#[prop(optional, into)] class_name: Option<String>",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer public API should keep defaults optional and easy to adopt via `{needle}`."
        );
    }

    assert!(
        mod_source.contains("pub use view::Spacer;"),
        "Spacer should expose a direct public component API entry."
    );
}

#[test]
fn spacer_is_not_composite_and_avoids_parallel_slot_api() {
    let view_source = load_source("src/spacer/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for forbidden in [
        "children: Children",
        "items:",
        "labels:",
        "titles:",
        "panels:",
        "ItemSpec",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spacer is a single-node layout primitive and must not regress to composite-slot API; found `{forbidden}`."
        );
    }

    for forbidden in ["labels + children", "titles + panels"] {
        assert!(
            !docs_source.contains(forbidden),
            "Spacer docs should not recommend parallel-array slot conventions; found `{forbidden}`."
        );
    }
}

#[test]
fn spacer_has_a11y_and_i18n_mount_points_without_hardcoded_copy() {
    let view_source = load_source("src/spacer/view.rs");
    let headless_source = load_source("../ui-headless/src/spacer.rs");

    for needle in [
        "use crate::a11y::{A11yDirection, locale_attrs};",
        "pub lang: Option<String>",
        "pub dir: Option<A11yDirection>",
        "role: \"presentation\"",
        "aria_hidden: \"true\"",
        "let locale = locale_attrs(options.lang, options.dir);",
    ] {
        assert!(
            headless_source.contains(needle),
            "Spacer headless layer should include shared A11y/i18n hook `{needle}`."
        );
    }

    for needle in [
        "role=attrs.role",
        "aria-hidden=attrs.aria_hidden",
        "lang=attrs.lang.clone()",
        "dir=attrs.dir",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer view should mount headless A11y attrs via `{needle}`."
        );
    }

    assert!(
        !view_source.contains("\"Top\""),
        "Spacer component view should not hardcode user-visible copy."
    );
}

#[test]
fn spacer_state_markers_are_observable_and_closed_set() {
    let view_source = load_source("src/spacer/view.rs");
    let headless_source = load_source("../ui-headless/src/spacer.rs");
    let primitive_source = load_source("../ui-state-primitives/src/spacer.rs");

    for needle in [
        "data-axis=attrs.data_axis",
        "data-size=attrs.data_size",
        "data-state=attrs.data_state",
        "data-vertical=attrs.data_vertical",
        "data-horizontal=attrs.data_horizontal",
        "data-custom-class=attrs.data_custom_class",
        "data-motion-source=motion_source",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer should expose observable marker `{needle}`."
        );
    }

    for needle in [
        "pub data_axis: &'static str",
        "pub data_size: &'static str",
        "pub data_state: &'static str",
        "pub data_vertical: Option<&'static str>",
        "pub data_horizontal: Option<&'static str>",
        "pub data_custom_class: Option<&'static str>",
    ] {
        assert!(
            headless_source.contains(needle),
            "Headless contract should keep marker types explicit via `{needle}`."
        );
    }

    for needle in [
        "pub fn as_attr(self) -> &'static str",
        "SpacerAxis::Vertical => \"vertical\"",
        "SpacerAxis::Horizontal => \"horizontal\"",
        "SpacerSize::Xs => \"xs\"",
        "SpacerSize::Sm => \"sm\"",
        "SpacerSize::Md => \"md\"",
        "SpacerSize::Lg => \"lg\"",
        "SpacerSize::Xl => \"xl\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Marker values should stay in a closed enum-backed set via `{needle}`."
        );
    }
}

#[test]
fn spacer_styles_depend_on_explicit_state_markers_only() {
    let styles_source = load_source("src/spacer/styles.rs");
    let view_source = load_source("src/spacer/view.rs");

    for needle in [
        ".ui-spacer--size-xs",
        ".ui-spacer[data-size=\"sm\"]",
        ".ui-spacer[data-axis=\"vertical\"]",
        ".ui-spacer[data-state=\"horizontal\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "Spacer styles should be driven by explicit markers via `{needle}`."
        );
    }

    for forbidden in [":nth-child", ":first-child", ":last-child", "> * > *"] {
        assert!(
            !styles_source.contains(forbidden),
            "Spacer styles should avoid fragile DOM-structure selectors; found `{forbidden}`."
        );
    }

    assert!(
        !view_source.contains("style="),
        "Spacer view should not push business style logic through inline style."
    );
}

#[test]
fn spacer_semantics_tests_prioritize_contract_assertions() {
    let check2_source = load_source("src/spacer/check2.md");
    let test_source = load_source("tests/spacer_semantics.rs");
    let view_source = load_source("src/spacer/view.rs");
    let semantics_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/spacer_semantics.rs");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "回归：`crates/ui-layout/tests/spacer_semantics.rs::spacer_semantics_tests_prioritize_contract_assertions`",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep semantics-priority governance token `{needle}`."
        );
    }

    assert!(
        semantics_path.exists(),
        "Spacer should keep dedicated `*_semantics.rs` coverage file."
    );

    for needle in [
        "role=attrs.role",
        "aria-hidden=attrs.aria_hidden",
        "data-axis=attrs.data_axis",
        "data-size=attrs.data_size",
        "data-state=attrs.data_state",
        "data-motion-source=motion_source",
        "fn spacer_state_markers_are_observable_and_closed_set()",
        "fn spacer_has_a11y_and_i18n_mount_points_without_hardcoded_copy()",
        "fn spacer_platform_and_motion_guards_remain_explicit()",
    ] {
        assert!(
            test_source.contains(needle),
            "Spacer semantic test suite should assert contract marker `{needle}`."
        );
    }

    for needle in [
        "role=attrs.role",
        "aria-hidden=attrs.aria_hidden",
        "data-state=attrs.data_state",
        "data-motion-source=motion_source",
    ] {
        assert!(
            view_source.contains(needle),
            "Semantic markers should remain continuously readable in view output via `{needle}`."
        );
    }
}

#[test]
fn spacer_e2e_selectors_and_wasm_wait_strategy_are_semantic_and_stable() {
    let check2_source = load_source("src/spacer/check2.md");
    let view_source = load_source("src/spacer/view.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");

    for needle in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "N/A：`Spacer` 非交互组件，无异步 ready/settled 等待路径",
        "回归：`crates/ui-layout/tests/spacer_semantics.rs::spacer_e2e_selectors_and_wasm_wait_strategy_are_semantic_and_stable`",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep E2E selector/wait governance token `{needle}`."
        );
    }

    for needle in [
        "data-slot=attrs.data_slot",
        "role=attrs.role",
        "aria-hidden=attrs.aria_hidden",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer should expose semantic selector marker `{needle}` for E2E stability."
        );
    }

    for needle in [
        "body:not(:has(#boot))",
        ".waitFor();",
        "page.locator(`[data-slot=\"${slug}\"]`).first()",
    ] {
        assert!(
            e2e_source.contains(needle),
            "docs-app coverage e2e should keep semantic selector/wait token `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "E2E wait strategy should avoid fixed-delay token `{forbidden}`."
        );
    }
}

#[test]
fn spacer_critical_flow_regression_scope_is_explicit_and_traceable() {
    let check2_source = load_source("src/spacer/check2.md");
    let view_source = load_source("src/spacer/view.rs");
    let headless_source = load_source("../ui-headless/src/spacer.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");

    for needle in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "N/A：`Spacer` 无“打开/交互/关闭/提交”关键流程；高风险交互路径不适用。",
        "回归：`crates/ui-layout/tests/spacer_semantics.rs::spacer_critical_flow_regression_scope_is_explicit_and_traceable`",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep critical-flow regression governance token `{needle}`."
        );
    }

    for needle in [
        "test(\"docs-app components pages render playgrounds (sample)\"",
        "test(\"docs-app components pages render playgrounds (all)\"",
        "await page.goto(`/#/components/${slug}`);",
        "await expect(page.locator(`[data-slot=\"${slug}\"]`).first()).toBeVisible();",
        "await expect(perfProbe).not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            e2e_source.contains(needle),
            "Coverage e2e should keep repeatable and traceable contract assertion `{needle}`."
        );
    }

    for forbidden in [
        "overlay",
        "focus",
        "keyboard",
        "on:keydown",
        "on_keydown",
        "async fn",
    ] {
        assert!(
            !view_source.contains(forbidden) && !headless_source.contains(forbidden),
            "Spacer should stay out of high-risk interaction path set; found `{forbidden}`."
        );
    }
}

#[test]
fn spacer_component_file_roles_and_spec_scope_are_correct() {
    let mod_source = load_source("src/spacer/mod.rs");
    let logic_source = load_source("src/spacer/logic.rs");
    let styles_source = load_source("src/spacer/styles.rs");
    let view_source = load_source("src/spacer/view.rs");
    let motion_source = load_source("src/spacer/motion.rs");
    let spec_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spacer/spec.rs");

    for needle in [
        "mod logic;",
        "mod view;",
        "pub mod styles;",
        "pub mod motion;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Spacer module boundary should keep expected file roles via `{needle}`."
        );
    }

    for forbidden in ["view! {", "web_sys", "wasm_bindgen"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs must stay normalization-only and avoid `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "styles.rs should define static token-first CSS contract."
    );
    assert!(
        view_source.contains("view! {"),
        "view.rs should remain the render/mount entry."
    );
    assert!(
        motion_source.contains("pub fn attach_motion("),
        "motion.rs should own motion attach mapping."
    );

    assert!(
        !spec_path.exists(),
        "Spacer should not introduce spec.rs for a simple layout primitive."
    );
}

#[test]
fn spacer_token_first_styles_flow_through_css_aggregator() {
    let styles_source = load_source("src/spacer/styles.rs");
    let css_source = load_source("src/css.rs");
    let view_source = load_source("src/spacer/view.rs");

    for needle in [
        "var(--ui-space-xs)",
        "var(--ui-space-sm)",
        "var(--ui-space-md)",
        "var(--ui-space-lg)",
    ] {
        assert!(
            styles_source.contains(needle),
            "Spacer styles should consume shared ui-theme variables via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-spacer\")]",
        "out.push_str(crate::spacer::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "Spacer CSS should be feature-gated in the central css aggregator via `{needle}`."
        );
    }

    assert!(
        !view_source.contains("style=\""),
        "Spacer view should avoid inline utility-style contracts."
    );
}

#[test]
fn spacer_tree_shaking_feature_chain_is_component_scoped() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    assert!(
        cargo_source.contains("component-spacer = []"),
        "ui-layout feature table should define an independent `component-spacer` gate."
    );

    for needle in [
        "#[cfg(feature = \"component-spacer\")]",
        "pub mod spacer;",
        "pub use spacer::{Spacer, SpacerAxis, SpacerMotion, SpacerSize};",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-layout public surface should keep spacer behind feature gate via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-spacer\")]",
        "out.push_str(crate::spacer::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "CSS aggregation should keep spacer CSS behind feature gate via `{needle}`."
        );
    }
}

#[test]
fn spacer_platform_and_motion_guards_remain_explicit() {
    let motion_source = load_source("src/spacer/motion.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "Spacer motion mapping should keep explicit wasm/non-wasm guards via `{needle}`."
        );
    }

    for forbidden in ["web_sys::", "window()", "document()"] {
        assert!(
            !motion_source.contains(forbidden),
            "Spacer motion mapping should not hard-bind browser objects in shared path; found `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion non-wasm fallback contract should include `{needle}`."
        );
    }

    assert!(
        headless_lib_source.contains(
            "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")"
        ),
        "ui-headless should keep web/ssr mutual-exclusion compile guard."
    );
}

#[test]
fn spacer_performance_governance_contract_is_mount_only_traceable_and_blocking() {
    let check2_source = load_source("src/spacer/check2.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../crates/ui-headless/src/perf.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let view_source = load_source("src/spacer/view.rs");

    for needle in [
        "- [x] 性能治理：关键路径有预算",
        "N/A：`Spacer` 无交互状态机与异步更新链路",
        "render_count",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep performance governance evidence token `{needle}`."
        );
    }

    assert!(
        pages_source.contains("component_doc!(\"Spacer\", \"spacer\", \"Layout\", layout::spacer)"),
        "Spacer docs page should stay in component coverage traversal."
    );

    for needle in [
        "_ => UiPerfBudget::mount_only(120.0)",
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
            "docs coverage e2e should keep blocking perf assertion `{needle}`."
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
        "data-axis=attrs.data_axis",
        "data-size=attrs.data_size",
        "data-state=attrs.data_state",
        "data-motion-source=motion_source",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer should expose state attribution marker `{needle}` for perf triage."
        );
    }
}

#[test]
fn spacer_view_macro_complexity_stays_single_block_and_shallow() {
    let view_source = load_source("src/spacer/view.rs");
    let check2_source = load_source("src/spacer/check2.md");

    assert_eq!(
        view_source.matches("view! {").count(),
        1,
        "Spacer should keep a single small `view!` block."
    );

    assert!(
        view_source.contains("<div"),
        "Spacer view should render a single semantic root node."
    );

    for forbidden in [
        "<header",
        "<section",
        "<article",
        "<ul",
        "<li",
        "for ",
        "collect::<Vec",
        "Show when=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spacer view should avoid deep/repetitive macro expansion token `{forbidden}`."
        );
    }

    for needle in [
        "- [x] `view!` 宏复杂度受控",
        "单个 `view!` 块不得承载超长深嵌套结构",
        "编译时间/产物体积异常增长时，优先排查宏展开体量",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep view-macro governance token `{needle}`."
        );
    }
}

#[test]
fn spacer_functional_split_policy_remains_noise_free_for_simple_component() {
    let view_source = load_source("src/spacer/view.rs");
    let check2_source = load_source("src/spacer/check2.md");

    assert!(
        view_source.contains("pub fn Spacer("),
        "Spacer should keep a single explicit public component entry."
    );
    assert!(
        view_source.contains(") -> impl IntoView {"),
        "Spacer should keep function-style render return (`impl IntoView`)."
    );
    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Spacer should not introduce extra local #[component] noise for simple UI fragments."
    );

    for forbidden in ["#[component]\nfn", "fn render_", "fn sub_component"] {
        assert!(
            !view_source.contains(forbidden),
            "Spacer simple view should avoid unnecessary local component/function abstraction `{forbidden}`."
        );
    }

    for needle in [
        "data-axis=attrs.data_axis",
        "data-size=attrs.data_size",
        "data-state=attrs.data_state",
    ] {
        assert!(
            view_source.contains(needle),
            "Semantic marker `{needle}` should stay stable under functional-split policy."
        );
    }

    for needle in [
        "- [x] 函数式拆分优先",
        "N/A：`Spacer` 仅一个顶层组件且无局部重复片段",
        "拆分后语义标记与测试定位仍需稳定",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep functional-split governance token `{needle}`."
        );
    }
}

#[test]
fn spacer_static_fragment_constantization_contract_stays_not_applicable_and_clean() {
    let view_source = load_source("src/spacer/view.rs");
    let check2_source = load_source("src/spacer/check2.md");

    assert_eq!(
        view_source.matches("<div").count(),
        1,
        "Spacer should keep a single static root fragment and avoid repeated static subtree construction."
    );

    for forbidden in [
        "<svg",
        "<footer",
        "<p",
        "<h1",
        "<h2",
        "inner_html",
        "const STATIC_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spacer should not accumulate heavy/static fragment duplication token `{forbidden}`."
        );
    }

    for needle in ["role=attrs.role", "aria-hidden=attrs.aria_hidden"] {
        assert!(
            view_source.contains(needle),
            "Static fragment baseline should keep a11y semantic token `{needle}`."
        );
    }

    for needle in [
        "- [x] 静态片段常量化",
        "N/A：`Spacer` 仅渲染单一空元素，无复杂静态片段可常量化",
        "常量化后仍需维持可访问语义",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep static-fragment governance token `{needle}`."
        );
    }
}

#[test]
fn spacer_inner_html_contract_is_not_applicable_and_safeguarded() {
    let view_source = load_source("src/spacer/view.rs");
    let logic_source = load_source("src/spacer/logic.rs");
    let motion_source = load_source("src/spacer/motion.rs");
    let headless_source = load_source("../ui-headless/src/spacer.rs");
    let check2_source = load_source("src/spacer/check2.md");

    for (name, source) in [
        ("view", view_source.as_str()),
        ("logic", logic_source.as_str()),
        ("motion", motion_source.as_str()),
        ("headless", headless_source.as_str()),
    ] {
        assert!(
            !source.contains("inner_html"),
            "Spacer {name} layer should not inject raw HTML for this N/A contract."
        );
    }

    for forbidden in [
        "dangerously_set_inner_html",
        "<script",
        "onerror=",
        "onclick=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spacer view should not include unsafe HTML/script injection token `{forbidden}`."
        );
    }

    for needle in ["role=attrs.role", "aria-hidden=attrs.aria_hidden"] {
        assert!(
            view_source.contains(needle),
            "Even without inner_html, spacer must preserve semantic marker `{needle}`."
        );
    }

    for needle in [
        "- [x] `inner_html` 使用约束",
        "N/A：`Spacer` 实现与文档示例均未使用 `inner_html`",
        "严禁直接或间接注入用户输入、远端返回或未清洗模板字符串",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep inner_html safety governance token `{needle}`."
        );
    }
}

#[test]
fn spacer_wasm_debug_contract_is_not_applicable_and_feature_isolated() {
    let check2_source = load_source("src/spacer/check2.md");
    let cargo_source = load_source("Cargo.toml");
    let view_source = load_source("src/spacer/view.rs");
    let logic_source = load_source("src/spacer/logic.rs");
    let headless_source = load_source("../ui-headless/src/spacer.rs");

    for forbidden in ["spacer-wasm-debug", "component-spacer-wasm-debug"] {
        assert!(
            !cargo_source.contains(forbidden),
            "Spacer should not expose dedicated wasm-debug feature `{forbidden}` for non-interactive N/A contract."
        );
    }

    for (name, source) in [
        ("view", view_source.as_str()),
        ("logic", logic_source.as_str()),
        ("headless", headless_source.as_str()),
    ] {
        for forbidden in [
            "data-debug-source=",
            "data-debug-before=",
            "data-debug-after=",
            "data-debug-timestamp-ms=",
            "request_replay",
            "wasm_debug_proxy!",
            "#[prop(optional)] debug",
            "UiDebugOverlay",
            "provide_ui_trace",
            "trace.emit(",
        ] {
            assert!(
                !source.contains(forbidden),
                "Spacer {name} layer should not leak wasm-debug internals `{forbidden}` into production contract."
            );
        }
    }

    for needle in [
        "data-axis=attrs.data_axis",
        "data-size=attrs.data_size",
        "data-state=attrs.data_state",
        "data-motion-source=motion_source",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer should keep stable semantic marker `{needle}` for state/source observability."
        );
    }

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "N/A：`Spacer` 无关键交互状态转移链，仅消费已归一化状态并输出语义标记",
        "调试开关默认不进入生产包体与公共 API",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep wasm-debug governance token `{needle}`."
        );
    }
}

#[test]
fn spacer_dx_fast_style_feedback_and_workbench_contract_is_stable() {
    let check2_source = load_source("src/spacer/check2.md");
    let view_source = load_source("src/spacer/view.rs");
    let styles_source = load_source("src/spacer/styles.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let dev_docs_script = load_source("../../scripts/dev-docs-app.sh");
    let dev_web_script = load_source("../../scripts/dev-web-demo.sh");

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "回归：`crates/ui-layout/tests/spacer_semantics.rs::spacer_dx_fast_style_feedback_and_workbench_contract_is_stable`",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep DX governance token `{needle}`."
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "Spacer should keep static styles in styles.rs for quick style-iteration path."
    );
    assert!(
        !view_source.contains("style="),
        "Spacer view should not push style logic into inline runtime path."
    );

    for needle in [
        "pub(super) fn spacer() -> AnyView",
        "Playground title=\"Hello World\"",
        "Playground title=\"Axis + Size\"",
        "Playground title=\"Custom Class Marker\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Spacer docs should keep a workbench-like isolated playground entry `{needle}`."
        );
    }

    for needle in ["#!/usr/bin/env bash", "docs-app", "web-demo"] {
        assert!(
            dev_docs_script.contains(needle) || dev_web_script.contains(needle),
            "Dev scripts should keep fast local feedback entry token `{needle}`."
        );
    }

    for forbidden in ["#[prop(optional)] state:", "#[prop(optional)] debug_state:"] {
        assert!(
            !view_source.contains(forbidden),
            "Spacer base API should not force internal debug/state object wiring `{forbidden}`."
        );
    }
}

#[test]
fn spacer_ui_layout_entry_files_are_wired_correctly() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");

    for needle in [
        "#[cfg(feature = \"component-spacer\")]",
        "pub mod spacer;",
        "pub use spacer::{Spacer, SpacerAxis, SpacerMotion, SpacerSize};",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-layout lib entry should expose spacer through feature-gated API via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-spacer\")]",
        "out.push_str(crate::spacer::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-layout css entry should feature-gate spacer CSS via `{needle}`."
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should remain the centralized base/theme/components-css injection surface via `{needle}`."
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for absent in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !manifest_dir.join(absent).exists(),
            "ui-layout root should not contain deprecated shared primitive shim `{absent}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        ".ui-active-highlight {",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared style/motion surface should include `{needle}`."
        );
    }
}

#[test]
fn spacer_component_directory_layout_is_standard() {
    let spacer_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spacer");

    for required in [
        "mod.rs",
        "logic.rs",
        "styles.rs",
        "view.rs",
        "motion.rs",
        "check2.md",
    ] {
        assert!(
            spacer_dir.join(required).exists(),
            "Spacer directory should contain standard file `{required}`."
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !spacer_dir.join(forbidden).exists(),
            "Spacer directory should not drift to `{forbidden}`."
        );
    }
}

#[test]
fn spacer_agent_contract_markers_are_machine_readable() {
    let check2_source = load_source("src/spacer/check2.md");
    let view_source = load_source("src/spacer/view.rs");
    let headless_source = load_source("../ui-headless/src/spacer.rs");
    let primitive_source = load_source("../ui-state-primitives/src/spacer.rs");

    for needle in [
        "data-slot=attrs.data_slot",
        "data-axis=attrs.data_axis",
        "data-size=attrs.data_size",
        "data-state=attrs.data_state",
        "data-custom-class=attrs.data_custom_class",
        "data-motion-source=motion_source",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer should expose machine-readable marker `{needle}` for agent/test consumers."
        );
    }

    for needle in [
        "pub struct SpacerSemanticState",
        "pub axis: &'static str",
        "pub size: &'static str",
        "pub is_vertical: bool",
        "pub is_horizontal: bool",
        "pub has_custom_class_name: bool",
        "pub struct SpacerContract",
        "pub attrs: SpacerAttrs",
        "pub state: SpacerSemanticState",
        "pub struct SpacerOptions",
        "pub state: SpacerState",
        "pub fn use_spacer(",
        "data_axis: options.state.axis_attr",
        "data_size: options.state.size_attr",
        "data_state: options.state.axis_attr",
        "data_vertical: options.state.is_vertical.then_some(\"true\")",
        "data_horizontal: options.state.is_horizontal.then_some(\"true\")",
        "data_custom_class: options.state.has_custom_class_name.then_some(\"true\")",
    ] {
        assert!(
            headless_source.contains(needle),
            "Headless spacer contract should stay typed and machine-readable via `{needle}`."
        );
    }

    for needle in [
        "pub struct SpacerState",
        "pub axis_attr: &'static str",
        "pub size_attr: &'static str",
        "pub is_vertical: bool",
        "pub is_horizontal: bool",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Agent-facing markers should trace back to typed state primitives via `{needle}`."
        );
    }

    for forbidden in ["format!(", "push_str(", "String::from(\"data-"] {
        assert!(
            !headless_source.contains(forbidden),
            "Agent contract fields should not rely on ad-hoc string assembly token `{forbidden}`."
        );
    }

    for forbidden in [
        "inner_html",
        "dangerously_set_inner_html",
        "<script",
        "onerror=",
        "onclick=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spacer render chain should stay in whitelist-safe boundary and reject script injection token `{forbidden}`."
        );
    }

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "回归：`crates/ui-layout/tests/spacer_semantics.rs::spacer_agent_contract_markers_are_machine_readable`",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep agent-contract governance token `{needle}`."
        );
    }

    assert!(
        !view_source.contains("inner_html"),
        "Spacer agent-consumable contract should not use raw HTML injection path."
    );
}

#[test]
fn spacer_streaming_scope_is_explicitly_snapshot_only() {
    let check2_source = load_source("src/spacer/check2.md");
    let view_source = load_source("src/spacer/view.rs");
    let headless_source = load_source("../ui-headless/src/spacer.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "回归：`crates/ui-layout/tests/spacer_semantics.rs::spacer_streaming_scope_is_explicitly_snapshot_only`",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep streaming-scope governance token `{needle}`."
        );
    }

    for forbidden in [
        "Streaming",
        "streaming",
        "snapshot_state",
        "draft",
        "validated",
        "fallback=snapshot",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spacer is not an LLM-reading surface and should not expose streaming protocol fields `{forbidden}`."
        );
    }

    for forbidden in [
        "Streaming",
        "streaming",
        "snapshot_state",
        "fallback=snapshot",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "Spacer headless contract should not leak LLM streaming protocol field `{forbidden}`."
        );
    }

    assert!(
        docs_source.contains("pub(super) fn spacer() -> AnyView"),
        "Spacer docs should remain in docs-app layout section as snapshot-rendered component docs."
    );

    for needle in [
        "logic::resolve_state(SpacerStateInput {",
        "let spacer = use_spacer(SpacerOptions { state, lang, dir });",
        "role=attrs.role",
        "aria-hidden=attrs.aria_hidden",
        "data-axis=attrs.data_axis",
        "data-size=attrs.data_size",
        "data-state=attrs.data_state",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer should consume complete normalized props and render stable snapshot marker `{needle}`."
        );
    }

    for forbidden in ["on_retry", "retry_delay", "reconnect", "on_stream_error"] {
        assert!(
            !view_source.contains(forbidden) && !headless_source.contains(forbidden),
            "Streaming recovery/data-validation policy should stay in upper layer; found `{forbidden}`."
        );
    }
}

#[test]
fn spacer_docs_are_synced_and_copy_paste_ready() {
    let check2_source = load_source("src/spacer/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_source = load_source("../ui-components/src/code_block/view.rs");
    let e2e_toggle_source = load_source("../../e2e/tests/docs_app_playground_code_toggle.spec.mjs");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "回归：`crates/ui-layout/tests/spacer_semantics.rs::spacer_docs_are_synced_and_copy_paste_ready`",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep source-first docs governance token `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn spacer() -> AnyView",
        "title=\"Spacer\"",
        "slug=\"spacer\"",
        "let hello_code = Signal::derive(move || r#\"<Spacer />\"#.to_string());",
        "Playground title=\"Hello World\"",
        "Playground title=\"Axis + Size\"",
        "Playground title=\"Custom Class Marker\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Spacer docs page should stay synchronized with component API via `{needle}`."
        );
    }

    assert!(
        pages_source
            .contains("component_doc!(\"Spacer\", \"spacer\", \"Layout\", layout::spacer),"),
        "docs-app component catalog should point to the real Spacer docs source entry."
    );

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "#[prop(optional, into)] code_imports: Option<String>",
        "#[prop(optional, into)] code_signal: Option<Signal<String>>",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "compose_copy_ready_code_prepends_imports_when_missing",
        "compose_copy_ready_code_keeps_existing_imports",
        "compose_copy_ready_code_does_not_duplicate_when_roots_exist",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground infra should keep copy-paste-ready code path via `{needle}`."
        );
    }

    assert!(
        playground_source.contains(
            "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";"
        ) || playground_source.contains(
            "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_layout::*;\";"
        ),
        "Playground default imports should keep copy-ready root import (`ui_components` global default or `ui_layout` explicit)."
    );

    for needle in [
        "data-slot=\"code-block\"",
        "data-copyable=state.copyable.then_some(\"true\")",
        "class_name=\"ui-code-block__copy-button\".to_string()",
    ] {
        assert!(
            code_block_source.contains(needle),
            "CodeBlock should keep explicit copy affordance/markers via `{needle}`."
        );
    }

    for needle in [
        "test(\"docs-app component playground can toggle code visibility\"",
        "const codeBlock = playground.locator('[data-slot=\"code-block\"]');",
    ] {
        assert!(
            e2e_toggle_source.contains(needle),
            "E2E should keep reproducible playground-code verification token `{needle}`."
        );
    }
}

#[test]
fn spacer_heroui_benchmark_docs_sync_contract_is_explicit() {
    let check2_source = load_source("src/spacer/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let view_source = load_source("src/spacer/view.rs");
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let research_source =
        load_source("../../docs/research/spectrum-heroui-style-interface-study.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "本轮 Spacer 参数语义未新增/变更，仅补文档示例与契约测试；组件文档入口可访问并被索引。",
        "回归：`crates/ui-layout/tests/spacer_semantics.rs::spacer_heroui_benchmark_docs_sync_contract_is_explicit`",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep HeroUI benchmarking-doc sync token `{needle}`."
        );
    }

    for needle in [
        "# HeroUI 参数设计风格对齐策略",
        "Goal",
        "Definition of Done",
    ] {
        assert!(
            strategy_source.contains(needle),
            "HeroUI strategy doc should contain baseline section token `{needle}`."
        );
    }

    for needle in [
        "# Spectrum × HeroUI 样式与接口综合学习（v0）",
        "Goal",
        "Definition of Done",
    ] {
        assert!(
            research_source.contains(needle),
            "Spectrum/HeroUI research doc should contain baseline section token `{needle}`."
        );
    }

    assert!(
        pages_source
            .contains("component_doc!(\"Spacer\", \"spacer\", \"Layout\", layout::spacer),"),
        "Spacer docs entry should stay indexable in docs catalog."
    );
    assert!(
        docs_source.contains("pub(super) fn spacer() -> AnyView"),
        "Spacer docs page source should remain accessible from docs-app."
    );

    for needle in [
        "#[prop(optional)] axis: SpacerAxis",
        "#[prop(optional)] size: SpacerSize",
        "#[prop(optional, into)] class_name: Option<String>",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer parameter model should remain stable for this no-param-change cycle via `{needle}`."
        );
    }
}

#[test]
fn spacer_forbidden_antipatterns_contract_is_locked() {
    let check2_source = load_source("src/spacer/check2.md");
    let primitive_source = load_source("../ui-state-primitives/src/spacer.rs");
    let headless_source = load_source("../ui-headless/src/spacer.rs");
    let view_source = load_source("src/spacer/view.rs");
    let logic_source = load_source("src/spacer/logic.rs");
    let mod_source = load_source("src/spacer/mod.rs");
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 在 `view` 层隐藏关键状态决策。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "- [x] 公共 API 泄露底层实现细节类型。",
        "- [x] 用临时补丁破坏跨组件一致性。",
        "回归：`crates/ui-layout/tests/spacer_semantics.rs::spacer_forbidden_antipatterns_contract_is_locked`",
        "- [x] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep forbidden-antipattern governance token `{needle}`."
        );
    }

    for forbidden in ["web_sys", "wasm_bindgen", "class=", "style="] {
        assert!(
            !primitive_source.contains(forbidden),
            "State primitives must stay DOM/style free; found `{forbidden}`."
        );
    }

    for forbidden in [
        "pub const CSS",
        "ui-spacer--",
        "keyframes",
        "duration",
        "easing",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "Headless layer must not include visual/motion choreography token `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("logic::resolve_state(SpacerStateInput {"),
        "View should consume normalized state from logic instead of rebuilding state rules."
    );
    for forbidden in [
        "match axis",
        "match size",
        "labels:",
        "titles:",
        "panels:",
        "children: Children",
        "ItemSpec",
        "#[prop(optional)] on_open_change:",
        "#[prop(optional)] default_open:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "View/API should avoid antipattern token `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains(
            "pub use ui_state_primitives::spacer::{SpacerState, SpacerStateInput, resolve_state};"
        ),
        "Reusable state primitive should remain sunk into ui-state-primitives."
    );

    for forbidden in ["web_sys::", "wasm_bindgen", "HtmlElement", "JsValue"] {
        assert!(
            !mod_source.contains(forbidden) && !lib_source.contains(forbidden),
            "Public API surface should not leak platform/private runtime type `{forbidden}`."
        );
    }

    for (name, source) in [
        ("view", view_source.as_str()),
        ("logic", logic_source.as_str()),
        ("mod", mod_source.as_str()),
    ] {
        for forbidden in [
            "temporary patch",
            "temp workaround",
            "bypass contract",
            "HACK:",
        ] {
            assert!(
                !source.contains(forbidden),
                "Spacer {name} should not carry temporary consistency-bypass marker `{forbidden}`."
            );
        }
    }
}

#[test]
fn spacer_final_merge_gate_contracts_are_satisfied() {
    let check2_source = load_source("src/spacer/check2.md");
    let test_source = load_source("tests/spacer_semantics.rs");
    let view_source = load_source("src/spacer/view.rs");
    let logic_source = load_source("src/spacer/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/spacer.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");

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
        "回归：`crates/ui-layout/tests/spacer_semantics.rs::spacer_final_merge_gate_contracts_are_satisfied`",
        "该条暂不做 smoke",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer merge-gate checklist should keep token `{needle}`."
        );
    }

    for needle in [
        "fn spacer_ui_layout_layering_contract_is_enforced()",
        "fn spacer_state_normalization_is_centralized_in_logic()",
        "fn spacer_has_a11y_and_i18n_mount_points_without_hardcoded_copy()",
        "fn spacer_public_api_naming_contract_is_stable()",
        "fn spacer_discrete_state_axes_are_type_constrained()",
        "fn spacer_state_markers_are_observable_and_closed_set()",
        "fn spacer_docs_playgrounds_lock_state_matrix_contract_values()",
        "fn spacer_forbidden_antipatterns_contract_is_locked()",
    ] {
        assert!(
            test_source.contains(needle),
            "Merge-gate evidence should include semantic regression `{needle}`."
        );
    }

    assert!(
        view_source.contains("role=attrs.role")
            && view_source.contains("aria-hidden=attrs.aria_hidden")
            && view_source.contains("data-state=attrs.data_state"),
        "Accessible and machine-readable markers should stay mounted in Spacer view."
    );

    assert!(
        logic_source.contains(
            "pub use ui_state_primitives::spacer::{SpacerState, SpacerStateInput, resolve_state};"
        ),
        "Layering gate requires logic to consume primitives instead of reimplementing state machines."
    );
    assert!(
        primitive_source.contains("pub enum SpacerAxis")
            && primitive_source.contains("pub enum SpacerSize")
            && primitive_source.contains("pub fn resolve_state("),
        "Invalid-state restriction gate requires enum-based axes and centralized normalization."
    );

    assert!(
        ui_motion_source.contains("pub fn prefers_reduced_motion() -> bool {"),
        "Reduced-motion gate should remain available through ui-motion contract."
    );
    assert!(
        headless_lib_source.contains(
            "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")"
        ),
        "SSR/wasm gate should keep ui-headless web+ssr compile_error protection."
    );

    for needle in [
        "cargo clippy -p ui-layout --no-default-features --features component-spacer --test spacer_semantics -- -D warnings",
        "cargo test -p ui-layout --no-default-features --features component-spacer --test spacer_semantics",
        "cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-spacer",
    ] {
        assert!(
            check2_source.contains(needle),
            "Merge-gate command evidence token `{needle}` should stay explicit."
        );
    }
}

#[test]
fn spacer_avoids_forbidden_architecture_antipatterns() {
    let primitive_source = load_source("../ui-state-primitives/src/spacer.rs");
    let headless_source = load_source("../ui-headless/src/spacer.rs");
    let view_source = load_source("src/spacer/view.rs");
    let logic_source = load_source("src/spacer/logic.rs");

    for forbidden in ["web_sys", "wasm_bindgen", "class=", "style="] {
        assert!(
            !primitive_source.contains(forbidden),
            "State primitives must stay pure and avoid DOM/style bindings; found `{forbidden}`."
        );
    }

    for forbidden in [
        "pub const CSS",
        "ui-spacer--",
        "keyframes",
        "duration",
        "easing",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "Headless spacer contract must not include visual/motion composition details `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("logic::resolve_state(SpacerStateInput {"),
        "View should consume centralized logic state output."
    );
    for forbidden in ["match axis", "match size"] {
        assert!(
            !view_source.contains(forbidden),
            "View should not hide core state decisions; found `{forbidden}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] is_open:",
        "#[prop(optional)] on_open_change:",
        "#[prop(optional)] default_open:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "New props must follow unified naming/contracts and avoid unrelated alias drift `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains(
            "pub use ui_state_primitives::spacer::{SpacerState, SpacerStateInput, resolve_state};"
        ),
        "Reusable state primitive should be sourced from ui-state-primitives, not reimplemented in component logic."
    );
}

#[test]
fn spacer_ui_layout_layering_contract_is_enforced() {
    let mod_source = load_source("src/spacer/mod.rs");
    let logic_source = load_source("src/spacer/logic.rs");
    let view_source = load_source("src/spacer/view.rs");
    let styles_source = load_source("src/spacer/styles.rs");
    let motion_source = load_source("src/spacer/motion.rs");
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "mod logic;",
        "mod view;",
        "pub mod styles;",
        "pub mod motion;",
        "pub use motion::SpacerMotion;",
        "pub use ui_state_primitives::spacer::{SpacerAxis, SpacerSize};",
        "pub use view::Spacer;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Spacer module boundary should include `{needle}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::spacer::{SpacerState, SpacerStateInput, resolve_state};",
        "pub fn normalize_optional_text(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Spacer logic layer should include `{needle}`."
        );
    }
    for forbidden in ["use ui_headless", "view! {", "web_sys", "wasm_bindgen"] {
        assert!(
            !logic_source.contains(forbidden),
            "Spacer logic layer should stay assembly-only; found forbidden `{forbidden}`."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, SpacerOptions, use_spacer};",
        "let spacer = use_spacer(SpacerOptions { state, lang, dir });",
        "super::motion::attach_motion(node_ref, motion);",
        "view! {",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer view layer should include `{needle}`."
        );
    }
    for forbidden in [
        "ui_state_primitives::spacer::resolve_state(",
        "web_sys",
        "wasm_bindgen",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spacer view should not bypass assembly boundaries; found `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "Spacer styles layer should provide a static CSS contract."
    );
    assert!(
        styles_source.contains("var(--ui-space-md)"),
        "Spacer styles should stay token-first."
    );

    for needle in [
        "pub struct SpacerMotion",
        "pub fn sanitize_motion(",
        "pub fn source_attr(",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "Spacer motion layer should include `{needle}`."
        );
    }

    for forbidden in ["web_sys::", "wasm_bindgen", "HtmlElement", "JsValue"] {
        assert!(
            !mod_source.contains(forbidden),
            "Spacer public module boundary must not expose platform details `{forbidden}`."
        );
        assert!(
            !lib_source.contains(&format!("spacer::{{Spacer, SpacerAxis, {forbidden}")),
            "ui-layout public re-export should not include platform detail `{forbidden}`."
        );
    }
}

#[test]
fn spacer_public_api_naming_contract_is_stable() {
    let view_source = load_source("src/spacer/view.rs");
    let mod_source = load_source("src/spacer/mod.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "#[prop(optional)] axis: SpacerAxis",
        "#[prop(optional)] size: SpacerSize",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional)] motion: super::motion::SpacerMotion",
        "#[prop(optional, into)] class_name: Option<String>",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer public props should keep stable naming; missing `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] class:",
        "#[prop(optional)] open:",
        "#[prop(optional)] is_open:",
        "#[prop(optional)] default_open:",
        "#[prop(optional)] on_open_change:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spacer should not introduce alias drift or unrelated controllable-axis names; found `{forbidden}`."
        );
    }

    for needle in [
        "pub use motion::SpacerMotion;",
        "pub use ui_state_primitives::spacer::{SpacerAxis, SpacerSize};",
        "pub use view::Spacer;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Spacer stable exports should include `{needle}`."
        );
    }

    assert!(
        docs_source.contains("class_name=\"docs-spacer-guide\".to_string()"),
        "Docs should align with public prop naming (`class_name`)."
    );
}

#[test]
fn spacer_has_no_controllable_state_axis_and_no_half_controlled_api() {
    let view_source = load_source("src/spacer/view.rs");
    let logic_source = load_source("src/spacer/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/spacer.rs");

    for forbidden in [
        "#[prop(optional)] value:",
        "#[prop(optional)] default_value:",
        "#[prop(optional)] on_value_change:",
        "#[prop(optional)] open:",
        "#[prop(optional)] default_open:",
        "#[prop(optional)] on_open_change:",
        "use_controllable",
        "RwSignal<",
        "WriteSignal<",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spacer should not expose controllable-axis API for a stateless layout primitive; found `{forbidden}`."
        );
    }

    for forbidden in ["use_controllable", "RwSignal<", "WriteSignal<"] {
        assert!(
            !logic_source.contains(forbidden),
            "Spacer logic should not implement hidden controlled/uncontrolled state; found `{forbidden}`."
        );
    }

    for needle in [
        "pub struct SpacerStateInput",
        "pub struct SpacerState",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Spacer should stay a pure derived-state primitive via `{needle}`."
        );
    }
}

#[test]
fn spacer_default_values_have_single_source_and_view_has_no_fallbacks() {
    let view_source = load_source("src/spacer/view.rs");
    let logic_source = load_source("src/spacer/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/spacer.rs");

    for needle in [
        "#[prop(optional)] axis: SpacerAxis",
        "#[prop(optional)] size: SpacerSize",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(SpacerStateInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer default and normalization path should include `{needle}`."
        );
    }

    for forbidden in ["unwrap_or(", "unwrap_or_else(", ".or(", ".or_else("] {
        assert!(
            !view_source.contains(forbidden),
            "Spacer view must not do secondary default fallback; found `{forbidden}`."
        );
    }

    for needle in ["#[default]", "pub enum SpacerAxis", "pub enum SpacerSize"] {
        assert!(
            primitive_source.contains(needle),
            "Default baseline should stay on typed primitives; missing `{needle}`."
        );
    }

    assert!(
        logic_source.contains("pub fn normalize_optional_text("),
        "logic.rs should remain the single normalization entry for optional text inputs."
    );
}

#[test]
fn spacer_state_normalization_is_centralized_in_logic() {
    let view_source = load_source("src/spacer/view.rs");
    let logic_source = load_source("src/spacer/logic.rs");
    let styles_source = load_source("src/spacer/styles.rs");

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(SpacerStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer view should consume centralized normalized state via `{needle}`."
        );
    }

    for forbidden in [
        "match axis",
        "match size",
        "if axis",
        "if size",
        "on:click",
        "on:keydown",
        "on:input",
        "on:change",
        "on:focus",
        "on:blur",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spacer view should not rebuild state logic or event-driven state machine; found `{forbidden}`."
        );
    }

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn compose_class_name(",
        "pub use ui_state_primitives::spacer::{SpacerState, SpacerStateInput, resolve_state};",
    ] {
        assert!(
            logic_source.contains(needle),
            "Spacer logic should own normalization/assembly contract via `{needle}`."
        );
    }

    assert!(
        styles_source.contains("data-axis"),
        "styles layer should consume semantic state markers only."
    );
}

#[test]
fn spacer_discrete_state_axes_are_type_constrained() {
    let primitive_source = load_source("../ui-state-primitives/src/spacer.rs");
    let view_source = load_source("src/spacer/view.rs");

    for needle in [
        "pub enum SpacerAxis",
        "Vertical",
        "Horizontal",
        "pub enum SpacerSize",
        "Xs",
        "Sm",
        "Md",
        "Lg",
        "Xl",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Spacer discrete axes should be enum-constrained; missing `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] axis: SpacerAxis",
        "#[prop(optional)] size: SpacerSize",
    ] {
        assert!(
            view_source.contains(needle),
            "Spacer API should expose typed axes via `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] axis: Option<bool>",
        "#[prop(optional)] size: Option<bool>",
        "#[prop(optional)] axis: Option<String>",
        "#[prop(optional)] size: Option<String>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spacer must not regress to bool/string state explosion; found `{forbidden}`."
        );
    }
}

#[test]
fn spacer_consumes_state_primitives_without_business_store_binding() {
    let logic_source = load_source("src/spacer/logic.rs");
    let view_source = load_source("src/spacer/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/spacer.rs");

    assert!(
        logic_source.contains(
            "pub use ui_state_primitives::spacer::{SpacerState, SpacerStateInput, resolve_state};"
        ),
        "Spacer logic must consume state-primitives via ui_state_primitives::spacer."
    );

    assert!(
        view_source.contains("logic::resolve_state(SpacerStateInput {"),
        "Spacer view should consume normalized state from logic instead of custom state implementation."
    );

    for forbidden in [
        "use leptos::store",
        "create_rw_signal(",
        "RwSignal<",
        "WriteSignal<",
        "Store<",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Spacer logic should not bind to business/global store directly; found `{forbidden}`."
        );
    }

    for needle in [
        "pub struct SpacerStateInput",
        "pub struct SpacerState",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Spacer primitive source contract should include `{needle}`."
        );
    }
}

#[test]
fn spacer_async_contract_is_explicitly_not_applicable() {
    let view_source = load_source("src/spacer/view.rs");
    let logic_source = load_source("src/spacer/logic.rs");
    let headless_source = load_source("../ui-headless/src/spacer.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "on_retry",
        "retry",
        "use_async_action",
        "async fn",
        "tokio::",
        "async_std::",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Spacer view should stay async-free for this N/A contract; found `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "Spacer logic should stay async-free for this N/A contract; found `{forbidden}`."
        );
        assert!(
            !headless_source.contains(forbidden),
            "Spacer headless contract should stay async-free for this N/A contract; found `{forbidden}`."
        );
    }
}

#[test]
fn spacer_engineering_capabilities_stay_unified_and_runtime_agnostic() {
    let check2_source = load_source("src/spacer/check2.md");
    let mod_source = load_source("src/spacer/mod.rs");
    let view_source = load_source("src/spacer/view.rs");
    let logic_source = load_source("src/spacer/logic.rs");
    let motion_source = load_source("src/spacer/motion.rs");
    let headless_source = load_source("../ui-headless/src/spacer.rs");

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "N/A：`Spacer` 不涉及 spec/config 反序列化与异步边界；组件 API 未暴露 runtime 类型，且无私有 tracing 语义漂移。",
        "回归：`crates/ui-layout/tests/spacer_semantics.rs::spacer_engineering_capabilities_stay_unified_and_runtime_agnostic`",
    ] {
        assert!(
            check2_source.contains(needle),
            "Spacer checklist should keep engineering-capability governance token `{needle}`."
        );
    }

    for (name, source) in [
        ("mod", mod_source.as_str()),
        ("view", view_source.as_str()),
        ("logic", logic_source.as_str()),
        ("motion", motion_source.as_str()),
        ("headless", headless_source.as_str()),
    ] {
        for forbidden in [
            "serde::",
            "Serialize",
            "Deserialize",
            "#[serde(",
            "tracing::",
            "tracing::instrument",
            "info_span!(",
            "event!(",
            "tokio::",
            "async_std::",
            "smol::",
            "JoinHandle",
            "Runtime",
            "async fn",
        ] {
            assert!(
                !source.contains(forbidden),
                "Spacer {name} layer should stay runtime-agnostic and free of spec/tracing plumbing token `{forbidden}`."
            );
        }
    }

    for forbidden in [
        "#[prop(optional)] runtime:",
        "#[prop(optional)] executor:",
        "#[prop(optional)] on_trace:",
        "pub use tokio::",
        "pub use async_std::",
    ] {
        assert!(
            !view_source.contains(forbidden) && !mod_source.contains(forbidden),
            "Spacer public API should not leak runtime/tracing implementation detail `{forbidden}`."
        );
    }
}
