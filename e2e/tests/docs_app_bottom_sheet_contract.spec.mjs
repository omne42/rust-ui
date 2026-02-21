import { expect, test } from "@playwright/test";

const WASM_READY_SELECTOR = "body:not(:has(#boot))";

async function waitForWasmReady(page) {
  await page.locator(WASM_READY_SELECTOR).waitFor();
}

function sheetForPanel(page, panel) {
  return page.locator('[data-slot="sheet"]').filter({ has: panel }).first();
}

async function expectFocusInsidePanel(panel) {
  await expect
    .poll(async () => panel.evaluate((node) => node.contains(node.ownerDocument.activeElement)))
    .toBe(true);
}

async function closeAllSheetPanels(page) {
  const sheetPanels = page.locator('[data-slot="sheet-panel"][role="dialog"]');
  for (const _ of [1, 2, 3, 4, 5, 6]) {
    if ((await sheetPanels.count()) === 0) {
      break;
    }
    await page.keyboard.press("Escape");
  }
  await expect(sheetPanels).toHaveCount(0);
}

async function expectBottomSheetReady(page, panel, bottomSheetRoot) {
  const sheetRoot = sheetForPanel(page, panel);
  await expect(sheetRoot).toHaveAttribute("data-state", "open");
  await expect(sheetRoot).toHaveAttribute("data-open", "true");
  await expect(sheetRoot).toHaveAttribute("data-placement", "bottom");
  await expect(panel).toBeVisible();
  await expect(panel).toHaveAttribute("aria-modal", "true");
  await expect(bottomSheetRoot).toHaveAttribute("data-ui-output-status", "verified");
  return sheetRoot;
}

async function expectBottomSheetSettledClosed(panel, bottomSheetRoot, sheetRoot) {
  await expect(panel).toHaveCount(0);
  await expect(bottomSheetRoot).toHaveCount(0);
  await expect(sheetRoot).toHaveCount(0);
}

test("docs-app bottom-sheet contract uses semantic selectors with settled waits", async ({ page }) => {
  await page.goto("/#/components/bottom-sheet");
  await waitForWasmReady(page);
  await closeAllSheetPanels(page);

  const docsRoot = page.locator('[data-component="bottom-sheet"]').first();
  await expect(docsRoot).toBeVisible();

  const controls = docsRoot.locator('[data-slot="bottom-sheet-e2e-semantic-controls"]').first();
  const openSemantic = controls
    .locator('[data-slot="bottom-sheet-e2e-open-semantic"] [data-slot="button"]')
    .first();
  await openSemantic.click();

  const semanticPanel = page
    .locator(
      '[data-slot="sheet-panel"][role="dialog"][aria-labelledby="docs-bottom-sheet-semantic-title"]',
    )
    .first();
  const semanticRoot = semanticPanel.locator('[data-slot="bottom-sheet"]').first();
  const semanticSheet = await expectBottomSheetReady(page, semanticPanel, semanticRoot);

  await expect(semanticRoot).toHaveAttribute("data-state", "with-description");
  await expect(semanticRoot).toHaveAttribute("data-description", "present");
  await expect(semanticRoot).toHaveAttribute("data-footer", "present");
  await expect(semanticRoot).toHaveAttribute("data-close-button", "shown");
  await expect(semanticRoot).toHaveAttribute("data-motion-source", "default");

  await semanticPanel.press("Escape");
  await expectBottomSheetSettledClosed(semanticPanel, semanticRoot, semanticSheet);
});

test("docs-app bottom-sheet motion path uses semantic ready and settled breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/bottom-sheet");
  await waitForWasmReady(page);
  await closeAllSheetPanels(page);

  const docsRoot = page.locator('[data-component="bottom-sheet"]').first();
  await expect(docsRoot).toBeVisible();

  const controls = docsRoot.locator('[data-slot="bottom-sheet-e2e-motion-controls"]').first();
  const openMotion = controls
    .locator('[data-slot="bottom-sheet-e2e-open-motion"] [data-slot="button"]')
    .first();
  await openMotion.click();

  const motionPanel = page
    .locator(
      '[data-slot="sheet-panel"][role="dialog"][aria-labelledby="docs-bottom-sheet-motion-title"]',
    )
    .first();
  const motionRoot = motionPanel.locator('[data-slot="bottom-sheet"]').first();
  const motionSheet = await expectBottomSheetReady(page, motionPanel, motionRoot);

  await expect(motionRoot).toHaveAttribute("data-state", "with-description");
  await expect(motionRoot).toHaveAttribute("data-description", "present");
  await expect(motionRoot).toHaveAttribute("data-motion-source", "custom");
  await expect(motionRoot).toHaveAttribute("data-custom-motion", "true");
  await expect(motionSheet).toHaveAttribute("data-motion-source", "custom");

  const backdrop = motionSheet.locator('[data-slot="sheet-backdrop"]').first();
  await backdrop.click();
  await expectBottomSheetSettledClosed(motionPanel, motionRoot, motionSheet);
});

