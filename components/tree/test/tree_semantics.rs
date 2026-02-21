use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let path = if let Some(suffix) = rel_path.strip_prefix("src/tree/") {
        workspace_dir.join("components/tree/src").join(suffix)
    } else if rel_path == "src/lib.rs" {
        workspace_dir.join("crates/ui-components/src/lib.rs")
    } else if rel_path == "src/css.rs" {
        workspace_dir.join("crates/ui-components/src/css.rs")
    } else if rel_path == "Cargo.toml" {
        workspace_dir.join("crates/ui-components/Cargo.toml")
    } else if let Some(suffix) = rel_path.strip_prefix("../ui-state-primitives/") {
        workspace_dir
            .join("crates/ui-state-primitives")
            .join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("../ui-headless/") {
        workspace_dir.join("crates/ui-headless").join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("../ui-motion/") {
        workspace_dir.join("crates/ui-motion").join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("../../") {
        workspace_dir.join(suffix)
    } else {
        manifest_dir.join(rel_path)
    };
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn tree_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/tree/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Tree internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn tree_status_primitives_are_sourced_from_ui_state_primitives() {
    let logic_source = load_source("src/tree/logic.rs");
    let mod_source = load_source("src/tree/mod.rs");
    let primitive_source = load_source("../ui-state-primitives/src/tree.rs");

    for needle in [
        "pub use ui_state_primitives::tree::{",
        "normalize_nodes",
        "collect_all_ids",
        "collect_expandable_ids",
        "sanitize_expanded_ids",
        "sanitize_selected_id",
        "toggle_expanded",
        "flatten_visible_nodes",
        "TreeNode",
        "TreeVisibleNode",
        "TreeStateCoreInput",
        "resolve_state_core",
    ] {
        assert!(
            logic_source.contains(needle) || mod_source.contains(needle),
            "Tree should consume primitive `{needle}` from ui-state-primitives."
        );
    }

    for needle in [
        "pub struct TreeNode",
        "pub struct TreeVisibleNode",
        "pub fn normalize_nodes(",
        "pub fn collect_all_ids(",
        "pub fn collect_expandable_ids(",
        "pub fn sanitize_expanded_ids(",
        "pub fn sanitize_selected_id(",
        "pub fn toggle_expanded(",
        "pub fn flatten_visible_nodes(",
        "pub fn resolve_state_core(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ui-state-primitives tree module should define `{needle}`."
        );
    }

    for forbidden in [
        "pub fn normalize_nodes(",
        "pub fn collect_all_ids(",
        "pub fn collect_expandable_ids(",
        "pub fn sanitize_expanded_ids(",
        "pub fn sanitize_selected_id(",
        "pub fn toggle_expanded(",
        "pub fn flatten_visible_nodes(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Tree logic must not reimplement state primitive `{forbidden}`."
        );
    }
}

#[test]
fn tree_view_mounts_headless_tree_contracts() {
    let source = load_source("src/tree/view.rs");

    for needle in [
        "use ui_headless::{",
        "CommonStrings",
        "tree_root_attrs",
        "use_tree_item",
        "TreeItemA11yInput",
        "TreeItemOptions",
        "use_controllable_state",
        "use_ui_i18n",
        "let i18n = use_ui_i18n();",
        "let common_strings = i18n.strings::<CommonStrings>();",
        "logic::normalize_aria_label_with_fallback(",
        "common_strings.tree_aria_label.as_ref()",
        "let root_a11y = tree_root_attrs(",
        "role=root_a11y.role",
        "aria-label=root_a11y.aria_label",
        "lang=root_a11y.lang",
        "dir=root_a11y.dir",
        "let tree_item = use_tree_item(",
        "role=tree_item.attrs.role",
        "aria-expanded=tree_item.attrs.aria_expanded",
        "aria-selected=Some(tree_item.attrs.aria_selected)",
        "aria-disabled=tree_item.attrs.aria_disabled",
        "tabindex=tree_item.attrs.tabindex",
        "disabled=!tree_item.state.is_interactive",
        "tree_item.handlers.on_click.run(())",
        "tree_item.handlers.on_key_down.run(ev.key())",
    ] {
        assert!(
            source.contains(needle),
            "Tree view should mount typed headless contracts via `{needle}`."
        );
    }

    for forbidden in [
        "role=\"tree\"",
        "role=\"treeitem\"",
        "if disabled || is_disabled",
        "aria-expanded=if node.has_children",
    ] {
        assert!(
            !source.contains(forbidden),
            "Tree view should avoid inline semantic branching `{forbidden}`."
        );
    }
}

#[test]
fn tree_api_naming_uses_prefixed_booleans_and_migration_alias() {
    let view_source = load_source("src/tree/view.rs");
    let logic_source = load_source("src/tree/logic.rs");

    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
        "let is_disabled = logic::normalize_is_disabled(is_disabled, disabled);",
        "#[prop(optional)] on_expanded_ids_change: Option<Callback<BTreeSet<String>>>",
        "#[prop(optional)] on_expanded_change: Option<Callback<BTreeSet<String>>>",
        "#[prop(optional)] default_expanded_ids: Option<BTreeSet<String>>",
        "#[prop(optional)] on_selected_id_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] on_selected_change: Option<Callback<Option<String>>>",
        "#[prop(optional)] default_selected_id: Option<String>",
    ] {
        assert!(
            view_source.contains(needle),
            "Tree API should expose naming contract token `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_is_disabled(is_disabled: Option<bool>, disabled: bool) -> bool",
        "is_disabled.unwrap_or(disabled)",
        "pub fn normalize_expanded_ids_change_handler(",
        "on_expanded_ids_change.or(on_expanded_change)",
        "pub fn normalize_selected_id_change_handler(",
        "on_selected_id_change.or(on_selected_change)",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tree logic should centralize disabled alias migration via `{needle}`."
        );
    }
}

