import { expect, test } from "@playwright/test";

test("docs-app native-select contract uses semantic selectors with settled waits", async ({
  page,
}) => {
  await page.goto("/#/components/native-select");
  await page.locator("body:not(:has(#boot))").waitFor();

  const controlledControl = page
    .locator("#docs-native-select-controlled-control")
    .first();
  const controlledRoot = controlledControl
    .locator('xpath=ancestor::*[@data-slot="native-select"][1]')
    .first();

  await expect(controlledRoot).toBeVisible();
  await expect(controlledControl).toBeVisible();
  await expect(controlledRoot).toHaveAttribute("data-selection-mode", "controlled");
  await expect(controlledRoot).toHaveAttribute("data-selection-source", "external");
  await expect(controlledRoot).toHaveAttribute("data-change-source", "initial");
  await expect(controlledRoot).toHaveAttribute("data-streaming-mode", "optional");
  await expect(controlledRoot).toHaveAttribute("data-streaming-fallback", "snapshot");
  await expect(controlledRoot).toHaveAttribute("data-output-status", "draft");

  await controlledControl.selectOption("manual");
  await expect(controlledRoot).toHaveAttribute("data-selected-index", "1");
  await expect(controlledRoot).toHaveAttribute("data-selected-value", "manual");
  await expect(controlledRoot).toHaveAttribute("data-change-source", "user");
  await expect(controlledRoot).toHaveAttribute("data-ui-action", "user-select");
  await expect(controlledRoot).toHaveAttribute("data-output-status", "submittable");
});

test("docs-app native-select key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/native-select");
  await page.locator("body:not(:has(#boot))").waitFor();

  const uncontrolledControl = page
    .locator("#docs-native-select-uncontrolled-control")
    .first();
  const uncontrolledRoot = uncontrolledControl
    .locator('xpath=ancestor::*[@data-slot="native-select"][1]')
    .first();

  await expect(uncontrolledRoot).toHaveAttribute("data-selection-mode", "uncontrolled");
  await expect(uncontrolledRoot).toHaveAttribute("data-selection-source", "default");
  await expect(uncontrolledRoot).toHaveAttribute("data-selected-index", "1");
  await expect(uncontrolledRoot).toHaveAttribute("data-selected-value", "manual");
  await uncontrolledControl.focus();
  await expect(uncontrolledControl).toBeFocused();

  await uncontrolledControl.selectOption("system");
  await expect(uncontrolledRoot).toHaveAttribute("data-selected-index", "0");
  await expect(uncontrolledRoot).toHaveAttribute("data-selected-value", "system");
  await expect(uncontrolledRoot).toHaveAttribute("data-change-source", "user");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedControl = page
    .locator("#docs-native-select-uncontrolled-control")
    .first();
  const reloadedRoot = reloadedControl
    .locator('xpath=ancestor::*[@data-slot="native-select"][1]')
    .first();
  await expect(reloadedRoot).toHaveAttribute("data-selected-index", "1");
  await expect(reloadedRoot).toHaveAttribute("data-selected-value", "manual");
  await expect(reloadedRoot).toHaveAttribute("data-change-source", "initial");
});

test("docs-app native-select keyboard path uses semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/native-select");
  await page.locator("body:not(:has(#boot))").waitFor();

  const matrixControl = page
    .locator("#docs-native-select-matrix-default-control")
    .first();
  const matrixRoot = matrixControl
    .locator('xpath=ancestor::*[@data-slot="native-select"][1]')
    .first();

  await expect(matrixRoot).toHaveAttribute("data-selection-mode", "uncontrolled");
  await expect(matrixRoot).toHaveAttribute("data-selected-index", "0");
  await matrixControl.focus();
  await expect(matrixControl).toBeFocused();

  await page.keyboard.press("ArrowDown");
  await expect(matrixRoot).toHaveAttribute("data-selected-index", "1");
  await expect(matrixRoot).toHaveAttribute("data-change-source", "user");

  await page.keyboard.press("ArrowUp");
  await expect(matrixRoot).toHaveAttribute("data-selected-index", "0");
  await expect(matrixRoot).toHaveAttribute("data-change-source", "user");
});
