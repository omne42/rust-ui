pub const CSS: &str = r#"
.ui-preview-link-card {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-preview-link-card[data-state="open"],
.ui-preview-link-card[data-open="true"],
.ui-preview-link-card[data-state="closed"],
.ui-preview-link-card[data-closed="true"] {
  cursor: default;
}

.ui-preview-link-card[data-disabled="true"] {
  opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity, 0.72));
}

.ui-preview-link-card[data-content="media"] {
  --ui-preview-link-card-has-image: 1;
}

.ui-preview-link-card[data-content="text"] {
  --ui-preview-link-card-has-image: 0;
}

.ui-preview-link-card[data-class-source="custom"],
.ui-preview-link-card--custom-class {
  --ui-preview-link-card-class-source: custom;
}

.ui-preview-link-card[data-delay-source="custom"],
.ui-preview-link-card--custom-delay {
  --ui-preview-link-card-delay-source: custom;
}

.ui-preview-link-card[data-motion-source="custom"],
.ui-preview-link-card[data-custom-motion="true"],
.ui-preview-link-card--custom-motion {
  --ui-preview-link-card-custom-motion: 1;
}

.ui-preview-link-card[data-id-source="custom"],
.ui-preview-link-card--custom-id {
  --ui-preview-link-card-id-source: custom;
}

.ui-preview-link-card[data-title-source="custom"],
.ui-preview-link-card--custom-title {
  --ui-preview-link-card-title-source: custom;
}

.ui-preview-link-card[data-description-source="custom"],
.ui-preview-link-card--custom-description {
  --ui-preview-link-card-description-source: custom;
}

.ui-preview-link-card[data-url-source="custom"],
.ui-preview-link-card--custom-url {
  --ui-preview-link-card-url-source: custom;
}

.ui-preview-link-card__trigger {
  all: unset;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  -webkit-tap-highlight-color: transparent;
  cursor: pointer;
}

.ui-preview-link-card__trigger[data-state="trigger"] {
  --ui-preview-link-card-trigger: 1;
}

.ui-preview-link-card__trigger:focus-visible {
  outline: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width, 3px))
    solid var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(--ui-button-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset, 3px));
  border-radius: var(--ui-radius-md);
}

.ui-preview-link-card__panel {
  position: fixed;
  top: var(--ui-preview-link-card-top, 0px);
  left: var(--ui-preview-link-card-left, 0px);
  width: min(
    var(--ui-tooltip-max-width, var(--ui-fallback-tooltip-max-width, 380px)),
    calc(
      100vw - (var(--ui-overlay-viewport-inset, var(--ui-fallback-overlay-viewport-inset, 16px)) * 2)
    )
  );
  min-width: max(
    var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width, 280px)),
    var(--ui-preview-link-card-anchor-width, 0px)
  );
  border-radius: var(--ui-radius-lg);
  border: var(--ui-border-width, var(--ui-fallback-border-width, 1px))
    solid var(--ui-border, var(--ui-fallback-border));
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));
  z-index: var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index, 1000));

  opacity: var(--ui-preview-link-card-opacity, 0);
  transform: translateY(
      var(
        --ui-preview-link-card-y,
        var(--ui-overlay-enter-offset-y, var(--ui-fallback-overlay-enter-offset-y, 8px))
      )
    )
    scale(
      var(
        --ui-preview-link-card-scale,
        var(--ui-overlay-enter-scale, var(--ui-fallback-overlay-enter-scale, 0.98))
      )
    );
  will-change: transform, opacity;
  overflow: hidden;
}

.ui-preview-link-card__panel[data-state="panel"] {
  --ui-preview-link-card-panel: 1;
}

.ui-preview-link-card__panel[data-placement="bottom-start"] {
  transform-origin: top left;
}

.ui-preview-link-card__panel[data-placement="bottom-end"] {
  transform-origin: top right;
}

.ui-preview-link-card__panel[data-placement="top-start"] {
  transform-origin: bottom left;
}

.ui-preview-link-card__panel[data-placement="top-end"] {
  transform-origin: bottom right;
}

.ui-preview-link-card__image {
  display: block;
  inline-size: 100%;
  block-size: auto;
  aspect-ratio: 16 / 9;
  object-fit: cover;
  background: var(--ui-bg-muted, var(--ui-fallback-bg-muted));
}

.ui-preview-link-card__body {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
  padding: var(--ui-space-md);
}

.ui-preview-link-card__title {
  font-size: var(--ui-heading-h6-font-size, var(--ui-fallback-heading-h6-font-size, 14px));
  line-height: var(--ui-heading-h6-line-height, var(--ui-fallback-heading-h6-line-height, 20px));
  font-weight: 650;
}

.ui-preview-link-card__description {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150, 14px));
  line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150, 20px));
}

.ui-preview-link-card__meta {
  margin-top: var(--ui-space-2xs);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--ui-space-sm);
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100, 12px));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100, 16px));
}

.ui-preview-link-card__meta-link {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
"#;
