pub use ui_state_primitives::skeleton::{
    SkeletonStateInput, SkeletonVariant, compose_class_name, normalize_optional_text, resolve_state,
};

pub const DEFAULT_IS_SHIMMER: bool = true;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SkeletonViewInput {
    pub variant: Option<SkeletonVariant>,
    pub is_shimmer: Option<bool>,
    pub has_custom_class_name: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SkeletonViewSourceState {
    pub variant_source_attr: &'static str,
    pub shimmer_source_attr: &'static str,
}

pub fn resolve_view_source_state(input: SkeletonViewInput) -> SkeletonViewSourceState {
    SkeletonViewSourceState {
        variant_source_attr: if input.variant.is_some() {
            "prop"
        } else {
            "default"
        },
        shimmer_source_attr: if input.is_shimmer.is_some() {
            "prop"
        } else {
            "default"
        },
    }
}

pub fn normalize_state_input(input: SkeletonViewInput) -> SkeletonStateInput {
    SkeletonStateInput {
        variant: input.variant.unwrap_or_default(),
        is_shimmer: input.is_shimmer.unwrap_or(DEFAULT_IS_SHIMMER),
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
