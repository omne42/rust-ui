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
#[test]
fn share_button_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/button/share/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ShareButton internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn share_button_component_files_follow_layered_responsibilities() {
    let mod_source = load_source("src/button/share/mod.rs");
    let logic_source = load_source("src/button/share/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/share_button.rs");
    let styles_source = load_source("src/button/share/styles.rs");
    let view_source = load_source("src/button/share/view.rs");
    let motion_source = load_source("src/button/share/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::ShareButton;",
    ] {
        assert!(
            mod_source.contains(needle),
            "ShareButton module boundary should include `{needle}`."
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;"] {
        assert!(
            !mod_source.contains(forbidden),
            "ShareButton module should keep internals private; found `{forbidden}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::share_button::{",
        "pub fn resolve_icon_button_size(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ShareButton logic layer should include `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_state(input: ShareButtonStateInput)",
        "pub struct ShareButtonState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ShareButton state primitive layer should include `{needle}`."
        );
    }

    for forbidden in ["view! {", "<Button", "<FlipButton", "web_sys::"] {
        assert!(
            !logic_source.contains(forbidden),
            "ShareButton logic layer should not contain view or platform details `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str", "var(--ui-fg)"] {
        assert!(
            styles_source.contains(needle),
            "ShareButton styles layer should include `{needle}`."
        );
    }

    for forbidden in ["view! {", "Callback::new", "on_press"] {
        assert!(
            !styles_source.contains(forbidden),
            "ShareButton styles layer should stay static and avoid runtime logic `{forbidden}`."
        );
    }

    for needle in [
        "<FlipButton",
        "<ButtonGroup attached=true>",
        "<Button",
        "data-slot=\"share-button\"",
    ] {
        assert!(
            view_source.contains(needle),
            "ShareButton view layer should compose button capabilities via `{needle}`."
        );
    }

    for forbidden in [
        "<button",
        "pub fn resolve_state(",
        "pub struct ShareButtonState",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ShareButton view layer should avoid reimplementing lower-layer details `{forbidden}`."
        );
    }

    for needle in [
        "pub struct ShareButtonMotion",
        "pub fn sanitize_motion(motion: ShareButtonMotion) -> ShareButtonMotion",
        "flip: super::super::flip::motion::sanitize_motion(motion.flip)",
    ] {
        assert!(
            motion_source.contains(needle),
            "ShareButton motion layer should include `{needle}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "request_animation_frame",
        "KeyframeEffect",
        "view! {",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "ShareButton motion layer should stay mapping-only and avoid `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn share_button_directory_uses_standard_component_file_layout() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let share_dir = manifest_dir.join("src/button/share");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            share_dir.join(required).exists(),
            "ShareButton directory should contain standard component file `{required}`."
        );
    }

    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !share_dir.join(forbidden).exists(),
            "ShareButton directory should not drift to `{forbidden}`."
        );
    }

    let mod_source = load_source("src/button/share/mod.rs");
    let view_source = load_source("src/button/share/view.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
    ] {
        assert!(
            mod_source.contains(needle),
            "ShareButton module should keep standard layout export token `{needle}`."
        );
    }

    for needle in ["<FlipButton", "<ButtonGroup attached=true>", "<Button"] {
        assert!(
            view_source.contains(needle),
            "ShareButton view should reuse button composition capability via `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn share_button_spec_boundary_reuses_button_spec_without_local_spec_file() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let button_mod_source = load_source("src/button/mod.rs");
    let share_mod_source = load_source("src/button/share/mod.rs");

    assert!(
        manifest_dir.join("src/button/spec.rs").exists(),
        "Button should keep canonical spec.rs boundary for complex schema contract."
    );
    assert!(
        !manifest_dir.join("src/button/share/spec.rs").exists(),
        "ShareButton should not introduce a parallel spec.rs file."
    );

    for needle in [
        "pub mod spec;",
        "pub use spec::{ButtonA11y, ButtonAction, ButtonIntent, ButtonSchema, ButtonSpec, ButtonText};",
    ] {
        assert!(
            button_mod_source.contains(needle),
            "button module should keep canonical spec export `{needle}`."
        );
    }

    for forbidden in ["mod spec;", "pub mod spec;", "ButtonSpec", "ButtonSchema"] {
        assert!(
            !share_mod_source.contains(forbidden),
            "ShareButton module should stay lightweight and avoid local spec boundary token `{forbidden}`."
        );
    }
}

#[test]
fn share_button_uses_logic_state_model() {
    let view_source = load_source("src/button/share/view.rs");
    let logic_source = load_source("src/button/share/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/share_button.rs");

    for needle in [
        "pub use ui_state_primitives::share_button::{",
        "pub fn compose_class_name(",
        "pub fn resolve_icon_button_size(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ShareButton logic should include `{needle}` as the component assembly layer."
        );
    }

    for needle in [
        "pub struct ShareButtonStateInput",
        "pub struct ShareButtonState",
        "pub struct ResolvedShareItems",
        "pub use crate::button::normalize_optional_text;",
        "pub fn resolve_label(",
        "pub fn resolve_items(",
        "pub fn resolve_state(input: ShareButtonStateInput)",
        "pub items_source_attr: &'static str",
        "pub handler_source_attr: &'static str",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ShareButton state primitive should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let normalized_label = logic::normalize_optional_text(label);",
        "let class_name = logic::normalize_optional_text(class_name);",
        "let provided_item_count = items.len();",
        "let resolved_items = logic::resolve_items_with_fallback(",
        "let state = logic::resolve_state(ShareButtonStateInput {",
        "has_custom_label: normalized_label.is_some(),",
        "let label =",
        "logic::resolve_label_with_fallback(normalized_label, common.share_button_label.as_ref())",
        "let class = logic::compose_class_name(class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "ShareButton view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn share_button_state_normalization_is_centralized_in_logic_layer() {
    let view_source = load_source("src/button/share/view.rs");
    let logic_source = load_source("src/button/share/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/share_button.rs");
    let styles_source = load_source("src/button/share/styles.rs");

    for needle in [
        "pub use ui_state_primitives::share_button::{",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ShareButton state normalization contract should stay in logic layer via `{needle}`."
        );
    }

    for needle in [
        "pub use crate::button::normalize_optional_text;",
        "pub fn resolve_label_with_fallback(",
        "pub fn resolve_items_with_fallback(",
        "pub fn resolve_state(input: ShareButtonStateInput)",
        "(\"ready\", \"ui-share-button--state-ready\")",
        "(\"empty\", \"ui-share-button--state-empty\")",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ShareButton state normalization primitive should include `{needle}`."
        );
    }

    for needle in [
        "let normalized_label = logic::normalize_optional_text(label);",
        "let class_name = logic::normalize_optional_text(class_name);",
        "let resolved_items = logic::resolve_items_with_fallback(",
        "let state = logic::resolve_state(ShareButtonStateInput {",
        "has_custom_label: normalized_label.is_some(),",
        "logic::resolve_label_with_fallback(normalized_label, common.share_button_label.as_ref())",
        "let class = logic::compose_class_name(class_name, state);",
        "<FlipButton",
        "<ButtonGroup attached=true>",
        "<Button",
    ] {
        assert!(
            view_source.contains(needle),
            "ShareButton view should consume centralized logic output and reuse button composition `{needle}`."
        );
    }

    for forbidden in [
        "let (state_attr, state_class)",
        "ui-share-button--state-ready",
        "ui-share-button--state-empty",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ShareButton view should not reconstruct state-machine detail `{forbidden}`."
        );
    }

    for selector in [
        ".ui-share-button[data-state=\"ready\"]",
        ".ui-share-button[data-state=\"empty\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "ShareButton styles should only consume semantic state marker `{selector}`."
        );
    }

    assert!(
        !styles_source.contains("resolve_state("),
        "Styles layer must not own normalization logic."
    );
}

