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
        PathBuf::from("src/popover/view.rs"),
        PathBuf::from("src/circular_progress/view.rs"),
        PathBuf::from("src/tooltip/view.rs"),
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
