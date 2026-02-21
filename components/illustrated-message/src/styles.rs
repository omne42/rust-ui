pub const CSS: &str = r#"
.ui-illustrated-message {
  --ui-im-surface-bg: color-mix(
    in oklab,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 86%,
    var(--ui-bg, var(--ui-fallback-bg)) 14%
  );
  --ui-im-surface-border: color-mix(
    in oklab,
    var(--ui-border, var(--ui-fallback-border)) 82%,
    transparent
  );
  --ui-im-surface-border-active: color-mix(
    in oklab,
    var(--ui-border, var(--ui-fallback-border)) 62%,
    var(--ui-accent, var(--ui-fallback-accent)) 38%
  );
  --ui-im-surface-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
  --ui-im-surface-shadow-active: var(--ui-shadow-md, var(--ui-fallback-shadow-md));
  --ui-im-focus-ring: color-mix(
    in oklab,
    var(--ui-focus-ring, var(--ui-fallback-focus-ring)) 34%,
    transparent
  );
  display: flex;
  gap: var(--ui-space-md, var(--ui-fallback-space-md));
  padding: var(--ui-space-lg, var(--ui-fallback-space-lg));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid var(--ui-im-surface-border);
  background: var(--ui-im-surface-bg);
  color: var(--ui-fg, var(--ui-fallback-fg));
  box-shadow: var(--ui-im-surface-shadow);

  --ui-im-opacity: 1;
  --ui-im-y: var(--ui-min-inline-size-none, var(--ui-fallback-min-inline-size-none));
  --ui-im-content-gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-im-actions-margin-top: var(--ui-space-sm, var(--ui-fallback-space-sm));
  opacity: var(--ui-im-opacity);
  transform: translateY(var(--ui-im-y));
  will-change: transform, opacity;
}

.ui-illustrated-message:hover {
  border-color: var(--ui-im-surface-border-active);
  box-shadow: var(--ui-im-surface-shadow-active);
}

.ui-illustrated-message:focus-within {
  border-color: var(--ui-im-surface-border-active);
  box-shadow:
    0 0 0 calc(var(--ui-border-width, var(--ui-fallback-border-width)) * 2) var(--ui-im-focus-ring),
    var(--ui-im-surface-shadow-active);
}

.ui-illustrated-message[data-view-state="empty"] {
  justify-content: center;
}

.ui-illustrated-message[data-content-state="hidden"] .ui-illustrated-message__content {
  display: none;
}

.ui-illustrated-message[data-description-state="hidden"] {
  --ui-im-content-gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
}

.ui-illustrated-message[data-actions-state="hidden"] {
  --ui-im-actions-margin-top: var(
    --ui-min-inline-size-none,
    var(--ui-fallback-min-inline-size-none)
  );
}

.ui-illustrated-message--orientation-vertical {
  flex-direction: column;
  align-items: flex-start;
}

.ui-illustrated-message--orientation-horizontal {
  flex-direction: row;
  align-items: center;
}

.ui-illustrated-message__illustration {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: calc(
    var(--ui-icon-size-200, var(--ui-fallback-icon-size-200))
      + var(--ui-space-xl, var(--ui-fallback-space-xl))
  );
  height: calc(
    var(--ui-icon-size-200, var(--ui-fallback-icon-size-200))
      + var(--ui-space-xl, var(--ui-fallback-space-xl))
  );
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  background: color-mix(
    in oklab,
    var(--ui-accent-soft, var(--ui-fallback-accent-soft)) 72%,
    var(--ui-bg, var(--ui-fallback-bg)) 28%
  );
  color: var(--ui-accent, var(--ui-fallback-accent));
  flex: 0 0 auto;
}

.ui-illustrated-message__content {
  display: flex;
  flex-direction: column;
  gap: var(--ui-im-content-gap);
  min-width: var(--ui-min-inline-size-none, var(--ui-fallback-min-inline-size-none));
}

.ui-illustrated-message__title {
  margin: var(--ui-min-inline-size-none, var(--ui-fallback-min-inline-size-none));
  font-size: var(--ui-heading-h6-font-size, var(--ui-fallback-heading-h6-font-size));
  line-height: var(
    --ui-heading-h6-line-height,
    var(--ui-fallback-heading-h6-line-height)
  );
  font-weight: 700;
}

.ui-illustrated-message__description {
  margin: var(--ui-min-inline-size-none, var(--ui-fallback-min-inline-size-none));
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-illustrated-message__actions {
  display: flex;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  flex-wrap: wrap;
  margin-top: var(--ui-im-actions-margin-top);
}
"#;
