pub const CSS: &str = r#"
.ui-skeleton {
  display: block;
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
  background: color-mix(
    in oklch,
    var(--ui-border, var(--ui-fallback-border)) 55%,
    var(--ui-bg, var(--ui-fallback-bg))
  );
  position: relative;
  overflow: hidden;
}

.ui-skeleton--variant-rect,
.ui-skeleton[data-variant="rect"] {
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
}

.ui-skeleton--variant-circle,
.ui-skeleton[data-variant="circle"] {
  border-radius: var(--ui-radius-full, var(--ui-fallback-radius-full));
}

.ui-skeleton[data-variant-source="prop"],
.ui-skeleton[data-shimmer-source="prop"] {
  --ui-skeleton-prop-source: 1;
}

.ui-skeleton--shimmer::after,
.ui-skeleton[data-shimmer="true"]::after {
  content: "";
  position: absolute;
  inset: 0;
  background: linear-gradient(
    100deg,
    transparent 0%,
    color-mix(
      in oklch,
      var(--ui-bg, var(--ui-fallback-bg)) 35%,
      transparent
    ) 50%,
    transparent 100%
  );
  transform: translateX(-100%);
  animation: ui-skeleton-shimmer
    var(--ui-image-skeleton-duration, var(--ui-fallback-image-skeleton-duration))
    ease-in-out infinite;
}

.ui-skeleton--still,
.ui-skeleton[data-still="true"] {
  animation: none;
}

@keyframes ui-skeleton-shimmer {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}

@media (prefers-reduced-motion: reduce) {
  .ui-skeleton--shimmer::after,
  .ui-skeleton[data-shimmer="true"]::after {
    animation: none;
  }
}
"#;
