use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(rel_path)
        .exists()
}

fn assert_line_guarded_by_cfg(source: &str, target_line: &str, expected_cfg_line: &str) {
    let lines: Vec<&str> = source.lines().collect();
    let mut matched = 0usize;

    for (idx, line) in lines.iter().enumerate() {
        if line.trim() != target_line {
            continue;
        }
        matched += 1;

        let mut prev_idx = idx;
        while prev_idx > 0 {
            prev_idx -= 1;
            let prev = lines[prev_idx].trim();
            if prev.is_empty() {
                continue;
            }
            assert_eq!(
                prev, expected_cfg_line,
                "line `{target_line}` must be guarded by `{expected_cfg_line}`"
            );
            break;
        }
    }

    assert!(
        matched > 0,
        "expected at least one `{target_line}` occurrence in guarded source"
    );
}

#[test]
fn tabs_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/tabs/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Tabs's `logic` module should stay private to avoid leaking implementation details into the public API."
    );
    assert!(
        !source.contains("pub mod view"),
        "Tabs's `view` module should stay private to avoid leaking internal module structure into the public API."
    );
}

#[test]
fn tabs_component_files_follow_layered_responsibilities() {
    let mod_source = load_source("src/tabs/mod.rs");
    let logic_source = load_source("src/tabs/logic.rs");
    let styles_source = load_source("src/tabs/styles.rs");
    let view_source = load_source("src/tabs/view.rs");
    let motion_source = load_source("src/tabs/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::TabsKeyboardActivation;",
        "pub use motion::TabsMotion;",
        "pub use view::Tabs;",
    ] {
        assert!(
            mod_source.contains(needle),
            "Tabs mod.rs should keep boundary token `{needle}`."
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;", "pub fn "] {
        assert!(
            !mod_source.contains(forbidden),
            "Tabs mod.rs should stay as export boundary only; found `{forbidden}`."
        );
    }

    for needle in [
        "pub fn normalize_selection_axis(",
        "pub fn normalize_disabled_axis(",
        "pub fn normalize_is_disabled(",
        "pub fn resolve_requested_selected_index(",
        "pub fn compose_class_name(",
        "pub fn resolve_motion_source(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tabs logic.rs should keep normalization/derivation helper `{needle}`."
        );
    }

    for forbidden in ["view! {", "NodeRef", "web_sys::", "on:pointer", "role=\""] {
        assert!(
            !logic_source.contains(forbidden),
            "Tabs logic.rs should not contain view/platform/event details `{forbidden}`."
        );
    }

    for needle in [
        "pub const CSS: &str",
        "var(--ui-",
        "data-selected",
        "data-disabled",
    ] {
        assert!(
            styles_source.contains(needle),
            "Tabs styles.rs should keep token-first static style contract `{needle}`."
        );
    }

    for forbidden in ["view! {", "Callback::new(", "on:click", "web_sys::"] {
        assert!(
            !styles_source.contains(forbidden),
            "Tabs styles.rs should avoid runtime logic/platform code `{forbidden}`."
        );
    }

    for needle in [
        "logic::normalize_disabled_axis(",
        "logic::normalize_selection_axis(",
        "logic::compose_class_name(",
        "use_controllable_state(",
        "tabs_list_a11y_attrs(",
        "tabs_tab_a11y_attrs(",
        "motion::attach_motion(",
    ] {
        assert!(
            view_source.contains(needle),
            "Tabs view.rs should assemble logic/headless/motion via `{needle}`."
        );
    }

    for forbidden in [
        "pub struct TabsState",
        "pub fn resolve_tabs_state(",
        "pub enum TabsSelectionTrigger",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Tabs view.rs should not re-declare primitive contracts `{forbidden}`."
        );
    }

    for needle in [
        "pub struct TabsMotion",
        "pub fn sanitize_motion(motion: TabsMotion) -> TabsMotion",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new",
    ] {
        assert!(
            motion_source.contains(needle),
            "Tabs motion.rs should keep motion mapping/attach contract `{needle}`."
        );
    }

    for forbidden in ["view! {", "role=", "aria-", "on:click=", "use_press("] {
        assert!(
            !motion_source.contains(forbidden),
            "Tabs motion.rs should avoid component semantics/event binding `{forbidden}`."
        );
    }
}

#[test]
fn tabs_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("src/tabs/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    for needle in [
        "fn render_tab_button(",
        "fn render_tab_panel(",
        "render_tab_button(",
        "render_tab_panel(",
    ] {
        assert!(
            view_source.contains(needle),
            "Tabs view should keep macro complexity split marker `{needle}`."
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 6,
        "Tabs view macro expansion should stay controlled after semantic split; got {view_macro_count} `view!` blocks."
    );

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Tabs should keep a single public component boundary."
    );

    for forbidden in [
        "let tabs_view = labels\n        .into_iter()\n        .take(item_count)\n        .enumerate()\n        .map({\n            let id_base = id_base.clone();\n            let roving = roving.clone();\n            let tab_refs = tab_refs.clone();\n            move |(index, label)| {\n                let tab_id = format!(\"{id_base}-tab-{index}\");",
        "view! {\n                    <button",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Tabs should avoid macro-overgrown inlined branch token `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test tabs_semantics tabs_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn tabs_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("src/tabs/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    for needle in [
        "fn render_tab_button(",
        ") -> AnyView {",
        "fn render_tab_panel(",
        "pub fn Tabs(",
    ] {
        assert!(
            view_source.contains(needle),
            "Tabs view should keep function-first split marker `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Tabs should keep only one `#[component]` (Tabs root), and keep local fragments as plain Rust functions."
    );

    for forbidden in [
        "#[component]\nfn render_tab_button(",
        "#[component]\nfn render_tab_panel(",
        "#[component]\r\nfn render_tab_button(",
        "#[component]\r\nfn render_tab_panel(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Tabs local fragments should stay plain functions, not extra components `{forbidden}`."
        );
    }

    for semantic_marker in [
        "data-slot=SLOT_TABS_TAB",
        "data-slot=SLOT_TABS_PANEL",
        "data-slot=SLOT_TABS_LIST",
    ] {
        assert!(
            view_source.contains(semantic_marker),
            "Tabs functional split should keep semantic marker `{semantic_marker}` stable."
        );
    }

    let script_needle = "cargo test -p ui-components --test tabs_semantics tabs_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn tabs_static_fragments_are_constantized_with_stable_semantics() {
    let view_source = load_source("src/tabs/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    for needle in [
        "const CLASS_TABS_TAB: &str = \"ui-tabs__tab\";",
        "const CLASS_TABS_PANEL: &str = \"ui-tabs__panel\";",
        "const CLASS_TABS_LIST: &str = \"ui-tabs__list\";",
        "const CLASS_TABS_INDICATOR: &str = \"ui-tabs__indicator\";",
        "const SLOT_TABS: &str = \"tabs\";",
        "const SLOT_TABS_LIST: &str = \"tabs-list\";",
        "const SLOT_TABS_INDICATOR: &str = \"tabs-indicator\";",
        "const SLOT_TABS_TAB: &str = \"tabs-tab\";",
        "const SLOT_TABS_PANEL: &str = \"tabs-panel\";",
        "const ROLE_TABPANEL: &str = \"tabpanel\";",
        "const ARIA_TRUE: &str = \"true\";",
        "const KEYBOARD_ACTIVATION_AUTOMATIC: &str = \"automatic\";",
        "const KEYBOARD_ACTIVATION_MANUAL: &str = \"manual\";",
    ] {
        assert!(
            view_source.contains(needle),
            "Tabs view should keep static fragment constants via `{needle}`."
        );
    }

    for needle in [
        "class=CLASS_TABS_TAB",
        "class=CLASS_TABS_PANEL",
        "class=CLASS_TABS_LIST",
        "class=CLASS_TABS_INDICATOR",
        "data-slot=SLOT_TABS",
        "data-slot=SLOT_TABS_LIST",
        "data-slot=SLOT_TABS_INDICATOR",
        "data-slot=SLOT_TABS_TAB",
        "data-slot=SLOT_TABS_PANEL",
        "role=ROLE_TABPANEL",
        "aria-hidden=ARIA_TRUE",
        "TabsKeyboardActivation::Automatic => KEYBOARD_ACTIVATION_AUTOMATIC",
        "TabsKeyboardActivation::Manual => KEYBOARD_ACTIVATION_MANUAL",
    ] {
        assert!(
            view_source.contains(needle),
            "Tabs view should consume static fragment constant `{needle}` in rendering."
        );
    }

    let script_needle = "cargo test -p ui-components --test tabs_semantics tabs_static_fragments_are_constantized_with_stable_semantics";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}

