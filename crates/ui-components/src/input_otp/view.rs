use crate::input_otp::logic::{apply_otp_backspace, apply_otp_input, normalize_otp_value};
use leptos::{ev, html, prelude::*};
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
fn focus_cell(cell_refs: &Arc<Vec<NodeRef<html::Input>>>, index: usize) {
    let Some(node_ref) = cell_refs.get(index) else {
        return;
    };
    let Some(el) = node_ref.get_untracked() else {
        return;
    };
    let _ = el.focus();
    let _ = el.select();
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_cell(_cell_refs: &Arc<Vec<NodeRef<html::Input>>>, _index: usize) {}

#[component]
pub fn InputOtp(
    id_base: String,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(optional)] length: usize,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let length = length.clamp(1, 12);
    let on_change = StoredValue::new(on_change);

    let aria_label = aria_label
        .filter(|value| !value.trim().is_empty())
        .or_else(|| label.clone())
        .unwrap_or_else(|| "One-time code".to_string());

    let label_id = label.as_ref().map(|_| format!("{id_base}-label"));
    let label_id = StoredValue::new(label_id);
    let label = StoredValue::new(label);

    let base_class = if disabled {
        "ui-input-otp ui-input-otp--disabled".to_string()
    } else {
        "ui-input-otp".to_string()
    };
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let cell_refs: Arc<Vec<NodeRef<html::Input>>> =
        Arc::new((0..length).map(|_| NodeRef::new()).collect());

    let normalized = Signal::derive(move || normalize_otp_value(&value.get(), length));

    let set_and_notify = move |next: String| {
        set_value.set(next.clone());
        if let Some(on_change) = on_change.get_value() {
            on_change.run(next);
        }
    };

    let cells = (0..length)
        .map(|index| {
            let node_ref = cell_refs[index];
            let digit = Signal::derive(move || {
                normalized
                    .get()
                    .chars()
                    .nth(index)
                    .map(|c| c.to_string())
                    .unwrap_or_default()
            });

            let on_input = {
                let cell_refs = cell_refs.clone();
                move |ev: ev::Event| {
                    if disabled {
                        return;
                    }
                    let raw = event_target_value(&ev);
                    let current = normalized.get_untracked();
                    let (next, next_focus) = apply_otp_input(&current, index, &raw, length);
                    set_and_notify(next);
                    if let Some(next_focus) = next_focus {
                        focus_cell(&cell_refs, next_focus);
                    }
                }
            };

            let on_key_down = {
                let cell_refs = cell_refs.clone();
                move |ev: ev::KeyboardEvent| {
                    if disabled {
                        return;
                    }
                    let key = ev.key();
                    match key.as_str() {
                        "Backspace" => {
                            ev.prevent_default();
                            let current = normalized.get_untracked();
                            let (next, focus_index) = apply_otp_backspace(&current, index, length);
                            set_and_notify(next);
                            focus_cell(&cell_refs, focus_index);
                        }
                        "ArrowLeft" => {
                            ev.prevent_default();
                            if index > 0 {
                                focus_cell(&cell_refs, index - 1);
                            }
                        }
                        "ArrowRight" => {
                            ev.prevent_default();
                            if index + 1 < length {
                                focus_cell(&cell_refs, index + 1);
                            }
                        }
                        _ => {}
                    }
                }
            };

            let aria_cell_label = format!("Digit {} of {}", index + 1, length);

            view! {
                <input
                    class="ui-input-otp__cell"
                    node_ref=node_ref
                    id=format!("{id_base}-cell-{index}")
                    type="text"
                    inputmode="numeric"
                    autocomplete="one-time-code"
                    pattern="[0-9]*"
                    aria-label=aria_cell_label
                    disabled=disabled
                    prop:value=move || digit.get()
                    on:input=on_input
                    on:keydown=on_key_down
                />
            }
        })
        .collect_view();

    view! {
        <div
            class=class
            role="group"
            aria-label=aria_label
            aria-labelledby=label_id.get_value()
            data-slot="input-otp"
        >
            {label.get_value().map(|label| {
                let label_id = label_id.get_value();
                view! {
                    <div
                        class="ui-input-otp__label"
                        id=label_id
                        data-slot="input-otp-label"
                    >
                        {label}
                    </div>
                }
            })}

            <div class="ui-input-otp__group" data-slot="input-otp-group">
                {cells}
            </div>
        </div>
    }
}
