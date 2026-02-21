fn load_source(path: &str) -> &'static str {
    match path {
        "logic" => include_str!("../src/logic.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "mod" => include_str!("../src/mod.rs"),
        "readme" => include_str!("../src/README.md"),
        "primitive" => include_str!("../../../crates/ui-state-primitives/src/avatar.rs"),
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn avatar_component_uses_state_primitives_without_reimplementing_them() {
    let logic = load_source("logic");
    let primitive = load_source("primitive");

    for required in [
        "pub use ui_state_primitives::avatar::{",
        "AvatarStateInput",
        "AvatarState",
        "AvatarImageRenderInput",
        "resolve_state",
        "resolve_image_render_state",
    ] {
        assert!(
            logic.contains(required),
            "avatar logic should consume ui-state-primitives via `{required}`"
        );
    }

    for forbidden in [
        "pub struct AvatarStateInput {",
        "pub struct AvatarState {",
        "pub struct AvatarImageRenderInput {",
        "pub struct AvatarImageRenderState {",
        "pub enum AvatarRenderMode {",
    ] {
        assert!(
            !logic.contains(forbidden),
            "avatar logic should not reimplement state primitives: `{forbidden}`"
        );
    }

    for forbidden in ["RwSignal<", "ReadSignal<", "WriteSignal<", "Signal<"] {
        assert!(
            !primitive.contains(forbidden),
            "ui-state-primitives::avatar must remain framework-agnostic: `{forbidden}`"
        );
    }
}

#[test]
fn avatar_view_mounts_semantic_state_markers_from_logic() {
    let view = load_source("view");

    for required in [
        "let normalized = logic::normalize_input(name, src, alt, class_name);",
        "let state = logic::resolve_state(logic::AvatarStateInput {",
        "let render_state = Signal::derive(move || {",
        "logic::resolve_image_render_state(logic::AvatarImageRenderInput {",
        "data-slot=\"avatar\"",
        "data-ui-schema=move || agent_contract.get().schema",
        "data-intent=move || agent_contract.get().intent.as_str()",
        "data-action=move || agent_contract.get().action.as_str()",
        "data-source=move || agent_contract.get().source.as_str()",
        "data-state=move || render_state.get().mode.as_str()",
        "data-image=move || render_state.get().mode.image_attr()",
        "data-fallback=move || render_state.get().mode.fallback_attr()",
        "data-label-source=state.label_source.as_str()",
    ] {
        assert!(
            view.contains(required),
            "avatar view should mount stable semantic markers via `{required}`"
        );
    }

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !view.contains(forbidden),
            "avatar view should keep whitelist-safe render path without `{forbidden}`"
        );
    }
}

#[test]
fn avatar_module_keeps_public_surface_minimal() {
    let module = load_source("mod");
    assert!(
        !module.contains("pub mod logic"),
        "avatar internal logic module should not be publicly exposed"
    );
    assert!(
        !module.contains("pub mod view"),
        "avatar internal view module should not be publicly exposed"
    );
}

#[test]
fn avatar_snapshot_contract_accepts_complete_props_without_streaming_protocol() {
    let view = load_source("view");
    let module = load_source("mod");

    for required in [
        "pub fn Avatar(",
        "#[prop(optional, into)] name: Option<String>",
        "#[prop(optional, into)] src: Option<String>",
        "#[prop(optional)] size: AvatarSize",
        "#[prop(optional, into)] alt: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let normalized = logic::normalize_input(name, src, alt, class_name);",
        "let state = logic::resolve_state(logic::AvatarStateInput {",
    ] {
        assert!(
            view.contains(required),
            "avatar should consume complete snapshot props via `{required}`"
        );
    }

    for forbidden in [
        "is_streaming",
        "on_stream",
        "streaming_state",
        "token_delta",
        "chunk",
    ] {
        assert!(
            !view.contains(forbidden) && !module.contains(forbidden),
            "avatar should not expose streaming-only protocol token `{forbidden}`"
        );
    }
}

#[test]
fn avatar_streaming_optional_contract_is_snapshot_fallback_with_semantic_continuity() {
    let readme = load_source("readme");
    let view = load_source("view");

    for required in [
        "Streaming: Optional",
        "fallback=snapshot",
        "draft`/`verified`/`submittable",
        "role`/`aria-*`/`data-*",
    ] {
        assert!(
            readme.contains(required),
            "avatar README should document streaming boundary token `{required}`"
        );
    }

    for required in [
        "data-state=move || render_state.get().mode.as_str()",
        "data-label-source=state.label_source.as_str()",
        "role=move || image_fallback_attrs(render_state.get().mode.shows_image(), aria_label.get_value()).role",
        "aria-label=move || image_fallback_attrs(render_state.get().mode.shows_image(), aria_label.get_value()).aria_label",
    ] {
        assert!(
            view.contains(required),
            "avatar should keep semantic continuity marker `{required}`"
        );
    }

    for forbidden in [
        "is_streaming",
        "on_stream",
        "token_delta",
        "streaming_state",
    ] {
        assert!(
            !view.contains(forbidden),
            "avatar should not require streaming protocol token `{forbidden}`"
        );
    }
}

#[test]
fn avatar_non_test_sources_follow_rust_hygiene_contract() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let module = load_source("mod");

    for forbidden in [".unwrap(", ".unwrap_err(", ".expect(", "let _ ="] {
        assert!(
            !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden)
                && !module.contains(forbidden),
            "avatar non-test source should not contain rust hygiene violation `{forbidden}`"
        );
    }

    {
        let forbidden = "common.avatar_fallback_aria_label.as_ref().to_string()";
        assert!(
            !view.contains(forbidden),
            "avatar view should avoid string clone hotspot `{forbidden}`"
        );
    }

    assert!(
        view.contains("common.avatar_fallback_aria_label.as_ref().into()"),
        "avatar view should use low-churn conversion for fallback aria label"
    );
}

#[test]
fn avatar_semantics_cover_aria_data_and_explicit_non_interactive_focus_flow() {
    let view = load_source("view");

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
            view.contains(required),
            "avatar semantics should cover aria/data contract via `{required}`"
        );
    }

    for forbidden in [
        "tabindex=",
        "autofocus",
        "on:focus",
        "on:blur",
        "on:keydown",
        "on:keyup",
    ] {
        assert!(
            !view.contains(forbidden),
            "avatar is non-interactive and should keep focus flow explicit N/A without `{forbidden}`"
        );
    }
}

#[test]
fn avatar_performance_baseline_uses_static_render_equivalent_instead_of_runtime_render_count() {
    let view = load_source("view");
    let logic = load_source("logic");

    for forbidden in [
        "create_effect(",
        "create_render_effect(",
        "create_memo(",
        "spawn_local(",
        "set_timeout(",
        "set_interval(",
        "request_animation_frame(",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "avatar performance baseline should avoid render-churn primitive `{forbidden}`"
        );
    }

    assert_eq!(
        view.matches("Signal::derive(").count(),
        2,
        "avatar static render path should keep a stable two-derive baseline (render state + agent contract)"
    );
}
