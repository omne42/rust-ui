pub const CSS: &str = r#"
.ui-pagination {
  display: inline-flex;
  align-items: center;
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
"#;
