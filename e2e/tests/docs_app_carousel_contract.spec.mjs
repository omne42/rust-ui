import { expect, test } from "@playwright/test";

const WASM_READY_SELECTOR = "body:not(:has(#boot))";

async function waitForWasmReady(page) {
  await page.locator(WASM_READY_SELECTOR).waitFor();
}

async function expectCarouselReady(carouselRoot) {
  await expect(carouselRoot).toBeVisible();
  await expect(carouselRoot).toHaveAttribute("data-ui-schema", "ui.carousel.agent");
  await expect(carouselRoot).toHaveAttribute(
    "data-state",
    /selected|focused|idle|empty/,
  );
  await expect(carouselRoot).toHaveAttribute("data-ui-output-status", "verified");
}

async function expectCarouselSettledSelection(carouselRoot, selectedIndex) {
  const selectedIndexAttr = String(selectedIndex);
  await expect(carouselRoot).toHaveAttribute("data-selected-index", selectedIndexAttr);
  await expect(carouselRoot).toHaveAttribute("data-selection", "selected");

  const selectedIndicator = carouselRoot
    .locator(`[data-slot="carousel-indicator"][data-index="${selectedIndexAttr}"]`)
    .first();
  await expect(selectedIndicator).toHaveAttribute("data-selected", "true");
}

test("docs-app carousel contract uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  await page.goto("/#/components/carousel");
  await waitForWasmReady(page);

  const docsRoot = page.locator('[data-component="carousel"]').first();
  await expect(docsRoot).toBeVisible();

  const matrixScope = docsRoot.locator('[data-slot="carousel-state-matrix"]').first();
  const matrixCarousel = matrixScope.locator('[data-slot="carousel"]').first();
  await expectCarouselReady(matrixCarousel);

  await expect(matrixCarousel).toHaveAttribute("role", /.+/);
  await expect(matrixCarousel).toHaveAttribute(
    "data-selection-mode",
    /controlled|uncontrolled/,
  );
  await expect(matrixCarousel.locator('[data-slot="carousel-controls"]').first()).toHaveAttribute(
    "role",
    "toolbar",
  );
  await expect(
    matrixCarousel.locator('[data-slot="carousel-indicators"]').first(),
  ).toHaveAttribute("role", "group");

  const controlledUncontrolledScope = docsRoot
    .locator('[data-slot="carousel-controlled-uncontrolled"]')
    .first();
  const controlledCarousel = controlledUncontrolledScope
    .locator('[data-slot="carousel"][data-selection-mode="controlled"]')
    .first();
  const uncontrolledCarousel = controlledUncontrolledScope
    .locator('[data-slot="carousel"][data-selection-mode="uncontrolled"]')
    .first();

  await expect(controlledCarousel).toHaveAttribute("data-selected-index-source", "external");
  await expect(uncontrolledCarousel).toHaveAttribute("data-selected-index-source", "default");
});

test("docs-app carousel motion interaction uses semantic ready and settled breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/carousel");
  await waitForWasmReady(page);

  const docsRoot = page.locator('[data-component="carousel"]').first();
  await expect(docsRoot).toBeVisible();

  const markerScope = docsRoot.locator('[data-slot="carousel-e2e-markers"]').first();
  const markerCarousel = markerScope.locator('[data-slot="carousel"]').first();
  await expectCarouselReady(markerCarousel);
  await expect(markerCarousel).toHaveAttribute("data-motion-source", "custom");
  await expect(markerCarousel).toHaveAttribute("data-selection-mode", "controlled");
  await expect(markerCarousel).toHaveAttribute("data-selected-index-source", "external");

  const selectOverview = markerScope.locator('[data-slot="carousel-e2e-select-overview"]').first();
  const selectAnalytics = markerScope
    .locator('[data-slot="carousel-e2e-select-analytics"]')
    .first();
  const clearSelection = markerScope.locator('[data-slot="carousel-e2e-clear"]').first();
  await expect(selectOverview).toBeVisible();
  await expect(selectAnalytics).toBeVisible();
  await expect(clearSelection).toBeVisible();

  await selectOverview.click();
  await expectCarouselSettledSelection(markerCarousel, 0);
  await selectAnalytics.click();
  await expectCarouselSettledSelection(markerCarousel, 1);
  await clearSelection.click();
  await expect(markerCarousel).not.toHaveAttribute("data-selected-index", /[0-9]+/);
  await expect(markerCarousel).toHaveAttribute("data-selection", "idle");
});

