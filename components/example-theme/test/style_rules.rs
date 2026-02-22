use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Hit {
    rel_path: PathBuf,
    line: usize,
    line_text: String,
}

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir).unwrap_or_else(|e| panic!("read_dir failed for {dir:?}: {e}"));
    for entry in entries {
        let entry = entry.unwrap_or_else(|e| panic!("read_dir entry failed for {dir:?}: {e}"));
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

fn find_hits(manifest_dir: &Path, path: &Path, needle: &str) -> Vec<Hit> {
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    let rel_path = path
        .strip_prefix(manifest_dir)
        .unwrap_or(path)
        .to_path_buf();

    contents
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            if !line.contains(needle) {
                return None;
            }
            Some(Hit {
                rel_path: rel_path.clone(),
                line: idx + 1,
                line_text: line.trim_end().to_string(),
            })
        })
        .collect()
}

fn format_hits(title: &str, hits: &[Hit]) -> String {
    let mut out = String::new();
    out.push_str(title);
    out.push('\n');
    for hit in hits {
        out.push_str(&format!(
            "- {}:{}: {}\n",
            hit.rel_path.display(),
            hit.line,
            hit.line_text
        ));
    }
    out
}

fn collect_workspace_style_rs_files(workspace_dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for root in [
        workspace_dir.join("crates/ui/src"),
        workspace_dir.join("crates/ui-layout/src"),
        workspace_dir.join("components"),
    ] {
        if root.exists() {
            collect_rs_files(&root, &mut files);
        }
    }
    files.retain(|path| path.file_name().and_then(|name| name.to_str()) == Some("styles.rs"));
    files.sort();
    files
}

#[test]
fn no_style_colon_bindings_exist() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");

    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);
    files.sort();

    let mut hits = Vec::new();
    for file in &files {
        hits.extend(find_hits(manifest_dir, file, "style:"));
    }

    assert!(
        hits.is_empty(),
        "{}",
        format_hits("Found forbidden `style:` bindings:", &hits)
    );
}

