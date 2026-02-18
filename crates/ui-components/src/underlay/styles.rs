pub const CSS: &str = r#"
.ui-underlay {
  position: fixed;
  inset: 0;
  pointer-events: none;
  opacity: 0;
  visibility: hidden;
  background: color-mix(
    in oklch,
    var(--ui-fg),
    transparent var(--ui-underlay-scrim-alpha, 56%)
  );
  backdrop-filter: blur(var(--ui-underlay-backdrop-blur, 1px));
  z-index: var(--ui-underlay-z-index, var(--ui-overlay-z-index, 20));
  transition:
    opacity var(--ui-underlay-runtime-duration, var(--ui-underlay-transition-duration, 220ms)) var(--ui-underlay-transition-easing, cubic-bezier(0.22, 1, 0.36, 1)),
    visibility var(--ui-underlay-runtime-visibility-duration, var(--ui-underlay-visibility-duration, 220ms)) linear;
}

.ui-underlay--open,
.ui-underlay[data-open="true"],
.ui-underlay[data-state="open"] {
  pointer-events: auto;
  opacity: 1;
  visibility: visible;
}

.ui-underlay--transparent,
.ui-underlay[data-transparent="true"],
.ui-underlay[data-tone="transparent"] {
  background: transparent;
  backdrop-filter: none;
}

.ui-underlay--interactive,
.ui-underlay[data-interactive="true"],
.ui-underlay[data-close-mode="interactive"] {
  cursor: pointer;
}

.ui-underlay--disabled,
.ui-underlay[data-disabled="true"],
.ui-underlay[data-state="disabled"] {
  pointer-events: none;
  opacity: 0;
  visibility: hidden;
}

.ui-underlay[data-transparent-source="custom"],
.ui-underlay[data-custom-transparent="true"] {
  --ui-underlay-custom-transparent: 1;
}

.ui-underlay[data-disabled-source="custom"],
.ui-underlay[data-custom-disabled="true"] {
  --ui-underlay-custom-disabled: 1;
}

.ui-underlay[data-close-source="custom"],
.ui-underlay[data-custom-close="true"] {
  --ui-underlay-custom-close: 1;
}

.ui-underlay--custom-class,
.ui-underlay[data-custom-class="true"],
.ui-underlay[data-class-source="custom"] {
  isolation: isolate;
}
"#;
