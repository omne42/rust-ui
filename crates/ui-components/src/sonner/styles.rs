pub const CSS: &str = r#"
.ui-sonner {
  --ui-sonner-offset: 16px;
  --ui-sonner-max-inline-width: 420px;
  pointer-events: none;
}

.ui-sonner[data-motion-source="custom"],
.ui-sonner[data-custom-motion="true"],
.ui-sonner--custom-motion {
  --ui-sonner-custom-motion: 1;
}

.ui-sonner[data-position-source="custom"],
.ui-sonner[data-custom-position="true"],
.ui-sonner--custom-position {
  --ui-sonner-custom-position: 1;
}

.ui-sonner[data-portal-source="custom"],
.ui-sonner[data-custom-portal="true"],
.ui-sonner--custom-portal {
  --ui-sonner-custom-portal: 1;
}

.ui-sonner[data-max-toasts-source="custom"],
.ui-sonner[data-custom-max-toasts="true"],
.ui-sonner--custom-max-toasts {
  --ui-sonner-custom-max-toasts: 1;
}

.ui-sonner[data-aria-source="custom"],
.ui-sonner[data-custom-aria="true"],
.ui-sonner--custom-aria {
  --ui-sonner-custom-aria: 1;
}

.ui-sonner[data-class-source="custom"],
.ui-sonner[data-custom-class="true"],
.ui-sonner--custom-class {
  --ui-sonner-custom-class: 1;
}

.ui-sonner[data-store-source="provided"] {
  --ui-sonner-store-source: 1;
}

.ui-sonner[data-store-source="context"] {
  --ui-sonner-store-source: 2;
}

.ui-sonner[data-store-source="local"] {
  --ui-sonner-store-source: 3;
}

.ui-sonner[data-state="inline"],
.ui-sonner[data-portal="false"] {
  width: 100%;
  display: flex;
}

.ui-sonner[data-state="inline"][data-position$="left"],
.ui-sonner[data-portal="false"][data-position$="left"] {
  justify-content: flex-start;
}

.ui-sonner[data-state="inline"][data-position$="center"],
.ui-sonner[data-portal="false"][data-position$="center"] {
  justify-content: center;
}

.ui-sonner[data-state="inline"][data-position$="right"],
.ui-sonner[data-portal="false"][data-position$="right"] {
  justify-content: flex-end;
}

.ui-sonner[data-queue="single"] .ui-sonner__viewport.ui-toast-viewport {
  max-width: min(100%, 360px);
}

.ui-sonner[data-queue="bounded"] .ui-sonner__viewport.ui-toast-viewport,
.ui-sonner[data-queue="extended"] .ui-sonner__viewport.ui-toast-viewport {
  max-width: min(100%, var(--ui-sonner-max-inline-width));
}

.ui-sonner__viewport.ui-toast-viewport {
  top: auto;
  right: auto;
  bottom: auto;
  left: auto;
}

.ui-sonner__viewport--top-left.ui-toast-viewport {
  top: var(--ui-sonner-offset);
  left: var(--ui-sonner-offset);
}

.ui-sonner__viewport--top-center.ui-toast-viewport {
  top: var(--ui-sonner-offset);
  left: 50%;
  transform: translateX(-50%);
}

.ui-sonner__viewport--top-right.ui-toast-viewport {
  top: var(--ui-sonner-offset);
  right: var(--ui-sonner-offset);
}

.ui-sonner__viewport--bottom-left.ui-toast-viewport {
  bottom: var(--ui-sonner-offset);
  left: var(--ui-sonner-offset);
}

.ui-sonner__viewport--bottom-center.ui-toast-viewport {
  bottom: var(--ui-sonner-offset);
  left: 50%;
  transform: translateX(-50%);
}

.ui-sonner__viewport--bottom-right.ui-toast-viewport {
  right: var(--ui-sonner-offset);
  bottom: var(--ui-sonner-offset);
}

.ui-sonner[data-state="inline"] .ui-sonner__viewport.ui-toast-viewport,
.ui-sonner[data-portal="false"] .ui-sonner__viewport.ui-toast-viewport,
.ui-sonner__viewport--inline.ui-toast-viewport {
  position: relative;
  transform: none;
}
"#;
