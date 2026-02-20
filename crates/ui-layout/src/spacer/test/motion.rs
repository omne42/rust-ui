use super::*;

#[test]
fn default_motion_is_noop_contract() {
    assert_eq!(SpacerMotion::default(), SpacerMotion { animate_in: false });
}

#[test]
fn source_attr_reflects_default_vs_custom_motion() {
    assert_eq!(source_attr(SpacerMotion::default()), "default");
    assert_eq!(source_attr(SpacerMotion { animate_in: true }), "custom");
}
