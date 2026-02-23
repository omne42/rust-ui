use std::fs;
use std::path::Path;

fn resolve_source_path(rel_path: &str) -> Option<std::path::PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let mut candidates = vec![manifest_dir.join(rel_path)];

    if let Some(component_rel_path) = rel_path.strip_prefix("../../components/") {
        let direct = workspace_dir.join("components").join(component_rel_path);
        candidates.push(direct.clone());

        let parts: Vec<&str> = component_rel_path.split('/').collect();
        if parts.len() > 3 && parts.get(1) == Some(&"src") && parts.get(2) == parts.first() {
            let collapsed = workspace_dir
                .join("components")
                .join(parts[0])
                .join("src")
                .join(parts[3..].join("/"));
            candidates.push(collapsed);
        }
    }

    if let Some(src_rel_path) = rel_path.strip_prefix("src/") {
        let segments: Vec<&str> = src_rel_path.split('/').collect();
        let components_root = workspace_dir.join("components");

        if let Ok(entries) = fs::read_dir(&components_root) {
            let component_dirs: Vec<String> = entries
                .flatten()
                .filter_map(|entry| {
                    let path = entry.path();
                    path.is_dir()
                        .then(|| entry.file_name().to_string_lossy().to_string())
                })
                .collect();

            for component_dir in component_dirs {
                for start in 0..segments.len() {
                    for end in start..segments.len() {
                        let name = segments[start..=end]
                            .iter()
                            .map(|segment| segment.replace('_', "-"))
                            .collect::<Vec<_>>()
                            .join("-");

                        if name != component_dir {
                            continue;
                        }

                        if end + 1 >= segments.len() {
                            candidates
                                .push(components_root.join(&component_dir).join("src/mod.rs"));
                            candidates
                                .push(components_root.join(&component_dir).join("src/check2.md"));
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                            continue;
                        }

                        let suffix = segments[end + 1..].join("/");
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("src")
                                .join(&suffix),
                        );
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("test")
                                .join(&suffix),
                        );

                        if suffix == "check2.md" {
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                        }
                    }
                }
            }
        }
    }

    candidates.into_iter().find(|path| path.exists())
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_source_path(rel_path)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path));

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}
fn path_exists(rel_path: &str) -> bool {
    resolve_source_path(rel_path).is_some()
}
#[test]
fn snippet_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/snippet/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Snippet internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn snippet_uses_logic_state_model() {
    let view_source = load_source("src/snippet/view.rs");
    let logic_source = load_source("src/snippet/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/snippet.rs");
    let headless_source = load_source("../ui-headless/src/snippet.rs");

    for needle in [
        "pub use ui_state_primitives::snippet::{",
        "SnippetStateInput, normalize_optional_text, resolve_state",
        "use ui_headless::{A11yDirection, SnippetCopyOptions, use_snippet_copy};",
        "pub enum SnippetCopyableSource",
        "pub struct SnippetCopyableContract",
        "pub enum SnippetCopiedSource",
        "pub struct SnippetControlledCopied",
        "pub fn resolve_copyable_contract(",
        "pub fn resolve_controlled_copied(",
        "pub struct SnippetTextFallbacks",
        "pub fn compose_class_name(",
        "pub struct SnippetLogic",
        "pub fn use_snippet_logic_with_options(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Snippet logic should include `{needle}` for primitives/headless assembly and centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(label)",
        "logic::normalize_optional_text(copied_label)",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_copyable_contract(is_copyable, copyable)",
        "logic::resolve_controlled_copied(is_copied, copied)",
        "logic::resolve_text_contract(",
        "use_ui_i18n().strings::<CommonStrings>()",
        "common_strings.snippet_copy_label",
        "common_strings.snippet_copied_label",
        "common_strings.snippet_copy_aria_label",
        "common_strings.snippet_copy_retry_label",
        "logic::resolve_state(SnippetStateInput {",
        "logic::compose_class_name(class_name, state)",
        "logic::use_snippet_logic_with_options(SnippetLogicOptions {",
    ] {
        assert!(
            view_source.contains(needle),
            "Snippet view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }

    for needle in [
        "pub enum SnippetCopyState",
        "pub struct SnippetStateInput",
        "pub fn resolve_state(input: SnippetStateInput)",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Snippet state primitive should include `{needle}`."
        );
    }

    for needle in [
        "pub struct SnippetCopyOptions",
        "pub struct SnippetCopyContract",
        "pub fn use_snippet_copy(options: SnippetCopyOptions)",
    ] {
        assert!(
            headless_source.contains(needle),
            "Snippet headless contract should include `{needle}`."
        );
    }
}

#[test]
fn snippet_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/snippet/view.rs");

    for attr in [
        "data-slot=\"snippet\"",
        "data-state=state.state_attr",
        "data-copy=state.copy_state_attr",
        "data-copyable-source=copyable_contract.source.as_attr()",
        "data-copied-source=controlled_copied.source.as_attr()",
        "data-copy-status=move || {",
        "data-multiline=state.is_multiline.then_some(\"true\")",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-label=state.has_label.then_some(\"true\")",
        "data-copyable=state.is_copyable.then_some(\"true\")",
        "data-copy-actionable=state.copy_is_actionable.then_some(\"true\")",
        "data-copied-label=state.copied_label_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-loading=move || logic.is_loading.get().then_some(\"true\")",
        "data-copy-error=move || logic.has_error.get().then_some(\"true\")",
        "data-retry-available=move || {",
        "data-slot=\"snippet-copy-button\"",
        "data-slot=\"snippet-copied-status\"",
    ] {
        assert!(
            source.contains(attr),
            "Snippet should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn snippet_styles_include_state_marker_contracts() {
    let source = load_source("src/snippet/styles.rs");

    for selector in [
        ".ui-snippet--state-multiline",
        ".ui-snippet[data-state=\"single-line\"]",
        ".ui-snippet--copyable",
        ".ui-snippet[data-copy=\"disabled\"]",
        ".ui-snippet--copy-static",
        ".ui-snippet--custom-copied-label",
        ".ui-snippet[data-copied-label=\"custom\"]",
        ".ui-snippet--custom-class",
        ".ui-snippet[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Snippet styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn snippet_styles_rely_on_semantic_markers_not_dom_shape() {
    let styles_source = load_source("src/snippet/styles.rs");
    let view_source = load_source("src/snippet/view.rs");

    for selector in [
        ".ui-snippet[data-copy-status=\"idle\"] .ui-snippet__copy-button",
        ".ui-snippet[data-copy-status=\"loading\"] .ui-snippet__copy-button",
        ".ui-snippet[data-copy-status=\"error\"] .ui-snippet__copy-button",
        ".ui-snippet[data-copy-status=\"copied\"] .ui-snippet__copy-button",
        ".ui-snippet__copy-button[data-copying=\"true\"]",
        ".ui-snippet__copy-button[data-copy-error=\"true\"]",
        ".ui-snippet__copy-button[data-copied=\"true\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "Snippet styles should map semantic markers via `{selector}`."
        );
    }

    for forbidden in [":nth-child(", ":nth-of-type("] {
        assert!(
            !styles_source.contains(forbidden),
            "Snippet styles should avoid brittle DOM-shape selector `{forbidden}`."
        );
    }

    assert!(
        !view_source.contains("style="),
        "Snippet view should avoid inline business styling; runtime style updates should stay in motion/custom properties."
    );
}

#[test]
fn snippet_token_first_static_style_contract_is_css_registry_injected_without_utility_or_css_in_rust_default()
 {
    let styles_source = load_source("src/snippet/styles.rs");
    let view_source = load_source("src/snippet/view.rs");
    let logic_source = load_source("src/snippet/logic.rs");
    let motion_source = load_source("src/snippet/motion.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let lib_source = load_source("src/lib.rs");
    let check2_source = load_source("src/snippet/check2.md");

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-accent, var(--ui-fallback-accent))",
    ] {
        assert!(
            styles_source.contains(required),
            "Snippet styles should stay token-first/static and include `{required}`."
        );
    }

    for forbidden in [
        "--snippet-",
        "@apply",
        "tailwind",
        "tw-",
        "styled(",
        "stylex",
        "emotion",
        "css!(",
        "style!(",
        "view! {",
        "Callback::new",
        "format!(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Snippet styles should not adopt utility/CSS-in-Rust/private-token marker `{forbidden}`."
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"px-",
        "class=\"py-",
        "class=\"rounded-",
        "class=\"bg-",
        "class=\"text-",
        "class=\"w-",
        "class=\"h-",
        "class=\"gap-",
        "tailwind",
        "tw!",
        "css!(",
        "style!(",
        "styled!(",
        "emotion",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Snippet component layer should not depend on utility-first/CSS-in-Rust default marker `{forbidden}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-snippet\")]",
        "out.push_str(crate::snippet::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "css.rs should include snippet feature-gated style aggregation marker `{required}`."
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should centralize component CSS injection with marker `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-snippet\")]",
        "pub use ui_snippet as snippet;",
        "pub use snippet::Snippet;",
    ] {
        assert!(
            lib_source.contains(required),
            "ui lib entry should keep snippet surface marker `{required}`."
        );
    }

    assert!(
        motion_source.contains("set_property(\"--ui-snippet-scale\""),
        "Snippet motion should only write runtime custom property `--ui-snippet-scale`."
    );

    for required in [
        "样式规则统一落在 `styles.rs`，由 `crates/ui/src/css.rs` 聚合并通过 `UiRoot` 注入。",
        "Utility-First 仅作为 `apps/*` 应用层布局手段，不得反向污染组件库契约。",
        "CSS-in-Rust 仅在有明确类型安全与构建成本净收益时作为例外采用。",
    ] {
        assert!(
            check2_source.contains(required),
            "Snippet checklist should retain token-first governance guidance `{required}`."
        );
    }
}

#[test]
fn snippet_semantics_matrix_covers_controlled_uncontrolled_disabled_keyboard_pointer_and_platform_split()
 {
    let logic_source = load_source("src/snippet/logic.rs");
    let view_source = load_source("src/snippet/view.rs");
    let headless_source = load_source("../ui-headless/src/snippet.rs");

    for needle in [
        "pub enum SnippetCopiedSource",
        "Controlled",
        "Uncontrolled",
        "data-copied-source=controlled_copied.source.as_attr()",
        "disabled=move || !state.copy_is_actionable || logic.is_loading.get()",
        "data-copy=state.copy_state_attr",
        "type=\"button\"",
        "on:click=move |_| {",
        "aria-label=copy_aria_label.get_value()",
        "aria-live=\"polite\"",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "Snippet matrix coverage should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "async fn write_to_clipboard",
        "set_is_loading.set(false);",
    ] {
        assert!(
            headless_source.contains(needle),
            "Snippet headless should keep explicit platform split evidence `{needle}`."
        );
    }
}

#[test]
fn snippet_component_files_follow_role_boundaries_and_avoid_spec_module() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let snippet_dir = {
        let local = manifest_dir.join("src/snippet");
        if local.exists() {
            local
        } else {
            manifest_dir
                .parent()
                .and_then(Path::parent)
                .unwrap_or_else(|| {
                    panic!("workspace root should be two levels above {manifest_dir:?}")
                })
                .join("components/snippet/src")
        }
    };

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            snippet_dir.join(required).exists(),
            "Snippet directory should include `{required}`."
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !snippet_dir.join(forbidden).exists(),
            "Snippet should not include `{forbidden}` for current component scope."
        );
    }

    let mod_source = load_source("src/snippet/mod.rs");
    let logic_source = load_source("src/snippet/logic.rs");
    let styles_source = load_source("src/snippet/styles.rs");
    let view_source = load_source("src/snippet/view.rs");
    let motion_source = load_source("src/snippet/motion.rs");

    for needle in [
        "pub(crate) mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::SnippetMotion;",
        "pub use view::Snippet;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Snippet mod.rs should keep minimal stable boundary via `{needle}`."
        );
    }

    assert!(
        !logic_source.contains("view! {"),
        "Snippet logic.rs should not render view markup."
    );
    assert!(
        !logic_source.contains("web_sys"),
        "Snippet logic.rs should avoid platform/DOM details."
    );
    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "Snippet styles.rs should provide static CSS contract."
    );
    assert!(
        !styles_source.contains("view! {"),
        "Snippet styles.rs should not embed rendering logic."
    );
    assert!(
        view_source.contains("#[component]"),
        "Snippet view.rs should own component rendering boundary."
    );
    assert!(
        view_source.contains("view! {"),
        "Snippet view.rs should mount structure + semantics."
    );
    assert!(
        motion_source.contains("pub struct SnippetMotion"),
        "Snippet motion.rs should define motion contract type."
    );
    assert!(
        motion_source.contains("pub fn attach_motion("),
        "Snippet motion.rs should expose attach entrypoint."
    );
    assert!(
        !motion_source.contains("#[component]"),
        "Snippet motion.rs should not declare Leptos components."
    );
}