#[test]
fn style_equals_is_only_used_in_approved_files() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");

    let allowed_paths: HashSet<PathBuf> = [
        PathBuf::from("src/autocomplete/view.rs"),
        PathBuf::from("src/button/action/view.rs"),
        PathBuf::from("src/button/toggle_button/view.rs"),
        PathBuf::from("src/color/area/view.rs"),
        PathBuf::from("../../components/color-handle/src/view.rs"),
        PathBuf::from("src/color/slider/view.rs"),
        PathBuf::from("../../components/color-swatch/src/view.rs"),
        PathBuf::from("src/field_form/fieldset/view.rs"),
        PathBuf::from("src/header/view.rs"),
        PathBuf::from("src/hover_card/view.rs"),
        PathBuf::from("src/legend/view.rs"),
        PathBuf::from("../../components/popover/src/view.rs"),
        PathBuf::from("../../components/preview-card/src/view.rs"),
        PathBuf::from("../../components/preview-link-card/src/view.rs"),
        PathBuf::from("src/resizable/view.rs"),
        PathBuf::from("src/ripple/view.rs"),
        PathBuf::from("src/scroll_area/view.rs"),
        PathBuf::from("src/scroll_shadow/view.rs"),
        PathBuf::from("src/spinner/view.rs"),
        PathBuf::from("../../components/swatch/src/view.rs"),
        PathBuf::from("src/text_input/text_area/view.rs"),
        PathBuf::from("src/text_input/text_field/view.rs"),
        PathBuf::from("src/text_input/textarea/view.rs"),
        PathBuf::from("src/tree/view.rs"),
    ]
    .into_iter()
    .collect();

    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);
    files.sort();

    let mut violations = Vec::new();
    for file in &files {
        let rel = file
            .strip_prefix(manifest_dir)
            .unwrap_or(file)
            .to_path_buf();
        let hits = find_hits(manifest_dir, file, "style=");
        if hits.is_empty() {
            continue;
        }
        if !allowed_paths.contains(&rel) {
            violations.extend(hits);
        }
    }

    if violations.is_empty() {
        return;
    }

    let mut allowed: Vec<String> = allowed_paths
        .iter()
        .map(|p| p.display().to_string())
        .collect();
    allowed.sort();

    let mut msg = String::new();
    msg.push_str("`style=` is only allowed in these files:\n");
    for path in allowed {
        msg.push_str(&format!("- {path}\n"));
    }
    msg.push('\n');
    msg.push_str(&format_hits(
        "Found forbidden `style=` usages in non-allowed files:",
        &violations,
    ));
    panic!("{msg}");
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn text_line_height_legacy_ratios_are_allowlisted_and_no_new_ones() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let files = collect_workspace_style_rs_files(workspace_dir);

    let policy: [(&str, HashSet<PathBuf>); 3] = [
        (
            "line-height: 1.2;",
            [
                "components/action-bar/src/styles.rs",
                "components/alert/src/styles.rs",
                "components/alert-dialog/src/styles.rs",
                "components/autocomplete/src/styles.rs",
                "components/checkbox/src/styles.rs",
                "components/code/src/styles.rs",
                "crates/ui/src/color/area/styles.rs",
                "components/color-field/src/styles.rs",
                "components/combo-box/src/styles.rs",
                "crates/ui/src/command/styles.rs",
                "components/dialog/src/styles.rs",
                "components/disclosure/src/styles.rs",
                "components/drawer/src/styles.rs",
                "components/modal/src/styles.rs",
                "crates/ui/src/radio/styles.rs",
                "components/sidebar/src/group/styles.rs",
                "components/status-light/src/styles.rs",
                "crates/ui/src/step_list/styles.rs",
                "components/switch/src/styles.rs",
                "components/toast/src/toast/styles.rs",
                "components/tooltip/src/styles.rs",
                "crates/ui/src/tray/styles.rs",
            ]
            .into_iter()
            .map(PathBuf::from)
            .collect(),
        ),
        (
            "line-height: 1.3;",
            [
                "components/autocomplete/src/styles.rs",
                "components/checkbox/src/styles.rs",
                "components/color-field/src/styles.rs",
                "components/combo-box/src/styles.rs",
                "crates/ui/src/command/styles.rs",
                "components/form-field/src/styles.rs",
                "components/preview-card/src/styles.rs",
                "components/preview-link-card/src/styles.rs",
            ]
            .into_iter()
            .map(PathBuf::from)
            .collect(),
        ),
        (
            "line-height: 1;",
            [
                "components/action-bar/src/styles.rs",
                "components/avatar/src/styles.rs",
                "components/badge/src/styles.rs",
                "components/breadcrumb/src/styles.rs",
                "components/chip/src/styles.rs",
                "components/code-block/src/styles.rs",
                "components/color-field/src/styles.rs",
                "crates/ui/src/command/styles.rs",
                "components/error-view/src/styles.rs",
                "crates/ui/src/field_form/field_error/styles.rs",
                "components/help-text/src/styles.rs",
                "crates/ui/src/icon/styles.rs",
                "components/kbd/src/styles.rs",
                "components/sidebar/src/menu/styles.rs",
                "components/sidebar/src/menu_action/styles.rs",
                "components/snippet/src/styles.rs",
                "components/tabs/src/styles.rs",
                "components/tag/src/styles.rs",
                "components/text-input/src/number/styles.rs",
            ]
            .into_iter()
            .map(PathBuf::from)
            .collect(),
        ),
    ];

    let mut violations = Vec::new();
    for file in &files {
        for (needle, allowed_paths) in &policy {
            for hit in find_hits(workspace_dir, file, needle) {
                if !allowed_paths.contains(&hit.rel_path) {
                    violations.push(hit);
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "{}",
        format_hits(
            "Found non-allowlisted legacy `line-height` ratios in style files. Use typography tokens (for example `--ui-line-height-100/150/200`) instead:",
            &violations
        )
    );
}
