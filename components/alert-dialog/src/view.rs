use crate::OnPress;
use crate::alert_dialog::{
    AlertDialogAutoFocusButton, AlertDialogMotion, AlertDialogPartState, AlertDialogPartStateInput,
    AlertDialogSlot, AlertDialogVariant, logic,
};
use crate::button::{Button, ButtonVariant};
use crate::overlay::Overlay;
use leptos::{html, prelude::*};
use std::sync::Arc;
use ui_headless::{A11yDirection, locale_attrs};

#[cfg(target_arch = "wasm32")]
fn focus_button_soon(node_ref: NodeRef<html::Button>) {
    use leptos::wasm_bindgen::{JsCast, closure::Closure};

    fn try_focus(node_ref: &NodeRef<html::Button>) -> bool {
        let Some(el) = node_ref.get_untracked() else {
            return false;
        };
        ui_observability::observe_js_result!(el.focus());
        true
    }

    if try_focus(&node_ref) {
        return;
    }

    let Some(window) = web_sys::window() else {
        return;
    };

    let callback = Closure::once_into_js(move || {
        try_focus(&node_ref);
    });

    let Some(callback) = callback.dyn_ref::<js_sys::Function>() else {
        return;
    };

    drop(window.set_timeout_with_callback_and_timeout_and_arguments_0(callback, 0));
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_button_soon(_node_ref: NodeRef<html::Button>) {}

const ALERT_DIALOG_TYPE_ICON_VIEWBOX: &str = "0 0 20 20";
const ALERT_DIALOG_TYPE_ICON_STROKE: &str = "currentColor";
const ALERT_DIALOG_WARNING_ICON_OUTLINE_D: &str =
    "M10 2.8l8.2 14.4c.6 1-.1 2.3-1.3 2.3H3.1c-1.2 0-1.9-1.3-1.3-2.3L10 2.8z";
const ALERT_DIALOG_WARNING_ICON_VERTICAL_D: &str = "M10 7.2v5.8";
const ALERT_DIALOG_WARNING_ICON_DOT_D: &str = "M10 15.8h.01";
const ALERT_DIALOG_ERROR_ICON_RING_D: &str = "M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16z";
const ALERT_DIALOG_ERROR_ICON_VERTICAL_D: &str = "M10 6.2v5.2";
const ALERT_DIALOG_ERROR_ICON_DOT_D: &str = "M10 14.2h.01";

#[derive(Clone, Copy)]
struct AlertDialogTypeIconPath {
    d: &'static str,
    stroke_width: &'static str,
    stroke_linecap: Option<&'static str>,
    stroke_linejoin: Option<&'static str>,
}

const ALERT_DIALOG_WARNING_ICON_PATHS: [AlertDialogTypeIconPath; 3] = [
    AlertDialogTypeIconPath {
        d: ALERT_DIALOG_WARNING_ICON_OUTLINE_D,
        stroke_width: "1.5",
        stroke_linecap: None,
        stroke_linejoin: Some("round"),
    },
    AlertDialogTypeIconPath {
        d: ALERT_DIALOG_WARNING_ICON_VERTICAL_D,
        stroke_width: "1.5",
        stroke_linecap: Some("round"),
        stroke_linejoin: None,
    },
    AlertDialogTypeIconPath {
        d: ALERT_DIALOG_WARNING_ICON_DOT_D,
        stroke_width: "2.5",
        stroke_linecap: Some("round"),
        stroke_linejoin: None,
    },
];

const ALERT_DIALOG_ERROR_ICON_PATHS: [AlertDialogTypeIconPath; 3] = [
    AlertDialogTypeIconPath {
        d: ALERT_DIALOG_ERROR_ICON_RING_D,
        stroke_width: "1.5",
        stroke_linecap: None,
        stroke_linejoin: None,
    },
    AlertDialogTypeIconPath {
        d: ALERT_DIALOG_ERROR_ICON_VERTICAL_D,
        stroke_width: "1.5",
        stroke_linecap: Some("round"),
        stroke_linejoin: None,
    },
    AlertDialogTypeIconPath {
        d: ALERT_DIALOG_ERROR_ICON_DOT_D,
        stroke_width: "2.5",
        stroke_linecap: Some("round"),
        stroke_linejoin: None,
    },
];

fn render_type_icon_path(path: AlertDialogTypeIconPath) -> AnyView {
    match (path.stroke_linecap, path.stroke_linejoin) {
        (Some(stroke_linecap), Some(stroke_linejoin)) => view! {
            <path
                d=path.d
                stroke=ALERT_DIALOG_TYPE_ICON_STROKE
                stroke_width=path.stroke_width
                stroke_linecap=stroke_linecap
                stroke_linejoin=stroke_linejoin
            />
        }
        .into_any(),
        (Some(stroke_linecap), None) => view! {
            <path
                d=path.d
                stroke=ALERT_DIALOG_TYPE_ICON_STROKE
                stroke_width=path.stroke_width
                stroke_linecap=stroke_linecap
            />
        }
        .into_any(),
        (None, Some(stroke_linejoin)) => view! {
            <path
                d=path.d
                stroke=ALERT_DIALOG_TYPE_ICON_STROKE
                stroke_width=path.stroke_width
                stroke_linejoin=stroke_linejoin
            />
        }
        .into_any(),
        (None, None) => view! {
            <path d=path.d stroke=ALERT_DIALOG_TYPE_ICON_STROKE stroke_width=path.stroke_width />
        }
        .into_any(),
    }
}

fn render_static_type_icon(paths: &'static [AlertDialogTypeIconPath]) -> AnyView {
    let icon_paths = paths
        .iter()
        .copied()
        .map(render_type_icon_path)
        .collect_view();

    view! {
        <svg viewBox=ALERT_DIALOG_TYPE_ICON_VIEWBOX fill="none">
            {icon_paths}
        </svg>
    }
    .into_any()
}

fn render_variant_type_icon(variant: AlertDialogVariant) -> AnyView {
    match variant {
        AlertDialogVariant::Warning => render_static_type_icon(&ALERT_DIALOG_WARNING_ICON_PATHS),
        AlertDialogVariant::Error => render_static_type_icon(&ALERT_DIALOG_ERROR_ICON_PATHS),
        _ => ().into_any(),
    }
}

struct DialogHeaderViewCtx {
    variant: AlertDialogVariant,
    root_state: Memo<AlertDialogPartState>,
    header_class: StoredValue<String>,
    header_state: AlertDialogPartState,
    type_icon_class: StoredValue<String>,
    type_icon_state: AlertDialogPartState,
    header_text_class: StoredValue<String>,
    header_text_state: AlertDialogPartState,
    title_class: StoredValue<String>,
    title_state: AlertDialogPartState,
    title_id_attr: Signal<String>,
    title: StoredValue<String>,
    description_class: StoredValue<String>,
    description_state: AlertDialogPartState,
    description_id_attr: Signal<String>,
    description: StoredValue<Option<String>>,
}

fn render_dialog_header(ctx: DialogHeaderViewCtx) -> AnyView {
    view! {
        <div
            class=move || ctx.header_class.with_value(|class_name| class_name.clone())
            data-slot=ctx.header_state.slot_attr
            data-state=ctx.header_state.state_attr
        >
            <Show when=move || ctx.root_state.get().show_type_icon>
                <span
                    class=move || ctx.type_icon_class.with_value(|class_name| class_name.clone())
                    data-slot=ctx.type_icon_state.slot_attr
                    data-state=ctx.type_icon_state.state_attr
                    data-variant=ctx.type_icon_state.variant_attr
                    aria-hidden="true"
                >
                    {render_variant_type_icon(ctx.variant)}
                </span>
            </Show>

            <div
                class=move || ctx.header_text_class.with_value(|class_name| class_name.clone())
                data-slot=ctx.header_text_state.slot_attr
                data-state=ctx.header_text_state.state_attr
            >
                <h2
                    class=move || ctx.title_class.with_value(|class_name| class_name.clone())
                    id=move || ctx.title_id_attr.get()
                    data-slot=ctx.title_state.slot_attr
                    data-state=ctx.title_state.state_attr
                    data-title-source=ctx.title_state.title_source_attr
                >
                    {move || ctx.title.get_value()}
                </h2>
                <Show when=move || ctx.root_state.get().show_description>
                    <p
                        class=move || ctx.description_class.with_value(|class_name| class_name.clone())
                        id=move || ctx.description_id_attr.get()
                        data-slot=ctx.description_state.slot_attr
                        data-state=ctx.description_state.state_attr
                        data-description-source=ctx.description_state.description_source_attr
                    >
                        {move || ctx.description.get_value().unwrap_or_default()}
                    </p>
                </Show>
            </div>
        </div>
    }
    .into_any()
}

struct DialogFooterViewCtx {
    root_state: Memo<AlertDialogPartState>,
    footer_class: StoredValue<String>,
    footer_state: AlertDialogPartState,
    cancel_class: StoredValue<String>,
    cancel_state: AlertDialogPartState,
    secondary_class: StoredValue<String>,
    secondary_state: AlertDialogPartState,
    confirm_class: StoredValue<String>,
    confirm_state: AlertDialogPartState,
    cancel_ref: NodeRef<html::Button>,
    secondary_ref: NodeRef<html::Button>,
    confirm_ref: NodeRef<html::Button>,
    on_cancel_press: OnPress,
    on_secondary_press: OnPress,
    on_confirm_press: OnPress,
    cancel_label: StoredValue<String>,
    secondary_label: StoredValue<Option<String>>,
    confirm_label: StoredValue<String>,
    secondary_disabled: bool,
    confirm_disabled: bool,
    confirm_variant: ButtonVariant,
}

fn render_dialog_footer(ctx: DialogFooterViewCtx) -> AnyView {
    view! {
        <div
            class=move || ctx.footer_class.with_value(|class_name| class_name.clone())
            data-slot=ctx.footer_state.slot_attr
            data-state=ctx.footer_state.state_attr
        >
            <Show when=move || ctx.root_state.get().show_cancel>
                <span
                    class=move || ctx.cancel_class.with_value(|class_name| class_name.clone())
                    data-slot=ctx.cancel_state.slot_attr
                    data-state=ctx.cancel_state.state_attr
                    data-cancel-source=ctx.cancel_state.cancel_source_attr
                >
                    <Button
                        variant=ButtonVariant::Secondary
                        is_disabled=false
                        node_ref=ctx.cancel_ref
                        on_press=ctx.on_cancel_press
                    >
                        {move || ctx.cancel_label.get_value()}
                    </Button>
                </span>
            </Show>
            <Show when=move || ctx.root_state.get().show_secondary>
                <span
                    class=move || ctx.secondary_class.with_value(|class_name| class_name.clone())
                    data-slot=ctx.secondary_state.slot_attr
                    data-state=ctx.secondary_state.state_attr
                    data-secondary-source=ctx.secondary_state.secondary_source_attr
                >
                    <Button
                        variant=ButtonVariant::Secondary
                        is_disabled=ctx.secondary_disabled
                        node_ref=ctx.secondary_ref
                        on_press=ctx.on_secondary_press
                    >
                        {move || ctx.secondary_label.get_value().unwrap_or_default()}
                    </Button>
                </span>
            </Show>
            <span
                class=move || ctx.confirm_class.with_value(|class_name| class_name.clone())
                data-slot=ctx.confirm_state.slot_attr
                data-state=ctx.confirm_state.state_attr
                data-confirm-source=ctx.confirm_state.confirm_source_attr
            >
                <Button
                    variant=ctx.confirm_variant
                    is_disabled=ctx.confirm_disabled
                    node_ref=ctx.confirm_ref
                    on_press=ctx.on_confirm_press
                >
                    {move || ctx.confirm_label.get_value()}
                </Button>
            </span>
        </div>
    }
    .into_any()
}

struct DialogContentViewCtx {
    open: Signal<bool>,
    root_class: Memo<String>,
    root_state: Memo<AlertDialogPartState>,
    agent_contract: Signal<logic::AlertDialogAgentContract>,
    locale_lang: StoredValue<Option<String>>,
    locale_dir: Option<&'static str>,
    header_view: AnyView,
    footer_view: AnyView,
}

fn render_dialog_content(ctx: DialogContentViewCtx) -> AnyView {
    view! {
        <div
            class=move || ctx.root_class.get()
            lang=move || ctx.locale_lang.with_value(|value| value.clone())
            dir=ctx.locale_dir
            data-slot=move || ctx.root_state.get().slot_attr
            data-state=move || ctx.root_state.get().state_attr
            data-open=move || ctx.open.get().then_some("true")
            data-closed=move || (!ctx.open.get()).then_some("true")
            data-variant=move || ctx.root_state.get().variant_attr
            data-description=move || ctx.root_state.get().description_attr
            data-cancel=move || ctx.root_state.get().cancel_attr
            data-secondary=move || ctx.root_state.get().secondary_attr
            data-confirm-disabled=move || ctx.root_state.get().confirm_disabled_attr
            data-secondary-disabled=move || ctx.root_state.get().secondary_disabled_attr
            data-auto-focus=move || ctx.root_state.get().auto_focus_attr
            data-with-description=move || ctx.root_state.get().show_description.then_some("true")
            data-show-cancel=move || ctx.root_state.get().show_cancel.then_some("true")
            data-show-secondary=move || ctx.root_state.get().show_secondary.then_some("true")
            data-with-type-icon=move || ctx.root_state.get().show_type_icon.then_some("true")
            data-custom-variant=move || ctx.root_state.get().has_custom_variant.then_some("true")
            data-custom-id=move || ctx.root_state.get().has_custom_id_base.then_some("true")
            data-custom-title=move || ctx.root_state.get().has_custom_title.then_some("true")
            data-custom-description=move || ctx.root_state.get().has_custom_description.then_some("true")
            data-custom-confirm=move || (ctx.root_state.get().confirm_source_attr == "custom").then_some("true")
            data-custom-cancel=move || (ctx.root_state.get().cancel_source_attr == "custom").then_some("true")
            data-custom-secondary=move || (ctx.root_state.get().secondary_source_attr == "custom").then_some("true")
            data-custom-auto-focus=move || {
                (ctx.root_state.get().auto_focus_source_attr == "custom").then_some("true")
            }
            data-custom-motion=move || ctx.root_state.get().has_custom_motion.then_some("true")
            data-custom-exit=move || ctx.root_state.get().has_on_exit_complete.then_some("true")
            data-variant-source=move || ctx.root_state.get().variant_source_attr
            data-id-source=move || ctx.root_state.get().id_source_attr
            data-title-source=move || ctx.root_state.get().title_source_attr
            data-description-source=move || ctx.root_state.get().description_source_attr
            data-cancel-source=move || ctx.root_state.get().cancel_source_attr
            data-secondary-source=move || ctx.root_state.get().secondary_source_attr
            data-confirm-source=move || ctx.root_state.get().confirm_source_attr
            data-auto-focus-source=move || ctx.root_state.get().auto_focus_source_attr
            data-motion-source=move || ctx.root_state.get().motion_source_attr
            data-exit-source=move || ctx.root_state.get().exit_source_attr
            data-ui-schema=move || ctx.agent_contract.get().schema_name
            data-ui-schema-version=move || ctx.agent_contract.get().schema_version.as_str()
            data-ui-intent=move || ctx.agent_contract.get().intent.as_str()
            data-ui-action=move || ctx.agent_contract.get().action.as_str()
            data-ui-state=move || ctx.agent_contract.get().state.as_str()
            data-ui-source=move || ctx.agent_contract.get().source.as_str()
            data-ui-config-policy=move || ctx.agent_contract.get().config_policy.as_str()
            data-ui-output-status=move || ctx.agent_contract.get().output_status.as_str()
            data-output-status=move || ctx.agent_contract.get().output_status.as_str()
            data-ui-capability-description=move || {
                ctx.agent_contract
                    .get()
                    .capabilities
                    .has_description
                    .then_some("true")
            }
            data-ui-capability-cancel=move || {
                ctx.agent_contract
                    .get()
                    .capabilities
                    .has_cancel
                    .then_some("true")
            }
            data-ui-capability-secondary=move || {
                ctx.agent_contract
                    .get()
                    .capabilities
                    .has_secondary
                    .then_some("true")
            }
            data-ui-capability-confirm=move || {
                ctx.agent_contract
                    .get()
                    .capabilities
                    .can_confirm
                    .then_some("true")
            }
            data-ui-capability-dismiss=move || {
                ctx.agent_contract
                    .get()
                    .capabilities
                    .can_dismiss
                    .then_some("true")
            }
            data-ui-source-variant=move || ctx.agent_contract.get().variant_source
            data-ui-source-title=move || ctx.agent_contract.get().title_source
            data-ui-source-description=move || ctx.agent_contract.get().description_source
            data-ui-source-cancel=move || ctx.agent_contract.get().cancel_source
            data-ui-source-secondary=move || ctx.agent_contract.get().secondary_source
            data-ui-source-confirm=move || ctx.agent_contract.get().confirm_source
            data-ui-source-auto-focus=move || ctx.agent_contract.get().auto_focus_source
            data-ui-source-motion=move || ctx.agent_contract.get().motion_source
        >
            {ctx.header_view}
            {ctx.footer_view}
        </div>
    }
    .into_any()
}

#[component]
pub fn AlertDialog(
    open: Signal<bool>,
    id_base: String,
    title: String,
    on_close: OnPress,
    confirm_label: String,
    on_confirm: OnPress,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] cancel_label: Option<String>,
    #[prop(optional, into)] secondary_label: Option<String>,
    #[prop(optional)] on_secondary: Option<OnPress>,
    #[prop(optional)] on_cancel: Option<OnPress>,
    #[prop(optional)] is_confirm_disabled: Option<bool>,
    #[prop(optional)] confirm_disabled: Option<bool>,
    #[prop(optional)] is_secondary_disabled: Option<bool>,
    #[prop(optional)] secondary_disabled: Option<bool>,
    #[prop(optional, default = logic::DEFAULT_AUTO_FOCUS_BUTTON)]
    auto_focus_button: AlertDialogAutoFocusButton,
    #[prop(optional)] variant: AlertDialogVariant,
    #[prop(optional)] motion: AlertDialogMotion,
    #[prop(optional)] on_exit_complete: Option<Callback<()>>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let has_custom_id_base = id_base != logic::DEFAULT_ID_BASE;

    let title = logic::normalize_required_text(title, logic::DEFAULT_TITLE);
    let has_custom_title = title != logic::DEFAULT_TITLE;
    let title = StoredValue::new(title);

    let description = logic::normalize_optional_text(description);
    let show_description = description.is_some();
    let has_custom_description = show_description;
    let description = StoredValue::new(description);

    let confirm_label = logic::normalize_required_text(confirm_label, logic::DEFAULT_CONFIRM_LABEL);
    let has_custom_confirm_label = confirm_label != logic::DEFAULT_CONFIRM_LABEL;
    let confirm_label = StoredValue::new(confirm_label);

    let cancel_label = logic::normalize_cancel_label(cancel_label);
    let has_custom_cancel_label = cancel_label != logic::DEFAULT_CANCEL_LABEL;
    let show_cancel = !cancel_label.trim().is_empty();
    let cancel_label = StoredValue::new(cancel_label);

    let secondary_label = logic::normalize_secondary_label(secondary_label);
    let has_custom_secondary_label = secondary_label.is_some();
    let show_secondary = secondary_label.is_some();
    let secondary_label = StoredValue::new(secondary_label);

    let has_custom_on_secondary = on_secondary.is_some();
    let has_custom_on_cancel = on_cancel.is_some();
    let on_secondary = StoredValue::new(on_secondary);
    let on_cancel = StoredValue::new(on_cancel);

    let confirm_disabled = logic::resolve_disabled_flag(
        is_confirm_disabled,
        confirm_disabled,
        logic::DEFAULT_CONFIRM_DISABLED,
    );
    let secondary_disabled = logic::resolve_disabled_flag(
        is_secondary_disabled,
        secondary_disabled,
        logic::DEFAULT_SECONDARY_DISABLED,
    );

    let has_custom_auto_focus_button = auto_focus_button != logic::DEFAULT_AUTO_FOCUS_BUTTON;
    let motion = crate::alert_dialog::motion::sanitize_motion(motion);
    let has_custom_motion = motion != AlertDialogMotion::default();
    let has_on_exit_complete = on_exit_complete.is_some();
    let locale = locale_attrs(lang, dir);
    let locale_lang = StoredValue::new(locale.lang);
    let locale_dir = locale.dir;

    let on_close = StoredValue::new(on_close);
    let on_confirm = StoredValue::new(on_confirm);
    let on_exit_complete =
        StoredValue::new(on_exit_complete.unwrap_or_else(|| Callback::new(|_| {})));

    let title_id = format!("{id_base}-title");
    let description_id = format!("{id_base}-description");
    let title_id_attr: Signal<String> = title_id.clone().into();
    let description_id_attr: Signal<String> = description_id.clone().into();

    let root_state = Memo::new(move |_| {
        logic::resolve_state(AlertDialogPartStateInput {
            slot: AlertDialogSlot::Root,
            is_open: open.get(),
            variant,
            auto_focus_button,
            show_description,
            show_cancel,
            show_secondary,
            confirm_disabled,
            secondary_disabled,
            has_custom_id_base,
            has_custom_title,
            has_custom_description,
            has_custom_confirm_label,
            has_custom_cancel_label,
            has_custom_secondary_label,
            has_custom_on_cancel,
            has_custom_on_secondary,
            has_custom_auto_focus_button,
            has_custom_motion,
            has_on_exit_complete,
        })
    });
    let root_class = Memo::new(move |_| logic::compose_class_name(None, root_state.get()));
    let agent_contract = Signal::derive(move || {
        logic::resolve_agent_contract(logic::AlertDialogAgentContractInput {
            is_open: open.get(),
            root_state: root_state.get(),
        })
    });

    let make_state = |slot| {
        logic::resolve_state(AlertDialogPartStateInput {
            slot,
            is_open: open.get_untracked(),
            variant,
            auto_focus_button,
            show_description,
            show_cancel,
            show_secondary,
            confirm_disabled,
            secondary_disabled,
            has_custom_id_base,
            has_custom_title,
            has_custom_description,
            has_custom_confirm_label,
            has_custom_cancel_label,
            has_custom_secondary_label,
            has_custom_on_cancel,
            has_custom_on_secondary,
            has_custom_auto_focus_button,
            has_custom_motion,
            has_on_exit_complete,
        })
    };

    let header_state = make_state(AlertDialogSlot::Header);
    let header_class = StoredValue::new(logic::compose_class_name(None, header_state));

    let header_text_state = make_state(AlertDialogSlot::HeaderText);
    let header_text_class = StoredValue::new(logic::compose_class_name(None, header_text_state));

    let type_icon_state = make_state(AlertDialogSlot::TypeIcon);
    let type_icon_class = StoredValue::new(logic::compose_class_name(None, type_icon_state));

    let title_state = make_state(AlertDialogSlot::Title);
    let title_class = StoredValue::new(logic::compose_class_name(None, title_state));

    let description_state = make_state(AlertDialogSlot::Description);
    let description_class = StoredValue::new(logic::compose_class_name(None, description_state));

    let footer_state = make_state(AlertDialogSlot::Footer);
    let footer_class = StoredValue::new(logic::compose_class_name(None, footer_state));

    let cancel_state = make_state(AlertDialogSlot::CancelAction);
    let cancel_class = StoredValue::new(logic::compose_class_name(None, cancel_state));

    let secondary_state = make_state(AlertDialogSlot::SecondaryAction);
    let secondary_class = StoredValue::new(logic::compose_class_name(None, secondary_state));

    let confirm_state = make_state(AlertDialogSlot::ConfirmAction);
    let confirm_class = StoredValue::new(logic::compose_class_name(None, confirm_state));

    let on_cancel_press: OnPress = Callback::new(move |_| {
        on_close.get_value().run(());
        if let Some(callback) = on_cancel.get_value() {
            callback.run(());
        }
    });

    let on_secondary_press: OnPress = Callback::new(move |_| {
        on_close.get_value().run(());
        if let Some(callback) = on_secondary.get_value() {
            callback.run(());
        }
    });

    let on_confirm_press: OnPress = Callback::new(move |_| {
        on_close.get_value().run(());
        on_confirm.get_value().run(());
    });

    let confirm_variant = match variant {
        AlertDialogVariant::Destructive => ButtonVariant::Destructive,
        AlertDialogVariant::Default
        | AlertDialogVariant::Confirmation
        | AlertDialogVariant::Warning
        | AlertDialogVariant::Error => ButtonVariant::Default,
    };

    let cancel_ref: NodeRef<html::Button> = NodeRef::new();
    let secondary_ref: NodeRef<html::Button> = NodeRef::new();
    let confirm_ref: NodeRef<html::Button> = NodeRef::new();

    Effect::new(move |_| {
        if !open.get() {
            return;
        }

        let target = match auto_focus_button {
            AlertDialogAutoFocusButton::Cancel if show_cancel => Some(cancel_ref),
            AlertDialogAutoFocusButton::Secondary if show_secondary => Some(secondary_ref),
            AlertDialogAutoFocusButton::Confirm => Some(confirm_ref),
            AlertDialogAutoFocusButton::None => None,
            _ => None,
        };

        if let Some(target) = target {
            focus_button_soon(target);
        }
    });

    let content = Arc::new(move || {
        let header_view = render_dialog_header(DialogHeaderViewCtx {
            variant,
            root_state,
            header_class,
            header_state,
            type_icon_class,
            type_icon_state,
            header_text_class,
            header_text_state,
            title_class,
            title_state,
            title_id_attr,
            title,
            description_class,
            description_state,
            description_id_attr,
            description,
        });

        let footer_view = render_dialog_footer(DialogFooterViewCtx {
            root_state,
            footer_class,
            footer_state,
            cancel_class,
            cancel_state,
            secondary_class,
            secondary_state,
            confirm_class,
            confirm_state,
            cancel_ref,
            secondary_ref,
            confirm_ref,
            on_cancel_press,
            on_secondary_press,
            on_confirm_press,
            cancel_label,
            secondary_label,
            confirm_label,
            secondary_disabled,
            confirm_disabled,
            confirm_variant,
        });

        render_dialog_content(DialogContentViewCtx {
            open,
            root_class,
            root_state,
            agent_contract,
            locale_lang,
            locale_dir,
            header_view,
            footer_view,
        })
    });

    if show_description {
        view! {
            <Overlay
                open=open
                on_close=on_close.get_value()
                role="alertdialog"
                aria_labelledby=title_id.clone()
                aria_describedby=description_id.clone()
                motion=motion.overlay
                on_exit_complete=on_exit_complete.get_value()
            >
                {{
                    let content = content.clone();
                    move || content()
                }}
            </Overlay>
        }
        .into_any()
    } else {
        view! {
            <Overlay
                open=open
                on_close=on_close.get_value()
                role="alertdialog"
                aria_labelledby=title_id.clone()
                motion=motion.overlay
                on_exit_complete=on_exit_complete.get_value()
            >
                {{
                    let content = content.clone();
                    move || content()
                }}
            </Overlay>
        }
        .into_any()
    }
}