#[test]
fn snippet_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn snippet() -> AnyView",
        "title=\"Snippet\"",
        "slug=\"snippet\"",
        "Playground title=\"Copyable + Copied Label\"",
        "Playground title=\"Static + Multiline Custom\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Snippet.",
        );
    }
}

#[test]
fn snippet_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Copyable + Copied Label\"",
        "text=\"cargo fmt --all\".to_string()",
        "label=\"Command\".to_string()",
        "text=\"RUST_LOG=debug\".to_string()",
        "copied_label=\"Done\".to_string()",
        "title=\"Static + Multiline Custom\"",
        "text=\"cargo test -p ui --test snippet_semantics\".to_string()",
        "text=\"cargo fmt --all\\ncargo clippy -p ui -p docs-app --all-targets -- -D warnings\".to_string()",
        "label=\"CI\".to_string()",
        "is_copyable=true",
        "is_copyable=false",
        "class_name=\"docs-snippet-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "snippet docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn snippet_visual_desire_baseline_is_backed_by_docs_registry_and_e2e_snapshots() {
    let theme_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let pages_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues.",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            theme_page_source.contains(needle),
            "Theme visual baseline docs page should include `{needle}`."
        );
    }

    for needle in [
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            pages_registry_source.contains(needle),
            "Docs registry should expose theme visual baseline entry `{needle}`."
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "renders button/input/overlay",
        "theme visual baseline screenshots",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            e2e_source.contains(needle),
            "E2E visual baseline spec should include `{needle}`."
        );
    }

    for needle in [
        "# HeroUI 参数设计风格对齐策略",
        "HeroUI 对齐结论",
        "体验目标",
    ] {
        assert!(
            heroui_strategy_source.contains(needle),
            "HeroUI strategy doc should include `{needle}`."
        );
    }
}

