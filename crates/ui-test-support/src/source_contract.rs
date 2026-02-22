use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

static CACHE: LazyLock<Mutex<HashMap<PathBuf, &'static str>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn find_workspace_root(start: &Path) -> PathBuf {
    let mut cursor = start.to_path_buf();
    loop {
        let manifest = cursor.join("Cargo.toml");
        if manifest.is_file() {
            let content = fs::read_to_string(&manifest).unwrap_or_else(|err| {
                panic!("failed to read {manifest:?}: {err}");
            });
            if content.contains("[workspace]") {
                return cursor;
            }
        }

        if !cursor.pop() {
            panic!("workspace root not found from {start:?}");
        }
    }
}

fn parse_include_path(source: &str, include_start: usize) -> Option<(PathBuf, usize)> {
    let needle = "include!(\"";
    let path_start = include_start + needle.len();
    let tail = &source[path_start..];
    let quote_end = tail.find('"')?;
    let path = PathBuf::from(&tail[..quote_end]);
    let mut cursor = path_start + quote_end + 1;

    let bytes = source.as_bytes();
    while cursor < source.len() && bytes[cursor].is_ascii_whitespace() {
        cursor += 1;
    }
    if cursor < source.len() && bytes[cursor] == b')' {
        cursor += 1;
    } else {
        return None;
    }
    while cursor < source.len() && bytes[cursor].is_ascii_whitespace() {
        cursor += 1;
    }
    if cursor < source.len() && bytes[cursor] == b';' {
        cursor += 1;
    }

    Some((path, cursor))
}

fn flatten_includes(path: &Path, stack: &mut Vec<PathBuf>) -> String {
    let canonical = fs::canonicalize(path).unwrap_or_else(|err| {
        panic!("failed to canonicalize {path:?}: {err}");
    });
    if stack.iter().any(|entry| entry == &canonical) {
        panic!("include! cycle detected while reading {canonical:?}");
    }

    stack.push(canonical);
    let source = fs::read_to_string(path).unwrap_or_else(|err| {
        panic!("failed to read {path:?}: {err}");
    });

    let mut out = String::new();
    let mut cursor = 0usize;

    while let Some(rel_start) = source[cursor..].find("include!(\"") {
        let include_start = cursor + rel_start;
        out.push_str(&source[cursor..include_start]);

        let Some((include_rel, next_cursor)) = parse_include_path(&source, include_start) else {
            out.push_str(&source[include_start..]);
            cursor = source.len();
            break;
        };

        let include_path = path
            .parent()
            .unwrap_or_else(|| panic!("path has no parent: {path:?}"))
            .join(include_rel);
        out.push_str(&flatten_includes(&include_path, stack));
        cursor = next_cursor;
    }

    if cursor < source.len() {
        out.push_str(&source[cursor..]);
    }

    stack.pop();
    out
}

pub fn source_from_path(path: &Path) -> String {
    flatten_includes(path, &mut Vec::new())
}

pub fn source_from_file_relative(caller_file: &str, relative_path: &str) -> &'static str {
    let workspace_root = find_workspace_root(Path::new(env!("CARGO_MANIFEST_DIR")));
    let caller_file_path = Path::new(caller_file);
    let caller_abs = if caller_file_path.is_absolute() {
        caller_file_path.to_path_buf()
    } else {
        let workspace_relative = workspace_root.join(caller_file_path);
        if workspace_relative.is_file() {
            workspace_relative
        } else {
            Path::new(env!("CARGO_MANIFEST_DIR")).join(caller_file_path)
        }
    };
    let file_abs = caller_abs
        .parent()
        .unwrap_or_else(|| panic!("caller file has no parent: {caller_abs:?}"))
        .join(relative_path);

    let canonical = fs::canonicalize(&file_abs).unwrap_or_else(|err| {
        panic!("failed to canonicalize {file_abs:?}: {err}");
    });

    {
        let cache = CACHE.lock().unwrap_or_else(|err| {
            panic!("source cache lock poisoned: {err}");
        });
        if let Some(cached) = cache.get(&canonical) {
            return cached;
        }
    }

    let within_workspace = canonical.strip_prefix(&workspace_root).unwrap_or_else(|_| {
        panic!("path {canonical:?} is not under workspace root {workspace_root:?}");
    });

    let flattened = flatten_includes(&workspace_root.join(within_workspace), &mut Vec::new());
    let leaked: &'static str = Box::leak(flattened.into_boxed_str());

    let mut cache = CACHE.lock().unwrap_or_else(|err| {
        panic!("source cache lock poisoned: {err}");
    });
    cache.insert(canonical, leaked);
    leaked
}