#[test]
fn tabs_inner_html_usage_is_explicitly_na_and_guarded() {
    let mod_source = load_source("src/tabs/mod.rs");
    let logic_source = load_source("src/tabs/logic.rs");
    let motion_source = load_source("src/tabs/motion.rs");
    let styles_source = load_source("src/tabs/styles.rs");
    let view_source = load_source("src/tabs/view.rs");
    let checklist_source = load_source("src/tabs/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-inner-html.sh");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "markdown_to_html(",
        "format!(\"<",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "Tabs should not use html injection path `{forbidden}`; this component has no trusted static-html requirement (N/A).",
        );
    }

    for required in [
        "- [ ] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "仅允许编译期常量或明确白名单内容进入 `inner_html`。",
        "严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。",
        "使用 `inner_html` 的节点必须补语义测试与安全回归说明。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Tabs checklist should keep inner_html safety governance rule `{required}`."
        );
    }

    for semantic_marker in [
        "role=ROLE_TABPANEL",
        "aria-label=list_a11y.aria_label",
        "data-slot=SLOT_TABS_TAB",
        "data-slot=SLOT_TABS_PANEL",
    ] {
        assert!(
            view_source.contains(semantic_marker),
            "Tabs semantic contract should remain explicit without inner_html fallback via `{semantic_marker}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test tabs_semantics tabs_inner_html_usage_is_explicitly_na_and_guarded";
    assert!(
        script_source.contains(script_needle),
        "inner-html check script should include `{script_needle}`.",
    );
}

#[test]
fn tabs_spec_boundary_stays_lightweight_while_button_keeps_schema_contract() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tabs_mod_source = load_source("src/tabs/mod.rs");
    let button_mod_source = load_source("src/button/mod.rs");

    assert!(
        !manifest_dir.join("src/tabs/spec.rs").exists(),
        "Tabs should not introduce a local spec.rs for simple component assembly."
    );
    assert!(
        manifest_dir.join("src/button/spec.rs").exists(),
        "Button should remain the canonical complex schema boundary with spec.rs."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "TabsSpec", "TabsSchema"] {
        assert!(
            !tabs_mod_source.contains(forbidden),
            "Tabs module should stay lightweight and avoid local spec boundary token `{forbidden}`."
        );
    }

    for needle in [
        "pub mod spec;",
        "pub use spec::{ButtonA11y, ButtonAction, ButtonIntent, ButtonSchema, ButtonSpec, ButtonText};",
    ] {
        assert!(
            button_mod_source.contains(needle),
            "Button module should keep canonical spec export `{needle}`."
        );
    }
}

#[test]
fn tabs_tree_shaking_feature_gates_stay_explicit() {
    let cargo_source = load_source("../ui-components/Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    assert!(
        cargo_source.contains("component-tabs = []"),
        "ui-components Cargo feature map must keep standalone `component-tabs` feature."
    );
    assert!(
        cargo_source.contains("default = [\"inject-css\", \"all-components\"]"),
        "default feature set should remain explicit and rely on feature flags."
    );

    assert_line_guarded_by_cfg(
        &lib_source,
        "pub mod tabs;",
        "#[cfg(feature = \"component-tabs\")]",
    );
    assert_line_guarded_by_cfg(
        &css_source,
        "out.push_str(crate::tabs::styles::CSS);",
        "#[cfg(feature = \"component-tabs\")]",
    );

    for forbidden in [
        "COMPONENT_REGISTRY",
        "ALL_COMPONENTS_REGISTRY",
        "GLOBAL_COMPONENT_MAP",
    ] {
        assert!(
            !lib_source.contains(forbidden) && !css_source.contains(forbidden),
            "tree-shaking boundary should avoid central reachability registry token `{forbidden}`."
        );
    }
}

#[test]
fn tabs_uses_headless_hooks() {
    let source = load_source("src/tabs/view.rs");

    for needle in [
        "use_roving_tabindex",
        "use_focus_ring",
        "use_hover",
        "use_press",
    ] {
        assert!(
            source.contains(needle),
            "Tabs should use headless `{needle}` hooks."
        );
    }
}

#[test]
fn tabs_attaches_indicator_motion_driver() {
    let source = load_source("src/tabs/view.rs");

    assert!(
        source.contains("motion::attach_motion"),
        "Tabs should attach a motion driver for the selection indicator (baseline-style feel)."
    );
}

#[test]
fn tabs_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/tabs/view.rs");

    for attr in [
        "data-slot=SLOT_TABS",
        "data-slot=SLOT_TABS_LIST",
        "data-slot=SLOT_TABS_INDICATOR",
        "data-slot=SLOT_TABS_TAB",
        "data-slot=SLOT_TABS_PANEL",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-selection-empty=move || state.get().selected_index.is_none().then_some(\"true\")",
        "data-has-disabled-tabs=move || state.get().has_disabled_tabs.then_some(\"true\")",
        "data-disabled-source=disabled_source",
        "data-control-mode=control_mode",
        "data-controlled=is_controlled.then_some(\"true\")",
        "data-uncontrolled=(!is_controlled).then_some(\"true\")",
        "data-keyboard-activation=match keyboard_activation",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
        "data-index=index",
        "data-selected",
        "data-hovered",
        "data-pressed",
        "data-disabled",
        "data-focused",
        "data-focus-visible",
    ] {
        assert!(
            source.contains(attr),
            "Tabs should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn tabs_uses_logic_state_model() {
    let view_source = load_source("src/tabs/view.rs");
    let logic_source = load_source("src/tabs/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/tabs.rs");

    for needle in [
        "pub use ui_state_primitives::tabs::{",
        "resolve_tabs_state",
        "normalize_index_skipping_disabled",
        "pub struct TabsSelectionAxisInput",
        "pub struct TabsDisabledAxis",
        "pub enum TabsDisabledSource",
        "pub fn normalize_selection_axis(",
        "pub fn normalize_disabled_axis(",
        "pub fn normalize_is_disabled(",
        "pub fn resolve_requested_selected_index(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tabs logic should consume centralized ui-state-primitives via `{needle}`."
        );
    }

    for needle in [
        "pub struct TabsState",
        "pub fn resolve_tabs_state(",
        "pub selected_index: Option<usize>",
        "pub has_disabled_tabs: bool",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Tabs primitives should define `{needle}` in ui-state-primitives."
        );
    }

    assert!(
        view_source.contains("resolve_tabs_state(item_count, selected.get(), has_disabled_tabs)"),
        "Tabs view should derive root state through resolve_tabs_state."
    );
}

