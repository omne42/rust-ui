pub const CSS: &str = r#"
.ui-alert-dialog {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-md);
  width: min(100%, 480px);
}

.ui-alert-dialog__header {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-alert-dialog__title {
  font-size: 16px;
  line-height: 1.2;
  font-weight: 700;
  margin: 0;
}

.ui-alert-dialog__description {
  font-size: 13px;
  line-height: 1.5;
  color: var(--ui-fg-muted);
  margin: 0;
}

.ui-alert-dialog__footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--ui-space-sm);
  flex-wrap: wrap;
}

.ui-alert-dialog--variant-destructive .ui-alert-dialog__title {
  color: var(--ui-fg);
}
"#;
