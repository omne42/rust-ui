pub const CSS: &str = r#"
.ui-chip {
  --ui-chip-font-size: var(--ui-button-size-s-font-size, 13px);
  --ui-chip-line-height: var(--ui-button-size-s-line-height, 18px);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  white-space: nowrap;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
  border-radius: 9999px;
  border: 1px solid transparent;
  box-sizing: border-box;
  font-size: var(--ui-chip-font-size);
  font-weight: 500;
  line-height: var(--ui-chip-line-height);
  color: var(--ui-fg);
  background: var(--ui-bg-muted);
  border-color: var(--ui-border);
}

.ui-chip__content {
  min-width: 0;
}

.ui-chip--size-sm,
.ui-chip[data-size="sm"] {
  height: 24px;
  padding: 0 10px;
  --ui-chip-font-size: var(--ui-button-size-xs-font-size, 12px);
  --ui-chip-line-height: var(--ui-button-size-xs-line-height, 16px);
}

.ui-chip--size-md,
.ui-chip[data-size="md"] {
  height: 28px;
  padding: 0 12px;
  --ui-chip-font-size: var(--ui-button-size-s-font-size, 13px);
  --ui-chip-line-height: var(--ui-button-size-s-line-height, 18px);
}

.ui-chip--size-lg,
.ui-chip[data-size="lg"] {
  height: 32px;
  padding: 0 14px;
  --ui-chip-font-size: var(--ui-button-size-m-font-size, 14px);
  --ui-chip-line-height: var(--ui-button-size-m-line-height, 20px);
}

.ui-chip--variant-default,
.ui-chip[data-variant="default"] {
  background: var(--ui-bg-muted);
  border-color: var(--ui-border);
  color: var(--ui-fg);
}

.ui-chip--variant-accent,
.ui-chip[data-variant="accent"] {
  background: var(--ui-accent-soft);
  border-color: color-mix(in oklch, var(--ui-accent) 24%, var(--ui-border));
  color: var(--ui-fg);
}

.ui-chip--variant-danger,
.ui-chip[data-variant="danger"] {
  background: color-mix(in oklch, var(--ui-danger) 12%, var(--ui-bg-muted));
  border-color: color-mix(in oklch, var(--ui-danger) 35%, var(--ui-border));
  color: var(--ui-fg);
}

.ui-chip--variant-outline,
.ui-chip[data-variant="outline"] {
  background: transparent;
  border-color: var(--ui-border);
  color: var(--ui-fg);
}

.ui-chip--enabled,
.ui-chip[data-enabled="true"] {
  --ui-chip-enabled-state: 1;
}

.ui-chip--disabled,
.ui-chip[data-state="disabled"],
.ui-chip[data-disabled="true"] {
  opacity: 0.65;
  cursor: not-allowed;
}

.ui-chip--static,
.ui-chip[data-state="static"],
.ui-chip[data-static="true"] {
  padding-inline-end: 12px;
}

.ui-chip--removable,
.ui-chip[data-state="removable"],
.ui-chip[data-removable="true"] {
  padding-inline-end: 8px;
}

.ui-chip--dismiss-label-default,
.ui-chip[data-dismiss-label-source="default"] {
  --ui-chip-dismiss-label-source: 0;
}

.ui-chip--dismiss-label-custom,
.ui-chip[data-dismiss-label-source="custom"] {
  --ui-chip-dismiss-label-source: 1;
}

.ui-chip--custom-class,
.ui-chip[data-custom-class="true"],
.ui-chip[data-class-source="custom"] {
  --ui-chip-custom-class: 1;
}

.ui-chip__dismiss {
  width: 18px;
  height: 18px;
  border-radius: 9999px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border: 0;
  padding: 0;
  margin: 0;
  background: transparent;
  color: inherit;
  cursor: pointer;
  opacity: 0.7;
}

.ui-chip__dismiss:hover {
  opacity: 1;
  background: color-mix(in oklch, var(--ui-fg) 8%, transparent);
}

.ui-chip__dismiss:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-chip__dismiss[data-disabled="true"] {
  cursor: not-allowed;
  opacity: 0.45;
}

.ui-chip__dismiss[data-label-source="custom"] {
  --ui-chip-dismiss-label-source: 1;
}
"#;