#[test]
fn tabs_view_mounts_headless_tabs_contracts() {
    let view_source = load_source("src/tabs/view.rs");
    let headless_source = load_source("../ui-headless/src/tabs.rs");

    for needle in [
        "tabs_list_a11y_attrs",
        "tabs_tab_a11y_attrs",
        "resolve_tabs_selection_intent",
        "TabsInteractionKind::Press",
        "TabsInteractionKind::Focus",
    ] {
        assert!(
            view_source.contains(needle),
            "Tabs view should mount headless tabs semantic contract via `{needle}`."
        );
    }

    for needle in [
        "pub struct TabsListA11yAttrs",
        "pub struct TabsTabA11yAttrs",
        "pub fn tabs_list_a11y_attrs(",
        "pub fn tabs_tab_a11y_attrs(",
        "pub fn resolve_tabs_selection_intent(",
    ] {
        assert!(
            headless_source.contains(needle),
            "ui-headless tabs primitive should define `{needle}`."
        );
    }
}

#[test]
fn tabs_type_system_and_machine_readable_state_contract_is_explicit() {
    let logic_source = load_source("src/tabs/logic.rs");
    let view_source = load_source("src/tabs/view.rs");

    for needle in [
        "pub enum TabsControlMode",
        "pub enum TabsDisabledSource",
        "pub struct TabsDisabledAxis",
        "pub const fn as_attr(self) -> &'static str",
        "TabsDisabledSource::IsDisabled => \"is-disabled\"",
        "TabsDisabledSource::Disabled => \"disabled\"",
        "pub fn normalize_disabled_axis(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Tabs type contract should include `{needle}`."
        );
    }

    for needle in [
        "data-control-mode=control_mode",
        "data-disabled-source=disabled_source",
        "data-keyboard-activation=match keyboard_activation",
    ] {
        assert!(
            view_source.contains(needle),
            "Tabs machine-readable state markers should include `{needle}`."
        );
    }
}

#[test]
fn tabs_semantics_cover_roles_aria_and_state_markers() {
    let source = load_source("src/tabs/view.rs");

    for needle in [
        "role=list_a11y.role",
        "role=tab_role",
        "role=ROLE_TABPANEL",
        "aria-label=list_a11y.aria_label",
        "aria-selected=move || tab_aria_selected.get()",
        "aria-controls=tab_aria_controls",
        "aria-disabled=tab_aria_disabled",
        "aria-labelledby=tab_id",
        "hidden=move || !is_selected()",
        "data-disabled-source=disabled_source",
        "data-control-mode=control_mode",
        "data-controlled=is_controlled.then_some(\"true\")",
        "data-uncontrolled=(!is_controlled).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Tabs semantic contract should include `{needle}`."
        );
    }
}

#[test]
fn tabs_semantics_cover_keyboard_and_pointer_interaction_paths() {
    let source = load_source("src/tabs/view.rs");

    for needle in [
        "on:pointerdown=move |_| press.handlers.on_pointer_down.run(())",
        "on:pointerup=move |_| press.handlers.on_pointer_up.run(())",
        "on:pointercancel=move |_| press.handlers.on_pointer_cancel.run(())",
        "on:click=move |_| press.handlers.on_click.run(())",
        "on:keydown=on_key_down",
        "on:keyup=on_key_up",
        "on:focus=on_focus",
        "on:blur=on_blur",
        "roving.handlers.on_item_focus.run(index);",
        "resolve_tabs_selection_intent(",
        "TabsInteractionKind::Press",
        "TabsInteractionKind::Focus",
    ] {
        assert!(
            source.contains(needle),
            "Tabs interaction matrix should include `{needle}`."
        );
    }
}

#[test]
fn tabs_semantics_cover_wasm_and_non_wasm_platform_branches() {
    let view_source = load_source("src/tabs/view.rs");
    let motion_source = load_source("src/tabs/motion.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "fn focus_tab(tab_refs: &Arc<Vec<NodeRef<html::Button>>>, index: usize)",
        "fn focus_tab(_tab_refs: &Arc<Vec<NodeRef<html::Button>>>, _index: usize) {}",
    ] {
        assert!(
            view_source.contains(needle),
            "Tabs view should keep platform branch `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "Tabs motion should keep platform branch `{needle}`."
        );
    }
}

#[test]
fn tabs_ui_headless_feature_mutex_contract_is_guarded() {
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless feature mutex should be guarded in lib.rs by `{needle}`."
        );
    }

    for needle in [
        "echo \"[platform] compile guard: ui-headless web+ssr must fail\"",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "mutually exclusive",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            script_source.contains(needle),
            "platform guard script should enforce ui-headless feature mutex and dual compile paths via `{needle}`."
        );
    }
}

#[test]
fn tabs_ui_motion_non_wasm_noop_stub_contract_is_guarded() {
    let motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let tabs_motion_source = load_source("src/tabs/motion.rs");
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion non-wasm noop/stub contract should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            tabs_motion_source.contains(needle),
            "Tabs motion non-wasm branch should safely degrade via `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            script_source.contains(needle),
            "platform/toolchain checks should guard ui-motion non-wasm/wasm paths via `{needle}`."
        );
    }
}

#[test]
fn tabs_non_wasm_paths_avoid_browser_types_in_logic_and_view() {
    let logic_source = load_source("src/tabs/logic.rs");
    let view_source = load_source("src/tabs/view.rs");

    for forbidden in ["web_sys::", "wasm_bindgen", "js_sys::"] {
        assert!(
            !logic_source.contains(forbidden),
            "Tabs logic non-wasm contract should avoid browser token `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "Tabs view non-wasm contract should avoid browser token `{forbidden}`."
        );
    }
}

#[test]
fn tabs_motion_wasm_browser_bindings_are_cfg_scoped() {
    let motion_source = load_source("src/tabs/motion.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "use leptos::wasm_bindgen::{JsCast, closure::Closure};",
        "let resize_observer = StoredValue::new_local(None::<leptos::web_sys::ResizeObserver>);",
        "let _ = sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle),
            "Tabs motion platform split should include `{needle}`."
        );
    }
}

#[test]
fn tabs_api_supports_is_prefixed_boolean_with_legacy_alias() {
    let source = load_source("src/tabs/view.rs");
    let logic_source = load_source("src/tabs/logic.rs");

    for needle in [
        "is_disabled: Option<bool>",
        "disabled: bool",
        "let disabled_axis = logic::normalize_disabled_axis(is_disabled, disabled);",
        "let disabled = disabled_axis.is_disabled;",
        "let disabled_source = disabled_axis.source.as_attr();",
        "let selection_axis = logic::normalize_selection_axis(",
        "use_controllable_state(",
    ] {
        assert!(
            source.contains(needle),
            "Tabs API naming contract should include `{needle}`."
        );
    }

    assert!(
        !source.contains("unwrap_or(default_selected_index)"),
        "Tabs view should not inline default_selected_index fallback; normalize in logic.rs instead."
    );
    assert!(
        logic_source.contains("pub fn resolve_requested_selected_index("),
        "Tabs logic should own selected_index default priority resolution."
    );
}

#[test]
fn tabs_styles_include_motion_marker_contracts() {
    let source = load_source("src/tabs/styles.rs");

    for selector in [
        ".ui-tabs[data-motion-source=\"custom\"]",
        ".ui-tabs[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Tabs styles should include `{selector}` as stable custom-motion selectors."
        );
    }
}

#[test]
fn tabs_styles_define_indicator_css_vars() {
    let source = load_source("src/tabs/styles.rs");

    for var in [
        "--ui-tabs-indicator-x",
        "--ui-tabs-indicator-w",
        "--ui-tabs-indicator-o",
    ] {
        assert!(
            source.contains(var),
            "Tabs styles should define `{var}` so motion can update the indicator without re-rendering."
        );
    }
}

