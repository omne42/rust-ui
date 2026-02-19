use crate::action_bar::{
    ActionBarMotion, ActionBarPosition, ActionBarStrings,
    logic::{self, resolve_selection_text},
    motion,
};
use crate::button::{Button, ButtonSize, ButtonVariant};
use leptos::{html, prelude::*};
use ui_headless::i18n;
use ui_headless::use_controllable_state;
use ui_headless::{A11yDirection, locale_attrs};

#[component]
pub fn ActionBar(
    #[prop(optional)] selected_count: Option<Signal<usize>>,
    #[prop(optional)] default_selected_count: Option<usize>,
    #[prop(optional)] on_selected_count_change: Option<Callback<usize>>,
    #[prop(optional)] on_clear_selection: Option<Callback<()>>,
    #[prop(optional)] position: ActionBarPosition,
    #[prop(optional)] is_force_visible: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] clear_label: Option<String>,
    #[prop(optional, into)] selection_text: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: ActionBarMotion,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<ActionBarStrings>();
    let is_controlled_selected_count = selected_count.is_some();
    let has_default_selected_count = default_selected_count.is_some();
    let has_selected_count_change_handler = on_selected_count_change.is_some();
    let default_selected_count = logic::normalize_default_selected_count(default_selected_count);
    let selected_count_state = use_controllable_state(
        selected_count,
        Some(default_selected_count),
        on_selected_count_change,
    );
    let selected_count = selected_count_state.value;
    let request_selected_count_change = selected_count_state.request_change;

    let (aria_label, has_custom_label) =
        logic::normalize_aria_label(aria_label, strings.aria_label.as_ref());
    let (clear_label, has_custom_clear_label) =
        logic::normalize_clear_label(clear_label, strings.clear_label.as_ref());
    let (selection_text, has_custom_selection_text) =
        logic::normalize_selection_text(selection_text);
    let locale = locale_attrs(lang, dir);

    let class_name = logic::normalize_optional_text(class_name);
    let class_name = StoredValue::new(class_name);

    let motion = motion::sanitize_motion(motion);

    let has_clear_action = on_clear_selection.is_some();
    let on_clear_selection = StoredValue::new(on_clear_selection);

    let strings = StoredValue::new(strings);

    let state = Signal::derive(move || {
        logic::resolve_view_state(logic::ActionBarViewStateInput {
            selected_count: selected_count.get(),
            position,
            is_force_visible,
            is_controlled_selected_count,
            has_default_selected_count,
            has_selected_count_change_handler,
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
            data-controlled=move || state.get().is_controlled_selected_count.then_some("true")
            data-uncontrolled=move || state.get().is_uncontrolled_selected_count.then_some("true")
            data-control-mode=move || state.get().control_mode_attr
            data-selected-count-source=move || state.get().selected_count_source_attr
            data-default-selected-count-source=move || state.get().default_selected_count_source_attr
            data-selected-count-change-source=move || state.get().selected_count_change_source_attr
            data-has-clear=move || state.get().has_clear_action.then_some("true")
            data-clear-action-source=move || state.get().clear_action_source_attr
            data-label-source=move || state.get().label_source_attr
            data-selection-source=move || state.get().selection_source_attr
            data-clear-label-source=move || state.get().clear_label_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            data-motion-source=move || state.get().motion_source_attr
            role="toolbar"
            aria-label=aria_label
            aria-hidden=move || state.get().is_hidden.then_some("true")
            lang=locale.lang.clone()
            dir=locale.dir
        >
            <p class="ui-action-bar__selection" data-slot="action-bar-selection">
                <span class="ui-action-bar__selection-count" data-slot="action-bar-selection-count">
                    {move || state.get().selected_count}
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
                    let on_press = Callback::new(move |_| {
                        request_selected_count_change.run(0);
                        on_clear_selection.run(());
                    });
                    view! {
                        <span data-slot="action-bar-clear">
                            <Button
                                variant=ButtonVariant::Link
                                size=ButtonSize::S
                                class_name="ui-action-bar__clear".to_string()
                                aria_label=clear_label_attr
                                on_press=on_press
                            >
                                {clear_label_text}
                            </Button>
                        </span>
                    }
                })}
            </div>
        </div>
    }
}
