pub const CSS: &str = r#"
.ui-field-error {
  margin: 0;
  min-width: 0;
  display: flex;
  align-items: flex-start;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-field-error--tone-auto,
.ui-field-error--tone-neutral,
.ui-field-error[data-tone="auto"],
.ui-field-error[data-tone="neutral"] {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-field-error--tone-negative,
.ui-field-error[data-tone="negative"] {
  color: color-mix(
    in oklab,
    var(--ui-danger, var(--ui-fallback-danger)) 74%,
    var(--ui-fg, var(--ui-fallback-fg)) 26%
  );
}

.ui-field-error[data-state="hidden"] {
  display: none;
}

.ui-field-error--disabled,
.ui-field-error[data-disabled="true"] {
  opacity: var(--ui-opacity-disabled, var(--ui-fallback-opacity-disabled));
}

.ui-field-error__icon {
  margin-top: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
}

.ui-field-error__text {
  margin: 0;
  min-width: 0;
}

.ui-field-error--custom-class,
.ui-field-error[data-custom-class="true"] {
  outline: var(--ui-border-width-thin, var(--ui-fallback-border-width-thin)) solid color-mix(
    in oklab,
    var(--ui-accent, var(--ui-fallback-accent)) 24%,
    var(--ui-transparent, var(--ui-fallback-transparent))
  );
  outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}
"#;
