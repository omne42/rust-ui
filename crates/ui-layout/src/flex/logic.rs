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
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.into(), false)
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
        state.direction_class.into(),
        state.wrap_class.into(),
        state.justify_class.into(),
        state.align_class.into(),
        state.gap_class.into(),
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
#[path = "test/logic.rs"]
mod tests;
