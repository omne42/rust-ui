use super::*;

#[test]
fn projection_detects_partial_first_item() {
    let input = r#"<Accordion
  id_base="docs-accordion-ai-stream".to_string()
  selection_mode=AccordionSelectionMode::Multiple
>
    <AccordionItem label="Chunk #1">"First part"#;

    let projection = project_streaming_accordion_markup(input);
    assert!(projection.has_root_start);
    assert!(projection.has_root_open);
    assert!(!projection.has_root_close);
    assert_eq!(projection.items.len(), 1);
    assert_eq!(projection.items[0].label, "Chunk #1");
    assert_eq!(projection.items[0].text, "First part");
    assert!(!projection.items[0].is_complete);
    assert!(!projection.is_complete());
}

#[test]
fn projection_detects_open_item_without_text() {
    let input = r#"<Accordion
  id_base="docs-accordion-ai-stream".to_string()
  selection_mode=AccordionSelectionMode::Multiple
>
    <AccordionItem label="Chunk #1">"#;

    let projection = project_streaming_accordion_markup(input);
    assert!(projection.has_root_start);
    assert!(projection.has_root_open);
    assert!(!projection.has_root_close);
    assert_eq!(projection.items.len(), 1);
    assert_eq!(projection.items[0].label, "Chunk #1");
    assert_eq!(projection.items[0].text, "");
    assert!(!projection.items[0].is_complete);
}

#[test]
fn projection_detects_complete_markup() {
    let input = r#"<Accordion
  id_base="docs-accordion-ai-stream".to_string()
  selection_mode=AccordionSelectionMode::Multiple
>
    <AccordionItem label="Chunk #1">"A"</AccordionItem>
    <AccordionItem label="Chunk #2">"B"</AccordionItem>
    <AccordionItem label="Chunk #3">"C"</AccordionItem>
</Accordion>"#;

    let projection = project_streaming_accordion_markup(input);
    assert!(projection.has_root_start);
    assert!(projection.has_root_open);
    assert!(projection.has_root_close);
    assert_eq!(projection.items.len(), 3);
    assert!(projection.items.iter().all(|item| item.is_complete));
    assert!(projection.is_complete());
}

#[test]
fn projection_keeps_item_order_when_last_item_is_partial() {
    let full = r#"<Accordion
  id_base="docs-accordion-ai-stream".to_string()
  selection_mode=AccordionSelectionMode::Multiple
>
    <AccordionItem label="Chunk #1">"First completed item from AI output."</AccordionItem>
    <AccordionItem label="Chunk #2">"Second completed item, mounted incrementally."</AccordionItem>
    <AccordionItem label="Chunk #3">"Final completed item."</AccordionItem>
</Accordion>"#;
    let partial: String = full.chars().take(360).collect();

    let projection = project_streaming_accordion_markup(&partial);
    assert_eq!(projection.items.len(), 3);
    assert_eq!(
        projection.items[0].text,
        "First completed item from AI output."
    );
    assert_eq!(
        projection.items[1].text,
        "Second completed item, mounted incrementally."
    );
    assert!(projection.items[2].text.starts_with("Final completed item"));
    assert!(!projection.items[2].is_complete);
}

#[test]
fn projection_preserves_partial_text_for_first_item_prefix() {
    let input = r#"<Accordion
  id_base="docs-accordion-ai-stream".to_string()
  selection_mode=AccordionSelectionMode::Multiple
>
    <AccordionItem label="Chunk #1">"First compl"#;

    let projection = project_streaming_accordion_markup(input);
    assert_eq!(projection.items.len(), 1);
    assert_eq!(projection.items[0].label, "Chunk #1");
    assert_eq!(projection.items[0].text, "First compl");
    assert!(!projection.items[0].is_complete);
}

#[test]
fn projection_ignores_partial_closing_tag_fragment_in_text() {
    let input = r#"<Accordion
  id_base="docs-accordion-ai-stream".to_string()
  selection_mode=AccordionSelectionMode::Multiple
>
    <AccordionItem label="Chunk #1">"First completed item from AI output."</AccordionIte"#;

    let projection = project_streaming_accordion_markup(input);
    assert_eq!(projection.items.len(), 1);
    assert_eq!(
        projection.items[0].text,
        "First completed item from AI output."
    );
    assert!(!projection.items[0].is_complete);
}
