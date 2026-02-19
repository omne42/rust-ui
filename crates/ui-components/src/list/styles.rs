pub const CSS: &str = r#"
.ui-listbox {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: var(--ui-space-sm);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-lg);
  background: var(--ui-bg);
  box-shadow: var(--ui-shadow-sm);
  box-sizing: border-box;
  outline: none;
}

.ui-listbox[data-motion-source="custom"],
.ui-listbox[data-custom-motion="true"] {
  --ui-listbox-custom-motion: 1;
}

.ui-listbox--focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-listbox__options {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.ui-listbox__option {
  position: relative;
  z-index: 1;
  padding: 6px 8px;
  border-radius: 8px;
  cursor: default;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-listbox .ui-active-highlight {
  transition:
    transform 160ms cubic-bezier(0.2, 0, 0, 1),
    height 160ms cubic-bezier(0.2, 0, 0, 1),
    opacity 120ms ease-out;
}

.ui-listbox__option[data-selected="true"] {
  font-weight: 600;
}

.ui-listbox__option[data-disabled="true"] {
  opacity: 0.5;
}
"#;

pub const ITEM_CSS: &str = r#"
.ui-listbox-item {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px;
  border-radius: 8px;
  cursor: default;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
}

.ui-listbox-item--selected,
.ui-listbox-item[data-selected="true"] {
  font-weight: 600;
}

.ui-listbox-item--focused,
.ui-listbox-item[data-focused="true"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 80%, var(--ui-accent) 20%);
}

.ui-listbox-item--disabled,
.ui-listbox-item[data-disabled="true"] {
  opacity: 0.5;
  cursor: not-allowed;
}

.ui-listbox-item--selection-indicator,
.ui-listbox-item[data-show-selection-indicator="true"] {
  padding-inline-start: 6px;
}

.ui-listbox-item__indicator {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  flex-shrink: 0;
}

.ui-listbox-item__label {
  min-width: 0;
  flex: 1;
  display: inline-flex;
  align-items: center;
}

.ui-listbox-item__divider {
  position: absolute;
  inset-inline: 8px;
  bottom: 0;
  height: 1px;
  background: color-mix(in oklab, var(--ui-border) 90%, var(--ui-bg-muted) 10%);
}

.ui-listbox-item__selection-sr {
  position: absolute;
  width: 1px;
  height: 1px;
  margin: -1px;
  padding: 0;
  border: 0;
  overflow: hidden;
  clip: rect(0 0 0 0);
  white-space: nowrap;
}

.ui-listbox-item--divider,
.ui-listbox-item[data-has-divider="true"] {
  border-bottom-left-radius: 0;
  border-bottom-right-radius: 0;
}

.ui-listbox-item--custom-class,
.ui-listbox-item[data-custom-class="true"] {
  box-shadow: 0 0 0 1px color-mix(in oklab, var(--ui-accent) 22%, transparent) inset;
}
"#;

pub const SECTION_CSS: &str = r#"
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
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
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
