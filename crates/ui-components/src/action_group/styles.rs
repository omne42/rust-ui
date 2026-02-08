pub const CSS: &str = r#"
.ui-action-group {
  display: grid;
  gap: var(--ui-space-2xs);
}

.ui-action-group--tone-default,
.ui-action-group[data-tone="default"] {
  color: var(--ui-fg);
}

.ui-action-group--tone-quiet,
.ui-action-group[data-tone="quiet"] {
  color: var(--ui-fg-muted);
}

.ui-action-group--tone-strong,
.ui-action-group[data-tone="strong"] {
  color: color-mix(in oklab, var(--ui-fg) 80%, var(--ui-accent) 20%);
}

.ui-action-group--disabled,
.ui-action-group[data-disabled="true"] {
  opacity: 0.72;
}

.ui-action-group--has-selection,
.ui-action-group[data-has-selection="true"] {
  box-shadow: inset 0 0 0 1px color-mix(in oklab, var(--ui-accent) 24%, transparent);
}

.ui-action-group--custom-class,
.ui-action-group[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
}

.ui-action-group__list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--ui-space-2xs);
  margin: 0;
  padding: 0;
  list-style-type: none;
}

.ui-action-group[data-selection-mode="single"] .ui-action-group__item,
.ui-action-group[data-selection-mode="multiple"] .ui-action-group__item,
.ui-action-group[data-selection-mode="none"] .ui-action-group__item {
  min-height: 2rem;
}

.ui-action-group__item {
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-sm);
  background: var(--ui-bg);
  color: inherit;
  padding: var(--ui-space-2xs) var(--ui-space-xs);
  cursor: pointer;
}

.ui-action-group__item--selected,
.ui-action-group__item[data-selected="true"] {
  border-color: color-mix(in oklab, var(--ui-accent) 48%, var(--ui-border) 52%);
  background: color-mix(in oklab, var(--ui-accent-soft) 55%, var(--ui-bg) 45%);
}

.ui-action-group__item--disabled,
.ui-action-group__item[data-disabled="true"] {
  color: var(--ui-fg-muted);
  cursor: not-allowed;
}
"#;
