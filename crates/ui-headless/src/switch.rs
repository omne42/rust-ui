use crate::a11y::{A11yDirection, locale_attrs};
use crate::focus_ring::{FocusRingHandlers, FocusRingOptions, use_focus_ring};
use crate::hover::{HoverHandlers, HoverOptions, use_hover};
use crate::press::{OnPress, PressActivationKeys, PressHandlers, PressOptions, use_press};
use leptos::prelude::*;
use ui_state_primitives::switch::{
    SwitchState as PrimitiveSwitchState, SwitchStateInput, resolve_state as resolve_switch_state,
};

#[derive(Clone)]
pub struct SwitchOptions {
    pub is_disabled: bool,
    pub is_checked: ReadSignal<bool>,
    pub on_press: Option<OnPress>,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

#[derive(Clone)]
pub struct SwitchHandlers {
    pub press: PressHandlers,
    pub hover: HoverHandlers,
    pub focus_ring: FocusRingHandlers,
}

#[derive(Clone)]
pub struct SwitchAttrs {
    pub role: &'static str,
    pub tabindex: i32,
    pub aria_checked: Memo<&'static str>,
    pub aria_disabled: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct SwitchState {
    pub is_pressed: ReadSignal<bool>,
    pub resolved: Memo<PrimitiveSwitchState>,
}

#[derive(Clone)]
pub struct SwitchAria {
    pub state: SwitchState,
    pub handlers: SwitchHandlers,
    pub attrs: SwitchAttrs,
}

pub fn use_switch(options: SwitchOptions) -> SwitchAria {
    let SwitchOptions {
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

    let is_pressed = press.is_pressed;
    let is_hovered = hover.is_hovered;
    let is_focused = focus_ring.is_focused;
    let is_focus_visible = focus_ring.is_focus_visible;
    let resolved = Memo::new(move |_| {
        resolve_switch_state(SwitchStateInput {
            is_checked: is_checked.get(),
            is_disabled,
            is_pressed: is_pressed.get(),
            is_hovered: is_hovered.get(),
            is_focused: is_focused.get(),
            is_focus_visible: is_focus_visible.get(),
        })
    });

    SwitchAria {
        state: SwitchState {
            is_pressed,
            resolved,
        },
        handlers: SwitchHandlers {
            press: press.handlers,
            hover: hover.handlers,
            focus_ring: focus_ring.handlers,
        },
        attrs: SwitchAttrs {
            role: "switch",
            tabindex: if is_disabled { -1 } else { 0 },
            aria_checked,
            aria_disabled: is_disabled.then_some("true"),
            lang: locale.lang,
            dir: locale.dir,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn switch_contract_exposes_derived_state_and_locale() {
        let (is_checked, set_checked) = signal(true);
        let switch = use_switch(SwitchOptions {
            is_disabled: false,
            is_checked,
            on_press: None,
            lang: Some("  zh-CN ".to_string()),
            dir: Some(A11yDirection::Rtl),
        });

        assert_eq!(switch.attrs.role, "switch");
        assert_eq!(switch.attrs.tabindex, 0);
        assert_eq!(switch.attrs.aria_checked.get_untracked(), "true");
        assert_eq!(switch.attrs.aria_disabled, None);
        assert_eq!(switch.attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(switch.attrs.dir, Some("rtl"));

        let resolved = switch.state.resolved.get_untracked();
        assert!(resolved.is_checked);
        assert_eq!(resolved.data_state(), "checked");

        set_checked.set(false);
        let resolved = switch.state.resolved.get_untracked();
        assert!(resolved.is_unchecked);
        assert_eq!(switch.attrs.aria_checked.get_untracked(), "false");
    }

    #[test]
    fn switch_contract_normalizes_interaction_state_when_disabled() {
        let (is_checked, _) = signal(false);
        let switch = use_switch(SwitchOptions {
            is_disabled: true,
            is_checked,
            on_press: None,
            lang: None,
            dir: None,
        });

        switch.handlers.press.on_pointer_down.run(());
        switch.handlers.hover.on_pointer_enter.run(());
        switch.handlers.focus_ring.on_focus.run(());

        let resolved = switch.state.resolved.get_untracked();
        assert!(resolved.is_disabled);
        assert!(!resolved.is_enabled);
        assert!(!resolved.is_pressed);
        assert!(!resolved.is_hovered);
        assert!(!resolved.is_focused);
        assert!(!resolved.is_focus_visible);
        assert_eq!(switch.attrs.tabindex, -1);
        assert_eq!(switch.attrs.aria_disabled, Some("true"));
    }
}
