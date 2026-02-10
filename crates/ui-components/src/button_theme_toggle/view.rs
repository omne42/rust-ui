use crate::button_theme_toggle::{ThemeMode, ThemeToggleMotion, logic, motion};
use crate::{Button, ButtonSize, ButtonVariant, OnPress};
use leptos::{html, prelude::*};

fn icon_view(icon: logic::ThemeToggleIcon) -> impl IntoView {
    match icon {
        logic::ThemeToggleIcon::Sun => view! {
            <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
                <circle cx="12" cy="12" r="4" stroke="currentColor" stroke_width="1.5" />
                <path
                    d="M12 2.5v2.5M12 19v2.5M2.5 12H5M19 12h2.5M4.7 4.7l1.8 1.8M17.5 17.5l1.8 1.8M19.3 4.7l-1.8 1.8M6.5 17.5l-1.8 1.8"
                    stroke="currentColor"
                    stroke_width="1.5"
                    stroke_linecap="round"
                />
            </svg>
        }
        .into_any(),
        logic::ThemeToggleIcon::Moon => view! {
            <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
                <path
                    d="M20 15.5a8.2 8.2 0 0 1-11.3-11 8.2 8.2 0 1 0 11.3 11"
                    stroke="currentColor"
                    stroke_width="1.5"
                    stroke_linecap="round"
                    stroke_linejoin="round"
                />
            </svg>
        }
        .into_any(),
        logic::ThemeToggleIcon::Oled => view! {
            <svg viewBox="0 0 24 24" fill="none" aria-hidden="true">
                <rect
                    x="4"
                    y="5"
                    width="16"
                    height="14"
                    rx="3"
                    stroke="currentColor"
                    stroke_width="1.5"
                />
                <path
                    d="M8 16h8"
                    stroke="currentColor"
                    stroke_width="1.5"
                    stroke_linecap="round"
                />
            </svg>
        }
        .into_any(),
    }
}

#[component]
pub fn ThemeToggleButton(
    mode: ReadSignal<ThemeMode>,
    set_mode: WriteSignal<ThemeMode>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] modes: Vec<ThemeMode>,
    #[prop(optional, default = ButtonVariant::Ghost)] variant: ButtonVariant,
    #[prop(optional, default = ButtonSize::IconSm)] size: ButtonSize,
    #[prop(optional)] motion: ThemeToggleMotion,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let has_custom_modes = !modes.is_empty();
    let modes = logic::normalize_modes(modes);
    let modes = StoredValue::new(modes);

    let class_name = logic::normalize_optional_text(class_name);
    let has_custom_class_name = class_name.is_some();

    let aria_label = logic::normalize_optional_text(aria_label);
    let has_custom_aria_label = aria_label.is_some();

    let on_press: OnPress = Callback::new(move |_| {
        if disabled {
            return;
        }
        let current = mode.get_untracked();
        let next = logic::resolve_next(current, &modes.get_value());
        set_mode.set(next);
    });

    let class = logic::compose_class_name(
        class_name,
        logic::resolve_state(
            mode.get_untracked(),
            &modes.get_value(),
            disabled,
            has_custom_modes,
            has_custom_aria_label,
            has_custom_class_name,
        ),
    );

    let aria_label = aria_label.unwrap_or_else(|| "Toggle theme".to_string());

    let state = Memo::new(move |_| {
        logic::resolve_state(
            mode.get(),
            &modes.get_value(),
            disabled,
            has_custom_modes,
            has_custom_aria_label,
            has_custom_class_name,
        )
    });

    let icon_ref: NodeRef<html::Span> = NodeRef::new();
    motion::attach_motion(icon_ref, mode.into(), motion);

    view! {
        <Button
            aria_label=aria_label
            class_name=class
            variant=variant
            size=size
            disabled=disabled
            on_press=on_press
        >
            <span
                class="ui-theme-toggle-button__icon"
                data-slot="theme-toggle-icon"
                data-state=move || if state.get().is_disabled { "disabled" } else { "enabled" }
                data-current-mode=move || state.get().current_mode_attr
                data-next-mode=move || state.get().next_mode_attr
                data-mode-count=move || state.get().mode_count.to_string()
                data-custom-modes=move || state.get().has_custom_modes.then_some("true")
                data-custom-aria-label=move || state.get().has_custom_aria_label.then_some("true")
                data-motion-source=if motion == ThemeToggleMotion::default() {
                    "default"
                } else {
                    "custom"
                }
                data-custom-motion=(motion != ThemeToggleMotion::default()).then_some("true")
                node_ref=icon_ref
            >
                {move || {
                    let view_state = logic::resolve_view_state(mode.get(), &modes.get_value());
                    icon_view(view_state.icon)
                }}
            </span>
        </Button>
    }
}
