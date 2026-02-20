use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = if let Some(suffix) = rel_path.strip_prefix("src/avatar/") {
        manifest_dir
            .join("../../components/avatar/src")
            .join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("src/avatar-group/") {
        manifest_dir
            .join("../../components/avatar-group/src")
            .join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("src/button/") {
        manifest_dir
            .join("../../components/button/src")
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
fn avatar_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/avatar/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Avatar internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn avatar_uses_logic_state_model() {
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");

    for needle in [
        "pub use ui_state_primitives::avatar::{",
        "AvatarStateInput",
        "AvatarState",
        "normalize_optional_text",
        "pub fn normalize_input(",
        "pub fn normalize_lang(",
        "AvatarImageRenderInput",
        "resolve_image_render_state",
        "resolve_initials",
        "resolve_accessibility",
        "resolve_state",
        "pub fn compose_class_name(",
        "ui-avatar--custom-class",
    ] {
        assert!(
            logic_source.contains(needle),
            "Avatar logic should consume state primitives and keep assembly helpers; missing `{needle}`."
        );
    }

    for forbidden in [
        "pub struct AvatarStateInput {",
        "pub struct AvatarState {",
        "pub struct AvatarAccessibility {",
        "pub struct AvatarImageRenderInput {",
        "pub struct AvatarImageRenderState {",
        "pub enum AvatarRenderMode {",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Avatar logic must not reimplement state primitives; found `{forbidden}`."
        );
    }

    for needle in [
        "let normalized = logic::normalize_input(name, src, alt, class_name);",
        "let locale = locale_attrs(logic::normalize_lang(lang), dir);",
        "let i18n = i18n::use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "let accessibility =",
        "logic::resolve_accessibility(normalized.name.as_deref(), normalized.alt.as_deref());",
        "let state = logic::resolve_state(logic::AvatarStateInput {",
        "let render_state = Signal::derive(move || {",
        "logic::resolve_image_render_state(logic::AvatarImageRenderInput {",
        "let class = logic::compose_class_name(normalized.class_name, state);",
    ] {
        assert!(
            view_source.contains(needle),
            "Avatar view should derive state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn avatar_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/avatar/view.rs");

    for attr in [
        "data-slot=\"avatar\"",
        "data-size=state.size_attr",
        "data-state=move || render_state.get().mode.as_str()",
        "data-image=move || render_state.get().mode.image_attr()",
        "data-fallback=move || render_state.get().mode.fallback_attr()",
        "data-has-name=state.has_name.then_some(\"true\")",
        "data-has-src=state.has_src.then_some(\"true\")",
        "data-has-alt=state.has_alt.then_some(\"true\")",
        "data-label-source=state.label_source.as_str()",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "class:ui-avatar--image=move || render_state.get().mode.shows_image()",
        "class:ui-avatar--fallback=move || !render_state.get().mode.shows_image()",
    ] {
        assert!(
            source.contains(attr),
            "Avatar should set `{attr}` to support baseline-style styling and state inspection."
        );
    }
}

#[test]
fn avatar_fallback_wires_accessible_name_contract() {
    let source = load_source("src/avatar/view.rs");

    for needle in [
        "role=move || image_fallback_attrs(render_state.get().mode.shows_image(), aria_label.get_value()).role",
        "aria-label=move || image_fallback_attrs(render_state.get().mode.shows_image(), aria_label.get_value()).aria_label",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "data-slot=\"avatar-initials\"",
    ] {
        assert!(
            source.contains(needle),
            "Avatar fallback should include `{needle}` for accessible image semantics."
        );
    }
}

#[test]
fn avatar_a11y_i18n_l10n_contract_is_headless_driven_and_no_view_hardcoded_copy() {
    let view_source = load_source("src/avatar/view.rs");
    let headless_a11y_source = load_source("../ui-headless/src/a11y.rs");
    let i18n_common_source = load_source("../ui-headless/src/i18n/common.rs");

    for required in [
        "use ui_headless::{A11yDirection, image_fallback_attrs, locale_attrs};",
        "let i18n = i18n::use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "common.avatar_fallback_aria_label.as_ref().to_string()",
        "let locale = locale_attrs(logic::normalize_lang(lang), dir);",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "image_fallback_attrs(",
    ] {
        assert!(
            view_source.contains(required),
            "Avatar should wire a11y/i18n/l10n contract via `{required}`."
        );
    }

    for required in [
        "pub fn image_fallback_attrs(",
        "pub fn locale_attrs(",
        "pub struct ImageFallbackA11yAttrs",
        "pub struct A11yLocaleAttrs",
    ] {
        assert!(
            headless_a11y_source.contains(required),
            "Avatar shared a11y utilities should come from ui-headless via `{required}`."
        );
    }

    for required in ["avatar_fallback_aria_label", "avatar_group_aria_label"] {
        assert!(
            i18n_common_source.contains(required),
            "Avatar i18n bundle should expose string slot `{required}`."
        );
    }

    for forbidden in [
        "\"Avatar\"",
        "\"Avatar group\"",
        "\"more collaborators\"",
        "role=\"img\"",
        "dir=\"ltr\"",
        "dir=\"rtl\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Avatar view should not hardcode user-visible copy/locale/a11y literal `{forbidden}`."
        );
    }
}

#[test]
fn avatar_image_slot_supports_error_fallback() {
    let source = load_source("src/avatar/view.rs");

    for needle in [
        "data-slot=\"avatar-img\"",
        "on:error=move |_| img_error.set(true)",
        "let render_state = Signal::derive(move || {",
        "has_img_error: img_error.get(),",
    ] {
        assert!(
            source.contains(needle),
            "Avatar image rendering should include `{needle}` so broken images fall back to initials."
        );
    }
}

#[test]
fn avatar_has_no_async_loading_protocol_and_keeps_sync_error_fallback_contract() {
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");

    for needle in [
        "on:error=move |_| img_error.set(true)",
        "let img_error = RwSignal::new(false);",
        "logic::resolve_image_render_state(logic::AvatarImageRenderInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "Avatar should keep synchronous fallback contract via `{needle}`."
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
            "Avatar has no async workflow; forbidden async/loading token `{forbidden}` should be absent."
        );
    }
}

#[test]
fn avatar_styles_include_dual_state_marker_contracts() {
    let source = load_source("src/avatar/styles.rs");

    for selector in [
        ".ui-avatar--sm",
        ".ui-avatar[data-size=\"md\"]",
        ".ui-avatar--lg",
        ".ui-avatar--image",
        ".ui-avatar[data-state=\"fallback\"]",
        ".ui-avatar[data-image=\"true\"]",
        ".ui-avatar--has-src.ui-avatar--image",
        ".ui-avatar[data-has-src=\"true\"][data-state=\"image\"]",
        ".ui-avatar--label-alt",
        ".ui-avatar[data-label-source=\"name\"]",
        ".ui-avatar[data-label-source=\"fallback\"]",
        ".ui-avatar[data-has-alt=\"true\"][data-fallback=\"true\"]",
        ".ui-avatar--custom-class",
        ".ui-avatar[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Avatar styles should include `{selector}` for stable state-marker contracts."
        );
    }
}

