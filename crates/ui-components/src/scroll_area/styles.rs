pub const CSS: &str = r#"
.ui-scroll-area {
  --ui-scroll-area-motion-duration: 160ms;

  position: relative;
  border-radius: inherit;
  transition: opacity var(--ui-scroll-area-motion-duration) ease;
}

.ui-scroll-area__viewport {
  max-height: var(--ui-scroll-area-max-h, 280px);
  overflow-y: auto;
  overflow-x: hidden;
  overscroll-behavior: contain;
  scrollbar-gutter: stable both-edges;
  border-radius: inherit;
  outline: none;
}

.ui-scroll-area--vertical .ui-scroll-area__viewport,
.ui-scroll-area[data-orientation="vertical"] .ui-scroll-area__viewport {
  overflow-y: auto;
  overflow-x: hidden;
}

.ui-scroll-area--horizontal .ui-scroll-area__viewport,
.ui-scroll-area[data-orientation="horizontal"] .ui-scroll-area__viewport {
  overflow-y: hidden;
  overflow-x: auto;
  white-space: nowrap;
}

.ui-scroll-area--both .ui-scroll-area__viewport,
.ui-scroll-area[data-orientation="both"] .ui-scroll-area__viewport {
  overflow-y: auto;
  overflow-x: auto;
}

.ui-scroll-area--max-height-custom .ui-scroll-area__viewport,
.ui-scroll-area[data-max-height="custom"] .ui-scroll-area__viewport {
  max-height: var(--ui-scroll-area-max-h, 280px);
}

.ui-scroll-area--disabled,
.ui-scroll-area[data-disabled="true"] {
  opacity: 0.7;
}

.ui-scroll-area--disabled .ui-scroll-area__viewport,
.ui-scroll-area[data-disabled="true"] .ui-scroll-area__viewport {
  pointer-events: none;
}

.ui-scroll-area__viewport::-webkit-scrollbar {
  width: 10px;
  height: 10px;
}

.ui-scroll-area__viewport::-webkit-scrollbar-track {
  background: color-mix(in oklch, var(--ui-bg) 94%, var(--ui-fg) 6%);
  border-radius: 999px;
}

.ui-scroll-area__viewport::-webkit-scrollbar-thumb {
  background: color-mix(in oklch, var(--ui-fg) 28%, transparent);
  border-radius: 999px;
  border: 2px solid color-mix(in oklch, var(--ui-bg) 94%, var(--ui-fg) 6%);
}

.ui-scroll-area__viewport::-webkit-scrollbar-thumb:hover {
  background: color-mix(in oklch, var(--ui-fg) 40%, transparent);
}

.ui-scroll-area__viewport:focus-visible {
  box-shadow: inset 0 0 0 2px var(--ui-focus-ring);
}

@media (prefers-reduced-motion: reduce) {
  .ui-scroll-area {
    --ui-scroll-area-motion-duration: 1ms;
  }
}
"#;
