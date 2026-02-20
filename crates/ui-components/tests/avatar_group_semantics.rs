use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = if let Some(suffix) = rel_path.strip_prefix("src/avatar/") {
        manifest_dir
            .join("../../components/avatar-group/src")
            .join(suffix)
    } else {
        manifest_dir.join(rel_path)
    };
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn function_signature(source: &str, fn_name: &str) -> String {
    let start = source
        .find(&format!("pub fn {fn_name}("))
        .unwrap_or_else(|| panic!("missing function signature for `{fn_name}`"));
    let end = source[start..]
        .find(") -> impl IntoView {")
        .unwrap_or_else(|| panic!("missing IntoView return marker for `{fn_name}`"));
    source[start..start + end].to_string()
}

#[test]
fn avatar_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/avatar/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "AvatarGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn avatar_group_uses_logic_state_model() {
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");

    for needle in [
        "pub use ui_state_primitives::avatar_group::{",
        "AvatarGroupStateInput",
        "normalize_optional_text",
        "normalize_avatar_group_max_visible",
        "resolve_avatar_group_aria_label",
        "pub fn normalize_avatar_group_input(",
        "pub fn normalize_avatar_group_item_fields(",
        "pub fn resolve_avatar_group_aria_label_with_fallback(",
        "AvatarGroupRenderState",
        "resolve_render_state",
        "pub fn compose_avatar_group_class_name(",
        "ui-avatar-group--custom-class",
    ] {
        assert!(
            logic_source.contains(needle),
            "AvatarGroup logic should consume state primitives and keep assembly helpers; missing `{needle}`."
        );
    }

    for forbidden in [
        "pub struct AvatarGroupStateInput {",
        "pub struct AvatarGroupState {",
        "pub enum AvatarGroupVisualState {",
        "pub enum AvatarGroupAriaLabelSource {",
        "pub enum AvatarGroupClassSource {",
        "pub struct AvatarGroupRenderState {",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "AvatarGroup logic must not reimplement state primitives; found `{forbidden}`."
        );
    }

    for needle in [
        "let i18n = i18n::use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "let normalized = logic::normalize_avatar_group_input(",
        "let group_a11y = labeled_group_attrs(normalized.aria_label, normalized.lang.clone(), dir);",
        "logic::resolve_avatar_group_render_state(logic::AvatarGroupStateInput {",
        "let class = logic::compose_avatar_group_class_name(normalized.class_name, state);",
        "let fields = logic::normalize_avatar_group_item_fields(item.name, item.src, item.alt);",
    ] {
        assert!(
            view_source.contains(needle),
            "AvatarGroup view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn avatar_group_emits_baseline_style_root_data_attributes() {
    let source = load_source("src/avatar/view.rs");

    for attr in [
        "data-slot=\"avatar-group\"",
        "data-size=state.size_attr",
        "data-state=state.visual_state.as_str()",
        "data-empty=state.visual_state.is_empty().then_some(\"true\")",
        "data-has-items=state.has_items().then_some(\"true\")",
        "data-has-overflow=state.visual_state.has_overflow().then_some(\"true\")",
        "data-count=state.total_count.to_string()",
        "data-visible-count=state.visible_count.to_string()",
        "data-overflow-count=state.overflow_count.to_string()",
        "data-max-visible=state.max_visible.to_string()",
        "data-custom-aria-label=state.aria_label_source.is_custom().then_some(\"true\")",
        "data-aria-label-source=state.aria_label_source.as_str()",
        "data-custom-class=state.class_source.is_custom().then_some(\"true\")",
        "data-class-source=state.class_source.as_str()",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label",
        "lang=group_a11y.lang.clone()",
        "dir=group_a11y.dir",
    ] {
        assert!(
            source.contains(attr),
            "AvatarGroup should set `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn avatar_group_a11y_i18n_l10n_contract_is_headless_driven_and_no_view_hardcoded_copy() {
    let view_source = load_source("src/avatar/view.rs");
    let headless_a11y_source = load_source("../ui-headless/src/a11y.rs");
    let i18n_common_source = load_source("../ui-headless/src/i18n/common.rs");

    for required in [
        "use ui_headless::labeled_group_attrs;",
        "let i18n = i18n::use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "common.avatar_group_aria_label.as_ref()",
        "common.avatar_group_overflow_aria_label_suffix.as_ref()",
        "let group_a11y = labeled_group_attrs(normalized.aria_label, normalized.lang.clone(), dir);",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label",
        "lang=group_a11y.lang.clone()",
        "dir=group_a11y.dir",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup should wire a11y/i18n/l10n contract via `{required}`."
        );
    }

    for required in [
        "pub fn labeled_group_attrs(",
        "pub struct LabeledGroupA11yAttrs",
        "pub fn locale_attrs(",
    ] {
        assert!(
            headless_a11y_source.contains(required),
            "AvatarGroup shared a11y utilities should come from ui-headless via `{required}`."
        );
    }

    for required in [
        "avatar_group_aria_label",
        "avatar_group_overflow_aria_label_suffix",
    ] {
        assert!(
            i18n_common_source.contains(required),
            "AvatarGroup i18n bundle should expose string slot `{required}`."
        );
    }

    for forbidden in [
        "\"Avatar group\"",
        "\"more collaborators\"",
        "role=\"group\"",
        "dir=\"ltr\"",
        "dir=\"rtl\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup view should not hardcode user-visible copy/locale/a11y literal `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_exposes_item_and_overflow_slots() {
    let source = load_source("src/avatar/view.rs");

    for attr in [
        "data-slot=\"avatar-group-item\"",
        "data-index=index",
        "data-has-src=fields.has_src.then_some(\"true\")",
        "class_name=\"ui-avatar-group__avatar\"",
        "data-slot=\"avatar-group-overflow\"",
        "data-count=state.overflow_count.to_string()",
    ] {
        assert!(
            source.contains(attr),
            "AvatarGroup should expose `{attr}` for deterministic item/overflow hooks."
        );
    }
}

#[test]
fn avatar_group_has_no_async_loading_protocol_and_keeps_sync_render_contract() {
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");

    for needle in [
        "let state = logic::resolve_avatar_group_render_state(logic::AvatarGroupStateInput {",
        "let fields = logic::normalize_avatar_group_item_fields(item.name, item.src, item.alt);",
        "<Show when=move || state.visual_state.has_overflow()>",
    ] {
        assert!(
            view_source.contains(needle),
            "AvatarGroup should keep synchronous render contract via `{needle}`."
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
            "AvatarGroup has no async workflow; forbidden async/loading token `{forbidden}` should be absent."
        );
    }
}

#[test]
fn avatar_group_styles_include_state_source_and_marker_contracts() {
    let source = load_source("src/avatar/styles.rs");

    for selector in [
        ".ui-avatar-group--size-sm",
        ".ui-avatar-group[data-size=\"md\"]",
        ".ui-avatar-group--size-lg",
        ".ui-avatar-group--stable",
        ".ui-avatar-group[data-state=\"overflow\"]",
        ".ui-avatar-group--overflow .ui-avatar-group__overflow",
        ".ui-avatar-group[data-has-overflow=\"true\"] .ui-avatar-group__overflow",
        ".ui-avatar-group[data-state=\"empty\"]",
        ".ui-avatar-group--label-source-custom",
        ".ui-avatar-group[data-aria-label-source=\"custom\"]",
        ".ui-avatar-group--custom-class",
        ".ui-avatar-group[data-custom-class=\"true\"]",
        ".ui-avatar-group[data-class-source=\"custom\"]",
    ] {
        assert!(
            source.contains(selector),
            "AvatarGroup styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn avatar_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn avatar_group() -> AnyView",
        "title=\"AvatarGroup\"",
        "slug=\"avatar-group\"",
        "Playground title=\"Hello World\"",
        "Playground title=\"Overflow Stack\"",
        "Playground title=\"Sizes Without Overflow\"",
        "Playground title=\"Custom Aria + Class\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for AvatarGroup.",
        );
    }
}

#[test]
fn avatar_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "let hello_code =",
        "r#\"<AvatarGroup items=Vec::<AvatarGroupItem>::new() />\"#.to_string()",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<AvatarGroup items=empty_items.clone() />",
        "title=\"Overflow Stack\"",
        "<AvatarGroup items=overflow_items.clone() max=3 size=AvatarSize::Md />",
        "max=2",
        "aria_label=\"Core collaborators\".to_string()",
        "title=\"Sizes Without Overflow\"",
        "<AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Sm />",
        "<AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Lg />",
        "title=\"Custom Aria + Class\"",
        "items=empty_items.clone()",
        "aria_label=\"No collaborators\".to_string()",
        "class_name=\"docs-avatar-group-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "avatar-group docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn avatar_group_docs_expose_hello_world_path_without_state_machine_wiring() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "let hello_code =",
        "r#\"<AvatarGroup items=Vec::<AvatarGroupItem>::new() />\"#.to_string()",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<AvatarGroup items=empty_items.clone() />",
    ] {
        assert!(
            source.contains(needle),
            "AvatarGroup docs should keep minimal hello-world usage path via `{needle}`."
        );
    }

    for forbidden in ["<AvatarGroup state=", "ui_state_primitives", "ui-headless"] {
        assert!(
            !source.contains(forbidden),
            "AvatarGroup docs minimal usage should not require internal wiring token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_does_not_define_component_motion_runtime() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let styles_source = load_source("src/avatar/styles.rs");

    assert!(
        !manifest_dir
            .join("../../components/avatar-group/src/motion.rs")
            .exists(),
        "AvatarGroup should not define `src/avatar/motion.rs` when no runtime animation contract is needed."
    );

    for forbidden in [
        "ui_motion::",
        "request_animation_frame",
        "cancel_animation_frame",
        "SpringAnimator::new",
        "attach_motion(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "AvatarGroup should stay motion-runtime free in component layer; found `{forbidden}`."
        );
    }

    for forbidden_css in ["transition:", "animation:"] {
        assert!(
            !styles_source.contains(forbidden_css),
            "AvatarGroup styles should stay static without runtime motion marker `{forbidden_css}`."
        );
    }
}

#[test]
fn avatar_group_theme_contract_consumes_ui_variables_only() {
    let styles_source = load_source("src/avatar/styles.rs");
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");

    for required_var in [
        "var(--ui-bg)",
        "var(--ui-bg-muted)",
        "var(--ui-fg)",
        "var(--ui-shadow-sm)",
        "var(--ui-accent-soft)",
        "var(--ui-accent)",
    ] {
        assert!(
            styles_source.contains(required_var),
            "AvatarGroup styles should consume ui-theme css variables via `{required_var}`."
        );
    }

    for forbidden in [
        "Theme::",
        "ThemeContext",
        "theme_to_css_variables",
        "ui_theme::",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "AvatarGroup component layer must not rebuild theme context; found `{forbidden}`."
        );
    }

    assert!(
        !styles_source.contains("--avatar-group-"),
        "AvatarGroup should not introduce private non-`--ui-*` token namespace."
    );
}

#[test]
fn avatar_group_stays_as_ui_components_assembly_layer_without_platform_leakage() {
    let mod_source = load_source("src/avatar/mod.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let lib_source = load_source("src/lib.rs");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::{AvatarGroup, AvatarGroupItem};",
    ] {
        assert!(
            mod_source.contains(required),
            "AvatarGroup module boundary should include `{required}`."
        );
    }

    for required in [
        "pub use ui_state_primitives::avatar_group::{",
        "pub fn compose_avatar_group_class_name(",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup logic should stay in assembly role and include `{required}`."
        );
    }

    for forbidden in ["view! {", "data-slot=", "labeled_group_attrs("] {
        assert!(
            !logic_source.contains(forbidden),
            "AvatarGroup logic must not carry view/headless wiring `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "logic::resolve_avatar_group_render_state(",
        "logic::compose_avatar_group_class_name(",
        "labeled_group_attrs(",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup view should compose logic + headless contract via `{required}`."
        );
    }

    for forbidden in [
        "pub struct AvatarGroupState {",
        "ui_state_primitives::avatar_group::AvatarGroupState {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup view must not reimplement primitives; found `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("var(--ui-"),
        "AvatarGroup styles should be token-first and consume `--ui-*` variables."
    );

    assert!(
        lib_source.contains("AvatarGroup") && lib_source.contains("AvatarGroupItem"),
        "ui-components public API should expose stable AvatarGroup exports."
    );

    for forbidden in [
        "pub use web_sys::",
        "pub use wasm_bindgen",
        "pub use leptos::html::",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-components public API should not leak platform detail `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_public_api_naming_contract_is_stable_and_prefix_ready() {
    let view_source = load_source("src/avatar/view.rs");
    let sig = function_signature(&view_source, "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        "max: Option<usize>",
        "size: AvatarSize",
        "aria_label: Option<String>",
        "class_name: Option<String>",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
    ] {
        assert!(
            sig.contains(required),
            "AvatarGroup public API should keep stable prop naming `{required}`."
        );
    }

    assert!(
        !sig.contains(": bool"),
        "AvatarGroup currently has no public boolean props; future booleans must use `is_*`."
    );
    assert!(
        !sig.contains("on_"),
        "AvatarGroup currently has no public callbacks; future callbacks must use `on_*`."
    );
    assert!(
        !sig.contains("default_"),
        "AvatarGroup currently has no public default-value props; future defaults must use `default_*`."
    );
}

#[test]
fn avatar_group_composition_api_uses_typed_item_specs_and_rejects_parallel_arrays() {
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");
    let sig = function_signature(&view_source, "AvatarGroup");

    for required in [
        "items: Vec<AvatarGroupItem>",
        "let fields = logic::normalize_avatar_group_item_fields(item.name, item.src, item.alt);",
        ".map(|(index, item)| {",
        "data-slot=\"avatar-group-item\"",
    ] {
        assert!(
            view_source.contains(required) || sig.contains(required),
            "AvatarGroup should bind title/semantics/content in one typed item dimension via `{required}`."
        );
    }

    for forbidden in [
        "labels: Vec<",
        "titles: Vec<",
        "children: Vec<",
        "labels=",
        "titles=",
        "titles + panels",
        "labels + children",
    ] {
        assert!(
            !sig.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "AvatarGroup should reject parallel-array composition token `{forbidden}`."
        );
    }

    for required in ["<AvatarGroup", "items=vec![", "AvatarGroupItem {"] {
        assert!(
            docs_source.contains(required),
            "AvatarGroup docs should keep typed ItemSpec composition sample via `{required}`."
        );
    }
}

#[test]
fn avatar_group_has_no_controllable_state_axis_and_no_half_controlled_api() {
    let view_source = load_source("src/avatar/view.rs");
    let sig = function_signature(&view_source, "AvatarGroup");

    for forbidden in [" value:", "default_", "on_value_change", "on_open_change"] {
        assert!(
            !sig.contains(forbidden),
            "AvatarGroup should not expose partial controllable API marker `{forbidden}` without full value/on_change/default pair."
        );
    }

    for forbidden in [
        "use_controllable_state(",
        "use_controllable_open_state_traced(",
        "on_value_change",
        "on_open_change",
        "default_value",
        "default_open",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup has no controllable state axis and should not include `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_defaults_are_centralized_in_logic() {
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");

    for required in [
        "pub fn normalize_avatar_group_input(",
        "pub fn normalize_avatar_group_item_fields(",
        "pub fn resolve_avatar_group_aria_label_with_fallback(",
        "name: name.unwrap_or_default()",
        "src: src.unwrap_or_default()",
        "alt: alt.unwrap_or_default()",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup logic should centralize default normalization via `{required}`."
        );
    }

    for forbidden in ["unwrap_or_default()", "logic::normalize_optional_text("] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup view should not perform fallback normalization directly; found `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_state_primitive_source_boundary_is_enforced() {
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/avatar_group.rs");
    let sig = function_signature(&view_source, "AvatarGroup");

    for required in [
        "pub use ui_state_primitives::avatar_group::{",
        "AvatarGroupStateInput",
        "AvatarGroupRenderState",
        "resolve_render_state",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup logic should source state primitives from ui-state-primitives via `{required}`."
        );
    }

    for required in [
        "pub enum AvatarGroupVisualState",
        "pub enum AvatarGroupAriaLabelSource",
        "pub enum AvatarGroupClassSource",
        "pub struct AvatarGroupRenderState",
        "pub fn resolve_render_state(",
    ] {
        assert!(
            primitive_source.contains(required),
            "AvatarGroup render-state primitives should be implemented in ui-state-primitives; missing `{required}`."
        );
    }

    for forbidden in [
        "use crate::store::",
        "use crate::state::",
        "global_store",
        "app_store",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "AvatarGroup component layer should not bind business store directly; found `{forbidden}`."
        );
    }

    for forbidden in ["RwSignal<", "ReadSignal<", "WriteSignal<", "Signal<"] {
        assert!(
            !sig.contains(forbidden),
            "AvatarGroup public API should not expose framework/store state container `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_state_normalization_is_centralized_in_logic() {
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");

    for required in [
        "pub struct AvatarGroupNormalizedInput",
        "pub struct AvatarGroupItemFields",
        "AvatarGroupRenderState",
        "pub fn normalize_avatar_group_input(",
        "pub fn normalize_avatar_group_item_fields(",
        "resolve_render_state",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup logic should type and derive group/item render state via `{required}`."
        );
    }

    for forbidden in [
        "pub enum AvatarGroupVisualState {",
        "pub enum AvatarGroupAriaLabelSource {",
        "pub enum AvatarGroupClassSource {",
        "pub struct AvatarGroupRenderState {",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "AvatarGroup logic should consume render-state primitives from ui-state-primitives; found local `{forbidden}`."
        );
    }

    for required in [
        "let normalized = logic::normalize_avatar_group_input(",
        "let state = logic::resolve_avatar_group_render_state(logic::AvatarGroupStateInput {",
        "let fields = logic::normalize_avatar_group_item_fields(item.name, item.src, item.alt);",
        "<Show when=move || state.visual_state.has_overflow()>",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup view should consume centralized state and only render by markers via `{required}`."
        );
    }

    for forbidden in [
        "data-state=if",
        "if items.len()",
        "if state.total_count",
        "if state.overflow_count",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup view should not rebuild root state machine branches; found `{forbidden}`."
        );
    }

    for required in [
        ".ui-avatar-group[data-state=\"overflow\"]",
        ".ui-avatar-group[data-state=\"empty\"]",
        ".ui-avatar-group[data-has-overflow=\"true\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "AvatarGroup styles should consume explicit state markers via `{required}`."
        );
    }
}

