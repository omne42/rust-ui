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
fn load_visually_hidden_component_source() -> String {
    [
        load_source("../../components/visually-hidden/src/mod.rs"),
        load_source("../../components/visually-hidden/src/view.rs"),
        load_source("../../components/visually-hidden/src/styles.rs"),
    ]
    .join("\n")
}

#[test]
fn visually_hidden_module_exposes_component_and_css_contract() {
    let source = load_visually_hidden_component_source();

    for needle in [
        "mod logic;",
        "pub fn VisuallyHidden(",
        "#[prop(optional, into)] is_focusable: Option<bool>",
        "normalize_props(VisuallyHiddenLogicInput",
        "ui_headless::a11y::{A11yDirection, locale_attrs}",
        "data-slot=\"visually-hidden\"",
        "data-focus-mode=state.focus_mode.as_attr()",
        "data-focus-source=state.focus_prop_source.as_attr()",
        "data-class-source=state.class_name_source.as_attr()",
        "pub const CSS: &str = r#\"",
        ".ui-visually-hidden--focusable:focus-within",
    ] {
        assert!(
            source.contains(needle),
            "visually_hidden module should include `{needle}` for @a11y-baseline/visually-hidden compatibility.",
        );
    }
}

#[test]
fn visually_hidden_component_files_keep_single_responsibility_boundaries() {
    let mod_source = load_source("../../components/visually-hidden/src/mod.rs");
    for needle in [
        "mod logic;",
        "mod styles;",
        "mod view;",
        "pub use styles::CSS;",
        "pub use view::VisuallyHidden;",
    ] {
        assert!(
            mod_source.contains(needle),
            "visually_hidden mod.rs should keep module boundary export `{needle}`.",
        );
    }
    for forbidden in ["#[component]", "view!", "pub const CSS", "normalize_props("] {
        assert!(
            !mod_source.contains(forbidden),
            "visually_hidden mod.rs should not carry implementation detail `{forbidden}`.",
        );
    }

    let logic_source = load_source("../../components/visually-hidden/src/logic.rs");
    for forbidden in [
        "view!",
        "data-slot=",
        ".ui-visually-hidden",
        "ui_headless::a11y",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "visually_hidden logic.rs should stay logic-only and avoid `{forbidden}`.",
        );
    }

    let styles_source = load_source("../../components/visually-hidden/src/styles.rs");
    for needle in [
        "pub const CSS: &str = r#\"",
        ".ui-visually-hidden--focusable:focus-within",
        ".ui-visually-hidden[data-focus-mode=\"focusable\"]:focus-within",
    ] {
        assert!(
            styles_source.contains(needle),
            "visually_hidden styles.rs should keep static css contract `{needle}`.",
        );
    }
    for forbidden in ["#[component]", "view!", "locale_attrs(", "normalize_props("] {
        assert!(
            !styles_source.contains(forbidden),
            "visually_hidden styles.rs should not carry runtime logic `{forbidden}`.",
        );
    }

    let view_source = load_source("../../components/visually-hidden/src/view.rs");
    for needle in [
        "#[component]",
        "pub fn VisuallyHidden(",
        "normalize_props(VisuallyHiddenLogicInput",
        "let locale = locale_attrs(lang, dir);",
        "data-slot=\"visually-hidden\"",
    ] {
        assert!(
            view_source.contains(needle),
            "visually_hidden view.rs should keep rendering + headless mount `{needle}`.",
        );
    }
    for forbidden in [
        "pub const CSS",
        "resolve_state(",
        "normalize_optional_text(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "visually_hidden view.rs should not reimplement lower-layer detail `{forbidden}`.",
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let motion_module = manifest_dir.join("../../components/visually-hidden/src/motion.rs");
    assert!(
        !motion_module.exists(),
        "visually_hidden should not add motion.rs when no reusable motion contract exists.",
    );
}

#[test]
fn visually_hidden_view_macro_complexity_stays_flat_and_small() {
    let view_source = load_source("../../components/visually-hidden/src/view.rs");

    let view_macro_count = view_source.matches("view! {").count();
    assert_eq!(
        view_macro_count, 1,
        "visually_hidden should keep a single small view! block; found {view_macro_count}.",
    );

    let slot_marker_count = view_source.matches("data-slot=\"visually-hidden\"").count();
    assert_eq!(
        slot_marker_count, 1,
        "visually_hidden should avoid repeated deep fragments; expected one slot marker, found {slot_marker_count}.",
    );

    for forbidden in [
        "<header",
        "<section",
        "<article",
        "<footer",
        "<nav",
        "<ul",
        "<li",
        ".map(|",
        "for item in",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "visually_hidden view! should avoid complex nesting/repetition token `{forbidden}`.",
        );
    }
}

