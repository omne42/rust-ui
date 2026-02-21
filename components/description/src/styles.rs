pub const CSS: &str = r#"
.ui-description {
  margin: 0;
  min-width: 0;
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-description--tone-default,
.ui-description[data-tone="default"] {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-description--tone-muted,
.ui-description[data-tone="muted"] {
  color: color-mix(
    in oklab,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 76%,
    var(--ui-bg, var(--ui-fallback-bg)) 24%
  );
}

.ui-description--tone-negative,
.ui-description[data-tone="negative"] {
  color: color-mix(
    in oklab,
    var(--ui-danger, var(--ui-fallback-danger)) 74%,
    var(--ui-fg, var(--ui-fallback-fg)) 26%
  );
}

.ui-description--disabled,
.ui-description[data-disabled="true"] {
  opacity: var(--ui-checkbox-disabled-opacity, var(--ui-fallback-checkbox-disabled-opacity));
}

.ui-description--truncate,
.ui-description[data-truncate="true"] {
  display: block;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.ui-description--custom-class,
.ui-description[data-custom-class="true"] {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid color-mix(
    in oklab,
    var(--ui-accent, var(--ui-fallback-accent)) 24%,
    transparent
  );
  outline-offset: var(--ui-button-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset));
}
"#;