#[test]
fn snippet_is_non_composite_api_and_avoids_parallel_array_conventions() {
    let view_source = load_source("src/snippet/view.rs");
    let logic_source = load_source("src/snippet/logic.rs");
    let mod_source = load_source("src/snippet/mod.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub fn Snippet(",
        "text: String",
        "label: Option<String>",
        "is_copyable: Option<bool>",
        "view! {",
    ] {
        assert!(
            view_source.contains(needle),
            "Snippet API should remain single-component and include `{needle}`."
        );
    }

    for forbidden in [
        "ItemSpec",
        "SnippetItem",
        "labels: Vec",
        "titles: Vec",
        "panels: Vec",
        "items: Vec",
        "children: Vec",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !mod_source.contains(forbidden),
            "Snippet implementation should avoid composite parallel-array convention `{forbidden}`."
        );
    }

    for needle in [
        "pub(super) fn snippet() -> AnyView",
        "<Snippet",
        "title=\"Copyable + Copied Label\"",
        "title=\"Static + Multiline Custom\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Snippet docs should keep explicit direct usage marker `{needle}`."
        );
    }
}

#[test]
fn snippet_tree_shaking_contract_is_feature_gated_and_budget_guarded() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let tree_script_source = load_source("../../scripts/check-ui-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "component-snippet = [\"dep:ui-snippet\"]",
        "all-components = [",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui Cargo features should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-snippet\")]",
        "pub use ui_snippet as snippet;",
        "pub use snippet::Snippet;",
    ] {
        assert!(
            lib_source.contains(needle),
            "lib.rs should gate snippet export with `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-snippet\")]",
        "out.push_str(crate::snippet::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "css.rs should gate snippet CSS aggregation with `{needle}`."
        );
    }

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "unexpected all-components in minimal feature tree",
        "cargo tree -e features -i ui -p web-demo",
        "web-demo should pull web-demo-components feature bundle",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "size regression",
    ] {
        assert!(
            tree_script_source.contains(needle),
            "tree-shaking gate script should include `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should include `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn snippet_type_system_and_semantic_markers_form_machine_readable_state_contract() {
    let primitive_source = load_source("../ui-state-primitives/src/snippet.rs");
    let logic_source = load_source("src/snippet/logic.rs");
    let view_source = load_source("src/snippet/view.rs");

    for needle in [
        "pub enum SnippetLayout",
        "pub enum SnippetCopyState",
        "pub enum SnippetSource",
        "pub struct SnippetStateInput",
        "pub struct SnippetState",
        "pub fn resolve_state(input: SnippetStateInput) -> SnippetState",
        "fn marker_values_are_closed_sets()",
    ] {
        assert!(
            primitive_source.contains(needle),
            "State primitive type contract should include `{needle}`."
        );
    }

    for needle in [
        "pub enum SnippetCopyableSource",
        "pub enum SnippetCopiedSource",
        "pub struct SnippetCopyableContract",
        "pub struct SnippetControlledCopied",
        "pub fn resolve_copyable_contract(",
        "pub fn resolve_controlled_copied(",
        "resolve_state(SnippetStateInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "Logic normalization contract should include `{needle}`."
        );
    }

    for marker in [
        "data-state=state.state_attr",
        "data-copy=state.copy_state_attr",
        "data-copied-label=state.copied_label_source_attr",
        "data-copyable-source=copyable_contract.source.as_attr()",
        "data-copied-source=controlled_copied.source.as_attr()",
        "data-copy-status=move || {",
    ] {
        assert!(
            view_source.contains(marker),
            "View semantic contract should expose machine-readable marker `{marker}`."
        );
    }
}

