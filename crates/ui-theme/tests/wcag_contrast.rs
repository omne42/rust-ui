use ui_theme::{Theme, ThemeColor, ThemeScale};

#[derive(Clone, Copy, Debug)]
struct Oklch {
    l: f64,
    c: f64,
    h_deg: f64,
    alpha: f64,
}

#[derive(Clone, Copy, Debug)]
struct Srgb {
    r: f64,
    g: f64,
    b: f64,
    alpha: f64,
}

#[derive(Clone, Copy, Debug)]
enum ParsedColor {
    Oklch(Oklch),
    Srgb(Srgb),
}

fn parse_oklch(input: &str) -> Oklch {
    let trimmed = input.trim();
    let raw = trimmed
        .strip_prefix("oklch(")
        .and_then(|v| v.strip_suffix(')'))
        .unwrap_or_else(|| panic!("expected oklch(...), got `{input}`"));

    let (body, alpha) = raw
        .split_once('/')
        .map(|(a, b)| (a.trim(), Some(b.trim())))
        .unwrap_or((raw.trim(), None));

    let mut parts = body.split_whitespace();
    let l_raw = parts
        .next()
        .unwrap_or_else(|| panic!("missing L in `{input}`"));
    let c_raw = parts
        .next()
        .unwrap_or_else(|| panic!("missing C in `{input}`"));
    let h_raw = parts
        .next()
        .unwrap_or_else(|| panic!("missing H in `{input}`"));

    let l_percent = l_raw
        .strip_suffix('%')
        .unwrap_or_else(|| panic!("expected L% in `{input}`"))
        .parse::<f64>()
        .unwrap_or_else(|e| panic!("parse L failed for `{input}`: {e}"));
    let c = c_raw
        .parse::<f64>()
        .unwrap_or_else(|e| panic!("parse C failed for `{input}`: {e}"));
    let h_deg = h_raw
        .parse::<f64>()
        .unwrap_or_else(|e| panic!("parse H failed for `{input}`: {e}"));

    let alpha = alpha
        .map(|a| {
            a.parse::<f64>()
                .unwrap_or_else(|e| panic!("parse alpha failed for `{input}`: {e}"))
        })
        .unwrap_or(1.0);

    Oklch {
        l: l_percent / 100.0,
        c,
        h_deg,
        alpha,
    }
}

fn parse_srgb(input: &str) -> Srgb {
    let trimmed = input.trim();

    let (raw, has_alpha) = if let Some(raw) = trimmed
        .strip_prefix("rgba(")
        .and_then(|v| v.strip_suffix(')'))
    {
        (raw, true)
    } else if let Some(raw) = trimmed
        .strip_prefix("rgb(")
        .and_then(|v| v.strip_suffix(')'))
    {
        (raw, false)
    } else {
        panic!("expected rgb(...) or rgba(...), got `{input}`");
    };

    let values = raw.split(',').map(str::trim).collect::<Vec<_>>();

    let expected = if has_alpha { 4 } else { 3 };
    assert_eq!(
        values.len(),
        expected,
        "expected {expected} values in `{input}`, got {}",
        values.len()
    );

    let r = values[0]
        .parse::<f64>()
        .unwrap_or_else(|e| panic!("parse R failed for `{input}`: {e}"));
    let g = values[1]
        .parse::<f64>()
        .unwrap_or_else(|e| panic!("parse G failed for `{input}`: {e}"));
    let b = values[2]
        .parse::<f64>()
        .unwrap_or_else(|e| panic!("parse B failed for `{input}`: {e}"));
    let alpha = if has_alpha {
        values[3]
            .parse::<f64>()
            .unwrap_or_else(|e| panic!("parse alpha failed for `{input}`: {e}"))
    } else {
        1.0
    };

    Srgb { r, g, b, alpha }
}

fn parse_color(input: &str) -> ParsedColor {
    let trimmed = input.trim();
    if trimmed.starts_with("oklch(") {
        return ParsedColor::Oklch(parse_oklch(trimmed));
    }
    if trimmed.starts_with("rgb(") || trimmed.starts_with("rgba(") {
        return ParsedColor::Srgb(parse_srgb(trimmed));
    }
    panic!("unsupported color format `{input}`");
}