#[test]
fn tabs_styles_use_semantic_state_selectors_without_structural_guessing() {
    let source = load_source("src/tabs/styles.rs");

    for selector in [
        ".ui-tabs__tab[data-selected=\\\"true\\\"]",
        ".ui-tabs__tab[data-hovered=\\\"true\\\"]:not([data-disabled=\\\"true\\\"])",
        ".ui-tabs__tab[data-pressed=\\\"true\\\"]:not([data-disabled=\\\"true\\\"])",
        ".ui-tabs__tab[data-disabled=\\\"true\\\"]",
    ] {
        assert!(
            source.contains(selector),
            "Tabs styles should branch on semantic selector `{selector}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", "style="] {
        assert!(
            !source.contains(forbidden),
            "Tabs styles should avoid brittle structural/inline selector `{forbidden}`."
        );
    }
}

#[test]
fn tabs_styles_follow_token_first_static_contract() {
    let styles_source = load_source("src/tabs/styles.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "gap: var(--ui-space-md);",
        "gap: var(--ui-space-2xs);",
        "padding: var(--ui-space-2xs);",
        "padding: var(--ui-space-sm) var(--ui-space-md);",
        "top: var(--ui-space-2xs);",
        "left: var(--ui-space-2xs);",
        "height: calc(100% - (var(--ui-space-2xs) * 2));",
        "border-radius: var(--ui-radius-md);",
        "border-radius: var(--ui-radius-sm);",
        "font-size: var(--ui-font-size-150);",
        "box-shadow: var(--ui-shadow-sm);",
        "transition:",
        "background-color var(--ui-text-field-motion-duration) var(--ui-text-field-motion-easing),",
        "outline: var(--ui-button-focus-outline-width) solid var(--ui-focus-ring);",
        "outline-offset: var(--ui-button-focus-outline-offset);",
    ] {
        assert!(
            styles_source.contains(needle),
            "Tabs styles should consume token variable `{needle}`."
        );
    }

    for forbidden in [
        "gap: 12px;",
        "gap: 4px;",
        "padding: 4px;",
        "padding: 8px 12px;",
        "font-size: 14px;",
        "top: 4px;",
        "left: 4px;",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Tabs styles should avoid hardcoded visual constant `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-tabs\")]",
        "out.push_str(crate::tabs::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css aggregator should include tabs style injection token `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should inject aggregated component css via `{needle}`."
        );
    }
}

#[test]
fn tabs_default_theme_visual_baseline_contract_is_present() {
    let styles_source = load_source("src/tabs/styles.rs");

    for needle in [
        ".ui-tabs__tab[data-selected=\\\"true\\\"] {",
        "font-weight: 600;",
        ".ui-tabs__tab[data-hovered=\\\"true\\\"]:not([data-disabled=\\\"true\\\"]) {",
        "background: var(--ui-accent-soft);",
        ".ui-tabs__tab[data-pressed=\\\"true\\\"]:not([data-disabled=\\\"true\\\"]) {",
        "transform: scale(0.98);",
        ".ui-tabs__tab--focus-visible {",
        "outline: var(--ui-button-focus-outline-width) solid var(--ui-focus-ring);",
        ".ui-tabs__panel {",
        "border: 1px solid var(--ui-border);",
        "background: var(--ui-bg);",
        "padding: var(--ui-space-md);",
    ] {
        assert!(
            styles_source.contains(needle),
            "Tabs visual baseline should include `{needle}`."
        );
    }
}

#[test]
fn tabs_view_keeps_runtime_style_logic_outside_markup() {
    let source = load_source("src/tabs/view.rs");

    for forbidden in ["style=", "style:"] {
        assert!(
            !source.contains(forbidden),
            "Tabs view should not inline runtime style logic via `{forbidden}`."
        );
    }
}

#[test]
fn tabs_motion_uses_spring_animator() {
    let source = load_source("src/tabs/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Tabs motion should be spring-driven to match the repo's motion spec."
    );
}

#[test]
fn tabs_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/tabs/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: TabsMotion) -> TabsMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_spring_values()",
        "fn sanitize_motion_keeps_valid_custom_spring_values()",
    ] {
        assert!(
            source.contains(needle),
            "Tabs motion should include `{needle}` so invalid custom spring contracts cannot leak into runtime animation state.",
        );
    }
}

#[test]
fn tabs_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "pub(super) fn tabs() -> AnyView",
        "title=\"Tabs\"",
        "slug=\"tabs\"",
        "description=\"Tabs with roving tabindex, spring indicator motion, and default-theme visual baseline hierarchy.\"",
        "<Playground",
        "title=\"Hello World (Uncontrolled)\"",
        "description=\"Zero-wiring default path for beginners: no controlled state setup required.\"",
        "id_base=\"docs-tabs-hello\".to_string()",
        "<Playground title=\"Automatic + Controlled\" code_signal=code>",
        "<Playground title=\"Manual + Disabled\" code_signal=states_code>",
        "title=\"Workbench (Isolated Canvas + Optional Persist)\"",
        "<Tabs",
        "keyboard_activation=TabsKeyboardActivation::Automatic",
        "keyboard_activation=TabsKeyboardActivation::Manual",
        "disabled_indices=vec![2]",
    ] {
        assert!(
            source.contains(needle),
            "collections docs page should include `{needle}` for tabs coverage.",
        );
    }
}

#[test]
fn tabs_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "let manual_labels = vec![\"Profile\", \"Billing\", \"Team\"]",
        "let workbench_labels = vec![\"Overview\", \"Details\", \"Settings\"]",
        "let hello_world_code = Signal::derive(move || {",
        "<Tabs labels=vec![\"Overview\", \"Details\", \"Settings\"] id_base=\"tabs\".to_string()>",
        "let code = Signal::derive(move || {",
        "let (selected_auto, set_selected_auto) = signal(0_usize);",
        "let (selected_manual, set_selected_manual) = signal(1_usize);",
        "let persisted_tabs_workbench_selected = load_tabs_workbench_selected();",
        "let (tabs_workbench_selected, set_tabs_workbench_selected) =",
        "id_base=\"docs-tabs\".to_string()",
        "labels=vec![\"Overview\", \"Details\", \"Settings\"]",
        "\"Arrow keys move + select in automatic mode.\"",
        "\"selected: \"",
        "id_base=\"docs-tabs-manual\".to_string()",
        "labels=manual_labels",
        "\"Manual mode: focus moves first, Enter/Space commits.\"",
        "\"Current selected index reflects committed tab.\"",
        "\"This tab is disabled and skipped by roving focus.\"",
        "\"Default theme baseline: clear hierarchy, layered contrast, and explicit hover/focus feedback.\"",
        "\"manual selected: \"",
        "\"disabled tab index: 2\"",
        "\"Beginner path first; advanced controls follow below.\"",
        "data-slot=\"tabs-workbench\"",
        "data-slot=\"tabs-workbench-canvas\"",
        "\" Persist selected index (optional)\"",
    ] {
        assert!(
            source.contains(needle),
            "tabs docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn tabs_check2_documents_docs_sync_and_interactive_playground_rules() {
    let checklist_source = load_source("src/tabs/check2.md");

    for required in [
        "- [ ] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
        "- [ ] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Tabs checklist should keep docs/playground governance rule `{required}`."
        );
    }
}

