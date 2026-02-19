use crate::{
    OnPress,
    bottom_sheet::{BottomSheetMotion, BottomSheetStateInput, logic},
    button::{Button, ButtonSize, ButtonVariant},
    sheet::{Sheet, SheetPlacement},
};
use leptos::children::ViewFn;
use leptos::prelude::*;

#[component]
pub fn BottomSheet(
    open: Signal<bool>,
    on_close: OnPress,
    id_base: String,
    title: String,
    children: ChildrenFn,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] footer: Option<ViewFn>,
    #[prop(optional)] motion: BottomSheetMotion,
    #[prop(optional, default = true)] show_handle: bool,
    #[prop(optional, default = true)] show_close_button: bool,
    #[prop(optional, default = logic::DEFAULT_CLOSE_LABEL)] close_label: &'static str,
    #[prop(optional)] detached: bool,
    #[prop(optional)] bottom_inset_px: f64,
    #[prop(optional, default = true)] is_dismissable: bool,
    #[prop(optional)] is_keyboard_dismiss_disabled: bool,
    /// Called after the close animation finishes (useful for presence/unmount).
    #[prop(optional)]
    on_exit_complete: Option<Callback<()>>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let id_base = logic::normalize_id_base(id_base);
    let title = logic::normalize_required_text(title, "Bottom sheet");
    let description = logic::normalize_optional_text(description);
    let class_name = logic::normalize_optional_text(class_name);
    let bottom_inset_px = logic::normalize_bottom_inset_px(bottom_inset_px);

    let title = StoredValue::new(title);
    let description = StoredValue::new(description);
    let footer = StoredValue::new(footer);
    let children = StoredValue::new(children);
    let motion = crate::bottom_sheet::motion::sanitize_motion(motion);

    let state = logic::resolve_state(BottomSheetStateInput {
        has_description: description.get_value().is_some(),
        has_footer: footer.get_value().is_some(),
        show_handle,
        show_close_button,
        detached,
        bottom_inset_px,
        has_custom_class_name: class_name.is_some(),
    });
    let class = StoredValue::new(logic::compose_class_name(class_name, state));

    let title_id = format!("{id_base}-title");
    let description_id = format!("{id_base}-description");
    let title_id_attr: Signal<String> = title_id.clone().into();
    let description_id_attr: Signal<String> = description_id.clone().into();

    let on_exit_complete = on_exit_complete.unwrap_or_else(|| Callback::new(|_| {}));

    if state.show_description {
        view! {
            <Sheet
                open=open
                on_close=on_close
                placement=SheetPlacement::Bottom
                aria_labelledby=title_id.clone()
                aria_describedby=description_id.clone()
                is_dismissable=is_dismissable
                is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled
                motion=motion.sheet
                on_exit_complete=on_exit_complete
            >
                {move || {
                    let children = children.get_value();
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
                            data-motion-source=if motion == BottomSheetMotion::default() {
                                "default"
                            } else {
                                "custom"
                            }
                            data-custom-motion=(motion != BottomSheetMotion::default()).then_some("true")
                            data-class-source=state.class_source_attr
                        >
                            <Show when=move || state.show_handle>
                                <div class="ui-bottom-sheet__handle" data-slot="bottom-sheet-handle" aria-hidden="true">
                                    <span class="ui-bottom-sheet__handle-bar"></span>
                                </div>
                            </Show>

                            <Show when=move || state.show_close_button>
                                <span class="ui-bottom-sheet__close" data-slot="bottom-sheet-close">
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

                            <div class="ui-bottom-sheet__header" data-slot="bottom-sheet-header">
                                <h2
                                    class="ui-bottom-sheet__title"
                                    id=move || title_id_attr.get()
                                    data-slot="bottom-sheet-title"
                                >
                                    {move || title.get_value()}
                                </h2>

                                <p
                                    class="ui-bottom-sheet__description"
                                    id=move || description_id_attr.get()
                                    data-slot="bottom-sheet-description"
                                >
                                    {move || description.get_value().unwrap_or_default()}
                                </p>
                            </div>

                            <div class="ui-bottom-sheet__body" data-slot="bottom-sheet-body">
                                {children()}
                            </div>

                            <Show when=move || state.show_footer>
                                <div class="ui-bottom-sheet__footer" data-slot="bottom-sheet-footer">
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
                aria_labelledby=title_id.clone()
                is_dismissable=is_dismissable
                is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled
                motion=motion.sheet
                on_exit_complete=on_exit_complete
            >
                {move || {
                    let children = children.get_value();
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
                            data-motion-source=if motion == BottomSheetMotion::default() {
                                "default"
                            } else {
                                "custom"
                            }
                            data-custom-motion=(motion != BottomSheetMotion::default()).then_some("true")
                            data-class-source=state.class_source_attr
                        >
                            <Show when=move || state.show_handle>
                                <div class="ui-bottom-sheet__handle" data-slot="bottom-sheet-handle" aria-hidden="true">
                                    <span class="ui-bottom-sheet__handle-bar"></span>
                                </div>
                            </Show>

                            <Show when=move || state.show_close_button>
                                <span class="ui-bottom-sheet__close" data-slot="bottom-sheet-close">
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

                            <div class="ui-bottom-sheet__header" data-slot="bottom-sheet-header">
                                <h2
                                    class="ui-bottom-sheet__title"
                                    id=move || title_id_attr.get()
                                    data-slot="bottom-sheet-title"
                                >
                                    {move || title.get_value()}
                                </h2>
                            </div>

                            <div class="ui-bottom-sheet__body" data-slot="bottom-sheet-body">
                                {children()}
                            </div>

                            <Show when=move || state.show_footer>
                                <div class="ui-bottom-sheet__footer" data-slot="bottom-sheet-footer">
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
