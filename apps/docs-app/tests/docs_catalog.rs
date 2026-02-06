use std::collections::BTreeSet;

const REQUIRED_DOC_ROUTES: &[&str] = &[
    "docs/start",
    "docs/rules",
    "docs/spec/mvp",
    "docs/spec/styling",
    "docs/spec/motion",
    "docs/research",
    "docs/research/bb-ui-web-notes",
    "docs/research/android-spike",
];

#[test]
fn docs_catalog_has_required_routes_and_unique_entries() {
    let catalog = docs_app::pages::docs::docs_catalog();

    let mut routes = BTreeSet::new();
    for entry in catalog {
        assert!(!entry.title.trim().is_empty(), "DocPage.title 不能为空");
        assert!(
            !entry.route.trim().is_empty(),
            "DocPage.route 不能为空 ({})",
            entry.title
        );
        assert!(
            !entry.group.trim().is_empty(),
            "DocPage.group 不能为空 ({})",
            entry.title
        );
        assert!(
            routes.insert(entry.route),
            "DocPage.route 必须唯一，重复 route: {}",
            entry.route
        );
    }

    for required in REQUIRED_DOC_ROUTES {
        assert!(
            routes.contains(required),
            "docs catalog must include required route: {required}"
        );
    }
}
