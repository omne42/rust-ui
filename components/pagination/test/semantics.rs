use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(suffix) = rel_path.strip_prefix("src/pagination/") {
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/pagination/src/{suffix}"));
        return fs::read_to_string(&migrated)
            .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let direct = manifest_dir.join(rel_path);
    if direct.exists() {
        return true;
    }

    if let Some(suffix) = rel_path.strip_prefix("src/pagination/") {
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        return workspace_dir
            .join(format!("components/pagination/src/{suffix}"))
            .exists();
    }

    false
}

#[test]
fn pagination_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/pagination/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Pagination internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn pagination_uses_state_model_for_navigation_and_root_attrs() {
    let view_source = load_source("src/pagination/view.rs");
    let logic_source = load_source("src/pagination/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/pagination.rs");

    for needle in [
        "let view_state = Signal::derive(move || {",
        "logic::resolve_pagination_view_state(",
        "let state = Signal::derive(move || view_state.get().state);",
        "data-slot=\"pagination\"",
        "data-disabled=is_disabled.then_some(\"true\")",
        "data-page-control=move || view_state.get().control_mode.as_data_attr()",
        "data-empty=(total_pages == 0).then_some(\"true\")",
        "data-page=move || state.get().current_page.to_string()",
        "data-total-pages=total_pages.to_string()",
        "data-single-page=move ||",
    ] {
        assert!(
            view_source.contains(needle),
            "Pagination should wire `{needle}` to expose stable baseline-style root states."
        );
    }

    for needle in [
        "pub use ui_state_primitives::pagination::{",
        "resolve_pagination_range",
        "resolve_pagination_state",
        "PaginationPageControlMode",
        "resolve_pagination_view_state",
        "resolve_prev_page_target",
    ] {
        assert!(
            logic_source.contains(needle),
            "Pagination logic should bridge primitive `{needle}` from ui-state-primitives."
        );
    }

    for needle in [
        "pub struct PaginationState",
        "pub is_empty: bool",
        "pub is_prev_disabled: bool",
        "pub is_next_disabled: bool",
        "pub enum PaginationPageControlMode",
        "pub struct PaginationViewState",
        "pub fn resolve_pagination_view_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Pagination primitives should define `{needle}` in ui-state-primitives."
        );
    }
}

#[test]
fn pagination_prev_next_buttons_expose_slots_and_disabled_state() {
    let source = load_source("src/pagination/view.rs");

    for needle in [
        "i18n::use_ui_i18n()",
        "i18n.strings::<PaginationStrings>()",
        "data-slot=\"pagination-prev\"",
        "data-slot=\"pagination-next\"",
        "let prev_page_label: String = strings.previous_page_aria_label.as_ref().into();",
        "let next_page_label: String = strings.next_page_aria_label.as_ref().into();",
        "aria_label=prev_page_label.clone()",
        "aria_label=next_page_label.clone()",
        "let is_prev_disabled = state.get().is_prev_disabled;",
        "let is_next_disabled = state.get().is_next_disabled;",
        "is_disabled=is_prev_disabled",
        "is_disabled=is_next_disabled",
        "data-disabled=is_prev_disabled.then_some(\"true\")",
        "data-disabled=is_next_disabled.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Pagination prev/next controls should keep `{needle}` for baseline-compatible semantics and styling."
        );
    }
}

#[test]
fn pagination_uses_headless_navigation_attrs_and_locale_contract() {
    let source = load_source("src/pagination/view.rs");

    for needle in [
        "use ui_headless::{A11yDirection, OnPress, navigation_attrs};",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "let nav_a11y = navigation_attrs(",
        "logic::normalize_optional_text(lang)",
        "aria-label=nav_aria_label",
        "lang=nav_lang.clone()",
        "dir=nav_dir",
    ] {
        assert!(
            source.contains(needle),
            "Pagination should keep headless a11y contract marker `{needle}`."
        );
    }
}

