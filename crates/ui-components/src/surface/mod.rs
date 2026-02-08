mod logic;
pub mod styles;
mod view;

pub use logic::{DEFAULT_ARIA_LABEL, SurfaceElevation, SurfaceTone};
pub use view::Surface;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceStateInput {
    pub tone: SurfaceTone,
    pub elevation: SurfaceElevation,
    pub bordered: bool,
    pub padded: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceState {
    pub tone: SurfaceTone,
    pub tone_class: &'static str,
    pub tone_attr: &'static str,
    pub elevation: SurfaceElevation,
    pub elevation_class: &'static str,
    pub elevation_attr: &'static str,
    pub is_bordered: bool,
    pub is_padded: bool,
    pub is_plain: bool,
    pub data_state_attr: &'static str,
    pub aria_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub has_custom_class_name: bool,
}
