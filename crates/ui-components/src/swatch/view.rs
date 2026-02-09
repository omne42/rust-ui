use super::{
    SwatchBorder, SwatchMotion, SwatchRounding, SwatchShape, SwatchSize, logic,
    motion as swatch_motion,
};
use crate::overlay_open;
use leptos::{ev, html, prelude::*};

#[component]
pub fn Swatch(
    #[prop(optional, into)] color: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional)] size: SwatchSize,
    #[prop(optional)] border: SwatchBorder,
    #[prop(optional)] rounding: SwatchRounding,
    #[prop(optional)] shape: SwatchShape,
    #[prop(optional)] nothing: bool,
    #[prop(optional)] mixed_value: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] decorative: bool,
    #[prop(optional, into)] selected: Option<Signal<bool>>,
    #[prop(optional)] default_selected: Option<bool>,
    #[prop(optional)] on_selected_change: Option<Callback<bool>>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] motion: SwatchMotion,
) -> impl IntoView {
    let color = crate::color_swatch::sanitize_color_value(color);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(logic::SwatchStateInput {
        size,
        border,
        rounding,
        shape,
        has_color: color.is_some(),
        nothing,
        mixed_value,
        disabled,
        decorative,
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_class_name(class_name, state);

    let (aria_label, aria_label_source) = logic::resolve_aria_label(
        aria_label,
        label,
        color.as_deref(),
        state.show_nothing,
        state.show_mixed_value,
    );

    let selected_state = overlay_open::use_controllable_state(
        selected,
        Some(default_selected.unwrap_or(false)),
        on_selected_change,
    );
    let selected = selected_state.value;
    let request_selected_change = selected_state.request_change;

    let node_ref: NodeRef<html::Div> = NodeRef::new();
    swatch_motion::attach_motion(node_ref, selected, motion);

    let on_activate = move || {
        if !state.is_interactive {
            return;
        }
        request_selected_change.run(!selected.get_untracked());
    };

    let on_keydown = move |ev: ev::KeyboardEvent| {
        let key = ev.key();
        if key == " " || key == "Enter" {
            ev.prevent_default();
            on_activate();
        }
    };

    view! {
        <div
            class=class
            node_ref=node_ref
            role=(!state.decorative).then_some("button")
            tabindex=state.is_interactive.then_some("0")
            aria-label=(!state.decorative).then_some(aria_label)
            aria-disabled=state.disabled.then_some("true")
            aria-pressed=move || {
                (!state.decorative && !state.show_mixed_value)
                    .then_some(if selected.get() { "true" } else { "false" })
            }
            aria-checked=state.show_mixed_value.then_some("mixed")
            aria-hidden=state.decorative.then_some("true")
            style=logic::compose_inline_style(color.as_deref()).unwrap_or_default()
            data-slot="swatch"
            data-size=state.size_attr
            data-border=state.border_attr
            data-rounding=state.rounding_attr
            data-shape=state.shape_attr
            data-state=state.data_state_attr
            data-selected=move || selected.get().then_some("true")
            data-disabled=state.disabled.then_some("true")
            data-nothing=state.show_nothing.then_some("true")
            data-mixed-value=state.show_mixed_value.then_some("true")
            data-has-color=state.has_color.then_some("true")
            data-decorative=state.decorative.then_some("true")
            data-aria-label-source=aria_label_source
            data-custom-class=state.has_custom_class_name.then_some("true")
            on:click=move |_| on_activate()
            on:keydown=on_keydown
        >
            <span class="ui-swatch__checker" data-slot="swatch-checker" aria-hidden="true"></span>
            <span class="ui-swatch__sample" data-slot="swatch-sample" aria-hidden="true"></span>
            <span class="ui-swatch__slash" data-slot="swatch-slash" aria-hidden="true"></span>
            <span class="ui-swatch__mixed-mark" data-slot="swatch-mixed-mark" aria-hidden="true"></span>
            <span class="ui-swatch__disabled-mark" data-slot="swatch-disabled-mark" aria-hidden="true"></span>
        </div>
    }
}
