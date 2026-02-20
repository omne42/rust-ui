use super::*;

#[test]
fn source_attr_tracks_default_vs_custom_motion() {
    assert_eq!(source_attr(ColorEditorMotion::default()), "default");
    assert_eq!(source_attr(ColorEditorMotion::disabled()), "custom");
}
