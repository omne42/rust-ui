use crate::flex::{FlexState, FlexStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Flex";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FlexDirection {
    #[default]
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl FlexDirection {
    pub fn class_name(self) -> &'static str {
        match self {
            FlexDirection::Row => "ui-flex--direction-row",
            FlexDirection::Column => "ui-flex--direction-column",
            FlexDirection::RowReverse => "ui-flex--direction-row-reverse",
            FlexDirection::ColumnReverse => "ui-flex--direction-column-reverse",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FlexDirection::Row => "row",
            FlexDirection::Column => "column",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::ColumnReverse => "column-reverse",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FlexWrap {
    #[default]
    NoWrap,
    Wrap,
}

impl FlexWrap {
    pub fn class_name(self) -> &'static str {
        match self {
            FlexWrap::NoWrap => "ui-flex--wrap-nowrap",
            FlexWrap::Wrap => "ui-flex--wrap-wrap",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FlexWrap::NoWrap => "nowrap",
            FlexWrap::Wrap => "wrap",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FlexJustify {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl FlexJustify {
    pub fn class_name(self) -> &'static str {
        match self {
            FlexJustify::Start => "ui-flex--justify-start",
            FlexJustify::Center => "ui-flex--justify-center",
            FlexJustify::End => "ui-flex--justify-end",
            FlexJustify::SpaceBetween => "ui-flex--justify-space-between",
            FlexJustify::SpaceAround => "ui-flex--justify-space-around",
            FlexJustify::SpaceEvenly => "ui-flex--justify-space-evenly",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FlexJustify::Start => "start",
            FlexJustify::Center => "center",
            FlexJustify::End => "end",
            FlexJustify::SpaceBetween => "space-between",
            FlexJustify::SpaceAround => "space-around",
            FlexJustify::SpaceEvenly => "space-evenly",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FlexAlign {
    Start,
    Center,
    End,
    Baseline,
    #[default]
    Stretch,
}

impl FlexAlign {
    pub fn class_name(self) -> &'static str {
        match self {
            FlexAlign::Start => "ui-flex--align-start",
            FlexAlign::Center => "ui-flex--align-center",
            FlexAlign::End => "ui-flex--align-end",
            FlexAlign::Baseline => "ui-flex--align-baseline",
            FlexAlign::Stretch => "ui-flex--align-stretch",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FlexAlign::Start => "start",
            FlexAlign::Center => "center",
            FlexAlign::End => "end",
            FlexAlign::Baseline => "baseline",
            FlexAlign::Stretch => "stretch",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FlexGap {
    None,
    Xs,
    #[default]
    Sm,
    Md,
    Lg,
}

impl FlexGap {
    pub fn class_name(self) -> &'static str {
        match self {
            FlexGap::None => "ui-flex--gap-none",
            FlexGap::Xs => "ui-flex--gap-xs",
            FlexGap::Sm => "ui-flex--gap-sm",
            FlexGap::Md => "ui-flex--gap-md",
            FlexGap::Lg => "ui-flex--gap-lg",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            FlexGap::None => "none",
            FlexGap::Xs => "xs",
            FlexGap::Sm => "sm",
            FlexGap::Md => "md",
            FlexGap::Lg => "lg",
        }
    }
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
}

pub fn resolve_state(input: FlexStateInput) -> FlexState {
    let aria_source_attr = if input.has_custom_aria_label {
        "custom"
    } else {
        "default"
    };
    let class_source_attr = if input.has_custom_class_name {
        "custom"
    } else {
        "default"
    };

    let data_state_attr =
        if input.inline && input.wrap == FlexWrap::Wrap && input.gap != FlexGap::None {
            "inline-wrap-gapped"
        } else if input.inline && input.wrap == FlexWrap::Wrap {
            "inline-wrap"
        } else if input.inline {
            "inline"
        } else if input.wrap == FlexWrap::Wrap && input.gap != FlexGap::None {
            "wrap-gapped"
        } else if input.wrap == FlexWrap::Wrap {
            "wrap"
        } else if input.gap == FlexGap::None {
            "no-gap"
        } else {
            "default"
        };

    FlexState {
        direction: input.direction,
        direction_class: input.direction.class_name(),
        direction_attr: input.direction.as_attr(),
        wrap: input.wrap,
        wrap_class: input.wrap.class_name(),
        wrap_attr: input.wrap.as_attr(),
        justify: input.justify,
        justify_class: input.justify.class_name(),
        justify_attr: input.justify.as_attr(),
        align: input.align,
        align_class: input.align.class_name(),
        align_attr: input.align.as_attr(),
        gap: input.gap,
        gap_class: input.gap.class_name(),
        gap_attr: input.gap.as_attr(),
        is_inline: input.inline,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: FlexState) -> String {
    let mut classes = vec![
        "ui-flex".to_string(),
        state.direction_class.to_string(),
        state.wrap_class.to_string(),
        state.justify_class.to_string(),
        state.align_class.to_string(),
        state.gap_class.to_string(),
    ];

    if state.is_inline {
        classes.push("ui-flex--inline".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-flex--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flex_contract_tokens_are_stable() {
        assert_eq!(FlexDirection::Row.class_name(), "ui-flex--direction-row");
        assert_eq!(FlexWrap::Wrap.class_name(), "ui-flex--wrap-wrap");
        assert_eq!(
            FlexJustify::SpaceEvenly.class_name(),
            "ui-flex--justify-space-evenly"
        );
        assert_eq!(FlexAlign::Baseline.class_name(), "ui-flex--align-baseline");
        assert_eq!(FlexGap::Md.class_name(), "ui-flex--gap-md");

        assert_eq!(FlexDirection::ColumnReverse.as_attr(), "column-reverse");
        assert_eq!(FlexWrap::NoWrap.as_attr(), "nowrap");
        assert_eq!(FlexJustify::SpaceBetween.as_attr(), "space-between");
        assert_eq!(FlexAlign::Stretch.as_attr(), "stretch");
        assert_eq!(FlexGap::None.as_attr(), "none");
    }

    #[test]
    fn normalize_optional_text_trims_and_drops_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-flex  ".to_string())),
            Some("docs-flex".to_string())
        );
    }

    #[test]
    fn normalize_aria_label_uses_fallback_when_missing() {
        let (label, custom) = normalize_aria_label(Some("  Toolbar layout  ".to_string()));
        assert_eq!(label, "Toolbar layout");
        assert!(custom);

        let (label, custom) = normalize_aria_label(Some("  ".to_string()));
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_state_tracks_layout_and_sources() {
        let state = resolve_state(FlexStateInput {
            direction: FlexDirection::Column,
            wrap: FlexWrap::Wrap,
            justify: FlexJustify::SpaceBetween,
            align: FlexAlign::Center,
            gap: FlexGap::Lg,
            inline: true,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.direction_attr, "column");
        assert_eq!(state.wrap_attr, "wrap");
        assert_eq!(state.justify_attr, "space-between");
        assert_eq!(state.align_attr, "center");
        assert_eq!(state.gap_attr, "lg");
        assert!(state.is_inline);
        assert_eq!(state.data_state_attr, "inline-wrap-gapped");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_custom_marker_and_user_class() {
        let state = resolve_state(FlexStateInput {
            direction: FlexDirection::Row,
            wrap: FlexWrap::Wrap,
            justify: FlexJustify::Start,
            align: FlexAlign::Stretch,
            gap: FlexGap::Sm,
            inline: false,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-flex-custom".to_string()), state);

        for token in [
            "ui-flex",
            "ui-flex--direction-row",
            "ui-flex--wrap-wrap",
            "ui-flex--justify-start",
            "ui-flex--align-stretch",
            "ui-flex--gap-sm",
            "ui-flex--custom-class",
            "docs-flex-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
