import { expect, test } from "@playwright/test";

const WASM_READY_SELECTOR = "body:not(:has(#boot))";

async function waitForWasmReady(page) {
  await page.locator(WASM_READY_SELECTOR).waitFor();
}

function overlayForPanel(page, overlayPanel) {
  return page.locator('[data-slot="overlay"]').filter({ has: overlayPanel }).first();
}

async function expectAlertDialogReady(page, overlayPanel, alertDialogRoot) {
  const overlayRoot = overlayForPanel(page, overlayPanel);
  await expect(overlayRoot).toHaveAttribute("data-state", "open");
  await expect(overlayRoot).toHaveAttribute("data-open", "true");
  await expect(overlayPanel).toBeVisible();
  await expect(overlayPanel).toHaveAttribute("aria-modal", "true");
  await expect(alertDialogRoot).toHaveAttribute("data-open", "true");
  await expect(alertDialogRoot).toHaveAttribute("data-output-status", "verified");
  return overlayRoot;
}

async function expectAlertDialogSettledClosed(overlayPanel, alertDialogRoot, overlayRoot) {
  await expect(overlayPanel).toHaveCount(0);
  await expect(alertDialogRoot).toHaveCount(0);
  await expect(overlayRoot).toHaveCount(0);
}

test("docs-app alert-dialog exposes stable role/source markers", async ({ page }) => {
  await page.goto("/#/components/alert-dialog");
  await waitForWasmReady(page);

  await page
    .locator('[data-slot="alert-dialog-e2e-open-marker"] [data-slot="button"]')
    .first()
    .click();

  const overlayPanel = page
    .locator(
      '[data-slot="overlay-panel"][role="alertdialog"][aria-labelledby="docs-alert-marker-title"]',
    )
    .first();
  const alertDialog = overlayPanel.locator('[data-slot="alert-dialog"]').first();
  const overlayRoot = await expectAlertDialogReady(page, overlayPanel, alertDialog);

  await expect(alertDialog).toHaveAttribute("data-state", "open");
  await expect(alertDialog).toHaveAttribute("data-id-source", "custom");
  await expect(alertDialog).toHaveAttribute("data-title-source", "custom");
  await expect(alertDialog).toHaveAttribute("data-description-source", "custom");
  await expect(alertDialog).toHaveAttribute("data-cancel-source", "custom");
  await expect(alertDialog).toHaveAttribute("data-secondary-source", "custom");
  await expect(alertDialog).toHaveAttribute("data-motion-source", "custom");
  await expect(alertDialog).toHaveAttribute("data-auto-focus", "secondary");

  const secondaryButton = overlayPanel
    .locator('[data-secondary-source] [data-slot="button"]')
    .first();
  await expect(secondaryButton).toBeFocused();
  await expect(secondaryButton).toBeDisabled();
  await overlayPanel.press("Escape");
  await expectAlertDialogSettledClosed(overlayPanel, alertDialog, overlayRoot);
});

test("docs-app alert-dialog closes via escape", async ({ page }) => {
  await page.goto("/#/components/alert-dialog");
  await waitForWasmReady(page);

  await page
    .locator('[data-slot="alert-dialog-e2e-open-destructive"] [data-slot="button"]')
    .first()
    .click();

  const overlayPanel = page
    .locator(
      '[data-slot="overlay-panel"][role="alertdialog"][aria-labelledby="docs-alert-title"]',
    )
    .first();
  const alertDialog = overlayPanel.locator('[data-slot="alert-dialog"]').first();
  const overlayRoot = await expectAlertDialogReady(page, overlayPanel, alertDialog);

  await overlayPanel.press("Escape");
  await expectAlertDialogSettledClosed(overlayPanel, alertDialog, overlayRoot);
});

test("docs-app alert-dialog keeps disabled semantics and closes via pointer confirm", async ({ page }) => {
  await page.goto("/#/components/alert-dialog");
  await waitForWasmReady(page);

  await page
    .locator('[data-slot="alert-dialog-e2e-open-marker"] [data-slot="button"]')
    .first()
    .click();

  const overlayPanel = page
    .locator(
      '[data-slot="overlay-panel"][role="alertdialog"][aria-labelledby="docs-alert-marker-title"]',
    )
    .first();
  const alertDialog = overlayPanel.locator('[data-slot="alert-dialog"]').first();
  const overlayRoot = await expectAlertDialogReady(page, overlayPanel, alertDialog);

  await expect(alertDialog).toHaveAttribute("data-secondary-disabled", "true");

  const secondaryButton = overlayPanel
    .locator('[data-secondary-source] [data-slot="button"]')
    .first();
  await expect(secondaryButton).toBeDisabled();

  const confirmButton = overlayPanel.locator('[data-confirm-source] [data-slot="button"]').first();
  await expect(confirmButton).toBeEnabled();
  await confirmButton.click();

  await expectAlertDialogSettledClosed(overlayPanel, alertDialog, overlayRoot);
});

test("docs-app alert-dialog key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/alert-dialog");
  await waitForWasmReady(page);

  const openMarkerButton = page
    .locator('[data-slot="alert-dialog-e2e-open-marker"] [data-slot="button"]')
    .first();

  for (const cycle of [1, 2]) {
    await openMarkerButton.focus();
    await expect(openMarkerButton).toBeFocused();
    await page.keyboard.press("Enter");

    const overlayPanel = page
      .locator(
        '[data-slot="overlay-panel"][role="alertdialog"][aria-labelledby="docs-alert-marker-title"]',
      )
      .first();
    const alertDialog = overlayPanel.locator('[data-slot="alert-dialog"]').first();
    const overlayRoot = await expectAlertDialogReady(page, overlayPanel, alertDialog);

    await expect(alertDialog).toHaveAttribute("data-state", "open");
    await expect(alertDialog).toHaveAttribute("data-secondary-disabled", "true");
    await expect(alertDialog).toHaveAttribute("data-output-status", "verified");
    await expect(alertDialog).toHaveAttribute("data-ui-output-status", "verified");

    const confirmButton = overlayPanel.locator('[data-confirm-source] [data-slot="button"]').first();
    await confirmButton.focus();
    await expect(confirmButton).toBeFocused();
    await page.keyboard.press("Enter");
    await expectAlertDialogSettledClosed(overlayPanel, alertDialog, overlayRoot);

    if (cycle === 1) {
      await page.reload();
      await waitForWasmReady(page);
    }
  }
});

