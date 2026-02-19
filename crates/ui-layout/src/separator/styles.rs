pub const CSS: &str = r#"
.ui-separator {
  flex-shrink: 0;
  align-self: stretch;
  background: var(--ui-border);
  opacity: var(--ui-separator-opacity, 1);
  transform: scaleX(var(--ui-separator-scale-x, 1)) scaleY(var(--ui-separator-scale-y, 1));
  will-change: transform, opacity;
}

.ui-separator[data-motion-source="custom"],
.ui-separator[data-custom-motion="true"] {
  --ui-separator-custom-motion: 1;
}

.ui-separator--horizontal,
.ui-separator[data-orientation="horizontal"] {
  width: 100%;
  height: 1px;
  transform-origin: left center;
}

.ui-separator--vertical,
.ui-separator[data-orientation="vertical"] {
  width: 1px;
  height: 100%;
  min-height: 1em;
  transform-origin: center top;
}

.ui-separator--element-div,
.ui-separator[data-element="div"] {
  display: block;
}

.ui-separator--element-hr,
.ui-separator[data-element="hr"] {
  border: 0;
  margin: 0;
}

.ui-separator--semantic,
.ui-separator[data-state="semantic"],
.ui-separator[data-semantic="true"] {
  opacity: var(--ui-separator-opacity, 1);
}

.ui-separator--decorative,
.ui-separator[data-state="decorative"],
.ui-separator[data-decorative="true"] {
  opacity: var(--ui-separator-decorative-opacity);
}
"#;
