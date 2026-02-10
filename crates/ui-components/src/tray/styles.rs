pub const CSS: &str = r#"
.ui-tray {
  position: relative;
  inline-size: 100%;
  max-inline-size: 100%;
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-sm);
  min-block-size: 0;
}

.ui-tray[data-motion-source="custom"],
.ui-tray[data-custom-motion="true"] {
  --ui-tray-custom-motion: 1;
}

.ui-tray--fixed-height,
.ui-tray[data-size="fixed"],
.ui-tray[data-fixed-height="true"] {
  block-size: min(28rem, 78vh);
}

.ui-tray--auto-height,
.ui-tray[data-size="auto"] {
  max-block-size: min(80vh, 36rem);
}

.ui-tray--with-description,
.ui-tray[data-state="with-description"],
.ui-tray[data-description="present"] {
  gap: var(--ui-space-xs);
}

.ui-tray--title-only,
.ui-tray[data-state="title-only"],
.ui-tray[data-description="absent"] {
  gap: var(--ui-space-sm);
}

.ui-tray--custom-class,
.ui-tray[data-custom-class="true"] {
  border-radius: inherit;
}

.ui-tray__header {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-2xs);
  padding-right: 0;
}

.ui-tray--close-shown .ui-tray__header,
.ui-tray[data-close-button="shown"] .ui-tray__header {
  padding-right: 44px;
}

.ui-tray__title {
  font-size: 1rem;
  line-height: 1.2;
  font-weight: 700;
  margin: 0;
}

.ui-tray__description {
  font-size: 0.8125rem;
  line-height: 1.45;
  color: var(--ui-fg-muted);
  margin: 0;
}

.ui-tray__body {
  flex: 1 1 auto;
  min-height: 0;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-sm);
  overflow: auto;
}

.ui-tray__footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--ui-space-sm);
  flex-wrap: wrap;
}

.ui-tray--with-footer .ui-tray__footer,
.ui-tray[data-footer="present"] .ui-tray__footer {
  padding-top: var(--ui-space-xs);
  border-top: 1px solid var(--ui-border);
}

.ui-tray__close {
  position: absolute;
  top: 2px;
  right: 2px;
}
"#;
