pub const CSS: &str = r#"
.ui-field-group {
  display: grid;
  gap: var(--ui-space-sm);
  min-width: 0;
  padding: var(--ui-space-sm);
  border-radius: var(--ui-radius-md);
  border: 1px solid color-mix(in oklab, var(--ui-border) 80%, transparent);
  background: color-mix(in oklab, var(--ui-bg) 85%, transparent);
}

.ui-field-group--density-comfortable,
.ui-field-group[data-density="comfortable"] {
  gap: var(--ui-space-sm);
  padding: var(--ui-space-sm);
}

.ui-field-group--density-compact,
.ui-field-group[data-density="compact"] {
  gap: var(--ui-space-xs);
  padding: var(--ui-space-xs);
}

.ui-field-group__label {
  margin: 0;
  font-size: var(--ui-font-size-150, 14px);
  line-height: var(--ui-line-height-150, 20px);
  font-weight: 600;
  color: var(--ui-fg);
}

.ui-field-group__content {
  display: grid;
  gap: var(--ui-space-sm);
  min-width: 0;
}

.ui-field-group--orientation-vertical .ui-field-group__content,
.ui-field-group[data-orientation="vertical"] .ui-field-group__content {
  grid-template-columns: minmax(0, 1fr);
}

.ui-field-group--orientation-horizontal .ui-field-group__content,
.ui-field-group[data-orientation="horizontal"] .ui-field-group__content {
  grid-template-columns: repeat(auto-fit, minmax(14rem, 1fr));
  align-items: start;
}

.ui-field-group__description {
  margin: 0;
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  color: var(--ui-fg-muted);
}

.ui-field-group--invalid,
.ui-field-group[data-invalid="true"] {
  border-color: color-mix(in oklab, var(--ui-danger) 64%, transparent);
}

.ui-field-group--invalid .ui-field-group__description,
.ui-field-group[data-invalid="true"] .ui-field-group__description {
  color: color-mix(in oklab, var(--ui-danger) 70%, var(--ui-fg) 30%);
}

.ui-field-group--disabled,
.ui-field-group[data-disabled="true"] {
  opacity: 0.68;
}

.ui-field-group--custom-class,
.ui-field-group[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}
"#;
