use std::fs;
use std::path::Path;

fn resolve_path(rel_path: &str) -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    if let Some(suffix) = rel_path.strip_prefix("src/drawer/") {
        workspace_dir.join("components/drawer/src").join(suffix)
    } else if rel_path == "src/lib.rs" {
        workspace_dir.join("crates/ui/src/lib.rs")
    } else if rel_path == "src/css.rs" {
        workspace_dir.join("crates/ui/src/css.rs")
    } else if rel_path == "Cargo.toml" {
        workspace_dir.join("crates/ui/Cargo.toml")
    } else if let Some(suffix) = rel_path.strip_prefix("../../") {
        workspace_dir.join(suffix)
    } else {
        manifest_dir.join(rel_path)
    }
}

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
fn load_drawer_test_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let candidates = [
        workspace_dir.join("components/drawer/test").join(rel_path),
        workspace_dir
            .join("components/drawer/src/test")
            .join(rel_path),
        resolve_path("src/drawer/test").join(rel_path),
    ];

    let path = candidates
        .iter()
        .find(|candidate| candidate.exists())
        .cloned()
        .unwrap_or_else(|| {
            panic!(
                "drawer test source `{rel_path}` not found in {:?}",
                candidates
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
            )
        });

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn snapshot_only_forbidden_patterns() -> [String; 4] {
    [
        ["assert", "_snapshot!"].concat(),
        ["assert_debug", "_snapshot!"].concat(),
        ["insta::assert", "_"].concat(),
        ["to_match", "_snapshot"].concat(),
    ]
}

#[test]
fn drawer_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/drawer/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Drawer internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn drawer_semantics_tests_are_migrated_to_component_test_directory() {
    let module_source = load_source("src/drawer/mod.rs");
    let semantics_source = load_source("../../components/drawer/test/semantics.rs");

    assert!(
        module_source.contains("#[path = \"../test/semantics.rs\"]"),
        "drawer module should mount sibling semantics tests from `components/drawer/test/semantics.rs`."
    );

    for needle in [
        "fn module_contract_keeps_component_assembly_boundaries()",
        "fn test_files_are_sibling_to_src_and_named_by_layer()",
        "fn logic_view_styles_motion_keep_single_responsibility()",
    ] {
        assert!(
            semantics_source.contains(needle),
            "drawer component semantics suite should include `{needle}`."
        );
    }
}

#[test]
fn drawer_does_not_introduce_spec_rs_builder_for_simple_scope() {
    let module_source = load_source("src/drawer/mod.rs");
    let readme_source = load_source("../../components/drawer/src/README.md");
    let check2_source = load_source("src/drawer/check2.md");
    let spec_source = resolve_source_path("../../components/drawer/src/spec.rs");

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::", "Spec::new("] {
        assert!(
            !module_source.contains(forbidden),
            "drawer module should not expose spec-builder token `{forbidden}`."
        );
        assert!(
            !readme_source.contains(forbidden),
            "drawer docs should not require spec-builder token `{forbidden}`."
        );
    }

    assert!(
        spec_source.is_none(),
        "drawer is not schema-heavy; `components/drawer/src/spec.rs` should not be present."
    );

    assert!(
        check2_source.contains("`spec.rs` 只用于少数复杂组件（如 button），避免泛滥"),
        "drawer checklist should preserve spec-rs scope discipline."
    );
}

#[test]
fn drawer_is_exported_and_exposes_state_contracts() {
    let module_source = load_source("src/drawer/mod.rs");
    let crate_source = load_source("src/lib.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/drawer.rs");

    for needle in [
        "pub use logic::{DrawerPartState, DrawerPartStateInput, DrawerPlacement, DrawerSlot};",
        "pub use motion::DrawerMotion;",
        "pub use view::Drawer;",
    ] {
        assert!(
            module_source.contains(needle),
            "drawer module should include `{needle}` state contracts."
        );
    }

    for needle in [
        "pub enum DrawerSlot",
        "pub struct DrawerPartStateInput",
        "pub struct DrawerPartState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "drawer state primitive should include `{needle}`."
        );
    }

    assert!(
        crate_source.contains("pub use ui_drawer as drawer;"),
        "crate root should expose `drawer` via `ui_drawer` re-export."
    );
    assert!(
        crate_source.contains("pub use drawer::{Drawer, DrawerMotion, DrawerPlacement};"),
        "crate root should re-export `Drawer`, `DrawerPlacement`, and `DrawerMotion` contracts."
    );
}

#[test]
fn drawer_logic_exposes_state_helpers() {
    let source = load_source("src/drawer/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/drawer.rs");

    for needle in [
        "pub use ui_state_primitives::drawer::{",
        "DEFAULT_ID_BASE",
        "DEFAULT_OPEN",
        "DEFAULT_TITLE",
        "pub enum DrawerOpenMode",
        "pub struct DrawerOpenConfigInput",
        "pub struct DrawerOpenConfig",
        "pub fn resolve_open_config(input: DrawerOpenConfigInput) -> DrawerOpenConfig",
        "pub fn can_request_open_change(mode: DrawerOpenMode, has_open_change_handler: bool)",
        "pub enum DrawerVisibility",
        "pub fn resolve_close_button_visibility(is_close_button_visible: Option<bool>) -> DrawerVisibility",
        "pub fn state_attr(has_description: bool)",
        "pub fn description_attr(has_description: bool)",
        "pub fn footer_attr(has_footer: bool)",
        "pub fn close_button_attr(show_close_button: bool)",
        "pub fn placement_class(placement: DrawerPlacement)",
        "pub fn placement_attr(placement: DrawerPlacement)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_required_text(value: String, fallback: &'static str)",
        "pub fn normalize_id_base(value: String)",
        "pub fn resolve_state(input: DrawerPartStateInput)",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Drawer primitive should include `{needle}` for centralized source/state contracts."
        );
    }

    for needle in [
        "pub fn resolve_part_states(input: DrawerPartStatesInput) -> DrawerPartStates",
        "pub fn resolve_part_classes(",
        "pub fn to_sheet_placement(placement: DrawerPlacement)",
        "pub fn normalize_view_config(input: DrawerViewConfigInput) -> DrawerViewConfig",
        "pub fn compose_class_name(base_class_name: Option<String>, state: DrawerPartState)",
    ] {
        assert!(
            source.contains(needle),
            "Drawer logic should include `{needle}` for assembly-only contracts."
        );
    }
}

#[test]
fn drawer_view_uses_logic_state_contracts() {
    let source = load_source("src/drawer/view.rs");

    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] placement: Option<DrawerPlacement>",
        "#[prop(optional)] is_close_button_visible: Option<bool>",
        "#[prop(optional)] close_label: Option<&'static str>",
        "logic::normalize_open_state(logic::DrawerOpenStateInput {",
        "let view_config = logic::normalize_view_config(logic::DrawerViewConfigInput {",
        "use_controllable_open_state_traced(",
        "let open = open_state_signal.open;",
        "let placement = view_config.placement;",
        "let close_button_visibility = view_config.close_button_visibility;",
        "show_close_button: root_state.show_close_button,",
        "let close_label = view_config.close_label;",
        "let on_exit_complete = view_config.on_exit_complete;",
        "let open_mode_attr = logic::open_mode_attr(open_state.mode);",
        "let open_value_source =",
        "logic::resolve_open_value_source(open_state.mode, open_state.has_default_open);",
        "let (open_action_source, set_open_action_source) =",
        "signal(logic::DrawerOpenActionSource::Programmatic);",
        "set_open_action_source.set(logic::DrawerOpenActionSource::Interaction);",
        "logic::can_request_open_change(open_state.mode, open_state.has_open_change_handler)",
        "logic::normalize_id_base(id_base)",
        "logic::normalize_required_text(title, logic::DEFAULT_TITLE)",
        "logic::normalize_optional_text(description)",
        "logic::normalize_optional_text(class_name)",
        "let sheet_placement = logic::to_sheet_placement(placement);",
        "let part_states = logic::resolve_part_states(logic::DrawerPartStatesInput {",
        "close_button_visibility,",
        "let part_classes = logic::resolve_part_classes(class_name, part_states);",
        "let root_state = part_states.root;",
        "let root_class = StoredValue::new(part_classes.root);",
        "placement=sheet_placement",
        "data-slot=root_state.slot_attr",
        "data-state=root_state.state_attr",
        "data-open-state=move || logic::open_state_attr(open.get())",
        "data-open-mode=open_mode_attr",
        "data-open-source=open_value_source.as_attr()",
        "data-open-action-source=move || open_action_source.get().as_attr()",
        "data-placement=root_state.placement_attr",
        "data-description=root_state.description_attr",
        "data-footer=root_state.footer_attr",
        "data-close-button=root_state.close_button_attr",
        "data-placement-source=root_state.placement_source_attr",
        "data-description-source=root_state.description_source_attr",
        "data-footer-source=root_state.footer_source_attr",
        "data-close-source=root_state.close_source_attr",
        "data-id-source=root_state.id_source_attr",
        "data-title-source=root_state.title_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-exit-source=root_state.exit_source_attr",
        "data-custom-id=root_state.has_custom_id_base.then_some(\"true\")",
        "data-custom-title=root_state.has_custom_title.then_some(\"true\")",
        "data-custom-description=root_state.has_custom_description.then_some(\"true\")",
        "data-custom-footer=(root_state.footer_source_attr == \"custom\").then_some(\"true\")",
        "data-custom-close=(root_state.close_source_attr == \"custom\").then_some(\"true\")",
        "data-custom-exit=root_state.has_on_exit_complete.then_some(\"true\")",
        "data-slot=header_state.slot_attr",
        "data-slot=title_state.slot_attr",
        "data-slot=body_state.slot_attr",
        "data-slot=footer_state.slot_attr",
        "data-slot=close_state.slot_attr",
        "motion=motion.sheet",
        "on_exit_complete=on_exit_complete",
    ] {
        assert!(
            source.contains(needle),
            "Drawer view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn drawer_delegates_interaction_and_a11y_to_sheet_headless_contract() {
    let drawer_view_source = load_source("src/drawer/view.rs");
    let sheet_view_source = load_source("../../components/sheet/src/view.rs");

    for needle in [
        "use ui_sheet::Sheet;",
        "<Sheet",
        "open=open",
        "on_close=close_action.get_value()",
        "placement=sheet_placement",
        "aria_labelledby=title_id.clone()",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let lang = logic::normalize_optional_text(lang);",
        "lang=lang.clone()",
        "dir=dir",
    ] {
        assert!(
            drawer_view_source.contains(needle),
            "Drawer should mount Sheet and delegate overlay interaction/a11y wiring (`{needle}`).",
        );
    }

    for forbidden in [
        "use_focus_trap(",
        "use_modal(",
        "overlay_dialog_attrs(",
        "use_overlay_stack_registration(",
        "on:keydown=",
    ] {
        assert!(
            !drawer_view_source.contains(forbidden),
            "Drawer should not re-implement headless interaction contracts in view layer (`{forbidden}`).",
        );
    }

    for needle in [
        "use_focus_trap",
        "use_modal",
        "overlay_dialog_attrs",
        "use_overlay_stack_registration",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
    ] {
        assert!(
            sheet_view_source.contains(needle),
            "Sheet should host reusable headless interaction/a11y contracts (`{needle}`).",
        );
    }
}

#[test]
fn drawer_only_sets_describedby_when_description_exists() {
    let source = load_source("src/drawer/view.rs");

    assert!(
        source.contains("if root_state.show_description"),
        "Drawer should branch on description presence so `aria-describedby` is only set when needed."
    );

    for needle in [
        "let description_id = format!(\"{id_base}-description\")",
        "aria_describedby=description_id.clone()",
        "data-slot=description_state.slot_attr",
        "data-description-source=description_state.description_source_attr",
    ] {
        assert!(
            source.contains(needle),
            "Drawer should wire description ids only on described path (`{needle}`)."
        );
    }
}

#[test]
fn drawer_styles_include_state_and_source_markers() {
    let source = load_source("src/drawer/styles.rs");
    let view_source = load_source("src/drawer/view.rs");

    for selector in [
        ".ui-drawer[data-motion-source=\"custom\"]",
        ".ui-drawer[data-custom-motion=\"true\"]",
        ".ui-drawer[data-placement-source=\"custom\"]",
        ".ui-drawer--custom-description",
        ".ui-drawer[data-custom-description=\"true\"]",
        ".ui-drawer[data-description-source=\"custom\"]",
        ".ui-drawer--custom-footer",
        ".ui-drawer[data-custom-footer=\"true\"]",
        ".ui-drawer[data-footer-source=\"custom\"]",
        ".ui-drawer--custom-close",
        ".ui-drawer[data-custom-close=\"true\"]",
        ".ui-drawer[data-close-source=\"custom\"]",
        ".ui-drawer--custom-id",
        ".ui-drawer[data-custom-id=\"true\"]",
        ".ui-drawer[data-id-source=\"custom\"]",
        ".ui-drawer--custom-title",
        ".ui-drawer[data-custom-title=\"true\"]",
        ".ui-drawer[data-title-source=\"custom\"]",
        ".ui-drawer[data-class-source=\"custom\"]",
        ".ui-drawer[data-exit-source=\"custom\"]",
        ".ui-drawer[data-custom-exit=\"true\"]",
        ".ui-drawer--placement-left",
        ".ui-drawer[data-placement=\"right\"]",
        ".ui-drawer--with-description",
        ".ui-drawer[data-state=\"title-only\"]",
        ".ui-drawer--close-hidden",
        ".ui-drawer[data-close-button=\"shown\"]",
        ".ui-drawer[data-footer=\"present\"]",
        ".ui-drawer__header[data-slot=\"drawer-header\"]",
        ".ui-drawer__title[data-slot=\"drawer-title\"]",
        ".ui-drawer__body[data-slot=\"drawer-body\"]",
        ".ui-drawer__footer[data-slot=\"drawer-footer\"]",
    ] {
        assert!(
            source.contains(selector),
            "Drawer styles should include `{selector}` as stable state/source contracts."
        );
    }

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        ":first-child",
        ":last-child",
        ".ui-drawer div div",
        ".ui-drawer > div > div >",
    ] {
        assert!(
            !source.contains(forbidden),
            "Drawer styles should avoid structural-guess selector `{forbidden}`."
        );
    }

    for forbidden in [" style=", "style=\"", "style:", "style:top", "style:left"] {
        assert!(
            !view_source.contains(forbidden),
            "Drawer view should not embed runtime inline business style `{forbidden}`."
        );
    }
}

