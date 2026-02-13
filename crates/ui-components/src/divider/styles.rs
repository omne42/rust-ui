pub const CSS: &str = r#"
.ui-divider {
  flex-shrink: 0;
  align-self: stretch;
  background: var(--ui-border);
  opacity: var(--ui-separator-opacity, 1);
  transform: scaleX(var(--ui-separator-scale-x, 1)) scaleY(var(--ui-separator-scale-y, 1));
  will-change: transform, opacity;
}

.ui-divider[data-motion-source="custom"],
.ui-divider[data-custom-motion="true"] {
  --ui-divider-custom-motion: 1;
}

.ui-divider--horizontal,
.ui-divider[data-orientation="horizontal"],
.ui-divider[data-state="horizontal"],
.ui-divider[data-horizontal="true"] {
  width: 100%;
  height: 1px;
  transform-origin: left center;
}

.ui-divider--vertical,
.ui-divider[data-orientation="vertical"],
.ui-divider[data-state="vertical"],
.ui-divider[data-vertical="true"] {
  width: 1px;
  height: 100%;
  min-height: 1em;
  transform-origin: center top;
}
"#;
