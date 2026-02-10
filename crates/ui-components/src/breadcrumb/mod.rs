mod logic;
pub mod styles;
mod view;

pub use view::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator,
};

pub const DEFAULT_ARIA_LABEL: &str = "breadcrumb";
pub const DEFAULT_ELLIPSIS_LABEL: &str = "More";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbRootStateInput {
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbRootState {
    pub state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbSlotStateInput {
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbSlotState {
    pub state_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbLinkStateInput {
    pub has_href: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbLinkState {
    pub state_attr: &'static str,
    pub href_state_attr: &'static str,
    pub class_source_attr: &'static str,
    pub interactive: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbSeparatorStateInput {
    pub has_custom_content: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreadcrumbSeparatorState {
    pub state_attr: &'static str,
    pub content_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