#[test]
fn avatar_group_state_markers_are_observable_and_closed_set_contracts() {
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let primitive_source = load_source("../ui-state-primitives/src/avatar_group.rs");

    for required in [
        "data-slot=\"avatar-group\"",
        "data-state=state.visual_state.as_str()",
        "data-empty=state.visual_state.is_empty().then_some(\"true\")",
        "data-has-overflow=state.visual_state.has_overflow().then_some(\"true\")",
        "data-aria-label-source=state.aria_label_source.as_str()",
        "data-class-source=state.class_source.as_str()",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label",
        "data-slot=\"avatar-group-item\"",
        "data-slot=\"avatar-group-overflow\"",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup markers should stay observable via `{required}`."
        );
    }

    for required in [
        "pub enum AvatarGroupVisualState",
        "Self::Stable => \"stable\"",
        "Self::Overflow => \"overflow\"",
        "Self::Empty => \"empty\"",
        "pub enum AvatarGroupAriaLabelSource",
        "Self::Default => \"default\"",
        "Self::Custom => \"custom\"",
        "pub enum AvatarGroupClassSource",
    ] {
        assert!(
            primitive_source.contains(required),
            "AvatarGroup marker values should come from enum closed set via `{required}`."
        );
    }

    for required in [
        ".ui-avatar-group[data-state=\"overflow\"]",
        ".ui-avatar-group[data-state=\"empty\"]",
        ".ui-avatar-group[data-aria-label-source=\"custom\"]",
        ".ui-avatar-group[data-class-source=\"custom\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "AvatarGroup selectors should be queryable from semantic markers via `{required}`."
        );
    }

    for forbidden in [
        "data-state=format!",
        "data-state=if",
        "data-aria-label-source=format!",
        ".ui-avatar-group:nth-child(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !styles_source.contains(forbidden),
            "AvatarGroup marker contract should avoid free-text or DOM-order selector pattern `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let styles_source = load_source("src/avatar/styles.rs");
    let view_source = load_source("src/avatar/view.rs");

    for required in [
        ".ui-avatar-group[data-state=\"stable\"]",
        ".ui-avatar-group[data-state=\"overflow\"]",
        ".ui-avatar-group[data-state=\"empty\"]",
        ".ui-avatar-group[data-has-overflow=\"true\"]",
        ".ui-avatar-group[data-empty=\"true\"]",
        ".ui-avatar-group[data-aria-label-source=\"default\"]",
        ".ui-avatar-group[data-aria-label-source=\"custom\"]",
        ".ui-avatar-group[data-class-source=\"custom\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "AvatarGroup state styles should rely on explicit markers via `{required}`."
        );
    }

    for forbidden in [
        ".ui-avatar-group:nth-child(",
        ".ui-avatar-group:nth-of-type(",
        ".ui-avatar-group > * > * > *",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "AvatarGroup state styling should not guess from fragile DOM selector `{forbidden}`."
        );
    }

    assert!(
        !view_source.contains("style="),
        "AvatarGroup runtime should not inject business style logic inline."
    );
}

