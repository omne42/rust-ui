//! Design tokens for `ui-theme`.
//!
//! This file is the single source of truth for token taxonomy and baselines.
//! Theme mapping happens in `theme.rs`; CSS variable emission happens in `css.rs`.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenScale {
    Medium,
    Large,
}

#[derive(Clone, Copy)]
pub struct ColorPaletteTokens {
    // Placeholder (v0): keep the taxonomy stable even if we don’t expose the full Spectrum palette yet.
}

#[derive(Clone, Copy)]
pub struct SemanticColorTokens {
    pub fg: &'static str,
    pub fg_muted: &'static str,
    pub bg: &'static str,
    pub bg_muted: &'static str,
    pub accent: &'static str,
    pub accent_fg: &'static str,
    pub accent_soft: &'static str,
    pub danger: &'static str,
    pub danger_fg: &'static str,
    pub border: &'static str,
    pub focus_ring: &'static str,
}

#[derive(Clone, Copy)]
pub struct ColorAliasTokens {
    pub text_default: &'static str,
    pub text_muted: &'static str,
    pub surface_default: &'static str,
    pub surface_muted: &'static str,
    pub border_default: &'static str,
    pub focus_ring: &'static str,
    pub accent: &'static str,
    pub accent_fg: &'static str,
    pub danger: &'static str,
    pub danger_fg: &'static str,
}

#[derive(Clone, Copy)]
pub struct ComponentColorTokens {
    // Placeholder (v0): component-specific colors should live here, not in component styles.
}

#[derive(Clone, Copy)]
pub struct IconTokens {
    // Placeholder (v0): icon size/radius/optical alignment tokens.
}

#[derive(Clone, Copy)]
pub struct LayoutTokens {
    pub radius: RadiusTokens,
    pub space: SpaceTokens,
    pub shadow: ShadowTokens,
}

#[derive(Clone, Copy)]
pub struct RadiusTokens {
    pub sm_px: u16,
    pub md_px: u16,
    pub lg_px: u16,
}

#[derive(Clone, Copy)]
pub struct SpaceTokens {
    pub xs_px: u16,
    pub sm_px: u16,
    pub md_px: u16,
    pub lg_px: u16,
}

#[derive(Clone, Copy)]
pub struct ShadowTokens {
    pub sm: &'static str,
    pub md: &'static str,
}

#[derive(Clone, Copy)]
pub struct ComponentLayoutTokens {
    // Spectrum baseline examples (must be regression-testable):
    pub component_height_100_px: u16,
}

#[derive(Clone, Copy)]
pub struct TypographyTokens {
    // Spectrum baseline examples (must be regression-testable):
    pub font_size_200_px: u16,
}

#[derive(Clone, Copy)]
pub struct ThemeTokens {
    pub palette: ColorPaletteTokens,
    pub semantic_colors: SemanticColorTokens,
    pub color_aliases: ColorAliasTokens,
    pub component_colors: ComponentColorTokens,
    pub icons: IconTokens,
    pub layout: LayoutTokens,
    pub component_layout: ComponentLayoutTokens,
    pub typography: TypographyTokens,
}
