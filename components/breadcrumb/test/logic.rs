use super::*;

fn item(label: &str, href: Option<&str>) -> BreadcrumbItem {
    BreadcrumbItem {
        label: label.into(),
        href: href.map(ToString::to_string),
    }
}

#[test]
fn resolve_root_state_tracks_default_sources() {
    let root = resolve_root_state(None, None, None);

    assert_eq!(
        root.aria_label,
        ui_state_primitives::breadcrumbs::DEFAULT_ARIA_LABEL
    );
    assert_eq!(root.aria_source_attr, "default");
    assert_eq!(root.class_name, "ui-breadcrumb");
    assert_eq!(root.class_source_attr, "default");
}

#[test]
fn resolve_root_state_tracks_custom_sources() {
    let root = resolve_root_state(
        Some("  Docs trail  ".to_string()),
        Some("Navigation"),
        Some("  docs-breadcrumb  ".to_string()),
    );

    assert_eq!(root.aria_label, "Docs trail");
    assert_eq!(root.aria_source_attr, "custom");
    assert_eq!(root.class_name, "ui-breadcrumb docs-breadcrumb");
    assert_eq!(root.class_source_attr, "custom");
}

#[test]
fn resolve_root_state_uses_i18n_aria_label_when_prop_missing() {
    let root = resolve_root_state(None, Some("  Navigation trail  "), None);

    assert_eq!(root.aria_label, "Navigation trail");
    assert_eq!(root.aria_source_attr, "i18n");
    assert_eq!(root.class_name, "ui-breadcrumb");
    assert_eq!(root.class_source_attr, "default");
}

#[test]
fn resolve_separator_tracks_source_priority() {
    let custom_separator = resolve_separator(Some(" > ".to_string()), "/");
    assert_eq!(custom_separator.separator, ">");
    assert_eq!(custom_separator.separator_source_attr, "custom");

    let i18n_separator = resolve_separator(None, " / ");
    assert_eq!(i18n_separator.separator, "/");
    assert_eq!(i18n_separator.separator_source_attr, "i18n");

    let default_separator = resolve_separator(None, "   ");
    assert_eq!(default_separator.separator, DEFAULT_SEPARATOR);
    assert_eq!(default_separator.separator_source_attr, "default");
}

#[test]
fn state_source_attrs_are_closed_enumerations() {
    let roots = [
        resolve_root_state(None, None, None),
        resolve_root_state(Some("Docs".to_string()), Some("Navigation"), None),
        resolve_root_state(None, Some("Navigation"), Some("docs".to_string())),
    ];
    for root in roots {
        assert!(matches!(
            root.aria_source_attr,
            "custom" | "i18n" | "default"
        ));
        assert!(matches!(root.class_source_attr, "custom" | "default"));
    }

    let separators = [
        resolve_separator(Some(">".to_string()), "/"),
        resolve_separator(None, "/"),
        resolve_separator(None, "   "),
    ];
    for separator in separators {
        assert!(matches!(
            separator.separator_source_attr,
            "custom" | "i18n" | "default"
        ));
    }
}

#[test]
fn resolve_agent_contract_is_schema_typed_and_traceable() {
    let empty_state = resolve_state(&[]);
    let empty_contract = resolve_agent_contract(&empty_state, "default", "default", "default");
    assert_eq!(empty_contract.schema_name, "ui.breadcrumb.agent-contract");
    assert_eq!(empty_contract.schema_version.as_str(), "v1");
    assert_eq!(empty_contract.intent.as_str(), "trail-navigation");
    assert_eq!(empty_contract.action.as_str(), "navigate");
    assert_eq!(empty_contract.state.as_str(), "empty");
    assert_eq!(empty_contract.source.as_str(), "default-only");

    let linked_state = resolve_state(&[
        item("Home", Some("/")),
        item("Components", Some("/components")),
        item("Breadcrumb", None),
    ]);
    let linked_contract = resolve_agent_contract(&linked_state, "custom", "default", "i18n");
    assert_eq!(linked_contract.state.as_str(), "linked-trail");
    assert_eq!(linked_contract.source.as_str(), "mixed");
}

#[test]
fn resolve_agent_source_uses_whitelist_mapping() {
    let state = resolve_state(&[item("Home", Some("/")), item("Breadcrumb", None)]);
    let contract = resolve_agent_contract(&state, "javascript:alert(1)", "default", "default");
    assert_eq!(contract.source.as_str(), "mixed");
    assert!(!contract.source.as_str().contains("javascript:"));
    assert!(!contract.source.as_str().contains("<script"));
}