#[test]
fn avatar_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn avatar() -> AnyView",
        "title=\"Avatar\"",
        "slug=\"avatar\"",
        "Playground title=\"Hello World\"",
        "Playground title=\"Image + Fallback\"",
        "Playground title=\"Sizes + Label Sources\"",
        "Playground title=\"Custom Class + Normalized Props\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for Avatar.",
        );
    }
}

#[test]
fn avatar_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "let hello_code = Signal::derive(move || r#\"<Avatar />\"#.to_string());",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Avatar />",
        "title=\"Image + Fallback\"",
        "<Avatar name=\"Ada Lovelace\".to_string() src=src.to_string() size=AvatarSize::Md />",
        "<Avatar name=\"Grace Hopper\".to_string() size=AvatarSize::Md />",
        "title=\"Sizes + Label Sources\"",
        "alt=\"Profile photo\".to_string()",
        "<Avatar alt=\"Anonymous collaborator\".to_string() size=AvatarSize::Sm />",
        "<Avatar size=AvatarSize::Lg />",
        "title=\"Custom Class + Normalized Props\"",
        "class_name=\"docs-avatar-custom\".to_string()",
        "src=\"   \".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "avatar docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn avatar_docs_expose_hello_world_path_without_state_machine_wiring() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "let hello_code = Signal::derive(move || r#\"<Avatar />\"#.to_string());",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Avatar />",
    ] {
        assert!(
            source.contains(needle),
            "Avatar docs should keep minimal hello-world usage path via `{needle}`."
        );
    }

    for forbidden in ["<Avatar state=", "ui_state_primitives", "ui-headless"] {
        assert!(
            !source.contains(forbidden),
            "Avatar docs minimal usage should not require internal wiring token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_stays_static_and_delegates_motion_runtime_to_ui_motion_layer() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let style_source = load_source("src/avatar/styles.rs");
    let ui_motion_source = load_source("../ui-motion/src/lib.rs");

    assert!(
        !manifest_dir
            .join("../../components/avatar/src/motion.rs")
            .exists(),
        "Avatar is a static component and should not introduce `src/avatar/motion.rs`."
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
            "Avatar should not reimplement motion runtime in component layer; found `{forbidden}`."
        );
    }

    for forbidden_css in ["transition:", "animation:"] {
        assert!(
            !style_source.contains(forbidden_css),
            "Avatar styles should stay static without component-level motion marker `{forbidden_css}`."
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(required),
            "ui-motion should keep non-wasm predictable stub contract via `{required}`."
        );
    }
}

