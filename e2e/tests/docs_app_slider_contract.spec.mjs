import { expect, test } from "@playwright/test";

const SLIDER_PAGE = "/#/components/slider";
const CONTROLLED_SLIDER_ROOT =
  '[data-component="slider"] [data-slot="slider"][data-control-mode="controlled"][data-max="100"]';

async function gotoSlider(page) {
  await page.goto(SLIDER_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
}

async function resolveControlledSliderRoot(page) {
  const root = page.locator(CONTROLLED_SLIDER_ROOT).first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-ui-schema", "ui.slider.agent-contract.v1");
  await expect(root).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(root).toHaveAttribute("data-ui-stream-mode", "snapshot");
  return root;
}

test("docs-app slider contract uses semantic selectors with wasm-safe ready waits", async ({ page }) => {
  await gotoSlider(page);
  const sliderRoot = await resolveControlledSliderRoot(page);
  const controlledInput = sliderRoot.locator('[data-slot="slider-input"]').first();

  await expect(controlledInput).toBeVisible();
  await expect(sliderRoot).toHaveAttribute("data-control-mode", "controlled");
  await expect(sliderRoot).toHaveAttribute("data-value-source", "external");
  await expect(sliderRoot).toHaveAttribute("data-ui-intent", "adjust-value");
  await expect(sliderRoot).toHaveAttribute("data-ui-action", "idle");
  await expect(sliderRoot).toHaveAttribute("data-value", "36");
});

test("docs-app slider key flow uses semantic breakpoints with explicit settled conditions", async ({ page }) => {
  await gotoSlider(page);
  const sliderRoot = await resolveControlledSliderRoot(page);
  const input = sliderRoot.locator('[data-slot="slider-input"]').first();

  await input.focus();
  await expect(input).toBeFocused();
  await expect(sliderRoot).toHaveAttribute("data-ui-action", "focus");
  await expect(sliderRoot).toHaveAttribute("data-focused", "true");

  await page.keyboard.press("ArrowRight");
  await expect(sliderRoot).toHaveAttribute("data-value", "37");
  await expect(sliderRoot).toHaveAttribute("data-value-percent", "37");
  await expect(sliderRoot).toHaveAttribute("data-ui-source", "on_value_change");
  await expect(sliderRoot).toHaveAttribute("data-ui-action", "focus");
  await expect(sliderRoot).not.toHaveAttribute("data-pressed", "true");

  const disabledRoot = page
    .locator('[data-component="slider"] [data-slot="slider"][data-disabled="true"]')
    .first();
  const disabledInput = disabledRoot.locator('[data-slot="slider-input"]').first();
  await expect(disabledRoot).toHaveAttribute("data-state", "disabled");
  await expect(disabledInput).toHaveAttribute("aria-disabled", "true");
});

test("docs-app slider key flow is repeatable and failures map to semantic breakpoints", async ({ page }) => {
  await gotoSlider(page);
  let sliderRoot = await resolveControlledSliderRoot(page);
  let input = sliderRoot.locator('[data-slot="slider-input"]').first();

  await input.focus();
  await page.keyboard.press("ArrowRight");
  await expect(sliderRoot).toHaveAttribute("data-value", "37");
  await expect(sliderRoot).toHaveAttribute("data-ui-source", "on_value_change");
  await expect(sliderRoot).toHaveAttribute("data-ui-action", "focus");
  await expect(sliderRoot).toHaveAttribute("data-ui-output-status", "submittable");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  sliderRoot = await resolveControlledSliderRoot(page);
  input = sliderRoot.locator('[data-slot="slider-input"]').first();

  await expect(sliderRoot).toHaveAttribute("data-value", "36");
  await expect(sliderRoot).toHaveAttribute("data-ui-action", "idle");
  await expect(sliderRoot).toHaveAttribute("data-ui-source", "on_value_change");

  await input.focus();
  await page.keyboard.press("ArrowRight");
  await expect(sliderRoot).toHaveAttribute("data-value", "37");
  await expect(sliderRoot).toHaveAttribute("data-focused", "true");
  await expect(sliderRoot).toHaveAttribute("data-focus-visible", "true");
  await expect(sliderRoot).toHaveAttribute("data-ui-action", "focus");
  await expect(sliderRoot).not.toHaveAttribute("data-pressed", "true");
});
