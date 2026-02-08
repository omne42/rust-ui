pub const CSS: &str = r#"
.ui-listbox-section {
  display: grid;
  gap: 4px;
}

.ui-listbox-section--tone-default,
.ui-listbox-section[data-tone="default"] {
  --ui-listbox-section-title-color: color-mix(in oklab, var(--ui-fg-muted) 88%, var(--ui-fg) 12%);
}

.ui-listbox-section--tone-quiet,
.ui-listbox-section[data-tone="quiet"] {
  --ui-listbox-section-title-color: color-mix(in oklab, var(--ui-fg-muted) 96%, var(--ui-bg) 4%);
}

.ui-listbox-section__header {
  font-size: 0.75rem;
  line-height: 1.25;
  font-weight: 600;
  color: var(--ui-listbox-section-title-color);
  padding-inline: 8px;
  padding-block: 4px;
}

.ui-listbox-section__header[data-sticky="true"] {
  position: sticky;
  top: 0;
  z-index: 1;
  backdrop-filter: blur(6px);
  background: color-mix(in oklab, var(--ui-bg) 88%, transparent 12%);
}

.ui-listbox-section__items {
  display: grid;
  gap: 4px;
}

.ui-listbox-section__divider {
  border-bottom: 1px solid color-mix(in oklab, var(--ui-border) 88%, var(--ui-bg-muted) 12%);
  margin-inline: 8px;
}

.ui-listbox-section--empty,
.ui-listbox-section[data-empty="true"] {
  opacity: 0.76;
}

.ui-listbox-section--disabled,
.ui-listbox-section[data-disabled="true"] {
  opacity: 0.52;
}

.ui-listbox-section--sticky-heading,
.ui-listbox-section[data-sticky-heading="true"] {
  gap: 2px;
}

.ui-listbox-section--divided,
.ui-listbox-section[data-divided="true"] {
  padding-bottom: 4px;
}

.ui-listbox-section--custom-class,
.ui-listbox-section[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 26%, transparent);
  outline-offset: 2px;
}
"#;
