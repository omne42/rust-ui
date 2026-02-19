pub const CSS: &str = r#"
.ui-text {
  --ui-text-font-size: var(--ui-font-size-150);
  --ui-text-line-height: var(--ui-line-height-150);

  display: block;
  min-width: 0;
  color: var(--ui-fg);
  font-size: var(--ui-text-font-size);
  line-height: var(--ui-text-line-height);
}

.ui-text--tone-default,
.ui-text[data-tone="default"] {
  color: var(--ui-fg);
}

.ui-text--tone-subtle,
.ui-text[data-tone="subtle"] {
  color: var(--ui-fg-muted);
}

.ui-text--tone-strong,
.ui-text[data-tone="strong"] {
  color: color-mix(in oklab, var(--ui-fg) 86%, var(--ui-accent) 14%);
}

.ui-text--align-start,
.ui-text[data-align="start"] {
  text-align: start;
}

.ui-text--align-center,
.ui-text[data-align="center"] {
  text-align: center;
}

.ui-text--align-end,
.ui-text[data-align="end"] {
  text-align: end;
}

.ui-text--align-justify,
.ui-text[data-align="justify"] {
  text-align: justify;
}

.ui-text--weight-regular,
.ui-text[data-weight="regular"] {
  font-weight: 400;
}

.ui-text--weight-medium,
.ui-text[data-weight="medium"] {
  font-weight: 500;
}

.ui-text--weight-semibold,
.ui-text[data-weight="semibold"] {
  font-weight: 600;
}

.ui-text--weight-bold,
.ui-text[data-weight="bold"] {
  font-weight: 700;
}

.ui-text--disabled,
.ui-text[data-disabled="true"] {
  opacity: 0.7;
}

.ui-text--truncate,
.ui-text[data-truncate="true"] {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.ui-text--custom-class,
.ui-text[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 18%, transparent);
}
"#;
