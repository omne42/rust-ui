use ui_theme::default_text_field_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResizableMotion {
    pub enabled: bool,
    pub panel_duration_ms: u32,
    pub handle_duration_ms: u32,
}

impl Default for ResizableMotion {
    fn default() -> Self {
        let tokens = default_text_field_motion_tokens();
        let duration_ms = u32::from(tokens.duration_ms);
        Self {
            enabled: true,
            panel_duration_ms: duration_ms,
            handle_duration_ms: duration_ms,
        }
    }
}

impl ResizableMotion {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }
}

fn sanitize_duration_ms(duration_ms: u32) -> u32 {
    duration_ms.clamp(40, 1_000)
}

pub fn sanitize_motion(motion: ResizableMotion) -> ResizableMotion {
    ResizableMotion {
        enabled: motion.enabled,
        panel_duration_ms: sanitize_duration_ms(motion.panel_duration_ms),
        handle_duration_ms: sanitize_duration_ms(motion.handle_duration_ms),
    }
}

pub fn motion_style_vars(motion: ResizableMotion) -> String {
    let motion = sanitize_motion(motion);
    let tokens = default_text_field_motion_tokens();
    let panel_duration_ms = if motion.enabled {
        motion.panel_duration_ms
    } else {
        0
    };
    let handle_duration_ms = if motion.enabled {
        motion.handle_duration_ms
    } else {
        0
    };
    format!(
        "--ui-resizable-panel-duration: {}ms; --ui-resizable-handle-duration: {}ms; --ui-resizable-motion-easing: {};",
        panel_duration_ms, handle_duration_ms, tokens.easing
    )
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_dragging: leptos::prelude::Signal<bool>,
    motion: ResizableMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = sanitize_motion(motion);

    Effect::new(move |_| {
        let Some(node) = node_ref.get() else {
            return;
        };
        let element: leptos::web_sys::HtmlElement = node.unchecked_into();
        let style = element.style();

        let disable_motion = !motion.enabled || ui_motion::web::prefers_reduced_motion();
        if disable_motion || is_dragging.get() {
            let _ = style.set_property("--ui-resizable-runtime-panel-duration", "1ms");
            let _ = style.set_property("--ui-resizable-runtime-handle-duration", "1ms");
        } else {
            let _ = style.remove_property("--ui-resizable-runtime-panel-duration");
            let _ = style.remove_property("--ui-resizable-runtime-handle-duration");
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _is_dragging: leptos::prelude::Signal<bool>,
    motion: ResizableMotion,
) {
    let _ = sanitize_motion(motion);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_comes_from_theme_tokens() {
        let motion = ResizableMotion::default();
        let tokens = default_text_field_motion_tokens();
        let duration_ms = u32::from(tokens.duration_ms);
        assert_eq!(motion.panel_duration_ms, duration_ms);
        assert_eq!(motion.handle_duration_ms, duration_ms);
        assert!(motion.enabled);
    }

    #[test]
    fn sanitize_motion_clamps_durations() {
        let motion = sanitize_motion(ResizableMotion {
            enabled: true,
            panel_duration_ms: 0,
            handle_duration_ms: 99_999,
        });
        assert_eq!(motion.panel_duration_ms, 40);
        assert_eq!(motion.handle_duration_ms, 1_000);
    }

    #[test]
    fn motion_style_vars_uses_theme_easing_and_durations() {
        let style = motion_style_vars(ResizableMotion {
            enabled: true,
            panel_duration_ms: 240,
            handle_duration_ms: 120,
        });

        assert!(style.contains("--ui-resizable-panel-duration: 240ms;"));
        assert!(style.contains("--ui-resizable-handle-duration: 120ms;"));
        assert!(style.contains("--ui-resizable-motion-easing: cubic-bezier(0.2, 0, 0, 1);"));
    }
}
