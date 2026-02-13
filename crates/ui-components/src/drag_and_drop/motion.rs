pub use crate::drop_zone::DropZoneMotion as DragAndDropMotion;

pub fn sanitize_motion(motion: DragAndDropMotion) -> DragAndDropMotion {
    crate::drop_zone::motion::sanitize_motion(motion)
}

pub fn source_attr(motion: DragAndDropMotion) -> &'static str {
    if sanitize_motion(motion) == DragAndDropMotion::default() {
        "default"
    } else {
        "custom"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_attr_reflects_default_and_custom_motion() {
        assert_eq!(source_attr(DragAndDropMotion::default()), "default");

        let custom = DragAndDropMotion {
            hover_scale: 1.08,
            ..DragAndDropMotion::default()
        };
        assert_eq!(source_attr(custom), "custom");
    }
}
