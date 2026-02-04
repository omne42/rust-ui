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
    value.clamp(range.min, range.max)
}

pub fn normalize_progress(value: f64, range: ProgressRange) -> f64 {
    (value - range.min) / range.span()
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
}
