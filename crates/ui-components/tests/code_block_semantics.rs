use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn code_block_does_not_ignore_motion_contract() {
    let source = load_source("src/code_block/view.rs");

    assert!(
        !source.contains("let _ = motion"),
        "CodeBlock should honor `CodeBlockMotion` rather than ignoring it."
    );
}

#[test]
fn code_block_attaches_motion_driver() {
    let source = load_source("src/code_block/view.rs");

    assert!(
        source.contains("attach_motion"),
        "CodeBlock should attach its motion driver to deliver copy feedback motion."
    );
}

#[test]
fn code_block_styles_define_css_vars_for_motion() {
    let source = load_source("src/code_block/styles.rs");

    assert!(
        source.contains("--ui-code-block-copy-flash"),
        "CodeBlock styles should define `--ui-code-block-copy-flash` so motion updates only touch CSS variables."
    );
}

#[test]
fn code_block_motion_uses_spring_animator() {
    let source = load_source("src/code_block/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "CodeBlock motion should animate via a spring to match the repo's motion spec."
    );
}
