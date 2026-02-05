use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sliding_number_does_not_ignore_motion_contract() {
    let source = load_source("src/number/view.rs");

    assert!(
        !source.contains("let _ = motion"),
        "SlidingNumber should honor `SlidingNumberMotion` rather than ignoring it."
    );
}

#[test]
fn sliding_number_attaches_motion_driver() {
    let source = load_source("src/number/view.rs");

    assert!(
        source.contains("attach_motion"),
        "SlidingNumber should attach its motion driver to deliver per-digit spring motion."
    );
}

#[test]
fn sliding_number_styles_define_css_vars_for_motion() {
    let source = load_source("src/number/styles.rs");

    assert!(
        source.contains("--ui-sliding-number-offset"),
        "SlidingNumber styles should define `--ui-sliding-number-offset` so motion updates only touch CSS variables."
    );
}

#[test]
fn sliding_number_motion_uses_spring_animator() {
    let source = load_source("src/number/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "SlidingNumber motion should animate via a spring to match the repo's motion spec."
    );
}

#[test]
fn sliding_number_hides_visual_digits_from_screen_readers() {
    let source = load_source("src/number/view.rs");

    assert!(
        source.contains("aria-hidden=\"true\""),
        "SlidingNumber should mark the animated digit rollers as `aria-hidden` and expose a single a11y value."
    );
}
