use leptos::{html, prelude::*};
use ui_motion::spring::SpringConfig;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChipMotion {
    pub spring: SpringConfig,
    pub enter_offset_y_px: f64,
    pub enter_scale: f64,
}

impl Default for ChipMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_soft(),
            enter_offset_y_px: 4.0,
            enter_scale: 0.985,
        }
    }
}

fn sanitize_number(value: f64, fallback: f64) -> f64 {
    if value.is_finite() { value } else { fallback }
}

pub fn sanitize_motion(motion: ChipMotion) -> ChipMotion {
    let default = ChipMotion::default();

    ChipMotion {
        spring: ui_motion::spring::sanitize_config(motion.spring, default.spring),
        enter_offset_y_px: sanitize_number(motion.enter_offset_y_px, default.enter_offset_y_px)
            .abs()
            .clamp(0.0, 24.0),
        enter_scale: sanitize_number(motion.enter_scale, default.enter_scale).clamp(0.8, 1.0),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(node_ref: NodeRef<html::Span>, motion: ChipMotion) {
    use leptos::wasm_bindgen::JsCast;

    if ui_motion::web::prefers_reduced_motion() {
        return;
    }

    let motion = StoredValue::new(sanitize_motion(motion));
    let spring = StoredValue::new_local(None::<ui_motion::spring::SpringAnimator>);

    Effect::new(move |_| {
        let Some(node) = node_ref.get() else {
            return;
        };

        if spring.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = node.unchecked_into();
        let style = element.style();
        let motion = motion.get_value();
        ui_observability::set_css_property_observed_auto!(&(style), "--ui-chip-opacity", "0");
        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-chip-translate-y",
            &format!("{}px", motion.enter_offset_y_px),
        );
        ui_observability::set_css_property_observed_auto!(
            &(style),
            "--ui-chip-scale",
            &format!("{}", motion.enter_scale)
        );

        let style_for_apply = style.clone();
        let offset_y_px = motion.enter_offset_y_px;
        let scale_from = motion.enter_scale;
        let animator = ui_motion::spring::SpringAnimator::new(0.0, motion.spring, move |v| {
            let progress = v.clamp(0.0, 1.0);
            let translate_y_px = (1.0 - progress) * offset_y_px;
            let scale = scale_from + (1.0 - scale_from) * progress;

            ui_observability::set_css_property_observed_auto!(
                &(style_for_apply),
                "--ui-chip-opacity",
                &format!("{progress}")
            );
            ui_observability::set_css_property_observed_auto!(
                &(style_for_apply),
                "--ui-chip-translate-y",
                &format!("{translate_y_px}px")
            );
            ui_observability::set_css_property_observed_auto!(
                &(style_for_apply),
                "--ui-chip-scale",
                &format!("{scale}")
            );
        });
        animator.set_target(1.0);

        let spring_for_cleanup = spring;
        on_cleanup(move || {
            if let Some(animator) = spring_for_cleanup.get_value() {
                animator.stop();
            }
            spring_for_cleanup.set_value(None);
        });

        spring.set_value(Some(animator));
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(_node_ref: NodeRef<html::Span>, motion: ChipMotion) {
    std::hint::black_box(sanitize_motion(motion));
}
