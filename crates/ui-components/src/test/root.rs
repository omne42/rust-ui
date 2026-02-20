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
