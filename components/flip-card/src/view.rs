use crate::{FlipCardMotion, FlipCardPartStateInput, FlipCardSlot, logic, motion};
use leptos::{children::ViewFn, ev, html, prelude::*};

fn next_id() -> u64 {
    use std::cell::Cell;
    thread_local! {
        static NEXT: Cell<u64> = const { Cell::new(1) };
    }
    NEXT.with(|cell| {
        let id = cell.get();
        cell.set(id + 1);
        id
    })
}

#[component]
pub fn FlipCard(
    #[prop(into)] front: ViewFn,
    #[prop(into)] back: ViewFn,
    #[prop(optional, default = logic::DEFAULT_FLIPPED)] default_flipped: bool,
    #[prop(optional, default = logic::DEFAULT_DISABLED)] disabled: bool,
    #[prop(optional, default = logic::DEFAULT_HOVER_FLIP)] flip_on_hover: bool,
    #[prop(optional)] motion: FlipCardMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    let motion = crate::motion::sanitize_motion(motion);
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let has_custom_motion = motion != FlipCardMotion::default();

    let (id, has_custom_id) = logic::resolve_id(id, format!("ui-flip-card-{}", next_id()));
    let id = StoredValue::new(id);

    let (is_flipped_raw, set_is_flipped_raw) = signal(default_flipped);
    let is_flipped: Signal<bool> = Signal::derive(move || is_flipped_raw.get());

    let (is_hovered_raw, set_is_hovered_raw) = signal(false);
    let is_hovered: Signal<bool> = Signal::derive(move || is_hovered_raw.get());

    let root_state = Memo::new(move |_| {
        logic::resolve_part_state(FlipCardPartStateInput {
            slot: FlipCardSlot::Root,
            disabled,
            is_flipped: is_flipped.get(),
            flip_on_hover,
            has_custom_class_name,
            has_custom_motion,
            has_custom_id,
        })
    });

    let front_state = Memo::new(move |_| {
        logic::resolve_part_state(FlipCardPartStateInput {
            slot: FlipCardSlot::Front,
            disabled,
            is_flipped: is_flipped.get(),
            flip_on_hover,
            has_custom_class_name: false,
            has_custom_motion,
            has_custom_id,
        })
    });

    let back_state = Memo::new(move |_| {
        logic::resolve_part_state(FlipCardPartStateInput {
            slot: FlipCardSlot::Back,
            disabled,
            is_flipped: is_flipped.get(),
            flip_on_hover,
            has_custom_class_name: false,
            has_custom_motion,
            has_custom_id,
        })
    });

    let root_class =
        Memo::new(move |_| logic::compose_class_name(class_name.clone(), root_state.get()));
    let front_class = Memo::new(move |_| logic::compose_class_name(None, front_state.get()));
    let back_class = Memo::new(move |_| logic::compose_class_name(None, back_state.get()));

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, is_flipped, is_hovered, motion);

    let toggle = move || {
        if disabled {
            return;
        }
        set_is_flipped_raw.update(|value| *value = !*value);
    };

    let on_click = move |_ev: ev::MouseEvent| {
        toggle();
    };

    let on_key_down = move |ev: ev::KeyboardEvent| {
        #[cfg(target_arch = "wasm32")]
        let is_composing = ev.is_composing();
        #[cfg(not(target_arch = "wasm32"))]
        let is_composing = false;

        if !logic::should_toggle_key(&ev.key(), is_composing) || disabled {
            return;
        }

        ev.prevent_default();
        ev.stop_propagation();
        toggle();
    };

    let on_pointer_enter = move |_ev: ev::PointerEvent| {
        if disabled {
            return;
        }

        set_is_hovered_raw.set(true);
        if flip_on_hover {
            set_is_flipped_raw.set(true);
        }
    };

    let on_pointer_leave = move |_ev: ev::PointerEvent| {
        set_is_hovered_raw.set(false);
        if flip_on_hover {
            set_is_flipped_raw.set(false);
        }
    };

    let on_focus_in = move |_ev: ev::FocusEvent| {
        if !disabled {
            set_is_hovered_raw.set(true);
        }
    };

    let on_focus_out = move |_ev: ev::FocusEvent| {
        set_is_hovered_raw.set(false);
    };

    let front = StoredValue::new(front);
    let back = StoredValue::new(back);

    view! {
        <div
            class=move || root_class.get()
            id=move || id.with_value(|id| id.clone())
            role="button"
            tabindex=move || if disabled { -1 } else { 0 }
            aria-pressed=move || is_flipped.get()
            aria-disabled=disabled.then_some("true")
            data-slot=move || root_state.get().slot_attr
            data-state=move || root_state.get().state_attr
            data-visible=move || root_state.get().visibility_attr
            data-flipped=move || is_flipped.get().then_some("true")
            data-default=move || (!is_flipped.get()).then_some("true")
            data-disabled=move || root_state.get().is_disabled.then_some("true")
            data-enabled=move || (!root_state.get().is_disabled).then_some("true")
            data-hovered=move || is_hovered.get().then_some("true")
            data-flip-mode=move || root_state.get().flip_mode_attr
            data-class-source=move || root_state.get().class_source_attr
            data-motion-source=move || root_state.get().motion_source_attr
            data-id-source=move || root_state.get().id_source_attr
            data-flip-mode-source=move || root_state.get().flip_mode_source_attr
            data-custom-class=move || root_state.get().has_custom_class_name.then_some("true")
            data-custom-motion=move || root_state.get().has_custom_motion.then_some("true")
            data-custom-id=move || root_state.get().has_custom_id.then_some("true")
            node_ref=root_ref
            on:click=on_click
            on:keydown=on_key_down
            on:pointerenter=on_pointer_enter
            on:pointerleave=on_pointer_leave
            on:focusin=on_focus_in
            on:focusout=on_focus_out
        >
            <div class="ui-flip-card__inner" data-slot="flip-card-inner">
                <div
                    class=move || front_class.get()
                    data-slot=move || front_state.get().slot_attr
                    data-state=move || front_state.get().state_attr
                    data-visible=move || (front_state.get().visibility_attr == "visible").then_some("true")
                    data-hidden=move || (front_state.get().visibility_attr == "hidden").then_some("true")
                >
                    {move || front.with_value(|front| front.run())}
                </div>

                <div
                    class=move || back_class.get()
                    data-slot=move || back_state.get().slot_attr
                    data-state=move || back_state.get().state_attr
                    data-visible=move || (back_state.get().visibility_attr == "visible").then_some("true")
                    data-hidden=move || (back_state.get().visibility_attr == "hidden").then_some("true")
                >
                    {move || back.with_value(|back| back.run())}
                </div>
            </div>
        </div>
    }
}