#[test]
fn snippet_platform_contract_uses_explicit_cfg_branches_and_keeps_non_wasm_web_api_free() {
    let view_source = load_source("src/snippet/view.rs");
    let logic_source = load_source("src/snippet/logic.rs");
    let motion_source = load_source("src/snippet/motion.rs");
    let headless_source = load_source("../ui-headless/src/snippet.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "Snippet motion platform split should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "async fn write_to_clipboard",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "set_is_loading.set(false);",
    ] {
        assert!(
            headless_source.contains(needle),
            "Snippet headless platform split should include `{needle}`."
        );
    }

    for forbidden in ["web_sys", "window(", "navigator("] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Snippet component non-platform files should not reference browser API marker `{forbidden}`."
        );
    }

    let (_, non_wasm_motion_segment) = motion_source
        .split_once("#[cfg(not(target_arch = \"wasm32\"))]")
        .expect("snippet motion should provide a non-wasm cfg branch");
    let non_wasm_motion_segment = non_wasm_motion_segment
        .split_once("#[cfg(test)]")
        .map(|(head, _)| head)
        .unwrap_or(non_wasm_motion_segment);
    assert!(
        !non_wasm_motion_segment.contains("web_sys"),
        "Snippet non-wasm motion path should not depend on web_sys."
    );
}

#[test]
fn snippet_headless_web_ssr_mutex_guard_is_enforced_and_regression_checked() {
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let snippet_logic_source = load_source("src/snippet/logic.rs");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless feature mutex guard should include `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, SnippetCopyOptions, use_snippet_copy};",
        "use_snippet_copy(SnippetCopyOptions {",
    ] {
        assert!(
            snippet_logic_source.contains(needle),
            "Snippet should consume ui-headless contract via `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "mutually exclusive",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform gate script should enforce `{needle}`."
        );
    }
}

#[test]
fn snippet_motion_non_wasm_noop_contract_is_present_and_toolchain_safe() {
    let ui_motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let snippet_motion_source = load_source("src/snippet/motion.rs");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");
    let ui_motion_non_wasm_test_source = load_source("../ui-motion/tests/non_wasm_stub.rs");

    for needle in [
        "Compile on non-wasm targets (no-op stubs) to support SSR/tooling builds.",
        "#[cfg(target_arch = \"wasm32\")]",
        "pub mod web;",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            snippet_motion_source.contains(needle),
            "Snippet motion should include predictable wasm/non-wasm split marker `{needle}`."
        );
    }

    let (_, non_wasm_motion_segment) = snippet_motion_source
        .split_once("#[cfg(not(target_arch = \"wasm32\"))]")
        .expect("snippet motion should provide non-wasm branch");
    let non_wasm_motion_segment = non_wasm_motion_segment
        .split_once("#[cfg(test)]")
        .map(|(head, _)| head)
        .unwrap_or(non_wasm_motion_segment);
    assert!(
        !non_wasm_motion_segment.contains("panic!(")
            && !non_wasm_motion_segment.contains("web_sys"),
        "Snippet non-wasm motion path should be no-op/safe and avoid web_sys."
    );

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
        "[platform] ui-motion non-wasm stub tests",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "Platform gate script should include motion toolchain guard `{needle}`."
        );
    }

    for needle in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "non_wasm_web_backend_prefers_reduced_motion",
        "non_wasm_web_backend_animate_is_safe_noop",
    ] {
        assert!(
            ui_motion_non_wasm_test_source.contains(needle),
            "ui-motion non-wasm test should include `{needle}`."
        );
    }
}

