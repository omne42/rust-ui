pub const CSS: &str = r#"
.ui-color-loupe {
  position: absolute;
  inline-size: 3.125rem;
  block-size: 4.125rem;
  transform: translate(-50%, calc(-100% - var(--ui-space-xs))) scale(0.84);
  transform-origin: 50% 100%;
  opacity: 0;
  pointer-events: none;
  z-index: 2;
  filter: drop-shadow(0 4px 10px color-mix(in oklch, var(--ui-fg), transparent 84%));
}

.ui-color-loupe--x-start {
  left: 16%;
}

.ui-color-loupe--x-center {
  left: 50%;
}

.ui-color-loupe--x-end {
  left: 84%;
}

.ui-color-loupe--y-start {
  top: 16%;
}

.ui-color-loupe--y-center {
  top: 50%;
}

.ui-color-loupe--y-end {
  top: 84%;
}

.ui-color-loupe__bubble {
  position: absolute;
  inset: 0 0 0.95rem;
  border-radius: var(--ui-radius-full, 999px);
  border: 1px solid color-mix(in oklch, var(--ui-border), transparent 18%);
  background: color-mix(in oklch, var(--ui-bg), var(--ui-fg) 2%);
  overflow: hidden;
}

.ui-color-loupe__checker {
  position: absolute;
  inset: 0.25rem;
  border-radius: inherit;
  background:
    linear-gradient(45deg, color-mix(in oklch, var(--ui-border), transparent 32%) 25%, transparent 25%, transparent 75%, color-mix(in oklch, var(--ui-border), transparent 32%) 75%),
    linear-gradient(45deg, color-mix(in oklch, var(--ui-border), transparent 32%) 25%, transparent 25%, transparent 75%, color-mix(in oklch, var(--ui-border), transparent 32%) 75%);
  background-size: 0.75rem 0.75rem;
  background-position: 0 0, 0.375rem 0.375rem;
  background-color: color-mix(in oklch, var(--ui-bg), var(--ui-fg) 8%);
}

.ui-color-loupe__fill {
  position: absolute;
  inset: 0.25rem;
  border-radius: inherit;
  overflow: hidden;
}

.ui-color-loupe__swatch.ui-color-swatch {
  inline-size: 100%;
  block-size: 100%;
  border-radius: inherit;
  min-inline-size: 0;
  min-block-size: 0;
}

.ui-color-loupe__tail {
  position: absolute;
  left: 50%;
  bottom: 0.1rem;
  inline-size: 1rem;
  block-size: 1rem;
  transform: translateX(-50%) rotate(45deg);
  border-right: 1px solid color-mix(in oklch, var(--ui-border), transparent 18%);
  border-bottom: 1px solid color-mix(in oklch, var(--ui-border), transparent 18%);
  border-radius: 0 0 0.22rem 0;
  background: color-mix(in oklch, var(--ui-bg), var(--ui-fg) 2%);
}

.ui-color-loupe--open,
.ui-color-loupe[data-open="true"],
.ui-color-loupe[data-state="open"] {
  opacity: 1;
  transform: translate(-50%, calc(-100% - var(--ui-space-xs))) scale(1);
  animation: ui-color-loupe-open 200ms cubic-bezier(0.18, 0.9, 0.3, 1.18);
}

.ui-color-loupe--disabled,
.ui-color-loupe[data-disabled="true"] {
  opacity: 0.4;
}

.ui-color-loupe--disabled .ui-color-loupe__bubble,
.ui-color-loupe[data-disabled="true"] .ui-color-loupe__bubble {
  border-color: color-mix(in oklch, var(--ui-border), transparent 34%);
}

.ui-color-loupe--custom-class,
.ui-color-loupe[data-custom-class="true"] {
  isolation: isolate;
}

@keyframes ui-color-loupe-open {
  from {
    opacity: 0;
    transform: translate(-50%, calc(-86% - var(--ui-space-2xs))) scale(0.76);
  }

  62% {
    opacity: 1;
    transform: translate(-50%, calc(-104% - var(--ui-space-xs))) scale(1.04);
  }

  to {
    opacity: 1;
    transform: translate(-50%, calc(-100% - var(--ui-space-xs))) scale(1);
  }
}
"#;
