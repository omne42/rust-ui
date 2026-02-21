import { expect, test } from "@playwright/test";

const WASM_READY_SELECTOR = "body:not(:has(#boot))";

async function waitForWasmReady(page) {
  await page.locator(WASM_READY_SELECTOR).waitFor();
}

function overlayForPanel(page, overlayPanel) {
  return page.locator('[data-slot="overlay"]').filter({ has: overlayPanel }).first();
}

async function expectDialogReady(page, overlayPanel, dialogRoot) {
  const overlayRoot = overlayForPanel(page, overlayPanel);
  await expect(overlayRoot).toHaveAttribute("data-state", "open");
  await expect(overlayRoot).toHaveAttribute("data-open", "true");
  await expect(overlayPanel).toBeVisible();
  await expect(overlayPanel).toHaveAttribute("aria-modal", "true");
  await expect(dialogRoot).toHaveAttribute("data-open", "true");
  await expect(dialogRoot).toHaveAttribute("data-ui-schema", "dialog");
  await expect(dialogRoot).toHaveAttribute("data-stream-mode", "snapshot");
  await expect(dialogRoot).toHaveAttribute("data-stream-fallback", "snapshot");
  await expect(dialogRoot).toHaveAttribute("data-output-status", "verified");
  return overlayRoot;
}

async function expectDialogSettledClosed(overlayPanel, dialogRoot, overlayRoot) {
  await expect(overlayPanel).toHaveCount(0);
  await expect(dialogRoot).toHaveCount(0);
  await expect(overlayRoot).toHaveCount(0);
}

test("docs-app dialog exposes stable role/source markers", async ({ page }) => {
  await page.goto("/#/components/dialog");
  await waitForWasmReady(page);

  await page.locator('[data-slot="dialog-e2e-open-marker"]').first().click();

  const overlayPanel = page.locator(
    '[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-dialog-marker-title"]',
  ).first();
  const dialogRoot = overlayPanel.locator('[data-slot="dialog"]').first();
  const overlayRoot = await expectDialogReady(page, overlayPanel, dialogRoot);

  await expect(dialogRoot).toHaveAttribute("data-state", "with-description");
  await expect(dialogRoot).toHaveAttribute("data-size", "lg");
  await expect(dialogRoot).toHaveAttribute("data-id-source", "custom");
  await expect(dialogRoot).toHaveAttribute("data-title-source", "custom");
  await expect(dialogRoot).toHaveAttribute("data-description-source", "custom");
  await expect(dialogRoot).toHaveAttribute("data-close-source", "custom");
  await expect(dialogRoot).toHaveAttribute("data-motion-source", "custom");

  await overlayPanel.locator('[data-slot="dialog-e2e-close-marker"]').first().click();
  await expectDialogSettledClosed(overlayPanel, dialogRoot, overlayRoot);
});

test("docs-app dialog closes via escape", async ({ page }) => {
  await page.goto("/#/components/dialog");
  await waitForWasmReady(page);

  await page.locator('[data-slot="dialog-e2e-open-default"]').first().click();

  const overlayPanel = page.locator(
    '[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-dialog-title"]',
  ).first();
  const dialogRoot = overlayPanel.locator('[data-slot="dialog"]').first();
  const overlayRoot = await expectDialogReady(page, overlayPanel, dialogRoot);

  await overlayPanel.press("Escape");
  await expectDialogSettledClosed(overlayPanel, dialogRoot, overlayRoot);
});

