pub const CSS: &str = r#"
.ui-step-list {
  --ui-step-list-marker-size: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.75
  );
  --ui-step-list-gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-step-list-marker-bg: color-mix(
    in oklab,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 28%,
    transparent
  );
  --ui-step-list-marker-fg: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  --ui-step-list-label: var(--ui-fg, var(--ui-fallback-fg));
  --ui-step-list-desc: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  --ui-step-list-connector: color-mix(
    in oklab,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 28%,
    transparent
  );
  --ui-step-list-focus-ring: var(--ui-accent, var(--ui-fallback-accent));
  --ui-step-list-focus-outline-width: var(
    --ui-button-focus-outline-width,
    var(--ui-fallback-button-focus-outline-width)
  );
  --ui-step-list-focus-outline-offset: var(
    --ui-button-focus-outline-offset,
    var(--ui-fallback-button-focus-outline-offset)
  );
  --ui-step-list-content-gap: var(--ui-space-3xs, var(--ui-fallback-space-3xs));
  --ui-step-list-connector-thickness: calc(
    var(--ui-border-width, var(--ui-fallback-border-width)) * 2
  );
  --ui-step-list-disabled-opacity: var(
    --ui-disabled-opacity,
    var(--ui-fallback-disabled-opacity)
  );
  --ui-step-list-empty-min-block-size: var(
    --ui-component-height-100,
    var(--ui-fallback-component-height-100)
  );
  display: flex;
  gap: var(--ui-step-list-gap);
  margin: 0;
  padding: 0;
}

.ui-step-list--orientation-horizontal,
.ui-step-list[data-orientation="horizontal"] {
  flex-direction: row;
  align-items: stretch;
}

.ui-step-list--orientation-vertical,
.ui-step-list[data-orientation="vertical"] {
  flex-direction: column;
}

.ui-step-list__item {
  position: relative;
  display: flex;
  flex: 1 1 0;
  min-inline-size: 0;
}

.ui-step-list__button {
  appearance: none;
  border: none;
  margin: 0;
  padding: 0;
  inline-size: 100%;
  background: transparent;
  display: inline-flex;
  align-items: flex-start;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  text-align: start;
  cursor: pointer;
  color: inherit;
}

.ui-step-list__button:focus-visible {
  outline: var(--ui-step-list-focus-outline-width) solid
    color-mix(in oklab, var(--ui-step-list-focus-ring) 84%, transparent);
  outline-offset: var(--ui-step-list-focus-outline-offset);
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
}

.ui-step-list__marker {
  inline-size: var(--ui-step-list-marker-size);
  block-size: var(--ui-step-list-marker-size);
  border-radius: var(--ui-radius-full, var(--ui-fallback-radius-full));
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  font-weight: 700;
  flex-shrink: 0;
  background: var(--ui-step-list-marker-bg);
  color: var(--ui-step-list-marker-fg);
}

.ui-step-list__content {
  display: inline-flex;
  flex-direction: column;
  gap: var(--ui-step-list-content-gap);
  min-inline-size: 0;
}

.ui-step-list__label {
  color: var(--ui-step-list-label);
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  font-weight: 600;
}

.ui-step-list__description {
  color: var(--ui-step-list-desc);
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
}

.ui-step-list__connector {
  position: absolute;
  background: var(--ui-step-list-connector);
  pointer-events: none;
}

.ui-step-list--orientation-horizontal .ui-step-list__connector,
.ui-step-list[data-orientation="horizontal"] .ui-step-list__connector {
  inset-block-start: calc(
    var(--ui-step-list-marker-size) / 2 - (var(--ui-step-list-connector-thickness) / 2)
  );
  inset-inline-start: calc(
    var(--ui-step-list-marker-size) + var(--ui-space-sm, var(--ui-fallback-space-sm))
  );
  inset-inline-end: calc(var(--ui-space-sm, var(--ui-fallback-space-sm)) * -0.5);
  block-size: var(--ui-step-list-connector-thickness);
}

