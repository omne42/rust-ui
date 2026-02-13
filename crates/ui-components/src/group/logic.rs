pub use crate::field_group::DEFAULT_ARIA_LABEL;
pub use crate::field_group::FieldGroupState as GroupState;
pub use crate::field_group::FieldGroupStateInput as GroupStateInput;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_group::{DEFAULT_ARIA_LABEL, FieldGroupDensity, FieldGroupOrientation};

    #[test]
    fn group_aliases_match_field_group_contracts() {
        let input = GroupStateInput {
            orientation: FieldGroupOrientation::Vertical,
            density: FieldGroupDensity::Comfortable,
            disabled: false,
            invalid: false,
            has_label: true,
            has_description: false,
            has_custom_aria_label: false,
            has_custom_class_name: false,
        };

        assert_eq!(input.orientation, FieldGroupOrientation::Vertical);
        assert_eq!(input.density, FieldGroupDensity::Comfortable);
        assert_eq!(DEFAULT_ARIA_LABEL, "Field group");
    }
}
