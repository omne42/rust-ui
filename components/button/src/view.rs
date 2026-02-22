use crate::button::{
    ButtonColor, ButtonLoadingPlacement, ButtonMotion, ButtonRadius, ButtonSize, ButtonType,
    ButtonVariant, logic, motion,
};
use leptos::{html, prelude::*};
#[cfg(all(
    feature = "button-wasm-debug",
    debug_assertions,
    target_arch = "wasm32"
))]
use std::borrow::Cow;
#[cfg(feature = "component-button_group")]
use ui_headless::labeled_group_attrs;
use ui_headless::{
    A11yDirection, ButtonOptions, FocusRingOptions, HoverOptions, OnPress, popup_trigger_attrs,
    use_button, use_focus_ring, use_hover,
};

const SLOT_BUTTON: &str = "button";
const SLOT_BUTTON_SPINNER: &str = "button-spinner";
const SLOT_BUTTON_LABEL: &str = "button-label";

const CLASS_BUTTON_SPINNER: &str = "ui-button__spinner";
const CLASS_BUTTON_LABEL: &str = "ui-button__label";

#[cfg(all(
    feature = "button-wasm-debug",
    debug_assertions,
    target_arch = "wasm32"
))]
mod wasm_debug {
    use leptos::prelude::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ButtonDebugSource {
        PointerDown,
        PointerUp,
        PointerCancel,
        PointerEnter,
        PointerLeave,
        Click,
        KeyDownEnter,
        KeyDownSpace,
        KeyDownOther,
        KeyUpEnter,
        KeyUpSpace,
        KeyUpOther,
        Focus,
        Blur,
    }

