import { expect, test } from "@playwright/test";

test("docs-app boots and renders shell", async ({ page }) => {
  await page.goto("/#/");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-shell")).toBeVisible();
});

