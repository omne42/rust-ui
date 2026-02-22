pub(super) fn bool_word(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

pub(super) fn push_line_when(lines: &mut Vec<String>, condition: bool, line: impl Into<String>) {
    if condition {
        lines.push(line.into());
    }
}

pub(super) fn rust_string_literal(value: &str) -> String {
    format!("{value:?}")
}