    impl ButtonDebugSource {
        pub fn as_str(self) -> &'static str {
            match self {
                ButtonDebugSource::PointerDown => "pointer-down",
                ButtonDebugSource::PointerUp => "pointer-up",
                ButtonDebugSource::PointerCancel => "pointer-cancel",
                ButtonDebugSource::PointerEnter => "pointer-enter",
                ButtonDebugSource::PointerLeave => "pointer-leave",
                ButtonDebugSource::Click => "click",
                ButtonDebugSource::KeyDownEnter => "keydown-enter",
                ButtonDebugSource::KeyDownSpace => "keydown-space",
                ButtonDebugSource::KeyDownOther => "keydown-other",
                ButtonDebugSource::KeyUpEnter => "keyup-enter",
                ButtonDebugSource::KeyUpSpace => "keyup-space",
                ButtonDebugSource::KeyUpOther => "keyup-other",
                ButtonDebugSource::Focus => "focus",
                ButtonDebugSource::Blur => "blur",
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ButtonDebugState {
        pub is_hovered: bool,
        pub is_pressed: bool,
        pub is_focus_visible: bool,
        pub is_disabled: bool,
        pub is_loading: bool,
    }

    #[derive(Clone)]
    pub struct ButtonDebugEvent {
        pub sequence: usize,
        pub timestamp_ms: f64,
        pub source: ButtonDebugSource,
        pub before: ButtonDebugState,
        pub after: ButtonDebugState,
    }

    #[derive(Clone, Copy)]
    pub struct ButtonDebugStore {
        sequence: RwSignal<usize>,
        pub events: RwSignal<Vec<ButtonDebugEvent>>,
    }

    impl ButtonDebugStore {
        pub fn new() -> Self {
            Self {
                sequence: RwSignal::new(0),
                events: RwSignal::new(Vec::new()),
            }
        }

        pub fn record(
            self,
            source: ButtonDebugSource,
            before: ButtonDebugState,
            after: ButtonDebugState,
        ) {
            if before == after {
                return;
            }

            let sequence = self.sequence.get_untracked().saturating_add(1);
            self.sequence.set(sequence);
            let timestamp_ms = js_sys::Date::now();

            self.events.update(|events| {
                events.push(ButtonDebugEvent {
                    sequence,
                    timestamp_ms,
                    source,
                    before,
                    after,
                });
                if events.len() > 200 {
                    let overflow = events.len().saturating_sub(200);
                    events.drain(0..overflow);
                }
            });

            tracing::event!(
                target: "ui::button::state_change",
                tracing::Level::DEBUG,
                sequence,
                timestamp_ms,
                source = source.as_str(),
                before = %format_state(before),
                after = %format_state(after),
                "button state transition"
            );
        }
    }

    pub fn format_state(state: ButtonDebugState) -> String {
        format!(
            "hovered={} pressed={} focus_visible={} disabled={} loading={}",
            state.is_hovered,
            state.is_pressed,
            state.is_focus_visible,
            state.is_disabled,
            state.is_loading
        )
    }
}

#[cfg(all(
    feature = "button-wasm-debug",
    debug_assertions,
    target_arch = "wasm32"
))]
fn snapshot_debug_state(
    is_hovered: ReadSignal<bool>,
    is_pressed: ReadSignal<bool>,
    is_focus_visible: Memo<bool>,
    state: logic::ButtonState,
) -> wasm_debug::ButtonDebugState {
    wasm_debug::ButtonDebugState {
        is_hovered: is_hovered.get_untracked(),
        is_pressed: is_pressed.get_untracked(),
        is_focus_visible: is_focus_visible.get_untracked(),
        is_disabled: state.is_disabled,
        is_loading: state.is_loading,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ButtonDebugTransitionSource {
    PointerDown,
    PointerUp,
    PointerCancel,
    PointerEnter,
    PointerLeave,
    Click,
    KeyDownEnter,
    KeyDownSpace,
    KeyDownOther,
    KeyUpEnter,
    KeyUpSpace,
    KeyUpOther,
    Focus,
    Blur,
}

#[cfg(all(
    feature = "button-wasm-debug",
    debug_assertions,
    target_arch = "wasm32"
))]
impl ButtonDebugTransitionSource {
    fn as_wasm_source(self) -> wasm_debug::ButtonDebugSource {
        match self {
            Self::PointerDown => wasm_debug::ButtonDebugSource::PointerDown,
            Self::PointerUp => wasm_debug::ButtonDebugSource::PointerUp,
            Self::PointerCancel => wasm_debug::ButtonDebugSource::PointerCancel,
            Self::PointerEnter => wasm_debug::ButtonDebugSource::PointerEnter,
            Self::PointerLeave => wasm_debug::ButtonDebugSource::PointerLeave,
            Self::Click => wasm_debug::ButtonDebugSource::Click,
            Self::KeyDownEnter => wasm_debug::ButtonDebugSource::KeyDownEnter,
            Self::KeyDownSpace => wasm_debug::ButtonDebugSource::KeyDownSpace,
            Self::KeyDownOther => wasm_debug::ButtonDebugSource::KeyDownOther,
            Self::KeyUpEnter => wasm_debug::ButtonDebugSource::KeyUpEnter,
            Self::KeyUpSpace => wasm_debug::ButtonDebugSource::KeyUpSpace,
            Self::KeyUpOther => wasm_debug::ButtonDebugSource::KeyUpOther,
            Self::Focus => wasm_debug::ButtonDebugSource::Focus,
            Self::Blur => wasm_debug::ButtonDebugSource::Blur,
        }
    }

    fn from_wasm_source(source: wasm_debug::ButtonDebugSource) -> Self {
        match source {
            wasm_debug::ButtonDebugSource::PointerDown => Self::PointerDown,
            wasm_debug::ButtonDebugSource::PointerUp => Self::PointerUp,
            wasm_debug::ButtonDebugSource::PointerCancel => Self::PointerCancel,
            wasm_debug::ButtonDebugSource::PointerEnter => Self::PointerEnter,
            wasm_debug::ButtonDebugSource::PointerLeave => Self::PointerLeave,
            wasm_debug::ButtonDebugSource::Click => Self::Click,
            wasm_debug::ButtonDebugSource::KeyDownEnter => Self::KeyDownEnter,
            wasm_debug::ButtonDebugSource::KeyDownSpace => Self::KeyDownSpace,
            wasm_debug::ButtonDebugSource::KeyDownOther => Self::KeyDownOther,
            wasm_debug::ButtonDebugSource::KeyUpEnter => Self::KeyUpEnter,
            wasm_debug::ButtonDebugSource::KeyUpSpace => Self::KeyUpSpace,
            wasm_debug::ButtonDebugSource::KeyUpOther => Self::KeyUpOther,
            wasm_debug::ButtonDebugSource::Focus => Self::Focus,
            wasm_debug::ButtonDebugSource::Blur => Self::Blur,
        }
    }
}

