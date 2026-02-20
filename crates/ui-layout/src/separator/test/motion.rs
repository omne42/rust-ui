use super::{SeparatorMotion, sanitize_motion};

#[test]
fn default_motion_disables_entry_animation() {
    let motion = SeparatorMotion::default();

    assert!(
        !motion.animate_in,
        "SeparatorMotion defaults should avoid unexpected decorative motion."
    );
}

#[test]
fn sanitize_motion_keeps_explicit_entry_flag() {
    let motion = sanitize_motion(SeparatorMotion { animate_in: true });

    assert!(
        motion.animate_in,
        "SeparatorMotion sanitize contract should preserve explicit animation requests."
    );
}

#[test]
fn motion_contract_supports_explicit_entry_animation() {
    let motion = SeparatorMotion { animate_in: true };

    assert!(
        motion.animate_in,
        "SeparatorMotion should allow explicit entry animation for custom motion presets."
    );
}
