pub const CSS: &str = r#"
.ui-drawer {
  position: relative;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-md);
  min-height: 0;
}

.ui-drawer--placement-bottom,
.ui-drawer[data-placement="bottom"] {
  width: 100%;
}

.ui-drawer--placement-left,
.ui-drawer[data-placement="left"],
.ui-drawer--placement-right,
.ui-drawer[data-placement="right"] {
  width: 100%;
}

.ui-drawer--with-description,
.ui-drawer[data-state="with-description"],
.ui-drawer[data-description="present"] {
  gap: var(--ui-space-sm);
}

.ui-drawer--title-only,
.ui-drawer[data-state="title-only"],
.ui-drawer[data-description="absent"] {
  gap: var(--ui-space-md);
}

.ui-drawer--custom-class,
.ui-drawer[data-custom-class="true"] {
  border-radius: inherit;
}

.ui-drawer__header {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
  padding-right: 0;
}

.ui-drawer--close-shown .ui-drawer__header,
.ui-drawer[data-close-button="shown"] .ui-drawer__header {
  padding-right: 44px;
}

.ui-drawer--close-hidden .ui-drawer__header,
.ui-drawer[data-close-button="hidden"] .ui-drawer__header {
  padding-right: 0;
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

.ui-drawer--with-footer .ui-drawer__footer,
.ui-drawer[data-footer="present"] .ui-drawer__footer {
  padding-top: var(--ui-space-xs);
  border-top: 1px solid var(--ui-border);
}

.ui-drawer__close {
  position: absolute;
  top: 2px;
  right: 2px;
}
"#;
