import { expect, test } from "@playwright/test";

async function gotoImageDocsAndWaitSettled(page) {
  await page.goto("/#/components/image");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="image"]').first();
  await expect(docsRoot).toBeVisible();

  const settledImage = docsRoot.locator('[data-slot="image-wrapper"][data-state]').first();
  await expect(settledImage).toBeVisible();
  await expect(settledImage).toHaveAttribute("data-state", /(idle|loading|loaded|error)/);
  await expect(settledImage).toHaveAttribute("data-status-source", /(initial|event)/);

  return docsRoot;
}

test("docs-app image uses semantic selectors with wasm-stable ready waits", async ({ page }) => {
  const docsRoot = await gotoImageDocsAndWaitSettled(page);

  const primaryImage = docsRoot
    .locator(
      '[data-slot="image-wrapper"][data-radius][data-shadow][data-motion-source][data-custom-motion]'
    )
    .first();
  await expect(primaryImage).toBeVisible();
  await expect(primaryImage).toHaveAttribute("data-radius", /(sm|md|lg|full)/);
  await expect(primaryImage).toHaveAttribute("data-shadow", /(none|sm|md)/);

  const matrixFallback = docsRoot
    .locator('[data-slot="image-wrapper"][data-fallback="true"] [data-slot="image-fallback"]')
    .first();
  await expect(matrixFallback).toBeVisible();
});

test("docs-app image key flow is repeatable via semantic breakpoints", async ({ page }) => {
  const docsRoot = await gotoImageDocsAndWaitSettled(page);

  const workbench = docsRoot
    .locator('[data-slot="playground"]')
    .filter({ has: docsRoot.locator('[data-slot="image-workbench-stage"]') })
    .first();
  await expect(workbench).toBeVisible();

  const settingsButton = workbench.locator('[data-slot="playground-toggle-settings"]').first();
  await settingsButton.click();

  const controls = workbench.locator('[data-slot="playground-controls"]').first();
  await expect(controls).toBeVisible();

  const sourceControl = controls
    .locator('[data-slot="segmented-control"]')
    .filter({ has: controls.locator('[id^="docs-image-source"]') })
    .first();
  await sourceControl.locator('[data-slot="segmented-control-option"][data-index="2"]').click();

  const configuredImage = workbench.locator('[data-slot="image-wrapper"]').first();
  await expect(configuredImage).toBeVisible();
  await expect(configuredImage).toHaveAttribute("data-fallback", "true");
  await expect(configuredImage).toHaveAttribute("data-state", "idle");
  await expect(configuredImage).toHaveAttribute("data-status-source", "initial");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  const reloadedImage = page
    .locator('[data-component="image"] [data-slot="image-wrapper"][data-state]')
    .first();
  await expect(reloadedImage).toBeVisible();
  await expect(reloadedImage).toHaveAttribute("data-state", /(idle|loading|loaded|error)/);
  await expect(reloadedImage).toHaveAttribute("data-status-source", /(initial|event)/);
});
