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
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

pub fn normalize_aria_label(value: Option<String>) -> (String, bool) {
    if let Some(label) = normalize_optional_text(value) {
        return (label, true);
    }

    (DEFAULT_ARIA_LABEL.to_string(), false)
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
        state.columns_class.to_string(),
        state.rows_class.to_string(),
        state.gap_class.to_string(),
        state.justify_class.to_string(),
        state.align_class.to_string(),
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
mod tests {
    use super::*;

    #[test]
    fn grid_tokens_contract_is_stable() {
        assert_eq!(GridColumns::One.class_name(), "ui-grid--columns-1");
        assert_eq!(GridColumns::AutoFit.as_attr(), "auto-fit");

        assert_eq!(GridRows::Equal.class_name(), "ui-grid--rows-equal");
        assert_eq!(GridRows::Compact.as_attr(), "compact");

        assert_eq!(GridGap::Md.class_name(), "ui-grid--gap-md");
        assert_eq!(GridGap::None.as_attr(), "none");

        assert_eq!(GridJustify::Center.class_name(), "ui-grid--justify-center");
        assert_eq!(GridJustify::Stretch.as_attr(), "stretch");

        assert_eq!(GridAlign::End.class_name(), "ui-grid--align-end");
        assert_eq!(GridAlign::Start.as_attr(), "start");
    }

    #[test]
    fn normalize_optional_text_trims_and_drops_blank_values() {
        assert_eq!(normalize_optional_text(None), None);
        assert_eq!(normalize_optional_text(Some(" \n\t ".to_string())), None);
        assert_eq!(
            normalize_optional_text(Some("  docs-grid  ".to_string())),
            Some("docs-grid".to_string())
        );
    }

    #[test]
    fn normalize_aria_label_uses_fallback_when_missing() {
        let (label, custom) = normalize_aria_label(Some("  Metrics Grid  ".to_string()));
        assert_eq!(label, "Metrics Grid");
        assert!(custom);

        let (label, custom) = normalize_aria_label(Some("  ".to_string()));
        assert_eq!(label, DEFAULT_ARIA_LABEL);
        assert!(!custom);
    }

    #[test]
    fn resolve_state_tracks_layout_and_sources() {
        let state = resolve_state(GridStateInput {
            columns: GridColumns::AutoFit,
            rows: GridRows::Equal,
            gap: GridGap::Lg,
            justify: GridJustify::Center,
            align: GridAlign::Stretch,
            dense: true,
            inline: false,
            has_custom_aria_label: true,
            has_custom_class_name: false,
        });

        assert_eq!(state.columns_attr, "auto-fit");
        assert_eq!(state.rows_attr, "equal");
        assert_eq!(state.gap_attr, "lg");
        assert_eq!(state.justify_attr, "center");
        assert_eq!(state.align_attr, "stretch");
        assert!(state.is_dense);
        assert!(!state.is_inline);
        assert_eq!(state.data_state_attr, "dense");
        assert_eq!(state.aria_source_attr, "custom");
        assert_eq!(state.class_source_attr, "default");
    }

    #[test]
    fn compose_class_name_includes_custom_marker_and_user_class() {
        let state = resolve_state(GridStateInput {
            columns: GridColumns::Three,
            rows: GridRows::Auto,
            gap: GridGap::Sm,
            justify: GridJustify::Start,
            align: GridAlign::Start,
            dense: false,
            inline: true,
            has_custom_aria_label: false,
            has_custom_class_name: true,
        });

        let class_name = compose_class_name(Some("docs-grid-custom".to_string()), state);

        for token in [
            "ui-grid",
            "ui-grid--columns-3",
            "ui-grid--rows-auto",
            "ui-grid--gap-sm",
            "ui-grid--justify-start",
            "ui-grid--align-start",
            "ui-grid--inline",
            "ui-grid--custom-class",
            "docs-grid-custom",
        ] {
            assert!(
                class_name.contains(token),
                "composed class should include `{token}`"
            );
        }
    }
}