#[test]
fn share_button_default_values_have_single_logic_source() {
    let view_source = load_source("src/button/share/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/share_button.rs");
    let common_i18n_source = load_source("../ui-headless/src/i18n/common.rs");

    for needle in [
        "pub fn resolve_label_with_fallback(label: Option<String>, fallback_label: &str) -> String",
        "pub fn resolve_label(label: Option<String>) -> String",
        "share_button_label: \"Share\".into(),",
        "let normalized_label = logic::normalize_optional_text(label);",
        "has_custom_label: normalized_label.is_some(),",
        "logic::resolve_label_with_fallback(normalized_label, common.share_button_label.as_ref())",
    ] {
        assert!(
            view_source.contains(needle)
                || primitive_source.contains(needle)
                || common_i18n_source.contains(needle),
            "ShareButton default label contract should include `{needle}`."
        );
    }

    for forbidden in [
        "let label = label.unwrap_or_else(|| \"Share\".to_string());",
        "let label = label.unwrap_or(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ShareButton view should not contain secondary default fallback `{forbidden}`."
        );
    }
}

#[test]
fn share_button_uses_flip_button_and_button_group_composition() {
    let source = load_source("src/button/share/view.rs");

    for needle in [
        "<FlipButton",
        "from=from",
        "motion=motion.flip",
        "front=move ||",
        "back=move ||",
        "start_content=move || view!",
        "end_content=move || view!",
        "<ButtonGroup attached=true>",
        "data-slot=\"share-button-front\"",
        "data-slot=\"share-button-back\"",
    ] {
        assert!(
            source.contains(needle),
            "ShareButton should compose flip/share surfaces via `{needle}`."
        );
    }
}

#[test]
fn share_button_emits_baseline_style_data_attributes() {
    let source = load_source("src/button/share/view.rs");

    for needle in [
        "data-slot=\"share-button\"",
        "data-state=state.state_attr",
        "data-provided-count=state.provided_item_count.to_string()",
        "data-count=state.resolved_item_count.to_string()",
        "data-has-items=state.has_items.then_some(\"true\")",
        "data-items-source=state.items_source_attr",
        "data-icon=state.icon_placement_attr",
        "data-label-source=state.label_source_attr",
        "data-handler-source=state.handler_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-motion-source=if motion == ShareButtonMotion::default()",
        "data-custom-motion=(motion != ShareButtonMotion::default()).then_some(\"true\")",
        "data-slot=\"share-button-platform\"",
        "data-platform=platform_attr",
    ] {
        assert!(
            source.contains(needle),
            "ShareButton should expose `{needle}` for baseline-style state inspection."
        );
    }
}

#[test]
fn share_button_state_markers_are_observable_and_closed_set_contracts() {
    let view_source = load_source("src/button/share/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/share_button.rs");

    for needle in [
        "data-state=state.state_attr",
        "data-items-source=state.items_source_attr",
        "data-icon=state.icon_placement_attr",
        "data-label-source=state.label_source_attr",
        "data-handler-source=state.handler_source_attr",
        "data-platform=platform_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "ShareButton should expose observable semantic marker `{needle}`."
        );
    }

    for closed_set_case in [
        "(\"ready\", \"ui-share-button--state-ready\")",
        "(\"empty\", \"ui-share-button--state-empty\")",
        "(\"default\", \"ui-share-button--default-items\")",
        "(\"custom\", \"ui-share-button--custom-items\")",
        "(\"custom\", \"ui-share-button--custom-label\")",
        "(\"default\", \"ui-share-button--default-label\")",
        "(\"provided\", \"ui-share-button--with-handler\")",
        "(\"none\", \"ui-share-button--without-handler\")",
        "ShareButtonIconPlacement::Prefix => \"prefix\"",
        "ShareButtonIconPlacement::Suffix => \"suffix\"",
        "ShareButtonIconPlacement::None => \"none\"",
        "SharePlatform::Github => \"github\"",
        "SharePlatform::X => \"x\"",
        "SharePlatform::Facebook => \"facebook\"",
    ] {
        assert!(
            primitive_source.contains(closed_set_case),
            "ShareButton marker values should be enumerable closed sets; missing `{closed_set_case}`."
        );
    }

    assert!(
        !primitive_source.contains("format!(\"{"),
        "ShareButton marker contracts should avoid free-form dynamic string formatting."
    );
}

