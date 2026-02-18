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

    for needle in [
        "resolve_pagination_state",
        "data-slot=\"pagination\"",
        "data-disabled=disabled.then_some(\"true\")",
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
        "pub struct PaginationState",
        "pub is_empty: bool",
        "pub is_prev_disabled: bool",
        "pub is_next_disabled: bool",
    ] {
        assert!(
            logic_source.contains(needle),
            "Pagination logic should define `{needle}` for centralized state derivation."
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
        "let prev_page_label = strings.previous_page_aria_label.as_ref().to_string();",
        "let next_page_label = strings.next_page_aria_label.as_ref().to_string();",
        "aria_label=prev_page_label.clone()",
        "aria_label=next_page_label.clone()",
        "let is_prev_disabled = state.get().is_prev_disabled;",
        "let is_next_disabled = state.get().is_next_disabled;",
        "disabled=is_prev_disabled",
        "disabled=is_next_disabled",
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
        "if state.is_prev_disabled",
        "if state.is_next_disabled",
        "if next == state.current_page",
        "if disabled {",
        "if current == p {",
    ] {
        assert!(
            source.contains(needle),
            "Pagination should guard `{needle}` so callbacks don't fire for blocked/no-op interactions."
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
        "on_change=on_change",
        "class_name=\"docs-pagination-custom\".to_string()",
        "disabled=true",
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
        "let on_change = Callback::new(move |next: usize| set_last_change.set(Some(next)));",
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
        "set_page=set_disabled_page",
        "\"disabled page: \"",
        "total_pages=0",
        "page=empty_page",
        "set_page=set_empty_page",
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
