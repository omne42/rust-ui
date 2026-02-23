pub const CSS: &str = r#"
.ui-pressable-feedback {
  --ui-pressable-feedback-scale: 1;
  --ui-pressable-feedback-highlight-opacity: 0;
  --ui-pressable-feedback-fg: var(--ui-fg, var(--ui-fallback-fg));
  --ui-pressable-feedback-fg-muted: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  --ui-pressable-feedback-accent: var(--ui-accent, var(--ui-fallback-accent));
  --ui-pressable-feedback-disabled-opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity, 0.62));
  --ui-pressable-feedback-highlight-mix: var(--ui-command-option-focus-mix, var(--ui-fallback-command-option-focus-mix, 16%));
  --ui-pressable-feedback-outline-mix: var(--ui-command-group-border-mix, var(--ui-fallback-command-group-border-mix, 24%));
  --ui-pressable-feedback-outline-width: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width, 1px));
  --ui-pressable-feedback-outline-offset: var(--ui-button-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset, 2px));
  --ui-pressable-feedback-ripple-duration-ms: var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration, 180ms));
  --ui-pressable-feedback-interaction-duration-ms: var(--ui-button-motion-duration, var(--ui-fallback-button-motion-duration, 180ms));
  --ui-pressable-feedback-interaction-ease: var(--ui-button-motion-ease, var(--ui-fallback-button-motion-ease, cubic-bezier(0.2, 0, 0, 1)));
  --ui-pressable-feedback-hover-highlight-opacity: var(--ui-button-hover-overlay-opacity, var(--ui-fallback-button-hover-overlay-opacity, 0.08));
  --ui-pressable-feedback-active-highlight-opacity: var(--ui-button-active-overlay-opacity, var(--ui-fallback-button-active-overlay-opacity, 0.16));
  --ui-pressable-feedback-hover-outline-mix: var(--ui-button-hover-outline-mix, var(--ui-fallback-button-hover-outline-mix, 18%));
  --ui-pressable-feedback-focus-outline-mix: var(--ui-button-focus-outline-mix, var(--ui-fallback-button-focus-outline-mix, 36%));

  position: relative;
  display: block;
  min-width: 0;
  transform: scale(var(--ui-pressable-feedback-scale));
  transform-origin: center;
  touch-action: manipulation;
  isolation: isolate;
  outline: var(--ui-pressable-feedback-outline-width) solid transparent;
  outline-offset: var(--ui-pressable-feedback-outline-offset);
  transition:
    transform var(--ui-pressable-feedback-interaction-duration-ms) var(--ui-pressable-feedback-interaction-ease),
    outline-color var(--ui-pressable-feedback-interaction-duration-ms) var(--ui-pressable-feedback-interaction-ease);
}

.ui-pressable-feedback__content {
  position: relative;
  z-index: 1;
  min-width: 0;
  transition:
    box-shadow var(--ui-pressable-feedback-interaction-duration-ms) var(--ui-pressable-feedback-interaction-ease);
}

.ui-pressable-feedback__highlight {
  position: absolute;
  inset: 0;
  border-radius: inherit;
  pointer-events: none;
  background: color-mix(
    in oklab,
    var(--ui-pressable-feedback-fg) var(--ui-pressable-feedback-highlight-mix),
    transparent
  );
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
  color: var(--ui-pressable-feedback-fg);
}

.ui-pressable-feedback--tone-neutral,
.ui-pressable-feedback[data-tone="neutral"] {
  color: var(--ui-pressable-feedback-fg-muted);
}

.ui-pressable-feedback--tone-accent,
.ui-pressable-feedback[data-tone="accent"] {
  color: var(--ui-pressable-feedback-accent);
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
  opacity: var(--ui-pressable-feedback-disabled-opacity);
}

.ui-pressable-feedback:not([data-disabled="true"]):hover {
  --ui-pressable-feedback-highlight-opacity: var(--ui-pressable-feedback-hover-highlight-opacity);
  outline-color: color-mix(
    in oklab,
    var(--ui-pressable-feedback-fg) var(--ui-pressable-feedback-hover-outline-mix),
    transparent
  );
}

.ui-pressable-feedback:not([data-disabled="true"]):active,
.ui-pressable-feedback:not([data-disabled="true"])[data-state="pressed"] {
  --ui-pressable-feedback-highlight-opacity: var(--ui-pressable-feedback-active-highlight-opacity);
}

.ui-pressable-feedback:not([data-disabled="true"]):focus-visible {
  outline-color: color-mix(
    in oklab,
    var(--ui-pressable-feedback-accent) var(--ui-pressable-feedback-focus-outline-mix),
    transparent
  );
}

.ui-pressable-feedback:not([data-disabled="true"]):hover .ui-pressable-feedback__content {
  box-shadow: 0 0 0 1px
    color-mix(
      in oklab,
      var(--ui-pressable-feedback-fg) var(--ui-pressable-feedback-hover-outline-mix),
      transparent
    );
}

.ui-pressable-feedback:not([data-disabled="true"]):focus-visible .ui-pressable-feedback__content {
  box-shadow: 0 0 0 1px
    color-mix(
      in oklab,
      var(--ui-pressable-feedback-accent) var(--ui-pressable-feedback-focus-outline-mix),
      transparent
    );
}

.ui-pressable-feedback--effect-scale,
.ui-pressable-feedback[data-effect="scale"] {
  --ui-pressable-feedback-highlight-opacity: 0;
}

.ui-pressable-feedback--effect-highlight,
.ui-pressable-feedback[data-effect="highlight"] {
  --ui-ripple-duration-ms: 0ms;
}

.ui-pressable-feedback--effect-ripple,
.ui-pressable-feedback[data-effect="ripple"] {
  --ui-pressable-feedback-highlight-opacity: 0;
}

.ui-pressable-feedback--effect-highlight-ripple,
.ui-pressable-feedback[data-effect="highlight-ripple"] {
  --ui-ripple-duration-ms: var(--ui-pressable-feedback-ripple-duration-ms);
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
  outline: var(--ui-pressable-feedback-outline-width) solid
    color-mix(
      in oklab,
      var(--ui-pressable-feedback-accent) var(--ui-pressable-feedback-outline-mix),
      transparent
    );
  outline-offset: var(--ui-pressable-feedback-outline-offset);
}
"#;
