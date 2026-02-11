use crate::dialog::{DialogMotion, DialogPartStateInput, DialogSize, DialogSlot, logic};
use crate::overlay::Overlay;
use crate::{ButtonSize, ButtonVariant, IconButton, OnPress};
use leptos::children::ViewFn;
use leptos::prelude::*;

#[component]
pub fn Dialog(
    open: Signal<bool>,
    on_close: OnPress,
    id_base: String,
    title: String,
    children: ChildrenFn,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] footer: Option<ViewFn>,
    #[prop(optional, default = logic::DEFAULT_SIZE)] size: DialogSize,
    #[prop(optional, default = logic::DEFAULT_SHOW_CLOSE_BUTTON)] show_close_button: bool,
    #[prop(optional, default = logic::DEFAULT_CLOSE_LABEL)] close_label: &'static str,
    #[prop(optional)] motion: DialogMotion,
    #[prop(optional)] on_exit_complete: Option<Callback<()>>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let has_custom_id_base = id_base != logic::DEFAULT_ID_BASE;

    let title = logic::normalize_required_text(title, logic::DEFAULT_TITLE);
    let has_custom_title = title != logic::DEFAULT_TITLE;
    let title = StoredValue::new(title);

    let description = logic::normalize_optional_text(description);
    let has_custom_description = description.is_some();
    let description = StoredValue::new(description);

    let footer = StoredValue::new(footer);
    let has_footer = footer.get_value().is_some();

    let close_label = if close_label.trim().is_empty() {
        logic::DEFAULT_CLOSE_LABEL
    } else {
        close_label
    };
    let has_custom_close_label = close_label != logic::DEFAULT_CLOSE_LABEL;

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    let motion = crate::dialog::motion::sanitize_motion(motion);
    let has_custom_motion = motion != DialogMotion::default();
    let has_on_exit_complete = on_exit_complete.is_some();

    let on_close = StoredValue::new(on_close);
    let on_exit_complete =
        StoredValue::new(on_exit_complete.unwrap_or_else(|| Callback::new(|_| {})));

    let root_state = logic::resolve_state(DialogPartStateInput {
        slot: DialogSlot::Root,
        size,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_close_label,
        has_custom_class_name,
        has_custom_motion,
        has_on_exit_complete,
    });
    let root_class = logic::compose_class_name(class_name, root_state);
    let root_class = StoredValue::new(root_class);

    let header_state = logic::resolve_state(DialogPartStateInput {
        slot: DialogSlot::Header,
        size,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_close_label,
        has_custom_class_name: false,
        has_custom_motion: false,
        has_on_exit_complete,
    });
    let header_class = logic::compose_class_name(None, header_state);
    let header_class = StoredValue::new(header_class);

    let title_state = logic::resolve_state(DialogPartStateInput {
        slot: DialogSlot::Title,
        size,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_close_label,
        has_custom_class_name: false,
        has_custom_motion: false,
        has_on_exit_complete,
    });
    let title_class = logic::compose_class_name(None, title_state);
    let title_class = StoredValue::new(title_class);

    let description_state = logic::resolve_state(DialogPartStateInput {
        slot: DialogSlot::Description,
        size,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_close_label,
        has_custom_class_name: false,
        has_custom_motion: false,
        has_on_exit_complete,
    });
    let description_class = logic::compose_class_name(None, description_state);
    let description_class = StoredValue::new(description_class);

    let body_state = logic::resolve_state(DialogPartStateInput {
        slot: DialogSlot::Body,
        size,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_close_label,
        has_custom_class_name: false,
        has_custom_motion: false,
        has_on_exit_complete,
    });
    let body_class = logic::compose_class_name(None, body_state);
    let body_class = StoredValue::new(body_class);

    let footer_state = logic::resolve_state(DialogPartStateInput {
        slot: DialogSlot::Footer,
        size,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_close_label,
        has_custom_class_name: false,
        has_custom_motion: false,
        has_on_exit_complete,
    });
    let footer_class = logic::compose_class_name(None, footer_state);
    let footer_class = StoredValue::new(footer_class);

    let close_state = logic::resolve_state(DialogPartStateInput {
        slot: DialogSlot::Close,
        size,
        has_description: has_custom_description,
        has_footer,
        show_close_button,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_close_label,
        has_custom_class_name: false,
        has_custom_motion: false,
        has_on_exit_complete,
    });
    let close_class = logic::compose_class_name(None, close_state);
    let close_class = StoredValue::new(close_class);

    let title_id = format!("{id_base}-title");
    let title_id_attr: Signal<String> = title_id.clone().into();

    let description_id = format!("{id_base}-description");
    let description_id_attr: Signal<String> = description_id.clone().into();

    let render_content = move || {
        view! {
            <div
                class=move || root_class.with_value(|class_name| class_name.clone())
                data-slot=root_state.slot_attr
                data-state=root_state.state_attr
                data-open=move || open.get().then_some("true")
                data-closed=move || (!open.get()).then_some("true")
                data-size=root_state.size_attr
                data-description=root_state.description_attr
                data-footer=root_state.footer_attr
                data-close-button=root_state.close_button_attr
                data-with-description=root_state.show_description.then_some("true")
                data-with-footer=root_state.show_footer.then_some("true")
                data-close-visible=root_state.show_close_button.then_some("true")
                data-custom-size=root_state.has_custom_size.then_some("true")
                data-custom-id=root_state.has_custom_id_base.then_some("true")
                data-custom-title=root_state.has_custom_title.then_some("true")
                data-custom-description=root_state.has_custom_description.then_some("true")
                data-custom-close=root_state.has_custom_close_label.then_some("true")
                data-custom-class=root_state.has_custom_class_name.then_some("true")
                data-custom-motion=root_state.has_custom_motion.then_some("true")
                data-custom-exit=root_state.has_on_exit_complete.then_some("true")
                data-size-source=root_state.size_source_attr
                data-id-source=root_state.id_source_attr
                data-title-source=root_state.title_source_attr
                data-description-source=root_state.description_source_attr
                data-footer-source=root_state.footer_source_attr
                data-close-source=root_state.close_source_attr
                data-class-source=root_state.class_source_attr
                data-motion-source=root_state.motion_source_attr
                data-exit-source=root_state.exit_source_attr
            >
                <Show when=move || root_state.show_close_button>
                    <span
                        class=move || close_class.with_value(|class_name| class_name.clone())
                        data-slot=close_state.slot_attr
                        data-state=close_state.state_attr
                        data-close-button=close_state.close_button_attr
                        data-close-source=close_state.close_source_attr
                    >
                        <IconButton
                            aria_label=close_label
                            variant=ButtonVariant::Ghost
                            size=ButtonSize::IconSm
                            on_press=on_close.get_value()
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
                    <Show when=move || root_state.show_description>
                        <p
                            class=move || {
                                description_class.with_value(|class_name| class_name.clone())
                            }
                            id=move || description_id_attr.get()
                            data-slot=description_state.slot_attr
                            data-state=description_state.state_attr
                            data-description-source=description_state.description_source_attr
                        >
                            {move || description.get_value().unwrap_or_default()}
                        </p>
                    </Show>
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
                        class=move || footer_class.with_value(|class_name| class_name.clone())
                        data-slot=footer_state.slot_attr
                        data-state=footer_state.state_attr
                        data-footer-source=footer_state.footer_source_attr
                    >
                        {move || footer.get_value().map(|slot| slot.run())}
                    </div>
                </Show>
            </div>
        }
    };

    if root_state.show_description {
        view! {
            <Overlay
                open=open
                on_close=on_close.get_value()
                aria_labelledby=title_id.clone()
                aria_describedby=description_id.clone()
                motion=motion.overlay
                on_exit_complete=on_exit_complete.get_value()
            >
                {render_content()}
            </Overlay>
        }
        .into_any()
    } else {
        view! {
            <Overlay
                open=open
                on_close=on_close.get_value()
                aria_labelledby=title_id.clone()
                motion=motion.overlay
                on_exit_complete=on_exit_complete.get_value()
            >
                {render_content()}
            </Overlay>
        }
        .into_any()
    }
}
