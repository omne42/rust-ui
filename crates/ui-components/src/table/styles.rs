pub const CSS: &str = r#"
.ui-table {
  display: block;
  width: 100%;
  max-width: 100%;
  overflow-x: auto;
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-md);
  background: var(--ui-bg);
}

.ui-table--variant-default,
.ui-table[data-variant="default"] {
  background: var(--ui-bg);
}

.ui-table--variant-quiet,
.ui-table[data-variant="quiet"] {
  border: 1px solid color-mix(in oklab, var(--ui-border) 46%, transparent);
  background: color-mix(in oklab, var(--ui-bg-muted) 68%, var(--ui-bg) 32%);
}

.ui-table--variant-outline,
.ui-table[data-variant="outline"] {
  border: 1px solid color-mix(in oklab, var(--ui-fg) 28%, var(--ui-border) 72%);
}

.ui-table--density-comfortable .ui-table__head-cell,
.ui-table--density-comfortable .ui-table__cell,
.ui-table[data-density="comfortable"] .ui-table__head-cell,
.ui-table[data-density="comfortable"] .ui-table__cell {
  padding: var(--ui-space-sm) var(--ui-space-md);
}

.ui-table--density-compact .ui-table__head-cell,
.ui-table--density-compact .ui-table__cell,
.ui-table[data-density="compact"] .ui-table__head-cell,
.ui-table[data-density="compact"] .ui-table__cell {
  padding: var(--ui-space-2xs) var(--ui-space-sm);
}

.ui-table--layout-auto .ui-table__table,
.ui-table[data-layout="auto"] .ui-table__table {
  table-layout: auto;
}

.ui-table--layout-fixed .ui-table__table,
.ui-table[data-layout="fixed"] .ui-table__table {
  table-layout: fixed;
}

.ui-table--striped .ui-table__body .ui-table__row:nth-child(even),
.ui-table[data-striped="true"] .ui-table__body .ui-table__row:nth-child(even) {
  background: color-mix(in oklab, var(--ui-bg-muted) 72%, var(--ui-bg) 28%);
}

.ui-table--sticky-header .ui-table__head-cell,
.ui-table[data-sticky-header="true"] .ui-table__head-cell {
  position: sticky;
  top: 0;
  z-index: 1;
  background: inherit;
}

.ui-table--with-caption .ui-table__caption,
.ui-table[data-has-caption="true"] .ui-table__caption {
  display: table-caption;
}

.ui-table--empty,
.ui-table[data-state="empty"] {
  color: var(--ui-fg-muted);
}

.ui-table--custom-class,
.ui-table[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 28%, transparent);
}

.ui-table__table {
  width: 100%;
  border-collapse: separate;
  border-spacing: 0;
  min-width: 28rem;
}

.ui-table__caption {
  caption-side: top;
  text-align: start;
  padding: var(--ui-space-sm) var(--ui-space-md);
  font-size: var(--ui-button-size-s-font-size, 13px);
  line-height: var(--ui-button-size-s-line-height, 18px);
  color: var(--ui-fg-muted);
}

.ui-table__head-cell,
.ui-table__cell {
  border-bottom: 1px solid color-mix(in oklab, var(--ui-border) 74%, transparent);
  color: var(--ui-fg);
  white-space: nowrap;
}

.ui-table__head-cell {
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  font-weight: 600;
  letter-spacing: 0.01em;
  color: var(--ui-fg-muted);
}

.ui-table__row:last-child .ui-table__cell {
  border-bottom: none;
}

.ui-table__row--empty .ui-table__cell {
  color: var(--ui-fg-muted);
}

.ui-table__cell--align-start,
.ui-table__head-cell[data-align="start"],
.ui-table__cell[data-align="start"] {
  text-align: start;
}

.ui-table__cell--align-center,
.ui-table__head-cell[data-align="center"],
.ui-table__cell[data-align="center"] {
  text-align: center;
}

.ui-table__cell--align-end,
.ui-table__head-cell[data-align="end"],
.ui-table__cell[data-align="end"] {
  text-align: end;
}
"#;
