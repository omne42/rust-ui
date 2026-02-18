import { expect, test } from "@playwright/test";

test("docs-app mobile nav sheet opens and closes", async ({ page }) => {
  await page.goto("/#/");
  await page.locator("body:not(:has(#boot))").waitFor();

  const openNav = page.getByRole("button", { name: "Open navigation" });
  await openNav.click();
  const navSheet = page.locator(
    '[data-slot="sheet"][data-state="open"][data-placement="left"]'
  );
  const navPanel = page.locator('[data-slot="sheet-panel"][role="dialog"]').first();
  await expect(navSheet).toBeVisible();
  await expect(navPanel).toBeVisible();

  await page.keyboard.press("Escape");
  await expect(page.locator('[data-slot="sheet"][data-placement="left"]')).toHaveCount(0);
});
