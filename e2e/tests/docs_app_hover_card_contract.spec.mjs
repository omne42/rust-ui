import { expect, test } from "@playwright/test";

const WASM_READY_SELECTOR = "body:not(:has(#boot))";

async function waitForWasmReady(page) {
  await page.locator(WASM_READY_SELECTOR).waitFor();
}

function docsRoot(page) {
  return page.locator('[data-component="hover-card"]').first();
}

function interactiveControls(page) {
  return docsRoot(page).locator('[data-slot="hover-card-e2e-controls"]').first();
}

function interactiveCanvas(page) {
  return docsRoot(page).locator('[data-slot="hover-card-e2e-canvas"]').first();
}

function interactiveHoverCardRoot(page) {
  return interactiveCanvas(page).locator('[data-slot="hover-card"]').first();
}

function interactiveHoverCardPanel(page) {
  return page.locator('[data-slot="hover-card-panel"][id="docs-hover-card-interactive"]').first();
}

async function expectHoverCardReady(root, panel) {
  await expect(root).toHaveAttribute("data-state", "open");
  await expect(root).toHaveAttribute("data-open", "true");
  await expect(root).toHaveAttribute("data-open-mode", "controlled");
  await expect(root).toHaveAttribute("data-open-value-source", "controlled");
  await expect(root).toHaveAttribute("data-open-intent-source", "interaction");
  await expect(root).toHaveAttribute("data-ui-output-status", "verified");

  await expect(panel).toBeVisible();
  await expect(panel).toHaveAttribute("role", "tooltip");
  await expect(panel).toHaveAttribute("data-state", "panel");
  await expect(panel).toHaveAttribute("data-open", "true");
  await expect(panel).toHaveAttribute("data-open-mode", "controlled");
  await expect(panel).toHaveAttribute("data-open-value-source", "controlled");
  await expect(panel).toHaveAttribute("data-open-intent-source", "interaction");
}

async function expectHoverCardSettledClosed(root, panel) {
  await expect(panel).toHaveCount(0);
  await expect(root).toHaveAttribute("data-state", "closed");
  await expect(root).toHaveAttribute("data-closed", "true");
}

test("docs-app hover-card contract uses semantic selectors with settled waits", async ({
  page,
}) => {
  await page.goto("/#/components/hover-card");
  await waitForWasmReady(page);

  const root = docsRoot(page);
  await expect(root).toBeVisible();

  const controls = interactiveControls(page);
  const canvas = interactiveCanvas(page);
  const openButton = controls.locator('[data-slot="hover-card-e2e-open"]').first();
  const closeButton = controls.locator('[data-slot="hover-card-e2e-close"]').first();
  const triggerButton = canvas.locator('[data-slot="hover-card-e2e-trigger"]').first();

  await expect(openButton).toBeVisible();
  await expect(closeButton).toBeVisible();
  await expect(triggerButton).toBeVisible();

  await openButton.click();

  const interactiveRoot = interactiveHoverCardRoot(page);
  const panel = interactiveHoverCardPanel(page);
  await expectHoverCardReady(interactiveRoot, panel);

  await closeButton.click();
  await expectHoverCardSettledClosed(interactiveRoot, panel);
});

test("docs-app hover-card contract covers ready and settled semantic breakpoints for overlay paths", async ({
  page,
}) => {
  await page.goto("/#/components/hover-card");
  await waitForWasmReady(page);

  const root = docsRoot(page);
  await expect(root).toBeVisible();

  const openButton = interactiveControls(page).locator('[data-slot="hover-card-e2e-open"]').first();
  await openButton.click();

  const interactiveRoot = interactiveHoverCardRoot(page);
  const panel = interactiveHoverCardPanel(page);
  await expectHoverCardReady(interactiveRoot, panel);

  await panel.press("Escape");
  await expectHoverCardSettledClosed(interactiveRoot, panel);

  await openButton.click();
  await expectHoverCardReady(interactiveRoot, panel);
  await panel.press("Escape");
  await expectHoverCardSettledClosed(interactiveRoot, panel);
});

test("docs-app hover-card key flow is repeatable and failure points are semantic", async ({
  page,
}) => {
  await page.goto("/#/components/hover-card");
  await waitForWasmReady(page);

  const root = docsRoot(page);
  await expect(root).toBeVisible();

  const controls = interactiveControls(page);
  const openButton = controls.locator('[data-slot="hover-card-e2e-open"]').first();
  const interactiveRoot = interactiveHoverCardRoot(page);
  const panel = interactiveHoverCardPanel(page);

  for (const cycle of [1, 2]) {
    await test.step(`hover-card key flow cycle ${cycle}`, async () => {
      await openButton.focus();
      await expect(openButton).toBeFocused();
      await page.keyboard.press("Enter");

      await expectHoverCardReady(interactiveRoot, panel);
      await expect(panel).toHaveAttribute("data-open-intent-source", "interaction");

      await page.keyboard.press("Escape");
      await expectHoverCardSettledClosed(interactiveRoot, panel);
      await expect(openButton).toBeFocused();
    });
  }
});

test("docs-app hover-card high-risk paths cover focus keyboard and settled semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/hover-card");
  await waitForWasmReady(page);

  const root = docsRoot(page);
  await expect(root).toBeVisible();

  const controls = interactiveControls(page);
  const canvas = interactiveCanvas(page);
  const closeButton = controls.locator('[data-slot="hover-card-e2e-close"]').first();
  const triggerButton = canvas.locator('[data-slot="hover-card-e2e-trigger"]').first();
  const interactiveRoot = interactiveHoverCardRoot(page);
  const panel = interactiveHoverCardPanel(page);

  await triggerButton.hover();
  await expectHoverCardReady(interactiveRoot, panel);
  await page.keyboard.press("Escape");
  await expectHoverCardSettledClosed(interactiveRoot, panel);

  await triggerButton.focus();
  await expect(triggerButton).toBeFocused();
  await expectHoverCardReady(interactiveRoot, panel);
  await closeButton.click();
  await expectHoverCardSettledClosed(interactiveRoot, panel);
});
