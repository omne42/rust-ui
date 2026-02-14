use crate::action_bar::{
    ActionBarMotion, ActionBarPosition, ActionBarStateInput, ActionBarStrings,
    logic::{self, resolve_selection_text},
    motion,
};
use leptos::{html, prelude::*};
use ui_headless::i18n;
use ui_headless::use_button;
use ui_headless::{ButtonOptions, FocusRingOptions, HoverOptions, use_focus_ring, use_hover};

#[component]
pub fn ActionBar(
    selected_count: Signal<usize>,
    #[prop(optional)] on_clear_selection: Option<Callback<()>>,
    #[prop(optional)] position: ActionBarPosition,
    #[prop(optional)] force_visible: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] clear_label: Option<String>,
    #[prop(optional, into)] selection_text: Option<String>,
    #[prop(optional)] motion: ActionBarMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<ActionBarStrings>();

    let (aria_label, has_custom_label) =
        logic::normalize_aria_label(aria_label, strings.aria_label.as_ref());
    let (clear_label, has_custom_clear_label) =
        logic::normalize_clear_label(clear_label, strings.clear_label.as_ref());
    let (selection_text, has_custom_selection_text) =
        logic::normalize_selection_text(selection_text);

    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);

    let motion = motion::sanitize_motion(motion);

    let has_clear_action = on_clear_selection.is_some();
    let on_clear_selection = StoredValue::new(on_clear_selection);

    let strings = StoredValue::new(strings);

    let state = Signal::derive(move || {
        logic::resolve_state(ActionBarStateInput {
            selected_count: selected_count.get(),
            position,
            force_visible,
            has_clear_action,
            has_custom_label,
            has_custom_class_name: class_name.get_value().is_some(),
            has_custom_selection_text,
            has_custom_clear_label,
            has_custom_motion: motion != ActionBarMotion::default(),
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));
    let selection_text = Signal::derive(move || {
        resolve_selection_text(
            state.get().selected_count,
            selection_text.clone(),
            strings.get_value().as_ref(),
        )
    });
    let visible = Signal::derive(move || state.get().is_visible);

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, visible, motion);

    view! {
        <div
            node_ref=root_ref
            class=move || class.get()
            data-slot="action-bar"
            data-state=move || state.get().phase_attr
            data-position=move || state.get().position_attr
            data-selection=move || state.get().selection_attr
            data-selected-count=move || state.get().selected_count.to_string()
            data-visible=move || state.get().is_visible.then_some("true")
            data-hidden=move || state.get().is_hidden.then_some("true")
            data-top=move || state.get().is_top.then_some("true")
            data-bottom=move || state.get().is_bottom.then_some("true")
            data-has-clear=move || state.get().has_clear_action.then_some("true")
            data-label-source=move || state.get().label_source_attr
            data-selection-source=move || state.get().selection_source_attr
            data-clear-label-source=move || state.get().clear_label_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            role="toolbar"
            aria-label=aria_label
            aria-hidden=move || state.get().is_hidden.then_some("true")
        >
            <p class="ui-action-bar__selection" data-slot="action-bar-selection">
                <span class="ui-action-bar__selection-count" data-slot="action-bar-selection-count">
                    {move || state.get().selected_count.to_string()}
                </span>
                <span class="ui-action-bar__selection-label" data-slot="action-bar-selection-label">
                    {move || selection_text.get()}
                </span>
            </p>

            <div class="ui-action-bar__actions" data-slot="action-bar-actions">
                {children.map(|children| children())}

                {on_clear_selection.get_value().map(|on_clear_selection| {
                    let clear_label_attr = clear_label.clone();
                    let clear_label_text = clear_label.clone();
                    let aria = use_button(ButtonOptions {
                        on_press: Some(on_clear_selection),
                        ..Default::default()
                    });
                    let focus_ring = use_focus_ring(FocusRingOptions::default());
                    let hover = use_hover(HoverOptions::default());
                    view! {
                        <button
                            type="button"
                            class="ui-action-bar__clear"
                            data-slot="action-bar-clear"
                            data-hovered=move || hover.is_hovered.get().then_some("true")
                            data-pressed=move || aria.is_pressed.get().then_some("true")
                            data-focus-visible=move || focus_ring.is_focus_visible.get().then_some("true")
                            aria-label=clear_label_attr
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
                            on:blur=move |_| {
                                aria.handlers.press.on_blur.run(());
                                focus_ring.handlers.on_blur.run(());
                            }
                            on:focus=move |_| focus_ring.handlers.on_focus.run(())
                        >
                            {clear_label_text}
                        </button>
                    }
                })}
            </div>
        </div>
    }
}
