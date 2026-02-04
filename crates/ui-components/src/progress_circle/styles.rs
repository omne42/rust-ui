pub const CSS: &str = r#"
.ui-progress-circle {
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.ui-progress-circle__svg {
  display: block;
  transform: rotate(-90deg);
  transform-origin: 50% 50%;
}

.ui-progress-circle__track {
  color: var(--ui-border);
}

.ui-progress-circle__indicator {
  color: var(--ui-accent);
}

.ui-progress-circle--indeterminate .ui-progress-circle__svg {
  animation: ui-progress-circle-spin 1s linear infinite;
}

@media (prefers-reduced-motion: reduce) {
  .ui-progress-circle--indeterminate .ui-progress-circle__svg {
    animation: none;
  }
}

@keyframes ui-progress-circle-spin {
  to {
    transform: rotate(270deg);
  }
}
"#;
