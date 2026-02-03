//! `ui-motion` — native (non-CSS) motion primitives.
//!
//! Design goals:
//! - Keep motion "contracts" (keyframes/options) decoupled from view code.
//! - Provide a web backend using the Web Animations API (WAAPI) on `wasm32`.
//! - Compile on non-wasm targets (no-op stubs) to support SSR/tooling builds.

pub mod keyframes;
pub mod options;
pub mod spring;

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(not(target_arch = "wasm32"))]
pub mod web {
    use crate::{keyframes::MotionKeyframe, options::MotionOptions};

    pub fn prefers_reduced_motion() -> bool {
        true
    }

    pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}
}
