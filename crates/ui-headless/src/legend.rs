use crate::a11y::{A11yDirection, locale_attrs};
use ui_state_primitives::legend::LegendState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LegendHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LegendAttrs {
    pub aria_disabled: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_tone: &'static str,
    pub data_state: &'static str,
    pub data_required: Option<&'static str>,
    pub data_disabled: Option<&'static str>,
    pub data_text_source: &'static str,
    pub data_indicator_source: &'static str,
    pub data_custom_class: Option<&'static str>,
    pub data_class_source: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LegendSemanticState {
    pub tone: &'static str,
    pub state: &'static str,
    pub is_required: bool,
    pub is_disabled: bool,
    pub text_source: &'static str,
    pub indicator_source: &'static str,
    pub class_source: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LegendContract {
    pub attrs: LegendAttrs,
    pub handlers: LegendHandlers,
    pub state: LegendSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LegendOptions {
    pub state: LegendState,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_legend(options: LegendOptions) -> LegendContract {
    let locale = locale_attrs(options.lang, options.dir);
    let state = options.state;

    LegendContract {
        attrs: LegendAttrs {
            aria_disabled: state.is_disabled.then_some("true"),
            lang: locale.lang,
            dir: locale.dir,
            data_tone: state.tone_attr,
            data_state: if state.is_required {
                "required"
            } else {
                "optional"
            },
            data_required: state.is_required.then_some("true"),
            data_disabled: state.is_disabled.then_some("true"),
            data_text_source: state.text_source_attr,
            data_indicator_source: state.indicator_source_attr,
            data_custom_class: state.has_custom_class_name.then_some("true"),
            data_class_source: state.class_source_attr,
        },
        handlers: LegendHandlers,
        state: LegendSemanticState {
            tone: state.tone_attr,
            state: if state.is_required {
                "required"
            } else {
                "optional"
            },
            is_required: state.is_required,
            is_disabled: state.is_disabled,
            text_source: state.text_source_attr,
            indicator_source: state.indicator_source_attr,
            class_source: state.class_source_attr,
        },
    }
}

#[cfg(test)]
#[path = "test/legend.rs"]
mod tests;
