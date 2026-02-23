use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    if rel_path == "../../apps/docs-app/src/pages/components/pages/collections_extra.rs" {
        let parent_path = manifest_dir
            .join("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");
        let child_path = manifest_dir
            .join("../../apps/docs-app/src/pages/components/pages/collections_extra/table.rs");
        let parent = fs::read_to_string(&parent_path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {parent_path:?}: {e}"));
        let child = fs::read_to_string(&child_path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {child_path:?}: {e}"));
        return format!("{parent}\n{child}").replace(
            "pub(crate) fn table() -> AnyView {",
            "pub(super) fn table() -> AnyView {",
        );
    }
    let mapped = match rel_path {
        _ if rel_path.starts_with("src/table/") => {
            format!("src/{}", &rel_path["src/table/".len()..])
        }
        _ => rel_path.to_string(),
    };
    let path = manifest_dir.join(mapped);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn table_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/table/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Table internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn table_uses_logic_state_model() {
    let logic_source = load_source("src/table/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/table.rs");
    let headless_source = load_source("../../crates/ui-headless/src/table.rs");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let view_source = load_source("src/table/view.rs");

    for needle in [
        "use ui_state_primitives::table as primitives;",
        "pub use primitives::{",
        "normalize_optional_text",
        "normalize_aria_label",
        "normalize_empty_text",
        "normalize_columns",
        "normalize_rows",
        "resolve_state",
        "TableVariant",
        "TableStateInput",
        "TableState",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Table logic should consume ui-state-primitives table contracts; missing `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_empty_text(",
        "pub fn normalize_columns(",
        "pub fn normalize_rows(",
        "pub fn resolve_state(",
        "data_state_attr",
        "aria_source_attr",
        "class_source_attr",
    ] {
        assert!(
            primitive_source.contains(needle),
            "table primitive should expose `{needle}` in ui-state-primitives.",
        );
    }

    for needle in ["pub mod table;", "TableA11yOptions", "use_table_a11y"] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless lib should export table a11y contract `{needle}`."
        );
    }

    for needle in [
        "pub struct TableA11yAttrs",
        "pub struct TableA11yHandlers",
        "pub struct TableA11yState",
        "pub struct TableA11yContract",
        "pub struct TableA11yOptions",
        "pub fn use_table_a11y(",
    ] {
        assert!(
            headless_source.contains(needle),
            "ui-headless table module should define typed attrs/handlers/state contract `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{",
        "A11yDirection",
        "TableA11yOptions",
        "use_table_a11y",
        "#[prop(optional)] is_striped: bool,",
        "#[prop(optional)] is_sticky_header: bool,",
        "logic::normalize_columns(columns)",
        "logic::normalize_rows(rows, columns.len())",
        "logic::normalize_optional_text(caption)",
        "logic::normalize_empty_text(empty_label)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(TableStateInput {",
        "let table_a11y = use_table_a11y(TableA11yOptions {",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Table view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn table_api_naming_contract_uses_prefixed_boolean_props() {
    let view_source = load_source("src/table/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for required in [
        "#[prop(optional)] is_striped: bool,",
        "#[prop(optional)] is_sticky_header: bool,",
        "is_striped=workbench_striped.get()",
        "is_sticky_header=workbench_sticky_header.get()",
        "is_striped=true",
        "is_sticky_header=true",
    ] {
        assert!(
            view_source.contains(required) || docs_source.contains(required),
            "Table API naming contract should keep `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] striped: bool,",
        "#[prop(optional)] sticky_header: bool,",
        " striped=",
        " sticky_header=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Table view should not expose legacy naming `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "Table docs should not keep legacy naming `{forbidden}`."
        );
    }
}

#[test]
fn table_has_no_controlled_uncontrolled_state_axes_yet() {
    let view_source = load_source("src/table/view.rs");

    for required in [
        "#[prop(optional)] is_striped: bool,",
        "#[prop(optional)] is_sticky_header: bool,",
        "striped: is_striped,",
        "sticky_header: is_sticky_header,",
    ] {
        assert!(
            view_source.contains(required),
            "Table should keep direct config mapping `{required}` while no mutable state axis exists."
        );
    }

    for forbidden in [
        "default_is_striped",
        "on_is_striped_change",
        "default_is_sticky_header",
        "on_is_sticky_header_change",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Table should not expose half-controlled API fragment `{forbidden}` without a real mutable axis."
        );
    }
}

#[test]
fn table_default_resolution_stays_out_of_view_layer() {
    let view_source = load_source("src/table/view.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/table.rs");

    for forbidden in [
        "unwrap_or(",
        "unwrap_or_else(",
        "DEFAULT_EMPTY_TEXT",
        "format!(\"col-{}\",",
        "format!(\"row-{}\",",
        "format!(\"Column {}\",",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Table view should not perform default fallback resolution `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_empty_text(",
        "pub fn normalize_columns(",
        "pub fn normalize_rows(",
        "normalize_optional_text(value).unwrap_or_else(|| DEFAULT_EMPTY_TEXT.into())",
        "unwrap_or_else(|| format!(\"col-{}\", index + 1))",
        "unwrap_or_else(|| format!(\"row-{}\", index + 1))",
        "unwrap_or_else(|| format!(\"Column {}\", index + 1))",
    ] {
        assert!(
            primitive_source.contains(required),
            "Default resolution should stay centralized in ui-state-primitives/logic; missing `{required}`."
        );
    }
}

#[test]
fn table_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/table/view.rs");

    for attr in [
        "data-slot=\"table\"",
        "data-variant=table_a11y_attrs.data_variant",
        "data-density=table_a11y_attrs.data_density",
        "data-layout=table_a11y_attrs.data_layout",
        "data-state=table_a11y_attrs.data_state",
        "data-striped=table_a11y_attrs.data_striped",
        "data-sticky-header=table_a11y_attrs.data_sticky_header",
        "data-has-caption=table_a11y_attrs.data_has_caption",
        "data-row-count=table_a11y_attrs.data_row_count",
        "data-aria-source=table_a11y_attrs.data_aria_source",
        "data-custom-class=table_a11y_attrs.data_custom_class",
        "data-class-source=table_a11y_attrs.data_class_source",
        "data-slot=\"table-element\"",
        "data-slot=\"table-head\"",
        "data-slot=\"table-head-row\"",
        "data-slot=\"table-head-cell\"",
        "data-slot=\"table-body\"",
        "data-slot=\"table-row\"",
        "data-slot=\"table-cell\"",
        "role=table_a11y_attrs.role",
        "aria-label=table_a11y_attrs.aria_label",
        "lang=table_a11y_attrs.lang",
        "dir=table_a11y_attrs.dir",
    ] {
        assert!(
            source.contains(attr),
            "Table should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn table_styles_include_variant_density_layout_and_markers() {
    let source = load_source("src/table/styles.rs");

    for selector in [
        ".ui-table--variant-default",
        ".ui-table[data-variant=\"default\"]",
        ".ui-table--variant-quiet",
        ".ui-table--variant-outline",
        ".ui-table--density-comfortable",
        ".ui-table[data-density=\"comfortable\"]",
        ".ui-table--density-compact",
        ".ui-table[data-density=\"compact\"]",
        ".ui-table--layout-auto",
        ".ui-table[data-layout=\"auto\"]",
        ".ui-table--layout-fixed",
        ".ui-table[data-layout=\"fixed\"]",
        ".ui-table--striped",
        ".ui-table[data-striped=\"true\"]",
        ".ui-table--sticky-header",
        ".ui-table[data-sticky-header=\"true\"]",
        ".ui-table--with-caption",
        ".ui-table[data-has-caption=\"true\"]",
        ".ui-table--empty",
        ".ui-table[data-state=\"empty\"]",
        ".ui-table--custom-class",
        ".ui-table[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Table styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn table_motion_contract_is_explicitly_not_coupled_for_static_component() {
    let cargo_source = load_source("Cargo.toml");
    let logic_source = load_source("src/table/logic.rs");
    let view_source = load_source("src/table/view.rs");
    let styles_source = load_source("src/table/styles.rs");

    assert!(
        !cargo_source.contains("ui-motion"),
        "Table should not depend on ui-motion when no component-level motion semantics are defined."
    );

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(
        !manifest_dir.join("src/motion.rs").exists(),
        "Table should not define src/motion.rs without explicit motion semantics."
    );

    for forbidden in [
        "ui_motion",
        "attach_motion",
        "Spring",
        "stiffness",
        "damping",
        "keyframe",
        "waapi",
        "prefers-reduced-motion",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Table logic should stay motion-free; found `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "Table view should stay motion-free; found `{forbidden}`."
        );
        assert!(
            !styles_source.contains(forbidden),
            "Table styles should stay motion-free; found `{forbidden}`."
        );
    }
}

#[test]
fn table_ui_aggregation_exports_feature_gated_public_api_without_dom_type_leaks() {
    let component_lib_source = load_source("src/lib.rs");
    let component_mod_source = load_source("src/table/mod.rs");
    let component_view_source = load_source("src/table/view.rs");
    let ui_cargo_source = load_source("../../crates/ui/Cargo.toml");
    let ui_lib_source = load_source("../../crates/ui/src/lib.rs");
    let ui_css_source = load_source("../../crates/ui/src/css.rs");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{",
        "pub use view::Table;",
    ] {
        assert!(
            component_mod_source.contains(required),
            "Table component boundary should keep scoped responsibility split via `{required}`."
        );
    }

    for required in [
        "#[path = \"mod.rs\"]",
        "pub mod table;",
        "pub use table::*;",
    ] {
        assert!(
            component_lib_source.contains(required),
            "Table crate entry should keep stable boundary export marker `{required}`."
        );
    }

    for required in [
        "ui-headless = { path = \"../ui-headless\" }",
        "ui-state-primitives = { path = \"../ui-state-primitives\" }",
        "ui-motion = { path = \"../ui-motion\" }",
        "ui-theme = { path = \"../ui-theme\" }",
        "component-table = [\"dep:ui-table\"]",
        "ui-table = { path = \"../../components/table\", optional = true }",
    ] {
        assert!(
            ui_cargo_source.contains(required),
            "ui crate should keep composition/feature-gate contract marker `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-table\")]",
        "pub use ui_table as table;",
        "pub use table::{",
        "Table, TableCellAlign, TableColumn, TableDensity, TableLayout, TableRow, TableVariant,",
    ] {
        assert!(
            ui_lib_source.contains(required),
            "ui crate should expose stable table public API marker `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-table\")]",
        "out.push_str(crate::table::styles::CSS);",
    ] {
        assert!(
            ui_css_source.contains(required),
            "ui css aggregation should keep feature-gated table style inclusion `{required}`."
        );
    }

    for forbidden in ["web_sys::", "web-sys", "wasm_bindgen::", "js_sys::"] {
        assert!(
            !component_lib_source.contains(forbidden),
            "Table public crate entry should not leak DOM/web platform detail `{forbidden}`."
        );
        assert!(
            !component_mod_source.contains(forbidden),
            "Table public module boundary should not leak DOM/web platform detail `{forbidden}`."
        );
        assert!(
            !component_view_source.contains(forbidden),
            "Table public component API surface should not leak DOM/web platform detail `{forbidden}`."
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(
        manifest_dir.join("tests/semantics.rs").exists(),
        "Table should keep a dedicated semantics regression suite at tests/semantics.rs."
    );
    for legacy in ["test/semantics.rs", "test/table_semantics.rs"] {
        assert!(
            !manifest_dir.join(legacy).exists(),
            "legacy semantics test path `{legacy}` should be migrated to tests/semantics.rs."
        );
    }
}

#[test]
fn table_theme_contract_consumes_global_ui_tokens_without_private_theme_system() {
    let styles_source = load_source("src/table/styles.rs");
    let logic_source = load_source("src/table/logic.rs");
    let view_source = load_source("src/table/view.rs");
    let tokens_source = load_source("../../crates/ui-theme/src/tokens.rs");
    let theme_source = load_source("../../crates/ui-theme/src/theme.rs");
    let css_source = load_source("../../crates/ui-theme/src/css.rs");

    for required in [
        "var(--ui-",
        "--ui-border",
        "--ui-bg",
        "--ui-fg",
        "--ui-space-",
    ] {
        assert!(
            styles_source.contains(required),
            "Table styles should consume shared ui-theme token variables via `{required}`."
        );
    }

    for required in [
        "pub enum TokenScale",
        "pub struct SemanticColorTokens",
        "pub struct LayoutTokens",
        "pub struct SpaceTokens",
        "pub struct ThemeTokens",
    ] {
        assert!(
            tokens_source.contains(required),
            "ui-theme token baseline should declare `{required}`."
        );
    }

    for required in [
        "pub enum ThemeSystem",
        "pub enum ThemeColor",
        "pub enum ThemeScale",
        "pub struct ThemeContext",
        "pub struct Theme",
        "pub fn new(ctx: ThemeContext) -> Self",
    ] {
        assert!(
            theme_source.contains(required),
            "ui-theme mapping layer should expose `{required}`."
        );
    }

    for required in [
        "theme_to_css_variables",
        "pub const BASE_CSS",
        "pub const SOURCE_CONTRACT_THEME_TOKEN_SENTINELS",
        "pub enum SemanticVariable",
    ] {
        assert!(
            css_source.contains(required),
            "ui-theme css emission layer should expose `{required}`."
        );
    }

    for forbidden in [
        "ui_theme::",
        "ThemeContext",
        "ThemeTokens::",
        "--table-",
        "--table-theme-",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Table logic should not rebuild private theme contracts; found `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "Table view should not rebuild private theme contracts; found `{forbidden}`."
        );
        assert!(
            !styles_source.contains(forbidden),
            "Table styles should not define private theme token system `{forbidden}`."
        );
    }
}

#[test]
fn table_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "pub(super) fn table() -> AnyView",
        "title=\"Table\"",
        "slug=\"table\"",
        "description=\"Data table primitive with centralized row/column normalization and baseline-style state markers for density/layout/variant contracts.\"",
        "<Playground title=\"Default + IsStriped\" code_signal=code>",
        "<Playground title=\"Compact + Fixed + IsStickyHeader\" code_signal=states_code>",
        "<Table",
        "is_striped=true",
        "variant=TableVariant::Outline",
        "density=TableDensity::Compact",
        "layout=TableLayout::Fixed",
        "is_sticky_header=true",
    ] {
        assert!(
            source.contains(needle),
            "collections-extra docs page should include `{needle}` for table coverage.",
        );
    }
}

#[test]
fn table_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "TableColumn::new(\"service\", \"Service\")",
        "TableColumn::new(\"region\", \"Region\")",
        "TableColumn::new(\"uptime\", \"Uptime\").with_align(TableCellAlign::End)",
        "TableRow::new(",
        "\"API Gateway\".to_string()",
        "\"Scheduler\".to_string()",
        "\"Worker\".to_string()",
        "caption=\"Service health\".to_string()",
        "let empty_rows: Vec<TableRow> = Vec::new();",
        "empty_label=\"No active incidents\".to_string()",
        "class_name=\"docs-table-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "table docs playgrounds should contain `{needle}`.",
        );
    }
}
