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
  font-size: 0.875rem;
  line-height: 1.45;
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

.ui-breadcrumb__link:hover,
.ui-breadcrumb__link[data-interactive="true"]:hover {
  color: var(--ui-fg, currentColor);
}

.ui-breadcrumb__link:focus-visible {
  outline: 3px solid var(--ui-focus-ring, currentColor);
  outline-offset: 2px;
  border-radius: var(--ui-radius-sm);
}

.ui-breadcrumb__link--placeholder,
.ui-breadcrumb__link[data-href-state="absent"] {
  color: var(--ui-fg-muted, currentColor);
  cursor: default;
}

.ui-breadcrumb__page {
  color: var(--ui-fg, currentColor);
  font-weight: 500;
}

.ui-breadcrumb__separator {
  color: var(--ui-fg-subtle, var(--ui-fg-muted, currentColor));
  user-select: none;
}

.ui-breadcrumb__separator--custom-content,
.ui-breadcrumb__separator[data-content-source="custom"] {
  color: var(--ui-fg-muted, currentColor);
}

.ui-breadcrumb__ellipsis {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  inline-size: 1.5rem;
  block-size: 1.5rem;
  color: var(--ui-fg-muted, currentColor);
}

.ui-breadcrumb__ellipsis-icon {
  line-height: 1;
}

.ui-breadcrumb__ellipsis-label {
  position: absolute;
  inline-size: 1px;
  block-size: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  border: 0;
  clip: rect(0 0 0 0);
  clip-path: inset(50%);
  white-space: nowrap;
}

.ui-breadcrumb--custom-class,
.ui-breadcrumb[data-custom-class="true"],
.ui-breadcrumb__list--custom-class,
.ui-breadcrumb__list[data-custom-class="true"],
.ui-breadcrumb__item--custom-class,
.ui-breadcrumb__item[data-custom-class="true"],
.ui-breadcrumb__link--custom-class,
.ui-breadcrumb__link[data-custom-class="true"],
.ui-breadcrumb__page--custom-class,
.ui-breadcrumb__page[data-custom-class="true"],
.ui-breadcrumb__separator--custom-class,
.ui-breadcrumb__separator[data-custom-class="true"],
.ui-breadcrumb__ellipsis--custom-class,
.ui-breadcrumb__ellipsis[data-custom-class="true"] {
  border-radius: inherit;
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
