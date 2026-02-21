import { expect, test } from "@playwright/test";

const COLOR_SWATCH_PICKER_PAGE = "/#/components/color-swatch-picker";
const CONTROLLED_ROOT =
  '[data-component="color-swatch-picker"] [data-slot="color-swatch-picker"][data-selection-mode="controlled"][aria-label="Controlled swatch picker"]';

async function gotoColorSwatchPicker(page) {
  await page.goto(COLOR_SWATCH_PICKER_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
}

async function resolveControlledRoot(page) {
  const root = page.locator(CONTROLLED_ROOT).first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-ui-schema", "ui.color-swatch-picker.agent-contract.v1");
  await expect(root).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(root).toHaveAttribute("data-ui-output-status", "verified");
  return root;
}

async function resolveUncontrolledRoot(page) {
  const root = page
    .locator(
      '[data-component="color-swatch-picker"] [data-slot="color-swatch-picker"][data-selection-mode="uncontrolled"][data-selection-init-source="default"]'
    )
    .first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-selection-source", "default");
  await expect(root).toHaveAttribute("data-ui-action", "sync");
  await expect(root).toHaveAttribute("data-ui-state", "active");
  return root;
}

test("docs-app color-swatch-picker contract uses semantic selectors with wasm-safe stable waits", async ({
  page,
}) => {
  await gotoColorSwatchPicker(page);
  const root = await resolveControlledRoot(page);

  const list = root.locator('[data-slot="color-swatch-picker-list"]').first();
  const firstOption = root.locator('[data-slot="color-swatch-picker-option"][data-index="0"]').first();

  await expect(list).toBeVisible();
  await expect(firstOption).toBeVisible();
  await expect(root).toHaveAttribute("role", "radiogroup");
  await expect(firstOption).toHaveAttribute("role", "radio");
  await expect(root).toHaveAttribute("data-selection-mode", "controlled");
  await expect(root).toHaveAttribute("data-selection-source", "external");
  await expect(root).toHaveAttribute("data-ui-intent", "pick-color-swatch");
  await expect(root).toHaveAttribute("data-ui-action", "sync");
  await expect(root).toHaveAttribute("data-ui-source", "external");
});

test("docs-app color-swatch-picker interaction path covers ready and settled semantic breakpoints", async ({
  page,
}) => {
  await gotoColorSwatchPicker(page);
  const uncontrolled_root = await resolveUncontrolledRoot(page);

  const option_green = uncontrolled_root
    .locator('[data-slot="color-swatch-picker-option"][data-index="2"]')
    .first();
  await option_green.click();
  await expect(uncontrolled_root).toHaveAttribute("data-selected-index", "2");
  await expect(uncontrolled_root).toHaveAttribute("data-selection-source", "interaction");
  await expect(uncontrolled_root).toHaveAttribute("data-ui-action", "select");
  await expect(uncontrolled_root).toHaveAttribute("data-ui-source", "interaction");
  await expect(option_green).toHaveAttribute("aria-checked", "true");

  const disabled_root = page
    .locator(
      '[data-component="color-swatch-picker"] [data-slot="color-swatch-picker"][data-has-disabled-items="true"][data-count="4"]'
    )
    .first();
  const disabled_option = disabled_root
    .locator('[data-slot="color-swatch-picker-option"][data-index="1"]')
    .first();
  await expect(disabled_root).toHaveAttribute("data-has-disabled-items", "true");
  await expect(disabled_option).toHaveAttribute("aria-disabled", "true");
  await expect(disabled_option).toBeDisabled();
});

test("docs-app color-swatch-picker key flow is repeatable and failures map to semantic breakpoints", async ({
  page,
}) => {
  await gotoColorSwatchPicker(page);
  let uncontrolled_root = await resolveUncontrolledRoot(page);
  let option_orange = uncontrolled_root
    .locator('[data-slot="color-swatch-picker-option"][data-index="1"]')
    .first();

  await option_orange.focus();
  await expect(option_orange).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(uncontrolled_root).toHaveAttribute("data-selected-index", "2");
  await expect(uncontrolled_root).toHaveAttribute("data-selection-source", "interaction");
  await expect(uncontrolled_root).toHaveAttribute("data-ui-action", "select");
  await expect(uncontrolled_root).toHaveAttribute("data-ui-source", "interaction");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  uncontrolled_root = await resolveUncontrolledRoot(page);
  option_orange = uncontrolled_root
    .locator('[data-slot="color-swatch-picker-option"][data-index="1"]')
    .first();
  await option_orange.focus();
  await expect(option_orange).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(uncontrolled_root).toHaveAttribute("data-selected-index", "2");
  await expect(uncontrolled_root).toHaveAttribute("data-selection-source", "interaction");
  await expect(uncontrolled_root).toHaveAttribute("data-ui-action", "select");
  await expect(uncontrolled_root).toHaveAttribute("data-ui-source", "interaction");
});

test("docs-app color-swatch-picker high-risk paths keep focus keyboard and disabled branches semantically explicit", async ({
  page,
}) => {
  await gotoColorSwatchPicker(page);
  const uncontrolled_root = await resolveUncontrolledRoot(page);
  const option_orange = uncontrolled_root
    .locator('[data-slot="color-swatch-picker-option"][data-index="1"]')
    .first();

  await option_orange.focus();
  await expect(option_orange).toBeFocused();
  await page.keyboard.press("ArrowLeft");
  await expect(uncontrolled_root).toHaveAttribute("data-selected-index", "0");
  await expect(uncontrolled_root).toHaveAttribute("data-selection-source", "interaction");
  await expect(uncontrolled_root).toHaveAttribute("data-ui-action", "select");
  await expect(uncontrolled_root).toHaveAttribute("data-ui-state", "active");

  const disabled_root = page
    .locator(
      '[data-component="color-swatch-picker"] [data-slot="color-swatch-picker"][data-has-disabled-items="true"][data-count="4"]'
    )
    .first();
  const disabled_option = disabled_root
    .locator('[data-slot="color-swatch-picker-option"][data-index="1"]')
    .first();
  await expect(disabled_root).toHaveAttribute("data-has-disabled-items", "true");
  await expect(disabled_option).toHaveAttribute("aria-disabled", "true");
  await expect(disabled_option).toBeDisabled();
});
