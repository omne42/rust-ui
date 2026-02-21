use crate::{DropZoneMotion, DroppedFile, motion};
use leptos::{ev, html, prelude::*};
use ui_headless::use_ui_trace;
use ui_headless::{A11yDirection, CommonStrings, locale_attrs, use_ui_i18n};
use ui_headless::{FocusRingOptions, HoverOptions, use_focus_ring, use_hover};
use ui_state_primitives::drop_zone::{DragDepth, resolve_labels};

#[cfg(target_arch = "wasm32")]
fn collect_files_from_data_transfer(dt: &leptos::web_sys::DataTransfer) -> Vec<DroppedFile> {
    let Some(files) = dt.files() else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for idx in 0..files.length() {
        let Some(file) = files.get(idx) else {
            continue;
        };
        out.push(DroppedFile {
            name: file.name(),
            size: file.size().max(0.0) as u64,
            mime: file.type_(),
        });
    }
    out
}

#[cfg(target_arch = "wasm32")]
fn collect_files_from_drag_event(ev: &ev::DragEvent) -> Vec<DroppedFile> {
    let Some(dt) = ev.data_transfer() else {
        return Vec::new();
    };
    collect_files_from_data_transfer(&dt)
}

#[cfg(target_arch = "wasm32")]
fn collect_files_from_clipboard_event(ev: &ev::ClipboardEvent) -> Vec<DroppedFile> {
    let Some(dt) = ev.clipboard_data() else {
        return Vec::new();
    };
    collect_files_from_data_transfer(&dt)
}

#[cfg(not(target_arch = "wasm32"))]
fn collect_files_from_drag_event(_ev: &ev::DragEvent) -> Vec<DroppedFile> {
    Vec::new()
}

#[cfg(not(target_arch = "wasm32"))]
fn collect_files_from_clipboard_event(_ev: &ev::ClipboardEvent) -> Vec<DroppedFile> {
    Vec::new()
}

fn render_label_slot(label: Option<String>) -> impl IntoView {
    label.map(|label| {
        view! {
            <div class="ui-drop-zone__label" data-slot="drop-zone-label">{label}</div>
        }
    })
}

fn render_zone_content<OnPaste>(
    focus_button_ref: NodeRef<html::Button>,
    is_disabled: bool,
    aria_label: String,
    on_focus: Callback<()>,
    on_blur: Callback<()>,
    on_paste: OnPaste,
    children: Children,
) -> impl IntoView
where
    OnPaste: Fn(ev::ClipboardEvent) + 'static,
{
    view! {
        <button
            type="button"
            class="ui-drop-zone__button"
            data-slot="drop-zone-button"
            node_ref=focus_button_ref
            disabled=is_disabled
            aria-label=aria_label
            on:focus=move |_| on_focus.run(())
            on:blur=move |_| on_blur.run(())
            on:paste=on_paste
        ></button>
        {children()}
    }
}

#[cfg(all(target_arch = "wasm32", debug_assertions))]
fn emit_drop_zone_debug_note(trace: StoredValue<Option<ui_headless::UiTrace>>, message: String) {
    if let Some(trace) = trace.get_value() {
        trace.emit("drop-zone", ui_headless::UiTraceEventKind::Note { message });
    }
}

#[cfg(not(all(target_arch = "wasm32", debug_assertions)))]
fn emit_drop_zone_debug_note(_trace: StoredValue<Option<ui_headless::UiTrace>>, _message: String) {}

