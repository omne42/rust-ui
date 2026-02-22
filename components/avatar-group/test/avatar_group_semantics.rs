use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = if let Some(suffix) = rel_path.strip_prefix("src/avatar/") {
        manifest_dir
            .join("../../components/avatar-group/src")
            .join(suffix)
    } else {
        manifest_dir.join(rel_path)
    };
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn function_signature(source: &str, fn_name: &str) -> String {
    let start = source
        .find(&format!("pub fn {fn_name}("))
        .unwrap_or_else(|| panic!("missing function signature for `{fn_name}`"));
    let end = source[start..]
        .find(") -> impl IntoView {")
        .unwrap_or_else(|| panic!("missing IntoView return marker for `{fn_name}`"));
    source[start..start + end].to_string()
}

#[test]
fn avatar_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/avatar/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "AvatarGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn avatar_group_uses_logic_state_model() {
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");

    for needle in [
        "pub use ui_state_primitives::avatar_group::{",
        "AvatarGroupStateInput",
        "normalize_optional_text",
        "normalize_avatar_group_max_visible",
        "resolve_avatar_group_aria_label",
        "pub fn normalize_avatar_group_input(",
        "pub fn normalize_avatar_group_item_fields(",
        "pub fn resolve_avatar_group_aria_label_with_fallback(",
        "AvatarGroupRenderState",
        "resolve_render_state",
        "pub fn compose_avatar_group_class_name(",
        "ui-avatar-group--custom-class",
    ] {
        assert!(
            logic_source.contains(needle),
            "AvatarGroup logic should consume state primitives and keep assembly helpers; missing `{needle}`."
        );
    }

    for forbidden in [
        "pub struct AvatarGroupStateInput {",
        "pub struct AvatarGroupState {",
        "pub enum AvatarGroupVisualState {",
        "pub enum AvatarGroupAriaLabelSource {",
        "pub enum AvatarGroupClassSource {",
        "pub struct AvatarGroupRenderState {",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "AvatarGroup logic must not reimplement state primitives; found `{forbidden}`."
        );
    }

    for needle in [
        "let i18n = i18n::use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "let normalized = logic::normalize_avatar_group_input(",
        "let group_a11y = labeled_group_attrs(normalized.aria_label, normalized.lang.clone(), dir);",
        "logic::resolve_avatar_group_render_state(logic::AvatarGroupStateInput {",
        "let class = logic::compose_avatar_group_class_name(normalized.class_name, state);",
        "let fields = logic::normalize_avatar_group_item_fields(item.name, item.src, item.alt);",
    ] {
        assert!(
            view_source.contains(needle),
            "AvatarGroup view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn avatar_group_emits_baseline_style_root_data_attributes() {
    let source = load_source("src/avatar/view.rs");

    for attr in [
        "data-slot=\"avatar-group\"",
        "data-size=state.size_attr",
        "data-state=state.visual_state.as_str()",
        "data-empty=state.visual_state.is_empty().then_some(\"true\")",
        "data-has-items=state.has_items().then_some(\"true\")",
        "data-has-overflow=state.visual_state.has_overflow().then_some(\"true\")",
        "data-count=state.total_count.to_string()",
        "data-visible-count=state.visible_count.to_string()",
        "data-overflow-count=state.overflow_count.to_string()",
        "data-max-visible=state.max_visible.to_string()",
        "data-custom-aria-label=state.aria_label_source.is_custom().then_some(\"true\")",
        "data-aria-label-source=state.aria_label_source.as_str()",
        "data-custom-class=state.class_source.is_custom().then_some(\"true\")",
        "data-class-source=state.class_source.as_str()",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label",
        "lang=group_a11y.lang.clone()",
        "dir=group_a11y.dir",
    ] {
        assert!(
            source.contains(attr),
            "AvatarGroup should set `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn avatar_group_a11y_i18n_l10n_contract_is_headless_driven_and_no_view_hardcoded_copy() {
    let view_source = load_source("src/avatar/view.rs");
    let headless_a11y_source = load_source("../ui-headless/src/a11y.rs");
    let i18n_common_source = load_source("../ui-headless/src/i18n/common.rs");

    for required in [
        "use ui_headless::labeled_group_attrs;",
        "let i18n = i18n::use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "common.avatar_group_aria_label.as_ref()",
        "common.avatar_group_overflow_aria_label_suffix.as_ref()",
        "let group_a11y = labeled_group_attrs(normalized.aria_label, normalized.lang.clone(), dir);",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label",
        "lang=group_a11y.lang.clone()",
        "dir=group_a11y.dir",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup should wire a11y/i18n/l10n contract via `{required}`."
        );
    }

    for required in [
        "pub fn labeled_group_attrs(",
        "pub struct LabeledGroupA11yAttrs",
        "pub fn locale_attrs(",
    ] {
        assert!(
            headless_a11y_source.contains(required),
            "AvatarGroup shared a11y utilities should come from ui-headless via `{required}`."
        );
    }

    for required in [
        "avatar_group_aria_label",
        "avatar_group_overflow_aria_label_suffix",
    ] {
        assert!(
            i18n_common_source.contains(required),
            "AvatarGroup i18n bundle should expose string slot `{required}`."
        );
    }

    for forbidden in [
        "\"Avatar group\"",
        "\"more collaborators\"",
        "role=\"group\"",
        "dir=\"ltr\"",
        "dir=\"rtl\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup view should not hardcode user-visible copy/locale/a11y literal `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_exposes_item_and_overflow_slots() {
    let source = load_source("src/avatar/view.rs");

    for attr in [
        "data-slot=\"avatar-group-item\"",
        "data-index=index",
        "data-has-src=fields.has_src.then_some(\"true\")",
        "class_name=\"ui-avatar-group__avatar\"",
        "data-slot=\"avatar-group-overflow\"",
        "data-count=state.overflow_count.to_string()",
    ] {
        assert!(
            source.contains(attr),
            "AvatarGroup should expose `{attr}` for deterministic item/overflow hooks."
        );
    }
}

#[test]
fn avatar_group_has_no_async_loading_protocol_and_keeps_sync_render_contract() {
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");

    for needle in [
        "let state = logic::resolve_avatar_group_render_state(logic::AvatarGroupStateInput {",
        "let fields = logic::normalize_avatar_group_item_fields(item.name, item.src, item.alt);",
        "<Show when=move || state.visual_state.has_overflow()>",
    ] {
        assert!(
            view_source.contains(needle),
            "AvatarGroup should keep synchronous render contract via `{needle}`."
        );
    }

    for forbidden in [
        "use_async_action",
        "is_loading",
        "aria-busy",
        "retry",
        "async fn",
        ".await",
        "Future<",
        "spawn_local",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "AvatarGroup has no async workflow; forbidden async/loading token `{forbidden}` should be absent."
        );
    }
}

#[test]
fn avatar_group_styles_include_state_source_and_marker_contracts() {
    let source = load_source("src/avatar/styles.rs");

    for selector in [
        ".ui-avatar-group--size-sm",
        ".ui-avatar-group[data-size=\"md\"]",
        ".ui-avatar-group--size-lg",
        ".ui-avatar-group--stable",
        ".ui-avatar-group[data-state=\"overflow\"]",
        ".ui-avatar-group--overflow .ui-avatar-group__overflow",
        ".ui-avatar-group[data-has-overflow=\"true\"] .ui-avatar-group__overflow",
        ".ui-avatar-group[data-state=\"empty\"]",
        ".ui-avatar-group--label-source-custom",
        ".ui-avatar-group[data-aria-label-source=\"custom\"]",
        ".ui-avatar-group--custom-class",
        ".ui-avatar-group[data-custom-class=\"true\"]",
        ".ui-avatar-group[data-class-source=\"custom\"]",
    ] {
        assert!(
            source.contains(selector),
            "AvatarGroup styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn avatar_group_styles_use_defensive_variable_fallback_chains() {
    let styles_source = load_source("src/avatar/styles.rs");
    let theme_css_source = load_source("../ui-theme/src/css.rs");

    for required in [
        "var(--ui-avatar-size-sm, var(--ui-fallback-avatar-size-sm))",
        "var(--ui-avatar-size-md, var(--ui-fallback-avatar-size-md))",
        "var(--ui-avatar-size-lg, var(--ui-fallback-avatar-size-lg))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-button-radius-full, var(--ui-fallback-button-radius-full))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
    ] {
        assert!(
            styles_source.contains(required),
            "AvatarGroup styles should keep defensive token fallback chain `{required}`."
        );
    }

    for required in [
        "  --ui-fallback-avatar-size-sm: 24px;",
        "  --ui-fallback-avatar-size-md: 32px;",
        "  --ui-fallback-avatar-size-lg: 40px;",
        "  --ui-fallback-bg: {};",
        "  --ui-fallback-bg-muted: {};",
        "  --ui-fallback-fg: {};",
        "  --ui-fallback-border-width: 1px;",
        "  --ui-fallback-accent: {};",
        "  --ui-fallback-accent-soft: {};",
        "  --ui-fallback-shadow-sm: {};",
        "  --ui-fallback-line-height-100: {}px;",
        "  --ui-fallback-font-size-100: {}px;",
        "  --ui-fallback-button-size-s-font-size: {}px;",
        "  --ui-fallback-space-xs: {}px;",
        "  --ui-fallback-space-sm: {}px;",
        "  --ui-fallback-space-md: {}px;",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme css output should provide avatar-group fallback variable `{required}`."
        );
    }

    for forbidden in [
        "--ui-avatar-group-size: 2rem;",
        "--ui-avatar-group-overlap: 10px;",
        "--ui-avatar-group-overflow-padding: 0.375rem;",
        "border-radius: 9999px;",
        "line-height: var(--ui-line-height-100, 16px);",
        "var(--ui-font-size-100, 12px)",
        "var(--ui-button-size-s-font-size, 13px)",
        "border: 2px solid var(--ui-bg);",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "AvatarGroup styles should not keep raw component terminal fallback `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_entry_source = load_source("src/css.rs");
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let check2_source = load_source("../../components/avatar-group/check2.md");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-avatar_group\")]",
        "out.push_str(crate::avatar_group::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(required),
            "ui css entry should keep cascade-layer contract marker `{required}`."
        );
    }

    for line in view_source.lines().chain(logic_source.lines()) {
        let trimmed = line.trim_start();
        assert!(
            !trimmed.starts_with("style="),
            "AvatarGroup should not use plain inline `style=...`; found `{trimmed}`."
        );
        if trimmed.contains("style:") {
            assert!(
                trimmed.contains("style:--"),
                "AvatarGroup runtime style mutation must use CSS custom properties only; found `{trimmed}`."
            );
        }
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
        "style=\"position:",
        "style:top",
        "style:left",
        "style:right",
        "style:bottom",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "AvatarGroup should avoid plain inline style token `{forbidden}`."
        );
    }

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "avatar_group_cascade_layer_and_runtime_style_contract_is_enforced",
        "avatar_group_cascade_layer_and_runtime_style_contract_is_enforced_local",
    ] {
        assert!(
            check2_source.contains(required),
            "avatar-group checklist should keep cascade-layer/runtime-style evidence `{required}`."
        );
    }
}

#[test]
fn avatar_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn avatar_group() -> AnyView",
        "title=\"AvatarGroup\"",
        "slug=\"avatar-group\"",
        "Playground title=\"Hello World\"",
        "Playground title=\"Overflow Stack\"",
        "title=\"Sizes Without Overflow\"",
        "Playground title=\"Custom Aria + Class\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "title=\"Interactive Playground (Props + State + Preview)\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for AvatarGroup.",
        );
    }
}

