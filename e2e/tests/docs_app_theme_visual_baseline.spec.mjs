import { expect, test } from "@playwright/test";

const visualMode = process.env.E2E_VISUAL_BASELINE ?? "off";

test("docs-app: theme visual baseline renders button/input/overlay", async ({ page }) => {
  await page.goto("/#/components/theme-visual-baseline");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toHaveText("ThemeVisualBaseline");

  await expect(page.locator('[data-slot="theme-visual-baseline"]').first()).toBeVisible();
  await expect(page.locator('[data-slot="theme-visual-baseline-button"] [data-slot="button"]').first()).toBeVisible();
  await expect(page.locator('[data-slot="theme-visual-baseline-input"] [data-slot="input"]').first()).toBeVisible();
  await expect(page.locator('[data-slot="overlay"][data-state="open"]').first()).toBeVisible();
});

test("docs-app: theme visual baseline screenshots", async ({ page }) => {
  test.skip(
    visualMode !== "on",
    "set E2E_VISUAL_BASELINE=on to run visual snapshot regression"
  );

  await page.goto("/#/components/theme-visual-baseline");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator('[data-slot="theme-visual-baseline"]').first()).toBeVisible();
  await expect(page.locator('[data-slot="overlay"][data-state="open"]').first()).toBeVisible();

  await expect(page.locator('[data-slot="theme-visual-baseline"]').first()).toHaveScreenshot(
    "docs-app-theme-visual-baseline-page.png",
    { animations: "disabled" }
  );
  await expect(page.locator('[data-slot="theme-visual-baseline-button"]').first()).toHaveScreenshot(
    "docs-app-theme-visual-baseline-button.png",
    { animations: "disabled" }
  );
  await expect(page.locator('[data-slot="theme-visual-baseline-input"]').first()).toHaveScreenshot(
    "docs-app-theme-visual-baseline-input.png",
    { animations: "disabled" }
  );
  await expect(page.locator('[data-slot="theme-visual-baseline-overlay"]').first()).toHaveScreenshot(
    "docs-app-theme-visual-baseline-overlay.png",
    { animations: "disabled" }
  );
});
