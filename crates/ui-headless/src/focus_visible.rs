use crate::modality::Modality;
use leptos::prelude::*;

#[derive(Clone)]
pub struct FocusVisibleState {
    modality: ReadSignal<Modality>,
    set_modality: WriteSignal<Modality>,
    is_focus_visible: ReadSignal<bool>,
    set_focus_visible: WriteSignal<bool>,
    has_event_before_focus: ReadSignal<bool>,
    set_has_event_before_focus: WriteSignal<bool>,
}

pub fn provide_focus_visible() -> FocusVisibleState {
    let (modality, set_modality) = signal(Modality::Pointer);
    let (is_focus_visible, set_focus_visible) = signal(false);
    let (has_event_before_focus, set_has_event_before_focus) = signal(false);

    let state = FocusVisibleState {
        modality,
        set_modality,
        is_focus_visible,
        set_focus_visible,
        has_event_before_focus,
        set_has_event_before_focus,
    };

    provide_context(state.clone());

    #[cfg(all(feature = "web", target_arch = "wasm32"))]
    setup_global_listeners(state.clone());

    state
}

pub fn use_focus_visible() -> Option<FocusVisibleState> {
    use_context::<FocusVisibleState>()
}

impl FocusVisibleState {
    pub fn modality(&self) -> ReadSignal<Modality> {
        self.modality
    }

    pub fn is_focus_visible(&self) -> ReadSignal<bool> {
        self.is_focus_visible
    }

    pub fn set_modality(&self, modality: Modality) {
        self.set_modality.set(modality);
        self.set_focus_visible
            .set(matches!(modality, Modality::Keyboard | Modality::Virtual));
    }

    #[cfg(all(feature = "web", target_arch = "wasm32"))]
    fn set_modality_from_user_event(&self, modality: Modality) {
        self.set_has_event_before_focus.set(true);
        self.set_modality(modality);
    }

    pub fn clear_event_before_focus(&self) {
        self.set_has_event_before_focus.set(false);
    }

    pub fn has_event_before_focus(&self) -> ReadSignal<bool> {
        self.has_event_before_focus
    }
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
fn setup_global_listeners(state: FocusVisibleState) {
    use send_wrapper::SendWrapper;
    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;

    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };

    let target: SendWrapper<web_sys::EventTarget> = SendWrapper::new(document.into());

    let keydown: SendWrapper<Closure<dyn FnMut(web_sys::KeyboardEvent)>> = SendWrapper::new({
        let state = state.clone();
        Closure::<dyn FnMut(web_sys::KeyboardEvent)>::wrap(Box::new(move |_evt| {
            state.set_modality_from_user_event(Modality::Keyboard);
        }))
    });

    let pointerdown: SendWrapper<Closure<dyn FnMut(web_sys::PointerEvent)>> = SendWrapper::new({
        let state = state.clone();
        Closure::<dyn FnMut(web_sys::PointerEvent)>::wrap(Box::new(move |_evt| {
            state.set_modality_from_user_event(Modality::Pointer);
        }))
    });

    let focusin: SendWrapper<Closure<dyn FnMut(web_sys::FocusEvent)>> = SendWrapper::new({
        let state = state.clone();
        Closure::<dyn FnMut(web_sys::FocusEvent)>::wrap(Box::new(move |_evt| {
            if !state.has_event_before_focus().get_untracked() {
                state.set_modality(Modality::Virtual);
            }
            state.clear_event_before_focus();
        }))
    });

    // We use capture to catch focus-related events reliably and early.
    let _ = target.add_event_listener_with_callback_and_bool(
        "keydown",
        keydown.as_ref().unchecked_ref(),
        true,
    );
    let _ = target.add_event_listener_with_callback_and_bool(
        "pointerdown",
        pointerdown.as_ref().unchecked_ref(),
        true,
    );
    let _ = target.add_event_listener_with_callback_and_bool(
        "focusin",
        focusin.as_ref().unchecked_ref(),
        true,
    );

    on_cleanup(move || {
        let _ = target.remove_event_listener_with_callback_and_bool(
            "keydown",
            keydown.as_ref().unchecked_ref(),
            true,
        );
        let _ = target.remove_event_listener_with_callback_and_bool(
            "pointerdown",
            pointerdown.as_ref().unchecked_ref(),
            true,
        );
        let _ = target.remove_event_listener_with_callback_and_bool(
            "focusin",
            focusin.as_ref().unchecked_ref(),
            true,
        );

        drop(keydown);
        drop(pointerdown);
        drop(focusin);
    });
}
