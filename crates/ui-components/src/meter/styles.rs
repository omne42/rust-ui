pub const CSS: &str = r#"
.ui-meter {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
  --ui-meter-indicator-color: var(--ui-accent);
}

.ui-meter--variant-default,
.ui-meter[data-variant="default"] {
  --ui-meter-indicator-color: var(--ui-accent);
}

.ui-meter--variant-danger,
.ui-meter[data-variant="danger"] {
  --ui-meter-indicator-color: var(--ui-danger);
}

.ui-meter__header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--ui-space-sm);
}

.ui-meter__label {
  font-size: 13px;
  font-weight: 600;
  color: var(--ui-fg);
}

.ui-meter__value-label {
  font-size: 12px;
  color: var(--ui-fg-muted);
}

.ui-meter--label-custom .ui-meter__label,
.ui-meter[data-label-source="custom"] .ui-meter__label {
  color: color-mix(in oklch, var(--ui-fg), var(--ui-accent) 20%);
}

.ui-meter--value-label-custom .ui-meter__value-label,
.ui-meter[data-value-label-source="custom"] .ui-meter__value-label {
  color: var(--ui-fg);
}

.ui-meter__track {
  position: relative;
  height: 10px;
  border-radius: 999px;
  background: var(--ui-bg);
  border: 1px solid var(--ui-border);
  overflow: hidden;
}

.ui-meter--size-sm .ui-meter__track,
.ui-meter[data-size="sm"] .ui-meter__track {
  height: 8px;
}

.ui-meter--size-lg .ui-meter__track,
.ui-meter[data-size="lg"] .ui-meter__track {
  height: 12px;
}

.ui-meter__indicator {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 100%;
  transform-origin: left center;
  transform: scaleX(var(--ui-meter-progress, 0));
  background: var(--ui-meter-indicator-color);
  border-radius: inherit;
  will-change: transform;
}

.ui-meter--motion-custom,
.ui-meter[data-motion-source="custom"] {
  transition: box-shadow 160ms ease;
}

.ui-meter--custom-class,
.ui-meter[data-custom-class="true"] {
  isolation: isolate;
}

.ui-meter--indeterminate .ui-meter__indicator,
.ui-meter--state-indeterminate .ui-meter__indicator,
.ui-meter[data-state="indeterminate"] .ui-meter__indicator {
  width: 40%;
  transform: translateX(-60%);
  animation: ui-meter-indeterminate 1.2s ease-in-out infinite;
}

.ui-meter--state-determinate .ui-meter__indicator,
.ui-meter[data-state="determinate"] .ui-meter__indicator {
  width: 100%;
  transform: scaleX(var(--ui-meter-progress, 0));
  animation: none;
}

@media (prefers-reduced-motion: reduce) {
  .ui-meter--indeterminate .ui-meter__indicator,
  .ui-meter--state-indeterminate .ui-meter__indicator,
  .ui-meter[data-state="indeterminate"] .ui-meter__indicator {
    animation: none;
  }
}

@keyframes ui-meter-indeterminate {
  0% {
    transform: translateX(-60%);
  }
  50% {
    transform: translateX(80%);
  }
  100% {
    transform: translateX(220%);
  }
}
"#;
