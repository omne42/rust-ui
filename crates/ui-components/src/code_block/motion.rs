#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CodeBlockMotion {
    pub spring: ui_motion::spring::SpringConfig,
    pub flash_hold_ms: u64,
}

impl Default for CodeBlockMotion {
    fn default() -> Self {
        Self {
            spring: ui_motion::presets::spring_soft(),
            flash_hold_ms: 120,
        }
    }
}

#[cfg(target_arch = "wasm32")]
use std::{cell::RefCell, rc::Rc};

#[cfg(any(test, target_arch = "wasm32"))]
struct CopyFlashDriver {
    flash: ui_motion::spring::SpringAnimator,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl CopyFlashDriver {
    fn new(config: ui_motion::spring::SpringConfig, mut apply: impl FnMut(f64) + 'static) -> Self {
        let flash = ui_motion::spring::SpringAnimator::new(0.0, config, move |v| {
            let v = v.clamp(0.0, 1.0);
            apply(v);
        });

        Self { flash }
    }

    fn stop(&self) {
        self.flash.stop();
    }

    #[cfg(test)]
    fn flash_immediate(&self) {
        self.flash.set_target(1.0);
        self.flash.set_target(0.0);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    copied: leptos::prelude::Signal<bool>,
    motion: CodeBlockMotion,
) {
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let motion = StoredValue::new(motion);
    let driver = StoredValue::new_local(None::<Rc<RefCell<CopyFlashDriver>>>);
    let last_copied = StoredValue::new(false);
    let reset_timeout = StoredValue::new_local(None::<TimeoutHandle>);

    Effect::new(move |_| {
        let config = motion.get_value().spring;
        let Some(div) = node_ref.get() else {
            return;
        };
        if driver.get_value().is_some() {
            return;
        }

        let element: leptos::web_sys::HtmlElement = div.unchecked_into();
        let style = element.style();

        let _ = style.set_property("--ui-code-block-copy-flash", "0");
        let driver_instance = Rc::new(RefCell::new(CopyFlashDriver::new(config, {
            let style = style.clone();
            move |v| {
                let _ = style.set_property("--ui-code-block-copy-flash", &format!("{v}"));
            }
        })));

        driver.set_value(Some(Rc::clone(&driver_instance)));

        let driver_for_cleanup = driver;
        let reset_timeout_for_cleanup = reset_timeout;
        on_cleanup(move || {
            if let Some(handle) = reset_timeout_for_cleanup.get_value() {
                handle.clear();
            }
            reset_timeout_for_cleanup.set_value(None);

            if let Some(driver) = driver_for_cleanup.get_value() {
                driver.borrow().stop();
            }
        });
    });

    Effect::new(move |_| {
        let copied = copied.get();

        if copied == last_copied.get_value() {
            return;
        }
        last_copied.set_value(copied);

        if !copied {
            return;
        }

        let Some(driver) = driver.get_value() else {
            return;
        };

        let motion = motion.get_value();

        if let Some(handle) = reset_timeout.get_value() {
            handle.clear();
        }
        reset_timeout.set_value(None);

        driver.borrow().flash.set_target(1.0);

        let driver_for_timeout = Rc::clone(&driver);
        if let Ok(handle) = set_timeout_with_handle(
            move || driver_for_timeout.borrow().flash.set_target(0.0),
            std::time::Duration::from_millis(motion.flash_hold_ms),
        ) {
            reset_timeout.set_value(Some(handle));
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(
    _node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    _copied: leptos::prelude::Signal<bool>,
    _motion: CodeBlockMotion,
) {
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn default_motion_uses_soft_spring() {
        let motion = CodeBlockMotion::default();
        assert_eq!(motion.spring, ui_motion::presets::spring_soft());
        assert!(motion.flash_hold_ms > 0);
    }

    #[test]
    fn flash_driver_triggers_peak_and_reset() {
        let values: Rc<RefCell<Vec<f64>>> = Rc::new(RefCell::new(Vec::new()));

        let driver = CopyFlashDriver::new(ui_motion::presets::spring_soft(), {
            let values = Rc::clone(&values);
            move |v| values.borrow_mut().push(v)
        });

        driver.flash_immediate();

        assert_eq!(&*values.borrow(), &[1.0, 0.0]);

        driver.stop();
    }
}
