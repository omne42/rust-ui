use crate::toc::TocItem;

pub struct MarkdownDoc {
    pub html: String,
    pub toc: Vec<TocItem>,
    pub text: String,
}

pub fn markdown_to_html(markdown: &str) -> String {
    render_markdown(markdown).html
}

pub fn render_markdown(markdown: &str) -> MarkdownDoc {
    use pulldown_cmark::{CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};
    use std::collections::BTreeMap;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let mut used_ids: BTreeMap<String, usize> = BTreeMap::new();
    let mut toc = Vec::new();
    let mut out_events: Vec<Event<'_>> = Vec::new();
    let mut plain_text = String::new();

    let mut parser = Parser::new_ext(markdown, options).peekable();

    while let Some(event) = parser.next() {
        match event {
            Event::Start(Tag::Heading {
                level,
                id,
                classes,
                attrs,
            }) => {
                let mut title_buf = String::new();
                let mut content_events = Vec::new();

                for inner in parser.by_ref() {
                    match &inner {
                        Event::Text(text) | Event::Code(text) => {
                            title_buf.push_str(text.as_ref());
                        }
                        Event::SoftBreak | Event::HardBreak => title_buf.push(' '),
                        _ => {}
                    }

                    if matches!(inner, Event::End(TagEnd::Heading(end_level)) if end_level == level)
                    {
                        break;
                    }

                    content_events.push(inner);
                }

                let title = title_buf.split_whitespace().collect::<Vec<_>>().join(" ");
                let title: String = title.trim().into();

                if !title.is_empty() {
                    plain_text.push_str(&title);
                    plain_text.push('\n');
                }

                let mut id_value: Option<String> =
                    id.map(|value| value.as_ref().trim_start_matches('#').into());
                if id_value.as_deref().unwrap_or_default().trim().is_empty() {
                    let base = slugify_id(&title);
                    id_value = Some(if base.is_empty() {
                        "section".to_string()
                    } else {
                        base
                    });
                }

                let mut id_value = id_value.unwrap_or_else(|| "section".to_string());
                let entry = used_ids.entry(id_value.clone()).or_insert(0);
                if *entry > 0 {
                    id_value = format!("{id_value}-{}", *entry + 1);
                }
                *entry += 1;

                let level_num = heading_level_to_u8(level);
                if matches!(level, HeadingLevel::H2 | HeadingLevel::H3) {
                    toc.push(TocItem {
                        id: id_value.clone(),
                        title: title.clone(),
                        level: level_num,
                    });
                }

                out_events.push(Event::Start(Tag::Heading {
                    level,
                    id: Some(CowStr::from(id_value)),
                    classes,
                    attrs,
                }));
                out_events.extend(content_events);
                out_events.push(Event::End(TagEnd::Heading(level)));
            }
            other => {
                match &other {
                    Event::Text(text) | Event::Code(text) => {
                        plain_text.push_str(text.as_ref());
                        plain_text.push(' ');
                    }
                    Event::SoftBreak | Event::HardBreak => plain_text.push('\n'),
                    Event::Rule => plain_text.push('\n'),
                    Event::End(TagEnd::Paragraph)
                    | Event::End(TagEnd::BlockQuote(_))
                    | Event::End(TagEnd::CodeBlock)
                    | Event::End(TagEnd::Item)
                    | Event::End(TagEnd::List(_)) => plain_text.push('\n'),
                    _ => {}
                }

                out_events.push(other);
            }
        }
    }

    let mut output = String::new();
    html::push_html(&mut output, out_events.into_iter());

    MarkdownDoc {
        html: output,
        toc,
        text: plain_text,
    }
}

fn heading_level_to_u8(level: pulldown_cmark::HeadingLevel) -> u8 {
    match level {
        pulldown_cmark::HeadingLevel::H1 => 1,
        pulldown_cmark::HeadingLevel::H2 => 2,
        pulldown_cmark::HeadingLevel::H3 => 3,
        pulldown_cmark::HeadingLevel::H4 => 4,
        pulldown_cmark::HeadingLevel::H5 => 5,
        pulldown_cmark::HeadingLevel::H6 => 6,
    }
}

fn slugify_id(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut prev_dash = false;

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if (ch.is_ascii_whitespace() || matches!(ch, '-' | '_' | '.'))
            && !out.is_empty()
            && !prev_dash
        {
            out.push('-');
            prev_dash = true;
        }
    }

    while out.ends_with('-') {
        out.pop();
    }

    out
}

#[cfg(test)]
mod tests {
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
}
