use crate::OnPress;
use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::dialog::{DialogMotion, DialogSize, logic};
use crate::overlay::Overlay;
use leptos::children::ViewFn;
use leptos::prelude::*;
use ui_headless::{A11yDirection, locale_attrs, use_controllable_open_state_traced};

const DIALOG_CLOSE_ICON_VIEWBOX: &str = "0 0 20 20";
const DIALOG_CLOSE_ICON_PATH_D: &str = "M5 5l10 10M15 5L5 15";
const DIALOG_CLOSE_ICON_STROKE_WIDTH: &str = "1.5";

fn render_dialog_close_icon() -> impl IntoView {
    view! {
        <svg viewBox=DIALOG_CLOSE_ICON_VIEWBOX fill="none" aria-hidden="true">
            <path
                d=DIALOG_CLOSE_ICON_PATH_D
                stroke="currentColor"
                stroke_width=DIALOG_CLOSE_ICON_STROKE_WIDTH
                stroke_linecap="round"
                stroke_linejoin="round"
            />
        </svg>
    }
}

fn render_dialog_close_section(
    root_state: crate::dialog::DialogPartState,
    close_state: crate::dialog::DialogPartState,
    close_class: StoredValue<String>,
    close_label: &'static str,
    close_action: StoredValue<OnPress>,
) -> AnyView {
    view! {
        <Show when=move || root_state.show_close_button>
            <span
                class=move || close_class.with_value(|class_name| class_name.clone())
                data-slot=close_state.slot_attr
                data-state=close_state.state_attr
                data-close-button=close_state.close_button_attr
                data-close-source=close_state.close_source_attr
            >
                <Button
                    aria_label=close_label
                    variant=ButtonVariant::Ghost
                    size=ButtonSize::IconSm
                    on_press=close_action.get_value()
                >
                    {render_dialog_close_icon()}
                </Button>
            </span>
        </Show>
    }
    .into_any()
}

#[derive(Clone, Copy)]
struct DialogHeaderRenderInput {
    root_state: crate::dialog::DialogPartState,
    header_state: crate::dialog::DialogPartState,
    title_state: crate::dialog::DialogPartState,
    description_state: crate::dialog::DialogPartState,
    header_class: StoredValue<String>,
    title_class: StoredValue<String>,
    description_class: StoredValue<String>,
    title_id_attr: Signal<String>,
    description_id_attr: Signal<String>,
    title: StoredValue<String>,
    description: StoredValue<Option<String>>,
}

fn render_dialog_header_section(input: DialogHeaderRenderInput) -> AnyView {
    let DialogHeaderRenderInput {
        root_state,
        header_state,
        title_state,
        description_state,
        header_class,
        title_class,
        description_class,
        title_id_attr,
        description_id_attr,
        title,
        description,
    } = input;

    view! {
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
                    class=move || description_class.with_value(|class_name| class_name.clone())
                    id=move || description_id_attr.get()
                    data-slot=description_state.slot_attr
                    data-state=description_state.state_attr
                    data-description-source=description_state.description_source_attr
                >
                    {move || description.get_value().unwrap_or_default()}
                </p>
            </Show>
        </div>
    }
    .into_any()
}

fn render_dialog_body_section(
    body_state: crate::dialog::DialogPartState,
    body_class: StoredValue<String>,
    children: AnyView,
) -> AnyView {
    view! {
        <div
            class=move || body_class.with_value(|class_name| class_name.clone())
            data-slot=body_state.slot_attr
            data-state=body_state.state_attr
        >
            {children}
        </div>
    }
    .into_any()
}

fn render_dialog_footer_section(
    root_state: crate::dialog::DialogPartState,
    footer_state: crate::dialog::DialogPartState,
    footer_class: StoredValue<String>,
    footer: StoredValue<Option<ViewFn>>,
) -> AnyView {
    view! {
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
    }
    .into_any()
}

