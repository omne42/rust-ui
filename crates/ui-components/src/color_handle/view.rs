use crate::ColorThumb;
use crate::color_handle::{
    ColorHandleStateInput,
    logic::{self},
};
use leptos::prelude::*;

#[component]
pub fn ColorHandle(
    id_base: String,
    #[prop(optional, into)] color: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] focused: bool,
    #[prop(optional)] dragging: bool,
    #[prop(optional, default = true)] show_loupe: bool,
    #[prop(optional, default = 50.0)] x_percent: f32,
    #[prop(optional, default = 50.0)] y_percent: f32,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let color = logic::sanitize_color(color);
    let has_color = color.is_some();
    let color = StoredValue::new(color);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let aria_label = StoredValue::new(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_state(ColorHandleStateInput {
            disabled,
            focused,
            dragging,
            show_loupe,
            has_color,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let thumb_id = format!("{id_base}-thumb");

    view! {
        <div
            id=id_base
            class=move || class.get()
            role="group"
            aria-label=move || aria_label.get_value()
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
        >
            <div class="ui-color-handle__surface" data-slot="color-handle-surface" aria-hidden="true">
                <ColorThumb
                    id_base=thumb_id
                    color=color.get_value().unwrap_or_default()
                    disabled=disabled
                    focused=focused
                    dragging=dragging
                    show_loupe=show_loupe
                    x_percent=x_percent
                    y_percent=y_percent
                    aria_label=aria_label.get_value()
                    class_name="ui-color-handle__thumb".to_string()
                />
            </div>
        </div>
    }
}
