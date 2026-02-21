pub const CSS: &str = r#"
.ui-calendar {
  --ui-calendar-motion-duration: var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration));
  --ui-calendar-motion-easing: var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing));
  --ui-calendar-day-outside-opacity: var(--ui-alert-opacity, var(--ui-fallback-alert-opacity));
  --ui-calendar-day-active-scale: var(--ui-alert-scale, var(--ui-fallback-alert-scale));
  display: grid;
  gap: var(--ui-space-xs, var(--ui-fallback-space-xs));
  width: min(100%, calc(var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 7));
  padding: var(--ui-space-sm, var(--ui-fallback-space-sm));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid var(--ui-border, var(--ui-fallback-border));
  border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-calendar--tone-default,
.ui-calendar[data-tone="default"] {
  background: var(--ui-bg, var(--ui-fallback-bg));
}

.ui-calendar--tone-quiet,
.ui-calendar[data-tone="quiet"] {
  background: color-mix(
    in oklab,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 72%,
    var(--ui-bg, var(--ui-fallback-bg)) 28%
  );
}

.ui-calendar--tone-strong,
.ui-calendar[data-tone="strong"] {
  background: color-mix(
    in oklab,
    var(--ui-accent-soft, var(--ui-fallback-accent-soft)) 24%,
    var(--ui-bg, var(--ui-fallback-bg)) 76%
  );
  border-color: color-mix(
    in oklab,
    var(--ui-accent, var(--ui-fallback-accent)) 36%,
    var(--ui-border, var(--ui-fallback-border)) 64%
  );
}

.ui-calendar--weekday-sunday,
.ui-calendar[data-first-weekday="sunday"] {
  direction: ltr;
}

.ui-calendar--weekday-monday,
.ui-calendar[data-first-weekday="monday"] {
  direction: ltr;
}

.ui-calendar--outside-days .ui-calendar__day--outside,
.ui-calendar[data-show-outside-days="true"] .ui-calendar__day--outside {
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-calendar--has-selection,
.ui-calendar[data-state="selected"] {
  box-shadow: inset 0 0 0 var(--ui-border-width, var(--ui-fallback-border-width))
    color-mix(in oklab, var(--ui-accent, var(--ui-fallback-accent)) 24%, transparent);
}

.ui-calendar--custom-class,
.ui-calendar[data-custom-class="true"] {
  outline: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(in oklab, var(--ui-accent, var(--ui-fallback-accent)) 28%, transparent);
}

.ui-calendar__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.ui-calendar__title {
  font-size: var(--ui-heading-h6-font-size, var(--ui-fallback-heading-h6-font-size));
  line-height: var(--ui-heading-h6-line-height, var(--ui-fallback-heading-h6-line-height));
  font-weight: 600;
}

.ui-calendar__weekdays,
.ui-calendar__grid {
  display: grid;
  grid-template-columns: repeat(7, minmax(0, 1fr));
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-calendar__weekday {
  text-align: center;
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));
}

.ui-calendar__day,
.ui-calendar__day-empty {
  min-height: var(--ui-component-height-100, var(--ui-fallback-component-height-100));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(in oklab, var(--ui-border, var(--ui-fallback-border)) 74%, transparent);
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
}

.ui-calendar__day {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
  transition:
    background-color
      var(--ui-calendar-motion-duration, var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration)))
      var(--ui-calendar-motion-easing, var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing))),
    border-color
      var(--ui-calendar-motion-duration, var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration)))
      var(--ui-calendar-motion-easing, var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing))),
    color
      var(--ui-calendar-motion-duration, var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration)))
      var(--ui-calendar-motion-easing, var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing))),
    transform
      var(--ui-calendar-motion-duration, var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration)))
      var(--ui-calendar-motion-easing, var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing)));
}

.ui-calendar__day--selected,
.ui-calendar__day[data-selected="true"] {
  border-color: color-mix(
    in oklab,
    var(--ui-accent, var(--ui-fallback-accent)) 52%,
    var(--ui-border, var(--ui-fallback-border)) 48%
  );
  background: color-mix(
    in oklab,
    var(--ui-accent-soft, var(--ui-fallback-accent-soft)) 52%,
    var(--ui-bg, var(--ui-fallback-bg)) 48%
  );
  color: color-mix(
    in oklab,
    var(--ui-fg, var(--ui-fallback-fg)) 80%,
    var(--ui-accent, var(--ui-fallback-accent)) 20%
  );
}

.ui-calendar__day--outside,
.ui-calendar__day[data-month-source="outside"] {
  opacity: var(--ui-calendar-day-outside-opacity, var(--ui-alert-opacity, var(--ui-fallback-alert-opacity)));
}

.ui-calendar__day:active {
  transform: scale(var(--ui-calendar-day-active-scale, var(--ui-alert-scale, var(--ui-fallback-alert-scale))));
}

@media (prefers-reduced-motion: reduce) {
  .ui-calendar {
    --ui-calendar-motion-duration: var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration));
  }
}

.ui-calendar__day-empty {
  display: block;
  background: color-mix(in oklab, var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 60%, transparent);
  border: var(--ui-border-width, var(--ui-fallback-border-width)) dashed
    color-mix(in oklab, var(--ui-border, var(--ui-fallback-border)) 56%, transparent);
}

.ui-calendar__debug {
  display: grid;
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  padding: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) dashed
    color-mix(in oklab, var(--ui-border, var(--ui-fallback-border)) 62%, transparent);
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  background: color-mix(
    in oklab,
    var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 58%,
    var(--ui-bg, var(--ui-fallback-bg)) 42%
  );
}

.ui-calendar__debug-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
}

.ui-calendar__debug-title {
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
}

.ui-calendar__debug-replay {
  padding: var(--ui-space-2xs, var(--ui-fallback-space-2xs));
  border-radius: var(--ui-radius-sm, var(--ui-fallback-radius-sm));
  border: var(--ui-border-width, var(--ui-fallback-border-width)) solid
    color-mix(in oklab, var(--ui-border, var(--ui-fallback-border)) 72%, transparent);
  background: var(--ui-bg, var(--ui-fallback-bg));
  color: var(--ui-fg, var(--ui-fallback-fg));
}

.ui-calendar__debug-list {
  margin: 0;
  padding-left: var(--ui-space-sm, var(--ui-fallback-space-sm));
  max-height: calc(var(--ui-component-height-100, var(--ui-fallback-component-height-100)) * 4);
  overflow: auto;
}

.ui-calendar__debug-event {
  font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));
  line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));
  color: color-mix(
    in oklab,
    var(--ui-fg, var(--ui-fallback-fg)) 84%,
    var(--ui-fg-muted, var(--ui-fallback-fg-muted)) 16%
  );
}
"#;
