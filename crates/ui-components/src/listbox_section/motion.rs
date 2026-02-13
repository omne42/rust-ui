pub type ListBoxSectionMotion = crate::illustrated_message::IllustratedMessageMotion;

pub fn sanitize_motion(motion: ListBoxSectionMotion) -> ListBoxSectionMotion {
    crate::illustrated_message::motion::sanitize_motion(motion)
}

pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    motion: ListBoxSectionMotion,
) {
    crate::illustrated_message::motion::attach_motion(node_ref, sanitize_motion(motion));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_preserves_default_contract() {
        let motion = sanitize_motion(ListBoxSectionMotion::default());
        assert_eq!(motion, ListBoxSectionMotion::default());
    }
}
