pub const CSS: &str = r#"
.ui-overlays {
  display: contents;
  /* Token-consumption marker for contract tests. */
  --ui-overlays-open-current: var(--ui-overlays-open, 0);
  --ui-overlays-layer-fallback: var(--ui-overlay-layer, modal);
}

.ui-overlays--open,
.ui-overlays[data-state="open"] {
  --ui-overlays-open: 1;
}

.ui-overlays--closed,
.ui-overlays[data-state="closed"] {
  --ui-overlays-open: 0;
}

.ui-overlays--modal,
.ui-overlays[data-layer="modal"] {
  --ui-overlays-layer: modal;
}

.ui-overlays--non-modal,
.ui-overlays[data-layer="non-modal"] {
  --ui-overlays-layer: non-modal;
}

.ui-overlays--custom-id,
.ui-overlays[data-custom-id="true"],
.ui-overlays[data-id-source="custom"] {
  --ui-overlays-custom-id: 1;
}

.ui-overlays--custom-class,
.ui-overlays[data-custom-class="true"],
.ui-overlays[data-class-source="custom"] {
  --ui-overlays-custom-class: 1;
}
"#;