#[test]
fn share_button_discrete_state_inputs_use_enums_and_reuse_button_types() {
    let view_source = load_source("src/button/share/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/share_button.rs");

    for needle in [
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum SharePlatform",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]\npub enum ShareButtonIconPlacement",
        "pub platform: SharePlatform,",
        "pub icon_placement: ShareButtonIconPlacement,",
        "ShareButtonIconPlacement::Prefix => \"prefix\"",
        "ShareButtonIconPlacement::Suffix => \"suffix\"",
        "ShareButtonIconPlacement::None => \"none\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ShareButton discrete state contract should include typed enum modeling `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] icon: ShareButtonIconPlacement,",
        "#[prop(optional)] size: ButtonSize,",
        "#[prop(optional)] variant: ButtonVariant,",
        "#[prop(optional)] on_icon_press: Option<Callback<SharePlatform>>,",
    ] {
        assert!(
            view_source.contains(needle),
            "ShareButton props should keep typed discrete inputs and reuse button enums via `{needle}`."
        );
    }

    for forbidden in [
        "icon: Option<bool>",
        "size: Option<bool>",
        "variant: Option<bool>",
        "icon: String",
        "size: String",
        "variant: String",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ShareButton should avoid bool/string-based discrete input modeling `{forbidden}`."
        );
    }
}

#[test]
fn share_button_has_no_async_loading_protocol_and_keeps_sync_press_contract() {
    let view_source = load_source("src/button/share/view.rs");
    let logic_source = load_source("src/button/share/logic.rs");

    for needle in [
        "#[prop(optional)] on_icon_press: Option<Callback<SharePlatform>>",
        "let on_press = Callback::new(move |_| {",
        "if let Some(cb) = on_icon_press {",
        "cb.run(platform);",
    ] {
        assert!(
            view_source.contains(needle),
            "ShareButton should keep synchronous press contract via `{needle}`."
        );
    }

    for forbidden in [
        "use_async_action",
        "is_loading",
        "aria-busy",
        "retry",
        "async fn",
        ".await",
        "Future<",
        "spawn_local",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "ShareButton has no async workflow; forbidden async/loading token `{forbidden}` should be absent."
        );
    }
}

#[test]
fn share_button_docs_expose_hello_world_path_without_state_machine_wiring() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "let hello_code = Signal::derive(move || r#\"<ShareButton />\"#.to_string());",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<ShareButton />",
    ] {
        assert!(
            source.contains(needle),
            "ShareButton docs should keep DX minimal path `{needle}`."
        );
    }

    for forbidden in ["<ShareButton state=", "ui_state_primitives", "ui-headless"] {
        assert!(
            !source.contains(forbidden),
            "ShareButton docs minimal usage should not require internal wiring token `{forbidden}`."
        );
    }
}

#[test]
fn share_button_public_api_follows_naming_contract_for_callbacks() {
    let view_source = load_source("src/button/share/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    assert!(
        view_source.contains("#[prop(optional)] on_icon_press: Option<Callback<SharePlatform>>,"),
        "ShareButton callback prop should use `on_*` naming contract."
    );

    for needle in [
        "<ShareButton on_icon_press=on_icon_press />",
        "on_icon_press=on_icon_press",
        "on_icon_press=Some(on_icon_press)",
    ] {
        assert!(
            docs_source.contains(needle),
            "ShareButton docs should use canonical callback name `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] on_press:",
        "#[prop(optional)] onclick:",
        "#[prop(optional)] on_click:",
        "#[prop(optional)] handle_press:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ShareButton public API should avoid callback alias drift `{forbidden}`."
        );
    }
}

#[test]
fn share_button_machine_readable_contract_uses_typed_inputs_and_semantic_markers() {
    let view_source = load_source("src/button/share/view.rs");
    let logic_source = load_source("src/button/share/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/share_button.rs");

    for needle in [
        "pub enum SharePlatform",
        "pub enum ShareButtonIconPlacement",
        "#[prop(optional)] size: ButtonSize,",
        "#[prop(optional)] variant: ButtonVariant,",
        "#[prop(optional)] on_icon_press: Option<Callback<SharePlatform>>,",
    ] {
        assert!(
            primitive_source.contains(needle)
                || logic_source.contains(needle)
                || view_source.contains(needle),
            "ShareButton machine-readable contract should keep typed input space `{needle}`."
        );
    }

    for needle in [
        "pub state_attr: &'static str,",
        "pub items_source_attr: &'static str,",
        "pub icon_placement_attr: &'static str,",
        "pub label_source_attr: &'static str,",
        "pub handler_source_attr: &'static str,",
        "data-state=state.state_attr",
        "data-items-source=state.items_source_attr",
        "data-icon=state.icon_placement_attr",
        "data-label-source=state.label_source_attr",
        "data-handler-source=state.handler_source_attr",
    ] {
        assert!(
            primitive_source.contains(needle)
                || logic_source.contains(needle)
                || view_source.contains(needle),
            "ShareButton machine-readable contract should expose semantic marker `{needle}`."
        );
    }

    for forbidden in [
        "data-state=\"",
        "data-items-source=\"",
        "data-label-source=\"",
        "data-handler-source=\"",
        "format!(\"data-state",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ShareButton state markers should come from typed derived state instead of ad-hoc literals `{forbidden}`."
        );
    }
}

#[test]
fn share_button_maps_icon_button_size_and_platform_icons() {
    let source = load_source("src/button/share/view.rs");

    for needle in [
        "let icon_button_size = logic::resolve_icon_button_size(size);",
        "size=icon_button_size",
        "is_icon_only=true",
        "SharePlatform::Github",
        "SharePlatform::X",
        "SharePlatform::Facebook",
    ] {
        assert!(
            source.contains(needle),
            "ShareButton should include `{needle}` for icon-button behavior and platform coverage."
        );
    }
}

#[test]
fn share_button_preserves_optional_press_handler_without_markup_branching() {
    let source = load_source("src/button/share/view.rs");

    for needle in [
        "let on_icon_press = StoredValue::new(on_icon_press);",
        "if let Some(cb) = on_icon_press {",
        "cb.run(platform);",
    ] {
        assert!(
            source.contains(needle),
            "ShareButton should wire optional callbacks via `{needle}`."
        );
    }

    assert!(
        !source.contains("match on_icon_press"),
        "ShareButton should avoid duplicating markup based on handler presence."
    );
}

