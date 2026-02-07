pub const CSS: &str = r#"
.ui-ripple {
  position: absolute;
  inset: 0;
  border-radius: inherit;
  background: currentColor;
  opacity: 0;
  transform: scale(0);
  transform-origin: var(--ui-ripple-origin-x, 50%) var(--ui-ripple-origin-y, 50%);
  pointer-events: none;
  will-change: transform, opacity;
  --ui-ripple-duration-ms: 420;
}

.ui-ripple--state-animated,
.ui-ripple[data-state="animated"] {
  opacity: 0;
}

.ui-ripple--state-static,
.ui-ripple[data-state="static"] {
  opacity: 0;
  transform: scale(0);
}

.ui-ripple--boundary-bounded,
.ui-ripple[data-boundary="bounded"],
.ui-ripple[data-bounded="true"] {
  border-radius: inherit;
}

.ui-ripple--boundary-unbounded,
.ui-ripple[data-boundary="unbounded"],
.ui-ripple[data-unbounded="true"] {
  border-radius: 9999px;
  inset: -12%;
}

.ui-ripple--motion-custom,
.ui-ripple[data-motion-source="custom"] {
  filter: saturate(1.08);
}

.ui-ripple--custom-class,
.ui-ripple[data-custom-class="true"] {
  isolation: isolate;
}

.ui-ripple[data-class-source="custom"] {
  mix-blend-mode: normal;
}
"#;
