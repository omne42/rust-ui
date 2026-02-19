#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FlexMotion {
    pub animate_in: bool,
}

pub fn sanitize_motion(motion: FlexMotion) -> FlexMotion {
    motion
}

#[cfg(target_arch = "wasm32")]
pub fn attach_motion(_node_ref: leptos::prelude::NodeRef<leptos::html::Div>, motion: FlexMotion) {
    sanitize_motion(motion);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn attach_motion(_node_ref: leptos::prelude::NodeRef<leptos::html::Div>, motion: FlexMotion) {
    sanitize_motion(motion);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_motion_preserves_default_contract() {
        let motion = sanitize_motion(FlexMotion::default());
        assert_eq!(motion, FlexMotion::default());
    }
}