#[test]
fn share_button_styles_include_state_marker_contracts() {
    let source = load_source("src/button/share/styles.rs");

    for selector in [
        ".ui-share-button--state-ready",
        ".ui-share-button[data-state=\"ready\"]",
        ".ui-share-button--icon-prefix",
        ".ui-share-button[data-icon=\"none\"] [data-slot=\"share-button-trigger-icon\"]",
        ".ui-share-button__platform[data-platform=\"github\"] .ui-button",
        ".ui-share-button--custom-class",
        ".ui-share-button[data-custom-class=\"true\"]",
        ".ui-share-button[data-motion-source=\"custom\"]",
        ".ui-share-button[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "ShareButton styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn share_button_token_first_styles_are_static_and_aggregated_via_ui_root() {
    let styles_source = load_source("src/button/share/styles.rs");
    let view_source = load_source("src/button/share/view.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-fg)",
        ".ui-share-button[data-state=\"ready\"]",
        ".ui-share-button[data-state=\"empty\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "ShareButton styles should remain token-first/static and include `{needle}`.",
        );
    }

    for forbidden in [
        "view! {",
        "Callback::new",
        "format!(",
        "@apply",
        "styled(",
        "css!(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "ShareButton styles should avoid runtime or utility/CSS-in-Rust default patterns `{forbidden}`.",
        );
    }

    assert!(
        !view_source.contains(" style="),
        "ShareButton view should avoid inline business style logic and rely on static styles.rs contracts.",
    );

    for needle in [
        "#[cfg(feature = \"component-button_share\")]",
        "out.push_str(crate::button::share::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "ShareButton styles must be aggregated via css.rs feature-gated registry token `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional)] inject_components_css: bool",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should remain the centralized component-css injection path via `{needle}`.",
        );
    }
}

#[test]
fn share_button_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let view_source = load_source("src/button/share/view.rs");
    let styles_source = load_source("src/button/share/styles.rs");

    for needle in [
        "data-state=state.state_attr",
        "data-icon=state.icon_placement_attr",
        "data-motion-source=if motion == ShareButtonMotion::default()",
        "data-custom-motion=(motion != ShareButtonMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "ShareButton view should expose explicit style state marker `{needle}`."
        );
    }

    for selector in [
        ".ui-share-button[data-state=\"ready\"]",
        ".ui-share-button[data-state=\"empty\"]",
        ".ui-share-button[data-icon=\"prefix\"] [data-slot=\"share-button-trigger-icon\"]",
        ".ui-share-button[data-icon=\"none\"] [data-slot=\"share-button-trigger-icon\"]",
        ".ui-share-button[data-motion-source=\"custom\"]",
        ".ui-share-button[data-custom-motion=\"true\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "ShareButton styles should consume explicit marker selector `{selector}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":first-child", ":last-child"] {
        assert!(
            !styles_source.contains(forbidden),
            "ShareButton styles should not use brittle structural selector `{forbidden}` to infer state."
        );
    }

    assert!(
        !view_source.contains(" style="),
        "ShareButton view should avoid inline style branches for business state."
    );
}

#[test]
fn share_button_semantics_suite_prioritizes_contract_assertions_over_snapshots() {
    let suite_source = load_source("tests/share_button_semantics.rs");

    for semantic_signal in [
        "share_button_emits_baseline_style_data_attributes",
        "share_button_state_markers_are_observable_and_closed_set_contracts",
        "share_button_styles_depend_on_explicit_state_markers_not_dom_guessing",
        "data-state=state.state_attr",
        "data-items-source=state.items_source_attr",
        "data-handler-source=state.handler_source_attr",
    ] {
        assert!(
            suite_source.contains(semantic_signal),
            "ShareButton semantic suite should keep contract assertion signal `{semantic_signal}`."
        );
    }

    let forbidden_assert_snapshot = ["assert", "_", "snapshot", "!"].concat();
    let forbidden_insta = ["in", "sta", "::"].concat();
    let forbidden_match_snapshot = ["to", "_", "match", "_", "snapshot"].concat();

    for forbidden in [
        forbidden_assert_snapshot,
        forbidden_insta,
        forbidden_match_snapshot,
    ] {
        assert!(
            !suite_source.contains(&forbidden),
            "ShareButton semantic suite should not rely on visual snapshot token `{forbidden}`."
        );
    }
}

#[test]
fn share_button_semantics_cover_data_aria_and_button_role_contracts() {
    let suite_source = load_source("tests/share_button_semantics.rs");
    let view_source = load_source("src/button/share/view.rs");

    for semantic_test in [
        "share_button_state_markers_are_observable_and_closed_set_contracts",
        "share_button_styles_depend_on_explicit_state_markers_not_dom_guessing",
        "share_button_semantics_suite_prioritizes_contract_assertions_over_snapshots",
    ] {
        assert!(
            suite_source.contains(semantic_test),
            "ShareButton semantic suite should include `{semantic_test}`."
        );
    }

    for semantic_marker in [
        "data-state=state.state_attr",
        "data-items-source=state.items_source_attr",
        "data-handler-source=state.handler_source_attr",
        "aria-hidden=\"true\"",
        "aria_label=aria_label",
        "<Button",
    ] {
        assert!(
            view_source.contains(semantic_marker),
            "ShareButton semantic contract should expose `{semantic_marker}`."
        );
    }

    assert!(
        !view_source.contains("<button"),
        "ShareButton should reuse Button role/a11y contract instead of custom raw button markup."
    );
}

#[test]
fn share_button_has_a11y_i18n_and_locale_entrypoints_via_headless_contracts() {
    let view_source = load_source("src/button/share/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/share_button.rs");
    let check_source = load_source("src/button/share/check2.md");

    for needle in [
        "use ui_headless::{A11yDirection, CommonStrings, labeled_group_attrs, use_ui_i18n};",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "let i18n = use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "let group_a11y = labeled_group_attrs(group_aria_label, lang, dir);",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label.clone()",
        "lang=group_a11y.lang.clone()",
        "dir=group_a11y.dir",
        "logic::resolve_label_with_fallback(normalized_label, common.share_button_label.as_ref())",
        "logic::resolve_items_with_fallback(",
        "data-group-label-source=if normalized_group_aria_label.is_some() {",
    ] {
        assert!(
            view_source.contains(needle),
            "ShareButton should expose A11y/i18n/locale contract token `{needle}`.",
        );
    }

    for needle in [
        "pub fn resolve_label_with_fallback(label: Option<String>, fallback_label: &str) -> String",
        "pub fn resolve_items_with_fallback(",
        "pub struct SharePlatformLabels<'a> {",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ShareButton primitive should include i18n-capable fallback token `{needle}`.",
        );
    }

    for forbidden in [
        "\"Share\".to_string()",
        "\"GitHub\"",
        "\"Facebook\"",
        "\"X\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ShareButton view should not hardcode user-visible strings; found `{forbidden}`.",
        );
    }

    assert!(
        check_source.contains("存在 A11y 实现、国际化与本地化实现"),
        "ShareButton checklist should keep this A11y/i18n item explicit.",
    );
}

