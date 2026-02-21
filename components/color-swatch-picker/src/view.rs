use crate::{
    color::swatch::{ColorSwatch, ColorSwatchRounding, ColorSwatchShape, ColorSwatchSize},
    color::swatch_picker::{
        ColorSwatchPickerItem, ColorSwatchPickerMotion,
        logic::{self},
        motion,
    },
};
use leptos::prelude::*;
use ui_headless as overlay_open;
use ui_headless::{A11yDirection, RadioGroupOptions, RadioOptions, RovingOrientation, use_radio};

const SLOT_COLOR_SWATCH_PICKER: &str = "color-swatch-picker";
const SLOT_COLOR_SWATCH_PICKER_LIST: &str = "color-swatch-picker-list";
const SLOT_COLOR_SWATCH_PICKER_OPTION: &str = "color-swatch-picker-option";
const CLASS_COLOR_SWATCH_PICKER_OPTION: &str = "ui-color-swatch-picker__option";
const ATTR_BUTTON_TYPE: &str = "button";
const ATTR_ROLE_RADIO: &str = "radio";

fn render_option_swatch(
    color: String,
    color_name: String,
    size: ColorSwatchSize,
    rounding: ColorSwatchRounding,
    shape: ColorSwatchShape,
    is_bordered: bool,
) -> impl IntoView {
    view! {
        <ColorSwatch
            color=color
            color_name=color_name
            size=size
            rounding=rounding
            shape=shape
            is_bordered=is_bordered
            is_decorative=true
        />
    }
}

