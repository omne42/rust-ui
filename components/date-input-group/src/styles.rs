pub const CSS: &str = r#"
.ui-date-input-group {
  display: inline-flex;
  align-items: stretch;
  width: fit-content;
  min-height: var(--ui-component-height-100, var(--ui-fallback-component-height-100));
  transform: scale(
    var(
      --ui-date-input-group-scale,
      var(--ui-alert-scale, var(--ui-fallback-alert-scale))
    )
  );
  transform-origin: center;
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  overflow: hidden;
}

.ui-date-input-group--variant-primary,
.ui-date-input-group[data-variant="primary"] {
  border-color: var(--ui-border, var(--ui-fallback-border));
  background: var(--ui-bg, var(--ui-fallback-bg));
}

.ui-date-input-group--variant-secondary,
.ui-date-input-group[data-variant="secondary"] {
  border-color: color-mix(
    in oklab,
    var(--ui-border, var(--ui-fallback-border)) 64%,
    var(--ui-accent, var(--ui-fallback-accent)) 36%
  );
  background: color-mix(
    in oklab,
    var(--ui-bg, var(--ui-fallback-bg)) 88%,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 12%
  );
}

.ui-date-input-group--full-width,
.ui-date-input-group[data-width="full"] {
  width: 100%;
}

.ui-date-input-group--disabled,
.ui-date-input-group[data-disabled="true"] {
  opacity: 0.62;
}

.ui-date-input-group--invalid,
.ui-date-input-group[data-invalid="true"] {
  border-color: color-mix(
    in oklab,
    var(--ui-danger, var(--ui-fallback-danger)) 74%,
    var(--ui-border, var(--ui-fallback-border)) 26%
  );
}

.ui-date-input-group--segmented .ui-date-input-group__segment,
.ui-date-input-group[data-segmented="true"] .ui-date-input-group__segment {
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-date-input-group__prefix,
.ui-date-input-group__suffix {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: var(--ui-component-height-100, var(--ui-fallback-component-height-100));
  padding-inline: var(--ui-space-sm, var(--ui-fallback-space-sm));
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  background: color-mix(
    in oklab,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)),
    var(--ui-bg, var(--ui-fallback-bg)) 28%
  );
}

.ui-date-input-group__prefix {
  border-inline-end: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(in oklab, var(--ui-border, var(--ui-fallback-border)), transparent 28%);
}

.ui-date-input-group__suffix {
  border-inline-start: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(in oklab, var(--ui-border, var(--ui-fallback-border)), transparent 28%);
}

.ui-date-input-group__input {
  display: flex;
  align-items: stretch;
  flex: 1 1 auto;
  min-width: 0;
}

.ui-date-input-group__segment {
  display: flex;
  align-items: stretch;
  flex: 1 1 auto;
  min-width: 0;
}

.ui-date-input-group__segment > * {
  flex: 1 1 auto;
  min-width: 0;
}

.ui-date-input-group__segment > .ui-date-field,
.ui-date-input-group__segment > .ui-time-field {
  width: 100%;
}

.ui-date-input-group__segment > .ui-date-field .ui-date-field__control,
.ui-date-input-group__segment > .ui-time-field .ui-time-field__control {
  width: 100%;
  border: 0;
  border-radius: 0;
  background: transparent;
}

.ui-date-input-group--custom-class,
.ui-date-input-group[data-custom-class="true"] {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(in oklab, var(--ui-accent, var(--ui-fallback-accent)) 24%, transparent);
  outline-offset: calc(var(--ui-space-2xs, var(--ui-fallback-space-2xs)) / 2);
}
"#;
