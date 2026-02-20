pub const CSS: &str = r#"
.ui-close-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border: 1px solid transparent;
  border-radius: 999px;
  background: color-mix(in oklab, var(--ui-bg-muted) 80%, transparent);
  color: var(--ui-fg-muted);
  cursor: pointer;
  padding: 0;
  flex-shrink: 0;
  transition: transform 250ms cubic-bezier(0.22, 1, 0.36, 1), color 150ms ease, background-color 100ms ease, box-shadow 150ms ease;
}

.ui-close-button--variant-default,
.ui-close-button[data-variant="default"] {
  color: var(--ui-fg-muted);
}

.ui-close-button--variant-over-background,
.ui-close-button[data-variant="over-background"] {
  color: color-mix(in oklab, var(--ui-fg) 92%, white 8%);
  background: color-mix(in oklab, var(--ui-bg) 70%, transparent);
}

.ui-close-button--size-sm,
.ui-close-button[data-size="sm"] {
  width: 1.25rem;
  height: 1.25rem;
  --ui-close-button-icon-size: 0.75rem;
}

.ui-close-button--size-md,
.ui-close-button[data-size="md"] {
  width: 1.5rem;
  height: 1.5rem;
  --ui-close-button-icon-size: 0.875rem;
}

.ui-close-button--size-lg,
.ui-close-button[data-size="lg"] {
  width: 1.75rem;
  height: 1.75rem;
  --ui-close-button-icon-size: 1rem;
}

.ui-close-button--size-xl,
.ui-close-button[data-size="xl"] {
  width: 2rem;
  height: 2rem;
  --ui-close-button-icon-size: 1.125rem;
}

.ui-close-button.is-hovered,
.ui-close-button[data-hovered="true"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 72%, var(--ui-accent) 28%);
  color: var(--ui-fg);
}

.ui-close-button.is-active,
.ui-close-button[data-pressed="true"] {
  transform: scale(0.93);
}

.ui-close-button--disabled,
.ui-close-button[data-disabled="true"] {
  opacity: 0.56;
  cursor: not-allowed;
}

.ui-close-button--focus-visible {
  outline: 2px solid color-mix(in oklab, var(--ui-accent) 48%, transparent);
  outline-offset: 2px;
}

.ui-close-button__icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: var(--ui-close-button-icon-size, 0.875rem);
  height: var(--ui-close-button-icon-size, 0.875rem);
}

.ui-close-button__icon svg {
  width: 100%;
  height: 100%;
}

.ui-close-button--custom-class,
.ui-close-button[data-custom-class="true"] {
  box-shadow: 0 0 0 1px color-mix(in oklab, var(--ui-accent) 24%, transparent) inset;
}
"#;
