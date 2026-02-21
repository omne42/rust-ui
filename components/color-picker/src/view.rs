use crate::OnPress;
use crate::color::picker::{
    ColorPickerMotion,
    logic::{self},
};
use crate::color::swatch::{ColorSwatch, ColorSwatchRounding, ColorSwatchShape, ColorSwatchSize};
use crate::popover::Popover;
use leptos::{html, prelude::*};
use ui_headless as overlay_open;
use ui_headless::use_presence;
use ui_headless::{
    A11yDirection, ButtonOptions, FocusRingOptions, HoverOptions, PopoverPlacement, locale_attrs,
    overlay_dialog_attrs, popup_trigger_attrs, use_button, use_focus_ring, use_hover,
};

const DEFAULT_ID_BASE: &str = "ui-color-picker";
const CLASS_TRIGGER: &str = "ui-color-picker__trigger";
const CLASS_SWATCH: &str = "ui-color-picker__swatch";
const CLASS_LABEL: &str = "ui-color-picker__label";
const CLASS_VALUE: &str = "ui-color-picker__value";
const CLASS_PANEL: &str = "ui-color-picker__panel";
const CLASS_CONTENT: &str = "ui-color-picker__content";
const SLOT_ROOT: &str = "color-picker";
const SLOT_TRIGGER: &str = "color-picker-trigger";
const SLOT_SWATCH: &str = "color-picker-swatch";
const SLOT_LABEL: &str = "color-picker-label";
const SLOT_VALUE: &str = "color-picker-value";
const SLOT_PANEL: &str = "color-picker-panel";
const SLOT_CONTENT: &str = "color-picker-content";
const ARIA_HIDDEN_TRUE: &str = "true";

fn render_empty_swatch(
    swatch_size: ColorSwatchSize,
    swatch_rounding: ColorSwatchRounding,
    swatch_shape: ColorSwatchShape,
    swatch_bordered: bool,
) -> impl IntoView {
    view! {
        <ColorSwatch
            is_decorative=true
            size=swatch_size
            rounding=swatch_rounding
            shape=swatch_shape
            is_bordered=swatch_bordered
        />
    }
}

fn render_selected_swatch(
    color: String,
    swatch_size: ColorSwatchSize,
    swatch_rounding: ColorSwatchRounding,
    swatch_shape: ColorSwatchShape,
    swatch_bordered: bool,
) -> impl IntoView {
    view! {
        <ColorSwatch
            color=color
            is_decorative=true
            size=swatch_size
            rounding=swatch_rounding
            shape=swatch_shape
            is_bordered=swatch_bordered
        />
    }
}

fn render_selected_value_text(selected_color: String) -> impl IntoView {
    view! {
        <span class=CLASS_VALUE data-slot=SLOT_VALUE>
            {selected_color}
        </span>
    }
}

