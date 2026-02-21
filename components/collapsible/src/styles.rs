pub const CSS: &str = r#"
.ui-collapsible {
  --ui-collapsible-open: 0;
  --ui-collapsible-disabled-opacity: var(
    --ui-disabled-opacity,
    var(--ui-fallback-disabled-opacity)
  );
  --ui-collapsible-motion-duration: var(
    --ui-text-field-motion-duration,
    var(--ui-fallback-text-field-motion-duration)
  );
  --ui-collapsible-motion-easing: var(
    --ui-text-field-motion-easing,
    var(--ui-fallback-text-field-motion-easing)
  );
  --ui-collapsible-open-trigger-border: color-mix(
    in oklch,
    var(--ui-accent, var(--ui-fallback-accent)) 35%,
    var(--ui-border, var(--ui-fallback-border))
  );
  --ui-collapsible-open-trigger-bg: color-mix(
    in oklch,
    var(--ui-bg, var(--ui-fallback-bg)) 85%,
    var(--ui-accent-soft, var(--ui-fallback-accent-soft))
  );
  --ui-collapsible-panel-border: color-mix(
    in oklch,
    var(--ui-border, var(--ui-fallback-border)) 85%,
    var(--ui-accent-soft, var(--ui-fallback-accent-soft))
  );
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
  opacity: var(--ui-collapsible-disabled-opacity);
}

.ui-collapsible .ui-disclosure__trigger {
  transition:
    border-color var(--ui-collapsible-motion-duration) var(--ui-collapsible-motion-easing),
    background-color var(--ui-collapsible-motion-duration) var(--ui-collapsible-motion-easing),
    box-shadow var(--ui-collapsible-motion-duration) var(--ui-collapsible-motion-easing);
}

.ui-collapsible[data-open="true"] .ui-disclosure__trigger,
.ui-collapsible[data-state="open"] .ui-disclosure__trigger {
  border-color: var(--ui-collapsible-open-trigger-border);
  background: var(--ui-collapsible-open-trigger-bg);
}

.ui-collapsible[data-disabled="true"] .ui-disclosure__trigger,
.ui-collapsible[data-state="disabled"] .ui-disclosure__trigger {
  box-shadow: none;
}

.ui-collapsible .ui-disclosure__panel {
  border-color: var(--ui-collapsible-panel-border);
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