#[test]
fn share_button_snapshot_mode_is_default_and_accepts_complete_configuration() {
    let view_source = load_source("src/button/share/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "#[component]\npub fn ShareButton(",
        "#[prop(optional, into)] label: Option<String>,",
        "#[prop(optional)] icon: ShareButtonIconPlacement,",
        "#[prop(optional)] from: FlipDirection,",
        "#[prop(optional)] size: ButtonSize,",
        "#[prop(optional)] variant: ButtonVariant,",
        "#[prop(optional)] items: Vec<ShareButtonItem>,",
        "#[prop(optional)] on_icon_press: Option<Callback<SharePlatform>>,",
        "#[prop(optional)] motion: ShareButtonMotion,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
    ] {
        assert!(
            view_source.contains(needle),
            "ShareButton should support snapshot-style complete configuration via `{needle}`."
        );
    }

    for needle in [
        "<ShareButton />",
        "<ShareButton on_icon_press=on_icon_press />",
        "icon=ShareButtonIconPlacement::Prefix",
        "from=FlipDirection::Right",
        "items=custom_items_for_custom.clone()",
    ] {
        assert!(
            docs_source.contains(needle),
            "ShareButton docs should demonstrate complete snapshot configuration token `{needle}`."
        );
    }

    for forbidden in [
        "data-ui-stream",
        "streaming",
        "delta",
        "partial_chunk",
        "fallback=snapshot",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ShareButton runtime should not depend on streaming-only token `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn share_button_motion_contract_exposes_default_and_custom_checks() {
    let source = load_source("src/button/share/motion.rs");

    for needle in [
        "pub struct ShareButtonMotion",
        "fn default_motion_matches_flip_button_defaults()",
        "fn supports_custom_flip_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "ShareButton motion module should include `{needle}` for baseline-level motion contract coverage."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn share_button_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/button/share/motion.rs");
    let view_source = load_source("src/button/share/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ShareButtonMotion) -> ShareButtonMotion",
        "flip: super::super::flip::motion::sanitize_motion(motion.flip)",
        "fn sanitize_motion_delegates_to_flip_button_contract()",
    ] {
        assert!(
            motion_source.contains(needle),
            "ShareButton motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = super::motion::sanitize_motion(motion);"),
        "ShareButton view should sanitize motion before forwarding to FlipButton.",
    );
}

#[test]
fn share_button_motion_layer_reuses_flip_and_ui_motion_without_engine_reimplementation() {
    let motion_source = load_source("src/button/share/motion.rs");

    for needle in [
        "pub struct ShareButtonMotion",
        "pub flip: FlipButtonMotion,",
        "flip: super::super::flip::motion::sanitize_motion(motion.flip)",
    ] {
        assert!(
            motion_source.contains(needle),
            "ShareButton motion layer should stay mapping-only and reuse flip contract via `{needle}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "request_animation_frame",
        "KeyframeEffect",
        "SpringSolver",
        "unsafe",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "ShareButton motion layer should not reimplement motion engine internals `{forbidden}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn share_button_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    for needle in [
        "pub(super) fn share_button() -> AnyView",
        "title=\"ShareButton\"",
        "slug=\"share-button\"",
        "description=\"Flip-based share surface with centralized item/icon/handler state attrs and baseline-level spring motion.\"",
        "<Playground title=\"Default + callback\" code_signal=code>",
        "<Playground title=\"Icon placement + custom items\" code_signal=states_code>",
        "<Playground title=\"Custom Class + Direction\" code_signal=custom_code>",
        "<ShareButton",
    ] {
        assert!(
            source.contains(needle),
            "actions docs should include `{needle}` for share-button primary playground coverage.",
        );
    }

    assert!(
        mod_source.contains("\"button-share\" => &[\"share-button\"]"),
        "components mod mapping should keep `button-share` mapped to `share-button` slug.",
    );
}

#[test]
fn share_button_docs_are_beginner_friendly_with_progressive_examples() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let start = source
        .find("pub(super) fn share_button() -> AnyView {")
        .expect("missing share_button docs entry start");
    let end = source
        .find("pub(super) fn action_menu() -> AnyView {")
        .expect("missing action_menu docs entry after share_button");
    let share_section = &source[start..end];

    for needle in [
        "pub(super) fn share_button() -> AnyView",
        "title=\"ShareButton\"",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<ShareButton />",
        "<Playground title=\"Default + callback\" code_signal=code>",
        "<Playground title=\"Icon placement + custom items\" code_signal=states_code>",
        "<Playground title=\"Custom Class + Direction\" code_signal=custom_code>",
    ] {
        assert!(
            share_section.contains(needle),
            "ShareButton docs should expose beginner-friendly progression token `{needle}`."
        );
    }

    let hello_pos = share_section
        .find("<Playground title=\"Hello World\" code_signal=hello_code>")
        .expect("missing share-button hello world playground");
    let default_pos = share_section
        .find("<Playground title=\"Default + callback\" code_signal=code>")
        .expect("missing share-button default playground");
    let advanced_pos = share_section
        .find("<Playground title=\"Custom Class + Direction\" code_signal=custom_code>")
        .expect("missing share-button advanced playground");

    assert!(
        hello_pos < default_pos && default_pos < advanced_pos,
        "ShareButton docs should present examples from simple to advanced."
    );
}

