#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressRange {
    pub min: f64,
    pub max: f64,
}

impl ProgressRange {
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

pub fn clamp_to_range(value: f64, range: ProgressRange) -> f64 {
    if !value.is_finite() {
        return range.min;
    }
    value.clamp(range.min, range.max)
}

pub fn normalize_progress(value: f64, range: ProgressRange) -> f64 {
    ((value - range.min) / range.span()).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_sanitizes_invalid_bounds() {
        assert_eq!(
            ProgressRange::sanitized(10.0, 2.0),
            ProgressRange { min: 0.0, max: 1.0 }
        );
        assert_eq!(
            ProgressRange::sanitized(f64::NAN, f64::INFINITY),
            ProgressRange { min: 0.0, max: 1.0 }
        );
    }

    #[test]
    fn clamp_and_normalize_are_consistent() {
        let range = ProgressRange::sanitized(0.0, 100.0);
        let value = clamp_to_range(25.0, range);
        assert_eq!(value, 25.0);
        assert!((normalize_progress(value, range) - 0.25).abs() < 1e-9);
    }

    #[test]
    fn clamp_treats_non_finite_as_min() {
        let range = ProgressRange::sanitized(10.0, 20.0);
        assert_eq!(clamp_to_range(f64::NAN, range), 10.0);
        assert_eq!(clamp_to_range(f64::INFINITY, range), 10.0);
        assert_eq!(clamp_to_range(f64::NEG_INFINITY, range), 10.0);
    }
}
