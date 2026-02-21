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

pub mod logic;
mod motion;
#[cfg(target_arch = "wasm32")]
mod observability;
mod protocol;
mod streaming;
pub mod styles;
mod view;

pub use logic::{
    AccordionPanelLifecycleEvent, AccordionSelectionMode, AccordionSlotProjection,
    AccordionVariant, open_set,
};
pub use motion::AccordionMotion;
pub use protocol::{
    ACCORDION_COMPONENT_SCHEMA_NAME, AccordionComponentItemSpec, AccordionComponentSchemaVersion,
    AccordionComponentSpec, AccordionComponentSpecError, AccordionSelectionModeSpec,
    AccordionVariantSpec, ResolvedAccordionComponentItemSpec, ResolvedAccordionComponentSpec,
};
pub use streaming::{
    AccordionStreamingItem, AccordionStreamingProjection, project_streaming_accordion_markup,
};
pub use view::{Accordion, AccordionItem};
