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

  font-size: 14px;
  line-height: 1.2;
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
  font-size: 12px;
  line-height: 1.3;
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
