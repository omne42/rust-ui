use crate::a11y::{A11yDirection, region_attrs};
use ui_state_primitives::surface::SurfaceState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SurfaceHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SurfaceAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_tone: &'static str,
    pub data_elevation: &'static str,
    pub data_state: &'static str,
    pub data_bordered: Option<&'static str>,
    pub data_padded: Option<&'static str>,
    pub data_plain: Option<&'static str>,
    pub data_aria_source: &'static str,
    pub data_custom_class: Option<&'static str>,
    pub data_class_source: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceSemanticState {
    pub tone: &'static str,
    pub elevation: &'static str,
    pub state: &'static str,
    pub aria_source: &'static str,
    pub class_source: &'static str,
    pub is_bordered: bool,
    pub is_padded: bool,
    pub is_plain: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SurfaceContract {
    pub attrs: SurfaceAttrs,
    pub handlers: SurfaceHandlers,
    pub state: SurfaceSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SurfaceOptions {
    pub state: SurfaceState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_surface(options: SurfaceOptions) -> SurfaceContract {
    let region = region_attrs(options.aria_label, options.lang, options.dir);

    SurfaceContract {
        attrs: SurfaceAttrs {
            role: region.role,
            aria_label: region.aria_label,
            lang: region.lang,
            dir: region.dir,
            data_tone: options.state.tone_attr,
            data_elevation: options.state.elevation_attr,
            data_state: options.state.data_state_attr,
            data_bordered: options.state.is_bordered.then_some("true"),
            data_padded: options.state.is_padded.then_some("true"),
            data_plain: options.state.is_plain.then_some("true"),
            data_aria_source: options.state.aria_source_attr,
            data_custom_class: options.state.has_custom_class_name.then_some("true"),
            data_class_source: options.state.class_source_attr,
        },
        handlers: SurfaceHandlers,
        state: SurfaceSemanticState {
            tone: options.state.tone_attr,
            elevation: options.state.elevation_attr,
            state: options.state.data_state_attr,
            aria_source: options.state.aria_source_attr,
            class_source: options.state.class_source_attr,
            is_bordered: options.state.is_bordered,
            is_padded: options.state.is_padded,
            is_plain: options.state.is_plain,
            has_custom_class_name: options.state.has_custom_class_name,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui_state_primitives::surface::{
        SurfaceElevation, SurfaceStateInput, SurfaceTone, resolve_state,
    };

    #[test]
    fn use_surface_maps_region_locale_and_state_markers() {
        let state = resolve_state(SurfaceStateInput {
            tone: SurfaceTone::Strong,
            elevation: SurfaceElevation::Floating,
            bordered: true,
            padded: false,
            has_custom_aria_label: true,
            has_custom_class_name: true,
        });

        let contract = use_surface(SurfaceOptions {
            state,
            aria_label: " Deployment summary ".to_string(),
            lang: Some(" zh-CN ".to_string()),
            dir: Some(A11yDirection::Rtl),
        });

        assert_eq!(contract.attrs.role, "region");
        assert_eq!(contract.attrs.aria_label, " Deployment summary ");
        assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(contract.attrs.dir, Some("rtl"));
        assert_eq!(contract.attrs.data_tone, "strong");
        assert_eq!(contract.attrs.data_elevation, "floating");
        assert_eq!(contract.attrs.data_state, "bordered");
        assert_eq!(contract.attrs.data_bordered, Some("true"));
        assert_eq!(contract.attrs.data_padded, None);
        assert_eq!(contract.attrs.data_plain, None);
        assert_eq!(contract.attrs.data_aria_source, "custom");
        assert_eq!(contract.attrs.data_custom_class, Some("true"));
        assert_eq!(contract.attrs.data_class_source, "custom");
        assert_eq!(contract.state.state, "bordered");
    }

    #[test]
    fn use_surface_exposes_plain_defaults_without_optional_markers() {
        let state = resolve_state(SurfaceStateInput {
            tone: SurfaceTone::Default,
            elevation: SurfaceElevation::Raised,
            bordered: false,
            padded: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        });

        let contract = use_surface(SurfaceOptions {
            state,
            aria_label: "Surface".to_string(),
            lang: None,
            dir: None,
        });

        assert_eq!(contract.attrs.data_tone, "default");
        assert_eq!(contract.attrs.data_elevation, "raised");
        assert_eq!(contract.attrs.data_state, "plain");
        assert_eq!(contract.attrs.data_bordered, None);
        assert_eq!(contract.attrs.data_padded, None);
        assert_eq!(contract.attrs.data_plain, Some("true"));
        assert_eq!(contract.attrs.data_aria_source, "default");
        assert_eq!(contract.attrs.data_custom_class, None);
        assert_eq!(contract.attrs.data_class_source, "default");
        assert_eq!(contract.attrs.lang, None);
        assert_eq!(contract.attrs.dir, None);
    }
}