#[test]
fn tree_controlled_uncontrolled_axes_have_value_default_on_change_triplets() {
    let view_source = load_source("src/tree/view.rs");
    let logic_source = load_source("src/tree/logic.rs");

    for needle in [
        "#[prop(optional)] expanded_ids: Option<Signal<BTreeSet<String>>>",
        "#[prop(optional)] default_expanded_ids: Option<BTreeSet<String>>",
        "#[prop(optional)] on_expanded_ids_change: Option<Callback<BTreeSet<String>>>",
        "let expanded_axis = logic::normalize_expanded_axis(",
        "logic::TreeExpandedAxisInput {",
        "is_controlled: expanded_ids.is_some(),",
        "default_expanded_ids,",
        "on_expanded_ids_change,",
        "on_expanded_change,",
        "Some(expanded_axis.default_expanded_ids)",
        "expanded_axis.on_expanded_change",
        "#[prop(optional)] selected_id: Option<Signal<Option<String>>>",
        "#[prop(optional)] default_selected_id: Option<String>",
        "#[prop(optional)] on_selected_id_change: Option<Callback<Option<String>>>",
        "let selected_axis = logic::normalize_selected_axis(",
        "logic::TreeSelectedAxisInput {",
        "is_controlled: selected_id.is_some(),",
        "default_selected_id,",
        "on_selected_id_change,",
        "on_selected_change,",
        "Some(selected_axis.default_selected_id)",
        "selected_axis.on_selected_change",
    ] {
        assert!(
            view_source.contains(needle),
            "Tree controlled/uncontrolled triplet contract should include `{needle}`."
        );
    }

    for needle in [
        "pub struct TreeExpandedAxisInput",
        "pub struct TreeExpandedAxis",
        "pub fn normalize_expanded_axis(",
        "pub fn normalize_expanded_ids_change_handler(",
        "pub struct TreeSelectedAxisInput",
        "pub struct TreeSelectedAxis",
        "pub fn normalize_selected_axis(",
        "pub fn normalize_selected_id_change_handler(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tree logic should centralize controlled handler alias migration via `{needle}`."
        );
    }

    for forbidden in [
        ".set(",
        "set_expanded_ids",
        "set_selected_id",
        "RwSignal::new(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Tree view should not bypass controllable-state single source-of-truth with `{forbidden}`."
        );
    }
}

