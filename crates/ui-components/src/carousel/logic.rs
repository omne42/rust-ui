use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl CarouselOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Horizontal => "ui-carousel--horizontal",
            Self::Vertical => "ui-carousel--vertical",
        }
    }

    pub fn attr(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    pub fn prev_key(self) -> &'static str {
        match self {
            Self::Horizontal => "ArrowLeft",
            Self::Vertical => "ArrowUp",
        }
    }

    pub fn next_key(self) -> &'static str {
        match self {
            Self::Horizontal => "ArrowRight",
            Self::Vertical => "ArrowDown",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CarouselItem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub disabled: bool,
}

impl CarouselItem {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            disabled: false,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CarouselItemResolved {
    pub id: String,
    pub slide_dom_id: String,
    pub dot_dom_id: String,
    pub title: String,
    pub description: Option<String>,
    pub disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarouselStateInput {
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub focused_index: Option<usize>,
    pub has_disabled_items: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub orientation: CarouselOrientation,
    pub loop_navigation: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarouselState {
    pub item_count: usize,
    pub is_empty: bool,
    pub has_items: bool,
    pub selected_index: Option<usize>,
    pub focused_index: Option<usize>,
    pub has_selection: bool,
    pub has_focus: bool,
    pub has_disabled_items: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub orientation: CarouselOrientation,
    pub orientation_attr: &'static str,
    pub orientation_class: &'static str,
    pub loop_navigation: bool,
    pub data_state_attr: &'static str,
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_id_base(id_base: String) -> String {
    normalize_optional_text(Some(id_base)).unwrap_or_else(|| "carousel".to_string())
}

fn sanitize_token(value: &str, fallback: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;

    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_dash = false;
            continue;
        }

        if (ch == '-' || ch == '_' || ch == ' ') && !last_dash {
            out.push('-');
            last_dash = true;
        }
    }

    while out.ends_with('-') {
        out.pop();
    }

    if out.is_empty() {
        return fallback.to_string();
    }

    out
}

pub fn resolve_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    ("Carousel".to_string(), false)
}

pub fn resolve_items(id_base: &str, items: Vec<CarouselItem>) -> Vec<CarouselItemResolved> {
    let mut seen_ids = BTreeSet::new();
    let mut resolved = Vec::new();

    for (index, item) in items.into_iter().enumerate() {
        let fallback_id = format!("slide-{}", index + 1);
        let raw_id = normalize_optional_text(Some(item.id)).unwrap_or_else(|| fallback_id.clone());
        let base_id = sanitize_token(&raw_id, &fallback_id);

        let mut unique_id = base_id.clone();
        let mut suffix = 2;
        while seen_ids.contains(&unique_id) {
            unique_id = format!("{base_id}-{suffix}");
            suffix += 1;
        }
        seen_ids.insert(unique_id.clone());

        let title = normalize_optional_text(Some(item.title))
            .unwrap_or_else(|| format!("Slide {}", index + 1));

        resolved.push(CarouselItemResolved {
            slide_dom_id: format!("{id_base}-{unique_id}-slide"),
            dot_dom_id: format!("{id_base}-{unique_id}-dot"),
            id: unique_id,
            title,
            description: normalize_optional_text(item.description),
            disabled: item.disabled,
        });
    }

    resolved
}

pub fn sanitize_index(index: Option<usize>, item_count: usize) -> Option<usize> {
    index.filter(|index| *index < item_count)
}

pub fn sanitize_selected_index(
    selected_index: Option<usize>,
    items: &[CarouselItemResolved],
) -> Option<usize> {
    let index = sanitize_index(selected_index, items.len())?;
    (!items[index].disabled).then_some(index)
}

pub fn sanitize_focused_index(
    focused_index: Option<usize>,
    items: &[CarouselItemResolved],
) -> Option<usize> {
    let index = sanitize_index(focused_index, items.len())?;
    (!items[index].disabled).then_some(index)
}

pub fn first_enabled_index(items: &[CarouselItemResolved]) -> Option<usize> {
    items.iter().position(|item| !item.disabled)
}

pub fn last_enabled_index(items: &[CarouselItemResolved]) -> Option<usize> {
    items.iter().rposition(|item| !item.disabled)
}

pub fn adjacent_enabled_index(
    items: &[CarouselItemResolved],
    current_index: usize,
    step: isize,
    should_loop: bool,
) -> Option<usize> {
    if items.is_empty() || step == 0 {
        return None;
    }

    if should_loop {
        let len = items.len() as isize;
        let mut cursor = current_index as isize;

        for _ in 0..items.len().saturating_sub(1) {
            cursor = (cursor + step).rem_euclid(len);
            let index = cursor as usize;
            if !items[index].disabled {
                return Some(index);
            }
        }

        return None;
    }

    let mut cursor = current_index as isize;
    loop {
        cursor += step;
        if cursor < 0 || cursor >= items.len() as isize {
            return None;
        }

        let index = cursor as usize;
        if !items[index].disabled {
            return Some(index);
        }
    }
}

pub fn resolve_initial_selected_index(
    items: &[CarouselItemResolved],
    selected_index: Option<usize>,
) -> Option<usize> {
    sanitize_selected_index(selected_index, items).or_else(|| first_enabled_index(items))
}

pub fn resolve_initial_focused_index(
    items: &[CarouselItemResolved],
    selected_index: Option<usize>,
) -> Option<usize> {
    sanitize_selected_index(selected_index, items).or_else(|| first_enabled_index(items))
}

pub fn resolve_state(input: CarouselStateInput) -> CarouselState {
    let has_items = input.item_count > 0;
    let has_selection = input.selected_index.is_some();
    let has_focus = input.focused_index.is_some();

    let data_state_attr = if !has_items {
        "empty"
    } else if has_selection {
        "selected"
    } else if has_focus {
        "focused"
    } else {
        "idle"
    };

    CarouselState {
        item_count: input.item_count,
        is_empty: !has_items,
        has_items,
        selected_index: input.selected_index,
        focused_index: input.focused_index,
        has_selection,
        has_focus,
        has_disabled_items: input.has_disabled_items,
        has_custom_aria_label: input.has_custom_aria_label,
        has_custom_class_name: input.has_custom_class_name,
        orientation: input.orientation,
        orientation_attr: input.orientation.attr(),
        orientation_class: input.orientation.class_name(),
        loop_navigation: input.loop_navigation,
        data_state_attr,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: CarouselState) -> String {
    let mut classes = vec![
        "ui-carousel".to_string(),
        state.orientation_class.to_string(),
    ];

    if state.is_empty {
        classes.push("ui-carousel--empty".to_string());
    }
    if state.has_selection {
        classes.push("ui-carousel--selected".to_string());
    }
    if state.has_disabled_items {
        classes.push("ui-carousel--has-disabled-items".to_string());
    }
    if state.loop_navigation {
        classes.push("ui-carousel--loop".to_string());
    }

    if state.has_custom_class_name
        && let Some(base_class_name) = base_class_name
    {
        classes.push(base_class_name);
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_base_and_aria_label_defaults_are_stable() {
        assert_eq!(
            normalize_id_base("  hero-carousel  ".to_string()),
            "hero-carousel"
        );
        assert_eq!(normalize_id_base(" ".to_string()), "carousel");

        assert_eq!(resolve_aria_label(None), ("Carousel".to_string(), false));
        assert_eq!(
            resolve_aria_label(Some("  Product slides  ".to_string())),
            ("Product slides".to_string(), true)
        );
    }

    #[test]
    fn resolve_items_normalizes_ids_and_titles() {
        let items = resolve_items(
            "docs-carousel",
            vec![
                CarouselItem::new("Hero", "Hero"),
                CarouselItem::new("Hero", " "),
                CarouselItem::new(" ", "Gallery"),
            ],
        );

        assert_eq!(items.len(), 3);
        assert_eq!(items[0].id, "hero");
        assert_eq!(items[1].id, "hero-2");
        assert_eq!(items[2].id, "slide-3");
        assert_eq!(items[1].title, "Slide 2");
        assert_eq!(items[0].slide_dom_id, "docs-carousel-hero-slide");
        assert_eq!(items[0].dot_dom_id, "docs-carousel-hero-dot");
    }

    #[test]
    fn adjacent_index_handles_loop_and_non_loop() {
        let items = resolve_items(
            "docs-carousel",
            vec![
                CarouselItem::new("a", "A"),
                CarouselItem::new("b", "B").disabled(true),
                CarouselItem::new("c", "C"),
            ],
        );

        assert_eq!(adjacent_enabled_index(&items, 0, 1, true), Some(2));
        assert_eq!(adjacent_enabled_index(&items, 2, 1, true), Some(0));
        assert_eq!(adjacent_enabled_index(&items, 2, 1, false), None);
        assert_eq!(adjacent_enabled_index(&items, 2, -1, false), Some(0));
    }

    #[test]
    fn selected_and_focus_indices_are_sanitized() {
        let items = resolve_items(
            "docs-carousel",
            vec![
                CarouselItem::new("a", "A").disabled(true),
                CarouselItem::new("b", "B"),
            ],
        );

        assert_eq!(sanitize_selected_index(Some(0), &items), None);
        assert_eq!(sanitize_selected_index(Some(1), &items), Some(1));
        assert_eq!(sanitize_focused_index(Some(1), &items), Some(1));
        assert_eq!(resolve_initial_selected_index(&items, Some(0)), Some(1));
        assert_eq!(resolve_initial_focused_index(&items, Some(0)), Some(1));
    }

    #[test]
    fn compose_class_name_includes_state_markers() {
        let class_name = compose_class_name(
            Some("custom".to_string()),
            resolve_state(CarouselStateInput {
                item_count: 3,
                selected_index: Some(1),
                focused_index: Some(1),
                has_disabled_items: true,
                has_custom_aria_label: true,
                has_custom_class_name: true,
                orientation: CarouselOrientation::Vertical,
                loop_navigation: true,
            }),
        );

        for token in [
            "ui-carousel",
            "ui-carousel--vertical",
            "ui-carousel--selected",
            "ui-carousel--has-disabled-items",
            "ui-carousel--loop",
            "custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class name should include `{token}`"
            );
        }
    }
}
