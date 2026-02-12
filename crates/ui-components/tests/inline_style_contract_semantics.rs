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
        "## 规范（必须遵守）",
        "- **组件禁止 inline CSS**：",
        "ui-components` 中禁止写“普通属性”的 inline style",
        "- **运行时数值只允许用 CSS variables（custom properties）**：",
        "- 快速排查违规：在仓库中搜索 `style=` 与 `style:`",
    ] {
        assert!(
            styling_spec.contains(needle),
            "styling spec should include `{needle}` for inline style contract documentation.",
        );
    }

    for needle in [
        "- **禁止 inline CSS（组件层）**：",
        "ui-components` 中禁止在 `view!` 里写 `style=\"...\"` / `style=...`",
        "禁止使用 `style:<prop>=...` 绑定普通 CSS 属性",
        "只允许设置 **CSS variables（custom properties，`--*`）**",
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
        "允许传递运行时数值时使用 custom properties（`--*`）",
        "推荐：`style:--x=...`",
        "允许：`style=...` 但内容必须 **只包含** `--*` 变量赋值",
        "禁止使用 `style:<prop>=...` 绑定普通 CSS 属性",
        "样式切换通过 `class`/`data-*` + `styles.rs` 完成",
    ] {
        assert!(
            styling_spec.contains(needle) || rules_zh.contains(needle),
            "inline style docs contracts should contain `{needle}`.",
        );
    }
}
