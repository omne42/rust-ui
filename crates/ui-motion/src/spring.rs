use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringConfig {
    pub stiffness: f64,
    pub damping: f64,
    pub mass: f64,
    pub precision: f64,
}

impl Default for SpringConfig {
    fn default() -> Self {
        Self {
            stiffness: 320.0,
            damping: 26.0,
            mass: 1.0,
            precision: 0.001,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SpringState {
    value: f64,
    velocity: f64,
}

impl SpringState {
    fn new(value: f64) -> Self {
        Self {
            value,
            velocity: 0.0,
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn step(&mut self, target: f64, dt_s: f64, config: SpringConfig) -> bool {
        let dt_s = dt_s.clamp(0.0, 0.064);
        if dt_s == 0.0 {
            return false;
        }

        let displacement = self.value - target;
        let spring_force = -config.stiffness * displacement;
        let damping_force = -config.damping * self.velocity;
        let acceleration = (spring_force + damping_force) / config.mass;

        self.velocity += acceleration * dt_s;
        self.value += self.velocity * dt_s;

        let is_at_rest = (self.value - target).abs() < config.precision
            && self.velocity.abs() < config.precision;
        if is_at_rest {
            self.value = target;
            self.velocity = 0.0;
        }
        is_at_rest
    }
}

#[derive(Clone)]
pub struct SpringAnimator {
    inner: Rc<SpringAnimatorInner>,
}

struct SpringAnimatorInner {
    #[cfg(target_arch = "wasm32")]
    config: SpringConfig,
    state: Cell<SpringState>,
    target: Cell<f64>,
    last_ts_ms: Cell<Option<f64>>,
    raf_handle: Cell<Option<i32>>,
    apply: RefCell<Box<dyn FnMut(f64)>>,
    on_rest: RefCell<Option<Box<dyn FnMut()>>>,
    #[cfg(target_arch = "wasm32")]
    raf_closure: RefCell<Option<wasm_bindgen::closure::Closure<dyn FnMut(f64)>>>,
}

impl SpringAnimator {
    pub fn new(initial: f64, config: SpringConfig, apply: impl FnMut(f64) + 'static) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let _ = config;

        Self {
            inner: Rc::new(SpringAnimatorInner {
                #[cfg(target_arch = "wasm32")]
                config,
                state: Cell::new(SpringState::new(initial)),
                target: Cell::new(initial),
                last_ts_ms: Cell::new(None),
                raf_handle: Cell::new(None),
                apply: RefCell::new(Box::new(apply)),
                on_rest: RefCell::new(None),
                #[cfg(target_arch = "wasm32")]
                raf_closure: RefCell::new(None),
            }),
        }
    }

    pub fn set_on_rest(&self, on_rest: impl FnMut() + 'static) {
        self.inner.on_rest.replace(Some(Box::new(on_rest)));
    }

    pub fn clear_on_rest(&self) {
        self.inner.on_rest.replace(None);
    }

    pub fn set_target(&self, target: f64) {
        self.inner.target.set(target);

        if crate::web::prefers_reduced_motion() {
            self.inner.state.set(SpringState::new(target));
            self.inner.last_ts_ms.set(None);
            self.inner.raf_handle.set(None);
            (self.inner.apply.borrow_mut())(target);
            if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {
                on_rest();
            }
            return;
        }

        if self.inner.raf_handle.get().is_some() {
            return;
        }

        self.start();
    }

    pub fn stop(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let Some(handle) = self.inner.raf_handle.take() else {
                return;
            };
            if let Some(window) = web_sys::window() {
                let _ = window.cancel_animation_frame(handle);
            }
            self.inner.last_ts_ms.set(None);
            self.inner.raf_closure.borrow_mut().take();
        }
    }

    fn start(&self) {
        #[cfg(target_arch = "wasm32")]
        {
            let Some(window) = web_sys::window() else {
                return;
            };
            let window_for_cb = window.clone();

            let inner = Rc::downgrade(&self.inner);

            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |ts_ms: f64| {
                let Some(inner) = inner.upgrade() else {
                    return;
                };

                let last_ts_ms = inner.last_ts_ms.get().unwrap_or(ts_ms);
                inner.last_ts_ms.set(Some(ts_ms));

                let dt_s = (ts_ms - last_ts_ms) / 1000.0;
                let target = inner.target.get();
                let mut state = inner.state.get();
                let is_at_rest = state.step(target, dt_s, inner.config);
                inner.state.set(state);

                {
                    (inner.apply.borrow_mut())(state.value);
                }

                if is_at_rest {
                    inner.raf_handle.set(None);
                    inner.last_ts_ms.set(None);
                    if let Some(on_rest) = inner.on_rest.borrow_mut().as_mut() {
                        on_rest();
                    }
                    return;
                }

                let cb = inner.raf_closure.borrow();
                let Some(cb) = cb.as_ref() else {
                    inner.raf_handle.set(None);
                    return;
                };
                let Ok(handle) = window_for_cb.request_animation_frame(cb.as_ref().unchecked_ref())
                else {
                    inner.raf_handle.set(None);
                    return;
                };
                inner.raf_handle.set(Some(handle));
            })
                as Box<dyn FnMut(f64)>);

            *self.inner.raf_closure.borrow_mut() = Some(closure);
            if let Some(cb) = self.inner.raf_closure.borrow().as_ref() {
                if let Ok(handle) = window.request_animation_frame(cb.as_ref().unchecked_ref()) {
                    self.inner.raf_handle.set(Some(handle));
                }
            }
        }
    }
}
