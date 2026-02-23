use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(suffix) = rel_path.strip_prefix("src/text_input/text/") {
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/text/src/{suffix}"));
        return fs::read_to_string(&migrated)
            .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn text_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/text_input/text/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Text internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn text_uses_logic_state_model() {
    let logic_source = load_source("src/text_input/text/logic.rs");
    let view_source = load_source("src/text_input/text/view.rs");

    for needle in [
        "pub enum TextTone",
        "pub enum TextAlign",
        "pub enum TextWeight",
        "pub enum TextElement",
        "pub fn normalize_optional_text(",
        "pub fn normalize_content(",
        "pub fn resolve_content(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
        "data_state_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Text logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::resolve_content(text, children.is_some())",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(TextStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
    ] {
        assert!(
            view_source.contains(needle),
            "Text view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn text_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/text_input/text/view.rs");

    for attr in [
        "data-slot=\"text\"",
        "data-tone=move || state.get().tone_attr",
        "data-align=move || state.get().align_attr",
        "data-weight=move || state.get().weight_attr",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-truncate=move || state.get().is_truncated.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Text should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn text_state_markers_are_stable_and_closed_surface_only() {
    let source = load_source("src/text_input/text/view.rs");

    for required in [
        "data-state=move || state.get().data_state_attr",
        "data-content-source=content_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-slot-kind=move || state.get().slot_kind_attr",
    ] {
        assert!(
            source.contains(required),
            "Text should expose stable marker `{required}` for state/source querying.",
        );
    }

    let forbidden = "data-slot-name=move || slot.get_value()";
    assert!(
        !source.contains(forbidden),
        "Text should avoid open-ended marker value surface `{forbidden}`.",
    );
}

#[test]
fn text_styles_include_tone_align_weight_and_markers() {
    let source = load_source("src/text_input/text/styles.rs");

    for selector in [
        ".ui-text--tone-default",
        ".ui-text[data-tone=\"default\"]",
        ".ui-text--tone-subtle",
        ".ui-text--tone-strong",
        ".ui-text--align-start",
        ".ui-text[data-align=\"start\"]",
        ".ui-text--align-center",
        ".ui-text--align-end",
        ".ui-text--align-justify",
        ".ui-text--weight-regular",
        ".ui-text[data-weight=\"regular\"]",
        ".ui-text--weight-medium",
        ".ui-text--weight-semibold",
        ".ui-text--weight-bold",
        ".ui-text--disabled",
        ".ui-text[data-disabled=\"true\"]",
        ".ui-text--truncate",
        ".ui-text[data-truncate=\"true\"]",
        ".ui-text--custom-class",
        ".ui-text[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Text styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn text_styles_depend_on_explicit_state_markers_without_structural_guessing() {
    let styles_source = load_source("src/text_input/text/styles.rs");
    let view_source = load_source("src/text_input/text/view.rs");

    for required in [
        ".ui-text--tone-default",
        ".ui-text[data-tone=\"default\"]",
        ".ui-text--align-start",
        ".ui-text[data-align=\"start\"]",
        ".ui-text--weight-regular",
        ".ui-text[data-weight=\"regular\"]",
        ".ui-text--disabled",
        ".ui-text[data-disabled=\"true\"]",
        ".ui-text--truncate",
        ".ui-text[data-truncate=\"true\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "Text styles should keep explicit state selector `{required}`.",
        );
    }

    for forbidden in [
        ":nth-child",
        ":first-child",
        ":last-child",
        ".ui-text .",
        ".ui-text >",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Text styles should not rely on structural selector guessing `{forbidden}`.",
        );
    }

    for forbidden in ["style=", "style:"] {
        assert!(
            !view_source.contains(forbidden),
            "Text view should not encode runtime business style logic via `{forbidden}`.",
        );
    }
}

#[test]
fn text_public_api_stays_platform_agnostic() {
    let mod_source = load_source("src/text_input/text/mod.rs");
    let view_source = load_source("src/text_input/text/view.rs");

    for required in ["pub use view::Text;", "pub use logic::{"] {
        assert!(
            mod_source.contains(required),
            "Text module should keep stable public exports, missing `{required}`.",
        );
    }

    for forbidden in [
        "web_sys",
        "web-sys",
        "wasm_bindgen",
        "NodeRef",
        "HtmlElement",
        "window(",
        "document(",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "Text public API/view should not expose platform DOM detail `{forbidden}`.",
        );
    }

    assert!(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/test/semantics.rs")
            .exists(),
        "Text must keep semantics test file at `src/test/semantics.rs`."
    );
    assert!(
        mod_source.contains("#[path = \"test/semantics.rs\"]"),
        "Text crate should wire semantics tests through mod.rs.",
    );
}

#[test]
fn text_boolean_props_follow_is_prefix_contract() {
    let view_source = load_source("src/text_input/text/view.rs");

    for required in [
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_truncated: bool",
    ] {
        assert!(
            view_source.contains(required),
            "Text public bool prop contract should include `{required}`.",
        );
    }

    for forbidden in [
        "#[prop(optional)] disabled: bool",
        "#[prop(optional)] truncate: bool",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Legacy bool prop name should not remain in public API: `{forbidden}`.",
        );
    }
}

#[test]
fn text_discrete_axes_use_enums_not_stringly_typed_props() {
    let view_source = load_source("src/text_input/text/view.rs");

    for required in [
        "#[prop(optional)] tone: TextTone",
        "#[prop(optional)] align: TextAlign",
        "#[prop(optional)] weight: TextWeight",
        "#[prop(optional)] element: TextElement",
    ] {
        assert!(
            view_source.contains(required),
            "Text discrete state axis must be enum-typed; missing `{required}`.",
        );
    }

    for forbidden in [
        "#[prop(optional)] tone: String",
        "#[prop(optional)] align: String",
        "#[prop(optional)] weight: String",
        "#[prop(optional)] element: String",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Text discrete state axis must not be stringly-typed: `{forbidden}`.",
        );
    }
}

#[test]
fn text_has_no_controlled_uncontrolled_state_axis_surface() {
    let view_source = load_source("src/text_input/text/view.rs");
    for forbidden in [
        "on_value_change",
        "default_value",
        "default_open",
        "#[prop(optional)] value:",
        "#[prop(optional)] on_",
        "#[prop(optional)] default_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Display-only Text should not expose controlled/uncontrolled state axis API `{forbidden}`.",
        );
    }
}

