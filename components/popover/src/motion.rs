use ui_headless::PopoverPlacement;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PopoverMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_scale: f64,
    pub offset_y_px: f64,
}

impl Default for PopoverMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::spring::SpringConfig {
                stiffness: 300.0,
                damping: 25.0,
                mass: 1.0,
                ..Default::default()
            },
            initial_scale: 0.98,
            offset_y_px: 6.0,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = PopoverMotion::default().spring;
    ui_motion::spring::sanitize_config(value, default)
}

pub fn sanitize_motion(motion: PopoverMotion) -> PopoverMotion {
    let default = PopoverMotion::default();

    PopoverMotion {
        spring: sanitize_spring(motion.spring),
        initial_scale: sanitize_number(motion.initial_scale, default.initial_scale).clamp(0.0, 3.0),
        offset_y_px: sanitize_number(motion.offset_y_px, default.offset_y_px)
            .abs()
            .clamp(0.0, 240.0),
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn placement_offset_y(placement: PopoverPlacement, base: f64) -> f64 {
    match placement {
        PopoverPlacement::BottomStart | PopoverPlacement::BottomEnd => base.abs(),
        PopoverPlacement::TopStart | PopoverPlacement::TopEnd => -base.abs(),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    placement: leptos::prelude::Signal<PopoverPlacement>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: PopoverMotion,
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

        drop(style.set_property("--ui-popover-opacity", &format!("{opacity_initial}")));
        drop(style.set_property("--ui-popover-scale", &format!("{scale_initial}")));
        drop(style.set_property("--ui-popover-y", &format!("{y_initial}px")));
        let style_for_opacity = style.clone();
        let opacity = ui_motion::spring::SpringAnimator::new(opacity_initial, config, move |v| {
            let v = v.clamp(0.0, 1.0);
            drop(style_for_opacity.set_property("--ui-popover-opacity", &format!("{v}")));
        });

        let style_for_scale = style.clone();
        let scale = ui_motion::spring::SpringAnimator::new(scale_initial, config, move |v| {
            let v = v.clamp(0.0, 10.0);
            drop(style_for_scale.set_property("--ui-popover-scale", &format!("{v}")));
        });

        let style_for_y = style.clone();
        let y = ui_motion::spring::SpringAnimator::new(y_initial, config, move |v| {
            let v = v.clamp(-1000.0, 1000.0);
            drop(style_for_y.set_property("--ui-popover-y", &format!("{v}px")));
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

        opacity.set_target(0.0);
        scale.set_target(motion.initial_scale);
        y.set_target(offset_y);

        let on_exit_complete = on_exit_complete.clone();
        scale.set_on_rest(move || on_exit_complete.run(()));
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    _placement: leptos::prelude::Signal<PopoverPlacement>,
    on_exit_complete: leptos::prelude::Callback<()>,
    _motion: PopoverMotion,
) {
    use leptos::prelude::*;

    Effect::new(move |_| {
        if !is_open.get() {
            on_exit_complete.run(());
        }
    });
}

#[cfg(test)]
#[path = "test/motion.rs"]
mod tests;
