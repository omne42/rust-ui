use crate::logic::{self, CircularProgressLogicInput};
use leptos::prelude::*;
use ui_headless::i18n;
use ui_headless::i18n::CommonStrings;
use ui_headless::{A11yDirection, CircularProgressOptions, use_circular_progress};

#[component]
pub fn CircularProgress(
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] size_px: Option<f64>,
    #[prop(optional)] thickness_px: Option<f64>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let logic::CircularProgressLogicOutput {
        state,
        aria_label,
        lang,
        class,
        style_vars,
        agent_contract,
    } = logic::resolve_component_contract(CircularProgressLogicInput {
        aria_label,
        size_px,
        thickness_px,
        class_name,
        lang,
        default_aria_label: common.loading_aria_label.as_ref(),
    });

    let semantics = use_circular_progress(CircularProgressOptions {
        state,
        aria_label,
        lang,
        dir,
    });

    view! {
        <span
            class=class
            style=style_vars
            data-slot="circular-progress"
            data-state=semantics.attrs.data_state
            data-motion=semantics.attrs.data_motion
            data-size=semantics.attrs.data_size
            data-thickness=semantics.attrs.data_thickness
            data-size-source=semantics.attrs.data_size_source
            data-thickness-source=semantics.attrs.data_thickness_source
            data-label-source=semantics.attrs.data_label_source
            data-custom-size=semantics.attrs.data_custom_size
            data-custom-thickness=semantics.attrs.data_custom_thickness
            data-custom-aria-label=semantics.attrs.data_custom_aria_label
            data-custom-class=semantics.attrs.data_custom_class
            data-class-source=semantics.attrs.data_class_source
            data-ui-schema=agent_contract.schema_name
            data-ui-schema-version=agent_contract.schema_version.as_str()
            data-ui-intent=agent_contract.intent.as_str()
            data-ui-action=agent_contract.action.as_str()
            data-ui-state=agent_contract.state.as_str()
            data-ui-source=agent_contract.source.as_str()
            data-ui-size-source=agent_contract.size_source
            data-ui-thickness-source=agent_contract.thickness_source
            data-ui-label-source=agent_contract.label_source
            data-ui-class-source=agent_contract.class_source
            role=semantics.attrs.role
            aria-label=semantics.attrs.aria_label
            aria-valuemin=semantics.attrs.aria_valuemin
            aria-valuemax=semantics.attrs.aria_valuemax
            lang=semantics.attrs.lang
            dir=semantics.attrs.dir
        ></span>
    }
}
