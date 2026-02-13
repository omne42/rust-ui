mod logic;
pub mod motion;
pub mod styles;
mod view;

pub use crate::field_group::FieldGroup as Group;
pub use crate::field_group::FieldGroupDensity as GroupDensity;
pub use crate::field_group::FieldGroupOrientation as GroupOrientation;
pub use logic::{DEFAULT_ARIA_LABEL, GroupState, GroupStateInput};
pub use motion::GroupMotion;
pub use view::Group as GroupForward;
