pub const CSS: &str = r#"
.ui-logic-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 2.125rem;
  min-height: 1.5rem;
  padding: 0 var(--ui-space-sm);
  border: 1px solid var(--ui-border);
  border-radius: 999px;
  background: var(--ui-bg-muted);
  color: var(--ui-fg);
  cursor: pointer;
  --ui-logic-button-transition-ms: 160ms;
  --ui-logic-button-press-scale: 0.97;
  transition:
    transform var(--ui-logic-button-transition-ms) ease,
    color var(--ui-logic-button-transition-ms) ease,
    background-color var(--ui-logic-button-transition-ms) ease,
    border-color var(--ui-logic-button-transition-ms) ease;
}

.ui-logic-button--variant-and,
.ui-logic-button[data-variant="and"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 86%, var(--ui-accent) 14%);
  border-color: color-mix(in oklab, var(--ui-border) 84%, var(--ui-accent) 16%);
}

.ui-logic-button--variant-or,
.ui-logic-button[data-variant="or"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 84%, var(--ui-danger) 16%);
  border-color: color-mix(in oklab, var(--ui-border) 80%, var(--ui-danger) 20%);
}

.ui-logic-button.is-hovered,
.ui-logic-button[data-hovered="true"] {
  color: var(--ui-fg);
  background: color-mix(in oklab, var(--ui-bg-muted) 72%, var(--ui-accent) 28%);
}

.ui-logic-button.is-active,
.ui-logic-button[data-pressed="true"] {
  transform: scale(var(--ui-logic-button-press-scale));
}

.ui-logic-button--disabled,
.ui-logic-button[data-disabled="true"] {
  opacity: 0.56;
  cursor: not-allowed;
}

.ui-logic-button--focus-visible {
  outline: 2px solid color-mix(in oklab, var(--ui-accent) 46%, transparent);
  outline-offset: 2px;
}

.ui-logic-button__label {
  font-size: 0.75rem;
  line-height: 1;
  letter-spacing: 0.02em;
  white-space: nowrap;
}

.ui-logic-button--custom-class,
.ui-logic-button[data-custom-class="true"] {
  box-shadow: 0 0 0 1px color-mix(in oklab, var(--ui-accent) 24%, transparent) inset;
}

.ui-logic-button[data-motion-source="custom"],
.ui-logic-button[data-custom-motion="true"] {
  --ui-logic-button-motion-source: custom;
}
"#;
