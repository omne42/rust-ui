use super::*;
use leptos::prelude::IntoAny;

#[test]
fn tokenize_splits_on_non_alphanumeric() {
    assert_eq!(tokenize("docs/spec/motion"), vec!["docs", "spec", "motion"]);
    assert_eq!(tokenize("中文-按钮"), vec!["中文", "按钮"]);
    assert_eq!(tokenize(""), Vec::<String>::new());
}

#[test]
fn fuzzy_subsequence_matches_abbreviations() {
    assert!(fuzzy_subsequence_score("btn", "button").is_some());
    assert!(fuzzy_subsequence_score("abc", "a_b_c").is_some());
    assert!(fuzzy_subsequence_score("zz", "button").is_none());
}

#[test]
fn search_prefers_exact_and_prefix_matches() {
    let records = vec![
        SearchRecord::new(
            SearchKind::Component,
            "a".to_string(),
            "Button".to_string(),
            "Actions".to_string(),
            "components/button".to_string(),
            "components/button".to_string(),
            String::new(),
        ),
        SearchRecord::new(
            SearchKind::Component,
            "b".to_string(),
            "ToggleButton".to_string(),
            "Actions".to_string(),
            "components/toggle-button".to_string(),
            "components/toggle-button".to_string(),
            String::new(),
        ),
    ];

    let res = search(&records, "button", 10);
    assert_eq!(res, vec![0, 1]);

    let res = search(&records, "toggle", 10);
    assert_eq!(res, vec![1]);
}

#[test]
fn search_matches_doc_content_with_lower_priority_than_title() {
    let records = vec![
        SearchRecord::new(
            SearchKind::DocPage,
            "doc".to_string(),
            "Rules".to_string(),
            "Docs".to_string(),
            "docs/rules".to_string(),
            "docs/rules".to_string(),
            "clippy --deny warnings zero unsafe".to_string(),
        ),
        SearchRecord::new(
            SearchKind::Component,
            "component".to_string(),
            "Tooltip".to_string(),
            "Overlays".to_string(),
            "components/tooltip".to_string(),
            "components/tooltip".to_string(),
            String::new(),
        ),
    ];

    let res = search(&records, "unsafe", 10);
    assert_eq!(res, vec![0]);

    let res = search(&records, "tooltip", 10);
    assert_eq!(res, vec![1]);
}

#[test]
fn doc_section_records_build_section_routes() {
    let doc = crate::pages::docs::DocPage {
        title: "Test Doc",
        route: "docs/test",
        group: "Spec",
        page: || ().into_any(),
        markdown: None,
    };

    let markdown = r#"# Title

## Section A

Text

### Sub A1
"#;

    let toc = crate::markdown::render_markdown(markdown).toc;
    let sections = doc_section_records_from_toc(&doc, toc);
    assert_eq!(sections.len(), 2);
    assert!(sections[0].route.starts_with("docs/test?section="));
    assert!(sections[0].route_label.starts_with("docs/test#"));
}
