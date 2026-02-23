use super::*;
use crate::TextStateInput;

#[test]
fn class_and_attr_contracts_are_stable() {
    assert_eq!(TextTone::Default.class_name(), "ui-text--tone-default");
    assert_eq!(TextAlign::Center.class_name(), "ui-text--align-center");
    assert_eq!(TextWeight::Bold.class_name(), "ui-text--weight-bold");

    assert_eq!(TextTone::Subtle.as_attr(), "subtle");
    assert_eq!(TextAlign::Justify.as_attr(), "justify");
    assert_eq!(TextWeight::Medium.as_attr(), "medium");
}

#[test]
fn normalization_helpers_use_defaults() {
    assert_eq!(normalize_content(Some("  hello  ".to_string())), "hello");
    assert_eq!(normalize_content(Some("   ".to_string())), DEFAULT_TEXT);

    let (aria, is_custom) = normalize_aria_label(None);
    assert_eq!(aria, None);
    assert!(!is_custom);

    let (aria, is_custom) = normalize_aria_label(Some("  Summary text  ".to_string()));
    assert_eq!(aria.as_deref(), Some("Summary text"));
    assert!(is_custom);
}

#[test]
fn resolve_content_centralizes_default_priority() {
    let (content, source) = resolve_content(None, false);
    assert_eq!(content, DEFAULT_TEXT);
    assert_eq!(source, "default");

    let (content, source) = resolve_content(Some("  release notes  ".to_string()), false);
    assert_eq!(content, "release notes");
    assert_eq!(source, "text");

    let (content, source) = resolve_content(Some("   ".to_string()), false);
    assert_eq!(content, DEFAULT_TEXT);
    assert_eq!(source, "default");

    let (content, source) = resolve_content(Some("custom".to_string()), true);
    assert_eq!(content, "custom");
    assert_eq!(source, "children");
}

#[test]
fn slot_kind_contract_is_stable() {
    assert_eq!(resolve_slot_kind_attr(None), "none");
    assert_eq!(resolve_slot_kind_attr(Some("label")), "label");
    assert_eq!(resolve_slot_kind_attr(Some("DESCRIPTION")), "description");
    assert_eq!(resolve_slot_kind_attr(Some("icon")), "icon");
    assert_eq!(resolve_slot_kind_attr(Some("metadata")), "custom");
}

#[test]
fn resolve_state_tracks_sources_and_flags() {
    let state = resolve_state(TextStateInput {
        tone: TextTone::Strong,
        align: TextAlign::End,
        weight: TextWeight::Semibold,
        disabled: false,
        truncate: true,
        has_custom_aria_label: true,
        has_custom_class_name: false,
        slot_kind_attr: "description",
        has_named_slot: true,
    });

    assert_eq!(state.tone_attr, "strong");
    assert_eq!(state.align_attr, "end");
    assert_eq!(state.weight_attr, "semibold");
    assert_eq!(state.data_state_attr, "truncate");
    assert_eq!(state.aria_source_attr, "custom");
    assert_eq!(state.class_source_attr, "default");
    assert_eq!(state.slot_kind_attr, "description");
    assert!(state.has_named_slot);
}

#[test]
fn resolve_state_marks_absent_aria_label_source_as_none() {
    let state = resolve_state(TextStateInput {
        tone: TextTone::Default,
        align: TextAlign::Start,
        weight: TextWeight::Regular,
        disabled: false,
        truncate: false,
        has_custom_aria_label: false,
        has_custom_class_name: false,
        slot_kind_attr: "none",
        has_named_slot: false,
    });

    assert_eq!(state.aria_source_attr, "none");
}

