pub const DEFAULT_ID_BASE: &str = "ui-chart";
pub const DEFAULT_ARIA_LABEL: &str = "Chart";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChartKind {
    #[default]
    Bar,
    Line,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChartPoint {
    pub id: String,
    pub label: String,
    pub value: f64,
}

impl ChartPoint {
    pub fn new(id: impl Into<String>, label: impl Into<String>, value: f64) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChartDomain {
    pub min: f64,
    pub max: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChartStateInput {
    pub kind: ChartKind,
    pub point_count: usize,
    pub active_index: usize,
    pub disabled: bool,
    pub show_grid: bool,
    pub is_controlled: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChartState {
    pub kind: ChartKind,
    pub kind_attr: &'static str,
    pub point_count: usize,
    pub active_index: usize,
    pub has_points: bool,
    pub is_empty: bool,
    pub disabled: bool,
    pub enabled: bool,
    pub show_grid: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub has_custom_class_name: bool,
    pub state_attr: &'static str,
    pub class_source_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_id_base(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ID_BASE.into())
}

pub fn normalize_aria_label(value: Option<String>) -> String {
    normalize_optional_text(value).unwrap_or_else(|| DEFAULT_ARIA_LABEL.into())
}

pub fn normalize_points(points: Vec<ChartPoint>) -> Vec<ChartPoint> {
    points
        .into_iter()
        .enumerate()
        .map(|(index, point)| {
            let fallback_id = format!("point-{index}");
            let fallback_label = format!("Point {}", index + 1);

            let id = normalize_optional_text(Some(point.id)).unwrap_or(fallback_id);
            let label = normalize_optional_text(Some(point.label)).unwrap_or(fallback_label);
            let value = if point.value.is_finite() {
                point.value
            } else {
                0.0
            };

            ChartPoint { id, label, value }
        })
        .collect()
}

pub fn value_domain(points: &[ChartPoint]) -> ChartDomain {
    if points.is_empty() {
        return ChartDomain { min: 0.0, max: 1.0 };
    }

    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;

    for point in points {
        min = min.min(point.value);
        max = max.max(point.value);
    }

    if !min.is_finite() || !max.is_finite() {
        return ChartDomain { min: 0.0, max: 1.0 };
    }

    if (max - min).abs() < f64::EPSILON {
        return ChartDomain {
            min: min - 0.5,
            max: max + 0.5,
        };
    }

    ChartDomain { min, max }
}

pub fn clamp_active_index(index: usize, point_count: usize) -> usize {
    if point_count == 0 {
        return 0;
    }

    index.min(point_count.saturating_sub(1))
}

pub fn default_active_index(point_count: usize, requested: Option<usize>) -> usize {
    clamp_active_index(requested.unwrap_or(0), point_count)
}

pub fn resolve_state(input: ChartStateInput) -> ChartState {
    let has_points = input.point_count > 0;
    let is_empty = !has_points;
    let enabled = !input.disabled;
    let active_index = clamp_active_index(input.active_index, input.point_count);
    let is_uncontrolled = !input.is_controlled;

    ChartState {
        kind: input.kind,
        kind_attr: match input.kind {
            ChartKind::Bar => "bar",
            ChartKind::Line => "line",
        },
        point_count: input.point_count,
        active_index,
        has_points,
        is_empty,
        disabled: input.disabled,
        enabled,
        show_grid: input.show_grid,
        is_controlled: input.is_controlled,
        is_uncontrolled,
        has_custom_class_name: input.has_custom_class_name,
        state_attr: if input.disabled && is_empty {
            "disabled-empty"
        } else if input.disabled {
            "disabled"
        } else if is_empty {
            "empty"
        } else {
            "ready"
        },
        class_source_attr: if input.has_custom_class_name {
            "custom"
        } else {
            "default"
        },
    }
}

pub fn compose_class_name(class_name: Option<String>, state: ChartState) -> String {
    let mut classes = vec!["ui-chart".to_string()];

    match state.kind {
        ChartKind::Bar => classes.push("ui-chart--bar".to_string()),
        ChartKind::Line => classes.push("ui-chart--line".to_string()),
    }

    if state.is_empty {
        classes.push("ui-chart--empty".to_string());
    }

    if state.disabled {
        classes.push("ui-chart--disabled".to_string());
    }

    if state.show_grid {
        classes.push("ui-chart--grid".to_string());
    }

    if state.is_controlled {
        classes.push("ui-chart--controlled".to_string());
    } else {
        classes.push("ui-chart--uncontrolled".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-chart--custom-class".to_string());
        if let Some(class_name) = class_name {
            classes.push(class_name);
        }
    }

    classes.join(" ")
}

pub fn point_x(index: usize, point_count: usize) -> f64 {
    if point_count <= 1 {
        return 50.0;
    }

    let step = 84.0 / (point_count.saturating_sub(1) as f64);
    8.0 + (index as f64) * step
}

pub fn point_y(value: f64, domain: ChartDomain) -> f64 {
    let span = (domain.max - domain.min).max(0.0001);
    let ratio = ((value - domain.min) / span).clamp(0.0, 1.0);
    52.0 - ratio * 44.0
}

pub fn bar_width(point_count: usize) -> f64 {
    if point_count == 0 {
        return 8.0;
    }

    (72.0 / point_count as f64).clamp(4.0, 12.0)
}

pub fn polyline_points(points: &[ChartPoint], domain: ChartDomain) -> String {
    points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let x = point_x(index, points.len());
            let y = point_y(point.value, domain);
            format!("{x:.3},{y:.3}")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn next_index_for_key(key: &str, current: usize, count: usize) -> Option<usize> {
    if count == 0 {
        return None;
    }

    match key {
        "ArrowLeft" | "ArrowUp" => Some(current.saturating_sub(1)),
        "ArrowRight" | "ArrowDown" => Some((current + 1).min(count.saturating_sub(1))),
        "Home" => Some(0),
        "End" => Some(count.saturating_sub(1)),
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/chart.rs"]
mod tests;