#[component]
pub fn Dialog(
    #[prop(optional)] is_open: Option<Signal<bool>>,
    #[prop(optional)] open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] on_close: Option<OnPress>,
    id_base: String,
    title: String,
    children: ChildrenFn,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] footer: Option<ViewFn>,
    #[prop(optional, default = logic::DEFAULT_SIZE)] size: DialogSize,
    #[prop(optional, default = logic::DEFAULT_SHOW_CLOSE_BUTTON)] is_close_button_visible: bool,
    #[prop(optional)] show_close_button: Option<bool>,
    #[prop(optional, default = logic::DEFAULT_CLOSE_LABEL)] close_label: &'static str,
    #[prop(optional)] motion: DialogMotion,
    #[prop(optional)] on_exit_complete: Option<Callback<()>>,
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let open_state = logic::normalize_open_state(logic::DialogOpenStateInput {
        is_open,
        open,
        default_open,
        on_open_change,
    });
    let open_mode = open_state.mode;
    let agent_source = logic::dialog_agent_source_from_open_mode(open_mode);
    let has_custom_default_open = open_state.has_default_open;
    let has_custom_on_open_change = open_state.has_open_change_handler;
    let can_request_close = logic::can_request_close(open_mode, has_custom_on_open_change);
    let open_mode_attr = open_state.open_mode_attr;
    let open_prop_source_attr = open_state.open_prop_source_attr;
    let open_source_attr = open_state.open_source_attr;
    let open_change_source_attr = open_state.open_change_source_attr;
    let open_state = use_controllable_open_state_traced(
        "dialog",
        open_state.open,
        Some(open_state.default_open),
        open_state.on_open_change,
    );
    let is_open = open_state.open;
    let request_open_change = open_state.request_open_change;

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

    let close_config = logic::normalize_close_config(logic::DialogCloseConfigInput {
        is_close_button_visible,
        show_close_button,
        close_label,
    });
    let close_button_visibility = close_config.close_button_visibility;
    let close_label = close_config.close_label;
    let has_custom_close_label = close_config.has_custom_close_label;

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    let motion = crate::dialog::motion::sanitize_motion(motion);
    let has_custom_motion = motion != DialogMotion::default();
    let exit_config = logic::normalize_exit_config(on_exit_complete);
    let has_on_exit_complete = exit_config.has_custom_on_exit_complete;
    let locale = locale_attrs(logic::normalize_optional_text(lang), dir);
    let locale_lang = StoredValue::new(locale.lang);
    let locale_dir = locale.dir;

    let on_close = StoredValue::new(on_close);
    let close_action: OnPress = Callback::new(move |_| {
        if can_request_close {
            request_open_change.run(false);
        }

        if let Some(on_close) = on_close.get_value() {
            on_close.run(());
        }
    });
    let close_action = StoredValue::new(close_action);
    let on_exit_complete = StoredValue::new(exit_config.on_exit_complete);
    let agent_contract = Signal::derive(move || {
        logic::resolve_agent_contract(logic::DialogAgentContractInput {
            is_open: is_open.get(),
            source: agent_source,
            open_change_source: open_change_source_attr,
        })
    });

    let part_states = logic::resolve_part_states(logic::DialogPartStatesInput {
        size,
        has_description: has_custom_description,
        has_footer,
        close_button_visibility,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_close_label,
        has_custom_class_name,
        has_custom_motion,
        has_on_exit_complete,
    });
    let part_classes = logic::resolve_part_classes(class_name, part_states);

    let root_state = part_states.root;
    let header_state = part_states.header;
    let title_state = part_states.title;
    let description_state = part_states.description;
    let body_state = part_states.body;
    let footer_state = part_states.footer;
    let close_state = part_states.close;

    let root_class = StoredValue::new(part_classes.root);
    let header_class = StoredValue::new(part_classes.header);
    let title_class = StoredValue::new(part_classes.title);
    let description_class = StoredValue::new(part_classes.description);
    let body_class = StoredValue::new(part_classes.body);
    let footer_class = StoredValue::new(part_classes.footer);
    let close_class = StoredValue::new(part_classes.close);

    let title_id = format!("{id_base}-title");
    let title_id_attr: Signal<String> = title_id.clone().into();

    let description_id = format!("{id_base}-description");
    let description_id_attr: Signal<String> = description_id.clone().into();

    let render_content = move || {
        let close_view = render_dialog_close_section(
            root_state,
            close_state,
            close_class,
            close_label,
            close_action,
        );
        let header_view = render_dialog_header_section(DialogHeaderRenderInput {
            root_state,
            header_state,
            title_state,
            description_state,
            header_class,
            title_class,
            description_class,
            title_id_attr,
            description_id_attr,
            title,
            description,
        });
        let body_view = render_dialog_body_section(body_state, body_class, children());
        let footer_view =
            render_dialog_footer_section(root_state, footer_state, footer_class, footer);

        view! {
            <div
                class=move || root_class.with_value(|class_name| class_name.clone())
                lang=move || locale_lang.with_value(|value| value.clone())
                dir=locale_dir
                data-slot=root_state.slot_attr
                data-state=root_state.state_attr
                data-open=move || is_open.get().then_some("true")
                data-closed=move || (!is_open.get()).then_some("true")
                data-open-mode=open_mode_attr
                data-open-source=open_source_attr
                data-open-change-source=open_change_source_attr
                data-open-prop-source=open_prop_source_attr
                data-ui-schema=move || agent_contract.get().schema_name
                data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
                data-ui-intent=move || agent_contract.get().intent.as_str()
                data-ui-action=move || agent_contract.get().action.as_str()
                data-ui-state=move || agent_contract.get().state.as_str()
                data-ui-source=move || agent_contract.get().source.as_str()
                data-ui-open-change-source=move || agent_contract.get().open_change_source
                data-ui-config-policy=move || agent_contract.get().config_policy.as_str()
                data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
                data-stream-mode=move || agent_contract.get().stream_mode.as_str()
                data-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
                data-output-status=move || agent_contract.get().output_status.as_str()
                data-controlled=(open_mode_attr == "controlled").then_some("true")
                data-uncontrolled=(open_mode_attr == "uncontrolled").then_some("true")
                data-custom-default-open=has_custom_default_open.then_some("true")
                data-custom-open-change=has_custom_on_open_change.then_some("true")
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
                data-custom-close=(root_state.close_source_attr == "custom").then_some("true")
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
                {close_view}
                {header_view}
                {body_view}
                {footer_view}
            </div>
        }
    };

    if root_state.show_description {
        view! {
            <Overlay
                open=is_open
                on_close=close_action.get_value()
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
                open=is_open
                on_close=close_action.get_value()
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