#[test]
fn tree_default_values_are_normalized_in_logic_only() {
    let view_source = load_source("src/tree/view.rs");
    let logic_source = load_source("src/tree/logic.rs");

    for needle in [
        "pub fn normalize_expanded_axis(",
        "input.default_expanded_ids.unwrap_or_default()",
        "sanitize_expanded_ids(",
        "pub fn normalize_selected_axis(",
        "normalize_optional_text(input.default_selected_id)",
        "sanitize_selected_id(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tree logic should own default normalization token `{needle}`."
        );
    }

    for forbidden in [
        "default_expanded_ids.unwrap_or_default()",
        "logic::normalize_optional_text(default_selected_id)",
        "logic::sanitize_expanded_ids(default_expanded_ids",
        "let default_selected_id = logic::sanitize_selected_id(",
        "let default_expanded_ids =",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Tree view should not contain secondary default fallback `{forbidden}`."
        );
    }
}

#[test]
fn tree_state_derivation_and_source_markers_are_centralized() {
    let view_source = load_source("src/tree/view.rs");
    let logic_source = load_source("src/tree/logic.rs");

    for needle in [
        "pub enum TreeControlMode",
        "pub enum TreeDefaultSource",
        "pub enum TreeChangeSource",
        "pub fn derive_state(",
        "pub fn resolve_expanded_toggle_request(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tree logic should keep typed state derivation token `{needle}`.",
        );
    }

    for needle in [
        "let derived_state = Memo::new(move |_| {",
        "logic::derive_state(logic::TreeDerivedStateInput {",
        "let next = logic::resolve_expanded_toggle_request(",
        "data-expanded-mode=expanded_mode_attr",
        "data-selected-mode=selected_mode_attr",
        "data-default-expanded-source=default_expanded_source_attr",
        "data-default-selected-source=default_selected_source_attr",
        "data-expanded-change-source=expanded_change_source_attr",
        "data-selected-change-source=selected_change_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Tree view should consume centralized derivation/source marker `{needle}`.",
        );
    }

    for forbidden in [
        "logic::flatten_visible_nodes(",
        "logic::sanitize_expanded_ids(",
        "logic::sanitize_selected_id(",
        "logic::toggle_expanded(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Tree view should not inline state-machine primitive `{forbidden}`.",
        );
    }
}

#[test]
fn tree_motion_layer_is_split_and_view_only_attaches() {
    let mod_source = load_source("src/tree/mod.rs");
    let view_source = load_source("src/tree/view.rs");
    let motion_source = load_source("src/tree/motion.rs");
    let lib_source = load_source("src/lib.rs");

    for needle in ["mod motion;", "pub use motion::TreeMotion;"] {
        assert!(
            mod_source.contains(needle),
            "Tree module should expose motion boundary token `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] motion: TreeMotion",
        "let motion = motion::sanitize_motion(motion);",
        "let inline_style = StoredValue::new(Some({",
        "motion::resolve_motion_css_vars(is_expanded, motion)",
        "style=inline_style.get_value().unwrap_or_default()",
        "format!(\"--ui-tree-motion-scale:{scale};--ui-tree-motion-opacity:{opacity};\")",
        "motion::attach_motion(root_ref, expanded_for_motion, motion);",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
        "data-custom-motion=has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "Tree view should only map semantic state to motion contract via `{needle}`."
        );
    }

    for needle in [
        "pub struct TreeMotion",
        "pub fn sanitize_motion(motion: TreeMotion) -> TreeMotion",
        "pub fn resolve_motion_css_vars(",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new",
    ] {
        assert!(
            motion_source.contains(needle),
            "Tree motion module should include `{needle}`."
        );
    }

    for forbidden in ["view! {", "role=", "aria-", "use_tree_item("] {
        assert!(
            !motion_source.contains(forbidden),
            "Tree motion layer should not include view/headless semantics `{forbidden}`."
        );
    }

    assert!(
        lib_source.contains("pub use tree::{Tree, TreeDensity, TreeMotion, TreeNode, TreeTone};"),
        "ui-components crate root should re-export `TreeMotion` with tree API."
    );
}

#[test]
fn tree_styles_remain_token_first_and_theme_scoped() {
    let source = load_source("src/tree/styles.rs");

    for needle in [
        "var(--ui-space-",
        "var(--ui-radius-",
        "var(--ui-bg)",
        "var(--ui-fg)",
        "var(--ui-border)",
        "var(--ui-accent)",
        "var(--ui-accent-soft)",
        "var(--ui-fg-muted)",
        "--ui-tree-motion-scale",
        "--ui-tree-motion-opacity",
    ] {
        assert!(
            source.contains(needle),
            "Tree styles should consume ui-theme tokens and ui-scoped vars via `{needle}`."
        );
    }

    for forbidden in ["--tree-", "--custom-tree-", "Theme::", "resolve_tokens("] {
        assert!(
            !source.contains(forbidden),
            "Tree styles should not introduce private theme pipelines `{forbidden}`."
        );
    }
}

