use crate::number::{NumberFormatOptions, SlidingNumberMotion, format_static_number, logic};
use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::number::motion;

#[cfg(target_arch = "wasm32")]
use leptos::html;

#[cfg(target_arch = "wasm32")]
#[component]
fn SlidingNumberRoller(digit: Signal<u8>, motion: SlidingNumberMotion) -> impl IntoView {
    let roller_ref: NodeRef<html::Span> = NodeRef::new();
    motion::attach_motion(roller_ref, digit, motion);

    view! {
        <span
            class="ui-sliding-number__roller"
            data-slot="sliding-number-roller"
            node_ref=roller_ref
        >
            <span class="ui-sliding-number__stack" data-slot="sliding-number-stack">
                {(0..30)
                    .map(|idx| {
                        let digit = (idx % 10).to_string();
                        view! {
                            <span class="ui-sliding-number__digit" data-slot="sliding-number-digit">
                                {digit}
                            </span>
                        }
                    })
                    .collect_view()}
            </span>
        </span>
    }
}

#[component]
pub fn StaticNumber(
    number: f64,
    #[prop(optional)] pad_start: bool,
    #[prop(optional, into)] decimal_separator: Option<String>,
    #[prop(optional)] decimal_places: Option<u32>,
    #[prop(optional, into)] thousand_separator: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = logic::normalize_optional_text(class_name);
    let (decimal_separator, has_custom_decimal_separator) =
        logic::resolve_decimal_separator(decimal_separator);
    let decimal_places = logic::sanitize_decimal_places(decimal_places);
    let has_custom_decimal_places = decimal_places.is_some();
    let (thousand_separator, has_custom_thousand_separator) =
        logic::resolve_thousand_separator(thousand_separator);

    let number = logic::sanitize_number(number);

    let state = logic::resolve_static_number_state(logic::StaticNumberStateInput {
        value: number,
        has_custom_decimal_separator,
        has_custom_decimal_places,
        has_custom_thousand_separator,
        has_custom_class_name: class_name.is_some(),
    });

    let class = logic::compose_static_number_class_name(class_name, state);

    let formatted = format_static_number(
        number,
        NumberFormatOptions {
            pad_start,
            decimal_separator: &decimal_separator,
            decimal_places,
            thousand_separator: thousand_separator.as_deref(),
        },
    );

    view! {
        <span
            class=class
            data-slot="static-number"
            data-state=state.sign_attr
            data-sign=state.sign_attr
            data-decimal-separator-source=state.decimal_separator_source_attr
            data-decimal-places-source=state.decimal_places_source_attr
            data-thousand-separator-source=state.thousand_separator_source_attr
            data-custom-decimal-separator=state.has_custom_decimal_separator.then_some("true")
            data-custom-decimal-places=state.has_custom_decimal_places.then_some("true")
            data-custom-thousand-separator=state.has_custom_thousand_separator.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
        >
            {formatted}
        </span>
    }
}

