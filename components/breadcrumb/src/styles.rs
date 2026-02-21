pub const CSS: &str = r#"
.ui-breadcrumb {
  display: block;
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-breadcrumb__list {
  list-style-type: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
}

.ui-breadcrumb__item {
  display: inline-flex;
  align-items: center;
  min-width: 0;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-breadcrumb__link {
  display: inline-flex;
  align-items: center;
  color: inherit;
  text-decoration: none;
  transition:
    color var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))
    var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing));
}

.ui-breadcrumb__link:hover {
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-breadcrumb__link:focus-visible {
  outline:
    var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width))
    solid var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(
    --ui-button-focus-outline-offset,
    var(--ui-fallback-button-focus-outline-offset)
  );
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
}

.ui-breadcrumb__label {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-breadcrumb__page {
  color: var(--ui-fg, var(--ui-fallback-fg));
  font-weight: 500;
}

.ui-breadcrumb__separator {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  user-select: none;
}

@media (forced-colors: active) {
  .ui-breadcrumb__link:focus-visible {
    outline-color: Highlight;
  }
}

@media (prefers-reduced-motion: reduce) {
  .ui-breadcrumb__link {
    transition: none;
  }
}
"#;
