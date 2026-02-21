import { expect, test } from "@playwright/test";

const WASM_READY_SELECTOR = "body:not(:has(#boot))";

async function waitForWasmReady(page) {
  await page.locator(WASM_READY_SELECTOR).waitFor();
}

function overlayForPanel(page, overlayPanel) {
  return page.locator('[data-slot="overlay"]').filter({ has: overlayPanel }).first();
}

async function expectFocusInsidePanel(panel) {
  await expect
    .poll(async () => panel.evaluate((node) => node.contains(node.ownerDocument.activeElement)))
    .toBe(true);
}

async function closeDrawerPanelByEscape(page, ariaLabelledBy) {
  const panel = page.locator(
    `[data-slot="overlay-panel"][role="dialog"][aria-labelledby="${ariaLabelledBy}"]`,
  );

  for (const _ of [1, 2, 3, 4]) {
    if ((await panel.count()) === 0) {
      break;
    }
    await page.keyboard.press("Escape");
  }

  await expect(panel).toHaveCount(0);
}

async function ensureDrawerBaseline(page) {
  await closeDrawerPanelByEscape(page, "docs-drawer-minimal-title");
  await closeDrawerPanelByEscape(page, "docs-drawer-stream-title");
}

async function expectDrawerReady(page, overlayPanel, drawerRoot) {
  const overlayRoot = overlayForPanel(page, overlayPanel);
  await expect(overlayRoot).toHaveAttribute("data-state", "open");
  await expect(overlayRoot).toHaveAttribute("data-open", "true");
  await expect(overlayPanel).toBeVisible();
  await expect(overlayPanel).toHaveAttribute("aria-modal", "true");
  await expect(drawerRoot).toHaveAttribute("data-open-state", "open");
  await expect(drawerRoot).toHaveAttribute("data-ui-output-status", "verified");
  return overlayRoot;
}

async function expectDrawerSettledClosed(overlayPanel, drawerRoot, overlayRoot) {
  await expect(overlayPanel).toHaveCount(0);
  await expect(drawerRoot).toHaveCount(0);
  await expect(overlayRoot).toHaveCount(0);
}

test("docs-app drawer contract uses semantic selectors with settled waits", async ({ page }) => {
  await page.goto("/#/components/drawer");
  await waitForWasmReady(page);
  await ensureDrawerBaseline(page);

  const docsRoot = page.locator('[data-component="drawer"]').first();
  await expect(docsRoot).toBeVisible();

  const controls = docsRoot.locator('[data-slot="drawer-e2e-right-controls"]').first();
  const openRight = controls.locator('[data-slot="drawer-e2e-open-right"]').first();
  await openRight.click();

  const rightPanel = page
    .locator('[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-drawer-right-title"]')
    .first();
  const rightDrawer = rightPanel.locator('[data-slot="drawer"]').first();
  const rightOverlay = await expectDrawerReady(page, rightPanel, rightDrawer);

  await expect(rightDrawer).toHaveAttribute("data-state", "with-description");
  await expect(rightDrawer).toHaveAttribute("data-description", "present");
  await expect(rightDrawer).toHaveAttribute("data-footer", "present");
  await expect(rightDrawer).toHaveAttribute("data-close-button", "shown");
  await expect(rightDrawer).toHaveAttribute("data-placement", "right");
  await expect(rightDrawer).toHaveAttribute("data-open-mode", "controlled");
  await expect(rightDrawer).toHaveAttribute("data-open-source", "external");
  await expect(rightDrawer).toHaveAttribute("data-open-action-source", "programmatic");
  await expect(rightDrawer).toHaveAttribute("data-title-source", "custom");
  await expect(rightDrawer).toHaveAttribute("data-description-source", "custom");

  await rightPanel.press("Escape");
  await expectDrawerSettledClosed(rightPanel, rightDrawer, rightOverlay);
});