#[test]
fn share_button_composite_api_uses_typed_items_not_parallel_arrays() {
    let view_source = load_source("src/button/share/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/share_button.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "#[prop(optional)] items: Vec<ShareButtonItem>,",
        "pub struct ShareButtonItem {",
        "pub platform: SharePlatform,",
        "pub label: String,",
        "pub fn resolve_items(items: &[ShareButtonItem]) -> ResolvedShareItems",
    ] {
        assert!(
            view_source.contains(needle) || primitive_source.contains(needle),
            "ShareButton composite API should keep typed item contract `{needle}`."
        );
    }

    for needle in [
        "ShareButtonItem::new(SharePlatform::Github, \"Repository\")",
        "ShareButtonItem::new(SharePlatform::X, \"Post\")",
        "ShareButtonItem::new(SharePlatform::Facebook, \"   \")",
        "items=custom_items_for_matrix.clone()",
    ] {
        assert!(
            docs_source.contains(needle),
            "ShareButton docs should demonstrate typed item usage `{needle}`."
        );
    }

    for forbidden in ["labels=", "children=", "titles=", "panels="] {
        assert!(
            !view_source.contains(forbidden) && !docs_source.contains(forbidden),
            "ShareButton should avoid parallel-array composite API token `{forbidden}`."
        );
    }
}

#[test]
fn share_button_docs_sync_param_and_state_matrix_with_current_logic_contract() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");
    let primitive_source = load_source("../ui-state-primitives/src/share_button.rs");
    let start = docs_source
        .find("pub(super) fn share_button() -> AnyView {")
        .expect("missing share_button docs entry start");
    let end = docs_source
        .find("pub(super) fn action_menu() -> AnyView {")
        .expect("missing action_menu docs entry after share_button");
    let share_section = &docs_source[start..end];

    for needle in [
        "title=\"Hello World\"",
        "<ShareButton />",
        "title=\"Icon placement + custom items\"",
        "icon=ShareButtonIconPlacement::Prefix",
        "icon=ShareButtonIconPlacement::None",
        "from=FlipDirection::Left",
        "items=custom_items_for_matrix.clone()",
        "title=\"Custom Class + Direction\"",
        "class_name=\"docs-share-button-custom\".to_string()",
        "from=FlipDirection::Right",
        "items=custom_items_for_custom.clone()",
        "on_icon_press=on_icon_press",
    ] {
        assert!(
            share_section.contains(needle),
            "ShareButton docs matrix should contain `{needle}`."
        );
    }

    for needle in [
        "pub fn resolve_label(label: Option<String>) -> String",
        "label.unwrap_or_else(|| \"Share\".to_string())",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ShareButton logic default contract should include `{needle}`."
        );
    }

    for forbidden in ["<ShareButton on_press=", "default_label=", "is_loading="] {
        assert!(
            !share_section.contains(forbidden),
            "ShareButton docs should avoid stale/incorrect API token `{forbidden}`."
        );
    }
}

#[test]
fn share_button_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "title=\"Default + callback\"",
        "let (last, set_last) = signal(None::<SharePlatform>);",
        "let on_icon_press = Callback::new(move |platform: SharePlatform| set_last.set(Some(platform)));",
        "<ShareButton on_icon_press=on_icon_press />",
        "title=\"Icon placement + custom items\"",
        "icon=ShareButtonIconPlacement::Prefix",
        "from=FlipDirection::Left",
        "label=\"Share now\".to_string()",
        "items=custom_items_for_matrix.clone()",
        "icon=ShareButtonIconPlacement::None",
        "label=\"Iconless\".to_string()",
        "Blank custom item labels fall back to platform defaults; missing handlers stay safe.",
        "title=\"Custom Class + Direction\"",
        "class_name=\"docs-share-button-custom\".to_string()",
        "from=FlipDirection::Right",
        "label=\"Share docs\".to_string()",
        "items=custom_items_for_custom.clone()",
        "label=\"Share defaults\".to_string()",
        "icon=ShareButtonIconPlacement::Suffix",
    ] {
        assert!(
            source.contains(needle),
            "actions docs playgrounds should contain `{needle}` for share-button contracts.",
        );
    }
}

#[test]
fn share_button_visual_desire_gate_reuses_theme_visual_baseline_and_heroui_contracts() {
    let baseline_page =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let e2e_spec = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_doc = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            baseline_page.contains(needle),
            "Theme visual baseline page should keep visual-quality contract token `{needle}`.",
        );
    }

    for needle in ["\"ThemeVisualBaseline\"", "\"theme-visual-baseline\""] {
        assert!(
            pages_registry.contains(needle),
            "Docs pages registry should expose theme visual baseline route token `{needle}`.",
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
    ] {
        assert!(
            e2e_spec.contains(needle),
            "Theme visual baseline e2e contract should include `{needle}`.",
        );
    }

    for needle in ["# HeroUI 参数设计风格对齐策略", "HeroUI 对齐结论"] {
        assert!(
            heroui_doc.contains(needle),
            "HeroUI strategy doc should keep alignment contract token `{needle}`.",
        );
    }
}

#[test]
fn share_button_tree_shaking_contract_enforces_component_feature_gates_and_budgeted_ci() {
    let cargo_source = load_source("Cargo.toml");
    let button_mod_source = load_source("src/button/mod.rs");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let tree_shaking_script = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "component-button_share = [",
        "\"component-button\"",
        "\"component-button_flip\"",
        "\"component-button_group\"",
        "web-demo-components = [",
        "all-components = [",
        "\"component-button_share\"",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components feature graph should keep ShareButton tree-shaking token `{needle}`.",
        );
    }

    assert!(
        button_mod_source.contains("#[cfg(feature = \"component-button_share\")]\npub mod share;"),
        "ShareButton module must stay feature-gated in button/mod.rs."
    );

    for needle in [
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]",
        "#[cfg(feature = \"all-components\")]",
        "pub use web_demo_components::*;",
        "pub use all_components::*;",
    ] {
        assert!(
            lib_source.contains(needle),
            "lib.rs should keep feature-bounded export surface token `{needle}`.",
        );
    }

    let share_reexport_count = lib_source.matches("pub use button::share::{").count();
    assert_eq!(
        share_reexport_count, 2,
        "ShareButton re-export should only exist inside gated feature bundles."
    );

    assert!(
        css_source.contains(
            "#[cfg(feature = \"component-button_share\")]\n    out.push_str(crate::button::share::styles::CSS);"
        ),
        "css.rs should aggregate ShareButton CSS behind component-button_share gate."
    );

    assert!(
        !css_source.contains(
            "#[cfg(feature = \"all-components\")]\n    out.push_str(crate::button::share::styles::CSS);"
        ),
        "ShareButton CSS should not be tied to all-components-only aggregation."
    );

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "cargo tree -e features -i ui-components -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_shaking_script.contains(needle) || budget_source.contains(needle),
            "Tree-shaking CI gate should include `{needle}`.",
        );
    }
}

