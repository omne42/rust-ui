use crate::menu_item::{
    MenuItemStateInput,
    logic::{self, MenuItemSelectionIndicator},
};
use leptos::children::Children;
use leptos::prelude::*;
use ui_headless::MenuItemKind;

#[component]
pub fn MenuItem(
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional)] index: Option<usize>,
    #[prop(optional)] kind: MenuItemKind,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] focused: bool,
    #[prop(optional)] has_submenu: bool,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional)] on_pointer_move: Option<Callback<()>>,
    #[prop(optional)] on_press: Option<Callback<()>>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let on_pointer_move = on_pointer_move.unwrap_or_else(|| Callback::new(|()| {}));
    let on_press = on_press.unwrap_or_else(|| Callback::new(|()| {}));

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

    let index_text = index.map(|value| value.to_string());
    let selection_indicator = logic::resolve_selection_indicator(kind);

    let indicator_text = move || selection_indicator.marker(state.get().is_checked);

    view! {
        <div
            class=move || class.get()
            id=id
            role=move || state.get().role_attr
            tabindex=if disabled { Some(-1) } else { Some(0) }
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
                if disabled {
                    return;
                }
                on_pointer_move.run(());
            }
            on:click=move |_| {
                if disabled {
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

            <Show
                when=move || state.get().has_submenu
                fallback=move || view! {
                    <span
                        class="ui-menu-item__submenu-indicator"
                        aria-hidden="true"
                        data-slot="menu-item-submenu-indicator"
                        data-visible="false"
                    ></span>
                }
            >
                <span
                    class="ui-menu-item__submenu-indicator"
                    aria-hidden="true"
                    data-slot="menu-item-submenu-indicator"
                    data-visible="true"
                >
                    "›"
                </span>
            </Show>

            <Show when=move || selection_indicator != MenuItemSelectionIndicator::Hidden>
                <span class="ui-menu-item__selection-sr" data-slot="menu-item-selection-sr">
                    {move || if state.get().is_checked { "selected" } else { "not selected" }}
                </span>
            </Show>
        </div>
    }
}
