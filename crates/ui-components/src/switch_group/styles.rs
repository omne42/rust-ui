pub const CSS: &str = r#"
.ui-switch-group {
  display: grid;
  gap: var(--ui-space-sm);
  margin: 0;
  padding: var(--ui-space-md);
  border: 1px solid color-mix(in oklab, var(--ui-border) 82%, transparent);
  border-radius: var(--ui-radius-md);
  min-width: 0;
}

.ui-switch-group--orientation-vertical,
.ui-switch-group[data-orientation="vertical"] {
  align-items: stretch;
}

.ui-switch-group--orientation-horizontal,
.ui-switch-group[data-orientation="horizontal"] {
  align-items: start;
}

.ui-switch-group--tone-default,
.ui-switch-group[data-tone="default"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 50%, transparent);
}

.ui-switch-group--tone-muted,
.ui-switch-group[data-tone="muted"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 28%, transparent);
}

.ui-switch-group__label {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--ui-fg);
}

.ui-switch-group--required .ui-switch-group__label::after,
.ui-switch-group[data-required="true"] .ui-switch-group__label::after {
  content: "*";
  margin-left: 4px;
  color: var(--ui-danger);
}

.ui-switch-group__group {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-switch-group--orientation-horizontal .ui-switch-group__group,
.ui-switch-group[data-orientation="horizontal"] .ui-switch-group__group {
  flex-direction: row;
  flex-wrap: wrap;
  gap: var(--ui-space-sm);
}

.ui-switch-group__description,
.ui-switch-group__error {
  margin: 0;
  font-size: 0.75rem;
  line-height: 1.35;
}

.ui-switch-group__description {
  color: var(--ui-fg-muted);
}

.ui-switch-group__error {
  color: color-mix(in oklab, var(--ui-danger) 74%, var(--ui-fg) 26%);
}

.ui-switch-group--invalid .ui-switch-group__group,
.ui-switch-group[data-invalid="true"] .ui-switch-group__group {
  border-left: 2px solid color-mix(in oklab, var(--ui-danger) 68%, transparent);
  padding-left: var(--ui-space-sm);
}

.ui-switch-group--label-custom .ui-switch-group__label,
.ui-switch-group[data-label-source="custom"] .ui-switch-group__label {
  letter-spacing: 0.01em;
}

.ui-switch-group--disabled,
.ui-switch-group[data-disabled="true"] {
  opacity: 0.68;
}

.ui-switch-group--custom-class,
.ui-switch-group[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}
"#;