#[test]
fn drawer_theme_layer_uses_ui_theme_tokens_and_component_styles_only_consume_vars() {
    let drawer_styles = load_source("src/drawer/styles.rs");
    let drawer_view = load_source("src/drawer/view.rs");
    let sheet_styles = load_source("../../components/sheet/src/styles.rs");
    let ui_root_source = load_source("src/root.rs");
    let theme_tokens = load_source("../../crates/ui-theme/src/tokens.rs");
    let theme_mapping = load_source("../../crates/ui-theme/src/theme.rs");
    let theme_css = load_source("../../crates/ui-theme/src/css.rs");
    let styling_spec = load_source("../../docs/spec/styling.md");

    for needle in [
        "pub struct OverlayLayoutTokens",
        "pub struct TypographyTokens",
        "pub struct ThemeTokens",
    ] {
        assert!(
            theme_tokens.contains(needle),
            "ui-theme tokens should define `{needle}` as the token baseline source."
        );
    }

    for needle in [
        "pub enum ThemeSystem",
        "pub enum ThemeColor",
        "pub enum ThemeScale",
        "pub struct ThemeContext",
        "pub overlay_layout: OverlayLayoutTokens",
    ] {
        assert!(
            theme_mapping.contains(needle),
            "ui-theme theme mapping should include `{needle}` for system/color/scale axes."
        );
    }

    for needle in [
        "--ui-space-md",
        "--ui-font-size-150",
        "--ui-heading-h5-font-size",
        "--ui-fg-muted",
        "--ui-border",
        "--ui-overlay-panel-min-width",
        "--ui-overlay-viewport-inset",
    ] {
        assert!(
            theme_css.contains(needle),
            "ui-theme css emitter should output `{needle}` for component consumption."
        );
    }

    for needle in [
        "var(--ui-space-md",
        "var(--ui-heading-h5-font-size",
        "var(--ui-font-size-150",
        "var(--ui-line-height-150",
        "var(--ui-fg-muted",
        "var(--ui-border",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "--ui-button-size-m-height",
        "var(--ui-fallback-button-size-m-height)",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-border-width, var(--ui-fallback-border-width)) solid",
        "var(--ui-border, var(--ui-fallback-border))",
    ] {
        assert!(
            drawer_styles.contains(needle),
            "Drawer styles should consume ui-theme variable `{needle}`."
        );
    }

    for needle in [
        "var(--ui-overlay-z-index)",
        "var(--ui-overlay-viewport-inset)",
        "var(--ui-overlay-panel-min-width)",
        "var(--ui-border)",
    ] {
        assert!(
            sheet_styles.contains(needle),
            "Sheet styles should consume ui-theme overlay token `{needle}`."
        );
    }

    for forbidden in [".ui-drawer", ".ui-sheet"] {
        assert!(
            !theme_css.contains(forbidden),
            "ui-theme css output should not encode component structure (`{forbidden}`).",
        );
    }

    for needle in [
        "Overlay 基线",
        "tokens.rs -> theme.rs -> css.rs",
        "组件侧仅消费变量",
    ] {
        assert!(
            styling_spec.contains(needle),
            "styling spec should keep token traceability guidance `{needle}`."
        );
    }

    for forbidden in ["padding-right: 44px;", "top: 2px;", "right: 2px;"] {
        assert!(
            !drawer_styles.contains(forbidden),
            "Drawer styles should avoid raw spacing literal `{forbidden}`."
        );
    }

    for forbidden in [
        "var(--ui-button-size-m-height, 44px)",
        "var(--ui-space-2xs, 2px)",
        "var(--ui-border-width, 1px) solid var(--ui-border)",
        "var(--ui-heading-h5-font-size, 16px)",
        "var(--ui-heading-h5-line-height, 24px)",
        "var(--ui-font-size-150, 14px)",
        "var(--ui-line-height-150, 20px)",
    ] {
        assert!(
            !drawer_styles.contains(forbidden),
            "Drawer styles should avoid direct terminal fallback literal `{forbidden}`."
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"gap-",
        "tailwind",
        "utility-first",
        "css!(",
        "styled!(",
        "Style::new(",
        "stylist::",
        "emotion::",
    ] {
        assert!(
            !drawer_styles.contains(forbidden),
            "Drawer styles should not depend on utility/CSS-in-Rust token `{forbidden}`."
        );
        assert!(
            !drawer_view.contains(forbidden),
            "Drawer view should not depend on utility/CSS-in-Rust token `{forbidden}`."
        );
    }

    assert!(
        ui_root_source.contains("if inject_components_css.get_value() {")
            && ui_root_source.contains("crate::css::push_components_css(&mut out);"),
        "UiRoot should inject component css via centralized css aggregator."
    );
}

#[test]
fn drawer_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals() {
    let styles_source = load_source("src/drawer/styles.rs");
    let theme_css_source = load_source("../../crates/ui-theme/src/css.rs");

    for needle in [
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "--ui-button-size-m-height",
        "var(--ui-fallback-button-size-m-height)",
        "var(--ui-heading-h5-font-size, var(--ui-fallback-heading-h5-font-size))",
        "--ui-heading-h5-line-height",
        "var(--ui-fallback-heading-h5-line-height)",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-border-width, var(--ui-fallback-border-width)) solid",
        "var(--ui-border, var(--ui-fallback-border))",
    ] {
        assert!(
            styles_source.contains(needle),
            "drawer styles should keep defensive fallback chain marker `{needle}`."
        );
    }

    for needle in [
        "--ui-fallback-space-md:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-2xs:",
        "--ui-fallback-button-size-m-height:",
        "--ui-fallback-heading-h5-font-size:",
        "--ui-fallback-heading-h5-line-height:",
        "--ui-fallback-font-size-150:",
        "--ui-fallback-line-height-150:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-border-width:",
        "--ui-fallback-border:",
    ] {
        assert!(
            theme_css_source.contains(needle),
            "ui-theme css should define fallback terminal `{needle}`."
        );
    }

    for forbidden in [
        "var(--ui-button-size-m-height, 44px)",
        "var(--ui-space-2xs, 2px)",
        "var(--ui-border-width, 1px) solid var(--ui-border)",
        "var(--ui-heading-h5-font-size, 16px)",
        "var(--ui-heading-h5-line-height, 24px)",
        "var(--ui-font-size-150, 14px)",
        "var(--ui-line-height-150, 20px)",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "drawer styles should avoid non-SSOT terminal token `{forbidden}`."
        );
    }
}

#[test]
fn drawer_defensive_variables_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    let needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn drawer_check2_marks_defensive_variable_contract_complete() {
    let source = load_source("src/drawer/check2.md");

    assert!(
        source.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "drawer check2 should mark defensive-variables gate complete.",
    );

    for needle in [
        "components/drawer/src/styles.rs",
        "crates/ui-theme/src/css.rs",
        "--ui-fallback-button-size-m-height",
        "scripts/check-ui-contract-hygiene.sh",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
        "components/drawer/test/semantics.rs::drawer_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
        "components/drawer/test/drawer/semantics.rs::drawer_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "drawer check2 defensive-variables section should reference `{needle}`.",
        );
    }
}

#[test]
fn drawer_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_entry_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let view_source = load_source("src/drawer/view.rs");
    let styles_source = load_source("src/drawer/styles.rs");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-drawer\")]",
        "out.push_str(crate::drawer::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(needle),
            "ui css entry should enforce cascade-layer contract `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized css injection contract `{needle}`."
        );
    }

    assert!(
        !view_source.contains(" style="),
        "drawer view should not embed plain inline style assignments."
    );

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "drawer view should not include fragile inline style token `{forbidden}`."
        );
    }

    for (line_index, line) in view_source.lines().enumerate() {
        if let Some(pos) = line.find("style:") {
            let key = line[pos + "style:".len()..]
                .split(|c: char| c == '=' || c.is_whitespace() || c == '>')
                .next()
                .unwrap_or_default()
                .trim();
            assert!(
                key.starts_with("--"),
                "drawer runtime style should only set css custom properties; found `style:{key}` at line {}.",
                line_index + 1
            );
        }
    }

    for needle in ["pub const CSS: &str", ".ui-drawer", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "drawer styles should remain static token css contract `{needle}`."
        );
    }
}

#[test]
fn drawer_cascade_layer_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    let needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn drawer_check2_marks_cascade_layer_contract_complete() {
    let source = load_source("src/drawer/check2.md");

    assert!(
        source.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "drawer check2 should mark cascade-layer gate complete.",
    );

    for needle in [
        "crates/ui/src/css.rs",
        "crates/ui/src/root.rs",
        "components/drawer/src/view.rs",
        "components/drawer/src/styles.rs",
        "scripts/check-ui-contract-hygiene.sh",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_cascade_layer_and_runtime_style_contract_is_enforced",
        "components/drawer/test/semantics.rs::drawer_cascade_layer_and_runtime_style_contract_is_enforced",
        "components/drawer/test/drawer/semantics.rs::drawer_cascade_layer_and_runtime_style_contract_is_enforced",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "drawer check2 cascade-layer section should reference `{needle}`.",
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn drawer_motion_contract_exposes_default_and_custom_sheet_checks() {
    let source = load_source("src/drawer/motion.rs");

    for needle in [
        "pub struct DrawerMotion",
        "pub sheet: ui_sheet::SheetMotion",
        "fn default_motion_uses_default_sheet_motion_contract()",
        "fn supports_custom_sheet_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "Drawer motion module should include `{needle}` for baseline-level contract coverage."
        );
    }
}

#[test]
fn drawer_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::drawer::styles::CSS);"),
        "ui css aggregator should include drawer styles."
    );
}

#[test]
fn drawer_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn drawer() -> AnyView",
        "title=\"Drawer\"",
        "slug=\"drawer\"",
        "State + Source Markers",
        "data-placement-source",
        "<Drawer",
    ] {
        assert!(
            source.contains(needle),
            "drawer docs page should contain `{needle}`."
        );
    }
}

#[test]
fn drawer_docs_page_includes_minimal_hello_world_path() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "let minimal_code = Signal::derive(move || {",
        "<Playground",
        "title=\"Hello World (Minimal API)\"",
        "code_signal=minimal_code",
        "No manual state wiring.",
        "<Drawer",
        "default_open=true",
        "id_base=\"docs-drawer-minimal\".to_string()",
        "title=\"Hello drawer\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "drawer docs should include minimal hello-world API path `{needle}`."
        );
    }

    assert!(
        source.contains("不需要先理解 `ui-state-primitives` / `ui-headless` 内部状态机细节即可上手。"),
        "drawer docs should keep primitive/headless internals optional on minimal path."
    );
}

#[test]
fn drawer_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/drawer/motion.rs");
    let motion_checks_source = load_drawer_test_source("motion.rs");
    let motion_combined = format!("{motion_source}\n{motion_checks_source}");
    let view_source = load_source("src/drawer/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: DrawerMotion) -> DrawerMotion",
        "sheet: ui_sheet::motion::sanitize_motion(motion.sheet)",
        "fn sanitize_motion_delegates_to_sheet_contract()",
    ] {
        assert!(
            motion_combined.contains(needle),
            "Drawer motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = motion::sanitize_motion(motion);"),
        "Drawer view should sanitize motion before forwarding to Sheet.",
    );
}

#[test]
fn drawer_motion_layer_boundaries_delegate_runtime_to_sheet_and_ui_motion() {
    let drawer_motion_source = load_source("src/drawer/motion.rs");
    let sheet_motion_source = load_source("../../components/sheet/src/motion.rs");
    let ui_motion_source = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "pub struct DrawerMotion",
        "pub sheet: ui_sheet::SheetMotion",
        "pub fn sanitize_motion(motion: DrawerMotion) -> DrawerMotion",
        "sheet: ui_sheet::motion::sanitize_motion(motion.sheet)",
    ] {
        assert!(
            drawer_motion_source.contains(needle),
            "Drawer motion should stay contract-only and include `{needle}`."
        );
    }

    for forbidden in [
        "SpringAnimator",
        "Effect::new",
        "NodeRef<",
        "prefers_reduced_motion",
        "attach_motion(",
    ] {
        assert!(
            !drawer_motion_source.contains(forbidden),
            "Drawer motion should not embed runtime driver logic (`{forbidden}`).",
        );
    }

    for needle in [
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new",
        "ui_motion::web::prefers_reduced_motion()",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if !is_open.get() {",
        "finish_exit.run(())",
    ] {
        assert!(
            sheet_motion_source.contains(needle),
            "Sheet motion should host runtime execution/reduced-motion/no-op behavior (`{needle}`).",
        );
    }

    for needle in [
        "pub mod spring;",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion crate should provide shared backend/no-op surface (`{needle}`).",
        );
    }
}

#[test]
fn drawer_semantics_matrix_covers_interactions_and_platform_paths() {
    let drawer_view_source = load_source("src/drawer/view.rs");
    let drawer_logic_source = load_source("src/drawer/logic.rs");
    let drawer_logic_tests = load_source("../../components/drawer/test/logic.rs");
    let drawer_semantics_tests = load_source("../../components/drawer/test/semantics.rs");
    let sheet_view_source = load_source("../../components/sheet/src/view.rs");
    let sheet_motion_source = load_source("../../components/sheet/src/motion.rs");

    for needle in [
        "data-state=root_state.state_attr",
        "data-open-state=move || logic::open_state_attr(open.get())",
        "data-open-source=open_value_source.as_attr()",
        "data-open-action-source=move || open_action_source.get().as_attr()",
        "aria_labelledby=title_id.clone()",
        "aria_describedby=description_id.clone()",
        "role=\"dialog\"",
        "aria-modal=\"true\"",
    ] {
        assert!(
            drawer_view_source.contains(needle) || sheet_view_source.contains(needle),
            "drawer semantic contract matrix should include `{needle}`."
        );
    }

    for needle in [
        "normalize_open_state_supports_controlled_mode",
        "normalize_open_state_supports_uncontrolled_mode_with_default",
        "open_axis_exposes_controlled_uncontrolled_triplet_contract",
    ] {
        assert!(
            drawer_logic_tests.contains(needle) || drawer_semantics_tests.contains(needle),
            "drawer semantics matrix should include controlled/uncontrolled coverage `{needle}`.",
        );
    }

    for needle in [
        "on:click=move |_|",
        "on:pointerdown=move |ev| ev.stop_propagation()",
        "on:keydown=on_key_down",
        "should_close_on_escape(",
    ] {
        assert!(
            sheet_view_source.contains(needle),
            "sheet contract should include keyboard/pointer interaction path `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            sheet_motion_source.contains(needle),
            "drawer semantics matrix should include platform branch `{needle}`."
        );
    }

    for forbidden in ["is_disabled", "aria-disabled", "aria_disabled"] {
        assert!(
            !drawer_view_source.contains(forbidden) && !drawer_logic_source.contains(forbidden),
            "drawer has no disabled axis; this branch should remain N/A (`{forbidden}`)."
        );
    }

    for forbidden in snapshot_only_forbidden_patterns() {
        assert!(
            !drawer_semantics_tests.contains(&forbidden),
            "drawer semantics should not rely on snapshot-only assertion `{forbidden}`."
        );
    }
}

#[test]
fn drawer_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let view_source = load_source("src/drawer/view.rs");
    let sheet_view_source = load_source("../../components/sheet/src/view.rs");
    let local_semantics_source = load_source("../../components/drawer/test/semantics.rs");
    let semantics_source = load_source("tests/drawer/semantics.rs");
    let perf_script_source = load_source("../../scripts/check-ui-performance.sh");

    for marker in [
        "data-state=root_state.state_attr",
        "data-open-state=move || logic::open_state_attr(open.get())",
        "data-open-mode=open_mode_attr",
        "data-open-source=open_value_source.as_attr()",
        "data-open-action-source=move || open_action_source.get().as_attr()",
        "data-placement-source=root_state.placement_source_attr",
        "data-description-source=root_state.description_source_attr",
        "data-footer-source=root_state.footer_source_attr",
        "data-close-source=root_state.close_source_attr",
        "data-id-source=root_state.id_source_attr",
        "data-title-source=root_state.title_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-exit-source=root_state.exit_source_attr",
        "aria_labelledby=title_id.clone()",
        "aria_describedby=description_id.clone()",
    ] {
        assert!(
            view_source.contains(marker),
            "drawer semantic-priority contract should keep marker `{marker}`."
        );
    }

    for marker in [
        "role=\"dialog\"",
        "aria-modal=\"true\"",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
        "on:keydown=on_key_down",
        "on:pointerdown=move |ev| ev.stop_propagation()",
    ] {
        assert!(
            sheet_view_source.contains(marker),
            "sheet semantic-priority contract should keep marker `{marker}`."
        );
    }

    for marker in [
        "fn drawer_semantics_tests_cover_contract_matrix_without_snapshot_dependency()",
        "drawer_state_markers_expose_observable_closed_source_sets",
        "snapshot_only_forbidden_patterns()",
        "drawer semantics suite should not rely on visual snapshot-only assertion",
    ] {
        assert!(
            local_semantics_source.contains(marker),
            "drawer local semantics suite should keep semantic-priority marker `{marker}`."
        );
    }

    for marker in [
        "fn drawer_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()",
        "fn drawer_semantics_matrix_covers_interactions_and_platform_paths()",
    ] {
        assert!(
            semantics_source.contains(marker),
            "workspace drawer semantics suite should keep semantic-priority marker `{marker}`."
        );
    }

    let script_needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script_source.contains(script_needle),
        "performance script should include drawer semantic-priority gate `{script_needle}`."
    );
}

