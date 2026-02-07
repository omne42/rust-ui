pub const CSS: &str = r#"
.ui-share-button {
  display: inline-flex;
  min-width: 0;
}

.ui-share-button--state-ready,
.ui-share-button[data-state="ready"] {
  opacity: 1;
}

.ui-share-button--state-empty,
.ui-share-button[data-state="empty"] {
  opacity: 0.72;
}

.ui-share-button--custom-class,
.ui-share-button[data-custom-class="true"] {
  border-radius: inherit;
}

.ui-share-button__front,
.ui-share-button__back {
  display: inline-flex;
  width: 100%;
}

.ui-share-button__front > .ui-button {
  width: 100%;
}

.ui-share-button__front [data-slot="share-button-label"] {
  white-space: nowrap;
}

.ui-share-button--icon-prefix [data-slot="share-button-trigger-icon"],
.ui-share-button[data-icon="prefix"] [data-slot="share-button-trigger-icon"] {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-share-button--icon-none [data-slot="share-button-trigger-icon"],
.ui-share-button[data-icon="none"] [data-slot="share-button-trigger-icon"] {
  display: none;
}

.ui-share-button__back > .ui-button-group {
  width: 100%;
}

.ui-share-button__platforms {
  display: inline-flex;
  align-items: center;
  gap: 0;
}

.ui-share-button__platform {
  display: inline-flex;
}

.ui-share-button__icon {
  width: 16px;
  height: 16px;
}

.ui-share-button[data-platform="github"] .ui-button,
.ui-share-button__platform[data-platform="github"] .ui-button {
  color: var(--ui-fg);
}

.ui-share-button[data-platform="x"] .ui-button,
.ui-share-button__platform[data-platform="x"] .ui-button {
  color: var(--ui-fg);
}

.ui-share-button[data-platform="facebook"] .ui-button,
.ui-share-button__platform[data-platform="facebook"] .ui-button {
  color: var(--ui-fg);
}
"#;