#[test]
fn avatar_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "let hello_code =",
        "r#\"<AvatarGroup items=empty_items.clone() />\"#.to_string()",
        "title=\"Hello World\" code_signal=hello_code",
        "<AvatarGroup items=empty_items.clone() />",
        "title=\"Overflow Stack\"",
        "<AvatarGroup items=overflow_items.clone() max=3 size=AvatarSize::Md />",
        "max=2",
        "aria_label=\"Core collaborators\".to_string()",
        "title=\"Sizes Without Overflow\"",
        "<AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Sm />",
        "<AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Lg />",
        "title=\"Custom Aria + Class\"",
        "items=empty_items.clone()",
        "aria_label=\"No collaborators\".to_string()",
        "class_name=\"docs-avatar-group-custom\".to_string()",
        "let state_matrix_code =",
        "title=\"State Matrix\"",
        "state_matrix_items.clone()",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "AvatarGroup has no controlled/uncontrolled state machine.",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "Streaming Optional; fallback=snapshot.",
        "title=\"Interactive Playground (Props + State + Preview)\"",
        "data-slot=\"avatar-group-workbench-controls\"",
        "data-slot=\"avatar-group-workbench-preview\"",
        "data-slot=\"avatar-group-workbench-configured\"",
        "data-slot=\"avatar-group-workbench-state\"",
        "data-slot=\"avatar-group-spec-preview-na\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "Copy action auto-injects missing imports for direct run.",
        "let code_imports =",
        "use ui::{AvatarGroup, AvatarGroupItem, AvatarSize};",
        "data-slot=\"avatar-group-copy-ready-hint\"",
    ] {
        assert!(
            source.contains(needle),
            "avatar-group docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn avatar_group_docs_expose_hello_world_path_without_state_machine_wiring() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "let hello_code =",
        "r#\"<AvatarGroup items=empty_items.clone() />\"#.to_string()",
        "title=\"Hello World\" code_signal=hello_code",
        "<AvatarGroup items=empty_items.clone() />",
    ] {
        assert!(
            source.contains(needle),
            "AvatarGroup docs should keep minimal hello-world usage path via `{needle}`."
        );
    }

    for forbidden in ["<AvatarGroup state=", "ui_state_primitives", "ui-headless"] {
        assert!(
            !source.contains(forbidden),
            "AvatarGroup docs minimal usage should not require internal wiring token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_docs_parameter_and_state_matrix_match_logic_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/avatar_group.rs");
    let signature = function_signature(&load_source("src/avatar/view.rs"), "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        "max: Option<usize>",
        "size: AvatarSize",
        "aria_label: Option<String>",
        "class_name: Option<String>",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
    ] {
        assert!(
            signature.contains(required),
            "AvatarGroup public API should expose `{required}` for docs alignment.",
        );
    }

    for required in [
        "data-slot=\"avatar-group-state-matrix\"",
        "data-slot=\"avatar-group-state-rows\"",
        "data-slot=\"avatar-group-parameter-matrix\"",
        "data-slot=\"avatar-group-parameter-rows\"",
        "\"max: Option&lt;usize&gt;\"",
        "default = None -> normalize to 4",
        "\"size: AvatarSize\"",
        "default = AvatarSize::Md",
        "\"aria_label: Option&lt;String&gt;\"",
        "\"class_name: Option&lt;String&gt;, lang: Option&lt;String&gt;\"",
        "\"dir: Option&lt;A11yDirection&gt;\"",
    ] {
        assert!(
            docs_source.contains(required),
            "AvatarGroup docs should keep parameter/state matrix marker `{required}`.",
        );
    }

    for required in [
        "pub const DEFAULT_MAX_VISIBLE: usize = 4;",
        "pub fn normalize_max_visible(value: Option<usize>) -> usize {",
        "value.unwrap_or(DEFAULT_MAX_VISIBLE)",
    ] {
        assert!(
            primitive_source.contains(required),
            "AvatarGroup primitive default contract should keep `{required}`.",
        );
    }

    for required in [
        "let max_visible = normalize_avatar_group_max_visible(max);",
        "resolve_avatar_group_aria_label_with_fallback(aria_label, default_aria_label);",
        "lang: normalize_avatar_group_optional_text(lang),",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup logic should keep docs-mapped normalization marker `{required}`.",
        );
    }
}

#[test]
fn avatar_group_docs_interactive_playground_supports_live_prop_controls_and_preview_feedback() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for required in [
        "let workbench_roster_options = vec![",
        "let workbench_size_options = vec![\"sm\".to_string(), \"md\".to_string(), \"lg\".to_string()];",
        "let workbench_max_options = vec![\"2\".to_string(), \"3\".to_string(), \"4\".to_string()];",
        "let (workbench_roster_index, set_workbench_roster_index) = signal(Some(2_usize));",
        "let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));",
        "let (workbench_max_index, set_workbench_max_index) = signal(Some(1_usize));",
        "let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);",
        "let (workbench_custom_class, set_workbench_custom_class) = signal(false);",
        "let (workbench_rtl, set_workbench_rtl) = signal(false);",
        "title=\"Interactive Playground (Props + State + Preview)\"",
        "test_config_signal=workbench_config",
        "data-slot=\"avatar-group-workbench-controls\"",
        "data-slot=\"avatar-group-workbench-preview\"",
        "data-slot=\"avatar-group-workbench-configured\"",
        "data-slot=\"avatar-group-workbench-state\"",
        "data-slot=\"avatar-group-spec-preview-na\"",
        "id_base=\"docs-avatar-group-workbench-roster\".to_string()",
        "id_base=\"docs-avatar-group-workbench-size\".to_string()",
        "id_base=\"docs-avatar-group-workbench-max\".to_string()",
        "<Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>",
        "<Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>",
        "<Switch checked=workbench_rtl set_checked=set_workbench_rtl>",
        "expected: state={expected_state}, size={size_attr}, total={configured_total}, overflow={overflow}",
        "AI Spec input/preview linkage: N/A for AvatarGroup (non-spec component).",
    ] {
        assert!(
            docs_source.contains(required),
            "AvatarGroup interactive docs playground should include `{required}`.",
        );
    }

    for forbidden in ["ui_state_primitives::", "ui_headless::", "state=..."] {
        assert!(
            !docs_source.contains(forbidden),
            "AvatarGroup interactive docs path should not require internal wiring token `{forbidden}`.",
        );
    }
}

#[test]
fn avatar_group_source_first_docs_are_copy_paste_ready_and_traceable() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_avatar_group_contract.spec.mjs");
    let check2_source = load_source("../../components/avatar-group/check2.md");

    for required in [
        "data-slot=\"avatar-group-source-first\"",
        "data-slot=\"avatar-group-source-first-contract\"",
        "data-slot=\"avatar-group-source-prerequisites\"",
        "component-avatar-group",
        "inject-css",
        "UiRoot",
        "<Snippet",
        "text=source_first_code.get()",
        "label=\"Copy avatar-group starter\".to_string()",
        "copyable=true",
        "class_name=\"docs-avatar-group-source-copy\".to_string()",
        "data-slot=\"avatar-group-source-paths\"",
        "components/avatar-group/src/mod.rs",
        "components/avatar-group/src/logic.rs",
        "components/avatar-group/src/view.rs",
        "components/avatar-group/src/styles.rs",
        "data-slot=\"avatar-group-source-sync-note\"",
    ] {
        assert!(
            docs_source.contains(required),
            "AvatarGroup source-first docs should include `{required}`."
        );
    }

    for required in [
        "docs-app avatar-group source-first section exposes copy-ready starter and source anchors",
        "[data-slot=\"avatar-group-source-first\"]",
        "[data-slot=\"snippet\"]",
        "[data-slot=\"snippet-copy-button\"]",
        "[data-slot=\"snippet-pre\"]",
        "[data-slot=\"avatar-group-source-paths\"]",
        "[data-slot=\"avatar-group-source-sync-note\"]",
    ] {
        assert!(
            e2e_source.contains(required),
            "AvatarGroup source-first e2e contract should include `{required}`."
        );
    }

    for required in [
        "Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮",
        "文档需指向真实源码落点并说明依赖前提",
        "文档代码与当前实现必须同步",
    ] {
        assert!(
            check2_source.contains(required),
            "AvatarGroup checklist should keep source-first copy-ready marker `{required}`."
        );
    }
}

#[test]
fn avatar_group_heroui_alignment_docs_and_component_entry_are_synced() {
    let heroui_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let spectrum_heroui_source =
        load_source("../../docs/research/spectrum-heroui-style-interface-study.md");
    let pages_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_display_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let readme_source = load_source("../../components/avatar-group/src/README.md");
    let check2_source = load_source("../../components/avatar-group/check2.md");

    for required in [
        "### AvatarGroup 同步记录（2026-02-20）",
        "`AvatarGroup` 参数主轴保持 `items/max/size/aria_label/class_name/lang/dir`",
        "component_doc!(\"AvatarGroup\", \"avatar-group\", \"Display\", display::avatar_group)",
        "`#/components/avatar-group` 可索引访问",
        "`apps/docs-app/src/pages/components/pages/display.rs::avatar_group()` 覆盖 `Hello World`",
        "`Interactive Playground (Props + State + Preview)`",
        "`Source-first Starter (Copy-Paste Ready)`",
        "`component-avatar-group`、`UiRoot`、`inject-css`",
        "`components/avatar-group/src/{mod,logic,view,styles}.rs`",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。",
        "仅代码更新无文档更新在接口变更场景下不允许合入。",
    ] {
        assert!(
            heroui_source.contains(required),
            "AvatarGroup HeroUI alignment doc should include `{required}`."
        );
    }

    for required in [
        "component_doc!(",
        "\"AvatarGroup\"",
        "\"avatar-group\"",
        "display::avatar_group",
    ] {
        assert!(
            pages_registry_source.contains(required),
            "docs pages registry should keep AvatarGroup index entry via `{required}`."
        );
    }

    for required in [
        "slug=\"avatar-group\"",
        "pub(super) fn avatar_group() -> AnyView {",
        "title=\"Interactive Playground (Props + State + Preview)\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
    ] {
        assert!(
            docs_display_source.contains(required),
            "AvatarGroup docs page should keep synced docs entry token `{required}`."
        );
    }

    for required in ["#/components/avatar-group", "## docs-app 入口"] {
        assert!(
            readme_source.contains(required),
            "AvatarGroup README should expose docs-app entry token `{required}`."
        );
    }

    assert!(
        spectrum_heroui_source.contains("# Spectrum × HeroUI 样式与接口综合学习（v0）"),
        "spectrum-heroui research baseline doc should remain available."
    );

    for required in [
        "[x] HeroUI 对标文档与组件文档同步",
        "docs/spec/heroui-parameter-design-strategy.md",
        "docs/research/spectrum-heroui-style-interface-study.md",
        "仅代码更新无文档更新",
    ] {
        assert!(
            check2_source.contains(required),
            "AvatarGroup checklist should record HeroUI docs-sync contract via `{required}`."
        );
    }
}