.ui-step-list--orientation-vertical .ui-step-list__connector,
.ui-step-list[data-orientation="vertical"] .ui-step-list__connector {
  inset-inline-start: calc(
    var(--ui-step-list-marker-size) / 2 - (var(--ui-step-list-connector-thickness) / 2)
  );
  inset-block-start: calc(
    var(--ui-step-list-marker-size) + var(--ui-space-2xs, var(--ui-fallback-space-2xs))
  );
  inset-block-end: calc(var(--ui-space-2xs, var(--ui-fallback-space-2xs)) * -1);
  inline-size: var(--ui-step-list-connector-thickness);
}

.ui-step-list__connector[data-last="true"] {
  display: none;
}

.ui-step-list__item--pending,
.ui-step-list__item[data-status="pending"] {
  --ui-step-list-marker-bg: color-mix(
    in oklab,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 24%,
    transparent
  );
  --ui-step-list-marker-fg: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-step-list__item--current,
.ui-step-list__item[data-status="current"] {
  --ui-step-list-marker-bg: color-mix(
    in oklab,
    var(--ui-accent, var(--ui-fallback-accent)) 88%,
    transparent
  );
  --ui-step-list-marker-fg: var(--ui-bg, var(--ui-fallback-bg));
  --ui-step-list-label: var(--ui-accent, var(--ui-fallback-accent));
}

.ui-step-list__item--completed,
.ui-step-list__item[data-status="completed"] {
  --ui-step-list-marker-bg: color-mix(
    in oklab,
    var(--ui-success, var(--ui-accent, var(--ui-fallback-accent))) 90%,
    transparent
  );
  --ui-step-list-marker-fg: var(--ui-bg, var(--ui-fallback-bg));
  --ui-step-list-label: color-mix(
    in oklab,
    var(--ui-success, var(--ui-accent, var(--ui-fallback-accent))) 86%,
    var(--ui-fg, var(--ui-fallback-fg))
  );
}

.ui-step-list__item--disabled,
.ui-step-list__item[data-status="disabled"],
.ui-step-list--disabled .ui-step-list__item,
.ui-step-list[data-disabled="true"] .ui-step-list__item {
  opacity: var(--ui-step-list-disabled-opacity);
}

.ui-step-list__item--disabled .ui-step-list__button,
.ui-step-list__item[data-status="disabled"] .ui-step-list__button {
  cursor: not-allowed;
}

.ui-step-list--size-s,
.ui-step-list[data-size="s"] {
  --ui-step-list-marker-size: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.625
  );
  --ui-step-list-gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-step-list--size-m,
.ui-step-list[data-size="m"] {
  --ui-step-list-marker-size: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.75
  );
}

.ui-step-list--size-l,
.ui-step-list[data-size="l"] {
  --ui-step-list-marker-size: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.875
  );
  --ui-step-list-gap: var(--ui-space-md, var(--ui-fallback-space-md));
}

.ui-step-list--size-xl,
.ui-step-list[data-size="xl"] {
  --ui-step-list-marker-size: var(
    --ui-component-height-100,
    var(--ui-fallback-component-height-100)
  );
  --ui-step-list-gap: var(--ui-space-md, var(--ui-fallback-space-md));
}

.ui-step-list--emphasized,
.ui-step-list[data-emphasized="true"] {
  --ui-step-list-label: color-mix(
    in oklab,
    var(--ui-fg, var(--ui-fallback-fg)) 90%,
    var(--ui-accent, var(--ui-fallback-accent))
  );
}

.ui-step-list--custom-class,
.ui-step-list[data-custom-class="true"],
.ui-step-list[data-class-source="custom"] {
  --ui-step-list-custom-class: 1;
}

.ui-step-list[data-empty="true"] {
  min-block-size: var(--ui-step-list-empty-min-block-size);
}
"#;
