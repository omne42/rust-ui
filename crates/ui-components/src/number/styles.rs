pub const CSS: &str = r#"
.ui-static-number {
  font-variant-numeric: tabular-nums;
}

.ui-static-number--sign-negative,
.ui-static-number[data-sign="negative"] {
  color: var(--ui-danger);
}

.ui-static-number--sign-zero,
.ui-static-number[data-sign="zero"] {
  color: var(--ui-fg-muted);
}

.ui-static-number--sign-positive,
.ui-static-number[data-sign="positive"] {
  color: var(--ui-fg);
}

.ui-static-number--decimal-separator-custom,
.ui-static-number[data-decimal-separator-source="custom"] {
  text-decoration: underline;
  text-decoration-thickness: 1px;
  text-underline-offset: 0.16em;
}

.ui-static-number--decimal-places-custom,
.ui-static-number[data-decimal-places-source="custom"] {
  letter-spacing: 0.01em;
}

.ui-static-number--thousand-separator-custom,
.ui-static-number[data-thousand-separator-source="custom"] {
  font-feature-settings: "tnum" 1;
}

.ui-static-number--custom-class,
.ui-static-number[data-custom-class="true"] {
  isolation: isolate;
}

.ui-sliding-number {
  position: relative;
  display: inline-flex;
  align-items: center;
  font-variant-numeric: tabular-nums;
}

.ui-sliding-number--sign-negative,
.ui-sliding-number[data-sign="negative"] {
  color: var(--ui-danger);
}

.ui-sliding-number--sign-zero,
.ui-sliding-number[data-sign="zero"] {
  color: var(--ui-fg-muted);
}

.ui-sliding-number--state-animated,
.ui-sliding-number[data-state="animated"] {
  transition: color 180ms ease;
}

.ui-sliding-number--state-static,
.ui-sliding-number[data-state="static"] {
  transition: none;
}

.ui-sliding-number--decimal-separator-custom .ui-sliding-number__separator,
.ui-sliding-number[data-decimal-separator-source="custom"] .ui-sliding-number__separator {
  opacity: 0.92;
}

.ui-sliding-number--decimal-places-custom .ui-sliding-number__digit,
.ui-sliding-number[data-decimal-places-source="custom"] .ui-sliding-number__digit {
  letter-spacing: 0.01em;
}

.ui-sliding-number--thousand-separator-custom .ui-sliding-number__separator,
.ui-sliding-number[data-thousand-separator-source="custom"] .ui-sliding-number__separator {
  font-feature-settings: "tnum" 1;
}

.ui-sliding-number--motion-custom,
.ui-sliding-number[data-motion-source="custom"] {
  transition: text-shadow 200ms ease;
}

.ui-sliding-number--custom-class,
.ui-sliding-number[data-custom-class="true"] {
  isolation: isolate;
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
