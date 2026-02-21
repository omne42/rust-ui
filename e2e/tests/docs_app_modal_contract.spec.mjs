import { expect, test } from "@playwright/test";

async function expectFocusInsidePanel(panel) {
  await expect
    .poll(async () => panel.evaluate((node) => node.contains(node.ownerDocument.activeElement)))
    .toBe(true);
}

test("docs-app modal contract uses semantic selectors with settled waits", async ({ page }) => {
  await page.goto("/#/components/modal");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="modal"]').first();
  await expect(docsRoot).toBeVisible();

  const describedControls = docsRoot.locator('[data-slot="modal-e2e-described-controls"]').first();
  const openDescribed = describedControls.locator('[data-slot="modal-e2e-open-described"]').first();
  await openDescribed.click();

  const describedPanel = page
    .locator('[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-modal-semantic-title"]')
    .first();
  const describedModal = describedPanel.locator('[data-slot="modal"]').first();

  await expect(describedPanel).toBeVisible();
  await expect(describedPanel).toHaveAttribute("aria-modal", "true");
  await expect(describedModal).toHaveAttribute("data-state", "with-description");
  await expect(describedModal).toHaveAttribute("data-description", "present");
  await expect(describedModal).toHaveAttribute("data-open-mode", "controlled");
  await expect(describedModal).toHaveAttribute("data-open-source", "controlled");
  await expect(describedModal).toHaveAttribute("data-open-change-source", "none");
  await expect(describedModal).toHaveAttribute("data-open-prop-source", "is_open");
  await expect(describedModal).toHaveAttribute("data-title-source", "custom");
  await expect(describedModal).toHaveAttribute("data-description-source", "custom");
  await expect(describedModal).toHaveAttribute("data-ui-output-status", "verified");

  await describedPanel.press("Escape");
  await expect(describedPanel).toHaveCount(0);
});

test("docs-app modal motion path uses semantic ready and settled breakpoints", async ({ page }) => {
  await page.goto("/#/components/modal");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="modal"]').first();
  await expect(docsRoot).toBeVisible();

  const customControls = docsRoot.locator('[data-slot="modal-e2e-custom-controls"]').first();
  const openCustom = customControls.locator('[data-slot="modal-e2e-open-custom"]').first();
  await openCustom.click();

  const customPanel = page
    .locator('[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-modal-custom-title"]')
    .first();
  const customModal = customPanel.locator('[data-slot="modal"]').first();

  await expect(customPanel).toBeVisible();
  await expect(customModal).toHaveAttribute("data-state", "title-only");
  await expect(customModal).toHaveAttribute("data-description", "absent");
  await expect(customModal).toHaveAttribute("data-motion-source", "custom");
  await expect(customModal).toHaveAttribute("data-open-mode", "controlled");
  await expect(customModal).toHaveAttribute("data-open-source", "controlled");
  await expect(customModal).toHaveAttribute("data-open-change-source", "none");
  await expect(customModal).toHaveAttribute("data-open-prop-source", "is_open");

  const customOverlay = page.locator('[data-slot="overlay"]').filter({ has: customPanel }).first();
  const customBackdrop = customOverlay.locator('[data-slot="overlay-backdrop"]').first();
  await customBackdrop.click();
  await expect(customPanel).toHaveCount(0);
});

test("docs-app modal critical flow is replayable with overlay focus and keyboard checkpoints", async ({
  page,
}) => {
  await page.goto("/#/components/modal");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="modal"]').first();
  await expect(docsRoot).toBeVisible();

  const describedControls = docsRoot.locator('[data-slot="modal-e2e-described-controls"]').first();
  const openDescribed = describedControls.locator('[data-slot="modal-e2e-open-described"]').first();

  for (const cycle of [1, 2]) {
    await test.step(`modal critical flow cycle ${cycle}`, async () => {
      await openDescribed.focus();
      await expect(openDescribed).toBeFocused();

      await openDescribed.click();

      const describedPanel = page
        .locator('[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-modal-semantic-title"]')
        .first();
      const describedModal = describedPanel.locator('[data-slot="modal"]').first();

      await expect(describedPanel).toBeVisible();
      await expect(describedPanel).toHaveAttribute("aria-modal", "true");
      await expect(describedModal).toHaveAttribute("data-open-mode", "controlled");
      await expect(describedModal).toHaveAttribute("data-open-source", "controlled");
      await expect(describedModal).toHaveAttribute("data-open-prop-source", "is_open");

      await expectFocusInsidePanel(describedPanel);
      await page.keyboard.press("Tab");
      await expectFocusInsidePanel(describedPanel);
      await page.keyboard.press("Tab");
      await expectFocusInsidePanel(describedPanel);

      await describedPanel.press("Escape");
      await expect(describedPanel).toHaveCount(0);
      await expect(openDescribed).toBeFocused();
    });
  }
});

test("docs-app modal interactive playground replays open-close flow with stable semantic anchors", async ({
  page,
}) => {
  await page.goto("/#/components/modal");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="modal"]').first();
  await expect(docsRoot).toBeVisible();

  const interactiveControls = docsRoot.locator('[data-slot="modal-interactive-controls"]').first();
  const openInteractive = interactiveControls.locator('[data-slot="modal-interactive-open"]').first();

  for (const cycle of [1, 2]) {
    await test.step(`modal interactive playground cycle ${cycle}`, async () => {
      await openInteractive.focus();
      await expect(openInteractive).toBeFocused();
      await openInteractive.click();

      const interactivePanel = page
        .locator('[data-slot="overlay-panel"][role="dialog"]', {
          has: page.locator(
            '[data-slot="modal"][data-id-source="custom"][data-title-source="custom"][data-open-mode="controlled"]',
          ),
        })
        .first();
      const interactiveModal = interactivePanel.locator('[data-slot="modal"]').first();
      const interactiveClose = interactivePanel.locator('[data-slot="modal-interactive-close"]').first();

      await expect(interactivePanel).toBeVisible();
      await expect(interactivePanel).toHaveAttribute("aria-modal", "true");
      await expect(interactiveModal).toHaveAttribute("data-description", "present");
      await expect(interactiveModal).toHaveAttribute("data-motion-source", "default");
      await expect(interactiveModal).toHaveAttribute("data-open-mode", "controlled");
      await expect(interactiveModal).toHaveAttribute("data-open-source", "controlled");
      await expect(interactiveModal).toHaveAttribute("data-open-prop-source", "is_open");

      await interactiveClose.click();
      await expect(interactivePanel).toHaveCount(0);
      await expect(openInteractive).toBeFocused();
    });
  }
});
