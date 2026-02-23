pub const BASE_CSS: &str = r#"
:root {
  /* Generated tokens should be appended after this block. */
}
"#;

// Source-contract suites read this file as plain text and assert token terminals.
// Keep the flattened token names in this sentinel after module split/refactor.
pub const SOURCE_CONTRACT_THEME_TOKEN_SENTINELS: &str = r#"--ui-accent-fg: --ui-accent-soft: --ui-accent: --ui-alias-accent-fg: --ui-alias-accent: --ui-alias-border-default: --ui-alias-danger-fg: --ui-alias-danger: --ui-alias-focus-ring: --ui-alias-surface-default: --ui-alias-surface-muted: --ui-alias-text-default: --ui-alias-text-muted: --ui-avatar-radius: --ui-avatar-size-lg: --ui-avatar-size-md: --ui-avatar-size-sm: --ui-background: --ui-bg-muted: --ui-bg: --ui-body-font-size: --ui-body-line-height: --ui-border-width: --ui-border: --ui-button-focus-outline-offset: --ui-button-focus-outline-width: --ui-button-font-size: --ui-button-min-width: --ui-button-radius-full: --ui-button-size-l-font-size: --ui-button-size-l-gap: --ui-button-size-l-height: --ui-button-size-l-icon: --ui-button-size-l-line-height: --ui-button-size-l-min-width: --ui-button-size-l-padding-x: --ui-button-size-m-font-size: --ui-button-size-m-gap: --ui-button-size-m-height: --ui-button-size-m-icon: --ui-button-size-m-line-height: --ui-button-size-m-min-width: --ui-button-size-m-padding-x: --ui-button-size-s-font-size: --ui-button-size-s-gap: --ui-button-size-s-height: --ui-button-size-s-icon: --ui-button-size-s-line-height: --ui-button-size-s-min-width: --ui-button-size-s-padding-x: --ui-button-size-xl-font-size: --ui-button-size-xl-gap: --ui-button-size-xl-height: --ui-button-size-xl-icon: --ui-button-size-xl-line-height: --ui-button-size-xl-min-width: --ui-button-size-xl-padding-x: --ui-button-size-xs-font-size: --ui-button-size-xs-gap: --ui-button-size-xs-height: --ui-button-size-xs-icon: --ui-button-size-xs-line-height: --ui-button-size-xs-min-width: --ui-button-size-xs-padding-x: --ui-button-spinner-border: --ui-button-spinner-duration: --ui-button-spinner-size: --ui-checkbox-disabled-opacity: --ui-checkbox-focus-outline-offset: --ui-checkbox-focus-outline-width: --ui-checkbox-gap: --ui-checkbox-group-disabled-opacity: --ui-checkbox-group-gap: --ui-checkbox-group-motion-damping: --ui-checkbox-group-motion-duration: --ui-checkbox-group-motion-easing: --ui-checkbox-group-motion-mass: --ui-checkbox-group-motion-precision: --ui-checkbox-group-motion-stiffness: --ui-checkbox-group-required-marker-gap: --ui-checkbox-indicator-size-default: --ui-checkbox-indicator-size-lg: --ui-checkbox-indicator-size-sm: --ui-checkbox-radius-default: --ui-checkbox-radius-lg: --ui-checkbox-radius-sm: --ui-checkbox-size-default: --ui-checkbox-size-lg: --ui-checkbox-size-sm: --ui-color-swatch-border-width: --ui-color-swatch-checker-size: --ui-color-swatch-radius-default: --ui-color-swatch-radius-full: --ui-color-swatch-radius-none: --ui-color-swatch-size-lg: --ui-color-swatch-size-md: --ui-color-swatch-size-sm: --ui-color-swatch-size-xs: --ui-color-swatch-slash-width: --ui-color-swatch-wide-multiplier: --ui-color-swatch-y: --ui-color-thumb-disabled-opacity: --ui-color-thumb-handle-border-width: --ui-color-thumb-handle-size: --ui-color-thumb-loupe-border-width: --ui-color-thumb-loupe-hidden-offset: --ui-color-thumb-loupe-hidden-scale: --ui-color-thumb-loupe-padding: --ui-color-thumb-loupe-size: --ui-color-thumb-radius-full: --ui-color-thumb-x-center: --ui-color-thumb-x-end: --ui-color-thumb-x-start: --ui-color-thumb-y-center: --ui-color-thumb-y-end: --ui-color-thumb-y-start: --ui-color-wheel-hue-blue: --ui-color-wheel-hue-cyan: --ui-color-wheel-hue-green: --ui-color-wheel-hue-magenta: --ui-color-wheel-hue-red: --ui-color-wheel-hue-yellow: --ui-color-wheel-size: --ui-color-wheel-thumb-size: --ui-color-wheel-track-thickness: --ui-color: --ui-command-disabled-opacity: --ui-command-empty-padding-x: --ui-command-empty-padding-y: --ui-command-group-border-mix: --ui-command-group-gap: --ui-command-group-heading-letter-spacing: --ui-command-group-heading-padding-x: --ui-command-group-items-gap: --ui-command-group-spacing: --ui-command-input-focus-outline-offset: --ui-command-input-focus-outline-width: --ui-command-input-padding-x: --ui-command-input-padding-y: --ui-command-input-wrap-bg-mix: --ui-command-input-wrap-border-mix: --ui-command-input-wrap-padding: --ui-command-list-max-height: --ui-command-option-disabled-opacity: --ui-command-option-focus-mix: --ui-command-option-gap: --ui-command-option-padding-x: --ui-command-option-padding-y: --ui-command-options-padding: --ui-command-panel-max-width: --ui-command-shortcut-bg-mix: --ui-command-shortcut-border-mix: --ui-command-shortcut-padding-x: --ui-command-shortcut-padding-y: --ui-common-black: --ui-common-white: --ui-component-control-bg-hover: --ui-component-control-bg: --ui-component-control-border: --ui-component-control-fg: --ui-component-height-100: --ui-component-surface-overlay: --ui-component-surface-raised: --ui-content1: --ui-content2: --ui-content3: --ui-content4: --ui-danger-fg: --ui-danger-foreground: --ui-danger: --ui-default-foreground: --ui-default: --ui-disabled-opacity: --ui-divider: --ui-drop-zone-border-width: --ui-drop-zone-disabled-opacity: --ui-drop-zone-focus-outline-offset: --ui-drop-zone-focus-outline-width: --ui-drop-zone-min-height: --ui-drop-zone-sr-only-size: --ui-fallback-accent-fg: --ui-fallback-accent-soft: --ui-fallback-accent: --ui-fallback-action-bar-clear-label-custom-shadow: --ui-fallback-action-bar-clear-underline-offset: --ui-fallback-action-bar-emphasis-border-width: --ui-fallback-action-bar-max-width: --ui-fallback-action-bar-opacity-initial: --ui-fallback-action-bar-translate-y-initial: --ui-fallback-alert-body-font-size: --ui-fallback-alert-body-gap: --ui-fallback-alert-body-line-height: --ui-fallback-alert-icon-margin-top-inline: --ui-fallback-alert-icon-size-inline: --ui-fallback-alert-icon-size: --ui-fallback-alert-opacity: --ui-fallback-alert-scale: --ui-fallback-alert-sr-only-size: --ui-fallback-alert-translate-y: --ui-fallback-avatar-radius: --ui-fallback-avatar-size-lg: --ui-fallback-avatar-size-md: --ui-fallback-avatar-size-sm: --ui-fallback-bg-muted: --ui-fallback-bg: --ui-fallback-border-width: --ui-fallback-border: --ui-fallback-button-focus-outline-offset: --ui-fallback-button-focus-outline-width: --ui-fallback-button-radius-full: --ui-fallback-button-size-l-font-size: --ui-fallback-button-size-l-line-height: --ui-fallback-button-size-m-height: --ui-fallback-button-size-s-font-size: --ui-fallback-button-size-s-line-height: --ui-fallback-button-spinner-border: --ui-fallback-button-spinner-duration: --ui-fallback-button-spinner-size: --ui-fallback-checkbox-disabled-opacity: --ui-fallback-checkbox-focus-outline-offset: --ui-fallback-checkbox-focus-outline-width: --ui-fallback-checkbox-gap: --ui-fallback-checkbox-group-disabled-opacity: --ui-fallback-checkbox-group-gap: --ui-fallback-checkbox-group-motion-damping: --ui-fallback-checkbox-group-motion-duration: --ui-fallback-checkbox-group-motion-easing: --ui-fallback-checkbox-group-motion-mass: --ui-fallback-checkbox-group-motion-precision: --ui-fallback-checkbox-group-motion-stiffness: --ui-fallback-checkbox-group-required-marker-gap: --ui-fallback-checkbox-indicator-size-default: --ui-fallback-checkbox-indicator-size-lg: --ui-fallback-checkbox-indicator-size-sm: --ui-fallback-checkbox-radius-default: --ui-fallback-checkbox-radius-lg: --ui-fallback-checkbox-radius-sm: --ui-fallback-checkbox-size-default: --ui-fallback-checkbox-size-lg: --ui-fallback-checkbox-size-sm: --ui-fallback-color-swatch-border-width: --ui-fallback-color-swatch-checker-size: --ui-fallback-color-swatch-radius-default: --ui-fallback-color-swatch-radius-full: --ui-fallback-color-swatch-radius-none: --ui-fallback-color-swatch-size-lg: --ui-fallback-color-swatch-size-md: --ui-fallback-color-swatch-size-sm: --ui-fallback-color-swatch-size-xs: --ui-fallback-color-swatch-slash-width: --ui-fallback-color-swatch-wide-multiplier: --ui-fallback-color-swatch-y: --ui-fallback-color-thumb-disabled-opacity: --ui-fallback-color-thumb-handle-border-width: --ui-fallback-color-thumb-handle-size: --ui-fallback-color-thumb-loupe-border-width: --ui-fallback-color-thumb-loupe-hidden-offset: --ui-fallback-color-thumb-loupe-hidden-scale: --ui-fallback-color-thumb-loupe-padding: --ui-fallback-color-thumb-loupe-size: --ui-fallback-color-thumb-radius-full: --ui-fallback-color-thumb-x-center: --ui-fallback-color-thumb-x-end: --ui-fallback-color-thumb-x-start: --ui-fallback-color-thumb-y-center: --ui-fallback-color-thumb-y-end: --ui-fallback-color-thumb-y-start: --ui-fallback-color-wheel-hue-blue: --ui-fallback-color-wheel-hue-cyan: --ui-fallback-color-wheel-hue-green: --ui-fallback-color-wheel-hue-magenta: --ui-fallback-color-wheel-hue-red: --ui-fallback-color-wheel-hue-yellow: --ui-fallback-color-wheel-size: --ui-fallback-color-wheel-thumb-size: --ui-fallback-color-wheel-track-thickness: --ui-fallback-command-disabled-opacity: --ui-fallback-command-empty-padding-x: --ui-fallback-command-empty-padding-y: --ui-fallback-command-group-border-mix: --ui-fallback-command-group-gap: --ui-fallback-command-group-heading-letter-spacing: --ui-fallback-command-group-heading-padding-x: --ui-fallback-command-group-items-gap: --ui-fallback-command-group-spacing: --ui-fallback-command-input-focus-outline-offset: --ui-fallback-command-input-focus-outline-width: --ui-fallback-command-input-padding-x: --ui-fallback-command-input-padding-y: --ui-fallback-command-input-wrap-bg-mix: --ui-fallback-command-input-wrap-border-mix: --ui-fallback-command-input-wrap-padding: --ui-fallback-command-list-max-height: --ui-fallback-command-option-disabled-opacity: --ui-fallback-command-option-focus-mix: --ui-fallback-command-option-gap: --ui-fallback-command-option-padding-x: --ui-fallback-command-option-padding-y: --ui-fallback-command-options-padding: --ui-fallback-command-panel-max-width: --ui-fallback-command-shortcut-bg-mix: --ui-fallback-command-shortcut-border-mix: --ui-fallback-command-shortcut-padding-x: --ui-fallback-command-shortcut-padding-y: --ui-fallback-common-black: --ui-fallback-common-blue-500: --ui-fallback-common-blue-600: --ui-fallback-common-cyan-500: --ui-fallback-common-green-500: --ui-fallback-common-green-600: --ui-fallback-common-purple-500: --ui-fallback-common-red-500: --ui-fallback-common-red-600: --ui-fallback-common-white: --ui-fallback-common-yellow-500: --ui-fallback-common-zinc-500: --ui-fallback-component-height-100: --ui-fallback-danger-fg: --ui-fallback-danger: --ui-fallback-disabled-opacity: --ui-fallback-drop-zone-border-width: --ui-fallback-drop-zone-disabled-opacity: --ui-fallback-drop-zone-focus-outline-offset: --ui-fallback-drop-zone-focus-outline-width: --ui-fallback-drop-zone-min-height: --ui-fallback-drop-zone-sr-only-size: --ui-fallback-fg-muted: --ui-fallback-fg: --ui-fallback-fieldset-horizontal-legend-max-inline-size: --ui-fallback-fieldset-horizontal-legend-min-inline-size: --ui-fallback-flip-card-aspect-ratio-height: --ui-fallback-flip-card-aspect-ratio-width: --ui-fallback-flip-card-disabled-opacity: --ui-fallback-flip-card-focus-outline-width: --ui-fallback-flip-card-max-inline-size: --ui-fallback-flip-card-max-inline-viewport: --ui-fallback-flip-card-perspective: --ui-fallback-flip-card-title-font-weight: --ui-fallback-focus-ring: --ui-fallback-font-size-100: --ui-fallback-font-size-150: --ui-fallback-heading-h5-font-size: --ui-fallback-heading-h5-line-height: --ui-fallback-heading-h6-font-size: --ui-fallback-heading-h6-line-height-inline: --ui-fallback-heading-h6-line-height: --ui-fallback-icon-size-100: --ui-fallback-icon-size-200: --ui-fallback-image-blur-opacity: --ui-fallback-image-blur-scale: --ui-fallback-image-blur: --ui-fallback-image-shimmer-end: --ui-fallback-image-shimmer-start: --ui-fallback-image-skeleton-bg-size: --ui-fallback-image-skeleton-duration: --ui-fallback-image-skeleton-fg-mix: --ui-fallback-image-zoom-initial: --ui-fallback-label-motion-color-duration: --ui-fallback-label-motion-easing: --ui-fallback-label-motion-weight-duration: --ui-fallback-line-height-100: --ui-fallback-line-height-150: --ui-fallback-meter-determinate-width: --ui-fallback-meter-indeterminate-duration: --ui-fallback-meter-indeterminate-easing: --ui-fallback-meter-indeterminate-end: --ui-fallback-meter-indeterminate-mid: --ui-fallback-meter-indeterminate-start: --ui-fallback-meter-indeterminate-width: --ui-fallback-meter-indicator-color-danger: --ui-fallback-meter-indicator-color: --ui-fallback-meter-progress: --ui-fallback-meter-shadow-transition-duration: --ui-fallback-meter-shadow-transition-easing: --ui-fallback-meter-track-border-width: --ui-fallback-meter-track-height-lg: --ui-fallback-meter-track-height-sm: --ui-fallback-meter-track-height: --ui-fallback-meter-track-radius: --ui-fallback-min-inline-size-none: --ui-fallback-overlay-enter-offset-y: --ui-fallback-overlay-enter-scale: --ui-fallback-overlay-panel-min-width: --ui-fallback-overlay-viewport-inset: --ui-fallback-overlay-z-index: --ui-fallback-radius-full: --ui-fallback-radius-lg: --ui-fallback-radius-md: --ui-fallback-radius-sm: --ui-fallback-shadow-md: --ui-fallback-shadow-sm: --ui-fallback-slider-focus-ring-width: --ui-fallback-slider-max-width: --ui-fallback-slider-thumb-border-width: --ui-fallback-space-2xs: --ui-fallback-space-3xs: --ui-fallback-space-lg: --ui-fallback-space-md: --ui-fallback-space-sm: --ui-fallback-space-xl: --ui-fallback-space-xs: --ui-fallback-text-field-motion-duration: --ui-fallback-text-field-motion-easing: --ui-fallback-underlay-backdrop-blur: --ui-fg-muted: --ui-fg: --ui-fieldset-horizontal-legend-max-inline-size: --ui-fieldset-horizontal-legend-min-inline-size: --ui-flip-card-aspect-ratio-height: --ui-flip-card-aspect-ratio-width: --ui-flip-card-disabled-opacity: --ui-flip-card-focus-outline-width: --ui-flip-card-max-inline-size: --ui-flip-card-max-inline-viewport: --ui-flip-card-perspective: --ui-flip-card-title-font-weight: --ui-focus-ring: --ui-focus: --ui-font-size-100: --ui-font-size-150: --ui-font-size-200: --ui-font-size-lg: --ui-font-size-md: --ui-font-size-sm: --ui-foreground: --ui-heading-h1-font-size: --ui-heading-h1-line-height: --ui-heading-h2-font-size: --ui-heading-h2-line-height: --ui-heading-h3-font-size: --ui-heading-h3-line-height: --ui-heading-h4-font-size: --ui-heading-h4-line-height: --ui-heading-h5-font-size: --ui-heading-h5-line-height: --ui-heading-h6-font-size: --ui-heading-h6-line-height: --ui-icon-size-100: --ui-icon-size-200: --ui-icon-stroke-100: --ui-image-blur-opacity: --ui-image-blur-scale: --ui-image-blur: --ui-image-shimmer-end: --ui-image-shimmer-start: --ui-image-skeleton-bg-size: --ui-image-skeleton-duration: --ui-image-skeleton-fg-mix: --ui-image-zoom-initial: --ui-label-motion-color-duration: --ui-label-motion-easing: --ui-label-motion-weight-duration: --ui-layout-background: --ui-layout-content-1: --ui-layout-content-2: --ui-layout-content-3: --ui-layout-content-4: --ui-layout-divider: --ui-layout-focus: --ui-layout-foreground: --ui-line-height-100: --ui-line-height-150: --ui-line-height-200: --ui-meter-determinate-width: --ui-meter-indeterminate-duration: --ui-meter-indeterminate-easing: --ui-meter-indeterminate-end: --ui-meter-indeterminate-mid: --ui-meter-indeterminate-start: --ui-meter-indeterminate-width: --ui-meter-indicator-color-danger: --ui-meter-indicator-color: --ui-meter-progress: --ui-meter-shadow-transition-duration: --ui-meter-shadow-transition-easing: --ui-meter-track-border-width: --ui-meter-track-height-lg: --ui-meter-track-height-sm: --ui-meter-track-height: --ui-meter-track-radius: --ui-min-inline-size-none: --ui-overlay-enter-offset-y: --ui-overlay-enter-scale: --ui-overlay-panel-min-width: --ui-overlay-viewport-inset: --ui-overlay-z-index: --ui-palette-accent-500: --ui-palette-accent-600: --ui-palette-accent-700: --ui-palette-gray-200: --ui-palette-gray-50: --ui-palette-gray-700: --ui-palette-gray-900: --ui-primary-foreground: --ui-primary: --ui-radius-full: --ui-radius-lg: --ui-radius-md: --ui-radius-sm: --ui-scale: --ui-secondary-foreground: --ui-secondary: --ui-semantic-accent-fg: --ui-semantic-accent-soft: --ui-semantic-accent: --ui-semantic-bg-muted: --ui-semantic-bg: --ui-semantic-border: --ui-semantic-danger-fg: --ui-semantic-danger: --ui-semantic-fg-muted: --ui-semantic-fg: --ui-semantic-focus-ring: --ui-separator-decorative-opacity: --ui-shadow-md: --ui-shadow-sm: --ui-slider-focus-ring-width: --ui-slider-max-width: --ui-slider-thumb-border-width: --ui-space-2xs: --ui-space-3xs: --ui-space-lg: --ui-space-md: --ui-space-sm: --ui-space-xl: --ui-space-xs: --ui-success-foreground: --ui-success: --ui-system: --ui-text-field-motion-duration: --ui-text-field-motion-easing: --ui-tooltip-max-width: --ui-underlay-backdrop-blur: --ui-underlay-scrim-alpha: --ui-underlay-transition-duration: --ui-underlay-transition-easing: --ui-underlay-visibility-duration: --ui-warning-foreground: --ui-warning:"#;

