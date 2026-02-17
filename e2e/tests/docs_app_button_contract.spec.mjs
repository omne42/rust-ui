import { expect, test } from "@playwright/test";

test("docs-app button workbench uses semantic selectors with settled loading/disabled states", async ({ page }) => {
  await page.goto("/#/components/button");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator("section.playground")
    .filter({ has: page.locator('[data-slot="button-workbench"]') })
    .first();
  await expect(playground).toBeVisible();

  const controls = playground.locator('[data-slot="playground-controls"]');
  const loadingSwitch = controls.locator('[data-slot="switch"]', { hasText: "Loading" }).first();
  const disabledSwitch = controls.locator('[data-slot="switch"]', { hasText: "Disabled" }).first();
  const button = playground.locator('[data-slot="button-workbench-canvas"] [data-slot="button"]').first();

  await expect(button).toBeVisible();
  await expect(button).toHaveAttribute("data-loading-source", "default");
  await expect(button).not.toHaveAttribute("data-loading", "true");

  await loadingSwitch.click();
  await expect(button).toHaveAttribute("data-loading", "true");
  await expect(button).toHaveAttribute("data-loading-source", "prop");
  await expect(button).toHaveAttribute("aria-busy", "true");
  await expect(button).toBeDisabled();

  await loadingSwitch.click();
  await expect(button).not.toHaveAttribute("data-loading", "true");
  await expect(button).toHaveAttribute("data-loading-source", "default");
  await expect(button).not.toHaveAttribute("aria-busy", "true");

  await disabledSwitch.click();
  await expect(button).toHaveAttribute("data-disabled", "true");
  await expect(button).toHaveAttribute("data-disabled-source", "prop");
  await expect(button).toBeDisabled();

  await disabledSwitch.click();
  await expect(button).not.toHaveAttribute("data-disabled", "true");
  await expect(button).toHaveAttribute("data-disabled-source", "default");
});

test("docs-app button workbench supports keyboard flow and code snapshot sync", async ({ page }) => {
  await page.goto("/#/components/button");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator("section.playground")
    .filter({ has: page.locator('[data-slot="button-workbench"]') })
    .first();
  await expect(playground).toBeVisible();

  const controls = playground.locator('[data-slot="playground-controls"]');
  const loadingSwitch = controls.locator('[data-slot="switch"]', { hasText: "Loading" }).first();
  const codeToggle = playground.getByRole("button", { name: /Show code|Hide code/ }).first();
  const button = playground.locator('[data-slot="button-workbench-canvas"] [data-slot="button"]').first();

  await loadingSwitch.focus();
  await page.keyboard.press("Space");
  await expect(button).toHaveAttribute("data-loading", "true");
  await expect(button).toHaveAttribute("aria-busy", "true");

  await page.keyboard.press("Space");
  await expect(button).not.toHaveAttribute("data-loading", "true");
  await expect(button).not.toHaveAttribute("aria-busy", "true");

  await codeToggle.click();
  const codeBlock = playground.locator('[data-slot="code-block-code"]').first();
  await expect(codeBlock).toBeVisible();

  await loadingSwitch.click();
  await expect(codeBlock).toContainText("is_loading=true");

  await loadingSwitch.click();
  await expect(codeBlock).not.toContainText("is_loading=true");
});
