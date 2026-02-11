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
  opacity: 0.72;
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
  outline: 3px solid var(--ui-focus-ring);
  outline-offset: 3px;
  border-radius: var(--ui-radius-md);
}

.ui-preview-card__panel {
  position: fixed;
  top: var(--ui-preview-card-top, 0px);
  left: var(--ui-preview-card-left, 0px);
  width: min(380px, 92vw);
  min-width: max(280px, var(--ui-preview-card-anchor-width, 0px));
  border-radius: var(--ui-radius-lg);
  border: 1px solid var(--ui-border);
  background: var(--ui-bg);
  color: var(--ui-fg);
  box-shadow: var(--ui-shadow-md);
  z-index: 1000;

  opacity: var(--ui-preview-card-opacity, 0);
  transform: translateY(var(--ui-preview-card-y, 8px))
    scale(var(--ui-preview-card-scale, 0.98));
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
  background: var(--ui-bg-muted);
}

.ui-preview-card__body {
  display: flex;
  flex-direction: column;
  gap: var(--ui-space-xs);
  padding: var(--ui-space-md);
}

.ui-preview-card__title {
  font-size: 0.95rem;
  font-weight: 650;
  line-height: 1.3;
}

.ui-preview-card__description {
  color: var(--ui-fg-muted);
  font-size: 0.86rem;
  line-height: 1.45;
}

.ui-preview-card__meta {
  margin-top: var(--ui-space-2xs);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--ui-space-sm);
  color: var(--ui-fg-muted);
  font-size: 0.76rem;
}

.ui-preview-card__meta-link {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
"#;