#[test]
fn text_has_no_async_interaction_protocol_surface() {
    let cargo_toml = load_source("Cargo.toml");
    let view_source = load_source("src/text_input/text/view.rs");

    for forbidden in [
        "#[prop(optional)] is_loading:",
        "#[prop(optional)] loading:",
        "#[prop(optional)] error:",
        "#[prop(optional)] on_retry:",
        "#[prop(optional)] on_error:",
        "aria-busy",
        "data-loading",
        "data-error",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Display-only Text should not expose async interaction protocol `{forbidden}`.",
        );
    }

    for forbidden in ["tokio", "reqwest", "gloo-net", "wasm-bindgen-futures"] {
        assert!(
            !cargo_toml.contains(forbidden),
            "Display-only Text should not add async transport/runtime dependency `{forbidden}`.",
        );
    }
}

#[test]
fn text_dx_default_path_is_state_free_and_docs_minimal() {
    let view_source = load_source("src/text_input/text/view.rs");

    for required in [
        "#[prop(optional, into)] text: Option<String>",
        "#[prop(optional)] tone: TextTone",
        "#[prop(optional)] align: TextAlign",
        "#[prop(optional)] weight: TextWeight",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_truncated: bool",
        "#[prop(optional)] element: TextElement",
        "#[prop(optional)] children: Option<Children>",
    ] {
        assert!(
            view_source.contains(required),
            "Text default API should keep optional prop `{required}` for minimal usage.",
        );
    }

    for forbidden in [
        "#[prop(into)] state:",
        "#[prop(optional)] state:",
        "#[prop(into)] machine:",
        "ui_state_primitives::",
        "ui_headless::",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Text should not require internal state wiring in basic API `{forbidden}`.",
        );
    }

    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra/text.rs");
    for required in [
        "title=\"Hello World (Default API)\"",
        "r#\"<Text text=\\\"Primary body copy\\\".into() />\"#",
        "<Text text=\"Primary body copy\".to_string() />",
    ] {
        assert!(
            docs_source.contains(required),
            "Text docs should expose minimal default path marker `{required}`.",
        );
    }

    let showcase_pos = docs_source
        .find("let showcase_code")
        .expect("Text docs should define `showcase_code` snippet.");
    let showcase_section = &docs_source[showcase_pos..];
    let marker = "Signal::derive(move || r#\"";
    let snippet_start = showcase_section
        .find(marker)
        .expect("Text docs should keep showcase snippet in raw string form.");
    let snippet_rest = &showcase_section[snippet_start + marker.len()..];
    let snippet_end = snippet_rest
        .find("\"#.to_string());")
        .expect("Text docs showcase snippet should end with `\"#.to_string());`.");
    let snippet = &snippet_rest[..snippet_end];
    let non_empty_lines = snippet
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    assert!(
        non_empty_lines <= 5,
        "Text Hello World snippet should stay within 5 non-empty lines, got {non_empty_lines}.",
    );
}

