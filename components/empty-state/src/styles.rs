pub const CSS: &str = r#"
.ui-empty-state {
  --ui-empty-state-enter: 1;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  padding: var(--ui-space-xl, var(--ui-fallback-space-xl));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  background: color-mix(
    in oklab,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 86%,
    var(--ui-bg, var(--ui-fallback-bg)) 14%
  );
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid transparent;
  color: var(--ui-fg, var(--ui-fallback-fg));
  opacity: var(--ui-empty-state-enter);
  transform: translateY(
    calc(
      (1 - var(--ui-empty-state-enter))
        * var(--ui-space-sm, var(--ui-fallback-space-sm))
    )
  );
}

.ui-empty-state--tone-default,
.ui-empty-state[data-tone="default"] {
  background: color-mix(
    in oklab,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 88%,
    var(--ui-bg, var(--ui-fallback-bg)) 12%
  );
}

.ui-empty-state--tone-muted,
.ui-empty-state[data-tone="muted"] {
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-empty-state--tone-accent,
.ui-empty-state[data-tone="accent"] {
  background: color-mix(
    in oklab,
    var(--ui-accent-soft, var(--ui-fallback-accent-soft)) 34%,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 66%
  );
  border-color: color-mix(
    in oklab,
    var(--ui-accent, var(--ui-fallback-accent)) 36%,
    transparent
  );
}

.ui-empty-state--align-start,
.ui-empty-state[data-align="start"] {
  align-items: flex-start;
  text-align: left;
}

.ui-empty-state--align-center,
.ui-empty-state[data-align="center"] {
  align-items: center;
  text-align: center;
}

.ui-empty-state--compact,
.ui-empty-state[data-compact="true"] {
  padding: var(--ui-space-md, var(--ui-fallback-space-md))
    var(--ui-space-lg, var(--ui-fallback-space-lg));
}

.ui-empty-state--bordered,
.ui-empty-state[data-bordered="true"] {
  border: var(--ui-border-width, var(--ui-fallback-border-width)) dashed
    color-mix(
      in oklab,
      var(--ui-border, var(--ui-fallback-border)) 78%,
      var(--ui-accent, var(--ui-fallback-accent)) 22%
    );
}

.ui-empty-state__icon {
  width: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100))
      + var(--ui-space-sm, var(--ui-fallback-space-sm))
  );
  height: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100))
      + var(--ui-space-sm, var(--ui-fallback-space-sm))
  );
  border-radius: calc(var(--ui-radius-lg, var(--ui-fallback-radius-lg)) * 2);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background: color-mix(
    in oklab,
    var(--ui-accent-soft, var(--ui-fallback-accent-soft)) 56%,
    transparent
  );
  color: var(--ui-accent, var(--ui-fallback-accent));
}

.ui-empty-state__title {
  margin: 0;
  font-size: var(--ui-heading-h5-font-size, var(--ui-fallback-heading-h5-font-size));
  line-height: var(
    --ui-heading-h5-line-height,
    var(--ui-fallback-heading-h5-line-height)
  );
  font-weight: 600;
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-empty-state__description {
  margin: 0;
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-empty-state__actions {
  margin-top: var(--ui-space-xs, var(--ui-fallback-space-xs));
  display: inline-flex;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  flex-wrap: wrap;
}

.ui-empty-state--custom-class,
.ui-empty-state[data-custom-class="true"] {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(
      in oklab,
      var(--ui-accent, var(--ui-fallback-accent)) 26%,
      transparent
    );
  outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-empty-state[data-motion-source="custom"],
.ui-empty-state[data-custom-motion="true"] {
  --ui-empty-state-motion-custom: 1;
}

@media (prefers-reduced-motion: reduce) {
  .ui-empty-state {
    transform: none;
  }
}
"#;