#[test]
fn avatar_theme_contract_consumes_ui_theme_tokens_without_rebuilding_theme() {
    let theme_tokens = load_source("../ui-theme/src/tokens.rs");
    let theme_mapping = load_source("../ui-theme/src/theme.rs");
    let theme_css = load_source("../ui-theme/src/css.rs");
    let theme_baseline = load_source("../ui-theme/tests/token_scale_baseline.rs");
    let styling_spec = load_source("../../docs/spec/styling.md");
    let avatar_styles = load_source("src/avatar/styles.rs");
    let avatar_view = load_source("src/avatar/view.rs");
    let avatar_logic = load_source("src/avatar/logic.rs");

    for needle in [
        "pub struct ThemeTokens",
        "pub struct TypographyTokens",
        "pub struct OverlayLayoutTokens",
    ] {
        assert!(
            theme_tokens.contains(needle),
            "ui-theme token taxonomy should define `{needle}`."
        );
    }

    for needle in [
        "pub enum ThemeSystem",
        "pub enum ThemeColor",
        "pub enum ThemeScale",
    ] {
        assert!(
            theme_mapping.contains(needle),
            "ui-theme context mapping should define `{needle}`."
        );
    }

    for needle in [
        "--ui-bg: {};",
        "--ui-fg: {};",
        "--ui-border: {};",
        "--ui-font-size-100: {}px;",
        "--ui-font-size-150: {}px;",
        "--ui-overlay-panel-min-width: {}px;",
        "--ui-overlay-viewport-inset: {}px;",
    ] {
        assert!(
            theme_css.contains(needle),
            "ui-theme css output should emit `{needle}`."
        );
    }

    for needle in [
        "Token 统一基线落点固定",
        "crates/ui-theme/src/tokens.rs",
        "crates/ui-theme/src/theme.rs",
        "crates/ui-theme/src/css.rs",
        "crates/ui-theme/tests/token_scale_baseline.rs",
    ] {
        assert!(
            styling_spec.contains(needle),
            "styling spec should keep theme contract marker `{needle}`."
        );
    }

    assert!(
        theme_baseline.contains("token_scale_baselines_are_regression_testable"),
        "ui-theme baseline regression test should exist for scale/token contract."
    );

    for required_var in [
        "var(--ui-bg",
        "var(--ui-bg-muted",
        "var(--ui-border",
        "var(--ui-fg",
    ] {
        assert!(
            avatar_styles.contains(required_var),
            "Avatar styles should consume ui-theme semantic variables; missing `{required_var}`."
        );
    }

    for forbidden in [
        "Theme::",
        "ThemeContext",
        "theme_to_css_variables",
        "ui_theme::",
    ] {
        assert!(
            !avatar_view.contains(forbidden) && !avatar_logic.contains(forbidden),
            "Avatar component layer must not rebuild theme mapping; found `{forbidden}`."
        );
    }

    assert!(
        !avatar_styles.contains("--avatar-"),
        "Avatar styles should not introduce private non-`--ui-*` token namespace."
    );
}