#[test]
fn text_non_composite_api_avoids_parallel_slot_pairing_contracts() {
    let view_source = load_source("src/text_input/text/view.rs");

    assert!(
        view_source.contains("#[prop(optional)] children: Option<Children>"),
        "Text should keep a single explicit content slot for display-only composition.",
    );

    for forbidden in [
        "#[prop(optional, into)] labels:",
        "#[prop(optional, into)] titles:",
        "#[prop(optional)] panels:",
        "#[prop(optional)] items:",
        "ItemSpec",
        "Vec<Children>",
        "Vec<String>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Display-only Text should not expose parallel-array or item-pairing API `{forbidden}`.",
        );
    }
}

#[test]
fn text_has_no_collection_registration_protocol_surface() {
    let mod_source = load_source("src/text_input/text/mod.rs");
    let logic_source = load_source("src/text_input/text/logic.rs");
    let view_source = load_source("src/text_input/text/view.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "Display-only Text should not expose collection registration protocol `{forbidden}`.",
        );
    }
}

#[test]
fn text_has_no_slot_projection_lifecycle_surface() {
    let mod_source = load_source("src/text_input/text/mod.rs");
    let logic_source = load_source("src/text_input/text/logic.rs");
    let view_source = load_source("src/text_input/text/view.rs");

    for forbidden in ["Lazy", "KeepAlive", "Eager", "NotifyHidden"] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "Display-only Text should not expose slot projection lifecycle contract `{forbidden}`.",
        );
    }
}

#[test]
fn text_has_no_env_stream_subscription_surface() {
    let logic_source = load_source("src/text_input/text/logic.rs");
    let view_source = load_source("src/text_input/text/view.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "BreakpointChanged",
        "on:resize",
        "on:scroll",
        "add_event_listener",
        "debounce",
        "throttle",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Display-only Text should not expose env-stream subscription surface `{forbidden}`.",
        );
    }
}

#[test]
fn text_has_no_event_light_cone_surface() {
    let logic_source = load_source("src/text_input/text/logic.rs");
    let view_source = load_source("src/text_input/text/view.rs");

    for forbidden in [
        "Context Bus",
        "ContextBus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "select_all",
        "prop drilling",
        "Table",
        "Grid",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Display-only Text should not expose event-light-cone collection surface `{forbidden}`.",
        );
    }
}

#[test]
fn text_has_no_causality_bus_trace_surface() {
    let mod_source = load_source("src/text_input/text/mod.rs");
    let logic_source = load_source("src/text_input/text/logic.rs");
    let view_source = load_source("src/text_input/text/view.rs");

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "causality_bus",
        "broadcast",
        "subscribe",
        "publish",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "Display-only Text should not expose causality-bus trace surface `{forbidden}`.",
        );
    }
}

#[test]
fn text_has_no_macro_micro_dragging_state_machine_surface() {
    let mod_source = load_source("src/text_input/text/mod.rs");
    let logic_source = load_source("src/text_input/text/logic.rs");
    let view_source = load_source("src/text_input/text/view.rs");

    for forbidden in [
        "Dragging",
        "DragStart",
        "DragMove",
        "DragEnd",
        "Action::DragEnd",
        "on:drag",
        "on:pointermove",
        "requestAnimationFrame",
        "raf",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "Display-only Text should not define macro/micro dragging state machine surface `{forbidden}`.",
        );
    }
}

