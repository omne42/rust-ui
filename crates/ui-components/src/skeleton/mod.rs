#[cfg(feature = "component-skeleton_group")]
pub mod group;
mod logic;
pub mod styles;
mod view;

pub use logic::SkeletonVariant;
pub use view::Skeleton;
