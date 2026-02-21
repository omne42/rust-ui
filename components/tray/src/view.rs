use crate::{
    OnPress,
    button::{Button, ButtonSize, ButtonVariant},
    sheet::{Sheet, SheetPlacement},
    tray::{TrayMotion, logic},
};
use leptos::children::ViewFn;
use leptos::prelude::*;
use ui_headless::{A11yDirection, overlay_dialog_attrs};

const TRAY_CLOSE_ICON_VIEWBOX: &str = "0 0 20 20";
const TRAY_CLOSE_ICON_PATH: &str = "M5 5l10 10M15 5L5 15";
const TRAY_CLOSE_ICON_STROKE_WIDTH: &str = "1.5";

struct TrayPanelRenderInputs {
    open: Signal<bool>,
    on_close: OnPress,
    close_label: &'static str,
    root_state: logic::TrayPartState,
    header_state: logic::TrayPartState,
    title_state: logic::TrayPartState,
    description_state: logic::TrayPartState,
    body_state: logic::TrayPartState,
    footer_state: logic::TrayPartState,
    close_state: logic::TrayPartState,
    root_class: StoredValue<String>,
    header_class: StoredValue<String>,
    title_class: StoredValue<String>,
    description_class: StoredValue<String>,
    body_class: StoredValue<String>,
    footer_class: StoredValue<String>,
    close_class: StoredValue<String>,
    title_id_attr: Signal<String>,
    description_id_attr: Signal<String>,
    title: StoredValue<String>,
    description_text: StoredValue<String>,
    children: StoredValue<ChildrenFn>,
    footer: StoredValue<Option<ViewFn>>,
    panel_lang: StoredValue<Option<String>>,
    panel_dir: Option<&'static str>,
}

fn render_tray_close_icon() -> AnyView {
    view! {
        <svg viewBox=TRAY_CLOSE_ICON_VIEWBOX fill="none" aria-hidden="true">
            <path
                d=TRAY_CLOSE_ICON_PATH
                stroke="currentColor"
                stroke_width=TRAY_CLOSE_ICON_STROKE_WIDTH
                stroke_linecap="round"
                stroke_linejoin="round"
            />
        </svg>
    }
    .into_any()
}

fn render_tray_close_slot(
    show_close_button: bool,
    close_state: logic::TrayPartState,
    close_class: StoredValue<String>,
    close_label: &'static str,
    on_close: OnPress,
) -> AnyView {
    if !show_close_button {
        return ().into_any();
    }

    view! {
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
                on_press=on_close
            >
                {render_tray_close_icon()}
            </Button>
        </span>
    }
    .into_any()
}

struct TrayHeaderRenderInputs {
    show_description: bool,
    header_state: logic::TrayPartState,
    title_state: logic::TrayPartState,
    description_state: logic::TrayPartState,
    header_class: StoredValue<String>,
    title_class: StoredValue<String>,
    description_class: StoredValue<String>,
    title_id_attr: Signal<String>,
    description_id_attr: Signal<String>,
    title: StoredValue<String>,
    description_text: StoredValue<String>,
}

fn render_tray_header_slot(inputs: TrayHeaderRenderInputs) -> AnyView {
    let TrayHeaderRenderInputs {
        show_description,
        header_state,
        title_state,
        description_state,
        header_class,
        title_class,
        description_class,
        title_id_attr,
        description_id_attr,
        title,
        description_text,
    } = inputs;

    let description_view = show_description.then(|| {
        view! {
            <p
                class=move || description_class.with_value(|class_name| class_name.clone())
                id=move || description_id_attr.get()
                data-slot=description_state.slot_attr
                data-state=description_state.state_attr
                data-description-source=description_state.description_source_attr
            >
                {move || description_text.with_value(|value| value.clone())}
            </p>
        }
        .into_any()
    });

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
            {description_view}
        </div>
    }
    .into_any()
}

fn render_tray_body_slot(
    body_state: logic::TrayPartState,
    body_class: StoredValue<String>,
    children: StoredValue<ChildrenFn>,
) -> AnyView {
    view! {
        <div
            class=move || body_class.with_value(|class_name| class_name.clone())
            data-slot=body_state.slot_attr
            data-state=body_state.state_attr
        >
            {move || {
                let children = children.get_value();
                children()
            }}
        </div>
    }
    .into_any()
}

fn render_tray_footer_slot(
    show_footer: bool,
    footer_state: logic::TrayPartState,
    footer_class: StoredValue<String>,
    footer: StoredValue<Option<ViewFn>>,
) -> AnyView {
    if !show_footer {
        return ().into_any();
    }

    view! {
        <div
            class=move || footer_class.with_value(|class_name| class_name.clone())
            data-slot=footer_state.slot_attr
            data-state=footer_state.state_attr
            data-footer-source=footer_state.footer_source_attr
        >
            {move || footer.get_value().map(|slot| slot.run())}
        </div>
    }
    .into_any()
}

