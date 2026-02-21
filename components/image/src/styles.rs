pub const CSS: &str = r#"
.ui-image {
  position: relative;
  display: inline-block;
  overflow: hidden;
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  background: var(--ui-bg, var(--ui-fallback-bg));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));

  --ui-image-zoom: var(
    --ui-image-zoom-initial,
    var(--ui-fallback-image-zoom-initial)
  );
}

.ui-image--radius-sm {
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
}
.ui-image[data-radius="sm"] {
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
}
.ui-image--radius-md {
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
}
.ui-image[data-radius="md"] {
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
}
.ui-image--radius-lg {
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
}
.ui-image[data-radius="lg"] {
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
}
.ui-image--radius-full {
  border-radius: var(--ui-radius-full, var(--ui-fallback-radius-full));
}
.ui-image[data-radius="full"] {
  border-radius: var(--ui-radius-full, var(--ui-fallback-radius-full));
}

.ui-image--shadow-none {
  box-shadow: none;
}
.ui-image[data-shadow="none"] {
  box-shadow: none;
}
.ui-image--shadow-sm {
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
}
.ui-image[data-shadow="sm"] {
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
}
.ui-image--shadow-md {
  box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));
}
.ui-image[data-shadow="md"] {
  box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));
}

.ui-image__img,
.ui-image__fallback,
.ui-image__blurred {
  width: 100%;
  height: 100%;
  display: block;
  object-fit: cover;
}

.ui-image__img {
  transform: scale(var(--ui-image-zoom, var(--ui-fallback-image-zoom-initial)));
  will-change: transform;
}

.ui-image__blurred {
  position: absolute;
  inset: 0;
  filter: blur(var(--ui-image-blur, var(--ui-fallback-image-blur)));
  transform: scale(var(--ui-image-blur-scale, var(--ui-fallback-image-blur-scale)));
  opacity: var(--ui-image-blur-opacity, var(--ui-fallback-image-blur-opacity));
  z-index: 0;
}

.ui-image__fallback {
  position: absolute;
  inset: 0;
  z-index: 0;
}

.ui-image__skeleton {
  position: absolute;
  inset: 0;
  background: linear-gradient(
      90deg,
      transparent,
      color-mix(
        in oklch,
        var(--ui-fg, var(--ui-fallback-fg))
          var(--ui-image-skeleton-fg-mix, var(--ui-fallback-image-skeleton-fg-mix)),
        transparent
      ),
      transparent
    ),
    var(--ui-bg-muted, var(--ui-fallback-bg-muted));
  background-size: var(
    --ui-image-skeleton-bg-size,
    var(--ui-fallback-image-skeleton-bg-size)
  );
  animation: ui-image-shimmer
    var(--ui-image-skeleton-duration, var(--ui-fallback-image-skeleton-duration))
    ease-in-out infinite;
  z-index: 2;
}

.ui-image[data-state="loaded"] .ui-image__skeleton {
  display: none;
}

.ui-image[data-custom-motion="true"] .ui-image__img {
  transform: scale(var(--ui-image-zoom, var(--ui-fallback-image-zoom-initial)));
}

@media (prefers-reduced-motion: reduce) {
  .ui-image__img {
    transform: none;
    will-change: auto;
  }

  .ui-image__skeleton {
    animation: none;
  }
}

@keyframes ui-image-shimmer {
  0% {
    background-position: var(
        --ui-image-shimmer-start,
        var(--ui-fallback-image-shimmer-start)
      )
      0;
  }
  100% {
    background-position: var(--ui-image-shimmer-end, var(--ui-fallback-image-shimmer-end))
      0;
  }
}
.ui-image[data-loaded="true"] .ui-image__skeleton {
  display: none;
}
"#;
