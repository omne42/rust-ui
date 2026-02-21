import { expect, test } from "@playwright/test";

test("docs-app calendar uses semantic selectors with wasm-stable waits", async ({ page }) => {
  await page.goto("/#/components/calendar");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toHaveText("Calendar");

  const defaultCalendar = page
    .locator('[data-component="calendar"] [data-slot="calendar"]')
    .first();
  await expect(defaultCalendar).toBeVisible();
  await expect(defaultCalendar).toHaveAttribute("role", "group");
  await expect(defaultCalendar).toHaveAttribute("data-ui-schema", "ui.calendar");
  await expect(defaultCalendar).toHaveAttribute("data-ui-intent", "date-selection");
  await expect(defaultCalendar).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(defaultCalendar).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(defaultCalendar).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(defaultCalendar).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app calendar key flow is repeatable with semantic contract breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/calendar");
  await page.locator("body:not(:has(#boot))").waitFor();

  const interactive = page.locator(
    '[data-component="calendar"] [data-slot="calendar"].docs-calendar-interactive',
  );
  const summary = page.locator('[data-component="calendar"] [data-slot="calendar-interactive-summary"]');

  await expect(interactive).toBeVisible();
  await expect(interactive).toHaveAttribute("data-ui-state", "selected");
  await expect(interactive).toHaveAttribute("data-ui-source", "props-selected-day");
  await expect(interactive).toHaveAttribute("data-ui-output-status", "verified");
  await expect(summary).toContainText("month=3");
  await expect(summary).toContainText("selected_day=Some(12)");

  const firstPressableDay = interactive
    .locator('[data-slot="calendar-day"][data-pressable="true"]')
    .first();
  await firstPressableDay.focus();
  await expect(firstPressableDay).toBeFocused();
  await firstPressableDay.press("Enter");
  await expect(interactive).toHaveAttribute("data-ui-action", "select-day");
  await expect(interactive).toHaveAttribute("data-ui-output-status", "verified");

  await page.locator('[data-action="next-month"]').click();
  await expect(interactive).toHaveAttribute("data-ui-output-status", "verified");
  await expect(summary).toContainText("month=4");

  await page.locator('[data-action="clear-selection"]').click();
  await expect(interactive).toHaveAttribute("data-ui-state", "default");
  await expect(interactive).toHaveAttribute("data-ui-source", "implicit-default");
  await expect(interactive).toHaveAttribute("data-ui-output-status", "verified");
  await expect(summary).toContainText("selected_day=None");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  const interactiveAfterReload = page.locator(
    '[data-component="calendar"] [data-slot="calendar"].docs-calendar-interactive',
  );
  const summaryAfterReload = page.locator(
    '[data-component="calendar"] [data-slot="calendar-interactive-summary"]',
  );
  await expect(interactiveAfterReload).toHaveAttribute("data-ui-state", "selected");
  await expect(interactiveAfterReload).toHaveAttribute("data-ui-source", "props-selected-day");
  await expect(interactiveAfterReload).toHaveAttribute("data-ui-output-status", "verified");
  await expect(summaryAfterReload).toContainText("month=3");
  await expect(summaryAfterReload).toContainText("selected_day=Some(12)");
});
