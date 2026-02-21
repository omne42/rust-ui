import { expect, test } from "@playwright/test";

const CHART_PAGE = "/#/components/chart";

async function gotoChart(page) {
  await page.goto(CHART_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
}

function legendItem(root, index) {
  return root.locator(`[data-slot="chart-legend-item"][data-index="${index}"]`).first();
}

test("docs-app chart uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  await gotoChart(page);

  const controlledRoot = page
    .locator('[data-slot="chart-e2e-controlled-line"] [data-slot="chart"]')
    .first();
  const legend0 = legendItem(controlledRoot, 0);
  const legend1 = legendItem(controlledRoot, 1);
  const legend2 = legendItem(controlledRoot, 2);

  await expect(controlledRoot).toBeVisible();
  await expect(controlledRoot).toHaveAttribute("data-state", "ready");
  await expect(controlledRoot).toHaveAttribute("data-kind", "line");
  await expect(controlledRoot).toHaveAttribute("data-controlled", "true");
  await expect(controlledRoot).toHaveAttribute("data-class-source", "custom");
  await expect(controlledRoot).toHaveAttribute("role", "region");
  await expect(controlledRoot).toHaveAttribute("aria-label", "Quarterly growth line chart");
  await expect(controlledRoot).toHaveAttribute("data-active-value-source", "controlled");

  await legend0.focus();
  await expect(legend0).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(controlledRoot).toHaveAttribute("data-active-index", "1");
  await expect(controlledRoot).toHaveAttribute("data-active-interaction-source", "keyboard");
  await expect(legend1).toHaveAttribute("aria-pressed", "true");
  await expect(legend1).toHaveAttribute("data-active", "true");

  await legend2.hover();
  await expect(controlledRoot).toHaveAttribute("data-active-index", "2");
  await expect(controlledRoot).toHaveAttribute("data-active-interaction-source", "pointer");
  await expect(controlledRoot).toHaveAttribute("data-state", "ready");
  await expect(legend2).toHaveAttribute("aria-pressed", "true");

  const disabledRoot = page
    .locator('[data-slot="chart-e2e-state-disabled"] [data-slot="chart"]')
    .first();
  const disabledLegend0 = legendItem(disabledRoot, 0);
  await expect(disabledRoot).toHaveAttribute("data-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");
  await expect(disabledLegend0).toBeDisabled();
});

test("docs-app chart interaction uses semantic ready and settled breakpoints", async ({ page }) => {
  await gotoChart(page);

  const workbenchCanvas = page.locator('[data-slot="chart-workbench-canvas"]').first();
  const workbenchRoot = workbenchCanvas.locator('[data-slot="chart"]').first();
  const langSwitch = page
    .locator('[data-slot="chart-workbench-toggle-lang"] [data-slot="switch"]')
    .first();

  await expect(workbenchCanvas).toBeVisible();
  await expect(workbenchRoot).toBeVisible();
  await expect(workbenchRoot).toHaveAttribute("data-state", "ready");
  await expect(workbenchRoot).toHaveAttribute("data-uncontrolled", "true");
  await expect(workbenchRoot).toHaveAttribute("role", "region");
  await expect(workbenchRoot).not.toHaveAttribute("lang", "en-US");

  await langSwitch.click();
  await expect(workbenchRoot).toHaveAttribute("lang", "en-US");
  await expect(workbenchRoot).toHaveAttribute("data-state", "ready");
  await expect(workbenchRoot).toHaveAttribute("data-ui-state", "ready");
});

test("docs-app chart key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await gotoChart(page);

  await page.evaluate(() => {
    window.localStorage.removeItem("docs:chart:workbench:state");
  });
  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const controlledRoot = page
    .locator('[data-slot="chart-e2e-controlled-line"] [data-slot="chart"]')
    .first();
  const legend0 = legendItem(controlledRoot, 0);
  const legend1 = legendItem(controlledRoot, 1);
  const legend2 = legendItem(controlledRoot, 2);

  const workbenchRoot = page.locator('[data-slot="chart-workbench-canvas"] [data-slot="chart"]').first();
  const workbenchLegend0 = legendItem(workbenchRoot, 0);
  const disabledSwitch = page
    .locator('[data-slot="chart-workbench-toggle-disabled"] [data-slot="switch"]')
    .first();

  await expect(controlledRoot).toBeVisible();
  await expect(workbenchRoot).toBeVisible();
  await expect(workbenchRoot).toHaveAttribute("data-disabled", "false");
  await expect(workbenchRoot).toHaveAttribute("data-state", "ready");

  for (const cycle of [1, 2]) {
    await test.step(`chart key flow cycle ${cycle}`, async () => {
      await legend0.focus();
      await expect(legend0).toBeFocused();
      await page.keyboard.press("ArrowRight");
      await expect(controlledRoot).toHaveAttribute("data-active-index", "1");
      await expect(controlledRoot).toHaveAttribute("data-active-interaction-source", "keyboard");

      await legend1.focus();
      await expect(legend1).toBeFocused();
      await page.keyboard.press("ArrowRight");
      await expect(controlledRoot).toHaveAttribute("data-active-index", "2");
      await expect(legend2).toHaveAttribute("aria-pressed", "true");

      await disabledSwitch.click();
      await expect(workbenchRoot).toHaveAttribute("data-disabled", "true");
      await expect(workbenchRoot).toHaveAttribute("data-state", "disabled");
      await expect(workbenchLegend0).toBeDisabled();

      await disabledSwitch.click();
      await expect(workbenchRoot).toHaveAttribute("data-disabled", "false");
      await expect(workbenchRoot).toHaveAttribute("data-state", "ready");
      await expect(workbenchLegend0).toBeEnabled();
    });
  }

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  const reloadedWorkbenchRoot = page
    .locator('[data-slot="chart-workbench-canvas"] [data-slot="chart"]')
    .first();
  await expect(reloadedWorkbenchRoot).toHaveAttribute("data-disabled", "false");
  await expect(reloadedWorkbenchRoot).toHaveAttribute("data-state", "ready");
});
