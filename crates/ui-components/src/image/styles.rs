pub const CSS: &str = r#"
.ui-image {
  position: relative;
  display: inline-block;
  overflow: hidden;
  border-radius: var(--ui-radius-lg);
  background: var(--ui-bg);
  border: 1px solid var(--ui-border);
  box-shadow: var(--ui-shadow-sm);

  --ui-image-zoom: 1;
}

.ui-image--radius-sm {
  border-radius: var(--ui-radius-sm);
}
.ui-image[data-radius="sm"] {
  border-radius: var(--ui-radius-sm);
}
.ui-image--radius-md {
  border-radius: var(--ui-radius-md);
}
.ui-image[data-radius="md"] {
  border-radius: var(--ui-radius-md);
}
.ui-image--radius-lg {
  border-radius: var(--ui-radius-lg);
}
.ui-image[data-radius="lg"] {
  border-radius: var(--ui-radius-lg);
}
.ui-image--radius-full {
  border-radius: 999px;
}
.ui-image[data-radius="full"] {
  border-radius: 999px;
}

.ui-image--shadow-none {
  box-shadow: none;
}
.ui-image[data-shadow="none"] {
  box-shadow: none;
}
.ui-image--shadow-sm {
  box-shadow: var(--ui-shadow-sm);
}
.ui-image[data-shadow="sm"] {
  box-shadow: var(--ui-shadow-sm);
}
.ui-image--shadow-md {
  box-shadow: var(--ui-shadow-md);
}
.ui-image[data-shadow="md"] {
  box-shadow: var(--ui-shadow-md);
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
  transform: scale(var(--ui-image-zoom, 1));
  will-change: transform;
}

.ui-image__blurred {
  position: absolute;
  inset: 0;
  filter: blur(14px);
  transform: scale(1.12);
  opacity: 0.45;
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
      color-mix(in oklch, var(--ui-fg) 10%, transparent),
      transparent
    ),
    var(--ui-bg-muted);
  background-size: 220% 100%;
  animation: ui-image-shimmer 1.3s ease-in-out infinite;
  z-index: 2;
}

.ui-image[data-state="loaded"] .ui-image__skeleton {
  display: none;
}

.ui-image[data-custom-motion="true"] .ui-image__img {
  transform: scale(var(--ui-image-zoom, 1));
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
    background-position: 120% 0;
  }
  100% {
    background-position: -120% 0;
  }
}
.ui-image[data-loaded="true"] .ui-image__skeleton {
  display: none;
}
"#;
