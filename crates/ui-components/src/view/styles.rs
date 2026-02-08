pub const CSS: &str = r#"
.ui-view {
  display: block;
  min-width: 0;
  color: var(--ui-fg);
  background: transparent;
  border: 0 solid transparent;
}

.ui-view--element-div,
.ui-view[data-element="div"] {
  display: block;
}

.ui-view--element-span,
.ui-view[data-element="span"] {
  display: inline-block;
}

.ui-view--element-section,
.ui-view[data-element="section"] {
  display: block;
}

.ui-view--background-default,
.ui-view[data-background="default"] {
  background: transparent;
}

.ui-view--background-subtle,
.ui-view[data-background="subtle"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 90%, var(--ui-bg) 10%);
}

.ui-view--background-accent,
.ui-view[data-background="accent"] {
  background: color-mix(in oklab, var(--ui-accent-soft) 46%, var(--ui-bg-muted) 54%);
}

.ui-view--border-none,
.ui-view[data-border="none"] {
  border-width: 0;
}

.ui-view--border-subtle,
.ui-view[data-border="subtle"] {
  border-width: 1px;
  border-color: var(--ui-border);
}

.ui-view--border-strong,
.ui-view[data-border="strong"] {
  border-width: 2px;
  border-color: color-mix(in oklab, var(--ui-accent) 42%, var(--ui-border) 58%);
}

.ui-view--padding-none,
.ui-view[data-padding="none"] {
  padding: 0;
}

.ui-view--padding-sm,
.ui-view[data-padding="sm"] {
  padding: var(--ui-space-sm);
}

.ui-view--padding-md,
.ui-view[data-padding="md"] {
  padding: var(--ui-space-md);
}

.ui-view--padding-lg,
.ui-view[data-padding="lg"] {
  padding: var(--ui-space-lg);
}

.ui-view--radius-none,
.ui-view[data-radius="none"] {
  border-radius: 0;
}

.ui-view--radius-sm,
.ui-view[data-radius="sm"] {
  border-radius: var(--ui-radius-sm);
}

.ui-view--radius-md,
.ui-view[data-radius="md"] {
  border-radius: var(--ui-radius-md);
}

.ui-view--radius-lg,
.ui-view[data-radius="lg"] {
  border-radius: var(--ui-radius-lg);
}

.ui-view--shadow-none,
.ui-view[data-shadow="none"] {
  box-shadow: none;
}

.ui-view--shadow-sm,
.ui-view[data-shadow="sm"] {
  box-shadow: var(--ui-shadow-xs);
}

.ui-view--shadow-md,
.ui-view[data-shadow="md"] {
  box-shadow: var(--ui-shadow-sm);
}

.ui-view--fluid,
.ui-view[data-fluid="true"] {
  width: 100%;
}

.ui-view--custom-class,
.ui-view[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
}
"#;
