pub const CSS: &str = r#"
.ui-skeleton-group {
  display: grid;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  min-width: 0;
}

.ui-skeleton-group--layout-vertical,
.ui-skeleton-group[data-layout="vertical"] {
  grid-template-columns: minmax(0, 1fr);
}

.ui-skeleton-group--layout-horizontal,
.ui-skeleton-group[data-layout="horizontal"] {
  grid-template-columns: repeat(
    auto-fit,
    minmax(calc(var(--ui-space-xl, var(--ui-fallback-space-xl)) * 6), 1fr)
  );
  align-items: center;
}

.ui-skeleton-group--density-compact,
.ui-skeleton-group[data-density="compact"] {
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
}

.ui-skeleton-group--density-comfortable,
.ui-skeleton-group[data-density="comfortable"] {
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
}

.ui-skeleton-group[data-loading-source="prop"],
.ui-skeleton-group[data-skeleton-only-source="prop"],
.ui-skeleton-group[data-variant-source="prop"],
.ui-skeleton-group[data-layout-source="prop"],
.ui-skeleton-group[data-density-source="prop"] {
  --ui-skeleton-group-prop-source: 1;
}

.ui-skeleton-group--variant-pulse .ui-skeleton,
.ui-skeleton-group[data-variant="pulse"] .ui-skeleton {
  animation: ui-skeleton-group-pulse
    var(--ui-image-skeleton-duration, var(--ui-fallback-image-skeleton-duration))
    ease-in-out infinite alternate;
}

.ui-skeleton-group--variant-pulse .ui-skeleton::after,
.ui-skeleton-group[data-variant="pulse"] .ui-skeleton::after,
.ui-skeleton-group--variant-none .ui-skeleton::after,
.ui-skeleton-group[data-variant="none"] .ui-skeleton::after {
  display: none;
}

.ui-skeleton-group--variant-none .ui-skeleton,
.ui-skeleton-group[data-variant="none"] .ui-skeleton {
  animation: none;
}

.ui-skeleton-group--loaded,
.ui-skeleton-group[data-state="loaded"] {
  opacity: 1;
}

.ui-skeleton-group--custom-class,
.ui-skeleton-group[data-custom-class="true"] {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(
      in oklab,
      var(--ui-accent, var(--ui-fallback-accent)) 24%,
      transparent
    );
  outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

@keyframes ui-skeleton-group-pulse {
  0% {
    opacity: 0.58;
  }
  100% {
    opacity: 1;
  }
}

@media (prefers-reduced-motion: reduce) {
  .ui-skeleton-group--variant-pulse .ui-skeleton,
  .ui-skeleton-group[data-variant="pulse"] .ui-skeleton {
    animation: none;
  }
}
"#;