#[test]
fn snippet_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let motion_source = load_source("src/snippet/motion.rs");
    let view_source = load_source("src/snippet/view.rs");
    let headless_source = load_source("../ui-headless/src/snippet.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if !motion.enabled || ui_motion::web::prefers_reduced_motion() {",
        "spring.set_target(target);",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "Snippet motion reduced-motion/wasm split should include `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"snippet\"",
        "aria-busy=logic.aria_busy",
        "data-copy-status=move || {",
        "data-copy-error=move || logic.has_error.get().then_some(\"true\")",
        "data-retry-available=move || {",
    ] {
        assert!(
            view_source.contains(needle),
            "Snippet semantic contract should stay stable across platforms via `{needle}`."
        );
    }

    assert!(
        !view_source.contains("#[cfg(target_arch = \"wasm32\")]")
            && !view_source.contains("#[cfg(not(target_arch = \"wasm32\"))]"),
        "Snippet view semantics should not split by target_arch and must remain hydration-compatible."
    );

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "async fn write_to_clipboard",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "set_is_loading.set(false);",
    ] {
        assert!(
            headless_source.contains(needle),
            "Snippet headless should include explicit platform branch `{needle}`."
        );
    }
}

#[test]
fn snippet_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let check2_source = load_source("src/snippet/check2.md");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let view_source = load_source("src/snippet/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep performance budget/probe contract token `{needle}`."
        );
    }

    let snippet_doc_needle =
        "component_doc!(\"Snippet\", \"snippet\", \"Display\", display::snippet)";
    assert!(
        pages_source.contains(snippet_doc_needle),
        "Snippet docs page should stay in component catalog coverage via `{snippet_doc_needle}`."
    );

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose repeatable perf marker `{needle}`."
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
            "docs coverage e2e should enforce perf regression guard `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace-based attribution marker `{needle}`."
        );
    }

    for needle in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "Snippet checklist should keep performance governance marker `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance follow-up plan should keep `{needle}`."
        );
    }

    for needle in [
        "logic::resolve_state(SnippetStateInput {",
        "motion::attach_motion(root_ref, logic.copied, motion);",
        "data-copy-status=move || {",
        "data-copy-error=move || logic.has_error.get().then_some(\"true\")",
        "data-copy-actionable=state.copy_is_actionable.then_some(\"true\")",
        "data-copyable-source=copyable_contract.source.as_attr()",
        "data-copied-source=controlled_copied.source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "Snippet view should expose state/render/style/motion attribution marker `{needle}`."
        );
    }
}

#[test]
fn snippet_performance_check_script_covers_budget_and_follow_up_gates() {
    let script_source = load_source("../../scripts/check-ui-performance.sh");

    for needle in [
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }
}

#[test]
fn snippet_view_macro_complexity_is_small_and_semantically_split_for_current_scope() {
    let view_source = load_source("src/snippet/view.rs");
    let logic_source = load_source("src/snippet/logic.rs");

    assert_eq!(
        view_source.matches("view! {").count(),
        3,
        "Snippet should keep a small/bounded view! macro footprint."
    );

    for needle in [
        "label.map(|label| {",
        "state.is_copyable.then(|| {",
        "logic::resolve_state(SnippetStateInput {",
        "logic::compose_class_name(class_name, state)",
        "motion::attach_motion(root_ref, logic.copied, motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "Snippet view should keep semantic sub-block marker `{needle}`."
        );
    }

    assert!(
        !logic_source.contains("view! {"),
        "Snippet logic should stay macro-render free."
    );
}

#[test]
fn snippet_view_functional_split_prefers_plain_helpers_without_local_component_noise() {
    let view_source = load_source("src/snippet/view.rs");
    let logic_source = load_source("src/snippet/logic.rs");
    let motion_source = load_source("src/snippet/motion.rs");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Snippet should expose one component boundary and avoid local component sprawl."
    );
    assert!(
        view_source.contains("pub fn Snippet("),
        "Snippet view should keep explicit top-level component function."
    );

    for forbidden in [
        "#[component]\nfn",
        "pub fn SnippetItem(",
        "pub fn SnippetSection(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Snippet should avoid local component noise marker `{forbidden}`."
        );
    }

    for needle in [
        "pub fn resolve_copyable_contract(",
        "pub fn resolve_controlled_copied(",
        "pub fn resolve_text_contract(",
        "pub fn compose_class_name(",
        "pub fn sanitize_motion(",
    ] {
        assert!(
            logic_source.contains(needle) || motion_source.contains(needle),
            "Snippet function-first split should keep helper `{needle}`."
        );
    }
}

#[test]
fn snippet_static_fragments_are_constantized_or_absent_for_simple_layout() {
    let view_source = load_source("src/snippet/view.rs");
    let logic_source = load_source("src/snippet/logic.rs");

    for needle in [
        "pub const DEFAULT_COPY_LABEL: &str = \"Copy\";",
        "pub const DEFAULT_COPIED_LABEL: &str = \"Copied\";",
        "pub const DEFAULT_COPY_ARIA_LABEL: &str = \"Copy to clipboard\";",
        "pub const DEFAULT_COPY_ERROR_LABEL: &str = \"Copy failed. Activate again to retry.\";",
    ] {
        assert!(
            logic_source.contains(needle),
            "Snippet static text baseline should be centralized constant `{needle}`."
        );
    }

    for forbidden in ["<svg", "path d=", "inner_html", "include_str!("] {
        assert!(
            !view_source.contains(forbidden),
            "Snippet simple layout should avoid heavy static fragment runtime marker `{forbidden}`."
        );
    }
}

