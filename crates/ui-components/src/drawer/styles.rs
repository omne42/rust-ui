pub const CSS: &str = r#"
.ui-drawer {
  position: relative;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-md);
  min-height: 0;
}

.ui-drawer[data-motion-source="custom"],
.ui-drawer[data-custom-motion="true"],
.ui-drawer--custom-motion {
  --ui-drawer-custom-motion: 1;
}

.ui-drawer[data-placement-source="custom"],
.ui-drawer--custom-placement {
  --ui-drawer-placement-source: custom;
}

.ui-drawer--custom-description,
.ui-drawer[data-description-source="custom"],
.ui-drawer[data-custom-description="true"] {
  --ui-drawer-description-source: custom;
}

.ui-drawer--custom-footer,
.ui-drawer[data-footer-source="custom"],
.ui-drawer[data-custom-footer="true"] {
  --ui-drawer-footer-source: custom;
}

.ui-drawer--custom-close,
.ui-drawer[data-close-source="custom"],
.ui-drawer[data-custom-close="true"] {
  --ui-drawer-close-source: custom;
}

.ui-drawer--custom-id,
.ui-drawer[data-id-source="custom"],
.ui-drawer[data-custom-id="true"] {
  --ui-drawer-id-source: custom;
}

.ui-drawer--custom-title,
.ui-drawer[data-title-source="custom"],
.ui-drawer[data-custom-title="true"] {
  --ui-drawer-title-source: custom;
}

.ui-drawer[data-class-source="custom"],
.ui-drawer--custom-class {
  --ui-drawer-class-source: custom;
}

.ui-drawer[data-exit-source="custom"],
.ui-drawer[data-custom-exit="true"],
.ui-drawer--custom-exit {
  --ui-drawer-exit-source: custom;
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

.ui-drawer__header,
.ui-drawer__header[data-slot="drawer-header"] {
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

.ui-drawer__title,
.ui-drawer__title[data-slot="drawer-title"] {
  font-size: 16px;
  line-height: 1.2;
  font-weight: 700;
  margin: 0;
}

.ui-drawer__title[data-title-source="custom"] {
  --ui-drawer-title: custom;
}

.ui-drawer__description,
.ui-drawer__description[data-slot="drawer-description"] {
  font-size: 13px;
  line-height: 1.5;
  color: var(--ui-fg-muted);
  margin: 0;
}

.ui-drawer__description[data-description-source="custom"] {
  --ui-drawer-description: custom;
}

.ui-drawer__body,
.ui-drawer__body[data-slot="drawer-body"] {
  flex: 1 1 auto;
  min-height: 0;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-sm);
}

.ui-drawer__footer,
.ui-drawer__footer[data-slot="drawer-footer"] {
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

.ui-drawer__close,
.ui-drawer__close[data-slot="drawer-close"] {
  position: absolute;
  top: 2px;
  right: 2px;
}
"#;
