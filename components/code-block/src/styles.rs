pub const CSS: &str = r#"
.ui-code-block {
  --ui-code-block-border-width: var(--ui-border-width, var(--ui-fallback-border-width));
  --ui-code-block-border-color: var(--ui-border, var(--ui-fallback-border));
  --ui-code-block-radius-lg: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  --ui-code-block-radius-sm: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  --ui-code-block-bg: var(--ui-bg, var(--ui-fallback-bg));
  --ui-code-block-bg-muted: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
  --ui-code-block-fg: var(--ui-fg, var(--ui-fallback-fg));
  --ui-code-block-fg-muted: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  --ui-code-block-shadow-sm: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
  --ui-code-block-accent: var(--ui-accent, var(--ui-fallback-accent));
  --ui-code-block-accent-soft: var(--ui-accent-soft, var(--ui-fallback-accent-soft));
  --ui-code-block-focus-ring: var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  --ui-code-block-space-xs: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-code-block-space-sm: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-code-block-space-md: var(--ui-space-md, var(--ui-fallback-space-md));
  --ui-code-block-font-size-code: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  --ui-code-block-line-height-code: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  --ui-code-block-font-size-meta: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  --ui-code-block-line-height-meta: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  --ui-code-block-label-font-size: var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size));
  --ui-code-block-label-line-height: var(--ui-button-size-s-line-height, var(--ui-fallback-button-size-s-line-height));
  --ui-code-block-focus-ring-width: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width));
  --ui-code-block-focus-ring-offset: var(--ui-button-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset));
  --ui-code-block-icon-size: var(--ui-space-md, var(--ui-fallback-space-md));
  --ui-code-block-icon-button-size: calc(
    var(--ui-space-md, var(--ui-fallback-space-md)) * 2 +
      var(--ui-space-xs, var(--ui-fallback-space-xs))
  );
  --ui-code-block-sr-only-size: var(--ui-alert-sr-only-size, var(--ui-fallback-alert-sr-only-size));
  --ui-code-block-motion-duration: var(--ui-checkbox-group-motion-duration, var(--ui-fallback-checkbox-group-motion-duration));
  --ui-code-block-motion-easing: var(--ui-checkbox-group-motion-easing, var(--ui-fallback-checkbox-group-motion-easing));
  position: relative;
  border: var(--ui-code-block-border-width) solid var(--ui-code-block-border-color);
  border-radius: var(--ui-code-block-radius-lg);
  background: var(--ui-code-block-bg);
  color: var(--ui-code-block-fg);
  box-shadow: var(--ui-code-block-shadow-sm);
  overflow: hidden;
  isolation: isolate;

  --ui-code-block-copy-flash: 0;
  --ui-code-block-custom-motion: 0;
}

.ui-code-block--state-multiline,
.ui-code-block[data-state="multiline"],
.ui-code-block[data-multiline="true"] {
  --ui-code-block-is-multiline: 1;
}

.ui-code-block--state-single-line,
.ui-code-block[data-state="single-line"] {
  --ui-code-block-is-multiline: 0;
}

.ui-code-block--header-visible,
.ui-code-block[data-header="visible"] {
  --ui-code-block-header-visible: 1;
}

.ui-code-block--header-hidden,
.ui-code-block[data-header="hidden"] {
  --ui-code-block-header-visible: 0;
}

.ui-code-block--copyable,
.ui-code-block[data-copyable="true"] {
  --ui-code-block-copyable: 1;
}

.ui-code-block--with-label,
.ui-code-block[data-label="true"] {
  --ui-code-block-has-label: 1;
}

.ui-code-block--with-language,
.ui-code-block[data-language="true"] {
  --ui-code-block-has-language: 1;
}

.ui-code-block--empty,
.ui-code-block[data-empty="true"] {
  --ui-code-block-is-empty: 1;
}

.ui-code-block--motion-custom,
.ui-code-block[data-motion-source="custom"] {
  --ui-code-block-custom-motion: 1;
}

.ui-code-block--custom-class,
.ui-code-block[data-custom-class="true"] {
  --ui-code-block-custom-class: 1;
}

.ui-code-block::after {
  content: "";
  position: absolute;
  inset: 0;
  z-index: 0;
  background: color-mix(
    in oklch,
    var(--ui-code-block-accent-soft) 70%,
    var(--ui-code-block-bg)
  );
  opacity: calc(var(--ui-code-block-copy-flash) * 0.18);
  pointer-events: none;
}

.ui-code-block > * {
  position: relative;
  z-index: 1;
}

.ui-code-block__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--ui-code-block-space-sm);
  padding: var(--ui-code-block-space-sm) var(--ui-code-block-space-md);
  border-bottom: var(--ui-code-block-border-width) solid var(--ui-code-block-border-color);
  background: color-mix(
    in oklch,
    var(--ui-code-block-bg-muted) 70%,
    var(--ui-code-block-bg)
  );
}

.ui-code-block__meta {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-code-block-space-sm);
  min-width: 0;
}

.ui-code-block__label {
  font-size: var(--ui-code-block-label-font-size);
  line-height: var(--ui-code-block-label-line-height);
  font-weight: 600;
}

.ui-code-block__language {
  font-size: var(--ui-code-block-font-size-meta);
  line-height: var(--ui-code-block-line-height-meta);
  color: var(--ui-code-block-fg-muted);
}

.ui-code-block__pre {
  margin: 0;
  padding: var(--ui-code-block-space-md);
  overflow: auto;
  font-size: var(--ui-code-block-font-size-code);
  line-height: var(--ui-code-block-line-height-code);
}

.ui-code-block__code {
  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono",
    "Courier New", monospace;
}

.ui-code-block__a11y-status {
  position: absolute;
  width: var(--ui-code-block-sr-only-size);
  height: var(--ui-code-block-sr-only-size);
  padding: 0;
  margin: calc(var(--ui-code-block-sr-only-size) * -1);
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

.ui-code-block__copy-button svg {
  width: var(--ui-code-block-icon-size);
  height: var(--ui-code-block-icon-size);
}

.ui-code-block__button {
  border: var(--ui-code-block-border-width) solid transparent;
  border-radius: var(--ui-code-block-radius-sm);
  background: transparent;
  color: var(--ui-code-block-fg);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition:
    background-color var(--ui-code-block-motion-duration) var(--ui-code-block-motion-easing),
    border-color var(--ui-code-block-motion-duration) var(--ui-code-block-motion-easing),
    color var(--ui-code-block-motion-duration) var(--ui-code-block-motion-easing);
}

.ui-code-block__button--ghost {
  background: transparent;
  border-color: transparent;
}

.ui-code-block__button--icon-sm {
  width: var(--ui-code-block-icon-button-size);
  height: var(--ui-code-block-icon-button-size);
}

.ui-code-block__button:hover {
  background: color-mix(in oklch, var(--ui-code-block-fg) 8%, transparent);
}

.ui-code-block__button:focus-visible {
  outline: var(--ui-code-block-focus-ring-width) solid var(--ui-code-block-focus-ring);
  outline-offset: var(--ui-code-block-focus-ring-offset);
}
"#;