#[test]
fn drawer_performance_script_covers_semantic_test_priority_contract() {
    let script_source = load_source("../../scripts/check-ui-performance.sh");

    for marker in [
        "echo \"[perf] contract: drawer semantic test priority\"",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
    ] {
        assert!(
            script_source.contains(marker),
            "performance script should include drawer semantic-priority marker `{marker}`."
        );
    }
}

#[test]
fn drawer_check2_marks_semantic_test_priority_contract_complete() {
    let source = load_source("src/drawer/check2.md");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "drawer_semantics_tests_cover_contract_matrix_without_snapshot_dependency",
        "drawer_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "drawer_performance_script_covers_semantic_test_priority_contract",
        "scripts/check-ui-performance.sh",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "drawer check2 semantic-priority section should include `{needle}`."
        );
    }
}

#[test]
fn drawer_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("src/drawer/check2.md");

    for marker in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2_source.contains(marker),
            "drawer check2 should keep e2e selector stability rule `{marker}`."
        );
    }
}

#[test]
fn drawer_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_drawer_contract.spec.mjs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for marker in [
        "page.goto(\"/#/components/drawer\")",
        "body:not(:has(#boot))",
        "waitForWasmReady(page)",
        "ensureDrawerBaseline(page)",
        "[data-component=\"drawer\"]",
        "[data-slot=\"drawer-e2e-right-controls\"]",
        "[data-slot=\"drawer-e2e-open-right\"]",
        "[data-slot=\"drawer-e2e-custom-controls\"]",
        "[data-slot=\"drawer-e2e-open-custom\"]",
        "[data-slot=\"overlay-panel\"][role=\"dialog\"]",
        "[data-slot=\"drawer\"]",
        "[data-slot=\"overlay\"]",
        "expectDrawerReady(page, rightPanel, rightDrawer)",
        "expectDrawerSettledClosed(rightPanel, rightDrawer, rightOverlay);",
        "expectDrawerSettledClosed(customPanel, customDrawer, customOverlay);",
        "toHaveAttribute(\"data-open-mode\", \"controlled\")",
        "toHaveAttribute(\"data-open-source\", \"external\")",
    ] {
        assert!(
            e2e_source.contains(marker),
            "drawer e2e selector/stable-wait contract should include `{marker}`."
        );
    }

    for marker in [
        "data-slot=\"drawer-e2e-right-controls\"",
        "data-slot=\"drawer-e2e-open-right\"",
        "data-slot=\"drawer-e2e-custom-controls\"",
        "data-slot=\"drawer-e2e-open-custom\"",
        "data-slot=\"drawer-e2e-dismiss-custom\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "drawer docs source should keep e2e semantic anchor `{marker}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "toHaveScreenshot(",
        "toMatchSnapshot(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "drawer e2e contract should avoid flaky/snapshot selector token `{forbidden}`."
        );
    }
}

#[test]
fn drawer_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths() {
    let e2e_source = load_source("../../e2e/tests/docs_app_drawer_contract.spec.mjs");

    for marker in [
        "async function expectDrawerReady(page, overlayPanel, drawerRoot)",
        "async function expectDrawerSettledClosed(overlayPanel, drawerRoot, overlayRoot)",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"data-open-state\", \"open\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "await rightPanel.press(\"Escape\");",
        "await backdrop.click();",
        "await expectDrawerSettledClosed(rightPanel, rightDrawer, rightOverlay);",
        "await expectDrawerSettledClosed(customPanel, customDrawer, customOverlay);",
    ] {
        assert!(
            e2e_source.contains(marker),
            "drawer e2e ready/settled contract should include `{marker}`."
        );
    }
}

#[test]
fn drawer_e2e_check_script_covers_selector_and_settled_wait_contract() {
    let script_source = load_source("../../components/drawer/scripts/check-ui-e2e-drawer.sh");

    for marker in [
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(marker),
            "drawer e2e check script should include `{marker}`."
        );
    }
}

#[test]
fn drawer_check2_marks_e2e_selector_stability_item_complete() {
    let check2_source = load_source("src/drawer/check2.md");

    assert!(
        check2_source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "drawer check2 should mark e2e selector stability item complete."
    );

    for marker in [
        "components/drawer/test/semantics.rs::drawer_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/drawer/test/semantics.rs::drawer_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "components/drawer/test/semantics.rs::drawer_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths",
        "components/drawer/test/semantics.rs::drawer_e2e_check_script_covers_selector_and_settled_wait_contract",
        "components/drawer/test/drawer/semantics.rs::drawer_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "components/drawer/scripts/check-ui-e2e-drawer.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "drawer check2 e2e selector stability section should include `{marker}`."
        );
    }
}

#[test]
fn drawer_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = load_source("src/drawer/check2.md");

    for marker in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(marker),
            "drawer check2 should keep replayable e2e critical-flow rule `{marker}`."
        );
    }
}

#[test]
fn drawer_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_drawer_contract.spec.mjs");

    for marker in [
        "docs-app drawer key flow is repeatable with semantic breakpoints",
        "for (const cycle of [1, 2]) {",
        "drawer key flow cycle ${cycle}",
        "await openRight.focus();",
        "await expect(openRight).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await expect(rightDrawer).toHaveAttribute(\"data-open-mode\", \"controlled\");",
        "await expect(rightDrawer).toHaveAttribute(\"data-open-source\", \"external\");",
        "await expect(rightDrawer).toHaveAttribute(\"data-open-action-source\", \"programmatic\");",
        "await expectFocusInsidePanel(rightPanel);",
        "await page.keyboard.press(\"Tab\");",
        "await rightPanel.press(\"Escape\");",
        "await expectDrawerSettledClosed(rightPanel, rightDrawer, rightOverlay);",
        "await expect(openRight).toBeFocused();",
    ] {
        assert!(
            e2e_source.contains(marker),
            "drawer replayable key-flow e2e should keep semantic breakpoint marker `{marker}`."
        );
    }
}

#[test]
fn drawer_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_drawer_contract.spec.mjs");

    for marker in [
        "docs-app drawer high-risk paths keep overlay focus keyboard and settled semantic breakpoints",
        "await openCustom.focus();",
        "await expect(openCustom).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await expect(customDrawer).toHaveAttribute(\"data-motion-source\", \"custom\");",
        "await expect(customDrawer).toHaveAttribute(\"data-placement\", \"left\");",
        "await expectFocusInsidePanel(customPanel);",
        "await page.keyboard.press(\"Tab\");",
        "await page.keyboard.press(\"Shift+Tab\");",
        "await backdrop.click();",
        "await expectDrawerSettledClosed(customPanel, customDrawer, customOverlay);",
    ] {
        assert!(
            e2e_source.contains(marker),
            "drawer high-risk e2e path should keep semantic breakpoint marker `{marker}`."
        );
    }
}

#[test]
fn drawer_e2e_check_script_covers_repeatable_key_flow_contracts() {
    let script_source = load_source("../../components/drawer/scripts/check-ui-e2e-drawer.sh");

    for marker in [
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(marker),
            "drawer e2e check script should include replayable critical-flow marker `{marker}`."
        );
    }
}

#[test]
fn drawer_check2_marks_replayable_e2e_critical_flow_item_complete() {
    let check2_source = load_source("src/drawer/check2.md");

    assert!(
        check2_source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "drawer check2 should mark replayable e2e critical-flow item complete."
    );

    for marker in [
        "docs-app drawer key flow is repeatable with semantic breakpoints",
        "for (const cycle of [1, 2])",
        "await expect(rightDrawer).toHaveAttribute(\"data-open-mode\", \"controlled\")",
        "await expect(openRight).toBeFocused()",
        "docs-app drawer high-risk paths keep overlay focus keyboard and settled semantic breakpoints",
        "await page.keyboard.press(\"Shift+Tab\")",
        "drawer_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "drawer_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "components/drawer/scripts/check-ui-e2e-drawer.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "drawer check2 replayable e2e critical-flow section should include `{marker}`."
        );
    }
}

#[test]
fn drawer_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "let custom_motion = DrawerMotion {",
        "sheet: SheetMotion {",
        "initial_offset_px: 52.0",
        "title=\"State + Source Markers\"",
        "motion=custom_motion",
        "placement=DrawerPlacement::Left",
        "is_close_button_visible=false",
        "class_name=\"docs-drawer-custom\".to_string()",
        "on_exit_complete=finish_exit",
        "on_exit_complete=on_custom_exit_complete",
        "Inspect data-placement-source / data-title-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "drawer docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn drawer_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn drawer() -> AnyView",
        "title=\"Drawer\"",
        "slug=\"drawer\"",
        "description=\"Sheet composition with centralized placement/description/footer/close state attrs and stable drawer slots.\"",
        "title=\"Hello World (Minimal API)\"",
        "<Playground title=\"Right Drawer + Slots\" code_signal=semantic_code>",
        "title=\"State + Source Markers\"",
        "<Drawer",
        "placement=DrawerPlacement::Right",
        "placement=DrawerPlacement::Left",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs page should include `{needle}` for drawer primary playground coverage.",
        );
    }
}

#[test]
fn drawer_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "id_base=\"docs-drawer-right\".to_string()",
        "title=\"Drawer title\".to_string()",
        "description=\"Drawer composes Sheet and keeps labeled/description semantics aligned.\".to_string()",
        "placement=DrawerPlacement::Right",
        "on_exit_complete=on_semantic_exit_complete",
        "id_base=\"docs-drawer-left\".to_string()",
        "title=\"Left drawer\".to_string()",
        "placement=DrawerPlacement::Left",
        "is_close_button_visible=false",
        "class_name=\"docs-drawer-custom\".to_string()",
        "motion=custom_motion",
        "let custom_motion = DrawerMotion {",
        "initial_offset_px: 52.0",
        "on_exit_complete=on_custom_exit_complete",
    ] {
        assert!(
            source.contains(needle),
            "drawer docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn drawer_visual_desire_reuses_theme_visual_baseline_and_screenshot_contracts() {
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let baseline_e2e_source =
        load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for needle in [
        "pub(super) fn theme_visual_baseline() -> AnyView",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            baseline_page_source.contains(needle),
            "theme visual baseline docs page should include `{needle}`."
        );
    }

    for needle in [
        "page.goto(\"/#/components/theme-visual-baseline\")",
        "theme visual baseline renders button/input/overlay",
        "theme visual baseline screenshots",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            baseline_e2e_source.contains(needle),
            "theme visual baseline e2e regression should include `{needle}`."
        );
    }
}

#[test]
fn drawer_tree_shaking_contract_is_feature_gated_and_budget_guarded() {
    let ui_components_cargo = load_source("Cargo.toml");
    let ui_components_lib = load_source("src/lib.rs");
    let ui_components_css = load_source("src/css.rs");
    let drawer_cargo = load_source("../../components/drawer/Cargo.toml");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let tree_shaking_script = load_source("../../scripts/check-ui-tree-shaking.sh");
    let tree_shaking_budget = load_source("../../scripts/tree_shaking_budget.env");
    let ci_source = load_source("../../.github/workflows/ci.yml");
    let check2_source = load_source("src/drawer/check2.md");

    for needle in [
        "component-drawer = [\"dep:ui-drawer\"]",
        "ui-drawer = { path = \"../../components/drawer\", optional = true }",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui feature table should keep drawer feature gate `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-drawer\")]",
        "pub use ui_drawer as drawer;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui lib should gate drawer export via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-drawer\")]",
        "out.push_str(crate::drawer::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "ui css aggregation should stay feature-gated via `{needle}`."
        );
    }

    assert!(
        drawer_cargo.contains("[features]\ndefault = []"),
        "drawer source-mode crate should keep default empty feature set for source slicing."
    );
    assert!(
        !drawer_cargo.contains("name = \"ui\"")
            && !drawer_cargo.contains("path = \"../../crates/ui\""),
        "drawer source-mode crate should not depend on ui central registry crate."
    );

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("features = [\"inject-css\", \"web-demo-components\"]")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui via web-demo-components without all-components."
    );

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"",
        "cargo tree -e features -i ui -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_shaking_script.contains(needle) || tree_shaking_budget.contains(needle),
            "tree-shaking script/budget should include `{needle}`."
        );
    }

    assert!(
        ci_source.contains("Tree Shaking Budget")
            && ci_source.contains("./scripts/check-ui-tree-shaking.sh"),
        "CI should execute tree-shaking budget gate."
    );

    for needle in [
        "- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。",
        "component-drawer = [\"dep:ui-drawer\"]",
        "`cargo tree -e features -p ui --no-default-features --features component-drawer,inject-css`",
        "`cargo tree -e features -i ui -p web-demo`",
        "`scripts/check-ui-tree-shaking.sh`",
        "回归：`components/drawer/test/drawer/semantics.rs::drawer_tree_shaking_contract_is_feature_gated_and_budget_guarded`",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 tree-shaking evidence should include `{needle}`."
        );
    }
}

#[test]
fn drawer_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let tree_shaking_script = load_source("../../scripts/check-ui-tree-shaking.sh");

    for needle in [
        "DRAWER_MIN_FEATURES=\"component-drawer,inject-css\"",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_tree_shaking_contract_is_feature_gated_and_budget_guarded",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "DRAWER_TREE_OUTPUT=\"$(cargo tree -e features -i ui -p ui --no-default-features --features \"$DRAWER_MIN_FEATURES\")\"",
        "if ! grep -q 'feature \"component-drawer\" (command-line)' <<<\"$DRAWER_TREE_OUTPUT\"",
        "if ! grep -q 'feature \"inject-css\" (command-line)' <<<\"$DRAWER_TREE_OUTPUT\"",
        "if grep -q 'all-components' <<<\"$DRAWER_TREE_OUTPUT\"",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$DRAWER_MIN_FEATURES\"",
    ] {
        assert!(
            tree_shaking_script.contains(needle),
            "tree-shaking script should enforce drawer minimal-feature gate `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_source("src/drawer/check2.md");

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "drawer_tree_shaking_contract_is_feature_gated_and_budget_guarded",
        "drawer_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "drawer_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "`component-drawer = [\"dep:ui-drawer\"]`",
        "`#[cfg(feature = \"component-drawer\")]`",
        "`out.push_str(crate::drawer::styles::CSS);`",
        "`scripts/check-ui-tree-shaking.sh`",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 tree-shaking section should reference `{needle}`.",
        );
    }
}

#[test]
fn drawer_type_system_and_semantic_markers_form_machine_readable_contract() {
    let drawer_logic_source = load_source("src/drawer/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/drawer.rs");
    let drawer_view_source = load_source("src/drawer/view.rs");
    let drawer_logic_tests = load_source("../../components/drawer/test/logic.rs");
    let drawer_semantics_tests = load_source("../../components/drawer/test/semantics.rs");
    let check2_source = load_source("src/drawer/check2.md");

    for needle in [
        "pub enum DrawerPlacement",
        "pub enum DrawerOpenMode",
        "pub enum DrawerVisibility",
        "pub enum DrawerOpenValueSource",
        "pub enum DrawerOpenActionSource",
    ] {
        assert!(
            drawer_logic_source.contains(needle) || primitive_source.contains(needle),
            "drawer should model discrete state/input space with typed enum `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_open_state(input: DrawerOpenStateInput) -> DrawerOpenState",
        "pub fn normalize_view_config(input: DrawerViewConfigInput) -> DrawerViewConfig",
        "pub fn resolve_part_states(input: DrawerPartStatesInput) -> DrawerPartStates",
        "pub fn open_state_attr(is_open: bool) -> &'static str",
        "pub fn open_mode_attr(mode: DrawerOpenMode) -> &'static str",
        "pub fn resolve_open_value_source(",
    ] {
        assert!(
            drawer_logic_source.contains(needle),
            "drawer logic should centralize invalid-state normalization via `{needle}`."
        );
    }

    for needle in [
        "data-state=root_state.state_attr",
        "data-open-state=move || logic::open_state_attr(open.get())",
        "data-open-mode=open_mode_attr",
        "data-open-source=open_value_source.as_attr()",
        "data-open-action-source=move || open_action_source.get().as_attr()",
        "data-placement-source=root_state.placement_source_attr",
        "data-description-source=root_state.description_source_attr",
        "data-footer-source=root_state.footer_source_attr",
        "data-close-source=root_state.close_source_attr",
    ] {
        assert!(
            drawer_view_source.contains(needle),
            "drawer view should expose machine-readable semantic marker `{needle}`."
        );
    }

    for needle in [
        "normalize_open_state_supports_controlled_mode",
        "normalize_open_state_supports_uncontrolled_mode_with_default",
        "open_state_source_markers_are_closed_sets",
        "resolve_part_states_centralizes_slot_state_derivation",
        "resolve_state_tracks_source_markers",
    ] {
        assert!(
            drawer_logic_tests.contains(needle) || drawer_semantics_tests.contains(needle),
            "drawer tests should let compiler/test feedback pinpoint contract break at `{needle}`."
        );
    }

    for needle in [
        "- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。",
        "components/drawer/test/semantics.rs::drawer_type_system_and_semantic_markers_form_machine_readable_contract",
        "components/drawer/test/drawer/semantics.rs::drawer_type_system_and_semantic_markers_form_machine_readable_contract",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 should keep machine-readable state evidence token `{needle}`."
        );
    }
}