#[cfg(all(
    feature = "button-wasm-debug",
    debug_assertions,
    target_arch = "wasm32"
))]
#[derive(Clone, Copy)]
struct ButtonDebugRuntime {
    store: wasm_debug::ButtonDebugStore,
    is_hovered: ReadSignal<bool>,
    is_pressed: ReadSignal<bool>,
    is_focus_visible: Memo<bool>,
    state: logic::ButtonState,
}

#[cfg(not(all(
    feature = "button-wasm-debug",
    debug_assertions,
    target_arch = "wasm32"
)))]
#[derive(Clone, Copy, Default)]
struct ButtonDebugRuntime;

#[cfg(all(
    feature = "button-wasm-debug",
    debug_assertions,
    target_arch = "wasm32"
))]
impl ButtonDebugRuntime {
    fn new(
        store: wasm_debug::ButtonDebugStore,
        is_hovered: ReadSignal<bool>,
        is_pressed: ReadSignal<bool>,
        is_focus_visible: Memo<bool>,
        state: logic::ButtonState,
    ) -> Self {
        Self {
            store,
            is_hovered,
            is_pressed,
            is_focus_visible,
            state,
        }
    }

    fn record_transition(self, source: ButtonDebugTransitionSource, run: impl FnOnce()) {
        let before = snapshot_debug_state(
            self.is_hovered,
            self.is_pressed,
            self.is_focus_visible,
            self.state,
        );
        run();
        let after = snapshot_debug_state(
            self.is_hovered,
            self.is_pressed,
            self.is_focus_visible,
            self.state,
        );
        let source = source.as_wasm_source();
        let debug_store = self.store;
        debug_store.record(source, before, after);
    }
}

#[cfg(not(all(
    feature = "button-wasm-debug",
    debug_assertions,
    target_arch = "wasm32"
)))]
impl ButtonDebugRuntime {
    fn new(
        _is_hovered: ReadSignal<bool>,
        _is_pressed: ReadSignal<bool>,
        _is_focus_visible: Memo<bool>,
        _state: logic::ButtonState,
    ) -> Self {
        Self
    }

    fn record_transition(self, _source: ButtonDebugTransitionSource, run: impl FnOnce()) {
        run();
    }
}

#[derive(Clone)]
struct ButtonDomHandlers {
    on_pointer_down: Callback<leptos::ev::PointerEvent>,
    on_pointer_up: Callback<leptos::ev::PointerEvent>,
    on_pointer_cancel: Callback<leptos::ev::PointerEvent>,
    on_pointer_enter: Callback<leptos::ev::PointerEvent>,
    on_pointer_leave: Callback<leptos::ev::PointerEvent>,
    on_click: Callback<leptos::ev::MouseEvent>,
    on_key_down: Callback<leptos::ev::KeyboardEvent>,
    on_key_up: Callback<leptos::ev::KeyboardEvent>,
    on_focus: Callback<leptos::ev::FocusEvent>,
    on_blur: Callback<leptos::ev::FocusEvent>,
}

#[cfg(all(
    feature = "button-wasm-debug",
    debug_assertions,
    target_arch = "wasm32"
))]
#[derive(Clone)]
struct ButtonReplayHandlers {
    on_pointer_down: Callback<()>,
    on_pointer_up: Callback<()>,
    on_pointer_cancel: Callback<()>,
    on_pointer_enter: Callback<()>,
    on_pointer_leave: Callback<()>,
    on_click: Callback<()>,
    on_key_down: Callback<String, bool>,
    on_key_up: Callback<String, bool>,
    on_focus: Callback<()>,
    on_blur_press: Callback<()>,
    on_blur_focus: Callback<()>,
}

