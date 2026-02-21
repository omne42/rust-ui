use std::borrow::Cow;

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

fn normalize_class_name(class_name: Option<Cow<'static, str>>) -> Option<Cow<'static, str>> {
    class_name.and_then(|value| match value {
        Cow::Borrowed(value) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(Cow::Borrowed(trimmed))
            }
        }
        Cow::Owned(mut value) => {
            let trimmed_end_len = value.trim_end().len();
            value.truncate(trimmed_end_len);

            let leading_whitespace = value.len() - value.trim_start().len();
            if leading_whitespace > 0 {
                value.drain(..leading_whitespace);
            }

            if value.is_empty() {
                None
            } else {
                Some(Cow::Owned(value))
            }
        }
    })
}

#[component]
pub fn Button(
    #[prop(optional, into)] class_name: Option<Cow<'static, str>>,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(into)] aria_label: String,
    on_press: Callback<()>,
    children: Children,
) -> impl IntoView {
    let class_name = normalize_class_name(class_name);

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
