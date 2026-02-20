use super::*;

#[test]
fn ai_render_mode_attr_values_are_stable() {
    assert_eq!(AiRenderMode::Snapshot.as_str(), "snapshot");
    assert_eq!(AiRenderMode::Streaming.as_str(), "streaming");
}

#[test]
fn ai_output_status_attr_values_are_stable() {
    assert_eq!(AiOutputStatus::Draft.as_str(), "draft");
    assert_eq!(AiOutputStatus::Verified.as_str(), "verified");
    assert_eq!(AiOutputStatus::Submittable.as_str(), "submittable");
}