#[test]
fn avatar_group_readme_is_beginner_friendly_with_default_path_before_advanced() {
    let readme_source = load_source("../../components/avatar-group/src/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for required in [
        "# AvatarGroup",
        "## 先用起来（Quick Start / Hello World）",
        "不需要先理解分层架构",
        "use ui::{AvatarGroup, AvatarGroupItem};",
        "<AvatarGroup items=Vec::<AvatarGroupItem>::new() />",
        "## 常见用法（Common Usage）",
        "基础头像组 + overflow",
        "自定义 aria 与 class",
        "## 默认参数（Defaults）",
        "max: Option<usize>",
        "默认 `None`，归一化为 `4`",
        "size: AvatarSize",
        "默认 `AvatarSize::Md`",
        "## 进阶（Advanced，按需使用）",
        "先用上面的 Quick Start 和 Common Usage，再按需进入这些进阶能力。",
        "## docs-app 入口",
        "/#/components/avatar-group",
    ] {
        assert!(
            readme_source.contains(required),
            "AvatarGroup README should include beginner-friendly token `{required}`.",
        );
    }

    let quick_start_pos = readme_source
        .find("## 先用起来（Quick Start / Hello World）")
        .expect("README should include quick-start section");
    let advanced_pos = readme_source
        .find("## 进阶（Advanced，按需使用）")
        .expect("README should include advanced section");
    assert!(
        quick_start_pos < advanced_pos,
        "README should present default quick-start path before advanced options."
    );

    for forbidden in [
        "ui_state_primitives::",
        "ui_headless::",
        "Signal<",
        "state=...",
    ] {
        assert!(
            !readme_source.contains(forbidden),
            "README starter path should not require internal layering token `{forbidden}`.",
        );
    }

    for required in [
        "pub(super) fn avatar_group() -> AnyView",
        "slug=\"avatar-group\"",
        "Playground title=\"Hello World\"",
    ] {
        assert!(
            docs_source.contains(required),
            "docs-app should expose discoverable AvatarGroup doc entry via `{required}`.",
        );
    }
}

#[test]
fn avatar_group_does_not_define_component_motion_runtime() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let styles_source = load_source("src/avatar/styles.rs");

    assert!(
        !manifest_dir
            .join("../../components/avatar-group/src/motion.rs")
            .exists(),
        "AvatarGroup should not define `src/avatar/motion.rs` when no runtime animation contract is needed."
    );

    for forbidden in [
        "ui_motion::",
        "request_animation_frame",
        "cancel_animation_frame",
        "SpringAnimator::new",
        "attach_motion(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "AvatarGroup should stay motion-runtime free in component layer; found `{forbidden}`."
        );
    }

    for forbidden_css in ["transition:", "animation:"] {
        assert!(
            !styles_source.contains(forbidden_css),
            "AvatarGroup styles should stay static without runtime motion marker `{forbidden_css}`."
        );
    }
}

#[test]
fn avatar_group_has_no_two_pass_geometry_rectification_pipeline_in_current_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let styles_source = load_source("src/avatar/styles.rs");

    assert!(
        !manifest_dir
            .join("../../components/avatar-group/src/motion.rs")
            .exists(),
        "AvatarGroup should not define a dedicated geometry runtime while no DOM-measure overlay contract exists."
    );

    for forbidden in [
        "getBoundingClientRect",
        "get_bounding_client_rect",
        "ResizeObserver",
        "MutationObserver",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "scrollWidth",
        "scrollHeight",
        "DOMRect",
        "Intent::Reposition",
        "Rectification",
        "tooltip",
        "popover",
        "menu-trigger",
        "placement",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "AvatarGroup should stay free of two-pass geometry marker `{forbidden}` in current scope."
        );
    }
}

#[test]
fn avatar_group_has_no_overlay_focus_stack_gc_contract_in_current_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let overlay_stack_source = load_source("../ui-headless/src/overlay_stack.rs");
    let focus_trap_source = load_source("../ui-headless/src/focus_trap.rs");
    let signature = function_signature(&view_source, "AvatarGroup");

    assert!(
        !manifest_dir
            .join("../../components/avatar-group/src/motion.rs")
            .exists(),
        "AvatarGroup should not define overlay runtime while focus-stack restoration is out of scope."
    );

    for required in [
        "items: Vec<AvatarGroupItem>",
        "logic::normalize_avatar_group_input(",
        "logic::resolve_avatar_group_render_state(",
    ] {
        assert!(
            view_source.contains(required) || signature.contains(required),
            "AvatarGroup should keep static group-render path via `{required}`."
        );
    }

    for forbidden in [
        "NodeRef",
        "document.body",
        "document().body",
        "use_focus_trap",
        "focus_trap",
        "should_restore_focus",
        "restore_focus",
        "OverlayStack",
        "overlay_stack",
        "provide_overlay_stack",
        "use_overlay_stack",
        "data-ui-overlay-portal",
        "FallbackTo",
        "focus_manager",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "AvatarGroup should remain free of overlay focus-stack/GC token `{forbidden}`."
        );
    }

    for required in [
        "pub fn provide_overlay_stack() -> OverlayStack",
        "pub fn use_overlay_stack_registration() -> OverlayRegistration",
    ] {
        assert!(
            overlay_stack_source.contains(required),
            "overlay stack primitive should stay in ui-headless via `{required}`."
        );
    }

    for required in [
        "pub struct FocusTrapOptions",
        "pub should_restore_focus: bool",
        "pub fn use_focus_trap(options: FocusTrapOptions) -> FocusTrapHandlers",
    ] {
        assert!(
            focus_trap_source.contains(required),
            "focus restoration primitive should stay in ui-headless via `{required}`."
        );
    }
}

#[test]
fn avatar_group_has_no_foreign_zone_escape_hatch_integration_in_current_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let signature = function_signature(&view_source, "AvatarGroup");

    assert!(
        manifest_dir
            .join("../../docs/spec/foreign_zone_escape_hatches.md")
            .exists(),
        "Foreign-zone governance spec should exist at docs/spec/foreign_zone_escape_hatches.md."
    );

    for required in [
        "items: Vec<AvatarGroupItem>",
        "logic::normalize_avatar_group_input(",
        "logic::resolve_avatar_group_render_state(",
    ] {
        assert!(
            view_source.contains(required) || signature.contains(required),
            "AvatarGroup should keep declarative render path via `{required}`."
        );
    }

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "google.maps",
        "mapboxgl",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "yield_control",
        "cleanup_foreign",
        "chart_instance",
        "map_instance",
        "imperative_handle",
        "js_instance",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "AvatarGroup should remain free of foreign-zone escape-hatch token `{forbidden}`."
        );
    }

    for forbidden in [
        "chart:",
        "map:",
        "chart_instance",
        "map_instance",
        "imperative_handle",
        "web_sys::",
        "wasm_bindgen::JsValue",
    ] {
        assert!(
            !signature.contains(forbidden),
            "AvatarGroup public API should not leak imperative third-party handle token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_hydration_discontinuity_contract_is_explicitly_na_without_time_or_random_id_init() {
    let mod_source = load_source("src/avatar/mod.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let root_source = load_source("src/root.rs");
    let id_provider_source = load_source("../ui-headless/src/id_provider.rs");

    for forbidden in [
        "SystemTime::now",
        "Instant::now",
        "js_sys::Date::now",
        "Date::now",
        "now(",
        "Uuid::new_v4",
        "uuid::Uuid",
        "nanoid",
        "rand::",
        "thread_rng",
        "random::<",
        "random_uuid",
        "use_id(",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "AvatarGroup should remain deterministic across SSR/hydration; forbidden entropy `{forbidden}`."
        );
    }

    assert!(
        root_source.contains("provide_ui_id_provider(id_seed);"),
        "UiRoot should keep deterministic id-provider injection entrypoint."
    );
    assert!(
        root_source.contains("#[prop(optional, default = 1)] id_seed: u64,"),
        "UiRoot should expose deterministic seed prop for hydration-stable IDs."
    );
    assert!(
        id_provider_source.contains("pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider"),
        "ui-headless should expose deterministic id provider factory."
    );
}

#[test]
fn avatar_group_ssr_cross_platform_contract_uses_explicit_cfg_and_keeps_non_wasm_clean() {
    let manifest_source = load_source("../../components/avatar-group/Cargo.toml");
    let check2_source = load_source("../../components/avatar-group/check2.md");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");
    let mod_source = load_source("src/avatar/mod.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let ui_headless_manifest_source = load_source("../ui-headless/Cargo.toml");
    let ui_headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let ui_components_manifest_source = load_source("Cargo.toml");

    for required in [
        "cargo check -p ui",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-<your_component>,inject-css",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep explicit compile-only evidence command `{required}`."
        );
    }

    for required in [
        "cargo check -p ui --no-default-features --features component-avatar_group,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-avatar_group,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            platform_script_source.contains(required),
            "platform check script should cover avatar-group cross-platform compile-only path `{required}`."
        );
    }

    assert!(
        manifest_source.contains(
            "leptos = { version = \"0.8.15\", default-features = false, features = [\"csr\"] }"
        ),
        "avatar-group should keep explicit platform behavior via feature-gated leptos dependency."
    );
    assert!(
        ui_headless_manifest_source.contains("default = [\"web\"]")
            && ui_headless_manifest_source.contains("web = [\"leptos/csr\"]")
            && ui_headless_manifest_source.contains("ssr = [\"leptos/ssr\"]"),
        "ui-headless should keep web/ssr split under explicit feature management."
    );
    assert!(
        ui_headless_lib_source.contains(
            "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")"
        ),
        "ui-headless should guard invalid web+ssr co-enable path via compile_error."
    );
    assert!(
        ui_components_manifest_source
            .contains("[target.'cfg(target_arch = \"wasm32\")'.dependencies]")
            && ui_components_manifest_source.contains("web-sys = { version = \"0.3.85\""),
        "wasm-only browser dependency should stay behind explicit target cfg in ui."
    );

    for forbidden in [
        "web_sys::",
        "web-sys",
        "window.",
        "document.",
        "HtmlElement",
        "NodeRef",
        "js_sys::",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "non-wasm avatar-group source should stay browser-object free; found `{forbidden}`."
        );
    }

    for forbidden in [
        "cfg!(target_arch = \"wasm32\")",
        "cfg!(feature = \"ssr\")",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(feature = \"ssr\")]",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "avatar-group should avoid accidental platform split marker `{forbidden}` in component layer."
        );
    }
}

#[test]
fn avatar_group_ui_headless_web_ssr_feature_mutex_contract_is_enforced() {
    let check2_source = load_source("../../components/avatar-group/check2.md");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");
    let ui_headless_manifest_source = load_source("../ui-headless/Cargo.toml");
    let ui_headless_lib_source = load_source("../ui-headless/src/lib.rs");

    assert!(
        check2_source.contains("`ui-headless` web/ssr feature 互斥受 `compile_error!` 保护"),
        "checklist should explicitly track ui-headless web/ssr feature mutex contract."
    );

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            ui_headless_lib_source.contains(required),
            "ui-headless lib.rs should keep feature-mutex guard `{required}`."
        );
    }

    for required in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            ui_headless_manifest_source.contains(required),
            "ui-headless manifest should preserve split feature mapping `{required}`."
        );
    }

    for required in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "[platform] compile guard: ui-headless web+ssr must fail",
        "if cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "rg -n \"mutually exclusive\" \"$MUTEX_LOG\"",
    ] {
        assert!(
            platform_script_source.contains(required),
            "platform check script should enforce ui-headless web/ssr mutex via `{required}`."
        );
    }
}