test("docs-app carousel key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await page.goto("/#/components/carousel");
  await waitForWasmReady(page);

  const docsRoot = page.locator('[data-component="carousel"]').first();
  await expect(docsRoot).toBeVisible();

  const workbenchScope = docsRoot.locator('[data-slot="carousel-workbench"]').first();
  const workbenchCarousel = workbenchScope.locator('[data-slot="carousel"]').first();
  await expectCarouselReady(workbenchCarousel);

  const select0 = workbenchScope.locator('[data-slot="carousel-workbench-select-0"]').first();
  const select1 = workbenchScope.locator('[data-slot="carousel-workbench-select-1"]').first();
  const clear = workbenchScope.locator('[data-slot="carousel-workbench-clear"]').first();
  const prev = workbenchCarousel.locator('[data-slot="carousel-prev"]').first();
  const next = workbenchCarousel.locator('[data-slot="carousel-next"]').first();

  for (const cycle of [1, 2]) {
    await test.step(`carousel key flow cycle ${cycle}`, async () => {
      await select0.click();
      await expectCarouselSettledSelection(workbenchCarousel, 0);
      await expect(workbenchCarousel).toHaveAttribute("data-selection-mode", "controlled");
      await expect(workbenchCarousel).toHaveAttribute("data-selected-index-source", "external");
      await expect(workbenchCarousel).toHaveAttribute("data-selected-index-change-source", "custom");

      await next.focus();
      await expect(next).toBeFocused();
      await page.keyboard.press("Enter");
      await expectCarouselSettledSelection(workbenchCarousel, 1);

      await prev.focus();
      await expect(prev).toBeFocused();
      await page.keyboard.press("Enter");
      await expectCarouselSettledSelection(workbenchCarousel, 0);

      await select1.focus();
      await expect(select1).toBeFocused();
      await page.keyboard.press("Enter");
      await expectCarouselSettledSelection(workbenchCarousel, 1);

      await clear.focus();
      await expect(clear).toBeFocused();
      await page.keyboard.press("Enter");
      await expect(workbenchCarousel).not.toHaveAttribute("data-selected-index", /[0-9]+/);
      await expect(workbenchCarousel).toHaveAttribute("data-selection", "idle");
    });
  }

  await page.reload();
  await waitForWasmReady(page);

  const reloadedWorkbench = page
    .locator('[data-slot="carousel-workbench"] [data-slot="carousel"]')
    .first();
  await expectCarouselReady(reloadedWorkbench);
});

test("docs-app carousel high-risk paths keep focus keyboard and settled semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/carousel");
  await waitForWasmReady(page);

  const docsRoot = page.locator('[data-component="carousel"]').first();
  await expect(docsRoot).toBeVisible();

  const markerScope = docsRoot.locator('[data-slot="carousel-e2e-markers"]').first();
  const markerCarousel = markerScope.locator('[data-slot="carousel"]').first();
  await expectCarouselReady(markerCarousel);

  const indicator0 = markerCarousel
    .locator('[data-slot="carousel-indicator"][data-index="0"]')
    .first();
  const indicator1 = markerCarousel
    .locator('[data-slot="carousel-indicator"][data-index="1"]')
    .first();

  await indicator0.focus();
  await expect(indicator0).toBeFocused();
  await expect(indicator0).toHaveAttribute("data-focused", "true");
  await page.keyboard.press("Enter");
  await expectCarouselSettledSelection(markerCarousel, 0);

  await indicator1.focus();
  await expect(indicator1).toBeFocused();
  await expect(indicator1).toHaveAttribute("data-focused", "true");
  await page.keyboard.press("Enter");
  await expectCarouselSettledSelection(markerCarousel, 1);

  await markerCarousel.focus();
  await expect(markerCarousel).toBeFocused();
  await page.keyboard.press("ArrowLeft");
  await expectCarouselSettledSelection(markerCarousel, 0);
  await page.keyboard.press("ArrowRight");
  await expectCarouselSettledSelection(markerCarousel, 1);

  await expect(markerCarousel).toHaveAttribute("data-motion-source", "custom");
  await expect(markerCarousel).toHaveAttribute("data-selection-mode", "controlled");
  await expect(markerCarousel).toHaveAttribute("data-selected-index-source", "external");
  await expect(markerCarousel).toHaveAttribute("data-selected-index-change-source", "custom");
  await expect(markerCarousel).toHaveAttribute("data-ui-output-status", "verified");
});
