pub const CSS: &str = r#"
.ui-calendar {
  --ui-calendar-motion-duration: 180ms;
  display: grid;
  gap: var(--ui-space-xs);
  width: min(100%, 22rem);
  padding: var(--ui-space-sm);
  border: 1px solid var(--ui-border);
  border-radius: var(--ui-radius-lg);
  background: var(--ui-bg);
  color: var(--ui-fg);
}

.ui-calendar--tone-default,
.ui-calendar[data-tone="default"] {
  background: var(--ui-bg);
}

.ui-calendar--tone-quiet,
.ui-calendar[data-tone="quiet"] {
  background: color-mix(in oklab, var(--ui-bg-muted) 72%, var(--ui-bg) 28%);
}

.ui-calendar--tone-strong,
.ui-calendar[data-tone="strong"] {
  background: color-mix(in oklab, var(--ui-accent-soft) 24%, var(--ui-bg) 76%);
  border-color: color-mix(in oklab, var(--ui-accent) 36%, var(--ui-border) 64%);
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
  color: var(--ui-fg-muted);
}

.ui-calendar--has-selection,
.ui-calendar[data-state="selected"] {
  box-shadow: inset 0 0 0 1px color-mix(in oklab, var(--ui-accent) 24%, transparent);
}

.ui-calendar--custom-class,
.ui-calendar[data-custom-class="true"] {
  outline: 1px solid color-mix(in oklab, var(--ui-accent) 28%, transparent);
}

.ui-calendar__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.ui-calendar__title {
  font-size: var(--ui-heading-h6-font-size, 14px);
  line-height: var(--ui-heading-h6-line-height, 20px);
  font-weight: 600;
}

.ui-calendar__weekdays,
.ui-calendar__grid {
  display: grid;
  grid-template-columns: repeat(7, minmax(0, 1fr));
  gap: var(--ui-space-3xs);
}

.ui-calendar__weekday {
  text-align: center;
  font-size: var(--ui-font-size-100, 12px);
  line-height: var(--ui-line-height-100, 16px);
  color: var(--ui-fg-muted);
}

.ui-calendar__day,
.ui-calendar__day-empty {
  min-height: 2rem;
  border: 1px solid color-mix(in oklab, var(--ui-border) 74%, transparent);
  border-radius: var(--ui-radius-sm);
}

.ui-calendar__day {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background: var(--ui-bg);
  color: var(--ui-fg);
  transition:
    background-color var(--ui-calendar-motion-duration) ease,
    border-color var(--ui-calendar-motion-duration) ease,
    color var(--ui-calendar-motion-duration) ease,
    transform var(--ui-calendar-motion-duration) ease;
}

.ui-calendar__day--selected,
.ui-calendar__day[data-selected="true"] {
  border-color: color-mix(in oklab, var(--ui-accent) 52%, var(--ui-border) 48%);
  background: color-mix(in oklab, var(--ui-accent-soft) 52%, var(--ui-bg) 48%);
  color: color-mix(in oklab, var(--ui-fg) 80%, var(--ui-accent) 20%);
}

.ui-calendar__day--outside,
.ui-calendar__day[data-month-source="outside"] {
  opacity: 0.78;
}

.ui-calendar__day:active {
  transform: scale(0.98);
}

.ui-calendar__day-empty {
  display: block;
  background: color-mix(in oklab, var(--ui-bg-muted) 60%, transparent);
  border: 1px dashed color-mix(in oklab, var(--ui-border) 56%, transparent);
}
"#;
