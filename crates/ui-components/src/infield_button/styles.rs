pub const CSS: &str = r#"
.ui-infield-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 1.875rem;
  min-height: 1.875rem;
  padding: 0 var(--ui-space-sm);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-sm);
  background: var(--ui-bg-muted);
  color: var(--ui-fg);
  cursor: pointer;
  transition: background-color 120ms ease, border-color 120ms ease, color 120ms ease, transform 120ms ease;
}

.ui-infield-button--quiet,
.ui-infield-button[data-quiet="true"] {
  background: transparent;
  border-color: transparent;
}

.ui-infield-button--invalid,
.ui-infield-button[data-invalid="true"] {
  border-color: color-mix(in oklab, var(--ui-danger) 55%, var(--ui-border));
  color: color-mix(in oklab, var(--ui-danger) 70%, var(--ui-fg));
}

.ui-infield-button.is-hovered,
.ui-infield-button[data-hovered="true"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 80%, var(--ui-accent) 20%);
}

.ui-infield-button.is-active,
.ui-infield-button[data-active="true"],
.ui-infield-button[data-pressed="true"] {
  transform: scale(0.98);
}

.ui-infield-button--disabled,
.ui-infield-button[data-disabled="true"] {
  opacity: 0.56;
  cursor: not-allowed;
}

.ui-infield-button--focus-visible {
  outline: 2px solid color-mix(in oklab, var(--ui-accent) 45%, transparent);
  outline-offset: 2px;
}

.ui-infield-button__label {
  min-width: 0;
  white-space: nowrap;
  font-size: 0.875rem;
  line-height: 1;
}

.ui-infield-button--custom-class,
.ui-infield-button[data-custom-class="true"] {
  box-shadow: 0 0 0 1px color-mix(in oklab, var(--ui-accent) 24%, transparent) inset;
}
"#;
