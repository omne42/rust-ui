use crate::color::handle::{
    ColorHandleStateInput,
    logic::{self},
    motion::{self, ColorHandleMotion},
};
use crate::color::thumb::ColorThumb;
use leptos::prelude::*;
use ui_headless::{A11yDirection, labeled_group_attrs};

#[component]
pub fn ColorHandle(
    id_base: String,
    #[prop(optional, into)] color: Option<String>,
    #[prop(optional, into)] is_disabled: Option<bool>,
    #[prop(optional, into)] is_focused: Option<bool>,
    #[prop(optional, into)] is_dragging: Option<bool>,
    #[prop(optional, into)] is_loupe_visible: Option<bool>,
    #[prop(optional, into)] x_percent: Option<f32>,
    #[prop(optional, into)] y_percent: Option<f32>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] motion: Option<ColorHandleMotion>,
) -> impl IntoView {
    let props = logic::resolve_props(logic::ColorHandlePropsInput {
        is_disabled,
        is_focused,
        is_dragging,
        is_loupe_visible,
        x_percent,
        y_percent,
        motion,
    });
    let props = StoredValue::new(props);

    let color = logic::sanitize_color(color);
    let has_color = color.is_some();
    let color = StoredValue::new(color);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let a11y = StoredValue::new(labeled_group_attrs(aria_label, lang, dir));

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);
    let motion_source_attr = motion::source_attr(props.get_value().motion);
    let style = StoredValue::new(motion::attach_motion(None, props.get_value().motion));

    let state = Memo::new(move |_| {
        let props = props.get_value();
        logic::resolve_state(ColorHandleStateInput {
            is_disabled: props.interaction_state.is_disabled(),
            is_focused: props.interaction_state.is_focused(),
            is_dragging: props.interaction_state.is_dragging(),
            is_loupe_visible: props.is_loupe_visible,
            has_color,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let agent_contract =
        Memo::new(move |_| logic::resolve_agent_contract(state.get(), motion_source_attr));

    let thumb_id = format!("{id_base}-thumb");

    view! {
        <div
            id=id_base
            class=move || class.get()
            style=move || style.get_value()
            role=move || a11y.get_value().role
            aria-label=move || a11y.get_value().aria_label
            lang=move || a11y.get_value().lang
            dir=move || a11y.get_value().dir
            data-slot="color-handle"
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-focused=move || state.get().is_focused.then_some("true")
            data-dragging=move || state.get().is_dragging.then_some("true")
            data-loupe-visible=move || state.get().loupe_visible.then_some("true")
            data-has-color=move || state.get().has_color.then_some("true")
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=motion_source_attr
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
            data-ui-stream-mode="snapshot"
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
            data-ui-capability-drag=move || {
                agent_contract.get().capabilities.can_drag.then_some("true")
            }
            data-ui-capability-focus=move || {
                agent_contract.get().capabilities.can_focus.then_some("true")
            }
        >
            <div class="ui-color-handle__surface" data-slot="color-handle-surface" aria-hidden="true">
                {move || {
                    let props = props.get_value();
                    let aria_label = a11y.get_value().aria_label;

                    match color.get_value() {
                        Some(color) => view! {
                            <ColorThumb
                                id_base=thumb_id.clone()
                                color=color
                                is_disabled=props.interaction_state.is_disabled()
                                is_focused=props.interaction_state.is_focused()
                                is_dragging=props.interaction_state.is_dragging()
                                is_loupe_visible=props.is_loupe_visible
                                x_percent=props.x_percent
                                y_percent=props.y_percent
                                aria_label=aria_label.clone()
                                class_name="ui-color-handle__thumb".to_string()
                            />
                        }
                        .into_any(),
                        None => view! {
                            <ColorThumb
                                id_base=thumb_id.clone()
                                is_disabled=props.interaction_state.is_disabled()
                                is_focused=props.interaction_state.is_focused()
                                is_dragging=props.interaction_state.is_dragging()
                                is_loupe_visible=props.is_loupe_visible
                                x_percent=props.x_percent
                                y_percent=props.y_percent
                                aria_label=aria_label
                                class_name="ui-color-handle__thumb".to_string()
                            />
                        }
                        .into_any(),
                    }
                }}
            </div>
        </div>
    }
}
