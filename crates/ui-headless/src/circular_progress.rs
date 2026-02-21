use crate::a11y::{A11yDirection, locale_attrs};
use ui_state_primitives::circular_progress::CircularProgressState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CircularProgressHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircularProgressAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub aria_valuemin: &'static str,
    pub aria_valuemax: &'static str,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_state: &'static str,
    pub data_motion: &'static str,
    pub data_size: Option<&'static str>,
    pub data_thickness: Option<&'static str>,
    pub data_size_source: &'static str,
    pub data_thickness_source: &'static str,
    pub data_label_source: &'static str,
    pub data_class_source: &'static str,
    pub data_custom_size: Option<&'static str>,
    pub data_custom_thickness: Option<&'static str>,
    pub data_custom_aria_label: Option<&'static str>,
    pub data_custom_class: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CircularProgressSemanticState {
    pub state: &'static str,
    pub motion: &'static str,
    pub size_source: &'static str,
    pub thickness_source: &'static str,
    pub label_source: &'static str,
    pub class_source: &'static str,
    pub has_custom_size: bool,
    pub has_custom_thickness: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CircularProgressContract {
    pub attrs: CircularProgressAttrs,
    pub handlers: CircularProgressHandlers,
    pub state: CircularProgressSemanticState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CircularProgressOptions {
    pub state: CircularProgressState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_circular_progress(options: CircularProgressOptions) -> CircularProgressContract {
    let locale = locale_attrs(options.lang, options.dir);

    CircularProgressContract {
        attrs: CircularProgressAttrs {
            role: "progressbar",
            aria_label: options.aria_label,
            aria_valuemin: "0",
            aria_valuemax: "100",
            lang: locale.lang,
            dir: locale.dir,
            data_state: "indeterminate",
            data_motion: "spin",
            data_size: options.state.has_custom_size.then_some("custom"),
            data_thickness: options.state.has_custom_thickness.then_some("custom"),
            data_size_source: options.state.size_source_attr,
            data_thickness_source: options.state.thickness_source_attr,
            data_label_source: options.state.label_source_attr,
            data_class_source: options.state.class_source_attr,
            data_custom_size: options.state.has_custom_size.then_some("true"),
            data_custom_thickness: options.state.has_custom_thickness.then_some("true"),
            data_custom_aria_label: options.state.has_custom_aria_label.then_some("true"),
            data_custom_class: options.state.has_custom_class_name.then_some("true"),
        },
        handlers: CircularProgressHandlers,
        state: CircularProgressSemanticState {
            state: "indeterminate",
            motion: "spin",
            size_source: options.state.size_source_attr,
            thickness_source: options.state.thickness_source_attr,
            label_source: options.state.label_source_attr,
            class_source: options.state.class_source_attr,
            has_custom_size: options.state.has_custom_size,
            has_custom_thickness: options.state.has_custom_thickness,
            has_custom_aria_label: options.state.has_custom_aria_label,
            has_custom_class_name: options.state.has_custom_class_name,
        },
    }
}

#[cfg(test)]
#[path = "test/circular_progress.rs"]
mod tests;
