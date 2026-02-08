pub const CSS: &str = r#"
.ui-flex {
  display: flex;
  min-width: 0;
  gap: var(--ui-space-sm);
  flex-direction: row;
  flex-wrap: nowrap;
  justify-content: flex-start;
  align-items: stretch;
}

.ui-flex--inline,
.ui-flex[data-inline="true"] {
  display: inline-flex;
}

.ui-flex--direction-row,
.ui-flex[data-direction="row"] {
  flex-direction: row;
}

.ui-flex--direction-column,
.ui-flex[data-direction="column"] {
  flex-direction: column;
}

.ui-flex--direction-row-reverse,
.ui-flex[data-direction="row-reverse"] {
  flex-direction: row-reverse;
}

.ui-flex--direction-column-reverse,
.ui-flex[data-direction="column-reverse"] {
  flex-direction: column-reverse;
}

.ui-flex--wrap-nowrap,
.ui-flex[data-wrap="nowrap"] {
  flex-wrap: nowrap;
}

.ui-flex--wrap-wrap,
.ui-flex[data-wrap="wrap"] {
  flex-wrap: wrap;
}

.ui-flex--justify-start,
.ui-flex[data-justify="start"] {
  justify-content: flex-start;
}

.ui-flex--justify-center,
.ui-flex[data-justify="center"] {
  justify-content: center;
}

.ui-flex--justify-end,
.ui-flex[data-justify="end"] {
  justify-content: flex-end;
}

.ui-flex--justify-space-between,
.ui-flex[data-justify="space-between"] {
  justify-content: space-between;
}

.ui-flex--justify-space-around,
.ui-flex[data-justify="space-around"] {
  justify-content: space-around;
}

.ui-flex--justify-space-evenly,
.ui-flex[data-justify="space-evenly"] {
  justify-content: space-evenly;
}

.ui-flex--align-start,
.ui-flex[data-align="start"] {
  align-items: flex-start;
}

.ui-flex--align-center,
.ui-flex[data-align="center"] {
  align-items: center;
}

.ui-flex--align-end,
.ui-flex[data-align="end"] {
  align-items: flex-end;
}

.ui-flex--align-baseline,
.ui-flex[data-align="baseline"] {
  align-items: baseline;
}

.ui-flex--align-stretch,
.ui-flex[data-align="stretch"] {
  align-items: stretch;
}

.ui-flex--gap-none,
.ui-flex[data-gap="none"] {
  gap: 0;
}

.ui-flex--gap-xs,
.ui-flex[data-gap="xs"] {
  gap: var(--ui-space-xs);
}

.ui-flex--gap-sm,
.ui-flex[data-gap="sm"] {
  gap: var(--ui-space-sm);
}

.ui-flex--gap-md,
.ui-flex[data-gap="md"] {
  gap: var(--ui-space-md);
}

.ui-flex--gap-lg,
.ui-flex[data-gap="lg"] {
  gap: var(--ui-space-lg);
}

.ui-flex--custom-class,
.ui-flex[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
}
"#;
