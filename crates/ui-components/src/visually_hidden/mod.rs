use leptos::prelude::*;

#[component]
pub fn VisuallyHidden(
    children: Children,
    #[prop(optional)] focusable: bool,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let class_name = class_name.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    });
    let has_custom_class_name = class_name.is_some();

    let mut class = String::from("ui-visually-hidden");
    if focusable {
        class.push_str(" ui-visually-hidden--focusable");
    }
    if let Some(custom_class_name) = class_name {
        class.push(' ');
        class.push_str(&custom_class_name);
    }

    view! {
        <span
            class=class
            data-slot="visually-hidden"
            data-focusable=focusable.then_some("true")
            data-custom-class=has_custom_class_name.then_some("true")
        >
            {children()}
        </span>
    }
}

pub const CSS: &str = r#"
.ui-visually-hidden {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0 0 0 0);
  clip-path: inset(50%);
  white-space: nowrap;
  border: 0;
}

.ui-visually-hidden--focusable:active,
.ui-visually-hidden--focusable:focus-within {
  position: static;
  width: auto;
  height: auto;
  padding: 0;
  margin: 0;
  overflow: visible;
  clip: auto;
  clip-path: none;
  white-space: normal;
}
"#;
