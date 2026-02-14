import { expect, test } from "@playwright/test";

test("debug overlay captures traced open/close events", async ({ page }) => {
  await page.goto("/#/components/date-picker");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toBeVisible();

  await expect(page.locator('[data-slot="ui-debug-overlay"]')).toBeVisible();

  // Trigger an open change from a traced overlay component.
  await page.locator('[data-slot="date-picker-trigger"] button').click();
  await expect(page.locator('[data-slot="date-picker-panel"]')).toBeVisible();

  // Open the debug overlay UI.
  await page.locator('[data-slot="ui-debug-overlay"] button').click();
  await expect(page.locator('[data-slot="ui-debug-overlay-panel"]')).toBeVisible();

  await expect(
    page.locator(
      '[data-slot="ui-debug-overlay-event"][data-component="date-picker"][data-kind="open-change"]'
    )
  ).toBeVisible();

  // Close and ensure another event arrives.
  await page.locator('[data-slot="date-picker-trigger"] button').click();
  await expect(page.locator('[data-slot="date-picker-panel"]')).toBeHidden();

  await expect(
    page.locator(
      '[data-slot="ui-debug-overlay-event"][data-component="date-picker"][data-kind="open-change"]'
    )
  ).toHaveCount(2);
});
