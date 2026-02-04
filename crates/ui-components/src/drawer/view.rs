use crate::drawer::{DrawerMotion, DrawerPlacement, logic};
use crate::sheet::Sheet;
use crate::{ButtonSize, ButtonVariant, IconButton, OnPress};
use leptos::children::ViewFn;
use leptos::prelude::*;

#[component]
pub fn Drawer(
    open: Signal<bool>,
    on_close: OnPress,
    id_base: String,
    title: String,
    children: ChildrenFn,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] footer: Option<ViewFn>,
    #[prop(optional, default = DrawerPlacement::Right)] placement: DrawerPlacement,
    #[prop(optional)] motion: DrawerMotion,
    #[prop(optional, default = true)] show_close_button: bool,
    #[prop(optional, default = "Close")] close_label: &'static str,
    /// Called after the close animation finishes (useful for presence/unmount).
    #[prop(optional)]
    on_exit_complete: Option<Callback<()>>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let title = StoredValue::new(title);
    let description = StoredValue::new(description);
    let footer = StoredValue::new(footer);
    let children = StoredValue::new(children);

    let has_footer = footer.get_value().is_some();
    let view_state = logic::resolve_view_state(
        description.get_value().as_deref(),
        has_footer,
        show_close_button,
    );

    let base_class = "ui-drawer".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);
    let class = StoredValue::new(class);

    let title_id = format!("{id_base}-title");
    let description_id = format!("{id_base}-description");
    let title_id_attr: Signal<String> = title_id.clone().into();
    let description_id_attr: Signal<String> = description_id.clone().into();

    let on_exit_complete = on_exit_complete.unwrap_or_else(|| Callback::new(|_| {}));

    if view_state.show_description {
        view! {
            <Sheet
                open=open
                on_close=on_close
                placement=placement
                aria_labelledby=title_id.clone()
                aria_describedby=description_id.clone()
                motion=motion.sheet
                on_exit_complete=on_exit_complete
            >
                {move || {
                    let class = class.get_value();
                    let children = children.get_value();
                    view! {
                        <div class=class data-slot="drawer">
                            <Show when=move || view_state.show_close_button>
                                <span class="ui-drawer__close" data-slot="drawer-close">
                                    <IconButton
                                        aria_label=close_label
                                        variant=ButtonVariant::Ghost
                                        size=ButtonSize::IconSm
                                        on_press=on_close
                                    >
                                        <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                            <path
                                                d="M5 5l10 10M15 5L5 15"
                                                stroke="currentColor"
                                                stroke_width="1.5"
                                                stroke_linecap="round"
                                                stroke_linejoin="round"
                                            />
                                        </svg>
                                    </IconButton>
                                </span>
                            </Show>

                            <div class="ui-drawer__header" data-slot="drawer-header">
                                <h2 class="ui-drawer__title" id=move || title_id_attr.get() data-slot="drawer-title">
                                    {move || title.get_value()}
                                </h2>
                                <Show when=move || view_state.show_description>
                                    <p class="ui-drawer__description" id=move || description_id_attr.get() data-slot="drawer-description">
                                        {move || description.get_value().unwrap_or_default()}
                                    </p>
                                </Show>
                            </div>

                            <div class="ui-drawer__body" data-slot="drawer-body">
                                {children()}
                            </div>

                            <Show when=move || view_state.show_footer>
                                <div class="ui-drawer__footer" data-slot="drawer-footer">
                                    {move || footer.get_value().map(|slot| slot.run())}
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Sheet>
        }
    } else {
        view! {
            <Sheet
                open=open
                on_close=on_close
                placement=placement
                aria_labelledby=title_id.clone()
                motion=motion.sheet
                on_exit_complete=on_exit_complete
            >
                {move || {
                    let class = class.get_value();
                    let children = children.get_value();
                    view! {
                        <div class=class data-slot="drawer">
                            <Show when=move || view_state.show_close_button>
                                <span class="ui-drawer__close" data-slot="drawer-close">
                                    <IconButton
                                        aria_label=close_label
                                        variant=ButtonVariant::Ghost
                                        size=ButtonSize::IconSm
                                        on_press=on_close
                                    >
                                        <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                            <path
                                                d="M5 5l10 10M15 5L5 15"
                                                stroke="currentColor"
                                                stroke_width="1.5"
                                                stroke_linecap="round"
                                                stroke_linejoin="round"
                                            />
                                        </svg>
                                    </IconButton>
                                </span>
                            </Show>

                            <div class="ui-drawer__header" data-slot="drawer-header">
                                <h2 class="ui-drawer__title" id=move || title_id_attr.get() data-slot="drawer-title">
                                    {move || title.get_value()}
                                </h2>
                                <Show when=move || view_state.show_description>
                                    <p class="ui-drawer__description" id=move || description_id_attr.get() data-slot="drawer-description">
                                        {move || description.get_value().unwrap_or_default()}
                                    </p>
                                </Show>
                            </div>

                            <div class="ui-drawer__body" data-slot="drawer-body">
                                {children()}
                            </div>

                            <Show when=move || view_state.show_footer>
                                <div class="ui-drawer__footer" data-slot="drawer-footer">
                                    {move || footer.get_value().map(|slot| slot.run())}
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Sheet>
        }
    }
}
