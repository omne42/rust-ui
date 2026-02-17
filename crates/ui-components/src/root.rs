use leptos::prelude::*;
use ui_theme::{SemanticOverrides, Theme, css};

use ui_headless::{UiI18n, provide_ui_i18n};

#[derive(Clone, Copy)]
pub struct UiRootStateInput {
    pub theme: Theme,
    pub safe_area: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiRootState {
    pub theme_scheme_attr: &'static str,
    pub theme_color_attr: &'static str,
    pub theme_system_attr: &'static str,
    pub theme_scale_attr: &'static str,
    pub safe_area: bool,
    pub has_safe_area: bool,
}

pub fn resolve_theme_scheme(theme: Theme) -> &'static str {
    theme.ctx.color.css_color_scheme()
}

pub fn resolve_theme_color(theme: Theme) -> &'static str {
    theme.ctx.color.as_str()
}

pub fn resolve_theme_system(theme: Theme) -> &'static str {
    theme.ctx.system.as_str()
}

pub fn resolve_theme_scale(theme: Theme) -> &'static str {
    theme.ctx.scale.as_str()
}

pub fn resolve_state(input: UiRootStateInput) -> UiRootState {
    UiRootState {
        theme_scheme_attr: resolve_theme_scheme(input.theme),
        theme_color_attr: resolve_theme_color(input.theme),
        theme_system_attr: resolve_theme_system(input.theme),
        theme_scale_attr: resolve_theme_scale(input.theme),
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
    #[prop(optional)] semantic_overrides: Option<SemanticOverrides>,
    #[prop(optional)] i18n: UiI18n,
) -> impl IntoView {
    provide_ui_i18n(i18n);
    let safe_area = StoredValue::new(safe_area);
    let inject_components_css = StoredValue::new(inject_components_css);
    let semantic_overrides = StoredValue::new(semantic_overrides);

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
        if let Some(overrides) = semantic_overrides.get_value() {
            out.push_str(&overrides.to_css_block(":root"));
        }
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
                data-theme-color=move || state.get().theme_color_attr
                data-theme-system=move || state.get().theme_system_attr
                data-theme-scale=move || state.get().theme_scale_attr
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
    fn resolve_theme_axes_are_stable_strings() {
        let theme = Theme::light();
        assert_eq!(resolve_theme_color(theme), "light");
        assert_eq!(resolve_theme_system(theme), "baseline-two");
        assert_eq!(resolve_theme_scale(theme), "medium");
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
