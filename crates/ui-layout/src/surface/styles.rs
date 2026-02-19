pub const CSS: &str = r#"
.ui-surface {
  display: block;
  min-width: 0;
  border-radius: var(--ui-radius-md);
  background: var(--ui-bg);
  color: var(--ui-fg);
}

.ui-surface--tone-default,
.ui-surface[data-tone="default"] {
  background: var(--ui-bg);
}

.ui-surface--tone-subtle,
.ui-surface[data-tone="subtle"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 58%, var(--ui-bg) 42%);
}

.ui-surface--tone-strong,
.ui-surface[data-tone="strong"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 74%, var(--ui-bg) 26%);
}

.ui-surface--elevation-flat,
.ui-surface[data-elevation="flat"] {
  box-shadow: none;
}

.ui-surface--elevation-raised,
.ui-surface[data-elevation="raised"] {
  box-shadow: var(--ui-shadow-sm);
}

.ui-surface--elevation-floating,
.ui-surface[data-elevation="floating"] {
  box-shadow: var(--ui-shadow-md);
}

.ui-surface--bordered,
.ui-surface[data-bordered="true"] {
  border: 1px solid color-mix(in oklab, var(--ui-border) 78%, transparent);
}

.ui-surface--padded,
.ui-surface[data-padded="true"] {
  padding: var(--ui-space-md);
}

.ui-surface[data-state="framed"] {
  border-color: color-mix(in oklab, var(--ui-border) 60%, var(--ui-accent) 40%);
}

.ui-surface--custom-class,
.ui-surface[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 26%, transparent);
  outline-offset: 2px;
}
"#;
