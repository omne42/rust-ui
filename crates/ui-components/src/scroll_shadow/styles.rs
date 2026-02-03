pub const CSS: &str = r#"
.ui-scroll-shadow {
  position: relative;
}

.ui-scroll-shadow__viewport {
  overflow: auto;
  max-height: var(--ui-scroll-shadow-max-h, 280px);
}

.ui-scroll-shadow::before,
.ui-scroll-shadow::after {
  content: "";
  position: absolute;
  left: 0;
  right: 0;
  height: 18px;
  pointer-events: none;
  opacity: 0;
  transition: opacity 120ms ease;
}

@media (prefers-reduced-motion: reduce) {
  .ui-scroll-shadow::before,
  .ui-scroll-shadow::after {
    transition: none;
  }
}

.ui-scroll-shadow::before {
  top: 0;
  background: linear-gradient(
    to bottom,
    color-mix(in oklch, var(--ui-bg) 96%, transparent),
    transparent
  );
}

.ui-scroll-shadow::after {
  bottom: 0;
  background: linear-gradient(
    to top,
    color-mix(in oklch, var(--ui-bg) 96%, transparent),
    transparent
  );
}

.ui-scroll-shadow[data-shadow-top="true"]::before {
  opacity: 1;
}

.ui-scroll-shadow[data-shadow-bottom="true"]::after {
  opacity: 1;
}
"#;
