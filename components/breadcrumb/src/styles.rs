pub const CSS: &str = r#"
.ui-breadcrumb {
  display: block;
  color: var(--ui-fg, currentColor);
}

.ui-breadcrumb[data-aria-source="custom"] {
  --ui-breadcrumb-aria-source: custom;
}

.ui-breadcrumb__list {
  list-style-type: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: var(--ui-space-xs);
  color: var(--ui-fg-muted, currentColor);
  font-size: var(--ui-font-size-150, 14px);
  line-height: var(--ui-line-height-150, 20px);
}

.ui-breadcrumb__item {
  display: inline-flex;
  align-items: center;
  min-width: 0;
  gap: var(--ui-space-xs);
}

.ui-breadcrumb__link {
  display: inline-flex;
  align-items: center;
  color: inherit;
  text-decoration: none;
  transition: color 180ms ease;
}

.ui-breadcrumb__link:hover {
  color: var(--ui-fg, currentColor);
}

.ui-breadcrumb__link:focus-visible {
  outline: 3px solid var(--ui-focus-ring, currentColor);
  outline-offset: 2px;
  border-radius: var(--ui-radius-sm);
}

.ui-breadcrumb__label {
  color: var(--ui-fg-muted, currentColor);
}

.ui-breadcrumb__page {
  color: var(--ui-fg, currentColor);
  font-weight: 500;
}

.ui-breadcrumb__separator {
  color: var(--ui-fg-subtle, var(--ui-fg-muted, currentColor));
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
