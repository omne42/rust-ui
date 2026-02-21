use crate::a11y::{A11yDirection, locale_attrs};
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarouselA11yOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarouselKeyCommand {
    StepBackward,
    StepForward,
    SelectFirst,
    SelectLast,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CarouselRootAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub tabindex: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarouselRootState {
    pub orientation: CarouselA11yOrientation,
    pub is_rtl: bool,
}

#[derive(Clone)]
pub struct CarouselRootHandlers {
    pub on_key_down: Callback<String, bool>,
}

#[derive(Clone)]
pub struct CarouselRootContract {
    pub attrs: CarouselRootAttrs,
    pub handlers: CarouselRootHandlers,
    pub state: CarouselRootState,
}

#[derive(Clone)]
pub struct CarouselRootOptions {
    pub aria_label: String,
    pub orientation: CarouselA11yOrientation,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
    pub on_key_command: Callback<CarouselKeyCommand>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarouselSlideA11yAttrs {
    pub role: &'static str,
    pub aria_roledescription: &'static str,
    pub aria_hidden: &'static str,
}

fn horizontal_nav_keys(dir: Option<A11yDirection>) -> (&'static str, &'static str) {
    match dir {
        Some(A11yDirection::Rtl) => ("ArrowRight", "ArrowLeft"),
        _ => ("ArrowLeft", "ArrowRight"),
    }
}

pub fn resolve_carousel_key_command(
    key: &str,
    orientation: CarouselA11yOrientation,
    dir: Option<A11yDirection>,
) -> Option<CarouselKeyCommand> {
    let (prev_key, next_key) = match orientation {
        CarouselA11yOrientation::Horizontal => horizontal_nav_keys(dir),
        CarouselA11yOrientation::Vertical => ("ArrowUp", "ArrowDown"),
    };

    if key == prev_key {
        return Some(CarouselKeyCommand::StepBackward);
    }
    if key == next_key {
        return Some(CarouselKeyCommand::StepForward);
    }

    match key {
        "Home" => Some(CarouselKeyCommand::SelectFirst),
        "End" => Some(CarouselKeyCommand::SelectLast),
        _ => None,
    }
}

pub fn carousel_slide_a11y_attrs(is_selected: bool) -> CarouselSlideA11yAttrs {
    CarouselSlideA11yAttrs {
        role: "group",
        aria_roledescription: "slide",
        aria_hidden: if is_selected { "false" } else { "true" },
    }
}

pub fn use_carousel_root(options: CarouselRootOptions) -> CarouselRootContract {
    let CarouselRootOptions {
        aria_label,
        orientation,
        lang,
        dir,
        on_key_command,
    } = options;
    let locale = locale_attrs(lang, dir);
    let is_rtl = matches!(dir, Some(A11yDirection::Rtl));

    let on_key_down = Callback::new(move |key: String| -> bool {
        let Some(command) = resolve_carousel_key_command(&key, orientation, dir) else {
            return false;
        };
        on_key_command.run(command);
        true
    });

    CarouselRootContract {
        attrs: CarouselRootAttrs {
            role: "region",
            aria_label,
            lang: locale.lang,
            dir: locale.dir,
            tabindex: 0,
        },
        handlers: CarouselRootHandlers { on_key_down },
        state: CarouselRootState {
            orientation,
            is_rtl,
        },
    }
}

#[cfg(test)]
#[path = "test/carousel.rs"]
mod tests;