test("docs-app alert-dialog high-risk paths cover overlay focus keyboard and settled semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/alert-dialog");
  await waitForWasmReady(page);

  const openDestructiveButton = page
    .locator('[data-slot="alert-dialog-e2e-open-destructive"] [data-slot="button"]')
    .first();
  await openDestructiveButton.focus();
  await expect(openDestructiveButton).toBeFocused();
  await page.keyboard.press("Enter");

  const overlayPanel = page
    .locator('[data-slot="overlay-panel"][role="alertdialog"][aria-labelledby="docs-alert-title"]')
    .first();
  const alertDialog = overlayPanel.locator('[data-slot="alert-dialog"]').first();
  const overlayRoot = await expectAlertDialogReady(page, overlayPanel, alertDialog);

  await expect(alertDialog).toHaveAttribute("data-state", "open");
  await expect(alertDialog).toHaveAttribute("data-output-status", "verified");
  await expect(alertDialog).toHaveAttribute("data-ui-output-status", "verified");

  // AlertDialog component boundary has no async workflow; keep this as explicit N/A guard.
  await expect(overlayPanel.locator('[aria-busy="true"]')).toHaveCount(0);
  await expect(overlayPanel.locator('[data-loading="true"]')).toHaveCount(0);
  await expect(overlayPanel.locator('[data-state="loading"]')).toHaveCount(0);

  const actionButtons = overlayPanel.locator('[data-slot="alert-dialog"] [data-slot="button"]');
  await expect(actionButtons).toHaveCount(2);

  const cancelButton = actionButtons.nth(0);
  const confirmButton = actionButtons.nth(1);

  await confirmButton.focus();
  await expect(confirmButton).toBeFocused();
  await page.keyboard.press("Shift+Tab");
  await expect(cancelButton).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(confirmButton).toBeFocused();

  await overlayPanel.press("Escape");
  await expectAlertDialogSettledClosed(overlayPanel, alertDialog, overlayRoot);
});

test("docs-app alert-dialog interactive playground keeps config/code in sync with workbench controls", async ({
  page,
}) => {
  await page.goto("/#/components/alert-dialog");
  await waitForWasmReady(page);

  const docsRoot = page.locator('[data-component="alert-dialog"]').first();
  const playground = docsRoot
    .locator('section.playground:has([data-slot="alert-dialog-workbench"])')
    .first();
  await expect(playground).toBeVisible();

  await playground.locator('[data-slot="playground-toggle-settings"]').first().click();
  const controls = playground
    .locator('[data-slot="playground-controls"] [data-slot="alert-dialog-workbench-controls"]')
    .first();
  await expect(controls).toBeVisible();

  await controls.locator('[data-slot="segmented-control-option"][data-index="1"]').first().click();
  await controls.locator('label:has-text("Enable secondary action") input[type="checkbox"]').first().click();
  await controls.locator('label:has-text("Disable confirm") input[type="checkbox"]').first().click();
  await controls.locator('label:has-text("Disable secondary") input[type="checkbox"]').first().click();
  await controls.locator('label:has-text("Auto-focus secondary") input[type="checkbox"]').first().click();
  await controls.locator('label:has-text("Custom motion") input[type="checkbox"]').first().click();

  await playground.locator('[data-slot="alert-dialog-workbench-open"]').first().click();
  const overlayPanel = page
    .locator(
      '[data-slot="overlay-panel"][role="alertdialog"][aria-labelledby="docs-alert-workbench-title"]',
    )
    .first();
  const alertDialog = overlayPanel.locator('[data-slot="alert-dialog"]').first();
  const overlayRoot = await expectAlertDialogReady(page, overlayPanel, alertDialog);

  await expect(alertDialog).toHaveAttribute("data-motion-source", "custom");
  await expect(alertDialog).toHaveAttribute("data-auto-focus", "secondary");
  await expect(alertDialog).toHaveAttribute("data-confirm-disabled", "true");
  await expect(alertDialog).toHaveAttribute("data-secondary", "shown");
  await expect(alertDialog).toHaveAttribute("data-secondary-disabled", "true");

  await overlayPanel.press("Escape");
  await expectAlertDialogSettledClosed(overlayPanel, alertDialog, overlayRoot);

  await playground.locator('[data-slot="playground-toggle-code"]').first().click();
  const codeBlock = playground
    .locator('[data-slot="playground-code"] [data-slot="code-block-code"]')
    .first();
  await expect(codeBlock).toContainText("variant=AlertDialogVariant::Warning");
  await expect(codeBlock).toContainText("confirm_disabled=true");
  await expect(codeBlock).toContainText("secondary_disabled=true");

  await playground.locator('[data-slot="playground-toggle-test"]').first().click();
  const testPanel = playground.locator('[data-slot="playground-test"]').first();
  await expect(testPanel).toContainText("Actual config");
  await expect(testPanel).toContainText("variant: Warning");
  await expect(testPanel).toContainText("show_secondary: true");
  await expect(testPanel).toContainText("custom_motion: true");
});
