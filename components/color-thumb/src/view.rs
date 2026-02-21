use std::borrow::Cow;

use crate::color::swatch::ColorSwatch;
use crate::color::thumb::{
    ColorThumbMotion,
    logic::{self},
    motion,
};
use leptos::prelude::*;
use ui_headless::{A11yDirection, ColorThumbOptions, use_color_thumb};

const SLOT_COLOR_THUMB: &str = "color-thumb";
const SLOT_COLOR_THUMB_HANDLE: &str = "color-thumb-handle";
const SLOT_COLOR_THUMB_FILL: &str = "color-thumb-fill";
const SLOT_COLOR_THUMB_LOUPE: &str = "color-thumb-loupe";
const SLOT_COLOR_THUMB_LOUPE_FILL: &str = "color-thumb-loupe-fill";

const CLASS_COLOR_THUMB_HANDLE: &str = "ui-color-thumb__handle";
const CLASS_COLOR_THUMB_FILL: &str = "ui-color-thumb__fill";
const CLASS_COLOR_THUMB_LOUPE: &str = "ui-color-thumb__loupe";
const CLASS_COLOR_THUMB_LOUPE_FILL: &str = "ui-color-thumb__loupe-fill";
const CLASS_COLOR_THUMB_SWATCH: &str = "ui-color-thumb__swatch";
const CLASS_COLOR_THUMB_LOUPE_SWATCH: &str = "ui-color-thumb__loupe-swatch";

const BOOL_TRUE: &str = "true";

fn render_decorative_swatch(color: Option<String>, class_name: &str) -> AnyView {
    let class_name = Cow::Borrowed(class_name).into_owned();
    match color {
        Some(color) => view! {
            <ColorSwatch color=color is_decorative=true class_name=class_name.clone() />
        }
        .into_any(),
        None => view! {
            <ColorSwatch is_decorative=true class_name=class_name />
        }
        .into_any(),
    }
}