#[component]
pub fn ColorPicker(
    id_base: String,
    children: ChildrenFn,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] value: Option<Signal<Option<String>>>,
    #[prop(optional, into)] default_value: Option<String>,
    #[prop(optional)] on_value_change: Option<Callback<Option<String>>>,
    #[prop(optional)] selected_color: Option<Signal<Option<String>>>,
    #[prop(optional, into)] default_selected_color: Option<String>,
    #[prop(optional)] on_selected_change: Option<Callback<Option<String>>>,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] motion: ColorPickerMotion,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] swatch_size: ColorSwatchSize,
    #[prop(optional)] swatch_rounding: ColorSwatchRounding,
    #[prop(optional)] swatch_shape: ColorSwatchShape,
    #[prop(optional, default = true)] swatch_bordered: bool,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_optional_text(Some(id_base))
        .unwrap_or_else(|| DEFAULT_ID_BASE.to_string());

    let ids = logic::resolve_ids(&id_base);
    let trigger_id = StoredValue::new(ids.trigger_id);
    let label_id = StoredValue::new(ids.label_id);
    let panel_id = StoredValue::new(ids.panel_id);
    let content_id = StoredValue::new(ids.content_id);

    let selected_color = logic::resolve_selected_color_axis(value, selected_color);
    let is_selection_controlled = selected_color.is_some();
    let default_selected_color =
        logic::resolve_default_selected_color(default_value, default_selected_color);
    let on_selected_change =
        logic::resolve_selected_change_axis(on_value_change, on_selected_change);
    let selected_state = overlay_open::use_controllable_state(
        selected_color,
        Some(default_selected_color),
        on_selected_change,
    );
    let selected_color =
        Memo::new(move |_| logic::sanitize_selected_color(selected_state.value.get()));

    let is_open_controlled = open.is_some();
    let open_state = overlay_open::use_controllable_open_state_traced(
        "color-picker",
        open,
        default_open,
        on_open_change,
    );
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let presence = use_presence(open);

    let (label, has_custom_label) = logic::normalize_label(label);
    let label = StoredValue::new(label);

    let (trigger_aria_label, has_custom_aria_label) =
        logic::normalize_aria_label(aria_label, &label.get_value());
    let trigger_aria_label = StoredValue::new(trigger_aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);

    let motion = crate::color_picker::motion::sanitize_motion(motion);
    let is_custom_motion = motion != ColorPickerMotion::default();
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        logic::resolve_derived_state(logic::ColorPickerDerivedStateInput {
            is_disabled,
            is_open: open.get(),
            selected_color: selected_color.get(),
            has_custom_label,
            has_custom_aria_label,
            has_custom_class_name,
            is_open_controlled,
        })
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));
    let agent_action = RwSignal::new(logic::ColorPickerAgentAction::SnapshotRender);
    let agent_contract = Memo::new(move |_| {
        logic::resolve_agent_contract(logic::ColorPickerAgentContractInput {
            render_state: state.get(),
            action: agent_action.get(),
            is_selection_controlled,
            is_custom_motion,
        })
    });

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();

    let on_trigger_press: OnPress = Callback::new(move |_| {
        if is_disabled {
            return;
        }
        let next_open = !open.get_untracked();
        agent_action.set(if next_open {
            logic::ColorPickerAgentAction::ToggleOpen
        } else {
            logic::ColorPickerAgentAction::ToggleClose
        });
        request_open_change.run(next_open);
    });

    let on_close: OnPress = Callback::new(move |_| {
        agent_action.set(logic::ColorPickerAgentAction::ToggleClose);
        request_open_change.run(false);
    });

    let aria_controls = ui_headless::aria_controls_when_open(open, panel_id.get_value());
    let locale = locale_attrs(lang, dir);
    let trigger_popup_a11y = popup_trigger_attrs(
        Some("dialog"),
        None,
        Some(aria_controls),
        Some(open),
        locale.lang.clone(),
        dir,
    );
    let trigger_aria_controls = trigger_popup_a11y.aria_controls;
    let trigger_aria_expanded = trigger_popup_a11y.aria_expanded;

    let trigger_aria = use_button(ButtonOptions {
        is_disabled,
        on_press: Some(on_trigger_press),
        ..Default::default()
    });
    let trigger_focus_ring = use_focus_ring(FocusRingOptions { is_disabled });
    let trigger_hover = use_hover(HoverOptions { is_disabled });

    let panel_a11y =
        overlay_dialog_attrs(Some(label_id.get_value()), None, locale.lang.clone(), dir);
    let panel_aria_labelledby = StoredValue::new(panel_a11y.aria_labelledby);
    let panel_lang = StoredValue::new(panel_a11y.lang);
    let panel_dir = panel_a11y.dir;

    let children = StoredValue::new(children);
    let render_trigger = move || {
        view! {
            <button
                id=trigger_id.get_value()
                node_ref=anchor_ref
                class=CLASS_TRIGGER
                type="button"
                disabled=is_disabled
                aria-haspopup="dialog"
                aria-expanded=move || trigger_aria_expanded.get().unwrap_or("false")
                aria-controls=move || trigger_aria_controls.get()
                aria-label=move || trigger_aria_label.get_value()
                role=trigger_aria.attrs.role
                tabindex=trigger_aria.attrs.tabindex
                aria-disabled=trigger_aria.attrs.aria_disabled
                data-hovered=move || trigger_hover.is_hovered.get().then_some("true")
                data-pressed=move || trigger_aria.is_pressed.get().then_some("true")
                data-focus-visible=move || trigger_focus_ring.is_focus_visible.get().then_some("true")
                on:pointerdown=move |_| trigger_aria.handlers.press.on_pointer_down.run(())
                on:pointerup=move |_| trigger_aria.handlers.press.on_pointer_up.run(())
                on:pointercancel=move |_| trigger_aria.handlers.press.on_pointer_cancel.run(())
                on:pointerenter=move |_| trigger_hover.handlers.on_pointer_enter.run(())
                on:pointerleave=move |_| trigger_hover.handlers.on_pointer_leave.run(())
                on:click=move |_| trigger_aria.handlers.press.on_click.run(())
                on:keydown=move |ev| {
                    let key = ev.key();
                    if trigger_aria.handlers.press.on_key_down.run(key) {
                        ev.prevent_default();
                    }
                }
                on:keyup=move |ev| {
                    let key = ev.key();
                    if trigger_aria.handlers.press.on_key_up.run(key) {
                        ev.prevent_default();
                    }
                }
                on:focus=move |_| trigger_focus_ring.handlers.on_focus.run(())
                on:blur=move |_| {
                    trigger_aria.handlers.press.on_blur.run(());
                    trigger_focus_ring.handlers.on_blur.run(());
                }
                data-slot=SLOT_TRIGGER
            >
                <span class=CLASS_SWATCH data-slot=SLOT_SWATCH aria-hidden=ARIA_HIDDEN_TRUE>
                    <Show
                        when=move || selected_color.get().is_some()
                        fallback=move || {
                            render_empty_swatch(
                                swatch_size,
                                swatch_rounding,
                                swatch_shape,
                                swatch_bordered,
                            )
                        }
                    >
                        {move || {
                            render_selected_swatch(
                                selected_color.get().unwrap_or_default(),
                                swatch_size,
                                swatch_rounding,
                                swatch_shape,
                                swatch_bordered,
                            )
                        }}
                    </Show>
                </span>

                <span id=label_id.get_value() class=CLASS_LABEL data-slot=SLOT_LABEL>
                    {label.get_value()}
                </span>

                <Show when=move || selected_color.get().is_some()>
                    {move || render_selected_value_text(selected_color.get().unwrap_or_default())}
                </Show>
            </button>
        }
    };

    let render_panel = move || {
        view! {
            <Show when=move || presence.is_present.get()>
                <Popover
                    open=open
                    anchor_ref=anchor_ref
                    on_close=on_close
                    placement=placement
                    motion=motion.popover
                    on_exit_complete=presence.finish_exit
                >
                    {move || {
                        let children = children.get_value();
                        view! {
                            <div
                                id=panel_id.get_value()
                                class=CLASS_PANEL
                                role="dialog"
                                aria-modal="false"
                                aria-label=move || trigger_aria_label.get_value()
                                aria-labelledby=panel_aria_labelledby.get_value()
                                lang=panel_lang.get_value()
                                dir=panel_dir
                                data-slot=SLOT_PANEL
                            >
                                <div
                                    id=content_id.get_value()
                                    class=CLASS_CONTENT
                                    data-slot=SLOT_CONTENT
                                >
                                    {children()}
                                </div>
                            </div>
                        }
                    }}
                </Popover>
            </Show>
        }
    };

    view! {
        <div
            id=id_base
            class=move || class.get()
            data-slot=SLOT_ROOT
            data-slot-projection="lazy"
            data-slot-projection-source="presence"
            data-state=move || state.get().data_state_attr
            data-open=move || state.get().is_open.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-selection-empty=move || state.get().selection_empty.then_some("true")
            data-open-mode=move || state.get().open_mode_attr
            data-label-source=move || state.get().label_source_attr
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=if motion == ColorPickerMotion::default() {
                "default"
            } else {
                "custom"
            }
            data-custom-motion=move || (motion != ColorPickerMotion::default()).then_some("true")
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
            data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()
            data-ui-selection-source=move || agent_contract.get().selection_source
            data-ui-open-source=move || agent_contract.get().open_source
            data-ui-motion-source=move || agent_contract.get().motion_source
            data-ui-label-source=move || agent_contract.get().label_source
            data-ui-aria-source=move || agent_contract.get().aria_source
            data-ui-class-source=move || agent_contract.get().class_source
            data-ui-config-policy=move || agent_contract.get().config_policy
            lang=locale.lang.clone()
            dir=locale.dir
        >
            {render_trigger()}
            {render_panel()}
        </div>
    }
}
