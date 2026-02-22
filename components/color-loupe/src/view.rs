use crate::color::loupe::logic::{self, ColorLoupeLogicInput, ColorLoupeOutputState};
use crate::color::swatch::ColorSwatch;
use leptos::prelude::*;
use ui_headless::a11y::{A11yDirection, locale_attrs};

const COLOR_LOUPE_SWATCH_CLASS: &str = "ui-color-loupe__swatch";
const COLOR_LOUPE_BUBBLE_CLASS: &str = "ui-color-loupe__bubble";
const COLOR_LOUPE_BUBBLE_SLOT: &str = "color-loupe-bubble";
const COLOR_LOUPE_CHECKER_CLASS: &str = "ui-color-loupe__checker";
const COLOR_LOUPE_CHECKER_SLOT: &str = "color-loupe-checker";
const COLOR_LOUPE_FILL_CLASS: &str = "ui-color-loupe__fill";
const COLOR_LOUPE_FILL_SLOT: &str = "color-loupe-fill";
const COLOR_LOUPE_TAIL_CLASS: &str = "ui-color-loupe__tail";
const COLOR_LOUPE_TAIL_SLOT: &str = "color-loupe-tail";
const ARIA_HIDDEN_TRUE: &str = "true";

fn render_loupe_fill(color: Option<String>) -> impl IntoView {
    let swatch_class = COLOR_LOUPE_SWATCH_CLASS.to_string();

    match color {
        Some(color) => view! {
            <ColorSwatch
                color=color
                is_decorative=true
                class_name=swatch_class.clone()
            />
        }
        .into_any(),
        None => view! { <ColorSwatch is_decorative=true class_name=swatch_class /> }.into_any(),
    }
}

fn render_loupe_bubble(color: Option<String>) -> impl IntoView {
    view! {
        <span
            class=COLOR_LOUPE_BUBBLE_CLASS
            data-slot=COLOR_LOUPE_BUBBLE_SLOT
            aria-hidden=ARIA_HIDDEN_TRUE
        >
            <span class=COLOR_LOUPE_CHECKER_CLASS data-slot=COLOR_LOUPE_CHECKER_SLOT></span>
            <span class=COLOR_LOUPE_FILL_CLASS data-slot=COLOR_LOUPE_FILL_SLOT>
                {render_loupe_fill(color)}
            </span>
        </span>
    }
}

fn render_loupe_tail() -> impl IntoView {
    view! {
        <span
            class=COLOR_LOUPE_TAIL_CLASS
            data-slot=COLOR_LOUPE_TAIL_SLOT
            aria-hidden=ARIA_HIDDEN_TRUE
        ></span>
    }
}

#[component]
pub fn ColorLoupe(
    id_base: String,
    #[prop(optional, into)] color: Option<String>,
    #[prop(optional)] is_open: bool,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] x_percent: Option<f32>,
    #[prop(optional)] y_percent: Option<f32>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] output_state: Option<ColorLoupeOutputState>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let locale = locale_attrs(logic::normalize_optional_text(lang), dir);
    let color = logic::sanitize_color(color);
    let has_color = color.is_some();
    let color = StoredValue::new(color);

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let aria_label = StoredValue::new(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);
    let output_state = logic::normalize_output_state(output_state);

    let state = Memo::new(move |_| {
        logic::resolve_component_state(ColorLoupeLogicInput {
            is_open,
            is_disabled,
            has_color,
            x_percent,
            y_percent,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let ui_schema =
        Memo::new(move |_| logic::agent_contract_schema_attr(state.get(), output_state));

    view! {
        <div
            id=id_base
            class=move || class.get()
            role="img"
            aria-label=move || aria_label.get_value()
            lang=locale.lang
            dir=locale.dir
            data-slot="color-loupe"
            data-state=move || state.get().data_state_attr
            data-ui-schema=move || ui_schema.get()
            data-output-state=output_state.as_attr()
            data-open=move || state.get().is_open.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-color=move || state.get().has_color.then_some("true")
            data-x=move || state.get().x_percent
            data-y=move || state.get().y_percent
            data-x-bucket=move || state.get().x_bucket_attr
            data-y-bucket=move || state.get().y_bucket_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
        >
            {move || render_loupe_bubble(color.get_value())}
            {render_loupe_tail()}
        </div>
    }
}
