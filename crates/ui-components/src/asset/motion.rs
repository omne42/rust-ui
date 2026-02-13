pub type AssetMotion = crate::thumbnail::ThumbnailMotion;

pub fn sanitize_motion(motion: AssetMotion) -> AssetMotion {
    crate::thumbnail::motion::sanitize_motion(motion)
}

pub fn attach_motion(
    node_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    active: leptos::prelude::Signal<bool>,
    motion: AssetMotion,
) {
    crate::thumbnail::motion::attach_motion(node_ref, active, sanitize_motion(motion));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_delegates_to_thumbnail_contract() {
        let motion = sanitize_motion(AssetMotion::default());
        assert_eq!(motion, AssetMotion::default());
    }
}