#[test]
fn pagination_motion_contract_is_component_mapped_and_token_first() {
    let mod_source = load_source("src/pagination/mod.rs");
    let view_source = load_source("src/pagination/view.rs");
    let motion_source = load_source("src/pagination/motion.rs");
    let styles_source = load_source("src/pagination/styles.rs");

    for needle in [
        "mod motion;",
        "pub use motion::PaginationMotion;",
        "#[prop(optional)] motion: PaginationMotion,",
        "let motion = motion::sanitize_motion(motion);",
        "let motion_source = motion::source_attr(motion);",
        "let style_vars = motion::attach_motion(None, motion);",
        "style=style_vars",
        "data-motion-source=motion_source",
    ] {
        assert!(
            mod_source.contains(needle) || view_source.contains(needle),
            "Pagination motion contract should include `{needle}`."
        );
    }

    for needle in [
        "default_text_field_motion_tokens()",
        "ui_motion::web::prefers_reduced_motion()",
        "--ui-pagination-motion-duration",
        "--ui-pagination-motion-easing",
    ] {
        assert!(
            motion_source.contains(needle) || styles_source.contains(needle),
            "Pagination motion implementation should keep `{needle}`."
        );
    }
}

#[test]
fn pagination_theme_contract_consumes_ui_theme_tokens() {
    let styles_source = load_source("src/pagination/styles.rs");
    let motion_source = load_source("src/pagination/motion.rs");

    for needle in [
        "var(--ui-space-xs)",
        "var(--ui-radius-md)",
        "var(--ui-fg-muted)",
        "--ui-pagination-motion-duration: var(",
        "--ui-text-field-motion-duration",
        "--ui-fallback-text-field-motion-duration",
        "--ui-pagination-motion-easing: var(",
        "--ui-text-field-motion-easing",
    ] {
        assert!(
            styles_source.contains(needle),
            "Pagination styles should consume theme token variable `{needle}`."
        );
    }

    for needle in [
        "use ui_theme::default_text_field_motion_tokens;",
        "let tokens = default_text_field_motion_tokens();",
    ] {
        assert!(
            motion_source.contains(needle),
            "Pagination motion defaults should map from ui-theme token source `{needle}`."
        );
    }
}

#[test]
fn pagination_items_expose_page_dots_and_current_states() {
    let source = load_source("src/pagination/view.rs");

    for needle in [
        "let slot = if page_number.is_some()",
        "\"pagination-page\"",
        "\"pagination-dots\"",
        "data-slot=slot",
        "data-slot=\"pagination-dots-label\"",
        "aria-current=aria_current",
        "data-page=page_number.map(|value| value.to_string())",
        "data-current=move || is_current().then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Pagination items should expose `{needle}` for regression-safe page/dots/current rendering."
        );
    }
}

#[test]
fn pagination_on_press_guards_disabled_and_duplicate_navigation() {
    let source = load_source("src/pagination/view.rs");

    for needle in [
        "logic::resolve_prev_page_target(current_view_state)",
        "logic::resolve_next_page_target(current_view_state)",
        "logic::resolve_direct_page_target(current_view_state, p)",
        "logic::should_sync_uncontrolled_page(current_view_state.control_mode)",
    ] {
        assert!(
            source.contains(needle),
            "Pagination should guard `{needle}` so callbacks don't fire for blocked/no-op interactions."
        );
    }
}

#[test]
fn pagination_public_bool_prop_uses_is_prefix() {
    let source = load_source("src/pagination/view.rs");

    assert!(
        source.contains("#[prop(optional)] is_disabled: bool,"),
        "Pagination public bool prop should follow `is_*` naming contract."
    );
    assert!(
        !source.contains("#[prop(optional)] disabled: bool,"),
        "Legacy bool prop name `disabled` should not remain in public API."
    );
}

