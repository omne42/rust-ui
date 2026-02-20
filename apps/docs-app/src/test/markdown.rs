use super::*;

#[test]
fn render_markdown_adds_heading_ids_and_toc() {
    let markdown = r#"# Title

## Section A

Text.

### Sub A1

More.

## Section A
"#;

    let doc = render_markdown(markdown);
    assert!(doc.html.contains("id=\"section-a\""));
    assert!(doc.html.contains("id=\"sub-a1\""));
    assert!(doc.html.contains("id=\"section-a-2\""));
    assert_eq!(doc.toc.len(), 3);
    assert_eq!(doc.toc[0].id, "section-a");
    assert_eq!(doc.toc[1].id, "sub-a1");
    assert_eq!(doc.toc[2].id, "section-a-2");
    assert!(doc.text.contains("Section A"));
    assert!(doc.text.contains("Text."));
}
