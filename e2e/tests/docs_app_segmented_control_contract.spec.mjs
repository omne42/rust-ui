import { expect, test } from "@playwright/test";

const SEGMENTED_CONTROL_PAGE = "/#/components/segmented-control";

async function gotoSegmentedControl(page) {
  await page.goto(SEGMENTED_CONTROL_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
}

async function resolveInteractiveRoot(page) {
  const root = page
    .locator('[data-slot="segmented-control"][data-control-mode="controlled"]')
    .filter({ has: page.locator("#docs-segments-interactive-radio-0") })
    .first();

  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("role", "radiogroup");
  await expect(root).toHaveAttribute("data-ui-schema", "ui.segmented-control");
  await expect(root).toHaveAttribute("data-ui-schema-version", "v1");
  await expect(root).toHaveAttribute("data-ui-intent", "single-choice-selection");
  await expect(root).toHaveAttribute("data-ui-action-model", "navigate|focus|select");
  return root;
}

test("docs-app segmented-control contract uses semantic selectors with wasm-safe ready waits", async ({
  page,
}) => {
  await gotoSegmentedControl(page);

  const root = await resolveInteractiveRoot(page);
  const option0 = page.locator("#docs-segments-interactive-radio-0");
  const option1 = page.locator("#docs-segments-interactive-radio-1");
  const indicator = root.locator('[data-slot="segmented-control-indicator"]');

  await expect(root).toHaveAttribute("data-selection-origin", "programmatic");
  await expect(root).toHaveAttribute("data-selection-source", "external-selected");
  await expect(root).toHaveAttribute("data-selected-index", "0");
  await expect(option0).toHaveAttribute("aria-checked", "true");
  await expect(option1).toHaveAttribute("aria-checked", "false");

  await option1.click();
  await expect(root).toHaveAttribute("data-selection-origin", "pointer");
  await expect(root).toHaveAttribute("data-selected-index", "1");
  await expect(option1).toHaveAttribute("aria-checked", "true");
  await expect(option0).toHaveAttribute("aria-checked", "false");

  await expect(indicator).toHaveAttribute("style", new RegExp("--ui-segmented-control-indicator-o:\\s*1"));
});

test("docs-app segmented-control key flow is repeatable with semantic ready/settled breakpoints", async ({
  page,
}) => {
  await gotoSegmentedControl(page);

  let root = await resolveInteractiveRoot(page);
  let option0 = page.locator("#docs-segments-interactive-radio-0");
  let option1 = page.locator("#docs-segments-interactive-radio-1");

  await option0.focus();
  await expect(option0).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(root).toHaveAttribute("data-selection-origin", "keyboard");
  await expect(root).toHaveAttribute("data-selected-index", "1");
  await expect(option1).toHaveAttribute("aria-checked", "true");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  root = await resolveInteractiveRoot(page);
  option0 = page.locator("#docs-segments-interactive-radio-0");
  option1 = page.locator("#docs-segments-interactive-radio-1");

  await expect(root).toHaveAttribute("data-selection-origin", "programmatic");
  await expect(root).toHaveAttribute("data-selected-index", "0");
  await expect(option0).toHaveAttribute("aria-checked", "true");
  await expect(option1).toHaveAttribute("aria-checked", "false");
});
