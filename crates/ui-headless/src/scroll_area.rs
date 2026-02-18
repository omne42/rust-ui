use crate::a11y::{A11yDirection, region_attrs};
use ui_state_primitives::scroll_area::ScrollAreaState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ScrollAreaHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScrollAreaRootAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_orientation: &'static str,
    pub data_disabled: Option<&'static str>,
    pub data_max_height: &'static str,
    pub data_aria_source: &'static str,
    pub data_class_source: &'static str,
    pub data_custom_class: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollAreaViewportAttrs {
    pub tabindex: i32,
    pub aria_disabled: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScrollAreaSemanticState {
    pub orientation: &'static str,
    pub is_disabled: bool,
    pub has_custom_max_height: bool,
    pub max_height_source: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScrollAreaContract {
    pub root_attrs: ScrollAreaRootAttrs,
    pub viewport_attrs: ScrollAreaViewportAttrs,
    pub handlers: ScrollAreaHandlers,
    pub state: ScrollAreaSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScrollAreaOptions {
    pub state: ScrollAreaState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_scroll_area(options: ScrollAreaOptions) -> ScrollAreaContract {
    let region = region_attrs(options.aria_label, options.lang, options.dir);
    let state = options.state;

    ScrollAreaContract {
        root_attrs: ScrollAreaRootAttrs {
            role: region.role,
            aria_label: region.aria_label,
            lang: region.lang,
            dir: region.dir,
            data_orientation: state.orientation_attr,
            data_disabled: state.disabled.then_some("true"),
            data_max_height: state.max_height_attr.as_attr(),
            data_aria_source: state.aria_source_attr.as_attr(),
            data_class_source: state.class_source_attr.as_attr(),
            data_custom_class: state.has_custom_class_name.then_some("true"),
        },
        viewport_attrs: ScrollAreaViewportAttrs {
            tabindex: if state.disabled { -1 } else { 0 },
            aria_disabled: state.disabled.then_some("true"),
        },
        handlers: ScrollAreaHandlers,
        state: ScrollAreaSemanticState {
            orientation: state.orientation_attr,
            is_disabled: state.disabled,
            has_custom_max_height: state.has_custom_max_height,
            max_height_source: state.max_height_attr.as_attr(),
            aria_source: state.aria_source_attr.as_attr(),
            class_source: state.class_source_attr.as_attr(),
            has_custom_class_name: state.has_custom_class_name,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_state_primitives::scroll_area::{
        ScrollAreaOrientation, ScrollAreaStateInput, resolve_state,
    };

    #[test]
    fn use_scroll_area_maps_region_locale_and_state_markers() {
        let state = resolve_state(ScrollAreaStateInput {
            orientation: ScrollAreaOrientation::Horizontal,
            disabled: true,
            max_height_px: Some(180),
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        let contract = use_scroll_area(ScrollAreaOptions {
            state,
            aria_label: " Activity feed ".to_string(),
            lang: Some(" zh-CN ".to_string()),
            dir: Some(A11yDirection::Rtl),
        });

        assert_eq!(contract.root_attrs.role, "region");
        assert_eq!(contract.root_attrs.aria_label, " Activity feed ");
        assert_eq!(contract.root_attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(contract.root_attrs.dir, Some("rtl"));
        assert_eq!(contract.root_attrs.data_orientation, "horizontal");
        assert_eq!(contract.root_attrs.data_disabled, Some("true"));
        assert_eq!(contract.root_attrs.data_max_height, "custom");
        assert_eq!(contract.root_attrs.data_aria_source, "custom");
        assert_eq!(contract.root_attrs.data_class_source, "custom");
        assert_eq!(contract.root_attrs.data_custom_class, Some("true"));

        assert_eq!(contract.viewport_attrs.tabindex, -1);
        assert_eq!(contract.viewport_attrs.aria_disabled, Some("true"));

        assert_eq!(contract.state.orientation, "horizontal");
        assert!(contract.state.is_disabled);
        assert!(contract.state.has_custom_max_height);
        assert_eq!(contract.state.max_height_source, "custom");
        assert_eq!(contract.state.aria_source, "custom");
        assert_eq!(contract.state.class_source, "custom");
        assert!(contract.state.has_custom_class_name);
    }

    #[test]
    fn use_scroll_area_keeps_defaults_without_optional_markers() {
        let state = resolve_state(ScrollAreaStateInput {
            orientation: ScrollAreaOrientation::Vertical,
            disabled: false,
            max_height_px: None,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        });

        let contract = use_scroll_area(ScrollAreaOptions {
            state,
            aria_label: "Scrollable region".to_string(),
            lang: None,
            dir: None,
        });

        assert_eq!(contract.root_attrs.data_orientation, "vertical");
        assert_eq!(contract.root_attrs.data_disabled, None);
        assert_eq!(contract.root_attrs.data_max_height, "default");
        assert_eq!(contract.root_attrs.data_aria_source, "default");
        assert_eq!(contract.root_attrs.data_class_source, "default");
        assert_eq!(contract.root_attrs.data_custom_class, None);
        assert_eq!(contract.root_attrs.lang, None);
        assert_eq!(contract.root_attrs.dir, None);

        assert_eq!(contract.viewport_attrs.tabindex, 0);
        assert_eq!(contract.viewport_attrs.aria_disabled, None);
        assert!(!contract.state.is_disabled);
        assert!(!contract.state.has_custom_max_height);
    }
}
