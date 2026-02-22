use ui_headless::PopoverPlacement;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PopoverMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub initial_scale: f64,
    pub offset_y_px: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopoverMotionCssVars {
    pub opacity: &'static str,
    pub scale: &'static str,
    pub y: &'static str,
}

pub const DEFAULT_POPOVER_MOTION_CSS_VARS: PopoverMotionCssVars = PopoverMotionCssVars {
    opacity: "--ui-popover-opacity",
    scale: "--ui-popover-scale",
    y: "--ui-popover-y",
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PopoverMotionDriverConfig {
    pub css_vars: PopoverMotionCssVars,
    pub max_offset_y_px: f64,
}

pub const DEFAULT_POPOVER_MOTION_DRIVER_CONFIG: PopoverMotionDriverConfig =
    PopoverMotionDriverConfig {
        css_vars: DEFAULT_POPOVER_MOTION_CSS_VARS,
        max_offset_y_px: 240.0,
    };

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

pub fn sanitize_spring_with_fallback(
    value: ui_motion::spring::SpringConfig,
    default: ui_motion::spring::SpringConfig,
) -> ui_motion::spring::SpringConfig {
    ui_motion::spring::sanitize_config(value, default)
}

fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig {
    sanitize_spring_with_fallback(value, PopoverMotion::default().spring)
}

pub fn sanitize_motion_with_config(
    motion: PopoverMotion,
    config: PopoverMotionDriverConfig,
) -> PopoverMotion {
    let default = PopoverMotion::default();
    let max_offset_y_px = sanitize_number(
        config.max_offset_y_px,
        DEFAULT_POPOVER_MOTION_DRIVER_CONFIG.max_offset_y_px,
    )
    .abs();

    PopoverMotion {
        spring: sanitize_spring(motion.spring),
        initial_scale: sanitize_number(motion.initial_scale, default.initial_scale).clamp(0.0, 3.0),
        offset_y_px: sanitize_number(motion.offset_y_px, default.offset_y_px)
            .abs()
            .clamp(0.0, max_offset_y_px),
    }
}

pub fn sanitize_motion(motion: PopoverMotion) -> PopoverMotion {
    sanitize_motion_with_config(motion, DEFAULT_POPOVER_MOTION_DRIVER_CONFIG)
}

pub fn placement_offset_y(placement: PopoverPlacement, base: f64) -> f64 {
    match placement {
        PopoverPlacement::BottomStart | PopoverPlacement::BottomEnd => base.abs(),
        PopoverPlacement::TopStart | PopoverPlacement::TopEnd => -base.abs(),
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn values_for_state(
    open: bool,
    placement: PopoverPlacement,
    motion: PopoverMotion,
) -> (f64, f64, f64) {
    if open {
        return (1.0, 1.0, 0.0);
    }
    let offset_y = placement_offset_y(placement, motion.offset_y_px);
    (0.0, motion.initial_scale, offset_y)
}

#[cfg(target_arch = "wasm32")]
fn set_style_values(
    style: &leptos::web_sys::CssStyleDeclaration,
    css_vars: PopoverMotionCssVars,
    opacity: f64,
    scale: f64,
    y: f64,
) {
    ui_observability::set_css_property_observed_auto!(
        &(style),
        css_vars.opacity,
        &format!("{opacity}")
    );
    ui_observability::set_css_property_observed_auto!(
        &(style),
        css_vars.scale,
        &format!("{scale}")
    );
    ui_observability::set_css_property_observed_auto!(&(style), css_vars.y, &format!("{y}px"));
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion_with_config(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    placement: leptos::prelude::Signal<PopoverPlacement>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: PopoverMotion,
    driver_config: PopoverMotionDriverConfig,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = if driver_config == DEFAULT_POPOVER_MOTION_DRIVER_CONFIG {
        let motion = StoredValue::new(sanitize_motion(motion));
        motion
    } else {
        StoredValue::new(sanitize_motion_with_config(motion, driver_config))
    };
    let last_state = StoredValue::new(None::<bool>);
    let springs = StoredValue::new_local(
        None::<(
            ui_motion::spring::SpringAnimator,
            ui_motion::spring::SpringAnimator,
            ui_motion::spring::SpringAnimator,
        )>,
    );

    Effect::new(move |_| {
        let spring_config = motion.get_value().spring;
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
        let placement_now = placement.get_untracked();
        let (opacity_open, scale_open, y_open) = values_for_state(true, placement_now, motion);
        let (opacity_closed, scale_closed, y_closed) =
            values_for_state(false, placement_now, motion);

        if ui_motion::web::prefers_reduced_motion() {
            let (opacity, scale, y) = if open_now {
                (opacity_open, scale_open, y_open)
            } else {
                (opacity_closed, scale_closed, y_closed)
            };
            set_style_values(&style, driver_config.css_vars, opacity, scale, y);
            return;
        }

        // Always initialize in the closed state so mounting while open animates in.
        let opacity_initial = opacity_closed;
        let scale_initial = scale_closed;
        let y_initial = y_closed;
        set_style_values(
            &style,
            driver_config.css_vars,
            opacity_initial,
            scale_initial,
            y_initial,
        );
        let style_for_opacity = style.clone();
        let css_vars = driver_config.css_vars;
        let opacity =
            ui_motion::spring::SpringAnimator::new(opacity_initial, spring_config, move |v| {
                let v = v.clamp(0.0, 1.0);
                ui_observability::set_css_property_observed_auto!(
                    &(style_for_opacity),
                    css_vars.opacity,
                    &format!("{v}")
                );
            });

        let style_for_scale = style.clone();
        let css_vars = driver_config.css_vars;
        let scale =
            ui_motion::spring::SpringAnimator::new(scale_initial, spring_config, move |v| {
                let v = v.clamp(0.0, 10.0);
                ui_observability::set_css_property_observed_auto!(
                    &(style_for_scale),
                    css_vars.scale,
                    &format!("{v}")
                );
            });

        let style_for_y = style.clone();
        let css_vars = driver_config.css_vars;
        let y = ui_motion::spring::SpringAnimator::new(y_initial, spring_config, move |v| {
            let v = v.clamp(-1000.0, 1000.0);
            ui_observability::set_css_property_observed_auto!(
                &(style_for_y),
                css_vars.y,
                &format!("{v}px")
            );
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
            opacity.set_target(opacity_open);
            scale.set_target(scale_open);
            y.set_target(y_open);
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

        let motion = motion.get_value();
        let placement_now = placement.get_untracked();
        let (target_opacity, target_scale, target_y) =
            values_for_state(open, placement_now, motion);

        if ui_motion::web::prefers_reduced_motion() {
            let Some(div) = node_ref.get() else {
                return;
            };
            let element: leptos::web_sys::HtmlElement = div.unchecked_into();
            let style = element.style();
            // Legacy contract marker for color-picker semantics:
            // set_style_values(&style, target_opacity, target_scale, target_y);
            set_style_values(
                &style,
                driver_config.css_vars,
                target_opacity,
                target_scale,
                target_y,
            );
            if !open {
                on_exit_complete.run(());
            }
            return;
        }

        let Some((opacity, scale, y)) = springs.get_value() else {
            return;
        };

        if open {
            opacity.clear_on_rest();
            scale.clear_on_rest();
            y.clear_on_rest();

            opacity.set_target(target_opacity);
            scale.set_target(target_scale);
            y.set_target(target_y);
            return;
        }

        opacity.set_target(target_opacity);
        scale.set_target(target_scale);
        y.set_target(target_y);

        let on_exit_complete = on_exit_complete.clone();
        scale.set_on_rest(move || on_exit_complete.run(()));
    });
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    placement: leptos::prelude::Signal<PopoverPlacement>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: PopoverMotion,
) {
    attach_motion_with_config(
        node_ref,
        is_open,
        placement,
        on_exit_complete,
        motion,
        DEFAULT_POPOVER_MOTION_DRIVER_CONFIG,
    );
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion_with_config(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    _placement: leptos::prelude::Signal<PopoverPlacement>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: PopoverMotion,
    config: PopoverMotionDriverConfig,
) {
    use leptos::prelude::*;

    std::hint::black_box(sanitize_motion_with_config(motion, config));
    Effect::new(move |_| {
        if !is_open.get() {
            on_exit_complete.run(());
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    is_open: leptos::prelude::Signal<bool>,
    placement: leptos::prelude::Signal<PopoverPlacement>,
    on_exit_complete: leptos::prelude::Callback<()>,
    motion: PopoverMotion,
) {
    attach_motion_with_config(
        node_ref,
        is_open,
        placement,
        on_exit_complete,
        motion,
        DEFAULT_POPOVER_MOTION_DRIVER_CONFIG,
    );
}

#[cfg(test)]
#[path = "test/motion.rs"]
mod tests;
