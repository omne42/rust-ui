use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn separator_does_not_ignore_motion_contract() {
    let source = load_source("src/separator/view.rs");

    assert!(
        !source.contains("let _ = motion"),
        "Separator should honor `SeparatorMotion` rather than ignoring it."
    );
}

#[test]
fn separator_attaches_motion_driver() {
    let source = load_source("src/separator/view.rs");

    assert!(
        source.contains("attach_motion"),
        "Separator should attach its motion driver when `SeparatorMotion` requests animation."
    );
}

#[test]
fn separator_styles_use_only_css_variables_for_motion() {
    let source = load_source("src/separator/styles.rs");

    for name in [
        "--ui-separator-scale-x",
        "--ui-separator-scale-y",
        "--ui-separator-opacity",
    ] {
        assert!(
            source.contains(name),
            "Separator styles should define `{name}` so motion updates only touch CSS variables."
        );
    }
}

#[test]
fn separator_motion_uses_spring_animator() {
    let source = load_source("src/separator/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "Separator motion should animate via a spring to match the repo's motion spec."
    );
}
