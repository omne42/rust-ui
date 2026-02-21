use crate::{
    OnPress,
    bottom_sheet::{BottomSheetMotion, logic},
    button::{Button, ButtonSize, ButtonVariant},
    sheet::{Sheet, SheetPlacement},
};
use leptos::children::ViewFn;
use leptos::prelude::*;
use ui_headless::{A11yDirection, UiTraceEventKind, use_ui_trace};

const CLOSE_ICON_VIEWBOX: &str = "0 0 20 20";
const CLOSE_ICON_FILL: &str = "none";
const CLOSE_ICON_PATH_D: &str = "M5 5l10 10M15 5L5 15";
const CLOSE_ICON_STROKE_WIDTH: &str = "1.5";

#[derive(Clone)]
struct BottomSheetContentInput {
    class: StoredValue<String>,
    state: logic::BottomSheetState,
    motion_source_attr: &'static str,
    has_custom_motion: bool,
    agent_contract: Signal<logic::BottomSheetAgentContract>,
    title: StoredValue<String>,
    title_id_attr: Signal<String>,
    description: StoredValue<Option<String>>,
    description_id_attr: Signal<String>,
    footer: StoredValue<Option<ViewFn>>,
    children: StoredValue<ChildrenFn>,
    close_label: &'static str,
    on_close: OnPress,
}

fn render_bottom_sheet_close_icon() -> impl IntoView {
    view! {
        <svg viewBox=CLOSE_ICON_VIEWBOX fill=CLOSE_ICON_FILL aria-hidden="true">
            <path
                d=CLOSE_ICON_PATH_D
                stroke="currentColor"
                stroke_width=CLOSE_ICON_STROKE_WIDTH
                stroke_linecap="round"
                stroke_linejoin="round"
            />
        </svg>
    }
}

fn render_bottom_sheet_handle(show_handle: bool) -> impl IntoView {
    view! {
        <Show when=move || show_handle>
            <div class="ui-bottom-sheet__handle" data-slot="bottom-sheet-handle" aria-hidden="true">
                <span class="ui-bottom-sheet__handle-bar"></span>
            </div>
        </Show>
    }
}

fn render_bottom_sheet_close_button(
    show_close_button: bool,
    close_label: &'static str,
    on_close: OnPress,
) -> impl IntoView {
    view! {
        <Show when=move || show_close_button>
            <span class="ui-bottom-sheet__close" data-slot="bottom-sheet-close">
                <Button
                    aria_label=close_label
                    variant=ButtonVariant::Ghost
                    size=ButtonSize::IconSm
                    on_press=on_close
                >
                    {render_bottom_sheet_close_icon()}
                </Button>
            </span>
        </Show>
    }
}

fn render_bottom_sheet_header(
    title: StoredValue<String>,
    title_id_attr: Signal<String>,
    description: StoredValue<Option<String>>,
    description_id_attr: Signal<String>,
    show_description: bool,
) -> impl IntoView {
    view! {
        <div class="ui-bottom-sheet__header" data-slot="bottom-sheet-header">
            <h2 class="ui-bottom-sheet__title" id=move || title_id_attr.get() data-slot="bottom-sheet-title">
                {move || title.get_value()}
            </h2>

            <Show when=move || show_description>
                <p
                    class="ui-bottom-sheet__description"
                    id=move || description_id_attr.get()
                    data-slot="bottom-sheet-description"
                >
                    {move || logic::resolve_description_text(description.get_value())}
                </p>
            </Show>
        </div>
    }
}

fn render_bottom_sheet_body(children: StoredValue<ChildrenFn>) -> impl IntoView {
    view! {
        <div class="ui-bottom-sheet__body" data-slot="bottom-sheet-body">
            {children.with_value(|children| children())}
        </div>
    }
}

fn render_bottom_sheet_footer(
    show_footer: bool,
    footer: StoredValue<Option<ViewFn>>,
) -> impl IntoView {
    view! {
        <Show when=move || show_footer>
            <div class="ui-bottom-sheet__footer" data-slot="bottom-sheet-footer">
                {move || footer.get_value().map(|slot| slot.run())}
            </div>
        </Show>
    }
}

