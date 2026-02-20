pub const CSS: &str = r#"
.ui-tag {
  --ui-tag-font-size: var(--ui-button-size-s-font-size, 13px);
  --ui-tag-line-height: var(--ui-button-size-s-line-height, 18px);
  display: inline-flex;
  align-items: center;
  gap: 6px;
  min-width: 0;
  border-radius: 9999px;
  border: 1px solid transparent;
  font-size: var(--ui-tag-font-size);
  font-weight: 500;
  line-height: var(--ui-tag-line-height);
  white-space: nowrap;
  user-select: none;
}

.ui-tag--size-sm,
.ui-tag[data-size="sm"] {
  height: 24px;
  padding-inline: 8px;
  --ui-tag-font-size: var(--ui-button-size-xs-font-size, 12px);
  --ui-tag-line-height: var(--ui-button-size-xs-line-height, 16px);
}

.ui-tag--size-md,
.ui-tag[data-size="md"] {
  height: 28px;
  padding-inline: 10px;
  --ui-tag-font-size: var(--ui-button-size-s-font-size, 13px);
  --ui-tag-line-height: var(--ui-button-size-s-line-height, 18px);
}

.ui-tag--size-lg,
.ui-tag[data-size="lg"] {
  height: 32px;
  padding-inline: 12px;
  --ui-tag-font-size: var(--ui-button-size-m-font-size, 14px);
  --ui-tag-line-height: var(--ui-button-size-m-line-height, 20px);
}

.ui-tag--variant-default,
.ui-tag[data-variant="default"] {
  background: var(--ui-bg-muted);
  border-color: color-mix(in oklab, var(--ui-border) 84%, transparent);
  color: var(--ui-fg);
}

.ui-tag--variant-surface,
.ui-tag[data-variant="surface"] {
  background: color-mix(in oklab, var(--ui-bg) 88%, var(--ui-bg-muted) 12%);
  border-color: var(--ui-border);
  color: var(--ui-fg);
}

.ui-tag--enabled,
.ui-tag[data-enabled="true"] {
  --ui-tag-enabled-state: 1;
}

.ui-tag--disabled,
.ui-tag[data-state="disabled"],
.ui-tag[data-disabled="true"] {
  opacity: 0.64;
  cursor: not-allowed;
}

.ui-tag--static,
.ui-tag[data-state="static"],
.ui-tag[data-static="true"] {
  padding-inline-end: 12px;
}

.ui-tag--removable,
.ui-tag[data-state="removable"],
.ui-tag[data-removable="true"] {
  padding-inline-end: 8px;
}

.ui-tag--custom-class,
.ui-tag[data-custom-class="true"],
.ui-tag[data-class-source="custom"] {
  --ui-tag-custom-class: 1;
}

.ui-tag__content {
  display: inline-flex;
  align-items: center;
  min-width: 0;
}

.ui-tag__remove {
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
  opacity: 0.72;
}

.ui-tag__remove:hover {
  opacity: 1;
  background: color-mix(in oklab, var(--ui-fg) 9%, transparent);
}

.ui-tag__remove:focus-visible {
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 2px;
}

.ui-tag__remove[data-disabled="true"] {
  cursor: not-allowed;
  opacity: 0.45;
}

.ui-tag__remove[data-label-source="custom"] {
  --ui-tag-remove-label-source: 1;
}
"#;
