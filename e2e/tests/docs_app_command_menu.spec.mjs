import { expect, test } from "@playwright/test";

test("docs-app command menu opens and navigates to a result", async ({ page }) => {
  await page.goto("/#/");
  await page.locator("body:not(:has(#boot))").waitFor();

  await page.getByRole("button", { name: "Search docs" }).click();
  await expect(page.locator(".docs-command-menu__input")).toBeVisible();

  await page.locator(".docs-command-menu__input").fill("Button");
  const results = page.locator("[data-slot=\"docs-command-menu-results\"] li");
  await expect(results.first()).toBeVisible();

  await results.first().click();
  await page.waitForFunction(() => window.location.hash.includes("button"));
});