#[test]
fn share_button_ssr_and_cross_platform_compile_paths_are_cfg_guarded() {
    let check_source = load_source("src/button/share/check2.md");
    let ui_components_cargo_source = load_source("Cargo.toml");
    let ui_components_lib_source = load_source("src/lib.rs");
    let ui_headless_cargo_source = load_source("../ui-headless/Cargo.toml");
    let ui_headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let ui_motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let share_view_source = load_source("src/button/share/view.rs");
    let share_logic_source = load_source("src/button/share/logic.rs");
    let share_motion_source = load_source("src/button/share/motion.rs");
    let share_styles_source = load_source("src/button/share/styles.rs");

    assert!(
        check_source.contains("SSR 与跨平台检查"),
        "ShareButton checklist should keep SSR/cross-platform contract item explicit.",
    );

    let wasm_target_deps_header = "[target.'cfg(target_arch = \"wasm32\")'.dependencies]";
    let wasm_target_deps_index = ui_components_cargo_source
        .find(wasm_target_deps_header)
        .expect("ui-components Cargo.toml should keep wasm32 target dependency section");
    let non_wasm_deps_section = &ui_components_cargo_source[..wasm_target_deps_index];

    assert!(
        non_wasm_deps_section.contains("[dependencies]"),
        "ui-components should keep a native dependency section before wasm32 target dependencies."
    );
    assert!(
        !non_wasm_deps_section.contains("web-sys ="),
        "ui-components should not pull web-sys in non-wasm dependency path."
    );

    for needle in [
        "[target.'cfg(target_arch = \"wasm32\")'.dependencies]",
        "js-sys = \"0.3.85\"",
        "web-sys = { version = \"0.3.85\"",
    ] {
        assert!(
            ui_components_cargo_source.contains(needle),
            "ui-components Cargo.toml should keep wasm-specific dependency token `{needle}`.",
        );
    }

    assert!(
        ui_components_lib_source.contains("#[cfg(target_arch = \"wasm32\")]"),
        "ui-components lib should keep explicit wasm cfg boundary."
    );

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
        "[target.'cfg(target_arch = \"wasm32\")'.dependencies]",
        "web-sys = { version = \"0.3.85\"",
    ] {
        assert!(
            ui_headless_cargo_source.contains(needle),
            "ui-headless feature/cfg contract should include `{needle}`.",
        );
    }

    assert!(
        ui_headless_lib_source
            .contains("#[cfg(all(feature = \"web\", feature = \"ssr\"))]\ncompile_error!("),
        "ui-headless should enforce web/ssr mutual exclusion via compile_error! guard."
    );

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]\npub mod web;",
        "#[cfg(not(target_arch = \"wasm32\"))]\npub mod web {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion should keep explicit wasm/non-wasm backend split token `{needle}`.",
        );
    }

    for forbidden in [
        "web_sys::",
        "js_sys::",
        "wasm_bindgen",
        "window()",
        "document()",
    ] {
        assert!(
            !share_view_source.contains(forbidden)
                && !share_logic_source.contains(forbidden)
                && !share_motion_source.contains(forbidden)
                && !share_styles_source.contains(forbidden),
            "ShareButton component files should keep non-wasm path clean from browser-only token `{forbidden}`.",
        );
    }
}

#[test]
fn share_button_headless_web_ssr_feature_mutex_is_compile_guarded_and_script_verified() {
    let check_source = load_source("src/button/share/check2.md");
    let share_view_source = load_source("src/button/share/view.rs");
    let ui_headless_cargo_source = load_source("../ui-headless/Cargo.toml");
    let ui_headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    assert!(
        check_source.contains("`ui-headless` web/ssr feature 互斥受 `compile_error!` 保护"),
        "ShareButton checklist should keep explicit ui-headless web/ssr mutex item.",
    );

    for needle in [
        "use ui_headless::{A11yDirection, CommonStrings, labeled_group_attrs, use_ui_i18n};",
        "let group_a11y = labeled_group_attrs(group_aria_label, lang, dir);",
    ] {
        assert!(
            share_view_source.contains(needle),
            "ShareButton should keep headless integration token `{needle}`.",
        );
    }

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            ui_headless_cargo_source.contains(needle),
            "ui-headless feature contract should include `{needle}`.",
        );
    }

    assert!(
        ui_headless_lib_source
            .contains("#[cfg(all(feature = \"web\", feature = \"ssr\"))]\ncompile_error!("),
        "ui-headless should keep web/ssr compile_error! mutex guard.",
    );
    assert!(
        ui_headless_lib_source.contains("mutually exclusive"),
        "ui-headless mutex compile_error should keep explicit mutually-exclusive diagnosis.",
    );

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "if cargo check -p ui-headless --no-default-features --features web,ssr >\"$MUTEX_LOG\" 2>&1; then",
        "expected ui-headless web+ssr to fail",
        "rg -n \"mutually exclusive\" \"$MUTEX_LOG\"",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep ui-headless web/ssr mutex verification token `{needle}`.",
        );
    }
}

#[test]
fn share_button_motion_non_wasm_stub_contract_is_predictable_and_toolchain_safe() {
    let check_source = load_source("src/button/share/check2.md");
    let ui_motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let ui_motion_stub_test_source = load_source("../ui-motion/tests/non_wasm_stub.rs");
    let share_motion_source = load_source("src/button/share/motion.rs");
    let share_view_source = load_source("src/button/share/view.rs");
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    assert!(
        check_source.contains("`ui-motion` 非 wasm 提供 no-op/stub"),
        "ShareButton checklist should keep explicit ui-motion non-wasm stub item.",
    );

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]\npub mod web;",
        "#[cfg(not(target_arch = \"wasm32\"))]\npub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`.",
        );
    }

    for needle in [
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "assert!(web::prefers_reduced_motion());",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
        "web::animate(&(), &keyframes, MotionOptions::default());",
    ] {
        assert!(
            ui_motion_stub_test_source.contains(needle),
            "ui-motion non-wasm stub tests should include `{needle}`.",
        );
    }

    for needle in [
        "pub fn sanitize_motion(motion: ShareButtonMotion) -> ShareButtonMotion",
        "flip: super::super::flip::motion::sanitize_motion(motion.flip)",
        "let motion = super::motion::sanitize_motion(motion);",
    ] {
        assert!(
            share_motion_source.contains(needle) || share_view_source.contains(needle),
            "ShareButton motion wiring should include `{needle}`.",
        );
    }

    for forbidden in [
        "web_sys::",
        "js_sys::",
        "request_animation_frame",
        "panic!(",
    ] {
        assert!(
            !share_motion_source.contains(forbidden),
            "ShareButton motion mapping layer should avoid runtime-only or unsafe stub token `{forbidden}`.",
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep ui-motion no-op/stub verification token `{needle}`.",
        );
    }
}