fn render_bottom_sheet_content(input: BottomSheetContentInput) -> impl IntoView {
    let BottomSheetContentInput {
        class,
        state,
        motion_source_attr,
        has_custom_motion,
        agent_contract,
        title,
        title_id_attr,
        description,
        description_id_attr,
        footer,
        children,
        close_label,
        on_close,
    } = input;

    view! {
        <div
            class=move || class.get_value()
            data-slot="bottom-sheet"
            data-state=state.state_attr
            data-description=state.description_attr
            data-footer=state.footer_attr
            data-handle=state.handle_attr
            data-close-button=state.close_button_attr
            data-detached=state.detached_attr
            data-bottom-inset=state.inset_attr
            data-with-description=state.show_description.then_some("true")
            data-with-footer=state.show_footer.then_some("true")
            data-handle-visible=state.show_handle.then_some("true")
            data-close-visible=state.show_close_button.then_some("true")
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-motion-source=motion_source_attr
            data-custom-motion=has_custom_motion.then_some("true")
            data-class-source=state.class_source_attr
            data-ui-schema=move || agent_contract.get().schema_name
            data-ui-schema-version=move || agent_contract.get().schema_version.as_str()
            data-ui-intent=move || agent_contract.get().intent.as_str()
            data-ui-action=move || agent_contract.get().action.as_str()
            data-ui-state=move || agent_contract.get().state.as_str()
            data-ui-source=move || agent_contract.get().source.as_str()
            data-ui-output-status=move || agent_contract.get().output_status.as_str()
            data-ui-stream-support=move || agent_contract.get().stream_support.as_str()
            data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()
            data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()
            data-ui-render-policy=move || agent_contract.get().render_policy.as_str()
        >
            {render_bottom_sheet_handle(state.show_handle)}
            {render_bottom_sheet_close_button(state.show_close_button, close_label, on_close)}
            {render_bottom_sheet_header(
                title,
                title_id_attr,
                description,
                description_id_attr,
                state.show_description,
            )}
            {render_bottom_sheet_body(children)}
            {render_bottom_sheet_footer(state.show_footer, footer)}
        </div>
    }
}

