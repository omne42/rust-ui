use super::*;
use ui_theme::default_text_field_motion_tokens;

#[test]
fn default_motion_is_theme_aligned() {
    let tokens = default_text_field_motion_tokens();
    assert_eq!(
        SidebarMotion::default(),
        SidebarMotion {
            duration_ms: tokens.duration_ms,
            reduced_duration_ms: 1,
        }
    );
}

#[test]
fn sanitize_motion_clamps_duration_values() {
    assert_eq!(
        sanitize_motion(SidebarMotion {
            duration_ms: 0,
            reduced_duration_ms: u16::MAX,
        }),
        SidebarMotion {
            duration_ms: 1,
            reduced_duration_ms: 5_000,
        }
    );
}

#[test]
fn attach_motion_emits_css_variable_contract() {
    let style = attach_motion(SidebarMotion {
        duration_ms: 320,
        reduced_duration_ms: 8,
    });
    let expected_runtime = if cfg!(target_arch = "wasm32") { 320 } else { 8 };

    assert!(style.contains("--ui-sidebar-motion-duration: 320ms;"));
    assert!(style.contains("--ui-sidebar-motion-reduced-duration: 8ms;"));
    assert!(style.contains(&format!(
        "--ui-sidebar-motion-runtime-duration: {expected_runtime}ms;"
    )));
}

#[test]
fn source_attr_tracks_default_vs_custom_motion() {
    assert_eq!(source_attr(SidebarMotion::default()), "default");
    assert_eq!(
        source_attr(SidebarMotion {
            duration_ms: 300,
            reduced_duration_ms: 1,
        }),
        "custom"
    );
}
