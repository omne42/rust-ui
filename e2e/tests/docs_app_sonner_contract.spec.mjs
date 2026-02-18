import { expect, test } from "@playwright/test";

test("docs-app sonner uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  await page.goto("/#/components/sonner");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="sonner"][data-slot="sonner"]');
  await expect(docsRoot).toBeVisible();
  await expect(docsRoot).toHaveAttribute("data-ui-schema", "ui.sonner.v1");
  await expect(docsRoot).toHaveAttribute("data-ui-stream-support", "optional");
  await expect(docsRoot).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(docsRoot).toHaveAttribute("data-ui-output-status", "verified");

  const sourceHost = docsRoot
    .locator(
      '[data-slot="sonner"][data-state="inline"][data-position="top-left"][data-store-source="provided"][data-motion-source="custom"]'
    )
    .first();
  await expect(sourceHost).toBeVisible();
  await expect(sourceHost).toHaveAttribute("role", "region");

  const sourceViewport = sourceHost.locator('[data-slot="toast-viewport"]').first();
  await expect(sourceViewport).toBeVisible();
  await expect(sourceViewport).toHaveAttribute("data-state", "inline");
  await expect(sourceViewport).toHaveAttribute("data-motion-source", "custom");
  await expect(sourceViewport).toHaveAttribute("data-store-source", "provided");
  await expect(sourceViewport).toHaveAttribute("data-queue", "extended");
});

test("docs-app sonner covers async/motion ready-settled path with semantic markers", async ({
  page,
}) => {
  await page.goto("/#/components/sonner");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="sonner"][data-slot="sonner"]');
  const sourceControls = docsRoot.locator('[data-slot="sonner-source-controls"]');
  const pushSource = sourceControls
    .locator('[data-slot="sonner-source-push"] [data-slot="button"]')
    .first();
  const clearSource = sourceControls
    .locator('[data-slot="sonner-source-clear"] [data-slot="button"]')
    .first();

  const sourceHost = docsRoot
    .locator(
      '[data-slot="sonner"][data-state="inline"][data-position="top-left"][data-store-source="provided"][data-motion-source="custom"]'
    )
    .first();
  const sourceViewport = sourceHost.locator('[data-slot="toast-viewport"]').first();

  await expect(pushSource).toBeVisible();
  await expect(clearSource).toBeVisible();
  await expect(sourceViewport).toBeVisible();

  await pushSource.click();
  const toast = sourceViewport.locator('[data-slot="toast"]').first();
  await expect(toast).toHaveAttribute("data-state", "open");
  await expect(toast).toHaveAttribute("data-open", "true");

  await clearSource.click();
  await expect(sourceViewport.locator('[data-slot="toast"][data-open="true"]')).toHaveCount(0);
  await expect(sourceViewport.locator('[data-slot="toast"]')).toHaveCount(0, {
    timeout: 6000,
  });
});

test("docs-app sonner key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/sonner");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="sonner"][data-slot="sonner"]');
  const portalControls = docsRoot.locator('[data-slot="sonner-portal-controls"]');
  const pushSuccess = portalControls
    .locator('[data-slot="sonner-portal-push-success"] [data-slot="button"]')
    .first();

  const portalViewport = page
    .locator('[data-slot="toast-viewport"][data-state="portal"][data-store-source="provided"]')
    .first();

  await expect(pushSuccess).toBeVisible();
  await expect(portalViewport).toBeVisible();

  await pushSuccess.click();
  const toast = portalViewport.locator('[data-slot="toast"]').first();
  const closeButton = toast.locator('[data-slot="toast-close"]').first();
  await expect(toast).toHaveAttribute("data-state", "open");
  await expect(toast).toHaveAttribute("data-open", "true");

  await closeButton.focus();
  await expect(closeButton).toBeFocused();
  await page.keyboard.press("Enter");

  await expect(portalViewport.locator('[data-slot="toast"][data-open="true"]')).toHaveCount(0);
  await expect(portalViewport.locator('[data-slot="toast"]')).toHaveCount(0, {
    timeout: 6000,
  });
});
