use crate::{
    OnPress,
    button::{Button, ButtonSize, ButtonVariant},
    sheet::{Sheet, SheetPlacement},
    tray::{TrayMotion, TrayPartStateInput, TraySlot, logic},
};
use leptos::children::ViewFn;
use leptos::prelude::*;
use ui_headless::{A11yDirection, overlay_dialog_attrs};

#[component]
pub fn Tray(
    open: Signal<bool>,
    on_close: OnPress,
    id_base: String,
    title: String,
    children: ChildrenFn,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] footer: Option<ViewFn>,
    #[prop(optional)] motion: TrayMotion,
    #[prop(optional, default = logic::DEFAULT_SHOW_CLOSE_BUTTON)] show_close_button: bool,
    #[prop(optional, default = "Close tray")] close_label: &'static str,
    #[prop(optional, default = logic::DEFAULT_FIXED_HEIGHT)] is_fixed_height: bool,
    #[prop(optional, default = logic::DEFAULT_DISMISSABLE)] is_dismissable: bool,
    #[prop(optional, default = logic::DEFAULT_KEYBOARD_DISMISS_DISABLED)]
    is_keyboard_dismiss_disabled: bool,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
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
    let motion = crate::tray::motion::sanitize_motion(motion);

    let has_custom_description = description.get_value().is_some();
    let has_footer = footer.get_value().is_some();
    let has_custom_class_name = class_name.is_some();
    let has_custom_motion = motion != TrayMotion::default();
    let has_on_exit_complete = on_exit_complete.is_some();

    let root_state = logic::resolve_state(TrayPartStateInput {
        slot: TraySlot::Root,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        is_fixed_height,
        is_dismissable,
        is_keyboard_dismiss_disabled,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name,
        has_custom_motion,
        has_on_exit_complete,
    });
    let root_class = logic::compose_class_name(class_name, root_state);
    let root_class = StoredValue::new(root_class);

    let header_state = logic::resolve_state(TrayPartStateInput {
        slot: TraySlot::Header,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        is_fixed_height,
        is_dismissable,
        is_keyboard_dismiss_disabled,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let header_class = logic::compose_class_name(None, header_state);
    let header_class = StoredValue::new(header_class);

    let title_state = logic::resolve_state(TrayPartStateInput {
        slot: TraySlot::Title,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        is_fixed_height,
        is_dismissable,
        is_keyboard_dismiss_disabled,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let title_class = logic::compose_class_name(None, title_state);
    let title_class = StoredValue::new(title_class);

    let description_state = logic::resolve_state(TrayPartStateInput {
        slot: TraySlot::Description,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        is_fixed_height,
        is_dismissable,
        is_keyboard_dismiss_disabled,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let description_class = logic::compose_class_name(None, description_state);
    let description_class = StoredValue::new(description_class);

    let body_state = logic::resolve_state(TrayPartStateInput {
        slot: TraySlot::Body,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        is_fixed_height,
        is_dismissable,
        is_keyboard_dismiss_disabled,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let body_class = logic::compose_class_name(None, body_state);
    let body_class = StoredValue::new(body_class);

    let footer_state = logic::resolve_state(TrayPartStateInput {
        slot: TraySlot::Footer,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        is_fixed_height,
        is_dismissable,
        is_keyboard_dismiss_disabled,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name: false,
        has_custom_motion,
        has_on_exit_complete,
    });
    let footer_class = logic::compose_class_name(None, footer_state);
    let footer_class = StoredValue::new(footer_class);

    let close_state = logic::resolve_state(TrayPartStateInput {
        slot: TraySlot::Close,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        is_fixed_height,
        is_dismissable,
        is_keyboard_dismiss_disabled,
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
    let panel_a11y = overlay_dialog_attrs(
        Some(title_id.clone()),
        root_state
            .show_description
            .then_some(description_id.clone()),
        lang,
        dir,
    );
    let panel_aria_labelledby = StoredValue::new(panel_a11y.aria_labelledby);
    let panel_aria_describedby = StoredValue::new(panel_a11y.aria_describedby);
    let panel_lang = StoredValue::new(panel_a11y.lang);
    let panel_dir = panel_a11y.dir;

    let on_exit_complete = on_exit_complete.unwrap_or_else(|| Callback::new(|_| {}));

    if root_state.show_description {
        view! {
            <Sheet
                open=open
                on_close=on_close
                placement=SheetPlacement::Bottom
                aria_labelledby=panel_aria_labelledby.get_value().unwrap_or_default()
                aria_describedby=panel_aria_describedby.get_value().unwrap_or_default()
                is_dismissable=is_dismissable
                is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled
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
                            data-description=root_state.description_attr
                            data-footer=root_state.footer_attr
                            data-close-button=root_state.close_button_attr
                            data-size=root_state.size_attr
                            data-dismiss=root_state.dismiss_attr
                            data-keyboard-dismiss=root_state.keyboard_dismiss_attr
                            data-with-description=root_state.show_description.then_some("true")
                            data-with-footer=root_state.show_footer.then_some("true")
                            data-close-visible=root_state.show_close_button.then_some("true")
                            data-fixed-height=root_state.is_fixed_height.then_some("true")
                            data-custom-id=root_state.has_custom_id_base.then_some("true")
                            data-custom-title=root_state.has_custom_title.then_some("true")
                            data-custom-description=root_state.has_custom_description.then_some("true")
                            data-custom-footer=(root_state.footer_source_attr == "custom").then_some("true")
                            data-custom-close=(root_state.close_source_attr == "custom").then_some("true")
                            data-custom-size=(root_state.size_source_attr == "custom").then_some("true")
                            data-custom-dismiss=(root_state.dismiss_source_attr == "custom").then_some("true")
                            data-custom-keyboard-dismiss=(root_state.keyboard_dismiss_source_attr == "custom").then_some("true")
                            data-custom-class=root_state.has_custom_class_name.then_some("true")
                            data-custom-motion=root_state.has_custom_motion.then_some("true")
                            data-custom-exit=root_state.has_on_exit_complete.then_some("true")
                            data-description-source=root_state.description_source_attr
                            data-footer-source=root_state.footer_source_attr
                            data-close-source=root_state.close_source_attr
                            data-size-source=root_state.size_source_attr
                            data-dismiss-source=root_state.dismiss_source_attr
                            data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr
                            data-id-source=root_state.id_source_attr
                            data-title-source=root_state.title_source_attr
                            data-class-source=root_state.class_source_attr
                            data-motion-source=root_state.motion_source_attr
                            data-exit-source=root_state.exit_source_attr
                            lang=panel_lang.get_value()
                            dir=panel_dir
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
                                    <Button
                                        aria_label=close_label
                                        variant=ButtonVariant::Ghost
                                        size=ButtonSize::IconSm
                                        is_icon_only=true
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
                                    </Button>
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
                placement=SheetPlacement::Bottom
                aria_labelledby=panel_aria_labelledby.get_value().unwrap_or_default()
                aria_describedby=panel_aria_describedby.get_value().unwrap_or_default()
                is_dismissable=is_dismissable
                is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled
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
                            data-description=root_state.description_attr
                            data-footer=root_state.footer_attr
                            data-close-button=root_state.close_button_attr
                            data-size=root_state.size_attr
                            data-dismiss=root_state.dismiss_attr
                            data-keyboard-dismiss=root_state.keyboard_dismiss_attr
                            data-with-description=root_state.show_description.then_some("true")
                            data-with-footer=root_state.show_footer.then_some("true")
                            data-close-visible=root_state.show_close_button.then_some("true")
                            data-fixed-height=root_state.is_fixed_height.then_some("true")
                            data-custom-id=root_state.has_custom_id_base.then_some("true")
                            data-custom-title=root_state.has_custom_title.then_some("true")
                            data-custom-description=root_state.has_custom_description.then_some("true")
                            data-custom-footer=(root_state.footer_source_attr == "custom").then_some("true")
                            data-custom-close=(root_state.close_source_attr == "custom").then_some("true")
                            data-custom-size=(root_state.size_source_attr == "custom").then_some("true")
                            data-custom-dismiss=(root_state.dismiss_source_attr == "custom").then_some("true")
                            data-custom-keyboard-dismiss=(root_state.keyboard_dismiss_source_attr == "custom").then_some("true")
                            data-custom-class=root_state.has_custom_class_name.then_some("true")
                            data-custom-motion=root_state.has_custom_motion.then_some("true")
                            data-custom-exit=root_state.has_on_exit_complete.then_some("true")
                            data-description-source=root_state.description_source_attr
                            data-footer-source=root_state.footer_source_attr
                            data-close-source=root_state.close_source_attr
                            data-size-source=root_state.size_source_attr
                            data-dismiss-source=root_state.dismiss_source_attr
                            data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr
                            data-id-source=root_state.id_source_attr
                            data-title-source=root_state.title_source_attr
                            data-class-source=root_state.class_source_attr
                            data-motion-source=root_state.motion_source_attr
                            data-exit-source=root_state.exit_source_attr
                            lang=panel_lang.get_value()
                            dir=panel_dir
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
                                    <Button
                                        aria_label=close_label
                                        variant=ButtonVariant::Ghost
                                        size=ButtonSize::IconSm
                                        is_icon_only=true
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
                                    </Button>
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
