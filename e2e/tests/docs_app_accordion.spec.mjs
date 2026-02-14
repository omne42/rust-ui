import { expect, test } from "@playwright/test";

test("docs-app: accordion interaction + keyboard roving", async ({ page }) => {
  await page.goto("/#/components/accordion");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toBeVisible();

  const openIndices = page.locator("span.ui-muted", { hasText: "open indices:" });

  const multiTrigger0 = page.locator("#docs-accordion-trigger-0");
  const multiTrigger1 = page.locator("#docs-accordion-trigger-1");
  const multiPanel1 = page.locator("#docs-accordion-panel-1");

  await multiTrigger0.scrollIntoViewIfNeeded();
  await expect(openIndices).toContainText("[0]");
  await expect(multiTrigger0).toHaveAttribute("aria-expanded", "true");
  await expect(multiTrigger1).toHaveAttribute("aria-expanded", "false");
  await expect(multiPanel1).toBeHidden();

  await multiTrigger1.click();
  await expect(openIndices).toContainText("[0, 1]");
  await expect(multiTrigger1).toHaveAttribute("aria-expanded", "true");
  await expect(multiPanel1).toBeVisible();

  await multiTrigger0.focus();
  await expect(multiTrigger0).toBeFocused();
  await page.keyboard.press("ArrowDown");
  await expect(multiTrigger1).toBeFocused();
  await page.keyboard.press("Space");

  await expect(openIndices).toContainText("[0]");
  await expect(multiTrigger1).toHaveAttribute("aria-expanded", "false");
  await expect(multiPanel1).toBeHidden();

  const singleOpen = page.locator("span.ui-muted", { hasText: "single open:" });
  const singleTrigger0 = page.locator("#docs-accordion-single-trigger-0");
  const singleTrigger1 = page.locator("#docs-accordion-single-trigger-1");
  const singleTrigger2 = page.locator("#docs-accordion-single-trigger-2");

  await singleTrigger0.scrollIntoViewIfNeeded();
  await expect(singleOpen).toContainText("[1]");
  await expect(singleTrigger2).toBeDisabled();

  await singleTrigger0.click();
  await expect(singleOpen).toContainText("[0]");
  await expect(singleTrigger0).toHaveAttribute("aria-expanded", "true");
  await expect(singleTrigger1).toHaveAttribute("aria-expanded", "false");

  await singleTrigger1.focus();
  await expect(singleTrigger1).toBeFocused();
  await page.keyboard.press("ArrowDown");
  await expect(singleTrigger0).toBeFocused();
});
