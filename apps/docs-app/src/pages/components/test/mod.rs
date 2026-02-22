use super::*;
use leptos::prelude::Owner;
use std::{collections::HashSet, fs, path::Path};

fn component_module_slugs() -> Vec<String> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib_rs = manifest_dir.join("../../crates/ui/src/lib.rs");
    let source = fs::read_to_string(&lib_rs)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", lib_rs.display()));

    source
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            line.strip_prefix("pub mod ")
                .and_then(|rest| rest.strip_suffix(';'))
                .map(|module| module.replace('_', "-"))
        })
        .collect()
}

fn expected_doc_slugs(module_slug: &str) -> &'static [&'static str] {
    match module_slug {
        "button-flip" => &["flip-button"],
        "button-search-input" => &["search-input-button"],
        "button-share" => &["share-button"],
        "button-theme-toggle" => &["theme-toggle-button"],
        "direction" => &["direction-provider"],
        "number" => &["static-number", "sliding-number"],
        "ripple" => &["motion-ripple"],
        "root" => &["ui-root"],
        "layout" => &["flex", "grid"],
        "group" => &["field-group"],
        "overlays" => &["overlay", "popover", "modal", "tray"],
        "overlay-arrow" => &["icon", "popover"],
        "collection" => &["item"],
        "color" => &[
            "color-picker",
            "color-field",
            "color-area",
            "color-slider",
            "color-wheel",
            "color-swatch",
            "color-editor",
            "color-swatch-picker",
        ],
        "area" => &["color-area"],
        "editor" => &["color-editor"],
        "handle" => &["color-handle"],
        "loupe" => &["color-loupe"],
        "swatch-picker" => &["color-swatch-picker"],
        "thumb" => &["color-thumb"],
        "wheel" => &["color-wheel"],
        "field-form" => &["field"],
        "list" => &["list", "list-item", "list-section"],
        "selection-indicator" => &["list-item", "menu-item"],
        "shared-element-transition" => &["view"],
        "virtualizer" => &["scroll-area"],
        "hidden-date-input" => &["date-input-group"],
        "dnd" => &["drop-zone", "file-trigger"],
        "drag-and-drop" => &["drop-zone", "file-trigger"],
        "theme-dark" => &["ui-root"],
        "theme-default" => &["ui-root"],
        "theme-express" => &["ui-root"],
        "theme-light" => &["ui-root"],
        "example-theme" => &["ui-root"],
        "spinbutton" => &["number-field"],
        "text-input" => &["input"],
        "toast" => &["toast-viewport"],
        "toolbar" => &["action-bar"],
        "ai-space" => &["accordion"],
        "active-highlight" => &[],
        _ => &[],
    }
}

#[test]
fn component_catalog_covers_public_component_modules() {
    let doc_slugs: HashSet<&str> = component_catalog().iter().map(|doc| doc.slug).collect();
    let mut missing = Vec::new();

    for module_slug in component_module_slugs() {
        let mapped = expected_doc_slugs(&module_slug);
        if mapped.is_empty() && module_slug == "active-highlight" {
            continue;
        }

        let covered = if mapped.is_empty() {
            doc_slugs.contains(module_slug.as_str())
        } else {
            mapped.iter().any(|slug| doc_slugs.contains(*slug))
        };

        if !covered {
            let expected = if mapped.is_empty() {
                module_slug.clone()
            } else {
                mapped.join(" | ")
            };
            missing.push(format!("{module_slug} -> {expected}"));
        }
    }

    assert!(
        missing.is_empty(),
        "docs catalog is missing component module coverage:\n{}",
        missing.join("\n")
    );
}

#[test]
fn every_component_doc_page_renders_at_least_one_playground() {
    drop(any_spawner::Executor::init_futures_executor());
    for doc in component_catalog().iter().copied() {
        Owner::new().with(|| {
            let _toc = crate::toc::provide_docs_toc();
            let registry = crate::playground::provide_playground_registry();

            drop((doc.page)());
            let titles = registry.titles().get_untracked();
            assert!(
                !titles.is_empty(),
                "component page `{}` is missing a <Playground>",
                doc.slug
            );
        });
    }
}
