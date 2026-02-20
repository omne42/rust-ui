use super::*;

#[test]
fn normalize_optional_text_filters_blank_values() {
    assert_eq!(normalize_optional_text(None), None);
    assert_eq!(normalize_optional_text(Some("\n\t".to_string())), None);
    assert_eq!(
        normalize_optional_text(Some("  Done  ".to_string())),
        Some("Done".to_string())
    );
}

#[test]
fn resolve_state_tracks_copy_and_label_sources() {
    let state = resolve_state(SnippetStateInput {
        is_multiline: true,
        has_text: true,
        has_label: true,
        is_copyable: true,
        has_custom_copied_label: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.layout, SnippetLayout::MultiLine);
    assert_eq!(state.copy_state, SnippetCopyState::Copyable);
    assert_eq!(state.copied_label_source, SnippetSource::Custom);
    assert_eq!(state.state_class, "ui-snippet--state-multiline");
    assert_eq!(state.copy_state_class, "ui-snippet--copyable");
    assert_eq!(
        state.copied_label_source_class,
        "ui-snippet--custom-copied-label"
    );
    assert!(!state.is_empty);
    assert!(state.has_label);
    assert!(state.is_copyable);
    assert!(state.copy_is_actionable);
    assert!(state.has_custom_class_name);
}

#[test]
fn resolve_state_marks_empty_copyable_snippet_as_disabled_copy() {
    let state = resolve_state(SnippetStateInput {
        is_multiline: false,
        has_text: false,
        has_label: false,
        is_copyable: true,
        has_custom_copied_label: false,
        has_custom_class_name: false,
    });

    assert_eq!(state.copy_state, SnippetCopyState::Disabled);
    assert_eq!(state.copy_state_attr, "disabled");
    assert!(state.is_empty);
    assert!(!state.copy_is_actionable);
}

#[test]
fn marker_values_are_closed_sets() {
    let allowed_layout = ["single-line", "multiline"];
    let allowed_copy = ["copyable", "disabled", "static"];
    let allowed_source = ["default", "custom"];

    for is_multiline in [false, true] {
        for has_text in [false, true] {
            for has_label in [false, true] {
                for is_copyable in [false, true] {
                    for has_custom_copied_label in [false, true] {
                        for has_custom_class_name in [false, true] {
                            let state = resolve_state(SnippetStateInput {
                                is_multiline,
                                has_text,
                                has_label,
                                is_copyable,
                                has_custom_copied_label,
                                has_custom_class_name,
                            });

                            assert!(allowed_layout.contains(&state.state_attr));
                            assert!(allowed_copy.contains(&state.copy_state_attr));
                            assert!(allowed_source.contains(&state.copied_label_source_attr));
                        }
                    }
                }
            }
        }
    }
}
