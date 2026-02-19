use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FormLabelPosition {
    #[default]
    Top,
    Left,
}

impl FormLabelPosition {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormLabelPosition::Top => "top",
            FormLabelPosition::Left => "left",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FormLabelAlign {
    #[default]
    Start,
    End,
}

impl FormLabelAlign {
    pub fn as_attr(self) -> &'static str {
        match self {
            FormLabelAlign::Start => "start",
            FormLabelAlign::End => "end",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormContextValue {
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub label_position: FormLabelPosition,
    pub label_align: FormLabelAlign,
}

pub fn use_form_context() -> Option<FormContextValue> {
    use_context::<FormContextValue>()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormViewState {
    pub label_position: &'static str,
    pub label_align: &'static str,
}

pub fn resolve_view_state(
    label_position: FormLabelPosition,
    label_align: FormLabelAlign,
) -> FormViewState {
    FormViewState {
        label_position: label_position.as_attr(),
        label_align: label_align.as_attr(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_label_layout_is_top_start() {
        let view = resolve_view_state(FormLabelPosition::default(), FormLabelAlign::default());
        assert_eq!(view.label_position, "top");
        assert_eq!(view.label_align, "start");
    }

    #[test]
    fn attr_mapping_matches_enum_variants() {
        assert_eq!(FormLabelPosition::Left.as_attr(), "left");
        assert_eq!(FormLabelAlign::End.as_attr(), "end");
    }
}
