use std::fs;
use std::path::{Path, PathBuf};

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
fn load_workspace_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let path = workspace_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn collect_rs_files(root: &Path) -> Vec<PathBuf> {
    let mut stack = vec![root.to_path_buf()];
    let mut files = Vec::new();

    while let Some(path) = stack.pop() {
        let entries =
            fs::read_dir(&path).unwrap_or_else(|e| panic!("read_dir failed for {path:?}: {e}"));

        for entry in entries {
            let entry = entry.unwrap_or_else(|e| panic!("read_dir entry failed for {path:?}: {e}"));
            let entry_path = entry.path();

            if entry_path.is_dir() {
                stack.push(entry_path);
                continue;
            }

            if entry_path.extension().is_some_and(|ext| ext == "rs") {
                files.push(entry_path);
            }
        }
    }

    files.sort();
    files
}

#[test]
fn ui_components_disallow_style_prop_syntax_bindings() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    for path in collect_rs_files(&src_root) {
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));

        assert!(
            !source.contains("style:"),
            "inline style prop binding is forbidden by styling spec; found `style:` in {path:?}."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn inline_style_bindings_use_css_variable_builders_only() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    let allowed_patterns = [
        "style=panel_vars",
        "style=state.style_vars",
        "style=style_vars",
        "style=motion_style.clone()",
        "style=logic::compose_inline_style(",
        "style=inline_style.get_value().unwrap_or_default()",
        "style=move || inline_style.get()",
        "style=move || style.get_value()",
        "style=move || motion_style.get_value()",
    ];

    let mut inline_style_lines = Vec::new();

    for path in collect_rs_files(&src_root) {
        if path.file_name().is_none_or(|name| name != "view.rs") {
            continue;
        }

        let source = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));

        for (line_index, line) in source.lines().enumerate() {
            if !line.contains("style=") {
                continue;
            }

            inline_style_lines.push((path.clone(), line_index + 1, line.to_string()));

            let is_allowed = allowed_patterns
                .iter()
                .any(|pattern| line.contains(pattern));
            assert!(
                is_allowed,
                "inline style binding must be CSS-variable contract based; unexpected style assignment at {path:?}:{} -> {}",
                line_index + 1,
                line.trim(),
            );
        }
    }

    assert!(
        !inline_style_lines.is_empty(),
        "expected at least one inline style binding contract in ui view layer."
    );
}

#[test]
fn inline_style_helper_outputs_are_css_variable_assignments() {
    let swatch_logic = load_source("../../components/swatch/src/logic.rs");
    let color_swatch_logic = load_source("../../components/color-swatch/src/logic.rs");
    let thumbnail_logic = load_workspace_source("components/thumbnail/src/logic.rs");
    let color_slider_logic = load_workspace_source("components/color-slider/src/logic.rs");
    let circular_progress_logic =
        load_workspace_source("components/circular-progress/src/logic.rs");
    let circular_progress_primitive =
        load_workspace_source("crates/ui-state-primitives/src/circular_progress.rs");

    for needle in [
        "--ui-swatch-color:",
        "--ui-color-swatch-color:",
        "--ui-thumbnail-background:",
        "--ui-color-slider-track-start:",
        "--ui-color-slider-track-end:",
        "--ui-cp-size:",
        "--ui-cp-thickness:",
    ] {
        let has_needle = swatch_logic.contains(needle)
            || color_swatch_logic.contains(needle)
            || thumbnail_logic.contains(needle)
            || color_slider_logic.contains(needle)
            || circular_progress_logic.contains(needle)
            || circular_progress_primitive.contains(needle);

        assert!(
            has_needle,
            "expected css-variable declaration `{needle}` in inline style helper contracts."
        );
    }
}

#[test]
fn inline_style_contract_docs_page_covers_primary_playgrounds() {
    let styling_spec = load_source("../../docs/spec/styling.md");
    let rules_zh = load_source("../../docs/RULES_ZH.md");

    for needle in [
        "## Rules (Required)",
        "- **Inline CSS is forbidden in components:**",
        "`ui` must not use inline style for normal CSS properties",
        "- **Runtime values must use CSS variables (custom properties) only:**",
        "- Quick violation check: search the repo for `style=` and `style:`",
    ] {
        assert!(
            styling_spec.contains(needle),
            "styling spec should include `{needle}` for inline style contract documentation.",
        );
    }

    for needle in [
        "- **Inline CSS forbidden (component layer):**",
        "`ui` must not use `style=\"...\"` / `style=...` inside `view!`",
        "Do not bind normal CSS properties via `style:<prop>=...`",
        "Only CSS variables (custom properties, `--*`) are allowed",
    ] {
        assert!(
            rules_zh.contains(needle),
            "RULES_ZH should include `{needle}` for inline style contract coverage.",
        );
    }
}

#[test]
fn inline_style_contract_docs_playgrounds_lock_state_matrix_contract_values() {
    let styling_spec = load_source("../../docs/spec/styling.md");
    let rules_zh = load_source("../../docs/RULES_ZH.md");

    for needle in [
        "When passing runtime values, use custom properties (`--*`)",
        "Recommended: `style:--x=...`",
        "Allowed: `style=...` only when it contains **only** `--*` variable assignments",
        "Do not bind normal CSS properties via `style:<prop>=...`",
        "Style switching must use `class`/`data-*` + `styles.rs`",
    ] {
        assert!(
            styling_spec.contains(needle) || rules_zh.contains(needle),
            "inline style docs contracts should contain `{needle}`.",
        );
    }
}