#[test]
fn tree_i18n_fallback_chain_is_available() {
    let view_source = load_source("src/tree/view.rs");
    let logic_source = load_source("src/tree/logic.rs");
    let i18n_common_source = load_source("../ui-headless/src/i18n/common.rs");

    for needle in [
        "pub tree_aria_label: Arc<str>,",
        "tree_aria_label: \"Tree\".into(),",
    ] {
        assert!(
            i18n_common_source.contains(needle),
            "Common i18n bundle should provide tree fallback token `{needle}`.",
        );
    }

    for needle in [
        "let i18n = use_ui_i18n();",
        "let common_strings = i18n.strings::<CommonStrings>();",
        "logic::normalize_aria_label_with_fallback(",
        "common_strings.tree_aria_label.as_ref()",
    ] {
        assert!(
            view_source.contains(needle),
            "Tree view should wire i18n fallback chain token `{needle}`.",
        );
    }

    assert!(
        logic_source.contains("pub fn normalize_aria_label_with_fallback("),
        "Tree logic should expose aria fallback normalization helper."
    );
}

#[test]
fn tree_ssr_and_platform_contracts_remain_guarded() {
    let tree_motion_source = load_source("src/tree/motion.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let motion_lib_source = load_source("../ui-motion/src/lib.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should keep web/ssr exclusivity guard `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion should keep non-wasm no-op backend token `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion() {",
    ] {
        assert!(
            tree_motion_source.contains(needle),
            "Tree motion adapter should keep platform/reduced-motion split token `{needle}`.",
        );
    }
}

#[test]
fn tree_feature_gates_preserve_shaking_boundaries() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "component-tree = [\"dep:ui-tree\"]",
        "\"component-tree\"",
        "#[cfg(feature = \"component-tree\")]",
        "pub use ui_tree as tree;",
        "pub use tree::{Tree, TreeDensity, TreeMotion, TreeNode, TreeTone};",
    ] {
        assert!(
            cargo_source.contains(needle) || lib_source.contains(needle),
            "Tree feature gate contract should include `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-tree\")]",
        "out.push_str(crate::tree::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "Tree css aggregation should stay feature-gated with `{needle}`.",
        );
    }
}

#[test]
fn tree_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections_extra.rs");

    for needle in [
        "pub(super) fn tree() -> AnyView",
        "title=\"Tree\"",
        "slug=\"tree\"",
        "description=\"Hierarchical tree with controllable expand/selection state and baseline-style density/tone/state marker contracts.\"",
        "<Playground title=\"Default + Expanded Root\" code_signal=code>",
        "<Playground title=\"Strong + Compact\" code_signal=states_code>",
        "<Tree",
        "tone=TreeTone::Strong",
        "density=TreeDensity::Compact",
        "class_name=\"docs-tree-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "collections-extra docs page should include `{needle}` for tree coverage.",
        );
    }
}

#[test]
fn tree_perf_budget_is_defined_and_regression_markers_are_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let perf_source = load_source("../../apps/docs-app/src/perf_probe.rs");

    for needle in [
        "\"tree\" => UiPerfBudget {",
        "max_mount_ms: 42.0,",
        "max_update_ms: Some(14.0),",
        "max_heap_kb: Some(896.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "Tree perf governance should define thresholded budget token `{needle}`.",
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
    ] {
        assert!(
            perf_source.contains(needle),
            "UiPerfProbe should expose machine-readable perf marker `{needle}`.",
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
            "docs e2e should keep blocking perf assertion `{needle}`.",
        );
    }
}

