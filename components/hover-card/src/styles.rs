pub const CSS: &str = r#"
.ui-hover-card {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-hover-card[data-state="open"],
.ui-hover-card[data-open="true"],
.ui-hover-card[data-state="closed"],
.ui-hover-card[data-closed="true"] {
  cursor: default;
}

.ui-hover-card[data-disabled="true"] {
  opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity));
}

.ui-hover-card[data-class-source="custom"],
.ui-hover-card--custom-class {
  --ui-hover-card-class-source: custom;
}

.ui-hover-card[data-motion-source="custom"],
.ui-hover-card[data-custom-motion="true"],
.ui-hover-card--custom-motion {
  --ui-hover-card-custom-motion: 1;
}

.ui-hover-card[data-delay-source="custom"],
.ui-hover-card--custom-delay {
  --ui-hover-card-delay-source: custom;
}

.ui-hover-card[data-id-source="custom"],
.ui-hover-card--custom-id {
  --ui-hover-card-id-source: custom;
}

.ui-hover-card__trigger {
  all: unset;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  -webkit-tap-highlight-color: transparent;
  cursor: default;
}

.ui-hover-card__trigger[data-state="trigger"] {
  --ui-hover-card-trigger: 1;
}

.ui-hover-card__trigger:focus-visible {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  border-radius: var(--ui-radius-md, var(--ui-fallback-radius-md));
}

.ui-hover-card__panel {
  position: fixed;
  top: var(--ui-hover-card-top, var(--ui-fallback-min-inline-size-none));
  left: var(--ui-hover-card-left, var(--ui-fallback-min-inline-size-none));
  min-width: max(
    var(
      --ui-overlay-panel-min-width,
      var(--ui-fallback-overlay-panel-min-width)
    ),
    var(--ui-hover-card-anchor-width, var(--ui-fallback-min-inline-size-none))
  );
  max-width: calc(
    100vw
      - (
          var(
              --ui-overlay-viewport-inset,
              var(--ui-fallback-overlay-viewport-inset)
            ) * 2
        )
  );
  padding: var(--ui-space-md, var(--ui-fallback-space-md));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    var(--ui-border, var(--ui-fallback-border));
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));
  z-index: var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index));

  opacity: var(--ui-hover-card-opacity, 0);
  transform: translateY(
      var(
        --ui-hover-card-y,
        var(
          --ui-overlay-enter-offset-y,
          var(--ui-fallback-overlay-enter-offset-y)
        )
      )
    )
    scale(
      var(
        --ui-hover-card-scale,
        var(--ui-overlay-enter-scale, var(--ui-fallback-overlay-enter-scale))
      )
    );
  will-change: transform, opacity;
}

.ui-hover-card__panel[data-state="panel"] {
  --ui-hover-card-panel: 1;
}

.ui-hover-card__panel[data-placement="bottom-start"] {
  transform-origin: top left;
}

.ui-hover-card__panel[data-placement="bottom-end"] {
  transform-origin: top right;
}

.ui-hover-card__panel[data-placement="top-start"] {
  transform-origin: bottom left;
}

.ui-hover-card__panel[data-placement="top-end"] {
  transform-origin: bottom right;
}
"#;
