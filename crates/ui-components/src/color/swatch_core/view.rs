use super::{
    SwatchBorder, SwatchMotion, SwatchRounding, SwatchShape, SwatchSize, logic,
    motion as swatch_motion,
};
use leptos::{html, prelude::*};
use ui_headless as overlay_open;
use ui_headless::{A11yDirection, CommonStrings, SwatchOptions, use_swatch, use_ui_i18n};

const SLOT_SWATCH: &str = "swatch";
const SLOT_SWATCH_CHECKER: &str = "swatch-checker";
const SLOT_SWATCH_SAMPLE: &str = "swatch-sample";
const SLOT_SWATCH_SLASH: &str = "swatch-slash";
const SLOT_SWATCH_MIXED_MARK: &str = "swatch-mixed-mark";
const SLOT_SWATCH_DISABLED_MARK: &str = "swatch-disabled-mark";

const CLASS_SWATCH_CHECKER: &str = "ui-swatch__checker";
const CLASS_SWATCH_SAMPLE: &str = "ui-swatch__sample";
const CLASS_SWATCH_SLASH: &str = "ui-swatch__slash";
const CLASS_SWATCH_MIXED_MARK: &str = "ui-swatch__mixed-mark";
const CLASS_SWATCH_DISABLED_MARK: &str = "ui-swatch__disabled-mark";

const BOOL_TRUE: &str = "true";

