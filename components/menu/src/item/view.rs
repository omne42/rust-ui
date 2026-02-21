use crate::menu::item::{
    MenuItemStateInput,
    logic::{self, MenuItemSelectionIndicator},
};
use leptos::children::Children;
use leptos::prelude::*;
use ui_headless::MenuItemKind;

const SUBMENU_INDICATOR_SLOT: &str = "menu-item-submenu-indicator";
const SUBMENU_INDICATOR_MARK: &str = "›";

fn render_submenu_indicator(has_submenu: bool) -> impl IntoView {
    let data_visible = if has_submenu { "true" } else { "false" };
    let marker = has_submenu.then_some(SUBMENU_INDICATOR_MARK);

    view! {
        <span
            class="ui-menu-item__submenu-indicator"
            aria-hidden="true"
            data-slot=SUBMENU_INDICATOR_SLOT
            data-visible=data_visible
        >
            {marker}
        </span>
    }
}

#[component]
pub fn MenuItem(
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional)] index: Option<usize>,
    #[prop(optional)] kind: MenuItemKind,
    #[prop(optional)] is_disabled: Option<bool>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] focused: bool,
    #[prop(optional)] has_submenu: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] on_pointer_move: Option<Callback<()>>,
    #[prop(optional)] on_press: Option<Callback<()>>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let interaction = logic::normalize_interaction(logic::MenuItemInteractionInput {
        is_disabled,
        disabled,
        on_pointer_move,
        on_press,
    });
    let disabled = interaction.disabled;
    let on_pointer_move = interaction.on_pointer_move;
    let on_press = interaction.on_press;

    let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);
    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();
    let class_name = StoredValue::new(class_name);

    let state = Signal::derive(move || {
        logic::resolve_state(MenuItemStateInput {
            kind,
            is_checked: logic::resolve_checked(kind),
            disabled,
            focused,
            has_submenu,
            has_custom_aria_label,
            has_custom_class_name,
        })
    });

    let class =
        Signal::derive(move || logic::compose_class_name(class_name.get_value(), state.get()));

    let index_text = index;
    let selection_indicator = logic::resolve_selection_indicator(kind);

    let indicator_text = move || selection_indicator.marker(state.get().is_checked);

    view! {
        <div
            class=move || class.get()
            id=id
            role=move || state.get().role_attr
            tabindex=logic::resolve_tabindex(disabled)
            aria-label=aria_label
            aria-checked=move || logic::resolve_aria_checked(kind)
            aria-disabled=disabled.then_some("true")
            data-slot="menu-item"
            data-index=index_text
            data-kind=move || state.get().kind_attr
            data-state=move || state.get().data_state_attr
            data-checkable=move || state.get().is_checkable.then_some("true")
            data-checked=move || state.get().is_checked.then_some("true")
            data-unchecked=move || (!state.get().is_checked && state.get().is_checkable).then_some("true")
            data-focused=move || state.get().is_focused.then_some("true")
            data-disabled=move || state.get().is_disabled.then_some("true")
            data-has-submenu=move || state.get().has_submenu.then_some("true")
            data-selection-indicator=selection_indicator.as_attr()
            data-aria-source=move || state.get().aria_source_attr
            data-custom-class=move || state.get().has_custom_class_name.then_some("true")
            data-class-source=move || state.get().class_source_attr
            on:pointermove=move |_| {
                if logic::should_ignore_interaction(disabled) {
                    return;
                }
                on_pointer_move.run(());
            }
            on:click=move |_| {
                if logic::should_ignore_interaction(disabled) {
                    return;
                }
                on_press.run(());
            }
        >
            <span
                class="ui-menu-item__indicator"
                data-slot="menu-item-indicator"
                data-visible=move || {
                    indicator_text().is_some().then_some("true")
                }
            >
                {indicator_text}
            </span>

            <span class="ui-menu-item__label" data-slot="menu-item-label">
                {children()}
            </span>

            {move || render_submenu_indicator(state.get().has_submenu)}

            <Show when=move || selection_indicator != MenuItemSelectionIndicator::Hidden>
                <span class="ui-menu-item__selection-sr" data-slot="menu-item-selection-sr">
                    {move || logic::resolve_selection_sr_text(state.get().is_checked)}
                </span>
            </Show>
        </div>
    }
}