#[component]
pub fn BottomSheet(
    open: Signal<bool>,
    on_close: OnPress,
    id_base: String,
    title: String,
    children: ChildrenFn,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] footer: Option<ViewFn>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] motion: BottomSheetMotion,
    #[prop(optional)] is_handle_visible: Option<bool>,
    #[prop(optional)] is_close_button_visible: Option<bool>,
    #[prop(optional)] close_label: Option<&'static str>,
    #[prop(optional)] is_detached: Option<bool>,
    #[prop(optional)] bottom_inset_px: Option<f64>,
    #[prop(optional)] is_dismissable: Option<bool>,
    #[prop(optional)] is_keyboard_dismiss_disabled: Option<bool>,
    #[prop(optional)] show_handle: Option<bool>,
    #[prop(optional)] show_close_button: Option<bool>,
    #[prop(optional)] detached: Option<bool>,
    /// Called after the close animation finishes (useful for presence/unmount).
    #[prop(optional)]
    on_exit_complete: Option<Callback<()>>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let title = logic::resolve_title(title);
    let description = logic::normalize_optional_text(description);
    let lang = logic::normalize_optional_text(lang);
    let class_name = logic::normalize_optional_text(class_name);
    let close_label = logic::resolve_close_label(close_label);
    let bottom_inset_px = logic::resolve_bottom_inset_px(bottom_inset_px);
    let handle_visibility = logic::resolve_handle_visibility(is_handle_visible, show_handle);
    let close_button_visibility =
        logic::resolve_close_button_visibility(is_close_button_visible, show_close_button);
    let attachment = logic::resolve_attachment(is_detached, detached);
    let is_dismissable = logic::resolve_dismissable(is_dismissable);
    let is_keyboard_dismiss_disabled =
        logic::resolve_keyboard_dismiss_disabled(is_keyboard_dismiss_disabled);

    let title = StoredValue::new(title);
    let description = StoredValue::new(description);
    let footer = StoredValue::new(footer);
    let children = StoredValue::new(children);
    let motion = crate::bottom_sheet::motion::sanitize_motion(motion);
    let has_custom_motion = motion != BottomSheetMotion::default();

    let derived_state = logic::derive_view_state(logic::BottomSheetDeriveInput {
        has_description: logic::has_slot(&description.get_value()),
        has_footer: logic::has_slot(&footer.get_value()),
        handle_visibility,
        close_button_visibility,
        attachment,
        bottom_inset_px,
        has_custom_class_name: class_name.is_some(),
        has_custom_motion,
    });
    let state = derived_state.state;
    let class = StoredValue::new(logic::compose_class_name(class_name, state));

    let title_id = format!("{id_base}-title");
    let description_id = format!("{id_base}-description");
    let title_id_attr: Signal<String> = title_id.clone().into();
    let description_id_attr: Signal<String> = description_id.clone().into();
    let lang = lang.unwrap_or_default();

    let on_exit_complete = logic::resolve_on_exit_complete(on_exit_complete);
    let trace = use_ui_trace();
    let last_open = RwSignal::new(open.get_untracked());
    Effect::new(move |_| {
        let next_open = open.get();
        let prev_open = last_open.get_untracked();
        if next_open == prev_open {
            return;
        }
        if let Some(trace) = trace {
            trace.emit(
                "bottom-sheet",
                UiTraceEventKind::OpenChange { open: next_open },
            );
        }
        last_open.set(next_open);
    });

    let agent_contract = Signal::derive(move || {
        logic::resolve_agent_contract(logic::BottomSheetAgentContractInput {
            is_open: open.get(),
            show_description: state.show_description,
            show_footer: state.show_footer,
            detached: state.detached,
            is_dismissable,
            is_keyboard_dismiss_disabled,
            motion_source_attr: derived_state.motion_source_attr,
        })
    });

    let content_input = StoredValue::new(BottomSheetContentInput {
        class,
        state,
        motion_source_attr: derived_state.motion_source_attr,
        has_custom_motion: derived_state.has_custom_motion,
        agent_contract,
        title,
        title_id_attr,
        description,
        description_id_attr,
        footer,
        children,
        close_label,
        on_close,
    });

    if state.show_description {
        if let Some(dir) = dir {
            view! {
                <Sheet
                    open=open
                    on_close=on_close
                    placement=SheetPlacement::Bottom
                    aria_labelledby=title_id.clone()
                    aria_describedby=description_id.clone()
                    lang=lang.clone()
                    dir=dir
                    is_dismissable=is_dismissable
                    is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled
                    motion=motion.sheet
                    on_exit_complete=on_exit_complete
                >
                    {move || {
                        content_input
                            .with_value(|input| render_bottom_sheet_content(input.clone()))
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
                    aria_labelledby=title_id.clone()
                    aria_describedby=description_id.clone()
                    lang=lang.clone()
                    is_dismissable=is_dismissable
                    is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled
                    motion=motion.sheet
                    on_exit_complete=on_exit_complete
                >
                    {move || {
                        content_input
                            .with_value(|input| render_bottom_sheet_content(input.clone()))
                    }}
                </Sheet>
            }
            .into_any()
        }
    } else if let Some(dir) = dir {
        view! {
            <Sheet
                open=open
                on_close=on_close
                placement=SheetPlacement::Bottom
                aria_labelledby=title_id.clone()
                lang=lang.clone()
                dir=dir
                is_dismissable=is_dismissable
                is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled
                motion=motion.sheet
                on_exit_complete=on_exit_complete
            >
                {move || {
                    content_input.with_value(|input| render_bottom_sheet_content(input.clone()))
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
                aria_labelledby=title_id.clone()
                lang=lang
                is_dismissable=is_dismissable
                is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled
                motion=motion.sheet
                on_exit_complete=on_exit_complete
            >
                {move || {
                    content_input.with_value(|input| render_bottom_sheet_content(input.clone()))
                }}
            </Sheet>
        }
        .into_any()
    }
}
