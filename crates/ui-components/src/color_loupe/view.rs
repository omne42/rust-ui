use crate::color_loupe::{
    ColorLoupeStateInput,
    logic::{self},
};
use crate::color_swatch::ColorSwatch;
use leptos::prelude::*;

#[component]
pub fn ColorLoupe(
    id_base: String,
    #[prop(optional, into)] color: Option<String>,
    #[prop(optional)] open: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, default = logic::DEFAULT_POSITION_PERCENT)] x_percent: f32,
    #[prop(optional, default = logic::DEFAULT_POSITION_PERCENT)] y_percent: f32,
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
        logic::resolve_state(ColorLoupeStateInput {
            open,
            disabled,
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
            role="img"
            aria-label=move || aria_label.get_value()
            data-slot="color-loupe"
            data-state=move || state.get().data_state_attr
            data-open=move || state.get().is_open.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-color=move || state.get().has_color.then_some("true")
            data-x=move || state.get().x_percent.to_string()
            data-y=move || state.get().y_percent.to_string()
            data-x-bucket=move || state.get().x_bucket_attr
            data-y-bucket=move || state.get().y_bucket_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
        >
            <span class="ui-color-loupe__bubble" data-slot="color-loupe-bubble" aria-hidden="true">
                <span class="ui-color-loupe__checker" data-slot="color-loupe-checker"></span>
                <span class="ui-color-loupe__fill" data-slot="color-loupe-fill">
                    {move || {
                        if let Some(color) = color.get_value() {
                            view! {
                                <ColorSwatch
                                    color=color
                                    decorative=true
                                    class_name="ui-color-loupe__swatch".to_string()
                                />
                            }
                                .into_any()
                        } else {
                            view! {                                    <ColorSwatch
                                        decorative=true
                                        class_name="ui-color-loupe__swatch".to_string()
                                    />
                            }
                                .into_any()
                        }
                    }}
                </span>
            </span>
            <span class="ui-color-loupe__tail" data-slot="color-loupe-tail" aria-hidden="true"></span>
        </div>
    }
}
