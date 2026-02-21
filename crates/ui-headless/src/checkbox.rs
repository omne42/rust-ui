use crate::a11y::{A11yDirection, locale_attrs};
use crate::focus_ring::{FocusRingHandlers, FocusRingOptions, use_focus_ring};
use crate::hover::{HoverHandlers, HoverOptions, use_hover};
use crate::press::{OnPress, PressActivationKeys, PressHandlers, PressOptions, use_press};
use leptos::prelude::*;

#[derive(Clone)]
pub struct CheckboxOptions {
    pub is_disabled: bool,
    pub is_checked: ReadSignal<bool>,
    pub on_press: Option<OnPress>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct CheckboxHandlers {
    pub press: PressHandlers,
    pub hover: HoverHandlers,
    pub focus_ring: FocusRingHandlers,
}

#[derive(Clone)]
pub struct CheckboxAttrs {
    pub role: &'static str,
    pub tabindex: i32,
    pub aria_checked: Memo<&'static str>,
    pub aria_disabled: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct CheckboxState {
    pub is_pressed: ReadSignal<bool>,
    pub is_hovered: ReadSignal<bool>,
    pub is_focused: ReadSignal<bool>,
    pub is_focus_visible: Memo<bool>,
}

#[derive(Clone)]
pub struct CheckboxAria {
    pub state: CheckboxState,
    pub handlers: CheckboxHandlers,
    pub attrs: CheckboxAttrs,
}

pub fn use_checkbox(options: CheckboxOptions) -> CheckboxAria {
    let CheckboxOptions {
        is_disabled,
        is_checked,
        on_press,
        lang,
        dir,
    } = options;

    let locale = locale_attrs(lang, dir);
    let aria_checked = Memo::new(move |_| if is_checked.get() { "true" } else { "false" });

    let press = use_press(PressOptions {
        is_disabled,
        on_press,
        activation_keys: PressActivationKeys::SPACE,
        prevent_default_for_keyboard: true,
        ..Default::default()
    });
    let hover = use_hover(HoverOptions { is_disabled });
    let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });

    CheckboxAria {
        state: CheckboxState {
            is_pressed: press.is_pressed,
            is_hovered: hover.is_hovered,
            is_focused: focus_ring.is_focused,
            is_focus_visible: focus_ring.is_focus_visible,
        },
        handlers: CheckboxHandlers {
            press: press.handlers,
            hover: hover.handlers,
            focus_ring: focus_ring.handlers,
        },
        attrs: CheckboxAttrs {
            role: "checkbox",
            tabindex: if is_disabled { -1 } else { 0 },
            aria_checked,
            aria_disabled: is_disabled.then_some("true"),
            lang: locale.lang,
            dir: locale.dir,
        },
    }
}

#[cfg(test)]
#[path = "test/checkbox.rs"]
mod tests;