#[test]
fn avatar_group_ui_motion_non_wasm_noop_stub_contract_is_enforced() {
    let check2_source = load_source("../../components/avatar-group/check2.md");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");
    let ui_motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let ui_motion_non_wasm_stub_test_source = load_source("../ui-motion/tests/non_wasm_stub.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");

    assert!(
        check2_source.contains("`ui-motion` 非 wasm 提供 no-op/stub"),
        "checklist should explicitly track ui-motion non-wasm no-op/stub contract."
    );

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(required),
            "ui-motion should keep non-wasm stub capability via `{required}`."
        );
    }

    for required in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "assert!(web::prefers_reduced_motion());",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
        "web::animate(&(), &keyframes, MotionOptions::default());",
    ] {
        assert!(
            ui_motion_non_wasm_stub_test_source.contains(required),
            "ui-motion non-wasm stub regression should cover `{required}`."
        );
    }

    for required in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(required),
            "platform script should protect ui-motion no-op/stub contract via `{required}`."
        );
    }

    for forbidden in ["ui_motion::", "attach_motion(", "SpringAnimator::new"] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "AvatarGroup should not assume runtime motion handles via `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_reduced_motion_ssr_wasm_branch_contract_is_explicitly_na_and_consistent() {
    let check2_source = load_source("../../components/avatar-group/check2.md");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");
    let ui_motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let ui_motion_non_wasm_stub_test_source = load_source("../ui-motion/tests/non_wasm_stub.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");

    assert!(
        check2_source.contains("组件实现覆盖 `reduced-motion` / SSR / wasm 分支"),
        "checklist should explicitly track reduced-motion/SSR/wasm branch contract."
    );

    for required in [
        "cargo check -p ui --no-default-features --features component-avatar_group,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-avatar_group,inject-css",
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
        "button_copy_reduced_motion_ssr_wasm_branches_are_covered_via_button_contract",
        "time_field_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
        "scroll_area_reduced_motion_ssr_wasm_contract_is_consistent",
    ] {
        assert!(
            platform_script_source.contains(required),
            "platform script should lock reduced-motion/SSR/wasm coverage via `{required}`."
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(required),
            "ui-motion should keep non-wasm reduced-motion safe fallback `{required}`."
        );
    }

    for required in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
    ] {
        assert!(
            ui_motion_non_wasm_stub_test_source.contains(required),
            "ui-motion non-wasm stub test should cover `{required}`."
        );
    }

    for forbidden in [
        "ui_motion::",
        "attach_motion(",
        "request_animation_frame",
        "animation:",
        "transition:",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "cfg!(target_arch = \"wasm32\")",
        "cfg!(feature = \"ssr\")",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "AvatarGroup should keep branch-neutral semantics without motion runtime token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let check2_source = load_source("../../components/avatar-group/check2.md");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");

    for required in [
        "\"avatar-group\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "max_update_ms: Some(8.0),",
        "max_heap_kb: Some(384.0),",
    ] {
        assert!(
            shell_source.contains(required),
            "docs component shell should define avatar-group performance budget via `{required}`."
        );
    }

    for required in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
    ] {
        assert!(
            perf_probe_source.contains(required),
            "UiPerfProbe should expose machine-readable perf marker `{required}`."
        );
    }

    for required in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_e2e_source.contains(required),
            "docs coverage should keep repeatable perf guard `{required}`."
        );
    }

    for required in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
        "Button",
        "Input",
    ] {
        assert!(
            check2_source.contains(required),
            "avatar-group checklist should preserve performance-governance marker `{required}`."
        );
    }

    for required in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(required),
            "performance follow-up plan should keep `{required}`."
        );
    }

    for required in [
        "cargo test -p ui --test avatar_group_semantics --no-default-features --features component-avatar_group,inject-css avatar_group_performance_governance_budget_is_defined_and_blocking",
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(required),
            "performance gate script should include `{required}`."
        );
    }

    for required in [
        "logic::resolve_avatar_group_render_state(logic::AvatarGroupStateInput {",
        "data-state=state.visual_state.as_str()",
        "data-visible-count=state.visible_count.to_string()",
        "data-overflow-count=state.overflow_count.to_string()",
        "<Show when=move || state.visual_state.has_overflow()>",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup view should expose render/state attribution marker `{required}`."
        );
    }

    for forbidden in [
        "request_animation_frame",
        "set_interval(",
        "while ",
        "loop {",
    ] {
        assert!(
            !view_source.contains(forbidden) && !styles_source.contains(forbidden),
            "AvatarGroup should avoid uncontrolled runtime perf hazard `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_e2e_selectors_are_semantic_and_wasm_waits_are_stable() {
    let e2e_source = load_source("../../e2e/tests/docs_app_avatar_group_contract.spec.mjs");
    let check2_source = load_source("../../components/avatar-group/check2.md");

    for required in [
        "docs-app avatar-group uses semantic selectors with wasm-stable ready waits",
        "docs-app avatar-group keeps streaming/snapshot semantics readable and async-motion path explicitly N/A",
        "docs-app avatar-group key flow is repeatable with semantic checkpoints",
        "docs-app avatar-group interactive playground updates semantic state markers with live controls",
        "docs-app avatar-group source-first section exposes copy-ready starter and source anchors",
        "overflowBeforeReload",
        "overflowAfterReload",
        "await page.reload();",
        "toHaveAttribute(\"data-ui-state\", \"overflow\")",
        "/#/components/avatar-group",
        "body:not(:has(#boot))",
        "[data-component=\"avatar-group\"][data-slot=\"avatar-group\"]",
        "[data-slot=\"avatar-group\"][data-ui-schema=\"ui.avatar-group.agent.v1\"]",
        "[data-slot=\"avatar-group\"][data-state=\"overflow\"][data-has-overflow=\"true\"]",
        "[data-slot=\"avatar-group-overflow\"]",
        "[data-slot=\"avatar-group-item\"]",
        "[data-slot=\"avatar-group-streaming-policy\"]",
        "[data-slot=\"avatar-group-copy-ready-hint\"]",
        "[data-slot=\"avatar-group-source-first\"]",
        "[data-slot=\"snippet-copy-button\"]",
        "[data-slot=\"avatar-group-source-paths\"]",
        "[data-slot=\"avatar-group-source-sync-note\"]",
        "[data-slot=\"avatar-group-workbench-controls\"]",
        "[data-slot=\"avatar-group-workbench-configured\"] [data-slot=\"avatar-group\"]",
        "[data-slot=\"avatar-group-workbench-state\"]",
        "[data-slot=\"avatar-group-spec-preview-na\"]",
        "toHaveCount(0)",
    ] {
        assert!(
            e2e_source.contains(required),
            "AvatarGroup e2e contract should keep semantic selector/wait marker `{required}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "section.playground",
        ".docs-page-title",
        "getByText(",
        "nth-child(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "AvatarGroup e2e contract should avoid fragile selector/wait token `{forbidden}`."
        );
    }

    for required in [
        "E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记",
        "语义状态就绪而非固定 sleep",
        "关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程",
        "回归失败需可定位到具体语义契约断点",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(required),
            "avatar-group checklist should keep e2e selector stability marker `{required}`."
        );
    }
}

#[test]
fn avatar_group_view_macro_complexity_is_bounded_and_semantically_split_for_items() {
    let view_source = load_source("src/avatar/view.rs");

    assert!(
        view_source.contains("view! {"),
        "AvatarGroup view should keep explicit leptos render macro entry."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        2,
        "AvatarGroup should keep macro expansion bounded to root + item semantic fragments."
    );
    assert!(
        view_source.lines().count() <= 220,
        "AvatarGroup view.rs should stay compact; split semantic subrenders if it grows significantly."
    );

    for required in [
        "data-slot=\"avatar-group\"",
        "data-slot=\"avatar-group-item\"",
        "data-slot=\"avatar-group-overflow\"",
        ".map(|(index, item)| render_avatar_group_item(index, item, state.size))",
        "<Show when=move || state.visual_state.has_overflow()>",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup should preserve semantic block split marker `{required}`."
        );
    }

    for forbidden in [
        "for item in",
        "match children",
        "<header",
        "<footer",
        "<article",
        "<section",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup should avoid heavy/expansion-prone view token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn avatar_group_prefers_functional_fragment_split_over_extra_component_defs() {
    let view_source = load_source("src/avatar/view.rs");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "AvatarGroup should keep a single public `#[component]` entry and avoid local component noise."
    );

    for required in [
        "fn render_avatar_group_item(",
        "index: usize,",
        "item: AvatarGroupItem,",
        "size: AvatarSize,",
        ") -> impl IntoView {",
        ".map(|(index, item)| render_avatar_group_item(index, item, state.size))",
        "data-slot=\"avatar-group-item\"",
        "class_name=\"ui-avatar-group__avatar\"",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup should preserve functional fragment split marker `{required}`."
        );
    }

    for forbidden in [
        "#[component]\nfn render_avatar_group_item(",
        "#[component]\r\nfn render_avatar_group_item(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup item fragment should stay a plain function, not a nested component `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_static_fragment_constantization_is_centralized_and_scope_bounded() {
    let view_source = load_source("src/avatar/view.rs");

    for required in [
        "const OVERFLOW_VISIBLE_LABEL_PREFIX: &str = \"+\";",
        "fn render_avatar_group_overflow_label(overflow_count: usize) -> String {",
        "format!(\"{OVERFLOW_VISIBLE_LABEL_PREFIX}{overflow_count}\")",
        "let overflow_label = render_avatar_group_overflow_label(state.overflow_count);",
        "aria-label=overflow_aria_label.clone()",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup should centralize static overflow template path via `{required}`."
        );
    }

    for forbidden in [
        "format!(\"+{}\", state.overflow_count)",
        "<svg",
        "<footer",
        "inner_html=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup should avoid scattered heavy static fragment token `{forbidden}` in current scope."
        );
    }
}

#[test]
fn avatar_group_inner_html_contract_is_explicitly_na_and_user_input_injection_free() {
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let signature = function_signature(&view_source, "AvatarGroup");

    for forbidden in [
        "inner_html=",
        "dangerouslySetInnerHTML",
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "set_inner_html",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !signature.contains(forbidden),
            "AvatarGroup should remain free of unsafe HTML injection token `{forbidden}`."
        );
    }

    for required in [
        "let i18n = i18n::use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "aria-label=group_a11y.aria_label",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup should keep semantic text via typed/i18n path `{required}` instead of raw html injection."
        );
    }
}

#[test]
fn avatar_group_wasm_debug_contract_is_explicitly_na_and_feature_isolation_clean() {
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let signature = function_signature(&view_source, "AvatarGroup");
    let avatar_group_manifest = load_source("../../components/avatar-group/Cargo.toml");
    let ui_components_manifest = load_source("Cargo.toml");

    for forbidden in [
        "tracing::",
        "trace!(",
        "debug!(",
        "console::",
        "console_log",
        "record_event",
        "event_log",
        "transition_log",
        "replay",
        "timeline",
        "devtools",
        "cfg(debug_assertions)",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !signature.contains(forbidden),
            "AvatarGroup should remain free of wasm debug/replay token `{forbidden}` in current scope."
        );
    }

    for required in [
        "default = []",
        "[features]",
        "component-avatar_group",
        "accordion-wasm-debug",
    ] {
        assert!(
            avatar_group_manifest.contains(required)
                || ui_components_manifest.contains(required)
                || view_source.contains(required)
                || signature.contains(required),
            "AvatarGroup wasm-debug N/A baseline should remain explicit via `{required}`."
        );
    }

    for forbidden_feature in [
        "avatar-group-wasm-debug",
        "avatar_group-wasm-debug",
        "component-avatar_group-wasm-debug",
    ] {
        assert!(
            !avatar_group_manifest.contains(forbidden_feature)
                && !ui_components_manifest.contains(forbidden_feature),
            "AvatarGroup should not leak production-facing debug feature `{forbidden_feature}`."
        );
    }
}

