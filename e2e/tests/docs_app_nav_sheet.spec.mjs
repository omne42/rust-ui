import { expect, test } from "@playwright/test";

test("docs-app mobile nav sheet opens and closes", async ({ page }) => {
  await page.goto("/#/");
  await page.locator("body:not(:has(#boot))").waitFor();

  const openNav = page.getByRole("button", { name: "Open navigation" });
  await openNav.click();
  await expect(page.locator(".docs-mobile-nav")).toBeVisible();

  await page.keyboard.press("Escape");
  await expect(page.locator(".docs-mobile-nav")).toHaveCount(0);
});

