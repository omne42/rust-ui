use ui_headless::TooltipPlacement;
use ui_theme::default_overlay_layout_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TooltipMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_scale: f64,
    pub offset_y_px: f64,
}

impl Default for TooltipMotion {
    fn default() -> Self {
        let overlay = default_overlay_layout_tokens();
        Self {
            spring: ui_motion::presets::spring_soft(),
            initial_scale: overlay.enter_scale,
            offset_y_px: f64::from(overlay.enter_offset_y_px),
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = TooltipMotion::default().spring;
    ui_motion::spring::sanitize_config(value, default)
}

pub fn sanitize_motion(motion: TooltipMotion) -> TooltipMotion {
    let default = TooltipMotion::default();

    TooltipMotion {
        spring: sanitize_spring(motion.spring),
        initial_scale: sanitize_number(motion.initial_scale, default.initial_scale).clamp(0.0, 3.0),
        offset_y_px: sanitize_number(motion.offset_y_px, default.offset_y_px)
            .abs()
            .clamp(0.0, 320.0),
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn placement_offset_y(placement: TooltipPlacement, base: f64) -> f64 {
    if placement.is_bottom() {
        base.abs()
    } else {
        -base.abs()
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    placement: leptos::prelude::Signal<TooltipPlacement>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: TooltipMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(sanitize_motion(motion));
    let last_state = StoredValue::new(None::<bool>);
    let springs = StoredValue::new_local(
        None::<(
            ui_motion::spring::SpringAnimator,
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
        let motion = motion.get_value();
        let offset_y = placement_offset_y(placement.get_untracked(), motion.offset_y_px);

        let open_now = is_open.get_untracked();
        // Always initialize in the closed state so mounting while open animates in.
        let opacity_initial = 0.0;
        let scale_initial = motion.initial_scale;
        let y_initial = offset_y;

        let _ = style.set_property("--ui-tooltip-opacity", &format!("{opacity_initial}"));
        let _ = style.set_property("--ui-tooltip-scale", &format!("{scale_initial}"));
        let _ = style.set_property("--ui-tooltip-y", &format!("{y_initial}px"));

        let style_for_opacity = style.clone();
        let opacity = ui_motion::spring::SpringAnimator::new(opacity_initial, config, move |v| {
            let v = v.clamp(0.0, 1.0);
            let _ = style_for_opacity.set_property("--ui-tooltip-opacity", &format!("{v}"));
        });

        let style_for_scale = style.clone();
        let scale = ui_motion::spring::SpringAnimator::new(scale_initial, config, move |v| {
            let v = v.clamp(0.0, 10.0);
            let _ = style_for_scale.set_property("--ui-tooltip-scale", &format!("{v}"));
        });

        let style_for_y = style.clone();
        let y = ui_motion::spring::SpringAnimator::new(y_initial, config, move |v| {
            let v = v.clamp(-1000.0, 1000.0);
            let _ = style_for_y.set_property("--ui-tooltip-y", &format!("{v}px"));
        });

        let springs_for_cleanup = springs;
        on_cleanup(move || {
            if let Some((opacity, scale, y)) = springs_for_cleanup.get_value() {
                opacity.stop();
                scale.stop();
                y.stop();
            }
        });

        if open_now {
            opacity.set_target(1.0);
            scale.set_target(1.0);
            y.set_target(0.0);
        }

        springs.set_value(Some((opacity, scale, y)));
    });

    Effect::new(move |_| {
        let open = is_open.get();
        let Some(prev) = last_state.get_value() else {
            last_state.set_value(Some(open));
            return;
        };
        if prev == open {
            return;
        }
        last_state.set_value(Some(open));

        let Some((opacity, scale, y)) = springs.get_value() else {
            return;
        };

        let motion = motion.get_value();
        let offset_y = placement_offset_y(placement.get_untracked(), motion.offset_y_px);

        if open {
            opacity.clear_on_rest();
            scale.clear_on_rest();
            y.clear_on_rest();

            opacity.set_target(1.0);
            scale.set_target(1.0);
            y.set_target(0.0);
            return;
        }

        let on_exit_complete = on_exit_complete.clone();
        scale.set_on_rest(move || on_exit_complete.run(()));

        opacity.set_target(0.0);
        scale.set_target(motion.initial_scale);
        y.set_target(offset_y);
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    _placement: leptos::prelude::Signal<TooltipPlacement>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: TooltipMotion,
) {
    use leptos::prelude::*;

    let _ = sanitize_motion(motion);

    Effect::new(move |_| {
        if !is_open.get() {
            on_exit_complete.run(());
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_motion_uses_soft_spring_contract() {
        let motion = TooltipMotion::default();

        assert_eq!(motion.spring, ui_motion::presets::spring_soft());
        assert_eq!(motion.initial_scale, 0.98);
        assert_eq!(motion.offset_y_px, 6.0);
    }

    #[test]
    fn placement_offset_y_follows_vertical_direction_contract() {
        assert_eq!(placement_offset_y(TooltipPlacement::Bottom, 10.0), 10.0);
        assert_eq!(placement_offset_y(TooltipPlacement::Bottom, -4.0), 4.0);
        assert_eq!(placement_offset_y(TooltipPlacement::Top, 10.0), -10.0);
        assert_eq!(placement_offset_y(TooltipPlacement::Top, -4.0), -4.0);
    }

    #[test]
    fn supports_custom_motion_contract() {
        let motion = TooltipMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 220.0,
                damping: 20.0,
                mass: 1.0,
                precision: 0.003,
            },
            initial_scale: 0.94,
            offset_y_px: 11.0,
        };

        assert_eq!(motion.spring.stiffness, 220.0);
        assert_eq!(motion.spring.damping, 20.0);
        assert_eq!(motion.spring.mass, 1.0);
        assert_eq!(motion.spring.precision, 0.003);
        assert_eq!(motion.initial_scale, 0.94);
        assert_eq!(motion.offset_y_px, 11.0);
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let motion = sanitize_motion(TooltipMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            initial_scale: f64::NAN,
            offset_y_px: f64::NAN,
        });

        let default = TooltipMotion::default();
        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.initial_scale, default.initial_scale);
        assert_eq!(motion.offset_y_px, default.offset_y_px);
    }

    #[test]
    fn sanitize_motion_clamps_scale_and_offset_ranges() {
        let motion = sanitize_motion(TooltipMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 220.0,
                damping: 20.0,
                mass: 1.05,
                precision: 0.003,
            },
            initial_scale: 12.0,
            offset_y_px: -9999.0,
        });

        assert_eq!(motion.initial_scale, 3.0);
        assert_eq!(motion.offset_y_px, 320.0);
        assert_eq!(motion.spring.stiffness, 220.0);
        assert_eq!(motion.spring.damping, 20.0);
        assert_eq!(motion.spring.mass, 1.05);
        assert_eq!(motion.spring.precision, 0.003);
    }
}