#[test]
fn avatar_group_dx_contract_prefers_playground_isolation_and_fast_style_feedback() {
    let view_source = load_source("src/avatar/view.rs");
    let signature = function_signature(&view_source, "AvatarGroup");
    let docs_display_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let docs_playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let dev_docs_script_source = load_source("../../scripts/dev-docs-app.sh");
    let dev_web_script_source = load_source("../../scripts/dev-web-demo.sh");

    for required in [
        "exec trunk serve --open true \"$@\"",
        "cd \"$ROOT_DIR/apps/docs-app\"",
        "cd \"$ROOT_DIR/apps/web-demo\"",
    ] {
        assert!(
            dev_docs_script_source.contains(required) || dev_web_script_source.contains(required),
            "dev scripts should preserve fast feedback loop via `{required}`."
        );
    }

    for required in [
        "title=\"Hello World\" code_signal=hello_code",
        "title=\"Overflow Stack\" code_signal=overflow_code",
        "title=\"Custom Aria + Class\" code_signal=custom_code",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled (N/A)\"",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "<AvatarGroup items=empty_items.clone() />",
        "data-slot=\"avatar-group-streaming-policy\"",
        "data-slot=\"avatar-group-copy-ready-hint\"",
        "data-slot=\"avatar-group-source-first\"",
        "label=\"Copy avatar-group starter\".to_string()",
        "data-slot=\"avatar-group-source-prerequisites\"",
        "data-slot=\"avatar-group-source-paths\"",
    ] {
        assert!(
            docs_display_source.contains(required),
            "AvatarGroup docs should provide isolated demo/workbench entry via `{required}`."
        );
    }

    for required in [
        "<section class=section_class id=anchor_id data-slot=\"playground\">",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "data-playground-scope=scope_id.clone()",
        "let (test_css, set_test_css) = signal(default_test_css.get_untracked());",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "code_imports: Option<String>",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "prop:value=move || test_css.get()",
        "\"Restore original CSS\"",
    ] {
        assert!(
            docs_playground_source.contains(required),
            "docs playground should keep scoped style editing and context-preserving panel via `{required}`."
        );
    }

    for forbidden in [
        "signal(",
        "create_signal",
        "RwSignal<",
        "on:click",
        "on:input",
    ] {
        assert!(
            !view_source.contains(forbidden) && !signature.contains(forbidden),
            "AvatarGroup itself should remain stateless display component in current scope; found `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_engineering_unification_contract_is_explicitly_na_and_runtime_agnostic() {
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let avatar_group_manifest = load_source("../../components/avatar-group/Cargo.toml");
    let ui_components_manifest = load_source("Cargo.toml");
    let signature = function_signature(&view_source, "AvatarGroup");

    for forbidden in [
        "serde::",
        "Serialize",
        "Deserialize",
        "serde_json::",
        "tracing::",
        "trace!(",
        "tokio::",
        "async_std::",
        "async-std",
        "Runtime",
        "JoinHandle",
        "spawn_local",
        "async fn",
        ".await",
        "Future<",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !avatar_group_manifest.contains(forbidden)
                && !signature.contains(forbidden),
            "AvatarGroup should remain runtime-agnostic and avoid per-component infra token `{forbidden}`."
        );
    }

    for required in [
        "component-avatar_group = [\"component-avatar\", \"dep:ui-avatar-group\"]",
        "component-button = [\"dep:serde\", \"dep:serde_json\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "default = []",
    ] {
        assert!(
            ui_components_manifest.contains(required) || avatar_group_manifest.contains(required),
            "engineering unification baseline should stay centralized via `{required}`."
        );
    }

    for forbidden in [
        "component-avatar_group = [\"component-avatar\", \"dep:ui-avatar-group\", \"dep:serde\"]",
        "component-avatar_group = [\"component-avatar\", \"dep:ui-avatar-group\", \"dep:serde_json\"]",
        "component-avatar_group = [\"component-avatar\", \"dep:ui-avatar-group\", \"dep:tracing\"]",
    ] {
        assert!(
            !ui_components_manifest.contains(forbidden),
            "AvatarGroup feature chain should not leak infra coupling `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_has_no_dynamic_registration_protocol_in_current_scope() {
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let signature = function_signature(&view_source, "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        ".enumerate()",
        "data-index=index",
    ] {
        assert!(
            view_source.contains(required) || signature.contains(required),
            "AvatarGroup should preserve deterministic ordering from typed Vec input via `{required}`."
        );
    }

    for forbidden in [
        "RegistrationContext",
        "register_item",
        "unregister_item",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "BTreeSet",
        "IndexSet",
        "roving",
        "focus_next",
        "focus_prev",
        "tabs",
        "accordion",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "AvatarGroup should remain free of collection-registration protocol token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_has_no_slot_projection_lifecycle_protocol_in_current_scope() {
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let signature = function_signature(&view_source, "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        ".take(state.visible_count)",
        "<Show when=move || state.visual_state.has_overflow()>",
    ] {
        assert!(
            view_source.contains(required) || signature.contains(required),
            "AvatarGroup should keep direct eager rendering path via `{required}`."
        );
    }

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "on_hidden",
        "on_shown",
        "suspend_polling",
        "resume_polling",
        "pause_animation",
        "resume_animation",
        "set_interval",
        "set_timeout",
        "request_animation_frame",
        "cancel_animation_frame",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "AvatarGroup should remain free of slot-projection lifecycle token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_has_no_env_stream_subscription_pipeline_in_current_scope() {
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let signature = function_signature(&view_source, "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        "logic::normalize_avatar_group_input(",
        "logic::resolve_avatar_group_render_state(",
    ] {
        assert!(
            view_source.contains(required) || signature.contains(required),
            "AvatarGroup should keep static prop-driven derivation path via `{required}`."
        );
    }

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "match_media",
        "matchMedia",
        "BreakpointChanged",
        "ThemeChanged",
        "IntersectionChanged",
        "Action::BreakpointChanged",
        "Action::ThemeChanged",
        "Action::IntersectionChanged",
        "debounce",
        "throttle",
        "on:resize",
        "window.add_event_listener",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "AvatarGroup should remain free of env-stream token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_has_no_event_light_cone_batch_protocol_in_current_scope() {
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let signature = function_signature(&view_source, "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        ".take(state.visible_count)",
        ".enumerate()",
    ] {
        assert!(
            view_source.contains(required) || signature.contains(required),
            "AvatarGroup should keep direct list rendering path via `{required}`."
        );
    }

    for forbidden in [
        "ContextBus",
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "select_all",
        "select_none",
        "selected_rows",
        "selected_columns",
        "row_selection",
        "column_selection",
        "Table",
        "Grid",
        "prop_drilling",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "AvatarGroup should remain free of event-light-cone token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_has_no_causality_bus_trace_pipeline_in_current_scope() {
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let signature = function_signature(&view_source, "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        "logic::normalize_avatar_group_input(",
        "logic::resolve_avatar_group_render_state(",
    ] {
        assert!(
            view_source.contains(required) || signature.contains(required),
            "AvatarGroup should keep direct local derivation path via `{required}`."
        );
    }

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "Causality Bus",
        "CommandBus",
        "EventBus",
        "publish(",
        "broadcast(",
        "subscribe(",
        "subscriber",
        "dispatch_command",
        "derived_command",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "AvatarGroup should remain free of causality-bus token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_theme_contract_consumes_ui_variables_only() {
    let styles_source = load_source("src/avatar/styles.rs");
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");

    for required_var in [
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
        "var(--ui-accent, var(--ui-fallback-accent))",
    ] {
        assert!(
            styles_source.contains(required_var),
            "AvatarGroup styles should consume ui-theme css variables via `{required_var}`."
        );
    }

    for forbidden in [
        "Theme::",
        "ThemeContext",
        "theme_to_css_variables",
        "ui_theme::",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "AvatarGroup component layer must not rebuild theme context; found `{forbidden}`."
        );
    }

    assert!(
        !styles_source.contains("--avatar-group-"),
        "AvatarGroup should not introduce private non-`--ui-*` token namespace."
    );
}

#[test]
fn avatar_group_stays_as_ui_components_assembly_layer_without_platform_leakage() {
    let mod_source = load_source("src/avatar/mod.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let lib_source = load_source("src/lib.rs");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::{AvatarGroup, AvatarGroupItem};",
    ] {
        assert!(
            mod_source.contains(required),
            "AvatarGroup module boundary should include `{required}`."
        );
    }

    for required in [
        "pub use ui_state_primitives::avatar_group::{",
        "pub fn compose_avatar_group_class_name(",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup logic should stay in assembly role and include `{required}`."
        );
    }

    for forbidden in ["view! {", "data-slot=", "labeled_group_attrs("] {
        assert!(
            !logic_source.contains(forbidden),
            "AvatarGroup logic must not carry view/headless wiring `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "logic::resolve_avatar_group_render_state(",
        "logic::compose_avatar_group_class_name(",
        "labeled_group_attrs(",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup view should compose logic + headless contract via `{required}`."
        );
    }

    for forbidden in [
        "pub struct AvatarGroupState {",
        "ui_state_primitives::avatar_group::AvatarGroupState {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup view must not reimplement primitives; found `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("var(--ui-"),
        "AvatarGroup styles should be token-first and consume `--ui-*` variables."
    );

    assert!(
        lib_source.contains("AvatarGroup") && lib_source.contains("AvatarGroupItem"),
        "ui public API should expose stable AvatarGroup exports."
    );

    for forbidden in [
        "pub use web_sys::",
        "pub use wasm_bindgen",
        "pub use leptos::html::",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui public API should not leak platform detail `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_public_api_naming_contract_is_stable_and_prefix_ready() {
    let view_source = load_source("src/avatar/view.rs");
    let sig = function_signature(&view_source, "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        "max: Option<usize>",
        "size: AvatarSize",
        "aria_label: Option<String>",
        "class_name: Option<String>",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
    ] {
        assert!(
            sig.contains(required),
            "AvatarGroup public API should keep stable prop naming `{required}`."
        );
    }

    assert!(
        !sig.contains(": bool"),
        "AvatarGroup currently has no public boolean props; future booleans must use `is_*`."
    );
    assert!(
        !sig.contains("on_"),
        "AvatarGroup currently has no public callbacks; future callbacks must use `on_*`."
    );
    assert!(
        !sig.contains("default_"),
        "AvatarGroup currently has no public default-value props; future defaults must use `default_*`."
    );
}

#[test]
fn avatar_group_composition_api_uses_typed_item_specs_and_rejects_parallel_arrays() {
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let sig = function_signature(&view_source, "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        "let fields = logic::normalize_avatar_group_item_fields(item.name, item.src, item.alt);",
        ".map(|(index, item)| render_avatar_group_item(index, item, state.size))",
        "data-slot=\"avatar-group-item\"",
    ] {
        assert!(
            view_source.contains(required) || sig.contains(required),
            "AvatarGroup should bind title/semantics/content in one typed item dimension via `{required}`."
        );
    }

    for forbidden in [
        "labels: Vec<",
        "titles: Vec<",
        "children: Vec<",
        "labels=",
        "titles=",
        "titles + panels",
        "labels + children",
    ] {
        assert!(
            !sig.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "AvatarGroup should reject parallel-array composition token `{forbidden}`."
        );
    }

    for required in ["<AvatarGroup", "items=vec![", "AvatarGroupItem {"] {
        assert!(
            docs_source.contains(required),
            "AvatarGroup docs should keep typed ItemSpec composition sample via `{required}`."
        );
    }
}

#[test]
fn avatar_group_has_no_controllable_state_axis_and_no_half_controlled_api() {
    let view_source = load_source("src/avatar/view.rs");
    let sig = function_signature(&view_source, "AvatarGroup");

    for forbidden in [" value:", "default_", "on_value_change", "on_open_change"] {
        assert!(
            !sig.contains(forbidden),
            "AvatarGroup should not expose partial controllable API marker `{forbidden}` without full value/on_change/default pair."
        );
    }

    for forbidden in [
        "use_controllable_state(",
        "use_controllable_open_state_traced(",
        "on_value_change",
        "on_open_change",
        "default_value",
        "default_open",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup has no controllable state axis and should not include `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_defaults_are_centralized_in_logic() {
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");

    for required in [
        "pub fn normalize_avatar_group_input(",
        "pub fn normalize_avatar_group_item_fields(",
        "pub fn resolve_avatar_group_aria_label_with_fallback(",
        "name: name.unwrap_or_default()",
        "src: src.unwrap_or_default()",
        "alt: alt.unwrap_or_default()",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup logic should centralize default normalization via `{required}`."
        );
    }

    for forbidden in ["unwrap_or_default()", "logic::normalize_optional_text("] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup view should not perform fallback normalization directly; found `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_state_primitive_source_boundary_is_enforced() {
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/avatar_group.rs");
    let sig = function_signature(&view_source, "AvatarGroup");

    for required in [
        "pub use ui_state_primitives::avatar_group::{",
        "AvatarGroupStateInput",
        "AvatarGroupRenderState",
        "resolve_render_state",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup logic should source state primitives from ui-state-primitives via `{required}`."
        );
    }

    for required in [
        "pub enum AvatarGroupVisualState",
        "pub enum AvatarGroupAriaLabelSource",
        "pub enum AvatarGroupClassSource",
        "pub struct AvatarGroupRenderState",
        "pub fn resolve_render_state(",
    ] {
        assert!(
            primitive_source.contains(required),
            "AvatarGroup render-state primitives should be implemented in ui-state-primitives; missing `{required}`."
        );
    }

    for forbidden in [
        "use crate::store::",
        "use crate::state::",
        "global_store",
        "app_store",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "AvatarGroup component layer should not bind business store directly; found `{forbidden}`."
        );
    }

    for forbidden in ["RwSignal<", "ReadSignal<", "WriteSignal<", "Signal<"] {
        assert!(
            !sig.contains(forbidden),
            "AvatarGroup public API should not expose framework/store state container `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_state_normalization_is_centralized_in_logic() {
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");

    for required in [
        "pub struct AvatarGroupNormalizedInput",
        "pub struct AvatarGroupItemFields",
        "AvatarGroupRenderState",
        "pub fn normalize_avatar_group_input(",
        "pub fn normalize_avatar_group_item_fields(",
        "resolve_render_state",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup logic should type and derive group/item render state via `{required}`."
        );
    }

    for forbidden in [
        "pub enum AvatarGroupVisualState {",
        "pub enum AvatarGroupAriaLabelSource {",
        "pub enum AvatarGroupClassSource {",
        "pub struct AvatarGroupRenderState {",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "AvatarGroup logic should consume render-state primitives from ui-state-primitives; found local `{forbidden}`."
        );
    }

    for required in [
        "let normalized = logic::normalize_avatar_group_input(",
        "let state = logic::resolve_avatar_group_render_state(logic::AvatarGroupStateInput {",
        "let fields = logic::normalize_avatar_group_item_fields(item.name, item.src, item.alt);",
        "<Show when=move || state.visual_state.has_overflow()>",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup view should consume centralized state and only render by markers via `{required}`."
        );
    }

    for forbidden in [
        "data-state=if",
        "if items.len()",
        "if state.total_count",
        "if state.overflow_count",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup view should not rebuild root state machine branches; found `{forbidden}`."
        );
    }

    for required in [
        ".ui-avatar-group[data-state=\"overflow\"]",
        ".ui-avatar-group[data-state=\"empty\"]",
        ".ui-avatar-group[data-has-overflow=\"true\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "AvatarGroup styles should consume explicit state markers via `{required}`."
        );
    }
}

#[test]
fn avatar_group_state_markers_are_observable_and_closed_set_contracts() {
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let primitive_source = load_source("../ui-state-primitives/src/avatar_group.rs");

    for required in [
        "data-slot=\"avatar-group\"",
        "data-state=state.visual_state.as_str()",
        "data-empty=state.visual_state.is_empty().then_some(\"true\")",
        "data-has-overflow=state.visual_state.has_overflow().then_some(\"true\")",
        "data-aria-label-source=state.aria_label_source.as_str()",
        "data-class-source=state.class_source.as_str()",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label",
        "data-slot=\"avatar-group-item\"",
        "data-slot=\"avatar-group-overflow\"",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup markers should stay observable via `{required}`."
        );
    }

    for required in [
        "pub enum AvatarGroupVisualState",
        "Self::Stable => \"stable\"",
        "Self::Overflow => \"overflow\"",
        "Self::Empty => \"empty\"",
        "pub enum AvatarGroupAriaLabelSource",
        "Self::Default => \"default\"",
        "Self::Custom => \"custom\"",
        "pub enum AvatarGroupClassSource",
    ] {
        assert!(
            primitive_source.contains(required),
            "AvatarGroup marker values should come from enum closed set via `{required}`."
        );
    }

    for required in [
        ".ui-avatar-group[data-state=\"overflow\"]",
        ".ui-avatar-group[data-state=\"empty\"]",
        ".ui-avatar-group[data-aria-label-source=\"custom\"]",
        ".ui-avatar-group[data-class-source=\"custom\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "AvatarGroup selectors should be queryable from semantic markers via `{required}`."
        );
    }

    for forbidden in [
        "data-state=format!",
        "data-state=if",
        "data-aria-label-source=format!",
        ".ui-avatar-group:nth-child(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !styles_source.contains(forbidden),
            "AvatarGroup marker contract should avoid free-text or DOM-order selector pattern `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let styles_source = load_source("src/avatar/styles.rs");
    let view_source = load_source("src/avatar/view.rs");

    for required in [
        ".ui-avatar-group[data-state=\"stable\"]",
        ".ui-avatar-group[data-state=\"overflow\"]",
        ".ui-avatar-group[data-state=\"empty\"]",
        ".ui-avatar-group[data-has-overflow=\"true\"]",
        ".ui-avatar-group[data-empty=\"true\"]",
        ".ui-avatar-group[data-aria-label-source=\"default\"]",
        ".ui-avatar-group[data-aria-label-source=\"custom\"]",
        ".ui-avatar-group[data-class-source=\"custom\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "AvatarGroup state styles should rely on explicit markers via `{required}`."
        );
    }

    for forbidden in [
        ".ui-avatar-group:nth-child(",
        ".ui-avatar-group:nth-of-type(",
        ".ui-avatar-group > * > * > *",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "AvatarGroup state styling should not guess from fragile DOM selector `{forbidden}`."
        );
    }

    assert!(
        !view_source.contains("style="),
        "AvatarGroup runtime should not inject business style logic inline."
    );
}

#[test]
fn avatar_group_semantics_suite_prioritizes_contract_assertions_over_snapshots() {
    let suite_source = load_source("tests/avatar_group_semantics.rs");

    for required in [
        "fn avatar_group_emits_baseline_style_root_data_attributes()",
        "fn avatar_group_a11y_i18n_l10n_contract_is_headless_driven_and_no_view_hardcoded_copy()",
        "fn avatar_group_state_markers_are_observable_and_closed_set_contracts()",
        "fn avatar_group_docs_parameter_and_state_matrix_match_logic_defaults()",
        "fn avatar_group_docs_interactive_playground_supports_live_prop_controls_and_preview_feedback()",
        "fn avatar_group_source_first_docs_are_copy_paste_ready_and_traceable()",
        "fn avatar_group_heroui_alignment_docs_and_component_entry_are_synced()",
        "fn avatar_group_readme_is_beginner_friendly_with_default_path_before_advanced()",
        "fn avatar_group_styles_use_defensive_variable_fallback_chains()",
        "fn avatar_group_cascade_layer_and_runtime_style_contract_is_enforced()",
        "fn avatar_group_styles_depend_on_explicit_state_markers_not_dom_guessing()",
        "fn avatar_group_state_normalization_is_centralized_in_logic()",
        "fn avatar_group_has_no_controllable_state_axis_and_no_half_controlled_api()",
        "fn avatar_group_has_no_async_loading_protocol_and_keeps_sync_render_contract()",
        "fn avatar_group_does_not_define_component_motion_runtime()",
        "fn avatar_group_has_no_two_pass_geometry_rectification_pipeline_in_current_scope()",
        "fn avatar_group_has_no_overlay_focus_stack_gc_contract_in_current_scope()",
        "fn avatar_group_has_no_foreign_zone_escape_hatch_integration_in_current_scope()",
        "fn avatar_group_hydration_discontinuity_contract_is_explicitly_na_without_time_or_random_id_init()",
        "fn avatar_group_ssr_cross_platform_contract_uses_explicit_cfg_and_keeps_non_wasm_clean()",
        "fn avatar_group_ui_headless_web_ssr_feature_mutex_contract_is_enforced()",
        "fn avatar_group_ui_motion_non_wasm_noop_stub_contract_is_enforced()",
        "fn avatar_group_reduced_motion_ssr_wasm_branch_contract_is_explicitly_na_and_consistent()",
        "fn avatar_group_performance_governance_budget_is_defined_and_blocking()",
        "fn avatar_group_e2e_selectors_are_semantic_and_wasm_waits_are_stable()",
        "fn avatar_group_view_macro_complexity_is_bounded_and_semantically_split_for_items()",
        "fn avatar_group_prefers_functional_fragment_split_over_extra_component_defs()",
        "fn avatar_group_static_fragment_constantization_is_centralized_and_scope_bounded()",
        "fn avatar_group_inner_html_contract_is_explicitly_na_and_user_input_injection_free()",
        "fn avatar_group_wasm_debug_contract_is_explicitly_na_and_feature_isolation_clean()",
        "fn avatar_group_dx_contract_prefers_playground_isolation_and_fast_style_feedback()",
        "fn avatar_group_engineering_unification_contract_is_explicitly_na_and_runtime_agnostic()",
        "fn avatar_group_has_no_dynamic_registration_protocol_in_current_scope()",
        "fn avatar_group_has_no_slot_projection_lifecycle_protocol_in_current_scope()",
        "fn avatar_group_has_no_env_stream_subscription_pipeline_in_current_scope()",
        "fn avatar_group_has_no_event_light_cone_batch_protocol_in_current_scope()",
        "fn avatar_group_has_no_causality_bus_trace_pipeline_in_current_scope()",
        "fn avatar_group_ui_components_entrypoints_and_headless_boundaries_are_correct()",
        "fn avatar_group_component_directory_standard_file_layout_is_correct()",
        "fn avatar_group_context_compression_manifest_and_rbi_are_present_and_consistent()",
        "fn avatar_group_agent_contract_schema_is_typed_traceable_and_whitelisted()",
        "fn avatar_group_streaming_definition_is_llm_output_only_with_two_modes()",
        "fn avatar_group_snapshot_base_capability_accepts_complete_configuration()",
        "fn avatar_group_streaming_policy_is_optional_with_snapshot_fallback_and_explicit_output_status()",
        "fn avatar_group_rust_hygiene_disallows_unwrap_expect_let_underscore_and_uses_cow()",
        "fn avatar_group_tree_shaking_contract_enforces_source_mode_reachability_boundaries()",
    ] {
        assert!(
            suite_source.contains(required),
            "AvatarGroup semantics suite should prioritize contract coverage via `{required}`."
        );
    }

    let forbidden_tokens = [
        format!("{}{}", "assert_", "snapshot!"),
        format!("{}{}", "assert_debug_", "snapshot!"),
        format!("{}{}", "assert_json_", "snapshot!"),
        format!("{}{}", "to_match_", "snapshot"),
        format!("{}{}", "ins", "ta::"),
        format!("{}{}", ".", "snap"),
        format!("{}{}", "gol", "den"),
        format!("{}{}", "pi", "xel"),
        format!("{}{}", "screen", "shot"),
    ];

    for forbidden in forbidden_tokens {
        assert!(
            !suite_source.contains(&forbidden),
            "AvatarGroup semantics suite should not depend on snapshot token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_component_files_follow_layered_responsibilities() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mod_source = load_source("src/avatar/mod.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let view_source = load_source("src/avatar/view.rs");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::{AvatarGroup, AvatarGroupItem};",
    ] {
        assert!(
            mod_source.contains(required),
            "AvatarGroup `mod.rs` should keep minimal export boundary via `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "pub fn normalize_avatar_group_input(",
        "pub const CSS:",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "AvatarGroup `mod.rs` should not carry implementation detail `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_avatar_group_input(",
        "pub fn normalize_avatar_group_item_fields(",
        "pub fn resolve_avatar_group_aria_label_with_fallback(",
        "pub fn compose_avatar_group_class_name(",
        "resolve_avatar_group_render_state(",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup `logic.rs` should keep normalization/derivation helpers via `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "data-slot=",
        "role=",
        "aria-label=",
        ".ui-avatar-group",
        "labeled_group_attrs(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "AvatarGroup `logic.rs` should not mix view/css/headless detail `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-avatar-group[data-state=\"overflow\"]",
        ".ui-avatar-group[data-aria-label-source=\"custom\"]",
        "var(--ui-",
    ] {
        assert!(
            styles_source.contains(required),
            "AvatarGroup `styles.rs` should keep token-first static CSS via `{required}`."
        );
    }

    for forbidden in ["view! {", "on:error", "labeled_group_attrs(", "logic::"] {
        assert!(
            !styles_source.contains(forbidden),
            "AvatarGroup `styles.rs` should not carry runtime/view logic `{forbidden}`."
        );
    }

    for required in [
        "use crate::logic::{self, AvatarSize};",
        "view! {",
        "logic::normalize_avatar_group_input(",
        "logic::resolve_avatar_group_render_state(",
        "logic::compose_avatar_group_class_name(",
        "labeled_group_attrs(",
        "<Avatar",
        "data-slot=\"avatar-group\"",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup `view.rs` should render structure, mount headless contract, and reuse Avatar via `{required}`."
        );
    }

    for forbidden in [
        "pub const CSS",
        "pub struct AvatarGroupState {",
        "pub enum AvatarGroupVisualState {",
        "ui_motion::",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup `view.rs` should not carry styles/primitive redefinition/motion engine detail `{forbidden}`."
        );
    }

    assert!(
        !manifest_dir
            .join("../../components/avatar-group/src/motion.rs")
            .exists(),
        "AvatarGroup is static in current scope; `motion.rs` should remain absent until motion contract is required."
    );
}

#[test]
fn avatar_group_component_directory_standard_file_layout_is_correct() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_src = manifest_dir.join("../../components/avatar-group/src");
    let mod_source = load_source("src/avatar/mod.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let view_source = load_source("src/avatar/view.rs");
    let check2_source = load_source("../../components/avatar-group/check2.md");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs"] {
        assert!(
            component_src.join(required).exists(),
            "AvatarGroup component directory should include required file `{required}`."
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !component_src.join(forbidden).exists(),
            "AvatarGroup should not introduce `{forbidden}` in current scope."
        );
    }

    assert!(
        !component_src.join("motion.rs").exists(),
        "AvatarGroup is non-interactive in current scope; `motion.rs` remains N/A and should stay absent."
    );

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::{AvatarGroup, AvatarGroupItem};",
    ] {
        assert!(
            mod_source.contains(required),
            "AvatarGroup `mod.rs` should keep minimal stable exports via `{required}`."
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "pub mod motion",
        "mod render;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "AvatarGroup `mod.rs` should avoid over-export/render.rs drift token `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_avatar_group_input(",
        "pub fn normalize_avatar_group_item_fields(",
        "pub fn compose_avatar_group_class_name(",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup `logic.rs` should keep normalization/derivation responsibilities via `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "data-slot=",
        "role=",
        "aria-",
        "labeled_group_attrs(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "AvatarGroup `logic.rs` should not carry view/headless mount details `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-avatar-group[data-state=\"overflow\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "AvatarGroup `styles.rs` should keep token-first static css contract `{required}`."
        );
    }

    for forbidden in ["view! {", "logic::", "labeled_group_attrs(", "data-slot="] {
        assert!(
            !styles_source.contains(forbidden),
            "AvatarGroup `styles.rs` should avoid runtime/view coupling token `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "logic::normalize_avatar_group_input(",
        "logic::resolve_avatar_group_render_state(",
        "labeled_group_attrs(",
        "data-slot=\"avatar-group\"",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup `view.rs` should keep structure render + headless mount via `{required}`."
        );
    }

    for forbidden in ["mod render;", "include!(\"render.rs\")"] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup `view.rs` should not drift to render.rs include pattern `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("组件目录标准文件落点正确。"),
        "AvatarGroup checklist should track component-directory standard file-layout contract."
    );
}

#[test]
fn avatar_group_context_compression_manifest_and_rbi_are_present_and_consistent() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_src = manifest_dir.join("../../components/avatar-group/src");
    let component_manifest_source = load_source("src/avatar/Component.toml");
    let component_rbi_source = load_source("src/avatar/avatar_group.rbi");
    let check2_source = load_source("../../components/avatar-group/check2.md");

    for required_file in ["Component.toml", "avatar_group.rbi"] {
        assert!(
            component_src.join(required_file).exists(),
            "AvatarGroup context-compression file should exist: `{required_file}`."
        );
    }

    for required in [
        "schema_version = \"1\"",
        "name = \"AvatarGroup\"",
        "crate = \"ui-avatar-group\"",
        "name = \"items\"",
        "name = \"max\"",
        "name = \"size\"",
        "name = \"aria_label\"",
        "name = \"class_name\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest_source.contains(required),
            "AvatarGroup Component.toml should keep context-compression marker `{required}`."
        );
    }

    for required in [
        "pub struct AvatarGroupItem {",
        "pub fn AvatarGroup(",
        "dir: Option<ui_headless::A11yDirection>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            component_rbi_source.contains(required),
            "avatar_group.rbi should keep signature-projection marker `{required}`."
        );
    }

    for required in [
        "上下文压缩协议（Manifest + RBI）",
        "avatar_group_context_compression_manifest_and_rbi_are_present_and_consistent",
        "avatar_group_context_compression_manifest_and_rbi_are_present_and_consistent_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "avatar-group check2 should keep context-compression marker `{required}`."
        );
    }
}

#[test]
fn avatar_group_ui_components_entrypoints_and_headless_boundaries_are_correct() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable_state_source =
        load_source("../ui-headless/src/controllable_state.rs");
    let headless_presence_source = load_source("../ui-headless/src/presence.rs");
    let headless_a11y_source = load_source("../ui-headless/src/a11y.rs");
    let check2_source = load_source("../../components/avatar-group/check2.md");

    for required in [
        "#[cfg(feature = \"component-avatar_group\")]",
        "pub use ui_avatar_group as avatar_group;",
        "pub use root::UiRoot;",
    ] {
        assert!(
            lib_source.contains(required),
            "ui lib.rs should keep stable feature-gated entrypoint `{required}`."
        );
    }

    for forbidden in [
        "pub use web_sys::",
        "pub use wasm_bindgen::",
        "pub use leptos::html::",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui lib.rs should not leak platform detail `{forbidden}`."
        );
    }

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-avatar_group\")]",
        "out.push_str(crate::avatar_group::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_source.contains(required),
            "ui css.rs should keep centralized feature-gated css aggregation `{required}`."
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
            "ui root.rs should keep centralized theme/i18n/css injection boundary `{required}`."
        );
    }

    for required in [
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringConfig",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "active_highlight primitive should keep shared highlight capability `{required}`."
        );
    }

    for forbidden in ["AvatarGroup", "Accordion", "business"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight primitive should avoid component-business semantic coupling `{forbidden}`."
        );
    }

    for relative in [
        "../../crates/ui/src/overlay_open.rs",
        "../../crates/ui/src/presence.rs",
        "../../crates/ui/src/a11y.rs",
    ] {
        assert!(
            !manifest_dir.join(relative).exists(),
            "ui should not define `{relative}`; primitive must stay in ui-headless."
        );
    }

    for required in [
        "pub fn use_controllable_state<T>(",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            headless_controllable_state_source.contains(required)
                || headless_presence_source.contains(required)
                || headless_a11y_source.contains(required),
            "headless primitive boundary should provide `{required}`."
        );
    }

    for required in [
        "- [x] `ui` 固定入口文件落点正确。",
        "avatar_group_ui_components_entrypoints_and_headless_boundaries_are_correct",
        "avatar_group_ui_components_entrypoints_and_headless_boundaries_are_correct_local",
    ] {
        assert!(
            check2_source.contains(required),
            "avatar-group checklist should keep fixed-entrypoint evidence `{required}`."
        );
    }
}

#[test]
fn avatar_group_does_not_introduce_spec_rs_and_keeps_lightweight_exports() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let group_mod_source = load_source("src/avatar/mod.rs");

    assert!(
        !manifest_dir
            .join("../../components/avatar-group/src/spec.rs")
            .exists(),
        "AvatarGroup should not introduce `src/avatar/spec.rs` without stable external schema need."
    );

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::{AvatarGroup, AvatarGroupItem};",
    ] {
        assert!(
            group_mod_source.contains(required),
            "AvatarGroup exports should remain minimal via `{required}`."
        );
    }

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !group_mod_source.contains(forbidden),
            "AvatarGroup `mod.rs` should stay assembly-only and avoid spec wiring `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_token_first_static_styles_contract_is_enforced_via_ui_root_css_pipeline() {
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");

    for required in [
        "#[cfg(feature = \"component-avatar_group\")]",
        "out.push_str(crate::avatar_group::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "Component CSS aggregation should include avatar-group styles via `{required}`."
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should be the CSS injection boundary via `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-accent-soft, var(--ui-fallback-accent-soft))",
        "var(--ui-accent, var(--ui-fallback-accent))",
    ] {
        assert!(
            styles_source.contains(required),
            "AvatarGroup styles should stay token-first and static via `{required}`."
        );
    }

    for forbidden in [
        "--avatar-group-",
        "@apply",
        "tailwind",
        "styled(",
        "emotion",
        "stylex",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "AvatarGroup styles should not introduce private-token or CSS-in-Rust utility marker `{forbidden}`."
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"items-",
        "class=\"gap-",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup view should not depend on utility-first class contract `{forbidden}`."
        );
    }

    for forbidden in ["style=", "style:"] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "AvatarGroup runtime should not carry inline business style logic `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_visual_desire_gate_reuses_theme_visual_baseline_and_heroui_contracts() {
    let baseline_page =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let e2e_spec = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_doc = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            baseline_page.contains(needle),
            "Theme visual baseline page should keep visual-quality contract token `{needle}`."
        );
    }

    for needle in ["\"ThemeVisualBaseline\"", "\"theme-visual-baseline\""] {
        assert!(
            pages_registry.contains(needle),
            "Docs pages registry should expose theme visual baseline route token `{needle}`.",
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
    ] {
        assert!(
            e2e_spec.contains(needle),
            "Theme visual baseline e2e contract should include `{needle}`."
        );
    }

    for needle in ["# HeroUI 参数设计风格对齐策略", "HeroUI 对齐结论"] {
        assert!(
            heroui_doc.contains(needle),
            "HeroUI strategy doc should keep alignment contract token `{needle}`.",
        );
    }
}

#[test]
fn avatar_group_tree_shaking_contract_enforces_source_mode_reachability_boundaries() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let cargo_source = load_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-avatar\")]\npub use ui_avatar as avatar;"),
        "avatar module should stay behind component-avatar gate for source-mode reachability."
    );

    for needle in [
        "component-avatar = [\"dep:ui-avatar\"]",
        "component-avatar_group = [\"component-avatar\", \"dep:ui-avatar-group\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "avatar-group feature relationship should remain explicit via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-avatar\")]\n    out.push_str(crate::avatar::styles::CSS);",
        "#[cfg(feature = \"component-avatar_group\")]\n    out.push_str(crate::avatar_group::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "css aggregation should keep source-mode reachability bounded by `{needle}`."
        );
    }

    for forbidden in [
        "static ALL_COMPONENTS",
        "const ALL_COMPONENTS",
        "HashMap<&'static str, fn",
    ] {
        assert!(
            !lib_source.contains(forbidden) && !css_source.contains(forbidden),
            "global registry pattern that defeats DCE should stay absent `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_machine_readable_contract_uses_typed_inputs_and_semantic_markers() {
    let primitive_source = load_source("../ui-state-primitives/src/avatar_group.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let logic_test_source = load_source("../../components/avatar-group/test/logic.rs");

    for required in [
        "pub struct AvatarGroupStateInput",
        "pub enum AvatarGroupVisualState",
        "pub enum AvatarGroupAriaLabelSource",
        "pub enum AvatarGroupClassSource",
        "pub struct AvatarGroupRenderState",
        "pub fn resolve_render_state(",
        "pub fn as_str(self) -> &'static str",
    ] {
        assert!(
            primitive_source.contains(required),
            "AvatarGroup machine-readable input/state should stay typed in primitives via `{required}`."
        );
    }

    for required in [
        "pub use ui_state_primitives::avatar_group::{",
        "AvatarGroupStateInput",
        "AvatarGroupRenderState",
        "resolve_render_state",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup logic should consume typed primitives via `{required}`."
        );
    }

    for forbidden in [
        "data-state=format!",
        "data-aria-label-source=format!",
        "data-class-source=format!",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup should avoid string-protocol state leakage `{forbidden}`."
        );
    }

    for required in [
        "data-state=state.visual_state.as_str()",
        "data-empty=state.visual_state.is_empty().then_some(\"true\")",
        "data-has-overflow=state.visual_state.has_overflow().then_some(\"true\")",
        "data-aria-label-source=state.aria_label_source.as_str()",
        "data-class-source=state.class_source.as_str()",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup should expose machine-readable semantic markers via `{required}`."
        );
    }

    for required in [
        ".ui-avatar-group[data-state=\"overflow\"]",
        ".ui-avatar-group[data-state=\"empty\"]",
        ".ui-avatar-group[data-aria-label-source=\"custom\"]",
        ".ui-avatar-group[data-class-source=\"custom\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "AvatarGroup style contracts should consume stable semantic marker `{required}`."
        );
    }

    let required = "resolve_render_state_maps_discrete_status_and_sources_to_enums";
    assert!(
        logic_source.contains(required)
            || primitive_source.contains(required)
            || logic_test_source.contains(
                "resolve_avatar_group_render_state_maps_discrete_status_and_sources_to_enums",
            ),
        "Typed state contract should keep a regression anchor `{required}`."
    );
}