#[test]
fn visually_hidden_keeps_spec_module_absent_for_simple_component_contract() {
    let mod_source = load_source("../../components/visually-hidden/src/mod.rs");
    for forbidden in ["mod spec;", "pub use spec::", "spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "visually_hidden should not introduce spec module binding `{forbidden}` for simple contract.",
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_module = manifest_dir.join("../../components/visually-hidden/src/spec.rs");
    assert!(
        !spec_module.exists(),
        "visually_hidden should not add spec.rs unless stable external schema contract is required.",
    );
}

#[test]
fn crate_root_registers_visually_hidden_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub use ui_visually_hidden as visually_hidden;",
        "pub use visually_hidden::VisuallyHidden;",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for visually-hidden compatibility.",
        );
    }
}

#[test]
fn visually_hidden_css_is_injected_by_ui_root_aggregation() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::visually_hidden::CSS);"),
        "ui css aggregation should include visually_hidden CSS for runtime compatibility.",
    );
}

#[test]
fn visually_hidden_tree_shaking_feature_gates_keep_module_and_css_conditionally_reachable() {
    let cargo_toml = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "component-visually_hidden = [\"dep:ui-visually-hidden\"]",
        "#[cfg(feature = \"component-visually_hidden\")]",
        "pub use ui_visually_hidden as visually_hidden;",
        "#[cfg(feature = \"component-visually_hidden\")]\n    out.push_str(crate::visually_hidden::CSS);",
    ] {
        let combined = format!("{cargo_toml}\n{lib_source}\n{css_source}");
        assert!(
            combined.contains(needle),
            "visually_hidden tree-shaking contract should keep feature-gated token `{needle}`.",
        );
    }

    // Guardrail: no unconditional CSS aggregation path for visually_hidden.
    assert!(
        !css_source.contains("out.push_str(crate::visually_hidden::CSS);\nout.push_str("),
        "visually_hidden css aggregation should remain conditional and avoid unconditional chain append.",
    );
}

#[test]
fn visually_hidden_token_first_static_styles_contract_stays_in_styles_and_css_aggregation() {
    let styles_source = load_source("../../components/visually-hidden/src/styles.rs");
    let view_source = load_source("../../components/visually-hidden/src/view.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        ".ui-visually-hidden {",
        "out.push_str(crate::visually_hidden::CSS);",
        "crate::css::push_components_css(&mut out);",
    ] {
        let combined = format!("{styles_source}\n{css_source}\n{root_source}");
        assert!(
            combined.contains(needle),
            "visually_hidden style contract should include `{needle}` in styles/css/root injection chain.",
        );
    }

    for forbidden in [
        "style=",
        "style:",
        "style! {",
        "styled_components",
        "tw-",
        "class=\"flex",
        "class=\"grid",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "visually_hidden view should avoid runtime style utility/c-in-rust token `{forbidden}`.",
        );
    }

    for forbidden in ["--vh-", "--visually-hidden-"] {
        assert!(
            !styles_source.contains(forbidden),
            "visually_hidden styles should not define private token system marker `{forbidden}`.",
        );
    }
}

#[test]
fn visually_hidden_visual_desire_is_n_a_and_must_not_introduce_visible_theme_styling() {
    let styles_source = load_source("../../components/visually-hidden/src/styles.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/forms_extra_visually_hidden.rs",
    );

    for forbidden in [
        "background:",
        "color:",
        "box-shadow:",
        "text-shadow:",
        "border-radius:",
        "linear-gradient(",
        "transition:",
        "animation:",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "visually_hidden should remain non-visual utility and avoid visible aesthetic style token `{forbidden}`.",
        );
    }

    assert!(
        docs_source.contains("title=\"VisuallyHidden\""),
        "visually_hidden docs baseline entry should remain available for non-visual utility verification.",
    );
}

#[test]
fn visually_hidden_has_dedicated_docs_playground() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/forms_extra_visually_hidden.rs",
    );

    for needle in [
        "title=\"VisuallyHidden\"",
        "slug=\"visually-hidden\"",
        "title=\"Hello World\"",
        "<VisuallyHidden",
    ] {
        assert!(
            source.contains(needle),
            "forms visually-hidden docs page should contain `{needle}` for compatibility coverage.",
        );
    }
}

