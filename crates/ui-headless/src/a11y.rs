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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aria_expanded_is_false_when_closed() {
        let (open, set_open) = signal(false);
        let expanded = aria_expanded(open.into());

        assert_eq!(expanded.get_untracked(), "false");

        set_open.set(true);
        assert_eq!(expanded.get_untracked(), "true");
    }

    #[test]
    fn locale_attrs_trims_lang_and_maps_dir() {
        let attrs = locale_attrs(Some("  zh-CN  ".to_string()), Some(A11yDirection::Rtl));
        assert_eq!(attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(attrs.dir, Some("rtl"));
    }

    #[test]
    fn locale_attrs_drops_blank_lang() {
        let attrs = locale_attrs(Some("   ".to_string()), None);
        assert_eq!(attrs.lang, None);
        assert_eq!(attrs.dir, None);
    }

    #[test]
    fn live_region_attrs_maps_priority_to_role_and_aria_live() {
        let polite = live_region_attrs(LiveRegionPriority::Polite);
        assert_eq!(polite.role, "status");
        assert_eq!(polite.aria_live, "polite");

        let assertive = live_region_attrs(LiveRegionPriority::Assertive);
        assert_eq!(assertive.role, "alert");
        assert_eq!(assertive.aria_live, "assertive");
    }

    #[test]
    fn region_attrs_maps_role_label_and_locale() {
        let attrs = region_attrs(
            "Notifications".to_string(),
            Some("  en-US ".to_string()),
            Some(A11yDirection::Rtl),
        );

        assert_eq!(attrs.role, "region");
        assert_eq!(attrs.aria_label, "Notifications");
        assert_eq!(attrs.lang.as_deref(), Some("en-US"));
        assert_eq!(attrs.dir, Some("rtl"));
    }

    #[test]
    fn disclosure_trigger_attrs_exposes_typed_aria_and_locale_fields() {
        let (open, set_open) = signal(false);
        let attrs = disclosure_trigger_attrs(
            open.into(),
            "demo-controls".to_string(),
            Some("en-US".to_string()),
            Some(A11yDirection::Ltr),
        );

        assert_eq!(attrs.aria_expanded.get_untracked(), "false");
        assert_eq!(attrs.aria_controls, "demo-controls");
        assert_eq!(attrs.lang.as_deref(), Some("en-US"));
        assert_eq!(attrs.dir, Some("ltr"));

        set_open.set(true);
        assert_eq!(attrs.aria_expanded.get_untracked(), "true");
    }

    #[test]
    fn aria_controls_when_open_is_none_when_closed() {
        let (open, set_open) = signal(false);
        let controls = aria_controls_when_open(open.into(), "demo-controls".to_string());

        assert_eq!(controls.get_untracked(), None);

        set_open.set(true);
        assert_eq!(controls.get_untracked(), Some("demo-controls".to_string()));
    }

    #[test]
    fn popup_trigger_attrs_maps_controls_expanded_and_locale() {
        let (open, set_open) = signal(false);
        let attrs = popup_trigger_attrs(
            Some("dialog"),
            Some("fallback-controls".to_string()),
            None,
            Some(open.into()),
            Some("  en-US ".to_string()),
            Some(A11yDirection::Ltr),
        );

        assert_eq!(attrs.aria_haspopup, Some("dialog"));
        assert_eq!(
            attrs.aria_controls.get_untracked(),
            Some("fallback-controls".to_string())
        );
        assert_eq!(attrs.aria_expanded.get_untracked(), Some("false"));
        assert_eq!(attrs.lang.as_deref(), Some("en-US"));
        assert_eq!(attrs.dir, Some("ltr"));

        set_open.set(true);
        assert_eq!(attrs.aria_expanded.get_untracked(), Some("true"));
    }

    #[test]
    fn popup_trigger_attrs_prefers_signal_controls_when_present() {
        let (controls, set_controls) = signal(Some("signal-controls".to_string()));
        let attrs = popup_trigger_attrs(
            Some("menu"),
            Some("fallback-controls".to_string()),
            Some(controls.into()),
            None,
            None,
            None,
        );

        assert_eq!(
            attrs.aria_controls.get_untracked(),
            Some("signal-controls".to_string())
        );
        assert_eq!(attrs.aria_expanded.get_untracked(), None);

        set_controls.set(Some("next-controls".to_string()));
        assert_eq!(
            attrs.aria_controls.get_untracked(),
            Some("next-controls".to_string())
        );
    }

    #[test]
    fn image_fallback_attrs_only_applies_when_image_is_hidden() {
        let hidden = image_fallback_attrs(false, "Avatar".to_string());
        assert_eq!(hidden.role, Some("img"));
        assert_eq!(hidden.aria_label.as_deref(), Some("Avatar"));

        let shown = image_fallback_attrs(true, "Avatar".to_string());
        assert_eq!(shown.role, None);
        assert_eq!(shown.aria_label, None);
    }

    #[test]
    fn labeled_group_attrs_exposes_typed_group_role_label_and_locale() {
        let attrs = labeled_group_attrs(
            "Collaborators".to_string(),
            Some(" en-US ".to_string()),
            Some(A11yDirection::Rtl),
        );
        assert_eq!(attrs.role, "group");
        assert_eq!(attrs.aria_label, "Collaborators");
        assert_eq!(attrs.lang.as_deref(), Some("en-US"));
        assert_eq!(attrs.dir, Some("rtl"));
    }

    #[test]
    fn overlay_dialog_attrs_trims_ids_and_maps_locale() {
        let attrs = overlay_dialog_attrs(
            Some(" dialog-title ".to_string()),
            Some(" dialog-description ".to_string()),
            Some(" zh-CN ".to_string()),
            Some(A11yDirection::Rtl),
        );

        assert_eq!(attrs.aria_labelledby.as_deref(), Some("dialog-title"));
        assert_eq!(
            attrs.aria_describedby.as_deref(),
            Some("dialog-description")
        );
        assert_eq!(attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(attrs.dir, Some("rtl"));
    }

    #[test]
    fn overlay_dialog_attrs_drops_blank_optional_ids() {
        let attrs = overlay_dialog_attrs(
            Some("   ".to_string()),
            Some("\n\t".to_string()),
            None,
            None,
        );

        assert_eq!(attrs.aria_labelledby, None);
        assert_eq!(attrs.aria_describedby, None);
        assert_eq!(attrs.lang, None);
        assert_eq!(attrs.dir, None);
    }
}
