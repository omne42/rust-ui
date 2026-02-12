pub const CSS: &str = r#"
.ui-toaster {
  --ui-toaster-max-inline-width: 420px;
  pointer-events: none;
}

.ui-toaster[data-motion-source="custom"],
.ui-toaster[data-custom-motion="true"],
.ui-toaster--custom-motion {
  --ui-toaster-custom-motion: 1;
}

.ui-toaster[data-position-source="custom"],
.ui-toaster[data-custom-position="true"],
.ui-toaster--custom-position {
  --ui-toaster-custom-position: 1;
}

.ui-toaster[data-portal-source="custom"],
.ui-toaster[data-custom-portal="true"],
.ui-toaster--custom-portal {
  --ui-toaster-custom-portal: 1;
}

.ui-toaster[data-max-toasts-source="custom"],
.ui-toaster[data-custom-max-toasts="true"],
.ui-toaster--custom-max-toasts {
  --ui-toaster-custom-max-toasts: 1;
}

.ui-toaster[data-aria-source="custom"],
.ui-toaster[data-custom-aria="true"],
.ui-toaster--custom-aria {
  --ui-toaster-custom-aria: 1;
}

.ui-toaster[data-class-source="custom"],
.ui-toaster[data-custom-class="true"],
.ui-toaster--custom-class {
  --ui-toaster-custom-class: 1;
}

.ui-toaster[data-store-source="provided"] {
  --ui-toaster-store-source: 1;
}

.ui-toaster[data-store-source="context"] {
  --ui-toaster-store-source: 2;
}

.ui-toaster[data-store-source="local"] {
  --ui-toaster-store-source: 3;
}

.ui-toaster[data-state="inline"],
.ui-toaster[data-portal="false"] {
  display: flex;
  width: 100%;
}

.ui-toaster[data-state="inline"][data-position$="left"],
.ui-toaster[data-portal="false"][data-position$="left"] {
  justify-content: flex-start;
}

.ui-toaster[data-state="inline"][data-position$="center"],
.ui-toaster[data-portal="false"][data-position$="center"] {
  justify-content: center;
}

.ui-toaster[data-state="inline"][data-position$="right"],
.ui-toaster[data-portal="false"][data-position$="right"] {
  justify-content: flex-end;
}

.ui-toaster[data-queue="single"] .ui-toaster__sonner.ui-sonner {
  max-width: min(100%, 360px);
}

.ui-toaster[data-queue="bounded"] .ui-toaster__sonner.ui-sonner,
.ui-toaster[data-queue="extended"] .ui-toaster__sonner.ui-sonner {
  max-width: min(100%, var(--ui-toaster-max-inline-width));
}

.ui-toaster__sonner.ui-sonner {
  width: 100%;
}

.ui-toaster__sonner[data-slot="toaster-sonner"].ui-sonner {
  width: 100%;
}
"#;
