pub const CSS: &str = r#"
.ui-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: var(--ui-button-min-width);
  --ui-button-padding-x: var(--ui-button-size-m-padding-x);
  --ui-button-gap: var(--ui-button-size-m-gap);
  --ui-button-inline-spinner-size: min(
    var(--ui-button-spinner-size),
    calc(var(--ui-button-padding-x) + var(--ui-button-gap))
  );
  gap: var(--ui-button-gap);
  padding: 0 var(--ui-button-padding-x);
  white-space: nowrap;
  position: relative;
  --ui-button-bg: var(--ui-accent);
  --ui-button-fg: var(--ui-accent-fg);
  --ui-button-soft: color-mix(in oklch, var(--ui-accent-soft), var(--ui-bg) 25%);
  --ui-button-border-tone: color-mix(in oklch, var(--ui-border), var(--ui-accent) 32%);
  --ui-button-radius: var(--ui-radius-md);
  border-radius: var(--ui-button-radius);
  border: 1px solid transparent;
  box-sizing: border-box;
  line-height: 1;
  font-weight: 500;
  font-size: var(--ui-button-font-size);
  user-select: none;
  -webkit-tap-highlight-color: transparent;
  text-decoration: none;

  transform: scale(var(--ui-button-scale, 1));
  transform-origin: center;
  will-change: transform;
}

.ui-button--color-default {
  --ui-button-bg: var(--ui-default);
  --ui-button-fg: var(--ui-default-foreground);
  --ui-button-soft: color-mix(in oklch, var(--ui-default), var(--ui-bg) 78%);
  --ui-button-border-tone: color-mix(in oklch, var(--ui-border), var(--ui-default) 32%);
}

.ui-button--color-primary {
  --ui-button-bg: var(--ui-primary);
  --ui-button-fg: var(--ui-primary-foreground);
  --ui-button-soft: color-mix(in oklch, var(--ui-primary), var(--ui-bg) 82%);
  --ui-button-border-tone: color-mix(in oklch, var(--ui-border), var(--ui-primary) 34%);
}

.ui-button--color-secondary {
  --ui-button-bg: var(--ui-secondary);
  --ui-button-fg: var(--ui-secondary-foreground);
  --ui-button-soft: color-mix(in oklch, var(--ui-secondary), var(--ui-bg) 80%);
  --ui-button-border-tone: color-mix(in oklch, var(--ui-border), var(--ui-secondary) 34%);
}

.ui-button--color-success {
  --ui-button-bg: var(--ui-success);
  --ui-button-fg: var(--ui-success-foreground);
  --ui-button-soft: color-mix(in oklch, var(--ui-success), var(--ui-bg) 82%);
  --ui-button-border-tone: color-mix(in oklch, var(--ui-border), var(--ui-success) 34%);
}

.ui-button--color-warning {
  --ui-button-bg: var(--ui-warning);
  --ui-button-fg: var(--ui-warning-foreground);
  --ui-button-soft: color-mix(in oklch, var(--ui-warning), var(--ui-bg) 84%);
  --ui-button-border-tone: color-mix(in oklch, var(--ui-border), var(--ui-warning) 34%);
}

.ui-button--color-danger {
  --ui-button-bg: var(--ui-danger);
  --ui-button-fg: var(--ui-danger-foreground);
  --ui-button-soft: color-mix(in oklch, var(--ui-danger), var(--ui-bg) 82%);
  --ui-button-border-tone: color-mix(in oklch, var(--ui-border), var(--ui-danger) 34%);
}

.ui-button--radius-none {
  --ui-button-radius: 0;
}

.ui-button--radius-sm {
  --ui-button-radius: var(--ui-radius-sm);
}

.ui-button--radius-md {
  --ui-button-radius: var(--ui-radius-md);
}

.ui-button--radius-lg {
  --ui-button-radius: var(--ui-radius-lg);
}

.ui-button--radius-full {
  --ui-button-radius: var(--ui-button-radius-full);
}

.ui-button[data-motion-source="custom"],
.ui-button[data-custom-motion="true"] {
  --ui-button-custom-motion: 1;
}

