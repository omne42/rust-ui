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
pub struct TooltipPanelA11yAttrs {
    pub role: &'static str,
    pub id: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TooltipPanelA11yHandlers;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TooltipPanelA11yState {
    pub is_open: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TooltipPanelA11yContract {
    pub attrs: TooltipPanelA11yAttrs,
    pub handlers: TooltipPanelA11yHandlers,
    pub state: TooltipPanelA11yState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TooltipPanelA11yOptions {
    pub tooltip_id: String,
    pub is_open: bool,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ProgressbarA11yHandlers;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressbarA11yPhase {
    Determinate,
    Indeterminate,
}

impl ProgressbarA11yPhase {
    pub const fn as_attr(self) -> &'static str {
        match self {
            Self::Determinate => "determinate",
            Self::Indeterminate => "indeterminate",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgressbarA11yAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub aria_valuemin: String,
    pub aria_valuemax: String,
    pub aria_valuenow: Option<String>,
    pub aria_valuetext: Option<String>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_state: &'static str,
    pub data_indeterminate: Option<&'static str>,
    pub data_determinate: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgressbarA11yState {
    pub phase: ProgressbarA11yPhase,
    pub is_indeterminate: bool,
    pub is_determinate: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgressbarA11yContract {
    pub attrs: ProgressbarA11yAttrs,
    pub handlers: ProgressbarA11yHandlers,
    pub state: ProgressbarA11yState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProgressbarA11yOptions {
    pub aria_label: String,
    pub aria_valuemin: f64,
    pub aria_valuemax: f64,
    pub aria_valuenow: Option<f64>,
    pub aria_valuetext: Option<String>,
    pub is_indeterminate: bool,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
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

pub fn tooltip_panel_attrs(options: TooltipPanelA11yOptions) -> TooltipPanelA11yContract {
    let locale = locale_attrs(options.lang, options.dir);

    TooltipPanelA11yContract {
        attrs: TooltipPanelA11yAttrs {
            role: "tooltip",
            id: options.tooltip_id,
            lang: locale.lang,
            dir: locale.dir,
        },
        handlers: TooltipPanelA11yHandlers,
        state: TooltipPanelA11yState {
            is_open: options.is_open,
        },
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

pub fn progressbar_attrs(options: ProgressbarA11yOptions) -> ProgressbarA11yContract {
    fn finite_or(default_value: f64, value: f64) -> f64 {
        if value.is_finite() {
            value
        } else {
            default_value
        }
    }

    let locale = locale_attrs(options.lang, options.dir);
    let phase = if options.is_indeterminate {
        ProgressbarA11yPhase::Indeterminate
    } else {
        ProgressbarA11yPhase::Determinate
    };

    let aria_valuemin = finite_or(0.0, options.aria_valuemin).to_string();
    let aria_valuemax = finite_or(100.0, options.aria_valuemax).to_string();
    let aria_valuenow = if phase == ProgressbarA11yPhase::Indeterminate {
        None
    } else {
        options
            .aria_valuenow
            .filter(|value| value.is_finite())
            .map(|value| value.to_string())
    };

    ProgressbarA11yContract {
        attrs: ProgressbarA11yAttrs {
            role: "progressbar",
            aria_label: options.aria_label,
            aria_valuemin,
            aria_valuemax,
            aria_valuenow,
            aria_valuetext: options.aria_valuetext,
            lang: locale.lang,
            dir: locale.dir,
            data_state: phase.as_attr(),
            data_indeterminate: (phase == ProgressbarA11yPhase::Indeterminate).then_some("true"),
            data_determinate: (phase == ProgressbarA11yPhase::Determinate).then_some("true"),
        },
        handlers: ProgressbarA11yHandlers,
        state: ProgressbarA11yState {
            phase,
            is_indeterminate: phase == ProgressbarA11yPhase::Indeterminate,
            is_determinate: phase == ProgressbarA11yPhase::Determinate,
        },
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