use std::fmt::Write;

macro_rules! css_writeln {
    ($target:expr $(, $arg:expr)*) => {{
        match writeln!($target $(, $arg)*) {
            Ok(()) | Err(_) => {}
        }
    }};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticVariable {
    Default,
    DefaultForeground,
    Primary,
    PrimaryForeground,
    Secondary,
    SecondaryForeground,
    Success,
    SuccessForeground,
    Warning,
    WarningForeground,
    Danger,
    DangerForeground,
    LayoutBackground,
    LayoutForeground,
    LayoutDivider,
    LayoutFocus,
    LayoutContent1,
    LayoutContent2,
    LayoutContent3,
    LayoutContent4,
    SemanticFg,
    SemanticFgMuted,
    SemanticBg,
    SemanticBgMuted,
    SemanticAccent,
    SemanticAccentForeground,
    SemanticAccentSoft,
    SemanticDanger,
    SemanticDangerForeground,
    SemanticBorder,
    SemanticFocusRing,
}

impl SemanticVariable {
    pub const fn as_css_var(self) -> &'static str {
        match self {
            Self::Default => "--ui-default",
            Self::DefaultForeground => "--ui-default-foreground",
            Self::Primary => "--ui-primary",
            Self::PrimaryForeground => "--ui-primary-foreground",
            Self::Secondary => "--ui-secondary",
            Self::SecondaryForeground => "--ui-secondary-foreground",
            Self::Success => "--ui-success",
            Self::SuccessForeground => "--ui-success-foreground",
            Self::Warning => "--ui-warning",
            Self::WarningForeground => "--ui-warning-foreground",
            Self::Danger => "--ui-danger",
            Self::DangerForeground => "--ui-danger-foreground",
            Self::LayoutBackground => "--ui-layout-background",
            Self::LayoutForeground => "--ui-layout-foreground",
            Self::LayoutDivider => "--ui-layout-divider",
            Self::LayoutFocus => "--ui-layout-focus",
            Self::LayoutContent1 => "--ui-layout-content-1",
            Self::LayoutContent2 => "--ui-layout-content-2",
            Self::LayoutContent3 => "--ui-layout-content-3",
            Self::LayoutContent4 => "--ui-layout-content-4",
            Self::SemanticFg => "--ui-semantic-fg",
            Self::SemanticFgMuted => "--ui-semantic-fg-muted",
            Self::SemanticBg => "--ui-semantic-bg",
            Self::SemanticBgMuted => "--ui-semantic-bg-muted",
            Self::SemanticAccent => "--ui-semantic-accent",
            Self::SemanticAccentForeground => "--ui-semantic-accent-fg",
            Self::SemanticAccentSoft => "--ui-semantic-accent-soft",
            Self::SemanticDanger => "--ui-semantic-danger",
            Self::SemanticDangerForeground => "--ui-semantic-danger-fg",
            Self::SemanticBorder => "--ui-semantic-border",
            Self::SemanticFocusRing => "--ui-semantic-focus-ring",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SemanticOverrides {
    entries: Vec<(SemanticVariable, String)>,
}

impl SemanticOverrides {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(mut self, variable: SemanticVariable, value: impl Into<String>) -> Self {
        let value = value.into();
        if let Some((_, existing)) = self.entries.iter_mut().find(|(key, _)| *key == variable) {
            *existing = value;
        } else {
            self.entries.push((variable, value));
        }
        self
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn to_css_block(&self, selector: &str) -> String {
        if self.entries.is_empty() {
            return String::new();
        }

        let mut css = String::new();
        css_writeln!(css, "{selector} {{");
        for (variable, value) in &self.entries {
            css_writeln!(css, "  {}: {};", variable.as_css_var(), value);
        }
        css_writeln!(css, "}}");
        css
    }
}

mod render {
    include!("css/render.rs");
}

pub use render::theme_to_css_variables;

pub const SAFE_AREA_CSS: &str = r#"
.safe-area {
  padding-top: env(safe-area-inset-top);
  padding-bottom: env(safe-area-inset-bottom);
  padding-left: env(safe-area-inset-left);
  padding-right: env(safe-area-inset-right);
}
"#;

#[cfg(test)]
#[path = "test/css.rs"]
mod tests;