#[test]
fn pagination_page_axis_supports_controlled_and_uncontrolled_contract() {
    let view_source = load_source("src/pagination/view.rs");
    let logic_source = load_source("src/pagination/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/pagination.rs");

    for needle in [
        "#[prop(optional, into)] page: Option<ReadSignal<usize>>,",
        "#[prop(optional, into)] default_page: Option<usize>,",
        "#[prop(optional, into)] on_page_change: Option<Callback<usize>>,",
        "let resolved_default_page = logic::resolve_default_page(default_page);",
        "let (uncontrolled_page, set_uncontrolled_page) =",
        "signal(resolved_default_page);",
        "data-page-control=move || view_state.get().control_mode.as_data_attr()",
        "let view_state = Signal::derive(move || {",
        "logic::resolve_pagination_view_state(",
        "set_uncontrolled_page.set(next);",
    ] {
        assert!(
            view_source.contains(needle),
            "Pagination controlled/uncontrolled contract should include `{needle}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::pagination::{",
        "PaginationPageControlMode",
        "resolve_default_page",
        "resolve_pagination_view_state",
        "resolve_prev_page_target",
        "resolve_next_page_target",
        "resolve_direct_page_target",
        "should_sync_uncontrolled_page",
    ] {
        assert!(
            logic_source.contains(needle),
            "Pagination logic should bridge controlled/uncontrolled primitive `{needle}`."
        );
    }

    for needle in [
        "pub const DEFAULT_PAGE: usize = 1;",
        "pub enum PaginationPageControlMode",
        "pub fn resolve_default_page(default_page: Option<usize>) -> usize",
        "pub fn normalize_default_page(default_page: usize) -> usize",
        "pub fn resolve_page_control_mode(controlled_page: Option<usize>)",
        "pub fn resolve_effective_page(controlled_page: Option<usize>, uncontrolled_page: usize)",
        "pub fn resolve_prev_page_target(view_state: PaginationViewState) -> Option<usize>",
        "pub fn resolve_next_page_target(view_state: PaginationViewState) -> Option<usize>",
        "pub fn resolve_direct_page_target(",
        "pub fn should_sync_uncontrolled_page(control_mode: PaginationPageControlMode) -> bool",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Pagination primitives should define controlled/uncontrolled primitive `{needle}`."
        );
    }
}

#[test]
fn pagination_discrete_states_are_type_modeled() {
    let view_source = load_source("src/pagination/view.rs");
    let logic_source = load_source("src/pagination/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/pagination.rs");

    for needle in [
        "pub enum PaginationPageControlMode",
        "pub struct PaginationViewState",
        "pub control_mode: PaginationPageControlMode,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "match item {",
        "PaginationItem::Page(value)",
        "PaginationItem::Dots",
    ] {
        assert!(
            logic_source.contains(needle)
                || view_source.contains(needle)
                || primitive_source.contains(needle),
            "Pagination discrete states should be type constrained via `{needle}`."
        );
    }

    for forbidden in [
        "mode: Option<String>",
        "status: Option<String>",
        "variant: Option<String>",
        "size: Option<String>",
        "mode: String",
        "status: String",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Pagination should avoid free-form discrete input `{forbidden}`."
        );
    }
}

#[test]
fn pagination_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "pub(super) fn pagination() -> AnyView",
        "title=\"Pagination\"",
        "slug=\"pagination\"",
        "description=\"Pagination control with display/config/code/css-test/state-matrix playgrounds and baseline-style state attrs.\"",
        "<Playground title=\"展示 Display\" code_signal=display_code>",
        "<Playground title=\"Config 配置对比\" code_signal=config_code>",
        "<Playground title=\"Code 代码示例\" code_signal=code_example>",
        "<Playground title=\"CSS Test\" code_signal=css_test_code>",
        "<Playground title=\"状态对比 State Matrix\" code_signal=states_code>",
        "data-slot=\"pagination-display-playground\"",
        "data-slot=\"pagination-config-playground\"",
        "data-slot=\"pagination-code-playground\"",
        "data-slot=\"pagination-css-test-playground\"",
        "data-slot=\"pagination-states-playground\"",
        "<Pagination",
        "on_page_change=on_page_change",
        "class_name=\"docs-pagination-custom\".to_string()",
        "is_disabled=true",
        "total_pages=0",
    ] {
        assert!(
            source.contains(needle),
            "collections docs page should include `{needle}` for pagination coverage.",
        );
    }
}

