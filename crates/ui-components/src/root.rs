use leptos::prelude::*;
use ui_theme::{css, Theme};

#[component]
pub fn UiRoot(
    children: Children,
    #[prop(into)] theme: Signal<Theme>,
    #[prop(optional)] safe_area: bool,
) -> impl IntoView {
    let safe_area = StoredValue::new(safe_area);

    let css_text = Memo::new(move |_| {
        let mut out = String::new();
        out.push_str(css::BASE_CSS);
        out.push_str(&theme.get().to_css_variables());
        out.push_str(
            r#"
html, body { height: 100%; }
body {
  margin: 0;
  background: var(--ui-bg);
  color: var(--ui-fg);
  font-family: system-ui, -apple-system, Segoe UI, Roboto, sans-serif;
}
.ui-muted { color: var(--ui-fg-muted); }
"#,
        );
        if safe_area.get_value() {
            out.push_str(css::SAFE_AREA_CSS);
        }
        out
    });

    view! {
        <>
            <style>{move || css_text.get()}</style>
            <div class="ui-root" class:safe-area=move || safe_area.get_value()>
                {children()}
            </div>
        </>
    }
}
