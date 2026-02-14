use leptos::prelude::*;
use ui_theme::theme::ColorScheme;
use ui_theme::{Theme, css};

use ui_headless::{UiI18n, provide_ui_i18n};

#[derive(Clone, Copy)]
pub struct UiRootStateInput {
    pub theme: Theme,
    pub safe_area: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiRootState {
    pub theme_scheme_attr: &'static str,
    pub safe_area: bool,
    pub has_safe_area: bool,
}

pub fn resolve_theme_scheme(theme: Theme) -> &'static str {
    match theme.scheme {
        ColorScheme::Light => "light",
        ColorScheme::Dark => "dark",
    }
}

pub fn resolve_state(input: UiRootStateInput) -> UiRootState {
    UiRootState {
        theme_scheme_attr: resolve_theme_scheme(input.theme),
        safe_area: input.safe_area,
        has_safe_area: input.safe_area,
    }
}

#[component]
pub fn UiRoot(
    children: Children,
    #[prop(into)] theme: Signal<Theme>,
    #[prop(optional)] inject_components_css: bool,
    #[prop(optional)] safe_area: bool,
    #[prop(optional)] i18n: UiI18n,
) -> impl IntoView {
    provide_ui_i18n(i18n);
    let safe_area = StoredValue::new(safe_area);
    let inject_components_css = StoredValue::new(inject_components_css);

    let state = Memo::new(move |_| {
        resolve_state(UiRootStateInput {
            theme: theme.get(),
            safe_area: safe_area.get_value(),
        })
    });

    let css_text = Memo::new(move |_| {
        let state = state.get();

        let mut out = String::new();
        out.push_str(css::BASE_CSS);
        out.push_str(&theme.get().to_css_variables());
        if inject_components_css.get_value() {
            crate::css::push_components_css(&mut out);
        }
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
.ui-root {
  min-height: 100%;
}
.ui-root--safe-area,
.ui-root[data-safe-area="true"] {
  min-height: 100%;
}
"#,
        );
        if state.safe_area {
            out.push_str(css::SAFE_AREA_CSS);
        }
        out
    });

    view! {
        <>
            <style>{move || css_text.get()}</style>
            <div
                class="ui-root"
                class:safe-area=move || state.get().has_safe_area
                class:ui-root--safe-area=move || state.get().has_safe_area
                data-slot="ui-root"
                data-state=move || {
                    if state.get().has_safe_area {
                        "safe-area"
                    } else {
                        "default"
                    }
                }
                data-theme-scheme=move || state.get().theme_scheme_attr
                data-safe-area=move || state.get().has_safe_area.then_some("true")
            >
                {children()}
            </div>
        </>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_theme_scheme_maps_theme_color_scheme() {
        assert_eq!(resolve_theme_scheme(Theme::light()), "light");
        assert_eq!(resolve_theme_scheme(Theme::dark()), "dark");
        assert_eq!(resolve_theme_scheme(Theme::oled()), "dark");
    }

    #[test]
    fn resolve_state_tracks_safe_area_and_scheme() {
        let state = resolve_state(UiRootStateInput {
            theme: Theme::dark(),
            safe_area: true,
        });

        assert_eq!(state.theme_scheme_attr, "dark");
        assert!(state.safe_area);
        assert!(state.has_safe_area);
    }
}
