pub const CSS: &str = r#"
.ui-checkbox-group {
  display: flex;
  flex-direction: column;
  gap: var(--ui-checkbox-group-gap, var(--ui-fallback-checkbox-group-gap));

  border: none;
  padding: 0;
  margin: 0;
  min-inline-size: 0;
}

.ui-checkbox-group__label {
  padding: 0;
  margin: 0;

  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));
  font-weight: 500;
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-checkbox-group--required .ui-checkbox-group__label::after {
  content: "*";
  margin-left: var(
    --ui-checkbox-group-required-marker-gap,
    var(--ui-fallback-checkbox-group-required-marker-gap)
  );
  color: var(--ui-danger, var(--ui-fallback-danger));
}

.ui-checkbox-group__list {
  display: flex;
  flex-direction: column;
  gap: var(--ui-checkbox-group-gap, var(--ui-fallback-checkbox-group-gap));
}

.ui-checkbox-group__description,
.ui-checkbox-group__error {
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  transition-duration: var(
    --ui-checkbox-group-motion-duration,
    var(--ui-fallback-checkbox-group-motion-duration)
  );
  transition-timing-function: var(
    --ui-checkbox-group-motion-easing,
    var(--ui-fallback-checkbox-group-motion-easing)
  );
  transition-property: color;
}

.ui-checkbox-group__description {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-checkbox-group--invalid .ui-checkbox-group__description {
  color: var(--ui-danger, var(--ui-fallback-danger));
}

.ui-checkbox-group__error {
  color: var(--ui-danger, var(--ui-fallback-danger));
}

.ui-checkbox-group:disabled {
  opacity: var(
    --ui-checkbox-group-disabled-opacity,
    var(--ui-fallback-checkbox-group-disabled-opacity)
  );
}
"#;
