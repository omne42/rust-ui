use super::*;

#[test]
fn default_label_layout_is_top_start() {
    let view = resolve_view_state(FormLabelPosition::default(), FormLabelAlign::default());
    assert_eq!(view.label_position, "top");
    assert_eq!(view.label_align, "start");
}

#[test]
fn attr_mapping_matches_enum_variants() {
    assert_eq!(FormLabelPosition::Left.as_attr(), "left");
    assert_eq!(FormLabelAlign::End.as_attr(), "end");
}
