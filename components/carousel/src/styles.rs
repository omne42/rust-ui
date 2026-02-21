pub const CSS: &str = r#"
.ui-carousel {
  --ui-carousel-gap: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-carousel-viewport-min-block-size: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 4
  );
  --ui-carousel-slide-gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  --ui-carousel-slide-pad: var(--ui-space-lg, var(--ui-fallback-space-lg));
  --ui-carousel-button-min-block-size: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.8
  );
  --ui-carousel-button-min-inline-size: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 2.2
  );
  --ui-carousel-button-inline-pad: var(--ui-space-sm, var(--ui-fallback-space-sm));
  --ui-carousel-indicators-gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  --ui-carousel-indicators-pad: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  --ui-carousel-indicator-size: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.7
  );
  --ui-carousel-indicator-dot-size: calc(
    var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 0.2
  );
  --ui-carousel-outline-width: calc(
    var(--ui-border-width, var(--ui-fallback-border-width)) * 3
  );
  --ui-carousel-outline-offset: calc(
    var(--ui-border-width, var(--ui-fallback-border-width)) * 2
  );
  --ui-carousel-border-width: var(--ui-border-width, var(--ui-fallback-border-width));
  display: grid;
  gap: var(--ui-carousel-gap);
  width: 100%;
  max-width: 100%;
}

.ui-carousel--empty,
.ui-carousel[data-state="empty"],
.ui-carousel[data-items="empty"] {
  --ui-carousel-empty: 1;
}

.ui-carousel--has-items,
.ui-carousel[data-items="populated"] {
  --ui-carousel-empty: 0;
}

.ui-carousel--selected,
.ui-carousel[data-state="selected"] {
  --ui-carousel-selected: 1;
}

.ui-carousel--focused,
.ui-carousel[data-state="focused"] {
  --ui-carousel-focused: 1;
}

.ui-carousel--loop,
.ui-carousel[data-navigation-mode="loop"],
.ui-carousel[data-loop="true"] {
  --ui-carousel-loop: 1;
}

.ui-carousel--bounded,
.ui-carousel[data-navigation-mode="bounded"],
.ui-carousel[data-bounded="true"] {
  --ui-carousel-loop: 0;
}

.ui-carousel--controlled,
.ui-carousel[data-selection-mode="controlled"] {
  --ui-carousel-controlled: 1;
}

.ui-carousel--uncontrolled,
.ui-carousel[data-selection-mode="uncontrolled"] {
  --ui-carousel-controlled: 0;
}

.ui-carousel--horizontal,
.ui-carousel[data-orientation="horizontal"] {
  --ui-carousel-orientation-vertical: 0;
}

.ui-carousel--vertical,
.ui-carousel[data-orientation="vertical"] {
  --ui-carousel-orientation-vertical: 1;
}

.ui-carousel--custom-motion,
.ui-carousel[data-motion-source="custom"],
.ui-carousel[data-custom-motion="true"] {
  --ui-carousel-custom-motion: 1;
}

.ui-carousel[data-id-source="custom"],
.ui-carousel[data-custom-id="true"],
.ui-carousel--custom-id {
  --ui-carousel-custom-id: 1;
}

.ui-carousel[data-aria-label-source="custom"],
.ui-carousel[data-custom-aria-label="true"],
.ui-carousel--custom-aria-label {
  --ui-carousel-custom-aria-label: 1;
}

.ui-carousel[data-class-source="custom"],
.ui-carousel[data-custom-class="true"],
.ui-carousel--custom-class {
  --ui-carousel-custom-class: 1;
}

.ui-carousel[data-orientation-source="custom"],
.ui-carousel[data-custom-orientation="true"],
.ui-carousel--custom-orientation {
  --ui-carousel-custom-orientation: 1;
}

.ui-carousel[data-loop-navigation-source="custom"],
.ui-carousel[data-custom-loop-navigation="true"],
.ui-carousel--custom-loop-navigation {
  --ui-carousel-custom-loop-navigation: 1;
}

.ui-carousel[data-selected-index-source="custom"],
.ui-carousel[data-custom-selected-index="true"],
.ui-carousel--custom-selected-index {
  --ui-carousel-custom-selected-index: 1;
}