#[test]
fn text_has_no_two_pass_geometry_measurement_surface() {
    let logic_source = load_source("src/text_input/text/logic.rs");
    let view_source = load_source("src/text_input/text/view.rs");

    for forbidden in [
        "Intent",
        "Measure",
        "Rectification",
        "getBoundingClientRect",
        "clientWidth",
        "clientHeight",
        "offsetWidth",
        "offsetHeight",
        "scrollWidth",
        "scrollHeight",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Display-only Text should not expose two-pass geometry measurement surface `{forbidden}`.",
        );
    }
}

#[test]
fn text_state_source_stays_decoupled_from_business_store_bindings() {
    let cargo_toml = load_source("Cargo.toml");
    let logic_source = load_source("src/text_input/text/logic.rs");
    let view_source = load_source("src/text_input/text/view.rs");

    for forbidden in ["ui-state-primitives", "ui_state_primitives"] {
        assert!(
            !cargo_toml.contains(forbidden),
            "Display-only Text should not introduce state-primitive dependency `{forbidden}` without a real reusable state axis.",
        );
    }

    for forbidden in [
        "::store::",
        "use_context::<",
        "provide_context(",
        "create_rw_signal(",
        "RwSignal<",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Text should not bind directly to business/global store path `{forbidden}`.",
        );
    }

    assert!(
        logic_source.contains("pub fn resolve_state(input: TextStateInput) -> TextState"),
        "Text logic should remain a pure assembly/mapping layer from `TextStateInput` to `TextState`.",
    );
}

#[test]
fn text_view_does_not_rebuild_state_machine_rules() {
    let view_source = load_source("src/text_input/text/view.rs");
    assert!(
        view_source.contains("logic::resolve_state(TextStateInput {"),
        "Text view should consume centralized state derivation via `logic::resolve_state`.",
    );

    for forbidden in [
        "let data_state_attr =",
        "if is_disabled",
        "if is_truncated",
        "if disabled",
        "if truncate",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Text view should not rebuild state-machine branch `{forbidden}`.",
        );
    }
}

#[test]
fn text_styles_use_typography_tokens_for_default_metrics() {
    let source = load_source("src/text_input/text/styles.rs");

    for needle in [
        "--ui-text-font-size: var(--ui-font-size-150);",
        "--ui-text-line-height: var(--ui-line-height-150);",
        "font-size: var(--ui-text-font-size);",
        "line-height: var(--ui-text-line-height);",
    ] {
        assert!(
            source.contains(needle),
            "Text styles should include tokenized typography metric `{needle}`."
        );
    }

    for forbidden in ["font-size: 0.9rem;", "line-height: 1.5;"] {
        assert!(
            !source.contains(forbidden),
            "Text styles should not keep legacy hardcoded metric `{forbidden}`."
        );
    }
}

#[test]
fn text_a11y_i18n_l10n_contract_has_locale_entrypoints_without_hardcoded_view_copy() {
    let mod_source = load_source("src/text_input/text/mod.rs");
    let logic_source = load_source("src/text_input/text/logic.rs");
    let view_source = load_source("src/text_input/text/view.rs");

    for required in [
        "TextDirection",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<TextDirection>",
        "lang=lang",
        "dir=dir",
        "logic::normalize_aria_label(aria_label)",
        "data-aria-source=move || state.get().aria_source_attr",
    ] {
        assert!(
            mod_source.contains(required)
                || logic_source.contains(required)
                || view_source.contains(required),
            "Text should expose locale/i18n a11y contract piece `{required}`.",
        );
    }

    for forbidden in [
        "pub const DEFAULT_ARIA_LABEL",
        "aria-label=\"Text\"",
        "aria-label=\"文本\"",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Text should not hardcode user-facing a11y copy `{forbidden}`.",
        );
    }

    assert!(
        logic_source.contains(
            "pub fn normalize_aria_label(value: Option<String>) -> (Option<String>, bool)"
        ),
        "Text a11y label normalization should preserve external override and allow absent fallback.",
    );
}