#[test]
fn avatar_group_agent_contract_schema_is_typed_traceable_and_whitelisted() {
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let component_manifest_source = load_source("src/avatar/Component.toml");
    let check2_source = load_source("../../components/avatar-group/check2.md");

    for required in [
        "pub const AVATAR_GROUP_AGENT_SCHEMA: &str = \"ui.avatar-group.agent.v1\";",
        "pub enum AvatarGroupAgentIntent",
        "pub enum AvatarGroupAgentAction",
        "pub enum AvatarGroupAgentStateAxis",
        "pub enum AvatarGroupAgentSourceAxis",
        "pub struct AvatarGroupAgentContract",
        "pub fn resolve_avatar_group_agent_state_axis(",
        "pub fn resolve_avatar_group_agent_source_axis(",
        "pub fn resolve_avatar_group_agent_contract(",
        "intent: AvatarGroupAgentIntent::DisplayIdentityCollection",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup logic should keep typed agent-contract marker `{required}`."
        );
    }

    for required in [
        "let agent_contract = logic::resolve_avatar_group_agent_contract(state);",
        "data-ui-schema=agent_contract.schema",
        "data-ui-intent=agent_contract.intent.as_str()",
        "data-ui-action=agent_contract.action.as_str()",
        "data-ui-state=agent_contract.state.as_str()",
        "data-ui-source=agent_contract.source.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup view should mount typed agent-contract marker `{required}`."
        );
    }

    for forbidden in [
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
        "data-ui-schema=format!(",
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "AvatarGroup agent-contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }

    for required in [
        "name = \"agent_contract_schema_markers\"",
        "name = \"whitelist_render_policy_no_script_injection\"",
    ] {
        assert!(
            component_manifest_source.contains(required),
            "AvatarGroup Component.toml should keep agent-contract capability `{required}`."
        );
    }

    for required in [
        "语义标记统一升级为 Agent Contract（Schema 化）",
        "avatar_group_agent_contract_schema_is_typed_traceable_and_whitelisted",
        "avatar_group_agent_contract_schema_is_typed_traceable_and_whitelisted_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "avatar-group check2 should keep agent-contract marker `{required}`."
        );
    }
}

