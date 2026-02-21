pub const CSS: &str = r#"
.ui-chip {
  --ui-chip-font-size: var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size));
  --ui-chip-line-height: var(--ui-button-size-s-line-height, var(--ui-fallback-button-size-s-line-height));
  --ui-chip-gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-chip-padding-inline-static: var(--ui-space-md, var(--ui-fallback-space-md));
  --ui-chip-padding-inline-removable: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-chip-radius: var(--ui-button-radius-full, var(--ui-fallback-radius-lg));
  --ui-chip-dismiss-size: calc(
    var(--ui-space-sm, var(--ui-fallback-space-sm)) * 2 +
      var(--ui-space-xs, var(--ui-fallback-space-xs)) / 2
  );
  --ui-chip-focus-ring-color: var(--ui-focus-ring, var(--ui-fallback-accent));
  --ui-chip-focus-offset: calc(var(--ui-space-xs, var(--ui-fallback-space-xs)) / 2);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--ui-chip-gap);
  white-space: nowrap;
  user-select: none;
  -webkit-tap-highlight-color: transparent;
  border-radius: var(--ui-chip-radius);
  border: solid transparent;
  border-width: thin;
  box-sizing: border-box;
  font-size: var(--ui-chip-font-size);
  font-weight: 500;
  line-height: var(--ui-chip-line-height);
  color: var(--ui-fg, var(--ui-fallback-fg));
  background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
  border-color: var(--ui-border, var(--ui-fallback-border));
  opacity: var(--ui-chip-opacity, 1);
  transform: translateY(var(--ui-chip-translate-y, 0px)) scale(var(--ui-chip-scale, 1));
  transform-origin: center;
}

.ui-chip__content {
  min-width: 0;
}

.ui-chip--size-sm,
.ui-chip[data-size="sm"] {
  height: calc(var(--ui-space-md, var(--ui-fallback-space-md)) * 2);
  padding: 0 calc(var(--ui-space-sm, var(--ui-fallback-space-sm)) * 1.25);
  --ui-chip-font-size: var(--ui-button-size-xs-font-size, var(--ui-fallback-font-size-100));
  --ui-chip-line-height: var(--ui-button-size-xs-line-height, var(--ui-fallback-line-height-100));
}

.ui-chip--size-md,
.ui-chip[data-size="md"] {
  height: calc(
    var(--ui-space-md, var(--ui-fallback-space-md)) * 2 +
      var(--ui-space-sm, var(--ui-fallback-space-sm)) / 2
  );
  padding: 0 var(--ui-space-md, var(--ui-fallback-space-md));
  --ui-chip-font-size: var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size));
  --ui-chip-line-height: var(--ui-button-size-s-line-height, var(--ui-fallback-button-size-s-line-height));
}

.ui-chip--size-lg,
.ui-chip[data-size="lg"] {
  height: calc(
    var(--ui-space-md, var(--ui-fallback-space-md)) * 2 +
      var(--ui-space-sm, var(--ui-fallback-space-sm))
  );
  padding: 0 calc(var(--ui-space-sm, var(--ui-fallback-space-sm)) * 1.75);
  --ui-chip-font-size: var(--ui-button-size-m-font-size, var(--ui-fallback-font-size-150));
  --ui-chip-line-height: var(--ui-button-size-m-line-height, var(--ui-fallback-line-height-150));
}

.ui-chip--variant-default,
.ui-chip[data-variant="default"] {
  background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
  border-color: var(--ui-border, var(--ui-fallback-border));
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-chip--variant-accent,
.ui-chip[data-variant="accent"] {
  background: var(--ui-accent-soft, var(--ui-fallback-accent));
  border-color: color-mix(
    in oklch,
    var(--ui-accent, var(--ui-fallback-accent)) 24%,
    var(--ui-border, var(--ui-fallback-border))
  );
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-chip--variant-danger,
.ui-chip[data-variant="danger"] {
  background: color-mix(
    in oklch,
    var(--ui-danger, var(--ui-fallback-danger)) 12%,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted))
  );
  border-color: color-mix(
    in oklch,
    var(--ui-danger, var(--ui-fallback-danger)) 35%,
    var(--ui-border, var(--ui-fallback-border))
  );
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-chip--variant-outline,
.ui-chip[data-variant="outline"] {
  background: transparent;
  border-color: var(--ui-border, var(--ui-fallback-border));
  color: var(--ui-fg, var(--ui-fallback-fg));
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
  padding-inline-end: var(--ui-chip-padding-inline-static);
}

.ui-chip--removable,
.ui-chip[data-state="removable"],
.ui-chip[data-removable="true"] {
  padding-inline-end: var(--ui-chip-padding-inline-removable);
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
  width: var(--ui-chip-dismiss-size);
  height: var(--ui-chip-dismiss-size);
  border-radius: var(--ui-chip-radius);
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
  background: color-mix(in oklch, var(--ui-fg, var(--ui-fallback-fg)) 8%, transparent);
}

.ui-chip__dismiss:focus-visible {
  outline: medium solid var(--ui-chip-focus-ring-color);
  outline-offset: var(--ui-chip-focus-offset);
}

.ui-chip__dismiss[data-disabled="true"] {
  cursor: not-allowed;
  opacity: 0.45;
}

.ui-chip__dismiss[data-label-source="custom"] {
  --ui-chip-dismiss-label-source: 1;
}
"#;
