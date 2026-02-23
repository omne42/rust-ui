pub const CSS: &str = r#"
.ui-pagination {
  display: inline-flex;
  align-items: center;
  --ui-pagination-motion-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration, 180ms)
  );
  --ui-pagination-motion-easing: var(
    --ui-text-field-motion-easing,
    var(--ui-fallback-text-field-motion-easing, cubic-bezier(0.2, 0, 0, 1))
  );
}

.ui-pagination__list {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-space-xs);
  padding: 0;
  margin: 0;
}

.ui-pagination__item {
  list-style-type: none;
  display: inline-flex;
  align-items: center;
}

.ui-pagination__dots {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: var(--ui-radius-md);
  color: var(--ui-fg-muted);
  user-select: none;
}

.ui-pagination__item[aria-current="page"] .ui-button {
  filter: brightness(0.93);
}

.ui-pagination .ui-button {
  transition-duration: var(--ui-pagination-motion-duration);
  transition-timing-function: var(--ui-pagination-motion-easing);
}
"#;
