use super::*;

#[test]
fn resolve_input_prefers_explicit_props_over_inherited_values() {
    let resolved = resolve_input(ActionButtonInputResolutionInput {
        is_disabled: Some(false),
        inherited_disabled: Some(true),
        size: Some(ActionButtonSize::L),
        inherited_size: Some(ActionButtonSize::S),
        is_quiet: Some(true),
        inherited_quiet: Some(false),
    });

    assert!(!resolved.is_disabled);
    assert_eq!(resolved.size, ActionButtonSize::L);
    assert!(resolved.is_quiet);
    assert_eq!(resolved.variant, ButtonVariant::Ghost);
}

#[test]
fn resolve_input_falls_back_to_inherited_or_defaults() {
    let resolved = resolve_input(ActionButtonInputResolutionInput {
        is_disabled: None,
        inherited_disabled: Some(true),
        size: None,
        inherited_size: None,
        is_quiet: None,
        inherited_quiet: None,
    });

    assert!(resolved.is_disabled);
    assert_eq!(resolved.size, ActionButtonSize::default());
    assert!(!resolved.is_quiet);
    assert_eq!(resolved.variant, ButtonVariant::Default);
}