#[test]
fn text_semantics_contract_tests_cover_applicable_matrix_without_snapshot_dependency() {
    let semantics_source = load_source("src/text_input/text/test/semantics.rs");
    let logic_tests_source = load_source("src/text_input/text/test/logic.rs");
    let view_source = load_source("src/text_input/text/view.rs");

    for required in [
        "text_emits_baseline_style_state_data_attributes",
        "text_state_markers_are_stable_and_closed_surface_only",
        "text_a11y_i18n_l10n_contract_has_locale_entrypoints_without_hardcoded_view_copy",
        "text_has_no_controlled_uncontrolled_state_axis_surface",
        "text_has_no_async_interaction_protocol_surface",
        "text_public_api_stays_platform_agnostic",
        "text_styles_depend_on_explicit_state_markers_without_structural_guessing",
    ] {
        assert!(
            semantics_source.contains(required),
            "Text semantic test matrix should include `{required}`.",
        );
    }

    let required = "text_stays_non_interactive_without_headless_dependency";
    assert!(
        logic_tests_source.contains(required),
        "Text interaction-path N/A guard should include `{required}`.",
    );

    let forbidden_markers = [
        format!("{}{}", "assert_", "snapshot!"),
        format!("{}{}{}", "assert_", "debug_", "snapshot!"),
        format!("{}{}{}", "assert_", "yaml_", "snapshot!"),
        format!("{}{}", "insta", "::"),
        format!("{}{}", "snap", "box"),
    ];
    for forbidden in forbidden_markers {
        assert!(
            !semantics_source.contains(&forbidden) && !logic_tests_source.contains(&forbidden),
            "Text tests should not rely on snapshot-only assertion `{forbidden}`.",
        );
    }

    for required in [
        "data-state=move || state.get().data_state_attr",
        "data-content-source=content_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "Text semantic contract should expose queryable marker `{required}`.",
        );
    }

    for forbidden in ["on:keydown", "on:keyup", "on:pointerdown", "on:pointermove"] {
        assert!(
            !view_source.contains(forbidden),
            "Display-only Text should keep keyboard/pointer interaction path N/A by avoiding `{forbidden}`.",
        );
    }
}

#[test]
fn text_component_file_responsibilities_stay_layered_and_motion_is_na() {
    let mod_source = load_source("src/text_input/text/mod.rs");
    let logic_source = load_source("src/text_input/text/logic.rs");
    let styles_source = load_source("src/text_input/text/styles.rs");
    let view_source = load_source("src/text_input/text/view.rs");
    let cargo_toml = load_source("Cargo.toml");

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Text;",
    ] {
        assert!(
            mod_source.contains(required),
            "Text mod.rs should keep minimal export boundary marker `{required}`.",
        );
    }

    for forbidden in [
        "pub mod view;",
        "pub mod logic;",
        "mod motion",
        "pub mod motion",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "Text mod.rs should not leak internal/motion module `{forbidden}`.",
        );
    }

    for forbidden in [
        "view!",
        "<div",
        "<span",
        "class=",
        "style=",
        "on:click",
        "on:keydown",
        "on:pointerdown",
        "data-",
        "aria-",
        "window(",
        "document(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Text logic.rs should stay normalization/derivation-only without `{forbidden}`.",
        );
    }

    for required in [".ui-text {", "var(--ui-"] {
        assert!(
            styles_source.contains(required),
            "Text styles.rs should keep token-first static CSS marker `{required}`.",
        );
    }

    for forbidden in [
        "#[component]",
        "view!",
        "children()",
        "on:",
        "aria-label=",
        "data-state=",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Text styles.rs should not contain rendering/interaction semantics `{forbidden}`.",
        );
    }

    for required in [
        "logic::resolve_content(text, children.is_some())",
        "logic::resolve_state(TextStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(required),
            "Text view.rs should stay structure + semantic mount surface `{required}`.",
        );
    }

    for forbidden in ["ui_motion", "requestAnimationFrame", "@keyframes", "spring"] {
        assert!(
            !view_source.contains(forbidden),
            "Text view.rs should not embed motion engine concern `{forbidden}`.",
        );
    }

    assert!(
        !cargo_toml.contains("ui-motion"),
        "Display-only Text should keep motion.rs responsibility as N/A (no ui-motion dependency).",
    );
}