#[component]
pub fn Swatch(
    #[prop(optional, into)] color: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional)] size: SwatchSize,
    #[prop(optional)] border: SwatchBorder,
    #[prop(optional)] rounding: SwatchRounding,
    #[prop(optional)] shape: SwatchShape,
    #[prop(optional)] is_nothing: bool,
    #[prop(optional)] is_mixed_value: bool,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_decorative: bool,
    #[prop(optional, into)] selected: Option<Signal<bool>>,
    #[prop(optional)] default_selected: Option<bool>,
    #[prop(optional)] on_selected_change: Option<Callback<bool>>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] motion: SwatchMotion,
) -> impl IntoView {
    let color = logic::sanitize_color_value(color);
    let class_name = logic::normalize_optional_text(class_name);

    let state = logic::resolve_state(logic::SwatchStateInput {
        size,
        border,
        rounding,
        shape,
        has_color: color.is_some(),
        nothing: is_nothing,
        mixed_value: is_mixed_value,
        disabled: is_disabled,
        decorative: is_decorative,
        has_custom_class_name: class_name.is_some(),
    });
    let class = logic::compose_class_name(class_name, state);

    let i18n = use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let mixed_label_fallback = common.swatch_mixed_aria_label.as_ref();
    let nothing_label_fallback = common.swatch_nothing_aria_label.as_ref();
    let default_label_fallback = common.swatch_default_aria_label.as_ref();

    let (aria_label, aria_label_source) = logic::resolve_aria_label_with_fallbacks(
        aria_label,
        label,
        color.as_deref(),
        state.show_nothing,
        state.show_mixed_value,
        logic::SwatchAriaLabelFallbacks {
            mixed: mixed_label_fallback,
            nothing: nothing_label_fallback,
            default: default_label_fallback,
        },
    );

    let selection_control =
        logic::resolve_selection_control_state(logic::SwatchSelectionControlInput {
            has_controlled_selected: selected.is_some(),
            default_selected,
            has_on_selected_change: on_selected_change.is_some(),
        });
    let agent_source = RwSignal::new(logic::resolve_agent_source(selection_control));

    let selected_state = overlay_open::use_controllable_state(
        selected,
        Some(selection_control.default_selected),
        on_selected_change,
    );
    let selected = selected_state.value;
    let request_selected_change = selected_state.request_change;

    let agent_contract = Signal::derive(move || {
        logic::resolve_agent_contract(state, selected.get(), agent_source.get())
    });

    let swatch_aria = use_swatch(SwatchOptions {
        is_disabled: state.disabled,
        is_decorative: state.decorative,
        is_mixed_value: state.show_mixed_value,
        is_selected: selected,
        aria_label: Some(aria_label),
        on_press: Some(Callback::new(move |_| {
            agent_source.set(logic::SwatchAgentSource::TogglePress);
            request_selected_change.run(!selected.get_untracked());
        })),
        lang,
        dir,
    });

    let node_ref: NodeRef<html::Div> = NodeRef::new();
    swatch_motion::attach_motion(node_ref, selected, motion);

    view! {
        <div
            class=class
            node_ref=node_ref
            role=swatch_aria.attrs.role
            tabindex=swatch_aria.attrs.tabindex
            aria-label=swatch_aria.attrs.aria_label.clone()
            aria-disabled=swatch_aria.attrs.aria_disabled
            aria-pressed=move || swatch_aria.attrs.aria_pressed.get()
            aria-checked=swatch_aria.attrs.aria_checked
            aria-hidden=swatch_aria.attrs.aria_hidden
            lang=swatch_aria.attrs.lang.clone()
            dir=swatch_aria.attrs.dir
            style=logic::compose_inline_style(color.as_deref()).unwrap_or_default()
            data-slot=SLOT_SWATCH
            data-size=state.size_attr
            data-border=state.border_attr
            data-rounding=state.rounding_attr
            data-shape=state.shape_attr
            data-state=state.data_state_attr
            data-selected=move || selected.get().then_some("true")
            data-pressed=move || swatch_aria.state.is_pressed.get().then_some("true")
            data-disabled=state.disabled.then_some("true")
            data-nothing=state.show_nothing.then_some("true")
            data-mixed-value=state.show_mixed_value.then_some("true")
            data-has-color=state.has_color.then_some("true")
            data-decorative=state.decorative.then_some("true")
            data-aria-label-source=aria_label_source
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-controlled=selection_control.is_controlled_selected.then_some("true")
            data-uncontrolled=selection_control.is_uncontrolled_selected.then_some("true")
            data-control-mode=selection_control.control_mode_attr
            data-default-selected-source=selection_control.default_selected_source_attr
            data-selected-change-source=selection_control.selected_change_source_attr
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
            data-ui-capability-toggle=move || {
                agent_contract.get().capabilities.can_toggle.then_some("true")
            }
            data-ui-capability-disable=move || {
                agent_contract.get().capabilities.can_disable.then_some("true")
            }
            on:pointerdown=move |_| swatch_aria.handlers.button.press.on_pointer_down.run(())
            on:pointerup=move |_| swatch_aria.handlers.button.press.on_pointer_up.run(())
            on:pointercancel=move |_| swatch_aria.handlers.button.press.on_pointer_cancel.run(())
            on:click=move |_| swatch_aria.handlers.button.press.on_click.run(())
            on:keydown=move |ev| {
                let key = ev.key();
                if swatch_aria.handlers.button.press.on_key_down.run(key) {
                    ev.prevent_default();
                }
            }
            on:keyup=move |ev| {
                let key = ev.key();
                if swatch_aria.handlers.button.press.on_key_up.run(key) {
                    ev.prevent_default();
                }
            }
            on:blur=move |_| swatch_aria.handlers.button.press.on_blur.run(())
        >
            <span class=CLASS_SWATCH_CHECKER data-slot=SLOT_SWATCH_CHECKER aria-hidden=BOOL_TRUE></span>
            <span class=CLASS_SWATCH_SAMPLE data-slot=SLOT_SWATCH_SAMPLE aria-hidden=BOOL_TRUE></span>
            <span class=CLASS_SWATCH_SLASH data-slot=SLOT_SWATCH_SLASH aria-hidden=BOOL_TRUE></span>
            <span class=CLASS_SWATCH_MIXED_MARK data-slot=SLOT_SWATCH_MIXED_MARK aria-hidden=BOOL_TRUE></span>
            <span
                class=CLASS_SWATCH_DISABLED_MARK
                data-slot=SLOT_SWATCH_DISABLED_MARK
                aria-hidden=BOOL_TRUE
            ></span>
        </div>
    }
}