#[test]
fn tabs_docs_api_names_and_defaults_match_logic_contract() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let view_source = load_source("src/tabs/view.rs");
    let logic_source = load_source("src/tabs/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/tabs.rs");

    for needle in [
        "#[prop(optional)] default_selected_index: usize,",
        "#[prop(optional)] selected_index: Option<ReadSignal<usize>>,",
        "#[prop(optional)] on_selection_change: Option<Callback<usize>>,",
        "#[prop(optional)] disabled_indices: Vec<usize>,",
        "pub fn resolve_requested_selected_index(",
        "pub fn resolve_tabs_state(",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || primitive_source.contains(needle),
            "tabs public/default contract should keep `{needle}`."
        );
    }

    for needle in [
        "title=\"Hello World (Uncontrolled)\"",
        "<Tabs labels=vec![\"Overview\", \"Details\", \"Settings\"] id_base=\"tabs\".to_string()>",
        "title=\"Automatic + Controlled\"",
        "selected_index=selected_auto",
        "on_selection_change=on_auto_change",
        "title=\"Manual + Disabled\"",
        "disabled_indices=vec![2]",
    ] {
        assert!(
            docs_source.contains(needle),
            "tabs docs should keep API/default/state matrix marker `{needle}`."
        );
    }
}

#[test]
fn tabs_documentation_as_product_keeps_beginner_path_before_advanced_controls() {
    let checklist_source = load_source("src/tabs/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let core_catalog_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_core_catalog.rs");

    for required in [
        "- [ ] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Tabs checklist should keep documentation-as-product rule `{required}`."
        );
    }

    let tabs_start = docs_source
        .find("pub(super) fn tabs() -> AnyView {")
        .expect("tabs docs source should include tabs page function");
    let tabs_section = &docs_source[tabs_start..];

    let hello_pos = tabs_section
        .find("title=\"Hello World (Uncontrolled)\"")
        .expect("tabs docs should include hello-world playground");
    let controlled_pos = tabs_section
        .find("title=\"Automatic + Controlled\"")
        .expect("tabs docs should include controlled playground");
    let workbench_pos = tabs_section
        .find("title=\"Workbench (Isolated Canvas + Optional Persist)\"")
        .expect("tabs docs should include workbench playground");
    assert!(
        hello_pos < controlled_pos && controlled_pos < workbench_pos,
        "tabs docs should present beginner default path before advanced controlled/workbench sections."
    );

    let needle = "collections_core_catalog::TABS_DOC";
    assert!(
        pages_source.contains(needle),
        "tabs docs index should keep pages catalog marker `{needle}`."
    );

    for needle in ["slug: \"tabs\",", "group: \"Collections\","] {
        assert!(
            core_catalog_source.contains(needle),
            "tabs docs index should keep core catalog marker `{needle}`."
        );
    }
}

#[test]
fn tabs_check2_documents_source_first_copy_paste_rules() {
    let checklist_source = load_source("src/tabs/check2.md");

    for required in [
        "- [ ] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Tabs checklist should keep source-first copy-paste governance rule `{required}`."
        );
    }
}

#[test]
fn tabs_docs_source_is_copy_paste_ready_with_imports_and_copy_control() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_field_button_contract.spec.mjs");

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        ".map(|snippet| compose_copy_ready_code(&snippet, &code_imports.get_value()))",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground source-first copy pipeline should keep token `{needle}`."
        );
    }

    for needle in [
        "docs-app field-button playground source is copy-paste ready",
        "toHaveAttribute(\"data-copyable\", \"true\")",
        "toHaveAttribute(\"aria-label\", /Copy to clipboard/i)",
    ] {
        assert!(
            e2e_source.contains(needle),
            "docs copy-flow e2e evidence should keep acceptance token `{needle}`."
        );
    }

    for needle in [
        "let hello_world_code = Signal::derive(move || {",
        "let code = Signal::derive(move || {",
        "let states_code = Signal::derive(move || {",
        "let workbench_code = Signal::derive(move || {",
        "title=\"Hello World (Uncontrolled)\"",
        "title=\"Workbench (Isolated Canvas + Optional Persist)\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "tabs docs source snippets should keep copy-ready token `{needle}`."
        );
    }
}

#[test]
fn tabs_check2_documents_heroui_alignment_rules() {
    let checklist_source = load_source("src/tabs/check2.md");

    for required in [
        "- [ ] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Tabs checklist should keep HeroUI alignment governance rule `{required}`."
        );
    }
}

#[test]
fn tabs_heroui_alignment_doc_and_docs_entry_stay_in_sync() {
    let heroui_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let catalog_source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_core_catalog.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let view_source = load_source("src/tabs/view.rs");

    for needle in [
        "### Tabs 同步记录（2026-02-17）",
        "`Tabs` 公开参数保持 `labels/id_base`",
        "`default_selected_index/selected_index/on_selection_change`",
        "`Hello World (Uncontrolled)`、`Automatic + Controlled`、`Manual + Disabled`、`Workbench (Isolated Canvas + Optional Persist)`",
        "`#/components/tabs` 可索引访问",
    ] {
        assert!(
            heroui_source.contains(needle),
            "HeroUI strategy doc should keep tabs sync token `{needle}`."
        );
    }

    let needle = "collections_core_catalog::TABS_DOC";
    assert!(
        pages_source.contains(needle),
        "docs pages catalog should expose tabs entry marker `{needle}`."
    );

    for needle in [
        "pub(super) const TABS_DOC: ComponentDoc = ComponentDoc {",
        "slug: \"tabs\",",
        "page: super::collections::tabs,",
    ] {
        assert!(
            catalog_source.contains(needle),
            "collections core catalog should expose tabs document entry `{needle}`."
        );
    }

    for needle in [
        "title=\"Hello World (Uncontrolled)\"",
        "title=\"Automatic + Controlled\"",
        "title=\"Manual + Disabled\"",
        "title=\"Workbench (Isolated Canvas + Optional Persist)\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "tabs docs page should keep indexed/example marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] keyboard_activation: TabsKeyboardActivation,",
        "#[prop(optional)] default_selected_index: usize,",
        "#[prop(optional)] selected_index: Option<ReadSignal<usize>>,",
        "#[prop(optional)] on_selection_change: Option<Callback<usize>>,",
        "#[prop(optional)] disabled_indices: Vec<usize>,",
    ] {
        assert!(
            view_source.contains(needle),
            "tabs runtime API should keep token `{needle}` to prevent docs/implementation drift."
        );
    }
}

#[test]
fn tabs_check2_documents_antipattern_guardrails_and_merge_gate_rules() {
    let checklist_source = load_source("src/tabs/check2.md");

    for required in [
        "### 8. 明确禁止的反模式",
        "在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "在 `ui-headless` 写视觉和动画编排。",
        "在 `view` 层隐藏关键状态决策。",
        "新增参数但不纳入统一命名与契约。",
        "用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "公共 API 泄露底层实现细节类型。",
        "用临时补丁破坏跨组件一致性。",
        "明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
        "### 9. 合并门禁（最终裁决）",
        "- [ ] 架构正确（边界不破）。",
        "- [ ] 行为正确（状态与交互语义成立）。",
        "- [ ] 可访问性达标（默认可用）。",
        "- [ ] 可测试（契约可断言）。",
        "- [ ] 文档与示例同步更新。",
        "- [ ] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Tabs checklist should keep anti-pattern/merge-gate rule `{required}`."
        );
    }
}

