pub const CSS: &str = r#"
.ui-checkbox-group {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);

  border: none;
  padding: 0;
  margin: 0;
  min-inline-size: 0;
}

.ui-checkbox-group__label {
  padding: 0;
  margin: 0;

  font-size: var(--ui-font-size-150, 14px);
  line-height: var(--ui-line-height-150, 20px);
  font-weight: 500;
  color: var(--ui-fg);
}

.ui-checkbox-group--required .ui-checkbox-group__label::after {
  content: "*";
  margin-left: 4px;
  color: var(--ui-danger);
}

.ui-checkbox-group__list {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-checkbox-group__description,
.ui-checkbox-group__error {
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
}

.ui-checkbox-group__description {
  color: var(--ui-fg-muted);
}

.ui-checkbox-group--invalid .ui-checkbox-group__description {
  color: var(--ui-danger);
}

.ui-checkbox-group__error {
  color: var(--ui-danger);
}

.ui-checkbox-group:disabled {
  opacity: 0.6;
}
"#;