test("docs-app drawer motion path uses semantic ready and settled breakpoints", async ({ page }) => {
  await page.goto("/#/components/drawer");
  await waitForWasmReady(page);
  await ensureDrawerBaseline(page);

  const docsRoot = page.locator('[data-component="drawer"]').first();
  await expect(docsRoot).toBeVisible();

  const controls = docsRoot.locator('[data-slot="drawer-e2e-custom-controls"]').first();
  const openCustom = controls.locator('[data-slot="drawer-e2e-open-custom"]').first();
  await openCustom.click();

  const customPanel = page
    .locator('[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-drawer-left-title"]')
    .first();
  const customDrawer = customPanel.locator('[data-slot="drawer"]').first();
  const customOverlay = await expectDrawerReady(page, customPanel, customDrawer);

  await expect(customDrawer).toHaveAttribute("data-state", "title-only");
  await expect(customDrawer).toHaveAttribute("data-description", "absent");
  await expect(customDrawer).toHaveAttribute("data-close-button", "hidden");
  await expect(customDrawer).toHaveAttribute("data-placement", "left");
  await expect(customDrawer).toHaveAttribute("data-placement-source", "custom");
  await expect(customDrawer).toHaveAttribute("data-motion-source", "custom");
  await expect(customDrawer).toHaveAttribute("data-open-mode", "controlled");
  await expect(customDrawer).toHaveAttribute("data-open-source", "external");

  const backdrop = customOverlay.locator('[data-slot="overlay-backdrop"]').first();
  await backdrop.click();
  await expectDrawerSettledClosed(customPanel, customDrawer, customOverlay);
});

test("docs-app drawer key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await page.goto("/#/components/drawer");
  await waitForWasmReady(page);
  await ensureDrawerBaseline(page);

  const docsRoot = page.locator('[data-component="drawer"]').first();
  await expect(docsRoot).toBeVisible();

  const controls = docsRoot.locator('[data-slot="drawer-e2e-right-controls"]').first();
  const openRight = controls.locator('[data-slot="drawer-e2e-open-right"]').first();

  for (const cycle of [1, 2]) {
    await test.step(`drawer key flow cycle ${cycle}`, async () => {
      await openRight.focus();
      await expect(openRight).toBeFocused();
      await page.keyboard.press("Enter");

      const rightPanel = page
        .locator('[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-drawer-right-title"]')
        .first();
      const rightDrawer = rightPanel.locator('[data-slot="drawer"]').first();
      const rightOverlay = await expectDrawerReady(page, rightPanel, rightDrawer);

      await expect(rightDrawer).toHaveAttribute("data-open-mode", "controlled");
      await expect(rightDrawer).toHaveAttribute("data-open-source", "external");
      await expect(rightDrawer).toHaveAttribute("data-open-action-source", "programmatic");
      await expect(rightDrawer).toHaveAttribute("data-state", "with-description");

      await expectFocusInsidePanel(rightPanel);
      await page.keyboard.press("Tab");
      await expectFocusInsidePanel(rightPanel);
      await page.keyboard.press("Tab");
      await expectFocusInsidePanel(rightPanel);

      await rightPanel.press("Escape");
      await expectDrawerSettledClosed(rightPanel, rightDrawer, rightOverlay);
      await expect(openRight).toBeFocused();
    });
  }
});

test("docs-app drawer high-risk paths keep overlay focus keyboard and settled semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/drawer");
  await waitForWasmReady(page);
  await ensureDrawerBaseline(page);

  const docsRoot = page.locator('[data-component="drawer"]').first();
  await expect(docsRoot).toBeVisible();

  const controls = docsRoot.locator('[data-slot="drawer-e2e-custom-controls"]').first();
  const openCustom = controls.locator('[data-slot="drawer-e2e-open-custom"]').first();

  await openCustom.focus();
  await expect(openCustom).toBeFocused();
  await page.keyboard.press("Enter");

  const customPanel = page
    .locator('[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-drawer-left-title"]')
    .first();
  const customDrawer = customPanel.locator('[data-slot="drawer"]').first();
  const customOverlay = await expectDrawerReady(page, customPanel, customDrawer);

  await expect(customDrawer).toHaveAttribute("data-motion-source", "custom");
  await expect(customDrawer).toHaveAttribute("data-placement", "left");
  await expect(customDrawer).toHaveAttribute("data-open-mode", "controlled");
  await expect(customDrawer).toHaveAttribute("data-open-source", "external");

  await expectFocusInsidePanel(customPanel);
  await page.keyboard.press("Tab");
  await expectFocusInsidePanel(customPanel);
  await page.keyboard.press("Shift+Tab");
  await expectFocusInsidePanel(customPanel);

  const backdrop = customOverlay.locator('[data-slot="overlay-backdrop"]').first();
  await backdrop.click();
  await expectDrawerSettledClosed(customPanel, customDrawer, customOverlay);
});
