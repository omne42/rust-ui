use ui_thumbnail::{ThumbnailMotion, ThumbnailSize};

#[test]
fn thumbnail_public_contract_exposes_size_and_motion_defaults() {
    assert_eq!(ThumbnailSize::default(), ThumbnailSize::Size500);
    assert!(ThumbnailMotion::default().enabled);
}
