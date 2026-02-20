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
#[path = "test/tabs.rs"]
mod tests;
