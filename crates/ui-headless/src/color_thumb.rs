use crate::a11y::{A11yDirection, locale_attrs};
use leptos::prelude::*;
use ui_state_primitives::color_thumb::ColorThumbState;

const BOOL_TRUE: &str = "true";

#[derive(Clone, Debug, PartialEq)]
pub struct ColorThumbOptions {
    pub state: ColorThumbState,
    pub aria_label: String,
    pub aria_value_text: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorThumbRootAttrs {
    pub role: &'static str,
    pub tabindex: i32,
    pub aria_label: String,
    pub aria_disabled: Option<&'static str>,
    pub aria_valuetext: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_state: &'static str,
    pub data_disabled: Option<&'static str>,
    pub data_focused: Option<&'static str>,
    pub data_dragging: Option<&'static str>,
    pub data_loupe_visible: Option<&'static str>,
    pub data_has_color: Option<&'static str>,
    pub data_x: f32,
    pub data_y: f32,
    pub data_x_bucket: &'static str,
    pub data_y_bucket: &'static str,
    pub data_interaction_source: &'static str,
    pub data_aria_source: &'static str,
    pub data_aria_valuetext_source: &'static str,
    pub data_custom_class: Option<&'static str>,
    pub data_class_source: &'static str,
    pub data_loupe_source: &'static str,
    pub data_x_source: &'static str,
    pub data_y_source: &'static str,
}

#[derive(Clone)]
pub struct ColorThumbHandlers {
    pub on_pointer_down: Callback<()>,
    pub on_pointer_up: Callback<()>,
    pub on_pointer_cancel: Callback<()>,
    pub on_focus: Callback<()>,
    pub on_blur: Callback<()>,
    pub on_key_down: Callback<String, bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorThumbSemanticState {
    pub is_disabled: bool,
    pub is_focused: bool,
    pub is_dragging: bool,
    pub loupe_visible: bool,
    pub has_color: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone)]
pub struct ColorThumbContract {
    pub root_attrs: ColorThumbRootAttrs,
    pub handlers: ColorThumbHandlers,
    pub state: ColorThumbSemanticState,
}

pub fn use_color_thumb(options: ColorThumbOptions) -> ColorThumbContract {
    let ColorThumbOptions {
        state,
        aria_label,
        aria_value_text,
        lang,
        dir,
    } = options;
    let locale = locale_attrs(lang, dir);

    let is_disabled = state.is_disabled;
    let on_key_down = Callback::new(move |key: String| {
        if is_disabled {
            return false;
        }

        matches!(
            key.as_str(),
            "ArrowLeft"
                | "ArrowRight"
                | "ArrowUp"
                | "ArrowDown"
                | "Home"
                | "End"
                | "PageUp"
                | "PageDown"
        )
    });

    ColorThumbContract {
        root_attrs: ColorThumbRootAttrs {
            role: "slider",
            tabindex: if state.is_disabled { -1 } else { 0 },
            aria_label,
            aria_disabled: state.is_disabled.then_some(BOOL_TRUE),
            aria_valuetext: aria_value_text,
            lang: locale.lang,
            dir: locale.dir,
            data_state: state.data_state_attr,
            data_disabled: state.is_disabled.then_some(BOOL_TRUE),
            data_focused: state.is_focused.then_some(BOOL_TRUE),
            data_dragging: state.is_dragging.then_some(BOOL_TRUE),
            data_loupe_visible: state.loupe_visible.then_some(BOOL_TRUE),
            data_has_color: state.has_color.then_some(BOOL_TRUE),
            data_x: state.x_percent,
            data_y: state.y_percent,
            data_x_bucket: state.x_bucket_attr,
            data_y_bucket: state.y_bucket_attr,
            data_interaction_source: state.interaction_source_attr,
            data_aria_source: state.aria_source_attr,
            data_aria_valuetext_source: state.aria_value_text_source_attr,
            data_custom_class: state.has_custom_class_name.then_some(BOOL_TRUE),
            data_class_source: state.class_source_attr,
            data_loupe_source: state.loupe_source_attr,
            data_x_source: state.x_source_attr,
            data_y_source: state.y_source_attr,
        },
        handlers: ColorThumbHandlers {
            on_pointer_down: Callback::new(|_| {}),
            on_pointer_up: Callback::new(|_| {}),
            on_pointer_cancel: Callback::new(|_| {}),
            on_focus: Callback::new(|_| {}),
            on_blur: Callback::new(|_| {}),
            on_key_down,
        },
        state: ColorThumbSemanticState {
            is_disabled: state.is_disabled,
            is_focused: state.is_focused,
            is_dragging: state.is_dragging,
            loupe_visible: state.loupe_visible,
            has_color: state.has_color,
            has_custom_class_name: state.has_custom_class_name,
        },
    }
}

#[cfg(test)]
#[path = "test/color_thumb.rs"]
mod tests;
