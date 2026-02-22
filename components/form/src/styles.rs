pub const CSS: &str = r#"
.ui-form {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-md);
  gap: var(--ui-space-md, var(--ui-fallback-space-md));
}

.ui-form[data-disabled=\"true\"] {
  opacity: 0.85;
}
"#;