#[test]
fn drawer_focus_stack_gc_contract_uses_global_focus_manager_and_policy_restore() {
    let drawer_view_source = load_source("src/drawer/view.rs");
    let sheet_view_source = load_source("../../components/sheet/src/view.rs");
    let headless_focus_trap_source = load_source("../../crates/ui-headless/src/focus_trap.rs");
    let check2_source = load_source("src/drawer/check2.md");

    for forbidden in [
        "NodeRef<",
        "restore_focus",
        "focus_manager_push_trap(",
        "focus_manager_pop_trap(",
    ] {
        assert!(
            !drawer_view_source.contains(forbidden),
            "drawer view should not own local focus-stack/restore internals `{forbidden}`."
        );
    }

    for needle in [
        "use_overlay_stack_registration();",
        "let focus_trap = use_focus_trap(",
        "FocusTrapOptions::enabled(panel_ref)",
        "on:keydown=on_key_down",
    ] {
        assert!(
            sheet_view_source.contains(needle),
            "sheet should mount focus-stack/focus-trap entry contract `{needle}`."
        );
    }

    for needle in [
        "pub enum RestorePolicy",
        "Selector(String)",
        "FallbackTo(String)",
        "pub struct FocusTrapFrame",
        "FOCUS_MANAGER_STACK",
        "focus_manager_push_trap(",
        "focus_manager_pop_trap(",
        "focus_manager_peek_trap()",
        "derive_restore_policy(",
        "restore_focus_chain(",
        "restore_focus_by_policy(",
        "if let Some(body) = document.body()",
    ] {
        assert!(
            headless_focus_trap_source.contains(needle),
            "ui-headless global focus manager should include `{needle}`."
        );
    }

    assert!(
        !headless_focus_trap_source.contains("Option<NodeRef"),
        "focus restore policy should be selector/policy based, not NodeRef-based."
    );

    for needle in [
        "- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。",
        "components/sheet/src/view.rs",
        "crates/ui-headless/src/focus_trap.rs",
        "RestorePolicy::{Selector,FallbackTo}",
        "components/drawer/test/drawer/semantics.rs::drawer_focus_stack_gc_contract_uses_global_focus_manager_and_policy_restore",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 focus-stack evidence should include `{needle}`."
        );
    }
}

#[test]
fn drawer_escape_hatches_foreign_zone_contract_is_not_applicable() {
    let drawer_mod_source = load_source("src/drawer/mod.rs");
    let drawer_logic_source = load_source("src/drawer/logic.rs");
    let drawer_view_source = load_source("src/drawer/view.rs");
    let drawer_motion_source = load_source("src/drawer/motion.rs");
    let sheet_view_source = load_source("../../components/sheet/src/view.rs");
    let check2_source = load_source("../../components/drawer/check2.md");

    for source in [
        drawer_mod_source.as_str(),
        drawer_logic_source.as_str(),
        drawer_view_source.as_str(),
        drawer_motion_source.as_str(),
        sheet_view_source.as_str(),
    ] {
        for forbidden in [
            "ECharts",
            "echarts",
            "Mapbox",
            "mapbox",
            "Leaflet",
            "leaflet",
            "google.maps",
            "amap",
            "YieldControl",
            "CleanupForeign",
            "ForeignZone",
            "foreign_zone",
        ] {
            assert!(
                !source.contains(forbidden),
                "drawer/sheet assembly should keep escape-hatch foreign zone out when N/A; found `{forbidden}`."
            );
        }
    }

    for forbidden in [
        "JsValue",
        "web_sys::Html",
        "wasm_bindgen::JsValue",
        "on_foreign",
        "foreign_instance",
        "map_instance",
        "chart_instance",
    ] {
        assert!(
            !drawer_view_source.contains(forbidden),
            "drawer public API should not expose imperative third-party instance token `{forbidden}`."
        );
    }

    for needle in [
        "- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。",
        "N/A：`Drawer` 当前未集成 ECharts/Map 等命令式第三方实例",
        "components/drawer/test/drawer/semantics.rs::drawer_escape_hatches_foreign_zone_contract_is_not_applicable",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 escape-hatch evidence should include `{needle}`."
        );
    }
}

#[test]
fn drawer_hydration_discontinuity_contract_is_n_a_without_time_or_random_id_init() {
    let drawer_logic_source = load_source("src/drawer/logic.rs");
    let drawer_view_source = load_source("src/drawer/view.rs");
    let drawer_motion_source = load_source("src/drawer/motion.rs");
    let sheet_logic_source = load_source("../../components/sheet/src/logic.rs");
    let sheet_view_source = load_source("../../components/sheet/src/view.rs");
    let root_source = load_source("../../crates/ui/src/root.rs");
    let id_provider_source = load_source("../../crates/ui-headless/src/id_provider.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/drawer.rs");
    let check2_source = load_source("../../components/drawer/check2.md");

    for source in [
        drawer_logic_source.as_str(),
        drawer_view_source.as_str(),
        drawer_motion_source.as_str(),
        sheet_logic_source.as_str(),
        sheet_view_source.as_str(),
    ] {
        for forbidden in [
            "now()",
            "now(",
            "SystemTime::now",
            "Instant::now",
            "Uuid::",
            "uuid::",
            "rand::",
            "random::<",
            "random()",
            "Math::random",
            "randomUUID",
            "getrandom",
        ] {
            assert!(
                !source.contains(forbidden),
                "drawer/sheet hydration init should not depend on time/random entropy source `{forbidden}`."
            );
        }
    }

    for needle in [
        "let id_base = logic::normalize_id_base(id_base);",
        "let title_id = format!(\"{id_base}-title\");",
        "let description_id = format!(\"{id_base}-description\");",
    ] {
        assert!(
            drawer_view_source.contains(needle),
            "drawer id derivation should stay deterministic via `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_id_base(value: String) -> String {",
        "DEFAULT_ID_BASE",
    ] {
        assert!(
            primitive_source.contains(needle),
            "drawer primitive id normalization should include deterministic fallback `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, default = 1)] id_seed: u64,",
        "provide_ui_id_provider(id_seed);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep deterministic id-seed injection path `{needle}`."
        );
    }

    for needle in [
        "pub struct UiIdProvider",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider>",
    ] {
        assert!(
            id_provider_source.contains(needle),
            "ui-headless id-provider should expose deterministic contract `{needle}`."
        );
    }

    for needle in [
        "- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。",
        "N/A（组件内随机 ID 生成）：`Drawer` 不在组件内生成随机/时间型 ID",
        "crates/ui/src/root.rs",
        "provide_ui_id_provider(id_seed)",
        "components/drawer/test/drawer/semantics.rs::drawer_hydration_discontinuity_contract_is_n_a_without_time_or_random_id_init",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 hydration evidence should include `{needle}`."
        );
    }
}

#[test]
fn drawer_ssr_and_cross_platform_compile_contract_is_documented_and_non_wasm_safe() {
    let drawer_mod_source = load_source("src/drawer/mod.rs");
    let drawer_logic_source = load_source("src/drawer/logic.rs");
    let drawer_styles_source = load_source("src/drawer/styles.rs");
    let drawer_view_source = load_source("src/drawer/view.rs");
    let drawer_motion_source = load_source("src/drawer/motion.rs");
    let sheet_view_source = load_source("../../components/sheet/src/view.rs");
    let sheet_motion_source = load_source("../../components/sheet/src/motion.rs");
    let platform_script = load_source("../../scripts/check-ui-platforms.sh");
    let check2_source = load_source("../../components/drawer/check2.md");

    for needle in [
        "cargo check -p ui --no-default-features --features component-drawer,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-drawer,inject-css",
    ] {
        assert!(
            platform_script.contains(needle),
            "platform script should keep drawer compile-only evidence path `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "let is_composing = false;",
        "let default_prevented = false;",
    ] {
        assert!(
            sheet_view_source.contains(needle),
            "sheet view should keep explicit platform fallback marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            sheet_motion_source.contains(needle),
            "sheet motion should keep explicit wasm/non-wasm cfg split marker `{needle}`."
        );
    }

    for source in [
        drawer_mod_source.as_str(),
        drawer_logic_source.as_str(),
        drawer_styles_source.as_str(),
        drawer_view_source.as_str(),
        drawer_motion_source.as_str(),
    ] {
        for forbidden in ["web_sys", "wasm_bindgen", "js_sys", "window(", "document("] {
            assert!(
                !source.contains(forbidden),
                "drawer non-wasm path should stay browser-api free (`{forbidden}`)."
            );
        }
    }

    let non_wasm_sheet_motion = sheet_motion_source
        .split("#[cfg(not(target_arch = \"wasm32\"))]")
        .nth(1)
        .expect("sheet motion should contain explicit non-wasm branch");
    for forbidden in ["web_sys", "wasm_bindgen", "js_sys", "window(", "document("] {
        assert!(
            !non_wasm_sheet_motion.contains(forbidden),
            "sheet non-wasm motion branch should avoid browser-only token `{forbidden}`."
        );
    }

    for needle in [
        "- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。",
        "component-drawer,inject-css",
        "components/sheet/src/motion.rs",
        "components/sheet/src/view.rs",
        "components/drawer/test/drawer/semantics.rs::drawer_ssr_and_cross_platform_compile_contract_is_documented_and_non_wasm_safe",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 SSR/cross-platform evidence should include `{needle}`."
        );
    }
}

#[test]
fn drawer_headless_web_ssr_mutex_guard_is_preserved() {
    let sheet_view_source = load_source("../../components/sheet/src/view.rs");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");
    let check2_source = load_source("../../components/drawer/check2.md");

    for needle in [
        "use_focus_trap",
        "use_modal",
        "use_overlay_stack_registration",
    ] {
        assert!(
            sheet_view_source.contains(needle),
            "drawer depends on sheet headless integration; missing `{needle}`."
        );
    }

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should preserve web/ssr mutex compile guard `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "rg -n \"mutually exclusive\" \"$MUTEX_LOG\"",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep ui-headless web/ssr mutex verification `{needle}`."
        );
    }

    for needle in [
        "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。",
        "components/sheet/src/view.rs",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "mutually exclusive",
        "components/drawer/test/drawer/semantics.rs::drawer_headless_web_ssr_mutex_guard_is_preserved",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 ui-headless mutex evidence should include `{needle}`."
        );
    }
}

#[test]
fn drawer_ui_motion_non_wasm_stub_contract_is_predictable_and_tooling_safe() {
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let ui_motion_non_wasm_test_source =
        load_source("../../crates/ui-motion/tests/non_wasm_stub.rs");
    let sheet_motion_source = load_source("../../components/sheet/src/motion.rs");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");
    let check2_source = load_source("../../components/drawer/check2.md");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion should keep non-wasm predictable no-op stub `{needle}`."
        );
    }

    for needle in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
        "web::animate(&(), &keyframes, MotionOptions::default());",
    ] {
        assert!(
            ui_motion_non_wasm_test_source.contains(needle),
            "ui-motion non-wasm regression test should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
        "if !is_open.get() {",
        "finish_exit.run(());",
    ] {
        assert!(
            sheet_motion_source.contains(needle),
            "sheet motion non-wasm degrade path should include `{needle}`."
        );
    }

    let sheet_non_wasm_branch = sheet_motion_source
        .split("#[cfg(not(target_arch = \"wasm32\"))]")
        .nth(1)
        .expect("sheet motion should contain explicit non-wasm branch");
    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "js_sys::",
        "window(",
        "document(",
    ] {
        assert!(
            !sheet_non_wasm_branch.contains(forbidden),
            "sheet non-wasm motion branch should not reference browser token `{forbidden}`."
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep ui-motion toolchain safety check `{needle}`."
        );
    }

    for needle in [
        "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。",
        "crates/ui-motion/tests/non_wasm_stub.rs",
        "components/sheet/src/motion.rs",
        "cargo test -p ui-motion --test non_wasm_stub",
        "components/drawer/test/drawer/semantics.rs::drawer_ui_motion_non_wasm_stub_contract_is_predictable_and_tooling_safe",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 ui-motion non-wasm evidence should include `{needle}`."
        );
    }
}

#[test]
fn drawer_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let drawer_view_source = load_source("src/drawer/view.rs");
    let sheet_view_source = load_source("../../components/sheet/src/view.rs");
    let sheet_motion_source = load_source("../../components/sheet/src/motion.rs");
    let check2_source = load_source("../../components/drawer/check2.md");

    for needle in [
        "ui_motion::web::prefers_reduced_motion()",
        "if prefers_reduced_motion {",
        "if reduced_motion {",
        "finish_exit.run(());",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            sheet_motion_source.contains(needle),
            "sheet motion should keep reduced-motion/ssr/wasm branch marker `{needle}`."
        );
    }

    for needle in [
        "role=\"dialog\"",
        "aria-modal=\"true\"",
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-slot=root_state.slot_attr",
    ] {
        assert!(
            sheet_view_source.contains(needle),
            "sheet view should keep stable semantic attrs across platform branches `{needle}`."
        );
    }

    for needle in [
        "data-state=root_state.state_attr",
        "data-open-state=move || logic::open_state_attr(open.get())",
        "data-open-mode=open_mode_attr",
        "data-open-source=open_value_source.as_attr()",
        "data-open-action-source=move || open_action_source.get().as_attr()",
        "aria_labelledby=title_id.clone()",
        "aria_describedby=description_id.clone()",
    ] {
        assert!(
            drawer_view_source.contains(needle),
            "drawer semantic contract should remain platform-agnostic `{needle}`."
        );
    }

    assert!(
        !drawer_view_source.contains("#[cfg(target_arch = \"wasm32\")]")
            && !drawer_view_source.contains("#[cfg(not(target_arch = \"wasm32\"))]"),
        "drawer view should not split semantic attrs by platform cfg."
    );

    for needle in [
        "- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
        "if prefers_reduced_motion",
        "if reduced_motion",
        "finish_exit.run(())",
        "components/drawer/test/drawer/semantics.rs::drawer_reduced_motion_ssr_wasm_branches_keep_semantics_consistent",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 reduced-motion/ssr/wasm evidence should include `{needle}`."
        );
    }
}

#[test]
fn drawer_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop() {
    drawer_motion_layer_boundaries_delegate_runtime_to_sheet_and_ui_motion();
    drawer_ui_motion_non_wasm_stub_contract_is_predictable_and_tooling_safe();
    drawer_reduced_motion_ssr_wasm_branches_keep_semantics_consistent();
}

#[test]
fn drawer_motion_contract_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    let needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`."
    );
}

