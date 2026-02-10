pub const CSS: &str = r#"
.ui-empty {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-sm);
}

.ui-empty[data-state="root"] {
  --ui-empty-root: 1;
}

.ui-empty__header[data-state="header"] {
  --ui-empty-header: 1;
}

.ui-empty__title[data-state="title"] {
  --ui-empty-title: 1;
}

.ui-empty__description[data-state="description"] {
  --ui-empty-description: 1;
}

.ui-empty__content[data-state="content"] {
  --ui-empty-content: 1;
}

.ui-empty__media[data-state="media"] {
  --ui-empty-media: 1;
}

.ui-empty__media[data-variant="icon"],
.ui-empty__media--icon {
  --ui-empty-media-variant: icon;
}

.ui-empty[data-class-source="custom"],
.ui-empty__header[data-class-source="custom"],
.ui-empty__title[data-class-source="custom"],
.ui-empty__description[data-class-source="custom"],
.ui-empty__content[data-class-source="custom"],
.ui-empty__media[data-class-source="custom"] {
  --ui-empty-class-source: custom;
}

.ui-empty__media[data-variant-source="custom"] {
  --ui-empty-variant-source: custom;
}

.ui-empty--custom-class,
.ui-empty[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
