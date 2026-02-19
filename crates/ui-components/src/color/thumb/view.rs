use crate::color::swatch::ColorSwatch;
use crate::color::thumb::{
    ColorThumbStateInput,
    logic::{self},
};
use leptos::prelude::*;

#[component]
pub fn ColorThumb(
    id_base: String,
    #[prop(optional, into)] color: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] focused: bool,
    #[prop(optional)] dragging: bool,
    #[prop(optional, default = logic::DEFAULT_POSITION_PERCENT)] x_percent: f32,
    #[prop(optional, default = logic::DEFAULT_POSITION_PERCENT)] y_percent: f32,
    #[prop(optional, default = true)] show_loupe: bool,
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
        logic::resolve_state(ColorThumbStateInput {
            disabled,
            focused,
            dragging,
            show_loupe,
            has_color,
            x_percent,
            y_percent,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    view! {
        <div
            id=id_base
            class=move || class.get()
            role="slider"
            tabindex=if disabled { -1 } else { 0 }
            aria-label=move || aria_label.get_value()
            aria-disabled=move || state.get().is_disabled.then_some("true")
            aria-valuetext=move || color.get_value().unwrap_or_else(|| "None".to_string())
            data-slot="color-thumb"
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-focused=move || state.get().is_focused.then_some("true")
            data-dragging=move || state.get().is_dragging.then_some("true")
            data-loupe-visible=move || state.get().loupe_visible.then_some("true")
            data-has-color=move || state.get().has_color.then_some("true")
            data-x=move || state.get().x_percent
            data-y=move || state.get().y_percent
            data-x-bucket=move || state.get().x_bucket_attr
            data-y-bucket=move || state.get().y_bucket_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
        >
            <span class="ui-color-thumb__handle" data-slot="color-thumb-handle" aria-hidden="true">
                <span class="ui-color-thumb__fill" data-slot="color-thumb-fill">
                    {move || {
                        if let Some(color) = color.get_value() {
                            view! {
                                <ColorSwatch
                                    color=color
                                    is_decorative=true
                                    class_name="ui-color-thumb__swatch".to_string()
                                />
                            }
                                .into_any()
                        } else {
                            view! {
                                <ColorSwatch
                                    is_decorative=true
                                    class_name="ui-color-thumb__swatch".to_string()
                                />
                            }
                                .into_any()
                        }
                    }}
                </span>
            </span>

            <Show when=move || state.get().loupe_visible>
                <span class="ui-color-thumb__loupe" data-slot="color-thumb-loupe" aria-hidden="true">
                    <span class="ui-color-thumb__loupe-fill" data-slot="color-thumb-loupe-fill">
                        {move || {
                            if let Some(color) = color.get_value() {
                                view! {
                                    <ColorSwatch
                                        color=color
                                        is_decorative=true
                                        class_name="ui-color-thumb__loupe-swatch".to_string()
                                    />
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <ColorSwatch
                                        is_decorative=true
                                        class_name="ui-color-thumb__loupe-swatch".to_string()
                                    />
                                }
                                    .into_any()
                            }
                        }}
                    </span>
                </span>
            </Show>
        </div>
    }
}