.ui-button__spinner {
  width: var(--ui-button-inline-spinner-size);
  height: var(--ui-button-inline-spinner-size);
  border-radius: var(--ui-button-radius-full);
  border: var(--ui-button-spinner-border) solid currentColor;
  border-top-color: transparent;
  box-sizing: border-box;

  animation: ui-button-spin var(--ui-button-spinner-duration) linear infinite;
}

.ui-button__start,
.ui-button__label,
.ui-button__end {
  display: inline-flex;
  align-items: center;
}

.ui-button__start {
  position: relative;
  justify-content: center;
}

.ui-button__start-content {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-button__start[data-loading-start="true"] .ui-button__start-content {
  visibility: hidden;
}

.ui-button__start[data-loading-start="true"] .ui-button__spinner {
  position: absolute;
  left: 50%;
  top: 50%;
  margin-left: calc(var(--ui-button-inline-spinner-size) / -2);
  margin-top: calc(var(--ui-button-inline-spinner-size) / -2);
}

.ui-button--full-width {
  width: 100%;
  min-width: 0;
}

.ui-button--icon-only {
  padding-inline: 0;
  min-width: 0;
}

.ui-button--icon-only.ui-button--size-xs {
  width: var(--ui-button-size-xs-icon);
}

.ui-button--icon-only.ui-button--size-s {
  width: var(--ui-button-size-s-icon);
}

.ui-button--icon-only.ui-button--size-m {
  width: var(--ui-button-size-m-icon);
}

.ui-button--icon-only.ui-button--size-l {
  width: var(--ui-button-size-l-icon);
}

.ui-button--icon-only.ui-button--size-xl {
  width: var(--ui-button-size-xl-icon);
}

.ui-button[data-loading-placement="center"] .ui-button__spinner {
  position: absolute;
  left: 50%;
  top: 50%;
  margin-left: calc(var(--ui-button-inline-spinner-size) / -2);
  margin-top: calc(var(--ui-button-inline-spinner-size) / -2);
}

.ui-button[data-loading="true"][data-loading-placement="center"] .ui-button__label {
  visibility: hidden;
}

.ui-button[data-loading="true"][data-loading-placement="start"]:not([data-has-start="true"]) {
  --ui-button-loading-start-offset: calc(
    (var(--ui-button-inline-spinner-size) + var(--ui-button-gap)) / 2
  );
  padding-inline-start: calc(var(--ui-button-padding-x) - var(--ui-button-loading-start-offset));
  padding-inline-end: calc(var(--ui-button-padding-x) - var(--ui-button-loading-start-offset));
}

@keyframes ui-button-spin {
  to {
    transform: rotate(360deg);
  }
}

@media (prefers-reduced-motion: reduce) {
  .ui-button__spinner {
    animation: none;
  }
}

.ui-button:not(:disabled) {
  cursor: pointer;
}

.ui-button:disabled {
  pointer-events: none;
  opacity: 0.5;
}

.ui-button svg {
  pointer-events: none;
  flex-shrink: 0;
}

.ui-button--focus-visible {
  outline: var(--ui-button-focus-outline-width) solid var(--ui-focus-ring);
  outline-offset: var(--ui-button-focus-outline-offset);
}

.ui-button--size-xs {
  height: var(--ui-button-size-xs-height);
  --ui-button-padding-x: var(--ui-button-size-xs-padding-x);
  --ui-button-gap: var(--ui-button-size-xs-gap);
}

.ui-button--size-s {
  height: var(--ui-button-size-s-height);
  --ui-button-padding-x: var(--ui-button-size-s-padding-x);
  --ui-button-gap: var(--ui-button-size-s-gap);
}

.ui-button--size-m {
  height: var(--ui-button-size-m-height);
  --ui-button-padding-x: var(--ui-button-size-m-padding-x);
  --ui-button-gap: var(--ui-button-size-m-gap);
}

.ui-button--size-l {
  height: var(--ui-button-size-l-height);
  --ui-button-padding-x: var(--ui-button-size-l-padding-x);
  --ui-button-gap: var(--ui-button-size-l-gap);
}

.ui-button--size-xl {
  height: var(--ui-button-size-xl-height);
  --ui-button-padding-x: var(--ui-button-size-xl-padding-x);
  --ui-button-gap: var(--ui-button-size-xl-gap);
}

.ui-button--size-icon-xs {
  width: var(--ui-button-size-xs-icon);
  height: var(--ui-button-size-xs-icon);
  --ui-button-padding-x: 0px;
  --ui-button-gap: 0px;
}

.ui-button--size-icon-s {
  width: var(--ui-button-size-s-icon);
  height: var(--ui-button-size-s-icon);
  --ui-button-padding-x: 0px;
  --ui-button-gap: 0px;
}

.ui-button--size-icon-m {
  width: var(--ui-button-size-m-icon);
  height: var(--ui-button-size-m-icon);
  --ui-button-padding-x: 0px;
  --ui-button-gap: 0px;
}

.ui-button--size-icon-l {
  width: var(--ui-button-size-l-icon);
  height: var(--ui-button-size-l-icon);
  --ui-button-padding-x: 0px;
  --ui-button-gap: 0px;
}

.ui-button--size-icon-xl {
  width: var(--ui-button-size-xl-icon);
  height: var(--ui-button-size-xl-icon);
  --ui-button-padding-x: 0px;
  --ui-button-gap: 0px;
}

.ui-button--variant-default,
.ui-button--variant-solid {
  background: var(--ui-button-bg);
  color: var(--ui-button-fg);
  box-shadow: var(--ui-shadow-sm);
}

.ui-button--variant-shadow {
  background: var(--ui-button-bg);
  color: var(--ui-button-fg);
  box-shadow: var(--ui-shadow-md);
}

.ui-button--variant-accent,
.ui-button--variant-faded {
  background: color-mix(in oklch, var(--ui-button-bg), var(--ui-bg) 90%);
  color: color-mix(in oklch, var(--ui-button-bg), var(--ui-fg) 25%);
  border-color: color-mix(in oklch, var(--ui-button-border-tone), transparent 20%);
  box-shadow: var(--ui-shadow-sm);
}

.ui-button--variant-secondary,
.ui-button--variant-flat {
  background: var(--ui-button-soft);
  color: color-mix(in oklch, var(--ui-button-bg), var(--ui-fg) 20%);
  border-color: color-mix(in oklch, var(--ui-button-border-tone), transparent 42%);
  box-shadow: var(--ui-shadow-sm);
}

.ui-button--variant-outline,
.ui-button--variant-bordered {
  background: transparent;
  border-color: var(--ui-button-border-tone);
  color: color-mix(in oklch, var(--ui-button-bg), var(--ui-fg) 20%);
  box-shadow: var(--ui-shadow-sm);
}

.ui-button--variant-light {
  background: transparent;
  color: color-mix(in oklch, var(--ui-button-bg), var(--ui-fg) 14%);
  box-shadow: none;
}

.ui-button--variant-ghost {
  background: transparent;
  color: var(--ui-fg);
  box-shadow: none;
}

.ui-button--variant-link {
  background: transparent;
  color: var(--ui-button-bg);
  box-shadow: none;
}

.ui-button--variant-destructive {
  --ui-button-bg: var(--ui-danger);
  --ui-button-fg: var(--ui-danger-fg);
  --ui-button-soft: color-mix(in oklch, var(--ui-danger), var(--ui-bg) 82%);
  --ui-button-border-tone: color-mix(in oklch, var(--ui-border), var(--ui-danger) 34%);
  background: var(--ui-button-bg);
  color: var(--ui-button-fg);
  box-shadow: var(--ui-shadow-sm);
}

.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-default,
.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-solid,
.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-shadow {
  filter: brightness(0.96);
}

.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-accent,
.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-faded,
.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-secondary,
.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-flat {
  filter: brightness(0.98);
}

.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-destructive {
  filter: brightness(0.96);
}

.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-outline,
.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-bordered,
.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-light {
  background: color-mix(in oklch, var(--ui-button-soft), transparent 45%);
}

.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-ghost {
  background: var(--ui-bg-muted);
}

.ui-button[data-hovered="true"]:not(:disabled).ui-button--variant-link {
  text-decoration: underline;
  text-decoration-thickness: 2px;
  text-underline-offset: 4px;
}
"#;