#[test]
fn avatar_stays_as_ui_components_assembly_layer_without_platform_leakage() {
    let mod_source = load_source("src/avatar/mod.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let lib_source = load_source("src/lib.rs");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::AvatarSize;",
        "pub use view::Avatar;",
    ] {
        assert!(
            mod_source.contains(required),
            "Avatar module boundary should include `{required}`."
        );
    }

    for required in [
        "pub use ui_state_primitives::avatar::{",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(required),
            "Avatar logic should stay in assembly role and include `{required}`."
        );
    }

    for forbidden in ["view! {", "data-slot=", "on:error", "image_fallback_attrs("] {
        assert!(
            !logic_source.contains(forbidden),
            "Avatar logic must not carry view/headless wiring `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "logic::resolve_state(",
        "logic::compose_class_name(",
        "image_fallback_attrs(",
        "locale_attrs(",
    ] {
        assert!(
            view_source.contains(required),
            "Avatar view should compose logic + headless contract via `{required}`."
        );
    }

    for forbidden in [
        "ui_state_primitives::avatar::AvatarState {",
        "pub struct AvatarState",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Avatar view must not reimplement primitives; found `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("var(--ui-"),
        "Avatar styles should be token-first and consume `--ui-*` variables."
    );
    for forbidden in [
        "Theme::",
        "ThemeContext",
        "theme_to_css_variables(",
        "view! {",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Avatar styles should stay static and not rebuild theme/view via `{forbidden}`."
        );
    }

    for required in [
        "pub use avatar::{Avatar, AvatarSize};",
        "pub use avatar_group::{AvatarGroup, AvatarGroupItem};",
    ] {
        assert!(
            lib_source.contains(required),
            "ui-components public API should expose stable avatar exports via `{required}`."
        );
    }

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
fn avatar_public_api_naming_contract_is_stable_and_prefix_ready() {
    let view_source = load_source("src/avatar/view.rs");
    let sig = function_signature(&view_source, "Avatar");

    for required in [
        "name: Option<String>",
        "src: Option<String>",
        "size: AvatarSize",
        "alt: Option<String>",
        "class_name: Option<String>",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
    ] {
        assert!(
            sig.contains(required),
            "Avatar public API should keep stable prop naming `{required}`."
        );
    }

    assert!(
        !sig.contains(": bool"),
        "Avatar currently has no public boolean props; future booleans must use `is_*`."
    );
    assert!(
        !sig.contains("on_"),
        "Avatar currently has no public callbacks; future callbacks must use `on_*`."
    );
    assert!(
        !sig.contains("default_"),
        "Avatar currently has no public default-value props; future defaults must use `default_*`."
    );
}

#[test]
fn avatar_has_no_controllable_state_axis_and_no_half_controlled_api() {
    let view_source = load_source("src/avatar/view.rs");
    let sig = function_signature(&view_source, "Avatar");

    for forbidden in [" value:", "default_", "on_value_change", "on_open_change"] {
        assert!(
            !sig.contains(forbidden),
            "Avatar should not expose partial controllable API marker `{forbidden}` without full value/on_change/default pair."
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
            "Avatar has no controllable state axis and should not include `{forbidden}`."
        );
    }
}

#[test]
fn avatar_defaults_are_centralized_in_logic() {
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");

    for required in [
        "pub fn normalize_input(",
        "pub fn normalize_lang(",
        "let image_src = src.clone().unwrap_or_default();",
        "image_src,",
    ] {
        assert!(
            logic_source.contains(required),
            "Avatar logic should centralize default normalization via `{required}`."
        );
    }

    for forbidden in ["unwrap_or_default()", "normalize_optional_text("] {
        assert!(
            !view_source.contains(forbidden),
            "Avatar view should not perform fallback normalization directly; found `{forbidden}`."
        );
    }
}

#[test]
fn avatar_state_primitive_source_boundary_is_enforced() {
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let primitive_source = load_source("../ui-state-primitives/src/avatar.rs");
    let sig = function_signature(&view_source, "Avatar");

    for required in [
        "pub use ui_state_primitives::avatar::{",
        "AvatarStateInput",
        "AvatarState",
        "resolve_state",
        "AvatarImageRenderInput",
        "resolve_image_render_state",
    ] {
        assert!(
            logic_source.contains(required),
            "Avatar logic should source state primitives from ui-state-primitives via `{required}`."
        );
    }

    for required in [
        "pub struct AvatarImageRenderInput",
        "pub enum AvatarRenderMode",
        "pub struct AvatarImageRenderState",
        "pub fn resolve_image_render_state(",
    ] {
        assert!(
            primitive_source.contains(required),
            "Avatar image render primitives should be implemented in ui-state-primitives; missing `{required}`."
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
            "Avatar component layer should not bind business store directly; found `{forbidden}`."
        );
    }

    for forbidden in ["RwSignal<", "ReadSignal<", "WriteSignal<", "Signal<"] {
        assert!(
            !sig.contains(forbidden),
            "Avatar public API should not expose framework/store state container `{forbidden}`."
        );
    }
}

#[test]
fn avatar_state_normalization_is_centralized_in_logic() {
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");

    for required in ["AvatarImageRenderInput", "resolve_image_render_state"] {
        assert!(
            logic_source.contains(required),
            "Avatar logic should type and derive image/fallback render state via `{required}`."
        );
    }

    for forbidden in [
        "pub struct AvatarImageRenderInput {",
        "pub struct AvatarImageRenderState {",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Avatar logic should consume image render primitives from ui-state-primitives; found local `{forbidden}`."
        );
    }

    for required in [
        "logic::resolve_image_render_state(logic::AvatarImageRenderInput {",
        "on:error=move |_| img_error.set(true)",
    ] {
        assert!(
            view_source.contains(required),
            "Avatar view should consume centralized render state and only trigger signal updates via `{required}`."
        );
    }

    for forbidden in [
        "if state.has_src && !img_error.get()",
        "if img_error.get()",
        "data-state=move || if",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Avatar view should not rebuild state machine branches; found `{forbidden}`."
        );
    }

    for required in [
        ".ui-avatar[data-state=\"image\"]",
        ".ui-avatar[data-state=\"fallback\"]",
        ".ui-avatar[data-image=\"true\"]",
        ".ui-avatar[data-fallback=\"true\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "Avatar styles should consume explicit state markers via `{required}`."
        );
    }
}

#[test]
fn avatar_state_markers_are_observable_and_closed_set_contracts() {
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let primitive_source = load_source("../ui-state-primitives/src/avatar.rs");

    for required in [
        "data-slot=\"avatar\"",
        "data-state=move || render_state.get().mode.as_str()",
        "data-image=move || render_state.get().mode.image_attr()",
        "data-fallback=move || render_state.get().mode.fallback_attr()",
        "data-label-source=state.label_source.as_str()",
        "role=move || image_fallback_attrs(render_state.get().mode.shows_image(), aria_label.get_value()).role",
        "aria-label=move || image_fallback_attrs(render_state.get().mode.shows_image(), aria_label.get_value()).aria_label",
    ] {
        assert!(
            view_source.contains(required),
            "Avatar state/a11y markers should stay observable via `{required}`."
        );
    }

    for required in [
        "pub enum AvatarRenderMode",
        "Self::Image => \"image\"",
        "Self::Fallback => \"fallback\"",
        "pub enum AvatarLabelSource",
        "Self::Alt => \"alt\"",
        "Self::Name => \"name\"",
        "Self::Fallback => \"fallback\"",
    ] {
        assert!(
            primitive_source.contains(required),
            "Avatar marker values should come from enum closed set via `{required}`."
        );
    }

    for required in [
        ".ui-avatar[data-state=\"image\"]",
        ".ui-avatar[data-state=\"fallback\"]",
        ".ui-avatar[data-label-source=\"name\"]",
        ".ui-avatar[data-label-source=\"fallback\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "Avatar selectors should be queryable from semantic markers via `{required}`."
        );
    }

    for forbidden in [
        "data-state=move || format!",
        "data-label-source=move ||",
        ".ui-avatar:nth-child(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !styles_source.contains(forbidden),
            "Avatar marker contract should avoid free-text or DOM-order selector pattern `{forbidden}`."
        );
    }
}

#[test]
fn avatar_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let styles_source = load_source("src/avatar/styles.rs");
    let view_source = load_source("src/avatar/view.rs");

    for required in [
        ".ui-avatar[data-state=\"image\"]",
        ".ui-avatar[data-state=\"fallback\"]",
        ".ui-avatar[data-image=\"true\"]",
        ".ui-avatar[data-fallback=\"true\"]",
        ".ui-avatar[data-label-source=\"alt\"]",
        ".ui-avatar[data-label-source=\"name\"]",
        ".ui-avatar[data-label-source=\"fallback\"]",
        ".ui-avatar[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "Avatar state styles should rely on explicit markers via `{required}`."
        );
    }

    for forbidden in [
        ".ui-avatar:nth-child(",
        ".ui-avatar:nth-of-type(",
        ".ui-avatar > * > *",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Avatar state styling should not guess from fragile DOM selector `{forbidden}`."
        );
    }

    assert!(
        !view_source.contains("style="),
        "Avatar runtime should not inject business style logic inline."
    );
}

#[test]
fn avatar_semantics_suite_prioritizes_contract_assertions_over_snapshots() {
    let suite_source = load_source("tests/avatar_semantics.rs");

    for required in [
        "fn avatar_fallback_wires_accessible_name_contract()",
        "fn avatar_state_markers_are_observable_and_closed_set_contracts()",
        "fn avatar_styles_depend_on_explicit_state_markers_not_dom_guessing()",
        "fn avatar_state_normalization_is_centralized_in_logic()",
        "fn avatar_has_no_controllable_state_axis_and_no_half_controlled_api()",
        "fn avatar_has_no_async_loading_protocol_and_keeps_sync_error_fallback_contract()",
        "fn avatar_stays_static_and_delegates_motion_runtime_to_ui_motion_layer()",
    ] {
        assert!(
            suite_source.contains(required),
            "Avatar semantics suite should prioritize contract coverage via `{required}`."
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
            "Avatar semantics suite should not depend on snapshot token `{forbidden}`."
        );
    }
}

#[test]
fn avatar_component_files_follow_layered_responsibilities() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mod_source = load_source("src/avatar/mod.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let view_source = load_source("src/avatar/view.rs");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::AvatarSize;",
        "pub use view::Avatar;",
    ] {
        assert!(
            mod_source.contains(required),
            "Avatar `mod.rs` should keep minimal export boundary via `{required}`."
        );
    }

    for forbidden in ["view! {", "pub fn normalize_input(", "pub const CSS:"] {
        assert!(
            !mod_source.contains(forbidden),
            "Avatar `mod.rs` should not carry implementation detail `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_input(",
        "pub fn normalize_lang(",
        "pub fn compose_class_name(",
        "resolve_state,",
        "resolve_image_render_state,",
    ] {
        assert!(
            logic_source.contains(required),
            "Avatar `logic.rs` should keep normalization/derivation helpers via `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "data-slot=",
        "role=",
        "aria-label=",
        ".ui-avatar",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Avatar `logic.rs` should not mix view/css detail `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-avatar[data-state=\"image\"]",
        ".ui-avatar[data-state=\"fallback\"]",
        "var(--ui-",
    ] {
        assert!(
            styles_source.contains(required),
            "Avatar `styles.rs` should keep token-first static CSS via `{required}`."
        );
    }

    for forbidden in [
        "view! {",
        "on:error",
        "labeled_group_attrs(",
        "image_fallback_attrs(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Avatar `styles.rs` should not carry runtime/view logic `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "logic::normalize_input(",
        "logic::resolve_state(",
        "logic::compose_class_name(",
        "image_fallback_attrs(",
        "locale_attrs(",
        "data-slot=\"avatar\"",
    ] {
        assert!(
            view_source.contains(required),
            "Avatar `view.rs` should render structure and mount logic/headless contract via `{required}`."
        );
    }

    for forbidden in [
        "pub const CSS",
        "pub struct AvatarState {",
        "pub enum AvatarRenderMode {",
        "ui_motion::",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Avatar `view.rs` should not carry styles/primitive redefinition/motion engine detail `{forbidden}`."
        );
    }

    assert!(
        !manifest_dir
            .join("../../components/avatar/src/motion.rs")
            .exists(),
        "Avatar is static in current scope; `motion.rs` should remain absent until motion contract is required."
    );
}

#[test]
fn avatar_spec_rs_is_reserved_for_complex_schema_components_only() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let avatar_mod_source = load_source("src/avatar/mod.rs");
    let button_mod_source = load_source("src/button/mod.rs");
    let button_spec_source = load_source("src/button/spec.rs");

    assert!(
        !manifest_dir
            .join("../../components/avatar/src/spec.rs")
            .exists(),
        "Avatar is a simple component and must not introduce `src/avatar/spec.rs`."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !avatar_mod_source.contains(forbidden),
            "Avatar module boundary should stay lightweight and avoid spec wiring `{forbidden}`."
        );
    }

    for required in ["pub mod spec;", "pub use spec::{"] {
        assert!(
            button_mod_source.contains(required),
            "Complex button component should keep schema boundary in `button/spec.rs` via `{required}`."
        );
    }

    for required in [
        "pub const BUTTON_SCHEMA_VERSION: u16 = 1;",
        "pub struct ButtonSchema",
        "pub struct ButtonSpec",
    ] {
        assert!(
            button_spec_source.contains(required),
            "Schema-bearing contract should stay centralized in button spec via `{required}`."
        );
    }
}

