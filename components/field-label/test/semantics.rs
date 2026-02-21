use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path);
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed reading {path:?}: {err}"))
}

#[test]
fn component_surface_stays_ui_components_layer_only() {
    let module_source = load_source("src/mod.rs");

    assert!(
        module_source.contains("pub use view::FieldLabel;"),
        "component should export FieldLabel as the stable public API"
    );
    assert!(
        module_source.contains("pub use logic::{DEFAULT_ARIA_LABEL, DEFAULT_REQUIRED_INDICATOR, DEFAULT_TEXT, FieldLabelTone};"),
        "component should export primitive-backed tokenized API types"
    );
    assert!(
        !module_source.contains("web_sys"),
        "component public API must not expose web-sys types"
    );
}

#[test]
fn view_mounts_headless_contract_without_redefining_it() {
    let view_source = load_source("src/view.rs");

    for needle in [
        "use ui_headless::{A11yDirection, FieldLabelOptions, use_field_label};",
        "logic::derive_view_model(",
        "FieldLabelLogicInput {",
        "let logic::FieldLabelViewModel {",
        "let semantics = Memo::new(move |_| {",
        "<label",
        "use_field_label(FieldLabelOptions {",
        "for=for_id",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-disabled=move || semantics.get().attrs.aria_disabled",
        "lang=move || semantics.get().attrs.lang",
        "dir=move || semantics.get().attrs.dir",
        "data-ui-schema=logic::FIELD_LABEL_AGENT_SCHEMA",
        "data-ui-intent=logic::FieldLabelAgentIntent::Label.as_attr()",
        "data-ui-action=logic::FieldLabelAgentAction::SnapshotRender.as_attr()",
        "data-ui-streaming=logic::FieldLabelAgentStreaming::Optional.as_attr()",
        "data-ui-fallback=logic::FieldLabelAgentFallback::Snapshot.as_attr()",
        "data-ui-output-state=logic::FieldLabelAgentOutputState::Verified.as_attr()",
        "data-state=move || semantics.get().attrs.data_state",
        "data-required=move || semantics.get().attrs.data_required",
        "data-has-for=move || semantics.get().attrs.data_has_for",
    ] {
        assert!(
            view_source.contains(needle),
            "view should mount headless semantic contract field `{needle}`"
        );
    }

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "window()",
        "document()",
        "resolve_state(",
        "inner_html=",
        ".inner_html(",
        "tokio::",
        "async_std::",
        "tracing::",
        "attach_motion(",
        "ui_motion::",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view should keep platform details internal; found `{forbidden}`"
        );
    }
}

#[test]
fn component_manifest_and_rbi_projection_exist_and_track_api_surface() {
    let manifest = load_source("src/Component.toml");
    let rbi = load_source("src/field_label.rbi");

    for needle in [
        "name = \"FieldLabel\"",
        "crate = \"ui-field-label\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"agent_contract_schema_markers\"",
        "name = \"text\"",
        "name = \"is_required\"",
        "name = \"is_disabled\"",
        "name = \"tone\"",
        "name = \"required_indicator\"",
        "name = \"aria_label\"",
        "name = \"class_name\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "schema = \"field_label.v1\"",
        "intent = \"label\"",
        "action = \"snapshot_render\"",
        "streaming = \"optional\"",
        "fallback = \"snapshot\"",
        "output_state = \"verified\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-intent\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-streaming\"",
        "attr = \"data-ui-fallback\"",
        "attr = \"data-ui-output-state\"",
        "name = \"render_path\"",
        "inner_html",
        "<script",
        "retry",
        "reconnect",
        "transport_validation",
    ] {
        assert!(
            manifest.contains(needle),
            "Component.toml should include `{needle}`"
        );
    }

    for needle in [
        "pub type FieldLabelTone = ui_state_primitives::field_label::FieldLabelTone;",
        "pub const FIELD_LABEL_AGENT_SCHEMA: &str = \"field_label.v1\";",
        "pub enum FieldLabelAgentIntent {",
        "pub enum FieldLabelAgentAction {",
        "pub enum FieldLabelAgentStreaming {",
        "pub enum FieldLabelAgentFallback {",
        "pub enum FieldLabelAgentOutputState {",
        "pub fn FieldLabel(",
        "text: Option<String>",
        "for_id: Option<String>",
        "is_required: bool",
        "is_disabled: bool",
        "tone: FieldLabelTone",
        "required_indicator: Option<String>",
        "aria_label: Option<String>",
        "class_name: Option<String>",
        "lang: Option<String>",
        "dir: Option<ui_headless::A11yDirection>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi.contains(needle),
            "field_label.rbi should include `{needle}`"
        );
    }
}

#[test]
fn rust_hygiene_contract_no_unwrap_expect_or_let_underscore_and_cow_hotpath() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let protocol_source = load_source("src/protocol.rs");

    for source in [&logic_source, &view_source, &protocol_source] {
        for forbidden in [".unwrap(", ".expect(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "field_label non-test source should forbid `{forbidden}`"
            );
        }
    }

    assert!(
        logic_source.contains("use std::borrow::Cow;"),
        "logic should use Cow to reduce string clone churn."
    );
    assert!(
        logic_source.contains("Vec<Cow<'static, str>>"),
        "class name aggregation should be modeled with Cow<'static, str>."
    );
}