#[test]
fn tabs_antipattern_guardrails_are_explicit_and_enforced() {
    let primitive_source = load_source("../ui-state-primitives/src/tabs.rs");
    let headless_source = load_source("../../crates/ui-headless/src/tabs.rs");
    let logic_source = load_source("src/tabs/logic.rs");
    let view_source = load_source("src/tabs/view.rs");
    let mod_source = load_source("src/tabs/mod.rs");
    let semantics_source = load_source("tests/tabs_semantics.rs");

    for forbidden in [
        "use leptos",
        "web_sys",
        "view! {",
        "NodeRef",
        "on:click",
        "on:keydown",
    ] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives tabs primitive must stay DOM/event-free; found `{forbidden}`."
        );
    }

    for forbidden in [
        "class=",
        ".css",
        "@keyframes",
        "animation:",
        "transition:",
        "style.set_property",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "ui-headless tabs primitive must not carry visual/animation orchestration marker `{forbidden}`."
        );
    }

    for needle in [
        "logic::normalize_disabled_axis(",
        "logic::normalize_selection_axis(",
        "logic::resolve_requested_selected_index(",
        "resolve_tabs_state(",
        "resolve_tabs_selection_intent(",
    ] {
        assert!(
            view_source.contains(needle),
            "tabs view should consume normalized logic/headless outputs via `{needle}`."
        );
    }

    for forbidden in [
        "pub struct TabsControlMode",
        "pub struct TabsSelectionAxisInput",
        "pub enum TabsDisabledSource",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "tabs view must not redefine logic primitives via `{forbidden}`."
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;", "web_sys", "HtmlElement"] {
        assert!(
            !mod_source.contains(forbidden),
            "tabs public API should not leak platform/internal marker `{forbidden}`."
        );
    }

    assert!(
        semantics_source.contains(
            "tabs_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope"
        ) && semantics_source
            .contains("tabs_engineering_contract_avoids_runtime_leaks_in_public_api_surface"),
        "tabs semantics suite should enforce naming/contract/runtime anti-pattern guards."
    );

    assert!(
        logic_source.contains("pub fn normalize_selection_axis(")
            && logic_source.contains("pub fn normalize_disabled_axis("),
        "tabs logic should keep reusable normalization primitives explicit in logic.rs."
    );
}

#[test]
fn tabs_check2_documents_e2e_selector_and_regression_rules() {
    let checklist_source = load_source("src/tabs/check2.md");

    for required in [
        "- [ ] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
        "- [ ] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Tabs checklist should keep e2e/repeatable-regression governance rule `{required}`."
        );
    }
}

#[test]
fn tabs_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_tabs_contract.spec.mjs");

    for needle in [
        "/#/components/tabs",
        "body:not(:has(#boot))",
        "section.playground",
        "[data-slot=\"tabs-workbench\"]",
        "[data-slot=\"playground-controls\"]",
        "[data-slot=\"tabs-workbench-canvas\"] [data-slot=\"tabs\"]",
        "[data-slot=\"tabs-list\"]",
        "[data-slot=\"tabs-indicator\"]",
        "toHaveAttribute(\"data-control-mode\", \"controlled\")",
        "toHaveAttribute(\"data-keyboard-activation\", \"automatic\")",
        "toHaveAttribute(\"data-selected-index\", \"1\")",
        "toHaveAttribute(\"data-selected\", \"true\")",
        "toHaveCSS(\"opacity\", \"1\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "tabs e2e contract should include semantic/settled marker `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "tabs e2e contract should avoid unstable fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn tabs_e2e_key_flow_covers_keyboard_focus_and_semantic_state_sync() {
    let e2e_source = load_source("../../e2e/tests/docs_app_tabs_contract.spec.mjs");

    for needle in [
        "keyboard focus path is repeatable and semantic",
        "await tab0.focus();",
        "await expect(tab0).toBeFocused();",
        "await page.keyboard.press(\"ArrowRight\");",
        "await expect(tab1).toBeFocused();",
        "await expect(tabsRoot).toHaveAttribute(\"data-selected-index\", \"1\")",
        "await expect(tab1).toHaveAttribute(\"aria-selected\", \"true\")",
        "await expect(panel1).toBeVisible();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "tabs e2e key-flow contract should include `{needle}`."
        );
    }
}

#[test]
fn tabs_playground_acceptance_path_is_repeatable_in_e2e_suite() {
    let coverage_e2e_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let tabs_e2e_source = load_source("../../e2e/tests/docs_app_tabs_contract.spec.mjs");

    for needle in [
        "docs-app components pages render playgrounds (sample)",
        "docs-app components pages render playgrounds (all)",
        "await expect(page.locator(\"section.playground\").first()).toBeVisible();",
        "await expect(page.locator(`[data-slot=\"${slug}\"]`).first()).toBeVisible();",
    ] {
        assert!(
            coverage_e2e_source.contains(needle),
            "docs components coverage e2e should keep repeatable playground acceptance marker `{needle}`."
        );
    }

    for needle in [
        "docs-app tabs workbench uses semantic selectors and settled marker waits",
        "docs-app tabs keyboard focus path is repeatable and semantic",
        "await page.goto(\"/#/components/tabs\");",
        "toHaveAttribute(\"data-selected-index\", \"1\")",
        "toHaveAttribute(\"aria-selected\", \"true\")",
    ] {
        assert!(
            tabs_e2e_source.contains(needle),
            "tabs e2e should keep repeatable key-flow marker `{needle}`."
        );
    }
}

#[test]
fn tabs_e2e_check_script_covers_selector_and_key_flow_contracts() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-tabs.sh");

    for needle in [
        "cargo test -p ui-components --test tabs_semantics tabs_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui-components --test tabs_semantics tabs_e2e_key_flow_covers_keyboard_focus_and_semantic_state_sync",
    ] {
        assert!(
            script_source.contains(needle),
            "tabs e2e check script should enforce `{needle}`."
        );
    }
}

#[test]
fn tabs_check2_documents_component_directory_standard_file_rules() {
    let checklist_source = load_source("src/tabs/check2.md");

    for required in [
        "- [ ] 组件目录标准文件落点正确。",
        "`<component>/mod.rs`：最小稳定导出面，存在且无过度导出。",
        "`<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。",
        "`<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。",
        "`<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。",
        "`<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。",
        "`<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Tabs checklist should keep component-directory governance rule `{required}`."
        );
    }
}

#[test]
fn tabs_component_directory_has_standard_file_layout() {
    for required in [
        "src/tabs/mod.rs",
        "src/tabs/logic.rs",
        "src/tabs/styles.rs",
        "src/tabs/view.rs",
        "src/tabs/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "tabs component directory should include required file `{required}`."
        );
    }

    for forbidden in ["src/tabs/render.rs", "src/tabs/spec.rs"] {
        assert!(
            !path_exists(forbidden),
            "tabs component directory should not include `{forbidden}`."
        );
    }
}

#[test]
fn tabs_mod_rs_keeps_minimal_stable_exports() {
    let mod_source = load_source("src/tabs/mod.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::TabsKeyboardActivation;",
        "pub use motion::TabsMotion;",
        "pub use view::Tabs;",
    ] {
        assert!(
            mod_source.contains(needle),
            "tabs/mod.rs should include stable export marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "web_sys",
        "leptos::web_sys",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "tabs/mod.rs should not leak internal/platform marker `{forbidden}`."
        );
    }
}