#[test]
fn avatar_token_first_static_styles_contract_is_enforced_via_ui_root_css_pipeline() {
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let view_source = load_source("src/avatar/view.rs");
    let logic_source = load_source("src/avatar/logic.rs");

    for required in [
        "#[cfg(feature = \"component-avatar\")]",
        "out.push_str(crate::avatar::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "Component CSS aggregation should include avatar styles via `{required}`."
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
        "var(--ui-border)",
        "var(--ui-fg)",
    ] {
        assert!(
            styles_source.contains(required),
            "Avatar styles should stay token-first and static via `{required}`."
        );
    }

    for forbidden in [
        "--avatar-",
        "@apply",
        "tailwind",
        "styled(",
        "emotion",
        "stylex",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Avatar styles should not introduce private-token or CSS-in-Rust utility marker `{forbidden}`."
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
            "Avatar view should not depend on utility-first class contract `{forbidden}`."
        );
    }

    for forbidden in ["style=", "style:"] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Avatar runtime should not carry inline business style logic `{forbidden}`."
        );
    }
}

#[test]
fn avatar_visual_desire_gate_reuses_theme_visual_baseline_and_heroui_contracts() {
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
fn avatar_tree_shaking_contract_enforces_component_feature_gates_and_budgeted_ci() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let tree_shaking_script = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");
    let web_demo_cargo_source = load_source("../../apps/web-demo/Cargo.toml");

    for needle in [
        "component-avatar = [\"dep:ui-avatar\"]",
        "component-avatar_group = [\"component-avatar\", \"dep:ui-avatar-group\"]",
        "web-demo-components = [",
        "\"component-avatar\"",
        "\"component-avatar_group\"",
        "all-components = [",
        "\"component-avatar\"",
        "\"component-avatar_group\"",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components feature graph should keep avatar tree-shaking token `{needle}`."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-avatar\")]\npub use ui_avatar as avatar;"),
        "avatar module must stay feature-gated in lib.rs."
    );

    for needle in [
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]",
        "#[cfg(feature = \"all-components\")]",
        "pub use web_demo_components::*;",
        "pub use all_components::*;",
    ] {
        assert!(
            lib_source.contains(needle),
            "lib.rs should keep feature-bounded export surface token `{needle}`."
        );
    }

    let avatar_reexport_count = lib_source
        .matches("pub use avatar::{Avatar, AvatarSize};")
        .count();
    let avatar_group_reexport_count = lib_source
        .matches("pub use avatar_group::{AvatarGroup, AvatarGroupItem};")
        .count();
    assert_eq!(
        avatar_reexport_count, 2,
        "Avatar re-export should only exist inside gated feature bundles."
    );
    assert_eq!(
        avatar_group_reexport_count, 2,
        "AvatarGroup re-export should only exist inside gated feature bundles."
    );

    for needle in [
        "#[cfg(feature = \"component-avatar\")]\n    out.push_str(crate::avatar::styles::CSS);",
        "#[cfg(feature = \"component-avatar_group\")]\n    out.push_str(crate::avatar_group::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "css.rs should aggregate avatar CSS behind component feature gate `{needle}`."
        );
    }

    for forbidden in [
        "#[cfg(feature = \"all-components\")]\n    out.push_str(crate::avatar::styles::CSS);",
        "#[cfg(feature = \"all-components\")]\n    out.push_str(crate::avatar_group::styles::CSS);",
    ] {
        assert!(
            !css_source.contains(forbidden),
            "avatar CSS should not be tied to all-components-only aggregation `{forbidden}`."
        );
    }

    for needle in [
        "ui-components = { path = \"../../crates/ui-components\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }",
        "default-features = false",
        "\"web-demo-components\"",
    ] {
        assert!(
            web_demo_cargo_source.contains(needle),
            "web-demo should consume ui-components via minimal tree-shake-friendly dependency token `{needle}`."
        );
    }

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
            "Tree-shaking CI gate should include `{needle}`."
        );
    }
}

