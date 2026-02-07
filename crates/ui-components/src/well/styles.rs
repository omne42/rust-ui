pub const CSS: &str = r#"
.ui-well {
  display: block;
  border-radius: var(--ui-radius-lg);
  border: 1px solid var(--ui-border);
  color: var(--ui-fg);
  background: var(--ui-bg-muted);
  box-shadow: inset 0 1px 0 color-mix(in oklab, white 9%, transparent), var(--ui-shadow-xs);
}

.ui-well--density-comfortable,
.ui-well[data-density="comfortable"] {
  padding: var(--ui-space-lg);
}

.ui-well--density-compact,
.ui-well[data-density="compact"] {
  padding: var(--ui-space-sm) var(--ui-space-md);
}

.ui-well--tone-default,
.ui-well[data-tone="default"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 92%, var(--ui-bg) 8%);
}

.ui-well--tone-quiet,
.ui-well[data-tone="quiet"] {
  background: var(--ui-bg);
  border: 1px dashed var(--ui-border);
}

.ui-well--tone-strong,
.ui-well[data-tone="strong"] {
  background: color-mix(in oklab, var(--ui-accent-soft) 36%, var(--ui-bg-muted) 64%);
  border-color: color-mix(in oklab, var(--ui-accent) 42%, var(--ui-border) 58%);
}

.ui-well--inset,
.ui-well[data-inset="true"] {
  box-shadow: inset 0 1px 0 color-mix(in oklab, white 8%, transparent),
    inset 0 0 0 1px color-mix(in oklab, var(--ui-border) 86%, transparent);
}

.ui-well--label-custom,
.ui-well[data-label-source="custom"] {
  border-width: 2px;
}

.ui-well--custom-class,
.ui-well[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
}
"#;
