use crate::contextual_help::{ContextualHelpMotion, ContextualHelpVariant, logic};
use crate::presence::use_presence;
use crate::{Button, ButtonSize, ButtonVariant, OnPress, Popover};
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
    let (uncontrolled_open, set_uncontrolled_open) = signal(default_open.unwrap_or(false));
    let is_controlled = open.is_some();
    let open = open.unwrap_or(uncontrolled_open.into());

    let on_open_change = on_open_change.unwrap_or_else(|| Callback::new(|_| {}));
    let request_open_change: Callback<bool> = Callback::new(move |next_open: bool| {
        if next_open == open.get_untracked() {
            return;
        }
        on_open_change.run(next_open);
        if !is_controlled {
            set_uncontrolled_open.set(next_open);
        }
    });

    let presence = use_presence(open);

    let children = StoredValue::new(children);

    let id = id.unwrap_or_else(|| format!("ui-contextual-help-{}", next_id()));
    let panel_id = StoredValue::new(format!("{id}-panel"));
    let heading_id = StoredValue::new(format!("{id}-heading"));
    let content_id = StoredValue::new(format!("{id}-content"));

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();
    let on_trigger_press: OnPress = {
        let request_open_change_for_trigger = request_open_change;
        Callback::new(move |_| request_open_change_for_trigger.run(!open.get_untracked()))
    };
    let on_close: OnPress = {
        let request_open_change_for_close = request_open_change;
        Callback::new(move |_| request_open_change_for_close.run(false))
    };

    let heading = logic::normalize_optional_text(heading.as_deref());
    let has_heading = heading.is_some();

    let trigger_aria_label = logic::resolve_trigger_aria_label(variant, aria_label.as_deref());
    let panel_aria_label = StoredValue::new((!has_heading).then(|| trigger_aria_label.clone()));
    let panel_aria_labelledby = StoredValue::new(has_heading.then(|| heading_id.get_value()));
    let panel_aria_describedby = StoredValue::new(content_id.get_value());

    let base_class = format!("ui-contextual-help {}", variant.class_name());
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    let heading = StoredValue::new(heading);
    let footer = StoredValue::new(footer);

    view! {
        <span class=class data-slot="contextual-help">
            <Button
                node_ref=anchor_ref
                aria_label=trigger_aria_label
                variant=ButtonVariant::Ghost
                size=ButtonSize::IconSm
                disabled=disabled
                aria_haspopup="dialog"
                aria_expanded=open
                aria_controls=panel_id.get_value()
                class_name="ui-contextual-help__trigger".to_string()
                on_press=on_trigger_press
            >
                <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                    {match variant {
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
                        }.into_any(),
                        ContextualHelpVariant::Info => view! {
                            <circle cx="10" cy="10" r="7" stroke="currentColor" stroke_width="1.5" />
                            <path d="M10 9v5" stroke="currentColor" stroke_width="1.5" stroke_linecap="round" />
                            <circle cx="10" cy="6" r="1" fill="currentColor" />
                        }.into_any(),
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
