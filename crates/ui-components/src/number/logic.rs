#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NumberFormatOptions<'a> {
    pub pad_start: bool,
    pub decimal_separator: &'a str,
    pub decimal_places: Option<u32>,
    pub thousand_separator: Option<&'a str>,
}

impl<'a> Default for NumberFormatOptions<'a> {
    fn default() -> Self {
        Self {
            pad_start: false,
            decimal_separator: ".",
            decimal_places: None,
            thousand_separator: None,
        }
    }
}

fn insert_thousand_separators(int_part: &str, sep: &str) -> String {
    if sep.is_empty() {
        return int_part.to_string();
    }

    let mut out = String::with_capacity(int_part.len() + int_part.len() / 3);
    let chars: Vec<char> = int_part.chars().collect();
    let len = chars.len();
    for (idx, ch) in chars.into_iter().enumerate() {
        out.push(ch);
        let remaining = len - idx - 1;
        if remaining > 0 && remaining.is_multiple_of(3) {
            out.push_str(sep);
        }
    }
    out
}

pub fn format_static_number(value: f64, options: NumberFormatOptions<'_>) -> String {
    let value = if value.is_finite() { value } else { 0.0 };
    let is_negative = value < 0.0;
    let abs_value = value.abs();

    let mut number = if let Some(places) = options.decimal_places {
        format!("{:.*}", places as usize, abs_value)
    } else {
        abs_value.to_string()
    };

    if options.decimal_separator != "."
        && let Some(dot) = number.find('.')
    {
        number.replace_range(dot..=dot, options.decimal_separator);
    }

    let (int_part, dec_part) = match number.split_once(options.decimal_separator) {
        Some((int_part, dec_part)) => (int_part, Some(dec_part)),
        None => (number.as_str(), None),
    };

    let int_part = if options.pad_start {
        int_part.to_string()
    } else {
        int_part.trim_start_matches('0').to_string()
    };
    let int_part = if int_part.is_empty() {
        "0".to_string()
    } else {
        int_part
    };

    let int_part = if let Some(sep) = options.thousand_separator {
        insert_thousand_separators(&int_part, sep)
    } else {
        int_part
    };

    let mut out = String::new();
    if is_negative {
        out.push('-');
    }
    out.push_str(&int_part);
    if let Some(dec_part) = dec_part
        && !dec_part.is_empty()
    {
        out.push_str(options.decimal_separator);
        out.push_str(dec_part);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_negative_and_decimals() {
        let out = format_static_number(
            -12.345,
            NumberFormatOptions {
                decimal_places: Some(2),
                ..Default::default()
            },
        );
        assert_eq!(out, "-12.35");
    }

    #[test]
    fn supports_thousand_separator() {
        let out = format_static_number(
            12345.0,
            NumberFormatOptions {
                thousand_separator: Some(","),
                ..Default::default()
            },
        );
        assert_eq!(out, "12,345");
    }

    #[test]
    fn supports_custom_decimal_separator() {
        let out = format_static_number(
            1.5,
            NumberFormatOptions {
                decimal_separator: ",",
                ..Default::default()
            },
        );
        assert_eq!(out, "1,5");
    }
}
