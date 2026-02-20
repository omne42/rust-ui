use crate::a11y::{A11yDirection, LiveRegionPriority, live_region_attrs, locale_attrs};
use ui_state_primitives::status_light::{StatusLightRole, StatusLightState};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct StatusLightHandlers;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusLightAttrs {
    pub role: Option<&'static str>,
    pub aria_live: Option<&'static str>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_variant: &'static str,
    pub data_state: &'static str,
    pub data_live: Option<&'static str>,
    pub data_static: Option<&'static str>,
    pub data_role: Option<&'static str>,
    pub data_role_source: &'static str,
    pub data_custom_class: Option<&'static str>,
    pub data_class_source: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusLightSemanticState {
    pub variant: &'static str,
    pub state: &'static str,
    pub role_source: &'static str,
    pub class_source: &'static str,
    pub is_live: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusLightContract {
    pub attrs: StatusLightAttrs,
    pub handlers: StatusLightHandlers,
    pub state: StatusLightSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusLightOptions {
    pub state: StatusLightState,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_status_light(options: StatusLightOptions) -> StatusLightContract {
    let locale = locale_attrs(options.lang, options.dir);
    let live_region = options.state.role.map(|role| match role {
        StatusLightRole::Status => live_region_attrs(LiveRegionPriority::Polite),
    });

    StatusLightContract {
        attrs: StatusLightAttrs {
            role: live_region.map(|attrs| attrs.role),
            aria_live: live_region.map(|attrs| attrs.aria_live),
            lang: locale.lang,
            dir: locale.dir,
            data_variant: options.state.variant_attr,
            data_state: options.state.state_attr,
            data_live: options.state.is_live.then_some("true"),
            data_static: (!options.state.is_live).then_some("true"),
            data_role: options.state.role_attr,
            data_role_source: options.state.role_source_attr,
            data_custom_class: options.state.has_custom_class_name.then_some("true"),
            data_class_source: options.state.class_source_attr,
        },
        handlers: StatusLightHandlers,
        state: StatusLightSemanticState {
            variant: options.state.variant_attr,
            state: options.state.state_attr,
            role_source: options.state.role_source_attr,
            class_source: options.state.class_source_attr,
            is_live: options.state.is_live,
            has_custom_class_name: options.state.has_custom_class_name,
        },
    }
}

#[cfg(test)]
#[path = "test/status_light.rs"]
mod tests;
