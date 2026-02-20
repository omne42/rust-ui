pub const CSS: &str = r#"
.ui-collapsible {
  --ui-collapsible-open: 0;
  display: flex;
  flex-direction: column;
}

.ui-collapsible--state-open,
.ui-collapsible[data-open="true"],
.ui-collapsible[data-state="open"] {
  --ui-collapsible-open: 1;
}

.ui-collapsible--state-closed,
.ui-collapsible[data-closed="true"],
.ui-collapsible[data-state="closed"] {
  --ui-collapsible-open: 0;
}

.ui-collapsible--state-disabled,
.ui-collapsible[data-state="disabled"] {
  opacity: 0.72;
}

.ui-collapsible .ui-disclosure__trigger {
  transition:
    border-color 200ms ease,
    background-color 200ms ease,
    box-shadow 200ms ease;
}

.ui-collapsible[data-open="true"] .ui-disclosure__trigger,
.ui-collapsible[data-state="open"] .ui-disclosure__trigger {
  border-color: color-mix(in oklch, var(--ui-accent) 35%, var(--ui-border));
  background: color-mix(in oklch, var(--ui-bg) 85%, var(--ui-accent-soft));
}

.ui-collapsible[data-disabled="true"] .ui-disclosure__trigger,
.ui-collapsible[data-state="disabled"] .ui-disclosure__trigger {
  box-shadow: none;
}

.ui-collapsible .ui-disclosure__panel {
  border-color: color-mix(in oklch, var(--ui-border) 85%, var(--ui-accent-soft));
}

.ui-collapsible[data-open-mode="controlled"] {
  --ui-collapsible-open-mode: controlled;
}

.ui-collapsible[data-motion-source="custom"],
.ui-collapsible[data-custom-motion="true"] {
  --ui-collapsible-custom-motion: 1;
}

.ui-collapsible--custom-class,
.ui-collapsible[data-custom-class="true"] {
  border-radius: inherit;
}

@media (forced-colors: active) {
  .ui-collapsible,
  .ui-collapsible * {
    forced-color-adjust: auto;
  }
}

@media (prefers-reduced-motion: reduce) {
  .ui-collapsible .ui-disclosure__trigger {
    transition: none;
  }
}
"#;
