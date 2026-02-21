use crate::{DrawerMotion, DrawerPlacement, logic, motion};
use leptos::children::ViewFn;
use leptos::prelude::*;
use ui_button::{Button, ButtonSize, ButtonVariant};
use ui_headless::{A11yDirection, OnPress, use_controllable_open_state_traced};
use ui_sheet::Sheet;

const DRAWER_CLOSE_ICON_VIEWBOX: &str = "0 0 20 20";
const DRAWER_CLOSE_ICON_PATH: &str = "M5 5l10 10M15 5L5 15";
const DRAWER_CLOSE_ICON_STROKE_WIDTH: &str = "1.5";

fn render_drawer_close_icon() -> impl IntoView {
    view! {
        <svg viewBox=DRAWER_CLOSE_ICON_VIEWBOX fill="none" aria-hidden="true">
            <path
                d=DRAWER_CLOSE_ICON_PATH
                stroke="currentColor"
                stroke_width=DRAWER_CLOSE_ICON_STROKE_WIDTH
                stroke_linecap="round"
                stroke_linejoin="round"
            />
        </svg>
    }
}

#[derive(Clone, Copy)]
struct DrawerCloseInputs {
    show_close_button: bool,
    close_state: logic::DrawerPartState,
    close_class: StoredValue<String>,
    close_label: &'static str,
    close_action: StoredValue<OnPress>,
}

fn render_drawer_close(inputs: DrawerCloseInputs) -> impl IntoView {
    let DrawerCloseInputs {
        show_close_button,
        close_state,
        close_class,
        close_label,
        close_action,
    } = inputs;

    view! {
        <Show when=move || show_close_button>
            <span
                class=move || close_class.with_value(|class_name| class_name.clone())
                data-slot=close_state.slot_attr
                data-state=close_state.state_attr
                data-close-source=close_state.close_source_attr
            >
                <Button
                    aria_label=close_label
                    variant=ButtonVariant::Ghost
                    size=ButtonSize::IconSm
                    on_press=close_action.get_value()
                >
                    {render_drawer_close_icon()}
                </Button>
            </span>
        </Show>
    }
}

#[derive(Clone, Copy)]
struct DrawerHeaderInputs {
    header_state: logic::DrawerPartState,
    header_class: StoredValue<String>,
    title_state: logic::DrawerPartState,
    title_class: StoredValue<String>,
    title_id_attr: Signal<String>,
    title: StoredValue<String>,
    show_description: bool,
    description_state: logic::DrawerPartState,
    description_class: StoredValue<String>,
    description_id_attr: Signal<String>,
    description: StoredValue<Option<String>>,
}

fn render_drawer_header(inputs: DrawerHeaderInputs) -> impl IntoView {
    let DrawerHeaderInputs {
        header_state,
        header_class,
        title_state,
        title_class,
        title_id_attr,
        title,
        show_description,
        description_state,
        description_class,
        description_id_attr,
        description,
    } = inputs;

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

            <Show when=move || show_description>
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
}

#[derive(Clone, Copy)]
struct DrawerBodyInputs {
    body_state: logic::DrawerPartState,
    body_class: StoredValue<String>,
    children: StoredValue<ChildrenFn>,
}

fn render_drawer_body(inputs: DrawerBodyInputs) -> impl IntoView {
    let DrawerBodyInputs {
        body_state,
        body_class,
        children,
    } = inputs;

    view! {
        <div
            class=move || body_class.with_value(|class_name| class_name.clone())
            data-slot=body_state.slot_attr
            data-state=body_state.state_attr
        >
            {move || children.with_value(|children| children())}
        </div>
    }
}

#[derive(Clone, Copy)]
struct DrawerFooterInputs {
    show_footer: bool,
    footer_state: logic::DrawerPartState,
    footer_class: StoredValue<String>,
    footer: StoredValue<Option<ViewFn>>,
}

