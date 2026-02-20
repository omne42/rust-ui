pub const CSS: &str = r#"
.ui-visually-hidden {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0 0 0 0);
  clip-path: inset(50%);
  white-space: nowrap;
  border: 0;
}

.ui-visually-hidden--focusable:active,
.ui-visually-hidden--focusable:focus-within,
.ui-visually-hidden[data-focus-mode="focusable"]:active,
.ui-visually-hidden[data-focus-mode="focusable"]:focus-within {
  position: static;
  width: auto;
  height: auto;
  padding: 0;
  margin: 0;
  overflow: visible;
  clip: auto;
  clip-path: none;
  white-space: normal;
}
"#;
