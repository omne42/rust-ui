use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn button_share_module_reexports_component_motion_and_types() {
    let source = load_source("src/button_share/mod.rs");

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
        "pub mod button_share;",
        "pub use button_share::{",
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
fn button_share_motion_contract_defaults_and_delegate_paths_are_locked() {
    let source = load_source("src/button_share/motion.rs");

    for needle in [
        "pub struct ShareButtonMotion",
        "pub flip: FlipButtonMotion",
        "pub fn sanitize_motion(motion: ShareButtonMotion) -> ShareButtonMotion",
        "flip: crate::button_flip::motion::sanitize_motion(motion.flip)",
        "fn default_motion_matches_flip_button_defaults()",
        "fn sanitize_motion_delegates_to_flip_button_contract()",
        "fn supports_custom_flip_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "share button motion should include `{needle}` for delegated HeroUI-grade flip contracts."
        );
    }
}

#[test]
fn button_share_view_wires_motion_sanitization_and_source_markers() {
    let source = load_source("src/button_share/view.rs");

    for needle in [
        "let motion = crate::button_share::motion::sanitize_motion(motion);",
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
        "description=\"Flip-based share surface with centralized item/icon/handler state attrs and HeroUI-grade spring motion.\"",
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
