pub const CSS: &str = r#"
.ui-tag-group {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-sm);
}

.ui-tag-group__label {
  font-size: 13px;
  font-weight: 600;
  color: var(--ui-fg);
}

.ui-tag-group__list {
  display: flex;
  flex-wrap: wrap;
  gap: var(--ui-space-xs);
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
  font-size: 12px;
  color: var(--ui-fg-muted);
}

.ui-tag-group__error {
  font-size: 12px;
  color: var(--ui-danger-fg);
}

.ui-tag-group[data-invalid="true"] .ui-tag-group__list {
  outline: 1px solid var(--ui-danger);
  outline-offset: 4px;
  border-radius: var(--ui-radius-sm);
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