#[component]
pub fn ColorThumb(
    id_base: String,
    #[prop(optional, into)] color: Option<String>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_focused: bool,
    #[prop(optional)] is_dragging: bool,
    #[prop(optional)] x_percent: Option<f32>,
    #[prop(optional)] y_percent: Option<f32>,
    #[prop(optional)] is_loupe_visible: Option<bool>,
    #[prop(optional)] motion: ColorThumbMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] aria_value_text: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let color = logic::sanitize_color(color);
    let has_color = color.is_some();
    let (aria_value_text, aria_value_text_source) =
        logic::normalize_aria_value_text(aria_value_text, color.clone());
    let color = StoredValue::new(color);
    let aria_value_text = StoredValue::new(aria_value_text);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let aria_label = StoredValue::new(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);
    let lang = StoredValue::new(logic::normalize_optional_text(lang));
    let dir = StoredValue::new(dir);
    let motion = motion::sanitize_motion(motion);
    let motion_source = motion::source_attr(motion);
    let agent_contract = logic::resolve_agent_contract();
    let style_vars = StoredValue::new(motion::attach_motion(None, motion));

    let state = Memo::new(move |_| {
        logic::resolve_component_state(logic::ColorThumbLogicInput {
            interaction_state: logic::interaction_state_from_flags(
                is_disabled,
                is_focused,
                is_dragging,
            ),
            is_loupe_visible,
            has_color,
            x_percent,
            y_percent,
            has_custom_aria_label,
            aria_value_text_source,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let ui_action = Memo::new(move |_| logic::resolve_ui_action(state.get()).as_attr());
    let semantics = Signal::derive(move || {
        use_color_thumb(ColorThumbOptions {
            state: state.get(),
            aria_label: aria_label.get_value(),
            aria_value_text: aria_value_text.get_value(),
            lang: lang.get_value(),
            dir: dir.get_value(),
        })
    });

    view! {
        <div
            id=id_base
            class=move || class.get()
            style=move || style_vars.get_value()
            role=move || semantics.get().root_attrs.role
            tabindex=move || semantics.get().root_attrs.tabindex
            aria-label=move || semantics.get().root_attrs.aria_label
            aria-disabled=move || semantics.get().root_attrs.aria_disabled
            aria-valuetext=move || semantics.get().root_attrs.aria_valuetext
            lang=move || semantics.get().root_attrs.lang
            dir=move || semantics.get().root_attrs.dir
            data-slot=SLOT_COLOR_THUMB
            data-state=move || semantics.get().root_attrs.data_state
            data-disabled=move || semantics.get().root_attrs.data_disabled
            data-focused=move || semantics.get().root_attrs.data_focused
            data-dragging=move || semantics.get().root_attrs.data_dragging
            data-loupe-visible=move || semantics.get().root_attrs.data_loupe_visible
            data-has-color=move || semantics.get().root_attrs.data_has_color
            data-x=move || semantics.get().root_attrs.data_x
            data-y=move || semantics.get().root_attrs.data_y
            data-x-bucket=move || semantics.get().root_attrs.data_x_bucket
            data-y-bucket=move || semantics.get().root_attrs.data_y_bucket
            data-interaction-source=move || semantics.get().root_attrs.data_interaction_source
            data-aria-source=move || semantics.get().root_attrs.data_aria_source
            data-aria-valuetext-source=move || semantics.get().root_attrs.data_aria_valuetext_source
            data-custom-class=move || semantics.get().root_attrs.data_custom_class
            data-class-source=move || semantics.get().root_attrs.data_class_source
            data-loupe-source=move || semantics.get().root_attrs.data_loupe_source
            data-x-source=move || semantics.get().root_attrs.data_x_source
            data-y-source=move || semantics.get().root_attrs.data_y_source
            data-motion-source=motion_source
            data-custom-motion=(motion_source == "custom").then_some("true")
            data-ui-schema=agent_contract.schema_attr
            data-ui-schema-version=agent_contract.schema_version_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action=move || ui_action.get()
            data-ui-state=move || semantics.get().root_attrs.data_state
            data-ui-source=move || semantics.get().root_attrs.data_interaction_source
            data-ui-stream-support=agent_contract.stream_support_attr
            data-ui-stream-fallback=agent_contract.stream_fallback_attr
            data-ui-output-status=agent_contract.output_status_attr
            on:pointerdown=move |_| semantics.get().handlers.on_pointer_down.run(())
            on:pointerup=move |_| semantics.get().handlers.on_pointer_up.run(())
            on:pointercancel=move |_| semantics.get().handlers.on_pointer_cancel.run(())
            on:focus=move |_| semantics.get().handlers.on_focus.run(())
            on:blur=move |_| semantics.get().handlers.on_blur.run(())
            on:keydown=move |ev| {
                if semantics.get().handlers.on_key_down.run(ev.key()) {
                    ev.prevent_default();
                }
            }
        >
            <span class=CLASS_COLOR_THUMB_HANDLE data-slot=SLOT_COLOR_THUMB_HANDLE aria-hidden=BOOL_TRUE>
                <span class=CLASS_COLOR_THUMB_FILL data-slot=SLOT_COLOR_THUMB_FILL>
                    {move || render_decorative_swatch(color.get_value(), CLASS_COLOR_THUMB_SWATCH)}
                </span>
            </span>

            <span class=CLASS_COLOR_THUMB_LOUPE data-slot=SLOT_COLOR_THUMB_LOUPE aria-hidden=BOOL_TRUE>
                <span class=CLASS_COLOR_THUMB_LOUPE_FILL data-slot=SLOT_COLOR_THUMB_LOUPE_FILL>
                    {move || {
                        render_decorative_swatch(color.get_value(), CLASS_COLOR_THUMB_LOUPE_SWATCH)
                    }}
                </span>
            </span>
        </div>
    }
}