#[test]
fn avatar_group_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("../../components/avatar-group/check2.md");
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let component_manifest_source = load_source("src/avatar/Component.toml");
    let component_rbi_source = load_source("src/avatar/avatar_group.rbi");

    for required in [
        "流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "avatar_group_streaming_definition_is_llm_output_only_with_two_modes",
        "avatar_group_streaming_definition_is_llm_output_only_with_two_modes_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "avatar-group check2 should keep streaming-definition marker `{required}`."
        );
    }

    for forbidden in [
        "AiSpace",
        "AiRenderMode",
        "is_streaming",
        "on_stream",
        "token_delta",
        "streaming_state",
        "data-ui-stream-mode",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !component_manifest_source.contains(forbidden)
                && !component_rbi_source.contains(forbidden),
            "AvatarGroup is snapshot-only in current scope; streaming protocol token `{forbidden}` should stay absent."
        );
    }
}

#[test]
fn avatar_group_streaming_policy_is_optional_with_snapshot_fallback_and_explicit_output_status() {
    let check2_source = load_source("../../components/avatar-group/check2.md");
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let component_manifest_source = load_source("src/avatar/Component.toml");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "avatar_group_streaming_policy_is_optional_with_snapshot_fallback_and_explicit_output_status",
        "avatar_group_streaming_policy_is_optional_with_snapshot_fallback_and_explicit_output_status_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "avatar-group check2 should keep streaming-optional policy marker `{required}`."
        );
    }

    for required in [
        "pub enum AvatarGroupAgentStreamSupport {",
        "Required,",
        "Optional,",
        "Self::Required => \"required\"",
        "Self::Optional => \"optional\"",
        "pub enum AvatarGroupAgentStreamFallback {",
        "Self::Snapshot => \"snapshot\"",
        "pub enum AvatarGroupAgentOutputStatus {",
        "Draft,",
        "Verified,",
        "Submittable,",
        "Self::Draft => \"draft\"",
        "Self::Verified => \"verified\"",
        "Self::Submittable => \"submittable\"",
        "pub stream_support: AvatarGroupAgentStreamSupport,",
        "pub stream_fallback: AvatarGroupAgentStreamFallback,",
        "pub output_status: AvatarGroupAgentOutputStatus,",
        "stream_support: AvatarGroupAgentStreamSupport::Optional,",
        "stream_fallback: AvatarGroupAgentStreamFallback::Snapshot,",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup logic should keep typed streaming-policy marker `{required}`."
        );
    }

    for required in [
        "data-slot=\"avatar-group\"",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label",
        "data-ui-stream-support=agent_contract.stream_support.as_str()",
        "data-ui-stream-fallback=agent_contract.stream_fallback.as_str()",
        "data-ui-output-status=agent_contract.output_status.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup view should expose streaming-policy semantic marker `{required}`."
        );
    }

    let required = "name = \"streaming_optional_with_snapshot_fallback_and_output_status_markers\"";
    assert!(
        component_manifest_source.contains(required),
        "AvatarGroup Component.toml should declare streaming-policy capability `{required}`."
    );

    for forbidden in ["retry", "reconnect", "validation_error", "transport_error"] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "AvatarGroup should keep upper-layer resilience out of component scope; found `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_rust_hygiene_disallows_unwrap_expect_let_underscore_and_uses_cow() {
    let check2_source = load_source("../../components/avatar-group/check2.md");
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");

    for forbidden in [
        ".unwrap(",
        ".unwrap_err(",
        ".expect(",
        "let _ =",
        ".to_owned(",
        "String::from(",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "AvatarGroup non-test source should satisfy rust-hygiene and avoid `{forbidden}`."
        );
    }

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![",
        "Cow::Borrowed(\"ui-avatar-group\")",
        "Cow::Owned(format!(\"ui-avatar-group--size-{}\", state.size_attr))",
        ".map(Cow::into_owned)",
        ".collect::<Vec<_>>()",
        ".join(\" \")",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup logic should use Cow-based class assembly marker `{required}`."
        );
    }

    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "avatar_group_rust_hygiene_disallows_unwrap_expect_let_underscore_and_uses_cow",
        "avatar_group_rust_hygiene_disallows_unwrap_expect_let_underscore_and_uses_cow_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "avatar-group check2 should keep rust-hygiene marker `{required}`."
        );
    }
}