#[test]
fn avatar_group_semantics_suite_prioritizes_contract_assertions_over_snapshots() {
    let suite_source = load_source("tests/avatar_group_semantics.rs");

    for required in [
        "fn avatar_group_emits_baseline_style_root_data_attributes()",
        "fn avatar_group_a11y_i18n_l10n_contract_is_headless_driven_and_no_view_hardcoded_copy()",
        "fn avatar_group_state_markers_are_observable_and_closed_set_contracts()",
        "fn avatar_group_styles_depend_on_explicit_state_markers_not_dom_guessing()",
        "fn avatar_group_state_normalization_is_centralized_in_logic()",
        "fn avatar_group_has_no_controllable_state_axis_and_no_half_controlled_api()",
        "fn avatar_group_has_no_async_loading_protocol_and_keeps_sync_render_contract()",
        "fn avatar_group_does_not_define_component_motion_runtime()",
    ] {
        assert!(
            suite_source.contains(required),
            "AvatarGroup semantics suite should prioritize contract coverage via `{required}`."
        );
    }

    let forbidden_tokens = [
        format!("{}{}", "assert_", "snapshot!"),
        format!("{}{}", "assert_debug_", "snapshot!"),
        format!("{}{}", "assert_json_", "snapshot!"),
        format!("{}{}", "to_match_", "snapshot"),
        format!("{}{}", "ins", "ta::"),
        format!("{}{}", ".", "snap"),
        format!("{}{}", "gol", "den"),
        format!("{}{}", "pi", "xel"),
        format!("{}{}", "screen", "shot"),
    ];

    for forbidden in forbidden_tokens {
        assert!(
            !suite_source.contains(&forbidden),
            "AvatarGroup semantics suite should not depend on snapshot token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_component_files_follow_layered_responsibilities() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mod_source = load_source("src/avatar/mod.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let view_source = load_source("src/avatar/view.rs");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::{AvatarGroup, AvatarGroupItem};",
    ] {
        assert!(
            mod_source.contains(required),
            "AvatarGroup `mod.rs` should keep minimal export boundary via `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "pub fn normalize_avatar_group_input(",
        "pub const CSS:",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "AvatarGroup `mod.rs` should not carry implementation detail `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_avatar_group_input(",
        "pub fn normalize_avatar_group_item_fields(",
        "pub fn resolve_avatar_group_aria_label_with_fallback(",
        "pub fn compose_avatar_group_class_name(",
        "resolve_avatar_group_render_state(",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup `logic.rs` should keep normalization/derivation helpers via `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "data-slot=",
        "role=",
        "aria-label=",
        ".ui-avatar-group",
        "labeled_group_attrs(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "AvatarGroup `logic.rs` should not mix view/css/headless detail `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-avatar-group[data-state=\"overflow\"]",
        ".ui-avatar-group[data-aria-label-source=\"custom\"]",
        "var(--ui-",
    ] {
        assert!(
            styles_source.contains(required),
            "AvatarGroup `styles.rs` should keep token-first static CSS via `{required}`."
        );
    }

    for forbidden in ["view! {", "on:error", "labeled_group_attrs(", "logic::"] {
        assert!(
            !styles_source.contains(forbidden),
            "AvatarGroup `styles.rs` should not carry runtime/view logic `{forbidden}`."
        );
    }

    for required in [
        "use crate::logic::{self, AvatarSize};",
        "view! {",
        "logic::normalize_avatar_group_input(",
        "logic::resolve_avatar_group_render_state(",
        "logic::compose_avatar_group_class_name(",
        "labeled_group_attrs(",
        "<Avatar",
        "data-slot=\"avatar-group\"",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup `view.rs` should render structure, mount headless contract, and reuse Avatar via `{required}`."
        );
    }

    for forbidden in [
        "pub const CSS",
        "pub struct AvatarGroupState {",
        "pub enum AvatarGroupVisualState {",
        "ui_motion::",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup `view.rs` should not carry styles/primitive redefinition/motion engine detail `{forbidden}`."
        );
    }

    assert!(
        !manifest_dir
            .join("../../components/avatar-group/src/motion.rs")
            .exists(),
        "AvatarGroup is static in current scope; `motion.rs` should remain absent until motion contract is required."
    );
}

#[test]
fn avatar_group_does_not_introduce_spec_rs_and_keeps_lightweight_exports() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let group_mod_source = load_source("src/avatar/mod.rs");

    assert!(
        !manifest_dir
            .join("../../components/avatar-group/src/spec.rs")
            .exists(),
        "AvatarGroup should not introduce `src/avatar/spec.rs` without stable external schema need."
    );

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::{AvatarGroup, AvatarGroupItem};",
    ] {
        assert!(
            group_mod_source.contains(required),
            "AvatarGroup exports should remain minimal via `{required}`."
        );
    }

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !group_mod_source.contains(forbidden),
            "AvatarGroup `mod.rs` should stay assembly-only and avoid spec wiring `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_token_first_static_styles_contract_is_enforced_via_ui_root_css_pipeline() {
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");

    for required in [
        "#[cfg(feature = \"component-avatar_group\")]",
        "out.push_str(crate::avatar_group::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "Component CSS aggregation should include avatar-group styles via `{required}`."
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should be the CSS injection boundary via `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-bg)",
        "var(--ui-bg-muted)",
        "var(--ui-fg)",
        "var(--ui-shadow-sm)",
        "var(--ui-accent-soft)",
        "var(--ui-accent)",
    ] {
        assert!(
            styles_source.contains(required),
            "AvatarGroup styles should stay token-first and static via `{required}`."
        );
    }

    for forbidden in [
        "--avatar-group-",
        "@apply",
        "tailwind",
        "styled(",
        "emotion",
        "stylex",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "AvatarGroup styles should not introduce private-token or CSS-in-Rust utility marker `{forbidden}`."
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"items-",
        "class=\"gap-",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup view should not depend on utility-first class contract `{forbidden}`."
        );
    }

    for forbidden in ["style=", "style:"] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "AvatarGroup runtime should not carry inline business style logic `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_visual_desire_gate_reuses_theme_visual_baseline_and_heroui_contracts() {
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
            "Theme visual baseline page should keep visual-quality contract token `{needle}`."
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
            "Theme visual baseline e2e contract should include `{needle}`."
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
fn avatar_group_tree_shaking_contract_enforces_source_mode_reachability_boundaries() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let cargo_source = load_source("Cargo.toml");

    assert!(
        lib_source.contains("#[cfg(feature = \"component-avatar\")]\npub use ui_avatar as avatar;"),
        "avatar module should stay behind component-avatar gate for source-mode reachability."
    );

    for needle in [
        "component-avatar = [\"dep:ui-avatar\"]",
        "component-avatar_group = [\"component-avatar\", \"dep:ui-avatar-group\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "avatar-group feature relationship should remain explicit via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-avatar\")]\n    out.push_str(crate::avatar::styles::CSS);",
        "#[cfg(feature = \"component-avatar_group\")]\n    out.push_str(crate::avatar_group::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "css aggregation should keep source-mode reachability bounded by `{needle}`."
        );
    }

    for forbidden in [
        "static ALL_COMPONENTS",
        "const ALL_COMPONENTS",
        "HashMap<&'static str, fn",
    ] {
        assert!(
            !lib_source.contains(forbidden) && !css_source.contains(forbidden),
            "global registry pattern that defeats DCE should stay absent `{forbidden}`."
        );
    }
}

#[test]
fn avatar_group_machine_readable_contract_uses_typed_inputs_and_semantic_markers() {
    let primitive_source = load_source("../ui-state-primitives/src/avatar_group.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let logic_test_source = load_source("../../components/avatar-group/test/logic.rs");

    for required in [
        "pub struct AvatarGroupStateInput",
        "pub enum AvatarGroupVisualState",
        "pub enum AvatarGroupAriaLabelSource",
        "pub enum AvatarGroupClassSource",
        "pub struct AvatarGroupRenderState",
        "pub fn resolve_render_state(",
        "pub fn as_str(self) -> &'static str",
    ] {
        assert!(
            primitive_source.contains(required),
            "AvatarGroup machine-readable input/state should stay typed in primitives via `{required}`."
        );
    }

    for required in [
        "pub use ui_state_primitives::avatar_group::{",
        "AvatarGroupStateInput",
        "AvatarGroupRenderState",
        "resolve_render_state",
    ] {
        assert!(
            logic_source.contains(required),
            "AvatarGroup logic should consume typed primitives via `{required}`."
        );
    }

    for forbidden in [
        "data-state=format!",
        "data-aria-label-source=format!",
        "data-class-source=format!",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "AvatarGroup should avoid string-protocol state leakage `{forbidden}`."
        );
    }

    for required in [
        "data-state=state.visual_state.as_str()",
        "data-empty=state.visual_state.is_empty().then_some(\"true\")",
        "data-has-overflow=state.visual_state.has_overflow().then_some(\"true\")",
        "data-aria-label-source=state.aria_label_source.as_str()",
        "data-class-source=state.class_source.as_str()",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label",
    ] {
        assert!(
            view_source.contains(required),
            "AvatarGroup should expose machine-readable semantic markers via `{required}`."
        );
    }

    for required in [
        ".ui-avatar-group[data-state=\"overflow\"]",
        ".ui-avatar-group[data-state=\"empty\"]",
        ".ui-avatar-group[data-aria-label-source=\"custom\"]",
        ".ui-avatar-group[data-class-source=\"custom\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "AvatarGroup style contracts should consume stable semantic marker `{required}`."
        );
    }

    let required = "resolve_render_state_maps_discrete_status_and_sources_to_enums";
    assert!(
        logic_source.contains(required)
            || primitive_source.contains(required)
            || logic_test_source.contains(
                "resolve_avatar_group_render_state_maps_discrete_status_and_sources_to_enums",
            ),
        "Typed state contract should keep a regression anchor `{required}`."
    );
}
