use crate::number::{NumberFormatOptions, SlidingNumberMotion, format_static_number};
use leptos::prelude::*;

#[component]
pub fn StaticNumber(
    number: f64,
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
        .unwrap_or(".");
    let thousand_separator = thousand_separator
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());

    let formatted = format_static_number(
        number,
        NumberFormatOptions {
            pad_start,
            decimal_separator,
            decimal_places,
            thousand_separator,
        },
    );

    let base_class = "ui-static-number".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! { <span class=class data-slot="static-number">{formatted}</span> }
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
    let _ = motion;

    let decimal_separator = decimal_separator
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(".")
        .to_string();
    let thousand_separator = thousand_separator
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);

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

    view! {
        <span class=class data-slot="sliding-number">
            {move || formatted.get()}
        </span>
    }
}
