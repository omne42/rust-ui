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

pub fn normalize_state_input(input: SkeletonViewInput) -> SkeletonStateInput {
    SkeletonStateInput {
        variant: input.variant.unwrap_or_default(),
        is_shimmer: input.is_shimmer.unwrap_or(DEFAULT_IS_SHIMMER),
        has_custom_class_name: input.has_custom_class_name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_state_input_applies_single_default_source() {
        let state_input = normalize_state_input(SkeletonViewInput {
            variant: None,
            is_shimmer: None,
            has_custom_class_name: false,
        });

        assert_eq!(state_input.variant, SkeletonVariant::default());
        assert!(state_input.is_shimmer);
        assert!(!state_input.has_custom_class_name);
    }

    #[test]
    fn normalize_state_input_prefers_explicit_values() {
        let state_input = normalize_state_input(SkeletonViewInput {
            variant: Some(SkeletonVariant::Circle),
            is_shimmer: Some(false),
            has_custom_class_name: true,
        });

        assert_eq!(state_input.variant, SkeletonVariant::Circle);
        assert!(!state_input.is_shimmer);
        assert!(state_input.has_custom_class_name);
    }
}
