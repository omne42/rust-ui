use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DirectionMode {
    #[default]
    Ltr,
    Rtl,
}

impl DirectionMode {
    pub fn as_attr(self) -> &'static str {
        match self {
            DirectionMode::Ltr => "ltr",
            DirectionMode::Rtl => "rtl",
        }
    }
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

#[component]
pub fn DirectionProvider(
    #[prop(optional)] direction: Option<DirectionMode>,
    #[prop(optional)] dir: Option<DirectionMode>,
    #[prop(optional, into)] class_name: Option<String>,
    children: Children,
) -> impl IntoView {
    let direction = direction.or(dir).unwrap_or_default();
    let class_name = normalize_optional_text(class_name)
        .map(|class_name| format!("ui-direction-provider {class_name}"))
        .unwrap_or_else(|| "ui-direction-provider".to_string());

    view! {
        <div
            class=class_name
            dir=direction.as_attr()
            data-slot="direction-provider"
            data-direction=direction.as_attr()
        >
            {children()}
        </div>
    }
}
