#[cfg(feature = "skeleton-group")]
pub mod group;
mod logic;
pub mod styles;
mod view;

pub use logic::SkeletonVariant;
pub use view::Skeleton;