#[test]
fn visually_hidden_module_docs_page_covers_primary_playgrounds() {
    visually_hidden_has_dedicated_docs_playground();
}

#[test]
fn visually_hidden_module_docs_playgrounds_lock_state_matrix_contract_values() {
    visually_hidden_has_dedicated_docs_playground();
}

#[test]
fn visually_hidden_remains_non_interactive_without_component_level_input_semantics() {
    let source = load_visually_hidden_component_source();

    for forbidden in [
        "on:click",
        "on:keydown",
        "on:keyup",
        "on:pointerdown",
        "on:pointerup",
        "on:focus",
        "on:blur",
    ] {
        assert!(
            !source.contains(forbidden),
            "visually_hidden should remain non-interactive and avoid `{forbidden}` component-level input semantics."
        );
    }
}

#[test]
fn visually_hidden_avoids_motion_contract_binding_and_component_motion_module() {
    let source = load_visually_hidden_component_source();
    assert!(
        !source.contains("ui_motion"),
        "visually_hidden should not bind ui-motion runtime contracts for static hidden content.",
    );
    assert!(
        !source.contains("attach_motion"),
        "visually_hidden should not attach component-level motion drivers.",
    );

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let motion_module = manifest_dir.join("../../components/visually-hidden/src/motion.rs");
    assert!(
        !motion_module.exists(),
        "visually_hidden should not require a dedicated motion.rs module for current static semantics.",
    );
}

#[test]
fn visually_hidden_motion_dependency_exposes_non_wasm_noop_stub_contract() {
    let motion_source = load_source("../ui-motion/src/lib.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`.",
        );
    }
}

#[test]
fn visually_hidden_reduced_motion_ssr_wasm_paths_stay_semantically_consistent() {
    let component_source = load_visually_hidden_component_source();
    let logic_source = load_source("../../components/visually-hidden/src/logic.rs");
    let combined = format!("{component_source}\n{logic_source}");

    // This component is static/non-animated, so no runtime reduced-motion branch is needed.
    for forbidden in [
        "prefers_reduced_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "cfg!(target_arch = \"wasm32\")",
        "hydrate::",
        "wasm_bindgen",
        "web_sys",
    ] {
        assert!(
            !combined.contains(forbidden),
            "visually_hidden should avoid platform-split semantic branch token `{forbidden}`.",
        );
    }

    for needle in [
        "data-focus-mode=state.focus_mode.as_attr()",
        "data-focus-source=state.focus_prop_source.as_attr()",
        "data-class-source=state.class_name_source.as_attr()",
        "lang=locale.lang",
        "dir=locale.dir",
    ] {
        assert!(
            component_source.contains(needle),
            "visually_hidden should keep cross-platform semantic marker `{needle}` stable.",
        );
    }
}

#[test]
fn visually_hidden_performance_baseline_stays_static_without_internal_reactive_loops() {
    let component_source = load_visually_hidden_component_source();
    let logic_source = load_source("../../components/visually-hidden/src/logic.rs");
    let styles_source = load_source("../../components/visually-hidden/src/styles.rs");
    let combined = format!("{component_source}\n{logic_source}\n{styles_source}");

    // Equivalent evidence for a lightweight, predictable render path:
    // no internal reactive effects/memos/async/motion loops in this component.
    for forbidden in [
        "Signal::derive(",
        "Memo::new(",
        "create_effect(",
        "spawn_local(",
        "request_animation_frame",
        "set_timeout",
        "Instant::now(",
        "performance.now",
        "ui_motion",
        "attach_motion",
        "animation:",
        "transition:",
    ] {
        assert!(
            !combined.contains(forbidden),
            "visually_hidden performance baseline should avoid internal reactive/motion loop token `{forbidden}`.",
        );
    }

    for needle in ["<span", "data-slot=\"visually-hidden\""] {
        assert!(
            component_source.contains(needle),
            "visually_hidden should keep a minimal static render structure containing `{needle}`.",
        );
    }
}

#[test]
fn visually_hidden_keeps_theme_boundary_without_local_token_system() {
    let source = load_visually_hidden_component_source();

    for forbidden in ["ui_theme", "UiTheme", "theme.get()", "theme()"] {
        assert!(
            !source.contains(forbidden),
            "visually_hidden should not couple to ui-theme runtime context via `{forbidden}`.",
        );
    }

    assert!(
        !source.contains("--ui-"),
        "visually_hidden should not define or rely on component-local `--ui-*` token variables for static hidden semantics.",
    );
}

#[test]
fn visually_hidden_stays_in_ui_components_assembly_layer_without_platform_type_leakage() {
    let module_source = load_visually_hidden_component_source();
    assert!(
        module_source.contains("normalize_props(VisuallyHiddenLogicInput"),
        "visually_hidden view assembly should consume normalized logic output rather than rebuilding state in mod.rs.",
    );

    for forbidden in ["web_sys", "js_sys", "wasm_bindgen"] {
        assert!(
            !module_source.contains(forbidden),
            "visually_hidden should not leak platform-specific types via `{forbidden}` in component assembly layer.",
        );
    }

    let logic_source = load_source("../../components/visually-hidden/src/logic.rs");
    for forbidden in ["web_sys", "js_sys", "wasm_bindgen"] {
        assert!(
            !logic_source.contains(forbidden),
            "visually_hidden logic should not leak platform-specific types via `{forbidden}`.",
        );
    }
    assert!(
        logic_source.contains("ui_state_primitives::visually_hidden"),
        "visually_hidden logic should source state primitives from ui-state-primitives.",
    );

    let crate_root_source = load_source("src/lib.rs");
    for needle in [
        "pub use ui_visually_hidden as visually_hidden;",
        "pub use visually_hidden::VisuallyHidden;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui public API should keep `{needle}` for visually-hidden stable export surface.",
        );
    }
}