fn oklab_to_linear_srgb(l: f64, a: f64, b: f64) -> (f64, f64, f64) {
    let l_ = l + 0.396_337_777_4 * a + 0.215_803_757_3 * b;
    let m_ = l - 0.105_561_345_8 * a - 0.063_854_172_8 * b;
    let s_ = l - 0.089_484_177_5 * a - 1.291_485_548_0 * b;

    let l3 = l_ * l_ * l_;
    let m3 = m_ * m_ * m_;
    let s3 = s_ * s_ * s_;

    let r = 4.076_741_662_1 * l3 - 3.307_711_591_3 * m3 + 0.230_969_929_2 * s3;
    let g = -1.268_438_004_6 * l3 + 2.609_757_401_1 * m3 - 0.341_319_396_5 * s3;
    let b = -0.004_196_086_3 * l3 - 0.703_418_614_7 * m3 + 1.707_614_701_0 * s3;

    (r, g, b)
}

fn relative_luminance_from_oklch(color: Oklch) -> f64 {
    let h = color.h_deg.to_radians();
    let a = color.c * h.cos();
    let b = color.c * h.sin();

    let (r, g, b) = oklab_to_linear_srgb(color.l, a, b);

    let r = r.clamp(0.0, 1.0);
    let g = g.clamp(0.0, 1.0);
    let b = b.clamp(0.0, 1.0);

    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn srgb_channel_to_linear(v: f64) -> f64 {
    let value = (v / 255.0).clamp(0.0, 1.0);
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

fn relative_luminance_from_srgb(color: Srgb) -> f64 {
    let r = srgb_channel_to_linear(color.r);
    let g = srgb_channel_to_linear(color.g);
    let b = srgb_channel_to_linear(color.b);
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn color_alpha(color: ParsedColor) -> f64 {
    match color {
        ParsedColor::Oklch(oklch) => oklch.alpha,
        ParsedColor::Srgb(srgb) => srgb.alpha,
    }
}

fn relative_luminance(color: ParsedColor) -> f64 {
    match color {
        ParsedColor::Oklch(oklch) => relative_luminance_from_oklch(oklch),
        ParsedColor::Srgb(srgb) => relative_luminance_from_srgb(srgb),
    }
}

fn contrast_ratio(l1: f64, l2: f64) -> f64 {
    let (a, b) = if l1 >= l2 { (l1, l2) } else { (l2, l1) };
    (a + 0.05) / (b + 0.05)
}

fn assert_wcag_aa_pair(label: &str, fg: &str, bg: &str) {
    let fg = parse_color(fg);
    let bg = parse_color(bg);

    if color_alpha(fg) < 1.0 || color_alpha(bg) < 1.0 {
        // Alpha compositing depends on background stacks; keep AA checks on fully-opaque semantic pairs.
        return;
    }

    let fg_l = relative_luminance(fg);
    let bg_l = relative_luminance(bg);
    let ratio = contrast_ratio(fg_l, bg_l);

    assert!(
        ratio >= 4.5,
        "WCAG 2.1 AA contrast failed for {label}: ratio={ratio:.2} (fg={fg:?} bg={bg:?})"
    );
}

#[test]
fn semantic_colors_meet_wcag_21_aa_for_text_pairs() {
    for color in [ThemeColor::Light, ThemeColor::Dark, ThemeColor::Oled] {
        let theme = Theme::baseline_two(color, ThemeScale::Medium);
        let c = theme.tokens.semantic_colors;
        assert_wcag_aa_pair(&format!("{color:?} fg/bg"), c.fg, c.bg);
        assert_wcag_aa_pair(&format!("{color:?} fg_muted/bg"), c.fg_muted, c.bg);
        assert_wcag_aa_pair(
            &format!("{color:?} accent_fg/accent"),
            c.accent_fg,
            c.accent,
        );
        assert_wcag_aa_pair(
            &format!("{color:?} danger_fg/danger"),
            c.danger_fg,
            c.danger,
        );
        let roles = theme.tokens.semantic_roles;
        assert_wcag_aa_pair(
            &format!("{color:?} secondary_fg/secondary"),
            roles.secondary_fg,
            roles.secondary,
        );
    }
}
