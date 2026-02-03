use leptos::prelude::*;

#[component]
pub fn CircularProgress(
    #[prop(optional, into, default = "Loading".to_string())] aria_label: String,
    #[prop(optional)] size_px: Option<f64>,
    #[prop(optional)] thickness_px: Option<f64>,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let vars = {
        let mut out = String::new();
        if let Some(size_px) = size_px {
            out.push_str(&format!("--ui-cp-size: {size_px}px;"));
        }
        if let Some(thickness_px) = thickness_px {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(&format!("--ui-cp-thickness: {thickness_px}px;"));
        }
        out
    };

    let base_class = "ui-circular-progress".to_string();
    let class = class_name
        .filter(|value| !value.trim().is_empty())
        .map(|value| format!("{base_class} {value}"))
        .unwrap_or(base_class);

    view! {
        <span
            class=class
            style=vars
            data-slot="circular-progress"
            role="status"
            aria-label=aria_label
        ></span>
    }
}