#[test]
fn avatar_machine_readable_contract_uses_typed_inputs_and_semantic_markers() {
    let primitive_source = load_source("../ui-state-primitives/src/avatar.rs");
    let logic_source = load_source("src/avatar/logic.rs");
    let view_source = load_source("src/avatar/view.rs");
    let styles_source = load_source("src/avatar/styles.rs");
    let logic_test_source = load_source("../../components/avatar/test/logic.rs");

    for required in [
        "pub enum AvatarSize",
        "pub enum AvatarLabelSource",
        "pub enum AvatarRenderMode",
        "pub struct AvatarStateInput",
        "pub struct AvatarImageRenderInput",
        "pub fn resolve_state(",
        "pub fn resolve_image_render_state(",
        "pub fn as_str(self) -> &'static str",
    ] {
        assert!(
            primitive_source.contains(required),
            "Avatar machine-readable input/state should stay typed in primitives via `{required}`."
        );
    }

    for required in [
        "pub use ui_state_primitives::avatar::{",
        "AvatarStateInput",
        "AvatarImageRenderInput",
        "resolve_state",
        "resolve_image_render_state",
    ] {
        assert!(
            logic_source.contains(required),
            "Avatar logic should consume typed primitives via `{required}`."
        );
    }

    for forbidden in [
        "pub enum AvatarRenderMode { Image(String)",
        "data-state=move || format!",
        "data-label-source=move || format!",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Avatar should avoid string-protocol state leakage `{forbidden}`."
        );
    }

    for required in [
        "data-state=move || render_state.get().mode.as_str()",
        "data-image=move || render_state.get().mode.image_attr()",
        "data-fallback=move || render_state.get().mode.fallback_attr()",
        "data-label-source=state.label_source.as_str()",
        "role=move || image_fallback_attrs(render_state.get().mode.shows_image(), aria_label.get_value()).role",
        "aria-label=move || image_fallback_attrs(render_state.get().mode.shows_image(), aria_label.get_value()).aria_label",
    ] {
        assert!(
            view_source.contains(required),
            "Avatar should expose machine-readable semantic markers via `{required}`."
        );
    }

    for required in [
        ".ui-avatar[data-state=\"image\"]",
        ".ui-avatar[data-state=\"fallback\"]",
        ".ui-avatar[data-label-source=\"alt\"]",
        ".ui-avatar[data-label-source=\"name\"]",
        ".ui-avatar[data-label-source=\"fallback\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "Avatar style contracts should consume stable semantic marker `{required}`."
        );
    }

    for required in [
        "resolve_image_render_state_tracks_image_and_fallback_markers",
        "resolve_state_tracks_size_source_and_flags",
    ] {
        assert!(
            logic_source.contains(required)
                || primitive_source.contains(required)
                || logic_test_source.contains(required),
            "Typed state contract should keep a regression anchor `{required}`."
        );
    }
}
