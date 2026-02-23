pub const CSS: &str = r#"
.ui-tag-group {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
}

.ui-tag-group__label {
  font-size: var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size));
  line-height: var(--ui-button-size-s-line-height, var(--ui-fallback-button-size-s-line-height));
  font-weight: 600;
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-tag-group__list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  padding: 0;
  margin: 0;
}

.ui-tag-group__item {
  list-style-type: none;
  display: inline-flex;
}

.ui-tag-group__item[data-disabled="true"] {
  opacity: 0.72;
}

.ui-tag-group__item[data-disabled-source="group"] {
  --ui-tag-group-item-disabled-source: 1;
}

.ui-tag-group__item[data-disabled-source="item"] {
  --ui-tag-group-item-disabled-source: 2;
}

.ui-tag-group__item[data-disabled-source="group-and-item"] {
  --ui-tag-group-item-disabled-source: 3;
}

.ui-tag-group__item[data-removable-source="unsupported"] {
  --ui-tag-group-item-removable-source: 0;
}

.ui-tag-group__item[data-removable-source="disabled"] {
  --ui-tag-group-item-removable-source: 1;
}

.ui-tag-group__item[data-removable-source="removable"] {
  --ui-tag-group-item-removable-source: 2;
}

.ui-tag-group__description {
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-tag-group__error {
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  color: var(--ui-danger-fg, var(--ui-fallback-danger-fg));
}

.ui-tag-group[data-invalid="true"] .ui-tag-group__list {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-danger, var(--ui-fallback-danger));
  outline-offset: calc(var(--ui-space-2xs, var(--ui-fallback-space-2xs)) * 2);
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
}

.ui-tag-group[data-class-source="custom"] {
  --ui-tag-group-class-source: 1;
}

.ui-tag-group[data-id-base-source="custom"] {
  --ui-tag-group-id-source: 1;
}

.ui-tag-group[data-aria-label-source="custom"] {
  --ui-tag-group-aria-label-source: 1;
}

.ui-tag-group[data-lang-source="provided"] {
  --ui-tag-group-lang-source: 1;
}
"#;
