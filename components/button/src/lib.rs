macro_rules! wasm_debug_proxy {
    ($feature:literal, $debug:block, $release:block $(,)?) => {{
        #[cfg(all(feature = $feature, debug_assertions, target_arch = "wasm32"))]
        {
            $debug
        }
        #[cfg(not(all(feature = $feature, debug_assertions, target_arch = "wasm32")))]
        {
            $release
        }
    }};
}

pub(crate) use wasm_debug_proxy;

#[cfg(target_arch = "wasm32")]
mod observability;

#[cfg(feature = "component-button_copy")]
pub use ui_snippet as snippet;

#[path = "mod.rs"]
pub mod button;

pub use button::*;
