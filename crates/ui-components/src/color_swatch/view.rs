use crate::color_swatch::{
    ColorSwatchRounding, ColorSwatchShape, ColorSwatchSize, ColorSwatchStateInput,
    logic::{self},
};
use leptos::prelude::*;

#[component]
pub fn ColorSwatch(
    #[prop(optional, into)] color: Option<String>,
    #[prop(optional, into)] color_name: Option<String>,
    #[prop(optional)] size: ColorSwatchSize,
    #[prop(optional)] rounding: ColorSwatchRounding,
    #[prop(optional)] shape: ColorSwatchShape,
    #[prop(optional, default = true)] bordered: bool,
    #[prop(optional, default = false)] decorative: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let color = logic::sanitize_color_value(color);
    let alpha = logic::resolve_alpha(color.as_deref());
    let (aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, color_name, color.as_deref(), alpha);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    let state = logic::resolve_state(ColorSwatchStateInput {
        size,
        rounding,
        shape,
        bordered,
        alpha,
        has_color: color.is_some(),
        has_custom_aria_label,
        has_custom_class_name,
    });

    let class = logic::compose_class_name(class_name, state);

    view! {
        <div
            class=class
            role=(!decorative).then_some("img")
            aria-label=(!decorative).then_some(aria_label)
            aria-hidden=decorative.then_some("true")
            style=logic::compose_inline_style(color.as_deref()).unwrap_or_default()
            data-slot="color-swatch"
            data-size=state.size_attr
            data-rounding=state.rounding_attr
            data-shape=state.shape_attr
            data-alpha=state.alpha_attr
            data-state=state.data_state_attr
            data-has-color=state.has_color.then_some("true")
            data-bordered=state.is_bordered.then_some("true")
            data-decorative=decorative.then_some("true")
            data-aria-source=state.aria_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
        >
            <span class="ui-color-swatch__checker" data-slot="color-swatch-checker" aria-hidden="true"></span>
            <span class="ui-color-swatch__sample" data-slot="color-swatch-sample" aria-hidden="true"></span>
            <span class="ui-color-swatch__slash" data-slot="color-swatch-slash" aria-hidden="true"></span>
        </div>
    }
}
