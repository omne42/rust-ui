import { expect, test } from "@playwright/test";

test("docs-app autocomplete uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  await page.goto("/#/components/autocomplete");
  await page.locator("body:not(:has(#boot))").waitFor();

  const controlledInput = page.locator("#docs-autocomplete-controlled-input").first();
  const controlledRoot = controlledInput
    .locator('xpath=ancestor::*[@data-slot="autocomplete"][1]')
    .first();
  const openMarker = page.locator('[data-slot="autocomplete-controlled-open"]').first();
  const selectedMarker = page.locator('[data-slot="autocomplete-controlled-selected"]').first();

  await expect(controlledRoot).toBeVisible();
  await expect(controlledInput).toBeVisible();
  await expect(openMarker).toHaveText("open: false");
  await expect(selectedMarker).toHaveText("selected: 2");
  await expect(controlledRoot).toHaveAttribute("data-controlled", "true");
  await expect(controlledRoot).toHaveAttribute("data-closed", "true");
  await expect(controlledRoot).not.toHaveAttribute("data-uncontrolled", "true");

  await controlledInput.focus();
  await expect(controlledInput).toBeFocused();
  await expect(openMarker).toHaveText("open: true");
  await expect(controlledRoot).toHaveAttribute("data-open", "true");

  await controlledInput.press("Escape");
  await expect(openMarker).toHaveText("open: false");
  await expect(controlledRoot).toHaveAttribute("data-closed", "true");
});

test("docs-app autocomplete key flow is repeatable with semantic contract breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/autocomplete");
  await page.locator("body:not(:has(#boot))").waitFor();

  const controlledInput = page.locator("#docs-autocomplete-controlled-input").first();
  const controlledRoot = controlledInput
    .locator('xpath=ancestor::*[@data-slot="autocomplete"][1]')
    .first();
  const openMarker = page.locator('[data-slot="autocomplete-controlled-open"]').first();
  const selectedMarker = page.locator('[data-slot="autocomplete-controlled-selected"]').first();

  await expect(selectedMarker).toHaveText("selected: 2");

  await controlledInput.focus();
  await controlledInput.fill("Shen");
  await expect(openMarker).toHaveText("open: true");
  await expect(controlledRoot).toHaveAttribute("data-open", "true");

  const option = page
    .locator('[data-slot="autocomplete-option"]')
    .filter({ hasText: "Shenzhen" })
    .first();
  await expect(option).toBeVisible();
  await option.click();

  await expect(selectedMarker).toHaveText("selected: 3");
  await expect(openMarker).toHaveText("open: false");
  await expect(controlledRoot).toHaveAttribute("data-closed", "true");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator('[data-slot="autocomplete-controlled-selected"]').first()).toHaveText(
    "selected: 2"
  );
  await expect(page.locator('[data-slot="autocomplete-controlled-open"]').first()).toHaveText(
    "open: false"
  );
});
