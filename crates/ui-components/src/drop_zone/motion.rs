#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DropZoneMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub hover_scale: f64,
    pub drop_scale: f64,
    pub hover_highlight: f64,
}

impl Default for DropZoneMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 260.0,
                damping: 18.0,
                mass: 1.0,
                ..Default::default()
            },
            hover_scale: 1.01,
            drop_scale: 1.02,
            hover_highlight: 0.35,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = DropZoneMotion::default().spring;

    ui_motion::spring::SpringConfig {
        stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0 {
            value.stiffness
        } else {
            default.stiffness
        },
        damping: if value.damping.is_finite() && value.damping > 0.0 {
            value.damping
        } else {
            default.damping
        },
        mass: if value.mass.is_finite() && value.mass > 0.0 {
            value.mass
        } else {
            default.mass
        },
        precision: if value.precision.is_finite() && value.precision > 0.0 {
            value.precision
        } else {
            default.precision
        },
    }
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

        drop(style.set_property("--ui-drop-zone-scale", "1"));
        drop(style.set_property("--ui-drop-zone-highlight", "0"));
        let style_for_scale = style.clone();
        let scale = ui_motion::spring::SpringAnimator::new(1.0, config, move |v| {
            let v = v.clamp(0.0, 10.0);
            drop(style_for_scale.set_property("--ui-drop-zone-scale", &format!("{v}")));
        });

        let style_for_highlight = style.clone();
        let highlight = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(0.0, 1.0);
            drop(style_for_highlight.set_property("--ui-drop-zone-highlight", &format!("{v}")));
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
mod tests {
    use super::*;

    #[test]
    fn default_motion_uses_expected_drop_zone_contract() {
        let motion = DropZoneMotion::default();

        assert_eq!(motion.spring.stiffness, 260.0);
        assert_eq!(motion.spring.damping, 18.0);
        assert_eq!(motion.spring.mass, 1.0);
        assert_eq!(motion.hover_scale, 1.01);
        assert_eq!(motion.drop_scale, 1.02);
        assert_eq!(motion.hover_highlight, 0.35);
    }

    #[test]
    fn sanitize_motion_falls_back_for_invalid_values() {
        let default = DropZoneMotion::default();

        let motion = sanitize_motion(DropZoneMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: f64::NAN,
                damping: -1.0,
                mass: 0.0,
                precision: f64::INFINITY,
            },
            hover_scale: f64::NAN,
            drop_scale: f64::INFINITY,
            hover_highlight: -2.0,
        });

        assert_eq!(motion.spring.stiffness, default.spring.stiffness);
        assert_eq!(motion.spring.damping, default.spring.damping);
        assert_eq!(motion.spring.mass, default.spring.mass);
        assert_eq!(motion.spring.precision, default.spring.precision);
        assert_eq!(motion.hover_scale, default.hover_scale);
        assert_eq!(motion.drop_scale, default.drop_scale);
        assert_eq!(motion.hover_highlight, 0.0);
    }

    #[test]
    fn supports_custom_drop_zone_motion_contract() {
        let motion = DropZoneMotion {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 220.0,
                damping: 20.0,
                mass: 1.0,
                precision: 0.002,
            },
            hover_scale: 1.015,
            drop_scale: 1.03,
            hover_highlight: 0.42,
        };

        assert_eq!(motion.spring.stiffness, 220.0);
        assert_eq!(motion.spring.damping, 20.0);
        assert_eq!(motion.spring.mass, 1.0);
        assert_eq!(motion.spring.precision, 0.002);
        assert_eq!(motion.hover_scale, 1.015);
        assert_eq!(motion.drop_scale, 1.03);
        assert_eq!(motion.hover_highlight, 0.42);
    }
}