#[test]
fn text_simple_component_avoids_spec_rs_surface_sprawl() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mod_source = load_source("src/text_input/text/mod.rs");
    let protocol_source = load_source("src/text_input/text/protocol.rs");

    assert!(
        !manifest_dir.join("src/spec.rs").exists(),
        "Display-only Text should not introduce `spec.rs` surface for formality.",
    );

    for forbidden in ["mod spec", "pub mod spec", "#[path = \"spec.rs\"]"] {
        assert!(
            !mod_source.contains(forbidden),
            "Text mod.rs should not wire `spec.rs` marker `{forbidden}`.",
        );
    }

    assert!(
        protocol_source.contains("pub struct TextComponentSpec"),
        "Text schema contract should stay in protocol.rs instead of a dedicated spec.rs.",
    );
}

#[test]
fn text_token_first_static_style_contract_avoids_utility_and_css_in_rust_defaults() {
    let mod_source = load_source("src/text_input/text/mod.rs");
    let styles_source = load_source("src/text_input/text/styles.rs");
    let view_source = load_source("src/text_input/text/view.rs");
    let cargo_toml = load_source("Cargo.toml");

    for required in [
        "pub mod styles;",
        "pub const CSS: &str = r#\"",
        ".ui-text {",
    ] {
        assert!(
            mod_source.contains(required) || styles_source.contains(required),
            "Text should keep style contract marker `{required}`.",
        );
    }

    for required in [
        "var(--ui-font-size-150)",
        "var(--ui-line-height-150)",
        "var(--ui-fg)",
        "var(--ui-fg-muted)",
        "var(--ui-accent)",
    ] {
        assert!(
            styles_source.contains(required),
            "Text styles should consume shared ui token `{required}`.",
        );
    }

    for forbidden in ["@apply", "theme(", "tailwind", "utility-first"] {
        assert!(
            !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !cargo_toml.contains(forbidden),
            "Text should not bake utility-first styling contract `{forbidden}` into component layer.",
        );
    }

    for forbidden in [
        "stylist",
        "emotion",
        "linaria",
        "stylex",
        "styled-components",
        "vanilla-extract",
    ] {
        assert!(
            !cargo_toml.contains(forbidden),
            "Text should not adopt CSS-in-Rust default dependency `{forbidden}`.",
        );
    }

    for forbidden in ["style=", "style:"] {
        assert!(
            !view_source.contains(forbidden),
            "Text view should not move business style logic to runtime inline style `{forbidden}`.",
        );
    }
}

#[test]
fn text_tree_shaking_contract_is_feature_gated_in_ui_aggregation_layer() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let ui_cargo_toml_path = workspace_dir.join("crates/ui/Cargo.toml");
    let ui_lib_path = workspace_dir.join("crates/ui/src/lib.rs");
    let ui_css_path = workspace_dir.join("crates/ui/src/css.rs");

    let ui_cargo_toml = fs::read_to_string(&ui_cargo_toml_path)
        .unwrap_or_else(|e| panic!("read_to_string failed for {ui_cargo_toml_path:?}: {e}"));
    let ui_lib = fs::read_to_string(&ui_lib_path)
        .unwrap_or_else(|e| panic!("read_to_string failed for {ui_lib_path:?}: {e}"));
    let ui_css = fs::read_to_string(&ui_css_path)
        .unwrap_or_else(|e| panic!("read_to_string failed for {ui_css_path:?}: {e}"));

    for required in [
        "component-text = [\"dep:ui-text\"]",
        "ui-text = { path = \"../../components/text\", optional = true }",
    ] {
        assert!(
            ui_cargo_toml.contains(required),
            "ui Cargo feature graph should preserve text tree-shaking contract `{required}`.",
        );
    }

    for required in [
        "#[cfg(feature = \"component-text\")]\npub use ui_text as text;",
        "#[cfg(feature = \"all-components\")]\nmod all_components {",
        "#[cfg(feature = \"all-components\")]\npub use all_components::*;",
        "#[cfg(feature = \"inject-css\")]\n#[doc(hidden)]\npub fn push_components_css(out: &mut String) {",
        "css::push_components_css(out);",
    ] {
        assert!(
            ui_lib.contains(required),
            "ui lib export surface should keep feature gate `{required}`.",
        );
    }

    for required in [
        "#[cfg(feature = \"component-text\")]\n    out.push_str(crate::text::styles::CSS);",
        "#[cfg(feature = \"inject-css\")]",
    ] {
        assert!(
            ui_css.contains(required),
            "ui css aggregation should keep text feature gate `{required}`.",
        );
    }

    assert_eq!(
        ui_lib.matches("pub use ui_text as text;").count(),
        1,
        "ui lib should expose text via a single feature-gated re-export.",
    );
    assert_eq!(
        ui_css
            .matches("out.push_str(crate::text::styles::CSS);")
            .count(),
        1,
        "ui css should aggregate text styles exactly once behind feature gate.",
    );
}

