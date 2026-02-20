pub const CSS: &str = r#"
.ui-pressable-feedback {
  --ui-pressable-feedback-scale: 1;
  --ui-pressable-feedback-highlight-opacity: 0;

  position: relative;
  display: block;
  min-width: 0;
  transform: scale(var(--ui-pressable-feedback-scale));
  transform-origin: center;
  touch-action: manipulation;
  isolation: isolate;
}

.ui-pressable-feedback__content {
  position: relative;
  z-index: 1;
  min-width: 0;
}

.ui-pressable-feedback__highlight {
  position: absolute;
  inset: 0;
  border-radius: inherit;
  pointer-events: none;
  background: color-mix(in oklab, var(--ui-fg) 16%, transparent);
  opacity: var(--ui-pressable-feedback-highlight-opacity);
  z-index: 2;
}

.ui-pressable-feedback__ripple {
  position: absolute;
  inset: 0;
  border-radius: inherit;
  z-index: 3;
}

.ui-pressable-feedback--tone-default,
.ui-pressable-feedback[data-tone="default"] {
  color: var(--ui-fg);
}

.ui-pressable-feedback--tone-neutral,
.ui-pressable-feedback[data-tone="neutral"] {
  color: var(--ui-fg-muted);
}

.ui-pressable-feedback--tone-accent,
.ui-pressable-feedback[data-tone="accent"] {
  color: var(--ui-accent);
}

.ui-pressable-feedback--state-idle,
.ui-pressable-feedback[data-state="idle"] {
  cursor: pointer;
}

.ui-pressable-feedback--state-pressed,
.ui-pressable-feedback[data-state="pressed"] {
  cursor: pointer;
}

.ui-pressable-feedback--state-disabled,
.ui-pressable-feedback[data-state="disabled"],
.ui-pressable-feedback[data-disabled="true"] {
  cursor: not-allowed;
  opacity: 0.62;
}

.ui-pressable-feedback--effect-scale,
.ui-pressable-feedback[data-effect="scale"] {
  --ui-pressable-feedback-highlight-opacity: 0;
}

.ui-pressable-feedback--effect-highlight,
.ui-pressable-feedback[data-effect="highlight"] {
  --ui-ripple-duration-ms: 0;
}

.ui-pressable-feedback--effect-ripple,
.ui-pressable-feedback[data-effect="ripple"] {
  --ui-pressable-feedback-highlight-opacity: 0;
}

.ui-pressable-feedback--effect-highlight-ripple,
.ui-pressable-feedback[data-effect="highlight-ripple"] {
  --ui-ripple-duration-ms: 420;
}

.ui-pressable-feedback--boundary-bounded,
.ui-pressable-feedback[data-boundary="bounded"] {
  overflow: clip;
}

.ui-pressable-feedback--boundary-unbounded,
.ui-pressable-feedback[data-boundary="unbounded"] {
  overflow: visible;
}

.ui-pressable-feedback--highlight-enabled .ui-pressable-feedback__highlight,
.ui-pressable-feedback[data-highlight="enabled"] .ui-pressable-feedback__highlight {
  display: block;
}

.ui-pressable-feedback[data-highlight="none"] .ui-pressable-feedback__highlight {
  display: none;
}

.ui-pressable-feedback--ripple-enabled .ui-pressable-feedback__ripple,
.ui-pressable-feedback[data-ripple="enabled"] .ui-pressable-feedback__ripple {
  display: block;
}

.ui-pressable-feedback[data-ripple="none"] .ui-pressable-feedback__ripple {
  display: none;
}

.ui-pressable-feedback--custom-class,
.ui-pressable-feedback[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 24%, transparent);
  outline-offset: 2px;
}
"#;
