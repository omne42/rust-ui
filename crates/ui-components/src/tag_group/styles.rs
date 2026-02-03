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
"#;