#[test]
fn avatar_group_snapshot_base_capability_accepts_complete_configuration() {
    let check2_source = load_source("../../components/avatar-group/check2.md");
    let view_source = load_source("src/avatar/view.rs");
    let component_manifest_source = load_source("src/avatar/Component.toml");
    let component_rbi_source = load_source("src/avatar/avatar_group.rbi");

    for required in [
        "`Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "avatar_group_snapshot_base_capability_accepts_complete_configuration",
        "avatar_group_snapshot_base_capability_accepts_complete_configuration_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "avatar-group check2 should keep snapshot-base marker `{required}`."
        );
    }

    for required in [
        "pub fn AvatarGroup(",
        "items: Vec<AvatarGroupItem>",
        "#[prop(optional)] max: Option<usize>",
        "#[prop(optional)] size: AvatarSize",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let normalized = logic::normalize_avatar_group_input(",
        "let state = logic::resolve_avatar_group_render_state(logic::AvatarGroupStateInput {",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup snapshot render path should support complete configuration marker `{required}`."
        );
    }

    for required in [
        "name = \"items\"",
        "name = \"max\"",
        "name = \"size\"",
        "name = \"aria_label\"",
        "name = \"class_name\"",
        "name = \"lang\"",
        "name = \"dir\"",
    ] {
        assert!(
            component_manifest_source.contains(required),
            "AvatarGroup Component.toml should expose snapshot-complete input marker `{required}`."
        );
    }

    for required in [
        "pub fn AvatarGroup(",
        "items: Vec<AvatarGroupItem>",
        "max: Option<usize>",
        "size: ui_avatar::AvatarSize",
        "aria_label: Option<String>",
        "class_name: Option<String>",
        "lang: Option<String>",
        "dir: Option<ui_headless::A11yDirection>",
    ] {
        assert!(
            component_rbi_source.contains(required),
            "avatar_group.rbi should project snapshot-complete signature marker `{required}`."
        );
    }

    for forbidden in [
        "AiSpace",
        "AiRenderMode",
        "is_streaming",
        "on_stream",
        "token_delta",
        "streaming_state",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !component_manifest_source.contains(forbidden)
                && !component_rbi_source.contains(forbidden),
            "AvatarGroup snapshot base capability should avoid streaming-only token `{forbidden}`."
        );
    }
}
