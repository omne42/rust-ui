pub const CSS: &str = r#"
.ui-flip-button {
  position: relative;
  display: inline-grid;
  grid-template-areas: \"stack\";
  perspective: 600px;
}

.ui-flip-button__face {
  grid-area: stack;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  backface-visibility: hidden;
}

.ui-flip-button__front {
  opacity: calc(1 - var(--ui-flip-progress, 0));
  pointer-events: auto;
}

.ui-flip-button__back {
  opacity: var(--ui-flip-progress, 0);
  pointer-events: none;
}

.ui-flip-button[data-active=\"true\"] .ui-flip-button__front {
  pointer-events: none;
}

.ui-flip-button[data-active=\"true\"] .ui-flip-button__back {
  pointer-events: auto;
}

.ui-flip-button[data-from=\"top\"] .ui-flip-button__front {
  transform: translateY(calc(var(--ui-flip-front-offset, 50%) * var(--ui-flip-progress, 0)))
    rotateX(calc(90deg * var(--ui-flip-progress, 0)));
}

.ui-flip-button[data-from=\"top\"] .ui-flip-button__back {
  transform: translateY(calc(var(--ui-flip-back-offset, -50%) * (1 - var(--ui-flip-progress, 0))))
    rotateX(calc(90deg * (1 - var(--ui-flip-progress, 0))));
}

.ui-flip-button[data-from=\"bottom\"] .ui-flip-button__front {
  transform: translateY(calc(var(--ui-flip-front-offset, -50%) * var(--ui-flip-progress, 0)))
    rotateX(calc(90deg * var(--ui-flip-progress, 0)));
}

.ui-flip-button[data-from=\"bottom\"] .ui-flip-button__back {
  transform: translateY(calc(var(--ui-flip-back-offset, 50%) * (1 - var(--ui-flip-progress, 0))))
    rotateX(calc(90deg * (1 - var(--ui-flip-progress, 0))));
}

.ui-flip-button[data-from=\"left\"] .ui-flip-button__front {
  transform: translateX(calc(var(--ui-flip-front-offset, 50%) * var(--ui-flip-progress, 0)))
    rotateY(calc(90deg * var(--ui-flip-progress, 0)));
}

.ui-flip-button[data-from=\"left\"] .ui-flip-button__back {
  transform: translateX(calc(var(--ui-flip-back-offset, -50%) * (1 - var(--ui-flip-progress, 0))))
    rotateY(calc(90deg * (1 - var(--ui-flip-progress, 0))));
}

.ui-flip-button[data-from=\"right\"] .ui-flip-button__front {
  transform: translateX(calc(var(--ui-flip-front-offset, -50%) * var(--ui-flip-progress, 0)))
    rotateY(calc(90deg * var(--ui-flip-progress, 0)));
}

.ui-flip-button[data-from=\"right\"] .ui-flip-button__back {
  transform: translateX(calc(var(--ui-flip-back-offset, 50%) * (1 - var(--ui-flip-progress, 0))))
    rotateY(calc(90deg * (1 - var(--ui-flip-progress, 0))));
}
"#;
