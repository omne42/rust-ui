import { expect, test } from "@playwright/test";

const COLOR_SLIDER_PAGE = "/#/components/color-slider";
const CONTROLLED_ROOT =
  '[data-component="color-slider"] #docs-color-slider-hue[data-slot="color-slider"][data-control-mode="controlled"][data-channel="hue"]';

async function gotoColorSlider(page) {
  await page.goto(COLOR_SLIDER_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
}

async function resolveControlledRoot(page) {
  const root = page.locator(CONTROLLED_ROOT).first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-ui-schema", "ui.color-slider.agent-contract.v1");
  await expect(root).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(root).toHaveAttribute("data-ui-stream-mode", "snapshot");
  return root;
}

test("docs-app color-slider contract uses semantic selectors with wasm-safe stable waits", async ({
  page,
}) => {
  await gotoColorSlider(page);
  const root = await resolveControlledRoot(page);

  const input = root.locator('[data-slot="color-slider-input"]').first();
  const label = root.locator('[data-slot="color-slider-label"]').first();

  await expect(input).toHaveAttribute("role", "slider");
  await expect(input).toHaveAttribute("aria-labelledby", "docs-color-slider-hue-label");
  await expect(label).toHaveAttribute("id", "docs-color-slider-hue-label");

  await expect(root).toHaveAttribute("data-state", "ready");
  await expect(root).toHaveAttribute("data-control-mode", "controlled");
  await expect(root).toHaveAttribute("data-value-source", "external");
  await expect(root).toHaveAttribute("data-value-change-source", "on_value_change");
  await expect(root).toHaveAttribute("data-ui-intent", "adjust-color-channel");
  await expect(root).toHaveAttribute("data-ui-action", "idle");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");
});

test("docs-app color-slider interaction path uses ready and settled semantic breakpoints", async ({
  page,
}) => {
  await gotoColorSlider(page);
  const root = await resolveControlledRoot(page);
  const input = root.locator('[data-slot="color-slider-input"]').first();

  const before = Number((await root.getAttribute("data-value")) ?? "0");

  await input.focus();
  await expect(input).toBeFocused();
  await expect(root).toHaveAttribute("data-ui-action", "focus");
  await expect(root).toHaveAttribute("data-focused", "true");

  await page.keyboard.press("ArrowRight");
  const after = Number((await root.getAttribute("data-value")) ?? "0");
  expect(after).toBeGreaterThan(before);
  await expect(root).toHaveAttribute("data-ui-source", "on_value_change");
  await expect(root).toHaveAttribute("data-ui-action", "focus");
  await expect(root).not.toHaveAttribute("data-pressed", "true");

  await input.dispatchEvent("pointerdown");
  await expect(root).toHaveAttribute("data-ui-action", "press");
  await input.dispatchEvent("pointerup");
  await expect(root).toHaveAttribute("data-ui-action", "focus");

  const disabledRoot = page
    .locator('[data-component="color-slider"] #docs-color-slider-alpha[data-slot="color-slider"]')
    .first();
  const disabledInput = disabledRoot.locator('[data-slot="color-slider-input"]').first();
  await expect(disabledRoot).toHaveAttribute("data-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");
  await expect(disabledRoot).toHaveAttribute("data-disabled-source", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-ui-output-status", "submittable");
  await expect(disabledInput).toHaveAttribute("aria-disabled", "true");

  const customRoot = page
    .locator('[data-component="color-slider"] #docs-color-slider-custom[data-slot="color-slider"]')
    .first();
  await expect(customRoot).toHaveAttribute("data-track-source", "custom");
  await expect(customRoot).toHaveAttribute("data-motion-source", "custom");
  await expect(customRoot).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app color-slider key flow is repeatable and failures map to semantic breakpoints", async ({
  page,
}) => {
  await gotoColorSlider(page);
  let root = await resolveControlledRoot(page);
  let input = root.locator('[data-slot="color-slider-input"]').first();

  await input.focus();
  await expect(input).toBeFocused();
  const before = Number((await root.getAttribute("data-value")) ?? "0");

  await page.keyboard.press("ArrowRight");
  const after = Number((await root.getAttribute("data-value")) ?? "0");
  expect(after).toBeGreaterThan(before);
  await expect(root).toHaveAttribute("data-ui-action", "focus");
  await expect(root).toHaveAttribute("data-ui-source", "on_value_change");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  root = await resolveControlledRoot(page);
  input = root.locator('[data-slot="color-slider-input"]').first();
  await expect(root).toHaveAttribute("data-ui-action", "idle");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");

  await input.focus();
  await page.keyboard.press("ArrowRight");
  const replayed = Number((await root.getAttribute("data-value")) ?? "0");
  expect(replayed).toBeGreaterThan(before);
  await expect(root).toHaveAttribute("data-focused", "true");
  await expect(root).toHaveAttribute("data-focus-visible", "true");
  await expect(root).toHaveAttribute("data-ui-action", "focus");
});

test("docs-app color-slider high-risk paths keep focus keyboard and disabled branches semantically explicit", async ({
  page,
}) => {
  await gotoColorSlider(page);
  const root = await resolveControlledRoot(page);
  const input = root.locator('[data-slot="color-slider-input"]').first();

  await input.focus();
  await expect(input).toBeFocused();
  await page.keyboard.press("ArrowLeft");
  await expect(root).toHaveAttribute("data-ui-action", "focus");
  await expect(root).toHaveAttribute("data-focused", "true");
  await expect(root).toHaveAttribute("data-focus-visible", "true");
  await expect(root).toHaveAttribute("data-ui-source", "on_value_change");

  const disabledRoot = page
    .locator('[data-component="color-slider"] #docs-color-slider-alpha[data-slot="color-slider"]')
    .first();
  const disabledInput = disabledRoot.locator('[data-slot="color-slider-input"]').first();
  await expect(disabledRoot).toBeVisible();
  await expect(disabledRoot).toHaveAttribute("data-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");
  await expect(disabledRoot).toHaveAttribute("data-ui-output-status", "submittable");
  await expect(disabledInput).toHaveAttribute("aria-disabled", "true");
  await expect(disabledInput).toBeDisabled();
});
