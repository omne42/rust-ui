use crate::{
    color::swatch::{ColorSwatch, ColorSwatchRounding, ColorSwatchShape, ColorSwatchSize},
    color::swatch_picker::{
        ColorSwatchPickerItem, ColorSwatchPickerMotion, ColorSwatchPickerStateInput,
        logic::{self},
        motion,
    },
};
use leptos::prelude::*;
use ui_headless as overlay_open;
use ui_headless::{RadioGroupOptions, RovingOrientation, use_radio_group};

#[component]
pub fn ColorSwatchPicker(
    swatches: ReadSignal<Vec<ColorSwatchPickerItem>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] size: ColorSwatchSize,
    #[prop(optional)] rounding: ColorSwatchRounding,
    #[prop(optional)] shape: ColorSwatchShape,
    #[prop(optional, default = true)] bordered: bool,
    #[prop(optional, into)] selected_color: Option<Signal<Option<String>>>,
    #[prop(optional, into)] default_selected_color: Option<String>,
    #[prop(optional)] on_selected_change: Option<Callback<Option<String>>>,
    #[prop(optional, into)] id_base: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] motion: ColorSwatchPickerMotion,
) -> impl IntoView {
    let motion = motion::sanitize_motion(motion);
    let has_custom_motion = motion != ColorSwatchPickerMotion::default();

    let id_base = logic::normalize_optional_text(id_base)
        .unwrap_or_else(|| "ui-color-swatch-picker".to_string());

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
    });

    let is_item_disabled = Callback::new(move |index: usize| {
        disabled
            || items
                .get_untracked()
                .get(index)
                .is_none_or(|item| item.disabled)
    });

    let selected_state_request_change = selected_state.request_change;

    let aria = use_radio_group(RadioGroupOptions {
        is_disabled: disabled,
        id_base: id_base.clone(),
        orientation: RovingOrientation::Horizontal,
        item_count,
        selected_index,
        set_selected_index,
        on_change: Some(Callback::new(move |index: usize| {
            let next = logic::resolve_selected_color(&items.get_untracked(), Some(index));
            selected_state_request_change.run(next);
        })),
        is_item_disabled: Some(is_item_disabled),
    });

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Memo::new(move |_| {
        let items = items.get();
        let disabled_item_count = items.iter().filter(|item| item.disabled).count();

        logic::resolve_state(ColorSwatchPickerStateInput {
            disabled,
            item_count: items.len(),
            selected_index: aria.selected_index.get(),
            disabled_item_count,
            has_custom_aria_label,
            has_custom_class_name,
        })
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

    view! {
        <div
            id=id_base
            class=move || class.get()
            role=aria.attrs.role
            aria-disabled=aria.attrs.aria_disabled
            aria-label=aria_label
            data-slot="color-swatch-picker"
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
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=if has_custom_motion { "custom" } else { "default" }
            data-custom-motion=has_custom_motion.then_some("true")
            on:keydown=on_key_down
        >
            <div class="ui-color-swatch-picker__list" data-slot="color-swatch-picker-list">
                {move || {
                    items
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(index, item)| {
                            let is_selected = move || aria.selected_index.get() == Some(index);
                            let option_disabled = disabled || item.disabled;
                            let option_label = logic::resolve_option_label(&item, index);
                            let option_label_for_color = option_label.clone();
                            let option_label_for_button = option_label.clone();

                            view! {
                                <button
                                    type="button"
                                    class="ui-color-swatch-picker__option"
                                    id=aria.radio_id.run(index)
                                    role="radio"
                                    tabindex=move || {
                                        if option_disabled {
                                            -1
                                        } else if aria.active_index.get() == index {
                                            0
                                        } else {
                                            -1
                                        }
                                    }
                                    aria-label=option_label_for_button.clone()
                                    aria-checked=move || if is_selected() { "true" } else { "false" }
                                    aria-disabled=option_disabled.then_some("true")
                                    disabled=option_disabled
                                    data-slot="color-swatch-picker-option"
                                    data-index=index
                                    data-color=item.color.clone()
                                    data-selected=move || is_selected().then_some("true")
                                    data-disabled=option_disabled.then_some("true")
                                    on:focus=move |_| aria.handlers.on_radio_focus.run(index)
                                    on:click=move |_| aria.handlers.on_radio_click.run(index)
                                >
                                    <ColorSwatch
                                        color=item.color.clone()
                                        color_name=option_label_for_color
                                        size=size
                                        rounding=rounding
                                        shape=shape
                                        is_bordered=bordered
                                        is_decorative=true
                                    />
                                </button>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </div>
    }
}