#[test]
fn visually_hidden_headless_dependency_preserves_web_ssr_mutual_exclusion_guard() {
    let headless_source = load_source("../ui-headless/src/lib.rs");
    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_source.contains(needle),
            "ui-headless mutual-exclusion guard should include `{needle}`.",
        );
    }
}

#[test]
fn visually_hidden_api_naming_uses_is_prefix_with_legacy_alias_migration_path() {
    let source = load_visually_hidden_component_source();
    let logic_source = load_source("../../components/visually-hidden/src/logic.rs");
    for needle in [
        "#[prop(optional, into)] is_focusable: Option<bool>",
        "#[prop(optional, into)] focusable: Option<bool>",
    ] {
        assert!(
            source.contains(needle),
            "visually_hidden api naming/migration contract should include `{needle}`.",
        );
    }
    assert!(
        logic_source.contains("FocusPropSource::resolve"),
        "visually_hidden naming migration should be normalized in logic.rs via typed props resolution.",
    );

    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/forms_extra_visually_hidden.rs",
    );
    assert!(
        docs_source.contains("<VisuallyHidden is_focusable=true>"),
        "docs should prefer the canonical `is_focusable` naming in public examples.",
    );
}

#[test]
fn visually_hidden_does_not_introduce_half_controlled_state_axis() {
    let source = load_source("../../components/visually-hidden/src/logic.rs");

    for forbidden in [
        "default_focusable",
        "on_focusable_change",
        "value:",
        "on_value_change",
        "default_value",
        "use_controllable_state",
        "signal(",
    ] {
        assert!(
            !source.contains(forbidden),
            "visually_hidden should stay stateless and avoid controlled/uncontrolled axis leakage via `{forbidden}`.",
        );
    }

    assert!(
        source.contains("FocusPropSource::resolve"),
        "visually_hidden should normalize props once in logic.rs and avoid internal state ownership.",
    );
}

#[test]
fn visually_hidden_state_normalization_is_centralized_in_logic_layer() {
    let module_source = load_visually_hidden_component_source();
    let logic_source = load_source("../../components/visually-hidden/src/logic.rs");

    assert!(
        module_source.contains("normalize_props(VisuallyHiddenLogicInput"),
        "visually_hidden mod.rs should consume normalized logic output.",
    );
    for forbidden in [
        ".or(focusable).unwrap_or(false)",
        "normalize_optional_text(",
        "resolve_state(",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "visually_hidden mod.rs should not perform logic-layer normalization via `{forbidden}`.",
        );
    }

    for needle in [
        "pub struct VisuallyHiddenLogicInput",
        "pub struct VisuallyHiddenLogicState",
        "pub fn normalize_props(input: VisuallyHiddenLogicInput) -> VisuallyHiddenLogicState",
    ] {
        assert!(
            logic_source.contains(needle),
            "visually_hidden logic layer should include `{needle}` for centralized state normalization.",
        );
    }
}