#[test]
fn pagination_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "let (page, set_page) = signal(1_usize);",
        "let (last_change, set_last_change) = signal(None::<usize>);",
        "let on_page_change = Callback::new(move |next: usize| set_last_change.set(Some(next)));",
        "let (compact_page, set_compact_page) = signal(8_usize);",
        "let (wide_page, set_wide_page) = signal(8_usize);",
        "let (code_page, set_code_page) = signal(3_usize);",
        "let (css_page, set_css_page) = signal(5_usize);",
        "let (first_page, set_first_page) = signal(1_usize);",
        "let (middle_page, set_middle_page) = signal(6_usize);",
        "let (last_page, set_last_page) = signal(12_usize);",
        "total_pages=12",
        "siblings=1",
        "boundaries=1",
        "\"page: \"",
        "\"last change: \"",
        "\"compact config (siblings=0 boundaries=1): \"",
        "\"wide config (siblings=2 boundaries=2): \"",
        "\"code sample page: \"",
        "\"custom class: docs-pagination-custom\"",
        "\"first page: \"",
        "\"middle page: \"",
        "\"last page: \"",
        "let (disabled_page, set_disabled_page) = signal(1_usize);",
        "let (empty_page, set_empty_page) = signal(1_usize);",
        "total_pages=1",
        "page=disabled_page",
        "on_page_change=on_disabled_page_change",
        "\"disabled page: \"",
        "total_pages=0",
        "page=empty_page",
        "on_page_change=on_empty_page_change",
        "\"empty page signal: \"",
    ] {
        assert!(
            source.contains(needle),
            "pagination docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn pagination_docs_include_readme_or_equivalent_entry() {
    let has_readme = path_exists("src/pagination/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    assert!(
        has_readme || docs_source.contains("pub(super) fn pagination() -> AnyView"),
        "Pagination should provide README or equivalent docs-app page."
    );
}

#[test]
fn pagination_readme_covers_display_config_code_css_and_state_comparison_sections() {
    let source = load_source("src/pagination/README.md");

    for needle in [
        "## 展示区（docs-app）",
        "展示 Display",
        "Config 配置对比",
        "Code 代码示例",
        "CSS Test",
        "状态对比 State Matrix",
    ] {
        assert!(
            source.contains(needle),
            "pagination README should include `{needle}` section for docs-playground parity."
        );
    }
}

#[test]
fn pagination_e2e_contract_uses_semantic_selectors_and_stable_waits() {
    let source = load_source("../../e2e/tests/docs_app_pagination_contract.spec.mjs");

    for needle in [
        "body:not(:has(#boot))",
        "data-slot=\"pagination-display-playground\"",
        "data-slot=\"pagination-state-disabled\"",
        "data-slot=\"pagination-state-empty\"",
        "data-slot=\"pagination-css-test-custom\"",
        "data-slot=\"pagination\"",
    ] {
        assert!(
            source.contains(needle),
            "pagination e2e contract should include semantic marker `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout", "setTimeout", "nth-child("] {
        assert!(
            !source.contains(forbidden),
            "pagination e2e contract should avoid unstable token `{forbidden}`."
        );
    }
}

#[test]
fn pagination_check2_is_marked_complete() {
    let source = load_source("src/pagination/check2.md");
    assert!(
        !source.contains("- [ ]"),
        "pagination/check2.md should not keep unchecked checklist items after completion."
    );
}
