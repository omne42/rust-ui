pub const CSS: &str = r#"
.ui-aspect-ratio {
  position: relative;
  display: block;
  width: 100%;
  overflow: hidden;
  border-radius: 0;
  background: var(--ui-bg-muted);
  aspect-ratio: 16 / 9;
}

.ui-aspect-ratio__inner {
  width: 100%;
  height: 100%;
}

.ui-aspect-ratio--ratio-square,
.ui-aspect-ratio[data-ratio="square"] {
  aspect-ratio: 1 / 1;
}

.ui-aspect-ratio--ratio-standard,
.ui-aspect-ratio[data-ratio="standard"] {
  aspect-ratio: 4 / 3;
}

.ui-aspect-ratio--ratio-video,
.ui-aspect-ratio[data-ratio="video"] {
  aspect-ratio: 16 / 9;
}

.ui-aspect-ratio--ratio-portrait,
.ui-aspect-ratio[data-ratio="portrait"] {
  aspect-ratio: 3 / 4;
}

.ui-aspect-ratio--ratio-ultra-wide,
.ui-aspect-ratio[data-ratio="ultra-wide"] {
  aspect-ratio: 21 / 9;
}

.ui-aspect-ratio--radius-none,
.ui-aspect-ratio[data-radius="none"] {
  border-radius: 0;
}

.ui-aspect-ratio--radius-sm,
.ui-aspect-ratio[data-radius="sm"] {
  border-radius: var(--ui-radius-sm);
}

.ui-aspect-ratio--radius-md,
.ui-aspect-ratio[data-radius="md"] {
  border-radius: var(--ui-radius-md);
}

.ui-aspect-ratio--radius-lg,
.ui-aspect-ratio[data-radius="lg"] {
  border-radius: var(--ui-radius-lg);
}

.ui-aspect-ratio--radius-full,
.ui-aspect-ratio[data-radius="full"] {
  border-radius: 999px;
}

.ui-aspect-ratio--bordered,
.ui-aspect-ratio[data-bordered="true"] {
  border: 1px solid var(--ui-border);
}

.ui-aspect-ratio--fill .ui-aspect-ratio__inner,
.ui-aspect-ratio[data-fill="true"] .ui-aspect-ratio__inner {
  display: block;
}

.ui-aspect-ratio--fill .ui-aspect-ratio__inner > *,
.ui-aspect-ratio[data-fill="true"] .ui-aspect-ratio__inner > * {
  width: 100%;
  height: 100%;
}

.ui-aspect-ratio--fill .ui-aspect-ratio__inner > img,
.ui-aspect-ratio[data-fill="true"] .ui-aspect-ratio__inner > img,
.ui-aspect-ratio--fill .ui-aspect-ratio__inner > video,
.ui-aspect-ratio[data-fill="true"] .ui-aspect-ratio__inner > video {
  object-fit: cover;
  display: block;
}

.ui-aspect-ratio--custom-class,
.ui-aspect-ratio[data-custom-class="true"] {
  --ui-aspect-ratio-has-custom-class: 1;
}
"#;
