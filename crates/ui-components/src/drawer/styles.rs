pub const CSS: &str = r#"
.ui-drawer {
  position: relative;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-md);
  min-height: 0;
}

.ui-drawer__header {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
  padding-right: 44px;
}

.ui-drawer__title {
  font-size: 16px;
  line-height: 1.2;
  font-weight: 700;
  margin: 0;
}

.ui-drawer__description {
  font-size: 13px;
  line-height: 1.5;
  color: var(--ui-fg-muted);
  margin: 0;
}

.ui-drawer__body {
  flex: 1 1 auto;
  min-height: 0;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-sm);
}

.ui-drawer__footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--ui-space-sm);
  flex-wrap: wrap;
}

.ui-drawer__close {
  position: absolute;
  top: 2px;
  right: 2px;
}
"#;
