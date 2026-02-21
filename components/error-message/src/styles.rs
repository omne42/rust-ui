pub const CSS: &str = r#"
.ui-error-message {
  margin: 0;
  min-width: 0;
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  color: color-mix(
    in oklab,
    var(--ui-danger, var(--ui-fallback-danger)) 74%,
    var(--ui-fg, var(--ui-fallback-fg)) 26%
  );
  transition:
    color var(--ui-error-message-transition-ms, var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration)))
      var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing)),
    opacity var(--ui-error-message-transition-ms, var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration)))
      var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing));
}

.ui-error-message--tone-auto,
.ui-error-message--tone-negative,
.ui-error-message[data-tone="auto"],
.ui-error-message[data-tone="negative"] {
  color: color-mix(
    in oklab,
    var(--ui-danger, var(--ui-fallback-danger)) 74%,
    var(--ui-fg, var(--ui-fallback-fg)) 26%
  );
}

.ui-error-message--tone-neutral,
.ui-error-message[data-tone="neutral"] {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-error-message--disabled,
.ui-error-message[data-disabled="true"] {
  opacity: 0.68;
}

.ui-error-message--truncate,
.ui-error-message[data-truncate="true"] {
  display: block;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
}

.ui-error-message--custom-class,
.ui-error-message[data-custom-class="true"] {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(in oklab, var(--ui-accent, var(--ui-fallback-accent)) 24%, transparent);
  outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}
"#;
