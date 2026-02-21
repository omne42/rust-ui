use ui_theme::default_drop_zone_motion_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DropZoneMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub hover_scale: f64,
    pub drop_scale: f64,
    pub hover_highlight: f64,
}

impl Default for DropZoneMotion {
    fn default() -> Self {
        let tokens = default_drop_zone_motion_tokens();
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: tokens.spring.stiffness,
                damping: tokens.spring.damping,
                mass: tokens.spring.mass,
                precision: tokens.spring.precision,
            },
            hover_scale: tokens.hover_scale,
            drop_scale: tokens.drop_scale,
            hover_highlight: tokens.hover_highlight,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    ui_motion::spring::sanitize_config(value, DropZoneMotion::default().spring)
}

pub fn sanitize_motion(motion: DropZoneMotion) -> DropZoneMotion {
    let default = DropZoneMotion::default();

    DropZoneMotion {
        spring: sanitize_spring(motion.spring),
        hover_scale: sanitize_number(motion.hover_scale, default.hover_scale).clamp(0.0, 3.0),
        drop_scale: sanitize_number(motion.drop_scale, default.drop_scale).clamp(0.0, 3.0),
        hover_highlight: sanitize_number(motion.hover_highlight, default.hover_highlight)
            .clamp(0.0, 1.0),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_hovered: leptos::prelude::ReadSignal<bool>,
    is_drop_target: leptos::prelude::ReadSignal<bool>,
    is_focused: leptos::prelude::ReadSignal<bool>,
    is_disabled: bool,
    motion: DropZoneMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    if is_disabled {
        return;
    }

    let motion = StoredValue::new(sanitize_motion(motion));
    if ui_motion::web::prefers_reduced_motion() {
        Effect::new(move |_| {
            let Some(div) = node_ref.get() else {
                return;
            };
            let element: leptos::web_sys::HtmlElement = div.unchecked_into();
            let style = element.style();

            let motion = motion.get_value();
            let hovered = is_hovered.get();
            let drop_target = is_drop_target.get();
            let focused = is_focused.get();

            let scale_target = if drop_target {
                motion.drop_scale
            } else if hovered || focused {
                motion.hover_scale
            } else {
                1.0
            };
            let highlight_target = if drop_target {
                1.0
            } else if hovered {
                motion.hover_highlight
            } else {
                0.0
            };

            ui_observability::set_css_property_observed_auto!(
                &(style),
                "--ui-drop-zone-scale",
                &format!("{scale_target}")
            );
            ui_observability::set_css_property_observed_auto!(
                &(style),
                "--ui-drop-zone-highlight",
                &format!("{highlight_target}")
            );
        });
        return;
    }

    let last_state = StoredValue::new(None::<(bool, bool, bool)>);
    let springs = StoredValue::new_local(
        None::<(
            ui_motion::spring::SpringAnimator,
            ui_motion::spring::SpringAnimator,
        )>,
    );

    Effect::new(move |_| {
        let config = motion.get_value().spring;
        let Some(div) = node_ref.get() else {
            return;
        };
        if springs.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = div.unchecked_into();
        let style = element.style();

        ui_observability::set_css_property_observed_auto!(&(style), "--ui-drop-zone-scale", "1");
        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-drop-zone-highlight",
            "0"
        );
        let style_for_scale = style.clone();
        let scale = ui_motion::spring::SpringAnimator::new(1.0, config, move |v| {
            let v = v.clamp(0.0, 10.0);
            ui_observability::set_css_property_observed_auto!(
                &(style_for_scale),
                "--ui-drop-zone-scale",
                &format!("{v}")
            );
        });

        let style_for_highlight = style.clone();
        let highlight = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(0.0, 1.0);
            ui_observability::set_css_property_observed_auto!(
                &(style_for_highlight),
                "--ui-drop-zone-highlight",
                &format!("{v}")
            );
        });

        let springs_for_cleanup = springs;
        on_cleanup(move || {
            if let Some((scale, highlight)) = springs_for_cleanup.get_value() {
                scale.stop();
                highlight.stop();
            }
        });

        springs.set_value(Some((scale, highlight)));
    });

    Effect::new(move |_| {
        let hovered = is_hovered.get();
        let drop_target = is_drop_target.get();
        let focused = is_focused.get();

        let Some(prev) = last_state.get_value() else {
            last_state.set_value(Some((hovered, drop_target, focused)));
            return;
        };
        if prev == (hovered, drop_target, focused) {
            return;
        }
        last_state.set_value(Some((hovered, drop_target, focused)));

        let Some((scale, highlight)) = springs.get_value() else {
            return;
        };

        let motion = motion.get_value();
        let scale_target = if drop_target {
            motion.drop_scale
        } else if hovered || focused {
            motion.hover_scale
        } else {
            1.0
        };
        let highlight_target = if drop_target {
            1.0
        } else if hovered {
            motion.hover_highlight
        } else {
            0.0
        };

        scale.set_target(scale_target);
        highlight.set_target(highlight_target);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _is_hovered: leptos::prelude::ReadSignal<bool>,
    _is_drop_target: leptos::prelude::ReadSignal<bool>,
    _is_focused: leptos::prelude::ReadSignal<bool>,
    _is_disabled: bool,
    motion: DropZoneMotion,
) {
    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