#[test]
fn visually_hidden_discrete_state_axes_are_type_constrained_by_enums() {
    let logic_source = load_source("../../components/visually-hidden/src/logic.rs");

    for needle in [
        "pub enum VisuallyHiddenFocusMode",
        "Hidden",
        "Focusable",
        "pub enum FocusPropSource",
        "IsFocusable",
        "FocusableAlias",
        "pub enum ClassNameSource",
        "Default",
        "Custom",
    ] {
        assert!(
            logic_source.contains(needle),
            "visually_hidden logic should model discrete state using enum variant `{needle}`.",
        );
    }
}

#[test]
fn visually_hidden_state_markers_are_observable_and_closed_set_contracts() {
    let module_source = load_visually_hidden_component_source();
    let logic_source = load_source("../../components/visually-hidden/src/logic.rs");

    for marker in [
        "data-focus-mode=state.focus_mode.as_attr()",
        "data-focus-source=state.focus_prop_source.as_attr()",
        "data-class-source=state.class_name_source.as_attr()",
        "data-focusable=state.primitive_state.focusable_attr",
        "data-custom-class=state.primitive_state.custom_class_attr",
    ] {
        assert!(
            module_source.contains(marker),
            "visually_hidden should expose stable observable state marker `{marker}`.",
        );
    }

    for closed_set_value in [
        "\"hidden\"",
        "\"focusable\"",
        "\"default\"",
        "\"is_focusable\"",
        "\"focusable\"",
        "\"custom\"",
    ] {
        assert!(
            logic_source.contains(closed_set_value),
            "visually_hidden marker attrs should map to closed-set value {closed_set_value}.",
        );
    }
}

#[test]
fn visually_hidden_machine_readable_state_contract_is_type_first_and_diagnostic_friendly() {
    let logic_source = load_source("../../components/visually-hidden/src/logic.rs");
    let view_source = load_source("../../components/visually-hidden/src/view.rs");

    for needle in [
        "pub enum VisuallyHiddenFocusMode",
        "pub enum FocusPropSource",
        "pub enum ClassNameSource",
        "pub struct VisuallyHiddenLogicState",
        "pub fn as_attr(self) -> &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "visually_hidden state contract should stay type-first via `{needle}`.",
        );
    }

    for marker in [
        "data-focus-mode=state.focus_mode.as_attr()",
        "data-focus-source=state.focus_prop_source.as_attr()",
        "data-class-source=state.class_name_source.as_attr()",
    ] {
        assert!(
            view_source.contains(marker),
            "visually_hidden should expose machine-readable marker `{marker}` from typed state.",
        );
    }

    for forbidden in [
        "pub focus_mode: String",
        "pub focus_prop_source: String",
        "pub class_name_source: String",
        "format!(\"{}\", state.focus_mode",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "visually_hidden logic should avoid stringly typed state axis token `{forbidden}`.",
        );
    }
}

#[test]
fn visually_hidden_styles_use_explicit_state_selectors_without_structural_guessing() {
    let module_source = load_visually_hidden_component_source();

    for needle in [
        ".ui-visually-hidden--focusable:focus-within",
        ".ui-visually-hidden[data-focus-mode=\"focusable\"]:focus-within",
        ".ui-visually-hidden[data-focus-mode=\"focusable\"]:active",
    ] {
        assert!(
            module_source.contains(needle),
            "visually_hidden styles should include explicit state selector `{needle}`.",
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", "style="] {
        assert!(
            !module_source.contains(forbidden),
            "visually_hidden styles should avoid structural guessing or inline style coupling via `{forbidden}`.",
        );
    }
}

#[test]
fn visually_hidden_state_primitives_source_is_ui_state_primitives_without_business_store_binding() {
    let logic_source = load_source("../../components/visually-hidden/src/logic.rs");

    for needle in [
        "use ui_state_primitives::visually_hidden::",
        "VisuallyHiddenStateInput",
        "resolve_state(",
        "normalize_optional_text(",
    ] {
        assert!(
            logic_source.contains(needle),
            "visually_hidden logic should consume state primitive contract `{needle}` from ui-state-primitives.",
        );
    }

    for forbidden in [
        "RwSignal",
        "ReadSignal",
        "WriteSignal",
        "signal(",
        "store",
        "Store",
        "use_store",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "visually_hidden logic should not bind business/global store container `{forbidden}`.",
        );
    }
}

