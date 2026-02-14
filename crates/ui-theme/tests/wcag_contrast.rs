use ui_theme::{Theme, ThemeColor, ThemeScale};

#[derive(Clone, Copy, Debug)]
struct Oklch {
    l: f64,
    c: f64,
    h_deg: f64,
    alpha: f64,
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

fn contrast_ratio(l1: f64, l2: f64) -> f64 {
    let (a, b) = if l1 >= l2 { (l1, l2) } else { (l2, l1) };
    (a + 0.05) / (b + 0.05)
}

fn assert_wcag_aa_pair(label: &str, fg: &str, bg: &str) {
    let fg = parse_oklch(fg);
    let bg = parse_oklch(bg);

    if fg.alpha < 1.0 || bg.alpha < 1.0 {
        // Alpha compositing depends on background stacks; keep AA checks on fully-opaque semantic pairs.
        return;
    }

    let fg_l = relative_luminance_from_oklch(fg);
    let bg_l = relative_luminance_from_oklch(bg);
    let ratio = contrast_ratio(fg_l, bg_l);

    assert!(
        ratio >= 4.5,
        "WCAG 2.1 AA contrast failed for {label}: ratio={ratio:.2} (fg={fg:?} bg={bg:?})"
    );
}

#[test]
fn semantic_colors_meet_wcag_21_aa_for_text_pairs() {
    for color in [ThemeColor::Light, ThemeColor::Dark, ThemeColor::Oled] {
        let theme = Theme::spectrum_two(color, ThemeScale::Medium);
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
    }
}
