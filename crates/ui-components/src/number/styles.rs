pub const CSS: &str = r#"
.ui-static-number {
  font-variant-numeric: tabular-nums;
}

.ui-sliding-number {
  position: relative;
  display: inline-flex;
  align-items: center;
  font-variant-numeric: tabular-nums;
}

.ui-sliding-number__a11y-value {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

.ui-sliding-number__visual {
  display: inline-flex;
  align-items: center;
}

.ui-sliding-number__roller {
  position: relative;
  display: inline-block;
  width: 1ch;
  height: 1em;
  overflow: hidden;
  line-height: 1;

  --ui-sliding-number-offset: 10;
}

.ui-sliding-number__stack {
  position: absolute;
  top: 0;
  left: 0;
  display: flex;
  flex-direction: column;
  transform: translateY(calc(var(--ui-sliding-number-offset) * -1em));
  will-change: transform;
}

.ui-sliding-number__digit {
  height: 1em;
  display: flex;
  align-items: center;
  justify-content: center;
}
"#;
