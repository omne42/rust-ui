#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccordionStreamingItem {
    pub label: String,
    pub text: String,
    pub is_complete: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccordionStreamingProjection {
    pub has_root_start: bool,
    pub has_root_open: bool,
    pub has_root_close: bool,
    pub items: Vec<AccordionStreamingItem>,
}

impl AccordionStreamingProjection {
    pub fn is_complete(&self) -> bool {
        self.has_root_open && self.has_root_close && self.items.iter().all(|item| item.is_complete)
    }
}

const ACCORDION_ROOT_START: &str = "<Accordion";
const ACCORDION_ROOT_END: &str = ">";
const ACCORDION_ROOT_CLOSE: &str = "</Accordion>";
const ITEM_START: &str = "<AccordionItem label=\"";
const ITEM_OPEN_END: &str = ">";
const ITEM_CLOSE: &str = "</AccordionItem>";

fn extract_item_text(raw: &str) -> String {
    let trimmed = raw.trim_start();

    if let Some(rest) = trimmed.strip_prefix('"') {
        if let Some(end_quote) = rest.find('"') {
            return rest[..end_quote].to_string();
        }
        if let Some(tag_start) = rest.find('<') {
            return rest[..tag_start].to_string();
        }
        return rest.to_string();
    }

    if let Some(tag_start) = trimmed.find('<') {
        return trimmed[..tag_start].to_string();
    }

    trimmed.to_string()
}

pub fn project_streaming_accordion_markup(input: &str) -> AccordionStreamingProjection {
    let has_root_start = input.contains(ACCORDION_ROOT_START);
    let has_root_open = if let Some(start) = input.find(ACCORDION_ROOT_START) {
        input[start..].contains(ACCORDION_ROOT_END)
    } else {
        false
    };
    let has_root_close = input.contains(ACCORDION_ROOT_CLOSE);

    let mut items = Vec::new();
    let mut cursor = 0_usize;

    while let Some(start_rel) = input[cursor..].find(ITEM_START) {
        let label_start = cursor + start_rel + ITEM_START.len();

        let Some(label_end_rel) = input[label_start..].find('"') else {
            break;
        };
        let label_end = label_start + label_end_rel;
        let label = input[label_start..label_end].to_string();

        let after_label = label_end + 1;
        let Some(open_end_rel) = input[after_label..].find(ITEM_OPEN_END) else {
            items.push(AccordionStreamingItem {
                label,
                text: String::new(),
                is_complete: false,
            });
            break;
        };
        let text_start = after_label + open_end_rel + ITEM_OPEN_END.len();

        if let Some(close_rel) = input[text_start..].find(ITEM_CLOSE) {
            let close_start = text_start + close_rel;
            let text = extract_item_text(&input[text_start..close_start]);
            items.push(AccordionStreamingItem {
                label,
                text,
                is_complete: true,
            });
            cursor = close_start + ITEM_CLOSE.len();
        } else {
            let text = extract_item_text(&input[text_start..]);
            items.push(AccordionStreamingItem {
                label,
                text,
                is_complete: false,
            });
            break;
        }
    }

    AccordionStreamingProjection {
        has_root_start,
        has_root_open,
        has_root_close,
        items,
    }
}

#[cfg(test)]
mod tests {
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
}