test("docs-app bottom-sheet key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await page.goto("/#/components/bottom-sheet");
  await waitForWasmReady(page);
  await closeAllSheetPanels(page);

  const docsRoot = page.locator('[data-component="bottom-sheet"]').first();
  await expect(docsRoot).toBeVisible();

  const controls = docsRoot.locator('[data-slot="bottom-sheet-e2e-semantic-controls"]').first();
  const openSemantic = controls
    .locator('[data-slot="bottom-sheet-e2e-open-semantic"] [data-slot="button"]')
    .first();

  for (const cycle of [1, 2]) {
    await test.step(`bottom-sheet key flow cycle ${cycle}`, async () => {
      await openSemantic.focus();
      await expect(openSemantic).toBeFocused();
      await page.keyboard.press("Enter");

      const semanticPanel = page
        .locator(
          '[data-slot="sheet-panel"][role="dialog"][aria-labelledby="docs-bottom-sheet-semantic-title"]',
        )
        .first();
      const semanticRoot = semanticPanel.locator('[data-slot="bottom-sheet"]').first();
      const semanticSheet = await expectBottomSheetReady(page, semanticPanel, semanticRoot);

      await expect(semanticRoot).toHaveAttribute("data-state", "with-description");
      await expect(semanticRoot).toHaveAttribute("data-description", "present");
      await expect(semanticRoot).toHaveAttribute("data-footer", "present");
      await expect(semanticRoot).toHaveAttribute("data-close-button", "shown");
      await expect(semanticSheet).toHaveAttribute("data-keyboard-dismiss", "enabled");

      await expectFocusInsidePanel(semanticPanel);
      await page.keyboard.press("Tab");
      await expectFocusInsidePanel(semanticPanel);

      await semanticPanel.press("Escape");
      await expectBottomSheetSettledClosed(semanticPanel, semanticRoot, semanticSheet);
      await expect(openSemantic).toBeFocused();
    });
  }
});

test("docs-app bottom-sheet high-risk paths keep overlay focus keyboard and settled semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/bottom-sheet");
  await waitForWasmReady(page);
  await closeAllSheetPanels(page);

  const docsRoot = page.locator('[data-component="bottom-sheet"]').first();
  await expect(docsRoot).toBeVisible();

  const controls = docsRoot.locator('[data-slot="bottom-sheet-e2e-motion-controls"]').first();
  const openMotion = controls
    .locator('[data-slot="bottom-sheet-e2e-open-motion"] [data-slot="button"]')
    .first();

  await openMotion.focus();
  await expect(openMotion).toBeFocused();
  await page.keyboard.press("Enter");

  const motionPanel = page
    .locator(
      '[data-slot="sheet-panel"][role="dialog"][aria-labelledby="docs-bottom-sheet-motion-title"]',
    )
    .first();
  const motionRoot = motionPanel.locator('[data-slot="bottom-sheet"]').first();
  const motionSheet = await expectBottomSheetReady(page, motionPanel, motionRoot);

  await expect(motionRoot).toHaveAttribute("data-motion-source", "custom");
  await expect(motionRoot).toHaveAttribute("data-custom-motion", "true");
  await expect(motionSheet).toHaveAttribute("data-motion-source", "custom");
  await expect(motionSheet).toHaveAttribute("data-keyboard-dismiss", "enabled");

  await expectFocusInsidePanel(motionPanel);
  await page.keyboard.press("Tab");
  await expectFocusInsidePanel(motionPanel);
  await page.keyboard.press("Shift+Tab");
  await expectFocusInsidePanel(motionPanel);

  const backdrop = motionSheet.locator('[data-slot="sheet-backdrop"]').first();
  await backdrop.click();
  await expectBottomSheetSettledClosed(motionPanel, motionRoot, motionSheet);
});
