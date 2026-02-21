use crate::OnPress;
use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::contextual_help::{
    ContextualHelpMotion, ContextualHelpVariant,
    logic::{
        self, ContextualHelpOpenInteractionIntent, ContextualHelpOpenInteractionSource,
        ContextualHelpOpenInteractionSyncInput, ContextualHelpOpenStateInput,
        ContextualHelpStateInput,
    },
};
use crate::popover::Popover;
use leptos::{children::ViewFn, html, prelude::*};
use ui_headless as overlay_open;
use ui_headless::use_presence;
use ui_headless::{A11yDirection, PopoverPlacement, use_ui_id_provider};

const TRIGGER_ICON_VIEWBOX: &str = "0 0 20 20";
const HELP_ICON_OUTLINE_PATH: &str = "M10 17a7 7 0 1 1 0-14a7 7 0 0 1 0 14Z";
const HELP_ICON_QUERY_PATH: &str =
    "M8.8 7.7A1.7 1.7 0 0 1 10.4 6.5c1 0 1.8.8 1.8 1.8 0 .9-.6 1.4-1.2 1.8-.6.4-1 .7-1 .9v.7";
const INFO_ICON_STEM_PATH: &str = "M10 9v5";

fn render_trigger_icon(variant: ContextualHelpVariant) -> impl IntoView {
    match variant {
        ContextualHelpVariant::Help => view! {
            <path
                d=HELP_ICON_OUTLINE_PATH
                stroke="currentColor"
                stroke_width="1.5"
            />
            <path
                d=HELP_ICON_QUERY_PATH
                stroke="currentColor"
                stroke_width="1.5"
                stroke_linecap="round"
                stroke_linejoin="round"
            />
            <circle cx="10" cy="14.1" r="1" fill="currentColor" />
        }
        .into_any(),
        ContextualHelpVariant::Info => view! {
            <circle cx="10" cy="10" r="7" stroke="currentColor" stroke_width="1.5" />
            <path d=INFO_ICON_STEM_PATH stroke="currentColor" stroke_width="1.5" stroke_linecap="round" />
            <circle cx="10" cy="6" r="1" fill="currentColor" />
        }
        .into_any(),
    }
}

