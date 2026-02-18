use crate::a11y::{A11yDirection, region_attrs};
use ui_state_primitives::aspect_ratio::AspectRatioState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct AspectRatioHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AspectRatioAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_ratio: &'static str,
    pub data_radius: &'static str,
    pub data_bordered: Option<&'static str>,
    pub data_fill: Option<&'static str>,
    pub data_state: &'static str,
    pub data_aria_source: &'static str,
    pub data_custom_class: Option<&'static str>,
    pub data_class_source: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AspectRatioSemanticState {
    pub ratio: &'static str,
    pub radius: &'static str,
    pub state: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
    pub is_bordered: bool,
    pub is_fill: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AspectRatioContract {
    pub attrs: AspectRatioAttrs,
    pub handlers: AspectRatioHandlers,
    pub state: AspectRatioSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AspectRatioOptions {
    pub state: AspectRatioState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_aspect_ratio(options: AspectRatioOptions) -> AspectRatioContract {
    let region = region_attrs(options.aria_label, options.lang, options.dir);

    AspectRatioContract {
        attrs: AspectRatioAttrs {
            role: region.role,
            aria_label: region.aria_label,
            lang: region.lang,
            dir: region.dir,
            data_ratio: options.state.ratio_attr,
            data_radius: options.state.radius_attr,
            data_bordered: options.state.is_bordered.then_some("true"),
            data_fill: options.state.is_fill.then_some("true"),
            data_state: options.state.data_state_attr,
            data_aria_source: options.state.aria_source_attr,
            data_custom_class: options.state.has_custom_class_name.then_some("true"),
            data_class_source: options.state.class_source_attr,
        },
        handlers: AspectRatioHandlers,
        state: AspectRatioSemanticState {
            ratio: options.state.ratio_attr,
            radius: options.state.radius_attr,
            state: options.state.data_state_attr,
            aria_source: options.state.aria_source_attr,
            class_source: options.state.class_source_attr,
            is_bordered: options.state.is_bordered,
            is_fill: options.state.is_fill,
            has_custom_class_name: options.state.has_custom_class_name,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_state_primitives::aspect_ratio::{
        AspectRatioPreset, AspectRatioRadius, AspectRatioStateInput, resolve_state,
    };

    #[test]
    fn use_aspect_ratio_maps_region_locale_and_state_markers() {
        let state = resolve_state(AspectRatioStateInput {
            ratio: AspectRatioPreset::UltraWide,
            radius: AspectRatioRadius::Lg,
            bordered: true,
            fill: true,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        let contract = use_aspect_ratio(AspectRatioOptions {
            state,
            aria_label: " Trailer preview ".to_string(),
            lang: Some(" zh-CN ".to_string()),
            dir: Some(A11yDirection::Rtl),
        });

        assert_eq!(contract.attrs.role, "region");
        assert_eq!(contract.attrs.aria_label, " Trailer preview ");
        assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(contract.attrs.dir, Some("rtl"));
        assert_eq!(contract.attrs.data_ratio, "ultra-wide");
        assert_eq!(contract.attrs.data_radius, "lg");
        assert_eq!(contract.attrs.data_bordered, Some("true"));
        assert_eq!(contract.attrs.data_fill, Some("true"));
        assert_eq!(contract.attrs.data_state, "media");
        assert_eq!(contract.attrs.data_aria_source, "custom");
        assert_eq!(contract.attrs.data_custom_class, Some("true"));
        assert_eq!(contract.attrs.data_class_source, "custom");
    }

    #[test]
    fn use_aspect_ratio_omits_optional_markers_for_plain_defaults() {
        let state = resolve_state(AspectRatioStateInput {
            ratio: AspectRatioPreset::Video,
            radius: AspectRatioRadius::None,
            bordered: false,
            fill: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        });

        let contract = use_aspect_ratio(AspectRatioOptions {
            state,
            aria_label: "Aspect ratio frame".to_string(),
            lang: None,
            dir: None,
        });

        assert_eq!(contract.attrs.data_ratio, "video");
        assert_eq!(contract.attrs.data_radius, "none");
        assert_eq!(contract.attrs.data_bordered, None);
        assert_eq!(contract.attrs.data_fill, None);
        assert_eq!(contract.attrs.data_state, "plain");
        assert_eq!(contract.attrs.data_aria_source, "default");
        assert_eq!(contract.attrs.data_custom_class, None);
        assert_eq!(contract.attrs.data_class_source, "default");
    }
}
