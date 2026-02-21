use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A11yDirection {
    Ltr,
    Rtl,
}

impl A11yDirection {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct A11yLocaleAttrs {
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiveRegionPriority {
    Polite,
    Assertive,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LiveRegionA11yAttrs {
    pub role: &'static str,
    pub aria_live: &'static str,
}

#[derive(Clone)]
pub struct ErrorViewA11yAttrs {
    pub role: &'static str,
    pub aria_live: Signal<&'static str>,
    pub aria_hidden: Signal<Option<&'static str>>,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct DisclosureTriggerA11yAttrs {
    pub aria_expanded: Signal<&'static str>,
    pub aria_controls: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct PopupTriggerA11yAttrs {
    pub aria_haspopup: Option<&'static str>,
    pub aria_controls: Signal<Option<String>>,
    pub aria_expanded: Signal<Option<&'static str>>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageFallbackA11yAttrs {
    pub role: Option<&'static str>,
    pub aria_label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LabeledGroupA11yAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LabeledToolbarA11yAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub aria_orientation: &'static str,
    pub aria_disabled: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct OverlayDialogA11yAttrs {
    pub aria_labelledby: Option<String>,
    pub aria_describedby: Option<String>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegionA11yAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavigationA11yAttrs {
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldsetA11yAttrs {
    pub aria_label: String,
    pub aria_disabled: Option<&'static str>,
    pub aria_invalid: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

pub fn is_focusable_element_kind(
    tag_name: &str,
    has_href: bool,
    has_contenteditable: bool,
    tabindex: Option<&str>,
) -> bool {
    match tag_name {
        "button" | "input" | "select" | "textarea" => return true,
        "a" => return has_href,
        _ => {}
    }

    if has_contenteditable {
        return true;
    }

    tabindex
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse::<i32>().ok())
        .is_some_and(|value| value >= 0)
}

pub fn locale_attrs(lang: Option<String>, dir: Option<A11yDirection>) -> A11yLocaleAttrs {
    let lang = lang.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    });

    A11yLocaleAttrs {
        lang,
        dir: dir.map(A11yDirection::as_attr),
    }
}

pub fn live_region_attrs(priority: LiveRegionPriority) -> LiveRegionA11yAttrs {
    match priority {
        LiveRegionPriority::Polite => LiveRegionA11yAttrs {
            role: "status",
            aria_live: "polite",
        },
        LiveRegionPriority::Assertive => LiveRegionA11yAttrs {
            role: "alert",
            aria_live: "assertive",
        },
    }
}

pub fn error_view_attrs(
    is_visible: Signal<bool>,
    aria_label: String,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> ErrorViewA11yAttrs {
    let locale = locale_attrs(lang, dir);
    let live_region = live_region_attrs(LiveRegionPriority::Assertive);
    let visible_for_live = is_visible;
    let visible_for_hidden = is_visible;

    ErrorViewA11yAttrs {
        role: live_region.role,
        aria_live: Signal::derive(move || {
            if visible_for_live.get() {
                live_region.aria_live
            } else {
                "off"
            }
        }),
        aria_hidden: Signal::derive(move || (!visible_for_hidden.get()).then_some("true")),
        aria_label,
        lang: locale.lang,
        dir: locale.dir,
    }
}

pub fn image_fallback_attrs(
    show_image: bool,
    fallback_aria_label: String,
) -> ImageFallbackA11yAttrs {
    if show_image {
        return ImageFallbackA11yAttrs {
            role: None,
            aria_label: None,
        };
    }

    ImageFallbackA11yAttrs {
        role: Some("img"),
        aria_label: Some(fallback_aria_label),
    }
}

pub fn labeled_group_attrs(
    aria_label: String,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> LabeledGroupA11yAttrs {
    let locale = locale_attrs(lang, dir);

    LabeledGroupA11yAttrs {
        role: "group",
        aria_label,
        lang: locale.lang,
        dir: locale.dir,
    }
}

pub fn labeled_toolbar_attrs(
    aria_label: String,
    aria_orientation: &'static str,
    is_disabled: bool,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> LabeledToolbarA11yAttrs {
    let locale = locale_attrs(lang, dir);

    LabeledToolbarA11yAttrs {
        role: "toolbar",
        aria_label,
        aria_orientation,
        aria_disabled: is_disabled.then_some("true"),
        lang: locale.lang,
        dir: locale.dir,
    }
}

pub fn overlay_dialog_attrs(
    aria_labelledby: Option<String>,
    aria_describedby: Option<String>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> OverlayDialogA11yAttrs {
    fn normalize_optional_text(value: Option<String>) -> Option<String> {
        value.and_then(|value| {
            let trimmed = value.trim();
            (!trimmed.is_empty()).then(|| trimmed.into())
        })
    }

    let locale = locale_attrs(lang, dir);

    OverlayDialogA11yAttrs {
        aria_labelledby: normalize_optional_text(aria_labelledby),
        aria_describedby: normalize_optional_text(aria_describedby),
        lang: locale.lang,
        dir: locale.dir,
    }
}

pub fn region_attrs(
    aria_label: String,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> RegionA11yAttrs {
    let locale = locale_attrs(lang, dir);

    RegionA11yAttrs {
        role: "region",
        aria_label,
        lang: locale.lang,
        dir: locale.dir,
    }
}

pub fn navigation_attrs(
    aria_label: String,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> NavigationA11yAttrs {
    let locale = locale_attrs(lang, dir);

    NavigationA11yAttrs {
        aria_label,
        lang: locale.lang,
        dir: locale.dir,
    }
}

pub fn fieldset_attrs(
    aria_label: String,
    is_disabled: bool,
    is_invalid: bool,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> FieldsetA11yAttrs {
    let locale = locale_attrs(lang, dir);

    FieldsetA11yAttrs {
        aria_label,
        aria_disabled: is_disabled.then_some("true"),
        aria_invalid: is_invalid.then_some("true"),
        lang: locale.lang,
        dir: locale.dir,
    }
}

pub fn aria_expanded(open: Signal<bool>) -> Signal<&'static str> {
    Signal::derive(move || if open.get() { "true" } else { "false" })
}

pub fn disclosure_trigger_attrs(
    open: Signal<bool>,
    controls_id: String,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> DisclosureTriggerA11yAttrs {
    let locale = locale_attrs(lang, dir);

    DisclosureTriggerA11yAttrs {
        aria_expanded: aria_expanded(open),
        aria_controls: controls_id,
        lang: locale.lang,
        dir: locale.dir,
    }
}

pub fn aria_controls_when_open(open: Signal<bool>, controls_id: String) -> Signal<Option<String>> {
    Signal::derive(move || open.get().then(|| controls_id.clone()))
}

pub fn popup_trigger_attrs(
    aria_haspopup: Option<&'static str>,
    aria_controls: Option<String>,
    aria_controls_signal: Option<Signal<Option<String>>>,
    aria_expanded: Option<Signal<bool>>,
    lang: Option<String>,
    dir: Option<A11yDirection>,
) -> PopupTriggerA11yAttrs {
    let locale = locale_attrs(lang, dir);
    let controls_fallback = aria_controls.clone();
    let resolved_controls = Signal::derive(move || {
        aria_controls_signal
            .map(|signal| signal.get())
            .unwrap_or_else(|| controls_fallback.clone())
    });
    let resolved_expanded = Signal::derive(move || {
        aria_expanded.map(|signal| if signal.get() { "true" } else { "false" })
    });

    PopupTriggerA11yAttrs {
        aria_haspopup,
        aria_controls: resolved_controls,
        aria_expanded: resolved_expanded,
        lang: locale.lang,
        dir: locale.dir,
    }
}

#[cfg(target_arch = "wasm32")]
fn is_focusable_element(el: &leptos::web_sys::Element) -> bool {
    let tag = el.tag_name().to_ascii_lowercase();
    is_focusable_element_kind(
        &tag,
        el.has_attribute("href"),
        el.has_attribute("contenteditable"),
        el.get_attribute("tabindex").as_deref(),
    )
}

#[cfg(target_arch = "wasm32")]
pub fn should_focus_proxy_button_on_click(
    container: &leptos::web_sys::Element,
    event_target: Option<leptos::web_sys::EventTarget>,
) -> bool {
    use leptos::wasm_bindgen::JsCast;

    let Some(target) = event_target else {
        return false;
    };

    let Some(mut target) = target
        .clone()
        .dyn_into::<leptos::web_sys::Element>()
        .ok()
        .or_else(|| {
            target
                .dyn_into::<leptos::web_sys::Node>()
                .ok()
                .and_then(|node| node.parent_element())
        })
    else {
        return false;
    };

    loop {
        if is_focusable_element(&target) {
            return false;
        }

        if target.is_same_node(Some(container)) {
            return true;
        }

        let Some(parent) = target.parent_element() else {
            return false;
        };
        target = parent;
    }
}

#[cfg(test)]
#[path = "test/a11y.rs"]
mod tests;
