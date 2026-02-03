#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MeterRange {
    pub min: f64,
    pub max: f64,
}

impl MeterRange {
    pub fn sanitized(min: f64, max: f64) -> Self {
        let min = if min.is_finite() { min } else { 0.0 };
        let max = if max.is_finite() { max } else { 100.0 };

        if max <= min {
            return Self {
                min: 0.0,
                max: 100.0,
            };
        }

        Self { min, max }
    }

    pub fn span(self) -> f64 {
        (self.max - self.min).max(0.0)
    }
}

pub fn clamp_to_range(value: f64, range: MeterRange) -> f64 {
    if !value.is_finite() {
        return range.min;
    }
    value.clamp(range.min, range.max)
}

/// Convert a numeric value into progress in `[0, 1]`.
pub fn normalize_progress(value: f64, range: MeterRange) -> f64 {
    let span = range.span();
    if span <= 0.0 {
        return 0.0;
    }
    let value = clamp_to_range(value, range);
    ((value - range.min) / span).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitizes_range_when_invalid() {
        assert_eq!(
            MeterRange::sanitized(10.0, 10.0),
            MeterRange {
                min: 0.0,
                max: 100.0
            }
        );
        assert_eq!(
            MeterRange::sanitized(f64::NAN, f64::INFINITY),
            MeterRange {
                min: 0.0,
                max: 100.0
            }
        );
    }

    #[test]
    fn clamps_to_range_limits() {
        let range = MeterRange::sanitized(0.0, 10.0);
        assert_eq!(clamp_to_range(-5.0, range), 0.0);
        assert_eq!(clamp_to_range(5.0, range), 5.0);
        assert_eq!(clamp_to_range(50.0, range), 10.0);
    }

    #[test]
    fn normalizes_progress_as_fraction() {
        let range = MeterRange::sanitized(0.0, 100.0);
        assert_eq!(normalize_progress(0.0, range), 0.0);
        assert_eq!(normalize_progress(50.0, range), 0.5);
        assert_eq!(normalize_progress(100.0, range), 1.0);
    }
}