test("docs-app dialog interactive + comparison playgrounds stay contract-stable", async ({ page }) => {
  await page.goto("/#/components/dialog");
  await waitForWasmReady(page);

  const workbench = page.locator('[data-slot="dialog-workbench"]').first();
  await workbench.locator('[data-slot="dialog-e2e-open-workbench"]').first().click();
  const workbenchPanel = page.locator(
    '[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-dialog-workbench-title"]',
  ).first();
  const workbenchDialog = workbenchPanel.locator('[data-slot="dialog"]').first();
  const workbenchOverlay = await expectDialogReady(page, workbenchPanel, workbenchDialog);
  await expect(workbenchDialog).toHaveAttribute("data-state", "with-description");
  await expect(workbenchDialog).toHaveAttribute("data-close-button", "shown");
  await workbenchPanel.locator('[data-slot="dialog-e2e-close-workbench"]').first().click();
  await expectDialogSettledClosed(workbenchPanel, workbenchDialog, workbenchOverlay);

  const comparison = page.locator('[data-slot="dialog-scenario-compare"]').first();

  await comparison.locator('[data-slot="dialog-e2e-open-compare-default"]').first().click();
  const defaultPanel = page.locator(
    '[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-dialog-compare-default-title"]',
  ).first();
  const defaultDialog = defaultPanel.locator('[data-slot="dialog"]').first();
  const defaultOverlay = await expectDialogReady(page, defaultPanel, defaultDialog);
  await expect(defaultDialog).toHaveAttribute("data-state", "with-description");
  await expect(defaultDialog).toHaveAttribute("data-size", "md");
  await defaultPanel.locator('[data-slot="dialog-e2e-close-compare-default"]').first().click();
  await expectDialogSettledClosed(defaultPanel, defaultDialog, defaultOverlay);

  await comparison.locator('[data-slot="dialog-e2e-open-compare-compact"]').first().click();
  const compactPanel = page.locator(
    '[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-dialog-compare-compact-title"]',
  ).first();
  const compactDialog = compactPanel.locator('[data-slot="dialog"]').first();
  const compactOverlay = await expectDialogReady(page, compactPanel, compactDialog);
  await expect(compactDialog).toHaveAttribute("data-state", "title-only");
  await expect(compactDialog).toHaveAttribute("data-close-button", "hidden");
  await compactPanel.locator('[data-slot="dialog-e2e-close-compare-compact"]').first().click();
  await expectDialogSettledClosed(compactPanel, compactDialog, compactOverlay);

  await comparison.locator('[data-slot="dialog-e2e-open-compare-motion"]').first().click();
  const motionPanel = page.locator(
    '[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-dialog-compare-motion-title"]',
  ).first();
  const motionDialog = motionPanel.locator('[data-slot="dialog"]').first();
  const motionOverlay = await expectDialogReady(page, motionPanel, motionDialog);
  await expect(motionDialog).toHaveAttribute("data-motion-source", "custom");
  await motionPanel.locator('[data-slot="dialog-e2e-close-compare-motion"]').first().click();
  await expectDialogSettledClosed(motionPanel, motionDialog, motionOverlay);
});

test("docs-app dialog key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await page.goto("/#/components/dialog");
  await waitForWasmReady(page);

  const openDefaultButton = page.locator('[data-slot="dialog-e2e-open-default"]').first();
  await openDefaultButton.focus();
  await expect(openDefaultButton).toBeFocused();
  await page.keyboard.press("Enter");

  const defaultPanel = page.locator(
    '[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-dialog-title"]',
  ).first();
  const defaultDialog = defaultPanel.locator('[data-slot="dialog"]').first();
  const defaultOverlay = await expectDialogReady(page, defaultPanel, defaultDialog);
  await expect(defaultDialog).toHaveAttribute("data-state", "with-description");
  await expect(defaultDialog).toHaveAttribute("data-close-button", "shown");
  await defaultPanel.press("Escape");
  await expectDialogSettledClosed(defaultPanel, defaultDialog, defaultOverlay);

  await page.reload();
  await waitForWasmReady(page);

  const openWorkbenchButton = page.locator('[data-slot="dialog-e2e-open-workbench"]').first();
  await openWorkbenchButton.focus();
  await expect(openWorkbenchButton).toBeFocused();
  await page.keyboard.press("Enter");

  const workbenchPanel = page.locator(
    '[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-dialog-workbench-title"]',
  ).first();
  const workbenchDialog = workbenchPanel.locator('[data-slot="dialog"]').first();
  const workbenchOverlay = await expectDialogReady(page, workbenchPanel, workbenchDialog);
  await expect(workbenchDialog).toHaveAttribute("data-state", "with-description");
  await expect(workbenchDialog).toHaveAttribute("data-close-button", "shown");

  const closeWorkbenchButton = workbenchPanel
    .locator('[data-slot="dialog-e2e-close-workbench"]')
    .first();
  await closeWorkbenchButton.focus();
  await expect(closeWorkbenchButton).toBeFocused();
  await page.keyboard.press("Enter");
  await expectDialogSettledClosed(workbenchPanel, workbenchDialog, workbenchOverlay);
});
