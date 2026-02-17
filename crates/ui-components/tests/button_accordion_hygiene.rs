use std::fs;
use std::path::{Path, PathBuf};

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir).unwrap_or_else(|error| {
        panic!("failed to read {}: {error}", dir.display());
    });

    for entry in entries {
        let entry = entry.unwrap_or_else(|error| {
            panic!("failed to read entry under {}: {error}", dir.display());
        });
        let path = entry.path();
        let file_type = entry.file_type().unwrap_or_else(|error| {
            panic!("failed to read file type for {}: {error}", path.display());
        });

        if file_type.is_dir() {
            collect_rs_files(&path, out);
            continue;
        }

        if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

fn non_test_source(content: &str) -> &str {
    content.split("\n#[cfg(test)]").next().unwrap_or(content)
}

fn component_sources() -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    collect_rs_files(&root.join("button"), &mut files);
    collect_rs_files(&root.join("accordion"), &mut files);
    files.sort();
    files
}

fn find_forbidden_lines<F>(predicate: F) -> Vec<String>
where
    F: Fn(&str) -> bool,
{
    let mut failures = Vec::new();

    for path in component_sources() {
        let content = fs::read_to_string(&path).unwrap_or_else(|error| {
            panic!("failed to read {}: {error}", path.display());
        });
        let non_test = non_test_source(&content);

        for (index, line) in non_test.lines().enumerate() {
            if predicate(line) {
                failures.push(format!("{}:{}: {}", path.display(), index + 1, line.trim()));
            }
        }
    }

    failures
}

#[test]
fn button_and_accordion_non_test_code_forbids_unwrap_and_expect() {
    let failures = find_forbidden_lines(|line| {
        let compact = line.replace(' ', "");
        compact.contains(".unwrap(")
            || compact.contains("unwrap(")
            || compact.contains(".expect(")
            || compact.contains("expect(")
    });

    assert!(
        failures.is_empty(),
        "forbidden unwrap/expect in non-test code:\n{}",
        failures.join("\n")
    );
}

#[test]
fn button_and_accordion_non_test_code_forbids_let_result_swallowing() {
    let failures = find_forbidden_lines(|line| {
        if !line.contains("let _ =") {
            return false;
        }

        // Non-wasm motion stub keeps sanitize marker intentionally.
        !line.contains("let _ = sanitize_motion(motion);")
    });

    assert!(
        failures.is_empty(),
        "forbidden 'let _ =' in non-test code:\n{}",
        failures.join("\n")
    );
}
