mod logic;
mod motion;
mod protocol;
pub mod styles;
mod view;

pub use logic::{AccordionSelectionMode, AccordionVariant, open_set};
pub use motion::AccordionMotion;
pub use protocol::{
    ACCORDION_COMPONENT_SCHEMA_NAME, AccordionComponentItemSpec, AccordionComponentSchemaVersion,
    AccordionComponentSpec, AccordionComponentSpecError, AccordionSelectionModeSpec,
    AccordionVariantSpec, ResolvedAccordionComponentItemSpec, ResolvedAccordionComponentSpec,
};
pub use view::{Accordion, AccordionItem};
