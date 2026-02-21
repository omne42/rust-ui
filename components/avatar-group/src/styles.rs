pub const CSS: &str = r#"
.ui-avatar-group {
  --ui-avatar-group-size-sm: var(--ui-avatar-size-sm, var(--ui-fallback-avatar-size-sm));
  --ui-avatar-group-size-md: var(--ui-avatar-size-md, var(--ui-fallback-avatar-size-md));
  --ui-avatar-group-size-lg: var(--ui-avatar-size-lg, var(--ui-fallback-avatar-size-lg));
  --ui-avatar-group-overlap-sm: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-avatar-group-overlap-md: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-avatar-group-overlap-lg: var(--ui-space-md, var(--ui-fallback-space-md));
  --ui-avatar-group-font-size-sm: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  --ui-avatar-group-font-size-md: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  --ui-avatar-group-font-size-lg: var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size));
  --ui-avatar-group-overflow-padding-sm: calc(var(--ui-space-xs, var(--ui-fallback-space-xs)) / 2);
  --ui-avatar-group-overflow-padding-md: calc(var(--ui-space-xs, var(--ui-fallback-space-xs)) * 0.75);
  --ui-avatar-group-overflow-padding-lg: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-avatar-group-border-width: calc(var(--ui-border-width, var(--ui-fallback-border-width)) * 2);
  --ui-avatar-group-overflow-radius: var(--ui-button-radius-full, var(--ui-fallback-button-radius-full));
  --ui-avatar-group-line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  --ui-avatar-group-ring-color: var(--ui-bg, var(--ui-fallback-bg));
  --ui-avatar-group-surface: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
  --ui-avatar-group-text-color: var(--ui-fg, var(--ui-fallback-fg));
  --ui-avatar-group-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
  --ui-avatar-group-overflow-accent-soft: var(--ui-accent-soft, var(--ui-fallback-accent-soft));
  --ui-avatar-group-overflow-accent: var(--ui-accent, var(--ui-fallback-accent));
  --ui-avatar-group-size: var(--ui-avatar-group-size-md);
  --ui-avatar-group-overlap: var(--ui-avatar-group-overlap-md);
  --ui-avatar-group-font-size: var(--ui-avatar-group-font-size-md);
  --ui-avatar-group-overflow-padding: var(--ui-avatar-group-overflow-padding-md);
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
  border: var(--ui-avatar-group-border-width) solid var(--ui-avatar-group-ring-color);
  border-radius: var(--ui-avatar-group-overflow-radius);
  box-shadow: var(--ui-avatar-group-shadow);
  background: var(--ui-avatar-group-surface);
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
  border-radius: var(--ui-avatar-group-overflow-radius);
  border: var(--ui-avatar-group-border-width) solid var(--ui-avatar-group-ring-color);
  background: var(--ui-avatar-group-surface);
  color: var(--ui-avatar-group-text-color);
  font-size: var(--ui-avatar-group-font-size);
  font-weight: 650;
  line-height: var(--ui-avatar-group-line-height);
  box-shadow: var(--ui-avatar-group-shadow);
}

.ui-avatar-group__overflow:not(:first-child) {
  margin-left: calc(var(--ui-avatar-group-overlap) * -1);
}

.ui-avatar-group--size-sm,
.ui-avatar-group[data-size="sm"] {
  --ui-avatar-group-size: var(--ui-avatar-group-size-sm);
  --ui-avatar-group-overlap: var(--ui-avatar-group-overlap-sm);
  --ui-avatar-group-font-size: var(--ui-avatar-group-font-size-sm);
  --ui-avatar-group-overflow-padding: var(--ui-avatar-group-overflow-padding-sm);
}

.ui-avatar-group--size-md,
.ui-avatar-group[data-size="md"] {
  --ui-avatar-group-size: var(--ui-avatar-group-size-md);
  --ui-avatar-group-overlap: var(--ui-avatar-group-overlap-md);
  --ui-avatar-group-font-size: var(--ui-avatar-group-font-size-md);
  --ui-avatar-group-overflow-padding: var(--ui-avatar-group-overflow-padding-md);
}

.ui-avatar-group--size-lg,
.ui-avatar-group[data-size="lg"] {
  --ui-avatar-group-size: var(--ui-avatar-group-size-lg);
  --ui-avatar-group-overlap: var(--ui-avatar-group-overlap-lg);
  --ui-avatar-group-font-size: var(--ui-avatar-group-font-size-lg);
  --ui-avatar-group-overflow-padding: var(--ui-avatar-group-overflow-padding-lg);
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
  background: color-mix(
    in oklch,
    var(--ui-avatar-group-overflow-accent-soft) 72%,
    var(--ui-avatar-group-surface) 28%
  );
  border-color: color-mix(
    in oklch,
    var(--ui-avatar-group-overflow-accent) 42%,
    var(--ui-avatar-group-ring-color) 58%
  );
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
