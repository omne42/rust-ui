import { expect, test } from "@playwright/test";

const COLOR_AREA_PAGE = "/#/components/color-area";
const CONTROLLED_ROOT =
  '[data-component="color-area"] [data-slot="color-area"][data-value-control-mode="controlled"][data-grid-size="11"]';

async function gotoColorArea(page) {
  await page.goto(COLOR_AREA_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
}

async function resolveControlledColorAreaRoot(page) {
  const root = page.locator(CONTROLLED_ROOT).first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-ui-schema", "ui.color-area.agent-contract.v1");
  await expect(root).toHaveAttribute("data-ui-stream-support", "optional");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(root).toHaveAttribute("data-ui-stream-mode", "snapshot");
  return root;
}

test("docs-app color-area contract uses semantic selectors with wasm-safe ready waits", async ({
  page,
}) => {
  await gotoColorArea(page);
  const root = await resolveControlledColorAreaRoot(page);

  await expect(root).toHaveAttribute("data-value-control-mode", "controlled");
  await expect(root).toHaveAttribute("data-value-source", "external");
  await expect(root).toHaveAttribute("data-ui-intent", "select-color-point");
  await expect(root).toHaveAttribute("data-ui-action", "select");
  await expect(root).toHaveAttribute("data-value-x", "60");
  await expect(root).toHaveAttribute("data-value-y", "40");
});

test("docs-app color-area key flow uses semantic breakpoints with explicit settled conditions", async ({
  page,
}) => {
  await gotoColorArea(page);
  const root = await resolveControlledColorAreaRoot(page);

  await root.focus();
  await expect(root).toBeFocused();

  await page.keyboard.press("ArrowRight");
  await expect(root).toHaveAttribute("data-value-x", "70");
  await expect(root).toHaveAttribute("data-selected-col", "7");
  await expect(root).toHaveAttribute("data-ui-source", "external");

  await page.keyboard.press("ArrowUp");
  await expect(root).toHaveAttribute("data-value-y", "50");
  await expect(root).toHaveAttribute("data-selected-row", "5");

  const disabledRoot = page
    .locator('[data-component="color-area"] [data-slot="color-area"][data-disabled="true"]')
    .first();
  await expect(disabledRoot).toBeVisible();
  await expect(disabledRoot).toHaveAttribute("data-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-disabled-source", "is-prop");

  const disabledX = disabledRoot.locator('[data-slot="color-area-axis-x"]').first();
  await expect(disabledX).toHaveAttribute("aria-disabled", "true");
});

test("docs-app color-area key flow is repeatable and failures map to semantic breakpoints", async ({
  page,
}) => {
  await gotoColorArea(page);
  let root = await resolveControlledColorAreaRoot(page);

  await root.focus();
  await page.keyboard.press("ArrowRight");
  await expect(root).toHaveAttribute("data-value-x", "70");
  await expect(root).toHaveAttribute("data-ui-output-status", "verified");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  root = await resolveControlledColorAreaRoot(page);
  await expect(root).toHaveAttribute("data-value-x", "60");
  await expect(root).toHaveAttribute("data-value-y", "40");

  await root.focus();
  await page.keyboard.press("ArrowRight");
  await expect(root).toHaveAttribute("data-value-x", "70");
  await expect(root).toHaveAttribute("data-ui-action", "select");
});
