use ui_combo_box::ComboBoxMotion;

#[test]
fn combo_box_public_contract_exposes_motion_defaults() {
    let motion = ComboBoxMotion::default();
    assert!(motion.popover.initial_scale > 0.0);
    assert!(motion.highlight.spring.stiffness > 0.0);
}