#[test]
fn tree_perf_attribution_markers_and_reactive_budget_stay_bounded() {
    let view_source = load_source("src/tree/view.rs");
    let motion_source = load_source("src/tree/motion.rs");

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-expanded-mode=expanded_mode_attr",
        "data-selected-mode=selected_mode_attr",
        "data-default-expanded-source=default_expanded_source_attr",
        "data-default-selected-source=default_selected_source_attr",
        "data-expanded-change-source=expanded_change_source_attr",
        "data-selected-change-source=selected_change_source_attr",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
    ] {
        assert!(
            view_source.contains(needle),
            "Tree should keep attributable perf marker `{needle}` for root-cause analysis.",
        );
    }

    let view_memo_count = view_source.matches("Memo::new(").count();
    assert!(
        view_memo_count <= 4,
        "Tree view reactive budget exceeded: expected <= 4 `Memo::new`, found {view_memo_count}.",
    );
    let view_signal_derive_count = view_source.matches("Signal::derive(").count();
    assert!(
        view_signal_derive_count <= 1,
        "Tree view reactive budget exceeded: expected <= 1 `Signal::derive`, found {view_signal_derive_count}.",
    );
    let view_effect_count = view_source.matches("Effect::new(").count();
    assert_eq!(
        view_effect_count, 0,
        "Tree view should avoid direct effect loops; found {view_effect_count}.",
    );

    let motion_effect_count = motion_source.matches("Effect::new(").count();
    assert!(
        motion_effect_count <= 2,
        "Tree motion should keep bounded effect loops (<=2), found {motion_effect_count}.",
    );
    let motion_spring_count = motion_source.matches("SpringAnimator::new").count();
    assert!(
        motion_spring_count <= 2,
        "Tree motion should keep bounded spring engine count (<=2), found {motion_spring_count}.",
    );
}

#[test]
fn tree_perf_render_count_follow_up_remains_tracked() {
    let todo_source = load_source("../../docs/plan/TODO.md");
    assert!(
        todo_source.contains("render_count"),
        "Perf governance should keep explicit render_count follow-up until runtime automation is available."
    );
}

#[test]
fn tree_view_macro_complexity_is_split_into_semantic_blocks() {
    let view_source = load_source("src/tree/view.rs");

    for needle in [
        "fn render_tree_node(",
        "fn render_tree_list(",
        "let render_context = TreeRenderContext {",
        "render_tree_list(derived_state.visible_nodes, render_context.clone())",
    ] {
        assert!(
            view_source.contains(needle),
            "Tree view should split macro-heavy rendering into semantic helper token `{needle}`.",
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 2,
        "Tree view macro complexity regression: expected <= 2 `view!` blocks, found {view_macro_count}.",
    );

    let component_macro_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_macro_count, 1,
        "Tree should keep one public component entry and avoid subcomponent noise; found {component_macro_count}.",
    );
}

#[test]
fn tree_view_prefers_functional_split_without_extra_component_abstraction() {
    let view_source = load_source("src/tree/view.rs");

    for needle in [
        "fn tree_item_row_class(",
        "fn tree_item_chevron(",
        "fn render_tree_node(",
        "fn render_tree_list(",
    ] {
        assert!(
            view_source.contains(needle),
            "Tree should keep lightweight function split token `{needle}`.",
        );
    }

    for forbidden in [
        "#[component]\nfn TreeNode",
        "#[component]\nfn TreeItem",
        "#[component]\nfn TreeList",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Tree should avoid upgrading local render helpers into nested components `{forbidden}`.",
        );
    }
}

#[test]
fn tree_static_fragments_are_constantized_for_repeated_render_paths() {
    let view_source = load_source("src/tree/view.rs");

    for needle in [
        "const TREE_CHEVRON_EXPANDED: &str = \"▾\";",
        "const TREE_CHEVRON_COLLAPSED: &str = \"▸\";",
        "const TREE_CHEVRON_LEAF: &str = \"•\";",
        "const TREE_NODE_SLOT: &str = \"tree-node\";",
        "const TREE_ITEM_SLOT: &str = \"tree-item\";",
        "const TREE_CHEVRON_SLOT: &str = \"tree-chevron\";",
        "const TREE_LABEL_SLOT: &str = \"tree-label\";",
        "TREE_CHEVRON_EXPANDED",
        "TREE_CHEVRON_COLLAPSED",
        "TREE_CHEVRON_LEAF",
    ] {
        assert!(
            view_source.contains(needle),
            "Tree should keep reusable static fragment constant `{needle}`.",
        );
    }

    for forbidden in [
        "if node.is_expanded { \"▾\" } else { \"▸\" }",
        "data-slot=\"tree-node\"",
        "data-slot=\"tree-item\"",
        "data-slot=\"tree-chevron\"",
        "data-slot=\"tree-label\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Tree should avoid scattered inline static fragment `{forbidden}`.",
        );
    }
}
