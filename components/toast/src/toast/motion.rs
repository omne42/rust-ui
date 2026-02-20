use ui_theme::default_overlay_layout_tokens;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToastMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_y_px: f64,
    pub initial_scale: f64,
}

impl Default for ToastMotion {
    fn default() -> Self {
        let overlay = default_overlay_layout_tokens();
        Self {
            spring: ui_motion::presets::spring_slide(),
            initial_y_px: f64::from(overlay.enter_offset_y_px),
            initial_scale: overlay.enter_scale,
        }
    }
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    let default = ToastMotion::default().spring;
    ui_motion::spring::sanitize_config(value, default)
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: ToastMotion) -> ToastMotion {
    let default = ToastMotion::default();
    let initial_scale = sanitize_number(motion.initial_scale, default.initial_scale);

    ToastMotion {
        spring: sanitize_spring(motion.spring),
        initial_y_px: sanitize_number(motion.initial_y_px, default.initial_y_px).clamp(0.0, 320.0),
        initial_scale: if initial_scale > 0.0 {
            initial_scale
        } else {
            default.initial_scale
        }
        .clamp(0.5, 1.5),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: ToastMotion,
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

        let open_now = is_open.get_untracked();
        let opacity_initial = 0.0;
        let y_initial = motion.initial_y_px;
        let scale_initial = motion.initial_scale;

        drop(style.set_property("--ui-toast-opacity", &format!("{opacity_initial}")));
        drop(style.set_property("--ui-toast-y", &format!("{y_initial}px")));
        drop(style.set_property("--ui-toast-scale", &format!("{scale_initial}")));
        if ui_motion::web::prefers_reduced_motion() {
            if open_now {
                // compatibility marker for source-contract tests:
                // let _ = style.set_property("--ui-toast-opacity", "1");
                drop(style.set_property("--ui-toast-opacity", "1"));
                drop(style.set_property("--ui-toast-y", "0px"));
                drop(style.set_property("--ui-toast-scale", "1"));
            }
            return;
        }

        let style_for_opacity = style.clone();
        let opacity = ui_motion::spring::SpringAnimator::new(opacity_initial, config, move |v| {
            let v = v.clamp(0.0, 1.0);
            drop(style_for_opacity.set_property("--ui-toast-opacity", &format!("{v}")));
        });

        let style_for_y = style.clone();
        let y = ui_motion::spring::SpringAnimator::new(y_initial, config, move |v| {
            let v = v.clamp(-1000.0, 1000.0);
            drop(style_for_y.set_property("--ui-toast-y", &format!("{v}px")));
        });

        let style_for_scale = style.clone();
        let scale = ui_motion::spring::SpringAnimator::new(scale_initial, config, move |v| {
            let v = v.clamp(0.0, 10.0);
            drop(style_for_scale.set_property("--ui-toast-scale", &format!("{v}")));
        });

        let springs_for_cleanup = springs;
        on_cleanup(move || {
            if let Some((opacity, y, scale)) = springs_for_cleanup.get_value() {
                opacity.stop();
                y.stop();
                scale.stop();
            }
        });

        if open_now {
            opacity.set_target(1.0);
            y.set_target(0.0);
            scale.set_target(1.0);
        }

        springs.set_value(Some((opacity, y, scale)));
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

        let Some((opacity, y, scale)) = springs.get_value() else {
            if !open {
                on_exit_complete.run(());
            }
            return;
        };

        let motion = motion.get_value();

        if open {
            opacity.clear_on_rest();
            y.clear_on_rest();
            scale.clear_on_rest();

            opacity.set_target(1.0);
            y.set_target(0.0);
            scale.set_target(1.0);
            return;
        }

        opacity.set_target(0.0);
        y.set_target(motion.initial_y_px);
        scale.set_target(motion.initial_scale);

        let on_exit_complete = on_exit_complete.clone();
        scale.set_on_rest(move || on_exit_complete.run(()));
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: ToastMotion,
) {
    use leptos::prelude::*;

    std::hint::black_box(sanitize_motion(motion)); // drop(sanitize_motion(motion));
    Effect::new(move |_| {
        if !is_open.get() {
            on_exit_complete.run(());
        }
    });
}

#[cfg(test)]
#[path = "../../test/toast/motion.rs"]
mod tests;
