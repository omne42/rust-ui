pub const CSS: &str = r#"
.ui-disclosure-group {
  display: grid;
  gap: var(--ui-space-sm);
}

.ui-disclosure-group--selection-single,
.ui-disclosure-group[data-selection-mode="single"] {
  --ui-disclosure-group-gap: var(--ui-space-xs);
}

.ui-disclosure-group--selection-multiple,
.ui-disclosure-group[data-selection-mode="multiple"] {
  --ui-disclosure-group-gap: var(--ui-space-sm);
}

.ui-disclosure-group__list {
  display: grid;
  gap: var(--ui-disclosure-group-gap, var(--ui-space-sm));
}

.ui-disclosure-group__accordion {
  border-radius: var(--ui-radius-md);
}

.ui-disclosure-group--empty,
.ui-disclosure-group[data-empty="true"] {
  opacity: 0.78;
}

.ui-disclosure-group--disabled,
.ui-disclosure-group[data-disabled="true"] {
  opacity: 0.68;
}

.ui-disclosure-group--multiple-expanded .ui-disclosure-group__accordion,
.ui-disclosure-group[data-multiple-expanded="true"] .ui-disclosure-group__accordion {
  border-color: color-mix(in oklab, var(--ui-border) 60%, var(--ui-accent) 40%);
}

.ui-disclosure-group[data-all-collapsed="true"] .ui-disclosure-group__accordion {
  background: color-mix(in oklab, var(--ui-bg) 88%, var(--ui-bg-muted) 12%);
}

.ui-disclosure-group--custom-class,
.ui-disclosure-group[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}
"#;
