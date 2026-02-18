pub const CSS: &str = r#"
.ui-scroll-shadow {
  position: relative;
}

.ui-scroll-shadow--scrollable,
.ui-scroll-shadow[data-scrollable="true"] {
  isolation: isolate;
}

.ui-scroll-shadow__viewport {
  overflow: auto;
  max-height: var(--ui-scroll-shadow-max-h);
}

.ui-scroll-shadow--max-height-custom .ui-scroll-shadow__viewport,
.ui-scroll-shadow[data-max-height="custom"] .ui-scroll-shadow__viewport {
  max-height: var(--ui-scroll-shadow-max-h);
}

.ui-scroll-shadow::before,
.ui-scroll-shadow::after {
  content: "";
  position: absolute;
  left: 0;
  right: 0;
  height: var(--ui-space-md);
  pointer-events: none;
  opacity: 0;
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

.ui-scroll-shadow--shadow-top::before,
.ui-scroll-shadow[data-shadow-top="true"]::before,
.ui-scroll-shadow[data-state="top"]::before,
.ui-scroll-shadow[data-state="both"]::before {
  opacity: 1;
}

.ui-scroll-shadow--shadow-bottom::after,
.ui-scroll-shadow[data-shadow-bottom="true"]::after,
.ui-scroll-shadow[data-state="bottom"]::after,
.ui-scroll-shadow[data-state="both"]::after {
  opacity: 1;
}
"#;