#[component]
pub fn DropZone(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] motion: Option<DropZoneMotion>,
    #[prop(optional)] on_drop_files: Option<Callback<Vec<DroppedFile>>>,
    children: Children,
) -> impl IntoView {
    let resolved = super::logic::resolve_props(super::logic::DropZonePropsInput {
        disabled_input: super::logic::classify_disabled_input(is_disabled, disabled),
        motion,
    });
    let is_disabled = resolved.is_disabled;
    let disabled_source = resolved.disabled_source;
    let motion = resolved.motion;
    let motion_source = resolved.motion_source;
    let hover = use_hover(HoverOptions { is_disabled });

    let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });

    let zone_ref: NodeRef<html::Div> = NodeRef::new();
    let focus_button_ref: NodeRef<html::Button> = NodeRef::new();

    let drag_depth = StoredValue::new(DragDepth::default());
    let (drag_phase, set_drag_phase) = signal(super::logic::DragLifecyclePhase::Idle);
    let (_drag_over_tick, set_drag_over_tick) = signal(0_u64);
    let (is_drop_target, set_drop_target) = signal(false);
    let on_drop_files = StoredValue::new(on_drop_files);
    let has_drop_callback = on_drop_files.get_value().is_some();
    let trace = StoredValue::new(use_ui_trace());

    motion::attach_motion(
        zone_ref,
        hover.is_hovered,
        is_drop_target,
        focus_ring.is_focused,
        is_disabled,
        motion,
    );

    let i18n = use_ui_i18n();
    let common = i18n.strings::<CommonStrings>();
    let locale = locale_attrs(lang, dir);

    let mut labels = resolve_labels(label, aria_label);
    if !labels.has_custom_aria_label && labels.label.is_none() {
        let i18n_default_aria_label = common.drop_zone_aria_label.trim();
        if !i18n_default_aria_label.is_empty() {
            labels.aria_label = i18n_default_aria_label.into();
        }
    }

    let aria_source = super::logic::resolve_aria_label_source(labels.has_custom_aria_label);
    let agent_contract = Signal::derive(move || {
        super::logic::resolve_agent_contract(super::logic::DropZoneAgentContractInput {
            drag_phase: drag_phase.get(),
            is_disabled,
            disabled_source,
            motion_source,
            aria_source,
            has_drop_callback,
        })
    });

    let on_drag_enter = move |ev: ev::DragEvent| {
        if is_disabled {
            return;
        }
        ev.prevent_default();

        let prev_drop_target = is_drop_target.get_untracked();
        let prev_phase = drag_phase.get_untracked();
        let next = super::logic::reduce_drag_interaction(
            drag_depth.get_value(),
            super::logic::DragInteractionAction::Enter,
        );
        drag_depth.set_value(next.depth);
        set_drop_target.set(next.is_drop_target);

        let next_phase = if next.is_drop_target {
            super::logic::reduce_drag_lifecycle(
                prev_phase,
                super::logic::DragLifecycleAction::DragStart,
            )
        } else {
            prev_phase
        };
        if next.is_drop_target {
            set_drag_phase.set(next_phase);
        }

        emit_drop_zone_debug_note(
            trace,
            format!(
                "event=drag_enter; source=pointer; drop_target:{}->{}; phase:{}->{}",
                prev_drop_target,
                next.is_drop_target,
                prev_phase.as_attr(),
                next_phase.as_attr(),
            ),
        );
    };

    let on_drag_over = move |ev: ev::DragEvent| {
        if is_disabled {
            return;
        }
        ev.prevent_default();

        if drag_phase.get() == super::logic::DragLifecyclePhase::Dragging {
            // Keep high-frequency drag-over feedback in the local view loop.
            set_drag_over_tick.update(|tick| *tick = tick.wrapping_add(1));
        }
    };

    let on_drag_leave = move |ev: ev::DragEvent| {
        if is_disabled {
            return;
        }
        ev.prevent_default();

        let prev_drop_target = is_drop_target.get_untracked();
        let prev_phase = drag_phase.get_untracked();
        let next = super::logic::reduce_drag_interaction(
            drag_depth.get_value(),
            super::logic::DragInteractionAction::Leave,
        );
        drag_depth.set_value(next.depth);
        set_drop_target.set(next.is_drop_target);

        let next_phase = if !next.is_drop_target {
            super::logic::reduce_drag_lifecycle(
                prev_phase,
                super::logic::DragLifecycleAction::DragEnd,
            )
        } else {
            prev_phase
        };
        if !next.is_drop_target {
            set_drag_phase.set(next_phase);
        }

        emit_drop_zone_debug_note(
            trace,
            format!(
                "event=drag_leave; source=pointer; drop_target:{}->{}; phase:{}->{}",
                prev_drop_target,
                next.is_drop_target,
                prev_phase.as_attr(),
                next_phase.as_attr(),
            ),
        );
    };

    let on_drop = move |ev: ev::DragEvent| {
        if is_disabled {
            return;
        }
        ev.prevent_default();

        let prev_drop_target = is_drop_target.get_untracked();
        let prev_phase = drag_phase.get_untracked();
        let next = super::logic::reduce_drag_interaction(
            drag_depth.get_value(),
            super::logic::DragInteractionAction::Drop,
        );
        drag_depth.set_value(next.depth);
        set_drop_target.set(next.is_drop_target);
        let next_phase = super::logic::reduce_drag_lifecycle(
            prev_phase,
            super::logic::DragLifecycleAction::DragEnd,
        );
        set_drag_phase.set(next_phase);

        let files = collect_files_from_drag_event(&ev);
        emit_drop_zone_debug_note(
            trace,
            format!(
                "event=drop; source=pointer; files={}; drop_target:{}->{}; phase:{}->{}",
                files.len(),
                prev_drop_target,
                next.is_drop_target,
                prev_phase.as_attr(),
                next_phase.as_attr(),
            ),
        );
        if let Some(cb) = on_drop_files.get_value() {
            cb.run(files);
        }
    };

    let on_paste = move |ev: ev::ClipboardEvent| {
        if is_disabled {
            return;
        }

        let files = collect_files_from_clipboard_event(&ev);
        if files.is_empty() {
            return;
        }
        ev.prevent_default();

        emit_drop_zone_debug_note(
            trace,
            format!(
                "event=paste; source=keyboard; files={}; phase={}",
                files.len(),
                drag_phase.get_untracked().as_attr(),
            ),
        );
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

            if ui_headless::a11y::should_focus_proxy_button_on_click(&zone, _ev.target()) {
                if let Some(button) = focus_button_ref.get_untracked() {
                    ui_observability::observe_js_result!(button.focus());
                }
            }
        }
    };

    let label_view = render_label_slot(labels.label.clone());
    let zone_content = render_zone_content(
        focus_button_ref,
        is_disabled,
        labels.aria_label.clone(),
        focus_ring.handlers.on_focus,
        focus_ring.handlers.on_blur,
        on_paste,
        children,
    );

    view! {
        <div
            class="ui-drop-zone"
            lang=locale.lang.clone()
            dir=locale.dir
            data-slot="drop-zone"
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-config-policy=move || agent_contract.get().config_policy.as_str()
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
            data-ui-motion-source=move || agent_contract.get().motion_source.as_attr()
            data-ui-aria-source=move || agent_contract.get().aria_source.as_attr()
            data-ui-capability-drop=move || super::logic::bool_data_attr(agent_contract.get().capabilities.can_drop)
            data-ui-capability-paste=move || super::logic::bool_data_attr(agent_contract.get().capabilities.can_paste)
            data-ui-capability-callback=move || super::logic::bool_data_attr(agent_contract.get().capabilities.has_drop_callback)
            data-motion-source=motion_source.as_attr()
            data-custom-motion=(motion_source == super::logic::MotionSource::Custom).then_some("true")
            data-aria-source=aria_source.as_attr()
            data-has-label=labels.label.is_some().then_some("true")
            data-disabled-source=disabled_source.as_attr()
        >
            {label_view}

            <div
                class="ui-drop-zone__zone"
                data-slot="drop-zone-zone"
                node_ref=zone_ref
                role="group"
                aria-label=labels.aria_label.clone()
                aria-disabled=super::logic::bool_data_attr(is_disabled)
                data-drag-phase=move || drag_phase.get().as_attr()
                data-hovered=move || super::logic::bool_data_attr(hover.is_hovered.get())
                data-focused=move || super::logic::bool_data_attr(focus_ring.is_focused.get())
                data-focus-visible=move || super::logic::bool_data_attr(focus_ring.is_focus_visible.get())
                data-drop-target=move || super::logic::bool_data_attr(is_drop_target.get())
                data-disabled=super::logic::bool_data_attr(is_disabled)
                on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
                on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
                on:dragenter=on_drag_enter
                on:dragover=on_drag_over
                on:dragleave=on_drag_leave
                on:drop=on_drop
                on:click=on_click
            >
                {zone_content}
            </div>
        </div>
    }
}

#[cfg(test)]
#[path = "../test/semantics.rs"]
mod semantics_tests;
