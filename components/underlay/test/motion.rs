use super::*;

#[test]
fn disabled_constructor_turns_motion_off() {
    assert!(!UnderlayMotion::disabled().enabled);
}
