pub const CSS: &str = r#"
.ui-toggle-group {
  display: inline-flex;
}

.ui-toggle-group__items {
  display: inline-flex;
  gap: var(--ui-space-xs);
}

.ui-toggle-group--horizontal .ui-toggle-group__items {
  flex-direction: row;
  align-items: center;
}

.ui-toggle-group--vertical .ui-toggle-group__items {
  flex-direction: column;
  align-items: flex-start;
}

.ui-toggle-group--disabled {
  opacity: 0.8;
}

.ui-toggle-group--attached .ui-toggle-group__items {
  gap: 0;
}

.ui-toggle-group--attached .ui-toggle-group__item {
  position: relative;
}

.ui-toggle-group--attached .ui-toggle-group__item.ui-toggle-button--focus-visible {
  z-index: 1;
}

.ui-toggle-group--attached.ui-toggle-group--horizontal .ui-toggle-group__item:not(:first-child) {
  margin-left: -1px;
  border-top-left-radius: 0;
  border-bottom-left-radius: 0;
}

.ui-toggle-group--attached.ui-toggle-group--horizontal .ui-toggle-group__item:not(:last-child) {
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
}

.ui-toggle-group--attached.ui-toggle-group--vertical .ui-toggle-group__item:not(:first-child) {
  margin-top: -1px;
  border-top-left-radius: 0;
  border-top-right-radius: 0;
}

.ui-toggle-group--attached.ui-toggle-group--vertical .ui-toggle-group__item:not(:last-child) {
  border-bottom-left-radius: 0;
  border-bottom-right-radius: 0;
}
"#;
