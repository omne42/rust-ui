use super::*;

#[test]
fn default_stream_has_no_value() {
    let stream: UiStream<String> = UiStream::default();
    assert_eq!(stream.checkpoint(), 0);
    assert_eq!(stream.value(), None);
    assert_eq!(stream.snapshot(), None);
}

#[test]
fn resume_restores_value_and_checkpoint() {
    let stream = UiStream::resume(UiStreamSnapshot {
        checkpoint: 12,
        value: "ok".to_string(),
    });
    assert_eq!(stream.checkpoint(), 12);
    assert_eq!(stream.value().map(String::as_str), Some("ok"));
}

#[test]
fn apply_requires_monotonic_checkpoints() {
    let mut stream = UiStream::default();

    let first_apply = stream.apply(UiStreamSnapshot {
        checkpoint: 1,
        value: "a".to_string(),
    });
    assert_eq!(first_apply, Ok(()));
    assert_eq!(stream.checkpoint(), 1);

    let err = stream
        .apply(UiStreamSnapshot {
            checkpoint: 0,
            value: "b".to_string(),
        })
        .err();
    assert_eq!(err, Some(UiStreamError::NonMonotonicCheckpoint));
    assert_eq!(stream.value().map(String::as_str), Some("a"));
}
