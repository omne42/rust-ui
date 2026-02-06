#[test]
fn title_for_path_matches_known_routes() {
    assert_eq!(docs_app::pages::title_for_path("docs/rules"), "Rules");
    assert_eq!(docs_app::pages::title_for_path("components"), "Components");
    assert_eq!(
        docs_app::pages::title_for_path("components/button"),
        "Button"
    );
}

#[test]
fn title_for_path_falls_back_to_not_found() {
    assert_eq!(
        docs_app::pages::title_for_path("definitely/not/a/route"),
        "Not found"
    );
}
