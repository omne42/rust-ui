pub const CSS: &str = r#"
.ui-toggle-button-group {
  --ui-toggle-button-group-border-overlap: calc(var(--ui-space-xs) / 4);

  display: inline-flex;
  gap: var(--ui-space-xs);
}

.ui-toggle-button-group--horizontal {
  flex-direction: row;
  align-items: center;
  justify-content: flex-start;
}

.ui-toggle-button-group--vertical {
  flex-direction: column;
  align-items: flex-start;
  justify-content: flex-start;
}

.ui-toggle-button-group--attached {
  gap: 0;
}

.ui-toggle-button-group--attached > .ui-toggle-button {
  position: relative;
}

.ui-toggle-button-group--attached > .ui-toggle-button.ui-toggle-button--focus-visible {
  z-index: 1;
}

.ui-toggle-button-group--attached.ui-toggle-button-group--horizontal > .ui-toggle-button:not(:first-child) {
  margin-left: calc(var(--ui-toggle-button-group-border-overlap) * -1);

  border-top-left-radius: 0;
  border-bottom-left-radius: 0;
}

.ui-toggle-button-group--attached.ui-toggle-button-group--horizontal > .ui-toggle-button:not(:last-child) {
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
}

.ui-toggle-button-group--attached.ui-toggle-button-group--vertical > .ui-toggle-button:not(:first-child) {
  margin-top: calc(var(--ui-toggle-button-group-border-overlap) * -1);

  border-top-left-radius: 0;
  border-top-right-radius: 0;
}

.ui-toggle-button-group--attached.ui-toggle-button-group--vertical > .ui-toggle-button:not(:last-child) {
  border-bottom-left-radius: 0;
  border-bottom-right-radius: 0;
}
"#;
