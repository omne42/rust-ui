pub const CSS: &str = r#"
.ui-clear-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 1.75rem;
  min-height: 1.75rem;
  padding: 0;
  border: 1px solid transparent;
  border-radius: var(--ui-radius-sm);
  background: transparent;
  color: var(--ui-fg-muted);
  cursor: pointer;
  transition: background-color 120ms ease, color 120ms ease, transform 120ms ease;
}

.ui-clear-button--variant-default,
.ui-clear-button[data-variant="default"] {
  color: var(--ui-fg-muted);
}

.ui-clear-button--variant-over-background,
.ui-clear-button[data-variant="over-background"] {
  color: color-mix(in oklab, var(--ui-fg) 92%, white 8%);
  background: color-mix(in oklab, var(--ui-bg) 72%, transparent);
}

.ui-clear-button--inset,
.ui-clear-button[data-inset="true"] {
  margin-inline-end: calc(var(--ui-space-xs) * -1);
}

.ui-clear-button.is-hovered,
.ui-clear-button[data-hovered="true"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 82%, var(--ui-accent) 18%);
  color: var(--ui-fg);
}

.ui-clear-button.is-active,
.ui-clear-button[data-pressed="true"] {
  transform: scale(0.96);
}

.ui-clear-button--disabled,
.ui-clear-button[data-disabled="true"] {
  opacity: 0.52;
  cursor: not-allowed;
}

.ui-clear-button--focus-visible {
  outline: 2px solid color-mix(in oklab, var(--ui-accent) 48%, transparent);
  outline-offset: 2px;
}

.ui-clear-button__label {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  line-height: var(--ui-line-height-100, 1rem);
  min-width: 0;
}

.ui-clear-button--custom-class,
.ui-clear-button[data-custom-class="true"] {
  box-shadow: 0 0 0 1px color-mix(in oklab, var(--ui-accent) 24%, transparent) inset;
}
"#;