#[component]
pub fn ContextualHelp(
    children: ChildrenFn,
    #[prop(optional)] variant: ContextualHelpVariant,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: Option<bool>,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] motion: ContextualHelpMotion,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional, into)] heading: Option<String>,
    #[prop(optional, into)] footer: Option<ViewFn>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let open_state_config = logic::resolve_open_state_config(ContextualHelpOpenStateInput {
        open,
        default_open,
        on_open_change,
    });
    let has_custom_open = open_state_config.has_custom_open;
    let has_custom_default_open = open_state_config.has_custom_default_open;
    let has_custom_on_open_change = open_state_config.has_custom_on_open_change;
    let is_controlled = open_state_config.is_controlled;
    let open_state = overlay_open::use_controllable_open_state_traced(
        "contextual-help",
        open_state_config.open,
        open_state_config.default_open,
        open_state_config.on_open_change,
    );
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let presence = use_presence(open);
    let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);

    let heading = logic::normalize_optional_text(heading);
    let footer = StoredValue::new(footer);
    let class_name = logic::normalize_optional_text(class_name);
    let lang = logic::normalize_optional_text(lang);

    let motion = crate::contextual_help::motion::sanitize_motion(motion);
    let has_custom_motion = motion != ContextualHelpMotion::default();

    let (trigger_aria_label, has_custom_aria_label) =
        logic::resolve_trigger_aria_label(variant, aria_label);
    let generated_id = logic::resolve_generated_id(
        use_ui_id_provider().map(|provider| provider.next_prefixed_id("ui-contextual-help")),
    );
    let (id, has_custom_id) = logic::resolve_id(id, generated_id);

    let state = logic::resolve_state(ContextualHelpStateInput {
        variant,
        placement,
        is_disabled,
        has_custom_open,
        has_custom_default_open,
        has_custom_on_open_change,
        has_heading: heading.is_some(),
        has_footer: footer.get_value().is_some(),
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
        has_custom_id,
        has_custom_motion,
        is_controlled,
    });
    let class = StoredValue::new(logic::compose_class_name(class_name, state));

    let children = StoredValue::new(children);
    let heading = StoredValue::new(heading);

    let panel_id = StoredValue::new(format!("{id}-panel"));
    let heading_id = StoredValue::new(format!("{id}-heading"));
    let content_id = StoredValue::new(format!("{id}-content"));
    let aria_controls = ui_headless::aria_controls_when_open(open, panel_id.get_value());

    let panel_aria_label =
        StoredValue::new((!state.has_heading).then(|| trigger_aria_label.clone()));
    let panel_a11y = ui_headless::overlay_dialog_attrs(
        state.has_heading.then(|| heading_id.get_value()),
        Some(content_id.get_value()),
        lang.clone(),
        dir,
    );
    let panel_aria_labelledby = StoredValue::new(panel_a11y.aria_labelledby);
    let panel_aria_describedby = StoredValue::new(panel_a11y.aria_describedby);
    let panel_lang = StoredValue::new(panel_a11y.lang);
    let panel_dir = panel_a11y.dir;

    let open_interaction_source = RwSignal::new(ContextualHelpOpenInteractionSource::Initial);
    let has_pending_local_open_change = RwSignal::new(false);
    let previous_open = RwSignal::new(open.get_untracked());
    let trace = ui_headless::use_ui_trace();
    Effect::new(move |_| {
        let previous_open_value = previous_open.get_untracked();
        let current_open_value = open.get();
        let sync = logic::resolve_open_interaction_sync(ContextualHelpOpenInteractionSyncInput {
            previous_open: previous_open_value,
            current_open: current_open_value,
            current_source: open_interaction_source.get_untracked(),
            has_pending_local_open_change: has_pending_local_open_change.get_untracked(),
        });
        if previous_open_value != current_open_value
            && let Some(trace) = trace
            && trace.enabled()
        {
            trace.emit(
                "contextual-help",
                ui_headless::UiTraceEventKind::Note {
                    message: format!(
                        "open:{}->{} source={}",
                        previous_open_value,
                        current_open_value,
                        sync.next_source.as_attr()
                    ),
                },
            );
        }
        previous_open.set(sync.next_previous_open);
        open_interaction_source.set(sync.next_source);
        has_pending_local_open_change.set(sync.has_pending_local_open_change);
    });

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();
    let on_trigger_press: OnPress = {
        let request_open_change_for_trigger = request_open_change;
        let open_interaction_source_for_trigger = open_interaction_source;
        let has_pending_local_open_change_for_trigger = has_pending_local_open_change;
        Callback::new(move |_| {
            let intent = logic::resolve_open_interaction_intent(
                ContextualHelpOpenInteractionIntent::TriggerPress,
            );
            has_pending_local_open_change_for_trigger.set(intent.has_pending_local_open_change);
            open_interaction_source_for_trigger.set(intent.next_source);
            request_open_change_for_trigger.run(!open.get_untracked());
        })
    };
    let on_close: OnPress = {
        let request_open_change_for_close = request_open_change;
        let open_interaction_source_for_close = open_interaction_source;
        let has_pending_local_open_change_for_close = has_pending_local_open_change;
        Callback::new(move |_| {
            let intent = logic::resolve_open_interaction_intent(
                ContextualHelpOpenInteractionIntent::DismissPress,
            );
            has_pending_local_open_change_for_close.set(intent.has_pending_local_open_change);
            open_interaction_source_for_close.set(intent.next_source);
            request_open_change_for_close.run(false);
        })
    };
    let streaming_policy = logic::resolve_streaming_policy(false);
    let llm_output_mode = logic::resolve_llm_output_mode(false);
    let llm_output_status = logic::resolve_llm_output_status(llm_output_mode);
    let agent_contract = move || {
        logic::resolve_agent_contract(state.variant, open_interaction_source.get(), open.get())
    };

    let trigger_view = move || {
        view! {
            <Button
                node_ref=anchor_ref
                aria_label=trigger_aria_label
                variant=ButtonVariant::Ghost
                size=ButtonSize::IconSm
                is_disabled=is_disabled
                aria_haspopup="dialog"
                aria_expanded=open
                aria_controls_signal=aria_controls
                class_name="ui-contextual-help__trigger".to_string()
                on_press=on_trigger_press
            >
                <svg viewBox=TRIGGER_ICON_VIEWBOX fill="none" aria-hidden="true">
                    {render_trigger_icon(state.variant)}
                </svg>
            </Button>
        }
    };

    let panel_view = move || {
        let children = children.get_value();
        view! {
            <div
                class="ui-contextual-help__panel"
                id=panel_id.get_value()
                role="dialog"
                aria-modal="false"
                aria-label=panel_aria_label.get_value()
                aria-labelledby=panel_aria_labelledby.get_value()
                aria-describedby=panel_aria_describedby.get_value()
                lang=panel_lang.get_value()
                dir=panel_dir
                data-slot="contextual-help-panel"
                data-open=move || open.get().then_some("true")
                data-closed=move || (!open.get()).then_some("true")
                data-variant=state.variant_attr
                data-placement=state.placement_attr
                data-heading=state.heading_attr
                data-footer=state.footer_attr
                data-open-mode=state.open_mode_attr
                data-open-source=state.open_source_attr
                data-default-open-source=state.default_open_source_attr
                data-open-change-source=state.open_change_source_attr
                data-open-interaction-source=move || open_interaction_source.get().as_attr()
                data-ui-schema=move || agent_contract().schema
                data-ui-intent=move || agent_contract().intent
                data-ui-action=move || agent_contract().action
                data-ui-state=move || agent_contract().state
                data-ui-source=move || agent_contract().source
                data-ui-output-status=llm_output_status.as_attr()
                data-ui-output-mode=llm_output_mode.as_attr()
                data-ui-streaming-requirement=streaming_policy.requirement.as_attr()
                data-ui-streaming-fallback=streaming_policy.fallback_mode.as_attr()
            >
                {heading.get_value().map(|heading| {
                    view! {
                        <h3
                            class="ui-contextual-help__heading"
                            id=heading_id.get_value()
                            data-slot="contextual-help-heading"
                        >
                            {heading}
                        </h3>
                    }
                })}
                <div
                    class="ui-contextual-help__content"
                    id=content_id.get_value()
                    data-slot="contextual-help-content"
                >
                    {children()}
                </div>
                {footer.get_value().map(|footer| {
                    view! {
                        <div class="ui-contextual-help__footer" data-slot="contextual-help-footer">
                            {footer.run()}
                        </div>
                    }
                })}
            </div>
        }
    };

    view! {
        <span
            class=move || class.get_value()
            data-slot="contextual-help"
            data-ui-schema=move || agent_contract().schema
            data-ui-intent=move || agent_contract().intent
            data-ui-action=move || agent_contract().action
            data-ui-state=move || agent_contract().state
            data-ui-source=move || agent_contract().source
            data-ui-output-status=llm_output_status.as_attr()
            data-ui-output-mode=llm_output_mode.as_attr()
            data-ui-streaming-requirement=streaming_policy.requirement.as_attr()
            data-ui-streaming-fallback=streaming_policy.fallback_mode.as_attr()
            data-state=state.state_attr
            data-variant=state.variant_attr
            data-placement=state.placement_attr
            data-heading=state.heading_attr
            data-footer=state.footer_attr
            data-open-mode=state.open_mode_attr
            data-open=move || open.get().then_some("true")
            data-closed=move || (!open.get()).then_some("true")
            data-open-source=state.open_source_attr
            data-default-open-source=state.default_open_source_attr
            data-open-change-source=state.open_change_source_attr
            data-open-interaction-source=move || open_interaction_source.get().as_attr()
            data-label-source=state.label_source_attr
            data-id-source=state.id_source_attr
            data-disabled=state.is_disabled.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-class-source=state.class_source_attr
            data-motion-source=state.motion_source_attr
            data-custom-motion=state.has_custom_motion.then_some("true")
            data-custom-open=has_custom_open.then_some("true")
            data-custom-default-open=has_custom_default_open.then_some("true")
            data-custom-open-change=has_custom_on_open_change.then_some("true")
        >
            {trigger_view()}

            <Show when=move || presence.is_present.get()>
                <Popover
                    open=open
                    anchor_ref=anchor_ref
                    on_close=on_close
                    placement=placement
                    motion=motion.popover
                    is_modal=false
                    on_exit_complete=presence.finish_exit
                >
                    {move || panel_view()}
                </Popover>
            </Show>
        </span>
    }
}