#[test]
fn snippet_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let view_source = load_source("src/snippet/view.rs");
    let logic_source = load_source("src/snippet/logic.rs");
    let styles_source = load_source("src/snippet/styles.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let script_source = load_source("../../scripts/check-ui-inner-html.sh");

    for source in [&view_source, &logic_source, &styles_source, &docs_source] {
        assert!(
            !source.contains("inner_html"),
            "Snippet component/docs contract should reject raw inner_html injection."
        );
    }

    for needle in [
        "[inner-html] contract: button runtime paths reject raw html injection",
        "[inner-html] contract: docs inner_html stays trusted and whitelisted",
    ] {
        assert!(
            script_source.contains(needle),
            "Global inner-html gate should include `{needle}`."
        );
    }
}

#[test]
fn snippet_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let view_source = load_source("src/snippet/view.rs");

    assert!(
        cargo_source.contains("component-snippet = [\"dep:ui-snippet\"]"),
        "Snippet feature should stay lean and not carry component-local wasm debug fan-out."
    );
    assert!(
        !cargo_source.contains("snippet-wasm-debug"),
        "Snippet should not introduce a component-local wasm debug feature."
    );

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(needle),
            "Docs app should keep shared wasm debug entry marker `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "let ts_ms = event.ts_ms;",
        "data-slot=\"ui-debug-overlay-event\"",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "Shared debug overlay should keep trace/replay marker `{needle}`."
        );
    }

    for marker in [
        "data-copy-status=move || {",
        "data-copyable-source=copyable_contract.source.as_attr()",
        "data-copied-source=controlled_copied.source.as_attr()",
    ] {
        assert!(
            view_source.contains(marker),
            "Snippet should expose inspectable state markers `{marker}` for shared debug tooling."
        );
    }
}

#[test]
fn snippet_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let dx_script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "pub(super) fn snippet() -> AnyView",
        "<Playground title=\"Copyable + Copied Label\" code_signal=copy_code>",
        "<Playground title=\"Static + Multiline Custom\" code_signal=custom_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "Snippet docs should keep interactive playground marker `{needle}`."
        );
    }

    for needle in [
        "fn compose_scoped_css(scope_selector: &str, raw: &str) -> String",
        "let (test_css, set_test_css) = signal(default_test_css.get_untracked());",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "data-playground-scope=scope_id.clone()",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload/isolated scope marker `{needle}`."
        );
    }

    assert!(
        dx_script_source.contains("[dx] contract: playground css hot-reload path"),
        "DX check script should keep playground CSS hot-reload gate."
    );
}

#[test]
fn snippet_dx_scope_uses_isolated_canvas_and_marks_state_persist_as_not_required() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let check2_source = load_source("src/snippet/check2.md");

    for needle in [
        "<div class=\"docs-stack\">",
        "title=\"Copyable + Copied Label\"",
        "title=\"Static + Multiline Custom\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "Snippet docs should keep isolated demo canvas marker `{needle}`."
        );
    }

    assert!(
        check2_source.contains("复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。"),
        "Snippet checklist should keep DX isolation rule and explicitly review N/A/适用性."
    );
}

#[test]
fn snippet_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/snippet/mod.rs");
    let logic_source = load_source("src/snippet/logic.rs");
    let view_source = load_source("src/snippet/view.rs");
    let motion_source = load_source("src/snippet/motion.rs");
    let styles_source = load_source("src/snippet/styles.rs");

    assert!(
        !path_exists("src/snippet/spec.rs"),
        "Snippet should keep spec/schema path as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-snippet = [\"dep:ui-snippet\"]"),
        "Snippet feature should stay serde/spec-free."
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{motion_source}\n{styles_source}");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Snippet engineering contract should keep serde/spec path N/A; found `{forbidden}`."
        );
    }
}

#[test]
fn snippet_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let combined = [
        load_source("src/snippet/mod.rs"),
        load_source("src/snippet/logic.rs"),
        load_source("src/snippet/view.rs"),
        load_source("src/snippet/motion.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(required),
            "Global tracing/wasm-debug baseline should include `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("snippet-wasm-debug"),
        "Snippet should not define local tracing/wasm-debug feature."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::snippet::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Snippet should avoid component-local tracing drift token `{forbidden}`."
        );
    }
}