#[cfg(all(
    feature = "button-wasm-debug",
    debug_assertions,
    target_arch = "wasm32"
))]
impl ButtonReplayHandlers {
    fn replay(&self, source: wasm_debug::ButtonDebugSource) {
        match source {
            wasm_debug::ButtonDebugSource::PointerDown => self.on_pointer_down.run(()),
            wasm_debug::ButtonDebugSource::PointerUp => self.on_pointer_up.run(()),
            wasm_debug::ButtonDebugSource::PointerCancel => self.on_pointer_cancel.run(()),
            wasm_debug::ButtonDebugSource::PointerEnter => self.on_pointer_enter.run(()),
            wasm_debug::ButtonDebugSource::PointerLeave => self.on_pointer_leave.run(()),
            wasm_debug::ButtonDebugSource::Click => self.on_click.run(()),
            wasm_debug::ButtonDebugSource::KeyDownEnter => {
                if self.on_key_down.run("Enter".to_string()) {}
            }
            wasm_debug::ButtonDebugSource::KeyDownSpace => {
                if self.on_key_down.run(" ".to_string()) {}
            }
            wasm_debug::ButtonDebugSource::KeyDownOther => {
                if self.on_key_down.run("KeyA".to_string()) {}
            }
            wasm_debug::ButtonDebugSource::KeyUpEnter => {
                if self.on_key_up.run("Enter".to_string()) {}
            }
            wasm_debug::ButtonDebugSource::KeyUpSpace => if self.on_key_up.run(" ".to_string()) {},
            wasm_debug::ButtonDebugSource::KeyUpOther => {
                if self.on_key_up.run("KeyA".to_string()) {}
            }
            wasm_debug::ButtonDebugSource::Focus => self.on_focus.run(()),
            wasm_debug::ButtonDebugSource::Blur => {
                self.on_blur_press.run(());
                self.on_blur_focus.run(());
            }
        }
    }
}

struct ButtonDebuggableHandlers {
    dom: ButtonDomHandlers,
    #[cfg(all(
        feature = "button-wasm-debug",
        debug_assertions,
        target_arch = "wasm32"
    ))]
    replay: ButtonReplayHandlers,
}

struct ButtonAriaViewBindings {
    is_pressed: ReadSignal<bool>,
    attrs: ui_headless::ButtonAttrs,
}

fn use_debuggable_handlers(
    press_handlers: ui_headless::PressHandlers,
    hover_handlers: ui_headless::HoverHandlers,
    focus_handlers: ui_headless::FocusRingHandlers,
    debug_runtime: ButtonDebugRuntime,
) -> ButtonDebuggableHandlers {
    #[cfg(all(
        feature = "button-wasm-debug",
        debug_assertions,
        target_arch = "wasm32"
    ))]
    let replay = ButtonReplayHandlers {
        on_pointer_down: press_handlers.on_pointer_down.clone(),
        on_pointer_up: press_handlers.on_pointer_up.clone(),
        on_pointer_cancel: press_handlers.on_pointer_cancel.clone(),
        on_pointer_enter: hover_handlers.on_pointer_enter.clone(),
        on_pointer_leave: hover_handlers.on_pointer_leave.clone(),
        on_click: press_handlers.on_click.clone(),
        on_key_down: press_handlers.on_key_down.clone(),
        on_key_up: press_handlers.on_key_up.clone(),
        on_focus: focus_handlers.on_focus.clone(),
        on_blur_press: press_handlers.on_blur.clone(),
        on_blur_focus: focus_handlers.on_blur.clone(),
    };

    let on_pointer_down_handler = press_handlers.on_pointer_down;
    let on_pointer_up_handler = press_handlers.on_pointer_up;
    let on_pointer_cancel_handler = press_handlers.on_pointer_cancel;
    let on_pointer_enter_handler = hover_handlers.on_pointer_enter;
    let on_pointer_leave_handler = hover_handlers.on_pointer_leave;
    let on_click_handler = press_handlers.on_click;
    let on_key_down_handler = press_handlers.on_key_down;
    let on_key_up_handler = press_handlers.on_key_up;
    let on_focus_handler = focus_handlers.on_focus;
    let on_blur_press_handler = press_handlers.on_blur;
    let on_blur_focus_handler = focus_handlers.on_blur;

    ButtonDebuggableHandlers {
        dom: ButtonDomHandlers {
            on_pointer_down: Callback::new(move |_ev: leptos::ev::PointerEvent| {
                debug_runtime.record_transition(ButtonDebugTransitionSource::PointerDown, || {
                    on_pointer_down_handler.run(());
                });
            }),
            on_pointer_up: Callback::new(move |_ev: leptos::ev::PointerEvent| {
                debug_runtime.record_transition(ButtonDebugTransitionSource::PointerUp, || {
                    on_pointer_up_handler.run(());
                });
            }),
            on_pointer_cancel: Callback::new(move |_ev: leptos::ev::PointerEvent| {
                debug_runtime.record_transition(ButtonDebugTransitionSource::PointerCancel, || {
                    on_pointer_cancel_handler.run(());
                });
            }),
            on_pointer_enter: Callback::new(move |_ev: leptos::ev::PointerEvent| {
                debug_runtime.record_transition(ButtonDebugTransitionSource::PointerEnter, || {
                    on_pointer_enter_handler.run(());
                });
            }),
            on_pointer_leave: Callback::new(move |_ev: leptos::ev::PointerEvent| {
                debug_runtime.record_transition(ButtonDebugTransitionSource::PointerLeave, || {
                    on_pointer_leave_handler.run(());
                });
            }),
            on_click: Callback::new(move |_ev: leptos::ev::MouseEvent| {
                debug_runtime.record_transition(ButtonDebugTransitionSource::Click, || {
                    on_click_handler.run(());
                });
            }),
            on_key_down: Callback::new(move |ev: leptos::ev::KeyboardEvent| {
                let key = ev.key();
                let source = debug_key_down_source(&key);
                debug_runtime.record_transition(source, || {
                    if on_key_down_handler.run(key) {
                        ev.prevent_default();
                    }
                });
            }),
            on_key_up: Callback::new(move |ev: leptos::ev::KeyboardEvent| {
                let key = ev.key();
                let source = debug_key_up_source(&key);
                debug_runtime.record_transition(source, || {
                    if on_key_up_handler.run(key) {
                        ev.prevent_default();
                    }
                });
            }),
            on_focus: Callback::new(move |_ev: leptos::ev::FocusEvent| {
                debug_runtime.record_transition(ButtonDebugTransitionSource::Focus, || {
                    on_focus_handler.run(());
                });
            }),
            on_blur: Callback::new(move |_ev: leptos::ev::FocusEvent| {
                debug_runtime.record_transition(ButtonDebugTransitionSource::Blur, || {
                    on_blur_press_handler.run(());
                    on_blur_focus_handler.run(());
                });
            }),
        },
        #[cfg(all(
            feature = "button-wasm-debug",
            debug_assertions,
            target_arch = "wasm32"
        ))]
        replay,
    }
}