#[test]
fn visually_hidden_has_no_async_loading_error_retry_contract() {
    let module_source = load_visually_hidden_component_source();
    let logic_source = load_source("../../components/visually-hidden/src/logic.rs");
    let combined = format!("{module_source}\n{logic_source}");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "aria_busy",
        "retry",
        "error",
        "use_async_action",
        "spawn_local",
        "async move",
        "Future",
    ] {
        assert!(
            !combined.contains(forbidden),
            "visually_hidden should remain async-free and avoid loading/error/retry protocol token `{forbidden}`.",
        );
    }
}

#[test]
fn visually_hidden_dx_contract_prefers_simple_default_api() {
    let module_source = load_visually_hidden_component_source();
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/forms_extra_visually_hidden.rs",
    );

    assert!(
        docs_source.contains("title=\"Hello World\""),
        "docs should expose a dedicated Hello World entry for the default visually-hidden usage path.",
    );
    assert!(
        docs_source.contains("<VisuallyHidden>\"Open account settings\"</VisuallyHidden>"),
        "docs default example should be a direct visually-hidden wrapper without extra wiring.",
    );

    for forbidden in ["state=", "ui_state_primitives", "ui_headless"] {
        assert!(
            !docs_source.contains(forbidden),
            "docs default path should not require manual state machine wiring via `{forbidden}`.",
        );
    }

    assert!(
        !module_source.contains("#[prop(into)] state"),
        "component API should not require a state object for basic usage.",
    );
}

#[test]
fn visually_hidden_is_not_a_composite_parent_item_api_surface() {
    let module_source = load_visually_hidden_component_source();
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/forms_extra_visually_hidden.rs",
    );

    for forbidden in [
        "ItemSpec",
        "item_specs",
        "labels",
        "titles",
        "panels",
        "items:",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "visually_hidden API should not expose composite parent/item contract token `{forbidden}`.",
        );
        assert!(
            !docs_source.contains(forbidden),
            "visually_hidden docs should not recommend composite parallel-slot contract token `{forbidden}`.",
        );
    }

    assert!(
        module_source.contains("pub fn VisuallyHidden("),
        "visually_hidden should remain a single-wrapper API surface, not a parent/item composition API.",
    );
}

#[test]
fn visually_hidden_a11y_i18n_locale_contract_uses_headless_and_passes_lang_dir() {
    let module_source = load_visually_hidden_component_source();

    for needle in [
        "ui_headless::a11y::{A11yDirection, locale_attrs}",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "lang=locale.lang",
        "dir=locale.dir",
    ] {
        assert!(
            module_source.contains(needle),
            "visually_hidden should wire a11y/i18n locale contract via `{needle}`.",
        );
    }

    for forbidden in [
        "aria_label=\"",
        "aria-label=\"",
        "Open account settings",
        "Skip to details",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "visually_hidden component source should not hardcode user-visible copy via `{forbidden}`.",
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn visually_hidden_semantic_contract_matrix_covers_applicable_branches() {
    let module_source = load_visually_hidden_component_source();
    let logic_source = load_source("../../components/visually-hidden/src/logic.rs");

    // Applicable semantic markers: a11y locale + state/source markers.
    for needle in [
        "data-focus-mode=state.focus_mode.as_attr()",
        "data-focus-source=state.focus_prop_source.as_attr()",
        "data-class-source=state.class_name_source.as_attr()",
        "lang=locale.lang",
        "dir=locale.dir",
    ] {
        assert!(
            module_source.contains(needle),
            "visually_hidden semantic contract matrix should include marker `{needle}`.",
        );
    }

    // Applicable branch matrix: default + canonical + legacy alias priority.
    for needle in [
        "FocusPropSource::resolve(None, None)",
        "FocusPropSource::resolve(Some(true), None)",
        "FocusPropSource::resolve(Some(false), Some(true))",
        "FocusPropSource::resolve(None, Some(true))",
    ] {
        assert!(
            logic_source.contains(needle),
            "visually_hidden logic tests should lock branch `{needle}`.",
        );
    }

    // Non-applicable branches must stay absent for this non-interactive wrapper.
    for forbidden in ["disabled", "on:keydown", "on:pointerdown"] {
        assert!(
            !module_source.contains(forbidden),
            "visually_hidden should keep non-applicable interaction branch `{forbidden}` out of component source.",
        );
    }
}

#[test]
fn visually_hidden_semantics_checks_do_not_use_visual_snapshot_as_primary_signal() {
    let source = load_source("tests/semantics.rs");

    let forbidden = [
        ["assert_", "snapshot!"].concat(),
        ["insta", "::"].concat(),
        ["snapbox", "::"].concat(),
        ["to_match_", "snapshot"].concat(),
    ];

    for forbidden in forbidden {
        assert!(
            !source.contains(&forbidden),
            "visually_hidden semantic tests should not rely on visual snapshot primitive `{forbidden}`.",
        );
    }
}

#[test]
fn visually_hidden_prefers_function_split_without_extra_local_components() {
    let view_source = load_source("../../components/visually-hidden/src/view.rs");

    let component_macro_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_macro_count, 1,
        "visually_hidden should keep only one top-level component; lightweight fragments must not be promoted to extra #[component] blocks.",
    );

    assert!(
        view_source.contains("let locale = locale_attrs(lang, dir);"),
        "visually_hidden should keep lightweight assembly as local function-level logic in view.rs.",
    );
}

#[test]
fn visually_hidden_static_fragment_stays_constant_and_minimal() {
    let view_source = load_source("../../components/visually-hidden/src/view.rs");
    let styles_source = load_source("../../components/visually-hidden/src/styles.rs");

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "visually_hidden should keep static fragment contract in a single CSS constant.",
    );

    for forbidden in [
        "<svg",
        "<footer",
        "<article",
        "<section",
        "String::from(\"<",
        "format!(\"<",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "visually_hidden should avoid generating large dynamic static fragments via `{forbidden}`.",
        );
    }
}

