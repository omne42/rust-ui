use super::*;

#[test]
fn normalize_bounds_clamps_and_recovers_invalid_range() {
    let bounds = normalize_bounds(-10.0, 130.0);
    assert_eq!(
        bounds,
        SplitBounds {
            min: 0.0,
            max: 100.0
        }
    );

    let recovered = normalize_bounds(95.0, 10.0);
    assert_eq!(
        recovered,
        SplitBounds {
            min: DEFAULT_MIN_SPLIT_PERCENT,
            max: DEFAULT_MAX_SPLIT_PERCENT,
        }
    );
}

#[test]
fn normalize_split_and_drag_result_respect_bounds() {
    let bounds = SplitBounds {
        min: 25.0,
        max: 75.0,
    };

    assert_eq!(normalize_split(None, bounds), 50.0);
    assert_eq!(normalize_split(Some(5.0), bounds), 25.0);
    assert_eq!(normalize_split(Some(80.0), bounds), 75.0);

    assert_eq!(split_from_drag(50.0, 100.0, 220.0, 400.0, bounds), 75.0);
    assert_eq!(split_from_drag(50.0, 220.0, 100.0, 400.0, bounds), 25.0);
}

#[test]
fn split_step_for_key_respects_orientation_and_acceleration() {
    assert_eq!(
        split_step_for_key("ArrowLeft", ResizableOrientation::Horizontal, false),
        Some(-2.0)
    );
    assert_eq!(
        split_step_for_key("ArrowRight", ResizableOrientation::Horizontal, true),
        Some(10.0)
    );
    assert_eq!(
        split_step_for_key("ArrowUp", ResizableOrientation::Vertical, false),
        Some(-2.0)
    );
    assert_eq!(
        split_step_for_key("ArrowDown", ResizableOrientation::Vertical, true),
        Some(10.0)
    );
    assert_eq!(
        split_step_for_key("ArrowDown", ResizableOrientation::Horizontal, false),
        None
    );
}

#[test]
fn resolve_state_surfaces_markers() {
    let bounds = SplitBounds {
        min: 20.0,
        max: 80.0,
    };

    let state = resolve_state(ResizableStateInput {
        orientation: ResizableOrientation::Vertical,
        split_percent: 88.0,
        bounds,
        disabled: false,
        dragging: true,
        is_controlled: true,
        with_handle: true,
        has_custom_class_name: true,
    });

    assert_eq!(state.orientation_attr, "vertical");
    assert_eq!(state.split_percent, 80.0);
    assert_eq!(state.state_attr, "dragging");
    assert_eq!(state.handle_attr, "with-handle");
    assert_eq!(state.class_source_attr, "custom");
    assert!(state.enabled);
    assert!(!state.idle);
    assert!(state.is_controlled);
    assert!(!state.is_uncontrolled);
}
