use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn provider_module_reexports_root_ui_root_as_provider() {
    let source = load_source("src/provider/mod.rs");

    assert!(
        source.contains("pub use crate::root::UiRoot as Provider;"),
        "provider module should expose `Provider` as an alias of `UiRoot`."
    );
}

#[test]
fn crate_root_registers_provider_module_and_alias() {
    let source = load_source("src/lib.rs");

    for needle in ["pub mod provider;", "pub use provider::Provider;"] {
        assert!(
            source.contains(needle),
            "crate root should include `{needle}` for react-spectrum Provider compatibility."
        );
    }
}

#[test]
fn provider_compatibility_reuses_ui_root_docs_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "pub(super) fn ui_root() -> AnyView",
        "title=\"UiRoot\"",
        "slug=\"ui-root\"",
        "<UiRoot",
    ] {
        assert!(
            source.contains(needle),
            "layout ui_root docs should contain `{needle}` for Provider compatibility coverage."
        );
    }
}

#[test]
fn provider_module_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    for needle in [
        "pub(super) fn ui_root() -> AnyView",
        "title=\"UiRoot\"",
        "slug=\"ui-root\"",
        "description=\"Provider that injects theme tokens + layered component CSS and exposes stable root state attrs.\"",
        "<Playground title=\"Usage\" code_signal=usage_code>",
        "<Playground title=\"State Contract\" code_signal=contract_code>",
        "<UiRoot",
    ] {
        assert!(
            source.contains(needle),
            "layout docs should include `{needle}` for provider module primary playground coverage.",
        );
    }

    assert!(
        mod_source.contains("\"provider\" => &[\"ui-root\"]"),
        "components mod mapping should keep `provider` mapped to `ui-root` slug.",
    );
}

#[test]
fn provider_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout.rs");

    for needle in [
        "title=\"Usage\"",
        "let theme = Signal::derive(|| Theme::dark());",
        "<UiRoot theme=theme safe_area=true>",
        "This docs app already mounts a global UiRoot at startup.",
        "UiRoot injects BASE_CSS + theme CSS variables + component CSS in one place.",
        "safe_area=true adds the safe-area inset contract used on mobile/WebView shells.",
        "title=\"State Contract\"",
        "data-slot=\"ui-root\"",
        "data-theme-scheme=\"light|dark\"",
        "data-state=\"default|safe-area\"",
        "data-safe-area=\"true\"",
        "`data-slot=ui-root` for stable root targeting.",
        "`data-theme-scheme` mirrors `Theme::scheme` (`light`/`dark`).",
        "`data-state` + `data-safe-area` describe safe-area mode.",
    ] {
        assert!(
            source.contains(needle),
            "layout docs playgrounds should contain `{needle}` for provider module contracts.",
        );
    }
}