#[test]
fn drawer_check2_marks_motion_contract_complete() {
    let source = load_source("src/drawer/check2.md");

    assert!(
        source.contains("- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。"),
        "drawer check2 should mark motion-contract gate complete.",
    );

    for needle in [
        "components/drawer/src/motion.rs",
        "components/sheet/src/motion.rs",
        "crates/ui-motion/src/lib.rs",
        "ui_motion::presets::spring_slide()",
        "ui_motion::web::prefers_reduced_motion()",
        "finish_exit.run(())",
        "scripts/check-ui-contract-hygiene.sh",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop",
        "components/drawer/test/semantics.rs::drawer_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop",
        "components/drawer/test/drawer/semantics.rs::drawer_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "drawer check2 motion-contract section should reference `{needle}`.",
        );
    }
}

#[test]
fn drawer_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let ui_components_lib_source = load_source("src/lib.rs");
    let ui_components_css_source = load_source("src/css.rs");
    let ui_components_root_source = load_source("src/root.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let ui_components_src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let ui_headless_src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../ui-headless/src");

    for needle in [
        "#[cfg(feature = \"component-drawer\")]",
        "pub use ui_drawer as drawer;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "pub use drawer::{Drawer, DrawerMotion, DrawerPlacement};",
    ] {
        assert!(
            ui_components_lib_source.contains(needle),
            "ui lib entry should keep feature-gated drawer/root surface via `{needle}`."
        );
    }

    for forbidden in ["web_sys::", "NodeRef<", "HtmlElement"] {
        assert!(
            !ui_components_lib_source.contains(forbidden),
            "ui lib entry should not leak platform detail token `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "#[cfg(feature = \"component-drawer\")]",
        "out.push_str(crate::drawer::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css_source.contains(needle),
            "ui css entry should stay feature-gated and no-op safe via `{needle}`."
        );
    }

    for needle in [
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root_source.contains(needle),
            "UiRoot should centralize theme/i18n/css injection via `{needle}`."
        );
    }

    for needle in [
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringAnimator::new",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep generic motion contract `{needle}`."
        );
    }

    for forbidden in [
        "Accordion",
        "Drawer",
        "Modal",
        "Popover",
        "Tooltip",
        "MenuItem",
    ] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should not carry component business semantics `{forbidden}`."
        );
    }

    for absent in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        let path = ui_components_src_dir.join(absent);
        assert!(
            !path.exists(),
            "ui should not add forbidden entrypoint file `{}`.",
            path.display()
        );
    }

    for present in ["controllable_state.rs", "presence.rs", "a11y.rs"] {
        let path = ui_headless_src_dir.join(present);
        assert!(
            path.exists(),
            "ui-headless canonical primitive entrypoint should exist `{}`.",
            path.display()
        );
    }
}

#[test]
fn drawer_entrypoints_check_script_covers_fixed_entry_files_gate() {
    let script_source = load_source("../../scripts/check-ui-entrypoints.sh");

    let needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn drawer_check2_marks_ui_components_fixed_entry_files_contract_complete() {
    let source = load_source("src/drawer/check2.md");

    assert!(
        source.contains("- [x] `ui` 固定入口文件落点正确。"),
        "drawer check2 should mark ui fixed-entry gate complete.",
    );

    for needle in [
        "crates/ui/src/lib.rs",
        "crates/ui/src/css.rs",
        "crates/ui/src/root.rs",
        "crates/ui-visual-primitive/src/active_highlight.rs",
        "crates/ui/src/overlay_open.rs",
        "crates/ui/src/presence.rs",
        "crates/ui/src/a11y.rs",
        "crates/ui-headless/src/controllable_state.rs",
        "crates/ui-headless/src/presence.rs",
        "crates/ui-headless/src/a11y.rs",
        "drawer_ui_components_fixed_entry_files_follow_layered_boundaries",
        "scripts/check-ui-entrypoints.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "drawer check2 fixed-entry section should reference `{needle}`.",
        );
    }
}

#[test]
fn drawer_component_directory_standard_files_follow_contract_and_na_paths() {
    let component_src_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/drawer/src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        let path = component_src_dir.join(required);
        assert!(
            path.exists(),
            "drawer component standard file should exist `{}`.",
            path.display()
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        let path = component_src_dir.join(forbidden);
        assert!(
            !path.exists(),
            "drawer simple component should not add forbidden file `{}`.",
            path.display()
        );
    }

    drawer_does_not_expose_logic_or_view_modules();
    drawer_semantics_tests_are_migrated_to_component_test_directory();
    drawer_logic_exposes_state_helpers();
    drawer_view_uses_logic_state_contracts();
    drawer_does_not_introduce_spec_rs_builder_for_simple_scope();
}

#[test]
fn drawer_component_files_check_script_covers_standard_layout_gate() {
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    let needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_component_directory_standard_files_follow_contract_and_na_paths";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn drawer_check2_marks_component_directory_standard_files_contract_complete() {
    let source = load_source("src/drawer/check2.md");

    assert!(
        source.contains("- [x] 组件目录标准文件落点正确。"),
        "drawer check2 should mark component-directory-standard-files gate complete.",
    );

    for needle in [
        "components/drawer/src/mod.rs",
        "components/drawer/src/logic.rs",
        "components/drawer/src/styles.rs",
        "components/drawer/src/view.rs",
        "components/drawer/src/motion.rs",
        "components/drawer/src/render.rs",
        "components/drawer/src/spec.rs",
        "scripts/check-ui-component-files.sh",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_component_directory_standard_files_follow_contract_and_na_paths",
        "components/drawer/test/semantics.rs::drawer_component_directory_standard_files_follow_contract_and_na_paths",
        "components/drawer/test/drawer/semantics.rs::drawer_component_directory_standard_files_follow_contract_and_na_paths",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "drawer check2 component-directory section should reference `{needle}`.",
        );
    }
}

#[test]
fn drawer_file_placement_discipline_is_strict_for_component_scope() {
    let component_src_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/drawer/src");
    let mod_source = load_source("../../components/drawer/src/mod.rs");
    let logic_source = load_source("../../components/drawer/src/logic.rs");
    let styles_source = load_source("../../components/drawer/src/styles.rs");
    let view_source = load_source("../../components/drawer/src/view.rs");
    let motion_source = load_source("../../components/drawer/src/motion.rs");
    let protocol_source = load_source("../../components/drawer/src/protocol.rs");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        let path = component_src_dir.join(required);
        assert!(
            path.exists(),
            "drawer file-placement discipline requires core file `{}`.",
            path.display()
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        let path = component_src_dir.join(forbidden);
        assert!(
            !path.exists(),
            "drawer should not introduce forbidden placement file `{}`.",
            path.display()
        );
    }

    assert!(
        component_src_dir.join("protocol.rs").exists(),
        "drawer keeps versioned schema in protocol.rs as explicit repository-level exception."
    );
    for needle in [
        "pub enum DrawerComponentSchemaVersion",
        "pub struct DrawerComponentSpec",
        "#[serde(default)]",
    ] {
        assert!(
            protocol_source.contains(needle),
            "protocol.rs should stay schema-only via `{needle}`."
        );
    }

    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");
    for needle in [
        "pub use view::Drawer;",
        "pub fn normalize_open_state(",
        "pub const CSS: &str",
        "view! {",
        "pub struct DrawerMotion",
    ] {
        assert!(
            combined.contains(needle),
            "file-placement discipline should keep core-layer marker `{needle}`."
        );
    }
}

#[test]
fn drawer_file_placement_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    let needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_file_placement_discipline_is_strict_for_component_scope";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn drawer_check2_marks_file_placement_discipline_contract_complete() {
    let source = load_source("src/drawer/check2.md");

    assert!(
        source.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
        "drawer check2 should mark file-placement-discipline gate complete.",
    );

    for needle in [
        "components/drawer/src/mod.rs",
        "components/drawer/src/logic.rs",
        "components/drawer/src/styles.rs",
        "components/drawer/src/view.rs",
        "components/drawer/src/motion.rs",
        "components/drawer/src/protocol.rs",
        "render.rs",
        "spec.rs",
        "drawer_file_placement_discipline_is_strict_for_component_scope",
        "scripts/check-ui-component-files.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "drawer check2 file-placement section should reference `{needle}`.",
        );
    }
}

#[test]
fn drawer_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let component_src_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../components/drawer/src");
    let mod_source = load_source("../../components/drawer/src/mod.rs");
    let view_source = load_source("../../components/drawer/src/view.rs");
    let logic_source = load_source("../../components/drawer/src/logic.rs");
    let protocol_source = load_source("../../components/drawer/src/protocol.rs");
    let check2_source = load_source("../../components/drawer/check2.md");

    let spec_path = component_src_dir.join("spec.rs");
    assert!(
        !spec_path.exists(),
        "drawer is not a complex spec-first component; spec.rs should remain absent."
    );

    assert!(
        component_src_dir.join("protocol.rs").exists(),
        "drawer keeps versioned schema in protocol.rs instead of introducing spec.rs."
    );
    for needle in [
        "pub enum DrawerComponentSchemaVersion",
        "pub struct DrawerComponentSpec",
    ] {
        assert!(
            protocol_source.contains(needle),
            "protocol.rs should keep versioned schema marker `{needle}`."
        );
    }

    let combined = format!("{mod_source}\n{view_source}\n{logic_source}");
    for forbidden in [
        "Spec::new()",
        ".render()",
        "pub struct DrawerSpec",
        "impl DrawerSpec",
    ] {
        assert!(
            !combined.contains(forbidden),
            "drawer should not expose complex builder API token `{forbidden}`."
        );
    }

    for needle in [
        "Hyper-Structure Builder（`spec.rs`）",
        "N/A（`drawer` 非复杂 builder/spec 组件",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 should document hyper-structure-builder applicability via `{needle}`."
        );
    }
}

#[test]
fn drawer_hyper_structure_builder_check_script_covers_na_contract() {
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    let needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`."
    );
}

#[test]
fn drawer_check2_marks_hyper_structure_builder_contract_complete() {
    let source = load_source("../../components/drawer/check2.md");

    assert!(
        source.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
        "drawer check2 should mark hyper-structure-builder gate complete.",
    );

    for needle in [
        "components/drawer/src/spec.rs",
        "components/drawer/src/protocol.rs",
        "drawer_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "scripts/check-ui-component-files.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "drawer check2 hyper-structure-builder section should reference `{needle}`.",
        );
    }
}

#[test]
fn drawer_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    for required_file in [
        "../../components/drawer/src/Component.toml",
        "../../components/drawer/src/drawer.rbi",
    ] {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(required_file);
        assert!(
            path.exists(),
            "drawer context-compression artifact should exist: `{required_file}`."
        );
    }

    let manifest_source = load_source("../../components/drawer/src/Component.toml");
    let rbi_source = load_source("../../components/drawer/src/drawer.rbi");
    let view_source = load_source("../../components/drawer/src/view.rs");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Drawer\"",
        "crate = \"ui-drawer\"",
        "rbi = \"drawer.rbi\"",
        "name = \"is_open\"",
        "name = \"default_open\"",
        "name = \"on_open_change\"",
        "name = \"on_close\"",
        "name = \"id_base\"",
        "name = \"title\"",
        "name = \"children\"",
        "name = \"description\"",
        "name = \"footer\"",
        "name = \"placement\"",
        "name = \"motion\"",
        "name = \"is_close_button_visible\"",
        "name = \"close_label\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"on_exit_complete\"",
        "name = \"class_name\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "drawer Component.toml should include context-compression marker `{needle}`."
        );
    }

    for needle in [
        "pub enum DrawerSlot {",
        "pub struct DrawerPartStateInput {",
        "pub struct DrawerPartState {",
        "pub enum DrawerOpenMode {",
        "pub enum DrawerOpenValueSource {",
        "pub enum DrawerOpenActionSource {",
        "pub struct DrawerComponentSpec {",
        "pub fn Drawer(",
        "is_open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
        "on_close: Option<ui_headless::OnPress>",
        "id_base: String",
        "title: String",
        "children: leptos::children::ChildrenFn",
        "description: Option<String>",
        "footer: Option<leptos::children::ViewFn>",
        "placement: Option<crate::logic::DrawerPlacement>",
        "motion: crate::DrawerMotion",
        "is_close_button_visible: Option<bool>",
        "close_label: Option<&'static str>",
        "lang: Option<String>",
        "dir: Option<ui_headless::A11yDirection>",
        "on_exit_complete: Option<leptos::prelude::Callback<()>>",
        "class_name: Option<String>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "drawer RBI projection should keep signature marker `{needle}`."
        );
    }

    for needle in [
        "pub fn Drawer(",
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] on_close: Option<OnPress>",
        "#[prop(optional, into)] description: Option<String>",
        "#[prop(optional, into)] footer: Option<ViewFn>",
        "#[prop(optional)] placement: Option<DrawerPlacement>",
        "#[prop(optional)] motion: DrawerMotion",
        "#[prop(optional)] is_close_button_visible: Option<bool>",
        "#[prop(optional)] close_label: Option<&'static str>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional)]",
        "on_exit_complete: Option<Callback<()>>",
        "#[prop(optional, into)] class_name: Option<String>",
        "children: ChildrenFn,",
    ] {
        assert!(
            view_source.contains(needle),
            "drawer view signature should include `{needle}` for manifest/rbi drift detection."
        );
    }
}

#[test]
fn drawer_component_files_check_script_covers_context_compression_manifest_contract() {
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    for needle in [
        "echo \"[component-files] contract: drawer context-compression manifest + rbi projection\"",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce drawer context-compression gate `{needle}`."
        );
    }
}

#[test]
fn drawer_check2_marks_context_compression_manifest_and_rbi_contract_complete() {
    let source = load_source("../../components/drawer/check2.md");

    assert!(
        source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "drawer check2 should mark context-compression manifest/rbi gate complete.",
    );

    for needle in [
        "components/drawer/src/Component.toml",
        "components/drawer/src/drawer.rbi",
        "drawer_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "drawer_component_files_check_script_covers_context_compression_manifest_contract",
        "scripts/check-ui-component-files.sh",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "drawer check2 context-compression section should reference `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("../../components/drawer/check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "drawer_agent_contract_is_schema_typed_and_machine_readable",
        "drawer_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "drawer_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "drawer checklist should keep Agent Contract governance rule `{required}`."
        );
    }
}

#[test]
fn drawer_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("../../components/drawer/src/logic.rs");
    let view_source = load_source("../../components/drawer/src/view.rs");

    for needle in [
        "pub const DRAWER_AGENT_SCHEMA: &str = \"ui.drawer.agent-contract\";",
        "pub enum DrawerAgentSchemaVersion",
        "pub enum DrawerAgentIntent",
        "pub enum DrawerAgentAction",
        "pub enum DrawerAgentState",
        "pub enum DrawerAgentSource",
        "pub enum DrawerAgentConfigPolicy",
        "pub enum DrawerAgentOutputStatus",
        "pub struct DrawerAgentCapabilities",
        "pub struct DrawerAgentContractInput",
        "pub struct DrawerAgentContract",
        "pub fn resolve_agent_contract(input: DrawerAgentContractInput) -> DrawerAgentContract",
    ] {
        assert!(
            logic_source.contains(needle),
            "drawer logic should keep typed agent contract marker `{needle}`."
        );
    }

    for needle in [
        "let agent_contract = Signal::derive(move || {",
        "logic::resolve_agent_contract(logic::DrawerAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-capability-description=move || {",
        "data-ui-capability-footer=move || {",
        "data-ui-capability-open=move || {",
        "data-ui-capability-close=move || {",
    ] {
        assert!(
            view_source.contains(needle),
            "drawer view should mount schemaized agent marker `{needle}`."
        );
    }
}

