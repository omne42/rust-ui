use crate::drawer::{DrawerMotion, DrawerPartStateInput, DrawerPlacement, DrawerSlot, logic};
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
    let has_custom_id_base = !id_base.trim().is_empty();
    let has_custom_title = !title.trim().is_empty();

    let id_base = logic::normalize_id_base(id_base);
    let title = logic::normalize_required_text(title, logic::DEFAULT_TITLE);
    let description = logic::normalize_optional_text(description);
    let class_name = logic::normalize_optional_text(class_name);

    let title = StoredValue::new(title);
    let description = StoredValue::new(description);
    let footer = StoredValue::new(footer);
    let children = StoredValue::new(children);
    let motion = crate::drawer::motion::sanitize_motion(motion);

    let has_custom_description = description.get_value().is_some();
    let has_footer = footer.get_value().is_some();
    let has_custom_class_name = class_name.is_some();
    let has_custom_motion = motion != DrawerMotion::default();
    let has_on_exit_complete = on_exit_complete.is_some();

    let root_state = logic::resolve_state(DrawerPartStateInput {
        slot: DrawerSlot::Root,
        placement,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name,
        has_custom_motion,
        has_on_exit_complete,
    });
    let root_class = logic::compose_class_name(class_name, root_state);
    let root_class = StoredValue::new(root_class);

    let header_state = logic::resolve_state(DrawerPartStateInput {
        slot: DrawerSlot::Header,
        placement,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let header_class = logic::compose_class_name(None, header_state);
    let header_class = StoredValue::new(header_class);

    let title_state = logic::resolve_state(DrawerPartStateInput {
        slot: DrawerSlot::Title,
        placement,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let title_class = logic::compose_class_name(None, title_state);
    let title_class = StoredValue::new(title_class);

    let description_state = logic::resolve_state(DrawerPartStateInput {
        slot: DrawerSlot::Description,
        placement,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let description_class = logic::compose_class_name(None, description_state);
    let description_class = StoredValue::new(description_class);

    let body_state = logic::resolve_state(DrawerPartStateInput {
        slot: DrawerSlot::Body,
        placement,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let body_class = logic::compose_class_name(None, body_state);
    let body_class = StoredValue::new(body_class);

    let footer_state = logic::resolve_state(DrawerPartStateInput {
        slot: DrawerSlot::Footer,
        placement,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let footer_class = logic::compose_class_name(None, footer_state);
    let footer_class = StoredValue::new(footer_class);

    let close_state = logic::resolve_state(DrawerPartStateInput {
        slot: DrawerSlot::Close,
        placement,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let close_class = logic::compose_class_name(None, close_state);
    let close_class = StoredValue::new(close_class);

    let title_id = format!("{id_base}-title");
    let description_id = format!("{id_base}-description");
    let title_id_attr: Signal<String> = title_id.clone().into();
    let description_id_attr: Signal<String> = description_id.clone().into();

    let on_exit_complete = on_exit_complete.unwrap_or_else(|| Callback::new(|_| {}));

    if root_state.show_description {
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
                    let children = children.get_value();
                    view! {
                        <div
                            class=move || root_class.with_value(|class_name| class_name.clone())
                            data-slot=root_state.slot_attr
                            data-state=root_state.state_attr
                            data-open=move || open.get().then_some("true")
                            data-closed=move || (!open.get()).then_some("true")
                            data-placement=root_state.placement_attr
                            data-description=root_state.description_attr
                            data-footer=root_state.footer_attr
                            data-close-button=root_state.close_button_attr
                            data-with-description=root_state.show_description.then_some("true")
                            data-with-footer=root_state.show_footer.then_some("true")
                            data-close-visible=root_state.show_close_button.then_some("true")
                            data-custom-id=root_state.has_custom_id_base.then_some("true")
                            data-custom-title=root_state.has_custom_title.then_some("true")
                            data-custom-description=root_state.has_custom_description.then_some("true")
                            data-custom-footer=(root_state.footer_source_attr == "custom").then_some("true")
                            data-custom-close=(root_state.close_source_attr == "custom").then_some("true")
                            data-custom-class=root_state.has_custom_class_name.then_some("true")
                            data-custom-motion=root_state.has_custom_motion.then_some("true")
                            data-custom-exit=root_state.has_on_exit_complete.then_some("true")
                            data-placement-source=root_state.placement_source_attr
                            data-description-source=root_state.description_source_attr
                            data-footer-source=root_state.footer_source_attr
                            data-close-source=root_state.close_source_attr
                            data-id-source=root_state.id_source_attr
                            data-title-source=root_state.title_source_attr
                            data-class-source=root_state.class_source_attr
                            data-motion-source=root_state.motion_source_attr
                            data-exit-source=root_state.exit_source_attr
                        >
                            <Show when=move || root_state.show_close_button>
                                <span
                                    class=move || {
                                        close_class.with_value(|class_name| class_name.clone())
                                    }
                                    data-slot=close_state.slot_attr
                                    data-state=close_state.state_attr
                                    data-close-source=close_state.close_source_attr
                                >
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

                            <div
                                class=move || header_class.with_value(|class_name| class_name.clone())
                                data-slot=header_state.slot_attr
                                data-state=header_state.state_attr
                            >
                                <h2
                                    class=move || title_class.with_value(|class_name| class_name.clone())
                                    id=move || title_id_attr.get()
                                    data-slot=title_state.slot_attr
                                    data-state=title_state.state_attr
                                    data-title-source=title_state.title_source_attr
                                >
                                    {move || title.get_value()}
                                </h2>
                                <p
                                    class=move || {
                                        description_class
                                            .with_value(|class_name| class_name.clone())
                                    }
                                    id=move || description_id_attr.get()
                                    data-slot=description_state.slot_attr
                                    data-state=description_state.state_attr
                                    data-description-source=description_state.description_source_attr
                                >
                                    {move || {
                                        description.get_value().unwrap_or_default()
                                    }}
                                </p>
                            </div>

                            <div
                                class=move || body_class.with_value(|class_name| class_name.clone())
                                data-slot=body_state.slot_attr
                                data-state=body_state.state_attr
                            >
                                {children()}
                            </div>

                            <Show when=move || root_state.show_footer>
                                <div
                                    class=move || {
                                        footer_class.with_value(|class_name| class_name.clone())
                                    }
                                    data-slot=footer_state.slot_attr
                                    data-state=footer_state.state_attr
                                    data-footer-source=footer_state.footer_source_attr
                                >
                                    {move || footer.get_value().map(|slot| slot.run())}
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Sheet>
        }
        .into_any()
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
                    let children = children.get_value();
                    view! {
                        <div
                            class=move || root_class.with_value(|class_name| class_name.clone())
                            data-slot=root_state.slot_attr
                            data-state=root_state.state_attr
                            data-open=move || open.get().then_some("true")
                            data-closed=move || (!open.get()).then_some("true")
                            data-placement=root_state.placement_attr
                            data-description=root_state.description_attr
                            data-footer=root_state.footer_attr
                            data-close-button=root_state.close_button_attr
                            data-with-description=root_state.show_description.then_some("true")
                            data-with-footer=root_state.show_footer.then_some("true")
                            data-close-visible=root_state.show_close_button.then_some("true")
                            data-custom-id=root_state.has_custom_id_base.then_some("true")
                            data-custom-title=root_state.has_custom_title.then_some("true")
                            data-custom-description=root_state.has_custom_description.then_some("true")
                            data-custom-footer=(root_state.footer_source_attr == "custom").then_some("true")
                            data-custom-close=(root_state.close_source_attr == "custom").then_some("true")
                            data-custom-class=root_state.has_custom_class_name.then_some("true")
                            data-custom-motion=root_state.has_custom_motion.then_some("true")
                            data-custom-exit=root_state.has_on_exit_complete.then_some("true")
                            data-placement-source=root_state.placement_source_attr
                            data-description-source=root_state.description_source_attr
                            data-footer-source=root_state.footer_source_attr
                            data-close-source=root_state.close_source_attr
                            data-id-source=root_state.id_source_attr
                            data-title-source=root_state.title_source_attr
                            data-class-source=root_state.class_source_attr
                            data-motion-source=root_state.motion_source_attr
                            data-exit-source=root_state.exit_source_attr
                        >
                            <Show when=move || root_state.show_close_button>
                                <span
                                    class=move || {
                                        close_class.with_value(|class_name| class_name.clone())
                                    }
                                    data-slot=close_state.slot_attr
                                    data-state=close_state.state_attr
                                    data-close-source=close_state.close_source_attr
                                >
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

                            <div
                                class=move || header_class.with_value(|class_name| class_name.clone())
                                data-slot=header_state.slot_attr
                                data-state=header_state.state_attr
                            >
                                <h2
                                    class=move || title_class.with_value(|class_name| class_name.clone())
                                    id=move || title_id_attr.get()
                                    data-slot=title_state.slot_attr
                                    data-state=title_state.state_attr
                                    data-title-source=title_state.title_source_attr
                                >
                                    {move || title.get_value()}
                                </h2>
                            </div>

                            <div
                                class=move || body_class.with_value(|class_name| class_name.clone())
                                data-slot=body_state.slot_attr
                                data-state=body_state.state_attr
                            >
                                {children()}
                            </div>

                            <Show when=move || root_state.show_footer>
                                <div
                                    class=move || {
                                        footer_class.with_value(|class_name| class_name.clone())
                                    }
                                    data-slot=footer_state.slot_attr
                                    data-state=footer_state.state_attr
                                    data-footer-source=footer_state.footer_source_attr
                                >
                                    {move || footer.get_value().map(|slot| slot.run())}
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Sheet>
        }
        .into_any()
    }
}
