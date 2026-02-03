use std::collections::BTreeSet;

pub fn resolve_rel(target: Option<&'static str>, rel: Option<String>) -> Option<String> {
    let mut tokens: BTreeSet<String> = BTreeSet::new();

    if let Some(rel) = rel {
        for token in rel.split_whitespace() {
            let token = token.trim();
            if !token.is_empty() {
                tokens.insert(token.to_string());
            }
        }
    }

    if matches!(target, Some("_blank")) {
        tokens.insert("noopener".to_string());
        tokens.insert("noreferrer".to_string());
    }

    if tokens.is_empty() {
        None
    } else {
        Some(tokens.into_iter().collect::<Vec<_>>().join(" "))
    }
}