#[component]
pub fn ColorSwatchPicker(
    swatches: ReadSignal<Vec<ColorSwatchPickerItem>>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] size: ColorSwatchSize,
    #[prop(optional)] rounding: ColorSwatchRounding,
    #[prop(optional)] shape: ColorSwatchShape,
    #[prop(optional, default = true)] is_bordered: bool,
    #[prop(optional, into)] selected_color: Option<Signal<Option<String>>>,
    #[prop(optional, into)] default_selected_color: Option<String>,
    #[prop(optional)] on_selected_change: Option<Callback<Option<String>>>,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: ColorSwatchPickerMotion,
) -> impl IntoView {
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != ColorSwatchPickerMotion::default();
    let style_vars = StoredValue::new(motion::attach_motion(None, motion));
    let agent_contract = logic::resolve_agent_contract();

    let is_controlled = selected_color.is_some();
    let selection_mode_attr = logic::resolve_selection_mode_attr(is_controlled);
    let has_default_selected_color = default_selected_color.is_some();
    let selection_init_source_attr =
        logic::resolve_selection_init_source_attr(is_controlled, has_default_selected_color);
    let (selection_source, set_selection_source) = signal(selection_init_source_attr);
    let (pending_user_selection, set_pending_user_selection) = signal(false);
    let pending_user_selection_for_effect = pending_user_selection;
    let set_pending_user_selection_for_effect = set_pending_user_selection;
    let set_pending_user_selection_for_change = set_pending_user_selection;
    let set_selection_source_for_effect = set_selection_source;

    let id_base = logic::normalize_id_base(id_base);

    let items = Memo::new(move |_| logic::normalize_items(swatches.get()));

    let item_count_initial = items.get_untracked().len();
    let (item_count, set_item_count) = signal(item_count_initial);
    Effect::new(move |_| set_item_count.set(items.get().len()));

    let selected_state = overlay_open::use_controllable_state(
        selected_color,
        Some(logic::sanitize_selected_color(default_selected_color)),
        on_selected_change,
    );

    let (selected_index, set_selected_index) = signal(None::<usize>);
    Effect::new(move |_| {
        let next = logic::resolve_selected_index(&items.get(), selected_state.value.get());
        set_selected_index.set(next);
        let next_source = logic::resolve_selection_source_attr(
            selection_source.get_untracked(),
            selection_mode_attr,
            selection_init_source_attr,
            pending_user_selection_for_effect.get_untracked(),
        );
        set_selection_source_for_effect.set(next_source);
        set_pending_user_selection_for_effect.set(false);
    });

    let is_item_disabled = Callback::new(move |index: usize| {
        logic::is_item_disabled_at(is_disabled, &items.get_untracked(), index)
    });

    let selected_state_request_change = selected_state.request_change;

    let aria = use_radio(RadioOptions {
        group: RadioGroupOptions {
            is_disabled,
            id_base: id_base.clone(),
            orientation: RovingOrientation::Horizontal,
            item_count,
            selected_index,
            set_selected_index,
            on_change: Some(Callback::new(move |index: usize| {
                let next = logic::resolve_selected_color(&items.get_untracked(), Some(index));
                set_pending_user_selection_for_change.set(true);
                selected_state_request_change.run(next);
            })),
            is_item_disabled: Some(is_item_disabled),
        },
        lang,
        dir,
    });

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        let items = items.get();
        logic::resolve_component_state(
            is_disabled,
            &items,
            aria.state.selected_index.get(),
            has_custom_aria_label,
            has_custom_class_name,
        )
    });

    let class = Memo::new(move |_| logic::compose_class_name(class_name.get_value(), state.get()));

    let on_key_down = {
        let on_key_down = aria.handlers.on_key_down;
        move |ev: leptos::ev::KeyboardEvent| {
            if on_key_down.run(ev.key()) {
                ev.prevent_default();
            }
        }
    };

    let render_options = move || {
        items
            .get()
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                let is_selected = move || aria.state.selected_index.get() == Some(index);
                let option_disabled = logic::resolve_option_disabled(is_disabled, item.disabled);
                let option_label = logic::resolve_option_label(&item, index);
                let option_label_for_color = option_label.clone();
                let option_label_for_button = option_label;

                view! {
                    <button
                        type=ATTR_BUTTON_TYPE
                        class=CLASS_COLOR_SWATCH_PICKER_OPTION
                        id=aria.state.radio_id.run(index)
                        role=ATTR_ROLE_RADIO
                        tabindex=move || {
                            logic::resolve_option_tabindex(
                                option_disabled,
                                aria.state.active_index.get(),
                                index,
                            )
                        }
                        aria-label=option_label_for_button.clone()
                        aria-checked=move || if is_selected() { "true" } else { "false" }
                        aria-disabled=option_disabled.then_some("true")
                        disabled=option_disabled
                        data-slot=SLOT_COLOR_SWATCH_PICKER_OPTION
                        data-index=index
                        data-color=item.color.clone()
                        data-selected=move || is_selected().then_some("true")
                        data-disabled=option_disabled.then_some("true")
                        on:focus=move |_| aria.handlers.on_radio_focus.run(index)
                        on:click=move |_| aria.handlers.on_radio_click.run(index)
                    >
                        {render_option_swatch(
                            item.color.clone(),
                            option_label_for_color,
                            size,
                            rounding,
                            shape,
                            is_bordered,
                        )}
                    </button>
                }
            })
            .collect_view()
    };

    view! {
        <div
            id=id_base
            class=move || class.get()
            style=move || style_vars.get_value()
            role=aria.attrs.role
            aria-disabled=aria.attrs.aria_disabled
            aria-label=aria_label
            lang=aria.attrs.lang
            dir=aria.attrs.dir
            data-slot=SLOT_COLOR_SWATCH_PICKER
            data-state=move || state.get().data_state_attr
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-empty=move || state.get().is_empty.then_some("true")
            data-has-items=move || state.get().has_items.then_some("true")
            data-count=move || state.get().item_count.to_string()
            data-has-selection=move || state.get().has_selection.then_some("true")
            data-selection-empty=move || state.get().selection_empty.then_some("true")
            data-selected-index=move || state.get().selected_index.map(|index| index.to_string())
            data-has-disabled-items=move || state.get().has_disabled_items.then_some("true")
            data-disabled-item-count=move || state.get().disabled_item_count.to_string()
            data-aria-source=move || state.get().aria_source_attr
            data-selection-mode=selection_mode_attr
            data-selection-init-source=selection_init_source_attr
            data-selection-source=move || selection_source.get()
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            data-ui-schema=agent_contract.schema_attr
            data-ui-schema-version=agent_contract.schema_version_attr
            data-ui-stream-support=agent_contract.stream_support_attr
            data-ui-stream-fallback=agent_contract.stream_fallback_attr
            data-ui-output-status=agent_contract.output_status_attr
            data-ui-intent=agent_contract.intent_attr
            data-ui-action=move || logic::resolve_ui_action(selection_source.get()).as_attr()
            data-ui-state=move || logic::resolve_ui_state(state.get().is_disabled, state.get().is_empty).as_attr()
            data-ui-source=move || logic::resolve_ui_source(selection_source.get()).as_attr()
            on:keydown=on_key_down
        >
            <div class="ui-color-swatch-picker__list" data-slot=SLOT_COLOR_SWATCH_PICKER_LIST>
                {render_options}
            </div>
        </div>
    }
}
