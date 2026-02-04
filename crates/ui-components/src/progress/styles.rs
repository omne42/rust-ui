pub const CSS: &str = r#"
.ui-progress {
  display: inline-flex;
  width: 220px;
  flex-direction: column;
  gap: var(--ui-space-xs);
}

.ui-progress__track {
  position: relative;
  height: 10px;
  border-radius: 999px;
  background: var(--ui-bg);
  border: 1px solid var(--ui-border);
  overflow: hidden;
}

.ui-progress__indicator {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 100%;
  transform-origin: left center;
  transform: scaleX(var(--ui-progress-progress, 0));
  background: var(--ui-accent);
  border-radius: inherit;
  will-change: transform;
}

.ui-progress--indeterminate .ui-progress__indicator {
  width: 40%;
  transform: translateX(-60%);
  animation: ui-progress-indeterminate 1.2s ease-in-out infinite;
}

@media (prefers-reduced-motion: reduce) {
  .ui-progress--indeterminate .ui-progress__indicator {
    animation: none;
  }
}

@keyframes ui-progress-indeterminate {
  0% {
    transform: translateX(-60%);
  }
  50% {
    transform: translateX(80%);
  }
  100% {
    transform: translateX(220%);
  }
}
"#;