#[test]
fn drawer_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let logic_source = load_source("../../components/drawer/src/logic.rs");
    let view_source = load_source("../../components/drawer/src/view.rs");
    let manifest_source = load_source("../../components/drawer/src/Component.toml");

    for typed_source in [
        "schema_name: DRAWER_AGENT_SCHEMA,",
        "schema_version: DrawerAgentSchemaVersion::V1,",
        "intent: DrawerAgentIntent::OverlayDrawer,",
        "DrawerAgentAction::Open",
        "DrawerAgentAction::Close",
        "DrawerAgentState::Open",
        "DrawerAgentState::Closed",
        "DrawerAgentSource::Controlled",
        "DrawerAgentSource::Uncontrolled",
        "config_policy: DrawerAgentConfigPolicy::Whitelist,",
        "output_status: DrawerAgentOutputStatus::Verified,",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "drawer agent fields should stay type-derived via `{typed_source}`."
        );
    }

    for marker in [
        "name = \"schema\"",
        "attr = \"data-ui-schema\"",
        "name = \"schema_version\"",
        "attr = \"data-ui-schema-version\"",
        "name = \"intent\"",
        "attr = \"data-ui-intent\"",
        "name = \"action\"",
        "attr = \"data-ui-action\"",
        "name = \"state\"",
        "attr = \"data-ui-state\"",
        "name = \"source\"",
        "attr = \"data-ui-source\"",
    ] {
        assert!(
            manifest_source.contains(marker),
            "drawer Component.toml should keep typed agent marker `{marker}`."
        );
    }

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
        "schema_name: format!(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "drawer agent contract should avoid free-form schema splicing `{forbidden}`.",
        );
    }
}

#[test]
fn drawer_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("../../components/drawer/src/view.rs");
    let logic_source = load_source("../../components/drawer/src/logic.rs");
    let styles_source = load_source("../../components/drawer/src/styles.rs");
    let mod_source = load_source("../../components/drawer/src/mod.rs");
    let motion_source = load_source("../../components/drawer/src/motion.rs");
    let manifest_source = load_source("../../components/drawer/src/Component.toml");
    let combined =
        format!("{view_source}\n{logic_source}\n{styles_source}\n{mod_source}\n{motion_source}");

    for needle in [
        "[[agent_contract_whitelist]]",
        "typed_state_from_ui_state_primitives::drawer::resolve_open_config",
        "typed_agent_contract_from_logic::resolve_agent_contract",
        "typed_render_mount_from_view::render_drawer_root",
    ] {
        assert!(
            manifest_source.contains(needle),
            "drawer manifest should keep whitelist-safe render-path marker `{needle}`."
        );
    }

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
            "drawer render path should stay whitelist-safe without `{forbidden}`.",
        );
    }
}

#[test]
fn drawer_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_marks_agent_contract_schema_governance_complete() {
    let check2_source = load_source("../../components/drawer/check2.md");

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "drawer_check2_documents_agent_contract_schema_governance_rules",
        "drawer_agent_contract_is_schema_typed_and_machine_readable",
        "drawer_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "drawer_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "drawer_contract_hygiene_script_covers_agent_contract_schema_guards",
        "scripts/check-ui-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 should keep Agent Contract governance marker `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("../../components/drawer/check2.md");
    let view_source = load_source("../../components/drawer/src/view.rs");
    let logic_source = load_source("../../components/drawer/src/logic.rs");
    let mod_source = load_source("../../components/drawer/src/mod.rs");
    let motion_source = load_source("../../components/drawer/src/motion.rs");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "N/A：`Drawer` 不是 LLM 正文渲染组件",
    ] {
        assert!(
            check2_source.contains(required),
            "drawer check2 should keep streaming-definition marker `{required}`."
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-stream-mode",
        "data-stream-fallback",
        "project_streaming_",
        "use_ai_space_state",
    ] {
        assert!(
            !combined.contains(forbidden),
            "drawer runtime path should not embed LLM streaming protocol marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`.",
    );
}

#[test]
fn drawer_streaming_script_covers_two_mode_definition_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    let needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`.",
    );
}

#[test]
fn drawer_check2_marks_streaming_two_mode_definition_complete() {
    let check2_source = load_source("../../components/drawer/check2.md");

    assert!(
        check2_source.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
        "drawer check2 should mark streaming two-mode definition gate complete.",
    );

    for needle in [
        "drawer_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "drawer_streaming_script_covers_two_mode_definition_contract",
        "scripts/check-ui-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 streaming section should reference `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_source("../../components/drawer/check2.md");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "N/A：`Drawer` 不直接渲染 LLM 正文",
        "drawer_check2_documents_snapshot_as_default_baseline_capability",
        "scripts/check-ui-streaming.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 should keep snapshot-baseline marker `{needle}`."
        );
    }
}

#[test]
fn drawer_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("../../components/drawer/src/view.rs");
    let logic_source = load_source("../../components/drawer/src/logic.rs");
    let check2_source = load_source("../../components/drawer/check2.md");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "let open_state = logic::normalize_open_state(logic::DrawerOpenStateInput {",
        "let view_config = logic::normalize_view_config(logic::DrawerViewConfigInput {",
        "let part_states = logic::resolve_part_states(logic::DrawerPartStatesInput {",
        "let part_classes = logic::resolve_part_classes(class_name, part_states);",
        "data-open-state=move || logic::open_state_attr(open.get())",
        "data-open-mode=open_mode_attr",
        "data-open-source=open_value_source.as_attr()",
        "data-open-action-source=move || open_action_source.get().as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "drawer snapshot baseline should keep stable complete-result render marker `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_open_state(input: DrawerOpenStateInput) -> DrawerOpenState",
        "pub fn normalize_view_config(input: DrawerViewConfigInput) -> DrawerViewConfig",
        "pub fn resolve_part_states(input: DrawerPartStatesInput) -> DrawerPartStates",
        "pub fn resolve_part_classes(",
    ] {
        assert!(
            logic_source.contains(needle),
            "drawer logic should keep snapshot-baseline normalization marker `{needle}`."
        );
    }

    for forbidden in [
        "streaming_chunk",
        "token_delta",
        "partial token",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-stream-mode",
        "data-stream-fallback",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "drawer snapshot baseline should avoid incremental streaming marker `{forbidden}`.",
        );
    }

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "drawer_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 snapshot section should reference `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`.",
    );
}

#[test]
fn drawer_streaming_script_covers_snapshot_baseline_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_marks_snapshot_baseline_capability_complete() {
    let check2_source = load_source("../../components/drawer/check2.md");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "drawer_check2_documents_snapshot_as_default_baseline_capability",
        "drawer_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "drawer_streaming_script_covers_snapshot_baseline_contract",
        "scripts/check-ui-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 snapshot-baseline section should reference `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_documents_streaming_required_optional_classification_rules() {
    let check2_source = load_source("../../components/drawer/check2.md");
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "`Drawer` 归类为 `Streaming Optional`",
        "fallback=snapshot",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 should keep streaming required/optional rule `{needle}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming check script should enforce `{script_needle}`.",
        );
    }
}

#[test]
fn drawer_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let drawer_view_source = load_source("../../components/drawer/src/view.rs");
    let drawer_logic_source = load_source("../../components/drawer/src/logic.rs");
    let sheet_view_source = load_source("../../components/sheet/src/view.rs");
    let sheet_logic_source = load_source("../../components/sheet/src/logic.rs");

    for needle in [
        "<Sheet",
        "aria_labelledby=title_id.clone()",
        "aria_describedby=description_id.clone()",
        "lang=lang.clone()",
        "dir=dir",
        "data-open-state=move || logic::open_state_attr(open.get())",
        "data-open-mode=open_mode_attr",
        "data-open-source=open_value_source.as_attr()",
        "data-open-action-source=move || open_action_source.get().as_attr()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            drawer_view_source.contains(needle),
            "drawer optional-streaming scope should keep semantic continuity marker `{needle}`.",
        );
    }

    for needle in [
        "pub enum DrawerAgentOutputStatus",
        "DrawerAgentOutputStatus::Verified",
        "data-ui-output-status",
    ] {
        assert!(
            drawer_logic_source.contains(needle) || drawer_view_source.contains(needle),
            "drawer optional-streaming scope should expose explicit output-status marker `{needle}`.",
        );
    }

    for needle in [
        "role=\"dialog\"",
        "aria-modal=\"true\"",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-ui-streaming=agent_contract.streaming_attr",
        "data-ui-fallback=agent_contract.fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
    ] {
        assert!(
            sheet_view_source.contains(needle),
            "sheet bridge should keep role/aria/data continuity marker `{needle}` for drawer optional-streaming path.",
        );
    }

    for needle in [
        "render_mode_attr: \"snapshot\"",
        "streaming_attr: \"optional\"",
        "fallback_attr: \"snapshot\"",
        "output_status_attr: \"verified\"",
    ] {
        assert!(
            sheet_logic_source.contains(needle),
            "sheet agent contract should keep explicit optional-streaming fallback marker `{needle}`.",
        );
    }
}

#[test]
fn drawer_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("../../components/drawer/src/view.rs");
    let logic_source = load_source("../../components/drawer/src/logic.rs");
    let mod_source = load_source("../../components/drawer/src/mod.rs");
    let motion_source = load_source("../../components/drawer/src/motion.rs");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "validate_stream",
        "stream_error",
        "network_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "drawer should keep validation/retry/resilience policy outside component layer; found `{forbidden}`.",
        );
    }
}

#[test]
fn drawer_streaming_script_covers_required_optional_classification_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_marks_streaming_required_optional_classification_complete() {
    let source = load_source("../../components/drawer/check2.md");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "drawer_check2_documents_streaming_required_optional_classification_rules",
        "drawer_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "drawer_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
        "drawer_streaming_script_covers_required_optional_classification_contract",
        "scripts/check-ui-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "drawer check2 should keep required/optional classification evidence marker `{needle}`.",
        );
    }
}

#[test]
fn drawer_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let check2_source = load_source("src/drawer/check2.md");
    let perf_script_source = load_source("../../scripts/check-ui-performance.sh");
    let docs_shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let drawer_logic_source = load_source("src/drawer/logic.rs");
    let drawer_view_source = load_source("src/drawer/view.rs");
    let button_check2_source = load_source("../../components/button/check2.md");
    let input_check2_source = load_source("../../components/text-input/src/input/check2.md");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "component_page_perf_budget + UiPerfProbe",
        "use_ui_trace()/trace.emit",
        "drawer_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "docs_perf_probe_budgets_are_wired_for_component_pages",
        "perf_render_count_follow_up_is_tracked_in_plan",
        "渲染次数预算为 `1`",
        "mount-only + trace 等价证据过渡",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 performance evidence should include `{needle}`."
        );
    }

    for needle in [
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget {",
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            docs_shell_source.contains(needle),
            "docs shell should keep performance baseline token `{needle}`."
        );
    }

    for needle in ["use_ui_trace()", "trace.emit("] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace attribution token `{needle}`."
        );
    }

    for needle in [
        "pub fn normalize_open_state(input: DrawerOpenStateInput) -> DrawerOpenState",
        "pub fn resolve_part_states(input: DrawerPartStatesInput) -> DrawerPartStates",
        "data-open-source=open_value_source.as_attr()",
        "data-open-action-source=move || open_action_source.get().as_attr()",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-exit-source=root_state.exit_source_attr",
    ] {
        let found = drawer_logic_source.contains(needle) || drawer_view_source.contains(needle);
        assert!(
            found,
            "drawer should keep performance attribution marker `{needle}`."
        );
    }

    for forbidden in [
        "on:mousemove=",
        "on:pointermove=",
        "on:touchmove=",
        "request_animation_frame",
        "ResizeObserver",
        "IntersectionObserver",
        "set_interval(",
        "spawn_local(",
    ] {
        assert!(
            !drawer_logic_source.contains(forbidden) && !drawer_view_source.contains(forbidden),
            "drawer should avoid high-frequency performance flood token `{forbidden}`."
        );
    }

    for needle in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
    ] {
        assert!(
            button_check2_source.contains(needle) && input_check2_source.contains(needle),
            "Button/Input shared baseline should include `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion",
    ] {
        assert!(
            todo_source.contains(needle),
            "todo plan should keep render_count follow-up token `{needle}`."
        );
    }
}

#[test]
fn drawer_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
{
    let local_semantics = load_source("../../components/drawer/test/semantics.rs");
    let aggregated_semantics = load_source("tests/drawer/semantics.rs");
    let drawer_view_source = load_source("src/drawer/view.rs");
    let sheet_view_source = load_source("../../components/sheet/src/view.rs");
    let focus_trap_source = load_source("../../crates/ui-headless/src/focus_trap.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for required_test in [
        "fn drawer_semantics_tests_cover_contract_matrix_without_snapshot_dependency()",
        "fn drawer_focus_stack_gc_contract_uses_global_focus_manager_and_policy_restore()",
        "fn drawer_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn drawer_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            local_semantics.contains(required_test) && aggregated_semantics.contains(required_test),
            "semantic/performance regression suite should include `{required_test}` in local and aggregated tests."
        );
    }

    for marker in [
        "data-state=root_state.state_attr",
        "data-open-mode=open_mode_attr",
        "data-open-source=open_value_source.as_attr()",
        "data-open-action-source=move || open_action_source.get().as_attr()",
        "aria_labelledby=title_id.clone()",
        "aria_describedby=description_id.clone()",
    ] {
        assert!(
            drawer_view_source.contains(marker),
            "drawer view should expose semantic/data marker `{marker}`."
        );
    }

    for marker in [
        "role=\"dialog\"",
        "aria-modal=\"true\"",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
        "on:keydown=on_key_down",
        "on:pointerdown=move |ev| ev.stop_propagation()",
    ] {
        assert!(
            sheet_view_source.contains(marker),
            "sheet bridge should expose aria/focus interaction marker `{marker}`."
        );
    }

    for marker in [
        "focus_manager_push_trap(FocusTrapFrame {",
        "focus_manager_pop_trap",
        "restore_focus_chain(",
    ] {
        assert!(
            focus_trap_source.contains(marker),
            "ui-headless focus manager stack should expose focus-flow marker `{marker}`."
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion",
    ] {
        assert!(
            todo_source.contains(marker),
            "render_count follow-up governance should include `{marker}`."
        );
    }
}

#[test]
fn drawer_semantics_and_performance_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-performance.sh");

    for marker in [
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance check script should include `{marker}`."
        );
    }
}

#[test]
fn drawer_check2_marks_semantics_and_performance_regression_contract_complete() {
    let check2_source = load_source("src/drawer/check2.md");

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "drawer_semantics_tests_cover_contract_matrix_without_snapshot_dependency",
        "drawer_focus_stack_gc_contract_uses_global_focus_manager_and_policy_restore",
        "drawer_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "drawer_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "`render_count` 自动化回归仍在仓库统一 follow-up",
        "scripts/check-ui-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "drawer check2 semantic/performance section should include `{marker}`."
        );
    }
}

#[test]
fn drawer_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("src/drawer/view.rs");
    let check2_source = load_source("../../components/drawer/check2.md");
    let view_macro_script_source = load_source("../../scripts/check-ui-view-macro.sh");

    for needle in [
        "fn render_drawer_close(",
        "fn render_drawer_header(",
        "fn render_drawer_body(",
        "fn render_drawer_footer(",
        "fn render_drawer_root(",
        "{render_drawer_close(close)}",
        "{render_drawer_header(header)}",
        "{render_drawer_body(body)}",
        "{render_drawer_footer(footer)}",
        "{render_drawer_root(root_inputs)}",
    ] {
        assert!(
            view_source.contains(needle),
            "drawer view should keep semantic subrender split marker `{needle}`."
        );
    }

    assert_eq!(
        view_source
            .matches("data-open-action-source=move || open_action_source.get().as_attr()")
            .count(),
        1,
        "drawer should keep a single root container marker path after macro split."
    );

    assert!(
        view_source.matches("view! {").count() <= 8,
        "drawer view! macro expansion should stay bounded after semantic subrender split."
    );

    assert!(
        !view_source.contains("let children = children.get_value();"),
        "drawer should avoid repeated nested children extraction in duplicated macro branches."
    );

    let needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        view_macro_script_source.contains(needle),
        "view-macro gate script should include `{needle}`."
    );

    for needle in [
        "- [x] `view!` 宏复杂度受控：",
        "render_drawer_root",
        "drawer_view_macro_complexity_is_split_into_semantic_subrenders",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer checklist should keep view-macro complexity evidence `{needle}`."
        );
    }
}

