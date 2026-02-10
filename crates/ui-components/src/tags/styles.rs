pub const CSS: &str = r#"
.ui-tags {
  display: block;
}

.ui-tags[data-state="disabled"] {
  opacity: 0.72;
}

.ui-tags[data-state="empty"] {
  --ui-tags-empty: 1;
}

.ui-tags[data-content="filled"] {
  --ui-tags-has-content: 1;
}

.ui-tags[data-removal="removable"] {
  --ui-tags-removal-mode: removable;
}

.ui-tags[data-removal="static"] {
  --ui-tags-removal-mode: static;
}

.ui-tags[data-constraint="invalid"] {
  --ui-tags-constraint: invalid;
}

.ui-tags[data-constraint="required"] {
  --ui-tags-constraint: required;
}

.ui-tags[data-label-source="custom"] {
  --ui-tags-label-source: custom;
}

.ui-tags[data-description-source="custom"] {
  --ui-tags-description-source: custom;
}

.ui-tags[data-error-source="custom"] {
  --ui-tags-error-source: custom;
}

.ui-tags[data-describedby-source="custom"] {
  --ui-tags-describedby-source: custom;
}

.ui-tags[data-aria-source="custom"] {
  --ui-tags-aria-source: custom;
}

.ui-tags[data-class-source="custom"] {
  --ui-tags-class-source: custom;
}

.ui-tags[data-variant-source="custom"] {
  --ui-tags-variant-source: custom;
}

.ui-tags[data-size-source="custom"] {
  --ui-tags-size-source: custom;
}

.ui-tags[data-handler-source="custom"] {
  --ui-tags-handler-source: custom;
}

.ui-tags--custom-class,
.ui-tags[data-custom-class="true"] {
  border-radius: inherit;
}
"#;
