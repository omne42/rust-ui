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

async function dismissSheetPanelsBestEffort(page) {
  const sheetPanels = page.locator('[data-slot="sheet-panel"][role="dialog"]');
  for (const _ of [1, 2, 3, 4, 5, 6, 7, 8]) {
    if ((await sheetPanels.count()) === 0) {
      return;
    }
    const backdrops = page.locator('[data-slot="sheet-backdrop"]');
    if ((await backdrops.count()) > 0) {
      await backdrops.last().click({ force: true });
    } else {
      await page.keyboard.press("Escape");
    }
  }
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

test("docs-app bottom-sheet interactive playground keeps config/code in sync with workbench controls", async ({
  page,
}) => {
  await page.goto("/#/components/bottom-sheet");
  await waitForWasmReady(page);
  await dismissSheetPanelsBestEffort(page);

  const docsRoot = page.locator('[data-component="bottom-sheet"]').first();
  const playground = docsRoot
    .locator('section.playground:has([data-slot="bottom-sheet-workbench"])')
    .first();
  await expect(playground).toBeVisible();

  await playground.locator('[data-slot="playground-toggle-settings"]').first().click({ force: true });
  const controls = playground
    .locator('[data-slot="playground-controls"] [data-slot="bottom-sheet-workbench-controls"]')
    .first();
  await expect(controls).toBeVisible();

  await controls.locator('[data-slot="segmented-control-option"][data-index="2"]').first().click();
  await controls.locator('label:has-text("Show footer actions") input[type="checkbox"]').first().click();
  await controls.locator('label:has-text("Detached mode") input[type="checkbox"]').first().click();
  await controls.locator('label:has-text("Show close button") input[type="checkbox"]').first().click();
  await controls.locator('label:has-text("Custom motion") input[type="checkbox"]').first().click();

  await playground.locator('[data-slot="bottom-sheet-workbench-open"]').first().click();
  const workbenchPanel = page
    .locator(
      '[data-slot="sheet-panel"][role="dialog"][aria-labelledby="docs-bottom-sheet-workbench-title"]',
    )
    .first();
  const workbenchRoot = workbenchPanel.locator('[data-slot="bottom-sheet"]').first();
  const workbenchSheet = await expectBottomSheetReady(page, workbenchPanel, workbenchRoot);

  await expect(workbenchRoot).toHaveAttribute("data-description", "present");
  await expect(workbenchRoot).toHaveAttribute("data-footer", "absent");
  await expect(workbenchRoot).toHaveAttribute("data-detached", "true");
  await expect(workbenchRoot).toHaveAttribute("data-close-button", "hidden");
  await expect(workbenchRoot).toHaveAttribute("data-motion-source", "custom");

  await workbenchPanel.press("Escape");
  await expectBottomSheetSettledClosed(workbenchPanel, workbenchRoot, workbenchSheet);

  await playground.locator('[data-slot="playground-toggle-code"]').first().click();
  const codeBlock = playground
    .locator('[data-slot="playground-code"] [data-slot="code-block-code"]')
    .first();
  await expect(codeBlock).toContainText('title="Install update".to_string()');
  await expect(codeBlock).toContainText("is_detached=true");
  await expect(codeBlock).toContainText("is_close_button_visible=false");
  await expect(codeBlock).toContainText("motion=BottomSheetMotion");
  await expect(codeBlock).not.toContainText("footer=move ||");

  await playground.locator('[data-slot="playground-toggle-test"]').first().click();
  const testPanel = playground.locator('[data-slot="playground-test"]').first();
  await expect(testPanel).toContainText("Actual config");
  await expect(testPanel).toContainText("show_footer: false");
  await expect(testPanel).toContainText("is_detached: true");
  await expect(testPanel).toContainText("custom_motion: true");
});