#[test]
fn drawer_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("src/drawer/view.rs");
    let check2_source = load_source("../../components/drawer/check2.md");
    let view_macro_script_source = load_source("../../scripts/check-ui-view-macro.sh");

    for needle in [
        "fn render_drawer_close(inputs: DrawerCloseInputs) -> impl IntoView {",
        "fn render_drawer_header(inputs: DrawerHeaderInputs) -> impl IntoView {",
        "fn render_drawer_body(inputs: DrawerBodyInputs) -> impl IntoView {",
        "fn render_drawer_footer(inputs: DrawerFooterInputs) -> impl IntoView {",
        "fn render_drawer_root(inputs: DrawerRootInputs) -> impl IntoView {",
        "pub fn Drawer(",
    ] {
        assert!(
            view_source.contains(needle),
            "drawer view should keep plain function split marker `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "drawer view should keep only one component entrypoint after function split."
    );

    for forbidden in [
        "#[component]\nfn render_drawer_close(",
        "#[component]\nfn render_drawer_header(",
        "#[component]\nfn render_drawer_body(",
        "#[component]\nfn render_drawer_footer(",
        "#[component]\nfn render_drawer_root(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "drawer helper subviews should stay plain functions, not nested components `{forbidden}`."
        );
    }

    let needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        view_macro_script_source.contains(needle),
        "view-macro gate script should include `{needle}`."
    );

    for needle in [
        "- [x] 函数式拆分优先：",
        "render_drawer_root",
        "drawer_view_functional_split_prefers_plain_functions_over_local_components",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer checklist should keep functional split evidence `{needle}`."
        );
    }
}

#[test]
fn drawer_static_fragments_are_constantized_or_absent_for_simple_overlay_layout() {
    let view_source = load_source("src/drawer/view.rs");
    let check2_source = load_source("../../components/drawer/check2.md");
    let view_macro_script_source = load_source("../../scripts/check-ui-view-macro.sh");

    for needle in [
        "const DRAWER_CLOSE_ICON_VIEWBOX: &str = \"0 0 20 20\";",
        "const DRAWER_CLOSE_ICON_PATH: &str = \"M5 5l10 10M15 5L5 15\";",
        "const DRAWER_CLOSE_ICON_STROKE_WIDTH: &str = \"1.5\";",
        "fn render_drawer_close_icon() -> impl IntoView {",
        "<svg viewBox=DRAWER_CLOSE_ICON_VIEWBOX fill=\"none\" aria-hidden=\"true\">",
        "d=DRAWER_CLOSE_ICON_PATH",
        "stroke_width=DRAWER_CLOSE_ICON_STROKE_WIDTH",
        "{render_drawer_close_icon()}",
    ] {
        assert!(
            view_source.contains(needle),
            "drawer view should keep static-fragment constantization marker `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("<svg").count(),
        1,
        "drawer should keep a single close-icon svg template after static-fragment constantization."
    );

    for forbidden in [
        "<svg viewBox=\"0 0 20 20\"",
        "d=\"M5 5l10 10M15 5L5 15\"",
        "inner_html",
        "set_inner_html(",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "drawer should avoid inline-heavy static fragment token `{forbidden}`."
        );
    }

    let needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_static_fragments_are_constantized_or_absent_for_simple_overlay_layout";
    assert!(
        view_macro_script_source.contains(needle),
        "view-macro gate script should include `{needle}`."
    );

    for needle in [
        "- [x] 静态片段常量化：",
        "DRAWER_CLOSE_ICON_PATH",
        "render_drawer_close_icon",
        "drawer_static_fragments_are_constantized_or_absent_for_simple_overlay_layout",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer checklist should keep static-fragment constantization evidence `{needle}`."
        );
    }
}

#[test]
fn drawer_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    for rel_path in [
        "../../components/drawer/src/mod.rs",
        "../../components/drawer/src/logic.rs",
        "../../components/drawer/src/styles.rs",
        "../../components/drawer/src/view.rs",
        "../../components/drawer/src/motion.rs",
        "../../components/drawer/src/README.md",
    ] {
        let source = load_source(rel_path);
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "markdown_to_html(",
            "<script",
            "javascript:",
            "onerror=",
            "onload=",
        ] {
            assert!(
                !source.contains(forbidden),
                "drawer source `{rel_path}` must not contain raw-html injection token `{forbidden}`."
            );
        }
    }

    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "drawer docs examples must not contain raw-html injection token `{forbidden}`."
        );
    }

    let check2_source = load_source("../../components/drawer/check2.md");
    for needle in [
        "- [x] `inner_html` 使用约束：",
        "零注入面",
        "drawer_inner_html_usage_is_forbidden_in_component_and_docs_examples",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer checklist should keep inner_html security evidence `{needle}`."
        );
    }
}

#[test]
fn drawer_inner_html_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-inner-html.sh");

    let needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(needle),
        "inner-html check script should enforce `{needle}`."
    );
}

#[test]
fn drawer_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let controllable_state_source = load_source("../ui-headless/src/controllable_state.rs");
    let drawer_view_source = load_source("src/drawer/view.rs");
    let drawer_logic_source = load_source("src/drawer/logic.rs");
    let docs_overlays_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let check2_source = load_source("../../components/drawer/check2.md");

    for needle in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "sheet-wasm-debug = [\"component-sheet\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui Cargo features should keep shared wasm-debug marker `{needle}`."
        );
    }
    assert!(
        !cargo_source.contains("drawer-wasm-debug"),
        "Drawer should not define a component-local wasm-debug feature that leaks into production API surface."
    );

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui root should keep wasm-debug isolation marker `{needle}`."
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(needle),
            "docs-app should expose dev-only wasm debug entry via `{needle}`."
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "ui_headless::UiTraceEventKind::Inspect",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep visual/temporal trace marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "Inspect {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "ts_ms: now_ms(),",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace should keep typed timestamp/source event marker `{needle}`."
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "trace.emit(component, UiTraceEventKind::OpenChange { open: next });",
    ] {
        assert!(
            controllable_state_source.contains(needle),
            "ui-headless controllable state should emit open-change trace event via `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, OnPress, use_controllable_open_state_traced};",
        "let open_state_signal = use_controllable_open_state_traced(",
        "\"drawer\",",
        "data-open-mode=open_mode_attr",
        "data-open-source=open_value_source.as_attr()",
        "data-open-action-source=move || open_action_source.get().as_attr()",
    ] {
        assert!(
            drawer_view_source.contains(needle),
            "Drawer should expose reproducible interaction/state markers for debug tracing via `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn drawer() -> AnyView {",
        "title=\"State + Source Markers\"",
        "on_press=open_custom_drawer",
        "\"Open left drawer\"",
        "\"open: \" {move || open_custom_raw.get()}",
        "on_close=close_custom",
        "on_exit_complete=on_custom_exit_complete",
        "Inspect data-placement-source / data-title-source / data-motion-source in DevTools.",
    ] {
        assert!(
            docs_overlays_source.contains(needle),
            "Drawer docs playground should keep minimal replay path marker `{needle}`."
        );
    }

    let combined = format!("{drawer_view_source}\n{drawer_logic_source}");
    for forbidden in [
        "wasm_debug_proxy!",
        "observability::",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
        "request_replay",
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Drawer component contract should not leak wasm-debug internals `{forbidden}`."
        );
    }

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "use_controllable_open_state_traced(",
        "drawer_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer checklist should keep wasm-debug evidence `{needle}`."
        );
    }
}

#[test]
fn drawer_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-wasm-debug.sh");

    let needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm-debug check script should enforce `{needle}`."
    );
}

#[test]
fn drawer_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn drawer() -> AnyView",
        "title=\"Hello World (Minimal API)\"",
        "title=\"Right Drawer + Slots\"",
        "code_signal=semantic_code",
        "title=\"State + Source Markers\"",
        "<Drawer",
        "open_custom_raw",
        "open_semantic_raw",
    ] {
        assert!(
            docs_source.contains(needle),
            "Drawer docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
fn drawer_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na()
{
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let check2_source = load_source("../../components/drawer/check2.md");

    for needle in [
        "<div data-playground-scope=scope_id.clone()>",
        "<Card class_name=\"playground__preview\".to_string()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "data-slot=\"playground-controls\"",
        "class_name=\"playground__panel playground__controls\".to_string()",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas contract marker `{needle}`."
        );
    }

    for needle in [
        "let (open_semantic_raw, set_open_semantic_raw) = signal(false);",
        "let open_semantic: Signal<bool> = Signal::derive(move || open_semantic_raw.get());",
        "let open_semantic_drawer: OnPress = Callback::new(move |_| set_open_semantic_raw.set(true));",
        "let (open_custom_raw, set_open_custom_raw) = signal(false);",
        "let open_custom: Signal<bool> = Signal::derive(move || open_custom_raw.get());",
        "let open_custom_drawer: OnPress = Callback::new(move |_| set_open_custom_raw.set(true));",
        "\"open: \" {move || open_semantic_raw.get()}",
        "\"open: \" {move || open_custom_raw.get()}",
        "on_close=close_custom",
        "on_exit_complete=on_custom_exit_complete",
        "Inspect data-placement-source / data-title-source / data-motion-source in DevTools.",
    ] {
        assert!(
            docs_source.contains(needle),
            "Drawer docs should keep context-preserving interactive marker `{needle}`."
        );
    }

    for forbidden in [
        "DRAWER_WORKBENCH_STORAGE_KEY",
        "load_drawer_workbench_state(",
        "save_drawer_workbench_state(",
        "clear_drawer_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Drawer keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
        );
    }

    for required in [
        "DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
    ] {
        assert!(
            check2_source.contains(required),
            "Drawer checklist should keep DX governance rule `{required}`."
        );
    }
}

#[test]
fn drawer_dx_check_script_covers_hot_reload_and_isolated_canvas_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn drawer_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_source("src/drawer/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(required),
            "drawer check2 should keep docs-sync/state-matrix rule `{required}`."
        );
    }
}

#[test]
fn drawer_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let logic_source = load_source("src/drawer/logic.rs");
    let view_source = load_source("src/drawer/view.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/drawer.rs");
    let check2_source = load_source("src/drawer/check2.md");

    for needle in [
        "pub const DEFAULT_ID_BASE: &str = \"ui-drawer\";",
        "pub const DEFAULT_TITLE: &str = \"Drawer\";",
        "pub const DEFAULT_OPEN: bool = false;",
        "pub struct DrawerOpenStateInput {",
        "pub is_open: Option<Signal<bool>>",
        "pub default_open: Option<bool>",
        "pub on_open_change: Option<Callback<bool>>",
        "default_open: input.default_open.unwrap_or(DEFAULT_OPEN)",
    ] {
        assert!(
            logic_source.contains(needle) || primitive_source.contains(needle),
            "drawer API/default contract should keep marker `{needle}` for docs sync."
        );
    }

    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            view_source.contains(needle),
            "drawer view props should keep API marker `{needle}` for docs sync."
        );
    }

    for needle in [
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"drawer-state-matrix\"",
        "data-slot=\"drawer-controlled-uncontrolled\"",
        "is_open=state_matrix_open",
        "default_open=state_matrix_default_open.get()",
        "on_open_change=on_state_matrix_open_change",
        "is_open=compare_controlled_open",
        "default_open=true",
        "on_open_change=on_compare_uncontrolled_open_change",
        "data-slot=\"drawer-defaults-contract\"",
        "components/drawer/src/logic.rs",
        "id_base=\\\"ui-drawer\\\"",
        "title=\\\"Drawer\\\"",
        "default_open=false",
    ] {
        assert!(
            docs_source.contains(needle),
            "drawer docs should keep synced example/matrix/default marker `{needle}`."
        );
    }

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "apps/docs-app/src/pages/components/pages/overlays.rs::drawer",
        "drawer_check2_documents_docs_sync_and_state_matrix_rules",
        "drawer_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "components/drawer/check2.md should keep docs-sync evidence marker `{needle}`."
        );
    }
}

#[test]
fn drawer_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: drawer docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include docs-sync/state-matrix marker `{needle}`."
        );
    }
}

#[test]
fn drawer_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let source = load_source("src/drawer/check2.md");

    assert!(
        source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "drawer check2 should mark docs-sync/state-matrix checklist item complete."
    );

    for needle in [
        "apps/docs-app/src/pages/components/pages/overlays.rs::drawer",
        "State Matrix",
        "Controlled vs Uncontrolled",
        "data-slot=\"drawer-defaults-contract\"",
        "id_base=\"ui-drawer\"",
        "title=\"Drawer\"",
        "default_open=false",
        "drawer_check2_documents_docs_sync_and_state_matrix_rules",
        "drawer_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "drawer_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "drawer check2 docs-sync/state-matrix section should reference `{needle}`."
        );
    }
}

#[test]
fn drawer_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("src/drawer/check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 documentation-as-product section should include `{needle}`.",
        );
    }
}

#[test]
fn drawer_documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = load_source("../../components/drawer/src/README.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "# Drawer",
        "## Hello World（最小可用）",
        "## 先用起来，再进阶",
        "## 常见用法",
        "默认路径：先用 `default_open + id_base + title + on_close`",
        "进阶控制：按需启用 `is_open + default_open + on_open_change`",
        "### Controlled Example（高级入口）",
    ] {
        assert!(
            readme_source.contains(needle),
            "drawer README should include beginner-first marker `{needle}`.",
        );
    }

    let readme_hello = readme_source
        .find("## Hello World（最小可用）")
        .expect("drawer README should include hello-world section");
    let readme_beginner = readme_source
        .find("## 先用起来，再进阶")
        .expect("drawer README should include beginner-first progression section");
    let readme_common = readme_source
        .find("## 常见用法")
        .expect("drawer README should include common-usage section");
    let readme_advanced = readme_source
        .find("### Controlled Example（高级入口）")
        .expect("drawer README should include controlled advanced section");
    assert!(
        readme_hello < readme_beginner
            && readme_beginner < readme_common
            && readme_common < readme_advanced,
        "drawer README should keep beginner-first progression order (hello -> beginner -> common -> advanced).",
    );

    for needle in [
        "component_doc!(\"Drawer\", \"drawer\", \"Overlays\", overlays::drawer),",
        "pub(super) fn drawer() -> AnyView",
        "title=\"Drawer\"",
        "slug=\"drawer\"",
        "title=\"Hello World (Minimal API)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
    ] {
        assert!(
            pages_source.contains(needle) || docs_source.contains(needle),
            "drawer docs entry should include `{needle}`.",
        );
    }
}

#[test]
fn drawer_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: drawer documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_documentation_as_product_rules",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include documentation-as-product marker `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_marks_documentation_as_product_item_complete() {
    let check2_source = load_source("src/drawer/check2.md");

    assert!(
        check2_source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "drawer check2 should mark documentation-as-product item complete.",
    );

    for needle in [
        "components/drawer/src/README.md",
        "apps/docs-app/src/pages/components/pages.rs",
        "## Hello World（最小可用）",
        "## 先用起来，再进阶",
        "## 常见用法",
        "drawer_check2_documents_documentation_as_product_rules",
        "drawer_documentation_entry_exists_with_beginner_first_progression",
        "drawer_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 documentation-as-product section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = load_source("src/drawer/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 heroui-benchmark docs-sync section should include `{needle}`.",
        );
    }
}

