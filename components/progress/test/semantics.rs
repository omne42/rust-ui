use std::fs;
use std::path::Path;

#[test]
fn progress_public_api_keeps_motion_module_internal() {
    let source = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("mod.rs"),
    )
    .expect("progress mod.rs should be readable");

    assert!(
        source.contains("mod motion;"),
        "progress motion module should remain internal"
    );
    assert!(
        !source.contains("pub mod motion;"),
        "progress should not expose motion module internals"
    );
}

#[test]
fn progress_circle_public_api_keeps_motion_module_internal() {
    let source = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("circle")
            .join("mod.rs"),
    )
    .expect("progress circle mod.rs should be readable");

    assert!(
        source.contains("mod motion;"),
        "progress-circle motion module should remain internal"
    );
    assert!(
        !source.contains("pub mod motion;"),
        "progress-circle should not expose motion module internals"
    );
}

#[path = "../tests/progress_bar_semantics.rs"]
mod progress_bar_semantics;
#[path = "../tests/progress_circle_semantics.rs"]
mod progress_circle_semantics;
#[path = "../tests/progress_semantics.rs"]
mod progress_semantics;
