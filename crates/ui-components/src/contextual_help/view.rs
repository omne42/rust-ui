use crate::contextual_help::{ContextualHelpMotion, ContextualHelpVariant, logic};
use crate::presence::use_presence;
use crate::{Button, ButtonSize, ButtonVariant, OnPress, Popover};
use leptos::{html, prelude::*};
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
    #[prop(optional, into)] heading: Option<String>,
    #[prop(optional, into)] footer: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let (is_open, set_open) = signal(false);
    let presence = use_presence(is_open.into());

    let children = StoredValue::new(children);

    let id = format!("ui-contextual-help-{}", next_id());
    let panel_id = StoredValue::new(format!("{id}-panel"));

    let anchor_ref: NodeRef<html::Button> = NodeRef::new();
    let on_trigger_press: OnPress = Callback::new(move |_| set_open.update(|v| *v = !*v));
    let on_close: OnPress = Callback::new(move |_| set_open.set(false));

    let aria_label = logic::resolve_trigger_aria_label(variant, aria_label.as_deref());

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
                aria_label=aria_label
                variant=ButtonVariant::Ghost
                size=ButtonSize::IconSm
                disabled=disabled
                aria_haspopup="dialog"
                aria_expanded=is_open.into()
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
                    open=is_open.into()
                    anchor_ref=anchor_ref
                    on_close=on_close
                    placement=placement
                    motion=motion.popover
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
                                data-slot="contextual-help-panel"
                            >
                                {heading.get_value().map(|heading| {
                                    view! {
                                        <div class="ui-contextual-help__heading" data-slot="contextual-help-heading">
                                            {heading}
                                        </div>
                                    }
                                })}
                                <div class="ui-contextual-help__content" data-slot="contextual-help-content">
                                    {children()}
                                </div>
                                {footer.get_value().map(|footer| {
                                    view! {
                                        <div class="ui-contextual-help__footer" data-slot="contextual-help-footer">
                                            {footer}
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
