import { expect, test } from "@playwright/test";

const WASM_READY_SELECTOR = "body:not(:has(#boot))";

async function waitForWasmReady(page) {
  await page.locator(WASM_READY_SELECTOR).waitFor();
}

function docsRoot(page) {
  return page.locator('[data-component="tooltip"]').first();
}

function controls(page) {
  return docsRoot(page).locator('[data-slot="tooltip-workbench-controls"]').first();
}

function controlledTooltipRoot(page) {
  return docsRoot(page)
    .locator('[data-slot="tooltip"][data-open-mode="controlled"]')
    .first();
}

function tooltipPanel(page) {
  return page.locator('[data-slot="tooltip-panel"][id="docs-tooltip-workbench"]').first();
}

async function expectTooltipReady(root, panel) {
  await expect(root).toHaveAttribute("data-state", "open");
  await expect(root).toHaveAttribute("data-open", "true");
  await expect(root).toHaveAttribute("data-open-mode", "controlled");

  await expect(panel).toBeVisible();
  await expect(panel).toHaveAttribute("role", "tooltip");
  await expect(panel).toHaveAttribute("data-state", "panel");
  await expect(panel).toHaveAttribute("data-open", "true");
  await expect(panel).toHaveAttribute("data-open-mode", "controlled");
}

async function expectTooltipSettledClosed(root, panel) {
  await expect(panel).toHaveCount(0);
  await expect(root).toHaveAttribute("data-state", "closed");
  await expect(root).toHaveAttribute("data-closed", "true");
}

test("docs-app tooltip contract uses semantic selectors with settled waits", async ({ page }) => {
  await page.goto("/#/components/tooltip");
  await waitForWasmReady(page);

  const root = docsRoot(page);
  await expect(root).toBeVisible();

  const openButton = controls(page).locator('[data-slot="tooltip-e2e-open"]').first();
  const closeButton = controls(page).locator('[data-slot="tooltip-e2e-close"]').first();
  const triggerButton = root.locator('[data-slot="tooltip-e2e-trigger"]').first();

  await expect(openButton).toBeVisible();
  await expect(closeButton).toBeVisible();
  await expect(triggerButton).toBeVisible();

  await openButton.click();

  const tooltipRoot = controlledTooltipRoot(page);
  const panel = tooltipPanel(page);
  await expectTooltipReady(tooltipRoot, panel);

  await closeButton.click();
  await expectTooltipSettledClosed(tooltipRoot, panel);
});

test("docs-app tooltip key flow is replayable with semantic breakpoints", async ({ page }) => {
  await page.goto("/#/components/tooltip");
  await waitForWasmReady(page);

  const root = docsRoot(page);
  await expect(root).toBeVisible();

  const openButton = controls(page).locator('[data-slot="tooltip-e2e-open"]').first();
  const tooltipRoot = controlledTooltipRoot(page);
  const panel = tooltipPanel(page);

  for (const cycle of [1, 2]) {
    await test.step(`tooltip key flow cycle ${cycle}`, async () => {
      await openButton.click();
      await expectTooltipReady(tooltipRoot, panel);

      await panel.press("Escape");
      await expectTooltipSettledClosed(tooltipRoot, panel);
    });
  }
});