#[test]
fn tabs_component_file_responsibilities_remain_scoped() {
    let logic_source = load_source("src/tabs/logic.rs");
    let styles_source = load_source("src/tabs/styles.rs");
    let view_source = load_source("src/tabs/view.rs");
    let motion_source = load_source("src/tabs/motion.rs");

    for forbidden in [
        "view! {",
        "on:pointer",
        "on:keydown",
        "aria-",
        "data-slot",
        "NodeRef<",
        "web_sys",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "tabs/logic.rs should stay normalization-only; found `{forbidden}`."
        );
    }

    for required in ["var(--ui-", "pub const CSS: &str ="] {
        assert!(
            styles_source.contains(required),
            "tabs/styles.rs should keep token-first CSS marker `{required}`."
        );
    }

    for forbidden in ["#[component]", "use leptos::", "on:click=", "view!"] {
        assert!(
            !styles_source.contains(forbidden),
            "tabs/styles.rs should stay static style contract; found `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn Tabs(",
        "use_roving_tabindex(",
        "tabs_list_a11y_attrs(",
        "tabs_tab_a11y_attrs(",
        "render_tab_button(",
        "render_tab_panel(",
    ] {
        assert!(
            view_source.contains(required),
            "tabs/view.rs should keep rendering + headless mount marker `{required}`."
        );
    }

    for forbidden in [
        "pub fn resolve_tabs_state(",
        "ui_state_primitives::tabs::resolve_tabs_state(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "tabs/view.rs should not bypass logic boundary with `{forbidden}`."
        );
    }

    for required in [
        "pub struct TabsMotion",
        "pub fn attach_motion(",
        "sanitize_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(required),
            "tabs/motion.rs should keep motion-contract marker `{required}`."
        );
    }

    for forbidden in ["aria-", "data-slot", "on:pointer", "on:click"] {
        assert!(
            !motion_source.contains(forbidden),
            "tabs/motion.rs should not carry view semantics; found `{forbidden}`."
        );
    }
}

#[test]
fn tabs_component_files_check_script_covers_directory_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    for needle in [
        "cargo test -p ui-components --test tabs_semantics tabs_component_directory_has_standard_file_layout",
        "cargo test -p ui-components --test tabs_semantics tabs_mod_rs_keeps_minimal_stable_exports",
        "cargo test -p ui-components --test tabs_semantics tabs_component_file_responsibilities_remain_scoped",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce `{needle}`."
        );
    }
}

#[test]
fn tabs_check2_documents_ui_components_entrypoint_rules() {
    let checklist_source = load_source("src/tabs/check2.md");

    for required in [
        "- [ ] `ui-components` 固定入口文件落点正确。",
        "`crates/ui-components/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui-components/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui-components/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-components/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui-components/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui-components/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui-components/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Tabs checklist should keep ui-components entrypoint governance rule `{required}`."
        );
    }
}

#[test]
fn tabs_ui_components_entry_files_keep_feature_gated_public_surface_and_no_platform_leaks() {
    let lib_source = load_source("src/lib.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-tabs\")]",
        "pub mod tabs;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod css;",
        "leptos::web_sys",
        "web_sys::",
        "wasm_bindgen",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-components lib entry should not leak platform/internal marker `{forbidden}`."
        );
    }
}

#[test]
fn tabs_ui_components_css_registry_remains_feature_gated_and_non_global() {
    let css_source = load_source("src/css.rs");

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-tabs\")]",
        "out.push_str(crate::tabs::styles::CSS);",
        "#[cfg(feature = \"component-active_highlight\")]",
        "out.push_str(crate::active_highlight::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css registry should keep feature-gated marker `{needle}`."
        );
    }
}

#[test]
fn tabs_ui_root_centralizes_theme_injection_and_i18n_context() {
    let root_source = load_source("src/root.rs");

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n};",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if let Some(overrides) = semantic_overrides.get_value() {",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "data-theme-scheme",
        "data-theme-color",
        "data-theme-system",
        "data-theme-scale",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized theme/i18n marker `{needle}`."
        );
    }
}

#[test]
fn tabs_active_highlight_stays_shared_motion_primitive_without_component_semantics() {
    let source = load_source("src/active_highlight.rs");

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "Accordion",
        "Button",
        "Sidebar",
        "aria-",
        "data-slot",
        "on:click",
    ] {
        assert!(
            !source.contains(forbidden),
            "active_highlight should stay generic and avoid component business marker `{forbidden}`."
        );
    }
}

#[test]
fn tabs_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present() {
    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden),
            "ui-components forbidden entrypoint file should not exist: `{forbidden}`."
        );
    }

    for required in [
        "../../crates/ui-headless/src/controllable_state.rs",
        "../../crates/ui-headless/src/presence.rs",
        "../../crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            path_exists(required),
            "ui-headless canonical primitive file should exist: `{required}`."
        );
    }

    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            controllable_state_source.contains(needle)
                || presence_source.contains(needle)
                || a11y_source.contains(needle),
            "headless canonical primitive files should keep marker `{needle}`."
        );
    }
}

#[test]
fn tabs_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-components-entrypoints.sh");

    let needle = "cargo test -p ui-components --test tabs_semantics tabs_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present";
    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn tabs_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show settings\"",
        "\"Show code\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }
}

#[test]
fn tabs_dx_workbench_supports_optional_state_persistence_and_isolated_canvas() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "TABS_WORKBENCH_STORAGE_KEY",
        "fn load_tabs_workbench_selected() -> Option<usize>",
        "fn save_tabs_workbench_selected(selected_index: usize)",
        "fn clear_tabs_workbench_selected()",
        "title=\"Workbench (Isolated Canvas + Optional Persist)\"",
        "description=\"Tune keyboard/disabled semantics while preserving context, with optional selected-index persistence.\"",
        "let (tabs_workbench_manual_mode, set_tabs_workbench_manual_mode) = signal(false);",
        "let (tabs_workbench_disable_settings, set_tabs_workbench_disable_settings) = signal(false);",
        "let (tabs_workbench_persist_state, set_tabs_workbench_persist_state) =",
        "Effect::new(move |_| {",
        "save_tabs_workbench_selected(selected_index);",
        "clear_tabs_workbench_selected();",
        "\" Persist selected index (optional)\"",
        "data-slot=\"tabs-workbench-controls\"",
        "data-slot=\"tabs-workbench\"",
        "data-slot=\"tabs-workbench-canvas\"",
    ] {
        assert!(
            source.contains(needle),
            "tabs workbench should keep DX marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            source.contains(needle),
            "tabs workbench persistence should keep platform guard `{needle}`."
        );
    }
}

#[test]
fn tabs_dx_check_script_covers_hot_reload_and_workbench_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test tabs_semantics tabs_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test tabs_semantics tabs_dx_workbench_supports_optional_state_persistence_and_isolated_canvas",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn tabs_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/tabs/mod.rs");
    let logic_source = load_source("src/tabs/logic.rs");
    let view_source = load_source("src/tabs/view.rs");
    let styles_source = load_source("src/tabs/styles.rs");
    let motion_source = load_source("src/tabs/motion.rs");
    let checklist_source = load_source("src/tabs/check2.md");

    assert!(
        !manifest_dir.join("src/tabs/spec.rs").exists(),
        "Tabs should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-tabs = []"),
        "Tabs feature should stay lightweight without serde/spec dependency fan-out."
    );
    assert!(
        !cargo_source.contains("component-tabs = [\"dep:serde\"")
            && !cargo_source.contains("component-tabs = [\"dep:serde_json\""),
        "Tabs should not opt into serde/spec migration dependencies without an explicit schema contract."
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "schema_version",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Tabs engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
        );
    }

    for required in [
        "- [ ] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Tabs checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
fn tabs_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("src/tabs/mod.rs"),
        load_source("src/tabs/logic.rs"),
        load_source("src/tabs/view.rs"),
        load_source("src/tabs/styles.rs"),
        load_source("src/tabs/motion.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_components::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("tabs-wasm-debug"),
        "Tabs should not define component-local tracing feature when no local debug event/replay contract exists."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::tabs::",
        "const TABS_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Tabs should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn tabs_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/tabs/mod.rs");
    let logic_source = load_source("src/tabs/logic.rs");
    let view_source = load_source("src/tabs/view.rs");
    let styles_source = load_source("src/tabs/styles.rs");
    let motion_source = load_source("src/tabs/motion.rs");

    let sources = [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
    ];
    for source in sources {
        for forbidden in [
            "tokio",
            "tokio::",
            "async_std",
            "async_std::",
            "async-std",
            "runtime::Handle",
            "smol::",
            "spawn_blocking(",
        ] {
            assert!(
                !source.contains(forbidden),
                "Tabs engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "Tabs public module boundary should not leak web_sys types."
    );
}

#[test]
fn tabs_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test tabs_semantics tabs_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui-components --test tabs_semantics tabs_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-components --test tabs_semantics tabs_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn tabs_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("src/tabs/check2.md");

    for required in [
        "- [ ] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Tabs checklist should keep Agent Contract governance rule `{required}`."
        );
    }
}

