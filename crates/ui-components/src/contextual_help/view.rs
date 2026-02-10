use crate::contextual_help::{
    ContextualHelpMotion, ContextualHelpVariant,
    logic::{self, ContextualHelpStateInput},
};
use crate::presence::use_presence;
use crate::{Button, ButtonSize, ButtonVariant, OnPress, Popover, overlay_open};
use leptos::{children::ViewFn, html, prelude::*};
use ui_headless::PopoverPlacement;

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
pub fn ContextualHelp(
    children: ChildrenFn,
    #[prop(optional)] variant: ContextualHelpVariant,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] motion: ContextualHelpMotion,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional, into)] heading: Option<String>,
    #[prop(optional, into)] footer: Option<ViewFn>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    let is_controlled = open.is_some();
    let open_state = overlay_open::use_controllable_open_state(open, default_open, on_open_change);
    let open = open_state.open;
    let request_open_change = open_state.request_open_change;

    let presence = use_presence(open);

    let heading = logic::normalize_optional_text(heading);
    let footer = StoredValue::new(footer);
    let class_name = logic::normalize_optional_text(class_name);

    let (trigger_aria_label, has_custom_aria_label) =
        logic::resolve_trigger_aria_label(variant, aria_label);
    let (id, has_custom_id) = logic::resolve_id(id, format!("ui-contextual-help-{}", next_id()));

    let state = logic::resolve_state(ContextualHelpStateInput {
        variant,
        placement,
        disabled,
        has_heading: heading.is_some(),
        has_footer: footer.get_value().is_some(),
        has_custom_aria_label,
        has_custom_class_name: class_name.is_some(),
        has_custom_id,
        is_controlled,
    });
    let class = StoredValue::new(logic::compose_class_name(class_name, state));

    let children = StoredValue::new(children);
    let heading = StoredValue::new(heading);

    let panel_id = StoredValue::new(format!("{id}-panel"));
    let heading_id = StoredValue::new(format!("{id}-heading"));
    let content_id = StoredValue::new(format!("{id}-content"));
    let aria_controls = crate::a11y::aria_controls_when_open(open, panel_id.get_value());

    let panel_aria_label =
        StoredValue::new((!state.has_heading).then(|| trigger_aria_label.clone()));
    let panel_aria_labelledby = StoredValue::new(state.has_heading.then(|| heading_id.get_value()));
    let panel_aria_describedby = StoredValue::new(content_id.get_value());

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();
    let on_trigger_press: OnPress = {
        let request_open_change_for_trigger = request_open_change;
        Callback::new(move |_| request_open_change_for_trigger.run(!open.get_untracked()))
    };
    let on_close: OnPress = {
        let request_open_change_for_close = request_open_change;
        Callback::new(move |_| request_open_change_for_close.run(false))
    };

    view! {
        <span
            class=move || class.get_value()
            data-slot="contextual-help"
            data-state=state.state_attr
            data-variant=state.variant_attr
            data-placement=state.placement_attr
            data-heading=state.heading_attr
            data-footer=state.footer_attr
            data-open-mode=state.open_mode_attr
            data-label-source=state.label_source_attr
            data-id-source=state.id_source_attr
            data-disabled=state.is_disabled.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-motion-source=if motion == ContextualHelpMotion::default() {
                "default"
            } else {
                "custom"
            }
            data-custom-motion=(motion != ContextualHelpMotion::default()).then_some("true")
        >
            <Button
                node_ref=anchor_ref
                aria_label=trigger_aria_label
                variant=ButtonVariant::Ghost
                size=ButtonSize::IconSm
                disabled=disabled
                aria_haspopup="dialog"
                aria_expanded=open
                aria_controls_signal=aria_controls
                class_name="ui-contextual-help__trigger".to_string()
                on_press=on_trigger_press
            >
                <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                    {match state.variant {
                        ContextualHelpVariant::Help => view! {
                            <path
                                d="M10 17a7 7 0 1 1 0-14a7 7 0 0 1 0 14Z"
                                stroke="currentColor"
                                stroke_width="1.5"
                            />
                            <path
                                d="M8.8 7.7A1.7 1.7 0 0 1 10.4 6.5c1 0 1.8.8 1.8 1.8 0 .9-.6 1.4-1.2 1.8-.6.4-1 .7-1 .9v.7"
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
                            <path d="M10 9v5" stroke="currentColor" stroke_width="1.5" stroke_linecap="round" />
                            <circle cx="10" cy="6" r="1" fill="currentColor" />
                        }
                        .into_any(),
                    }}
                </svg>
            </Button>

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
                    {move || {
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
                                data-slot="contextual-help-panel"
                                data-variant=state.variant_attr
                                data-placement=state.placement_attr
                                data-heading=state.heading_attr
                                data-footer=state.footer_attr
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
                    }}
                </Popover>
            </Show>
        </span>
    }
}
