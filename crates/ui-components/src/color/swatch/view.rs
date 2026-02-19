use crate::color::swatch::{
    ColorSwatchMotion, ColorSwatchRounding, ColorSwatchShape, ColorSwatchSize,
    ColorSwatchStateInput,
    logic::{self},
    motion,
};
use leptos::{html, prelude::*};
use ui_headless::a11y::{A11yDirection, locale_attrs};

#[component]
pub fn ColorSwatch(
    #[prop(optional, into)] color: Option<String>,
    #[prop(optional, into)] color_name: Option<String>,
    #[prop(optional)] size: ColorSwatchSize,
    #[prop(optional)] rounding: ColorSwatchRounding,
    #[prop(optional)] shape: ColorSwatchShape,
    #[prop(optional, into)] is_bordered: Option<bool>,
    #[prop(optional, into)] is_decorative: Option<bool>,
    #[prop(optional)] motion: ColorSwatchMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let locale = locale_attrs(lang, dir);
    let color = logic::sanitize_color_value(color);
    let alpha = logic::resolve_alpha(color.as_deref());
    let (aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, color_name, color.as_deref(), alpha);
    let (is_bordered, bordered_source) = logic::normalize_is_bordered(is_bordered);
    let (is_decorative, decorative_source) = logic::normalize_is_decorative(is_decorative);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != ColorSwatchMotion::default();

    let state = logic::resolve_state(ColorSwatchStateInput {
        size,
        rounding,
        shape,
        bordered: is_bordered,
        alpha,
        has_color: color.is_some(),
        has_custom_aria_label,
        has_custom_class_name,
    });

    let class = logic::compose_class_name(class_name, state);
    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, motion);

    view! {
        <div
            node_ref=root_ref
            class=class
            role=(!is_decorative).then_some("img")
            aria-label=(!is_decorative).then_some(aria_label)
            aria-hidden=is_decorative.then_some("true")
            lang=locale.lang
            dir=locale.dir
            style=logic::compose_inline_style(color.as_deref()).unwrap_or_default()
            data-slot="color-swatch"
            data-size=state.size_attr
            data-rounding=state.rounding_attr
            data-shape=state.shape_attr
            data-alpha=state.alpha_attr
            data-state=state.data_state_attr
            data-has-color=state.has_color.then_some("true")
            data-bordered=state.is_bordered.then_some("true")
            data-bordered-source=bordered_source.as_attr()
            data-decorative=is_decorative.then_some("true")
            data-decorative-source=decorative_source.as_attr()
            data-aria-source=state.aria_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-ui-schema="ui.color-swatch.agent-contract"
            data-ui-schema-version="1"
            data-ui-intent="color-preview"
            data-ui-action="render"
            data-ui-state=state.data_state_attr
            data-ui-source=state.aria_source_attr
            data-ui-stream-support="optional"
            data-ui-stream-fallback="snapshot"
            data-ui-output-status="verified"
        >
            <span class="ui-color-swatch__checker" data-slot="color-swatch-checker" aria-hidden="true"></span>
            <span class="ui-color-swatch__sample" data-slot="color-swatch-sample" aria-hidden="true"></span>
            <span class="ui-color-swatch__slash" data-slot="color-swatch-slash" aria-hidden="true"></span>
        </div>
    }
}