#[component]
pub fn SlidingNumber(
    #[prop(into)] number: Signal<f64>,
    #[prop(optional)] motion: SlidingNumberMotion,
    #[prop(optional)] pad_start: bool,
    #[prop(optional, into)] decimal_separator: Option<String>,
    #[prop(optional)] decimal_places: Option<u32>,
    #[prop(optional, into)] thousand_separator: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let decimal_separator = decimal_separator
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(".")
        .to_string();

    #[cfg(target_arch = "wasm32")]
    let decimal_separator_for_view = decimal_separator.clone();
    #[cfg(target_arch = "wasm32")]
    let decimal_separator_for_digits = decimal_separator.clone();

    let thousand_separator = thousand_separator
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);

    #[cfg(target_arch = "wasm32")]
    let thousand_separator_for_view = thousand_separator.clone().unwrap_or_default();

    let base_class = "ui-sliding-number".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let formatted = Signal::derive(move || {
        format_static_number(
            number.get(),
            NumberFormatOptions {
                pad_start,
                decimal_separator: &decimal_separator,
                decimal_places,
                thousand_separator: thousand_separator.as_deref(),
            },
        )
    });

    #[cfg(not(target_arch = "wasm32"))]
    {
        view! {
            <span
                class=class
                data-slot="sliding-number"
                data-animated=motion.animate.then_some("true")
            >
                {move || formatted.get()}
            </span>
        }
        .into_any()
    }

    #[cfg(target_arch = "wasm32")]
    {
        if !motion.animate {
            return view! {
                <span class=class data-slot="sliding-number">
                    {move || formatted.get()}
                </span>
            }
            .into_any();
        }

        let has_thousand_separator = !thousand_separator_for_view.is_empty();
        let thousand_separator = StoredValue::new(thousand_separator_for_view);
        let decimal_separator = StoredValue::new(decimal_separator_for_view);

        let decimal_separator_for_int_digits = decimal_separator_for_digits.clone();
        let decimal_separator_for_dec_digits = decimal_separator_for_digits;

        let int_digits: Signal<Vec<u8>> = Signal::derive(move || {
            let formatted = format_static_number(
                number.get(),
                NumberFormatOptions {
                    pad_start,
                    decimal_separator: &decimal_separator_for_int_digits,
                    decimal_places,
                    thousand_separator: None,
                },
            );
            let formatted = formatted.strip_prefix('-').unwrap_or(&formatted);
            let int_part = formatted
                .split_once(&decimal_separator_for_int_digits)
                .map(|(int, _)| int)
                .unwrap_or(formatted);

            int_part
                .chars()
                .filter_map(|ch| ch.to_digit(10).map(|d| d as u8))
                .collect()
        });

        let dec_digits: Signal<Vec<u8>> = Signal::derive(move || {
            let formatted = format_static_number(
                number.get(),
                NumberFormatOptions {
                    pad_start,
                    decimal_separator: &decimal_separator_for_dec_digits,
                    decimal_places,
                    thousand_separator: None,
                },
            );
            let formatted = formatted.strip_prefix('-').unwrap_or(&formatted);
            let dec_part = formatted
                .split_once(&decimal_separator_for_dec_digits)
                .map(|(_, dec)| dec)
                .unwrap_or("");

            dec_part
                .chars()
                .filter_map(|ch| ch.to_digit(10).map(|d| d as u8))
                .collect()
        });

        let is_negative: Signal<bool> = Signal::derive(move || {
            let value = number.get();
            value.is_finite() && value < 0.0
        });

        let int_indices: Signal<Vec<usize>> =
            Signal::derive(move || (0..int_digits.get().len()).collect());
        let dec_indices: Signal<Vec<usize>> =
            Signal::derive(move || (0..dec_digits.get().len()).collect());

        view! {
            <span class=class data-slot="sliding-number">
                <span class="ui-sliding-number__a11y-value" data-slot="sliding-number-a11y-value">
                    {move || formatted.get()}
                </span>

                <span class="ui-sliding-number__visual" data-slot="sliding-number-visual" aria-hidden="true">
                    <Show when=move || is_negative.get()>
                        <span data-slot="sliding-number-sign">"-"</span>
                    </Show>

                    <For
                        each=move || int_indices.get()
                        key=|idx| *idx
                        children=move |idx| {
                            let digit = Signal::derive(move || int_digits.get().get(idx).copied().unwrap_or(0));
                            let show_separator = Signal::derive(move || {
                                if !has_thousand_separator {
                                    return false;
                                }
                                let len = int_digits.get().len();
                                let digits_to_right = len.saturating_sub(idx + 1);
                                digits_to_right > 0 && digits_to_right.is_multiple_of(3)
                            });

                            view! {
                                <span data-slot="sliding-number-int-group">
                                    <SlidingNumberRoller digit=digit motion=motion />
                                    <Show when=move || show_separator.get()>
                                        <span class="ui-sliding-number__separator" data-slot="sliding-number-separator">
                                            {thousand_separator.get_value()}
                                        </span>
                                    </Show>
                                </span>
                            }
                        }
                    />

                    <Show when=move || !dec_indices.get().is_empty()>
                        <span class="ui-sliding-number__separator" data-slot="sliding-number-decimal-separator">
                            {move || decimal_separator.get_value()}
                        </span>
                        <For
                            each=move || dec_indices.get()
                            key=|idx| *idx
                            children=move |idx| {
                                let digit = Signal::derive(move || dec_digits.get().get(idx).copied().unwrap_or(0));
                                view! { <SlidingNumberRoller digit=digit motion=motion /> }
                            }
                        />
                    </Show>
                </span>
            </span>
        }
        .into_any()
    }
}
