pub const CSS: &str = r#"
.ui-divider {
  flex-shrink: 0;
  align-self: stretch;
  background: var(--ui-border);
}

.ui-divider--horizontal,
.ui-divider[data-orientation="horizontal"],
.ui-divider[data-state="horizontal"],
.ui-divider[data-horizontal="true"] {
  width: 100%;
  height: 1px;
}

.ui-divider--vertical,
.ui-divider[data-orientation="vertical"],
.ui-divider[data-state="vertical"],
.ui-divider[data-vertical="true"] {
  width: 1px;
  height: 100%;
  min-height: 1em;
}
"#;
