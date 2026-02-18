pub const DEFAULT_ARIA_LABEL: &str = "Resizable panels";
pub const DEFAULT_SPLIT_PERCENT: f64 = 50.0;
pub const DEFAULT_MIN_SPLIT_PERCENT: f64 = 10.0;
pub const DEFAULT_MAX_SPLIT_PERCENT: f64 = 90.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ResizableOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SplitBounds {
    pub min: f64,
    pub max: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResizableStateInput {
    pub orientation: ResizableOrientation,
    pub split_percent: f64,
    pub bounds: SplitBounds,
    pub disabled: bool,
    pub dragging: bool,
    pub is_controlled: bool,
    pub with_handle: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResizableState {
    pub orientation: ResizableOrientation,
    pub orientation_attr: &'static str,
    pub split_percent: f64,
    pub min_split_percent: f64,
    pub max_split_percent: f64,
    pub disabled: bool,
    pub enabled: bool,
    pub dragging: bool,
    pub idle: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub with_handle: bool,
    pub has_custom_class_name: bool,
    pub state_attr: &'static str,
    pub handle_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_bounds(min_split_percent: f64, max_split_percent: f64) -> SplitBounds {
    let min = min_split_percent.clamp(0.0, 100.0);
    let max = max_split_percent.clamp(0.0, 100.0);

    if min > max {
        return SplitBounds {
            min: DEFAULT_MIN_SPLIT_PERCENT,
            max: DEFAULT_MAX_SPLIT_PERCENT,
        };
    }

    SplitBounds { min, max }
}

pub fn clamp_split(split_percent: f64, bounds: SplitBounds) -> f64 {
    split_percent.clamp(bounds.min, bounds.max)
}

pub fn normalize_split(split_percent: Option<f64>, bounds: SplitBounds) -> f64 {
    clamp_split(
        split_percent.unwrap_or(DEFAULT_SPLIT_PERCENT),
        normalize_bounds(bounds.min, bounds.max),
    )
}

pub fn split_from_drag(
    start_split_percent: f64,
    start_position: f64,
    current_position: f64,
    container_extent: f64,
    bounds: SplitBounds,
) -> f64 {
    if container_extent <= 0.0 {
        return clamp_split(start_split_percent, bounds);
    }

    let delta_percent = ((current_position - start_position) / container_extent) * 100.0;
    clamp_split(start_split_percent + delta_percent, bounds)
}

pub fn split_step_for_key(
    key: &str,
    orientation: ResizableOrientation,
    accelerated: bool,
) -> Option<f64> {
    let step = if accelerated { 10.0 } else { 2.0 };

    match (orientation, key) {
        (ResizableOrientation::Horizontal, "ArrowLeft") => Some(-step),
        (ResizableOrientation::Horizontal, "ArrowRight") => Some(step),
        (ResizableOrientation::Vertical, "ArrowUp") => Some(-step),
        (ResizableOrientation::Vertical, "ArrowDown") => Some(step),
        _ => None,
    }
}

pub fn resolve_state(input: ResizableStateInput) -> ResizableState {
    let split_percent = clamp_split(input.split_percent, input.bounds);
    let enabled = !input.disabled;
    let idle = !input.dragging;
    let is_uncontrolled = !input.is_controlled;

    ResizableState {
        orientation: input.orientation,
        orientation_attr: match input.orientation {
            ResizableOrientation::Horizontal => "horizontal",
            ResizableOrientation::Vertical => "vertical",
        },
        split_percent,
        min_split_percent: input.bounds.min,
        max_split_percent: input.bounds.max,
        disabled: input.disabled,
        enabled,
        dragging: input.dragging,
        idle,
        is_controlled: input.is_controlled,
        is_uncontrolled,
        with_handle: input.with_handle,
        has_custom_class_name: input.has_custom_class_name,
        state_attr: if input.disabled {
            "disabled"
        } else if input.dragging {
            "dragging"
        } else {
            "idle"
        },
        handle_attr: if input.with_handle {
            "with-handle"
        } else {
            "plain"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

#[cfg(test)]
mod tests {
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
}
