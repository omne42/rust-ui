use crate::grid::{GridState, GridStateInput};

pub const DEFAULT_ARIA_LABEL: &str = "Grid";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum GridColumns {
    One,
    #[default]
    Two,
    Three,
    Four,
    AutoFit,
}

impl GridColumns {
    pub fn class_name(self) -> &'static str {
        match self {
            GridColumns::One => "ui-grid--columns-1",
            GridColumns::Two => "ui-grid--columns-2",
            GridColumns::Three => "ui-grid--columns-3",
            GridColumns::Four => "ui-grid--columns-4",
            GridColumns::AutoFit => "ui-grid--columns-auto-fit",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            GridColumns::One => "1",
            GridColumns::Two => "2",
            GridColumns::Three => "3",
            GridColumns::Four => "4",
            GridColumns::AutoFit => "auto-fit",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum GridRows {
    #[default]
    Auto,
    Equal,
    Compact,
}

impl GridRows {
    pub fn class_name(self) -> &'static str {
        match self {
            GridRows::Auto => "ui-grid--rows-auto",
            GridRows::Equal => "ui-grid--rows-equal",
            GridRows::Compact => "ui-grid--rows-compact",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            GridRows::Auto => "auto",
            GridRows::Equal => "equal",
            GridRows::Compact => "compact",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum GridGap {
    None,
    Xs,
    #[default]
    Sm,
    Md,
    Lg,
}

impl GridGap {
    pub fn class_name(self) -> &'static str {
        match self {
            GridGap::None => "ui-grid--gap-none",
            GridGap::Xs => "ui-grid--gap-xs",
            GridGap::Sm => "ui-grid--gap-sm",
            GridGap::Md => "ui-grid--gap-md",
            GridGap::Lg => "ui-grid--gap-lg",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            GridGap::None => "none",
            GridGap::Xs => "xs",
            GridGap::Sm => "sm",
            GridGap::Md => "md",
            GridGap::Lg => "lg",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum GridJustify {
    Start,
    Center,
    End,
    #[default]
    Stretch,
}

impl GridJustify {
    pub fn class_name(self) -> &'static str {
        match self {
            GridJustify::Start => "ui-grid--justify-start",
            GridJustify::Center => "ui-grid--justify-center",
            GridJustify::End => "ui-grid--justify-end",
            GridJustify::Stretch => "ui-grid--justify-stretch",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            GridJustify::Start => "start",
            GridJustify::Center => "center",
            GridJustify::End => "end",
            GridJustify::Stretch => "stretch",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum GridAlign {
    Start,
    Center,
    End,
    #[default]
    Stretch,
}

impl GridAlign {
    pub fn class_name(self) -> &'static str {
        match self {
            GridAlign::Start => "ui-grid--align-start",
            GridAlign::Center => "ui-grid--align-center",
            GridAlign::End => "ui-grid--align-end",
            GridAlign::Stretch => "ui-grid--align-stretch",
        }
    }

    pub fn as_attr(self) -> &'static str {
        match self {
            GridAlign::Start => "start",
            GridAlign::Center => "center",
            GridAlign::End => "end",
            GridAlign::Stretch => "stretch",
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

pub fn resolve_state(input: GridStateInput) -> GridState {
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

    let data_state_attr = if input.inline && input.dense {
        "inline-dense"
    } else if input.inline {
        "inline"
    } else if input.dense {
        "dense"
    } else if input.columns == GridColumns::AutoFit {
        "auto-fit"
    } else if input.gap == GridGap::None {
        "no-gap"
    } else {
        "default"
    };

    GridState {
        columns: input.columns,
        columns_class: input.columns.class_name(),
        columns_attr: input.columns.as_attr(),
        rows: input.rows,
        rows_class: input.rows.class_name(),
        rows_attr: input.rows.as_attr(),
        gap: input.gap,
        gap_class: input.gap.class_name(),
        gap_attr: input.gap.as_attr(),
        justify: input.justify,
        justify_class: input.justify.class_name(),
        justify_attr: input.justify.as_attr(),
        align: input.align,
        align_class: input.align.class_name(),
        align_attr: input.align.as_attr(),
        is_dense: input.dense,
        is_inline: input.inline,
        data_state_attr,
        aria_source_attr,
        class_source_attr,
        has_custom_class_name: input.has_custom_class_name,
    }
}

pub fn compose_class_name(base_class_name: Option<String>, state: GridState) -> String {
    let mut classes = vec![
        "ui-grid".to_string(),
        state.columns_class.into(),
        state.rows_class.into(),
        state.gap_class.into(),
        state.justify_class.into(),
        state.align_class.into(),
    ];

    if state.is_dense {
        classes.push("ui-grid--dense".to_string());
    }

    if state.is_inline {
        classes.push("ui-grid--inline".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-grid--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "test/logic.rs"]
mod tests;