#[test]
fn text_has_no_focus_stack_overlay_surface() {
    let mod_source = load_source("src/text_input/text/mod.rs");
    let logic_source = load_source("src/text_input/text/logic.rs");
    let view_source = load_source("src/text_input/text/view.rs");

    for forbidden in [
        "NodeRef",
        "FocusManager",
        "focus_manager",
        "FocusStack",
        "OverlayStack",
        "FallbackTo",
        "restore_focus",
        "focus_restore",
        "document.body",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "Display-only Text should not expose overlay focus-stack contract `{forbidden}`.",
        );
    }
}

#[test]
fn text_has_no_foreign_zone_escape_hatch_surface() {
    let mod_source = load_source("src/text_input/text/mod.rs");
    let logic_source = load_source("src/text_input/text/logic.rs");
    let view_source = load_source("src/text_input/text/view.rs");
    let cargo_toml = load_source("Cargo.toml");

    for forbidden in [
        "ForeignZone",
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "third_party",
        "third-party",
        "imperative",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !cargo_toml.contains(forbidden),
            "Display-only Text should not expose foreign-zone escape hatch surface `{forbidden}`.",
        );
    }
}

#[test]
fn text_has_no_hydration_discontinuity_surface() {
    let mod_source = load_source("src/text_input/text/mod.rs");
    let logic_source = load_source("src/text_input/text/logic.rs");
    let view_source = load_source("src/text_input/text/view.rs");
    let cargo_toml = load_source("Cargo.toml");

    for forbidden in [
        "now()",
        "Date.now",
        "SystemTime::now",
        "Instant::now",
        "uuid::",
        "Uuid",
        "new_v4",
        "rand::",
        "getrandom",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !cargo_toml.contains(forbidden),
            "Display-only Text should not introduce hydration-discontinuity source `{forbidden}`.",
        );
    }
}

#[test]
fn text_cross_platform_compile_contract_uses_explicit_cfg_and_keeps_non_wasm_browser_free() {
    let mod_source = load_source("src/text_input/text/mod.rs");
    let logic_source = load_source("src/text_input/text/logic.rs");
    let styles_source = load_source("src/text_input/text/styles.rs");
    let view_source = load_source("src/text_input/text/view.rs");
    let headless_lib = load_source("../../crates/ui-headless/src/lib.rs");
    let headless_cargo = load_source("../../crates/ui-headless/Cargo.toml");
    let ui_cargo = load_source("../../crates/ui/Cargo.toml");

    for forbidden in [
        "web_sys",
        "web-sys",
        "js_sys",
        "wasm_bindgen",
        "window(",
        "document(",
        "HtmlElement",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "Text non-wasm component files should stay browser-object free: `{forbidden}`.",
        );
    }

    for forbidden_branch in [
        "target_arch = \"wasm32\"",
        "feature = \"ssr\"",
        "feature = \"web\"",
    ] {
        assert!(
            !mod_source.contains(forbidden_branch)
                && !logic_source.contains(forbidden_branch)
                && !styles_source.contains(forbidden_branch)
                && !view_source.contains(forbidden_branch),
            "Text should not hide platform differences in component files via `{forbidden_branch}`.",
        );
    }

    assert!(
        headless_lib.contains(
            "#[cfg(all(feature = \"web\", feature = \"ssr\"))]\ncompile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");"
        ),
        "ui-headless must keep explicit web/ssr feature mutex compile_error guard.",
    );

    for required in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            headless_cargo.contains(required),
            "ui-headless feature contract should include `{required}`.",
        );
    }

    for required in [
        "[target.'cfg(target_arch = \"wasm32\")'.dependencies]",
        "web-sys =",
    ] {
        assert!(
            ui_cargo.contains(required),
            "ui crate should keep wasm-only dependency gating marker `{required}`.",
        );
    }
}
