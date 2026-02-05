use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn meter_role_includes_progressbar_fallback() {
    let source = load_source("src/meter/view.rs");

    assert!(
        source.contains("role=\"meter progressbar\""),
        "Meter should include `progressbar` as a fallback role for browsers that don't support `meter` (React Spectrum parity)."
    );
}

#[test]
fn meter_attaches_motion_driver() {
    let source = load_source("src/meter/view.rs");

    assert!(
        source.contains("motion::attach_motion"),
        "Meter should attach its motion driver to animate progress changes via spring-driven CSS variables."
    );
}

#[test]
fn meter_motion_uses_spring_animator() {
    let source = load_source("src/meter/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Meter motion should animate via a spring to match the repo's motion spec."
    );
}

#[test]
fn meter_styles_define_css_vars_for_motion() {
    let source = load_source("src/meter/styles.rs");

    assert!(
        source.contains("--ui-meter-progress"),
        "Meter styles should consume `--ui-meter-progress` so motion updates only touch CSS variables."
    );
}
