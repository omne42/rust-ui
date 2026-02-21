pub const CSS: &str = r#"
.ui-help-text {
  display: flex;
  align-items: flex-start;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  min-width: 0;
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-help-text--tone-auto,
.ui-help-text--tone-neutral,
.ui-help-text[data-tone="auto"],
.ui-help-text[data-tone="neutral"] {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-help-text--tone-negative,
.ui-help-text[data-tone="negative"] {
  color: color-mix(
    in oklab,
    var(--ui-danger, var(--ui-fallback-danger)) 74%,
    var(--ui-fg, var(--ui-fallback-fg)) 26%
  );
}

.ui-help-text--invalid,
.ui-help-text[data-invalid="true"] {
  color: color-mix(
    in oklab,
    var(--ui-danger, var(--ui-fallback-danger)) 74%,
    var(--ui-fg, var(--ui-fallback-fg)) 26%
  );
}

.ui-help-text--disabled,
.ui-help-text[data-disabled="true"] {
  opacity: 0.68;
}

.ui-help-text__icon {
  margin-top: var(--ui-border-width, var(--ui-fallback-border-width));
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
}

.ui-help-text__text {
  margin: 0;
  min-width: 0;
}

.ui-help-text--custom-class,
.ui-help-text[data-custom-class="true"] {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(in oklab, var(--ui-accent, var(--ui-fallback-accent)) 24%, transparent);
  outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}
"#;