fn debug_key_down_source(key: &str) -> ButtonDebugTransitionSource {
    match key {
        "Enter" => ButtonDebugTransitionSource::KeyDownEnter,
        " " | "Space" | "Spacebar" => ButtonDebugTransitionSource::KeyDownSpace,
        _ => ButtonDebugTransitionSource::KeyDownOther,
    }
}

fn debug_key_up_source(key: &str) -> ButtonDebugTransitionSource {
    match key {
        "Enter" => ButtonDebugTransitionSource::KeyUpEnter,
        " " | "Space" | "Spacebar" => ButtonDebugTransitionSource::KeyUpSpace,
        _ => ButtonDebugTransitionSource::KeyUpOther,
    }
}

#[cfg(all(
    feature = "button-wasm-debug",
    debug_assertions,
    target_arch = "wasm32"
))]
fn render_debug_panel(
    debug_store: wasm_debug::ButtonDebugStore,
    request_replay: Callback<wasm_debug::ButtonDebugSource>,
) -> impl IntoView {
    let events = debug_store.events;
    view! {
        <details class="ui-button__debug" data-slot="button-debug" open>
            <summary data-slot="button-debug-entry">
                "Button Debug (wasm dev)"
            </summary>
            <ul data-slot="button-debug-list">
                {move || {
                    events
                        .get()
                        .into_iter()
                        .rev()
                        .map(|event| {
                            let source: Cow<'static, str> = Cow::Borrowed(event.source.as_str());
                            let before_text = wasm_debug::format_state(event.before);
                            let after_text = wasm_debug::format_state(event.after);
                            let before_attr = before_text.clone();
                            let after_attr = after_text.clone();
                            view! {
                                <li
                                    data-slot="button-debug-event"
                                    data-debug-sequence=event.sequence
                                    data-debug-source=source.clone()
                                    data-debug-before=before_attr
                                    data-debug-after=after_attr
                                    data-debug-timestamp-ms=format!("{:.0}", event.timestamp_ms)
                                >
                                    <code>{format!(
                                        "#{} @ {:.0}ms {}",
                                        event.sequence, event.timestamp_ms, source.as_ref()
                                    )}</code>
                                    <div>"before: " {before_text}</div>
                                    <div>"after: " {after_text}</div>
                                    <button
                                        type="button"
                                        data-slot="button-debug-replay"
                                        on:click=move |_| request_replay.run(event.source)
                                    >
                                        "Replay"
                                    </button>
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>
        </details>
    }
}

fn render_spinner() -> impl IntoView {
    view! { <span class=CLASS_BUTTON_SPINNER data-slot=SLOT_BUTTON_SPINNER aria-hidden="true"></span> }
}

pub(crate) fn render_button_content(
    render: logic::ButtonRenderState,
    children: Children,
) -> impl IntoView {
    view! {
        <Show when=move || render.show_start_inline_spinner>{render_spinner()}</Show>
        <span class=CLASS_BUTTON_LABEL data-slot=SLOT_BUTTON_LABEL>
            {children()}
        </span>
        <Show when=move || render.show_end_spinner>{render_spinner()}</Show>
        <Show when=move || render.show_center_spinner>{render_spinner()}</Show>
    }
}

#[component]
pub fn Button(
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional)] is_disabled: bool,
    #[prop(optional)] is_loading: bool,
    #[prop(optional, into)] variant: ButtonVariant,
    #[prop(optional, into)] color: ButtonColor,
    #[prop(optional, into)] radius: ButtonRadius,
    #[prop(optional, into)] size: ButtonSize,
    #[prop(optional)] is_full_width: bool,
    #[prop(optional)] motion: ButtonMotion,
    #[prop(optional)] loading_placement: ButtonLoadingPlacement,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] schema_json: Option<String>,
    #[prop(optional, into)] button_type: ButtonType,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] aria_haspopup: Option<&'static str>,
    #[prop(optional)] aria_expanded: Option<Signal<bool>>,
    #[prop(optional, into)] aria_controls: Option<String>,
    #[prop(optional)] aria_controls_signal: Option<Signal<Option<String>>>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] node_ref: NodeRef<html::Button>,
    #[prop(optional)] on_press: Option<OnPress>,
    children: Children,
) -> impl IntoView {
    let normalized = logic::normalize_input(logic::ButtonInputNormalizationInput {
        is_disabled,
        is_full_width,
        class_name,
        aria_label,
        button_type,
    });
    let normalized_button_type = normalized.button_type;
    let normalized_aria_label = normalized.aria_label.clone();
    let normalized_aria_label_source = normalized.aria_label_source;
    let view_state = logic::resolve_view_state(logic::ButtonLogicInput {
        normalized,
        is_loading,
        variant,
        color,
        radius,
        size,
        loading_placement,
        has_custom_motion: motion != ButtonMotion::default(),
    });
    let state = view_state.state;
    let render = view_state.render;

    let aria_state = use_button(ButtonOptions {
        is_disabled: state.is_disabled,
        on_press,
        ..Default::default()
    });

    let focus_ring = use_focus_ring(FocusRingOptions {
        is_disabled: state.is_disabled,
    });

    let hover = use_hover(HoverOptions {
        is_disabled: state.is_disabled,
    });
    let ui_headless::ButtonAria {
        is_pressed,
        handlers: button_handlers,
        attrs: button_attrs,
        ..
    } = aria_state;
    let aria = ButtonAriaViewBindings {
        is_pressed,
        attrs: button_attrs,
    };
    let ui_headless::FocusRingState {
        is_focus_visible,
        handlers: focus_handlers,
        ..
    } = focus_ring;
    let ui_headless::HoverState {
        is_hovered,
        handlers: hover_handlers,
    } = hover;
    let press_handlers = button_handlers.press;

    motion::attach_motion(
        node_ref,
        is_hovered,
        aria.is_pressed,
        state.is_disabled,
        motion,
    );

    let class = view_state.class_name;
    let normalized_schema = logic::normalize_schema_json_input(schema_json);
    let has_popup_trigger = aria_haspopup.is_some();
    let agent_contract = logic::resolve_agent_contract(state, has_popup_trigger);
    let output_status = logic::resolve_output_status(state, normalized_button_type);
    let popup_a11y = popup_trigger_attrs(
        aria_haspopup,
        aria_controls,
        aria_controls_signal,
        aria_expanded,
        lang,
        dir,
    );

    #[cfg(all(
        feature = "button-wasm-debug",
        debug_assertions,
        target_arch = "wasm32"
    ))]
    let debug_store = wasm_debug::ButtonDebugStore::new();
    #[cfg(all(
        feature = "button-wasm-debug",
        debug_assertions,
        target_arch = "wasm32"
    ))]
    let debug_runtime = ButtonDebugRuntime::new(
        debug_store,
        is_hovered,
        aria.is_pressed,
        is_focus_visible,
        state,
    );
    #[cfg(not(all(
        feature = "button-wasm-debug",
        debug_assertions,
        target_arch = "wasm32"
    )))]
    let debug_runtime =
        ButtonDebugRuntime::new(is_hovered, aria.is_pressed, is_focus_visible, state);

    let debuggable_handlers = use_debuggable_handlers(
        press_handlers,
        hover_handlers,
        focus_handlers,
        debug_runtime,
    );
    #[cfg(all(
        feature = "button-wasm-debug",
        debug_assertions,
        target_arch = "wasm32"
    ))]
    let ButtonDebuggableHandlers {
        dom,
        replay: replay_handlers,
    } = debuggable_handlers;
    #[cfg(not(all(
        feature = "button-wasm-debug",
        debug_assertions,
        target_arch = "wasm32"
    )))]
    let ButtonDebuggableHandlers { dom } = debuggable_handlers;
    let ButtonDomHandlers {
        on_pointer_down: on_pointer_down_handler,
        on_pointer_up: on_pointer_up_handler,
        on_pointer_cancel: on_pointer_cancel_handler,
        on_pointer_enter: on_pointer_enter_handler,
        on_pointer_leave: on_pointer_leave_handler,
        on_click: on_click_handler,
        on_key_down: on_key_down_handler,
        on_key_up: on_key_up_handler,
        on_focus: on_focus_handler,
        on_blur: on_blur_handler,
    } = dom;
    let on_pointer_down = move |ev| on_pointer_down_handler.run(ev);
    let on_pointer_up = move |ev| on_pointer_up_handler.run(ev);
    let on_pointer_cancel = move |ev| on_pointer_cancel_handler.run(ev);
    let on_pointer_enter = move |ev| on_pointer_enter_handler.run(ev);
    let on_pointer_leave = move |ev| on_pointer_leave_handler.run(ev);
    let on_click = move |ev| on_click_handler.run(ev);
    let on_key_down = move |ev| on_key_down_handler.run(ev);
    let on_key_up = move |ev| on_key_up_handler.run(ev);
    let on_focus = move |ev| on_focus_handler.run(ev);
    let on_blur = move |ev| on_blur_handler.run(ev);

    let debug_panel: Option<AnyView> = crate::wasm_debug_proxy!(
        "button-wasm-debug",
        {
            let request_replay = Callback::new(move |source: wasm_debug::ButtonDebugSource| {
                let transition_source = ButtonDebugTransitionSource::from_wasm_source(source);
                debug_runtime.record_transition(transition_source, || {
                    replay_handlers.replay(source);
                });
            });
            Some(render_debug_panel(debug_store, request_replay).into_any())
        },
        { None }
    );

    view! {
        <button
            id=id
            type=normalized_button_type.as_attr()
            node_ref=node_ref
            class=class
            class:ui-button--focus-visible=move || is_focus_visible.get()
            disabled=state.is_disabled
            data-slot=SLOT_BUTTON
            data-state=state.state_attr
            data-focus-visible=move || {
                if is_focus_visible.get() {
                    Some("true")
                } else {
                    None
                }
            }
            data-hovered=move || if is_hovered.get() { Some("true") } else { None }
            data-pressed=move || if aria.is_pressed.get() { Some("true") } else { None }
            data-loading=state.is_loading.then_some("true")
            data-loading-source=view_state.source.loading_source_attr
            data-disabled-source=view_state.source.disabled_source_attr
            data-disabled-input-source=view_state.source.disabled_input_source_attr
            data-full-width-source=view_state.source.full_width_input_source_attr
            data-loading-placement=state.loading_placement_attr
            data-full-width=state.is_full_width.then_some("true")
            data-label-source=normalized_aria_label_source.as_attr()
            data-color=state.color_attr
            data-radius=state.radius_attr
            data-ui-schema=agent_contract.schema_name
            data-ui-schema-version=agent_contract.schema_version.as_str()
            data-ui-schema-payload=normalized_schema.schema_json
            data-ui-schema-input-source=normalized_schema.source.as_attr()
            data-ui-agent-schema=agent_contract.schema_name
            data-ui-agent-schema-version=agent_contract.schema_version.as_str()
            data-ui-intent=agent_contract.intent.as_str()
            data-ui-action=agent_contract.action.as_str()
            data-ui-state=agent_contract.state.as_str()
            data-ui-source=agent_contract.source.as_str()
            data-ui-output-status=output_status.as_attr()
            data-ui-capability-press=agent_contract.capabilities.can_press.then_some("true")
            data-ui-capability-focus=agent_contract.capabilities.can_focus.then_some("true")
            data-ui-capability-hover=agent_contract.capabilities.can_hover.then_some("true")
            data-ui-capability-popup-trigger=agent_contract
                .capabilities
                .can_popup_trigger
                .then_some("true")
            data-motion-source=if state.has_custom_motion {
                "custom"
            } else {
                "default"
            }
            data-custom-motion=state.has_custom_motion.then_some("true")
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            aria-label=normalized_aria_label
            aria-haspopup=popup_a11y.aria_haspopup
            aria-controls=move || popup_a11y.aria_controls.get()
            aria-busy=state.is_loading.then_some("true")
            aria-expanded=move || popup_a11y.aria_expanded.get()
            lang=popup_a11y.lang.clone()
            dir=popup_a11y.dir
            on:pointerdown=on_pointer_down
            on:pointerup=on_pointer_up
            on:pointercancel=on_pointer_cancel
            on:pointerenter=on_pointer_enter
            on:pointerleave=on_pointer_leave
            on:click=on_click
            on:keydown=on_key_down
            on:keyup=on_key_up
            on:focus=on_focus
            on:blur=on_blur
        >
            {render_button_content(render, children)}
        </button>
        {debug_panel}
    }
}

