pub const CSS: &str = r#"
.ui-button-group {
  --ui-button-group-border-overlap: calc(var(--ui-space-xs) / 4);

  display: inline-flex;
  gap: var(--ui-space-xs);
  transform: scale(var(--ui-button-group-scale, 1));
  transform-origin: center;
  will-change: transform;
}

.ui-button-group[data-motion-source="custom"],
.ui-button-group[data-custom-motion="true"] {
  --ui-button-group-custom-motion: 1;
}

.ui-button-group--horizontal {
  flex-direction: row;
  align-items: center;
  justify-content: flex-start;
}

.ui-button-group--vertical {
  flex-direction: column;
  align-items: flex-start;
  justify-content: flex-start;
}

.ui-button-group--attached {
  gap: 0;
}

.ui-button-group--attached > .ui-button {
  position: relative;
}

.ui-button-group--attached > .ui-button.ui-button--focus-visible {
  z-index: 1;
}

.ui-button-group--attached.ui-button-group--horizontal > .ui-button:not(:first-child) {
  margin-left: calc(var(--ui-button-group-border-overlap) * -1);

  border-top-left-radius: 0;
  border-bottom-left-radius: 0;
}

.ui-button-group--attached.ui-button-group--horizontal > .ui-button:not(:last-child) {
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
}

.ui-button-group--attached.ui-button-group--vertical > .ui-button:not(:first-child) {
  margin-top: calc(var(--ui-button-group-border-overlap) * -1);

  border-top-left-radius: 0;
  border-top-right-radius: 0;
}

.ui-button-group--attached.ui-button-group--vertical > .ui-button:not(:last-child) {
  border-bottom-left-radius: 0;
  border-bottom-right-radius: 0;
}
"#;
