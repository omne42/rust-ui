use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressCircleRange {
    pub min: f64,
    pub max: f64,
}

impl ProgressCircleRange {
    pub fn sanitized(min: f64, max: f64) -> Self {
        let mut min = if min.is_finite() { min } else { 0.0 };
        let mut max = if max.is_finite() { max } else { 1.0 };
        if max <= min {
            (min, max) = (0.0, 1.0);
        }
        Self { min, max }
    }

    pub fn span(self) -> f64 {
        (self.max - self.min).max(f64::EPSILON)
    }
}

pub fn clamp_to_range(value: f64, range: ProgressCircleRange) -> f64 {
    value.clamp(range.min, range.max)
}

pub fn normalize_progress(value: f64, range: ProgressCircleRange) -> f64 {
    (value - range.min) / range.span()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressCircleMetrics {
    pub size_px: f64,
    pub stroke_width_px: f64,
    pub radius_px: f64,
    pub circumference: f64,
}

pub fn resolve_metrics(size_px: f64, stroke_width_px: f64) -> ProgressCircleMetrics {
    let size_px = if size_px.is_finite() && size_px > 0.0 {
        size_px
    } else {
        24.0
    };
    let stroke_width_px = if stroke_width_px.is_finite() && stroke_width_px > 0.0 {
        stroke_width_px
    } else {
        3.0
    };

    let radius_px = (size_px - stroke_width_px).max(1.0) / 2.0;
    let circumference = 2.0 * PI * radius_px;

    ProgressCircleMetrics {
        size_px,
        stroke_width_px,
        radius_px,
        circumference,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metrics_sanitize_inputs() {
        let metrics = resolve_metrics(f64::NAN, -1.0);
        assert_eq!(metrics.size_px, 24.0);
        assert_eq!(metrics.stroke_width_px, 3.0);
        assert!(metrics.radius_px > 0.0);
        assert!(metrics.circumference > 0.0);
    }

    #[test]
    fn normalize_maps_min_to_0_and_max_to_1() {
        let range = ProgressCircleRange::sanitized(10.0, 20.0);
        assert_eq!(normalize_progress(10.0, range), 0.0);
        assert_eq!(normalize_progress(20.0, range), 1.0);
    }
}