#[test]
fn snippet_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/snippet/mod.rs");
    let logic_source = load_source("src/snippet/logic.rs");
    let view_source = load_source("src/snippet/view.rs");
    let motion_source = load_source("src/snippet/motion.rs");

    for source in [&mod_source, &logic_source, &view_source, &motion_source] {
        for forbidden in [
            "tokio::",
            "async_std::",
            "smol::",
            "runtime::Handle",
            "spawn_blocking(",
        ] {
            assert!(
                !source.contains(forbidden),
                "Snippet public surface should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "Snippet module boundary should not leak web_sys/platform internals."
    );
}

#[test]
fn snippet_ui_components_entrypoints_and_forbidden_files_contract_hold() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");

    for required in [
        "#[cfg(feature = \"component-snippet\")]",
        "pub use ui_snippet as snippet;",
    ] {
        assert!(
            lib_source.contains(required),
            "ui lib entry should include `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-snippet\")]",
        "out.push_str(crate::snippet::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "css entry should include snippet-gated marker `{required}`."
        );
    }

    for required in [
        "provide_ui_i18n(i18n);",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should centralize injection/i18n marker `{required}`."
        );
    }

    for required in [
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "shared active_highlight entry should keep reusable-only marker `{required}`."
        );
    }

    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden),
            "forbidden ui entrypoint should stay absent: `{forbidden}`."
        );
    }

    for required in [
        "../../crates/ui-headless/src/controllable_state.rs",
        "../../crates/ui-headless/src/presence.rs",
        "../../crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            path_exists(required),
            "canonical ui-headless primitive file should exist: `{required}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn snippet_agent_contract_is_machine_readable_and_type_backed_without_dom_guessing() {
    let primitive_source = load_source("../ui-state-primitives/src/snippet.rs");
    let logic_source = load_source("src/snippet/logic.rs");
    let view_source = load_source("src/snippet/view.rs");

    for required in [
        "pub enum SnippetLayout",
        "pub enum SnippetCopyState",
        "pub enum SnippetSource",
        "pub struct SnippetStateInput",
        "pub struct SnippetState",
        "fn marker_values_are_closed_sets()",
    ] {
        assert!(
            primitive_source.contains(required),
            "Snippet state primitive should keep typed Agent-contract marker `{required}`."
        );
    }

    for required in [
        "pub enum SnippetCopyableSource",
        "pub enum SnippetCopiedSource",
        "pub const fn as_attr(self) -> &'static str",
    ] {
        assert!(
            logic_source.contains(required),
            "Snippet logic should keep typed source marker mapping `{required}`."
        );
    }

    for required in [
        "data-state=state.state_attr",
        "data-copy=state.copy_state_attr",
        "data-copy-status=move || {",
        "data-copyable-source=copyable_contract.source.as_attr()",
        "data-copied-source=controlled_copied.source.as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "Snippet view should expose machine-readable semantic marker `{required}`."
        );
    }
}

#[test]
fn snippet_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("src/snippet/check2.md");

    for required in [
        "流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            check2_source.contains(required),
            "Snippet checklist should keep streaming definition marker `{required}`."
        );
    }
}

#[test]
fn snippet_snapshot_baseline_and_streaming_fallback_contract_are_explicit() {
    let check2_source = load_source("src/snippet/check2.md");
    let view_source = load_source("src/snippet/view.rs");

    for required in [
        "`Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "若不支持流式，必须明确 `fallback=snapshot`。",
    ] {
        assert!(
            check2_source.contains(required),
            "Snippet checklist should keep snapshot/fallback marker `{required}`."
        );
    }

    for forbidden in ["data-stream", "data-streaming", "data-token", "data-draft"] {
        assert!(
            !view_source.contains(forbidden),
            "Snippet should remain snapshot-scope and avoid stream protocol marker `{forbidden}`."
        );
    }
}

#[test]
fn snippet_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let check2_source = load_source("src/snippet/check2.md");
    let view_source = load_source("src/snippet/view.rs");

    assert!(
        check2_source.contains("并保持 `role`/`aria-*`/`data-*` 连续可读。"),
        "Snippet checklist should keep continuity rule for streaming-optional scope."
    );

    for required in [
        "aria-busy=logic.aria_busy",
        "aria-live=\"polite\"",
        "data-slot=\"snippet\"",
        "data-copy-status=move || {",
    ] {
        assert!(
            view_source.contains(required),
            "Snippet view should keep continuous a11y/data marker `{required}`."
        );
    }
}

#[test]
fn snippet_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let check2_source = load_source("src/snippet/check2.md");
    let logic_source = load_source("src/snippet/logic.rs");
    let view_source = load_source("src/snippet/view.rs");

    assert!(
        check2_source.contains("数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。"),
        "Snippet checklist should keep streaming responsibility boundary contract."
    );

    for forbidden in [
        "reqwest",
        "fetch(",
        "websocket",
        "retry_with_backoff",
        "connection_lost",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Snippet should avoid component-layer streaming resilience marker `{forbidden}`."
        );
    }
}

#[test]
fn snippet_semantics_test_priority_contract_is_present() {
    let check2_source = load_source("src/snippet/check2.md");
    let semantics_source = load_source("tests/semantics.rs");

    for required in [
        "语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
    ] {
        assert!(
            check2_source.contains(required),
            "Snippet checklist should keep semantic-test-first rule `{required}`."
        );
    }

    assert!(
        semantics_source.matches("#[test]").count() >= 20,
        "Snippet semantics file should keep substantial contract coverage."
    );
}

#[test]
fn snippet_e2e_selectors_use_semantic_markers_and_stable_wait_strategy() {
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let check2_source = load_source("src/snippet/check2.md");

    for required in [
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "await expect(page.locator(`[data-slot=\"${slug}\"]`).first()).toBeVisible();",
        "await expect(page.locator(\"section.playground\").first()).toBeVisible();",
    ] {
        assert!(
            e2e_source.contains(required),
            "E2E should keep stable semantic selector/wait marker `{required}`."
        );
    }

    assert!(
        !e2e_source.contains("waitForTimeout("),
        "E2E should avoid fixed sleep waits and prefer readiness markers."
    );

    for required in [
        "E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
    ] {
        assert!(
            check2_source.contains(required),
            "Snippet checklist should retain E2E selector stability rule `{required}`."
        );
    }
}

