use std::fs;
use std::path::Path;

fn read(rel_path: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path:?}: {err}"))
}

#[test]
fn docs_index_loads_dev_overrides_css_after_base_css() {
    let index_html = read("index.html");

    let app_link = "<link data-trunk rel=\"css\" href=\"app.css\" />";
    let overrides_link = "<link data-trunk rel=\"css\" href=\"dev-overrides.css\" />";

    let app_pos = index_html.find(app_link).unwrap_or_else(|| {
        panic!("docs-app index.html should include `{app_link}` to load base docs styles")
    });
    let overrides_pos = index_html.find(overrides_link).unwrap_or_else(|| {
        panic!("docs-app index.html should include `{overrides_link}` for dev hot-style overrides")
    });

    assert!(
        overrides_pos > app_pos,
        "`dev-overrides.css` should be loaded after `app.css` so local dev tweaks win without rebuilding Rust code"
    );
}

#[test]
fn dev_overrides_file_documents_hot_style_workflow() {
    let overrides = read("dev-overrides.css");

    for needle in [
        "Fast, local styling iteration",
        "Loaded after `app.css`",
        "move them into the relevant component `styles.rs`",
    ] {
        assert!(
            overrides.contains(needle),
            "dev-overrides.css should document `{needle}` to keep the no-recompile style workflow explicit"
        );
    }
}