#[test]
fn observable_marker_domains_stay_closed_and_enumerable() {
    for (content, has_children, expected_source) in [
        (None, false, "default"),
        (Some("  body  ".to_string()), false, "text"),
        (Some(" body ".to_string()), true, "children"),
        (Some("   ".to_string()), false, "default"),
    ] {
        let (_, source) = resolve_content(content, has_children);
        assert_eq!(source, expected_source);
    }

    for (slot, expected_kind) in [
        (None, "none"),
        (Some("label"), "label"),
        (Some("description"), "description"),
        (Some("icon"), "icon"),
        (Some("metadata"), "custom"),
    ] {
        assert_eq!(resolve_slot_kind_attr(slot), expected_kind);
    }

    for (disabled, truncate, expected_state) in [
        (false, false, "default"),
        (true, false, "disabled"),
        (false, true, "truncate"),
        (true, true, "disabled"),
    ] {
        let state = resolve_state(TextStateInput {
            tone: TextTone::Default,
            align: TextAlign::Start,
            weight: TextWeight::Regular,
            disabled,
            truncate,
            has_custom_aria_label: false,
            has_custom_class_name: false,
            slot_kind_attr: "none",
            has_named_slot: false,
        });

        assert_eq!(state.data_state_attr, expected_state);
        assert_eq!(state.aria_source_attr, "none");
        assert_eq!(state.class_source_attr, "default");
    }
}

#[test]
fn compose_class_name_includes_markers() {
    let class_name = compose_class_name(
        Some("docs-text".to_string()),
        resolve_state(TextStateInput {
            tone: TextTone::Subtle,
            align: TextAlign::Center,
            weight: TextWeight::Bold,
            disabled: true,
            truncate: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
            slot_kind_attr: "none",
            has_named_slot: false,
        }),
    );

    for token in [
        "ui-text",
        "ui-text--tone-subtle",
        "ui-text--align-center",
        "ui-text--weight-bold",
        "ui-text--disabled",
        "ui-text--truncate",
        "ui-text--custom-class",
        "docs-text",
    ] {
        assert!(class_name.contains(token), "class should include `{token}`");
    }
}

#[test]
fn text_stays_non_interactive_without_headless_dependency() {
    let view_source = include_str!("../view.rs");
    for forbidden in [
        "use ui_headless",
        "ui_headless::",
        "on:click",
        "on:keydown",
        "on:keyup",
        "on:input",
        "on:pointerdown",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Text view should stay display-only; found `{forbidden}`.",
        );
    }

    let cargo_toml = include_str!("../../Cargo.toml");
    assert!(
        !cargo_toml.contains("ui-headless"),
        "Display-only Text should not depend on `ui-headless`.",
    );
}

#[test]
fn text_stays_motionless_without_motion_dependency_or_executor() {
    let cargo_toml = include_str!("../../Cargo.toml");
    assert!(
        !cargo_toml.contains("ui-motion"),
        "Display-only Text should not depend on `ui-motion`.",
    );

    let mod_source = include_str!("../mod.rs");
    for forbidden in ["mod motion", "pub mod motion"] {
        assert!(
            !mod_source.contains(forbidden),
            "Text module should not expose motion module; found `{forbidden}`.",
        );
    }

    let styles_source = include_str!("../styles.rs");
    for forbidden in ["transition:", "animation:", "@keyframes"] {
        assert!(
            !styles_source.contains(forbidden),
            "Text styles should stay static and token-first; found `{forbidden}`.",
        );
    }
}

#[test]
fn text_consumes_shared_theme_tokens_without_local_theme_system() {
    let cargo_toml = include_str!("../../Cargo.toml");
    assert!(
        !cargo_toml.contains("ui-theme"),
        "Text should not implement theme runtime; it only consumes shared theme vars.",
    );

    let styles_source = include_str!("../styles.rs");
    for expected in [
        "var(--ui-font-size-150)",
        "var(--ui-line-height-150)",
        "var(--ui-fg)",
        "var(--ui-fg-muted)",
        "var(--ui-accent)",
    ] {
        assert!(
            styles_source.contains(expected),
            "Text styles should consume shared token `{expected}`.",
        );
    }

    for forbidden in [
        "--text-",
        "--ui-text-private-",
        ":root",
        "[data-theme",
        "color: #",
        "background: #",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Text styles should not rebuild theme layer or introduce private token contract `{forbidden}`.",
        );
    }
}
