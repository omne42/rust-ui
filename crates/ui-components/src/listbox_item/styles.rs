pub const CSS: &str = r#"
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
