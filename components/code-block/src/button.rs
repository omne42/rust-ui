use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Ghost,
}

impl ButtonVariant {
    fn class_name(self) -> &'static str {
        match self {
            ButtonVariant::Ghost => "ui-code-block__button--ghost",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[default]
    IconSm,
}

impl ButtonSize {
    fn class_name(self) -> &'static str {
        match self {
            ButtonSize::IconSm => "ui-code-block__button--icon-sm",
        }
    }
}

#[component]
pub fn Button(
    #[prop(optional, into)] class_name: Option<String>,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(into)] aria_label: String,
    on_press: Callback<()>,
    children: Children,
) -> impl IntoView {
    let class_name = class_name
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    let class_name = if let Some(class_name) = class_name {
        format!(
            "ui-code-block__button {} {} {}",
            variant.class_name(),
            size.class_name(),
            class_name
        )
    } else {
        format!(
            "ui-code-block__button {} {}",
            variant.class_name(),
            size.class_name()
        )
    };

    view! {
        <button
            type="button"
            class=class_name
            aria-label=aria_label
            on:click=move |_| on_press.run(())
        >
            {children()}
        </button>
    }
}
