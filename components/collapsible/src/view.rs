use crate::{
    CollapsibleMotion, CollapsibleStateInput,
    logic::{self},
};
use leptos::{html, prelude::*};
use ui_disclosure::DisclosureIds;
use ui_headless::{
    ButtonAria, ButtonOptions, DisclosureTriggerA11yAttrs, FocusRingOptions, FocusRingState,
    HoverOptions, HoverState, UiTraceEventKind, disclosure_trigger_attrs, locale_attrs, use_button,
    use_focus_ring, use_hover, use_ui_trace,
};

const SLOT_COLLAPSIBLE: &str = "collapsible";
const SLOT_COLLAPSIBLE_TRIGGER: &str = "collapsible-trigger";
const SLOT_COLLAPSIBLE_LABEL: &str = "collapsible-label";
const SLOT_COLLAPSIBLE_INDICATOR: &str = "collapsible-indicator";
const SLOT_COLLAPSIBLE_PANEL: &str = "collapsible-panel";
const SLOT_COLLAPSIBLE_PANEL_SURFACE: &str = "collapsible-panel-surface";
const ARIA_HIDDEN_TRUE: &str = "true";
const COLLAPSIBLE_INDICATOR_GLYPH: &str = "›";

struct TriggerViewModel {
    trigger_id: String,
    title: StoredValue<String>,
    indicator_ref: NodeRef<html::Span>,
    state: Memo<logic::CollapsibleState>,
    open: Signal<bool>,
    trigger_a11y: DisclosureTriggerA11yAttrs,
    aria_label: String,
    is_disabled: bool,
    hover: HoverState,
    focus_ring: FocusRingState,
    aria: ButtonAria,
}

fn render_trigger(input: TriggerViewModel) -> impl IntoView {
    let TriggerViewModel {
        trigger_id,
        title,
        indicator_ref,
        state,
        open,
        trigger_a11y,
        aria_label,
        is_disabled,
        hover,
        focus_ring,
        aria,
    } = input;

    view! {
        <button
            type="button"
            class="ui-disclosure__trigger ui-collapsible__trigger"
            class:ui-disclosure__trigger--focus-visible=move || focus_ring.is_focus_visible.get()
            id=trigger_id
            data-slot=SLOT_COLLAPSIBLE_TRIGGER
            data-state=move || state.get().state_attr
            data-open-mode=move || state.get().open_mode_attr
            data-label-source=move || state.get().label_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-open-value-source=move || state.get().open_value_source_attr
            data-open-change-source=move || state.get().open_change_source_attr
            data-custom-motion=move || state.get().motion_source.is_custom().then_some("true")
            lang=trigger_a11y.lang
            dir=trigger_a11y.dir
            aria-label=aria_label
            aria-expanded=trigger_a11y.aria_expanded
            aria-controls=trigger_a11y.aria_controls
            disabled=is_disabled
            data-open=move || if open.get() { Some("true") } else { None }
            data-closed=move || (!open.get()).then_some("true")
            data-hovered=move || if hover.is_hovered.get() { Some("true") } else { None }
            data-pressed=move || if aria.is_pressed.get() { Some("true") } else { None }
            data-disabled=is_disabled.then_some("true")
            role=aria.attrs.role
            tabindex=aria.attrs.tabindex
            aria-disabled=aria.attrs.aria_disabled
            on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())
            on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())
            on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())
            on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
            on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
            on:click=move |_| aria.handlers.press.on_click.run(())
            on:keydown=move |ev| {
                let key = ev.key();
                if aria.handlers.press.on_key_down.run(key) {
                    ev.prevent_default();
                }
            }
            on:keyup=move |ev| {
                let key = ev.key();
                if aria.handlers.press.on_key_up.run(key) {
                    ev.prevent_default();
                }
            }
            on:focus=move |_| focus_ring.handlers.on_focus.run(())
            on:blur=move |_| {
                aria.handlers.press.on_blur.run(());
                focus_ring.handlers.on_blur.run(());
            }
        >
            <span class="ui-disclosure__label ui-collapsible__label" data-slot=SLOT_COLLAPSIBLE_LABEL>
                {title.get_value()}
            </span>
            <span
                node_ref=indicator_ref
                class="ui-disclosure__indicator ui-collapsible__indicator"
                aria-hidden=ARIA_HIDDEN_TRUE
                data-slot=SLOT_COLLAPSIBLE_INDICATOR
            >
                {COLLAPSIBLE_INDICATOR_GLYPH}
            </span>
        </button>
    }
}

