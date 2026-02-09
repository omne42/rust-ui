pub const CSS: &str = r#"
.ui-chart {
  display: grid;
  gap: 0.75rem;
  width: 100%;
}

.ui-chart__plot-wrap {
  border: 1px solid var(--ui-border-subtle, color-mix(in oklab, currentColor 22%, transparent));
  border-radius: var(--ui-radius-md, 0.75rem);
  background: var(--ui-bg-surface, color-mix(in oklab, currentColor 4%, transparent));
  padding: 0.5rem;
}

.ui-chart__plot {
  width: 100%;
  height: 14rem;
}

.ui-chart__grid-line {
  stroke: var(--ui-border-subtle, color-mix(in oklab, currentColor 26%, transparent));
  stroke-width: 0.4;
}

.ui-chart__line {
  fill: none;
  stroke: var(--ui-accent-solid, color-mix(in oklab, currentColor 62%, transparent));
  stroke-width: 1.4;
  stroke-linecap: round;
  stroke-linejoin: round;
}

.ui-chart__bar {
  fill: var(--ui-accent-solid, color-mix(in oklab, currentColor 62%, transparent));
  transition: fill 160ms ease;
}

.ui-chart__dot {
  fill: var(--ui-accent-solid, color-mix(in oklab, currentColor 62%, transparent));
  stroke: var(--ui-bg-canvas, white);
  stroke-width: 0.8;
  transition: fill 160ms ease;
}

.ui-chart__bar[data-active="true"],
.ui-chart__dot[data-active="true"] {
  fill: var(--ui-accent-emphasis, color-mix(in oklab, currentColor 78%, transparent));
}

.ui-chart__legend {
  position: relative;
  display: grid;
  gap: 0.25rem;
}

.ui-chart__legend-highlight {
  position: absolute;
  inset-inline: 0;
  height: var(--ui-active-highlight-h, 0px);
  transform: translateY(var(--ui-active-highlight-y, 0px));
  opacity: var(--ui-active-highlight-o, 0);
  border-radius: var(--ui-radius-sm, 0.5rem);
  background: color-mix(in oklab, var(--ui-accent-solid, currentColor) 16%, transparent);
  pointer-events: none;
}

.ui-chart__legend-item {
  position: relative;
  z-index: 1;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.5rem;
  width: 100%;
  border: 0;
  border-radius: var(--ui-radius-sm, 0.5rem);
  background: transparent;
  color: inherit;
  padding: 0.4rem 0.5rem;
  text-align: left;
}

.ui-chart__legend-item:focus-visible {
  outline: 2px solid var(--ui-accent-solid, color-mix(in oklab, currentColor 62%, transparent));
  outline-offset: 1px;
}

.ui-chart__legend-value {
  color: var(--ui-fg-muted, color-mix(in oklab, currentColor 72%, transparent));
  font-variant-numeric: tabular-nums;
}

.ui-chart--line .ui-chart__plot-wrap {
  background: color-mix(in oklab, var(--ui-bg-surface, white) 96%, transparent);
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
