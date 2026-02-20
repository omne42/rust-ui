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
fn button_share_module_reexports_component_motion_and_types() {
    let source = load_source("src/button/share/mod.rs");

    for needle in [
        "pub use logic::{ShareButtonIconPlacement, ShareButtonItem, SharePlatform};",
        "pub use motion::ShareButtonMotion;",
        "pub use view::ShareButton;",
    ] {
        assert!(
            source.contains(needle),
            "button_share module should expose `{needle}`.",
        );
    }
}

#[test]
fn crate_root_registers_button_share_compatibility_exports() {
    let source = load_source("src/lib.rs");

    for needle in [
        "pub mod button;",
        "pub use button::share::{",
        "ShareButton, ShareButtonIconPlacement, ShareButtonItem, ShareButtonMotion, SharePlatform,",
    ] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for button_share compatibility.",
        );
    }
}

#[test]
fn docs_actions_page_covers_share_button_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn share_button() -> AnyView",
        "title=\"ShareButton\"",
        "slug=\"share-button\"",
        "<ShareButton",
        "ShareButtonIconPlacement::None",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for share-button coverage.",
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn button_share_motion_contract_defaults_and_delegate_paths_are_locked() {
    let source = load_source("src/button/share/motion.rs");

    for needle in [
        "pub struct ShareButtonMotion",
        "pub flip: FlipButtonMotion",
        "pub fn sanitize_motion(motion: ShareButtonMotion) -> ShareButtonMotion",
        "flip: super::super::flip::motion::sanitize_motion(motion.flip)",
        "fn default_motion_matches_flip_button_defaults()",
        "fn sanitize_motion_delegates_to_flip_button_contract()",
        "fn supports_custom_flip_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "share button motion should include `{needle}` for delegated baseline-level flip contracts."
        );
    }
}

#[test]
fn button_share_view_wires_motion_sanitization_and_source_markers() {
    let source = load_source("src/button/share/view.rs");

    for needle in [
        "let motion = super::motion::sanitize_motion(motion);",
        "motion=motion.flip",
        "data-motion-source=if motion == ShareButtonMotion::default()",
        "data-custom-motion=(motion != ShareButtonMotion::default()).then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "share button view should include `{needle}` for stable motion/source marker contracts."
        );
    }
}

#[test]
fn docs_actions_page_locks_share_button_motion_contract_narrative() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "description=\"Flip-based share surface with centralized item/icon/handler state attrs and baseline-level spring motion.\"",
        "from=FlipDirection::Left",
        "from=FlipDirection::Right",
    ] {
        assert!(
            source.contains(needle),
            "actions docs page should include `{needle}` for share-button motion contract stability."
        );
    }
}

#[test]
fn share_button_docs_default_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn share_button() -> AnyView",
        "<Playground title=\"Default + callback\" code_signal=code>",
        "let on_icon_press = Callback::new(move |platform: SharePlatform| set_last.set(Some(platform)));",
        "<ShareButton on_icon_press=on_icon_press />",
        "\"last: \"",
        "unwrap_or_else(|| \"None\".to_string())",
    ] {
        assert!(
            source.contains(needle),
            "share-button docs default playground should contain `{needle}`.",
        );
    }
}

#[test]
fn share_button_docs_state_and_custom_playgrounds_lock_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<Playground title=\"Icon placement + custom items\" code_signal=states_code>",
        "icon=ShareButtonIconPlacement::Prefix",
        "from=FlipDirection::Left",
        "label=\"Share now\".to_string()",
        "items=custom_items_for_matrix.clone()",
        "icon=ShareButtonIconPlacement::None",
        "label=\"Iconless\".to_string()",
        "Blank custom item labels fall back to platform defaults; missing handlers stay safe.",
        "<Playground title=\"Custom Class + Direction\" code_signal=custom_code>",
        "class_name=\"docs-share-button-custom\".to_string()",
        "from=FlipDirection::Right",
        "label=\"Share docs\".to_string()",
        "label=\"Share defaults\".to_string()",
        "icon=ShareButtonIconPlacement::Suffix",
    ] {
        assert!(
            source.contains(needle),
            "share-button docs state/custom playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn button_share_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "pub(super) fn share_button() -> AnyView",
        "title=\"ShareButton\"",
        "slug=\"share-button\"",
        "title=\"Default + callback\"",
        "title=\"Icon placement + custom items\"",
        "title=\"Custom Class + Direction\"",
    ] {
        assert!(
            source.contains(needle),
            "share-button docs page should contain `{needle}`.",
        );
    }
}

#[test]
fn button_share_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/actions.rs");

    for needle in [
        "<Playground title=\"Default + callback\" code_signal=code>",
        "<ShareButton on_icon_press=on_icon_press />",
        "<Playground title=\"Icon placement + custom items\" code_signal=states_code>",
        "icon=ShareButtonIconPlacement::Prefix",
        "from=FlipDirection::Left",
        "label=\"Share now\".to_string()",
        "icon=ShareButtonIconPlacement::None",
        "label=\"Iconless\".to_string()",
        "<Playground title=\"Custom Class + Direction\" code_signal=custom_code>",
        "class_name=\"docs-share-button-custom\".to_string()",
        "from=FlipDirection::Right",
    ] {
        assert!(
            source.contains(needle),
            "share-button docs playground should contain `{needle}`.",
        );
    }
}
