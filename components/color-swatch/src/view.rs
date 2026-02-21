use crate::color::swatch::{
    ColorSwatchMotion, ColorSwatchRounding, ColorSwatchShape, ColorSwatchSize,
    logic::{self},
    motion,
};
use leptos::{html, prelude::*};
use ui_headless::{A11yDirection, ColorSwatchA11yOptions, use_color_swatch_a11y};

const SLOT_COLOR_SWATCH: &str = "color-swatch";
const SLOT_COLOR_SWATCH_CHECKER: &str = "color-swatch-checker";
const SLOT_COLOR_SWATCH_SAMPLE: &str = "color-swatch-sample";
const SLOT_COLOR_SWATCH_SLASH: &str = "color-swatch-slash";

const CLASS_COLOR_SWATCH_CHECKER: &str = "ui-color-swatch__checker";
const CLASS_COLOR_SWATCH_SAMPLE: &str = "ui-color-swatch__sample";
const CLASS_COLOR_SWATCH_SLASH: &str = "ui-color-swatch__slash";

const BOOL_TRUE: &str = "true";

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
    let render_state = logic::resolve_render_state(logic::ColorSwatchRenderInput {
        color,
        color_name,
        size,
        rounding,
        shape,
        is_bordered,
        is_decorative,
        aria_label,
        class_name,
    });
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != ColorSwatchMotion::default();
    let agent_contract = logic::resolve_agent_contract();
    let a11y = use_color_swatch_a11y(ColorSwatchA11yOptions {
        is_decorative: render_state.is_decorative,
        aria_label: render_state.aria_label,
        lang,
        dir,
    });

    let state = render_state.state;
    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, motion);

    view! {
        <div
            node_ref=root_ref
            class=render_state.class_name
            role=a11y.attrs.role
            aria-label=a11y.attrs.aria_label.clone()
            aria-hidden=a11y.attrs.aria_hidden
            lang=a11y.attrs.lang.clone()
            dir=a11y.attrs.dir
            style=render_state.inline_style
            data-slot=SLOT_COLOR_SWATCH
            data-size=state.size_attr
            data-rounding=state.rounding_attr
            data-shape=state.shape_attr
            data-alpha=state.alpha_attr
            data-state=state.data_state_attr
            data-has-color=state.has_color.then_some("true")
            data-bordered=state.is_bordered.then_some("true")
            data-bordered-source=render_state.bordered_source.as_attr()
            data-decorative=render_state.is_decorative.then_some("true")
            data-decorative-source=render_state.decorative_source.as_attr()
            data-aria-source=state.aria_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-ui-schema=agent_contract.schema_attr
            data-ui-schema-version=agent_contract.schema_version_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action=agent_contract.action_attr
            data-ui-state=state.data_state_attr
            data-ui-source=state.aria_source_attr
            data-ui-stream-support=agent_contract.stream_support_attr
            data-ui-stream-fallback=agent_contract.stream_fallback_attr
            data-ui-output-status=agent_contract.output_status_attr
        >
            <span class=CLASS_COLOR_SWATCH_CHECKER data-slot=SLOT_COLOR_SWATCH_CHECKER aria-hidden=BOOL_TRUE></span>
            <span class=CLASS_COLOR_SWATCH_SAMPLE data-slot=SLOT_COLOR_SWATCH_SAMPLE aria-hidden=BOOL_TRUE></span>
            <span class=CLASS_COLOR_SWATCH_SLASH data-slot=SLOT_COLOR_SWATCH_SLASH aria-hidden=BOOL_TRUE></span>
        </div>
    }
}
