pub const CSS: &str = r#"
.ui-circular-progress {
  display: inline-block;
  width: var(--ui-cp-size, 20px);
  height: var(--ui-cp-size, 20px);
  box-sizing: border-box;
  border-radius: 9999px;
  border: var(--ui-cp-thickness, 2px) solid var(--ui-border);
  border-top-color: var(--ui-accent);

  animation: ui-circular-progress-spin 0.9s linear infinite;
}

@keyframes ui-circular-progress-spin {
  to {
    transform: rotate(360deg);
  }
}

@media (prefers-reduced-motion: reduce) {
  .ui-circular-progress {
    animation: none;
  }
}
"#;
