use super::*;

#[test]
fn resolve_label_falls_back_to_default_label() {
    assert_eq!(resolve_label(Some("Share docs".to_string())), "Share docs");
    assert_eq!(resolve_label(None), "Share");
}

#[test]
fn resolve_label_uses_custom_fallback_when_provided() {
    assert_eq!(resolve_label_with_fallback(None, "Partager"), "Partager");
    assert_eq!(
        resolve_label_with_fallback(Some("Share docs".to_string()), "Partager"),
        "Share docs"
    );
}

#[test]
fn resolve_items_defaults_to_three_platforms() {
    let resolved = resolve_items(&[]);
    assert!(resolved.uses_default_items);
    assert_eq!(resolved.items.len(), 3);
    assert_eq!(resolved.items[0].label, "GitHub");
}

#[test]
fn resolve_items_deduplicates_platforms_and_normalizes_labels() {
    let resolved = resolve_items(&[
        ShareButtonItem::new(SharePlatform::Github, "  Repo  "),
        ShareButtonItem::new(SharePlatform::Github, "Ignore me"),
        ShareButtonItem::new(SharePlatform::X, "   "),
    ]);

    assert!(!resolved.uses_default_items);
    assert_eq!(resolved.items.len(), 2);
    assert_eq!(resolved.items[0].label, "Repo");
    assert_eq!(resolved.items[1].label, "X");
}

#[test]
fn resolve_items_uses_label_fallback_overrides() {
    let labels = SharePlatformLabels {
        github: "Repositorio",
        x: "Publicar",
        facebook: "Facebook ES",
    };
    let resolved = resolve_items_with_fallback(&[], labels);

    assert!(resolved.uses_default_items);
    assert_eq!(resolved.items.len(), 3);
    assert_eq!(resolved.items[0].label, "Repositorio");
    assert_eq!(resolved.items[1].label, "Publicar");
    assert_eq!(resolved.items[2].label, "Facebook ES");
}

#[test]
fn resolve_state_tracks_items_icon_placement_and_metadata() {
    let state = resolve_state(ShareButtonStateInput {
        provided_item_count: 0,
        resolved_item_count: 3,
        uses_default_items: true,
        icon_placement: ShareButtonIconPlacement::Prefix,
        has_custom_label: true,
        has_custom_class_name: true,
        has_custom_press_handler: true,
    });

    assert_eq!(state.provided_item_count, 0);
    assert_eq!(state.resolved_item_count, 3);
    assert!(state.has_items);
    assert_eq!(state.state_attr, "ready");
    assert!(state.uses_default_items);
    assert_eq!(state.items_source_attr, "default");
    assert_eq!(state.icon_placement_attr, "prefix");
    assert_eq!(state.icon_placement_class, "ui-share-button--icon-prefix");
    assert_eq!(state.label_source_attr, "custom");
    assert_eq!(state.handler_source_attr, "provided");
    assert!(state.has_custom_class_name);
}
