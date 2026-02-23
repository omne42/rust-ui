#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProgressCircleMotion {
    pub spring: ui_motion::spring::SpringConfig,
}

impl Default for ProgressCircleMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_soft(),
        }
    }
}

pub fn sanitize_motion(motion: ProgressCircleMotion) -> ProgressCircleMotion {
    let fallback = ProgressCircleMotion::default().spring;

    ProgressCircleMotion {
        spring: ui_motion::spring::sanitize_config(motion.spring, fallback),
    }
}

pub fn use_progress_spring(
    target: leptos::prelude::Signal<f64>,
    motion: ProgressCircleMotion,
) -> leptos::prelude::ReadSignal<f64> {
    use leptos::prelude::*;

    let initial = target.get_untracked().clamp(0.0, 1.0);
    let (value, set_value) = signal(initial);

    let motion = StoredValue::new(sanitize_motion(motion));
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let config = motion.get_value().spring;
        if spring.get_value().is_some() {
            return;
        }

        let set_value_for_apply = set_value;
        let animator = ui_motion::spring::SpringAnimator::new(initial, config, move |v| {
            set_value_for_apply.set(v.clamp(0.0, 1.0));
        });

        let spring_for_cleanup = spring;
        on_cleanup(move || {
            if let Some(animator) = spring_for_cleanup.get_value() {
                animator.stop();
            }
        });

        spring.set_value(Some(animator));
    });

    Effect::new(move |_| {
        let v = target.get().clamp(0.0, 1.0);
        if let Some(animator) = spring.get_value() {
            animator.set_target(v);
        } else {
            set_value.set(v);
        }
    });

    value
}

#[cfg(test)]
#[path = "../../test/circle/motion.rs"]
mod tests;