fn render_tray_panel(inputs: TrayPanelRenderInputs) -> AnyView {
    let TrayPanelRenderInputs {
        open,
        on_close,
        close_label,
        root_state,
        header_state,
        title_state,
        description_state,
        body_state,
        footer_state,
        close_state,
        root_class,
        header_class,
        title_class,
        description_class,
        body_class,
        footer_class,
        close_class,
        title_id_attr,
        description_id_attr,
        title,
        description_text,
        children,
        footer,
        panel_lang,
        panel_dir,
    } = inputs;

    let close_slot = render_tray_close_slot(
        root_state.show_close_button,
        close_state,
        close_class,
        close_label,
        on_close,
    );
    let header_slot = render_tray_header_slot(TrayHeaderRenderInputs {
        show_description: root_state.show_description,
        header_state,
        title_state,
        description_state,
        header_class,
        title_class,
        description_class,
        title_id_attr,
        description_id_attr,
        title,
        description_text,
    });
    let body_slot = render_tray_body_slot(body_state, body_class, children);
    let footer_slot =
        render_tray_footer_slot(root_state.show_footer, footer_state, footer_class, footer);

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
            {close_slot}
            {header_slot}
            {body_slot}
            {footer_slot}
        </div>
    }
    .into_any()
}

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
    #[prop(optional, default = logic::DEFAULT_CLOSE_LABEL)] close_label: &'static str,
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
    let state_inputs = logic::TrayStateInputs {
        description_mode: logic::TrayDescriptionMode::from_has_description(has_custom_description),
        footer_mode: logic::TrayFooterMode::from_has_footer(has_footer),
        close_button_mode: logic::TrayCloseButtonMode::from_show_close_button(show_close_button),
        size_mode: logic::TraySizeMode::from_is_fixed_height(is_fixed_height),
        dismiss_mode: logic::TrayDismissMode::from_is_dismissable(is_dismissable),
        keyboard_dismiss_mode: logic::TrayKeyboardDismissMode::from_is_disabled(
            is_keyboard_dismiss_disabled,
        ),
        has_custom_id_base,
        has_custom_title,
        has_custom_description,
        has_custom_class_name,
        has_custom_motion,
        has_on_exit_complete: on_exit_complete.is_some(),
    };
    let resolved_states = logic::resolve_states(state_inputs);
    let root_state = resolved_states.root_state;
    let header_state = resolved_states.header_state;
    let title_state = resolved_states.title_state;
    let description_state = resolved_states.description_state;
    let body_state = resolved_states.body_state;
    let footer_state = resolved_states.footer_state;
    let close_state = resolved_states.close_state;

    let root_class = StoredValue::new(logic::compose_class_name(class_name, root_state));
    let header_class = StoredValue::new(logic::compose_class_name(None, header_state));
    let title_class = StoredValue::new(logic::compose_class_name(None, title_state));
    let description_class = StoredValue::new(logic::compose_class_name(None, description_state));
    let body_class = StoredValue::new(logic::compose_class_name(None, body_state));
    let footer_class = StoredValue::new(logic::compose_class_name(None, footer_state));
    let close_class = StoredValue::new(logic::compose_class_name(None, close_state));

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
    let panel_aria_labelledby = logic::normalize_optional_attr(panel_a11y.aria_labelledby);
    let panel_aria_describedby = logic::normalize_optional_attr(panel_a11y.aria_describedby);
    let description_text = logic::normalize_optional_attr(description.get_value());
    let description_text = StoredValue::new(description_text);
    let panel_lang = StoredValue::new(panel_a11y.lang);
    let panel_dir = panel_a11y.dir;

    let on_exit_complete = logic::normalize_on_exit_complete(on_exit_complete);

    let panel = move || {
        render_tray_panel(TrayPanelRenderInputs {
            open,
            on_close,
            close_label,
            root_state,
            header_state,
            title_state,
            description_state,
            body_state,
            footer_state,
            close_state,
            root_class,
            header_class,
            title_class,
            description_class,
            body_class,
            footer_class,
            close_class,
            title_id_attr,
            description_id_attr,
            title,
            description_text,
            children,
            footer,
            panel_lang,
            panel_dir,
        })
    };

    view! {
        <Sheet
            open=open
            on_close=on_close
            placement=SheetPlacement::Bottom
            aria_labelledby=panel_aria_labelledby
            aria_describedby=panel_aria_describedby
            is_dismissable=is_dismissable
            is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled
            motion=motion.sheet
            on_exit_complete=on_exit_complete
        >
            {panel}
        </Sheet>
    }
    .into_any()
}
