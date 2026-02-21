use crate::a11y::{A11yDirection, locale_attrs};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ColorSwatchA11yHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorSwatchA11yAttrs {
    pub role: Option<&'static str>,
    pub aria_label: Option<String>,
    pub aria_hidden: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorSwatchA11yState {
    pub is_decorative: bool,
    pub exposes_image_role: bool,
    pub has_label: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorSwatchA11yContract {
    pub attrs: ColorSwatchA11yAttrs,
    pub handlers: ColorSwatchA11yHandlers,
    pub state: ColorSwatchA11yState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColorSwatchA11yOptions {
    pub is_decorative: bool,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_color_swatch_a11y(options: ColorSwatchA11yOptions) -> ColorSwatchA11yContract {
    let locale = locale_attrs(options.lang, options.dir);
    let exposes_image_role = !options.is_decorative;

    let aria_label = exposes_image_role.then_some(options.aria_label);
    let has_label = aria_label.is_some();

    ColorSwatchA11yContract {
        attrs: ColorSwatchA11yAttrs {
            role: exposes_image_role.then_some("img"),
            aria_label,
            aria_hidden: options.is_decorative.then_some("true"),
            lang: locale.lang,
            dir: locale.dir,
        },
        handlers: ColorSwatchA11yHandlers,
        state: ColorSwatchA11yState {
            is_decorative: options.is_decorative,
            exposes_image_role,
            has_label,
        },
    }
}

#[cfg(test)]
#[path = "test/color_swatch.rs"]
mod tests;
