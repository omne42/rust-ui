use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(component_path) = rel_path.strip_prefix("src/") {
        let mut parts = component_path.splitn(2, '/');
        let component = parts.next().unwrap_or_default();
        let Some(suffix) = parts.next() else {
            return fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        };

        let component_dir = component.replace('_', "-");
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/{component_dir}/src/{suffix}"));

        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn color_handle_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/color-handle/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ColorHandle internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn color_handle_uses_logic_state_model() {
    let logic_source = load_source("../../components/color-handle/src/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/color_handle.rs");
    let primitive_lib_source = load_source("../ui-state-primitives/src/lib.rs");
    let view_source = load_source("../../components/color-handle/src/view.rs");

    for needle in [
        "pub use ui_state_primitives::color_handle::{",
        "ColorHandleState",
        "ColorHandleStateInput",
        "pub const DEFAULT_ARIA_LABEL",
        "pub fn sanitize_color(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ColorHandle logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "pub struct ColorHandleStateInput",
        "pub struct ColorHandleState",
        "pub fn sanitize_color(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ColorHandle state primitive should define `{needle}` in ui-state-primitives."
        );
    }

    assert!(
        primitive_lib_source.contains("pub mod color_handle;"),
        "ui-state-primitives should export `color_handle` module."
    );

    for needle in [
        "logic::resolve_state(ColorHandleStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "<ColorThumb",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorHandle view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn color_handle_mounts_headless_a11y_contract_with_lang_dir() {
    let source = load_source("../../components/color-handle/src/view.rs");

    for needle in [
        "use ui_headless::{A11yDirection, labeled_group_attrs};",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "StoredValue::new(labeled_group_attrs(",
        "role=move || a11y.get_value().role",
        "aria-label=move || a11y.get_value().aria_label",
        "lang=move || a11y.get_value().lang",
        "dir=move || a11y.get_value().dir",
    ] {
        assert!(
            source.contains(needle),
            "ColorHandle should mount headless a11y contract marker `{needle}`."
        );
    }
}

#[test]
fn color_handle_exposes_baseline_style_data_markers() {
    let source = load_source("../../components/color-handle/src/view.rs");

    for attr in [
        "data-slot=\"color-handle\"",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-focused=move || state.get().is_focused.then_some(\"true\")",
        "data-dragging=move || state.get().is_dragging.then_some(\"true\")",
        "data-slot=\"color-handle-surface\"",
    ] {
        assert!(
            source.contains(attr),
            "ColorHandle should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn color_handle_styles_include_focus_drag_disabled_and_custom_contracts() {
    let source = load_source("../../components/color-handle/src/styles.rs");

    for selector in [
        ".ui-color-handle",
        ".ui-color-handle__surface",
        ".ui-color-handle__thumb.ui-color-thumb",
        ".ui-color-handle--focused .ui-color-handle__surface",
        ".ui-color-handle[data-focused=\"true\"] .ui-color-handle__surface",
        ".ui-color-handle--dragging .ui-color-handle__surface",
        ".ui-color-handle[data-dragging=\"true\"] .ui-color-handle__surface",
        ".ui-color-handle--disabled",
        ".ui-color-handle[data-disabled=\"true\"]",
        ".ui-color-handle--custom-class",
        ".ui-color-handle[data-custom-class=\"true\"]",
        "--ui-color-handle-motion-duration",
        "--ui-color-handle-motion-easing",
        "--ui-text-field-motion-duration",
        "--ui-text-field-motion-easing",
        "--ui-color-handle-space: var(--ui-space-2xs, var(--ui-fallback-space-2xs));",
        "--ui-color-handle-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));",
        "--ui-color-handle-border: var(--ui-border, var(--ui-fallback-border));",
        "--ui-color-handle-accent: var(--ui-accent, var(--ui-fallback-accent));",
        "--ui-color-handle-bg: var(--ui-bg, var(--ui-fallback-bg));",
        "--ui-color-handle-fg: var(--ui-fg, var(--ui-fallback-fg));",
    ] {
        assert!(
            source.contains(selector),
            "ColorHandle styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn color_handle_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "pub(super) fn color_handle() -> AnyView",
        "title=\"ColorHandle\"",
        "slug=\"color-handle\"",
        "title=\"Focused + Dragging + Position\"",
        "title=\"Disabled + Custom Class + Loupe Off\"",
    ] {
        assert!(
            source.contains(needle),
            "color-handle docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn color_handle_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "<Playground title=\"Focused + Dragging + Position\" code_signal=basic_code>",
        "id_base=\"docs-color-handle-idle\".to_string()",
        "id_base=\"docs-color-handle-focused\".to_string()",
        "is_focused=true",
        "id_base=\"docs-color-handle-dragging\".to_string()",
        "is_dragging=true",
        "<Playground title=\"Disabled + Custom Class + Loupe Off\" code_signal=states_code>",
        "id_base=\"docs-color-handle-disabled\".to_string()",
        "is_disabled=true",
        "id_base=\"docs-color-handle-custom\".to_string()",
        "is_loupe_visible=false",
        "class_name=\"docs-color-handle-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "color-handle docs playground should contain `{needle}`.",
        );
    }
}

#[test]
fn color_handle_docs_workbench_exposes_display_config_code_and_css_test_contract() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_color.rs");

    for needle in [
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "code_signal=workbench_code",
        "test_css_source=workbench_test_css",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/color-handle/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"color-handle-workbench-controls\"",
        "display: baseline vs configured",
        "docs-color-handle-workbench",
    ] {
        assert!(
            source.contains(needle),
            "color-handle workbench should contain `{needle}`.",
        );
    }
}

#[test]
fn color_handle_exposes_motion_contract_and_view_mount() {
    let mod_source = load_source("../../components/color-handle/src/mod.rs");
    let motion_source = load_source("../../components/color-handle/src/motion.rs");
    let view_source = load_source("../../components/color-handle/src/view.rs");

    for needle in ["pub mod motion;", "pub use motion::ColorHandleMotion;"] {
        assert!(
            mod_source.contains(needle),
            "ColorHandle module should export motion contract marker `{needle}`."
        );
    }

    for needle in [
        "pub struct ColorHandleMotion",
        "pub fn sanitize_motion(",
        "pub fn source_attr(",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "ColorHandle motion contract should define `{needle}`."
        );
    }

    for needle in [
        "motion::source_attr(motion)",
        "motion::attach_motion(None, motion)",
        "data-motion-source=motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "ColorHandle view should mount motion contract marker `{needle}`."
        );
    }

    for needle in [
        "use ui_theme::default_text_field_motion_tokens;",
        "let tokens = default_text_field_motion_tokens();",
    ] {
        assert!(
            motion_source.contains(needle),
            "ColorHandle motion contract should source defaults from theme tokens `{needle}`."
        );
    }
}

#[test]
fn color_handle_readme_covers_workbench_display_config_code_css_test_sections() {
    let source = load_source("../../components/color-handle/src/README.md");

    for needle in [
        "# ColorHandle",
        "Docs Playground（展示 / Config / Code / CSS Test）",
        "展示",
        "Config",
        "Code",
        "CSS Test",
        "对比场景",
        "Workbench (Display + Config + Code + CSS Test)",
    ] {
        assert!(
            source.contains(needle),
            "color-handle README should contain `{needle}`.",
        );
    }
}

#[test]
fn color_handle_check2_has_no_remaining_unchecked_items() {
    let check2_source = load_source("../../components/color-handle/src/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "ColorHandle check2.md should not keep unchecked checklist items after completion."
    );
}