#[test]
fn resolve_agent_render_mode_is_limited_to_streaming_or_snapshot() {
    assert_eq!(BreadcrumbAgentRenderMode::Streaming.as_str(), "streaming");
    assert_eq!(BreadcrumbAgentRenderMode::Snapshot.as_str(), "snapshot");

    let state = resolve_state(&[item("Home", Some("/")), item("Breadcrumb", None)]);
    let contract = resolve_agent_contract(&state, "default", "default", "default");
    assert_eq!(contract.render_mode.as_str(), "snapshot");
}

#[test]
fn snapshot_baseline_consumes_complete_configuration_and_stays_stable() {
    let root = resolve_root_state(
        Some("  Docs trail  ".to_string()),
        Some("Navigation"),
        Some("  docs-breadcrumb  ".to_string()),
    );
    let separator = resolve_separator(Some(" > ".to_string()), "/");
    let items = vec![
        item("Home", Some("/")),
        item("Components", Some("/components")),
        item("Breadcrumb", None),
    ];
    let state = resolve_state(&items);
    let contract = resolve_agent_contract(
        &state,
        root.aria_source_attr,
        root.class_source_attr,
        separator.separator_source_attr,
    );

    assert_eq!(root.aria_label, "Docs trail");
    assert_eq!(root.class_name, "ui-breadcrumb docs-breadcrumb");
    assert_eq!(separator.separator, ">");
    assert_eq!(state.item_count, 3);
    assert!(state.has_current_page);
    assert_eq!(contract.render_mode.as_str(), "snapshot");
    assert_eq!(contract.intent.as_str(), "trail-navigation");
    assert_eq!(contract.action.as_str(), "navigate");
}

#[test]
fn resolve_streaming_policy_is_optional_with_snapshot_fallback_and_verified_output() {
    assert_eq!(BreadcrumbAgentStreamSupport::Required.as_str(), "required");
    assert_eq!(BreadcrumbAgentStreamSupport::Optional.as_str(), "optional");
    assert_eq!(BreadcrumbAgentStreamFallback::Snapshot.as_str(), "snapshot");
    assert_eq!(BreadcrumbAgentOutputStatus::Draft.as_str(), "draft");
    assert_eq!(BreadcrumbAgentOutputStatus::Verified.as_str(), "verified");
    assert_eq!(
        BreadcrumbAgentOutputStatus::Submittable.as_str(),
        "submittable"
    );

    let state = resolve_state(&[item("Home", Some("/")), item("Breadcrumb", None)]);
    let contract = resolve_agent_contract(&state, "default", "default", "default");
    assert_eq!(contract.stream_support.as_str(), "optional");
    assert_eq!(contract.stream_fallback.as_str(), "snapshot");
    assert_eq!(contract.output_status.as_str(), "verified");
}

#[test]
fn resolve_state_tracks_empty_and_count() {
    let state = resolve_state(&[]);
    assert_eq!(state.item_count, 0);
    assert!(state.is_empty);
    assert!(!state.has_items);
    assert!(!state.has_links);
    assert!(!state.has_current_page);
}

#[test]
fn resolve_state_tracks_links_on_non_last_items() {
    let state = resolve_state(&[
        item("Home", Some("/")),
        item("Components", Some("/components")),
        item("Breadcrumb", None),
    ]);

    assert_eq!(state.item_count, 3);
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert!(state.has_links);
    assert!(state.has_current_page);
}

#[test]
fn resolve_state_ignores_blank_and_last_item_links() {
    let state = resolve_state(&[item("Home", Some("   ")), item("Details", Some("/details"))]);

    assert_eq!(state.item_count, 2);
    assert!(!state.is_empty);
    assert!(state.has_items);
    assert!(!state.has_links);
    assert!(state.has_current_page);
}

#[test]
fn resolve_item_href_sanitizes_non_last_items() {
    assert_eq!(
        resolve_item_href(&item("Home", Some("  /docs  ")), 0, 2),
        Some("/docs".to_string())
    );
    assert_eq!(resolve_item_href(&item("Home", Some("  ")), 0, 2), None);
    assert!(!is_current_page(0, 2));
    assert!(is_current_page(1, 2));
    assert_eq!(
        resolve_item_href(&item("Current", Some("/current")), 1, 2),
        None
    );
}