#[test]
fn visually_hidden_forbids_inner_html_injection_path() {
    let source = load_visually_hidden_component_source();
    assert!(
        !source.contains("inner_html"),
        "visually_hidden should not use inner_html; all content should remain typed children.",
    );
}

#[test]
fn visually_hidden_wasm_debug_capability_stays_feature_isolated() {
    let crate_root = load_source("src/lib.rs");
    let component_source = load_visually_hidden_component_source();

    for needle in ["macro_rules! wasm_debug_proxy"] {
        assert!(
            crate_root.contains(needle),
            "ui should keep wasm debug capability isolated through `{needle}`.",
        );
    }

    for forbidden in ["wasm_debug_proxy!", "observability::", "console_log"] {
        assert!(
            !component_source.contains(forbidden),
            "visually_hidden production contract should not couple to wasm-only debug hook `{forbidden}`.",
        );
    }
}

#[test]
fn visually_hidden_docs_workbench_uses_interactive_playground_contract() {
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/forms_extra_visually_hidden.rs",
    );

    for needle in [
        "<Playground title=\"Hello World\"",
        "<Playground title=\"Icon Button Accessible Label\"",
        "<Playground title=\"Focusable Skip Link\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "visually_hidden docs workbench should provide interactive playground entry `{needle}`.",
        );
    }
}

#[test]
fn visually_hidden_engineering_contract_stays_spec_free_and_runtime_agnostic() {
    let component_source = load_visually_hidden_component_source();
    let logic_source = load_source("../../components/visually-hidden/src/logic.rs");
    let combined = format!("{component_source}\n{logic_source}");

    for forbidden in [
        "serde::",
        "Serialize",
        "Deserialize",
        "tracing::",
        "tokio::",
        "async_std::",
        "Runtime",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "visually_hidden should keep runtime/spec concerns out of component contract via `{forbidden}`.",
        );
    }
}

#[test]
fn visually_hidden_ui_components_fixed_entry_layout_is_consistent() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "#[cfg(feature = \"component-visually_hidden\")]",
        "pub use ui_visually_hidden as visually_hidden;",
        "pub use visually_hidden::VisuallyHidden;",
        "#[cfg(feature = \"component-visually_hidden\")]\n    out.push_str(crate::visually_hidden::CSS);",
        "crate::css::push_components_css(&mut out);",
        "provide_ui_i18n(i18n);",
    ] {
        let combined = format!("{lib_source}\n{css_source}\n{root_source}");
        assert!(
            combined.contains(needle),
            "ui fixed entry layout should include `{needle}`.",
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    for forbidden_path in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !manifest_dir.join(forbidden_path).exists(),
            "ui should not host forbidden shared primitive file `{forbidden_path}`.",
        );
    }
}

