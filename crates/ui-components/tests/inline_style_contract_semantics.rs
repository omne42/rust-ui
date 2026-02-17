use std::fs;
use std::path::{Path, PathBuf};

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
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
fn inline_style_bindings_use_css_variable_builders_only() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    let allowed_patterns = [
        "style=panel_vars",
        "style=state.style_vars",
        "style=logic::compose_inline_style(",
        "style=inline_style.get_value().unwrap_or_default()",
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
        "expected at least one inline style binding contract in ui-components view layer."
    );
}

#[test]
fn inline_style_helper_outputs_are_css_variable_assignments() {
    let swatch_logic = load_source("src/swatch/logic.rs");
    let color_swatch_logic = load_source("src/color_swatch/logic.rs");
    let thumbnail_logic = load_source("src/thumbnail/logic.rs");
    let color_slider_logic = load_source("src/color_slider/logic.rs");
    let circular_progress_logic = load_source("src/circular_progress/logic.rs");

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
            || circular_progress_logic.contains(needle);

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
        "`ui-components` must not use inline style for normal CSS properties",
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
        "`ui-components` must not use `style=\"...\"` / `style=...` inside `view!`",
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
