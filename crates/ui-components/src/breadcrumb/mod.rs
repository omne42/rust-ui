mod logic;
pub mod styles;
mod view;

pub use view::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator,
};

pub use ui_state_primitives::breadcrumb::{
    BreadcrumbLinkState, BreadcrumbLinkStateInput, BreadcrumbRootState, BreadcrumbRootStateInput,
    BreadcrumbSeparatorState, BreadcrumbSeparatorStateInput, BreadcrumbSlotState,
    BreadcrumbSlotStateInput,
};

pub const DEFAULT_ARIA_LABEL: &str = ui_state_primitives::breadcrumb::DEFAULT_ARIA_LABEL;
pub const DEFAULT_ELLIPSIS_LABEL: &str = "More";
