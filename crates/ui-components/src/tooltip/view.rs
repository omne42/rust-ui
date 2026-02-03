use crate::tooltip::{TooltipMotion, motion};
use leptos::{children::ViewFn, html, prelude::*};
use ui_headless::{FocusWithinOptions, HoverOptions, use_focus_within, use_hover};

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
pub fn Tooltip(
    #[prop(into)] content: ViewFn,
    children: Children,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] motion: TooltipMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    let hover = use_hover(HoverOptions {
        is_disabled: disabled,
    });
    let focus_within = use_focus_within(FocusWithinOptions {
        is_disabled: disabled,
    });

    let id = id.unwrap_or_else(|| format!("ui-tooltip-{}", next_id()));
    let id = StoredValue::new(id);

    let open = Signal::derive(move || hover.is_hovered.get() || focus_within.is_focus_within.get());
    let presence = crate::presence::use_presence(open);

    let content = StoredValue::new(content);

    let panel_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(panel_ref, open, presence.finish_exit, motion);

    let base_class = "ui-tooltip".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <span
            class=class
            data-slot="tooltip"
        >
            <button
                type="button"
                class="ui-tooltip__trigger"
                data-slot="tooltip-trigger"
                disabled=disabled
                aria-describedby=move || {
                    presence
                        .is_present
                        .get()
                        .then(|| id.with_value(|id| id.clone()))
                }
                on:pointerenter=move |_| hover.handlers.on_pointer_enter.run(())
                on:pointerleave=move |_| hover.handlers.on_pointer_leave.run(())
                on:focusin=move |_| focus_within.handlers.on_focus_in.run(())
                on:focusout=move |_| focus_within.handlers.on_focus_out.run(())
            >
                {children()}
            </button>
            <Show when=move || presence.is_present.get()>
                <div
                    class="ui-tooltip__panel"
                    node_ref=panel_ref
                    id=move || id.with_value(|id| id.clone())
                    role="tooltip"
                    data-slot="tooltip-panel"
                >
                    {move || content.with_value(|content| content.run())}
                </div>
            </Show>
        </span>
    }
}