.ui-carousel[data-default-selected-index-source="custom"],
.ui-carousel[data-custom-default-selected-index="true"],
.ui-carousel--custom-default-selected-index {
  --ui-carousel-custom-default-selected-index: 1;
}

.ui-carousel[data-selected-index-change-source="custom"],
.ui-carousel[data-custom-selected-index-change="true"],
.ui-carousel--custom-selected-index-change {
  --ui-carousel-custom-selected-index-change: 1;
}

.ui-carousel__viewport {
  position: relative;
  overflow: hidden;
  min-block-size: var(--ui-carousel-viewport-min-block-size);
  border: var(--ui-carousel-border-width) solid var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  background: color-mix(
    in oklab,
    var(--ui-bg, var(--ui-fallback-bg)) 92%,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 8%
  );
}

.ui-carousel__slide {
  display: none;
  grid-template-rows: auto auto;
  gap: var(--ui-carousel-slide-gap);
  min-block-size: var(--ui-carousel-viewport-min-block-size);
  padding: var(--ui-carousel-slide-pad);
}

.ui-carousel__slide[data-selected="true"],
.ui-carousel__slide[data-state="selected"] {
  display: grid;
}

.ui-carousel__slide[data-disabled="true"],
.ui-carousel__slide[data-state="disabled"] {
  opacity: 0.56;
}

.ui-carousel__title {
  margin: 0;
  font-size: var(--ui-font-size-lg, var(--ui-fallback-font-size-150));
  font-weight: var(--ui-font-weight-semibold, 600);
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-carousel__description {
  margin: 0;
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
  font-size: var(--ui-font-size-sm, var(--ui-fallback-font-size-100));
}

.ui-carousel__controls {
  display: inline-flex;
  align-items: center;
  gap: var(--ui-carousel-slide-gap);
}

.ui-carousel__button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-block-size: var(--ui-carousel-button-min-block-size);
  min-inline-size: var(--ui-carousel-button-min-inline-size);
  padding: 0 var(--ui-carousel-button-inline-pad);
  border: var(--ui-carousel-border-width) solid var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  font: inherit;
  cursor: pointer;
}

.ui-carousel__button:focus-visible {
  outline: var(--ui-carousel-outline-width) solid
    var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(--ui-carousel-outline-offset);
}

.ui-carousel__button:disabled {
  opacity: 0.56;
  cursor: not-allowed;
}

.ui-carousel__indicators {
  position: relative;
  display: inline-flex;
  align-items: center;
  gap: var(--ui-carousel-indicators-gap);
  padding: var(--ui-carousel-indicators-pad);
}

.ui-carousel__indicator {
  position: relative;
  z-index: 1;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: var(--ui-carousel-indicator-size);
  height: var(--ui-carousel-indicator-size);
  border: 0;
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  background: transparent;
  cursor: pointer;
}

.ui-carousel__indicator-dot {
  width: var(--ui-carousel-indicator-dot-size);
  height: var(--ui-carousel-indicator-dot-size);
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  background: color-mix(
    in oklab,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 72%,
    transparent 28%
  );
}

.ui-carousel__indicator[data-selected="true"] .ui-carousel__indicator-dot,
.ui-carousel__indicator[data-state="selected"] .ui-carousel__indicator-dot {
  background: var(--ui-accent-fg, var(--ui-fallback-accent-fg));
}

.ui-carousel__indicator:focus-visible {
  outline: var(--ui-carousel-outline-width) solid
    var(--ui-focus-ring, var(--ui-fallback-focus-ring));
  outline-offset: var(--ui-carousel-outline-offset);
}

.ui-carousel__indicator:disabled,
.ui-carousel__indicator[data-disabled="true"],
.ui-carousel__indicator[data-state="disabled"] {
  opacity: 0.56;
  cursor: not-allowed;
}

.ui-carousel--vertical .ui-carousel__controls,
.ui-carousel[data-orientation="vertical"] .ui-carousel__controls {
  justify-content: flex-start;
}

.ui-carousel--empty .ui-carousel__viewport,
.ui-carousel[data-state="empty"] .ui-carousel__viewport {
  border-color: color-mix(
    in oklab,
    var(--ui-border, var(--ui-fallback-border)) 72%,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 28%
  );
}

.ui-carousel--selected .ui-carousel__viewport,
.ui-carousel[data-state="selected"] .ui-carousel__viewport {
  box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));
}
"#;