#[test]
fn snippet_e2e_regression_flow_is_repeatable_and_locatable() {
    let e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let check2_source = load_source("src/snippet/check2.md");

    for required in [
        "test(\"docs-app components pages render playgrounds (sample)\"",
        "test(\"docs-app components pages render playgrounds (all)\"",
        "const coverageMode = process.env.E2E_COVERAGE ?? \"sample\";",
        "for (const slug of slugs",
    ] {
        assert!(
            e2e_source.contains(required),
            "E2E regression flow should keep repeatable marker `{required}`."
        );
    }

    for required in [
        "关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
    ] {
        assert!(
            check2_source.contains(required),
            "Snippet checklist should retain repeatable E2E regression rule `{required}`."
        );
    }
}

#[test]
fn snippet_docs_are_beginner_friendly_with_default_path_before_advanced_usage() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let check2_source = load_source("src/snippet/check2.md");

    for required in [
        "title=\"Snippet\"",
        "slug=\"snippet\"",
        "description=\"Text snippet with centralized multiline/copy state attrs and optional copied-label/custom-class contracts.\"",
        "Playground title=\"Copyable + Copied Label\"",
        "Playground title=\"Static + Multiline Custom\"",
    ] {
        assert!(
            docs_source.contains(required),
            "Snippet docs should keep beginner-friendly marker `{required}`."
        );
    }

    assert!(
        docs_source.find("Playground title=\"Copyable + Copied Label\"")
            < docs_source.find("Playground title=\"Static + Multiline Custom\""),
        "Snippet docs should place default path before advanced customization."
    );

    for required in [
        "组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
    ] {
        assert!(
            check2_source.contains(required),
            "Snippet checklist should keep beginner-friendly docs rule `{required}`."
        );
    }
}

#[test]
fn snippet_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for required in [
        "<Playground title=\"Copyable + Copied Label\" code_signal=copy_code>",
        "<Playground title=\"Static + Multiline Custom\" code_signal=custom_code>",
        "is_copyable=true",
        "is_copyable=false",
    ] {
        assert!(
            docs_source.contains(required),
            "Snippet docs should keep interactive playground state marker `{required}`."
        );
    }

    for required in [
        "code_signal: Option<Signal<String>>",
        "if let Some(dynamic_code) = code_signal {",
        "<UiPerfProbe name=format!(\"Playground::{title}\")>",
    ] {
        assert!(
            playground_source.contains(required),
            "Playground runtime should keep live preview marker `{required}`."
        );
    }
}

#[test]
fn snippet_source_first_copy_paste_ready_contract_is_documented_and_wired() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for required in [
        "let copy_code = Signal::derive(move || {",
        "let custom_code = Signal::derive(move || {",
        "code_signal=copy_code",
        "code_signal=custom_code",
    ] {
        assert!(
            docs_source.contains(required),
            "Snippet docs should keep source-first code snapshot marker `{required}`."
        );
    }

    for required in [
        "const DEFAULT_PLAYGROUND_IMPORTS",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "return compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value());",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(required),
            "Playground should keep copy-paste-ready pipeline marker `{required}`."
        );
    }

    for required in [
        "Source-first / Copy-Paste Ready",
        "Snippet(copyable=true)",
        "compose_copy_ready_code",
    ] {
        assert!(
            strategy_source.contains(required),
            "HeroUI strategy doc should keep source-first/copy-ready governance marker `{required}`."
        );
    }
}

#[test]
fn snippet_heroui_strategy_and_component_docs_entry_stay_synced() {
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let check2_source = load_source("src/snippet/check2.md");

    let docs_entry = "component_doc!(\"Snippet\", \"snippet\", \"Display\", display::snippet)";
    assert!(
        pages_source.contains(docs_entry),
        "Snippet docs registry should include `{docs_entry}`."
    );

    for required in [
        "# HeroUI 参数设计风格对齐策略",
        "HeroUI 对齐结论",
        "Copy-Paste Ready",
    ] {
        assert!(
            strategy_source.contains(required),
            "HeroUI strategy doc should keep marker `{required}`."
        );
    }

    for required in [
        "HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
    ] {
        assert!(
            check2_source.contains(required),
            "Snippet checklist should keep HeroUI/docs sync rule `{required}`."
        );
    }
}

#[test]
fn snippet_no_temporary_patch_contract_drift_is_detectable_via_source_markers() {
    let logic_source = load_source("src/snippet/logic.rs");
    let view_source = load_source("src/snippet/view.rs");
    let check2_source = load_source("src/snippet/check2.md");

    for required in [
        "pub enum SnippetCopyableSource",
        "LegacyCopyableProp",
        "data-copyable-source=copyable_contract.source.as_attr()",
        "data-copied-source=controlled_copied.source.as_attr()",
    ] {
        assert!(
            logic_source.contains(required) || view_source.contains(required),
            "Snippet compatibility path should remain explicit and inspectable via `{required}`."
        );
    }

    for forbidden in ["TODO(", "FIXME", "HACK", "temporary patch"] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Snippet should avoid temporary patch debt marker `{forbidden}`."
        );
    }

    assert!(
        check2_source.contains("用临时补丁破坏跨组件一致性。"),
        "Snippet checklist should keep temporary-patch anti-pattern gate."
    );
}
