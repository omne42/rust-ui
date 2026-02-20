pub const CSS: &str = r#"
.ui-avatar-group {
  --ui-avatar-group-size: 2rem;
  --ui-avatar-group-overlap: 10px;
  --ui-avatar-group-font-size: var(--ui-font-size-100, 12px);
  --ui-avatar-group-overflow-padding: 0.375rem;
  display: inline-flex;
  align-items: center;
  min-height: var(--ui-avatar-group-size);
}

.ui-avatar-group__item {
  position: relative;
  z-index: 1;
  display: inline-flex;
  align-items: center;
}

.ui-avatar-group__avatar {
  border: 2px solid var(--ui-bg);
  border-radius: 9999px;
  box-shadow: var(--ui-shadow-sm);
  background: var(--ui-bg-muted);
}

.ui-avatar-group__item:not(:first-child) {
  margin-left: calc(var(--ui-avatar-group-overlap) * -1);
}

.ui-avatar-group__overflow {
  position: relative;
  z-index: 2;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: var(--ui-avatar-group-size);
  height: var(--ui-avatar-group-size);
  padding-inline: var(--ui-avatar-group-overflow-padding);
  border-radius: 9999px;
  border: 2px solid var(--ui-bg);
  background: var(--ui-bg-muted);
  color: var(--ui-fg);
  font-size: var(--ui-avatar-group-font-size);
  font-weight: 650;
  line-height: var(--ui-line-height-100, 16px);
  box-shadow: var(--ui-shadow-sm);
}

.ui-avatar-group__overflow:not(:first-child) {
  margin-left: calc(var(--ui-avatar-group-overlap) * -1);
}

.ui-avatar-group--size-sm,
.ui-avatar-group[data-size="sm"] {
  --ui-avatar-group-size: 1.5rem;
  --ui-avatar-group-overlap: 8px;
  --ui-avatar-group-font-size: var(--ui-font-size-100, 12px);
  --ui-avatar-group-overflow-padding: 0.25rem;
}

.ui-avatar-group--size-md,
.ui-avatar-group[data-size="md"] {
  --ui-avatar-group-size: 2rem;
  --ui-avatar-group-overlap: 10px;
  --ui-avatar-group-font-size: var(--ui-font-size-100, 12px);
  --ui-avatar-group-overflow-padding: 0.375rem;
}

.ui-avatar-group--size-lg,
.ui-avatar-group[data-size="lg"] {
  --ui-avatar-group-size: 2.5rem;
  --ui-avatar-group-overlap: 12px;
  --ui-avatar-group-font-size: var(--ui-button-size-s-font-size, 13px);
  --ui-avatar-group-overflow-padding: 0.5rem;
}

.ui-avatar-group--stable,
.ui-avatar-group[data-state="stable"] {
  --ui-avatar-group-state: 0;
}

.ui-avatar-group--overflow,
.ui-avatar-group[data-state="overflow"],
.ui-avatar-group[data-has-overflow="true"] {
  --ui-avatar-group-state: 1;
}

.ui-avatar-group--overflow .ui-avatar-group__overflow,
.ui-avatar-group[data-has-overflow="true"] .ui-avatar-group__overflow,
.ui-avatar-group[data-state="overflow"] .ui-avatar-group__overflow {
  background: color-mix(in oklch, var(--ui-accent-soft) 72%, var(--ui-bg-muted) 28%);
  border-color: color-mix(in oklch, var(--ui-accent) 42%, var(--ui-bg) 58%);
}

.ui-avatar-group--empty,
.ui-avatar-group[data-empty="true"],
.ui-avatar-group[data-state="empty"] {
  opacity: 0.88;
}

.ui-avatar-group--label-source-default,
.ui-avatar-group[data-aria-label-source="default"] {
  --ui-avatar-group-aria-label-source: 0;
}

.ui-avatar-group--label-source-custom,
.ui-avatar-group[data-custom-aria-label="true"],
.ui-avatar-group[data-aria-label-source="custom"] {
  --ui-avatar-group-aria-label-source: 1;
}

.ui-avatar-group--custom-class,
.ui-avatar-group[data-custom-class="true"],
.ui-avatar-group[data-class-source="custom"] {
  --ui-avatar-group-custom-class: 1;
}
"#;
