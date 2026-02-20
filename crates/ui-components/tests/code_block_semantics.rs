use std::fs;
use std::path::Path;

fn resolve_source_path(rel_path: &str) -> Option<std::path::PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let mut candidates = vec![manifest_dir.join(rel_path)];

    if let Some(component_rel_path) = rel_path.strip_prefix("../../components/") {
        let direct = workspace_dir.join("components").join(component_rel_path);
        candidates.push(direct.clone());

        let parts: Vec<&str> = component_rel_path.split('/').collect();
        if parts.len() > 3 && parts.get(1) == Some(&"src") && parts.get(2) == parts.first() {
            let collapsed = workspace_dir
                .join("components")
                .join(parts[0])
                .join("src")
                .join(parts[3..].join("/"));
            candidates.push(collapsed);
        }
    }

    if let Some(src_rel_path) = rel_path.strip_prefix("src/") {
        let segments: Vec<&str> = src_rel_path.split('/').collect();
        let components_root = workspace_dir.join("components");

        if let Ok(entries) = fs::read_dir(&components_root) {
            let component_dirs: Vec<String> = entries
                .flatten()
                .filter_map(|entry| {
                    let path = entry.path();
                    path.is_dir()
                        .then(|| entry.file_name().to_string_lossy().to_string())
                })
                .collect();

            for component_dir in component_dirs {
                for start in 0..segments.len() {
                    for end in start..segments.len() {
                        let name = segments[start..=end]
                            .iter()
                            .map(|segment| segment.replace('_', "-"))
                            .collect::<Vec<_>>()
                            .join("-");

                        if name != component_dir {
                            continue;
                        }

                        if end + 1 >= segments.len() {
                            candidates
                                .push(components_root.join(&component_dir).join("src/mod.rs"));
                            candidates
                                .push(components_root.join(&component_dir).join("src/check2.md"));
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                            continue;
                        }

                        let suffix = segments[end + 1..].join("/");
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("src")
                                .join(&suffix),
                        );
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("test")
                                .join(&suffix),
                        );

                        if suffix == "check2.md" {
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                        }
                    }
                }
            }
        }
    }

    candidates.into_iter().find(|path| path.exists())
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_source_path(rel_path)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path));

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}
#[test]
fn code_block_does_not_expose_logic_or_view_modules() {
    let source = load_source("../../components/code-block/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "CodeBlock internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn code_block_uses_logic_state_model() {
    let view_source = load_source("../../components/code-block/src/view.rs");
    let logic_source = load_source("../../components/code-block/src/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/code_block.rs");

    for needle in [
        "pub use crate::button::normalize_optional_text;",
        "pub struct CodeBlockStateInput",
        "pub struct CodeBlockViewState",
        "pub fn resolve_state(input: CodeBlockStateInput)",
        "pub fn resolve_view_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "CodeBlock state primitive should include `{needle}` in ui-state-primitives."
        );
    }

    assert!(
        logic_source.contains("pub use ui_state_primitives::code_block::{"),
        "CodeBlock logic should consume state primitives from ui-state-primitives."
    );
    assert!(
        !logic_source.contains("pub struct CodeBlockStateInput"),
        "CodeBlock logic should not re-define primitive structs."
    );
    assert!(
        logic_source.contains("pub fn compose_class_name("),
        "CodeBlock logic should keep component assembly helpers such as class composition."
    );

    for needle in [
        "logic::normalize_optional_text(label)",
        "logic::normalize_optional_text(language)",
        "logic::normalize_optional_text(class_name)",
        "logic::resolve_state(CodeBlockStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "CodeBlock view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn code_block_emits_baseline_style_state_data_attributes() {
    let source = load_source("../../components/code-block/src/view.rs");

    for attr in [
        "data-slot=\"code-block\"",
        "data-state=state.state_attr",
        "data-header=state.header_attr",
        "data-multiline=state.is_multiline.then_some(\"true\")",
        "data-empty=state.is_empty.then_some(\"true\")",
        "data-label=state.has_label.then_some(\"true\")",
        "data-language=state.has_language.then_some(\"true\")",
        "data-copyable=state.copyable.then_some(\"true\")",
        "data-motion-source=state.motion_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-slot=\"code-block-status\"",
    ] {
        assert!(
            source.contains(attr),
            "CodeBlock should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn code_block_styles_include_state_marker_contracts() {
    let source = load_source("../../components/code-block/src/styles.rs");

    for selector in [
        ".ui-code-block--state-multiline",
        ".ui-code-block[data-state=\"single-line\"]",
        ".ui-code-block--header-visible",
        ".ui-code-block[data-header=\"hidden\"]",
        ".ui-code-block--copyable",
        ".ui-code-block[data-motion-source=\"custom\"]",
        ".ui-code-block--custom-class",
        ".ui-code-block[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "CodeBlock styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn code_block_does_not_ignore_motion_contract() {
    let source = load_source("../../components/code-block/src/view.rs");

    assert!(
        !source.contains("let _ = motion"),
        "CodeBlock should honor `CodeBlockMotion` rather than ignoring it."
    );
}

#[test]
fn code_block_attaches_motion_driver() {
    let source = load_source("../../components/code-block/src/view.rs");

    assert!(
        source.contains("attach_motion"),
        "CodeBlock should attach its motion driver to deliver copy feedback motion."
    );
}

#[test]
fn code_block_styles_define_css_vars_for_motion() {
    let source = load_source("../../components/code-block/src/styles.rs");

    assert!(
        source.contains("--ui-code-block-copy-flash"),
        "CodeBlock styles should define `--ui-code-block-copy-flash` so motion updates only touch CSS variables."
    );
}

#[test]
fn code_block_motion_uses_spring_animator() {
    let source = load_source("../../components/code-block/src/motion.rs");

    assert!(
        source.contains("SpringAnimator"),
        "CodeBlock motion should animate via a spring to match the repo's motion spec."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn code_block_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("../../components/code-block/src/motion.rs");
    let view_source = load_source("../../components/code-block/src/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: CodeBlockMotion) -> CodeBlockMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "CodeBlock motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::motion::sanitize_motion(motion);"),
        "CodeBlock view should sanitize motion before attaching copy-flash driver.",
    );
}

#[test]
fn code_block_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "pub(super) fn code_block() -> AnyView",
        "title=\"CodeBlock\"",
        "slug=\"code-block\"",
        "title=\"Header + Copy Motion\"",
        "title=\"Compact + No Copy\"",
    ] {
        assert!(
            source.contains(needle),
            "display docs page should contain `{needle}` for CodeBlock.",
        );
    }
}

#[test]
fn code_block_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display.rs");

    for needle in [
        "title=\"Header + Copy Motion\"",
        "code=rust_code.to_string()",
        "language=\"rust\".to_string()",
        "label=\"deploy.rs\".to_string()",
        "title=\"Compact + No Copy\"",
        "code=\"cargo test -p ui-components --test code_block_semantics\".to_string()",
        "copyable=false",
        "class_name=\"docs-code-block-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "code-block docs playgrounds should contain `{needle}`.",
        );
    }
}
