pub const CSS: &str = r#"
.ui-ripple {
  position: absolute;
  inset: 0;
  border-radius: inherit;
  background: currentColor;
  opacity: 0;
  transform: scale(0);
  transform-origin:
    var(--ui-ripple-origin-x, var(--ui-color-thumb-x-center, var(--ui-fallback-color-thumb-x-center)))
    var(--ui-ripple-origin-y, var(--ui-color-thumb-y-center, var(--ui-fallback-color-thumb-y-center)));
  pointer-events: none;
  will-change: transform, opacity;
  --ui-ripple-duration-ms: var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration));
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
  border-radius: var(--ui-radius-full, var(--ui-fallback-radius-full));
  inset: calc(var(--ui-overlay-viewport-inset, var(--ui-fallback-overlay-viewport-inset)) * -1);
}

.ui-ripple--motion-custom,
.ui-ripple[data-motion-source="custom"] {
  filter: saturate(var(--ui-image-blur-scale, var(--ui-fallback-image-blur-scale)));
}

.ui-ripple--custom-class,
.ui-ripple[data-custom-class="true"] {
  isolation: isolate;
}

.ui-ripple[data-class-source="custom"] {
  mix-blend-mode: normal;
}
"#;
