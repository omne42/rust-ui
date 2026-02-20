pub const CSS: &str = r#"
.ui-illustrated-message {
  display: flex;
  gap: var(--ui-space-md);
  padding: var(--ui-space-lg);
  border-radius: var(--ui-radius-lg);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg-muted);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-sm);

  --ui-im-opacity: 1;
  --ui-im-y: 0px;
  opacity: var(--ui-im-opacity);
  transform: translateY(var(--ui-im-y));
  will-change: transform, opacity;
}

.ui-illustrated-message--orientation-vertical {
  flex-direction: column;
  align-items: flex-start;
}

.ui-illustrated-message--orientation-horizontal {
  flex-direction: row;
  align-items: center;
}

.ui-illustrated-message__illustration {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 52px;
  height: 52px;
  border-radius: 16px;
  background: var(--ui-accent-soft);
  color: var(--ui-accent);
  flex: 0 0 auto;
}

.ui-illustrated-message__content {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
  min-width: 0;
}

.ui-illustrated-message__title {
  margin: 0;
  font-size: var(--ui-heading-h6-font-size, 14px);
  line-height: var(--ui-heading-h6-line-height, 20px);
  font-weight: 700;
}

.ui-illustrated-message__description {
  margin: 0;
  font-size: var(--ui-font-size-150, 14px);
  line-height: var(--ui-line-height-150, 20px);
  color: var(--ui-fg-muted);
}

.ui-illustrated-message__actions {
  display: flex;
  gap: var(--ui-space-sm);
  flex-wrap: wrap;
  margin-top: var(--ui-space-sm);
}
"#;
