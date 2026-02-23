pub const CSS: &str = r#"
.ui-tag {
  --ui-tag-font-size: var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size));
  --ui-tag-line-height: var(--ui-button-size-s-line-height, var(--ui-fallback-button-size-s-line-height));
  display: inline-flex;
  align-items: center;
  gap: var(--ui-button-size-s-gap, var(--ui-space-2xs, var(--ui-fallback-space-2xs)));
  min-width: 0;
  border-radius: var(--ui-radius-full, var(--ui-fallback-radius-full));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid transparent;
  font-size: var(--ui-tag-font-size);
  font-weight: 500;
  line-height: var(--ui-tag-line-height);
  white-space: nowrap;
  user-select: none;
}

.ui-tag--size-sm,
.ui-tag[data-size="sm"] {
  height: var(--ui-button-size-xs-height, calc(var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.75));
  padding-inline: var(--ui-button-size-xs-padding-x, var(--ui-space-xs, var(--ui-fallback-space-xs)));
  --ui-tag-font-size: var(--ui-button-size-xs-font-size, var(--ui-font-size-100, var(--ui-fallback-font-size-100)));
  --ui-tag-line-height: var(--ui-button-size-xs-line-height, var(--ui-line-height-100, var(--ui-fallback-line-height-100)));
}

.ui-tag--size-md,
.ui-tag[data-size="md"] {
  height: var(--ui-button-size-s-height, calc(var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.875));
  padding-inline: var(--ui-button-size-s-padding-x, var(--ui-space-sm, var(--ui-fallback-space-sm)));
  --ui-tag-font-size: var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size));
  --ui-tag-line-height: var(--ui-button-size-s-line-height, var(--ui-fallback-button-size-s-line-height));
}

.ui-tag--size-lg,
.ui-tag[data-size="lg"] {
  height: var(--ui-button-size-m-height, var(--ui-fallback-button-size-m-height));
  padding-inline: var(--ui-button-size-m-padding-x, var(--ui-space-md, var(--ui-fallback-space-md)));
  --ui-tag-font-size: var(--ui-button-size-m-font-size, var(--ui-font-size-150, var(--ui-fallback-font-size-150)));
  --ui-tag-line-height: var(--ui-button-size-m-line-height, var(--ui-line-height-150, var(--ui-fallback-line-height-150)));
}

.ui-tag--variant-default,
.ui-tag[data-variant="default"] {
  background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
  border-color: color-mix(in oklab, var(--ui-border, var(--ui-fallback-border)) 84%, transparent);
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-tag--variant-surface,
.ui-tag[data-variant="surface"] {
  background: color-mix(
    in oklab,
    var(--ui-bg, var(--ui-fallback-bg)) 88%,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 12%
  );
  border-color: var(--ui-border, var(--ui-fallback-border));
  color: var(--ui-fg, var(--ui-fallback-fg));
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
  padding-inline-end: var(--ui-space-sm, var(--ui-fallback-space-sm));
}

.ui-tag--removable,
.ui-tag[data-state="removable"],
.ui-tag[data-removable="true"] {
  padding-inline-end: var(--ui-space-xs, var(--ui-fallback-space-xs));
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
  width: var(--ui-button-size-xs-icon, calc(var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.5625));
  height: var(--ui-button-size-xs-icon, calc(var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.5625));
  border-radius: var(--ui-radius-full, var(--ui-fallback-radius-full));
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
  background: color-mix(in oklab, var(--ui-fg, var(--ui-fallback-fg)) 9%, transparent);
}

.ui-tag__remove:focus-visible {
  outline: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width))
    solid var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(--ui-button-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset));
}

.ui-tag__remove[data-disabled="true"] {
  cursor: not-allowed;
  opacity: 0.45;
}

.ui-tag__remove[data-label-source="custom"] {
  --ui-tag-remove-label-source: 1;
}
"#;
