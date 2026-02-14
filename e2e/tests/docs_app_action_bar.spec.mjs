import { expect, test } from "@playwright/test";

test("docs-app: action-bar visibility + clear selection", async ({ page }) => {
  await page.goto("/#/components/action-bar");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toBeVisible();

  const actionBar = page.locator('[data-slot="action-bar"]').first();
  const selectedCount = actionBar.locator('[data-slot="action-bar-selection-count"]').first();
  const clear = actionBar.locator('[data-slot="action-bar-clear"]').first();

  await actionBar.scrollIntoViewIfNeeded();
  await expect(actionBar).toBeVisible();
  await expect(actionBar).toHaveAttribute("data-state", "visible");
  await expect(selectedCount).toContainText("2");
  await expect(clear).toBeVisible();

  await clear.focus();
  await expect(clear).toBeFocused();
  await page.keyboard.press("Space");

  await expect(actionBar).toHaveAttribute("data-state", "hidden");
  await expect(actionBar).toHaveAttribute("aria-hidden", "true");

  const selectPlus = page.getByRole("button", { name: "Select +1" }).first();
  await selectPlus.click();
  await expect(actionBar).toHaveAttribute("data-state", "visible");
  await expect(actionBar).not.toHaveAttribute("aria-hidden", "true");
});

