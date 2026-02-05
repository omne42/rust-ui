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
        if let Some(size_px) = size_px.filter(|value| value.is_finite() && *value > 0.0) {
            out.push_str(&format!("--ui-cp-size: {size_px}px;"));
        }
        if let Some(thickness_px) = thickness_px.filter(|value| value.is_finite() && *value > 0.0) {
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
            role="progressbar"
            aria-label=aria_label
            aria-valuemin="0"
            aria-valuemax="100"
        ></span>
    }
}
