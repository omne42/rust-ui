use crate::SheetPlacement;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SheetMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_offset_px: f64,
}

impl Default for SheetMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_slide(),
            initial_offset_px: 32.0,
        }
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = SheetMotion::default().spring;

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

pub fn sanitize_motion(motion: SheetMotion) -> SheetMotion {
    let default = SheetMotion::default();

    SheetMotion {
        spring: sanitize_spring(motion.spring),
        initial_offset_px: if motion.initial_offset_px.is_finite() {
            motion.initial_offset_px.abs().clamp(0.0, 640.0)
        } else {
            default.initial_offset_px
        },
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn placement_offset(placement: SheetPlacement, base: f64) -> (f64, f64) {
    match placement {
        SheetPlacement::Bottom => (0.0, base.abs()),
        SheetPlacement::Left => (-base.abs(), 0.0),
        SheetPlacement::Right => (base.abs(), 0.0),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    placement: SheetPlacement,
    finish_exit: leptos::prelude::Callback<()>,
    motion: SheetMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(sanitize_motion(motion));
    let prefers_reduced_motion = StoredValue::new(ui_motion::web::prefers_reduced_motion());
    let last_state = StoredValue::new(None::<bool>);
    let springs = StoredValue::new_local(
        None::<(
            ui_motion::spring::SpringAnimator,
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
        let prefers_reduced_motion = prefers_reduced_motion.get_value();

        let open_now = is_open.get_untracked();
        let (x_initial, y_initial) = placement_offset(placement, motion.initial_offset_px);

        let backdrop_initial = 0.0;
        let panel_opacity_initial = 0.0;

        drop(style.set_property(
            "--ui-sheet-backdrop-opacity",
            &format!("{backdrop_initial}"),
        ));
        drop(style.set_property(
            "--ui-sheet-panel-opacity",
            &format!("{panel_opacity_initial}"),
        ));
        drop(style.set_property("--ui-sheet-panel-x", &format!("{x_initial}px")));
        drop(style.set_property("--ui-sheet-panel-y", &format!("{y_initial}px")));
        if prefers_reduced_motion {
            let (target_backdrop, target_opacity, target_x, target_y) = if open_now {
                (1.0, 1.0, 0.0, 0.0)
            } else {
                (0.0, 0.0, x_initial, y_initial)
            };
            drop(style.set_property("--ui-sheet-backdrop-opacity", &format!("{target_backdrop}")));
            drop(style.set_property("--ui-sheet-panel-opacity", &format!("{target_opacity}")));
            drop(style.set_property("--ui-sheet-panel-x", &format!("{target_x}px")));
            drop(style.set_property("--ui-sheet-panel-y", &format!("{target_y}px")));
            return;
        }

        let style_for_backdrop = style.clone();
        let backdrop = ui_motion::spring::SpringAnimator::new(backdrop_initial, config, move |v| {
            let v = v.clamp(0.0, 1.0);
            drop(style_for_backdrop.set_property("--ui-sheet-backdrop-opacity", &format!("{v}")));
        });

        let style_for_opacity = style.clone();
        let panel_opacity =
            ui_motion::spring::SpringAnimator::new(panel_opacity_initial, config, move |v| {
                let v = v.clamp(0.0, 1.0);
                drop(style_for_opacity.set_property("--ui-sheet-panel-opacity", &format!("{v}")));
            });

        let style_for_x = style.clone();
        let panel_x = ui_motion::spring::SpringAnimator::new(x_initial, config, move |v| {
            let v = v.clamp(-5000.0, 5000.0);
            drop(style_for_x.set_property("--ui-sheet-panel-x", &format!("{v}px")));
        });

        let style_for_y = style.clone();
        let panel_y = ui_motion::spring::SpringAnimator::new(y_initial, config, move |v| {
            let v = v.clamp(-5000.0, 5000.0);
            drop(style_for_y.set_property("--ui-sheet-panel-y", &format!("{v}px")));
        });

        let springs_for_cleanup = springs;
        on_cleanup(move || {
            if let Some((backdrop, opacity, x, y)) = springs_for_cleanup.get_value() {
                backdrop.stop();
                opacity.stop();
                x.stop();
                y.stop();
            }
        });

        if open_now {
            backdrop.set_target(1.0);
            panel_opacity.set_target(1.0);
            panel_x.set_target(0.0);
            panel_y.set_target(0.0);
        }

        springs.set_value(Some((backdrop, panel_opacity, panel_x, panel_y)));
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

        let Some((backdrop, opacity, x, y)) = springs.get_value() else {
            return;
        };

        let motion = motion.get_value();
        let (x_initial, y_initial) = placement_offset(placement, motion.initial_offset_px);
        let reduced_motion = prefers_reduced_motion.get_value();

        if reduced_motion {
            let Some(div) = node_ref.get() else {
                return;
            };
            let element: leptos::web_sys::HtmlElement = div.unchecked_into();
            let style = element.style();

            let (target_backdrop, target_opacity, target_x, target_y) = if open {
                (1.0, 1.0, 0.0, 0.0)
            } else {
                (0.0, 0.0, x_initial, y_initial)
            };

            drop(style.set_property("--ui-sheet-backdrop-opacity", &format!("{target_backdrop}")));
            drop(style.set_property("--ui-sheet-panel-opacity", &format!("{target_opacity}")));
            drop(style.set_property("--ui-sheet-panel-x", &format!("{target_x}px")));
            drop(style.set_property("--ui-sheet-panel-y", &format!("{target_y}px")));
            if !open {
                finish_exit.run(());
            }
            return;
        }

        if open {
            backdrop.clear_on_rest();
            opacity.clear_on_rest();
            x.clear_on_rest();
            y.clear_on_rest();

            backdrop.set_target(1.0);
            opacity.set_target(1.0);
            x.set_target(0.0);
            y.set_target(0.0);
            return;
        }

        backdrop.set_target(0.0);
        opacity.set_target(0.0);
        x.set_target(x_initial);
        y.set_target(y_initial);

        let finish_exit = finish_exit.clone();
        opacity.set_on_rest(move || finish_exit.run(()));
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    _placement: SheetPlacement,
    finish_exit: leptos::prelude::Callback<()>,
    motion: SheetMotion,
) {
    use leptos::prelude::*;

    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
    Effect::new(move |_| {
        if !is_open.get() {
            finish_exit.run(());
        }
    });
}

#[cfg(test)]
#[path = "../test/motion.rs"]
mod tests;