fn render_drawer_footer(inputs: DrawerFooterInputs) -> impl IntoView {
    let DrawerFooterInputs {
        show_footer,
        footer_state,
        footer_class,
        footer,
    } = inputs;

    view! {
        <Show when=move || show_footer>
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
}

#[derive(Clone, Copy)]
struct DrawerRootInputs {
    root_state: logic::DrawerPartState,
    root_class: StoredValue<String>,
    open: Signal<bool>,
    open_mode_attr: &'static str,
    open_value_source: logic::DrawerOpenValueSource,
    open_action_source: ReadSignal<logic::DrawerOpenActionSource>,
    agent_contract: Signal<logic::DrawerAgentContract>,
    close: DrawerCloseInputs,
    header: DrawerHeaderInputs,
    body: DrawerBodyInputs,
    footer: DrawerFooterInputs,
}

fn render_drawer_root(inputs: DrawerRootInputs) -> impl IntoView {
    let DrawerRootInputs {
        root_state,
        root_class,
        open,
        open_mode_attr,
        open_value_source,
        open_action_source,
        agent_contract,
        close,
        header,
        body,
        footer,
    } = inputs;

    view! {
        <div
            class=move || root_class.with_value(|class_name| class_name.clone())
            data-slot=root_state.slot_attr
            data-state=root_state.state_attr
            data-open-state=move || logic::open_state_attr(open.get())
            data-open-mode=open_mode_attr
            data-open-source=open_value_source.as_attr()
            data-open-action-source=move || open_action_source.get().as_attr()
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
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-config-policy=move || agent_contract.get().config_policy.as_str()
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
            data-ui-capability-description=move || {
                agent_contract.get().capabilities.has_description.then_some("true")
            }
            data-ui-capability-footer=move || {
                agent_contract.get().capabilities.has_footer.then_some("true")
            }
            data-ui-capability-open=move || {
                agent_contract.get().capabilities.can_open.then_some("true")
            }
            data-ui-capability-close=move || {
                agent_contract.get().capabilities.can_close.then_some("true")
            }
        >
            {render_drawer_close(close)}
            {render_drawer_header(header)}
            {render_drawer_body(body)}
            {render_drawer_footer(footer)}
        </div>
    }
}

#[component]
pub fn Drawer(
    #[prop(optional)] is_open: Option<Signal<bool>>,
    #[prop(optional)] default_open: Option<bool>,
    #[prop(optional)] on_open_change: Option<Callback<bool>>,
    #[prop(optional)] on_close: Option<OnPress>,
    id_base: String,
    title: String,
    children: ChildrenFn,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] footer: Option<ViewFn>,
    #[prop(optional)] placement: Option<DrawerPlacement>,
    #[prop(optional)] motion: DrawerMotion,
    #[prop(optional)] is_close_button_visible: Option<bool>,
    #[prop(optional)] close_label: Option<&'static str>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    /// Called after the close animation finishes (useful for presence/unmount).
    #[prop(optional)]
    on_exit_complete: Option<Callback<()>>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let open_state = logic::normalize_open_state(logic::DrawerOpenStateInput {
        is_open,
        default_open,
        on_open_change,
    });
    let open_state_signal = use_controllable_open_state_traced(
        "drawer",
        open_state.open,
        Some(open_state.default_open),
        open_state.on_open_change,
    );
    let open = open_state_signal.open;
    let request_open_change = open_state_signal.request_open_change;
    let open_mode_attr = logic::open_mode_attr(open_state.mode);
    let open_value_source =
        logic::resolve_open_value_source(open_state.mode, open_state.has_default_open);
    let (open_action_source, set_open_action_source) =
        signal(logic::DrawerOpenActionSource::Programmatic);

    let on_close = StoredValue::new(on_close);
    let can_request_open_change =
        logic::can_request_open_change(open_state.mode, open_state.has_open_change_handler);
    let close_action: OnPress = Callback::new(move |_| {
        set_open_action_source.set(logic::DrawerOpenActionSource::Interaction);
        if can_request_open_change {
            request_open_change.run(false);
        }

        if let Some(on_close) = on_close.get_value() {
            on_close.run(());
        }
    });
    let close_action = StoredValue::new(close_action);

    let view_config = logic::normalize_view_config(logic::DrawerViewConfigInput {
        placement,
        is_close_button_visible,
        close_label,
        on_exit_complete,
    });
    let placement = view_config.placement;
    let close_button_visibility = view_config.close_button_visibility;
    let close_label = view_config.close_label;
    let has_on_exit_complete = view_config.has_on_exit_complete;
    let on_exit_complete = view_config.on_exit_complete;

    let has_custom_id_base = !id_base.trim().is_empty();
    let has_custom_title = !title.trim().is_empty();

    let id_base = logic::normalize_id_base(id_base);
    let title = logic::normalize_required_text(title, logic::DEFAULT_TITLE);
    let description = logic::normalize_optional_text(description);
    let lang = logic::normalize_optional_text(lang);
    let class_name = logic::normalize_optional_text(class_name);

    let title = StoredValue::new(title);
    let description = StoredValue::new(description);
    let footer = StoredValue::new(footer);
    let children = StoredValue::new(children);
    let motion = motion::sanitize_motion(motion);
    let sheet_placement = logic::to_sheet_placement(placement);

    let has_custom_description = description.get_value().is_some();
    let has_footer = footer.get_value().is_some();
    let has_custom_class_name = class_name.is_some();
    let has_custom_motion = motion != DrawerMotion::default();

    let part_states = logic::resolve_part_states(logic::DrawerPartStatesInput {
        placement,
        has_description: has_custom_description,
        has_footer,
        close_button_visibility,
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
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
    let description_id = format!("{id_base}-description");
    let title_id_attr: Signal<String> = title_id.clone().into();
    let description_id_attr: Signal<String> = description_id.clone().into();
    let agent_open_mode = open_state.mode;
    let agent_contract = Signal::derive(move || {
        logic::resolve_agent_contract(logic::DrawerAgentContractInput {
            is_open: open.get(),
            open_mode: agent_open_mode,
            has_description: root_state.show_description,
            has_footer: root_state.show_footer,
        })
    });

    let root_inputs = DrawerRootInputs {
        root_state,
        root_class,
        open,
        open_mode_attr,
        open_value_source,
        open_action_source,
        agent_contract,
        close: DrawerCloseInputs {
            show_close_button: root_state.show_close_button,
            close_state,
            close_class,
            close_label,
            close_action,
        },
        header: DrawerHeaderInputs {
            header_state,
            header_class,
            title_state,
            title_class,
            title_id_attr,
            title,
            show_description: root_state.show_description,
            description_state,
            description_class,
            description_id_attr,
            description,
        },
        body: DrawerBodyInputs {
            body_state,
            body_class,
            children,
        },
        footer: DrawerFooterInputs {
            show_footer: root_state.show_footer,
            footer_state,
            footer_class,
            footer,
        },
    };

    if root_state.show_description {
        view! {
            <Sheet
                open=open
                on_close=close_action.get_value()
                placement=sheet_placement
                aria_labelledby=title_id.clone()
                aria_describedby=description_id.clone()
                lang=lang.clone().unwrap_or_default()
                dir=dir.unwrap_or(A11yDirection::Ltr)
                motion=motion.sheet
                on_exit_complete=on_exit_complete
            >
                {render_drawer_root(root_inputs)}
            </Sheet>
        }
        .into_any()
    } else {
        view! {
            <Sheet
                open=open
                on_close=close_action.get_value()
                placement=sheet_placement
                aria_labelledby=title_id.clone()
                lang=lang.clone().unwrap_or_default()
                dir=dir.unwrap_or(A11yDirection::Ltr)
                motion=motion.sheet
                on_exit_complete=on_exit_complete
            >
                {render_drawer_root(root_inputs)}
            </Sheet>
        }
        .into_any()
    }
}