struct PanelViewModel {
    panel_id: String,
    trigger_id: String,
    panel_ref: NodeRef<html::Div>,
    panel_surface_ref: NodeRef<html::Div>,
    panel_hidden: RwSignal<bool>,
    open: Signal<bool>,
    state: Memo<logic::CollapsibleState>,
    locale_lang: StoredValue<Option<String>>,
    locale_dir: StoredValue<Option<&'static str>>,
    children: Children,
}

fn render_panel(input: PanelViewModel) -> impl IntoView {
    let PanelViewModel {
        panel_id,
        trigger_id,
        panel_ref,
        panel_surface_ref,
        panel_hidden,
        open,
        state,
        locale_lang,
        locale_dir,
        children,
    } = input;

    view! {
        <div
            id=panel_id
            class="ui-disclosure__panel ui-collapsible__panel"
            node_ref=panel_ref
            role="region"
            aria-labelledby=trigger_id
            lang=move || locale_lang.get_value()
            dir=move || locale_dir.get_value()
            hidden=move || panel_hidden.get()
            data-open=move || if open.get() { Some("true") } else { None }
            data-closed=move || (!open.get()).then_some("true")
            data-state=move || state.get().state_attr
            data-open-mode=move || state.get().open_mode_attr
            data-motion-source=move || state.get().motion_source_attr
            data-open-value-source=move || state.get().open_value_source_attr
            data-open-change-source=move || state.get().open_change_source_attr
            data-custom-motion=move || state.get().motion_source.is_custom().then_some("true")
            data-slot=SLOT_COLLAPSIBLE_PANEL
        >
            <div
                class="ui-disclosure__panel-surface ui-collapsible__panel-surface"
                node_ref=panel_surface_ref
                data-slot=SLOT_COLLAPSIBLE_PANEL_SURFACE
            >
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn Collapsible(
    id_base: String,
    title: String,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: CollapsibleMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional, into)] dir: Option<String>,
    children: Children,
) -> impl IntoView {
    let open_prop = open;
    let motion = crate::motion::sanitize_motion(motion);
    let id_base = logic::normalize_id_base(id_base);
    let title = logic::resolve_title(title);
    let (aria_label, has_custom_aria_label) = logic::resolve_aria_label(&title, aria_label);
    let normalized_class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = normalized_class_name.is_some();
    let normalized_class_name = StoredValue::new(normalized_class_name);
    let is_disabled = logic::normalize_is_disabled(is_disabled, disabled);
    let dir = logic::normalize_dir(dir);
    let locale = locale_attrs(lang, dir);

    let has_custom_motion = motion != CollapsibleMotion::default();

    let ids = DisclosureIds::new(&id_base);
    let trigger_id = ids.trigger_id.clone();
    let panel_id = ids.panel_id.clone();

    let open_input = open_prop.map(|value| value.get_untracked());
    let open_state_options = logic::normalize_open_state_options(open_input, default_open);
    let open_value_source = logic::normalize_open_value_source(open_input, default_open);
    let open_state = RwSignal::new(logic::use_collapsible_open_state(open_state_options));
    let open_change_source = RwSignal::new(logic::CollapsibleOpenChangeSource::Initial);
    let last_controlled_open = RwSignal::new(open_input);
    Effect::new(move |_| {
        open_state.update(|state| state.sync_controlled(open_prop.map(|value| value.get())));
        let controlled_open = open_prop.map(|value| value.get());

        if controlled_open.is_some() && controlled_open != last_controlled_open.get_untracked() {
            open_change_source.set(logic::normalize_open_change_source(false));
        }
        last_controlled_open.set(controlled_open);
    });
    let open = Signal::derive(move || open_state.with(|state| state.is_open()));
    let trigger_a11y = disclosure_trigger_attrs(open, panel_id.clone(), locale.lang.clone(), dir);
    let locale_lang = StoredValue::new(locale.lang);
    let locale_dir = StoredValue::new(locale.dir);

    let trace = use_ui_trace();
    let request_open_change = Callback::new(move |next: bool| {
        let current = open_state.with_untracked(|state| state.is_open());
        if !logic::should_emit_open_change(current, next) {
            return;
        }

        if let Some(trace) = trace {
            trace.emit("collapsible", UiTraceEventKind::OpenChange { open: next });
        }
        if let Some(on_open_change) = on_open_change {
            on_open_change.run(next);
        }

        open_change_source.set(logic::normalize_open_change_source(true));
        open_state.update(|state| {
            logic::apply_open_change(state, open_prop.map(|value| value.get_untracked()), next);
        });
    });

    let on_press = Callback::new(move |_| {
        let next = logic::compute_next_open(open.get_untracked());
        request_open_change.run(next);
    });

    let aria = use_button(ButtonOptions {
        is_disabled,
        on_press: Some(on_press),
        ..Default::default()
    });

    let focus_ring = use_focus_ring(FocusRingOptions { is_disabled });
    let hover = use_hover(HoverOptions { is_disabled });

    let indicator_ref: NodeRef<html::Span> = NodeRef::new();
    crate::motion::attach_indicator_motion(indicator_ref, open, motion);

    let panel_ref: NodeRef<html::Div> = NodeRef::new();
    let panel_surface_ref: NodeRef<html::Div> = NodeRef::new();
    let panel_hidden = RwSignal::new(!open.get_untracked());
    crate::motion::attach_panel_motion(panel_ref, panel_surface_ref, open, panel_hidden, motion);

    let title = StoredValue::new(title);

    let state = Memo::new(move |_| {
        let status = logic::normalize_status(open.get(), is_disabled);
        let open_mode = logic::normalize_open_mode(open_state.with(|state| state.is_controlled()));
        let label_source = logic::normalize_label_source(has_custom_aria_label);
        let class_source = logic::normalize_class_source(has_custom_class_name);
        let motion_source = logic::normalize_motion_source(has_custom_motion);
        let open_change_source = open_change_source.get();

        logic::resolve_state(CollapsibleStateInput {
            status,
            open_mode,
            label_source,
            class_source,
            motion_source,
            open_value_source,
            open_change_source,
        })
    });
    let agent_contract = Signal::derive(move || {
        logic::resolve_agent_contract(logic::CollapsibleAgentContractInput {
            render_state: state.get(),
        })
    });

    let class = Memo::new(move |_| {
        logic::compose_class_name(normalized_class_name.get_value(), state.get())
    });

    let trigger = render_trigger(TriggerViewModel {
        trigger_id: trigger_id.clone(),
        title,
        indicator_ref,
        state,
        open,
        trigger_a11y,
        aria_label,
        is_disabled,
        hover,
        focus_ring,
        aria,
    });
    let panel = render_panel(PanelViewModel {
        panel_id,
        trigger_id,
        panel_ref,
        panel_surface_ref,
        panel_hidden,
        open,
        state,
        locale_lang,
        locale_dir,
        children,
    });

    view! {
        <div
            class=move || class.get()
            lang=move || locale_lang.get_value()
            dir=move || locale_dir.get_value()
            data-slot=SLOT_COLLAPSIBLE
            data-state=move || state.get().state_attr
            data-open=move || state.get().is_open.then_some("true")
            data-closed=move || state.get().is_closed.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-open-mode=move || state.get().open_mode_attr
            data-label-source=move || state.get().label_source_attr
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            data-open-value-source=move || state.get().open_value_source_attr
            data-open-change-source=move || state.get().open_change_source_attr
            data-custom-motion=move || state.get().motion_source.is_custom().then_some("true")
            data-custom-class=move || state.get().class_source.is_custom().then_some("true")
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
            data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()
            data-ui-state-source=move || agent_contract.get().state_source
            data-ui-motion-source=move || agent_contract.get().motion_source
            data-ui-open-value-source=move || agent_contract.get().open_value_source
            data-ui-open-change-source=move || agent_contract.get().open_change_source
            data-ui-config-policy=move || agent_contract.get().config_policy
        >
            {trigger}
            {panel}
        </div>
    }
}