#[test]
fn tabs_check2_documents_semantics_first_testing_rules() {
    let checklist_source = load_source("src/tabs/check2.md");

    for required in [
        "- [ ] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Tabs checklist should keep semantics-first testing rule `{required}`."
        );
    }
}

#[test]
fn tabs_agent_contract_markers_are_schema_like_and_machine_readable() {
    let view_source = load_source("src/tabs/view.rs");
    let logic_source = load_source("src/tabs/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/tabs.rs");
    let combined = format!("{view_source}\n{logic_source}\n{primitives_source}");

    for marker in [
        "data-control-mode=control_mode",
        "data-disabled-source=disabled_source",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-keyboard-activation=match keyboard_activation",
        "data-motion-source=motion_source",
        "data-custom-motion=custom_motion",
        "data-selected=move || tab_is_selected.get().then_some(\"true\")",
        "data-disabled=tab_is_disabled.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(marker),
            "Tabs should expose agent-readable machine marker `{marker}`."
        );
    }

    for typed_source in [
        "pub enum TabsControlMode",
        "pub enum TabsDisabledSource",
        "pub struct TabsSelectionAxisInput",
        "pub const fn as_attr(self) -> &'static str",
        "pub struct TabsState",
        "pub fn resolve_tabs_state(",
    ] {
        assert!(
            combined.contains(typed_source),
            "Tabs Agent Contract values should remain type-derived via `{typed_source}`."
        );
    }
}

#[test]
fn tabs_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let view_source = load_source("src/tabs/view.rs");
    let logic_source = load_source("src/tabs/logic.rs");
    let primitives_source = load_source("../../crates/ui-state-primitives/src/tabs.rs");
    let combined = format!("{view_source}\n{logic_source}\n{primitives_source}");

    for forbidden in [
        "data-ui-schema=",
        "data-ui-schema-version=",
        "data-ui-intent=",
        "data-ui-action=",
        "data-ui-state=",
        "data-ui-source=",
        "intent=\"",
        "action=\"",
        "format!(\"data-",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Tabs should avoid free-form/fake schema field token `{forbidden}`."
        );
    }

    for required_interaction in [
        "on:click=move |_| press.handlers.on_click.run(())",
        "on:keydown=on_key_down",
    ] {
        assert!(
            view_source.contains(required_interaction),
            "Tabs interactive intent/action path should remain explicit via `{required_interaction}`."
        );
    }
}

#[test]
fn tabs_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("src/tabs/view.rs");
    let logic_source = load_source("src/tabs/logic.rs");
    let styles_source = load_source("src/tabs/styles.rs");
    let mod_source = load_source("src/tabs/mod.rs");
    let motion_source = load_source("src/tabs/motion.rs");
    let combined =
        format!("{view_source}\n{logic_source}\n{styles_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Tabs Agent Contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn tabs_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test tabs_semantics tabs_agent_contract_markers_are_schema_like_and_machine_readable",
        "cargo test -p ui-components --test tabs_semantics tabs_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "cargo test -p ui-components --test tabs_semantics tabs_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui-components --test tabs_semantics tabs_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui-components --test tabs_semantics tabs_semantic_markers_changed_in_view_must_be_covered_by_semantics_tests",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn tabs_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let checklist_source = load_source("src/tabs/check2.md");

    for required in [
        "- [ ] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "- [ ] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "- [ ] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "Tabs 归类为 `Streaming Optional` 且当前实现为 `N/A`（snapshot-only，`fallback=snapshot`）。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Tabs checklist should keep streaming definition marker `{required}`."
        );
    }
}

#[test]
fn tabs_snapshot_base_capability_renders_complete_panel_set() {
    let view_source = load_source("src/tabs/view.rs");

    for required in [
        "let panels = children().nodes;",
        "debug_assert_eq!(",
        "labels.len(),",
        "panels.iter().len(),",
        "let item_count = labels.len().min(panels.iter().len());",
        "let tabs_view = labels",
        ".take(item_count)",
        "let panels_view = panels",
        "render_tab_panel(",
    ] {
        assert!(
            view_source.contains(required),
            "Tabs snapshot base capability should keep full children->panel render path marker `{required}`."
        );
    }
}

#[test]
fn tabs_stays_snapshot_only_and_does_not_mount_stream_contract_fields() {
    let view_source = load_source("src/tabs/view.rs");
    let logic_source = load_source("src/tabs/logic.rs");
    let mod_source = load_source("src/tabs/mod.rs");
    let motion_source = load_source("src/tabs/motion.rs");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "streaming",
        "data-stream",
        "data-output-status",
        "data-draft",
        "data-verified",
        "data-commit-ready",
        "retry",
        "fallback=snapshot",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Tabs is snapshot-only; forbidden streaming marker `{forbidden}` should not appear."
        );
    }
}

#[test]
fn tabs_streaming_optional_scope_declares_snapshot_fallback() {
    let checklist_source = load_source("src/tabs/check2.md");
    assert!(
        checklist_source.contains("fallback=snapshot"),
        "Tabs streaming optional scope should explicitly declare `fallback=snapshot`."
    );
}

#[test]
fn tabs_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("tests/tabs_semantics.rs");

    for required in [
        "tabs_semantics_cover_roles_aria_and_state_markers",
        "tabs_semantics_cover_keyboard_and_pointer_interaction_paths",
        "data-disabled-source=disabled_source",
        "data-control-mode=control_mode",
        "role=ROLE_TABPANEL",
        "aria-label=list_a11y.aria_label",
    ] {
        assert!(
            semantics_source.contains(required),
            "Tabs semantic test suite should assert contract marker `{required}`."
        );
    }

    let forbidden_terms = [
        ["assert", "_snapshot"].concat(),
        ["insta", "::"].concat(),
        ["toMatch", "Snapshot"].concat(),
        ["image", "_snapshot"].concat(),
    ];

    for forbidden in forbidden_terms {
        assert!(
            !semantics_source.contains(forbidden.as_str()),
            "Tabs semantic test suite should not rely on visual snapshot assertion `{forbidden}` as primary signal."
        );
    }
}

#[test]
fn tabs_semantic_markers_changed_in_view_must_be_covered_by_semantics_tests() {
    let view_source = load_source("src/tabs/view.rs");
    let semantics_source = load_source("tests/tabs_semantics.rs");

    for marker in [
        "data-disabled-source=disabled_source",
        "data-control-mode=control_mode",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-keyboard-activation=match keyboard_activation",
        "role=list_a11y.role",
        "role=tab_role",
        "role=ROLE_TABPANEL",
        "aria-label=list_a11y.aria_label",
        "on:pointerdown=move |_| press.handlers.on_pointer_down.run(())",
        "on:keydown=on_key_down",
    ] {
        assert!(
            view_source.contains(marker),
            "Tabs view should expose semantic marker `{marker}`."
        );
        assert!(
            semantics_source.contains(marker),
            "Tabs semantic marker `{marker}` changed without matching semantics assertion update."
        );
    }
}

#[test]
fn tabs_streaming_check_script_covers_snapshot_only_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test tabs_semantics tabs_stays_snapshot_only_and_does_not_mount_stream_contract_fields",
        "cargo test -p ui-components --test tabs_semantics tabs_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}
