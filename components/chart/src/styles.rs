pub const CSS: &str = r#"
.ui-chart {
  --ui-chart-gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-chart-plot-padding: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-chart-plot-height: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 7
  );
  --ui-chart-grid-stroke-width: calc(
    var(--ui-border-width, var(--ui-fallback-border-width)) * 0.4
  );
  --ui-chart-line-stroke-width: calc(
    var(--ui-border-width, var(--ui-fallback-border-width)) * 1.4
  );
  --ui-chart-dot-stroke-width: calc(
    var(--ui-border-width, var(--ui-fallback-border-width)) * 0.8
  );
  --ui-chart-motion-duration: var(
    --ui-checkbox-group-motion-duration,
    var(--ui-fallback-checkbox-group-motion-duration)
  );
  --ui-chart-motion-easing: var(
    --ui-checkbox-group-motion-easing,
    var(--ui-fallback-checkbox-group-motion-easing)
  );
  --ui-chart-legend-gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  --ui-chart-legend-item-gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-chart-legend-item-pad-y: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  --ui-chart-legend-item-pad-x: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-chart-outline-offset: var(
    --ui-border-width,
    var(--ui-fallback-border-width)
  );
  --ui-chart-focus-outline-width: calc(
    var(--ui-border-width, var(--ui-fallback-border-width)) * 2
  );
  --ui-chart-highlight-height-default: calc(
    var(--ui-space-2xs, var(--ui-fallback-space-2xs)) * 0
  );
  --ui-chart-highlight-y-default: calc(
    var(--ui-space-2xs, var(--ui-fallback-space-2xs)) * 0
  );
  display: grid;
  gap: var(--ui-chart-gap);
  width: 100%;
}

.ui-chart__plot-wrap {
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
  padding: var(--ui-chart-plot-padding);
}

.ui-chart__plot {
  width: 100%;
  height: var(--ui-chart-plot-height);
}

.ui-chart__grid-line {
  stroke: var(--ui-border, var(--ui-fallback-border));
  stroke-width: var(--ui-chart-grid-stroke-width);
}

.ui-chart__line {
  fill: none;
  stroke: var(--ui-accent, var(--ui-fallback-accent));
  stroke-width: var(--ui-chart-line-stroke-width);
  stroke-linecap: round;
  stroke-linejoin: round;
}

.ui-chart__bar {
  fill: var(--ui-accent, var(--ui-fallback-accent));
  transition: fill var(--ui-chart-motion-duration) var(--ui-chart-motion-easing);
}

.ui-chart__dot {
  fill: var(--ui-accent, var(--ui-fallback-accent));
  stroke: var(--ui-bg, var(--ui-fallback-bg-muted));
  stroke-width: var(--ui-chart-dot-stroke-width);
  transition: fill var(--ui-chart-motion-duration) var(--ui-chart-motion-easing);
}

.ui-chart__bar[data-active="true"],
.ui-chart__dot[data-active="true"] {
  fill: color-mix(
    in oklab,
    var(--ui-accent, var(--ui-fallback-accent)) 82%,
    var(--ui-fg, var(--ui-fallback-fg)) 18%
  );
}

.ui-chart__legend {
  position: relative;
  display: grid;
  gap: var(--ui-chart-legend-gap);
}

.ui-chart__legend-highlight {
  position: absolute;
  inset-inline: 0;
  height: var(--ui-active-highlight-h, var(--ui-chart-highlight-height-default));
  transform: translateY(var(--ui-active-highlight-y, var(--ui-chart-highlight-y-default)));
  opacity: var(--ui-active-highlight-o, 0);
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  background: color-mix(in oklab, var(--ui-accent, var(--ui-fallback-accent)) 16%, transparent);
  pointer-events: none;
}

.ui-chart__legend-item {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--ui-chart-legend-item-gap);
  width: 100%;
  border: 0;
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  background: transparent;
  color: inherit;
  padding: var(--ui-chart-legend-item-pad-y) var(--ui-chart-legend-item-pad-x);
  text-align: left;
}

.ui-chart__legend-item:focus-visible {
  outline: var(--ui-chart-focus-outline-width) solid
    var(--ui-accent, var(--ui-fallback-accent));
  outline-offset: var(--ui-chart-outline-offset);
}

.ui-chart__legend-value {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  font-variant-numeric: tabular-nums;
}

.ui-chart--line .ui-chart__plot-wrap {
  background: color-mix(
    in oklab,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 96%,
    transparent
  );
}

.ui-chart--disabled,
.ui-chart[data-disabled="true"] {
  opacity: 0.64;
}

.ui-chart--empty,
.ui-chart[data-empty="true"] {
  opacity: 0.82;
}

.ui-chart--custom-class,
.ui-chart[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
