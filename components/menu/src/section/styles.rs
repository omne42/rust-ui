pub const CSS: &str = r#"
.ui-menu-section {
  display: grid;
  gap: 4px;
}

.ui-menu-section--tone-default,
.ui-menu-section[data-tone="default"] {
  --ui-menu-section-title-color: color-mix(in oklab, var(--ui-fg-muted) 88%, var(--ui-fg) 12%);
}

.ui-menu-section--tone-quiet,
.ui-menu-section[data-tone="quiet"] {
  --ui-menu-section-title-color: color-mix(in oklab, var(--ui-fg-muted) 96%, var(--ui-bg) 4%);
}

.ui-menu-section__header {
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  font-weight: 600;
  color: var(--ui-menu-section-title-color);
  padding-inline: 10px;
  padding-block: 4px;
}

.ui-menu-section__header[data-sticky="true"] {
  position: sticky;
  top: 0;
  z-index: 1;
  backdrop-filter: blur(6px);
  background: color-mix(in oklab, var(--ui-bg) 88%, transparent 12%);
}

.ui-menu-section__items {
  display: grid;
  gap: 4px;
}

.ui-menu-section__divider {
  border-bottom: 1px solid color-mix(in oklab, var(--ui-border) 88%, var(--ui-bg-muted) 12%);
  margin-inline: 10px;
}

.ui-menu-section--empty,
.ui-menu-section[data-empty="true"] {
  opacity: 0.76;
}

.ui-menu-section--disabled,
.ui-menu-section[data-disabled="true"] {
  opacity: 0.52;
}

.ui-menu-section--sticky-heading,
.ui-menu-section[data-sticky-heading="true"] {
  gap: 2px;
}

.ui-menu-section--divided,
.ui-menu-section[data-divided="true"] {
  padding-bottom: 4px;
}

.ui-menu-section--custom-class,
.ui-menu-section[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 26%, transparent);
  outline-offset: 2px;
}
"#;
