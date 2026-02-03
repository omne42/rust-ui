#[derive(Clone, Copy)]
pub struct ColorTokens {
    pub fg: &'static str,
    pub fg_muted: &'static str,
    pub bg: &'static str,
    pub bg_muted: &'static str,
    pub accent: &'static str,
    pub accent_soft: &'static str,
    pub border: &'static str,
    pub focus_ring: &'static str,
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
