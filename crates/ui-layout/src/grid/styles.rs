pub const CSS: &str = r#"
.ui-grid {
  display: grid;
  min-width: 0;
  gap: var(--ui-space-sm);
  grid-template-columns: repeat(2, minmax(0, 1fr));
  grid-auto-rows: minmax(0, auto);
  justify-items: stretch;
  align-items: stretch;
}

.ui-grid--inline,
.ui-grid[data-inline="true"] {
  display: inline-grid;
}

.ui-grid--dense,
.ui-grid[data-dense="true"] {
  grid-auto-flow: row dense;
}

.ui-grid--columns-1,
.ui-grid[data-columns="1"] {
  grid-template-columns: repeat(1, minmax(0, 1fr));
}

.ui-grid--columns-2,
.ui-grid[data-columns="2"] {
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.ui-grid--columns-3,
.ui-grid[data-columns="3"] {
  grid-template-columns: repeat(3, minmax(0, 1fr));
}

.ui-grid--columns-4,
.ui-grid[data-columns="4"] {
  grid-template-columns: repeat(4, minmax(0, 1fr));
}

.ui-grid--columns-auto-fit,
.ui-grid[data-columns="auto-fit"] {
  grid-template-columns: repeat(auto-fit, minmax(12rem, 1fr));
}

.ui-grid--rows-auto,
.ui-grid[data-rows="auto"] {
  grid-auto-rows: minmax(0, auto);
}

.ui-grid--rows-equal,
.ui-grid[data-rows="equal"] {
  grid-auto-rows: 1fr;
}

.ui-grid--rows-compact,
.ui-grid[data-rows="compact"] {
  grid-auto-rows: min-content;
}

.ui-grid--justify-start,
.ui-grid[data-justify="start"] {
  justify-items: start;
}

.ui-grid--justify-center,
.ui-grid[data-justify="center"] {
  justify-items: center;
}

.ui-grid--justify-end,
.ui-grid[data-justify="end"] {
  justify-items: end;
}

.ui-grid--justify-stretch,
.ui-grid[data-justify="stretch"] {
  justify-items: stretch;
}

.ui-grid--align-start,
.ui-grid[data-align="start"] {
  align-items: start;
}

.ui-grid--align-center,
.ui-grid[data-align="center"] {
  align-items: center;
}

.ui-grid--align-end,
.ui-grid[data-align="end"] {
  align-items: end;
}

.ui-grid--align-stretch,
.ui-grid[data-align="stretch"] {
  align-items: stretch;
}

.ui-grid--gap-none,
.ui-grid[data-gap="none"] {
  gap: 0;
}

.ui-grid--gap-xs,
.ui-grid[data-gap="xs"] {
  gap: var(--ui-space-xs);
}

.ui-grid--gap-sm,
.ui-grid[data-gap="sm"] {
  gap: var(--ui-space-sm);
}

.ui-grid--gap-md,
.ui-grid[data-gap="md"] {
  gap: var(--ui-space-md);
}

.ui-grid--gap-lg,
.ui-grid[data-gap="lg"] {
  gap: var(--ui-space-lg);
}

.ui-grid--custom-class,
.ui-grid[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
}
"#;
