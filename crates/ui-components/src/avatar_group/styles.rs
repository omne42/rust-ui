pub const CSS: &str = r#"
.ui-avatar-group {
  display: inline-flex;
  align-items: center;
}

.ui-avatar-group__item {
  border: 2px solid var(--ui-bg);
  border-radius: 9999px;
  box-shadow: var(--ui-shadow-sm);
}

.ui-avatar-group__item:not(:first-child) {
  margin-left: -10px;
}

.ui-avatar-group__overflow {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 9999px;
  border: 2px solid var(--ui-bg);
  background: var(--ui-bg-muted);
  color: var(--ui-fg);
  font-weight: 650;
  box-shadow: var(--ui-shadow-sm);
}

.ui-avatar-group__overflow:not(:first-child) {
  margin-left: -10px;
}
"#;
