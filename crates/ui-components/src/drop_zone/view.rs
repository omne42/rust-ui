use crate::drop_zone::{DropZoneMotion, DroppedFile, motion};
use leptos::{ev, html, prelude::*};
use ui_headless::{FocusRingOptions, HoverOptions, use_focus_ring, use_hover};

#[cfg(target_arch = "wasm32")]
fn is_focusable_element(el: &leptos::web_sys::Element) -> bool {
    let tag = el.tag_name().to_ascii_lowercase();
    match tag.as_str() {
        "button" | "input" | "select" | "textarea" => return true,
        "a" => return el.has_attribute("href"),
        _ => {}
    }

    if el.has_attribute("contenteditable") {
        return true;
    }

    match el
        .get_attribute("tabindex")
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        Some(value) => value.parse::<i32>().is_ok_and(|v| v >= 0),
        None => false,
    }
}

#[component]
pub fn DropZone(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: DropZoneMotion,
    #[prop(optional)] on_drop_files: Option<Callback<Vec<DroppedFile>>>,
    children: Children,
) -> impl IntoView {
    let hover = use_hover(HoverOptions {
        is_disabled: disabled,
    });

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: disabled,
    });

    let zone_ref: NodeRef<html::Div> = NodeRef::new();
    let focus_button_ref: NodeRef<html::Button> = NodeRef::new();

    let drag_depth = StoredValue::new(super::logic::DragDepth::default());
    let (is_drop_target, set_drop_target) = signal(false);
    let on_drop_files = StoredValue::new(on_drop_files);

    motion::attach_motion(
        zone_ref,
        hover.is_hovered,
        is_drop_target,
        focus_ring.is_focused,
        disabled,
        motion,
    );

    let label = label.filter(|value| !value.trim().is_empty());
    let aria_label = aria_label
        .filter(|value| !value.trim().is_empty())
        .or_else(|| label.clone())
        .unwrap_or_else(|| "Drop files".to_string());

    let on_drag_enter = move |ev: ev::DragEvent| {
        if disabled {
            return;
        }
        ev.prevent_default();

        let next = drag_depth.get_value().enter();
        drag_depth.set_value(next);
        set_drop_target.set(next.is_active());
    };

    let on_drag_over = move |ev: ev::DragEvent| {
        if disabled {
            return;
        }
        ev.prevent_default();
    };

    let on_drag_leave = move |ev: ev::DragEvent| {
        if disabled {
            return;
        }
        ev.prevent_default();

        let next = drag_depth.get_value().leave();
        drag_depth.set_value(next);
        set_drop_target.set(next.is_active());
    };

    let on_drop = move |ev: ev::DragEvent| {
        if disabled {
            return;
        }
        ev.prevent_default();

        let next = drag_depth.get_value().reset();
        drag_depth.set_value(next);
        set_drop_target.set(next.is_active());

        let files = super::logic::collect_files_from_drag_event(&ev);
        if let Some(cb) = on_drop_files.get_value() {
            cb.run(files);
        }
    };

    let on_paste = move |ev: ev::ClipboardEvent| {
        if disabled {
            return;
        }

        let files = super::logic::collect_files_from_clipboard_event(&ev);
        if files.is_empty() {
            return;
        }
        ev.prevent_default();

        if let Some(cb) = on_drop_files.get_value() {
            cb.run(files);
        }
    };

    let on_click = move |_ev: ev::MouseEvent| {
        #[cfg(target_arch = "wasm32")]
        {
            use leptos::wasm_bindgen::JsCast;

            let Some(zone) = zone_ref.get_untracked() else {
                return;
            };
            let zone: leptos::web_sys::Element = zone.unchecked_into();

            let Some(target) = _ev.target() else {
                return;
            };
            let Some(mut target) = target
                .clone()
                .dyn_into::<leptos::web_sys::Element>()
                .ok()
                .or_else(|| {
                    target
                        .dyn_into::<leptos::web_sys::Node>()
                        .ok()
                        .and_then(|node| node.parent_element())
                })
            else {
                return;
            };

            loop {
                if is_focusable_element(&target) {
                    return;
                }

                if target.is_same_node(Some(&zone)) {
                    if let Some(button) = focus_button_ref.get_untracked() {
                        let _ = button.focus();
                    }
                    return;
                }

                let Some(parent) = target.parent_element() else {
                    return;
                };
                target = parent;
            }
        }
    };

    view! {
        <div
            class="ui-drop-zone"
            data-slot="drop-zone"
            data-motion-source=if motion == DropZoneMotion::default() {
                "default"
            } else {
                "custom"
            }
            data-custom-motion=(motion != DropZoneMotion::default()).then_some("true")
        >
            {label.clone().map(|label| view! {
                <div class="ui-drop-zone__label" data-slot="drop-zone-label">{label}</div>
            })}

            <div
                class="ui-drop-zone__zone"
                data-slot="drop-zone-zone"
                node_ref=zone_ref
                data-hovered=move || if hover.is_hovered.get() { Some("true") } else { None }
                data-focused=move || if focus_ring.is_focused.get() { Some("true") } else { None }
                data-focus-visible=move || if focus_ring.is_focus_visible.get() { Some("true") } else { None }
                data-drop-target=move || if is_drop_target.get() { Some("true") } else { None }
                data-disabled=disabled.then_some("true")
                on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
                on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
                on:dragenter=on_drag_enter
                on:dragover=on_drag_over
                on:dragleave=on_drag_leave
                on:drop=on_drop
                on:click=on_click
            >
                <button
                    type="button"
                    class="ui-drop-zone__button"
                    node_ref=focus_button_ref
                    disabled=disabled
                    aria-label=aria_label
                    on:focus=move |_| focus_ring.handlers.on_focus.run(())
                    on:blur=move |_| focus_ring.handlers.on_blur.run(())
                    on:paste=on_paste
                ></button>
                {children()}
            </div>
        </div>
    }
}
