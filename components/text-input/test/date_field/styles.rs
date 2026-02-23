use super::*;
use std::{fs, path::Path};

fn read_workspace_file(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let file = workspace_root.join(rel_path);
    fs::read_to_string(&file).unwrap_or_else(|e| panic!("read_to_string failed for {file:?}: {e}"))
}

#[test]
fn date_field_styles_consume_shared_theme_tokens() {
    for token in [
        "var(--ui-space-2xs)",
        "var(--ui-space-3xs)",
        "var(--ui-fg)",
        "var(--ui-fg-muted)",
        "var(--ui-bg)",
        "var(--ui-border)",
        "var(--ui-radius-sm)",
        "var(--ui-radius-xs)",
        "var(--ui-disabled-opacity)",
    ] {
        assert!(
            CSS.contains(token),
            "DateField styles should consume shared theme token `{token}`."
        );
    }

    assert!(
        !CSS.contains("--ui-date-field-"),
        "DateField should not define a parallel private token namespace."
    );
}

#[test]
fn date_field_token_inputs_are_defined_by_ui_theme() {
    let tokens_source = read_workspace_file("crates/ui-theme/src/tokens.rs");
    let css_source = read_workspace_file("crates/ui-theme/src/css.rs");

    for contract in [
        "space_2xs_px",
        "disabled_opacity_percent",
        "--ui-space-2xs:",
        "--ui-disabled-opacity:",
        "--ui-bg:",
        "--ui-border:",
    ] {
        let exists = tokens_source.contains(contract) || css_source.contains(contract);
        assert!(
            exists,
            "Theme layer should expose `{contract}` as a stable token contract."
        );
    }
}

#[test]
fn date_field_styles_depend_on_explicit_state_markers_not_fragile_dom_shape() {
    for selector in [
        ".ui-date-field[data-tone=\"default\"]",
        ".ui-date-field[data-tone=\"quiet\"]",
        ".ui-date-field[data-tone=\"strong\"]",
        ".ui-date-field[data-disabled=\"true\"]",
        ".ui-date-field[data-has-value=\"true\"] .ui-date-field__control",
        ".ui-date-field[data-custom-class=\"true\"]",
    ] {
        assert!(
            CSS.contains(selector),
            "DateField styles should use explicit semantic selector `{selector}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":has(", "> .", "+ .", "~ ."] {
        assert!(
            !CSS.contains(forbidden),
            "DateField styles must not depend on fragile DOM-shape selector `{forbidden}`."
        );
    }
}

#[test]
fn date_field_view_avoids_runtime_business_inline_style_logic() {
    let view_source = read_workspace_file("components/text-input/src/date_field/view.rs");

    for forbidden in [" style=", "style =", "style:"] {
        assert!(
            !view_source.contains(forbidden),
            "DateField view should not encode business style logic via inline style marker `{forbidden}`."
        );
    }
}

#[test]
fn date_field_styles_are_aggregated_via_ui_css_and_ui_root_injection() {
    let ui_css_source = read_workspace_file("crates/ui/src/css.rs");
    let ui_root_source = read_workspace_file("crates/ui/src/root.rs");

    for needle in [
        "#[cfg(feature = \"component-date_field\")]",
        "out.push_str(crate::text_input::date_field::styles::CSS);",
    ] {
        assert!(
            ui_css_source.contains(needle),
            "ui css aggregation should include DateField styles marker `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "UiRoot should inject aggregated component css marker `{needle}`."
        );
    }
}

#[test]
fn date_field_component_avoids_utility_first_and_css_in_rust_defaults() {
    let view_source = read_workspace_file("components/text-input/src/date_field/view.rs");
    let styles_source = read_workspace_file("components/text-input/src/date_field/styles.rs");

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"gap-",
        "class=\"p-",
        "class=\"m-",
        "class=\"w-",
        "class=\"h-",
        "class=\"text-",
        "tw-",
        "tailwind",
        "stylex",
        "styled(",
        "css!(",
        "emotion",
    ] {
        assert!(
            !view_source.contains(forbidden) && !styles_source.contains(forbidden),
            "DateField component should not default to utility-first/CSS-in-Rust marker `{forbidden}`."
        );
    }
}

#[test]
fn date_field_default_theme_visual_desire_contract_is_present() {
    for needle in [
        ".ui-date-field {",
        "gap: var(--ui-space-2xs);",
        ".ui-date-field__label {",
        "font-size: var(--ui-button-size-s-font-size, 13px);",
        "line-height: var(--ui-button-size-s-line-height, 18px);",
        "font-weight: 600;",
        ".ui-date-field__control {",
        "border: 1px solid var(--ui-border);",
        "background: var(--ui-bg);",
        ".ui-date-field--tone-default,",
        ".ui-date-field--tone-strong,",
        ".ui-date-field--disabled,",
        ".ui-date-field__clear:hover,",
        ".ui-date-field__clear:focus-visible {",
        "border-color: color-mix(in oklab, var(--ui-accent) 45%, var(--ui-border) 55%);",
    ] {
        assert!(
            CSS.contains(needle),
            "DateField default-theme visual contract should include marker `{needle}`."
        );
    }

    for forbidden in ["bootstrap", ".btn", ".form-control", "btn-primary"] {
        assert!(
            !CSS.contains(forbidden),
            "DateField visual baseline should not regress to legacy-style marker `{forbidden}`."
        );
    }
}