#[test]
fn share_button_reduced_motion_ssr_wasm_branches_are_covered_via_flip_and_motion_contracts() {
    let check_source = load_source("src/button/share/check2.md");
    let share_view_source = load_source("src/button/share/view.rs");
    let share_motion_source = load_source("src/button/share/motion.rs");
    let flip_view_source = load_source("src/button/flip/view.rs");
    let flip_motion_source = load_source("src/button/flip/motion.rs");
    let button_styles_source = load_source("src/button/styles.rs");
    let ui_motion_spring_source = load_source("../ui-motion/src/spring.rs");
    let ui_motion_spring_checks_source = load_source("../ui-motion/tests/spring.rs");
    let platform_script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    assert!(
        check_source.contains("组件实现覆盖 `reduced-motion` / SSR / wasm 分支"),
        "ShareButton checklist should keep explicit reduced-motion/SSR/wasm branch item.",
    );

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            ui_motion_spring_source.contains(needle),
            "ui-motion spring should keep reduced-motion downgrade behavior token `{needle}`.",
        );
    }

    for needle in [
        "fn reduced_motion_set_target_applies_immediately()",
        "fn reduced_motion_set_target_triggers_on_rest_synchronously()",
        "fn reduced_motion_clear_on_rest_stops_triggering()",
    ] {
        assert!(
            ui_motion_spring_checks_source.contains(needle),
            "ui-motion reduced-motion regression tests should include `{needle}`.",
        );
    }

    assert!(
        button_styles_source.contains("@media (prefers-reduced-motion: reduce)"),
        "ShareButton should inherit reduced-motion CSS baseline via reused Button contract.",
    );

    for needle in [
        "data-state=state.state_attr",
        "data-motion-source=if motion == ShareButtonMotion::default()",
        "data-group-label-source=if normalized_group_aria_label.is_some() {",
        "role=group_a11y.role",
        "lang=group_a11y.lang.clone()",
        "dir=group_a11y.dir",
        "<FlipButton",
        "motion=motion.flip",
        "<ButtonGroup attached=true>",
    ] {
        assert!(
            share_view_source.contains(needle),
            "ShareButton view should keep SSR/hydration-stable semantic contract token `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            flip_motion_source.contains(needle),
            "FlipButton motion should keep explicit wasm/non-wasm split token `{needle}` for ShareButton reuse.",
        );
    }

    for needle in [
        "motion::attach_motion(node_ref, is_active, direction, motion);",
        "data-state=move || state.get().state_attr",
        "data-motion-source=move || state.get().motion_source_attr",
    ] {
        assert!(
            flip_view_source.contains(needle),
            "FlipButton view should keep wasm enhancement while preserving semantic marker token `{needle}`.",
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            !share_view_source.contains(forbidden) && !share_motion_source.contains(forbidden),
            "ShareButton semantic surface should not split by platform in component-level view/motion `{forbidden}`.",
        );
    }

    for needle in [
        "cargo check -p ui-components --no-default-features --features component-button,inject-css",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-button,inject-css",
        "cargo test -p ui-motion --test non_wasm_stub",
        "cargo test -p ui-components --test button_copy_semantics button_copy_reduced_motion_ssr_wasm_branches_are_covered_via_button_contract",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep reduced-motion/SSR/wasm verification token `{needle}`.",
        );
    }
}

#[test]
fn share_button_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let check2_source = load_source("src/button/share/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_source("src/button/share/view.rs");

    for needle in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "\"share-button\" => UiPerfBudget {",
        "max_mount_ms: 32.0,",
        "max_update_ms: Some(11.0),",
        "max_heap_kb: Some(576.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep performance budget contract token `{needle}`.",
        );
    }

    for needle in [
        "\"ShareButton\"",
        "\"share-button\"",
        "actions::share_button",
    ] {
        assert!(
            pages_source.contains(needle),
            "ShareButton docs page should remain in component coverage traversal via `{needle}`.",
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
            "docs coverage e2e should enforce repeatable perf regression guard `{needle}`.",
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace-based perf attribution token `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "ShareButton performance governance should keep render_count follow-up marker `{needle}`.",
        );
    }

    for needle in [
        "data-state=state.state_attr",
        "data-items-source=state.items_source_attr",
        "data-motion-source=if motion == ShareButtonMotion::default()",
        "data-custom-motion=(motion != ShareButtonMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "ShareButton should expose state/render/style/motion attribution marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test share_button_semantics share_button_performance_governance_budget_is_defined_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`.",
    );
}

#[test]
fn share_button_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("src/button/share/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    for needle in [
        "fn render_platform_icon(platform: SharePlatform) -> AnyView",
        "fn render_front_button(",
        "fn render_front_panel(",
        "fn render_platform_button(",
        "fn render_back_panel(",
        "render_front_panel(icon, variant, size, label)",
        "render_back_panel(items, icon_button_size, on_icon_press)",
    ] {
        assert!(
            view_source.contains(needle),
            "ShareButton view should keep macro complexity split marker `{needle}`."
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count <= 12,
        "ShareButton view macro expansion should stay controlled after semantic split; got {view_macro_count} `view!` blocks."
    );

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "ShareButton should keep a single public component boundary."
    );

    for forbidden in [
        "let front_button = match icon {",
        "let icon = match platform {",
        "#[component]\nfn render_front_panel(",
        "#[component]\nfn render_back_panel(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ShareButton should avoid macro-overgrown inlined branch token `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test share_button_semantics share_button_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );
}