#[test]
fn visually_hidden_component_directory_layout_matches_contract() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs"] {
        assert!(
            manifest_dir
                .join(format!("../../components/visually-hidden/src/{required}"))
                .exists(),
            "visually_hidden should keep required file `{required}`.",
        );
    }

    for forbidden in ["motion.rs", "spec.rs", "render.rs"] {
        assert!(
            !manifest_dir
                .join(format!("../../components/visually-hidden/src/{forbidden}"))
                .exists(),
            "visually_hidden should avoid non-applicable file `{forbidden}`.",
        );
    }
}

#[test]
fn visually_hidden_agent_contract_markers_are_schema_like_and_safe() {
    let view_source = load_source("../../components/visually-hidden/src/view.rs");
    let logic_source = load_source("../../components/visually-hidden/src/logic.rs");
    let combined = format!("{view_source}\n{logic_source}");

    for needle in [
        "data-focus-mode=state.focus_mode.as_attr()",
        "data-focus-source=state.focus_prop_source.as_attr()",
        "data-class-source=state.class_name_source.as_attr()",
    ] {
        assert!(
            combined.contains(needle),
            "visually_hidden should expose agent-readable state marker `{needle}`.",
        );
    }

    for forbidden in ["<script", "javascript:", "onerror=", "inner_html"] {
        assert!(
            !combined.contains(forbidden),
            "visually_hidden agent contract should keep a whitelist-safe render path without `{forbidden}`.",
        );
    }
}

#[test]
fn visually_hidden_streaming_semantics_are_not_required_for_snapshot_wrapper() {
    let source = load_visually_hidden_component_source();
    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "streaming",
        "fallback=snapshot",
    ] {
        assert!(
            !source.contains(forbidden),
            "visually_hidden should stay snapshot-compatible wrapper and avoid unnecessary streaming protocol token `{forbidden}`.",
        );
    }
}

#[test]
fn visually_hidden_docs_are_copy_paste_ready_and_beginner_focused() {
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/forms_extra_visually_hidden.rs",
    );
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "title=\"Hello World\"",
        "\"Default usage is a single semantic wrapper without extra state wiring.\"",
        "code_signal=hello_world_code",
    ] {
        assert!(
            docs_source.contains(needle),
            "visually_hidden docs should include beginner-friendly/copy-ready marker `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "CodeBlock code=resolved_code.get()",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground should keep copy-paste-ready infrastructure marker `{needle}`.",
        );
    }
}

#[test]
fn visually_hidden_anti_pattern_guards_hold_for_layering_and_api_surface() {
    let view_source = load_source("../../components/visually-hidden/src/view.rs");
    let logic_source = load_source("../../components/visually-hidden/src/logic.rs");
    let styles_source = load_source("../../components/visually-hidden/src/styles.rs");
    let state_source = load_source("../ui-state-primitives/src/visually_hidden.rs");
    let headless_a11y_source = load_source("../ui-headless/src/a11y.rs");

    for forbidden in ["view!", "class=", ".ui-"] {
        assert!(
            !state_source.contains(forbidden),
            "ui-state-primitives visually_hidden must stay DOM/style free and avoid `{forbidden}`.",
        );
    }

    for forbidden in [".ui-", "animation:", "transition:", "spring", "keyframes"] {
        assert!(
            !headless_a11y_source.contains(forbidden),
            "ui-headless a11y layer should avoid visual/animation orchestration token `{forbidden}`.",
        );
    }

    assert!(
        view_source.contains("normalize_props(VisuallyHiddenLogicInput"),
        "view layer should consume normalized logic output instead of hiding decision branches.",
    );
    assert!(
        !view_source.contains(".or(focusable).unwrap_or(false)"),
        "view layer should not duplicate default-resolution decisions.",
    );

    for forbidden in ["web_sys", "js_sys", "wasm_bindgen"] {
        assert!(
            !view_source.contains(forbidden),
            "public component surface should not leak platform detail `{forbidden}`.",
        );
    }

    for forbidden in ["labels", "titles", "panels", "ItemSpec"] {
        let combined = format!("{view_source}\n{logic_source}\n{styles_source}");
        assert!(
            !combined.contains(forbidden),
            "visually_hidden should avoid implicit parallel-array contract token `{forbidden}`.",
        );
    }
}
