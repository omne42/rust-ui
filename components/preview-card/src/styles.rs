pub const CSS: &str = r#"
.ui-preview-card {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-preview-card[data-state="open"],
.ui-preview-card[data-open="true"],
.ui-preview-card[data-state="closed"],
.ui-preview-card[data-closed="true"] {
  cursor: default;
}

.ui-preview-card[data-disabled="true"] {
  opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity, 0.72));
}

.ui-preview-card[data-content="media"] {
  --ui-preview-card-has-image: 1;
}

.ui-preview-card[data-content="text"] {
  --ui-preview-card-has-image: 0;
}

.ui-preview-card[data-class-source="custom"],
.ui-preview-card--custom-class {
  --ui-preview-card-class-source: custom;
}

.ui-preview-card[data-delay-source="custom"],
.ui-preview-card--custom-delay {
  --ui-preview-card-delay-source: custom;
}

.ui-preview-card[data-motion-source="custom"],
.ui-preview-card[data-custom-motion="true"],
.ui-preview-card--custom-motion {
  --ui-preview-card-custom-motion: 1;
}

.ui-preview-card[data-id-source="custom"],
.ui-preview-card--custom-id {
  --ui-preview-card-id-source: custom;
}

.ui-preview-card[data-title-source="custom"],
.ui-preview-card--custom-title {
  --ui-preview-card-title-source: custom;
}

.ui-preview-card[data-description-source="custom"],
.ui-preview-card--custom-description {
  --ui-preview-card-description-source: custom;
}

.ui-preview-card[data-url-source="custom"],
.ui-preview-card--custom-url {
  --ui-preview-card-url-source: custom;
}

.ui-preview-card__trigger {
  all: unset;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  -webkit-tap-highlight-color: transparent;
  cursor: pointer;
}

.ui-preview-card__trigger[data-state="trigger"] {
  --ui-preview-card-trigger: 1;
}

.ui-preview-card__trigger:focus-visible {
  outline: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width, 3px))
    solid var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(--ui-button-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset, 3px));
  border-radius: var(--ui-radius-md);
}

.ui-preview-card__panel {
  position: fixed;
  top: var(--ui-preview-card-top, var(--ui-fallback-min-inline-size-none));
  left: var(--ui-preview-card-left, var(--ui-fallback-min-inline-size-none));
  width: min(
    var(--ui-tooltip-max-width, var(--ui-fallback-tooltip-max-width, 380px)),
    calc(
      100vw - (var(--ui-overlay-viewport-inset, var(--ui-fallback-overlay-viewport-inset, 16px)) * 2)
    )
  );
  min-width: max(
    var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width, 280px)),
    var(--ui-preview-card-anchor-width, var(--ui-fallback-min-inline-size-none))
  );
  border-radius: var(--ui-radius-lg);
  border: var(--ui-border-width, var(--ui-fallback-border-width, 1px))
    solid var(--ui-border, var(--ui-fallback-border));
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));
  z-index: var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index, 1000));

  --ui-preview-card-opacity: 0;
  --ui-preview-card-scale: var(
    --ui-overlay-enter-scale,
    var(--ui-fallback-overlay-enter-scale)
  );
  --ui-preview-card-y: var(
    --ui-overlay-enter-offset-y,
    var(--ui-fallback-overlay-enter-offset-y)
  );

  opacity: var(--ui-preview-card-opacity);
  transform: translateY(var(--ui-preview-card-y)) scale(var(--ui-preview-card-scale));
  will-change: transform, opacity;
  overflow: hidden;
}

.ui-preview-card__panel[data-state="panel"] {
  --ui-preview-card-panel: 1;
}

.ui-preview-card__panel[data-placement="bottom-start"] {
  transform-origin: top left;
}

.ui-preview-card__panel[data-placement="bottom-end"] {
  transform-origin: top right;
}

.ui-preview-card__panel[data-placement="top-start"] {
  transform-origin: bottom left;
}

.ui-preview-card__panel[data-placement="top-end"] {
  transform-origin: bottom right;
}

.ui-preview-card__image {
  display: block;
  inline-size: 100%;
  block-size: auto;
  aspect-ratio: 16 / 9;
  object-fit: cover;
  background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
}

.ui-preview-card__body {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  padding: var(--ui-space-md, var(--ui-fallback-space-md));
}

.ui-preview-card__title {
  font-size: var(--ui-heading-h6-font-size, var(--ui-fallback-heading-h6-font-size, 14px));
  line-height: var(--ui-heading-h6-line-height, var(--ui-fallback-heading-h6-line-height, 20px));
  font-weight: 650;
}

.ui-preview-card__description {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150, 14px));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150, 20px));
}

.ui-preview-card__meta {
  margin-top: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100, 12px));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100, 16px));
}

.ui-preview-card__meta-link {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
"#;