#[cfg(feature = "component-button_group")]
#[leptos::component]
pub fn ButtonGroup(
    #[prop(optional)] orientation: logic::ButtonGroupOrientation,
    #[prop(optional)] is_attached: bool,
    #[prop(optional)] motion: motion::ButtonGroupMotion,
    #[prop(optional)] node_ref: NodeRef<html::Div>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let (aria_label, has_explicit_label) = logic::normalize_button_group_aria_label(aria_label);
    let group_a11y = labeled_group_attrs(aria_label, lang, dir);
    let class_name = logic::normalize_optional_text(class_name);
    let state = Memo::new(move |_| {
        logic::resolve_button_group_state(orientation, is_attached, has_explicit_label)
    });
    let class = logic::compose_button_group_class_name(class_name, orientation, is_attached);

    motion::attach_button_group_motion(node_ref, motion);

    let motion_source = if motion == motion::ButtonGroupMotion::default() {
        "default"
    } else {
        "custom"
    };
    let custom_motion = (motion != motion::ButtonGroupMotion::default()).then_some("true");

    view! {
        <div
            node_ref=node_ref
            class=class
            data-slot="button-group"
            data-orientation=orientation.data_orientation()
            data-horizontal=move || state.get().is_horizontal.then_some("true")
            data-vertical=move || state.get().is_vertical.then_some("true")
            data-attached=move || state.get().is_attached.then_some("true")
            data-detached=move || state.get().is_detached.then_some("true")
            data-has-explicit-label=move || state.get().has_explicit_label.then_some("true")
            data-has-fallback-label=move || state.get().has_fallback_label.then_some("true")
            data-motion-source=motion_source
            data-custom-motion=custom_motion
            role=group_a11y.role
            aria-label=group_a11y.aria_label.clone()
            lang=group_a11y.lang.clone()
            dir=group_a11y.dir
        >
            {children()}
        </div>
    }
}