#[test]
fn drawer_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let readme_source = load_source("../../components/drawer/src/README.md");

    for needle in [
        "### Drawer 同步记录（2026-02-20）",
        "参数模型同步：`Drawer` 参数主轴保持 `is_open/default_open/on_open_change`",
        "component_doc!(\"Drawer\", \"drawer\", \"Overlays\", overlays::drawer)",
        "`apps/docs-app/src/pages/components/pages/overlays.rs::drawer()`",
        "`components/drawer/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "heroui strategy doc should include drawer synchronization marker `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"Drawer\"",
        "\"drawer\"",
        "overlays::drawer",
    ] {
        assert!(
            pages_source.contains(needle),
            "component docs index should expose drawer entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn drawer() -> AnyView {",
        "title=\"Drawer\"",
        "slug=\"drawer\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app drawer page should stay indexable via marker `{needle}`.",
        );
    }

    for needle in ["# Drawer", "## Hello World（最小可用）"] {
        assert!(
            readme_source.contains(needle),
            "drawer README should remain an equivalent component doc entry via `{needle}`.",
        );
    }
}

#[test]
fn drawer_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: drawer heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce heroui-benchmark docs-sync contract `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("src/drawer/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "drawer_check2_documents_heroui_benchmark_docs_sync_rules",
        "drawer_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "drawer_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 should keep heroui-benchmark docs-sync evidence marker `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("src/drawer/check2.md");

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 interactive-playground section should include `{needle}`.",
        );
    }
}

#[test]
fn drawer_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn drawer() -> AnyView",
        "title=\"State Matrix\"",
        "description=\"State matrix over controlled/uncontrolled + default_open + description branches.\"",
        "data-slot=\"drawer-state-matrix\"",
        "SegmentedControl",
        "id_base=\"docs-drawer-state-matrix-scenario\".to_string()",
        "is_open=state_matrix_open",
        "default_open=state_matrix_default_open.get()",
        "on_open_change=on_state_matrix_open_change",
        "\"mode: \"",
        "\"default_open: \"",
        "\"with_description: \"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"drawer-controlled-uncontrolled\"",
        "is_open=compare_controlled_open",
        "default_open=true",
        "on_open_change=on_compare_uncontrolled_open_change",
        "\"open: \"",
        "\"open (reported by on_open_change): \"",
    ] {
        assert!(
            docs_source.contains(needle),
            "drawer docs interactive playground should include `{needle}`.",
        );
    }
}

#[test]
fn drawer_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_drawer_contract.spec.mjs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "docs-app drawer key flow is repeatable with semantic breakpoints",
        "docs-app drawer high-risk paths keep overlay focus keyboard and settled semantic breakpoints",
        "[data-slot=\"drawer-e2e-right-controls\"]",
        "[data-slot=\"drawer-e2e-open-right\"]",
        "[data-slot=\"drawer-e2e-custom-controls\"]",
        "[data-slot=\"drawer-e2e-open-custom\"]",
        "for (const cycle of [1, 2]) {",
        "drawer key flow cycle ${cycle}",
        "await expect(rightDrawer).toHaveAttribute(\"data-open-mode\", \"controlled\");",
        "await expect(rightDrawer).toHaveAttribute(\"data-open-source\", \"external\");",
        "await expect(customDrawer).toHaveAttribute(\"data-motion-source\", \"custom\");",
        "await expect(customDrawer).toHaveAttribute(\"data-placement\", \"left\");",
        "await expectDrawerSettledClosed(rightPanel, rightDrawer, rightOverlay);",
        "await expectDrawerSettledClosed(customPanel, customDrawer, customOverlay);",
    ] {
        assert!(
            e2e_source.contains(needle),
            "drawer interactive e2e flow should include `{needle}`.",
        );
    }

    for needle in [
        "data-slot=\"drawer-e2e-right-controls\"",
        "data-slot=\"drawer-e2e-open-right\"",
        "data-slot=\"drawer-e2e-custom-controls\"",
        "data-slot=\"drawer-e2e-open-custom\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "drawer docs should expose stable interactive anchor `{needle}` for repeatable e2e replay.",
        );
    }
}

#[test]
fn drawer_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: drawer interactive playground docs acceptance surface\"",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_interactive_playground_rules",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include interactive-playground marker `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_marks_interactive_playground_item_complete() {
    let check2_source = load_source("src/drawer/check2.md");

    assert!(
        check2_source.contains(
            "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"
        ),
        "drawer check2 should mark interactive-playground item complete.",
    );

    for needle in [
        "State Matrix",
        "Controlled vs Uncontrolled",
        "apps/docs-app/src/pages/components/pages/overlays.rs::drawer",
        "N/A：`Drawer` 非 AI Spec 组件",
        "drawer_check2_documents_interactive_playground_rules",
        "drawer_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "drawer_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "drawer_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 interactive-playground section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn drawer_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "const DRAWER_DOC_IMPORTS: &str =",
        "use leptos::prelude::*;\\nuse ui::{Button, ButtonVariant, Drawer, DrawerMotion, DrawerPlacement, OnPress, SheetMotion};",
        "code_imports=DRAWER_DOC_IMPORTS.to_string()",
        "title=\"Hello World (Minimal API)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "requested mode:",
        "requested output status:",
        "effective component status: data-ui-output-status=verified",
        "data-slot=\"drawer-source-first\"",
        "data-slot=\"drawer-source-paths\"",
        "component-drawer",
        "inject-css",
        "compose_copy_ready_code",
    ] {
        assert!(
            docs_source.contains(needle),
            "drawer docs should keep copy-ready + streaming/snapshot contract `{needle}`.",
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str =",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "missing_import_lines(&raw, &imports)",
        "code_imports: Option<String>",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground copy pipeline should keep import completion marker `{needle}`.",
        );
    }
}

#[test]
fn drawer_dx_check_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    let needle = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract";
    assert!(
        script_source.contains(needle),
        "DX check script should enforce `{needle}`.",
    );
}

#[test]
fn drawer_check2_marks_docs_product_copy_paste_ready_contract_complete() {
    let source = load_source("../../components/drawer/check2.md");

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "Hello World (Minimal API)",
        "State Matrix",
        "Controlled vs Uncontrolled",
        "Streaming / Snapshot Contract",
        "DRAWER_DOC_IMPORTS",
        "compose_copy_ready_code",
        "drawer_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract",
        "drawer_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "drawer check2 docs-product section should reference `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("src/drawer/check2.md");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 source-first section should include `{needle}`.",
        );
    }
}

#[test]
fn drawer_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "data-slot=\"drawer-source-first\"",
        "data-slot=\"drawer-source-paths\"",
        "<code>\"Show code\"</code>",
        "DRAWER_DOC_IMPORTS",
        "compose_copy_ready_code",
        "component-drawer",
        "inject-css",
        "components/drawer/src/mod.rs",
        "components/drawer/src/logic.rs",
        "components/drawer/src/view.rs",
        "components/drawer/src/styles.rs",
        "components/drawer/src/motion.rs",
    ] {
        assert!(
            docs_source.contains(needle),
            "drawer source-first docs should include `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "code_imports",
        "data-slot=\"playground-toggle-code\"",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground copy pipeline should include `{needle}`.",
        );
    }
}

#[test]
fn drawer_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "echo \"[dx] contract: drawer source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include source-first marker `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_source("src/drawer/check2.md");

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "drawer check2 should mark source-first copy-paste-ready item complete."
    );

    for needle in [
        "apps/docs-app/src/pages/components/pages/overlays.rs::drawer",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "drawer_check2_documents_source_first_copy_paste_ready_rules",
        "drawer_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "drawer_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 source-first section should retain marker `{needle}`.",
        );
    }
}

#[test]
fn drawer_engineering_contract_uses_serde_protocol_and_structured_schema_defaults() {
    let mod_source = load_source("src/drawer/mod.rs");
    let protocol_source = load_source("src/drawer/protocol.rs");

    assert!(
        mod_source.contains("pub mod protocol;"),
        "drawer module should expose `protocol` for schema contract discoverability."
    );

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "pub enum DrawerComponentSchemaVersion",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(rename_all = \"snake_case\")]",
        "pub struct DrawerComponentSpec",
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(default)]",
        "pub schema_version: DrawerComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "drawer protocol should keep serde/schema contract marker `{needle}`."
        );
    }

    for forbidden in [
        "serde_json::",
        "from_json(",
        "to_json_result(",
        "SchemaError",
    ] {
        assert!(
            !protocol_source.contains(forbidden),
            "drawer protocol should avoid ad-hoc serde drift token `{forbidden}`."
        );
    }
}

#[test]
fn drawer_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let manifest_source = load_source("../../components/drawer/src/Component.toml");
    let rbi_source = load_source("../../components/drawer/src/drawer.rbi");
    let mod_source = load_source("../../components/drawer/src/mod.rs");
    let logic_source = load_source("../../components/drawer/src/logic.rs");
    let view_source = load_source("../../components/drawer/src/view.rs");
    let styles_source = load_source("../../components/drawer/src/styles.rs");
    let motion_source = load_source("../../components/drawer/src/motion.rs");
    let protocol_source = load_source("../../components/drawer/src/protocol.rs");
    let check2_source = load_source("../../components/drawer/check2.md");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Drawer\"",
        "crate = \"ui-drawer\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "drawer manifest should keep stable v1 schema marker `{needle}`."
        );
    }

    for needle in [
        "pub fn Drawer(",
        "is_open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "drawer RBI should keep stable public API marker `{needle}`."
        );
    }

    let combined = format!(
        "{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}\n{protocol_source}"
    );
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "contract.v2",
        "SchemaRegistry",
    ] {
        assert!(
            !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !combined.contains(forbidden),
            "drawer should not introduce major-version migration marker `{forbidden}` in current scope."
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `Drawer` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "drawer_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "scripts/check-ui-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer/check2.md should keep version-migration governance marker `{needle}`."
        );
    }
}

#[test]
fn drawer_version_deprecation_migration_script_covers_engineering_gate() {
    let script_source = load_source("../../scripts/check-ui-engineering.sh");

    let marker = "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_version_deprecation_migration_is_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`."
    );
}

#[test]
fn drawer_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("src/drawer/mod.rs"),
        load_source("src/drawer/logic.rs"),
        load_source("src/drawer/view.rs"),
        load_source("src/drawer/styles.rs"),
        load_source("src/drawer/motion.rs"),
        load_source("src/drawer/protocol.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "sheet-wasm-debug = [\"component-sheet\", \"dep:tracing\"]",
        "target: \"ui::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing marker `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("drawer-wasm-debug")
            && !cargo_source.contains("drawer_wasm_debug")
            && !cargo_source.contains("component-drawer-wasm-debug"),
        "drawer should not define component-local tracing feature aliases."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::drawer::",
        "const DRAWER_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "drawer should avoid tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn drawer_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    for source in [
        load_source("src/drawer/mod.rs"),
        load_source("src/drawer/logic.rs"),
        load_source("src/drawer/view.rs"),
        load_source("src/drawer/styles.rs"),
        load_source("src/drawer/motion.rs"),
        load_source("src/drawer/protocol.rs"),
    ] {
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
                "drawer engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    let mod_source = load_source("src/drawer/mod.rs");
    assert!(
        !mod_source.contains("web_sys"),
        "drawer public module boundary should not leak web_sys types."
    );
}

#[test]
fn drawer_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-engineering.sh");

    for needle in [
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn drawer_check2_marks_engineering_contract_complete() {
    let source = load_source("src/drawer/check2.md");

    assert!(
        source.contains("- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。"),
        "drawer check2 should mark engineering gate complete.",
    );

    for needle in [
        "components/drawer/src/protocol.rs",
        "DrawerComponentSchemaVersion",
        "DrawerComponentSpec",
        "use_controllable_open_state_traced(",
        "scripts/check-ui-engineering.sh",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "components/drawer/test/semantics.rs::drawer_engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
        "components/drawer/test/semantics.rs::drawer_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "components/drawer/test/semantics.rs::drawer_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "components/drawer/test/drawer/semantics.rs::drawer_engineering_contract_uses_serde_protocol_and_structured_schema_defaults",
        "components/drawer/test/drawer/semantics.rs::drawer_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "components/drawer/test/drawer/semantics.rs::drawer_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "drawer check2 engineering section should reference `{needle}`.",
        );
    }
}

#[test]
fn drawer_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let mod_source = load_source("../../components/drawer/src/mod.rs");
    let logic_source = load_source("../../components/drawer/src/logic.rs");
    let styles_source = load_source("../../components/drawer/src/styles.rs");
    let view_source = load_source("../../components/drawer/src/view.rs");
    let motion_source = load_source("../../components/drawer/src/motion.rs");
    let combined =
        format!("{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}");

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "drawer non-test source should forbid rust-hygiene anti-pattern `{forbidden}`.",
        );
    }
}

#[test]
fn drawer_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_source("../../components/drawer/src/logic.rs");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> =",
        "Cow::Borrowed(state.base_class)",
        "Cow::Borrowed(state.placement_class)",
        "Cow::Borrowed(\"ui-drawer--custom-class\")",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic_source.contains(required),
            "drawer logic should keep Cow-based string hotspot mitigation marker `{required}`.",
        );
    }

    for forbidden in [
        "\"ui-drawer--with-description\".to_string()",
        "\"ui-drawer--title-only\".to_string()",
        "\"ui-drawer--with-footer\".to_string()",
        "\"ui-drawer--no-footer\".to_string()",
        "\"ui-drawer--close-shown\".to_string()",
        "\"ui-drawer--close-hidden\".to_string()",
        "\"ui-drawer--custom-placement\".to_string()",
        "\"ui-drawer--custom-id\".to_string()",
        "\"ui-drawer--custom-title\".to_string()",
        "\"ui-drawer--custom-description\".to_string()",
        "\"ui-drawer--custom-footer\".to_string()",
        "\"ui-drawer--custom-close\".to_string()",
        "\"ui-drawer--custom-motion\".to_string()",
        "\"ui-drawer--custom-exit\".to_string()",
        "\"ui-drawer--custom-class\".to_string()",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "drawer logic should avoid string clone hotspot `{forbidden}`.",
        );
    }
}

#[test]
fn drawer_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let script_source = load_source("../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("../../scripts/check-ui-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            script_source.contains(required),
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }

    for needle in [
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui --test drawer_semantics --no-default-features --features component-drawer,inject-css drawer_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn drawer_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("src/drawer/check2.md");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "drawer_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "drawer_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "drawer_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "./scripts/check-rust-hygiene.sh",
        "Cow<'static, str>",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer check2 rust-hygiene section should reference `{needle}`.",
        );
    }
}

#[test]
fn drawer_merge_gate_verdicts_are_explicit_and_fully_completed() {
    let check2_source = load_source("src/drawer/check2.md");

    for needle in [
        "### 9. 合并门禁（最终裁决）",
        "- [x] 架构正确（边界不破）。",
        "- [x] 行为正确（状态与交互语义成立）。",
        "- [x] 可访问性达标（默认可用）。",
        "- [x] 默认主题美学质量达标（与可访问性同级门禁）。",
        "- [x] 可测试（契约可断言）。",
        "- [x] 可维护（命名和模式一致）。",
        "- [x] 可解释（人和自动化都能读懂）。",
        "- [x] 改动在正确层。",
        "- [x] 命名与全库一致。",
        "- [x] 无效状态被限制或归一化。",
        "- [x] 暴露必要语义标记。",
        "- [x] 覆盖 reduced-motion / SSR / wasm 分支。",
        "- [x] 文档与示例同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer merge-gate checklist should include `{needle}`."
        );
    }
}

#[test]
fn drawer_checklist_sections_one_to_nine_have_no_unchecked_items() {
    let check2_source = load_source("src/drawer/check2.md");

    for needle in [
        "### 1. 大骨架（架构边界与层职责）",
        "### 2. 小骨架（API 设计检查 + 状态管理检查）",
        "### 3. 实现细节（A11y / i18n-l10n / 可观测 / 样式与动效）",
        "### 4. SSR / 跨平台 / WASM / 性能 / 工程能力",
        "### 5. 文件落点检查（必须提及）",
        "### 6. AI 原生能力（Agent Contract + 流式）",
        "### 7. 测试与文档（验证闭环）",
        "### 8. 明确禁止的反模式",
        "### 9. 合并门禁（最终裁决）",
    ] {
        assert!(
            check2_source.contains(needle),
            "drawer checklist should include section `{needle}`."
        );
    }

    assert!(
        !check2_source.contains("- [ ]"),
        "drawer checklist must not keep unchecked items after completion."
    );
}
