use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn list_docs_section(source: &str) -> &str {
    let start = source
        .find("pub(super) fn list() -> AnyView")
        .expect("list docs function should exist");
    let tail = &source[start..];
    let end = tail
        .find("pub(super) fn menu() -> AnyView")
        .expect("list docs section should end before menu docs function");
    &tail[..end]
}

#[test]
fn list_module_reexports_canonical_list_contracts() {
    let source = load_source("src/list/mod.rs");

    for needle in [
        "pub use logic::{ListItemSelectionIndicator, ListSectionHeadingTone, ListState};",
        "pub use motion::ListMotion;",
        "pub use motion::ListSectionMotion;",
        "pub use view::{List, ListItem, ListSection};",
    ] {
        assert!(
            source.contains(needle),
            "list module should expose canonical `{needle}`."
        );
    }

    for removed in [
        "pub use crate::listbox::ListBox as ListView;",
        "pub use crate::item::Item;",
    ] {
        assert!(
            !source.contains(removed),
            "list module should not keep removed alias `{removed}`."
        );
    }
}

#[test]
fn crate_root_registers_list_and_hides_listbox_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("pub mod list;"),
        "crate root should include `pub mod list;`."
    );
    assert!(
        !source.contains("mod listbox;"),
        "crate root should not keep legacy listbox module."
    );
}

#[test]
fn list_docs_use_list_family_slugs_and_components() {
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let collections_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let list_section = list_docs_section(&collections_source);
    let collections_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    for needle in [
        "component_doc!(\"List\", \"list\", \"Collections\", collections::list)",
        "\"ListItem\"",
        "\"list-item\"",
        "\"ListSection\"",
        "\"list-section\"",
        "collections_extra::list_item",
        "collections_extra::list_section",
    ] {
        assert!(
            pages_source.contains(needle),
            "components catalog should include `{needle}` for list family docs."
        );
    }

    for needle in ["title=\"List\"", "slug=\"list\"", "<List"] {
        assert!(
            list_section.contains(needle),
            "collections docs should include `{needle}` for the canonical List page."
        );
    }

    for needle in [
        "pub(super) fn list_item() -> AnyView",
        "title=\"ListItem\"",
        "slug=\"list-item\"",
        "<ListItem",
        "pub(super) fn list_section() -> AnyView",
        "title=\"ListSection\"",
        "slug=\"list-section\"",
        "<ListSection",
    ] {
        assert!(
            collections_extra_source.contains(needle),
            "collections-extra docs should include `{needle}` for list item/section pages."
        );
    }

    assert!(
        mod_source.contains("\"list\" => &[\"list\", \"list-item\", \"list-section\"]"),
        "components mapping should point `list` to list/list-item/list-section."
    );
    assert!(
        !mod_source.contains("\"list-box\" =>"),
        "components mapping should not contain the removed `list-box` alias."
    );
}

#[test]
fn list_docs_page_exposes_showcase_and_workbench_contracts() {
    let collections_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let section = list_docs_section(&collections_source);

    for needle in [
        "title=\"展示：多场景对比\"",
        "title=\"Workbench（展示 + Config + Code + CSS Test）\"",
        "test_css_source=workbench_test_css",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"list-showcase\"",
        "data-slot=\"list-workbench-controls\"",
        "data-slot=\"list-workbench-canvas\"",
        "sync_active_index_to_selected=false",
        "disabled_indices=vec![2]",
        "disabled=true",
        "items=empty_items",
    ] {
        assert!(
            section.contains(needle),
            "list docs section should contain `{needle}` for showcase/workbench coverage."
        );
    }
}

#[test]
fn list_readme_documents_display_config_code_css_test_sections() {
    let source = load_source("src/list/README.md");

    for needle in [
        "## 展示 (Display)",
        "## Config (Workbench Settings)",
        "## Code (Workbench Snippet)",
        "## CSS Test (Scoped CSS)",
        "collections.rs` 的 `list()`",
        "sync_active_index_to_selected=false",
        "test_css_source",
        "test_config_signal",
    ] {
        assert!(
            source.contains(needle),
            "list README should contain `{needle}` to lock workbench docs contract."
        );
    }
}

#[test]
fn list_check2_has_no_unchecked_checklist_items() {
    let source = load_source("src/list/check2.md");

    assert!(
        !source.contains("- [ ]"),
        "list/check2.md should not keep unchecked checklist items after sequential verification."
    );
}

#[test]
fn list_check2_marks_async_scope_as_explicit_na() {
    let source = load_source("src/list/check2.md");

    assert!(
        source.contains("N/A：`List` 当前仅本地集合导航与选择，不包含远程请求/异步加载状态。"),
        "list/check2.md should explicitly mark async contract as N/A in current scope."
    );
}

#[test]
fn list_check2_marks_streaming_scope_as_optional_with_snapshot_fallback() {
    let source = load_source("src/list/check2.md");

    for needle in [
        "归类为 `Streaming Optional`",
        "`Snapshot` 渲染为基线",
        "`fallback=snapshot`",
    ] {
        assert!(
            source.contains(needle),
            "list/check2.md should keep streaming governance marker `{needle}`."
        );
    }
}

#[test]
fn list_check2_documents_semantic_e2e_selector_and_ready_wait_contract() {
    let source = load_source("src/list/check2.md");

    for needle in [
        "e2e/tests/docs_app_components_coverage.spec.mjs",
        "`data-slot` 选择器",
        "`body:not(:has(#boot))`",
    ] {
        assert!(
            source.contains(needle),
            "list/check2.md should keep e2e selector/wait marker `{needle}`."
        );
    }
}

#[test]
fn list_feature_graph_declares_required_motion_dependencies() {
    let cargo_toml = load_source("Cargo.toml");

    assert!(
        cargo_toml.contains(
            "component-list = [\"component-active_highlight\", \"component-illustrated_message\"]"
        ),
        "ui-components feature graph should declare list -> active_highlight/illustrated_message dependencies for minimal-feature builds."
    );
}
