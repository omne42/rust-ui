use crate::a11y::{A11yDirection, locale_attrs};
use leptos::prelude::*;
use ui_state_primitives::tabs::{
    TabsKeyboardActivation, TabsSelectionTrigger, resolve_next_selected_index,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TabsInteractionKind {
    Focus,
    Press,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TabsListA11yAttrs {
    pub role: &'static str,
    pub aria_label: Option<String>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct TabsTabA11yAttrs {
    pub role: &'static str,
    pub aria_selected: Signal<&'static str>,
    pub aria_controls: String,
    pub aria_disabled: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

pub fn tabs_list_a11y_attrs(
    aria_label: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> TabsListA11yAttrs {
    let locale = locale_attrs(lang, dir);
    let aria_label = aria_label.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    });

    TabsListA11yAttrs {
        role: "tablist",
        aria_label,
        lang: locale.lang,
        dir: locale.dir,
    }
}

pub fn tabs_tab_a11y_attrs(
    is_selected: Signal<bool>,
    controls_id: String,
    is_disabled: bool,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> TabsTabA11yAttrs {
    let locale = locale_attrs(lang, dir);

    TabsTabA11yAttrs {
        role: "tab",
        aria_selected: Signal::derive(move || if is_selected.get() { "true" } else { "false" }),
        aria_controls: controls_id,
        aria_disabled: is_disabled.then_some("true"),
        lang: locale.lang,
        dir: locale.dir,
    }
}

pub fn resolve_tabs_selection_intent(
    current: usize,
    candidate: usize,
    item_count: usize,
    is_disabled: impl Fn(usize) -> bool,
    keyboard_activation: TabsKeyboardActivation,
    interaction: TabsInteractionKind,
) -> Option<usize> {
    let trigger = match interaction {
        TabsInteractionKind::Focus => TabsSelectionTrigger::Focus,
        TabsInteractionKind::Press => TabsSelectionTrigger::Press,
    };

    let next = resolve_next_selected_index(
        current,
        candidate,
        item_count,
        is_disabled,
        keyboard_activation,
        trigger,
    );

    (next != current).then_some(next)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_selection_intent_respects_keyboard_activation() {
        let current = 0;
        let candidate = 1;
        let is_disabled = |_| false;

        let manual_focus = resolve_tabs_selection_intent(
            current,
            candidate,
            3,
            is_disabled,
            TabsKeyboardActivation::Manual,
            TabsInteractionKind::Focus,
        );
        assert_eq!(manual_focus, None);

        let automatic_focus = resolve_tabs_selection_intent(
            current,
            candidate,
            3,
            is_disabled,
            TabsKeyboardActivation::Automatic,
            TabsInteractionKind::Focus,
        );
        assert_eq!(automatic_focus, Some(candidate));
    }

    #[test]
    fn resolve_selection_intent_ignores_disabled_candidate() {
        let next = resolve_tabs_selection_intent(
            0,
            1,
            3,
            |idx| idx == 1,
            TabsKeyboardActivation::Automatic,
            TabsInteractionKind::Press,
        );

        assert_eq!(next, None);
    }

    #[test]
    fn list_attrs_expose_locale_and_trimmed_label() {
        let attrs = tabs_list_a11y_attrs(
            Some("  Main tabs  ".to_string()),
            Some("  en-US ".to_string()),
            Some(A11yDirection::Rtl),
        );

        assert_eq!(attrs.role, "tablist");
        assert_eq!(attrs.aria_label.as_deref(), Some("Main tabs"));
        assert_eq!(attrs.lang.as_deref(), Some("en-US"));
        assert_eq!(attrs.dir, Some("rtl"));
    }

    #[test]
    fn tab_attrs_track_selected_and_locale() {
        let (selected, set_selected) = signal(false);
        let attrs = tabs_tab_a11y_attrs(
            selected.into(),
            "panel-1".to_string(),
            true,
            Some(" zh-CN ".to_string()),
            Some(A11yDirection::Ltr),
        );

        assert_eq!(attrs.role, "tab");
        assert_eq!(attrs.aria_selected.get_untracked(), "false");
        assert_eq!(attrs.aria_controls, "panel-1");
        assert_eq!(attrs.aria_disabled, Some("true"));
        assert_eq!(attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(attrs.dir, Some("ltr"));

        set_selected.set(true);
        assert_eq!(attrs.aria_selected.get_untracked(), "true");
    }
}
