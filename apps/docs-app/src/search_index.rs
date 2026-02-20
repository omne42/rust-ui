use crate::route;
use crate::toc::TocItem;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchKind {
    DocPage,
    DocSection,
    Component,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchRecord {
    pub kind: SearchKind,
    pub key: String,
    pub title: String,
    pub subtitle: String,
    pub route: String,
    pub route_label: String,
    pub content: String,
    title_lower: String,
    route_label_lower: String,
    haystack: String,
    content_lower: String,
}

#[derive(Clone, Debug)]
struct Query {
    lower: String,
    tokens: Vec<String>,
}

pub fn build_records() -> Vec<SearchRecord> {
    let mut out = Vec::new();

    for doc in crate::pages::docs::docs_catalog() {
        let mut content = String::new();

        let sections = doc.markdown.map(|markdown| {
            let rendered = crate::markdown::render_markdown(markdown);
            content = rendered.text;
            doc_section_records_from_toc(doc, rendered.toc)
        });

        out.push(SearchRecord::new(
            SearchKind::DocPage,
            format!("doc:{}", doc.route),
            doc.title.into(),
            doc.group.into(),
            doc.route.into(),
            doc.route.into(),
            content,
        ));

        if let Some(sections) = sections {
            out.extend(sections);
        }
    }

    for doc in crate::pages::components::component_catalog() {
        let route = format!("components/{}", doc.slug);
        out.push(SearchRecord::new(
            SearchKind::Component,
            format!("component:{}", doc.slug),
            doc.name.into(),
            doc.group.into(),
            route.clone(),
            route,
            String::new(),
        ));
    }

    out
}

pub fn search(records: &[SearchRecord], query: &str, limit: usize) -> Vec<usize> {
    let query = Query::new(query);
    if query.lower.is_empty() {
        return records
            .iter()
            .enumerate()
            .filter_map(|(idx, record)| {
                if record.kind == SearchKind::DocSection {
                    None
                } else {
                    Some(idx)
                }
            })
            .take(limit)
            .collect();
    }

    let mut scored = Vec::new();
    for (idx, record) in records.iter().enumerate() {
        if let Some(score) = record.score(&query) {
            scored.push((idx, score));
        }
    }

    scored.sort_by(|(a_idx, a_score), (b_idx, b_score)| {
        let Some(a) = records.get(*a_idx) else {
            return Ordering::Equal;
        };
        let Some(b) = records.get(*b_idx) else {
            return Ordering::Equal;
        };

        b_score
            .cmp(a_score)
            .then_with(|| a.title.len().cmp(&b.title.len()))
            .then_with(|| a.key.cmp(&b.key))
    });

    scored.into_iter().take(limit).map(|(idx, _)| idx).collect()
}

impl SearchRecord {
    fn new(
        kind: SearchKind,
        key: String,
        title: String,
        subtitle: String,
        route: String,
        route_label: String,
        content: String,
    ) -> Self {
        let title_lower = title.to_lowercase();
        let route_label_lower = route_label.to_lowercase();
        let content = normalize_whitespace(&content);
        let content_lower = content.to_lowercase();
        let haystack = format!("{title} {subtitle} {route_label}").to_lowercase();
        Self {
            kind,
            key,
            title,
            subtitle,
            route,
            route_label,
            content,
            title_lower,
            route_label_lower,
            haystack,
            content_lower,
        }
    }

    fn score(&self, query: &Query) -> Option<i32> {
        if query.lower.is_empty() {
            return Some(0);
        }

        if self.title_lower == query.lower {
            return Some(10_000);
        }

        if self.title_lower.starts_with(&query.lower) {
            let delta = (self.title_lower.len().saturating_sub(query.lower.len())) as i32;
            return Some(9_000 - delta.min(500));
        }

        if let Some(pos) = self.title_lower.find(&query.lower) {
            return Some(8_000 - (pos as i32).min(500));
        }

        let mut token_score = None;
        if !query.tokens.is_empty() {
            if query.tokens.iter().all(|t| self.haystack.contains(t)) {
                token_score = Some(7_000);
            } else if query.tokens.iter().all(|t| self.content_lower.contains(t)) {
                token_score = Some(3_600);
            }
        }

        let fuzzy_title =
            fuzzy_subsequence_score(&query.lower, &self.title_lower).map(|value| 6_000 + value);
        let fuzzy_route = fuzzy_subsequence_score(&query.lower, &self.route_label_lower)
            .map(|value| 5_500 + value);

        let mut best = token_score;
        best = max_option(best, fuzzy_title);
        best = max_option(best, fuzzy_route);

        if let Some(pos) = self.content_lower.find(&query.lower) {
            best = max_option(best, Some(3_500 - (pos as i32).min(500)));
        }

        if best.is_none()
            && (self.haystack.contains(&query.lower) || self.content_lower.contains(&query.lower))
        {
            best = Some(4_000);
        }

        best
    }
}

impl Query {
    fn new(raw: &str) -> Self {
        let lower = raw.trim().to_lowercase();
        let tokens = tokenize(&lower);
        Self { lower, tokens }
    }
}

fn tokenize(query: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();

    for ch in query.chars() {
        if ch.is_alphanumeric() {
            buf.push(ch);
            continue;
        }

        if !buf.is_empty() {
            out.push(std::mem::take(&mut buf));
        }
    }

    if !buf.is_empty() {
        out.push(buf);
    }

    out
}

fn fuzzy_subsequence_score(needle: &str, haystack: &str) -> Option<i32> {
    if needle.is_empty() {
        return Some(0);
    }

    let mut needle_iter = needle.chars();
    let mut current = needle_iter.next()?;

    let mut first_match = None::<usize>;
    let mut prev_match = None::<usize>;
    let mut gaps: i32 = 0;
    let mut matched = 0_usize;

    for (pos, ch) in haystack.chars().enumerate() {
        if ch != current {
            continue;
        }

        if first_match.is_none() {
            first_match = Some(pos);
        }

        if let Some(prev) = prev_match {
            gaps += pos.saturating_sub(prev.saturating_add(1)) as i32;
        }

        prev_match = Some(pos);
        matched += 1;

        match needle_iter.next() {
            Some(next) => current = next,
            None => break,
        }
    }

    if matched != needle.chars().count() {
        return None;
    }

    let first_match = first_match.unwrap_or(0) as i32;
    let len_penalty = (haystack.chars().count().saturating_sub(matched) as i32).min(400);

    // Higher is better.
    Some(1_000 - first_match * 8 - gaps * 4 - len_penalty)
}

fn max_option(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn normalize_whitespace(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for word in input.split_whitespace() {
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(word);
    }
    out
}

fn doc_section_records_from_toc(
    doc: &crate::pages::docs::DocPage,
    toc: Vec<TocItem>,
) -> Vec<SearchRecord> {
    let mut out = Vec::new();

    for item in toc {
        if item.level < 2 || item.level > 3 {
            continue;
        }

        let route = route::route_with_section(doc.route, &item.id);
        let route_label = format!("{}#{}", doc.route, item.id);
        let subtitle = format!("{} · {}", doc.group, doc.title);

        out.push(SearchRecord::new(
            SearchKind::DocSection,
            format!("doc-section:{}#{}", doc.route, item.id),
            item.title,
            subtitle,
            route,
            route_label,
            String::new(),
        ));
    }

    out
}

#[cfg(test)]
#[path = "test/search_index.rs"]
mod tests;
