pub const CSS: &str = r#"
.ui-underlay {
  position: fixed;
  inset: 0;
  pointer-events: none;
  opacity: 0;
  visibility: hidden;
  background: color-mix(in oklch, var(--ui-fg), transparent 56%);
  backdrop-filter: blur(1px);
  z-index: 20;
  transition:
    opacity 220ms cubic-bezier(0.22, 1, 0.36, 1),
    visibility 220ms linear;
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

.ui-underlay--custom-class,
.ui-underlay[data-custom-class="true"] {
  isolation: isolate;
}
"#;
